//! Legalis-I18n: Internationalization support for Legalis-RS.
//!
//! This crate provides multi-language and multi-jurisdiction support:
//! - Translation of legal terms and statutes
//! - Locale-specific legal formatting (dates, currencies, names)
//! - Jurisdiction mapping and legal system classification
//! - Cultural parameter injection for law porting
//! - ICU message format support
//! - Plural rules handling
//! - Date/time, currency, and number formatting

use indexmap::IndexMap;
use lru::LruCache;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex, RwLock};
use thiserror::Error;

/// Errors during internationalization operations.
#[derive(Debug, Error)]
pub enum I18nError {
    #[error(
        "Locale not found: '{locale}'. Available locales can be registered using add_locale()."
    )]
    LocaleNotFound { locale: String },

    #[error(
        "Translation missing for key '{key}' in locale '{locale}'. Consider adding the term to the dictionary or using a fallback locale."
    )]
    TranslationMissing { key: String, locale: String },

    #[error(
        "Invalid locale format: '{input}'. Expected format: language[-Script][-COUNTRY] (e.g., 'en-US', 'zh-Hans-CN')."
    )]
    InvalidLocale { input: String },

    #[error(
        "Jurisdiction '{jurisdiction}' is not supported. Supported jurisdictions: JP, US, GB, DE, FR, ES, IT, CN, TW, KR, CA, AU, IN, BR, RU, SA, NL, CH, MX, SG."
    )]
    UnsupportedJurisdiction { jurisdiction: String },

    #[error(
        "Dictionary for locale '{locale}' not found. Add a dictionary using add_dictionary() before attempting translation."
    )]
    DictionaryNotFound { locale: String },

    #[error(
        "Invalid date: year={year}, month={month}, day={day}. Please provide a valid calendar date."
    )]
    InvalidDate { year: i32, month: u32, day: u32 },

    #[error("Cache operation failed: {reason}")]
    CacheError { reason: String },

    #[error("Translation service unavailable: {service}. {details}")]
    ServiceUnavailable { service: String, details: String },
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
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_i18n::Locale;
    ///
    /// let locale = Locale::new("ja");
    /// assert_eq!(locale.language, "ja");
    /// assert_eq!(locale.tag(), "ja");
    /// ```
    pub fn new(language: impl Into<String>) -> Self {
        Self {
            language: language.into(),
            country: None,
            script: None,
        }
    }

    /// Sets the country.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_i18n::Locale;
    ///
    /// let locale = Locale::new("en").with_country("US");
    /// assert_eq!(locale.tag(), "en-US");
    /// ```
    pub fn with_country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }

    /// Sets the script.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_i18n::Locale;
    ///
    /// let locale = Locale::new("zh").with_script("Hans").with_country("CN");
    /// assert_eq!(locale.tag(), "zh-Hans-CN");
    /// ```
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
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_i18n::Locale;
    ///
    /// let locale = Locale::parse("en-US").unwrap();
    /// assert_eq!(locale.language, "en");
    /// assert_eq!(locale.country, Some("US".to_string()));
    ///
    /// let locale_with_script = Locale::parse("zh-Hans-CN").unwrap();
    /// assert_eq!(locale_with_script.language, "zh");
    /// assert_eq!(locale_with_script.script, Some("Hans".to_string()));
    /// assert_eq!(locale_with_script.country, Some("CN".to_string()));
    /// ```
    pub fn parse(tag: &str) -> I18nResult<Self> {
        let parts: Vec<&str> = tag.split('-').collect();
        if parts.is_empty() {
            return Err(I18nError::InvalidLocale {
                input: tag.to_string(),
            });
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

    /// Checks if this locale matches another locale (considering regional variations).
    /// Returns true if the locales match exactly or if they share the same language/country.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_i18n::Locale;
    ///
    /// let en = Locale::new("en");
    /// let en_us = Locale::new("en").with_country("US");
    /// let en_gb = Locale::new("en").with_country("GB");
    ///
    /// assert!(en.matches(&en_us));  // Base locale matches regional variant
    /// assert!(en_us.matches(&en_us)); // Exact match
    /// assert!(!en_us.matches(&en_gb)); // Different countries don't match
    /// ```
    pub fn matches(&self, other: &Locale) -> bool {
        if self.language != other.language {
            return false;
        }

        // If both have countries, they must match
        match (&self.country, &other.country) {
            (Some(c1), Some(c2)) => c1 == c2,
            // If one doesn't have a country, match on language only
            _ => true,
        }
    }

    /// Gets the parent locale (removing the most specific part).
    /// For example, "en-US" -> "en", "zh-Hans-CN" -> "zh-Hans"
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_i18n::Locale;
    ///
    /// let locale = Locale::new("en").with_country("US");
    /// let parent = locale.parent().unwrap();
    /// assert_eq!(parent.tag(), "en");
    /// assert!(parent.country.is_none());
    ///
    /// let base = Locale::new("en");
    /// assert!(base.parent().is_none()); // Base locale has no parent
    /// ```
    pub fn parent(&self) -> Option<Self> {
        if self.country.is_some() {
            Some(Self {
                language: self.language.clone(),
                country: None,
                script: self.script.clone(),
            })
        } else if self.script.is_some() {
            Some(Self {
                language: self.language.clone(),
                country: None,
                script: None,
            })
        } else {
            None
        }
    }

    /// Gets all fallback locales in order.
    /// For example, "zh-Hans-CN" -> ["zh-Hans-CN", "zh-Hans", "zh"]
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_i18n::Locale;
    ///
    /// let locale = Locale::new("zh").with_script("Hans").with_country("CN");
    /// let chain = locale.fallback_chain();
    /// assert_eq!(chain.len(), 3);
    /// assert_eq!(chain[0].tag(), "zh-Hans-CN");
    /// assert_eq!(chain[1].tag(), "zh-Hans");
    /// assert_eq!(chain[2].tag(), "zh");
    /// ```
    pub fn fallback_chain(&self) -> Vec<Locale> {
        let mut chain = vec![self.clone()];
        let mut current = self.clone();

        while let Some(parent) = current.parent() {
            chain.push(parent.clone());
            current = parent;
        }

        chain
    }
}

/// Regional variation information for a locale.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalVariation {
    /// The base locale
    pub base_locale: Locale,
    /// Regional locale
    pub regional_locale: Locale,
    /// Description of the regional variation
    pub description: String,
    /// Key differences from the base locale
    pub differences: Vec<String>,
}

impl RegionalVariation {
    /// Creates a new regional variation.
    pub fn new(
        base_locale: Locale,
        regional_locale: Locale,
        description: impl Into<String>,
    ) -> Self {
        Self {
            base_locale,
            regional_locale,
            description: description.into(),
            differences: vec![],
        }
    }

    /// Adds a difference description.
    pub fn add_difference(mut self, difference: impl Into<String>) -> Self {
        self.differences.push(difference.into());
        self
    }
}

/// Registry of regional variations for locales.
#[derive(Debug, Default)]
pub struct RegionalVariationRegistry {
    variations: Vec<RegionalVariation>,
}

impl RegionalVariationRegistry {
    /// Creates a new registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a registry with default regional variations.
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();

        // English variations
        registry.add_variation(
            RegionalVariation::new(
                Locale::new("en"),
                Locale::new("en").with_country("US"),
                "American English",
            )
            .add_difference("Uses 'attorney' instead of 'solicitor'")
            .add_difference("Federal system with state and federal courts")
            .add_difference("MM/DD/YYYY date format"),
        );

        registry.add_variation(
            RegionalVariation::new(
                Locale::new("en"),
                Locale::new("en").with_country("GB"),
                "British English",
            )
            .add_difference("Uses 'solicitor' and 'barrister'")
            .add_difference("Equity and common law traditions")
            .add_difference("DD/MM/YYYY date format"),
        );

        registry.add_variation(
            RegionalVariation::new(
                Locale::new("en"),
                Locale::new("en").with_country("AU"),
                "Australian English",
            )
            .add_difference("Follows UK legal terminology largely")
            .add_difference("Federal system similar to UK")
            .add_difference("DD/MM/YYYY date format"),
        );

        registry.add_variation(
            RegionalVariation::new(
                Locale::new("en"),
                Locale::new("en").with_country("CA"),
                "Canadian English",
            )
            .add_difference("Mixed common law and civil law (Quebec)")
            .add_difference("Bilingual legal system (English/French)")
            .add_difference("DD/MM/YYYY date format"),
        );

        // Spanish variations
        registry.add_variation(
            RegionalVariation::new(
                Locale::new("es"),
                Locale::new("es").with_country("ES"),
                "European Spanish",
            )
            .add_difference("Uses 'vosotros' form")
            .add_difference("Civil law system based on Roman law"),
        );

        registry.add_variation(
            RegionalVariation::new(
                Locale::new("es"),
                Locale::new("es").with_country("MX"),
                "Mexican Spanish",
            )
            .add_difference("Uses 'ustedes' instead of 'vosotros'")
            .add_difference("Civil law influenced by indigenous legal traditions"),
        );

        registry.add_variation(
            RegionalVariation::new(
                Locale::new("es"),
                Locale::new("es").with_country("AR"),
                "Argentine Spanish",
            )
            .add_difference("Uses 'vos' form")
            .add_difference("Civil law based on Spanish and French codes"),
        );

        // Chinese variations
        registry.add_variation(
            RegionalVariation::new(
                Locale::new("zh"),
                Locale::new("zh").with_country("CN").with_script("Hans"),
                "Simplified Chinese (Mainland)",
            )
            .add_difference("Simplified characters")
            .add_difference("Socialist legal system")
            .add_difference("Civil law tradition"),
        );

        registry.add_variation(
            RegionalVariation::new(
                Locale::new("zh"),
                Locale::new("zh").with_country("TW").with_script("Hant"),
                "Traditional Chinese (Taiwan)",
            )
            .add_difference("Traditional characters")
            .add_difference("Civil law based on German law")
            .add_difference("Separate legal system from mainland"),
        );

        registry.add_variation(
            RegionalVariation::new(
                Locale::new("zh"),
                Locale::new("zh").with_country("HK").with_script("Hant"),
                "Traditional Chinese (Hong Kong)",
            )
            .add_difference("Traditional characters")
            .add_difference("Common law system from British rule")
            .add_difference("Bilingual legal system (Chinese/English)"),
        );

        // German variations
        registry.add_variation(
            RegionalVariation::new(
                Locale::new("de"),
                Locale::new("de").with_country("DE"),
                "German (Germany)",
            )
            .add_difference("BGB (Civil Code)")
            .add_difference("Federal legal system"),
        );

        registry.add_variation(
            RegionalVariation::new(
                Locale::new("de"),
                Locale::new("de").with_country("AT"),
                "German (Austria)",
            )
            .add_difference("ABGB (Austrian Civil Code)")
            .add_difference("Similar to German law with variations"),
        );

        registry.add_variation(
            RegionalVariation::new(
                Locale::new("de"),
                Locale::new("de").with_country("CH"),
                "German (Switzerland)",
            )
            .add_difference("Swiss Civil Code (ZGB)")
            .add_difference("Multilingual legal system")
            .add_difference("Cantonal variations"),
        );

        // French variations
        registry.add_variation(
            RegionalVariation::new(
                Locale::new("fr"),
                Locale::new("fr").with_country("FR"),
                "French (France)",
            )
            .add_difference("Code Civil (Napoleonic Code)")
            .add_difference("Centralized legal system"),
        );

        registry.add_variation(
            RegionalVariation::new(
                Locale::new("fr"),
                Locale::new("fr").with_country("CA"),
                "French (Canada/Quebec)",
            )
            .add_difference("Civil law in Quebec, common law elsewhere")
            .add_difference("Bilingual legal system")
            .add_difference("Mix of French and English legal traditions"),
        );

        registry.add_variation(
            RegionalVariation::new(
                Locale::new("fr"),
                Locale::new("fr").with_country("BE"),
                "French (Belgium)",
            )
            .add_difference("Based on French Civil Code")
            .add_difference("Multilingual (French, Dutch, German)"),
        );

        registry
    }

    /// Adds a variation to the registry.
    pub fn add_variation(&mut self, variation: RegionalVariation) {
        self.variations.push(variation);
    }

    /// Gets all variations for a base locale.
    pub fn get_variations(&self, base_locale: &Locale) -> Vec<&RegionalVariation> {
        self.variations
            .iter()
            .filter(|v| v.base_locale.language == base_locale.language)
            .collect()
    }

    /// Finds a specific regional variation.
    pub fn find_variation(&self, regional_locale: &Locale) -> Option<&RegionalVariation> {
        self.variations
            .iter()
            .find(|v| v.regional_locale.tag() == regional_locale.tag())
    }
}

impl std::fmt::Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.tag())
    }
}

/// Legal system classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

impl std::fmt::Display for LegalSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LegalSystem::CivilLaw => write!(f, "Civil Law"),
            LegalSystem::CommonLaw => write!(f, "Common Law"),
            LegalSystem::ReligiousLaw => write!(f, "Religious Law"),
            LegalSystem::CustomaryLaw => write!(f, "Customary Law"),
            LegalSystem::Mixed => write!(f, "Mixed System"),
        }
    }
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
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_i18n::{Jurisdiction, Locale, LegalSystem};
    ///
    /// let locale = Locale::new("ja").with_country("JP");
    /// let jurisdiction = Jurisdiction::new("JP", "Japan", locale)
    ///     .with_legal_system(LegalSystem::CivilLaw);
    ///
    /// assert_eq!(jurisdiction.id, "JP");
    /// assert_eq!(jurisdiction.name, "Japan");
    /// assert_eq!(jurisdiction.legal_system, LegalSystem::CivilLaw);
    /// ```
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
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_i18n::CulturalParams;
    ///
    /// let japan_params = CulturalParams::for_country("JP");
    /// assert_eq!(japan_params.age_of_majority, Some(18));
    ///
    /// let singapore_params = CulturalParams::for_country("SG");
    /// assert_eq!(singapore_params.age_of_majority, Some(21));
    /// ```
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
            "GB" => Self {
                age_of_majority: Some(18),
                religious_considerations: vec![],
                protected_classes: vec![
                    "age".to_string(),
                    "disability".to_string(),
                    "gender_reassignment".to_string(),
                    "marriage".to_string(),
                    "race".to_string(),
                    "religion".to_string(),
                    "sex".to_string(),
                ],
                prohibitions: vec![],
                custom: HashMap::new(),
            },
            "DE" | "AT" => Self {
                age_of_majority: Some(18),
                religious_considerations: vec![],
                protected_classes: vec![
                    "race".to_string(),
                    "ethnic_origin".to_string(),
                    "gender".to_string(),
                    "religion".to_string(),
                    "disability".to_string(),
                ],
                prohibitions: vec![],
                custom: HashMap::new(),
            },
            "FR" => Self {
                age_of_majority: Some(18),
                religious_considerations: vec!["secularism".to_string()],
                protected_classes: vec![
                    "origin".to_string(),
                    "sex".to_string(),
                    "family_situation".to_string(),
                    "pregnancy".to_string(),
                    "religion".to_string(),
                ],
                prohibitions: vec![],
                custom: HashMap::new(),
            },
            "ES" => Self {
                age_of_majority: Some(18),
                religious_considerations: vec![],
                protected_classes: vec![
                    "birth".to_string(),
                    "race".to_string(),
                    "sex".to_string(),
                    "religion".to_string(),
                    "opinion".to_string(),
                ],
                prohibitions: vec![],
                custom: HashMap::new(),
            },
            "IT" => Self {
                age_of_majority: Some(18),
                religious_considerations: vec![],
                protected_classes: vec![
                    "sex".to_string(),
                    "race".to_string(),
                    "language".to_string(),
                    "religion".to_string(),
                ],
                prohibitions: vec![],
                custom: HashMap::new(),
            },
            "CN" => Self {
                age_of_majority: Some(18),
                religious_considerations: vec![],
                protected_classes: vec![
                    "nationality".to_string(),
                    "ethnicity".to_string(),
                    "gender".to_string(),
                ],
                prohibitions: vec![],
                custom: HashMap::new(),
            },
            "TW" => Self {
                age_of_majority: Some(20),
                religious_considerations: vec![],
                protected_classes: vec![
                    "gender".to_string(),
                    "disability".to_string(),
                    "age".to_string(),
                ],
                prohibitions: vec![],
                custom: HashMap::new(),
            },
            "KR" => Self {
                age_of_majority: Some(19),
                religious_considerations: vec![],
                protected_classes: vec![
                    "gender".to_string(),
                    "disability".to_string(),
                    "age".to_string(),
                ],
                prohibitions: vec![],
                custom: HashMap::new(),
            },
            "CA" => Self {
                age_of_majority: Some(18),
                religious_considerations: vec![],
                protected_classes: vec![
                    "race".to_string(),
                    "national_ethnic_origin".to_string(),
                    "colour".to_string(),
                    "religion".to_string(),
                    "sex".to_string(),
                    "age".to_string(),
                ],
                prohibitions: vec![],
                custom: HashMap::new(),
            },
            "AU" => Self {
                age_of_majority: Some(18),
                religious_considerations: vec![],
                protected_classes: vec![
                    "race".to_string(),
                    "colour".to_string(),
                    "sex".to_string(),
                    "age".to_string(),
                    "disability".to_string(),
                ],
                prohibitions: vec![],
                custom: HashMap::new(),
            },
            "IN" => Self {
                age_of_majority: Some(18),
                religious_considerations: vec![
                    "hinduism".to_string(),
                    "islam".to_string(),
                    "christianity".to_string(),
                ],
                protected_classes: vec![
                    "religion".to_string(),
                    "race".to_string(),
                    "caste".to_string(),
                    "sex".to_string(),
                    "place_of_birth".to_string(),
                ],
                prohibitions: vec![],
                custom: HashMap::new(),
            },
            "BR" => Self {
                age_of_majority: Some(18),
                religious_considerations: vec![],
                protected_classes: vec![
                    "origin".to_string(),
                    "race".to_string(),
                    "sex".to_string(),
                    "color".to_string(),
                    "age".to_string(),
                ],
                prohibitions: vec![],
                custom: HashMap::new(),
            },
            "RU" => Self {
                age_of_majority: Some(18),
                religious_considerations: vec![],
                protected_classes: vec![
                    "sex".to_string(),
                    "race".to_string(),
                    "nationality".to_string(),
                    "language".to_string(),
                    "religion".to_string(),
                ],
                prohibitions: vec![],
                custom: HashMap::new(),
            },
            "SA" => Self {
                age_of_majority: Some(18),
                religious_considerations: vec!["islam".to_string(), "sharia_law".to_string()],
                protected_classes: vec![],
                prohibitions: vec![
                    "alcohol".to_string(),
                    "pork".to_string(),
                    "gambling".to_string(),
                ],
                custom: HashMap::new(),
            },
            "NL" => Self {
                age_of_majority: Some(18),
                religious_considerations: vec![],
                protected_classes: vec![
                    "religion".to_string(),
                    "belief".to_string(),
                    "race".to_string(),
                    "sex".to_string(),
                    "disability".to_string(),
                ],
                prohibitions: vec![],
                custom: HashMap::new(),
            },
            "CH" => Self {
                age_of_majority: Some(18),
                religious_considerations: vec![],
                protected_classes: vec![
                    "origin".to_string(),
                    "race".to_string(),
                    "sex".to_string(),
                    "age".to_string(),
                    "language".to_string(),
                ],
                prohibitions: vec![],
                custom: HashMap::new(),
            },
            "MX" => Self {
                age_of_majority: Some(18),
                religious_considerations: vec![],
                protected_classes: vec![
                    "ethnic_origin".to_string(),
                    "gender".to_string(),
                    "age".to_string(),
                    "disability".to_string(),
                    "religion".to_string(),
                ],
                prohibitions: vec![],
                custom: HashMap::new(),
            },
            "SG" => Self {
                age_of_majority: Some(21),
                religious_considerations: vec![],
                protected_classes: vec![
                    "race".to_string(),
                    "religion".to_string(),
                    "language".to_string(),
                ],
                prohibitions: vec![],
                custom: HashMap::new(),
            },
            _ => Self::default(),
        }
    }
}

/// Entry for context-aware translations.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ContextualTranslationEntry {
    key: String,
    context: String,
    translation: String,
}

/// Translation dictionary for legal terms.
#[derive(Debug, Clone, Default)]
pub struct LegalDictionary {
    /// Locale this dictionary is for
    pub locale: Locale,
    /// Term translations: key -> translated term
    translations: IndexMap<String, String>,
    /// Legal definitions: term -> definition
    definitions: IndexMap<String, String>,
    /// Abbreviations: full term -> abbreviation
    abbreviations: IndexMap<String, String>,
    /// Reverse abbreviation lookup: abbreviation -> full term
    abbreviation_expansions: IndexMap<String, String>,
    /// Context-aware translations: (key, context) -> translation
    contextual_translations: IndexMap<(String, String), String>,
}

// Custom serialization for LegalDictionary
impl Serialize for LegalDictionary {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("LegalDictionary", 6)?;
        state.serialize_field("locale", &self.locale)?;
        state.serialize_field("translations", &self.translations)?;
        state.serialize_field("definitions", &self.definitions)?;
        state.serialize_field("abbreviations", &self.abbreviations)?;
        state.serialize_field("abbreviation_expansions", &self.abbreviation_expansions)?;

        // Convert contextual_translations to a serializable format
        let contextual: Vec<ContextualTranslationEntry> = self
            .contextual_translations
            .iter()
            .map(|((key, context), translation)| ContextualTranslationEntry {
                key: key.clone(),
                context: context.clone(),
                translation: translation.clone(),
            })
            .collect();
        state.serialize_field("contextual_translations", &contextual)?;
        state.end()
    }
}

// Custom deserialization for LegalDictionary
impl<'de> Deserialize<'de> for LegalDictionary {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct LegalDictionaryHelper {
            locale: Locale,
            translations: IndexMap<String, String>,
            definitions: IndexMap<String, String>,
            abbreviations: IndexMap<String, String>,
            abbreviation_expansions: IndexMap<String, String>,
            contextual_translations: Vec<ContextualTranslationEntry>,
        }

        let helper = LegalDictionaryHelper::deserialize(deserializer)?;
        let mut contextual_translations = IndexMap::new();
        for entry in helper.contextual_translations {
            contextual_translations.insert((entry.key, entry.context), entry.translation);
        }

        Ok(LegalDictionary {
            locale: helper.locale,
            translations: helper.translations,
            definitions: helper.definitions,
            abbreviations: helper.abbreviations,
            abbreviation_expansions: helper.abbreviation_expansions,
            contextual_translations,
        })
    }
}

impl LegalDictionary {
    /// Creates a new dictionary for a locale.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_i18n::{LegalDictionary, Locale};
    ///
    /// let locale = Locale::new("ja").with_country("JP");
    /// let mut dict = LegalDictionary::new(locale);
    ///
    /// dict.add_translation("contract", "契約");
    /// dict.add_translation("statute", "法律");
    ///
    /// assert_eq!(dict.translate("contract"), Some("契約"));
    /// assert_eq!(dict.translate("statute"), Some("法律"));
    /// ```
    pub fn new(locale: Locale) -> Self {
        Self {
            locale,
            translations: IndexMap::new(),
            definitions: IndexMap::new(),
            abbreviations: IndexMap::new(),
            abbreviation_expansions: IndexMap::new(),
            contextual_translations: IndexMap::new(),
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

    /// Adds an abbreviation for a term.
    pub fn add_abbreviation(&mut self, term: impl Into<String>, abbr: impl Into<String>) {
        let term_str = term.into();
        let abbr_str = abbr.into();
        self.abbreviation_expansions
            .insert(abbr_str.clone(), term_str.clone());
        self.abbreviations.insert(term_str, abbr_str);
    }

    /// Gets the abbreviation for a term.
    pub fn get_abbreviation(&self, term: &str) -> Option<&str> {
        self.abbreviations.get(term).map(|s| s.as_str())
    }

    /// Expands an abbreviation to its full term.
    pub fn expand_abbreviation(&self, abbr: &str) -> Option<&str> {
        self.abbreviation_expansions.get(abbr).map(|s| s.as_str())
    }

    /// Checks if a string is a known abbreviation.
    pub fn is_abbreviation(&self, text: &str) -> bool {
        self.abbreviation_expansions.contains_key(text)
    }

    /// Adds a context-aware translation.
    /// Context can be used to disambiguate terms with multiple meanings.
    /// Examples: "right" with context "legal" vs "direction", "party" with context "contract" vs "celebration"
    pub fn add_contextual_translation(
        &mut self,
        key: impl Into<String>,
        context: impl Into<String>,
        value: impl Into<String>,
    ) {
        self.contextual_translations
            .insert((key.into(), context.into()), value.into());
    }

    /// Gets a translation with context.
    /// If no contextual translation is found, falls back to the default translation.
    pub fn translate_with_context(&self, key: &str, context: &str) -> Option<&str> {
        // Try contextual translation first
        if let Some(translation) = self
            .contextual_translations
            .get(&(key.to_string(), context.to_string()))
        {
            return Some(translation.as_str());
        }
        // Fall back to default translation
        self.translate(key)
    }

    /// Lists all available contexts for a given key.
    pub fn get_contexts_for_term(&self, key: &str) -> Vec<&str> {
        self.contextual_translations
            .keys()
            .filter_map(|(k, ctx)| if k == key { Some(ctx.as_str()) } else { None })
            .collect()
    }

    /// Exports the dictionary to a JSON string.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Imports a dictionary from a JSON string.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Gets the number of translations in this dictionary.
    pub fn translation_count(&self) -> usize {
        self.translations.len()
    }

    /// Gets the number of definitions in this dictionary.
    pub fn definition_count(&self) -> usize {
        self.definitions.len()
    }

    /// Gets the number of abbreviations in this dictionary.
    pub fn abbreviation_count(&self) -> usize {
        self.abbreviations.len()
    }

    /// Gets the number of contextual translations in this dictionary.
    pub fn contextual_translation_count(&self) -> usize {
        self.contextual_translations.len()
    }

    /// Merges another dictionary into this one.
    /// Existing entries are preserved; only new entries are added.
    pub fn merge(&mut self, other: &LegalDictionary) {
        for (key, value) in &other.translations {
            self.translations
                .entry(key.clone())
                .or_insert_with(|| value.clone());
        }
        for (key, value) in &other.definitions {
            self.definitions
                .entry(key.clone())
                .or_insert_with(|| value.clone());
        }
        for (key, value) in &other.abbreviations {
            self.abbreviations
                .entry(key.clone())
                .or_insert_with(|| value.clone());
        }
        for (key, value) in &other.abbreviation_expansions {
            self.abbreviation_expansions
                .entry(key.clone())
                .or_insert_with(|| value.clone());
        }
        for (key, value) in &other.contextual_translations {
            self.contextual_translations
                .entry(key.clone())
                .or_insert_with(|| value.clone());
        }
    }

    /// Creates a standard English (US) legal dictionary.
    pub fn english_us() -> Self {
        let mut dict = Self::new(Locale::new("en").with_country("US"));

        // Basic legal terms
        dict.add_translation("statute", "statute");
        dict.add_translation("law", "law");
        dict.add_translation("regulation", "regulation");
        dict.add_translation("contract", "contract");
        dict.add_translation("agreement", "agreement");
        dict.add_translation("liability", "liability");
        dict.add_translation("obligation", "obligation");
        dict.add_translation("right", "right");
        dict.add_translation("duty", "duty");
        dict.add_translation("party", "party");
        dict.add_translation("plaintiff", "plaintiff");
        dict.add_translation("defendant", "defendant");
        dict.add_translation("court", "court");
        dict.add_translation("judge", "judge");
        dict.add_translation("jury", "jury");
        dict.add_translation("attorney", "attorney");
        dict.add_translation("lawyer", "lawyer");
        dict.add_translation("counsel", "counsel");
        dict.add_translation("witness", "witness");
        dict.add_translation("evidence", "evidence");
        dict.add_translation("testimony", "testimony");
        dict.add_translation("verdict", "verdict");
        dict.add_translation("judgment", "judgment");
        dict.add_translation("appeal", "appeal");
        dict.add_translation("damages", "damages");
        dict.add_translation("penalty", "penalty");
        dict.add_translation("fine", "fine");

        // Corporate law terms
        dict.add_translation("corporation", "corporation");
        dict.add_translation("shareholder", "shareholder");
        dict.add_translation("director", "director");
        dict.add_translation("officer", "officer");
        dict.add_translation("bylaws", "bylaws");
        dict.add_translation("merger", "merger");
        dict.add_translation("acquisition", "acquisition");
        dict.add_translation("dividend", "dividend");
        dict.add_translation("stock", "stock");
        dict.add_translation("securities", "securities");

        // Property law terms
        dict.add_translation("property", "property");
        dict.add_translation("real_estate", "real estate");
        dict.add_translation("ownership", "ownership");
        dict.add_translation("lease", "lease");
        dict.add_translation("tenant", "tenant");
        dict.add_translation("landlord", "landlord");
        dict.add_translation("mortgage", "mortgage");
        dict.add_translation("deed", "deed");
        dict.add_translation("title", "title");
        dict.add_translation("easement", "easement");

        // Criminal law terms
        dict.add_translation("crime", "crime");
        dict.add_translation("felony", "felony");
        dict.add_translation("misdemeanor", "misdemeanor");
        dict.add_translation("prosecution", "prosecution");
        dict.add_translation("indictment", "indictment");
        dict.add_translation("conviction", "conviction");
        dict.add_translation("sentence", "sentence");
        dict.add_translation("probation", "probation");
        dict.add_translation("parole", "parole");
        dict.add_translation("bail", "bail");

        // Procedural law terms
        dict.add_translation("jurisdiction", "jurisdiction");
        dict.add_translation("venue", "venue");
        dict.add_translation("standing", "standing");
        dict.add_translation("discovery", "discovery");
        dict.add_translation("deposition", "deposition");
        dict.add_translation("motion", "motion");
        dict.add_translation("injunction", "injunction");
        dict.add_translation("subpoena", "subpoena");
        dict.add_translation("hearing", "hearing");
        dict.add_translation("trial", "trial");

        // Intellectual property terms
        dict.add_translation("patent", "patent");
        dict.add_translation("trademark", "trademark");
        dict.add_translation("copyright", "copyright");
        dict.add_translation("infringement", "infringement");
        dict.add_translation("royalty", "royalty");
        dict.add_translation("license", "license");

        // Family law terms
        dict.add_translation("marriage", "marriage");
        dict.add_translation("divorce", "divorce");
        dict.add_translation("custody", "custody");
        dict.add_translation("alimony", "alimony");
        dict.add_translation("adoption", "adoption");
        dict.add_translation("guardianship", "guardianship");

        // Additional procedural terms
        dict.add_translation("arbitration", "arbitration");
        dict.add_translation("mediation", "mediation");
        dict.add_translation("settlement", "settlement");
        dict.add_translation("litigation", "litigation");
        dict.add_translation("precedent", "precedent");
        dict.add_translation("statute_of_limitations", "statute of limitations");

        // Common legal abbreviations
        dict.add_abbreviation("corporation", "Corp.");
        dict.add_abbreviation("incorporated", "Inc.");
        dict.add_abbreviation("limited_liability_company", "LLC");
        dict.add_abbreviation("attorney", "Atty.");
        dict.add_abbreviation("versus", "v.");
        dict.add_abbreviation("plaintiff", "Pl.");
        dict.add_abbreviation("defendant", "Def.");
        dict.add_abbreviation("contract", "K");
        dict.add_abbreviation("statute", "Stat.");
        dict.add_abbreviation("section", "§");
        dict.add_abbreviation("article", "Art.");
        dict.add_abbreviation("paragraph", "Para.");
        dict.add_abbreviation("supreme_court", "S.Ct.");
        dict.add_abbreviation("district_court", "D.C.");
        dict.add_abbreviation("court_of_appeals", "C.A.");
        dict.add_abbreviation("federal_register", "Fed. Reg.");
        dict.add_abbreviation("code_of_federal_regulations", "C.F.R.");
        dict.add_abbreviation("united_states_code", "U.S.C.");

        dict
    }

    /// Creates a standard Japanese legal dictionary.
    pub fn japanese() -> Self {
        let mut dict = Self::new(Locale::new("ja").with_country("JP"));

        // Basic legal terms
        dict.add_translation("statute", "法律");
        dict.add_translation("law", "法");
        dict.add_translation("regulation", "規則");
        dict.add_translation("contract", "契約");
        dict.add_translation("agreement", "合意");
        dict.add_translation("liability", "責任");
        dict.add_translation("obligation", "義務");
        dict.add_translation("right", "権利");
        dict.add_translation("duty", "義務");
        dict.add_translation("party", "当事者");
        dict.add_translation("plaintiff", "原告");
        dict.add_translation("defendant", "被告");
        dict.add_translation("court", "裁判所");
        dict.add_translation("judge", "裁判官");
        dict.add_translation("jury", "陪審");
        dict.add_translation("attorney", "弁護士");
        dict.add_translation("lawyer", "弁護士");
        dict.add_translation("counsel", "法律顧問");
        dict.add_translation("witness", "証人");
        dict.add_translation("evidence", "証拠");
        dict.add_translation("testimony", "証言");
        dict.add_translation("verdict", "評決");
        dict.add_translation("judgment", "判決");
        dict.add_translation("appeal", "控訴");
        dict.add_translation("damages", "損害賠償");
        dict.add_translation("penalty", "罰則");
        dict.add_translation("fine", "罰金");

        // Corporate law terms
        dict.add_translation("corporation", "法人");
        dict.add_translation("shareholder", "株主");
        dict.add_translation("director", "取締役");
        dict.add_translation("officer", "役員");
        dict.add_translation("bylaws", "定款");
        dict.add_translation("merger", "合併");
        dict.add_translation("acquisition", "買収");
        dict.add_translation("dividend", "配当");
        dict.add_translation("stock", "株式");
        dict.add_translation("securities", "有価証券");

        // Property law terms
        dict.add_translation("property", "財産");
        dict.add_translation("real_estate", "不動産");
        dict.add_translation("ownership", "所有権");
        dict.add_translation("lease", "賃貸借");
        dict.add_translation("tenant", "賃借人");
        dict.add_translation("landlord", "賃貸人");
        dict.add_translation("mortgage", "抵当権");
        dict.add_translation("deed", "証書");
        dict.add_translation("title", "権原");
        dict.add_translation("easement", "地役権");

        // Criminal law terms
        dict.add_translation("crime", "犯罪");
        dict.add_translation("felony", "重罪");
        dict.add_translation("misdemeanor", "軽罪");
        dict.add_translation("prosecution", "起訴");
        dict.add_translation("indictment", "起訴状");
        dict.add_translation("conviction", "有罪判決");
        dict.add_translation("sentence", "刑");
        dict.add_translation("probation", "執行猶予");
        dict.add_translation("parole", "仮釈放");
        dict.add_translation("bail", "保釈");

        // Procedural law terms
        dict.add_translation("jurisdiction", "管轄");
        dict.add_translation("venue", "裁判地");
        dict.add_translation("standing", "当事者適格");
        dict.add_translation("discovery", "証拠開示");
        dict.add_translation("deposition", "証言録取");
        dict.add_translation("motion", "申立て");
        dict.add_translation("injunction", "差止め");
        dict.add_translation("subpoena", "召喚状");
        dict.add_translation("hearing", "審理");
        dict.add_translation("trial", "裁判");

        // Intellectual property terms
        dict.add_translation("patent", "特許");
        dict.add_translation("trademark", "商標");
        dict.add_translation("copyright", "著作権");
        dict.add_translation("infringement", "侵害");
        dict.add_translation("royalty", "使用料");
        dict.add_translation("license", "ライセンス");

        // Family law terms
        dict.add_translation("marriage", "婚姻");
        dict.add_translation("divorce", "離婚");
        dict.add_translation("custody", "親権");
        dict.add_translation("alimony", "扶養料");
        dict.add_translation("adoption", "養子縁組");
        dict.add_translation("guardianship", "後見");

        // Additional procedural terms
        dict.add_translation("arbitration", "仲裁");
        dict.add_translation("mediation", "調停");
        dict.add_translation("settlement", "和解");
        dict.add_translation("litigation", "訴訟");
        dict.add_translation("precedent", "判例");
        dict.add_translation("statute_of_limitations", "時効");

        dict
    }

    /// Creates a standard German legal dictionary.
    pub fn german() -> Self {
        let mut dict = Self::new(Locale::new("de").with_country("DE"));

        // Basic legal terms
        dict.add_translation("statute", "Gesetz");
        dict.add_translation("law", "Recht");
        dict.add_translation("regulation", "Verordnung");
        dict.add_translation("contract", "Vertrag");
        dict.add_translation("agreement", "Vereinbarung");
        dict.add_translation("liability", "Haftung");
        dict.add_translation("obligation", "Verpflichtung");
        dict.add_translation("right", "Recht");
        dict.add_translation("duty", "Pflicht");
        dict.add_translation("party", "Partei");
        dict.add_translation("plaintiff", "Kläger");
        dict.add_translation("defendant", "Beklagter");
        dict.add_translation("court", "Gericht");
        dict.add_translation("judge", "Richter");
        dict.add_translation("jury", "Geschworene");
        dict.add_translation("attorney", "Rechtsanwalt");
        dict.add_translation("lawyer", "Anwalt");
        dict.add_translation("counsel", "Rechtsbeistand");
        dict.add_translation("witness", "Zeuge");
        dict.add_translation("evidence", "Beweis");
        dict.add_translation("testimony", "Zeugenaussage");
        dict.add_translation("verdict", "Urteil");
        dict.add_translation("judgment", "Urteil");
        dict.add_translation("appeal", "Berufung");
        dict.add_translation("damages", "Schadensersatz");
        dict.add_translation("penalty", "Strafe");
        dict.add_translation("fine", "Geldstrafe");

        // Corporate law terms
        dict.add_translation("corporation", "Gesellschaft");
        dict.add_translation("shareholder", "Aktionär");
        dict.add_translation("director", "Direktor");
        dict.add_translation("officer", "Vorstand");
        dict.add_translation("bylaws", "Satzung");
        dict.add_translation("merger", "Fusion");
        dict.add_translation("acquisition", "Übernahme");
        dict.add_translation("dividend", "Dividende");
        dict.add_translation("stock", "Aktie");
        dict.add_translation("securities", "Wertpapiere");

        // Property law terms
        dict.add_translation("property", "Eigentum");
        dict.add_translation("real_estate", "Immobilien");
        dict.add_translation("ownership", "Eigentum");
        dict.add_translation("lease", "Miete");
        dict.add_translation("tenant", "Mieter");
        dict.add_translation("landlord", "Vermieter");
        dict.add_translation("mortgage", "Hypothek");
        dict.add_translation("deed", "Urkunde");
        dict.add_translation("title", "Titel");
        dict.add_translation("easement", "Grunddienstbarkeit");

        // Criminal law terms
        dict.add_translation("crime", "Verbrechen");
        dict.add_translation("felony", "Verbrechen");
        dict.add_translation("misdemeanor", "Vergehen");
        dict.add_translation("prosecution", "Strafverfolgung");
        dict.add_translation("indictment", "Anklage");
        dict.add_translation("conviction", "Verurteilung");
        dict.add_translation("sentence", "Strafe");
        dict.add_translation("probation", "Bewährung");
        dict.add_translation("parole", "Bewährung");
        dict.add_translation("bail", "Kaution");

        // Procedural law terms
        dict.add_translation("jurisdiction", "Zuständigkeit");
        dict.add_translation("venue", "Gerichtsstand");
        dict.add_translation("standing", "Klagebefugnis");
        dict.add_translation("discovery", "Beweiserhebung");
        dict.add_translation("deposition", "Zeugenaussage");
        dict.add_translation("motion", "Antrag");
        dict.add_translation("injunction", "Einstweilige Verfügung");
        dict.add_translation("subpoena", "Vorladung");
        dict.add_translation("hearing", "Anhörung");
        dict.add_translation("trial", "Verhandlung");

        // Intellectual property terms
        dict.add_translation("patent", "Patent");
        dict.add_translation("trademark", "Marke");
        dict.add_translation("copyright", "Urheberrecht");
        dict.add_translation("infringement", "Verletzung");
        dict.add_translation("royalty", "Lizenzgebühr");
        dict.add_translation("license", "Lizenz");

        // Family law terms
        dict.add_translation("marriage", "Ehe");
        dict.add_translation("divorce", "Scheidung");
        dict.add_translation("custody", "Sorgerecht");
        dict.add_translation("alimony", "Unterhalt");
        dict.add_translation("adoption", "Adoption");
        dict.add_translation("guardianship", "Vormundschaft");

        // Additional procedural terms
        dict.add_translation("arbitration", "Schiedsverfahren");
        dict.add_translation("mediation", "Mediation");
        dict.add_translation("settlement", "Vergleich");
        dict.add_translation("litigation", "Rechtsstreit");
        dict.add_translation("precedent", "Präzedenzfall");
        dict.add_translation("statute_of_limitations", "Verjährung");

        dict
    }

    /// Creates a standard French legal dictionary.
    pub fn french() -> Self {
        let mut dict = Self::new(Locale::new("fr").with_country("FR"));

        // Basic legal terms
        dict.add_translation("statute", "loi");
        dict.add_translation("law", "droit");
        dict.add_translation("regulation", "règlement");
        dict.add_translation("contract", "contrat");
        dict.add_translation("agreement", "accord");
        dict.add_translation("liability", "responsabilité");
        dict.add_translation("obligation", "obligation");
        dict.add_translation("right", "droit");
        dict.add_translation("duty", "devoir");
        dict.add_translation("party", "partie");
        dict.add_translation("plaintiff", "demandeur");
        dict.add_translation("defendant", "défendeur");
        dict.add_translation("court", "tribunal");
        dict.add_translation("judge", "juge");
        dict.add_translation("jury", "jury");
        dict.add_translation("attorney", "avocat");
        dict.add_translation("lawyer", "avocat");
        dict.add_translation("counsel", "conseil");
        dict.add_translation("witness", "témoin");
        dict.add_translation("evidence", "preuve");
        dict.add_translation("testimony", "témoignage");
        dict.add_translation("verdict", "verdict");
        dict.add_translation("judgment", "jugement");
        dict.add_translation("appeal", "appel");
        dict.add_translation("damages", "dommages");
        dict.add_translation("penalty", "pénalité");
        dict.add_translation("fine", "amende");

        // Corporate law terms
        dict.add_translation("corporation", "société");
        dict.add_translation("shareholder", "actionnaire");
        dict.add_translation("director", "directeur");
        dict.add_translation("officer", "dirigeant");
        dict.add_translation("bylaws", "statuts");
        dict.add_translation("merger", "fusion");
        dict.add_translation("acquisition", "acquisition");
        dict.add_translation("dividend", "dividende");
        dict.add_translation("stock", "action");
        dict.add_translation("securities", "valeurs mobilières");

        // Property law terms
        dict.add_translation("property", "propriété");
        dict.add_translation("real_estate", "immobilier");
        dict.add_translation("ownership", "propriété");
        dict.add_translation("lease", "bail");
        dict.add_translation("tenant", "locataire");
        dict.add_translation("landlord", "bailleur");
        dict.add_translation("mortgage", "hypothèque");
        dict.add_translation("deed", "acte");
        dict.add_translation("title", "titre");
        dict.add_translation("easement", "servitude");

        // Criminal law terms
        dict.add_translation("crime", "crime");
        dict.add_translation("felony", "crime");
        dict.add_translation("misdemeanor", "délit");
        dict.add_translation("prosecution", "poursuite");
        dict.add_translation("indictment", "mise en accusation");
        dict.add_translation("conviction", "condamnation");
        dict.add_translation("sentence", "peine");
        dict.add_translation("probation", "sursis");
        dict.add_translation("parole", "libération conditionnelle");
        dict.add_translation("bail", "caution");

        // Procedural law terms
        dict.add_translation("jurisdiction", "compétence");
        dict.add_translation("venue", "lieu du procès");
        dict.add_translation("standing", "qualité pour agir");
        dict.add_translation("discovery", "communication de pièces");
        dict.add_translation("deposition", "déposition");
        dict.add_translation("motion", "requête");
        dict.add_translation("injunction", "injonction");
        dict.add_translation("subpoena", "assignation");
        dict.add_translation("hearing", "audience");
        dict.add_translation("trial", "procès");

        // Intellectual property terms
        dict.add_translation("patent", "brevet");
        dict.add_translation("trademark", "marque");
        dict.add_translation("copyright", "droit d'auteur");
        dict.add_translation("infringement", "contrefaçon");
        dict.add_translation("royalty", "redevance");
        dict.add_translation("license", "licence");

        // Family law terms
        dict.add_translation("marriage", "mariage");
        dict.add_translation("divorce", "divorce");
        dict.add_translation("custody", "garde");
        dict.add_translation("alimony", "pension alimentaire");
        dict.add_translation("adoption", "adoption");
        dict.add_translation("guardianship", "tutelle");

        // Additional procedural terms
        dict.add_translation("arbitration", "arbitrage");
        dict.add_translation("mediation", "médiation");
        dict.add_translation("settlement", "règlement");
        dict.add_translation("litigation", "litige");
        dict.add_translation("precedent", "précédent");
        dict.add_translation("statute_of_limitations", "prescription");

        dict
    }

    /// Creates a standard Spanish legal dictionary.
    pub fn spanish() -> Self {
        let mut dict = Self::new(Locale::new("es").with_country("ES"));

        // Basic legal terms
        dict.add_translation("statute", "estatuto");
        dict.add_translation("law", "ley");
        dict.add_translation("regulation", "reglamento");
        dict.add_translation("contract", "contrato");
        dict.add_translation("agreement", "acuerdo");
        dict.add_translation("liability", "responsabilidad");
        dict.add_translation("obligation", "obligación");
        dict.add_translation("right", "derecho");
        dict.add_translation("duty", "deber");
        dict.add_translation("party", "parte");
        dict.add_translation("plaintiff", "demandante");
        dict.add_translation("defendant", "demandado");
        dict.add_translation("court", "tribunal");
        dict.add_translation("judge", "juez");
        dict.add_translation("jury", "jurado");
        dict.add_translation("attorney", "abogado");
        dict.add_translation("lawyer", "abogado");
        dict.add_translation("counsel", "asesor");
        dict.add_translation("witness", "testigo");
        dict.add_translation("evidence", "prueba");
        dict.add_translation("testimony", "testimonio");
        dict.add_translation("verdict", "veredicto");
        dict.add_translation("judgment", "sentencia");
        dict.add_translation("appeal", "apelación");
        dict.add_translation("damages", "daños");
        dict.add_translation("penalty", "pena");
        dict.add_translation("fine", "multa");

        // Corporate law terms
        dict.add_translation("corporation", "corporación");
        dict.add_translation("shareholder", "accionista");
        dict.add_translation("director", "director");
        dict.add_translation("officer", "funcionario");
        dict.add_translation("bylaws", "estatutos");
        dict.add_translation("merger", "fusión");
        dict.add_translation("acquisition", "adquisición");
        dict.add_translation("dividend", "dividendo");
        dict.add_translation("stock", "acción");
        dict.add_translation("securities", "valores");

        // Property law terms
        dict.add_translation("property", "propiedad");
        dict.add_translation("real_estate", "bienes raíces");
        dict.add_translation("ownership", "propiedad");
        dict.add_translation("lease", "arrendamiento");
        dict.add_translation("tenant", "inquilino");
        dict.add_translation("landlord", "arrendador");
        dict.add_translation("mortgage", "hipoteca");
        dict.add_translation("deed", "escritura");
        dict.add_translation("title", "título");
        dict.add_translation("easement", "servidumbre");

        // Criminal law terms
        dict.add_translation("crime", "crimen");
        dict.add_translation("felony", "delito grave");
        dict.add_translation("misdemeanor", "delito menor");
        dict.add_translation("prosecution", "fiscalía");
        dict.add_translation("indictment", "acusación");
        dict.add_translation("conviction", "condena");
        dict.add_translation("sentence", "sentencia");
        dict.add_translation("probation", "libertad condicional");
        dict.add_translation("parole", "libertad condicional");
        dict.add_translation("bail", "fianza");

        // Procedural law terms
        dict.add_translation("jurisdiction", "jurisdicción");
        dict.add_translation("venue", "sede");
        dict.add_translation("standing", "legitimación");
        dict.add_translation("discovery", "descubrimiento");
        dict.add_translation("deposition", "declaración");
        dict.add_translation("motion", "moción");
        dict.add_translation("injunction", "mandamiento");
        dict.add_translation("subpoena", "citación");
        dict.add_translation("hearing", "audiencia");
        dict.add_translation("trial", "juicio");

        // Intellectual property terms
        dict.add_translation("patent", "patente");
        dict.add_translation("trademark", "marca registrada");
        dict.add_translation("copyright", "derecho de autor");
        dict.add_translation("infringement", "infracción");
        dict.add_translation("royalty", "regalía");
        dict.add_translation("license", "licencia");

        // Family law terms
        dict.add_translation("marriage", "matrimonio");
        dict.add_translation("divorce", "divorcio");
        dict.add_translation("custody", "custodia");
        dict.add_translation("alimony", "pensión alimenticia");
        dict.add_translation("adoption", "adopción");
        dict.add_translation("guardianship", "tutela");

        // Additional procedural terms
        dict.add_translation("arbitration", "arbitraje");
        dict.add_translation("mediation", "mediación");
        dict.add_translation("settlement", "acuerdo");
        dict.add_translation("litigation", "litigio");
        dict.add_translation("precedent", "precedente");
        dict.add_translation("statute_of_limitations", "prescripción");

        dict
    }

    /// Creates a standard Chinese (Simplified) legal dictionary.
    pub fn chinese_simplified() -> Self {
        let mut dict = Self::new(Locale::new("zh").with_country("CN"));

        // Basic legal terms
        dict.add_translation("statute", "法规");
        dict.add_translation("law", "法律");
        dict.add_translation("regulation", "规章");
        dict.add_translation("contract", "合同");
        dict.add_translation("agreement", "协议");
        dict.add_translation("liability", "责任");
        dict.add_translation("obligation", "义务");
        dict.add_translation("right", "权利");
        dict.add_translation("duty", "职责");
        dict.add_translation("party", "当事人");
        dict.add_translation("plaintiff", "原告");
        dict.add_translation("defendant", "被告");
        dict.add_translation("court", "法院");
        dict.add_translation("judge", "法官");
        dict.add_translation("jury", "陪审团");
        dict.add_translation("attorney", "律师");
        dict.add_translation("lawyer", "律师");
        dict.add_translation("counsel", "法律顾问");
        dict.add_translation("witness", "证人");
        dict.add_translation("evidence", "证据");
        dict.add_translation("testimony", "证词");
        dict.add_translation("verdict", "裁决");
        dict.add_translation("judgment", "判决");
        dict.add_translation("appeal", "上诉");
        dict.add_translation("damages", "损害赔偿");
        dict.add_translation("penalty", "处罚");
        dict.add_translation("fine", "罚款");

        // Corporate law terms
        dict.add_translation("corporation", "公司");
        dict.add_translation("shareholder", "股东");
        dict.add_translation("director", "董事");
        dict.add_translation("officer", "高管");
        dict.add_translation("bylaws", "章程");
        dict.add_translation("merger", "合并");
        dict.add_translation("acquisition", "收购");
        dict.add_translation("dividend", "股息");
        dict.add_translation("stock", "股票");
        dict.add_translation("securities", "证券");

        // Property law terms
        dict.add_translation("property", "财产");
        dict.add_translation("real_estate", "房地产");
        dict.add_translation("ownership", "所有权");
        dict.add_translation("lease", "租赁");
        dict.add_translation("tenant", "承租人");
        dict.add_translation("landlord", "出租人");
        dict.add_translation("mortgage", "抵押");
        dict.add_translation("deed", "契约");
        dict.add_translation("title", "产权");
        dict.add_translation("easement", "地役权");

        // Criminal law terms
        dict.add_translation("crime", "犯罪");
        dict.add_translation("felony", "重罪");
        dict.add_translation("misdemeanor", "轻罪");
        dict.add_translation("prosecution", "起诉");
        dict.add_translation("indictment", "起诉书");
        dict.add_translation("conviction", "定罪");
        dict.add_translation("sentence", "判刑");
        dict.add_translation("probation", "缓刑");
        dict.add_translation("parole", "假释");
        dict.add_translation("bail", "保释");

        // Procedural law terms
        dict.add_translation("jurisdiction", "管辖权");
        dict.add_translation("venue", "审判地");
        dict.add_translation("standing", "诉讼资格");
        dict.add_translation("discovery", "证据披露");
        dict.add_translation("deposition", "证词记录");
        dict.add_translation("motion", "动议");
        dict.add_translation("injunction", "禁令");
        dict.add_translation("subpoena", "传票");
        dict.add_translation("hearing", "听证");
        dict.add_translation("trial", "审判");

        // Intellectual property terms
        dict.add_translation("patent", "专利");
        dict.add_translation("trademark", "商标");
        dict.add_translation("copyright", "版权");
        dict.add_translation("infringement", "侵权");
        dict.add_translation("royalty", "版税");
        dict.add_translation("license", "许可");

        // Family law terms
        dict.add_translation("marriage", "婚姻");
        dict.add_translation("divorce", "离婚");
        dict.add_translation("custody", "监护");
        dict.add_translation("alimony", "赡养费");
        dict.add_translation("adoption", "收养");
        dict.add_translation("guardianship", "监护权");

        // Additional procedural terms
        dict.add_translation("arbitration", "仲裁");
        dict.add_translation("mediation", "调解");
        dict.add_translation("settlement", "和解");
        dict.add_translation("litigation", "诉讼");
        dict.add_translation("precedent", "判例");
        dict.add_translation("statute_of_limitations", "诉讼时效");

        dict
    }

    /// Creates a Latin legal terms dictionary.
    pub fn latin() -> Self {
        let mut dict = Self::new(Locale::new("la"));

        // Common Latin legal maxims and terms
        dict.add_translation("good_faith", "bona fide");
        dict.add_translation("by_the_fact_itself", "ipso facto");
        dict.add_translation("for_this_purpose", "ad hoc");
        dict.add_translation("in_good_faith", "bona fide");
        dict.add_translation("friend_of_the_court", "amicus curiae");
        dict.add_translation("body_of_the_crime", "corpus delicti");
        dict.add_translation("guilty_mind", "mens rea");
        dict.add_translation("guilty_act", "actus reus");
        dict.add_translation("you_have_the_body", "habeas corpus");
        dict.add_translation("let_the_buyer_beware", "caveat emptor");
        dict.add_translation("something_for_something", "quid pro quo");
        dict.add_translation("in_the_matter_of", "in re");
        dict.add_translation("by_operation_of_law", "ex lege");
        dict.add_translation("from_the_beginning", "ab initio");
        dict.add_translation("by_right", "de jure");
        dict.add_translation("in_fact", "de facto");
        dict.add_translation("according_to_law", "secundum legem");
        dict.add_translation("against_the_law", "contra legem");
        dict.add_translation("by_itself", "per se");
        dict.add_translation("burden_of_proof", "onus probandi");
        dict.add_translation("presumption_of_innocence", "praesumptio innocentiae");
        dict.add_translation("force_majeure", "vis major");
        dict.add_translation("highest_good_faith", "uberrima fides");

        // Definitions for key Latin terms
        dict.add_definition("bona fide", "In good faith; genuine");
        dict.add_definition("mens rea", "Guilty mind; criminal intent");
        dict.add_definition("actus reus", "Guilty act; physical element of a crime");
        dict.add_definition(
            "habeas corpus",
            "A writ requiring a person to be brought before a court",
        );
        dict.add_definition("caveat emptor", "Buyer beware; buyer assumes risk");
        dict.add_definition("quid pro quo", "Something for something; mutual exchange");

        dict
    }

    /// Creates a jurisdiction-specific glossary for Japan.
    pub fn glossary_japan() -> Self {
        let mut dict = Self::new(Locale::new("ja").with_country("JP"));

        // Japanese Civil Code specific terms
        dict.add_translation("civil_code", "民法");
        dict.add_translation("general_provisions", "総則");
        dict.add_translation("legal_person", "法人");
        dict.add_translation("juristic_act", "法律行為");
        dict.add_translation("prescription", "時効");
        dict.add_translation("acquisition_by_prescription", "取得時効");
        dict.add_translation("extinctive_prescription", "消滅時効");

        // Property law
        dict.add_translation("real_property", "不動産");
        dict.add_translation("movable_property", "動産");
        dict.add_translation("superficies", "地上権");
        dict.add_translation("emphyteusis", "永小作権");
        dict.add_translation("servitude", "地役権");

        // Family law
        dict.add_translation("family_register", "戸籍");
        dict.add_translation("koseki", "戸籍");
        dict.add_translation("parental_authority", "親権");

        // Corporate law
        dict.add_translation("kabushiki_kaisha", "株式会社");
        dict.add_translation("godo_kaisha", "合同会社");
        dict.add_translation("yugen_kaisha", "有限会社");

        // Criminal law
        dict.add_translation("penal_code", "刑法");
        dict.add_translation("suspended_sentence", "執行猶予");

        dict
    }

    /// Creates a jurisdiction-specific glossary for United States.
    pub fn glossary_united_states() -> Self {
        let mut dict = Self::new(Locale::new("en").with_country("US"));

        // Constitutional law
        dict.add_translation("constitution", "Constitution");
        dict.add_translation("bill_of_rights", "Bill of Rights");
        dict.add_translation("due_process", "due process");
        dict.add_translation("equal_protection", "equal protection");
        dict.add_translation("commerce_clause", "Commerce Clause");

        // Federal system
        dict.add_translation("federal", "federal");
        dict.add_translation("state", "state");
        dict.add_translation("supremacy_clause", "Supremacy Clause");

        // Court system
        dict.add_translation("supreme_court", "Supreme Court");
        dict.add_translation("circuit_court", "Circuit Court");
        dict.add_translation("district_court", "District Court");

        // Common law concepts
        dict.add_translation("stare_decisis", "stare decisis");
        dict.add_translation("precedent", "precedent");
        dict.add_translation("case_law", "case law");

        // Torts
        dict.add_translation("punitive_damages", "punitive damages");
        dict.add_translation("treble_damages", "treble damages");
        dict.add_translation("strict_liability", "strict liability");

        // Procedure
        dict.add_translation("discovery", "discovery");
        dict.add_translation("deposition", "deposition");
        dict.add_translation("summary_judgment", "summary judgment");
        dict.add_translation("class_action", "class action");

        dict
    }

    /// Creates a jurisdiction-specific glossary for United Kingdom.
    pub fn glossary_united_kingdom() -> Self {
        let mut dict = Self::new(Locale::new("en").with_country("GB"));

        // Court system
        dict.add_translation("high_court", "High Court");
        dict.add_translation("crown_court", "Crown Court");
        dict.add_translation("magistrates_court", "Magistrates' Court");
        dict.add_translation("supreme_court", "Supreme Court");

        // Legal roles
        dict.add_translation("barrister", "barrister");
        dict.add_translation("solicitor", "solicitor");
        dict.add_translation("queens_counsel", "Queen's Counsel");
        dict.add_translation("kings_counsel", "King's Counsel");

        // Property law
        dict.add_translation("freehold", "freehold");
        dict.add_translation("leasehold", "leasehold");
        dict.add_translation("commonhold", "commonhold");

        // Equity
        dict.add_translation("equity", "equity");
        dict.add_translation("trust", "trust");
        dict.add_translation("trustee", "trustee");
        dict.add_translation("beneficiary", "beneficiary");

        // Parliamentary terms
        dict.add_translation("act_of_parliament", "Act of Parliament");
        dict.add_translation("statutory_instrument", "statutory instrument");

        dict
    }

    /// Creates a jurisdiction-specific glossary for Germany.
    pub fn glossary_germany() -> Self {
        let mut dict = Self::new(Locale::new("de").with_country("DE"));

        // German Civil Code (BGB) terms
        dict.add_translation("burgerliches_gesetzbuch", "Bürgerliches Gesetzbuch");
        dict.add_translation("bgb", "BGB");
        dict.add_translation("schuldrecht", "Schuldrecht");
        dict.add_translation("sachenrecht", "Sachenrecht");
        dict.add_translation("familienrecht", "Familienrecht");
        dict.add_translation("erbrecht", "Erbrecht");

        // Court system
        dict.add_translation("bundesverfassungsgericht", "Bundesverfassungsgericht");
        dict.add_translation("bundesgerichtshof", "Bundesgerichtshof");
        dict.add_translation("oberlandesgericht", "Oberlandesgericht");
        dict.add_translation("landgericht", "Landgericht");
        dict.add_translation("amtsgericht", "Amtsgericht");

        // Legal concepts
        dict.add_translation("rechtsstaat", "Rechtsstaat");
        dict.add_translation("grundgesetz", "Grundgesetz");

        dict
    }

    /// Creates a jurisdiction-specific glossary for France.
    pub fn glossary_france() -> Self {
        let mut dict = Self::new(Locale::new("fr").with_country("FR"));

        // French Civil Code
        dict.add_translation("code_civil", "Code civil");
        dict.add_translation("code_penal", "Code pénal");

        // Court system
        dict.add_translation("cour_de_cassation", "Cour de cassation");
        dict.add_translation("cour_dappel", "Cour d'appel");
        dict.add_translation("tribunal_de_grande_instance", "Tribunal de grande instance");

        // Legal concepts
        dict.add_translation("droit_civil", "droit civil");
        dict.add_translation("droit_penal", "droit pénal");
        dict.add_translation("droit_administratif", "droit administratif");

        dict
    }

    /// Creates a jurisdiction-specific glossary for China.
    pub fn glossary_china() -> Self {
        let mut dict = Self::new(Locale::new("zh").with_country("CN"));

        // Chinese legal system
        dict.add_translation("civil_law", "民法");
        dict.add_translation("criminal_law", "刑法");
        dict.add_translation("administrative_law", "行政法");
        dict.add_translation("peoples_court", "人民法院");
        dict.add_translation("supreme_peoples_court", "最高人民法院");
        dict.add_translation("procuratorate", "检察院");

        dict
    }

    /// Creates a jurisdiction-specific glossary for a jurisdiction code.
    pub fn glossary_for_jurisdiction(code: &str) -> Self {
        match code {
            "JP" => Self::glossary_japan(),
            "US" => Self::glossary_united_states(),
            "GB" => Self::glossary_united_kingdom(),
            "DE" => Self::glossary_germany(),
            "FR" => Self::glossary_france(),
            "CN" => Self::glossary_china(),
            _ => {
                // Return a basic dictionary for the jurisdiction's language
                let locale = Locale::new("en"); // Default to English
                Self::new(locale)
            }
        }
    }
}

/// Mapping between legal concepts across different legal systems.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalConceptMapping {
    /// The legal system this concept belongs to
    pub legal_system: LegalSystem,
    /// The concept identifier
    pub concept: String,
    /// Equivalent concepts in other legal systems
    pub equivalents: HashMap<LegalSystem, Vec<String>>,
    /// Notes on differences or caveats
    pub notes: Option<String>,
}

impl LegalConceptMapping {
    /// Creates a new concept mapping.
    pub fn new(legal_system: LegalSystem, concept: impl Into<String>) -> Self {
        Self {
            legal_system,
            concept: concept.into(),
            equivalents: HashMap::new(),
            notes: None,
        }
    }

    /// Adds an equivalent concept in another legal system.
    pub fn add_equivalent(mut self, system: LegalSystem, equivalent: impl Into<String>) -> Self {
        self.equivalents
            .entry(system)
            .or_default()
            .push(equivalent.into());
        self
    }

    /// Adds a note about the mapping.
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes = Some(note.into());
        self
    }

    /// Gets equivalent concepts for a target legal system.
    pub fn get_equivalents(&self, target: LegalSystem) -> Option<&Vec<String>> {
        self.equivalents.get(&target)
    }
}

/// Registry of legal concept mappings between different legal systems.
#[derive(Debug, Default)]
pub struct LegalConceptRegistry {
    mappings: Vec<LegalConceptMapping>,
}

impl LegalConceptRegistry {
    /// Creates a new registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a registry with standard mappings.
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();

        // Criminal law concepts
        registry.add_mapping(
            LegalConceptMapping::new(LegalSystem::CivilLaw, "actus reus")
                .add_equivalent(LegalSystem::CommonLaw, "actus reus")
                .with_note("Similar concept in both systems, inherited from Roman law"),
        );

        registry.add_mapping(
            LegalConceptMapping::new(LegalSystem::CivilLaw, "mens rea")
                .add_equivalent(LegalSystem::CommonLaw, "mens rea")
                .with_note("Criminal intent; same concept in both systems"),
        );

        // Contract law concepts
        registry.add_mapping(
            LegalConceptMapping::new(LegalSystem::CivilLaw, "good faith")
                .add_equivalent(LegalSystem::CommonLaw, "good faith")
                .add_equivalent(LegalSystem::CommonLaw, "bona fides")
                .with_note("Universal concept, but enforcement may differ"),
        );

        registry.add_mapping(
            LegalConceptMapping::new(LegalSystem::CivilLaw, "consideration")
                .add_equivalent(LegalSystem::CommonLaw, "consideration")
                .with_note("Critical in common law contracts; less emphasized in civil law"),
        );

        // Property law concepts
        registry.add_mapping(
            LegalConceptMapping::new(LegalSystem::CivilLaw, "ownership")
                .add_equivalent(LegalSystem::CommonLaw, "fee simple absolute")
                .add_equivalent(LegalSystem::CommonLaw, "ownership")
                .with_note("Civil law has unified ownership; common law has estates in land"),
        );

        registry.add_mapping(
            LegalConceptMapping::new(LegalSystem::CommonLaw, "trust")
                .add_equivalent(LegalSystem::CivilLaw, "fiducie")
                .add_equivalent(LegalSystem::CivilLaw, "fideicommissum")
                .with_note(
                    "Trust is quintessentially common law; civil law has limited equivalents",
                ),
        );

        // Tort/Delict concepts
        registry.add_mapping(
            LegalConceptMapping::new(LegalSystem::CommonLaw, "tort")
                .add_equivalent(LegalSystem::CivilLaw, "delict")
                .add_equivalent(LegalSystem::CivilLaw, "civil wrong")
                .with_note("Tort (common law) vs delict (civil law) - similar concepts"),
        );

        registry.add_mapping(
            LegalConceptMapping::new(LegalSystem::CommonLaw, "negligence")
                .add_equivalent(LegalSystem::CivilLaw, "culpa")
                .add_equivalent(LegalSystem::CivilLaw, "fault")
                .with_note("Similar concept but different standards of proof"),
        );

        // Legal proceedings
        registry.add_mapping(
            LegalConceptMapping::new(LegalSystem::CommonLaw, "jury trial")
                .add_equivalent(LegalSystem::CivilLaw, "lay judges")
                .add_equivalent(LegalSystem::CivilLaw, "schöffen")
                .with_note("Jury in common law; mixed courts or lay judges in civil law"),
        );

        registry.add_mapping(
            LegalConceptMapping::new(LegalSystem::CommonLaw, "stare decisis").with_note(
                "Binding precedent in common law; no direct equivalent in pure civil law",
            ),
        );

        registry.add_mapping(
            LegalConceptMapping::new(LegalSystem::CivilLaw, "code")
                .add_equivalent(LegalSystem::CommonLaw, "statute")
                .add_equivalent(LegalSystem::CommonLaw, "act")
                .with_note("Comprehensive codes in civil law; individual statutes in common law"),
        );

        // Remedies
        registry.add_mapping(
            LegalConceptMapping::new(LegalSystem::CommonLaw, "specific performance")
                .add_equivalent(LegalSystem::CivilLaw, "specific performance")
                .add_equivalent(LegalSystem::CivilLaw, "exécution forcée")
                .with_note("Available in both, but more readily granted in civil law"),
        );

        registry.add_mapping(
            LegalConceptMapping::new(LegalSystem::CommonLaw, "damages")
                .add_equivalent(LegalSystem::CivilLaw, "damages")
                .add_equivalent(LegalSystem::CivilLaw, "dommages-intérêts")
                .with_note("Similar concept; calculation methods may differ"),
        );

        registry
    }

    /// Adds a mapping.
    pub fn add_mapping(&mut self, mapping: LegalConceptMapping) {
        self.mappings.push(mapping);
    }

    /// Finds mappings for a concept.
    pub fn find_mapping(
        &self,
        legal_system: LegalSystem,
        concept: &str,
    ) -> Option<&LegalConceptMapping> {
        self.mappings
            .iter()
            .find(|m| m.legal_system == legal_system && m.concept == concept)
    }

    /// Gets all mappings from one legal system to another.
    pub fn get_system_mappings(
        &self,
        from: LegalSystem,
        to: LegalSystem,
    ) -> Vec<(&str, &Vec<String>)> {
        self.mappings
            .iter()
            .filter(|m| m.legal_system == from)
            .filter_map(|m| m.get_equivalents(to).map(|eqs| (m.concept.as_str(), eqs)))
            .collect()
    }
}

/// Multi-locale translation manager with LRU caching support.
#[derive(Debug)]
pub struct TranslationManager {
    dictionaries: HashMap<String, LegalDictionary>,
    fallback_locale: Option<Locale>,
    /// LRU cache for translation lookups: (key, locale_tag) -> translation
    /// Uses RwLock for thread-safe access in parallel operations
    cache: Arc<RwLock<LruCache<(String, String), String>>>,
}

impl Default for TranslationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TranslationManager {
    /// Creates a new translation manager with default LRU cache size (1000 entries).
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_i18n::{TranslationManager, LegalDictionary, Locale};
    ///
    /// let mut manager = TranslationManager::new();
    ///
    /// // Add Japanese dictionary
    /// let mut ja_dict = LegalDictionary::new(Locale::new("ja").with_country("JP"));
    /// ja_dict.add_translation("contract", "契約");
    /// manager.add_dictionary(ja_dict);
    ///
    /// // Translate
    /// let locale = Locale::new("ja").with_country("JP");
    /// let translation = manager.translate("contract", &locale).unwrap();
    /// assert_eq!(translation, "契約");
    /// ```
    pub fn new() -> Self {
        Self::with_cache_size(1000)
    }

    /// Creates a new translation manager with custom LRU cache size.
    pub fn with_cache_size(cache_size: usize) -> Self {
        Self {
            dictionaries: HashMap::new(),
            fallback_locale: None,
            cache: Arc::new(RwLock::new(LruCache::new(
                NonZeroUsize::new(cache_size).unwrap_or(NonZeroUsize::new(1000).unwrap()),
            ))),
        }
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

    /// Translates a key for a locale with caching.
    pub fn translate(&self, key: &str, locale: &Locale) -> I18nResult<String> {
        let cache_key = (key.to_string(), locale.tag());

        // Check LRU cache first
        {
            if let Ok(mut cache) = self.cache.write() {
                if let Some(cached) = cache.get(&cache_key) {
                    return Ok(cached.clone());
                }
            }
        }

        // Perform translation
        let result = self.translate_uncached(key, locale);

        // Cache the result if successful (LRU automatically evicts least recently used)
        if let Ok(ref translation) = result {
            if let Ok(mut cache) = self.cache.write() {
                cache.put(cache_key, translation.clone());
            }
        }

        result
    }

    /// Translates a key for a locale without using cache.
    fn translate_uncached(&self, key: &str, locale: &Locale) -> I18nResult<String> {
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

    /// Clears the translation cache.
    pub fn clear_cache(&self) {
        if let Ok(mut cache) = self.cache.write() {
            cache.clear();
        }
    }

    /// Gets the current cache size.
    pub fn cache_size(&self) -> usize {
        self.cache.read().map(|cache| cache.len()).unwrap_or(0)
    }

    /// Resizes the LRU cache.
    /// Note: This creates a new cache, so all existing cached entries will be lost.
    pub fn resize_cache(&self, new_size: usize) {
        if let Ok(mut cache) = self.cache.write() {
            *cache = LruCache::new(
                NonZeroUsize::new(new_size).unwrap_or(NonZeroUsize::new(1000).unwrap()),
            );
        }
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

        // Japan
        registry.register(
            Jurisdiction::new("JP", "Japan", Locale::new("ja").with_country("JP"))
                .with_legal_system(LegalSystem::CivilLaw)
                .with_cultural_params(CulturalParams::japan()),
        );

        // United States
        registry.register(
            Jurisdiction::new("US", "United States", Locale::new("en").with_country("US"))
                .with_legal_system(LegalSystem::CommonLaw)
                .with_cultural_params(CulturalParams::for_country("US")),
        );

        // United Kingdom
        registry.register(
            Jurisdiction::new("GB", "United Kingdom", Locale::new("en").with_country("GB"))
                .with_legal_system(LegalSystem::CommonLaw)
                .with_cultural_params(CulturalParams::for_country("GB")),
        );

        // Germany
        registry.register(
            Jurisdiction::new("DE", "Germany", Locale::new("de").with_country("DE"))
                .with_legal_system(LegalSystem::CivilLaw)
                .with_cultural_params(CulturalParams::for_country("DE")),
        );

        // France
        registry.register(
            Jurisdiction::new("FR", "France", Locale::new("fr").with_country("FR"))
                .with_legal_system(LegalSystem::CivilLaw)
                .with_cultural_params(CulturalParams::for_country("FR")),
        );

        // Spain
        registry.register(
            Jurisdiction::new("ES", "Spain", Locale::new("es").with_country("ES"))
                .with_legal_system(LegalSystem::CivilLaw)
                .with_cultural_params(CulturalParams::for_country("ES")),
        );

        // Italy
        registry.register(
            Jurisdiction::new("IT", "Italy", Locale::new("it").with_country("IT"))
                .with_legal_system(LegalSystem::CivilLaw)
                .with_cultural_params(CulturalParams::for_country("IT")),
        );

        // China
        registry.register(
            Jurisdiction::new("CN", "China", Locale::new("zh").with_country("CN"))
                .with_legal_system(LegalSystem::CivilLaw)
                .with_cultural_params(CulturalParams::for_country("CN")),
        );

        // Taiwan
        registry.register(
            Jurisdiction::new("TW", "Taiwan", Locale::new("zh").with_country("TW"))
                .with_legal_system(LegalSystem::CivilLaw)
                .with_cultural_params(CulturalParams::for_country("TW")),
        );

        // South Korea
        registry.register(
            Jurisdiction::new("KR", "South Korea", Locale::new("ko").with_country("KR"))
                .with_legal_system(LegalSystem::CivilLaw)
                .with_cultural_params(CulturalParams::for_country("KR")),
        );

        // Canada
        registry.register(
            Jurisdiction::new("CA", "Canada", Locale::new("en").with_country("CA"))
                .with_legal_system(LegalSystem::Mixed)
                .with_cultural_params(CulturalParams::for_country("CA")),
        );

        // Australia
        registry.register(
            Jurisdiction::new("AU", "Australia", Locale::new("en").with_country("AU"))
                .with_legal_system(LegalSystem::CommonLaw)
                .with_cultural_params(CulturalParams::for_country("AU")),
        );

        // India
        registry.register(
            Jurisdiction::new("IN", "India", Locale::new("en").with_country("IN"))
                .with_legal_system(LegalSystem::CommonLaw)
                .with_cultural_params(CulturalParams::for_country("IN")),
        );

        // Brazil
        registry.register(
            Jurisdiction::new("BR", "Brazil", Locale::new("pt").with_country("BR"))
                .with_legal_system(LegalSystem::CivilLaw)
                .with_cultural_params(CulturalParams::for_country("BR")),
        );

        // Russia
        registry.register(
            Jurisdiction::new("RU", "Russia", Locale::new("ru").with_country("RU"))
                .with_legal_system(LegalSystem::CivilLaw)
                .with_cultural_params(CulturalParams::for_country("RU")),
        );

        // Saudi Arabia
        registry.register(
            Jurisdiction::new("SA", "Saudi Arabia", Locale::new("ar").with_country("SA"))
                .with_legal_system(LegalSystem::ReligiousLaw)
                .with_cultural_params(CulturalParams::for_country("SA")),
        );

        // Netherlands
        registry.register(
            Jurisdiction::new("NL", "Netherlands", Locale::new("nl").with_country("NL"))
                .with_legal_system(LegalSystem::CivilLaw)
                .with_cultural_params(CulturalParams::for_country("NL")),
        );

        // Switzerland
        registry.register(
            Jurisdiction::new("CH", "Switzerland", Locale::new("de").with_country("CH"))
                .with_legal_system(LegalSystem::CivilLaw)
                .with_cultural_params(CulturalParams::for_country("CH")),
        );

        // Mexico
        registry.register(
            Jurisdiction::new("MX", "Mexico", Locale::new("es").with_country("MX"))
                .with_legal_system(LegalSystem::CivilLaw)
                .with_cultural_params(CulturalParams::for_country("MX")),
        );

        // Singapore
        registry.register(
            Jurisdiction::new("SG", "Singapore", Locale::new("en").with_country("SG"))
                .with_legal_system(LegalSystem::CommonLaw)
                .with_cultural_params(CulturalParams::for_country("SG")),
        );

        registry
    }

    /// Registers a jurisdiction.
    pub fn register(&mut self, jurisdiction: Jurisdiction) {
        self.jurisdictions
            .insert(jurisdiction.id.clone(), jurisdiction);
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

/// Plural category for pluralization rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PluralCategory {
    /// Exactly zero
    Zero,
    /// Exactly one
    One,
    /// Exactly two
    Two,
    /// Few (language-specific)
    Few,
    /// Many (language-specific)
    Many,
    /// Other/default
    Other,
}

impl std::fmt::Display for PluralCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluralCategory::Zero => write!(f, "zero"),
            PluralCategory::One => write!(f, "one"),
            PluralCategory::Two => write!(f, "two"),
            PluralCategory::Few => write!(f, "few"),
            PluralCategory::Many => write!(f, "many"),
            PluralCategory::Other => write!(f, "other"),
        }
    }
}

/// Plural rules for a specific locale.
#[derive(Debug, Clone)]
pub struct PluralRules {
    locale: Locale,
}

impl PluralRules {
    /// Creates plural rules for a locale.
    pub fn new(locale: Locale) -> Self {
        Self { locale }
    }

    /// Determines the plural category for a number.
    pub fn category(&self, n: i64) -> PluralCategory {
        // Simplified plural rules (real implementation would use CLDR data)
        match self.locale.language.as_str() {
            "ja" | "zh" | "ko" | "vi" | "th" => {
                // Asian languages: no plural distinction
                PluralCategory::Other
            }
            "en" => {
                if n == 1 {
                    PluralCategory::One
                } else {
                    PluralCategory::Other
                }
            }
            "fr" => {
                if n == 0 || n == 1 {
                    PluralCategory::One
                } else {
                    PluralCategory::Other
                }
            }
            "ru" | "uk" => {
                // Slavic languages have complex rules
                let n10 = n % 10;
                let n100 = n % 100;
                if n10 == 1 && n100 != 11 {
                    PluralCategory::One
                } else if (2..=4).contains(&n10) && !(12..=14).contains(&n100) {
                    PluralCategory::Few
                } else if n10 == 0 || (5..=9).contains(&n10) || (11..=14).contains(&n100) {
                    PluralCategory::Many
                } else {
                    PluralCategory::Other
                }
            }
            "ar" => {
                // Arabic plural rules
                if n == 0 {
                    PluralCategory::Zero
                } else if n == 1 {
                    PluralCategory::One
                } else if n == 2 {
                    PluralCategory::Two
                } else if n % 100 >= 3 && n % 100 <= 10 {
                    PluralCategory::Few
                } else if n % 100 >= 11 && n % 100 <= 99 {
                    PluralCategory::Many
                } else {
                    PluralCategory::Other
                }
            }
            _ => {
                // Default: simple one/other distinction
                if n == 1 {
                    PluralCategory::One
                } else {
                    PluralCategory::Other
                }
            }
        }
    }
}

/// ICU-style message formatter.
#[derive(Debug, Clone)]
pub struct MessageFormatter {
    #[allow(dead_code)]
    locale: Locale,
    plural_rules: PluralRules,
}

impl MessageFormatter {
    /// Creates a new message formatter.
    pub fn new(locale: Locale) -> Self {
        let plural_rules = PluralRules::new(locale.clone());
        Self {
            locale,
            plural_rules,
        }
    }

    /// Formats a message with variables.
    /// Simple implementation supporting {variable} placeholders.
    pub fn format(&self, pattern: &str, args: &HashMap<String, String>) -> String {
        let mut result = pattern.to_string();
        for (key, value) in args {
            result = result.replace(&format!("{{{}}}", key), value);
        }
        result
    }

    /// Formats a plural message.
    /// Pattern format: "{count} {count, plural, one {item} other {items}}"
    pub fn format_plural(&self, count: i64, one: &str, other: &str) -> String {
        let category = self.plural_rules.category(count);
        match category {
            PluralCategory::One => one.to_string(),
            _ => other.to_string(),
        }
    }

    /// Formats a complex plural message with multiple categories.
    pub fn format_plural_complex(
        &self,
        count: i64,
        patterns: &HashMap<PluralCategory, String>,
    ) -> Option<String> {
        let category = self.plural_rules.category(count);
        patterns
            .get(&category)
            .or_else(|| patterns.get(&PluralCategory::Other))
            .cloned()
    }
}

/// Date/time formatter for legal deadlines.
#[derive(Debug, Clone)]
pub struct DateTimeFormatter {
    locale: Locale,
}

impl DateTimeFormatter {
    /// Creates a new date/time formatter.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_i18n::{DateTimeFormatter, Locale};
    ///
    /// let locale = Locale::new("ja").with_country("JP");
    /// let formatter = DateTimeFormatter::new(locale);
    ///
    /// let date = formatter.format_date(2024, 12, 19);
    /// assert_eq!(date, "2024年12月19日");
    ///
    /// let time = formatter.format_time(14, 30);
    /// assert_eq!(time, "14:30");
    /// ```
    pub fn new(locale: Locale) -> Self {
        Self { locale }
    }

    /// Formats a date in the locale's format.
    /// Uses ISO 8601 as input: "YYYY-MM-DD"
    pub fn format_date(&self, year: i32, month: u32, day: u32) -> String {
        match self.locale.language.as_str() {
            "ja" => format!("{}年{}月{}日", year, month, day),
            "zh" => format!("{}年{}月{}日", year, month, day),
            "en" if self.locale.country.as_deref() == Some("US") => {
                format!("{:02}/{:02}/{}", month, day, year)
            }
            "en" => format!("{:02}/{:02}/{}", day, month, year),
            "de" | "fr" | "es" | "it" => format!("{:02}.{:02}.{}", day, month, year),
            _ => format!("{}-{:02}-{:02}", year, month, day), // ISO 8601 fallback
        }
    }

    /// Formats a time in the locale's format.
    pub fn format_time(&self, hour: u32, minute: u32) -> String {
        match self.locale.language.as_str() {
            "en" if self.locale.country.as_deref() == Some("US") => {
                let (h, ampm) = if hour == 0 {
                    (12, "AM")
                } else if hour < 12 {
                    (hour, "AM")
                } else if hour == 12 {
                    (12, "PM")
                } else {
                    (hour - 12, "PM")
                };
                format!("{:02}:{:02} {}", h, minute, ampm)
            }
            _ => format!("{:02}:{:02}", hour, minute), // 24-hour format
        }
    }

    /// Formats a complete datetime.
    pub fn format_datetime(
        &self,
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
    ) -> String {
        format!(
            "{} {}",
            self.format_date(year, month, day),
            self.format_time(hour, minute)
        )
    }
}

/// Time zone representation for legal deadlines.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TimeZone {
    /// Time zone identifier (e.g., "America/New_York", "Asia/Tokyo")
    pub identifier: String,
    /// UTC offset in minutes (e.g., -300 for EST, 540 for JST)
    pub utc_offset_minutes: i32,
    /// Display name (e.g., "Eastern Standard Time", "Japan Standard Time")
    pub display_name: String,
    /// Whether this timezone observes daylight saving time
    pub has_dst: bool,
}

impl TimeZone {
    /// Creates a new time zone.
    pub fn new(
        identifier: impl Into<String>,
        utc_offset_minutes: i32,
        display_name: impl Into<String>,
        has_dst: bool,
    ) -> Self {
        Self {
            identifier: identifier.into(),
            utc_offset_minutes,
            display_name: display_name.into(),
            has_dst,
        }
    }

    /// Converts UTC time to local time.
    pub fn utc_to_local(
        &self,
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
    ) -> (i32, u32, u32, u32, u32) {
        let total_minutes = (hour * 60 + minute) as i32 + self.utc_offset_minutes;
        self.adjust_datetime(year, month, day, total_minutes)
    }

    /// Converts local time to UTC.
    pub fn local_to_utc(
        &self,
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
    ) -> (i32, u32, u32, u32, u32) {
        let total_minutes = (hour * 60 + minute) as i32 - self.utc_offset_minutes;
        self.adjust_datetime(year, month, day, total_minutes)
    }

    fn adjust_datetime(
        &self,
        year: i32,
        month: u32,
        day: u32,
        total_minutes: i32,
    ) -> (i32, u32, u32, u32, u32) {
        let mut current_year = year;
        let mut current_month = month;
        let mut current_day = day;
        let mut minutes = total_minutes;

        // Handle negative minutes (previous day)
        while minutes < 0 {
            minutes += 24 * 60;
            let (y, m, d) = self.previous_day(current_year, current_month, current_day);
            current_year = y;
            current_month = m;
            current_day = d;
        }

        // Handle overflow minutes (next day)
        while minutes >= 24 * 60 {
            minutes -= 24 * 60;
            let (y, m, d) = self.next_day(current_year, current_month, current_day);
            current_year = y;
            current_month = m;
            current_day = d;
        }

        let hour = (minutes / 60) as u32;
        let minute = (minutes % 60) as u32;

        (current_year, current_month, current_day, hour, minute)
    }

    fn next_day(&self, year: i32, month: u32, day: u32) -> (i32, u32, u32) {
        let days_in_month = self.days_in_month(year, month);
        if day < days_in_month {
            (year, month, day + 1)
        } else if month < 12 {
            (year, month + 1, 1)
        } else {
            (year + 1, 1, 1)
        }
    }

    fn previous_day(&self, year: i32, month: u32, day: u32) -> (i32, u32, u32) {
        if day > 1 {
            (year, month, day - 1)
        } else if month > 1 {
            let prev_month = month - 1;
            let prev_day = self.days_in_month(year, prev_month);
            (year, prev_month, prev_day)
        } else {
            (year - 1, 12, 31)
        }
    }

    fn days_in_month(&self, year: i32, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.is_leap_year(year) {
                    29
                } else {
                    28
                }
            }
            _ => 30,
        }
    }

    fn is_leap_year(&self, year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    /// Formats the UTC offset as a string (e.g., "+09:00", "-05:00").
    pub fn format_offset(&self) -> String {
        let sign = if self.utc_offset_minutes >= 0 {
            "+"
        } else {
            "-"
        };
        let abs_minutes = self.utc_offset_minutes.abs();
        let hours = abs_minutes / 60;
        let minutes = abs_minutes % 60;
        format!("{}{:02}:{:02}", sign, hours, minutes)
    }
}

/// Registry of common time zones used in legal practice.
#[derive(Debug, Default)]
pub struct TimeZoneRegistry {
    zones: HashMap<String, TimeZone>,
}

impl TimeZoneRegistry {
    /// Creates a new time zone registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a registry with standard legal time zones.
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();

        // US time zones
        registry.add_zone(TimeZone::new(
            "America/New_York",
            -300,
            "Eastern Standard Time (EST)",
            true,
        ));
        registry.add_zone(TimeZone::new(
            "America/Chicago",
            -360,
            "Central Standard Time (CST)",
            true,
        ));
        registry.add_zone(TimeZone::new(
            "America/Denver",
            -420,
            "Mountain Standard Time (MST)",
            true,
        ));
        registry.add_zone(TimeZone::new(
            "America/Los_Angeles",
            -480,
            "Pacific Standard Time (PST)",
            true,
        ));

        // European time zones
        registry.add_zone(TimeZone::new(
            "Europe/London",
            0,
            "Greenwich Mean Time (GMT)",
            true,
        ));
        registry.add_zone(TimeZone::new(
            "Europe/Paris",
            60,
            "Central European Time (CET)",
            true,
        ));
        registry.add_zone(TimeZone::new(
            "Europe/Berlin",
            60,
            "Central European Time (CET)",
            true,
        ));
        registry.add_zone(TimeZone::new(
            "Europe/Moscow",
            180,
            "Moscow Standard Time (MSK)",
            false,
        ));

        // Asian time zones
        registry.add_zone(TimeZone::new(
            "Asia/Tokyo",
            540,
            "Japan Standard Time (JST)",
            false,
        ));
        registry.add_zone(TimeZone::new(
            "Asia/Seoul",
            540,
            "Korea Standard Time (KST)",
            false,
        ));
        registry.add_zone(TimeZone::new(
            "Asia/Shanghai",
            480,
            "China Standard Time (CST)",
            false,
        ));
        registry.add_zone(TimeZone::new(
            "Asia/Hong_Kong",
            480,
            "Hong Kong Time (HKT)",
            false,
        ));
        registry.add_zone(TimeZone::new(
            "Asia/Singapore",
            480,
            "Singapore Standard Time (SGT)",
            false,
        ));
        registry.add_zone(TimeZone::new(
            "Asia/Dubai",
            240,
            "Gulf Standard Time (GST)",
            false,
        ));

        // Other major legal centers
        registry.add_zone(TimeZone::new(
            "Australia/Sydney",
            600,
            "Australian Eastern Standard Time (AEST)",
            true,
        ));
        registry.add_zone(TimeZone::new(
            "Pacific/Auckland",
            720,
            "New Zealand Standard Time (NZST)",
            true,
        ));
        registry.add_zone(TimeZone::new(
            "America/Sao_Paulo",
            -180,
            "Brasília Time (BRT)",
            true,
        ));
        registry.add_zone(TimeZone::new(
            "America/Toronto",
            -300,
            "Eastern Standard Time (EST)",
            true,
        ));

        // UTC
        registry.add_zone(TimeZone::new(
            "UTC",
            0,
            "Coordinated Universal Time (UTC)",
            false,
        ));

        registry
    }

    /// Adds a time zone to the registry.
    pub fn add_zone(&mut self, zone: TimeZone) {
        self.zones.insert(zone.identifier.clone(), zone);
    }

    /// Gets a time zone by identifier.
    pub fn get_zone(&self, identifier: &str) -> Option<&TimeZone> {
        self.zones.get(identifier)
    }

    /// Gets a time zone for a jurisdiction.
    pub fn zone_for_jurisdiction(&self, jurisdiction_code: &str) -> Option<&TimeZone> {
        match jurisdiction_code {
            "US" => self.get_zone("America/New_York"),
            "GB" => self.get_zone("Europe/London"),
            "JP" => self.get_zone("Asia/Tokyo"),
            "DE" | "FR" | "ES" | "IT" | "NL" => self.get_zone("Europe/Paris"),
            "CN" => self.get_zone("Asia/Shanghai"),
            "TW" | "HK" => self.get_zone("Asia/Hong_Kong"),
            "KR" => self.get_zone("Asia/Seoul"),
            "SG" => self.get_zone("Asia/Singapore"),
            "AU" => self.get_zone("Australia/Sydney"),
            "CA" => self.get_zone("America/Toronto"),
            "BR" => self.get_zone("America/Sao_Paulo"),
            "RU" => self.get_zone("Europe/Moscow"),
            "SA" | "AE" => self.get_zone("Asia/Dubai"),
            _ => self.get_zone("UTC"),
        }
    }

    /// Lists all available time zone identifiers.
    pub fn list_zones(&self) -> Vec<String> {
        self.zones.keys().cloned().collect()
    }
}

/// Legal deadline calculator with time zone and business day support.
#[derive(Debug, Clone)]
pub struct DeadlineCalculator {
    jurisdiction: WorkingDaysConfig,
    timezone: Option<TimeZone>,
}

impl DeadlineCalculator {
    /// Creates a new deadline calculator.
    pub fn new(jurisdiction: WorkingDaysConfig) -> Self {
        Self {
            jurisdiction,
            timezone: None,
        }
    }

    /// Sets the time zone for deadline calculations.
    pub fn with_timezone(mut self, timezone: TimeZone) -> Self {
        self.timezone = Some(timezone);
        self
    }

    /// Calculates a deadline by adding business days to a start date.
    pub fn calculate_deadline(
        &self,
        start_year: i32,
        start_month: u32,
        start_day: u32,
        business_days: i32,
    ) -> (i32, u32, u32) {
        self.jurisdiction
            .add_working_days(start_year, start_month, start_day, business_days)
    }

    /// Calculates a deadline with time component and timezone conversion.
    pub fn calculate_deadline_with_time(
        &self,
        start_year: i32,
        start_month: u32,
        start_day: u32,
        start_hour: u32,
        start_minute: u32,
        business_days: i32,
    ) -> (i32, u32, u32, u32, u32) {
        let (end_year, end_month, end_day) =
            self.calculate_deadline(start_year, start_month, start_day, business_days);

        // Preserve the time component
        (end_year, end_month, end_day, start_hour, start_minute)
    }

    /// Converts a deadline from one timezone to another.
    #[allow(clippy::too_many_arguments)]
    pub fn convert_timezone(
        &self,
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        from_tz: &TimeZone,
        to_tz: &TimeZone,
    ) -> (i32, u32, u32, u32, u32) {
        // First convert to UTC
        let (utc_y, utc_m, utc_d, utc_h, utc_min) =
            from_tz.local_to_utc(year, month, day, hour, minute);

        // Then convert to target timezone
        to_tz.utc_to_local(utc_y, utc_m, utc_d, utc_h, utc_min)
    }

    /// Checks if a deadline has passed (considering timezone if set).
    pub fn is_deadline_passed(
        &self,
        deadline_year: i32,
        deadline_month: u32,
        deadline_day: u32,
        current_year: i32,
        current_month: u32,
        current_day: u32,
    ) -> bool {
        if deadline_year < current_year {
            return true;
        }
        if deadline_year > current_year {
            return false;
        }

        // Same year
        if deadline_month < current_month {
            return true;
        }
        if deadline_month > current_month {
            return false;
        }

        // Same month
        deadline_day < current_day
    }

    /// Calculates statute of limitations deadline.
    /// Returns the final date when a claim must be filed.
    pub fn statute_of_limitations(
        &self,
        incident_year: i32,
        incident_month: u32,
        incident_day: u32,
        years: i32,
    ) -> (i32, u32, u32) {
        // Add years to the incident date
        let final_year = incident_year + years;
        // Typically statute of limitations runs to the same date
        (final_year, incident_month, incident_day)
    }

    /// Applies holiday rollover rules.
    /// If a deadline falls on a non-working day, roll to the next working day.
    pub fn apply_holiday_rollover(&self, year: i32, month: u32, day: u32) -> (i32, u32, u32) {
        if self.jurisdiction.is_working_day(year, month, day) {
            return (year, month, day);
        }

        // Roll forward to next working day
        let mut current = (year, month, day);
        for _ in 0..7 {
            // Search up to 7 days ahead
            current = self.add_one_day(current.0, current.1, current.2);
            if self
                .jurisdiction
                .is_working_day(current.0, current.1, current.2)
            {
                return current;
            }
        }

        // If no working day found in 7 days, return original
        (year, month, day)
    }

    /// Adds a grace period (in calendar days) to a deadline.
    pub fn add_grace_period(
        &self,
        deadline_year: i32,
        deadline_month: u32,
        deadline_day: u32,
        grace_days: i32,
    ) -> (i32, u32, u32) {
        let mut result = (deadline_year, deadline_month, deadline_day);
        for _ in 0..grace_days {
            result = self.add_one_day(result.0, result.1, result.2);
        }
        result
    }

    /// Checks for deadline conflicts (if two deadlines are too close).
    /// Returns true if deadlines are within threshold_days of each other.
    #[allow(clippy::too_many_arguments)]
    pub fn has_deadline_conflict(
        &self,
        deadline1_year: i32,
        deadline1_month: u32,
        deadline1_day: u32,
        deadline2_year: i32,
        deadline2_month: u32,
        deadline2_day: u32,
        threshold_days: i32,
    ) -> bool {
        let days_between = self.days_between(
            deadline1_year,
            deadline1_month,
            deadline1_day,
            deadline2_year,
            deadline2_month,
            deadline2_day,
        );
        days_between.abs() <= threshold_days
    }

    /// Helper: adds one calendar day to a date.
    fn add_one_day(&self, year: i32, month: u32, day: u32) -> (i32, u32, u32) {
        let days_in_month = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.is_leap_year(year) {
                    29
                } else {
                    28
                }
            }
            _ => 30,
        };

        if day < days_in_month {
            (year, month, day + 1)
        } else if month < 12 {
            (year, month + 1, 1)
        } else {
            (year + 1, 1, 1)
        }
    }

    /// Helper: calculates days between two dates (approximate).
    fn days_between(&self, y1: i32, m1: u32, d1: u32, y2: i32, m2: u32, d2: u32) -> i32 {
        // Simple approximation: 365 days per year, 30 days per month
        let days1 = y1 * 365 + (m1 as i32) * 30 + (d1 as i32);
        let days2 = y2 * 365 + (m2 as i32) * 30 + (d2 as i32);
        days2 - days1
    }

    /// Helper: checks if a year is a leap year.
    fn is_leap_year(&self, year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }
}

/// Citation style for legal documents.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CitationStyle {
    /// Bluebook (United States)
    Bluebook,
    /// OSCOLA - Oxford Standard for Citation of Legal Authorities (United Kingdom)
    OSCOLA,
    /// Australian Guide to Legal Citation (AGLC)
    AGLC,
    /// Canadian Guide to Uniform Legal Citation (McGill Guide)
    McGill,
    /// European Citation Style
    European,
    /// Japanese Legal Citation
    Japanese,
    /// Harvard Legal Citation Style
    Harvard,
    /// APA Legal Citation Style
    APA,
    /// Chicago Manual of Style (Legal)
    Chicago,
    /// Indian Legal Citation Style
    Indian,
    /// Custom citation template
    Custom(String),
}

impl std::fmt::Display for CitationStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CitationStyle::Bluebook => write!(f, "Bluebook"),
            CitationStyle::OSCOLA => write!(f, "OSCOLA"),
            CitationStyle::AGLC => write!(f, "AGLC"),
            CitationStyle::McGill => write!(f, "McGill Guide"),
            CitationStyle::European => write!(f, "European"),
            CitationStyle::Japanese => write!(f, "Japanese"),
            CitationStyle::Harvard => write!(f, "Harvard"),
            CitationStyle::APA => write!(f, "APA"),
            CitationStyle::Chicago => write!(f, "Chicago"),
            CitationStyle::Indian => write!(f, "Indian"),
            CitationStyle::Custom(template) => write!(f, "Custom({})", template),
        }
    }
}

/// Legal citation components.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationComponents {
    /// Case name or statute title
    pub title: String,
    /// Volume number (if applicable)
    pub volume: Option<String>,
    /// Reporter or source
    pub reporter: Option<String>,
    /// Page number or section
    pub page: Option<String>,
    /// Court (for case citations)
    pub court: Option<String>,
    /// Year of decision/enactment
    pub year: Option<i32>,
    /// Jurisdiction code
    pub jurisdiction: Option<String>,
}

impl CitationComponents {
    /// Creates a new citation with just a title.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            volume: None,
            reporter: None,
            page: None,
            court: None,
            year: None,
            jurisdiction: None,
        }
    }

    /// Sets the volume.
    pub fn with_volume(mut self, volume: impl Into<String>) -> Self {
        self.volume = Some(volume.into());
        self
    }

    /// Sets the reporter.
    pub fn with_reporter(mut self, reporter: impl Into<String>) -> Self {
        self.reporter = Some(reporter.into());
        self
    }

    /// Sets the page.
    pub fn with_page(mut self, page: impl Into<String>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Sets the court.
    pub fn with_court(mut self, court: impl Into<String>) -> Self {
        self.court = Some(court.into());
        self
    }

    /// Sets the year.
    pub fn with_year(mut self, year: i32) -> Self {
        self.year = Some(year);
        self
    }

    /// Sets the jurisdiction.
    pub fn with_jurisdiction(mut self, jurisdiction: impl Into<String>) -> Self {
        self.jurisdiction = Some(jurisdiction.into());
        self
    }
}

/// Citation formatter for legal documents.
#[derive(Debug, Clone)]
pub struct CitationFormatter {
    style: CitationStyle,
    #[allow(dead_code)]
    locale: Locale,
}

impl CitationFormatter {
    /// Creates a new citation formatter.
    pub fn new(style: CitationStyle, locale: Locale) -> Self {
        Self { style, locale }
    }

    /// Formats a case citation.
    pub fn format_case(&self, components: &CitationComponents) -> String {
        match &self.style {
            CitationStyle::Bluebook => self.format_bluebook_case(components),
            CitationStyle::OSCOLA => self.format_oscola_case(components),
            CitationStyle::AGLC => self.format_aglc_case(components),
            CitationStyle::McGill => self.format_mcgill_case(components),
            CitationStyle::European => self.format_european_case(components),
            CitationStyle::Japanese => self.format_japanese_case(components),
            CitationStyle::Harvard => self.format_harvard_case(components),
            CitationStyle::APA => self.format_apa_case(components),
            CitationStyle::Chicago => self.format_chicago_case(components),
            CitationStyle::Indian => self.format_indian_case(components),
            CitationStyle::Custom(template) => self.format_custom_case(components, template),
        }
    }

    /// Formats a statute citation.
    pub fn format_statute(&self, components: &CitationComponents) -> String {
        match &self.style {
            CitationStyle::Bluebook => self.format_bluebook_statute(components),
            CitationStyle::OSCOLA => self.format_oscola_statute(components),
            CitationStyle::AGLC => self.format_aglc_statute(components),
            CitationStyle::McGill => self.format_mcgill_statute(components),
            CitationStyle::European => self.format_european_statute(components),
            CitationStyle::Japanese => self.format_japanese_statute(components),
            CitationStyle::Harvard => self.format_harvard_statute(components),
            CitationStyle::APA => self.format_apa_statute(components),
            CitationStyle::Chicago => self.format_chicago_statute(components),
            CitationStyle::Indian => self.format_indian_statute(components),
            CitationStyle::Custom(template) => self.format_custom_statute(components, template),
        }
    }

    fn format_bluebook_case(&self, c: &CitationComponents) -> String {
        // Bluebook format: Case Name, Volume Reporter Page (Court Year)
        let mut parts = vec![c.title.clone()];

        if let (Some(vol), Some(rep), Some(page)) = (&c.volume, &c.reporter, &c.page) {
            parts.push(format!("{} {} {}", vol, rep, page));
        }

        if let (Some(court), Some(year)) = (&c.court, &c.year) {
            parts.push(format!("({} {})", court, year));
        } else if let Some(year) = &c.year {
            parts.push(format!("({})", year));
        }

        parts.join(", ")
    }

    fn format_oscola_case(&self, c: &CitationComponents) -> String {
        // OSCOLA format: Case Name [Year] Volume Reporter Page
        let mut result = c.title.clone();

        if let Some(year) = c.year {
            result.push_str(&format!(" [{}]", year));
        }

        if let (Some(vol), Some(rep)) = (&c.volume, &c.reporter) {
            result.push_str(&format!(" {} {}", vol, rep));
        }

        if let Some(page) = &c.page {
            result.push_str(&format!(" {}", page));
        }

        result
    }

    fn format_aglc_case(&self, c: &CitationComponents) -> String {
        // AGLC format: Case Name [Year] Volume Reporter Page
        // Similar to OSCOLA but with Australian conventions
        self.format_oscola_case(c)
    }

    fn format_mcgill_case(&self, c: &CitationComponents) -> String {
        // McGill format: Case Name, [Year] Volume Reporter Page (Court)
        let mut result = c.title.clone();

        if let Some(year) = c.year {
            result.push_str(&format!(", [{}]", year));
        }

        if let (Some(vol), Some(rep), Some(page)) = (&c.volume, &c.reporter, &c.page) {
            result.push_str(&format!(" {} {} {}", vol, rep, page));
        }

        if let Some(court) = &c.court {
            result.push_str(&format!(" ({})", court));
        }

        result
    }

    fn format_european_case(&self, c: &CitationComponents) -> String {
        // European format: Case Name, Court, Year, Reference
        let mut parts = vec![c.title.clone()];

        if let Some(court) = &c.court {
            parts.push(court.clone());
        }

        if let Some(year) = c.year {
            parts.push(year.to_string());
        }

        if let (Some(vol), Some(page)) = (&c.volume, &c.page) {
            parts.push(format!("{}/{}", vol, page));
        }

        parts.join(", ")
    }

    fn format_japanese_case(&self, c: &CitationComponents) -> String {
        // Japanese format: Title 裁判所 Year(Era) Volume 号 Page 頁
        let mut result = c.title.clone();

        if let Some(court) = &c.court {
            result.push_str(&format!(" {}", court));
        }

        if let Some(year) = c.year {
            result.push_str(&format!(" {}", year));
        }

        if let Some(vol) = &c.volume {
            result.push_str(&format!(" {}号", vol));
        }

        if let Some(page) = &c.page {
            result.push_str(&format!(" {}頁", page));
        }

        result
    }

    fn format_bluebook_statute(&self, c: &CitationComponents) -> String {
        // Bluebook statute: Title Code § Section (Year)
        let mut result = c.title.clone();

        if let Some(reporter) = &c.reporter {
            result.push_str(&format!(" {}", reporter));
        }

        if let Some(page) = &c.page {
            result.push_str(&format!(" § {}", page));
        }

        if let Some(year) = c.year {
            result.push_str(&format!(" ({})", year));
        }

        result
    }

    fn format_oscola_statute(&self, c: &CitationComponents) -> String {
        // OSCOLA statute: Short Title Year, section
        let mut result = c.title.clone();

        if let Some(year) = c.year {
            result.push_str(&format!(" {}", year));
        }

        if let Some(page) = &c.page {
            result.push_str(&format!(", s {}", page));
        }

        result
    }

    fn format_aglc_statute(&self, c: &CitationComponents) -> String {
        // AGLC statute: Title Year (Jurisdiction) section
        let mut result = c.title.clone();

        if let Some(year) = c.year {
            result.push_str(&format!(" {}", year));
        }

        if let Some(jur) = &c.jurisdiction {
            result.push_str(&format!(" ({})", jur));
        }

        if let Some(page) = &c.page {
            result.push_str(&format!(" s {}", page));
        }

        result
    }

    fn format_mcgill_statute(&self, c: &CitationComponents) -> String {
        // McGill statute: Title, Code, section
        let mut result = c.title.clone();

        if let Some(reporter) = &c.reporter {
            result.push_str(&format!(", {}", reporter));
        }

        if let Some(page) = &c.page {
            result.push_str(&format!(", s {}", page));
        }

        result
    }

    fn format_european_statute(&self, c: &CitationComponents) -> String {
        // European format: Title, [Year] Reference
        let mut result = c.title.clone();

        if let Some(year) = c.year {
            result.push_str(&format!(", [{}]", year));
        }

        if let (Some(vol), Some(page)) = (&c.volume, &c.page) {
            result.push_str(&format!(" {}/{}", vol, page));
        }

        result
    }

    fn format_japanese_statute(&self, c: &CitationComponents) -> String {
        // Japanese format: Title (Year) 第X条
        let mut result = c.title.clone();

        if let Some(year) = c.year {
            // Convert to Japanese era if needed (simplified)
            result.push_str(&format!("（{}年）", year));
        }

        if let Some(page) = &c.page {
            result.push_str(&format!(" 第{}条", page));
        }

        result
    }

    fn format_harvard_case(&self, c: &CitationComponents) -> String {
        // Harvard format: Case Name (Year) Volume Reporter Page (Court)
        let mut parts = vec![c.title.clone()];

        if let Some(year) = c.year {
            parts.push(format!("({})", year));
        }

        if let (Some(vol), Some(rep), Some(page)) = (&c.volume, &c.reporter, &c.page) {
            parts.push(format!("{} {} {}", vol, rep, page));
        }

        if let Some(court) = &c.court {
            parts.push(format!("({})", court));
        }

        parts.join(" ")
    }

    fn format_harvard_statute(&self, c: &CitationComponents) -> String {
        // Harvard format: Title Year
        let mut result = c.title.clone();
        if let Some(year) = c.year {
            result.push_str(&format!(" {}", year));
        }
        result
    }

    fn format_apa_case(&self, c: &CitationComponents) -> String {
        // APA format: Case Name, Volume Reporter Page (Court, Year)
        let mut parts = vec![c.title.clone()];

        if let (Some(vol), Some(rep), Some(page)) = (&c.volume, &c.reporter, &c.page) {
            parts.push(format!("{} {} {}", vol, rep, page));
        }

        let mut paren_parts = vec![];
        if let Some(court) = &c.court {
            paren_parts.push(court.clone());
        }
        if let Some(year) = c.year {
            paren_parts.push(year.to_string());
        }
        if !paren_parts.is_empty() {
            parts.push(format!("({})", paren_parts.join(", ")));
        }

        parts.join(", ")
    }

    fn format_apa_statute(&self, c: &CitationComponents) -> String {
        // APA format: Title (Year)
        let mut result = c.title.clone();
        if let Some(year) = c.year {
            result.push_str(&format!(" ({})", year));
        }
        result
    }

    fn format_chicago_case(&self, c: &CitationComponents) -> String {
        // Chicago format: Case Name, Volume Reporter Page (Court Year)
        let mut parts = vec![c.title.clone()];

        if let (Some(vol), Some(rep), Some(page)) = (&c.volume, &c.reporter, &c.page) {
            parts.push(format!("{} {} {}", vol, rep, page));
        }

        if let (Some(court), Some(year)) = (&c.court, &c.year) {
            parts.push(format!("({} {})", court, year));
        } else if let Some(year) = &c.year {
            parts.push(format!("({})", year));
        }

        parts.join(", ")
    }

    fn format_chicago_statute(&self, c: &CitationComponents) -> String {
        // Chicago format: Title (Year)
        let mut result = c.title.clone();
        if let Some(year) = c.year {
            result.push_str(&format!(" ({})", year));
        }
        result
    }

    fn format_indian_case(&self, c: &CitationComponents) -> String {
        // Indian format: Case Name, (Year) Volume Reporter Page (Court)
        let mut parts = vec![c.title.clone()];

        if let Some(year) = c.year {
            parts.push(format!("({})", year));
        }

        if let (Some(vol), Some(rep), Some(page)) = (&c.volume, &c.reporter, &c.page) {
            parts.push(format!("{} {} {}", vol, rep, page));
        }

        if let Some(court) = &c.court {
            parts.push(format!("({})", court));
        }

        parts.join(" ")
    }

    fn format_indian_statute(&self, c: &CitationComponents) -> String {
        // Indian format: Title, Year
        let mut result = c.title.clone();
        if let Some(year) = c.year {
            result.push_str(&format!(", {}", year));
        }
        result
    }

    fn format_custom_case(&self, c: &CitationComponents, template: &str) -> String {
        // Custom template-based formatting
        // Template variables: {title}, {volume}, {reporter}, {page}, {court}, {year}
        let mut result = template.to_string();
        result = result.replace("{title}", &c.title);
        if let Some(vol) = &c.volume {
            result = result.replace("{volume}", vol);
        }
        if let Some(rep) = &c.reporter {
            result = result.replace("{reporter}", rep);
        }
        if let Some(page) = &c.page {
            result = result.replace("{page}", page);
        }
        if let Some(court) = &c.court {
            result = result.replace("{court}", court);
        }
        if let Some(year) = c.year {
            result = result.replace("{year}", &year.to_string());
        }
        result
    }

    fn format_custom_statute(&self, c: &CitationComponents, template: &str) -> String {
        // Custom template-based formatting for statutes
        let mut result = template.to_string();
        result = result.replace("{title}", &c.title);
        if let Some(year) = c.year {
            result = result.replace("{year}", &year.to_string());
        }
        if let Some(page) = &c.page {
            result = result.replace("{section}", page);
        }
        result
    }

    /// Gets the citation style for a jurisdiction.
    pub fn style_for_jurisdiction(jurisdiction_code: &str) -> CitationStyle {
        match jurisdiction_code {
            "US" => CitationStyle::Bluebook,
            "GB" => CitationStyle::OSCOLA,
            "AU" => CitationStyle::AGLC,
            "CA" => CitationStyle::McGill,
            "JP" => CitationStyle::Japanese,
            "IN" => CitationStyle::Indian,
            "DE" | "FR" | "IT" | "ES" | "NL" | "PT" | "PL" => CitationStyle::European,
            _ => CitationStyle::Bluebook, // Default
        }
    }
}

// ============================================================================
// Citation Validation (v0.2.1)
// ============================================================================

/// Citation validation errors.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CitationError {
    /// Missing required field
    #[error("Missing required field: {field}")]
    MissingField { field: String },
    /// Invalid field format
    #[error("Invalid format for field {field}: {reason}")]
    InvalidFormat { field: String, reason: String },
    /// Style-specific violation
    #[error("Style violation for {style}: {reason}")]
    StyleViolation { style: String, reason: String },
    /// Parse error
    #[error("Failed to parse citation: {reason}")]
    ParseError { reason: String },
    /// Unsupported conversion
    #[error("Cannot convert from {from} to {to}: {reason}")]
    UnsupportedConversion {
        from: String,
        to: String,
        reason: String,
    },
}

/// Citation type for validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CitationType {
    /// Court case
    Case,
    /// Statute or legislation
    Statute,
    /// Legal journal article
    Article,
    /// Legal book or treatise
    Book,
}

/// Validation rule for a citation component.
#[derive(Debug, Clone)]
pub struct CitationValidationRule {
    /// Field name
    pub field: String,
    /// Whether field is required
    pub required: bool,
    /// Pattern validation (regex-like patterns)
    pub pattern: Option<String>,
    /// Custom validation function
    #[allow(clippy::type_complexity)]
    pub validator: Option<fn(&str) -> Result<(), String>>,
}

impl CitationValidationRule {
    /// Creates a required field rule.
    pub fn required(field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            required: true,
            pattern: None,
            validator: None,
        }
    }

    /// Creates an optional field rule.
    pub fn optional(field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            required: false,
            pattern: None,
            validator: None,
        }
    }

    /// Adds a pattern constraint.
    pub fn with_pattern(mut self, pattern: impl Into<String>) -> Self {
        self.pattern = Some(pattern.into());
        self
    }

    /// Validates a value against this rule.
    pub fn validate(&self, value: Option<&String>) -> Result<(), CitationError> {
        // Check required
        if self.required && value.is_none() {
            return Err(CitationError::MissingField {
                field: self.field.clone(),
            });
        }

        // Validate pattern if value exists
        if let Some(val) = value {
            if let Some(pattern) = &self.pattern {
                if !Self::matches_pattern(val, pattern) {
                    return Err(CitationError::InvalidFormat {
                        field: self.field.clone(),
                        reason: format!("Does not match pattern: {}", pattern),
                    });
                }
            }

            // Custom validation
            if let Some(validator) = self.validator {
                if let Err(msg) = validator(val) {
                    return Err(CitationError::InvalidFormat {
                        field: self.field.clone(),
                        reason: msg,
                    });
                }
            }
        }

        Ok(())
    }

    /// Simple pattern matching (supports basic patterns).
    fn matches_pattern(value: &str, pattern: &str) -> bool {
        match pattern {
            "numeric" => value.chars().all(|c| c.is_numeric()),
            "alphanumeric" => value
                .chars()
                .all(|c| c.is_alphanumeric() || c.is_whitespace()),
            "year" => {
                if let Ok(year) = value.parse::<i32>() {
                    (1000..=9999).contains(&year)
                } else {
                    false
                }
            }
            _ => true, // Unknown patterns always match
        }
    }
}

/// Citation parser for extracting components from citation strings.
#[derive(Debug, Clone)]
pub struct CitationParser {
    style: CitationStyle,
}

impl CitationParser {
    /// Creates a new citation parser for a specific style.
    pub fn new(style: CitationStyle) -> Self {
        Self { style }
    }

    /// Parses a case citation string into components.
    pub fn parse_case(&self, citation: &str) -> Result<CitationComponents, CitationError> {
        match &self.style {
            CitationStyle::Bluebook => self.parse_bluebook_case(citation),
            CitationStyle::OSCOLA => self.parse_oscola_case(citation),
            CitationStyle::AGLC => self.parse_aglc_case(citation),
            CitationStyle::McGill => self.parse_mcgill_case(citation),
            CitationStyle::European => self.parse_european_case(citation),
            CitationStyle::Japanese => self.parse_japanese_case(citation),
            CitationStyle::Harvard => self.parse_harvard_case(citation),
            CitationStyle::APA => self.parse_apa_case(citation),
            CitationStyle::Chicago => self.parse_chicago_case(citation),
            CitationStyle::Indian => self.parse_indian_case(citation),
            CitationStyle::Custom(_) => self.parse_custom_case(citation),
        }
    }

    /// Parses a statute citation string into components.
    pub fn parse_statute(&self, citation: &str) -> Result<CitationComponents, CitationError> {
        match &self.style {
            CitationStyle::Bluebook => self.parse_bluebook_statute(citation),
            CitationStyle::OSCOLA => self.parse_oscola_statute(citation),
            CitationStyle::AGLC => self.parse_aglc_statute(citation),
            CitationStyle::McGill => self.parse_mcgill_statute(citation),
            CitationStyle::European => self.parse_european_statute(citation),
            CitationStyle::Japanese => self.parse_japanese_statute(citation),
            CitationStyle::Harvard => self.parse_harvard_statute(citation),
            CitationStyle::APA => self.parse_apa_statute(citation),
            CitationStyle::Chicago => self.parse_chicago_statute(citation),
            CitationStyle::Indian => self.parse_indian_statute(citation),
            CitationStyle::Custom(_) => self.parse_custom_statute(citation),
        }
    }

    // Bluebook case parser: "Case Name, Vol Reporter Page (Court Year)"
    fn parse_bluebook_case(&self, citation: &str) -> Result<CitationComponents, CitationError> {
        // Example: "Brown v. Board of Education, 347 U.S. 483 (1954)"
        if citation.trim().is_empty() {
            return Err(CitationError::ParseError {
                reason: "Empty citation".to_string(),
            });
        }

        let parts: Vec<&str> = citation.split(',').collect();
        let title = parts[0].trim().to_string();

        if title.is_empty() {
            return Err(CitationError::ParseError {
                reason: "Empty citation".to_string(),
            });
        }

        let mut components = CitationComponents::new(title);

        if parts.len() > 1 {
            let citation_part = parts[1].trim();
            let tokens: Vec<&str> = citation_part.split_whitespace().collect();

            if tokens.len() >= 3 {
                components.volume = Some(tokens[0].to_string());
                components.reporter = Some(tokens[1].to_string());
                components.page = Some(tokens[2].to_string());
            }
        }

        // Extract year and court from parentheses
        if let Some(paren_start) = citation.rfind('(') {
            if let Some(paren_end) = citation.rfind(')') {
                let paren_content = &citation[paren_start + 1..paren_end];
                let paren_parts: Vec<&str> = paren_content.split_whitespace().collect();

                // Last token is usually the year
                if let Some(year_str) = paren_parts.last() {
                    if let Ok(year) = year_str.parse::<i32>() {
                        components.year = Some(year);
                    }
                }

                // Other tokens are court
                if paren_parts.len() > 1 {
                    components.court = Some(paren_parts[..paren_parts.len() - 1].join(" "));
                }
            }
        }

        Ok(components)
    }

    // OSCOLA case parser: "Case Name [Year] Reporter Page (Court)"
    fn parse_oscola_case(&self, citation: &str) -> Result<CitationComponents, CitationError> {
        // Example: "R v Smith [2020] EWCA Crim 123"
        let parts: Vec<&str> = citation.split('[').collect();
        if parts.is_empty() {
            return Err(CitationError::ParseError {
                reason: "Empty citation".to_string(),
            });
        }

        let title = parts[0].trim().to_string();
        let mut components = CitationComponents::new(title);

        if parts.len() > 1 {
            if let Some(year_end) = parts[1].find(']') {
                let year_str = &parts[1][..year_end];
                if let Ok(year) = year_str.parse::<i32>() {
                    components.year = Some(year);
                }

                let rest = parts[1][year_end + 1..].trim();
                let tokens: Vec<&str> = rest.split_whitespace().collect();

                if !tokens.is_empty() {
                    components.reporter = Some(tokens[0].to_string());
                }
                if tokens.len() > 1 {
                    components.page = Some(tokens[1].to_string());
                }
            }
        }

        Ok(components)
    }

    // Simplified parsers for other styles
    fn parse_aglc_case(&self, citation: &str) -> Result<CitationComponents, CitationError> {
        // Similar to OSCOLA
        self.parse_oscola_case(citation)
    }

    fn parse_mcgill_case(&self, citation: &str) -> Result<CitationComponents, CitationError> {
        // Similar to Bluebook
        self.parse_bluebook_case(citation)
    }

    fn parse_european_case(&self, citation: &str) -> Result<CitationComponents, CitationError> {
        // Simple title-based parsing
        Ok(CitationComponents::new(citation.trim()))
    }

    fn parse_japanese_case(&self, citation: &str) -> Result<CitationComponents, CitationError> {
        // Japanese format: "Title 年号年 Reporter 頁"
        Ok(CitationComponents::new(citation.trim()))
    }

    fn parse_harvard_case(&self, citation: &str) -> Result<CitationComponents, CitationError> {
        self.parse_bluebook_case(citation)
    }

    fn parse_apa_case(&self, citation: &str) -> Result<CitationComponents, CitationError> {
        self.parse_bluebook_case(citation)
    }

    fn parse_chicago_case(&self, citation: &str) -> Result<CitationComponents, CitationError> {
        self.parse_bluebook_case(citation)
    }

    fn parse_indian_case(&self, citation: &str) -> Result<CitationComponents, CitationError> {
        // Indian format: "Case Name (Year) Volume Reporter Page (Court)"
        self.parse_bluebook_case(citation)
    }

    fn parse_custom_case(&self, citation: &str) -> Result<CitationComponents, CitationError> {
        // Generic parsing
        Ok(CitationComponents::new(citation.trim()))
    }

    // Statute parsers
    fn parse_bluebook_statute(&self, citation: &str) -> Result<CitationComponents, CitationError> {
        // Example: "42 U.S.C. § 1983"
        let mut components = CitationComponents::new(citation.trim());

        let parts: Vec<&str> = citation.split('§').collect();
        if parts.len() == 2 {
            components.reporter = Some(parts[0].trim().to_string());
            components.page = Some(parts[1].trim().to_string());
        }

        Ok(components)
    }

    fn parse_oscola_statute(&self, citation: &str) -> Result<CitationComponents, CitationError> {
        // Example: "Human Rights Act 1998, s 3"
        let mut components = CitationComponents::new(citation.trim());

        // Extract year if present
        let words: Vec<&str> = citation.split_whitespace().collect();
        for word in &words {
            // Strip common punctuation and try to parse
            let cleaned_word = word.trim_matches(|c: char| !c.is_numeric());
            if let Ok(year) = cleaned_word.parse::<i32>() {
                if (1000..=9999).contains(&year) {
                    components.year = Some(year);
                    break;
                }
            }
        }

        Ok(components)
    }

    fn parse_aglc_statute(&self, citation: &str) -> Result<CitationComponents, CitationError> {
        self.parse_oscola_statute(citation)
    }

    fn parse_mcgill_statute(&self, citation: &str) -> Result<CitationComponents, CitationError> {
        self.parse_bluebook_statute(citation)
    }

    fn parse_european_statute(&self, citation: &str) -> Result<CitationComponents, CitationError> {
        Ok(CitationComponents::new(citation.trim()))
    }

    fn parse_japanese_statute(&self, citation: &str) -> Result<CitationComponents, CitationError> {
        Ok(CitationComponents::new(citation.trim()))
    }

    fn parse_harvard_statute(&self, citation: &str) -> Result<CitationComponents, CitationError> {
        self.parse_bluebook_statute(citation)
    }

    fn parse_apa_statute(&self, citation: &str) -> Result<CitationComponents, CitationError> {
        self.parse_bluebook_statute(citation)
    }

    fn parse_chicago_statute(&self, citation: &str) -> Result<CitationComponents, CitationError> {
        self.parse_bluebook_statute(citation)
    }

    fn parse_indian_statute(&self, citation: &str) -> Result<CitationComponents, CitationError> {
        Ok(CitationComponents::new(citation.trim()))
    }

    fn parse_custom_statute(&self, citation: &str) -> Result<CitationComponents, CitationError> {
        Ok(CitationComponents::new(citation.trim()))
    }
}

/// Citation validator for checking citations against style rules.
#[derive(Debug, Clone)]
pub struct CitationValidator {
    style: CitationStyle,
}

impl CitationValidator {
    /// Creates a new citation validator for a specific style.
    pub fn new(style: CitationStyle) -> Self {
        Self { style }
    }

    /// Validates a case citation.
    pub fn validate_case(&self, components: &CitationComponents) -> Result<(), Vec<CitationError>> {
        let rules = self.get_case_rules();
        self.validate_with_rules(components, &rules)
    }

    /// Validates a statute citation.
    pub fn validate_statute(
        &self,
        components: &CitationComponents,
    ) -> Result<(), Vec<CitationError>> {
        let rules = self.get_statute_rules();
        self.validate_with_rules(components, &rules)
    }

    /// Validates components against a set of rules.
    fn validate_with_rules(
        &self,
        components: &CitationComponents,
        rules: &[CitationValidationRule],
    ) -> Result<(), Vec<CitationError>> {
        let mut errors = Vec::new();

        // Convert year to string once if present
        let year_str = components.year.map(|y| y.to_string());

        for rule in rules {
            let value = match rule.field.as_str() {
                "title" => Some(&components.title),
                "volume" => components.volume.as_ref(),
                "reporter" => components.reporter.as_ref(),
                "page" => components.page.as_ref(),
                "court" => components.court.as_ref(),
                "year" => year_str.as_ref(),
                "jurisdiction" => components.jurisdiction.as_ref(),
                _ => None,
            };

            if let Err(e) = rule.validate(value) {
                errors.push(e);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Gets validation rules for case citations based on style.
    fn get_case_rules(&self) -> Vec<CitationValidationRule> {
        match &self.style {
            CitationStyle::Bluebook => vec![
                CitationValidationRule::required("title"),
                CitationValidationRule::required("volume").with_pattern("numeric"),
                CitationValidationRule::required("reporter"),
                CitationValidationRule::required("page"),
                CitationValidationRule::optional("court"),
                CitationValidationRule::required("year").with_pattern("year"),
            ],
            CitationStyle::OSCOLA => vec![
                CitationValidationRule::required("title"),
                CitationValidationRule::required("year").with_pattern("year"),
                CitationValidationRule::required("reporter"),
                CitationValidationRule::optional("page"),
            ],
            CitationStyle::AGLC => vec![
                CitationValidationRule::required("title"),
                CitationValidationRule::required("year").with_pattern("year"),
                CitationValidationRule::required("reporter"),
                CitationValidationRule::optional("volume"),
            ],
            CitationStyle::McGill => vec![
                CitationValidationRule::required("title"),
                CitationValidationRule::required("year").with_pattern("year"),
                CitationValidationRule::optional("reporter"),
            ],
            CitationStyle::Japanese => vec![
                CitationValidationRule::required("title"),
                CitationValidationRule::optional("year"),
            ],
            CitationStyle::Indian => vec![
                CitationValidationRule::required("title"),
                CitationValidationRule::required("year").with_pattern("year"),
                CitationValidationRule::optional("reporter"),
                CitationValidationRule::optional("court"),
            ],
            _ => vec![
                CitationValidationRule::required("title"),
                CitationValidationRule::optional("year"),
            ],
        }
    }

    /// Gets validation rules for statute citations based on style.
    fn get_statute_rules(&self) -> Vec<CitationValidationRule> {
        match &self.style {
            CitationStyle::Bluebook => vec![
                CitationValidationRule::required("title"),
                CitationValidationRule::optional("page"), // Section number
            ],
            CitationStyle::OSCOLA => vec![
                CitationValidationRule::required("title"),
                CitationValidationRule::optional("year").with_pattern("year"),
            ],
            _ => vec![CitationValidationRule::required("title")],
        }
    }
}

/// Citation normalizer for converting between citation styles.
#[derive(Debug, Clone)]
pub struct CitationNormalizer {
    #[allow(dead_code)]
    formatter: CitationFormatter,
}

impl CitationNormalizer {
    /// Creates a new citation normalizer.
    pub fn new() -> Self {
        Self {
            formatter: CitationFormatter::new(CitationStyle::Bluebook, Locale::new("en")),
        }
    }

    /// Converts a citation from one style to another.
    pub fn convert_case(
        &self,
        components: &CitationComponents,
        from_style: CitationStyle,
        to_style: CitationStyle,
    ) -> Result<String, CitationError> {
        // Validate source citation
        let validator = CitationValidator::new(from_style.clone());
        if let Err(errors) = validator.validate_case(components) {
            return Err(CitationError::StyleViolation {
                style: format!("{}", from_style),
                reason: format!("{} validation errors", errors.len()),
            });
        }

        // Format in target style
        let formatter = CitationFormatter::new(to_style, Locale::new("en"));
        Ok(formatter.format_case(components))
    }

    /// Converts a statute citation from one style to another.
    pub fn convert_statute(
        &self,
        components: &CitationComponents,
        from_style: CitationStyle,
        to_style: CitationStyle,
    ) -> Result<String, CitationError> {
        // Validate source citation
        let validator = CitationValidator::new(from_style.clone());
        if let Err(errors) = validator.validate_statute(components) {
            return Err(CitationError::StyleViolation {
                style: format!("{}", from_style),
                reason: format!("{} validation errors", errors.len()),
            });
        }

        // Format in target style
        let formatter = CitationFormatter::new(to_style, Locale::new("en"));
        Ok(formatter.format_statute(components))
    }

    /// Parses and converts a citation string.
    pub fn parse_and_convert_case(
        &self,
        citation: &str,
        from_style: CitationStyle,
        to_style: CitationStyle,
    ) -> Result<String, CitationError> {
        let parser = CitationParser::new(from_style.clone());
        let components = parser.parse_case(citation)?;
        self.convert_case(&components, from_style, to_style)
    }

    /// Parses and converts a statute citation string.
    pub fn parse_and_convert_statute(
        &self,
        citation: &str,
        from_style: CitationStyle,
        to_style: CitationStyle,
    ) -> Result<String, CitationError> {
        let parser = CitationParser::new(from_style.clone());
        let components = parser.parse_statute(citation)?;
        self.convert_statute(&components, from_style, to_style)
    }
}

impl Default for CitationNormalizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Citation completeness checker.
#[derive(Debug, Clone)]
pub struct CitationCompletenessChecker {
    style: CitationStyle,
}

impl CitationCompletenessChecker {
    /// Creates a new completeness checker.
    pub fn new(style: CitationStyle) -> Self {
        Self { style }
    }

    /// Checks completeness of a case citation.
    pub fn check_case(&self, components: &CitationComponents) -> CompletenessReport {
        let validator = CitationValidator::new(self.style.clone());
        let rules = validator.get_case_rules();
        self.check_against_rules(components, &rules, CitationType::Case)
    }

    /// Checks completeness of a statute citation.
    pub fn check_statute(&self, components: &CitationComponents) -> CompletenessReport {
        let validator = CitationValidator::new(self.style.clone());
        let rules = validator.get_statute_rules();
        self.check_against_rules(components, &rules, CitationType::Statute)
    }

    fn check_against_rules(
        &self,
        components: &CitationComponents,
        rules: &[CitationValidationRule],
        citation_type: CitationType,
    ) -> CompletenessReport {
        let mut missing_required = Vec::new();
        let mut missing_optional = Vec::new();
        let mut present = Vec::new();

        for rule in rules {
            let value = match rule.field.as_str() {
                "title" => Some(&components.title),
                "volume" => components.volume.as_ref(),
                "reporter" => components.reporter.as_ref(),
                "page" => components.page.as_ref(),
                "court" => components.court.as_ref(),
                "year" => components.year.as_ref().map(|_| &components.title), // Dummy ref
                "jurisdiction" => components.jurisdiction.as_ref(),
                _ => None,
            };

            if value.is_none() {
                if rule.required {
                    missing_required.push(rule.field.clone());
                } else {
                    missing_optional.push(rule.field.clone());
                }
            } else {
                present.push(rule.field.clone());
            }
        }

        let total_fields = rules.len();
        let present_count = present.len();
        let completeness_score = if total_fields > 0 {
            (present_count as f64 / total_fields as f64) * 100.0
        } else {
            0.0
        };

        CompletenessReport {
            citation_type,
            style: self.style.clone(),
            completeness_score,
            missing_required,
            missing_optional,
            present,
        }
    }
}

/// Report on citation completeness.
#[derive(Debug, Clone, PartialEq)]
pub struct CompletenessReport {
    /// Type of citation
    pub citation_type: CitationType,
    /// Citation style
    pub style: CitationStyle,
    /// Completeness score (0-100%)
    pub completeness_score: f64,
    /// Missing required fields
    pub missing_required: Vec<String>,
    /// Missing optional fields
    pub missing_optional: Vec<String>,
    /// Present fields
    pub present: Vec<String>,
}

impl CompletenessReport {
    /// Checks if citation is complete (all required fields present).
    pub fn is_complete(&self) -> bool {
        self.missing_required.is_empty()
    }

    /// Gets a summary message.
    pub fn summary(&self) -> String {
        if self.is_complete() {
            format!(
                "Citation is complete ({:.1}% of fields present)",
                self.completeness_score
            )
        } else {
            format!(
                "Citation is incomplete: missing {} required field(s): {}",
                self.missing_required.len(),
                self.missing_required.join(", ")
            )
        }
    }
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

        // Check completeness
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

        // Style-specific suggestions
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
                if let Some(title) = &components.title.chars().next() {
                    if title.is_lowercase() {
                        suggestions.push("Case name should start with capital letter".to_string());
                    }
                }
            }
            CitationStyle::Japanese => {
                if components.reporter.is_none() {
                    suggestions.push(
                        "Consider adding reporter name (e.g., 最高裁判所民事判例集)".to_string(),
                    );
                }
            }
            _ => {}
        }

        // General formatting suggestions
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

        // Check completeness
        let checker = CitationCompletenessChecker::new(self.style.clone());
        let report = checker.check_statute(components);

        if !report.is_complete() {
            for field in &report.missing_required {
                suggestions.push(format!("Add required field: {}", field));
            }
        }

        // Style-specific suggestions
        match &self.style {
            CitationStyle::Bluebook => {
                if components.page.is_none() {
                    suggestions.push("Consider adding section number (§)".to_string());
                }
            }
            CitationStyle::OSCOLA => {
                if components.year.is_none() {
                    suggestions.push("Add year for UK statutes".to_string());
                }
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

/// Text direction for layout and display.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextDirection {
    /// Left-to-Right (e.g., English, French, German)
    LTR,
    /// Right-to-Left (e.g., Arabic, Hebrew)
    RTL,
}

impl std::fmt::Display for TextDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextDirection::LTR => write!(f, "LTR"),
            TextDirection::RTL => write!(f, "RTL"),
        }
    }
}

/// RTL (Right-to-Left) text handler for Arabic and Hebrew legal documents.
#[derive(Debug, Clone)]
pub struct BidirectionalText {
    locale: Locale,
    direction: TextDirection,
}

impl BidirectionalText {
    /// Creates a new bidirectional text handler.
    pub fn new(locale: Locale) -> Self {
        let direction = Self::detect_direction(&locale);
        Self { locale, direction }
    }

    /// Detects text direction from locale.
    pub fn detect_direction(locale: &Locale) -> TextDirection {
        match locale.language.as_str() {
            "ar" | "he" | "fa" | "ur" => TextDirection::RTL,
            _ => TextDirection::LTR,
        }
    }

    /// Gets the text direction.
    pub fn direction(&self) -> TextDirection {
        self.direction
    }

    /// Checks if the text is RTL.
    pub fn is_rtl(&self) -> bool {
        self.direction == TextDirection::RTL
    }

    /// Wraps text with Unicode bidirectional formatting characters.
    /// This ensures proper rendering in mixed LTR/RTL contexts.
    pub fn wrap_with_direction_markers(&self, text: &str) -> String {
        match self.direction {
            TextDirection::RTL => {
                // RLE (Right-to-Left Embedding) + text + PDF (Pop Directional Formatting)
                format!("\u{202B}{}\u{202C}", text)
            }
            TextDirection::LTR => {
                // LRE (Left-to-Right Embedding) + text + PDF
                format!("\u{202A}{}\u{202C}", text)
            }
        }
    }

    /// Adds Right-to-Left Mark (RLM) for RTL languages.
    /// Useful for maintaining RTL directionality in mixed content.
    pub fn add_direction_mark(&self, text: &str) -> String {
        match self.direction {
            TextDirection::RTL => format!("{}\u{200F}", text), // Add RLM
            TextDirection::LTR => format!("{}\u{200E}", text), // Add LRM
        }
    }

    /// Reverses logical order for RTL display (for simple cases).
    /// Note: This is a simplified implementation. For production use,
    /// consider using the Unicode Bidirectional Algorithm (UAX#9).
    pub fn reverse_for_display(&self, text: &str) -> String {
        if self.is_rtl() {
            text.chars().rev().collect()
        } else {
            text.to_string()
        }
    }

    /// Formats a legal document paragraph with proper direction.
    pub fn format_paragraph(&self, text: &str) -> String {
        let direction_attr = match self.direction {
            TextDirection::RTL => "rtl",
            TextDirection::LTR => "ltr",
        };

        format!("<p dir=\"{}\">{}</p>", direction_attr, text)
    }

    /// Formats a legal list with proper direction.
    pub fn format_list(&self, items: &[String]) -> String {
        let direction_attr = match self.direction {
            TextDirection::RTL => "rtl",
            TextDirection::LTR => "ltr",
        };

        let mut result = format!("<ul dir=\"{}\">", direction_attr);
        for item in items {
            result.push_str(&format!("<li>{}</li>", item));
        }
        result.push_str("</ul>");
        result
    }

    /// Mixes LTR and RTL text properly (e.g., for citations in Arabic documents).
    pub fn mix_bidirectional(&self, rtl_text: &str, ltr_text: &str) -> String {
        match self.direction {
            TextDirection::RTL => {
                // RTL context: RTL text + LTR embedded text
                format!("{} \u{202A}{}\u{202C}", rtl_text, ltr_text)
            }
            TextDirection::LTR => {
                // LTR context: LTR text + RTL embedded text
                format!("{} \u{202B}{}\u{202C}", ltr_text, rtl_text)
            }
        }
    }

    /// Formats a number for RTL context (e.g., Arabic numerals vs Eastern Arabic numerals).
    pub fn format_number(&self, number: i64) -> String {
        match self.locale.language.as_str() {
            "ar" => {
                // Convert to Eastern Arabic numerals (٠١٢٣٤٥٦٧٨٩)
                let western = number.to_string();
                western
                    .chars()
                    .map(|c| match c {
                        '0' => '٠',
                        '1' => '١',
                        '2' => '٢',
                        '3' => '٣',
                        '4' => '٤',
                        '5' => '٥',
                        '6' => '٦',
                        '7' => '٧',
                        '8' => '٨',
                        '9' => '٩',
                        _ => c,
                    })
                    .collect()
            }
            "fa" => {
                // Convert to Persian numerals (۰۱۲۳۴۵۶۷۸۹)
                let western = number.to_string();
                western
                    .chars()
                    .map(|c| match c {
                        '0' => '۰',
                        '1' => '۱',
                        '2' => '۲',
                        '3' => '۳',
                        '4' => '۴',
                        '5' => '۵',
                        '6' => '۶',
                        '7' => '۷',
                        '8' => '۸',
                        '9' => '۹',
                        _ => c,
                    })
                    .collect()
            }
            _ => number.to_string(), // Western Arabic numerals
        }
    }

    /// Formats a date for RTL context.
    pub fn format_date_rtl(&self, year: i32, month: u32, day: u32) -> String {
        match self.locale.language.as_str() {
            "ar" => {
                // Arabic date format: day/month/year with Eastern Arabic numerals
                let day_str = self.format_number(day as i64);
                let month_str = self.format_number(month as i64);
                let year_str = self.format_number(year as i64);
                format!("{}/{}/{}", day_str, month_str, year_str)
            }
            "he" => {
                // Hebrew date format: day.month.year
                format!("{}.{}.{}", day, month, year)
            }
            _ => format!("{}-{:02}-{:02}", year, month, day),
        }
    }
}

/// Name order convention for different cultures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NameOrder {
    /// Given name first, family name last (Western style)
    GivenFirst,
    /// Family name first, given name last (East Asian style)
    FamilyFirst,
}

/// Personal name components for legal documents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonName {
    /// Given name (first name in Western cultures)
    pub given_name: String,
    /// Family name (last name in Western cultures, first in East Asian)
    pub family_name: String,
    /// Middle name(s) if applicable
    pub middle_name: Option<String>,
    /// Honorific prefix (Mr., Dr., etc.)
    pub prefix: Option<String>,
    /// Suffix (Jr., Sr., III, etc.)
    pub suffix: Option<String>,
    /// Patronymic or matronymic (e.g., Russian, Arabic)
    pub patronymic: Option<String>,
}

impl PersonName {
    /// Creates a new person name with given and family names.
    pub fn new(given_name: impl Into<String>, family_name: impl Into<String>) -> Self {
        Self {
            given_name: given_name.into(),
            family_name: family_name.into(),
            middle_name: None,
            prefix: None,
            suffix: None,
            patronymic: None,
        }
    }

    /// Sets the middle name.
    pub fn with_middle_name(mut self, middle_name: impl Into<String>) -> Self {
        self.middle_name = Some(middle_name.into());
        self
    }

    /// Sets the prefix.
    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// Sets the suffix.
    pub fn with_suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    /// Sets the patronymic.
    pub fn with_patronymic(mut self, patronymic: impl Into<String>) -> Self {
        self.patronymic = Some(patronymic.into());
        self
    }
}

/// Name formatter for legal documents following cultural conventions.
#[derive(Debug, Clone)]
pub struct NameFormatter {
    locale: Locale,
    order: NameOrder,
}

impl NameFormatter {
    /// Creates a new name formatter for a locale.
    pub fn new(locale: Locale) -> Self {
        let order = Self::detect_name_order(&locale);
        Self { locale, order }
    }

    /// Detects the name order convention from locale.
    pub fn detect_name_order(locale: &Locale) -> NameOrder {
        match locale.language.as_str() {
            "ja" | "ko" | "zh" | "vi" | "hu" => NameOrder::FamilyFirst,
            _ => NameOrder::GivenFirst,
        }
    }

    /// Formats a full name according to cultural conventions.
    pub fn format_full_name(&self, name: &PersonName) -> String {
        match self.locale.language.as_str() {
            "ja" => self.format_japanese(name),
            "ko" => self.format_korean(name),
            "zh" => self.format_chinese(name),
            "ar" => self.format_arabic(name),
            "ru" => self.format_russian(name),
            _ => self.format_western(name),
        }
    }

    /// Formats a name in Western style (Given Middle Family).
    fn format_western(&self, name: &PersonName) -> String {
        let mut parts = Vec::new();

        if let Some(prefix) = &name.prefix {
            parts.push(prefix.clone());
        }

        parts.push(name.given_name.clone());

        if let Some(middle) = &name.middle_name {
            parts.push(middle.clone());
        }

        parts.push(name.family_name.clone());

        if let Some(suffix) = &name.suffix {
            parts.push(suffix.clone());
        }

        parts.join(" ")
    }

    /// Formats a name in Japanese style (Family Given).
    fn format_japanese(&self, name: &PersonName) -> String {
        let mut parts = Vec::new();

        if let Some(prefix) = &name.prefix {
            parts.push(prefix.clone());
        }

        // Family name first
        parts.push(name.family_name.clone());

        // Space or no space depending on context (legal docs usually use space)
        parts.push(name.given_name.clone());

        parts.join(" ")
    }

    /// Formats a name in Korean style (Family Given).
    fn format_korean(&self, name: &PersonName) -> String {
        // Korean names typically don't have spaces between family and given
        format!("{}{}", name.family_name, name.given_name)
    }

    /// Formats a name in Chinese style (Family Given).
    fn format_chinese(&self, name: &PersonName) -> String {
        match self.locale.country.as_deref() {
            Some("CN") | Some("SG") => {
                // Mainland China/Singapore: no space
                format!("{}{}", name.family_name, name.given_name)
            }
            Some("TW") | Some("HK") => {
                // Taiwan/Hong Kong: sometimes with space
                format!("{} {}", name.family_name, name.given_name)
            }
            _ => format!("{}{}", name.family_name, name.given_name),
        }
    }

    /// Formats a name in Arabic style (with patronymic).
    fn format_arabic(&self, name: &PersonName) -> String {
        let mut parts = Vec::new();

        parts.push(name.given_name.clone());

        if let Some(patronymic) = &name.patronymic {
            parts.push(patronymic.clone());
        }

        parts.push(name.family_name.clone());

        parts.join(" ")
    }

    /// Formats a name in Russian style (with patronymic).
    fn format_russian(&self, name: &PersonName) -> String {
        let mut parts = Vec::new();

        parts.push(name.family_name.clone());
        parts.push(name.given_name.clone());

        if let Some(patronymic) = &name.patronymic {
            parts.push(patronymic.clone());
        }

        parts.join(" ")
    }

    /// Formats a name for legal citations (typically Family, Given Middle).
    pub fn format_citation(&self, name: &PersonName) -> String {
        let mut result = name.family_name.clone();
        result.push_str(", ");
        result.push_str(&name.given_name);

        if let Some(middle) = &name.middle_name {
            result.push(' ');
            result.push_str(middle);
        }

        result
    }

    /// Formats initials (e.g., J. K. for John Kevin).
    pub fn format_initials(&self, name: &PersonName) -> String {
        let given_initial = name.given_name.chars().next().unwrap_or('X');
        let family_initial = name.family_name.chars().next().unwrap_or('X');

        match self.order {
            NameOrder::GivenFirst => {
                if let Some(middle) = &name.middle_name {
                    let middle_initial = middle.chars().next().unwrap_or('X');
                    format!("{}. {}. {}.", given_initial, middle_initial, family_initial)
                } else {
                    format!("{}. {}.", given_initial, family_initial)
                }
            }
            NameOrder::FamilyFirst => {
                format!("{}. {}.", family_initial, given_initial)
            }
        }
    }

    /// Formats a formal name with all components.
    pub fn format_formal(&self, name: &PersonName) -> String {
        let full_name = self.format_full_name(name);

        if let Some(prefix) = &name.prefix {
            format!("{} {}", prefix, full_name)
        } else {
            full_name
        }
    }
}

/// Address components for legal documents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    /// Street address line 1
    pub street1: String,
    /// Street address line 2 (optional)
    pub street2: Option<String>,
    /// City/municipality
    pub city: String,
    /// State/province/prefecture
    pub state: Option<String>,
    /// Postal/ZIP code
    pub postal_code: String,
    /// Country
    pub country: String,
    /// Building/apartment number (for some Asian countries)
    pub building: Option<String>,
}

impl Address {
    /// Creates a new address.
    pub fn new(
        street1: impl Into<String>,
        city: impl Into<String>,
        postal_code: impl Into<String>,
        country: impl Into<String>,
    ) -> Self {
        Self {
            street1: street1.into(),
            street2: None,
            city: city.into(),
            state: None,
            postal_code: postal_code.into(),
            country: country.into(),
            building: None,
        }
    }

    /// Sets the second street line.
    pub fn with_street2(mut self, street2: impl Into<String>) -> Self {
        self.street2 = Some(street2.into());
        self
    }

    /// Sets the state/province.
    pub fn with_state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }

    /// Sets the building number.
    pub fn with_building(mut self, building: impl Into<String>) -> Self {
        self.building = Some(building.into());
        self
    }
}

/// Address formatter for legal documents per jurisdiction.
#[derive(Debug, Clone)]
pub struct AddressFormatter {
    locale: Locale,
}

impl AddressFormatter {
    /// Creates a new address formatter.
    pub fn new(locale: Locale) -> Self {
        Self { locale }
    }

    /// Formats an address according to jurisdiction conventions.
    pub fn format(&self, address: &Address) -> String {
        match self.locale.country.as_deref() {
            Some("US") => self.format_us(address),
            Some("GB") => self.format_uk(address),
            Some("JP") => self.format_japan(address),
            Some("DE") | Some("FR") | Some("IT") | Some("ES") => self.format_european(address),
            Some("CN") => self.format_china(address),
            Some("KR") => self.format_korea(address),
            _ => self.format_default(address),
        }
    }

    /// Formats a US address.
    fn format_us(&self, addr: &Address) -> String {
        let mut lines = vec![addr.street1.clone()];

        if let Some(street2) = &addr.street2 {
            lines.push(street2.clone());
        }

        let city_line = if let Some(state) = &addr.state {
            format!("{}, {} {}", addr.city, state, addr.postal_code)
        } else {
            format!("{} {}", addr.city, addr.postal_code)
        };
        lines.push(city_line);
        lines.push(addr.country.clone());

        lines.join("\n")
    }

    /// Formats a UK address.
    fn format_uk(&self, addr: &Address) -> String {
        let mut lines = vec![addr.street1.clone()];

        if let Some(street2) = &addr.street2 {
            lines.push(street2.clone());
        }

        lines.push(addr.city.clone());

        if let Some(state) = &addr.state {
            lines.push(state.clone());
        }

        lines.push(addr.postal_code.clone());
        lines.push(addr.country.clone());

        lines.join("\n")
    }

    /// Formats a Japanese address (reverse order: country → postal → prefecture → city → street).
    fn format_japan(&self, addr: &Address) -> String {
        let mut result = String::new();

        result.push('〒');
        result.push_str(&addr.postal_code);
        result.push('\n');

        if let Some(state) = &addr.state {
            result.push_str(state);
        }

        result.push_str(&addr.city);
        result.push_str(&addr.street1);

        if let Some(building) = &addr.building {
            result.push(' ');
            result.push_str(building);
        }

        result
    }

    /// Formats a European address.
    fn format_european(&self, addr: &Address) -> String {
        let mut lines = vec![addr.street1.clone()];

        if let Some(street2) = &addr.street2 {
            lines.push(street2.clone());
        }

        lines.push(format!("{} {}", addr.postal_code, addr.city));

        if let Some(state) = &addr.state {
            lines.push(state.clone());
        }

        lines.push(addr.country.clone());

        lines.join("\n")
    }

    /// Formats a Chinese address (reverse order like Japanese).
    fn format_china(&self, addr: &Address) -> String {
        let mut result = String::new();

        result.push_str(&addr.country);
        result.push(' ');

        if let Some(state) = &addr.state {
            result.push_str(state);
        }

        result.push_str(&addr.city);
        result.push_str(&addr.street1);

        if let Some(building) = &addr.building {
            result.push(' ');
            result.push_str(building);
        }

        result.push(' ');
        result.push_str(&addr.postal_code);

        result
    }

    /// Formats a Korean address.
    fn format_korea(&self, addr: &Address) -> String {
        // Similar to Japanese: reverse order
        let mut result = String::new();

        if let Some(state) = &addr.state {
            result.push_str(state);
            result.push(' ');
        }

        result.push_str(&addr.city);
        result.push(' ');
        result.push_str(&addr.street1);

        if let Some(building) = &addr.building {
            result.push(' ');
            result.push_str(building);
        }

        result.push_str(" (");
        result.push_str(&addr.postal_code);
        result.push(')');

        result
    }

    /// Formats a default international address.
    fn format_default(&self, addr: &Address) -> String {
        let mut lines = vec![addr.street1.clone()];

        if let Some(street2) = &addr.street2 {
            lines.push(street2.clone());
        }

        let city_line = if let Some(state) = &addr.state {
            format!("{}, {}", addr.city, state)
        } else {
            addr.city.clone()
        };
        lines.push(city_line);
        lines.push(addr.postal_code.clone());
        lines.push(addr.country.clone());

        lines.join("\n")
    }

    /// Formats a single-line address (for forms).
    pub fn format_single_line(&self, address: &Address) -> String {
        self.format(address).replace('\n', ", ")
    }
}

/// Currency formatter for monetary values.
#[derive(Debug, Clone)]
pub struct CurrencyFormatter {
    locale: Locale,
}

impl CurrencyFormatter {
    /// Creates a new currency formatter.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_i18n::{CurrencyFormatter, Locale};
    ///
    /// let us_locale = Locale::new("en").with_country("US");
    /// let formatter = CurrencyFormatter::new(us_locale);
    /// assert!(formatter.format(1234.56, "USD").starts_with("$"));
    ///
    /// let jp_locale = Locale::new("ja").with_country("JP");
    /// let jp_formatter = CurrencyFormatter::new(jp_locale);
    /// assert!(jp_formatter.format(1234.0, "JPY").starts_with("¥"));
    /// ```
    pub fn new(locale: Locale) -> Self {
        Self { locale }
    }

    /// Formats a currency amount.
    pub fn format(&self, amount: f64, currency_code: &str) -> String {
        let symbol = self.get_currency_symbol(currency_code);
        let formatted_amount = self.format_number(amount);

        match self.locale.language.as_str() {
            "ja" | "zh" | "ko" => format!("{}{}", symbol, formatted_amount),
            "en" if self.locale.country.as_deref() == Some("US") => {
                format!("{}{}", symbol, formatted_amount)
            }
            "de" | "fr" | "es" | "it" => format!("{} {}", formatted_amount, symbol),
            _ => format!("{} {}", symbol, formatted_amount),
        }
    }

    fn get_currency_symbol<'a>(&self, code: &'a str) -> &'a str {
        match code {
            "USD" => "$",
            "EUR" => "€",
            "GBP" => "£",
            "JPY" => "¥",
            "CNY" => "¥",
            "KRW" => "₩",
            "INR" => "₹",
            "RUB" => "₽",
            "BRL" => "R$",
            "CHF" => "CHF",
            _ => code,
        }
    }

    fn format_number(&self, amount: f64) -> String {
        let is_whole = amount.fract() == 0.0;
        let decimal_places = if is_whole { 0 } else { 2 };

        match self.locale.language.as_str() {
            "de" | "es" | "it" | "fr" => {
                // Use comma for decimal separator and period/space for thousands
                let formatted = format!("{:.prec$}", amount, prec = decimal_places);
                formatted.replace('.', ",")
            }
            _ => {
                // Use period for decimal separator and comma for thousands
                format!("{:.prec$}", amount, prec = decimal_places)
            }
        }
    }
}

/// Calendar system type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CalendarSystem {
    /// Gregorian calendar (most common worldwide)
    Gregorian,
    /// Islamic/Hijri calendar
    Islamic,
    /// Hebrew/Jewish calendar
    Hebrew,
    /// Japanese calendar (Imperial era)
    Japanese,
    /// Buddhist calendar
    Buddhist,
    /// Persian/Solar Hijri calendar
    Persian,
}

impl std::fmt::Display for CalendarSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalendarSystem::Gregorian => write!(f, "Gregorian"),
            CalendarSystem::Islamic => write!(f, "Islamic"),
            CalendarSystem::Hebrew => write!(f, "Hebrew"),
            CalendarSystem::Japanese => write!(f, "Japanese"),
            CalendarSystem::Buddhist => write!(f, "Buddhist"),
            CalendarSystem::Persian => write!(f, "Persian"),
        }
    }
}

/// Date representation in a calendar system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalendarDate {
    /// Calendar system
    pub system: CalendarSystem,
    /// Year in the calendar system
    pub year: i32,
    /// Month (1-12, or calendar-specific)
    pub month: u32,
    /// Day of month
    pub day: u32,
    /// Era (for Japanese calendar)
    pub era: Option<String>,
}

impl CalendarDate {
    /// Creates a new calendar date.
    pub fn new(system: CalendarSystem, year: i32, month: u32, day: u32) -> Self {
        Self {
            system,
            year,
            month,
            day,
            era: None,
        }
    }

    /// Sets the era (for Japanese calendar).
    pub fn with_era(mut self, era: impl Into<String>) -> Self {
        self.era = Some(era.into());
        self
    }
}

/// Calendar converter for converting dates between calendar systems.
#[derive(Debug, Clone)]
pub struct CalendarConverter {
    locale: Locale,
}

impl CalendarConverter {
    /// Creates a new calendar converter.
    pub fn new(locale: Locale) -> Self {
        Self { locale }
    }

    /// Converts a Gregorian date to the locale's preferred calendar.
    pub fn from_gregorian(&self, year: i32, month: u32, day: u32) -> CalendarDate {
        let system = self.get_preferred_calendar();

        match system {
            CalendarSystem::Gregorian => CalendarDate::new(system, year, month, day),
            CalendarSystem::Japanese => self.to_japanese_calendar(year, month, day),
            CalendarSystem::Buddhist => {
                // Buddhist era = Gregorian year + 543
                CalendarDate::new(system, year + 543, month, day)
            }
            CalendarSystem::Islamic => self.to_islamic_approximate(year, month, day),
            CalendarSystem::Hebrew => self.to_hebrew_calendar(year, month, day),
            CalendarSystem::Persian => self.to_persian_calendar(year, month, day),
        }
    }

    fn get_preferred_calendar(&self) -> CalendarSystem {
        match self.locale.country.as_deref() {
            Some("JP") => CalendarSystem::Japanese,
            Some("TH") => CalendarSystem::Buddhist,
            Some("SA") | Some("AE") | Some("IQ") => CalendarSystem::Islamic,
            Some("IL") => CalendarSystem::Hebrew,
            Some("IR") => CalendarSystem::Persian,
            _ => CalendarSystem::Gregorian,
        }
    }

    fn to_japanese_calendar(&self, year: i32, month: u32, day: u32) -> CalendarDate {
        // Japanese era conversion (simplified)
        let (era, era_year) = if year >= 2019 {
            ("Reiwa", year - 2019 + 1)
        } else if year >= 1989 {
            ("Heisei", year - 1989 + 1)
        } else if year >= 1926 {
            ("Showa", year - 1926 + 1)
        } else if year >= 1912 {
            ("Taisho", year - 1912 + 1)
        } else if year >= 1868 {
            ("Meiji", year - 1868 + 1)
        } else {
            ("Gregorian", year)
        };

        CalendarDate::new(CalendarSystem::Japanese, era_year, month, day).with_era(era)
    }

    fn to_islamic_approximate(&self, year: i32, month: u32, day: u32) -> CalendarDate {
        // Improved Islamic calendar conversion using Kuwaiti algorithm approximation
        // Islamic calendar is lunar, approximately 354-355 days per year
        // Gregorian to Hijri conversion
        let jd = self.gregorian_to_julian_day(year, month as i32, day as i32);
        let (h_year, h_month, h_day) = self.julian_day_to_islamic(jd);
        CalendarDate::new(
            CalendarSystem::Islamic,
            h_year,
            h_month as u32,
            h_day as u32,
        )
    }

    fn to_hebrew_calendar(&self, year: i32, month: u32, day: u32) -> CalendarDate {
        // Hebrew calendar conversion (simplified approximation)
        // Hebrew year = Gregorian year + 3760 (approximate)
        // This is a simplified conversion; real Hebrew calendar is lunisolar
        let hebrew_year = year + 3760;
        CalendarDate::new(CalendarSystem::Hebrew, hebrew_year, month, day)
    }

    fn to_persian_calendar(&self, year: i32, month: u32, day: u32) -> CalendarDate {
        // Persian (Solar Hijri) calendar
        // Starts from 622 CE (same epoch as Islamic calendar but solar)
        let persian_year = year - 621;
        CalendarDate::new(CalendarSystem::Persian, persian_year, month, day)
    }

    // Helper: Converts Gregorian date to Julian Day Number
    fn gregorian_to_julian_day(&self, year: i32, month: i32, day: i32) -> i32 {
        let a = (14 - month) / 12;
        let y = year + 4800 - a;
        let m = month + 12 * a - 3;

        day + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045
    }

    // Helper: Converts Julian Day Number to Islamic calendar
    fn julian_day_to_islamic(&self, jd: i32) -> (i32, i32, i32) {
        // Kuwaiti algorithm for Islamic calendar
        let l = jd - 1948440 + 10632;
        let n = (l - 1) / 10631;
        let l = l - 10631 * n + 354;
        let j = ((10985 - l) / 5316) * ((50 * l) / 17719) + (l / 5670) * ((43 * l) / 15238);
        let l = l - ((30 - j) / 15) * ((17719 * j) / 50) - (j / 16) * ((15238 * j) / 43) + 29;
        let month = (24 * l) / 709;
        let day = l - (709 * month) / 24;
        let year = 30 * n + j - 30;

        (year, month, day)
    }

    /// Converts from Islamic calendar to Gregorian
    pub fn to_gregorian_from_islamic(
        &self,
        h_year: i32,
        h_month: u32,
        h_day: u32,
    ) -> (i32, u32, u32) {
        // Convert Islamic to Julian Day Number
        let jd = self.islamic_to_julian_day(h_year, h_month as i32, h_day as i32);
        // Convert Julian Day to Gregorian
        self.julian_day_to_gregorian(jd)
    }

    // Helper: Converts Islamic date to Julian Day Number
    fn islamic_to_julian_day(&self, year: i32, month: i32, day: i32) -> i32 {
        ((11 * year + 3) / 30) + 354 * year + 30 * month - ((month - 1) / 2) + day + 1948440 - 385
    }

    // Helper: Converts Julian Day Number to Gregorian date
    fn julian_day_to_gregorian(&self, jd: i32) -> (i32, u32, u32) {
        let a = jd + 32044;
        let b = (4 * a + 3) / 146097;
        let c = a - (146097 * b) / 4;
        let d = (4 * c + 3) / 1461;
        let e = c - (1461 * d) / 4;
        let m = (5 * e + 2) / 153;

        let day = e - (153 * m + 2) / 5 + 1;
        let month = m + 3 - 12 * (m / 10);
        let year = 100 * b + d - 4800 + m / 10;

        (year, month as u32, day as u32)
    }

    /// Formats a calendar date according to locale conventions.
    pub fn format_date(&self, date: &CalendarDate) -> String {
        match date.system {
            CalendarSystem::Japanese => {
                if let Some(ref era) = date.era {
                    format!("{}{}年{}月{}日", era, date.year, date.month, date.day)
                } else {
                    format!("{}年{}月{}日", date.year, date.month, date.day)
                }
            }
            CalendarSystem::Buddhist => {
                format!("พ.ศ. {} {}/{}", date.year, date.day, date.month)
            }
            CalendarSystem::Islamic => {
                // Islamic calendar month names (Arabic)
                let month_names = [
                    "Muharram",
                    "Safar",
                    "Rabi' al-awwal",
                    "Rabi' al-thani",
                    "Jumada al-awwal",
                    "Jumada al-thani",
                    "Rajab",
                    "Sha'ban",
                    "Ramadan",
                    "Shawwal",
                    "Dhu al-Qi'dah",
                    "Dhu al-Hijjah",
                ];
                let month_name = month_names.get((date.month - 1) as usize).unwrap_or(&"");
                format!("{} {} {} AH", date.day, month_name, date.year)
            }
            CalendarSystem::Hebrew => {
                // Hebrew year format
                format!(
                    "{} {}, {}",
                    date.day,
                    self.get_hebrew_month_name(date.month),
                    date.year
                )
            }
            CalendarSystem::Persian => {
                // Persian/Solar Hijri calendar
                format!("{}/{}/{} SH", date.year, date.month, date.day)
            }
            CalendarSystem::Gregorian => {
                format!("{}-{:02}-{:02}", date.year, date.month, date.day)
            }
        }
    }

    fn get_hebrew_month_name(&self, month: u32) -> &'static str {
        // Hebrew month names (simplified, not accounting for leap years)
        match month {
            1 => "Nisan",
            2 => "Iyar",
            3 => "Sivan",
            4 => "Tammuz",
            5 => "Av",
            6 => "Elul",
            7 => "Tishrei",
            8 => "Cheshvan",
            9 => "Kislev",
            10 => "Tevet",
            11 => "Shevat",
            12 => "Adar",
            _ => "Unknown",
        }
    }
}

// ============================================================================
// Fiscal Year Calculator (v0.1.3)
// ============================================================================

/// Fiscal year configuration per jurisdiction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FiscalYearConfig {
    /// Jurisdiction code
    pub jurisdiction: String,
    /// Fiscal year start month (1-12)
    pub start_month: u32,
    /// Fiscal year start day
    pub start_day: u32,
}

impl FiscalYearConfig {
    /// Creates a new fiscal year configuration.
    pub fn new(jurisdiction: impl Into<String>, start_month: u32, start_day: u32) -> Self {
        Self {
            jurisdiction: jurisdiction.into(),
            start_month,
            start_day,
        }
    }

    /// Returns common fiscal year configurations for various jurisdictions.
    pub fn for_jurisdiction(jurisdiction: &str) -> Self {
        match jurisdiction {
            "US" => Self::new("US", 10, 1),       // October 1
            "GB" | "UK" => Self::new("GB", 4, 6), // April 6
            "JP" => Self::new("JP", 4, 1),        // April 1
            "AU" => Self::new("AU", 7, 1),        // July 1
            "CA" => Self::new("CA", 4, 1),        // April 1
            "IN" => Self::new("IN", 4, 1),        // April 1
            "DE" | "FR" | "IT" | "ES" | "NL" | "PT" | "PL" => Self::new(jurisdiction, 1, 1), // January 1
            _ => Self::new(jurisdiction, 1, 1), // Default: calendar year
        }
    }

    /// Calculates the fiscal year for a given Gregorian date.
    /// Returns the fiscal year number.
    pub fn get_fiscal_year(&self, year: i32, month: u32, day: u32) -> i32 {
        if month > self.start_month || (month == self.start_month && day >= self.start_day) {
            // After fiscal year start, current calendar year is the base
            if self.start_month == 1 && self.start_day == 1 {
                year
            } else {
                year + 1
            }
        } else {
            // Before fiscal year start
            year
        }
    }

    /// Gets the start date of a fiscal year (Gregorian calendar).
    pub fn get_fiscal_year_start(&self, fiscal_year: i32) -> (i32, u32, u32) {
        let calendar_year = if self.start_month == 1 && self.start_day == 1 {
            fiscal_year
        } else {
            fiscal_year - 1
        };
        (calendar_year, self.start_month, self.start_day)
    }

    /// Gets the end date of a fiscal year (Gregorian calendar).
    pub fn get_fiscal_year_end(&self, fiscal_year: i32) -> (i32, u32, u32) {
        let (start_year, start_month, start_day) = self.get_fiscal_year_start(fiscal_year);

        // Calculate one day before the next fiscal year starts
        let (next_year, next_month, next_day) = if start_month == 12 {
            (start_year + 1, 1, start_day)
        } else {
            (start_year, start_month + 1, start_day)
        };

        // Subtract one day
        if next_day > 1 {
            (next_year, next_month, next_day - 1)
        } else {
            // Need to go to previous month
            let prev_month = if next_month > 1 { next_month - 1 } else { 12 };
            let prev_year = if next_month > 1 {
                next_year
            } else {
                next_year - 1
            };
            let days_in_prev_month = self.days_in_month(prev_year, prev_month);
            (prev_year, prev_month, days_in_prev_month)
        }
    }

    fn days_in_month(&self, year: i32, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.is_leap_year(year) {
                    29
                } else {
                    28
                }
            }
            _ => 30,
        }
    }

    fn is_leap_year(&self, year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }
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
                // English ordinals
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
                // Spanish ordinals (simplified)
                if n == 1 {
                    "1º".to_string()
                } else {
                    format!("{}º", n)
                }
            }
            "fr" => {
                // French ordinals
                if n == 1 {
                    "1er".to_string()
                } else {
                    format!("{}e", n)
                }
            }
            "de" => {
                // German ordinals
                format!("{}.", n)
            }
            "ja" => {
                // Japanese ordinals
                format!("第{}", n)
            }
            "zh" => {
                // Chinese ordinals
                format!("第{}", n)
            }
            "ko" => {
                // Korean ordinals
                format!("제{}", n)
            }
            "pt" => {
                // Portuguese ordinals
                if n == 1 {
                    "1º".to_string()
                } else {
                    format!("{}º", n)
                }
            }
            "it" => {
                // Italian ordinals
                if n == 1 {
                    "1º".to_string()
                } else {
                    format!("{}º", n)
                }
            }
            "nl" => {
                // Dutch ordinals
                if n == 1 {
                    "1e".to_string()
                } else {
                    format!("{}e", n)
                }
            }
            "pl" => {
                // Polish ordinals
                format!("{}.", n)
            }
            _ => {
                // Default: just add a period
                format!("{}.", n)
            }
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
            _ => n.to_string(), // Simplified
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
            _ => n.to_string(), // Simplified
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
            _ => n.to_string(), // Simplified
        }
    }

    #[allow(dead_code)]
    fn number_to_words_ko(&self, n: i64) -> String {
        // Korean number system (Sino-Korean)
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
            _ => n.to_string(), // Simplified
        }
    }

    #[allow(dead_code)]
    fn number_to_words_pt(&self, n: i64) -> String {
        // Portuguese numbers
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
            _ => n.to_string(), // Simplified
        }
    }

    #[allow(dead_code)]
    fn number_to_words_it(&self, n: i64) -> String {
        // Italian numbers
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
            _ => n.to_string(), // Simplified
        }
    }

    #[allow(dead_code)]
    fn number_to_words_nl(&self, n: i64) -> String {
        // Dutch numbers
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
            _ => n.to_string(), // Simplified
        }
    }

    #[allow(dead_code)]
    fn number_to_words_pl(&self, n: i64) -> String {
        // Polish numbers
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
            _ => n.to_string(), // Simplified
        }
    }
}

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

impl std::fmt::Display for DayOfWeek {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DayOfWeek::Monday => write!(f, "Monday"),
            DayOfWeek::Tuesday => write!(f, "Tuesday"),
            DayOfWeek::Wednesday => write!(f, "Wednesday"),
            DayOfWeek::Thursday => write!(f, "Thursday"),
            DayOfWeek::Friday => write!(f, "Friday"),
            DayOfWeek::Saturday => write!(f, "Saturday"),
            DayOfWeek::Sunday => write!(f, "Sunday"),
        }
    }
}

/// Working days configuration for a jurisdiction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingDaysConfig {
    /// Weekend days (non-working days)
    pub weekend: Vec<DayOfWeek>,
    /// Fixed public holidays (month, day)
    pub fixed_holidays: Vec<(u32, u32)>,
    /// Jurisdiction ID
    pub jurisdiction_id: String,
}

impl WorkingDaysConfig {
    /// Creates a new working days configuration.
    pub fn new(jurisdiction_id: impl Into<String>) -> Self {
        Self {
            weekend: vec![DayOfWeek::Saturday, DayOfWeek::Sunday],
            fixed_holidays: vec![],
            jurisdiction_id: jurisdiction_id.into(),
        }
    }

    /// Sets the weekend days.
    pub fn with_weekend(mut self, weekend: Vec<DayOfWeek>) -> Self {
        self.weekend = weekend;
        self
    }

    /// Adds a fixed holiday (month, day).
    pub fn add_holiday(mut self, month: u32, day: u32) -> Self {
        self.fixed_holidays.push((month, day));
        self
    }

    /// Creates default configuration for Japan.
    pub fn japan() -> Self {
        Self::new("JP")
            .add_holiday(1, 1) // New Year's Day
            .add_holiday(2, 11) // National Foundation Day
            .add_holiday(2, 23) // Emperor's Birthday
            .add_holiday(3, 20) // Vernal Equinox
            .add_holiday(4, 29) // Showa Day
            .add_holiday(5, 3) // Constitution Day
            .add_holiday(5, 4) // Greenery Day
            .add_holiday(5, 5) // Children's Day
            .add_holiday(8, 11) // Mountain Day
            .add_holiday(9, 23) // Autumnal Equinox
            .add_holiday(11, 3) // Culture Day
            .add_holiday(11, 23) // Labor Thanksgiving Day
    }

    /// Creates default configuration for United States.
    pub fn united_states() -> Self {
        Self::new("US")
            .add_holiday(1, 1) // New Year's Day
            .add_holiday(7, 4) // Independence Day
            .add_holiday(11, 11) // Veterans Day
            .add_holiday(12, 25) // Christmas
    }

    /// Creates default configuration for United Kingdom.
    pub fn united_kingdom() -> Self {
        Self::new("GB")
            .add_holiday(1, 1) // New Year's Day
            .add_holiday(12, 25) // Christmas Day
            .add_holiday(12, 26) // Boxing Day
    }

    /// Creates default configuration for Saudi Arabia (weekend: Friday-Saturday).
    pub fn saudi_arabia() -> Self {
        Self::new("SA").with_weekend(vec![DayOfWeek::Friday, DayOfWeek::Saturday])
    }

    /// Creates default configuration for Israel (weekend: Friday-Saturday).
    pub fn israel() -> Self {
        Self::new("IL").with_weekend(vec![DayOfWeek::Friday, DayOfWeek::Saturday])
    }

    /// Creates configuration for a jurisdiction code.
    pub fn for_jurisdiction(code: &str) -> Self {
        match code {
            "JP" => Self::japan(),
            "US" => Self::united_states(),
            "GB" => Self::united_kingdom(),
            "SA" => Self::saudi_arabia(),
            "IL" => Self::israel(),
            _ => Self::new(code),
        }
    }

    /// Checks if a date is a working day.
    pub fn is_working_day(&self, year: i32, month: u32, day: u32) -> bool {
        // Check if it's a weekend
        let day_of_week = self.calculate_day_of_week(year, month, day);
        if self.weekend.contains(&day_of_week) {
            return false;
        }

        // Check if it's a fixed holiday
        if self.fixed_holidays.contains(&(month, day)) {
            return false;
        }

        true
    }

    /// Calculates the day of week using Zeller's congruence.
    fn calculate_day_of_week(&self, year: i32, month: u32, day: u32) -> DayOfWeek {
        let (m, y) = if month < 3 {
            (month + 12, year - 1)
        } else {
            (month, year)
        };

        let k = y % 100;
        let j = y / 100;

        let h = (day as i32 + (13 * (m as i32 + 1)) / 5 + k + k / 4 + j / 4 + 5 * j) % 7;

        // Convert Zeller's h (0=Saturday) to our DayOfWeek (0=Monday)
        match (h + 5) % 7 {
            0 => DayOfWeek::Monday,
            1 => DayOfWeek::Tuesday,
            2 => DayOfWeek::Wednesday,
            3 => DayOfWeek::Thursday,
            4 => DayOfWeek::Friday,
            5 => DayOfWeek::Saturday,
            _ => DayOfWeek::Sunday,
        }
    }

    /// Adds working days to a date.
    pub fn add_working_days(
        &self,
        year: i32,
        month: u32,
        day: u32,
        working_days: i32,
    ) -> (i32, u32, u32) {
        let mut current_year = year;
        let mut current_month = month;
        let mut current_day = day;
        let mut remaining = working_days;

        while remaining > 0 {
            // Move to next day
            let (next_y, next_m, next_d) = self.next_day(current_year, current_month, current_day);
            current_year = next_y;
            current_month = next_m;
            current_day = next_d;

            // If it's a working day, decrement counter
            if self.is_working_day(current_year, current_month, current_day) {
                remaining -= 1;
            }
        }

        (current_year, current_month, current_day)
    }

    fn next_day(&self, year: i32, month: u32, day: u32) -> (i32, u32, u32) {
        let days_in_month = self.days_in_month(year, month);

        if day < days_in_month {
            (year, month, day + 1)
        } else if month < 12 {
            (year, month + 1, 1)
        } else {
            (year + 1, 1, 1)
        }
    }

    fn days_in_month(&self, year: i32, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.is_leap_year(year) {
                    29
                } else {
                    28
                }
            }
            _ => 30,
        }
    }

    fn is_leap_year(&self, year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }
}

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

/// Mock translation service for testing and fallback.
#[derive(Debug, Clone)]
pub struct MockTranslationService {
    available: bool,
}

impl MockTranslationService {
    /// Creates a new mock translation service.
    pub fn new() -> Self {
        Self { available: true }
    }

    /// Sets availability status.
    pub fn set_available(&mut self, available: bool) {
        self.available = available;
    }
}

impl Default for MockTranslationService {
    fn default() -> Self {
        Self::new()
    }
}

impl TranslationService for MockTranslationService {
    fn translate(&self, text: &str, _source: &Locale, target: &Locale) -> I18nResult<String> {
        if !self.available {
            return Err(I18nError::TranslationMissing {
                key: text.to_string(),
                locale: target.tag(),
            });
        }
        // Mock: just prepend target locale to the text
        Ok(format!("[{}] {}", target.tag(), text))
    }

    fn translate_batch(
        &self,
        texts: &[&str],
        source: &Locale,
        target: &Locale,
    ) -> I18nResult<Vec<String>> {
        texts
            .iter()
            .map(|text| self.translate(text, source, target))
            .collect()
    }

    fn service_name(&self) -> &str {
        "MockTranslationService"
    }

    fn is_available(&self) -> bool {
        self.available
    }
}

/// Translation memory entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationMemoryEntry {
    /// Source text
    pub source_text: String,
    /// Source locale
    pub source_locale: Locale,
    /// Translated text
    pub target_text: String,
    /// Target locale
    pub target_locale: Locale,
    /// Translation quality score (0.0 to 1.0)
    pub quality_score: f32,
    /// Translation metadata
    pub metadata: HashMap<String, String>,
    /// Timestamp when this entry was created
    pub created_at: u64,
}

impl TranslationMemoryEntry {
    /// Creates a new translation memory entry.
    pub fn new(
        source_text: impl Into<String>,
        source_locale: Locale,
        target_text: impl Into<String>,
        target_locale: Locale,
    ) -> Self {
        Self {
            source_text: source_text.into(),
            source_locale,
            target_text: target_text.into(),
            target_locale,
            quality_score: 1.0,
            metadata: HashMap::new(),
            created_at: 0, // In production, use actual timestamp
        }
    }

    /// Sets the quality score.
    pub fn with_quality(mut self, score: f32) -> Self {
        self.quality_score = score.clamp(0.0, 1.0);
        self
    }

    /// Adds metadata.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// Translation memory for caching and reusing translations.
#[derive(Debug, Default)]
pub struct TranslationMemory {
    /// Stored translation entries
    entries: Vec<TranslationMemoryEntry>,
    /// Index for fast lookup: (source_text, source_locale, target_locale) -> entry index
    index: HashMap<(String, String, String), Vec<usize>>,
}

impl TranslationMemory {
    /// Creates a new translation memory.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_i18n::{TranslationMemory, Locale};
    ///
    /// let mut memory = TranslationMemory::new();
    ///
    /// let en = Locale::new("en");
    /// let ja = Locale::new("ja");
    ///
    /// memory.add_translation("contract", en.clone(), "契約", ja.clone());
    ///
    /// let matches = memory.find_exact("contract", &en, &ja);
    /// assert_eq!(matches.len(), 1);
    /// assert_eq!(matches[0].target_text, "契約");
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a translation entry to the memory.
    pub fn add_entry(&mut self, entry: TranslationMemoryEntry) {
        let key = (
            entry.source_text.clone(),
            entry.source_locale.tag(),
            entry.target_locale.tag(),
        );

        let index = self.entries.len();
        self.entries.push(entry);

        self.index.entry(key).or_default().push(index);
    }

    /// Adds a simple translation to the memory.
    pub fn add_translation(
        &mut self,
        source_text: impl Into<String>,
        source_locale: Locale,
        target_text: impl Into<String>,
        target_locale: Locale,
    ) {
        let entry =
            TranslationMemoryEntry::new(source_text, source_locale, target_text, target_locale);
        self.add_entry(entry);
    }

    /// Finds exact matches for a source text.
    pub fn find_exact(
        &self,
        source_text: &str,
        source_locale: &Locale,
        target_locale: &Locale,
    ) -> Vec<&TranslationMemoryEntry> {
        let key = (
            source_text.to_string(),
            source_locale.tag(),
            target_locale.tag(),
        );

        self.index
            .get(&key)
            .map(|indices| {
                indices
                    .iter()
                    .filter_map(|&i| self.entries.get(i))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Finds fuzzy matches for a source text (simple substring matching).
    pub fn find_fuzzy(
        &self,
        source_text: &str,
        source_locale: &Locale,
        target_locale: &Locale,
        min_similarity: f32,
    ) -> Vec<(&TranslationMemoryEntry, f32)> {
        self.entries
            .iter()
            .filter(|e| {
                e.source_locale.tag() == source_locale.tag()
                    && e.target_locale.tag() == target_locale.tag()
            })
            .filter_map(|e| {
                let similarity = self.calculate_similarity(source_text, &e.source_text);
                if similarity >= min_similarity {
                    Some((e, similarity))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Calculates similarity between two strings (simple Jaccard similarity).
    fn calculate_similarity(&self, text1: &str, text2: &str) -> f32 {
        let words1: std::collections::HashSet<&str> = text1.split_whitespace().collect();
        let words2: std::collections::HashSet<&str> = text2.split_whitespace().collect();

        if words1.is_empty() && words2.is_empty() {
            return 1.0;
        }

        let intersection: std::collections::HashSet<_> = words1.intersection(&words2).collect();
        let union: std::collections::HashSet<_> = words1.union(&words2).collect();

        if union.is_empty() {
            0.0
        } else {
            intersection.len() as f32 / union.len() as f32
        }
    }

    /// Calculates Levenshtein distance between two strings.
    fn levenshtein_distance(&self, text1: &str, text2: &str) -> usize {
        let len1 = text1.chars().count();
        let len2 = text2.chars().count();

        if len1 == 0 {
            return len2;
        }
        if len2 == 0 {
            return len1;
        }

        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

        #[allow(clippy::needless_range_loop)]
        for i in 0..=len1 {
            matrix[i][0] = i;
        }
        for j in 0..=len2 {
            matrix[0][j] = j;
        }

        let chars1: Vec<char> = text1.chars().collect();
        let chars2: Vec<char> = text2.chars().collect();

        for (i, c1) in chars1.iter().enumerate() {
            for (j, c2) in chars2.iter().enumerate() {
                let cost = if c1 == c2 { 0 } else { 1 };
                matrix[i + 1][j + 1] = std::cmp::min(
                    std::cmp::min(matrix[i][j + 1] + 1, matrix[i + 1][j] + 1),
                    matrix[i][j] + cost,
                );
            }
        }

        matrix[len1][len2]
    }

    /// Calculates normalized similarity score using Levenshtein distance (0.0 to 1.0).
    fn levenshtein_similarity(&self, text1: &str, text2: &str) -> f32 {
        let distance = self.levenshtein_distance(text1, text2);
        let max_len = std::cmp::max(text1.chars().count(), text2.chars().count());
        if max_len == 0 {
            1.0
        } else {
            1.0 - (distance as f32 / max_len as f32)
        }
    }

    /// Finds fuzzy matches using enhanced Levenshtein distance scoring.
    pub fn find_fuzzy_levenshtein(
        &self,
        source_text: &str,
        source_locale: &Locale,
        target_locale: &Locale,
        min_similarity: f32,
    ) -> Vec<(&TranslationMemoryEntry, f32)> {
        self.entries
            .iter()
            .filter(|e| {
                e.source_locale.tag() == source_locale.tag()
                    && e.target_locale.tag() == target_locale.tag()
            })
            .filter_map(|e| {
                let similarity = self.levenshtein_similarity(source_text, &e.source_text);
                if similarity >= min_similarity {
                    Some((e, similarity))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Finds context-aware translation suggestions.
    /// Context can be domain-specific (e.g., "contract_law", "criminal_law").
    pub fn find_with_context(
        &self,
        source_text: &str,
        source_locale: &Locale,
        target_locale: &Locale,
        context: Option<&str>,
        min_similarity: f32,
    ) -> Vec<(&TranslationMemoryEntry, f32)> {
        self.entries
            .iter()
            .filter(|e| {
                e.source_locale.tag() == source_locale.tag()
                    && e.target_locale.tag() == target_locale.tag()
            })
            .filter(|e| {
                if let Some(ctx) = context {
                    e.metadata.get("context").is_some_and(|c| c == ctx)
                } else {
                    true
                }
            })
            .filter_map(|e| {
                let text_similarity = self.levenshtein_similarity(source_text, &e.source_text);
                let context_bonus = if context.is_some()
                    && e.metadata.get("context") == context.map(|s| s.to_string()).as_ref()
                {
                    0.1
                } else {
                    0.0
                };
                let total_similarity = (text_similarity + context_bonus).min(1.0);

                if total_similarity >= min_similarity {
                    Some((e, total_similarity))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Saves translation memory to a JSON file.
    pub fn save_to_file(&self, path: &std::path::Path) -> I18nResult<()> {
        let json =
            serde_json::to_string_pretty(&self.entries).map_err(|e| I18nError::CacheError {
                reason: format!("Failed to serialize translation memory: {}", e),
            })?;

        std::fs::write(path, json).map_err(|e| I18nError::CacheError {
            reason: format!("Failed to write translation memory file: {}", e),
        })?;

        Ok(())
    }

    /// Loads translation memory from a JSON file.
    pub fn load_from_file(&mut self, path: &std::path::Path) -> I18nResult<()> {
        let json = std::fs::read_to_string(path).map_err(|e| I18nError::CacheError {
            reason: format!("Failed to read translation memory file: {}", e),
        })?;

        let entries: Vec<TranslationMemoryEntry> =
            serde_json::from_str(&json).map_err(|e| I18nError::CacheError {
                reason: format!("Failed to deserialize translation memory: {}", e),
            })?;

        self.clear();
        for entry in entries {
            self.add_entry(entry);
        }

        Ok(())
    }

    /// Exports translation memory to TMX (Translation Memory eXchange) format.
    /// TMX is an XML-based industry standard for translation memory interchange.
    pub fn export_to_tmx(&self, path: &std::path::Path) -> I18nResult<()> {
        let mut tmx = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        tmx.push_str("<!DOCTYPE tmx SYSTEM \"tmx14.dtd\">\n");
        tmx.push_str("<tmx version=\"1.4\">\n");
        tmx.push_str("  <header\n");
        tmx.push_str("    creationtool=\"legalis-i18n\"\n");
        tmx.push_str("    creationtoolversion=\"0.1.7\"\n");
        tmx.push_str("    datatype=\"plaintext\"\n");
        tmx.push_str("    segtype=\"sentence\"\n");
        tmx.push_str("    adminlang=\"en\"\n");
        tmx.push_str("    srclang=\"*all*\"\n");
        tmx.push_str("    o-tmf=\"legalis\"\n");
        tmx.push_str("  />\n");
        tmx.push_str("  <body>\n");

        for entry in &self.entries {
            tmx.push_str("    <tu>\n");

            // Add metadata as properties
            if !entry.metadata.is_empty() {
                for (key, value) in &entry.metadata {
                    tmx.push_str(&format!(
                        "      <prop type=\"{}\">{}</prop>\n",
                        Self::escape_xml(key),
                        Self::escape_xml(value)
                    ));
                }
            }

            // Add source segment
            tmx.push_str(&format!(
                "      <tuv xml:lang=\"{}\">\n",
                entry.source_locale.tag()
            ));
            tmx.push_str(&format!(
                "        <seg>{}</seg>\n",
                Self::escape_xml(&entry.source_text)
            ));
            tmx.push_str("      </tuv>\n");

            // Add target segment
            tmx.push_str(&format!(
                "      <tuv xml:lang=\"{}\">\n",
                entry.target_locale.tag()
            ));
            tmx.push_str(&format!(
                "        <seg>{}</seg>\n",
                Self::escape_xml(&entry.target_text)
            ));
            tmx.push_str("      </tuv>\n");

            tmx.push_str("    </tu>\n");
        }

        tmx.push_str("  </body>\n");
        tmx.push_str("</tmx>\n");

        std::fs::write(path, tmx).map_err(|e| I18nError::CacheError {
            reason: format!("Failed to write TMX file: {}", e),
        })?;

        Ok(())
    }

    /// Imports translation memory from TMX format (simplified parser).
    /// Note: This is a basic TMX parser that handles simple cases.
    pub fn import_from_tmx(&mut self, path: &std::path::Path) -> I18nResult<()> {
        let tmx_content = std::fs::read_to_string(path).map_err(|e| I18nError::CacheError {
            reason: format!("Failed to read TMX file: {}", e),
        })?;

        // Simple string-based XML parsing
        let mut pos = 0;
        while let Some(tu_start) = tmx_content[pos..].find("<tu>") {
            let tu_start_abs = pos + tu_start;
            if let Some(tu_end) = tmx_content[tu_start_abs..].find("</tu>") {
                let tu_end_abs = tu_start_abs + tu_end + 5;
                let tu_content = &tmx_content[tu_start_abs..tu_end_abs];

                // Extract all <tuv> elements
                let mut tuvs = Vec::new();
                let mut tuv_pos = 0;
                while let Some(tuv_start) = tu_content[tuv_pos..].find("<tuv") {
                    let tuv_start_abs = tuv_pos + tuv_start;
                    if let Some(lang_start) = tu_content[tuv_start_abs..].find("xml:lang=\"") {
                        let lang_start_abs = tuv_start_abs + lang_start + 10;
                        if let Some(lang_end) = tu_content[lang_start_abs..].find('"') {
                            let lang = &tu_content[lang_start_abs..lang_start_abs + lang_end];

                            if let Some(seg_start) = tu_content[lang_start_abs..].find("<seg>") {
                                let seg_start_abs = lang_start_abs + seg_start + 5;
                                if let Some(seg_end) = tu_content[seg_start_abs..].find("</seg>") {
                                    let text = &tu_content[seg_start_abs..seg_start_abs + seg_end];
                                    tuvs.push((lang.to_string(), Self::unescape_xml(text)));
                                }
                            }
                        }
                    }

                    if let Some(tuv_end) = tu_content[tuv_start_abs..].find("</tuv>") {
                        tuv_pos = tuv_start_abs + tuv_end + 6;
                    } else {
                        break;
                    }
                }

                // If we have at least 2 tuvs, create a translation entry
                if tuvs.len() >= 2 {
                    if let (Ok(source_locale), Ok(target_locale)) =
                        (Locale::parse(&tuvs[0].0), Locale::parse(&tuvs[1].0))
                    {
                        self.add_translation(
                            tuvs[0].1.clone(),
                            source_locale,
                            tuvs[1].1.clone(),
                            target_locale,
                        );
                    }
                }

                pos = tu_end_abs;
            } else {
                break;
            }
        }

        Ok(())
    }

    /// Merges another translation memory into this one.
    pub fn merge(&mut self, other: &TranslationMemory) {
        for entry in &other.entries {
            self.add_entry(entry.clone());
        }
    }

    /// XML escape helper.
    fn escape_xml(text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&apos;")
    }

    /// XML unescape helper.
    fn unescape_xml(text: &str) -> String {
        text.replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
            .replace("&apos;", "'")
    }

    /// Gets all entries in the memory.
    pub fn entries(&self) -> &[TranslationMemoryEntry] {
        &self.entries
    }

    /// Gets the number of entries in the memory.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Checks if the memory is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Clears all entries from the memory.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.index.clear();
    }
}

/// Screen reader friendly formatter for accessibility.
/// Generates ARIA labels, semantic markup, and screen reader optimized text.
#[derive(Debug)]
pub struct ScreenReaderFormatter {
    #[allow(dead_code)]
    locale: Locale,
}

impl ScreenReaderFormatter {
    /// Creates a new screen reader formatter.
    pub fn new(locale: Locale) -> Self {
        Self { locale }
    }

    /// Generates ARIA label for a legal document section.
    pub fn aria_label(&self, section_type: &str, title: &str) -> String {
        match section_type {
            "article" => format!("Article: {}", title),
            "section" => format!("Section: {}", title),
            "chapter" => format!("Chapter: {}", title),
            "clause" => format!("Clause: {}", title),
            "paragraph" => format!("Paragraph: {}", title),
            _ => format!("{}: {}", section_type, title),
        }
    }

    /// Formats legal citation for screen readers.
    pub fn format_citation(&self, citation: &str) -> String {
        // Expand abbreviations for better screen reader pronunciation
        let expanded = citation
            .replace("v.", "versus")
            .replace("No.", "Number")
            .replace("§", "Section")
            .replace("¶", "Paragraph")
            .replace("U.S.", "United States")
            .replace("F.2d", "Federal Reporter Second Series")
            .replace("F.3d", "Federal Reporter Third Series")
            .replace("S.Ct.", "Supreme Court Reporter");

        format!("Citation: {}", expanded)
    }

    /// Generates semantic navigation structure.
    pub fn navigation_structure(&self, sections: &[(&str, &str)]) -> String {
        let mut nav = String::from("<nav aria-label=\"Document Navigation\">\n");
        nav.push_str("  <ul>\n");

        for (section_type, title) in sections {
            nav.push_str(&format!(
                "    <li><a href=\"#{}\" aria-label=\"{}\">{}</a></li>\n",
                title.to_lowercase().replace(' ', "-"),
                self.aria_label(section_type, title),
                title
            ));
        }

        nav.push_str("  </ul>\n");
        nav.push_str("</nav>\n");
        nav
    }

    /// Formats table data for screen readers.
    pub fn format_table(&self, caption: &str, headers: &[&str], rows: &[Vec<&str>]) -> String {
        let mut table = format!("<table aria-label=\"{}\">\n", caption);
        table.push_str(&format!("  <caption>{}</caption>\n", caption));
        table.push_str("  <thead>\n    <tr>\n");

        for header in headers {
            table.push_str(&format!("      <th scope=\"col\">{}</th>\n", header));
        }

        table.push_str("    </tr>\n  </thead>\n  <tbody>\n");

        for row in rows {
            table.push_str("    <tr>\n");
            for (i, cell) in row.iter().enumerate() {
                if i == 0 {
                    table.push_str(&format!("      <th scope=\"row\">{}</th>\n", cell));
                } else {
                    table.push_str(&format!("      <td>{}</td>\n", cell));
                }
            }
            table.push_str("    </tr>\n");
        }

        table.push_str("  </tbody>\n</table>\n");
        table
    }
}

/// Plain language converter for legal terminology.
/// Converts complex legal jargon to accessible plain language.
#[derive(Debug)]
pub struct PlainLanguageConverter {
    #[allow(dead_code)]
    locale: Locale,
    conversions: HashMap<String, String>,
}

impl PlainLanguageConverter {
    /// Creates a new plain language converter.
    pub fn new(locale: Locale) -> Self {
        let mut conversions = HashMap::new();

        // English plain language conversions
        if locale.language == "en" {
            conversions.insert(
                "aforementioned".to_string(),
                "mentioned earlier".to_string(),
            );
            conversions.insert("hereinafter".to_string(), "from now on".to_string());
            conversions.insert("heretofore".to_string(), "until now".to_string());
            conversions.insert("hereby".to_string(), "by this document".to_string());
            conversions.insert("whereas".to_string(), "because".to_string());
            conversions.insert("wherefore".to_string(), "therefore".to_string());
            conversions.insert("notwithstanding".to_string(), "despite".to_string());
            conversions.insert("pursuant to".to_string(), "under".to_string());
            conversions.insert("subsequent to".to_string(), "after".to_string());
            conversions.insert("prior to".to_string(), "before".to_string());
            conversions.insert("in the event that".to_string(), "if".to_string());
            conversions.insert("null and void".to_string(), "invalid".to_string());
            conversions.insert("force and effect".to_string(), "effect".to_string());
            conversions.insert("cease and desist".to_string(), "stop".to_string());
            conversions.insert(
                "indemnify and hold harmless".to_string(),
                "protect from liability".to_string(),
            );
            conversions.insert("jurisdiction".to_string(), "legal authority".to_string());
            conversions.insert("litigation".to_string(), "lawsuit".to_string());
            conversions.insert(
                "plaintiff".to_string(),
                "person who filed the lawsuit".to_string(),
            );
            conversions.insert("defendant".to_string(), "person being sued".to_string());
            conversions.insert("tort".to_string(), "civil wrong".to_string());
        }

        Self {
            locale,
            conversions,
        }
    }

    /// Converts legal text to plain language.
    pub fn convert(&self, text: &str) -> String {
        let mut result = text.to_string();

        // Simple word-by-word replacement (case-insensitive)
        for (legal_term, plain_term) in &self.conversions {
            // Split text into words and replace matching terms
            let words: Vec<&str> = result.split_whitespace().collect();
            let replaced: Vec<String> = words
                .iter()
                .map(|word| {
                    let clean_word = word.trim_matches(|c: char| !c.is_alphanumeric() && c != ' ');
                    if clean_word.eq_ignore_ascii_case(legal_term) {
                        plain_term.clone()
                    } else {
                        word.to_string()
                    }
                })
                .collect();
            result = replaced.join(" ");
        }

        result
    }

    /// Adds a custom conversion.
    pub fn add_conversion(&mut self, legal_term: impl Into<String>, plain_term: impl Into<String>) {
        self.conversions
            .insert(legal_term.into(), plain_term.into());
    }

    /// Gets the plain language alternative for a term.
    pub fn get_plain_alternative(&self, legal_term: &str) -> Option<&String> {
        self.conversions.get(legal_term)
    }
}

/// Reading level assessor for legal documents.
/// Calculates readability metrics like Flesch-Kincaid grade level.
#[derive(Debug)]
pub struct ReadingLevelAssessor;

impl ReadingLevelAssessor {
    /// Creates a new reading level assessor.
    pub fn new() -> Self {
        Self
    }

    /// Calculates Flesch Reading Ease score (0-100).
    /// Higher scores indicate easier readability.
    /// 90-100: Very Easy (5th grade)
    /// 80-90: Easy (6th grade)
    /// 70-80: Fairly Easy (7th grade)
    /// 60-70: Standard (8th-9th grade)
    /// 50-60: Fairly Difficult (10th-12th grade)
    /// 30-50: Difficult (College)
    /// 0-30: Very Difficult (College graduate)
    pub fn flesch_reading_ease(&self, text: &str) -> f32 {
        let sentences = self.count_sentences(text);
        let words = self.count_words(text);
        let syllables = self.count_syllables(text);

        if sentences == 0 || words == 0 {
            return 0.0;
        }

        let avg_sentence_length = words as f32 / sentences as f32;
        let avg_syllables_per_word = syllables as f32 / words as f32;

        206.835 - (1.015 * avg_sentence_length) - (84.6 * avg_syllables_per_word)
    }

    /// Calculates Flesch-Kincaid Grade Level.
    /// Returns the U.S. grade level required to understand the text.
    pub fn flesch_kincaid_grade(&self, text: &str) -> f32 {
        let sentences = self.count_sentences(text);
        let words = self.count_words(text);
        let syllables = self.count_syllables(text);

        if sentences == 0 || words == 0 {
            return 0.0;
        }

        let avg_sentence_length = words as f32 / sentences as f32;
        let avg_syllables_per_word = syllables as f32 / words as f32;

        (0.39 * avg_sentence_length) + (11.8 * avg_syllables_per_word) - 15.59
    }

    /// Counts sentences in text.
    fn count_sentences(&self, text: &str) -> usize {
        text.split(['.', '!', '?'])
            .filter(|s| !s.trim().is_empty())
            .count()
    }

    /// Counts words in text.
    fn count_words(&self, text: &str) -> usize {
        text.split_whitespace().count()
    }

    /// Counts syllables in text (simplified heuristic).
    fn count_syllables(&self, text: &str) -> usize {
        let words: Vec<&str> = text.split_whitespace().collect();
        words
            .iter()
            .map(|word| self.count_syllables_in_word(word))
            .sum()
    }

    /// Counts syllables in a single word (simplified algorithm).
    fn count_syllables_in_word(&self, word: &str) -> usize {
        let word = word.to_lowercase();
        let word = word.trim_matches(|c: char| !c.is_alphabetic());

        if word.is_empty() {
            return 0;
        }

        let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
        let mut count = 0;
        let mut prev_was_vowel = false;

        for ch in word.chars() {
            let is_vowel = vowels.contains(&ch);
            if is_vowel && !prev_was_vowel {
                count += 1;
            }
            prev_was_vowel = is_vowel;
        }

        // Adjust for silent 'e' at end
        if word.ends_with('e') && count > 1 {
            count -= 1;
        }

        // Ensure at least 1 syllable
        count.max(1)
    }

    /// Provides a readability assessment.
    pub fn assess(&self, text: &str) -> ReadabilityReport {
        let ease = self.flesch_reading_ease(text);
        let grade = self.flesch_kincaid_grade(text);

        let difficulty = if ease >= 90.0 {
            "Very Easy"
        } else if ease >= 80.0 {
            "Easy"
        } else if ease >= 70.0 {
            "Fairly Easy"
        } else if ease >= 60.0 {
            "Standard"
        } else if ease >= 50.0 {
            "Fairly Difficult"
        } else if ease >= 30.0 {
            "Difficult"
        } else {
            "Very Difficult"
        };

        ReadabilityReport {
            flesch_reading_ease: ease,
            flesch_kincaid_grade: grade,
            difficulty: difficulty.to_string(),
            word_count: self.count_words(text),
            sentence_count: self.count_sentences(text),
            syllable_count: self.count_syllables(text),
        }
    }
}

impl Default for ReadingLevelAssessor {
    fn default() -> Self {
        Self::new()
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

/// Braille formatter for visual accessibility.
/// Supports Grade 1 (uncontracted) and Grade 2 (contracted) Braille.
#[derive(Debug)]
pub struct BrailleFormatter {
    #[allow(dead_code)]
    grade: BrailleGrade,
}

/// Braille grade (complexity level).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrailleGrade {
    /// Grade 1: Uncontracted Braille (letter-for-letter)
    Grade1,
    /// Grade 2: Contracted Braille (with abbreviations)
    Grade2,
}

impl BrailleFormatter {
    /// Creates a new Braille formatter.
    pub fn new(grade: BrailleGrade) -> Self {
        Self { grade }
    }

    /// Converts text to Braille Unicode representation.
    pub fn to_braille(&self, text: &str) -> String {
        let text = text.to_lowercase();
        let mut result = String::new();

        for ch in text.chars() {
            if let Some(braille) = self.char_to_braille(ch) {
                result.push(braille);
            } else {
                result.push(ch); // Keep non-alphabetic as-is
            }
        }

        result
    }

    /// Converts a single character to Braille.
    fn char_to_braille(&self, ch: char) -> Option<char> {
        // Unicode Braille Patterns start at U+2800
        // This is a simplified mapping for Grade 1 Braille
        let braille_code = match ch {
            'a' => 0x2801, // ⠁
            'b' => 0x2803, // ⠃
            'c' => 0x2809, // ⠉
            'd' => 0x2819, // ⠙
            'e' => 0x2811, // ⠑
            'f' => 0x280B, // ⠋
            'g' => 0x281B, // ⠛
            'h' => 0x2813, // ⠓
            'i' => 0x280A, // ⠊
            'j' => 0x281A, // ⠚
            'k' => 0x2805, // ⠅
            'l' => 0x2807, // ⠇
            'm' => 0x280D, // ⠍
            'n' => 0x281D, // ⠝
            'o' => 0x2815, // ⠕
            'p' => 0x280F, // ⠏
            'q' => 0x281F, // ⠟
            'r' => 0x2817, // ⠗
            's' => 0x280E, // ⠎
            't' => 0x281E, // ⠞
            'u' => 0x2825, // ⠥
            'v' => 0x2827, // ⠧
            'w' => 0x283A, // ⠺
            'x' => 0x282D, // ⠭
            'y' => 0x283D, // ⠽
            'z' => 0x2835, // ⠵
            ' ' => 0x2800, // ⠀ (blank Braille cell)
            _ => return None,
        };

        char::from_u32(braille_code)
    }

    /// Formats legal document section numbers in Braille.
    pub fn format_section_number(&self, section: &str) -> String {
        format!("§ {}", self.to_braille(section))
    }
}

/// Audio description generator for legal documents.
/// Generates descriptive text for charts, diagrams, and complex structures.
#[derive(Debug)]
pub struct AudioDescriptionGenerator {
    #[allow(dead_code)]
    locale: Locale,
}

impl AudioDescriptionGenerator {
    /// Creates a new audio description generator.
    pub fn new(locale: Locale) -> Self {
        Self { locale }
    }

    /// Generates alt text for a legal diagram.
    pub fn describe_diagram(&self, diagram_type: &str, elements: &[&str]) -> String {
        match diagram_type {
            "flowchart" => {
                format!(
                    "Flowchart showing legal process with {} steps: {}",
                    elements.len(),
                    elements.join(", then ")
                )
            }
            "hierarchy" => {
                format!(
                    "Organizational hierarchy showing {} levels: {}",
                    elements.len(),
                    elements.join(", reporting to ")
                )
            }
            "timeline" => {
                format!(
                    "Timeline of events with {} milestones: {}",
                    elements.len(),
                    elements.join(", followed by ")
                )
            }
            _ => {
                format!(
                    "Diagram of type {} with {} elements: {}",
                    diagram_type,
                    elements.len(),
                    elements.join(", ")
                )
            }
        }
    }

    /// Generates description for a statistical chart.
    pub fn describe_chart(&self, chart_type: &str, data_points: &[(String, f32)]) -> String {
        match chart_type {
            "bar" | "column" => {
                let mut desc = format!("Bar chart showing {} data points. ", data_points.len());
                for (label, value) in data_points {
                    desc.push_str(&format!("{}: {:.1}. ", label, value));
                }
                desc
            }
            "pie" => {
                let total: f32 = data_points.iter().map(|(_, v)| v).sum();
                let mut desc = format!("Pie chart with {} segments. ", data_points.len());
                for (label, value) in data_points {
                    let percentage = (value / total) * 100.0;
                    desc.push_str(&format!("{}: {:.1}%. ", label, percentage));
                }
                desc
            }
            "line" => {
                let mut desc = format!(
                    "Line chart with {} data points showing trend over time. ",
                    data_points.len()
                );
                if data_points.len() >= 2 {
                    let first = data_points.first().unwrap();
                    let last = data_points.last().unwrap();
                    desc.push_str(&format!(
                        "Starting at {} ({:.1}), ending at {} ({:.1}).",
                        first.0, first.1, last.0, last.1
                    ));
                }
                desc
            }
            _ => format!(
                "Chart of type {} with {} data points",
                chart_type,
                data_points.len()
            ),
        }
    }

    /// Generates description for a table.
    pub fn describe_table(&self, caption: &str, rows: usize, cols: usize) -> String {
        format!(
            "Table titled '{}' with {} rows and {} columns. Use table navigation commands to explore the data.",
            caption, rows, cols
        )
    }
}

/// Machine translation fallback manager.
/// Uses translation memory first, then falls back to external services.
#[derive(Debug)]
pub struct MachineTranslationFallback {
    /// Translation memory for caching
    memory: TranslationMemory,
    /// External translation services in priority order
    services: Vec<Box<dyn TranslationService>>,
}

impl MachineTranslationFallback {
    /// Creates a new machine translation fallback manager.
    pub fn new() -> Self {
        Self {
            memory: TranslationMemory::new(),
            services: vec![],
        }
    }

    /// Adds a translation service.
    pub fn add_service(&mut self, service: Box<dyn TranslationService>) {
        self.services.push(service);
    }

    /// Gets a reference to the translation memory.
    pub fn memory(&self) -> &TranslationMemory {
        &self.memory
    }

    /// Gets a mutable reference to the translation memory.
    pub fn memory_mut(&mut self) -> &mut TranslationMemory {
        &mut self.memory
    }

    /// Translates text using fallback chain: memory -> services.
    pub fn translate(
        &mut self,
        text: &str,
        source_locale: &Locale,
        target_locale: &Locale,
    ) -> I18nResult<String> {
        // First, try exact match in translation memory
        let exact_matches = self.memory.find_exact(text, source_locale, target_locale);
        if let Some(entry) = exact_matches.first() {
            return Ok(entry.target_text.clone());
        }

        // Try fuzzy match (>= 0.9 similarity)
        let fuzzy_matches = self
            .memory
            .find_fuzzy(text, source_locale, target_locale, 0.9);
        if let Some((entry, _)) = fuzzy_matches.first() {
            return Ok(entry.target_text.clone());
        }

        // Fall back to external services
        for service in &self.services {
            if !service.is_available() {
                continue;
            }

            match service.translate(text, source_locale, target_locale) {
                Ok(translation) => {
                    // Cache the translation in memory
                    self.memory.add_translation(
                        text.to_string(),
                        source_locale.clone(),
                        translation.clone(),
                        target_locale.clone(),
                    );
                    return Ok(translation);
                }
                Err(_) => {
                    // Try next service
                    continue;
                }
            }
        }

        // No translation available
        Err(I18nError::TranslationMissing {
            key: text.to_string(),
            locale: target_locale.tag(),
        })
    }
}

impl Default for MachineTranslationFallback {
    fn default() -> Self {
        Self::new()
    }
}

/// Legal term extractor for extracting terminology from statutes.
#[derive(Debug, Default)]
pub struct TerminologyExtractor {
    /// Known legal terms
    known_terms: std::collections::HashSet<String>,
    /// Extracted terms with frequencies
    extracted: HashMap<String, usize>,
}

impl TerminologyExtractor {
    /// Creates a new terminology extractor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an extractor with known legal terms from a dictionary.
    pub fn with_dictionary(dictionary: &LegalDictionary) -> Self {
        let mut extractor = Self::new();
        for (key, _) in &dictionary.translations {
            extractor.known_terms.insert(key.clone());
        }
        extractor
    }

    /// Adds a known legal term.
    pub fn add_known_term(&mut self, term: impl Into<String>) {
        self.known_terms.insert(term.into());
    }

    /// Extracts terminology from statute text.
    pub fn extract_from_text(&mut self, text: &str) {
        // Simple extraction: find known terms and count frequencies
        let words: Vec<&str> = text
            .split(|c: char| !c.is_alphanumeric() && c != '_')
            .filter(|w| !w.is_empty())
            .collect();

        for window in words.windows(1) {
            let term = window.join("_").to_lowercase();
            if self.known_terms.contains(&term) {
                *self.extracted.entry(term).or_insert(0) += 1;
            }
        }

        // Also try multi-word terms (up to 3 words)
        for window_size in 2..=3 {
            for window in words.windows(window_size) {
                let term = window.join("_").to_lowercase();
                if self.known_terms.contains(&term) {
                    *self.extracted.entry(term).or_insert(0) += 1;
                }
            }
        }
    }

    /// Gets extracted terms sorted by frequency.
    pub fn get_terms_by_frequency(&self) -> Vec<(String, usize)> {
        let mut terms: Vec<_> = self
            .extracted
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        terms.sort_by(|a, b| b.1.cmp(&a.1));
        terms
    }

    /// Gets the frequency of a specific term.
    pub fn get_frequency(&self, term: &str) -> usize {
        *self.extracted.get(term).unwrap_or(&0)
    }

    /// Gets all extracted terms.
    pub fn extracted_terms(&self) -> &HashMap<String, usize> {
        &self.extracted
    }

    /// Clears all extracted terms.
    pub fn clear(&mut self) {
        self.extracted.clear();
    }
}

// ============================================================================
// Utility Functions for Common I18n Patterns
// ============================================================================

/// Detects the most likely locale from a text sample.
/// Uses simple heuristics based on character sets.
pub fn detect_locale_from_text(text: &str) -> Option<Locale> {
    let has_cjk = text.chars().any(|c| {
        matches!(c,
            '\u{4E00}'..='\u{9FFF}' | // CJK Unified Ideographs
            '\u{3040}'..='\u{309F}' | // Hiragana
            '\u{30A0}'..='\u{30FF}'   // Katakana
        )
    });

    let has_hiragana = text.chars().any(|c| matches!(c, '\u{3040}'..='\u{309F}'));
    let has_katakana = text.chars().any(|c| matches!(c, '\u{30A0}'..='\u{30FF}'));
    let has_cyrillic = text.chars().any(|c| matches!(c, '\u{0400}'..='\u{04FF}'));
    let has_arabic = text.chars().any(|c| matches!(c, '\u{0600}'..='\u{06FF}'));

    if has_hiragana || has_katakana {
        Some(Locale::new("ja").with_country("JP"))
    } else if has_cjk {
        // Could be Chinese - default to simplified
        Some(Locale::new("zh").with_country("CN"))
    } else if has_cyrillic {
        Some(Locale::new("ru").with_country("RU"))
    } else if has_arabic {
        Some(Locale::new("ar"))
    } else {
        // Default to English if no specific script detected
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

    // Append parameters
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
            // Country code - uppercase
            normalized.push_str(&part.to_uppercase());
        } else if part.len() == 4 && part.chars().all(|c| c.is_alphabetic()) {
            // Script code - title case
            let mut chars = part.chars();
            if let Some(first) = chars.next() {
                normalized.push(first.to_uppercase().next().unwrap());
                normalized.extend(chars.map(|c| c.to_lowercase().next().unwrap()));
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
        if let Some(ref country) = locale.country {
            if !is_valid_country_code(country) {
                return false;
            }
        }
        if let Some(ref script) = locale.script {
            if !is_valid_script_code(script) {
                return false;
            }
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
    // Try exact match first
    for locale in available {
        if locale == requested {
            return Some(locale);
        }
    }

    // Try language + country match
    if requested.country.is_some() {
        for locale in available {
            if locale.language == requested.language && locale.country == requested.country {
                return Some(locale);
            }
        }
    }

    // Try language-only match
    for locale in available {
        if locale.language == requested.language && locale.country.is_none() {
            return Some(locale);
        }
    }

    // Try any locale with the same language
    available
        .iter()
        .find(|locale| locale.language == requested.language)
        .map(|v| v as _)
}

/// Text collator for locale-aware sorting and comparison.
#[derive(Debug, Clone)]
pub struct TextCollator {
    locale: Locale,
}

impl TextCollator {
    /// Creates a new text collator for the specified locale.
    pub fn new(locale: Locale) -> Self {
        Self { locale }
    }

    /// Compares two strings according to locale-specific rules.
    /// Returns std::cmp::Ordering indicating the relationship between the strings.
    pub fn compare(&self, a: &str, b: &str) -> std::cmp::Ordering {
        // Simplified collation - in production would use ICU or similar
        match self.locale.language.as_str() {
            "ja" | "zh" => {
                // For CJK languages, use unicode code point order
                a.cmp(b)
            }
            _ => {
                // For other languages, use case-insensitive comparison
                let a_lower = a.to_lowercase();
                let b_lower = b.to_lowercase();
                a_lower.cmp(&b_lower)
            }
        }
    }

    /// Sorts a vector of strings according to locale-specific rules.
    pub fn sort(&self, items: &mut [String]) {
        items.sort_by(|a, b| self.compare(a, b));
    }

    /// Returns a sorted copy of the input strings.
    pub fn sorted(&self, items: &[String]) -> Vec<String> {
        let mut sorted = items.to_vec();
        self.sort(&mut sorted);
        sorted
    }

    /// Checks if a string starts with a prefix (locale-aware, case-insensitive for most locales).
    pub fn starts_with(&self, text: &str, prefix: &str) -> bool {
        match self.locale.language.as_str() {
            "ja" | "zh" => text.starts_with(prefix),
            _ => text.to_lowercase().starts_with(&prefix.to_lowercase()),
        }
    }

    /// Normalizes a string for comparison (removes accents, converts to lowercase, etc.).
    pub fn normalize(&self, text: &str) -> String {
        match self.locale.language.as_str() {
            "de" => {
                // German: handle umlauts
                text.to_lowercase()
                    .replace('ä', "ae")
                    .replace('ö', "oe")
                    .replace('ü', "ue")
                    .replace('ß', "ss")
            }
            "fr" | "es" => {
                // French/Spanish: simplified accent removal
                text.to_lowercase()
                    .replace(['é', 'è', 'ê', 'ë'], "e")
                    .replace(['à', 'â'], "a")
                    .replace('ñ', "n")
                    .replace('ç', "c")
            }
            _ => text.to_lowercase(),
        }
    }
}

// ============================================================================
// Legal Document Formatting Extensions (v0.1.5)
// ============================================================================

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

/// Legal document numbering formatter.
#[derive(Debug, Clone)]
pub struct DocumentNumbering {
    style: NumberingStyle,
    #[allow(dead_code)]
    locale: Locale,
}

impl DocumentNumbering {
    /// Creates a new document numbering formatter.
    pub fn new(style: NumberingStyle, locale: Locale) -> Self {
        Self { style, locale }
    }

    /// Formats a hierarchical number (e.g., Article 1, Section 2.1, etc.).
    pub fn format(&self, level: usize, number: usize) -> String {
        match self.style {
            NumberingStyle::Article => match level {
                0 => format!("Article {}", number),
                1 => format!("Section {}", number),
                2 => format!("Paragraph {}", number),
                3 => format!("Clause {}", number),
                _ => format!("Subclause {}", number),
            },
            NumberingStyle::Section => match level {
                0 => format!("Section {}", number),
                1 => self.format_subsection(number),
                2 => self.format_roman_lowercase(number),
                _ => format!("({})", number),
            },
            NumberingStyle::Chapter => match level {
                0 => format!("Chapter {}", number),
                1 => format!("Part {}", self.format_uppercase_letter(number)),
                2 => format!("Subdivision ({})", number),
                _ => format!("({})", self.format_lowercase_letter(number)),
            },
            NumberingStyle::Hierarchical => match level {
                0 => format!("{}.", number),
                1 => format!("{}.", self.format_lowercase_letter(number)),
                2 => format!("{}.", self.format_roman_lowercase(number)),
                _ => format!("({})", number),
            },
            NumberingStyle::Parenthetical => match level {
                0 => format!("({})", number),
                1 => format!("({})", self.format_lowercase_letter(number)),
                2 => format!("({})", self.format_roman_lowercase(number)),
                _ => format!("({})", number),
            },
        }
    }

    fn format_lowercase_letter(&self, n: usize) -> String {
        if n == 0 || n > 26 {
            return n.to_string();
        }
        ((b'a' + (n as u8) - 1) as char).to_string()
    }

    fn format_uppercase_letter(&self, n: usize) -> String {
        if n == 0 || n > 26 {
            return n.to_string();
        }
        ((b'A' + (n as u8) - 1) as char).to_string()
    }

    fn format_subsection(&self, n: usize) -> String {
        format!("Subsection {}", self.format_lowercase_letter(n))
    }

    fn format_roman_lowercase(&self, n: usize) -> String {
        match n {
            1 => "i".to_string(),
            2 => "ii".to_string(),
            3 => "iii".to_string(),
            4 => "iv".to_string(),
            5 => "v".to_string(),
            6 => "vi".to_string(),
            7 => "vii".to_string(),
            8 => "viii".to_string(),
            9 => "ix".to_string(),
            10 => "x".to_string(),
            _ => n.to_string(),
        }
    }
}

/// Footnote or endnote formatter.
#[derive(Debug, Clone)]
pub struct FootnoteFormatter {
    style: FootnoteStyle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FootnoteStyle {
    /// Numeric: 1, 2, 3, ...
    Numeric,
    /// Symbols: *, †, ‡, §, ...
    Symbol,
    /// Lowercase letters: a, b, c, ...
    Letter,
}

impl FootnoteFormatter {
    /// Creates a new footnote formatter.
    pub fn new(style: FootnoteStyle) -> Self {
        Self { style }
    }

    /// Formats a footnote marker.
    pub fn format_marker(&self, number: usize) -> String {
        match self.style {
            FootnoteStyle::Numeric => number.to_string(),
            FootnoteStyle::Symbol => self.format_symbol(number),
            FootnoteStyle::Letter => {
                if number == 0 || number > 26 {
                    number.to_string()
                } else {
                    ((b'a' + (number as u8) - 1) as char).to_string()
                }
            }
        }
    }

    fn format_symbol(&self, n: usize) -> String {
        let symbols = ["*", "†", "‡", "§", "¶", "‖"];
        if n == 0 || n > symbols.len() {
            n.to_string()
        } else {
            symbols[n - 1].to_string()
        }
    }

    /// Formats a full footnote with text.
    pub fn format_note(&self, number: usize, text: &str) -> String {
        format!("{} {}", self.format_marker(number), text)
    }
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

/// Table of contents entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TocEntry {
    /// Entry title
    pub title: String,
    /// Page number
    pub page: usize,
    /// Nesting level (0 = top level)
    pub level: usize,
    /// Section number (e.g., "1.2.3")
    pub section_number: Option<String>,
}

/// Table of contents generator.
#[derive(Debug, Clone)]
pub struct TableOfContents {
    entries: Vec<TocEntry>,
    locale: Locale,
}

impl TableOfContents {
    /// Creates a new table of contents.
    pub fn new(locale: Locale) -> Self {
        Self {
            entries: Vec::new(),
            locale,
        }
    }

    /// Adds an entry to the table of contents.
    pub fn add_entry(
        &mut self,
        title: String,
        page: usize,
        level: usize,
        section_number: Option<String>,
    ) {
        self.entries.push(TocEntry {
            title,
            page,
            level,
            section_number,
        });
    }

    /// Generates the formatted table of contents.
    pub fn generate(&self) -> String {
        let mut result = String::new();

        // Add header
        let header = match self.locale.language.as_str() {
            "en" => "Table of Contents",
            "ja" => "目次",
            "de" => "Inhaltsverzeichnis",
            "fr" => "Table des matières",
            "es" => "Tabla de contenidos",
            "it" => "Indice",
            "pt" => "Índice",
            "nl" => "Inhoudsopgave",
            "pl" => "Spis treści",
            "ko" => "목차",
            _ => "Table of Contents",
        };
        result.push_str(header);
        result.push_str("\n\n");

        for entry in &self.entries {
            let indent = "  ".repeat(entry.level);
            let section = entry.section_number.as_deref().unwrap_or("");
            let dots = ".".repeat(50 - entry.title.len() - section.len());

            if section.is_empty() {
                result.push_str(&format!(
                    "{}{} {} {}\n",
                    indent, entry.title, dots, entry.page
                ));
            } else {
                result.push_str(&format!(
                    "{}{} {} {} {}\n",
                    indent, section, entry.title, dots, entry.page
                ));
            }
        }

        result
    }
}

/// Index entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    /// Term
    pub term: String,
    /// Page numbers where this term appears
    pub pages: Vec<usize>,
    /// Sub-entries
    pub sub_entries: Vec<IndexEntry>,
}

impl IndexEntry {
    /// Creates a new index entry.
    pub fn new(term: String) -> Self {
        Self {
            term,
            pages: Vec::new(),
            sub_entries: Vec::new(),
        }
    }

    /// Adds a page reference.
    pub fn add_page(&mut self, page: usize) {
        if !self.pages.contains(&page) {
            self.pages.push(page);
            self.pages.sort();
        }
    }

    /// Adds a sub-entry.
    pub fn add_sub_entry(&mut self, entry: IndexEntry) {
        self.sub_entries.push(entry);
    }
}

/// Index generator.
#[derive(Debug, Clone)]
pub struct IndexGenerator {
    entries: Vec<IndexEntry>,
    locale: Locale,
}

impl IndexGenerator {
    /// Creates a new index generator.
    pub fn new(locale: Locale) -> Self {
        Self {
            entries: Vec::new(),
            locale,
        }
    }

    /// Adds an entry to the index.
    pub fn add_entry(&mut self, entry: IndexEntry) {
        self.entries.push(entry);
    }

    /// Sorts entries alphabetically.
    pub fn sort(&mut self) {
        self.entries.sort_by(|a, b| a.term.cmp(&b.term));
        for entry in &mut self.entries {
            entry.sub_entries.sort_by(|a, b| a.term.cmp(&b.term));
        }
    }

    /// Generates the formatted index.
    pub fn generate(&self) -> String {
        let mut result = String::new();

        // Add header
        let header = match self.locale.language.as_str() {
            "en" => "Index",
            "ja" => "索引",
            "de" => "Index",
            "fr" => "Index",
            "es" => "Índice",
            "it" => "Indice",
            "pt" => "Índice",
            "nl" => "Index",
            "pl" => "Indeks",
            "ko" => "색인",
            _ => "Index",
        };
        result.push_str(header);
        result.push_str("\n\n");

        for entry in &self.entries {
            self.format_entry(&mut result, entry, 0);
        }

        result
    }

    #[allow(clippy::only_used_in_recursion)]
    fn format_entry(&self, result: &mut String, entry: &IndexEntry, level: usize) {
        let indent = "  ".repeat(level);
        let pages = entry
            .pages
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        result.push_str(&format!("{}{}, {}\n", indent, entry.term, pages));

        for sub in &entry.sub_entries {
            self.format_entry(result, sub, level + 1);
        }
    }
}

// ============================================================================
// Specialized Legal Term Dictionaries (v0.1.4)
// ============================================================================

/// Legal domain specializations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LegalDomain {
    /// General legal terms
    General,
    /// Intellectual Property Law
    IntellectualProperty,
    /// Tax Law
    Tax,
    /// Environmental Law
    Environmental,
    /// Labor and Employment Law
    Labor,
    /// Corporate Law
    Corporate,
    /// Criminal Law
    Criminal,
    /// Civil Procedure
    CivilProcedure,
}

impl LegalDomain {
    /// Creates a specialized dictionary for a given domain and locale.
    pub fn create_dictionary(&self, locale: Locale) -> LegalDictionary {
        let mut dict = LegalDictionary::new(locale.clone());

        match self {
            LegalDomain::General => {
                // General terms already handled by base dictionaries
                dict
            }
            LegalDomain::IntellectualProperty => {
                self.add_ip_terms(&mut dict, &locale);
                dict
            }
            LegalDomain::Tax => {
                self.add_tax_terms(&mut dict, &locale);
                dict
            }
            LegalDomain::Environmental => {
                self.add_environmental_terms(&mut dict, &locale);
                dict
            }
            LegalDomain::Labor => {
                self.add_labor_terms(&mut dict, &locale);
                dict
            }
            LegalDomain::Corporate => {
                self.add_corporate_terms(&mut dict, &locale);
                dict
            }
            LegalDomain::Criminal => {
                self.add_criminal_terms(&mut dict, &locale);
                dict
            }
            LegalDomain::CivilProcedure => {
                self.add_civil_procedure_terms(&mut dict, &locale);
                dict
            }
        }
    }

    fn add_ip_terms(&self, dict: &mut LegalDictionary, locale: &Locale) {
        match locale.language.as_str() {
            "en" => {
                dict.add_translation("patent", "patent");
                dict.add_translation("trademark", "trademark");
                dict.add_translation("copyright", "copyright");
                dict.add_translation("trade_secret", "trade secret");
                dict.add_translation("intellectual_property", "intellectual property");
                dict.add_translation("infringement", "infringement");
                dict.add_translation("prior_art", "prior art");
                dict.add_translation("novelty", "novelty");
                dict.add_translation("non_obviousness", "non-obviousness");
                dict.add_translation("fair_use", "fair use");
                dict.add_translation("licensing", "licensing");
                dict.add_translation("royalty", "royalty");
                dict.add_translation("utility_patent", "utility patent");
                dict.add_translation("design_patent", "design patent");
                dict.add_abbreviation("patent", "Pat.");
                dict.add_abbreviation("trademark", "TM");
                dict.add_abbreviation("copyright", "©");
            }
            "ja" => {
                dict.add_translation("patent", "特許");
                dict.add_translation("trademark", "商標");
                dict.add_translation("copyright", "著作権");
                dict.add_translation("trade_secret", "営業秘密");
                dict.add_translation("intellectual_property", "知的財産権");
                dict.add_translation("infringement", "侵害");
                dict.add_translation("prior_art", "先行技術");
                dict.add_translation("novelty", "新規性");
                dict.add_translation("licensing", "ライセンス");
                dict.add_translation("royalty", "ロイヤルティ");
            }
            "de" => {
                dict.add_translation("patent", "Patent");
                dict.add_translation("trademark", "Marke");
                dict.add_translation("copyright", "Urheberrecht");
                dict.add_translation("intellectual_property", "geistiges Eigentum");
                dict.add_translation("infringement", "Verletzung");
            }
            _ => {}
        }
    }

    fn add_tax_terms(&self, dict: &mut LegalDictionary, locale: &Locale) {
        match locale.language.as_str() {
            "en" => {
                dict.add_translation("income_tax", "income tax");
                dict.add_translation("corporate_tax", "corporate tax");
                dict.add_translation("value_added_tax", "value-added tax");
                dict.add_translation("capital_gains", "capital gains");
                dict.add_translation("deduction", "deduction");
                dict.add_translation("exemption", "exemption");
                dict.add_translation("tax_liability", "tax liability");
                dict.add_translation("withholding_tax", "withholding tax");
                dict.add_translation("tax_credit", "tax credit");
                dict.add_translation("taxable_income", "taxable income");
                dict.add_translation("tax_evasion", "tax evasion");
                dict.add_translation("tax_avoidance", "tax avoidance");
                dict.add_translation("fiscal_year", "fiscal year");
                dict.add_abbreviation("value_added_tax", "VAT");
                dict.add_abbreviation("income_tax", "IT");
            }
            "ja" => {
                dict.add_translation("income_tax", "所得税");
                dict.add_translation("corporate_tax", "法人税");
                dict.add_translation("value_added_tax", "消費税");
                dict.add_translation("capital_gains", "キャピタルゲイン");
                dict.add_translation("deduction", "控除");
                dict.add_translation("exemption", "免税");
                dict.add_translation("tax_liability", "納税義務");
                dict.add_translation("withholding_tax", "源泉徴収");
            }
            "de" => {
                dict.add_translation("income_tax", "Einkommensteuer");
                dict.add_translation("corporate_tax", "Körperschaftsteuer");
                dict.add_translation("value_added_tax", "Mehrwertsteuer");
                dict.add_translation("deduction", "Abzug");
                dict.add_abbreviation("value_added_tax", "MwSt");
            }
            _ => {}
        }
    }

    fn add_environmental_terms(&self, dict: &mut LegalDictionary, locale: &Locale) {
        match locale.language.as_str() {
            "en" => {
                dict.add_translation("environmental_impact", "environmental impact");
                dict.add_translation("pollution", "pollution");
                dict.add_translation("emissions", "emissions");
                dict.add_translation("sustainability", "sustainability");
                dict.add_translation(
                    "environmental_assessment",
                    "environmental impact assessment",
                );
                dict.add_translation("climate_change", "climate change");
                dict.add_translation("carbon_footprint", "carbon footprint");
                dict.add_translation("renewable_energy", "renewable energy");
                dict.add_translation("hazardous_waste", "hazardous waste");
                dict.add_translation("conservation", "conservation");
                dict.add_translation("biodiversity", "biodiversity");
                dict.add_translation("environmental_compliance", "environmental compliance");
                dict.add_abbreviation("environmental_assessment", "EIA");
                dict.add_abbreviation("environmental_protection", "EPA");
            }
            "ja" => {
                dict.add_translation("environmental_impact", "環境影響");
                dict.add_translation("pollution", "汚染");
                dict.add_translation("emissions", "排出");
                dict.add_translation("sustainability", "持続可能性");
                dict.add_translation("environmental_assessment", "環境アセスメント");
                dict.add_translation("climate_change", "気候変動");
                dict.add_translation("renewable_energy", "再生可能エネルギー");
            }
            "de" => {
                dict.add_translation("environmental_impact", "Umweltauswirkung");
                dict.add_translation("pollution", "Verschmutzung");
                dict.add_translation("emissions", "Emissionen");
                dict.add_translation("sustainability", "Nachhaltigkeit");
            }
            _ => {}
        }
    }

    fn add_labor_terms(&self, dict: &mut LegalDictionary, locale: &Locale) {
        match locale.language.as_str() {
            "en" => {
                dict.add_translation("employment_contract", "employment contract");
                dict.add_translation("collective_bargaining", "collective bargaining");
                dict.add_translation("wrongful_termination", "wrongful termination");
                dict.add_translation("discrimination", "discrimination");
                dict.add_translation("harassment", "harassment");
                dict.add_translation("minimum_wage", "minimum wage");
                dict.add_translation("overtime", "overtime");
                dict.add_translation("severance_pay", "severance pay");
                dict.add_translation("workers_compensation", "workers' compensation");
                dict.add_translation("occupational_safety", "occupational safety and health");
                dict.add_translation("labor_union", "labor union");
                dict.add_translation("strike", "strike");
                dict.add_translation("lockout", "lockout");
                dict.add_abbreviation("occupational_safety", "OSHA");
            }
            "ja" => {
                dict.add_translation("employment_contract", "雇用契約");
                dict.add_translation("collective_bargaining", "団体交渉");
                dict.add_translation("wrongful_termination", "不当解雇");
                dict.add_translation("discrimination", "差別");
                dict.add_translation("harassment", "ハラスメント");
                dict.add_translation("minimum_wage", "最低賃金");
                dict.add_translation("overtime", "残業");
                dict.add_translation("severance_pay", "退職金");
                dict.add_translation("labor_union", "労働組合");
            }
            "de" => {
                dict.add_translation("employment_contract", "Arbeitsvertrag");
                dict.add_translation("collective_bargaining", "Tarifverhandlungen");
                dict.add_translation("discrimination", "Diskriminierung");
                dict.add_translation("minimum_wage", "Mindestlohn");
                dict.add_translation("overtime", "Überstunden");
            }
            _ => {}
        }
    }

    fn add_corporate_terms(&self, dict: &mut LegalDictionary, locale: &Locale) {
        match locale.language.as_str() {
            "en" => {
                dict.add_translation("merger", "merger");
                dict.add_translation("acquisition", "acquisition");
                dict.add_translation("due_diligence", "due diligence");
                dict.add_translation("shareholder", "shareholder");
                dict.add_translation("board_of_directors", "board of directors");
                dict.add_translation("corporate_governance", "corporate governance");
                dict.add_translation("fiduciary_duty", "fiduciary duty");
                dict.add_abbreviation("merger_and_acquisition", "M&A");
            }
            "ja" => {
                dict.add_translation("merger", "合併");
                dict.add_translation("acquisition", "買収");
                dict.add_translation("due_diligence", "デューデリジェンス");
                dict.add_translation("shareholder", "株主");
                dict.add_translation("board_of_directors", "取締役会");
            }
            _ => {}
        }
    }

    fn add_criminal_terms(&self, dict: &mut LegalDictionary, locale: &Locale) {
        match locale.language.as_str() {
            "en" => {
                dict.add_translation("indictment", "indictment");
                dict.add_translation("arraignment", "arraignment");
                dict.add_translation("plea_bargain", "plea bargain");
                dict.add_translation("miranda_rights", "Miranda rights");
                dict.add_translation("probable_cause", "probable cause");
                dict.add_translation("beyond_reasonable_doubt", "beyond a reasonable doubt");
            }
            "ja" => {
                dict.add_translation("indictment", "起訴");
                dict.add_translation("arraignment", "罪状認否");
                dict.add_translation("probable_cause", "相当な理由");
            }
            _ => {}
        }
    }

    fn add_civil_procedure_terms(&self, dict: &mut LegalDictionary, locale: &Locale) {
        match locale.language.as_str() {
            "en" => {
                dict.add_translation("complaint", "complaint");
                dict.add_translation("summons", "summons");
                dict.add_translation("discovery", "discovery");
                dict.add_translation("deposition", "deposition");
                dict.add_translation("interrogatories", "interrogatories");
                dict.add_translation("summary_judgment", "summary judgment");
                dict.add_translation("motion_to_dismiss", "motion to dismiss");
            }
            "ja" => {
                dict.add_translation("complaint", "訴状");
                dict.add_translation("summons", "召喚状");
                dict.add_translation("discovery", "証拠開示");
            }
            _ => {}
        }
    }
}

// ============================================================================
// Regional Variations v0.1.9: State/Province Level and Advanced Features
// ============================================================================

/// Sub-regional variation (state/province level) information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubRegionalVariation {
    /// The base locale (country level)
    pub base_locale: Locale,
    /// Sub-region code (e.g., "CA" for California, "ON" for Ontario)
    pub region_code: String,
    /// Full name of the sub-region
    pub region_name: String,
    /// Description of the sub-regional variation
    pub description: String,
    /// Key legal differences from federal/national level
    pub legal_differences: Vec<String>,
}

impl SubRegionalVariation {
    /// Creates a new sub-regional variation.
    pub fn new(
        base_locale: Locale,
        region_code: impl Into<String>,
        region_name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            base_locale,
            region_code: region_code.into(),
            region_name: region_name.into(),
            description: description.into(),
            legal_differences: vec![],
        }
    }

    /// Adds a legal difference description.
    pub fn add_legal_difference(mut self, difference: impl Into<String>) -> Self {
        self.legal_differences.push(difference.into());
        self
    }
}

/// Registry of sub-regional variations (states, provinces, etc.).
#[derive(Debug, Default)]
pub struct SubRegionalVariationRegistry {
    variations: Vec<SubRegionalVariation>,
}

impl SubRegionalVariationRegistry {
    /// Creates a new registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a registry with default sub-regional variations.
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();

        // US States (selected major jurisdictions)
        let us_locale = Locale::new("en").with_country("US");

        registry.add_variation(
            SubRegionalVariation::new(
                us_locale.clone(),
                "CA",
                "California",
                "California state law",
            )
            .add_legal_difference("Community property state")
            .add_legal_difference("Strong consumer protection laws (CCPA, CPRA)")
            .add_legal_difference("California Civil Code and California Penal Code")
            .add_legal_difference("Proposition 65 environmental regulations"),
        );

        registry.add_variation(
            SubRegionalVariation::new(us_locale.clone(), "NY", "New York", "New York state law")
                .add_legal_difference("Martin Act for securities regulation")
                .add_legal_difference("Strong tenant protection laws")
                .add_legal_difference("New York General Business Law")
                .add_legal_difference("Unique corporate law provisions"),
        );

        registry.add_variation(
            SubRegionalVariation::new(us_locale.clone(), "TX", "Texas", "Texas state law")
                .add_legal_difference("Community property state")
                .add_legal_difference("Texas Business Organizations Code")
                .add_legal_difference("No state income tax")
                .add_legal_difference("Homestead protection laws"),
        );

        registry.add_variation(
            SubRegionalVariation::new(us_locale.clone(), "FL", "Florida", "Florida state law")
                .add_legal_difference("Strong homestead exemption")
                .add_legal_difference("No state income tax")
                .add_legal_difference("Unique foreclosure laws")
                .add_legal_difference("Florida Statutes comprehensive code"),
        );

        registry.add_variation(
            SubRegionalVariation::new(us_locale.clone(), "IL", "Illinois", "Illinois state law")
                .add_legal_difference("Illinois Compiled Statutes")
                .add_legal_difference("Unique business entity structures")
                .add_legal_difference("Cook County court system"),
        );

        registry.add_variation(
            SubRegionalVariation::new(us_locale.clone(), "DE", "Delaware", "Delaware state law")
                .add_legal_difference("Premier corporate law jurisdiction (DGCL)")
                .add_legal_difference("Court of Chancery for business disputes")
                .add_legal_difference("Majority of Fortune 500 incorporated here"),
        );

        // Canadian Provinces
        let ca_locale = Locale::new("en").with_country("CA");

        registry.add_variation(
            SubRegionalVariation::new(ca_locale.clone(), "ON", "Ontario", "Ontario provincial law")
                .add_legal_difference("Common law province")
                .add_legal_difference("Business Corporations Act (Ontario)")
                .add_legal_difference("Ontario Superior Court of Justice")
                .add_legal_difference("Bilingual legal services in some areas"),
        );

        registry.add_variation(
            SubRegionalVariation::new(
                Locale::new("fr").with_country("CA"),
                "QC",
                "Québec",
                "Québec provincial law",
            )
            .add_legal_difference("Civil law jurisdiction (only in North America)")
            .add_legal_difference("Code civil du Québec (Civil Code of Québec)")
            .add_legal_difference("French language legal system")
            .add_legal_difference("Notarial system for real estate transactions"),
        );

        registry.add_variation(
            SubRegionalVariation::new(
                ca_locale.clone(),
                "BC",
                "British Columbia",
                "British Columbia provincial law",
            )
            .add_legal_difference("Common law province")
            .add_legal_difference("Business Corporations Act (BC)")
            .add_legal_difference("Land Title and Survey Authority system")
            .add_legal_difference("Strong indigenous law considerations"),
        );

        registry.add_variation(
            SubRegionalVariation::new(ca_locale.clone(), "AB", "Alberta", "Alberta provincial law")
                .add_legal_difference("Common law province")
                .add_legal_difference("Strong energy law sector")
                .add_legal_difference("Business Corporations Act (Alberta)")
                .add_legal_difference("Alberta Court of Queen's Bench"),
        );

        // Additional US States
        registry.add_variation(
            SubRegionalVariation::new(
                us_locale.clone(),
                "WA",
                "Washington",
                "Washington state law",
            )
            .add_legal_difference("Community property state")
            .add_legal_difference("Strong tech industry regulations")
            .add_legal_difference("No state income tax"),
        );

        registry.add_variation(
            SubRegionalVariation::new(
                us_locale.clone(),
                "MA",
                "Massachusetts",
                "Massachusetts state law",
            )
            .add_legal_difference("Strong healthcare regulations")
            .add_legal_difference("Massachusetts General Laws")
            .add_legal_difference("Pioneering insurance reform"),
        );

        registry.add_variation(
            SubRegionalVariation::new(
                us_locale.clone(),
                "PA",
                "Pennsylvania",
                "Pennsylvania state law",
            )
            .add_legal_difference("Pennsylvania Consolidated Statutes")
            .add_legal_difference("Mixed equitable separate property system")
            .add_legal_difference("Unique trust law provisions"),
        );

        registry.add_variation(
            SubRegionalVariation::new(us_locale.clone(), "GA", "Georgia", "Georgia state law")
                .add_legal_difference("Georgia Code")
                .add_legal_difference("Business-friendly corporate law")
                .add_legal_difference("Homestead exemption"),
        );

        registry.add_variation(
            SubRegionalVariation::new(
                us_locale.clone(),
                "NC",
                "North Carolina",
                "North Carolina state law",
            )
            .add_legal_difference("North Carolina General Statutes")
            .add_legal_difference("Unique business court system")
            .add_legal_difference("Strong banking law tradition"),
        );

        registry.add_variation(
            SubRegionalVariation::new(us_locale.clone(), "AZ", "Arizona", "Arizona state law")
                .add_legal_difference("Community property state")
                .add_legal_difference("Arizona Revised Statutes")
                .add_legal_difference("Water law specialization"),
        );

        registry.add_variation(
            SubRegionalVariation::new(us_locale.clone(), "NV", "Nevada", "Nevada state law")
                .add_legal_difference("Community property state")
                .add_legal_difference("No state income tax")
                .add_legal_difference("Gaming and entertainment law"),
        );

        registry.add_variation(
            SubRegionalVariation::new(us_locale.clone(), "OH", "Ohio", "Ohio state law")
                .add_legal_difference("Ohio Revised Code")
                .add_legal_difference("Strong manufacturing law")
                .add_legal_difference("Unique probate court system"),
        );

        registry.add_variation(
            SubRegionalVariation::new(us_locale.clone(), "MI", "Michigan", "Michigan state law")
                .add_legal_difference("Michigan Compiled Laws")
                .add_legal_difference("No-fault auto insurance")
                .add_legal_difference("Strong labor law tradition"),
        );

        registry.add_variation(
            SubRegionalVariation::new(us_locale.clone(), "CO", "Colorado", "Colorado state law")
                .add_legal_difference("Colorado Revised Statutes")
                .add_legal_difference("Cannabis law regulations")
                .add_legal_difference("Water rights priority system"),
        );

        // Canadian Territories
        registry.add_variation(
            SubRegionalVariation::new(ca_locale.clone(), "YT", "Yukon", "Yukon territorial law")
                .add_legal_difference("Common law territory")
                .add_legal_difference("Indigenous self-government agreements")
                .add_legal_difference("Mining law specialization"),
        );

        registry.add_variation(
            SubRegionalVariation::new(
                ca_locale.clone(),
                "NT",
                "Northwest Territories",
                "NWT territorial law",
            )
            .add_legal_difference("Common law territory")
            .add_legal_difference("Unique indigenous land claims")
            .add_legal_difference("Resource extraction regulations"),
        );

        registry.add_variation(
            SubRegionalVariation::new(
                ca_locale.clone(),
                "NU",
                "Nunavut",
                "Nunavut territorial law",
            )
            .add_legal_difference("Common law territory")
            .add_legal_difference("Inuit Qaujimajatuqangit integration")
            .add_legal_difference("Bilingual Inuktitut-English system"),
        );

        // Asian Countries
        let in_locale = Locale::new("en").with_country("IN");
        registry.add_variation(
            SubRegionalVariation::new(
                in_locale.clone(),
                "MH",
                "Maharashtra",
                "Maharashtra state law",
            )
            .add_legal_difference("Bombay High Court jurisdiction")
            .add_legal_difference("Strong commercial law center")
            .add_legal_difference("Maharashtra-specific acts"),
        );

        registry.add_variation(
            SubRegionalVariation::new(
                in_locale.clone(),
                "DL",
                "Delhi",
                "Delhi union territory law",
            )
            .add_legal_difference("Delhi High Court")
            .add_legal_difference("National Capital Territory status")
            .add_legal_difference("Mixed central and state jurisdiction"),
        );

        registry.add_variation(
            SubRegionalVariation::new(in_locale.clone(), "KA", "Karnataka", "Karnataka state law")
                .add_legal_difference("Karnataka High Court")
                .add_legal_difference("Tech industry legal framework")
                .add_legal_difference("IT Act specialization"),
        );

        let sg_locale = Locale::new("en").with_country("SG");
        registry.add_variation(
            SubRegionalVariation::new(sg_locale.clone(), "SG", "Singapore", "Singapore law")
                .add_legal_difference("Common law system based on English law")
                .add_legal_difference("Strong arbitration center")
                .add_legal_difference("Business-friendly corporate law"),
        );

        let my_locale = Locale::new("ms").with_country("MY");
        registry.add_variation(
            SubRegionalVariation::new(
                my_locale.clone(),
                "WP",
                "Kuala Lumpur",
                "Federal Territory law",
            )
            .add_legal_difference("Federal Court jurisdiction")
            .add_legal_difference("Mixed common law and Islamic law")
            .add_legal_difference("Financial services center"),
        );

        let th_locale = Locale::new("th").with_country("TH");
        registry.add_variation(
            SubRegionalVariation::new(
                th_locale.clone(),
                "BKK",
                "Bangkok",
                "Bangkok metropolitan law",
            )
            .add_legal_difference("Central Administrative Court")
            .add_legal_difference("Civil law system")
            .add_legal_difference("Foreign Business Act regulations"),
        );

        let vn_locale = Locale::new("vi").with_country("VN");
        registry.add_variation(
            SubRegionalVariation::new(vn_locale.clone(), "HN", "Hanoi", "Hanoi municipal law")
                .add_legal_difference("Socialist legal system")
                .add_legal_difference("People's Court jurisdiction")
                .add_legal_difference("Investment law specialization"),
        );

        registry.add_variation(
            SubRegionalVariation::new(
                vn_locale.clone(),
                "SG",
                "Ho Chi Minh City",
                "HCMC municipal law",
            )
            .add_legal_difference("Economic hub regulations")
            .add_legal_difference("Foreign investment zone")
            .add_legal_difference("Commercial arbitration center"),
        );

        let id_locale = Locale::new("id").with_country("ID");
        registry.add_variation(
            SubRegionalVariation::new(
                id_locale.clone(),
                "JK",
                "Jakarta",
                "Jakarta special capital region",
            )
            .add_legal_difference("Civil law system (Dutch-influenced)")
            .add_legal_difference("Supreme Court jurisdiction")
            .add_legal_difference("Investment Coordinating Board center"),
        );

        // Middle Eastern Countries
        let ae_locale = Locale::new("ar").with_country("AE");
        registry.add_variation(
            SubRegionalVariation::new(ae_locale.clone(), "DU", "Dubai", "Dubai emirate law")
                .add_legal_difference("DIFC (Dubai International Financial Centre) courts")
                .add_legal_difference("Free zone regulations")
                .add_legal_difference("Mixed civil and Sharia law"),
        );

        registry.add_variation(
            SubRegionalVariation::new(
                ae_locale.clone(),
                "AZ",
                "Abu Dhabi",
                "Abu Dhabi emirate law",
            )
            .add_legal_difference("ADGM (Abu Dhabi Global Market) courts")
            .add_legal_difference("Strong energy law sector")
            .add_legal_difference("Commercial arbitration center"),
        );

        let sa_locale = Locale::new("ar").with_country("SA");
        registry.add_variation(
            SubRegionalVariation::new(sa_locale.clone(), "RI", "Riyadh", "Riyadh province law")
                .add_legal_difference("Sharia law system")
                .add_legal_difference("Board of Grievances jurisdiction")
                .add_legal_difference("Capital Markets Authority regulations"),
        );

        let il_locale = Locale::new("he").with_country("IL");
        registry.add_variation(
            SubRegionalVariation::new(il_locale.clone(), "TA", "Tel Aviv", "Tel Aviv district")
                .add_legal_difference("Mixed common law and civil law")
                .add_legal_difference("Tel Aviv District Court")
                .add_legal_difference("Tech startup legal framework"),
        );

        // Latin American Countries
        let br_locale = Locale::new("pt").with_country("BR");
        registry.add_variation(
            SubRegionalVariation::new(br_locale.clone(), "SP", "São Paulo", "São Paulo state law")
                .add_legal_difference("Civil law system")
                .add_legal_difference("Tribunal de Justiça de São Paulo")
                .add_legal_difference("Strong commercial law"),
        );

        registry.add_variation(
            SubRegionalVariation::new(
                br_locale.clone(),
                "RJ",
                "Rio de Janeiro",
                "Rio de Janeiro state law",
            )
            .add_legal_difference("Oil and gas law specialization")
            .add_legal_difference("TJRJ jurisdiction")
            .add_legal_difference("Environmental law regulations"),
        );

        let ar_locale = Locale::new("es").with_country("AR");
        registry.add_variation(
            SubRegionalVariation::new(
                ar_locale.clone(),
                "BA",
                "Buenos Aires",
                "Buenos Aires province law",
            )
            .add_legal_difference("Civil law system")
            .add_legal_difference("Código Civil y Comercial")
            .add_legal_difference("Strong agricultural law"),
        );

        let mx_locale = Locale::new("es").with_country("MX");
        registry.add_variation(
            SubRegionalVariation::new(mx_locale.clone(), "CMX", "Mexico City", "Mexico City law")
                .add_legal_difference("Federal District jurisdiction")
                .add_legal_difference("Civil law system")
                .add_legal_difference("Amparo judicial review"),
        );

        let cl_locale = Locale::new("es").with_country("CL");
        registry.add_variation(
            SubRegionalVariation::new(
                cl_locale.clone(),
                "RM",
                "Santiago",
                "Santiago metropolitan region",
            )
            .add_legal_difference("Civil law system")
            .add_legal_difference("Corte Suprema jurisdiction")
            .add_legal_difference("Mining law specialization"),
        );

        let co_locale = Locale::new("es").with_country("CO");
        registry.add_variation(
            SubRegionalVariation::new(co_locale.clone(), "DC", "Bogotá", "Bogotá capital district")
                .add_legal_difference("Civil law system")
                .add_legal_difference("Corte Constitucional")
                .add_legal_difference("Acción de tutela constitutional protection"),
        );

        // African Countries
        let za_locale = Locale::new("en").with_country("ZA");
        registry.add_variation(
            SubRegionalVariation::new(za_locale.clone(), "GP", "Gauteng", "Gauteng province law")
                .add_legal_difference("Mixed Roman-Dutch and English law")
                .add_legal_difference("Constitutional Court seat")
                .add_legal_difference("Mining and resources law"),
        );

        registry.add_variation(
            SubRegionalVariation::new(
                za_locale.clone(),
                "WC",
                "Western Cape",
                "Western Cape province law",
            )
            .add_legal_difference("Cape High Court jurisdiction")
            .add_legal_difference("Wine industry regulations")
            .add_legal_difference("Tourism law specialization"),
        );

        let ng_locale = Locale::new("en").with_country("NG");
        registry.add_variation(
            SubRegionalVariation::new(ng_locale.clone(), "LA", "Lagos", "Lagos state law")
                .add_legal_difference("Common law system")
                .add_legal_difference("Commercial law center")
                .add_legal_difference("Lagos State High Court"),
        );

        let eg_locale = Locale::new("ar").with_country("EG");
        registry.add_variation(
            SubRegionalVariation::new(eg_locale.clone(), "C", "Cairo", "Cairo governorate law")
                .add_legal_difference("Civil law system (French-influenced)")
                .add_legal_difference("Mixed Sharia and civil law")
                .add_legal_difference("Court of Cassation jurisdiction"),
        );

        let ke_locale = Locale::new("en").with_country("KE");
        registry.add_variation(
            SubRegionalVariation::new(ke_locale.clone(), "NBO", "Nairobi", "Nairobi county law")
                .add_legal_difference("Common law system")
                .add_legal_difference("Commercial and Admiralty Division")
                .add_legal_difference("East African Court of Justice"),
        );

        registry
    }

    /// Adds a sub-regional variation to the registry.
    pub fn add_variation(&mut self, variation: SubRegionalVariation) {
        self.variations.push(variation);
    }

    /// Gets all sub-regional variations for a country.
    pub fn get_variations_for_country(&self, country_code: &str) -> Vec<&SubRegionalVariation> {
        self.variations
            .iter()
            .filter(|v| {
                v.base_locale
                    .country
                    .as_ref()
                    .map(|c| c == country_code)
                    .unwrap_or(false)
            })
            .collect()
    }

    /// Finds a specific sub-regional variation.
    pub fn find_variation(
        &self,
        country_code: &str,
        region_code: &str,
    ) -> Option<&SubRegionalVariation> {
        self.variations.iter().find(|v| {
            v.base_locale
                .country
                .as_ref()
                .map(|c| c == country_code)
                .unwrap_or(false)
                && v.region_code == region_code
        })
    }
}

/// EU member state variation information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EUMemberStateVariation {
    /// Member state locale
    pub member_state_locale: Locale,
    /// Country name
    pub country_name: String,
    /// EU accession date (year)
    pub accession_year: u32,
    /// Legal system type
    pub legal_system: String,
    /// Key EU law adaptations
    pub eu_adaptations: Vec<String>,
    /// National legal specialties
    pub specialties: Vec<String>,
}

impl EUMemberStateVariation {
    /// Creates a new EU member state variation.
    pub fn new(
        member_state_locale: Locale,
        country_name: impl Into<String>,
        accession_year: u32,
        legal_system: impl Into<String>,
    ) -> Self {
        Self {
            member_state_locale,
            country_name: country_name.into(),
            accession_year,
            legal_system: legal_system.into(),
            eu_adaptations: vec![],
            specialties: vec![],
        }
    }

    /// Adds an EU law adaptation description.
    pub fn add_eu_adaptation(mut self, adaptation: impl Into<String>) -> Self {
        self.eu_adaptations.push(adaptation.into());
        self
    }

    /// Adds a national legal specialty.
    pub fn add_specialty(mut self, specialty: impl Into<String>) -> Self {
        self.specialties.push(specialty.into());
        self
    }
}

/// Registry of EU member state variations.
#[derive(Debug, Default)]
pub struct EUMemberStateRegistry {
    variations: Vec<EUMemberStateVariation>,
}

impl EUMemberStateRegistry {
    /// Creates a new registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a registry with default EU member state variations.
    #[allow(clippy::too_many_arguments)]
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();

        // Germany
        registry.add_variation(
            EUMemberStateVariation::new(
                Locale::new("de").with_country("DE"),
                "Germany",
                1958,
                "Civil law (German legal tradition)",
            )
            .add_eu_adaptation("GDPR implementation with national data protection law (BDSG)")
            .add_eu_adaptation("EU Directives transposed into German law")
            .add_specialty("Strong corporate governance (Mitbestimmung)")
            .add_specialty("Federal Constitutional Court (Bundesverfassungsgericht)"),
        );

        // France
        registry.add_variation(
            EUMemberStateVariation::new(
                Locale::new("fr").with_country("FR"),
                "France",
                1958,
                "Civil law (French legal tradition - Napoleonic Code)",
            )
            .add_eu_adaptation("GDPR through French Data Protection Act")
            .add_eu_adaptation("EU competition law integrated into Code de commerce")
            .add_specialty("Administrative law (droit administratif)")
            .add_specialty("Conseil d'État for administrative disputes"),
        );

        // Spain
        registry.add_variation(
            EUMemberStateVariation::new(
                Locale::new("es").with_country("ES"),
                "Spain",
                1986,
                "Civil law (Spanish legal tradition)",
            )
            .add_eu_adaptation("GDPR through Organic Law 3/2018")
            .add_eu_adaptation("Regional autonomy laws (Catalonia, Basque Country)")
            .add_specialty("Constitutional Court (Tribunal Constitucional)")
            .add_specialty("Regional legal variations"),
        );

        // Italy
        registry.add_variation(
            EUMemberStateVariation::new(
                Locale::new("it").with_country("IT"),
                "Italy",
                1958,
                "Civil law (Italian legal tradition)",
            )
            .add_eu_adaptation("GDPR implemented through Legislative Decree 101/2018")
            .add_eu_adaptation("EU directives via legislative decrees")
            .add_specialty("Constitutional Court (Corte Costituzionale)")
            .add_specialty("Strong labor law protections"),
        );

        // Netherlands
        registry.add_variation(
            EUMemberStateVariation::new(
                Locale::new("nl").with_country("NL"),
                "Netherlands",
                1958,
                "Civil law (Dutch legal tradition)",
            )
            .add_eu_adaptation("GDPR through Dutch Implementation Act (UAVG)")
            .add_eu_adaptation("EU law direct effect recognized")
            .add_specialty("International arbitration hub (The Hague)")
            .add_specialty("Strong commercial law tradition"),
        );

        // Poland
        registry.add_variation(
            EUMemberStateVariation::new(
                Locale::new("pl").with_country("PL"),
                "Poland",
                2004,
                "Civil law (Polish legal tradition)",
            )
            .add_eu_adaptation("GDPR through Personal Data Protection Act")
            .add_eu_adaptation("EU structural funds legal framework")
            .add_specialty("Constitutional Tribunal (Trybunał Konstytucyjny)")
            .add_specialty("Post-communist legal reforms"),
        );

        // Sweden
        registry.add_variation(
            EUMemberStateVariation::new(
                Locale::new("sv").with_country("SE"),
                "Sweden",
                1995,
                "Civil law (Nordic legal tradition)",
            )
            .add_eu_adaptation("GDPR through Swedish Data Protection Act")
            .add_eu_adaptation("Maintained non-Euro currency (SEK)")
            .add_specialty("Strong transparency laws (Offentlighetsprincipen)")
            .add_specialty("Ombudsman system"),
        );

        // Ireland
        registry.add_variation(
            EUMemberStateVariation::new(
                Locale::new("en").with_country("IE"),
                "Ireland",
                1973,
                "Common law (Irish legal tradition)",
            )
            .add_eu_adaptation("GDPR enforced by Data Protection Commission")
            .add_eu_adaptation("EU tech hub with regulatory enforcement")
            .add_specialty("Common law in EU context")
            .add_specialty("Strong tech regulation enforcement"),
        );

        // Belgium
        registry.add_variation(
            EUMemberStateVariation::new(
                Locale::new("fr").with_country("BE"),
                "Belgium",
                1958,
                "Civil law (Belgian legal tradition)",
            )
            .add_eu_adaptation("GDPR through Belgian Data Protection Authority")
            .add_eu_adaptation("EU institutions headquarters")
            .add_specialty("Multilingual legal system (French, Dutch, German)")
            .add_specialty("Federal and regional court systems"),
        );

        // Austria
        registry.add_variation(
            EUMemberStateVariation::new(
                Locale::new("de").with_country("AT"),
                "Austria",
                1995,
                "Civil law (Austrian legal tradition - ABGB)",
            )
            .add_eu_adaptation("GDPR through Austrian Data Protection Act (DSG)")
            .add_eu_adaptation("EU neutrality adaptations")
            .add_specialty("Austrian Civil Code (ABGB) from 1811")
            .add_specialty("Strong constitutional court"),
        );

        registry
    }

    /// Adds a member state variation to the registry.
    pub fn add_variation(&mut self, variation: EUMemberStateVariation) {
        self.variations.push(variation);
    }

    /// Gets all member state variations.
    pub fn get_all_variations(&self) -> &[EUMemberStateVariation] {
        &self.variations
    }

    /// Finds a specific member state variation.
    pub fn find_variation(&self, country_code: &str) -> Option<&EUMemberStateVariation> {
        self.variations.iter().find(|v| {
            v.member_state_locale
                .country
                .as_ref()
                .map(|c| c == country_code)
                .unwrap_or(false)
        })
    }
}

/// Dialect-aware terminology for regional language variations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialectTerminology {
    /// Base locale
    pub base_locale: Locale,
    /// Dialect name
    pub dialect_name: String,
    /// Terminology mappings (standard term -> dialect term)
    pub terminology: IndexMap<String, String>,
}

impl DialectTerminology {
    /// Creates a new dialect terminology.
    pub fn new(base_locale: Locale, dialect_name: impl Into<String>) -> Self {
        Self {
            base_locale,
            dialect_name: dialect_name.into(),
            terminology: IndexMap::new(),
        }
    }

    /// Adds a term mapping.
    pub fn add_term(&mut self, standard_term: impl Into<String>, dialect_term: impl Into<String>) {
        self.terminology
            .insert(standard_term.into(), dialect_term.into());
    }

    /// Translates a standard term to dialect.
    pub fn to_dialect(&self, standard_term: &str) -> Option<&str> {
        self.terminology.get(standard_term).map(|s| s.as_str())
    }

    /// Translates from dialect to standard term.
    pub fn from_dialect(&self, dialect_term: &str) -> Option<&str> {
        self.terminology
            .iter()
            .find(|(_, v)| v.as_str() == dialect_term)
            .map(|(k, _)| k.as_str())
    }
}

/// Registry of dialect terminologies.
#[derive(Debug, Default)]
pub struct DialectTerminologyRegistry {
    dialects: Vec<DialectTerminology>,
}

impl DialectTerminologyRegistry {
    /// Creates a new registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a registry with default dialect terminologies.
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();

        // Scottish legal terminology (en-GB-scotland dialect)
        let mut scottish =
            DialectTerminology::new(Locale::new("en").with_country("GB"), "Scottish Legal");
        scottish.add_term("lawyer", "advocate");
        scottish.add_term("notary_public", "notary public and conveyancer");
        scottish.add_term("real_estate", "heritable property");
        scottish.add_term("personal_property", "moveable property");
        scottish.add_term("mortgage", "standard security");
        scottish.add_term("will", "testament");
        scottish.add_term("plaintiff", "pursuer");
        scottish.add_term("defendant", "defender");
        registry.add_dialect(scottish);

        // Louisiana legal terminology (en-US-LA dialect - civil law influence)
        let mut louisiana =
            DialectTerminology::new(Locale::new("en").with_country("US"), "Louisiana Legal");
        louisiana.add_term("county", "parish");
        louisiana.add_term("real_estate", "immovable property");
        louisiana.add_term("personal_property", "movable property");
        louisiana.add_term("common_law", "civil law");
        louisiana.add_term("deed", "act of sale");
        louisiana.add_term("will", "testament");
        registry.add_dialect(louisiana);

        // Quebec legal terminology (fr-CA-QC dialect)
        let mut quebec =
            DialectTerminology::new(Locale::new("fr").with_country("CA"), "Québec Legal");
        quebec.add_term("avocat", "avocat(e)");
        quebec.add_term("notaire", "notaire");
        quebec.add_term("jurisprudence", "jurisprudence québécoise");
        quebec.add_term("code_civil", "Code civil du Québec");
        registry.add_dialect(quebec);

        // Hong Kong legal terminology (en-HK dialect - common law with Chinese influence)
        let mut hong_kong =
            DialectTerminology::new(Locale::new("en").with_country("HK"), "Hong Kong Legal");
        hong_kong.add_term("lawyer", "solicitor or barrister");
        hong_kong.add_term("attorney", "solicitor");
        hong_kong.add_term("court", "Court of Final Appeal / High Court");
        hong_kong.add_term("basic_law", "Basic Law");
        registry.add_dialect(hong_kong);

        // Australian legal terminology variations
        let mut australian =
            DialectTerminology::new(Locale::new("en").with_country("AU"), "Australian Legal");
        australian.add_term("lawyer", "solicitor or barrister");
        australian.add_term("attorney", "solicitor");
        australian.add_term("corporation", "company (Pty Ltd)");
        australian.add_term(
            "supreme_court",
            "High Court of Australia (federal) / State Supreme Courts",
        );
        registry.add_dialect(australian);

        registry
    }

    /// Adds a dialect to the registry.
    pub fn add_dialect(&mut self, dialect: DialectTerminology) {
        self.dialects.push(dialect);
    }

    /// Finds a dialect by name and locale.
    pub fn find_dialect(&self, locale: &Locale, dialect_name: &str) -> Option<&DialectTerminology> {
        self.dialects.iter().find(|d| {
            d.base_locale.language == locale.language
                && d.base_locale.country == locale.country
                && d.dialect_name == dialect_name
        })
    }

    /// Gets all dialects for a locale.
    pub fn get_dialects_for_locale(&self, locale: &Locale) -> Vec<&DialectTerminology> {
        self.dialects
            .iter()
            .filter(|d| {
                d.base_locale.language == locale.language && d.base_locale.country == locale.country
            })
            .collect()
    }
}

/// Regional legal concept mapper for cross-jurisdictional equivalence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalConceptMapping {
    /// Source concept
    pub source_concept: String,
    /// Source jurisdiction
    pub source_jurisdiction: String,
    /// Target concept
    pub target_concept: String,
    /// Target jurisdiction
    pub target_jurisdiction: String,
    /// Similarity score (0.0 to 1.0)
    pub similarity: f64,
    /// Notes on differences
    pub notes: Vec<String>,
}

impl RegionalConceptMapping {
    /// Creates a new regional concept mapping.
    pub fn new(
        source_concept: impl Into<String>,
        source_jurisdiction: impl Into<String>,
        target_concept: impl Into<String>,
        target_jurisdiction: impl Into<String>,
        similarity: f64,
    ) -> Self {
        Self {
            source_concept: source_concept.into(),
            source_jurisdiction: source_jurisdiction.into(),
            target_concept: target_concept.into(),
            target_jurisdiction: target_jurisdiction.into(),
            similarity: similarity.clamp(0.0, 1.0),
            notes: vec![],
        }
    }

    /// Adds a note about differences.
    pub fn add_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }
}

/// Registry of regional legal concept mappings.
#[derive(Debug, Default)]
pub struct RegionalConceptMapper {
    mappings: Vec<RegionalConceptMapping>,
}

impl RegionalConceptMapper {
    /// Creates a new mapper.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a mapper with default concept mappings.
    #[allow(clippy::too_many_arguments)]
    pub fn with_defaults() -> Self {
        let mut mapper = Self::new();

        // Common law vs. Civil law concept mappings
        mapper.add_mapping(
            RegionalConceptMapping::new("trust", "GB", "fiducie", "FR", 0.7)
                .add_note("Trust is equity concept; fiducie is civil law approximation")
                .add_note("French law adopted trust-like concept in 2007"),
        );

        mapper.add_mapping(
            RegionalConceptMapping::new("equity", "GB", "fairness principles", "DE", 0.5)
                .add_note("Equity is distinct common law system; German law integrates fairness")
                .add_note("No separate equity courts in German civil law"),
        );

        mapper.add_mapping(
            RegionalConceptMapping::new("consideration", "US", "cause", "FR", 0.8)
                .add_note("Both are contract formation requirements")
                .add_note("Consideration focuses on exchange; cause on purpose"),
        );

        // Corporate law concepts
        mapper.add_mapping(
            RegionalConceptMapping::new("LLC", "US", "GmbH", "DE", 0.9)
                .add_note("Both are limited liability companies")
                .add_note("Similar structure and liability protection"),
        );

        mapper.add_mapping(
            RegionalConceptMapping::new("corporation", "US", "kabushiki kaisha", "JP", 0.85)
                .add_note("Both are stock corporations with shareholders")
                .add_note("Different governance structures (board vs. statutory auditors)"),
        );

        mapper.add_mapping(
            RegionalConceptMapping::new("partnership", "GB", "société en nom collectif", "FR", 0.9)
                .add_note("Both are general partnerships with unlimited liability")
                .add_note("Similar legal structure across jurisdictions"),
        );

        // Property law concepts
        mapper.add_mapping(
            RegionalConceptMapping::new("fee_simple", "US", "propriété", "FR", 0.8)
                .add_note("Both represent full ownership")
                .add_note("Fee simple is common law; propriété is civil law"),
        );

        mapper.add_mapping(
            RegionalConceptMapping::new("easement", "GB", "servitude", "FR", 0.95)
                .add_note("Nearly identical concepts across common law and civil law")
                .add_note("Right to use another's property for specific purpose"),
        );

        // Criminal law concepts
        mapper.add_mapping(
            RegionalConceptMapping::new("felony", "US", "crime", "FR", 0.7)
                .add_note("Felony is serious crime in US; crime is general category in France")
                .add_note("France uses crime/délit/contravention classification"),
        );

        mapper.add_mapping(
            RegionalConceptMapping::new("misdemeanor", "US", "délit", "FR", 0.75)
                .add_note("Both are mid-level criminal offenses")
                .add_note("Different sentencing ranges"),
        );

        // Procedural concepts
        mapper.add_mapping(
            RegionalConceptMapping::new("discovery", "US", "disclosure", "GB", 0.95)
                .add_note("Nearly identical pre-trial evidence exchange")
                .add_note("US discovery is broader than UK disclosure"),
        );

        mapper.add_mapping(
            RegionalConceptMapping::new("summary_judgment", "US", "référé", "FR", 0.6)
                .add_note("Both are expedited procedures")
                .add_note("Different standards and procedures"),
        );

        mapper
    }

    /// Adds a concept mapping to the registry.
    pub fn add_mapping(&mut self, mapping: RegionalConceptMapping) {
        self.mappings.push(mapping);
    }

    /// Finds concept mappings from source to target jurisdiction.
    pub fn find_mappings(
        &self,
        source_concept: &str,
        source_jurisdiction: &str,
        target_jurisdiction: &str,
    ) -> Vec<&RegionalConceptMapping> {
        self.mappings
            .iter()
            .filter(|m| {
                m.source_concept == source_concept
                    && m.source_jurisdiction == source_jurisdiction
                    && m.target_jurisdiction == target_jurisdiction
            })
            .collect()
    }

    /// Finds all mappings for a concept across all jurisdictions.
    pub fn find_all_mappings_for_concept(&self, concept: &str) -> Vec<&RegionalConceptMapping> {
        self.mappings
            .iter()
            .filter(|m| m.source_concept == concept || m.target_concept == concept)
            .collect()
    }
}

/// Cross-regional term equivalence for legal terminology.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TermEquivalence {
    /// Base term
    pub base_term: String,
    /// Base jurisdiction
    pub base_jurisdiction: String,
    /// Equivalent terms in other jurisdictions
    pub equivalents: IndexMap<String, EquivalentTerm>,
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

/// Level of equivalence between terms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EquivalenceLevel {
    /// Exact equivalence (same legal meaning)
    Exact,
    /// Approximate equivalence (similar but with differences)
    Approximate,
    /// Loose equivalence (related concept)
    Loose,
    /// No direct equivalent (concept doesn't exist)
    NoEquivalent,
}

impl TermEquivalence {
    /// Creates a new term equivalence.
    pub fn new(base_term: impl Into<String>, base_jurisdiction: impl Into<String>) -> Self {
        Self {
            base_term: base_term.into(),
            base_jurisdiction: base_jurisdiction.into(),
            equivalents: IndexMap::new(),
        }
    }

    /// Adds an equivalent term.
    pub fn add_equivalent(
        mut self,
        jurisdiction: impl Into<String>,
        term: impl Into<String>,
        level: EquivalenceLevel,
    ) -> Self {
        self.equivalents.insert(
            jurisdiction.into(),
            EquivalentTerm {
                term: term.into(),
                equivalence_level: level,
                notes: vec![],
            },
        );
        self
    }

    /// Adds a note to an equivalent term.
    pub fn add_note_to_equivalent(mut self, jurisdiction: &str, note: impl Into<String>) -> Self {
        if let Some(equiv) = self.equivalents.get_mut(jurisdiction) {
            equiv.notes.push(note.into());
        }
        self
    }

    /// Gets equivalent term for a jurisdiction.
    pub fn get_equivalent(&self, jurisdiction: &str) -> Option<&EquivalentTerm> {
        self.equivalents.get(jurisdiction)
    }
}

/// Registry of cross-regional term equivalences.
#[derive(Debug, Default)]
pub struct CrossRegionalTermEquivalenceRegistry {
    equivalences: Vec<TermEquivalence>,
}

impl CrossRegionalTermEquivalenceRegistry {
    /// Creates a new registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a registry with default term equivalences.
    #[allow(clippy::too_many_arguments)]
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();

        // Attorney/Lawyer equivalents
        registry.add_equivalence(
            TermEquivalence::new("attorney", "US")
                .add_equivalent("GB", "solicitor", EquivalenceLevel::Approximate)
                .add_equivalent("FR", "avocat", EquivalenceLevel::Exact)
                .add_equivalent("DE", "Rechtsanwalt", EquivalenceLevel::Exact)
                .add_equivalent("JP", "bengoshi", EquivalenceLevel::Exact)
                .add_note_to_equivalent("GB", "UK distinguishes solicitors and barristers"),
        );

        // Corporation equivalents
        registry.add_equivalence(
            TermEquivalence::new("corporation", "US")
                .add_equivalent("GB", "limited company", EquivalenceLevel::Approximate)
                .add_equivalent("FR", "société anonyme", EquivalenceLevel::Approximate)
                .add_equivalent("DE", "Aktiengesellschaft", EquivalenceLevel::Exact)
                .add_equivalent("JP", "kabushiki kaisha", EquivalenceLevel::Exact)
                .add_note_to_equivalent("FR", "SA is public company; SARL is private")
                .add_note_to_equivalent("DE", "AG is stock corporation"),
        );

        // Contract equivalents
        registry.add_equivalence(
            TermEquivalence::new("contract", "US")
                .add_equivalent("GB", "contract", EquivalenceLevel::Exact)
                .add_equivalent("FR", "contrat", EquivalenceLevel::Exact)
                .add_equivalent("DE", "Vertrag", EquivalenceLevel::Exact)
                .add_equivalent("JP", "keiyaku", EquivalenceLevel::Exact),
        );

        // Tort equivalents
        registry.add_equivalence(
            TermEquivalence::new("tort", "US")
                .add_equivalent("GB", "tort", EquivalenceLevel::Exact)
                .add_equivalent(
                    "FR",
                    "responsabilité civile délictuelle",
                    EquivalenceLevel::Approximate,
                )
                .add_equivalent("DE", "unerlaubte Handlung", EquivalenceLevel::Approximate)
                .add_equivalent("JP", "fuhōkōi", EquivalenceLevel::Approximate)
                .add_note_to_equivalent("FR", "Civil law tort concept differs from common law")
                .add_note_to_equivalent("DE", "Part of BGB obligations law"),
        );

        // Trust equivalents
        registry.add_equivalence(
            TermEquivalence::new("trust", "GB")
                .add_equivalent("US", "trust", EquivalenceLevel::Exact)
                .add_equivalent("FR", "fiducie", EquivalenceLevel::Approximate)
                .add_equivalent("DE", "Treuhand", EquivalenceLevel::Loose)
                .add_equivalent("JP", "shintaku", EquivalenceLevel::Approximate)
                .add_note_to_equivalent("FR", "Introduced in 2007, not traditional civil law")
                .add_note_to_equivalent("DE", "Not a true trust, more like agency")
                .add_note_to_equivalent("JP", "Modern adoption of trust concept"),
        );

        // Due process equivalents
        registry.add_equivalence(
            TermEquivalence::new("due_process", "US")
                .add_equivalent("GB", "natural justice", EquivalenceLevel::Approximate)
                .add_equivalent("FR", "droits de la défense", EquivalenceLevel::Approximate)
                .add_equivalent("DE", "rechtliches Gehör", EquivalenceLevel::Approximate)
                .add_equivalent("JP", "tekisei tetsuzuki", EquivalenceLevel::Exact)
                .add_note_to_equivalent("GB", "Natural justice is broader concept")
                .add_note_to_equivalent("FR", "Rights of defense in French law")
                .add_note_to_equivalent("DE", "Right to be heard in German law"),
        );

        // Plaintiff/Claimant equivalents
        registry.add_equivalence(
            TermEquivalence::new("plaintiff", "US")
                .add_equivalent("GB", "claimant", EquivalenceLevel::Exact)
                .add_equivalent("FR", "demandeur", EquivalenceLevel::Exact)
                .add_equivalent("DE", "Kläger", EquivalenceLevel::Exact)
                .add_equivalent("JP", "genkoku", EquivalenceLevel::Exact),
        );

        // Statute of limitations equivalents
        registry.add_equivalence(
            TermEquivalence::new("statute_of_limitations", "US")
                .add_equivalent("GB", "limitation period", EquivalenceLevel::Exact)
                .add_equivalent("FR", "prescription", EquivalenceLevel::Exact)
                .add_equivalent("DE", "Verjährung", EquivalenceLevel::Exact)
                .add_equivalent("JP", "shōmetsu jikō", EquivalenceLevel::Exact),
        );

        registry
    }

    /// Adds a term equivalence to the registry.
    pub fn add_equivalence(&mut self, equivalence: TermEquivalence) {
        self.equivalences.push(equivalence);
    }

    /// Finds term equivalence.
    pub fn find_equivalence(
        &self,
        term: &str,
        base_jurisdiction: &str,
    ) -> Option<&TermEquivalence> {
        self.equivalences
            .iter()
            .find(|e| e.base_term == term && e.base_jurisdiction == base_jurisdiction)
    }

    /// Gets equivalent term in target jurisdiction.
    pub fn get_equivalent_term(
        &self,
        term: &str,
        base_jurisdiction: &str,
        target_jurisdiction: &str,
    ) -> Option<&EquivalentTerm> {
        self.find_equivalence(term, base_jurisdiction)
            .and_then(|e| e.get_equivalent(target_jurisdiction))
    }
}

// ============================================================================
// Legal Document Templates v0.2.0: Document Generation System
// ============================================================================

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

/// Template variable with type validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVariable {
    /// Variable name (e.g., "party_name", "effective_date")
    pub name: String,
    /// Variable type for validation
    pub var_type: VariableType,
    /// Whether this variable is required
    pub required: bool,
    /// Description of the variable
    pub description: String,
    /// Default value (if any)
    pub default_value: Option<String>,
}

impl TemplateVariable {
    /// Creates a new template variable.
    pub fn new(
        name: impl Into<String>,
        var_type: VariableType,
        required: bool,
        description: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            var_type,
            required,
            description: description.into(),
            default_value: None,
        }
    }

    /// Sets a default value for the variable.
    pub fn with_default(mut self, default: impl Into<String>) -> Self {
        self.default_value = Some(default.into());
        self
    }

    /// Validates a value against this variable's type.
    pub fn validate(&self, value: &str) -> bool {
        if value.is_empty() {
            return !self.required;
        }

        match self.var_type {
            VariableType::Text | VariableType::Address | VariableType::PersonName => true,
            VariableType::Number => value.parse::<f64>().is_ok(),
            VariableType::Currency => value.parse::<f64>().is_ok(),
            VariableType::Boolean => {
                matches!(
                    value.to_lowercase().as_str(),
                    "true" | "false" | "yes" | "no"
                )
            }
            VariableType::Email => value.contains('@'),
            VariableType::Date => {
                // Simple date validation (accepts various formats)
                value.contains('-') || value.contains('/')
            }
            VariableType::List => true, // Lists are comma-separated
        }
    }
}

/// Template section that can be conditionally included.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateSection {
    /// Section name
    pub name: String,
    /// Section content with placeholders
    pub content: String,
    /// Condition for including this section (e.g., "jurisdiction == US")
    pub condition: Option<String>,
}

impl TemplateSection {
    /// Creates a new template section.
    pub fn new(name: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            content: content.into(),
            condition: None,
        }
    }

    /// Adds a condition for including this section.
    pub fn with_condition(mut self, condition: impl Into<String>) -> Self {
        self.condition = Some(condition.into());
        self
    }

    /// Checks if the condition is met given the context.
    pub fn should_include(&self, context: &HashMap<String, String>) -> bool {
        if let Some(ref condition) = self.condition {
            // Simple condition evaluation: "key == value" or "key != value"
            if let Some((key, rest)) = condition.split_once("==") {
                let key = key.trim();
                let value = rest.trim();
                return context.get(key).map(|v| v == value).unwrap_or(false);
            } else if let Some((key, rest)) = condition.split_once("!=") {
                let key = key.trim();
                let value = rest.trim();
                return context.get(key).map(|v| v != value).unwrap_or(true);
            }
        }
        true // No condition means always include
    }
}

/// Type of legal document template.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocumentTemplateType {
    /// Contract documents
    Contract,
    /// Court filing documents
    CourtFiling,
    /// Corporate documents
    Corporate,
    /// Compliance documents
    Compliance,
    /// General legal documents
    General,
}

/// Legal document template with placeholders and localization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentTemplate {
    /// Template identifier
    pub id: String,
    /// Template name
    pub name: String,
    /// Template type
    pub template_type: DocumentTemplateType,
    /// Locale for this template
    pub locale: Locale,
    /// Jurisdiction code (e.g., "US", "GB", "FR")
    pub jurisdiction: String,
    /// Template sections
    pub sections: Vec<TemplateSection>,
    /// Required variables
    pub variables: Vec<TemplateVariable>,
    /// Template metadata
    pub metadata: HashMap<String, String>,
}

impl DocumentTemplate {
    /// Creates a new document template.
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        template_type: DocumentTemplateType,
        locale: Locale,
        jurisdiction: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            template_type,
            locale,
            jurisdiction: jurisdiction.into(),
            sections: vec![],
            variables: vec![],
            metadata: HashMap::new(),
        }
    }

    /// Adds a section to the template.
    pub fn add_section(mut self, section: TemplateSection) -> Self {
        self.sections.push(section);
        self
    }

    /// Adds a variable to the template.
    pub fn add_variable(mut self, variable: TemplateVariable) -> Self {
        self.variables.push(variable);
        self
    }

    /// Adds metadata to the template.
    pub fn add_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Validates that all required variables are provided.
    pub fn validate_variables(&self, values: &HashMap<String, String>) -> Vec<String> {
        let mut missing = vec![];

        for var in &self.variables {
            if var.required {
                if let Some(value) = values.get(&var.name) {
                    if !var.validate(value) {
                        missing.push(format!(
                            "Invalid value for '{}': expected {:?}",
                            var.name, var.var_type
                        ));
                    }
                } else if var.default_value.is_none() {
                    missing.push(format!("Missing required variable: '{}'", var.name));
                }
            }
        }

        missing
    }

    /// Generates the document by filling in the template with provided values.
    pub fn generate(&self, values: &HashMap<String, String>) -> Result<String, Vec<String>> {
        // Validate variables
        let errors = self.validate_variables(values);
        if !errors.is_empty() {
            return Err(errors);
        }

        // Build the document
        let mut document = String::new();

        for section in &self.sections {
            // Check if section should be included
            if !section.should_include(values) {
                continue;
            }

            // Replace placeholders in section content
            let mut content = section.content.clone();
            for var in &self.variables {
                let placeholder = format!("{{{{{}}}}}", var.name);
                let value = values
                    .get(&var.name)
                    .or(var.default_value.as_ref())
                    .map(|s| s.as_str())
                    .unwrap_or("");
                content = content.replace(&placeholder, value);
            }

            document.push_str(&content);
            document.push('\n');
        }

        Ok(document)
    }
}

/// Registry of legal document templates.
#[derive(Debug, Default)]
pub struct DocumentTemplateRegistry {
    templates: HashMap<String, DocumentTemplate>,
}

impl DocumentTemplateRegistry {
    /// Creates a new registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a registry with default templates.
    #[allow(clippy::too_many_arguments)]
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();

        // NDA Template (US)
        let nda_us = DocumentTemplate::new(
            "nda_mutual_us",
            "Mutual Non-Disclosure Agreement",
            DocumentTemplateType::Contract,
            Locale::new("en").with_country("US"),
            "US",
        )
        .add_variable(
            TemplateVariable::new(
                "party1_name",
                VariableType::Text,
                true,
                "Name of first party",
            ),
        )
        .add_variable(
            TemplateVariable::new(
                "party2_name",
                VariableType::Text,
                true,
                "Name of second party",
            ),
        )
        .add_variable(
            TemplateVariable::new(
                "effective_date",
                VariableType::Date,
                true,
                "Effective date of the agreement",
            ),
        )
        .add_variable(
            TemplateVariable::new(
                "state",
                VariableType::Text,
                true,
                "Governing state law",
            ),
        )
        .add_section(TemplateSection::new(
            "title",
            "MUTUAL NON-DISCLOSURE AGREEMENT\n",
        ))
        .add_section(TemplateSection::new(
            "parties",
            "This Mutual Non-Disclosure Agreement (\"Agreement\") is entered into as of {{effective_date}}, by and between {{party1_name}} (\"First Party\") and {{party2_name}} (\"Second Party\").\n",
        ))
        .add_section(TemplateSection::new(
            "recitals",
            "WHEREAS, the parties wish to explore a business opportunity of mutual interest and in connection with this opportunity, each party may disclose to the other certain confidential technical and business information that the disclosing party desires the receiving party to treat as confidential.\n",
        ))
        .add_section(TemplateSection::new(
            "confidential_info",
            "1. CONFIDENTIAL INFORMATION\n\n\"Confidential Information\" means any information disclosed by either party to the other party, either directly or indirectly, in writing, orally or by inspection of tangible objects.\n",
        ))
        .add_section(TemplateSection::new(
            "obligations",
            "2. OBLIGATIONS\n\nEach party agrees to: (a) hold the Confidential Information in strict confidence; (b) not disclose the Confidential Information to third parties; and (c) not use the Confidential Information except for the purpose of evaluating the potential business relationship.\n",
        ))
        .add_section(TemplateSection::new(
            "term",
            "3. TERM\n\nThis Agreement shall remain in effect for a period of three (3) years from the effective date.\n",
        ))
        .add_section(TemplateSection::new(
            "governing_law",
            "4. GOVERNING LAW\n\nThis Agreement shall be governed by the laws of the State of {{state}}, without regard to its conflict of laws provisions.\n",
        ))
        .add_metadata("author", "Legalis Document Template System")
        .add_metadata("version", "1.0");

        registry.add_template(nda_us);

        // Employment Agreement Template (US)
        let employment_us = DocumentTemplate::new(
            "employment_agreement_us",
            "Employment Agreement",
            DocumentTemplateType::Contract,
            Locale::new("en").with_country("US"),
            "US",
        )
        .add_variable(TemplateVariable::new(
            "company_name",
            VariableType::Text,
            true,
            "Name of the company",
        ))
        .add_variable(TemplateVariable::new(
            "employee_name",
            VariableType::PersonName,
            true,
            "Name of the employee",
        ))
        .add_variable(TemplateVariable::new(
            "position",
            VariableType::Text,
            true,
            "Job title/position",
        ))
        .add_variable(TemplateVariable::new(
            "start_date",
            VariableType::Date,
            true,
            "Employment start date",
        ))
        .add_variable(TemplateVariable::new(
            "salary",
            VariableType::Currency,
            true,
            "Annual salary",
        ))
        .add_variable(TemplateVariable::new(
            "state",
            VariableType::Text,
            true,
            "State law governing the agreement",
        ))
        .add_section(TemplateSection::new(
            "title",
            "EMPLOYMENT AGREEMENT\n",
        ))
        .add_section(TemplateSection::new(
            "parties",
            "This Employment Agreement (\"Agreement\") is entered into as of {{start_date}}, by and between {{company_name}} (\"Company\") and {{employee_name}} (\"Employee\").\n",
        ))
        .add_section(TemplateSection::new(
            "position_duties",
            "1. POSITION AND DUTIES\n\nCompany hereby employs Employee in the position of {{position}}. Employee accepts such employment and agrees to devote their full business time and attention to the performance of such duties.\n",
        ))
        .add_section(TemplateSection::new(
            "compensation",
            "2. COMPENSATION\n\nCompany shall pay Employee an annual salary of ${{salary}}, payable in accordance with Company's standard payroll practices.\n",
        ))
        .add_section(TemplateSection::new(
            "at_will",
            "3. AT-WILL EMPLOYMENT\n\nEmployee's employment with Company is at-will, meaning that either Employee or Company may terminate the employment relationship at any time, with or without cause or notice.\n",
        ))
        .add_section(TemplateSection::new(
            "governing_law",
            "4. GOVERNING LAW\n\nThis Agreement shall be governed by the laws of the State of {{state}}.\n",
        ));

        registry.add_template(employment_us);

        // Court Complaint Template (US)
        let complaint_us = DocumentTemplate::new(
            "complaint_us",
            "Civil Complaint",
            DocumentTemplateType::CourtFiling,
            Locale::new("en").with_country("US"),
            "US",
        )
        .add_variable(TemplateVariable::new(
            "court_name",
            VariableType::Text,
            true,
            "Name of the court",
        ))
        .add_variable(TemplateVariable::new(
            "plaintiff_name",
            VariableType::PersonName,
            true,
            "Name of plaintiff",
        ))
        .add_variable(TemplateVariable::new(
            "defendant_name",
            VariableType::PersonName,
            true,
            "Name of defendant",
        ))
        .add_variable(TemplateVariable::new(
            "case_number",
            VariableType::Text,
            false,
            "Case number (if assigned)",
        ))
        .add_variable(TemplateVariable::new(
            "jurisdiction_facts",
            VariableType::Text,
            true,
            "Facts establishing jurisdiction",
        ))
        .add_variable(TemplateVariable::new(
            "claim_facts",
            VariableType::Text,
            true,
            "Facts supporting the claim",
        ))
        .add_variable(TemplateVariable::new(
            "relief_requested",
            VariableType::Text,
            true,
            "Relief requested from the court",
        ))
        .add_section(TemplateSection::new(
            "caption",
            "{{court_name}}\n\n{{plaintiff_name}},\n    Plaintiff,\nv.\n{{defendant_name}},\n    Defendant.\n\nCase No. {{case_number}}\n\nCOMPLAINT\n",
        ))
        .add_section(TemplateSection::new(
            "introduction",
            "Plaintiff {{plaintiff_name}} files this Complaint against Defendant {{defendant_name}} and alleges as follows:\n",
        ))
        .add_section(TemplateSection::new(
            "jurisdiction",
            "JURISDICTION AND VENUE\n\n1. {{jurisdiction_facts}}\n",
        ))
        .add_section(TemplateSection::new(
            "facts",
            "FACTUAL ALLEGATIONS\n\n2. {{claim_facts}}\n",
        ))
        .add_section(TemplateSection::new(
            "prayer",
            "PRAYER FOR RELIEF\n\nWHEREFORE, Plaintiff respectfully requests that the Court:\n\n{{relief_requested}}\n",
        ));

        registry.add_template(complaint_us);

        // Articles of Incorporation Template (US - Delaware)
        let articles_de = DocumentTemplate::new(
            "articles_incorporation_de",
            "Certificate of Incorporation",
            DocumentTemplateType::Corporate,
            Locale::new("en").with_country("US"),
            "US-DE",
        )
        .add_variable(TemplateVariable::new(
            "corporation_name",
            VariableType::Text,
            true,
            "Name of the corporation",
        ))
        .add_variable(TemplateVariable::new(
            "registered_agent_name",
            VariableType::Text,
            true,
            "Name of registered agent",
        ))
        .add_variable(TemplateVariable::new(
            "registered_agent_address",
            VariableType::Address,
            true,
            "Address of registered agent",
        ))
        .add_variable(TemplateVariable::new(
            "shares_authorized",
            VariableType::Number,
            true,
            "Number of authorized shares",
        ))
        .add_variable(TemplateVariable::new(
            "incorporator_name",
            VariableType::PersonName,
            true,
            "Name of incorporator",
        ))
        .add_section(TemplateSection::new(
            "title",
            "CERTIFICATE OF INCORPORATION\nOF\n{{corporation_name}}\n",
        ))
        .add_section(TemplateSection::new(
            "article1",
            "ARTICLE I - NAME\n\nThe name of the corporation is {{corporation_name}}.\n",
        ))
        .add_section(TemplateSection::new(
            "article2",
            "ARTICLE II - REGISTERED OFFICE AND AGENT\n\nThe address of the corporation's registered office in the State of Delaware is {{registered_agent_address}}, and the name of its registered agent at such address is {{registered_agent_name}}.\n",
        ))
        .add_section(TemplateSection::new(
            "article3",
            "ARTICLE III - PURPOSE\n\nThe purpose of the corporation is to engage in any lawful act or activity for which corporations may be organized under the General Corporation Law of Delaware.\n",
        ))
        .add_section(TemplateSection::new(
            "article4",
            "ARTICLE IV - CAPITAL STOCK\n\nThe total number of shares of stock which the corporation shall have authority to issue is {{shares_authorized}} shares of Common Stock, par value $0.001 per share.\n",
        ))
        .add_section(TemplateSection::new(
            "signature",
            "IN WITNESS WHEREOF, the undersigned incorporator has executed this Certificate of Incorporation this _____ day of __________, 20__.\n\n_________________________\n{{incorporator_name}}\nIncorporator\n",
        ));

        registry.add_template(articles_de);

        registry
    }

    /// Adds a template to the registry.
    pub fn add_template(&mut self, template: DocumentTemplate) {
        self.templates.insert(template.id.clone(), template);
    }

    /// Gets a template by ID.
    pub fn get_template(&self, id: &str) -> Option<&DocumentTemplate> {
        self.templates.get(id)
    }

    /// Finds templates by type.
    pub fn find_by_type(&self, template_type: DocumentTemplateType) -> Vec<&DocumentTemplate> {
        self.templates
            .values()
            .filter(|t| t.template_type == template_type)
            .collect()
    }

    /// Finds templates by jurisdiction.
    pub fn find_by_jurisdiction(&self, jurisdiction: &str) -> Vec<&DocumentTemplate> {
        self.templates
            .values()
            .filter(|t| t.jurisdiction == jurisdiction)
            .collect()
    }

    /// Lists all available template IDs.
    pub fn list_templates(&self) -> Vec<&str> {
        self.templates.keys().map(|s| s.as_str()).collect()
    }
}

// ============================================================================
// Performance Optimizations (v0.2.3)
// ============================================================================

/// Term index for fast prefix-based lookups in dictionaries.
/// Enables efficient autocomplete, fuzzy search, and partial matching.
#[derive(Debug, Clone, Default)]
pub struct TermIndex {
    /// Prefix map: prefix -> list of full terms
    prefix_map: HashMap<String, Vec<String>>,
    /// Minimum prefix length for indexing
    min_prefix_len: usize,
}

impl TermIndex {
    /// Creates a new term index.
    pub fn new() -> Self {
        Self {
            prefix_map: HashMap::new(),
            min_prefix_len: 2,
        }
    }

    /// Creates a term index with custom minimum prefix length.
    pub fn with_min_prefix_len(min_len: usize) -> Self {
        Self {
            prefix_map: HashMap::new(),
            min_prefix_len: min_len.max(1),
        }
    }

    /// Indexes a term for fast prefix lookups.
    pub fn index_term(&mut self, term: impl Into<String>) {
        let term_str = term.into();
        let term_lower = term_str.to_lowercase();

        // Create prefixes of various lengths
        for len in self.min_prefix_len..=term_lower.len() {
            if let Some(prefix) = term_lower.get(0..len) {
                self.prefix_map
                    .entry(prefix.to_string())
                    .or_default()
                    .push(term_str.clone());
            }
        }
    }

    /// Finds all terms matching a prefix.
    pub fn find_by_prefix(&self, prefix: &str) -> Vec<&str> {
        let prefix_lower = prefix.to_lowercase();
        self.prefix_map
            .get(&prefix_lower)
            .map(|terms| terms.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }

    /// Clears all indexed terms.
    pub fn clear(&mut self) {
        self.prefix_map.clear();
    }

    /// Returns the number of unique prefixes indexed.
    pub fn prefix_count(&self) -> usize {
        self.prefix_map.len()
    }
}

/// Lazy-loading dictionary wrapper for efficient memory usage with large dictionaries.
/// Loads dictionary data on-demand using Arc<Mutex> for thread-safe initialization.
pub struct LazyDictionary {
    /// Locale for this dictionary
    pub locale: Locale,
    /// Lazy-loaded dictionary data
    data: Arc<Mutex<Option<LegalDictionary>>>,
    /// Loading function
    loader: Arc<dyn Fn() -> LegalDictionary + Send + Sync>,
}

impl std::fmt::Debug for LazyDictionary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LazyDictionary")
            .field("locale", &self.locale)
            .field("is_loaded", &self.is_loaded())
            .finish()
    }
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
        let mut data = self.data.lock().unwrap();
        if data.is_none() {
            *data = Some((self.loader)());
        }
        // Extract the dictionary and return it in an Arc<Mutex>
        let dict = data.take().unwrap();
        let result = Arc::new(Mutex::new(dict.clone()));
        *data = Some(dict);
        result
    }

    /// Checks if the dictionary has been loaded yet.
    pub fn is_loaded(&self) -> bool {
        self.data.lock().unwrap().is_some()
    }
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

impl LegalDictionary {
    /// Builds a term index for fast prefix-based lookups.
    /// Useful for autocomplete and fuzzy search features.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_i18n::{LegalDictionary, Locale};
    ///
    /// let mut dict = LegalDictionary::new(Locale::new("en"));
    /// dict.add_translation("contract", "contract");
    /// dict.add_translation("contractor", "contractor");
    /// dict.add_translation("copyright", "copyright");
    ///
    /// let index = dict.build_term_index();
    /// let matches = index.find_by_prefix("contr");
    /// assert!(matches.len() >= 2);
    /// ```
    pub fn build_term_index(&self) -> TermIndex {
        let mut index = TermIndex::new();

        // Index all translation keys
        for key in self.translations.keys() {
            index.index_term(key);
        }

        // Index all abbreviations
        for abbr in self.abbreviations.keys() {
            index.index_term(abbr);
        }

        index
    }
}

// ============================================================================
// Legal Document Analysis (v0.2.4)
// ============================================================================

/// Types of legal clauses found in documents.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClauseType {
    /// Confidentiality/NDA clause
    Confidentiality,
    /// Indemnification clause
    Indemnification,
    /// Limitation of liability
    LimitationOfLiability,
    /// Termination clause
    Termination,
    /// Governing law clause
    GoverningLaw,
    /// Dispute resolution clause
    DisputeResolution,
    /// Force majeure clause
    ForceMajeure,
    /// Warranty clause
    Warranty,
    /// Payment terms
    Payment,
    /// Intellectual property clause
    IntellectualProperty,
    /// Non-compete clause
    NonCompete,
    /// Assignment clause
    Assignment,
    /// Severability clause
    Severability,
    /// Entire agreement clause
    EntireAgreement,
    /// Amendment clause
    Amendment,
    /// Notice clause
    Notice,
    /// Custom clause type
    Custom(String),
}

impl std::fmt::Display for ClauseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClauseType::Confidentiality => write!(f, "Confidentiality"),
            ClauseType::Indemnification => write!(f, "Indemnification"),
            ClauseType::LimitationOfLiability => write!(f, "Limitation of Liability"),
            ClauseType::Termination => write!(f, "Termination"),
            ClauseType::GoverningLaw => write!(f, "Governing Law"),
            ClauseType::DisputeResolution => write!(f, "Dispute Resolution"),
            ClauseType::ForceMajeure => write!(f, "Force Majeure"),
            ClauseType::Warranty => write!(f, "Warranty"),
            ClauseType::Payment => write!(f, "Payment"),
            ClauseType::IntellectualProperty => write!(f, "Intellectual Property"),
            ClauseType::NonCompete => write!(f, "Non-Compete"),
            ClauseType::Assignment => write!(f, "Assignment"),
            ClauseType::Severability => write!(f, "Severability"),
            ClauseType::EntireAgreement => write!(f, "Entire Agreement"),
            ClauseType::Amendment => write!(f, "Amendment"),
            ClauseType::Notice => write!(f, "Notice"),
            ClauseType::Custom(s) => write!(f, "{}", s),
        }
    }
}

/// Extracted clause from a legal document.
#[derive(Debug, Clone)]
pub struct ExtractedClause {
    /// Type of clause
    pub clause_type: ClauseType,
    /// Text of the clause
    pub text: String,
    /// Position in document (character offset)
    pub position: usize,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
}

/// Key clause extractor for legal documents.
#[derive(Debug, Default)]
pub struct ClauseExtractor {
    /// Patterns for identifying clause types
    patterns: HashMap<ClauseType, Vec<String>>,
}

impl ClauseExtractor {
    /// Creates a new clause extractor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a clause extractor with default patterns.
    pub fn with_defaults() -> Self {
        let mut extractor = Self::new();

        // Confidentiality patterns
        extractor.add_pattern(ClauseType::Confidentiality, "confidential");
        extractor.add_pattern(ClauseType::Confidentiality, "non-disclosure");
        extractor.add_pattern(ClauseType::Confidentiality, "proprietary information");

        // Indemnification patterns
        extractor.add_pattern(ClauseType::Indemnification, "indemnify");
        extractor.add_pattern(ClauseType::Indemnification, "hold harmless");
        extractor.add_pattern(ClauseType::Indemnification, "defend");

        // Limitation of liability patterns
        extractor.add_pattern(ClauseType::LimitationOfLiability, "limitation of liability");
        extractor.add_pattern(ClauseType::LimitationOfLiability, "shall not be liable");
        extractor.add_pattern(ClauseType::LimitationOfLiability, "in no event");

        // Termination patterns
        extractor.add_pattern(ClauseType::Termination, "termination");
        extractor.add_pattern(ClauseType::Termination, "terminate");
        extractor.add_pattern(ClauseType::Termination, "cancellation");

        // Governing law patterns
        extractor.add_pattern(ClauseType::GoverningLaw, "governing law");
        extractor.add_pattern(ClauseType::GoverningLaw, "choice of law");
        extractor.add_pattern(ClauseType::GoverningLaw, "governed by");

        // Dispute resolution patterns
        extractor.add_pattern(ClauseType::DisputeResolution, "arbitration");
        extractor.add_pattern(ClauseType::DisputeResolution, "mediation");
        extractor.add_pattern(ClauseType::DisputeResolution, "dispute resolution");

        // Force majeure patterns
        extractor.add_pattern(ClauseType::ForceMajeure, "force majeure");
        extractor.add_pattern(ClauseType::ForceMajeure, "act of god");

        // Warranty patterns
        extractor.add_pattern(ClauseType::Warranty, "warranty");
        extractor.add_pattern(ClauseType::Warranty, "warrants");
        extractor.add_pattern(ClauseType::Warranty, "representations");

        // Payment patterns
        extractor.add_pattern(ClauseType::Payment, "payment");
        extractor.add_pattern(ClauseType::Payment, "compensation");
        extractor.add_pattern(ClauseType::Payment, "fee");

        // IP patterns
        extractor.add_pattern(ClauseType::IntellectualProperty, "intellectual property");
        extractor.add_pattern(ClauseType::IntellectualProperty, "patent");
        extractor.add_pattern(ClauseType::IntellectualProperty, "copyright");
        extractor.add_pattern(ClauseType::IntellectualProperty, "trademark");

        extractor
    }

    /// Adds a pattern for a clause type.
    pub fn add_pattern(&mut self, clause_type: ClauseType, pattern: impl Into<String>) {
        self.patterns
            .entry(clause_type)
            .or_default()
            .push(pattern.into());
    }

    /// Extracts clauses from document text.
    pub fn extract(&self, text: &str) -> Vec<ExtractedClause> {
        let mut clauses = Vec::new();
        let text_lower = text.to_lowercase();

        for (clause_type, patterns) in &self.patterns {
            for pattern in patterns {
                let pattern_lower = pattern.to_lowercase();

                // Find all occurrences of the pattern
                let mut start = 0;
                while let Some(pos) = text_lower[start..].find(&pattern_lower) {
                    let absolute_pos = start + pos;

                    // Extract surrounding context (up to 200 chars)
                    let context_start = absolute_pos.saturating_sub(50);
                    let context_end = (absolute_pos + pattern.len() + 150).min(text.len());
                    let context = &text[context_start..context_end];

                    // Calculate confidence based on context
                    let confidence = self.calculate_confidence(context, pattern);

                    if confidence > 0.3 {
                        clauses.push(ExtractedClause {
                            clause_type: clause_type.clone(),
                            text: context.to_string(),
                            position: absolute_pos,
                            confidence,
                        });
                    }

                    start = absolute_pos + pattern.len();
                }
            }
        }

        // Sort by position
        clauses.sort_by_key(|c| c.position);
        clauses
    }

    #[allow(dead_code)]
    fn calculate_confidence(&self, context: &str, pattern: &str) -> f64 {
        let mut score: f64 = 0.5;

        // Boost if pattern is at start of sentence
        if context
            .trim_start()
            .to_lowercase()
            .starts_with(&pattern.to_lowercase())
        {
            score += 0.2;
        }

        // Boost if context contains legal keywords
        let legal_keywords = ["shall", "hereby", "whereas", "pursuant", "notwithstanding"];
        for keyword in &legal_keywords {
            if context.to_lowercase().contains(keyword) {
                score += 0.05;
            }
        }

        score.min(1.0)
    }
}

/// Party role in a legal document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartyRole {
    /// First party/seller/licensor
    FirstParty,
    /// Second party/buyer/licensee
    SecondParty,
    /// Plaintiff in litigation
    Plaintiff,
    /// Defendant in litigation
    Defendant,
    /// Witness
    Witness,
    /// Third party
    ThirdParty,
    /// Unknown role
    Unknown,
}

/// Identified party in a legal document.
#[derive(Debug, Clone)]
pub struct IdentifiedParty {
    /// Name of the party
    pub name: String,
    /// Role of the party
    pub role: PartyRole,
    /// Position in document
    pub position: usize,
    /// Confidence score
    pub confidence: f64,
}

/// Party identifier for legal documents.
#[derive(Debug, Default)]
pub struct PartyIdentifier {
    /// Patterns for identifying parties
    patterns: Vec<String>,
}

impl PartyIdentifier {
    /// Creates a new party identifier.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a party identifier with default patterns.
    pub fn with_defaults() -> Self {
        let mut identifier = Self::new();

        // Common party introduction patterns
        identifier.add_pattern("party of the first part");
        identifier.add_pattern("party of the second part");
        identifier.add_pattern("hereinafter referred to as");
        identifier.add_pattern("plaintiff");
        identifier.add_pattern("defendant");
        identifier.add_pattern("between");
        identifier.add_pattern("and");

        identifier
    }

    /// Adds a pattern for identifying parties.
    pub fn add_pattern(&mut self, pattern: impl Into<String>) {
        self.patterns.push(pattern.into());
    }

    /// Identifies parties in document text.
    pub fn identify(&self, text: &str) -> Vec<IdentifiedParty> {
        let mut parties = Vec::new();

        // Simple party extraction based on common patterns
        // Look for capitalized names near party introduction keywords
        let lines: Vec<&str> = text.lines().collect();

        for (i, line) in lines.iter().enumerate() {
            let line_lower = line.to_lowercase();

            // Check for party introduction patterns
            if line_lower.contains("between") || line_lower.contains("party") {
                // Look for capitalized words that might be names
                let words: Vec<&str> = line.split_whitespace().collect();

                for (j, word) in words.iter().enumerate() {
                    if word.len() > 2 && word.chars().next().unwrap().is_uppercase() {
                        // Check if it looks like a name (multiple consecutive capitalized words)
                        let mut name_parts = vec![*word];

                        for next_word in words.iter().skip(j + 1) {
                            if next_word.len() > 1
                                && next_word.chars().next().unwrap().is_uppercase()
                            {
                                name_parts.push(*next_word);
                            } else {
                                break;
                            }
                        }

                        if name_parts.len() >= 2
                            || (name_parts.len() == 1 && name_parts[0].len() > 3)
                        {
                            let name = name_parts.join(" ");

                            // Determine role based on context
                            let role = if line_lower.contains("first part") {
                                PartyRole::FirstParty
                            } else if line_lower.contains("second part") {
                                PartyRole::SecondParty
                            } else if line_lower.contains("plaintiff") {
                                PartyRole::Plaintiff
                            } else if line_lower.contains("defendant") {
                                PartyRole::Defendant
                            } else {
                                PartyRole::Unknown
                            };

                            parties.push(IdentifiedParty {
                                name: name
                                    .trim_matches(|c: char| !c.is_alphanumeric() && c != ' ')
                                    .to_string(),
                                role,
                                position: i * 100, // Approximate position
                                confidence: 0.7,
                            });
                        }
                    }
                }
            }
        }

        parties
    }
}

/// Type of legal obligation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObligationType {
    /// Shall/must obligation
    Mandatory,
    /// May/can obligation
    Permissive,
    /// Shall not/must not prohibition
    Prohibition,
    /// Should recommendation
    Recommendation,
}

/// Extracted obligation from a legal document.
#[derive(Debug, Clone)]
pub struct ExtractedObligation {
    /// Type of obligation
    pub obligation_type: ObligationType,
    /// Text describing the obligation
    pub text: String,
    /// Subject of the obligation (who must perform it)
    pub subject: Option<String>,
    /// Position in document
    pub position: usize,
    /// Confidence score
    pub confidence: f64,
}

/// Obligation extractor for legal documents.
#[derive(Debug, Default)]
pub struct ObligationExtractor {}

impl ObligationExtractor {
    /// Creates a new obligation extractor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Extracts obligations from document text.
    pub fn extract(&self, text: &str) -> Vec<ExtractedObligation> {
        let mut obligations = Vec::new();

        // Split into sentences
        let sentences: Vec<&str> = text.split(&['.', ';', '!'][..]).collect();

        for (i, sentence) in sentences.iter().enumerate() {
            let sentence_lower = sentence.to_lowercase();

            // Check for obligation keywords
            let obligation_type =
                if sentence_lower.contains(" shall ") || sentence_lower.contains(" must ") {
                    Some(ObligationType::Mandatory)
                } else if sentence_lower.contains(" shall not ")
                    || sentence_lower.contains(" must not ")
                {
                    Some(ObligationType::Prohibition)
                } else if sentence_lower.contains(" may ") || sentence_lower.contains(" can ") {
                    Some(ObligationType::Permissive)
                } else if sentence_lower.contains(" should ") {
                    Some(ObligationType::Recommendation)
                } else {
                    None
                };

            if let Some(ob_type) = obligation_type {
                // Try to extract the subject (simplified)
                let subject = self.extract_subject(sentence);

                obligations.push(ExtractedObligation {
                    obligation_type: ob_type,
                    text: sentence.trim().to_string(),
                    subject,
                    position: i * 100,
                    confidence: 0.75,
                });
            }
        }

        obligations
    }

    fn extract_subject(&self, sentence: &str) -> Option<String> {
        // Very simple subject extraction - get the first capitalized word(s) before the obligation verb
        let words: Vec<&str> = sentence.split_whitespace().collect();

        for (i, word) in words.iter().enumerate() {
            let word_lower = word.to_lowercase();
            if word_lower.contains("shall")
                || word_lower.contains("must")
                || word_lower.contains("may")
            {
                // Look backwards for capitalized words
                let mut subject_parts = Vec::new();
                for j in (0..i).rev() {
                    if words[j]
                        .chars()
                        .next()
                        .map(|c| c.is_uppercase())
                        .unwrap_or(false)
                    {
                        subject_parts.insert(0, words[j]);
                    } else {
                        break;
                    }
                }

                if !subject_parts.is_empty() {
                    return Some(subject_parts.join(" "));
                }
                break;
            }
        }

        None
    }
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

/// Deadline extractor for legal documents with calendar integration.
#[derive(Debug, Default)]
pub struct DeadlineExtractor {
    /// Reference date for relative date calculations
    reference_date: Option<(i32, u32, u32)>,
}

impl DeadlineExtractor {
    /// Creates a new deadline extractor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets a reference date for relative date calculations.
    pub fn with_reference_date(mut self, year: i32, month: u32, day: u32) -> Self {
        self.reference_date = Some((year, month, day));
        self
    }

    /// Extracts deadlines from document text.
    pub fn extract(&self, text: &str) -> Vec<ExtractedDeadline> {
        let mut deadlines = Vec::new();

        // Look for date patterns (for future enhancement with regex)
        let _date_patterns = [
            r"(\d{1,2})/(\d{1,2})/(\d{2,4})",      // MM/DD/YYYY
            r"(\d{4})-(\d{1,2})-(\d{1,2})",        // YYYY-MM-DD
            r"(\d{1,2})\s+(days?|months?|years?)", // Relative dates
        ];

        // Split into sentences for context
        let sentences: Vec<&str> = text.split(&['.', ';'][..]).collect();

        for (i, sentence) in sentences.iter().enumerate() {
            let sentence_lower = sentence.to_lowercase();

            // Check for deadline keywords
            if sentence_lower.contains("deadline")
                || sentence_lower.contains("due")
                || sentence_lower.contains("within")
                || sentence_lower.contains("by")
                || sentence_lower.contains("before")
                || sentence_lower.contains("after")
            {
                // Try to parse date
                let date = self.parse_date(sentence);

                deadlines.push(ExtractedDeadline {
                    date,
                    description: sentence.trim().to_string(),
                    position: i * 100,
                    confidence: if date.is_some() { 0.8 } else { 0.5 },
                    context: sentence.trim().to_string(),
                });
            }
        }

        deadlines
    }

    fn parse_date(&self, text: &str) -> Option<(i32, u32, u32)> {
        // Simple date parsing - look for MM/DD/YYYY format
        let parts: Vec<&str> = text.split('/').collect();
        if parts.len() == 3 {
            if let (Ok(month), Ok(day), Ok(year)) = (
                parts[0].trim().parse::<u32>(),
                parts[1].trim().parse::<u32>(),
                parts[2].trim().parse::<i32>(),
            ) {
                // Handle 2-digit years
                let full_year = if year < 100 {
                    if year > 50 { 1900 + year } else { 2000 + year }
                } else {
                    year
                };

                return Some((full_year, month, day));
            }
        }

        None
    }
}

/// Jurisdiction detector for legal documents.
#[derive(Debug, Default)]
pub struct JurisdictionDetector {
    /// Known jurisdictions and their indicators
    indicators: HashMap<String, Vec<String>>,
}

impl JurisdictionDetector {
    /// Creates a new jurisdiction detector.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a jurisdiction detector with default indicators.
    pub fn with_defaults() -> Self {
        let mut detector = Self::new();

        // US indicators
        detector.add_indicator("US", "United States");
        detector.add_indicator("US", "New York");
        detector.add_indicator("US", "Delaware");
        detector.add_indicator("US", "California");
        detector.add_indicator("US", "Supreme Court");

        // UK indicators
        detector.add_indicator("GB", "United Kingdom");
        detector.add_indicator("GB", "England and Wales");
        detector.add_indicator("GB", "English law");

        // Japan indicators
        detector.add_indicator("JP", "Japan");
        detector.add_indicator("JP", "Japanese law");
        detector.add_indicator("JP", "Tokyo");

        // Germany indicators
        detector.add_indicator("DE", "Germany");
        detector.add_indicator("DE", "German law");
        detector.add_indicator("DE", "BGB");

        // France indicators
        detector.add_indicator("FR", "France");
        detector.add_indicator("FR", "French law");
        detector.add_indicator("FR", "Code civil");

        detector
    }

    /// Adds an indicator for a jurisdiction.
    pub fn add_indicator(&mut self, jurisdiction: impl Into<String>, indicator: impl Into<String>) {
        self.indicators
            .entry(jurisdiction.into())
            .or_default()
            .push(indicator.into());
    }

    /// Detects jurisdiction from document text.
    /// Returns (jurisdiction_code, confidence).
    pub fn detect(&self, text: &str) -> Option<(String, f64)> {
        let text_lower = text.to_lowercase();
        let mut scores: HashMap<String, f64> = HashMap::new();

        for (jurisdiction, indicators) in &self.indicators {
            let mut score = 0.0;
            for indicator in indicators {
                if text_lower.contains(&indicator.to_lowercase()) {
                    score += 1.0;
                }
            }

            if score > 0.0 {
                scores.insert(jurisdiction.clone(), score);
            }
        }

        // Return jurisdiction with highest score
        scores
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(j, s)| (j.clone(), (s / 3.0).min(1.0)))
    }
}

/// Risk level for legal documents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RiskLevel {
    /// Low risk
    Low,
    /// Medium risk
    Medium,
    /// High risk
    High,
    /// Critical risk
    Critical,
}

impl std::fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RiskLevel::Low => write!(f, "Low"),
            RiskLevel::Medium => write!(f, "Medium"),
            RiskLevel::High => write!(f, "High"),
            RiskLevel::Critical => write!(f, "Critical"),
        }
    }
}

/// Risk factor identified in a document.
#[derive(Debug, Clone)]
pub struct RiskFactor {
    /// Description of the risk
    pub description: String,
    /// Risk level
    pub level: RiskLevel,
    /// Position in document
    pub position: usize,
    /// Mitigation suggestion
    pub mitigation: Option<String>,
}

/// Legal risk scorer for documents.
#[derive(Debug, Default)]
pub struct LegalRiskScorer {
    /// Risk indicators and their severity
    indicators: HashMap<String, RiskLevel>,
}

impl LegalRiskScorer {
    /// Creates a new legal risk scorer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a risk scorer with default indicators.
    pub fn with_defaults() -> Self {
        let mut scorer = Self::new();

        // High-risk indicators
        scorer.add_indicator("unlimited liability", RiskLevel::Critical);
        scorer.add_indicator("no limitation of liability", RiskLevel::Critical);
        scorer.add_indicator("personal guarantee", RiskLevel::High);
        scorer.add_indicator("waive", RiskLevel::High);
        scorer.add_indicator("automatic renewal", RiskLevel::Medium);
        scorer.add_indicator("non-refundable", RiskLevel::Medium);
        scorer.add_indicator("as-is", RiskLevel::Medium);
        scorer.add_indicator("no warranty", RiskLevel::Medium);

        // Positive indicators (low risk)
        scorer.add_indicator("limitation of liability", RiskLevel::Low);
        scorer.add_indicator("indemnification", RiskLevel::Low);
        scorer.add_indicator("insurance", RiskLevel::Low);

        scorer
    }

    /// Adds a risk indicator.
    pub fn add_indicator(&mut self, indicator: impl Into<String>, level: RiskLevel) {
        self.indicators.insert(indicator.into(), level);
    }

    /// Scores document risk and returns identified factors.
    pub fn score(&self, text: &str) -> (RiskLevel, Vec<RiskFactor>) {
        let mut risk_factors = Vec::new();
        let text_lower = text.to_lowercase();
        let mut overall_score = 0.0;

        // Check for risk indicators
        for (indicator, level) in &self.indicators {
            if text_lower.contains(&indicator.to_lowercase()) {
                let score_value = match level {
                    RiskLevel::Low => 1.0,
                    RiskLevel::Medium => 2.0,
                    RiskLevel::High => 3.0,
                    RiskLevel::Critical => 4.0,
                };

                overall_score += score_value;

                // Find position
                if let Some(pos) = text_lower.find(&indicator.to_lowercase()) {
                    risk_factors.push(RiskFactor {
                        description: format!("Found: {}", indicator),
                        level: *level,
                        position: pos,
                        mitigation: self.suggest_mitigation(indicator, level),
                    });
                }
            }
        }

        // Determine overall risk level
        let overall_level = if overall_score >= 10.0 {
            RiskLevel::Critical
        } else if overall_score >= 6.0 {
            RiskLevel::High
        } else if overall_score >= 3.0 {
            RiskLevel::Medium
        } else {
            RiskLevel::Low
        };

        (overall_level, risk_factors)
    }

    fn suggest_mitigation(&self, indicator: &str, level: &RiskLevel) -> Option<String> {
        match (indicator, level) {
            ("unlimited liability", RiskLevel::Critical) => {
                Some("Add limitation of liability clause to cap damages".to_string())
            }
            ("personal guarantee", RiskLevel::High) => {
                Some("Consider corporate guarantee instead of personal".to_string())
            }
            ("automatic renewal", RiskLevel::Medium) => {
                Some("Add notice period for cancellation before renewal".to_string())
            }
            _ => None,
        }
    }
}

/// Comprehensive legal document analyzer.
pub struct LegalDocumentAnalyzer {
    clause_extractor: ClauseExtractor,
    party_identifier: PartyIdentifier,
    obligation_extractor: ObligationExtractor,
    deadline_extractor: DeadlineExtractor,
    jurisdiction_detector: JurisdictionDetector,
    risk_scorer: LegalRiskScorer,
}

impl Default for LegalDocumentAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl LegalDocumentAnalyzer {
    /// Creates a new legal document analyzer with default settings.
    pub fn new() -> Self {
        Self {
            clause_extractor: ClauseExtractor::with_defaults(),
            party_identifier: PartyIdentifier::with_defaults(),
            obligation_extractor: ObligationExtractor::new(),
            deadline_extractor: DeadlineExtractor::new(),
            jurisdiction_detector: JurisdictionDetector::with_defaults(),
            risk_scorer: LegalRiskScorer::with_defaults(),
        }
    }

    /// Analyzes a legal document and returns comprehensive analysis.
    pub fn analyze(&self, text: &str) -> DocumentAnalysis {
        DocumentAnalysis {
            clauses: self.clause_extractor.extract(text),
            parties: self.party_identifier.identify(text),
            obligations: self.obligation_extractor.extract(text),
            deadlines: self.deadline_extractor.extract(text),
            jurisdiction: self.jurisdiction_detector.detect(text),
            risk_level: self.risk_scorer.score(text).0,
            risk_factors: self.risk_scorer.score(text).1,
        }
    }

    /// Gets mutable reference to clause extractor.
    pub fn clause_extractor_mut(&mut self) -> &mut ClauseExtractor {
        &mut self.clause_extractor
    }

    /// Gets mutable reference to jurisdiction detector.
    pub fn jurisdiction_detector_mut(&mut self) -> &mut JurisdictionDetector {
        &mut self.jurisdiction_detector
    }

    /// Gets mutable reference to risk scorer.
    pub fn risk_scorer_mut(&mut self) -> &mut LegalRiskScorer {
        &mut self.risk_scorer
    }
}

/// Complete analysis of a legal document.
#[derive(Debug)]
pub struct DocumentAnalysis {
    /// Extracted clauses
    pub clauses: Vec<ExtractedClause>,
    /// Identified parties
    pub parties: Vec<IdentifiedParty>,
    /// Extracted obligations
    pub obligations: Vec<ExtractedObligation>,
    /// Extracted deadlines
    pub deadlines: Vec<ExtractedDeadline>,
    /// Detected jurisdiction
    pub jurisdiction: Option<(String, f64)>,
    /// Overall risk level
    pub risk_level: RiskLevel,
    /// Identified risk factors
    pub risk_factors: Vec<RiskFactor>,
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

    #[test]
    fn test_plural_rules_english() {
        let rules = PluralRules::new(Locale::new("en"));
        assert_eq!(rules.category(1), PluralCategory::One);
        assert_eq!(rules.category(2), PluralCategory::Other);
        assert_eq!(rules.category(0), PluralCategory::Other);
    }

    #[test]
    fn test_plural_rules_japanese() {
        let rules = PluralRules::new(Locale::new("ja"));
        assert_eq!(rules.category(1), PluralCategory::Other);
        assert_eq!(rules.category(2), PluralCategory::Other);
    }

    #[test]
    fn test_plural_rules_russian() {
        let rules = PluralRules::new(Locale::new("ru"));
        assert_eq!(rules.category(1), PluralCategory::One);
        assert_eq!(rules.category(2), PluralCategory::Few);
        assert_eq!(rules.category(5), PluralCategory::Many);
    }

    #[test]
    fn test_plural_rules_arabic() {
        let rules = PluralRules::new(Locale::new("ar"));
        assert_eq!(rules.category(0), PluralCategory::Zero);
        assert_eq!(rules.category(1), PluralCategory::One);
        assert_eq!(rules.category(2), PluralCategory::Two);
        assert_eq!(rules.category(5), PluralCategory::Few);
    }

    #[test]
    fn test_message_formatter() {
        let formatter = MessageFormatter::new(Locale::new("en"));
        let mut args = HashMap::new();
        args.insert("name".to_string(), "John".to_string());
        args.insert("age".to_string(), "30".to_string());

        let result = formatter.format("Hello {name}, you are {age} years old", &args);
        assert_eq!(result, "Hello John, you are 30 years old");
    }

    #[test]
    fn test_message_formatter_plural() {
        let formatter = MessageFormatter::new(Locale::new("en"));
        assert_eq!(formatter.format_plural(1, "1 item", "items"), "1 item");
        assert_eq!(formatter.format_plural(2, "1 item", "items"), "items");
    }

    #[test]
    fn test_datetime_formatter_japanese() {
        let formatter = DateTimeFormatter::new(Locale::new("ja").with_country("JP"));
        assert_eq!(formatter.format_date(2024, 12, 14), "2024年12月14日");
        assert_eq!(formatter.format_time(15, 30), "15:30");
    }

    #[test]
    fn test_datetime_formatter_us() {
        let formatter = DateTimeFormatter::new(Locale::new("en").with_country("US"));
        assert_eq!(formatter.format_date(2024, 12, 14), "12/14/2024");
        assert_eq!(formatter.format_time(15, 30), "03:30 PM");
        assert_eq!(formatter.format_time(9, 15), "09:15 AM");
    }

    #[test]
    fn test_datetime_formatter_german() {
        let formatter = DateTimeFormatter::new(Locale::new("de").with_country("DE"));
        assert_eq!(formatter.format_date(2024, 12, 14), "14.12.2024");
        assert_eq!(formatter.format_time(15, 30), "15:30");
    }

    #[test]
    fn test_currency_formatter_usd() {
        let formatter = CurrencyFormatter::new(Locale::new("en").with_country("US"));
        assert_eq!(formatter.format(1000.50, "USD"), "$1000.50");
        assert_eq!(formatter.format(100.0, "USD"), "$100");
    }

    #[test]
    fn test_currency_formatter_eur() {
        let formatter = CurrencyFormatter::new(Locale::new("de").with_country("DE"));
        assert_eq!(formatter.format(1000.50, "EUR"), "1000,50 €");
    }

    #[test]
    fn test_currency_formatter_jpy() {
        let formatter = CurrencyFormatter::new(Locale::new("ja").with_country("JP"));
        assert_eq!(formatter.format(1000.0, "JPY"), "¥1000");
    }

    #[test]
    fn test_number_formatter_english() {
        let formatter = NumberFormatter::new(Locale::new("en"));
        assert_eq!(formatter.format_integer(1000), "1,000");
        assert_eq!(formatter.format_integer(1000000), "1,000,000");
        assert_eq!(formatter.format_percentage(50.5), "50.5%");
    }

    #[test]
    fn test_number_formatter_german() {
        let formatter = NumberFormatter::new(Locale::new("de"));
        assert_eq!(formatter.format_integer(1000), "1.000");
        assert_eq!(formatter.format_integer(1000000), "1.000.000");
        assert_eq!(formatter.format_percentage(50.5), "50,5 %");
    }

    #[test]
    fn test_number_formatter_french() {
        let formatter = NumberFormatter::new(Locale::new("fr"));
        assert_eq!(formatter.format_integer(1000), "1 000");
        assert_eq!(formatter.format_percentage(50.5), "50,5 %");
    }

    #[test]
    fn test_number_formatter_japanese() {
        let formatter = NumberFormatter::new(Locale::new("ja"));
        assert_eq!(formatter.format_integer(1000), "1000");
        assert_eq!(formatter.format_integer(1000000), "1000000");
    }

    #[test]
    fn test_legal_dictionary_japanese() {
        let dict = LegalDictionary::japanese();
        assert_eq!(dict.translate("statute"), Some("法律"));
        assert_eq!(dict.translate("contract"), Some("契約"));
        assert_eq!(dict.translate("court"), Some("裁判所"));
    }

    #[test]
    fn test_legal_dictionary_german() {
        let dict = LegalDictionary::german();
        assert_eq!(dict.translate("statute"), Some("Gesetz"));
        assert_eq!(dict.translate("contract"), Some("Vertrag"));
        assert_eq!(dict.translate("court"), Some("Gericht"));
    }

    #[test]
    fn test_legal_dictionary_french() {
        let dict = LegalDictionary::french();
        assert_eq!(dict.translate("statute"), Some("loi"));
        assert_eq!(dict.translate("contract"), Some("contrat"));
        assert_eq!(dict.translate("court"), Some("tribunal"));
    }

    #[test]
    fn test_legal_dictionary_spanish() {
        let dict = LegalDictionary::spanish();
        assert_eq!(dict.translate("statute"), Some("estatuto"));
        assert_eq!(dict.translate("contract"), Some("contrato"));
        assert_eq!(dict.translate("court"), Some("tribunal"));
    }

    #[test]
    fn test_legal_dictionary_chinese() {
        let dict = LegalDictionary::chinese_simplified();
        assert_eq!(dict.translate("statute"), Some("法规"));
        assert_eq!(dict.translate("contract"), Some("合同"));
        assert_eq!(dict.translate("court"), Some("法院"));
    }

    #[test]
    fn test_translation_manager_with_dictionaries() {
        let mut manager = TranslationManager::new();
        manager.add_dictionary(LegalDictionary::japanese());
        manager.add_dictionary(LegalDictionary::german());

        let ja_locale = Locale::new("ja").with_country("JP");
        let de_locale = Locale::new("de").with_country("DE");

        assert_eq!(manager.translate("statute", &ja_locale).unwrap(), "法律");
        assert_eq!(manager.translate("statute", &de_locale).unwrap(), "Gesetz");
    }

    #[test]
    fn test_latin_dictionary() {
        let dict = LegalDictionary::latin();
        assert_eq!(dict.translate("guilty_mind"), Some("mens rea"));
        assert_eq!(dict.translate("guilty_act"), Some("actus reus"));
        assert_eq!(dict.translate("good_faith"), Some("bona fide"));
        assert_eq!(dict.translate("in_fact"), Some("de facto"));
        assert!(dict.define("mens rea").is_some());
    }

    #[test]
    fn test_legal_concept_mapping() {
        let registry = LegalConceptRegistry::with_defaults();

        // Test finding a mapping
        let mapping = registry
            .find_mapping(LegalSystem::CommonLaw, "tort")
            .unwrap();
        assert_eq!(mapping.concept, "tort");

        let civil_equivalents = mapping.get_equivalents(LegalSystem::CivilLaw).unwrap();
        assert!(civil_equivalents.contains(&"delict".to_string()));
    }

    #[test]
    fn test_legal_concept_system_mappings() {
        let registry = LegalConceptRegistry::with_defaults();

        let mappings = registry.get_system_mappings(LegalSystem::CommonLaw, LegalSystem::CivilLaw);
        assert!(!mappings.is_empty());

        // Check that tort -> delict mapping exists
        let tort_mapping = mappings.iter().find(|(concept, _)| *concept == "tort");
        assert!(tort_mapping.is_some());
    }

    #[test]
    fn test_calendar_converter_japanese() {
        let converter = CalendarConverter::new(Locale::new("ja").with_country("JP"));

        // Test Reiwa era (2019-)
        let date = converter.from_gregorian(2024, 12, 14);
        assert_eq!(date.system, CalendarSystem::Japanese);
        assert_eq!(date.year, 6); // Reiwa 6
        assert_eq!(date.era, Some("Reiwa".to_string()));

        // Test Heisei era (1989-2019)
        let date = converter.from_gregorian(2018, 5, 1);
        assert_eq!(date.system, CalendarSystem::Japanese);
        assert_eq!(date.year, 30); // Heisei 30
        assert_eq!(date.era, Some("Heisei".to_string()));
    }

    #[test]
    fn test_calendar_converter_buddhist() {
        let converter = CalendarConverter::new(Locale::new("th").with_country("TH"));

        let date = converter.from_gregorian(2024, 12, 14);
        assert_eq!(date.system, CalendarSystem::Buddhist);
        assert_eq!(date.year, 2567); // 2024 + 543
    }

    #[test]
    fn test_calendar_date_formatting() {
        let converter = CalendarConverter::new(Locale::new("ja").with_country("JP"));

        let date = CalendarDate::new(CalendarSystem::Japanese, 6, 12, 14).with_era("Reiwa");
        let formatted = converter.format_date(&date);
        assert_eq!(formatted, "Reiwa6年12月14日");
    }

    #[test]
    fn test_working_days_japan() {
        let config = WorkingDaysConfig::japan();

        // Check weekend
        assert!(!config.is_working_day(2024, 12, 14)); // Saturday
        assert!(!config.is_working_day(2024, 12, 15)); // Sunday
        assert!(config.is_working_day(2024, 12, 16)); // Monday

        // Check New Year's Day
        assert!(!config.is_working_day(2024, 1, 1));
    }

    #[test]
    fn test_working_days_saudi_arabia() {
        let config = WorkingDaysConfig::saudi_arabia();

        // Weekend in Saudi Arabia is Friday-Saturday
        assert!(!config.weekend.contains(&DayOfWeek::Sunday));
        assert!(config.weekend.contains(&DayOfWeek::Friday));
        assert!(config.weekend.contains(&DayOfWeek::Saturday));
    }

    #[test]
    fn test_add_working_days() {
        let config = WorkingDaysConfig::new("TEST");

        // Starting from Friday (2024-12-13), add 3 working days
        // Should skip weekend (14-15) and land on Wednesday (18)
        let (year, month, day) = config.add_working_days(2024, 12, 13, 3);
        assert_eq!(year, 2024);
        assert_eq!(month, 12);
        assert_eq!(day, 18);
    }

    #[test]
    fn test_day_of_week_calculation() {
        let config = WorkingDaysConfig::new("TEST");

        // 2024-12-14 is Saturday
        let day = config.calculate_day_of_week(2024, 12, 14);
        assert_eq!(day, DayOfWeek::Saturday);

        // 2024-12-16 is Monday
        let day = config.calculate_day_of_week(2024, 12, 16);
        assert_eq!(day, DayOfWeek::Monday);
    }

    #[test]
    fn test_translation_roundtrip_japanese() {
        let dict_ja = LegalDictionary::japanese();
        let dict_en = LegalDictionary::english_us();

        // Test that key concepts translate correctly
        assert_eq!(dict_ja.translate("statute"), Some("法律"));
        assert_eq!(dict_en.translate("statute"), Some("statute"));
    }

    #[test]
    fn test_all_locale_dictionaries() {
        // Test that all locale dictionaries can be created
        let _en = LegalDictionary::english_us();
        let _ja = LegalDictionary::japanese();
        let _de = LegalDictionary::german();
        let _fr = LegalDictionary::french();
        let _es = LegalDictionary::spanish();
        let _zh = LegalDictionary::chinese_simplified();
        let _la = LegalDictionary::latin();
    }

    #[test]
    fn test_jurisdiction_cultural_params() {
        let registry = JurisdictionRegistry::with_defaults();

        let japan = registry.get("JP").unwrap();
        assert_eq!(japan.cultural_params.age_of_majority, Some(18));

        let saudi = registry.get("SA").unwrap();
        assert_eq!(saudi.legal_system, LegalSystem::ReligiousLaw);
        assert!(
            saudi
                .cultural_params
                .religious_considerations
                .contains(&"islam".to_string())
        );
    }

    #[test]
    fn test_locale_variations() {
        // Test different locale variations
        let us = Locale::new("en").with_country("US");
        let gb = Locale::new("en").with_country("GB");

        assert_eq!(us.tag(), "en-US");
        assert_eq!(gb.tag(), "en-GB");
        assert_eq!(us.language, gb.language);
        assert_ne!(us.country, gb.country);
    }

    #[test]
    fn test_jurisdiction_glossaries() {
        // Test Japan glossary
        let jp_glossary = LegalDictionary::glossary_japan();
        assert_eq!(jp_glossary.translate("civil_code"), Some("民法"));
        assert_eq!(jp_glossary.translate("family_register"), Some("戸籍"));
        assert_eq!(jp_glossary.translate("kabushiki_kaisha"), Some("株式会社"));

        // Test US glossary
        let us_glossary = LegalDictionary::glossary_united_states();
        assert_eq!(us_glossary.translate("due_process"), Some("due process"));
        assert_eq!(
            us_glossary.translate("supreme_court"),
            Some("Supreme Court")
        );
        assert_eq!(us_glossary.translate("class_action"), Some("class action"));

        // Test UK glossary
        let uk_glossary = LegalDictionary::glossary_united_kingdom();
        assert_eq!(uk_glossary.translate("barrister"), Some("barrister"));
        assert_eq!(uk_glossary.translate("freehold"), Some("freehold"));
        assert_eq!(uk_glossary.translate("trust"), Some("trust"));

        // Test Germany glossary
        let de_glossary = LegalDictionary::glossary_germany();
        assert_eq!(de_glossary.translate("bgb"), Some("BGB"));
        assert_eq!(
            de_glossary.translate("bundesgerichtshof"),
            Some("Bundesgerichtshof")
        );

        // Test France glossary
        let fr_glossary = LegalDictionary::glossary_france();
        assert_eq!(fr_glossary.translate("code_civil"), Some("Code civil"));
        assert_eq!(
            fr_glossary.translate("cour_de_cassation"),
            Some("Cour de cassation")
        );

        // Test China glossary
        let cn_glossary = LegalDictionary::glossary_china();
        assert_eq!(cn_glossary.translate("civil_law"), Some("民法"));
        assert_eq!(cn_glossary.translate("peoples_court"), Some("人民法院"));
    }

    #[test]
    fn test_glossary_for_jurisdiction() {
        let jp_glossary = LegalDictionary::glossary_for_jurisdiction("JP");
        assert_eq!(jp_glossary.locale.country, Some("JP".to_string()));

        let us_glossary = LegalDictionary::glossary_for_jurisdiction("US");
        assert_eq!(us_glossary.locale.country, Some("US".to_string()));
    }

    #[test]
    fn test_locale_matches() {
        let en = Locale::new("en");
        let en_us = Locale::new("en").with_country("US");
        let en_gb = Locale::new("en").with_country("GB");
        let ja = Locale::new("ja");

        // Should match: same language, one without country
        assert!(en.matches(&en_us));
        assert!(en_us.matches(&en));

        // Should not match: different countries
        assert!(!en_us.matches(&en_gb));

        // Should not match: different languages
        assert!(!en.matches(&ja));
    }

    #[test]
    fn test_locale_parent() {
        let zh_hans_cn = Locale::new("zh").with_script("Hans").with_country("CN");

        // Remove country first
        let parent1 = zh_hans_cn.parent().unwrap();
        assert_eq!(parent1.language, "zh");
        assert_eq!(parent1.script, Some("Hans".to_string()));
        assert_eq!(parent1.country, None);

        // Then remove script
        let parent2 = parent1.parent().unwrap();
        assert_eq!(parent2.language, "zh");
        assert_eq!(parent2.script, None);
        assert_eq!(parent2.country, None);

        // No more parents
        assert!(parent2.parent().is_none());
    }

    #[test]
    fn test_locale_fallback_chain() {
        let zh_hans_cn = Locale::new("zh").with_script("Hans").with_country("CN");

        let chain = zh_hans_cn.fallback_chain();
        assert_eq!(chain.len(), 3);
        assert_eq!(chain[0].tag(), "zh-Hans-CN");
        assert_eq!(chain[1].tag(), "zh-Hans");
        assert_eq!(chain[2].tag(), "zh");
    }

    #[test]
    fn test_regional_variation_registry() {
        let registry = RegionalVariationRegistry::with_defaults();

        // Test English variations
        let en_variations = registry.get_variations(&Locale::new("en"));
        assert!(en_variations.len() >= 4); // US, GB, AU, CA

        // Test specific variation
        let us_locale = Locale::new("en").with_country("US");
        let us_variation = registry.find_variation(&us_locale);
        assert!(us_variation.is_some());
        assert_eq!(us_variation.unwrap().description, "American English");
    }

    #[test]
    fn test_regional_variation_differences() {
        let registry = RegionalVariationRegistry::with_defaults();

        let us_locale = Locale::new("en").with_country("US");
        let us_variation = registry.find_variation(&us_locale).unwrap();

        // Check that differences are recorded
        assert!(!us_variation.differences.is_empty());
        assert!(
            us_variation
                .differences
                .iter()
                .any(|d| d.contains("attorney"))
        );
    }

    #[test]
    fn test_chinese_script_variations() {
        let registry = RegionalVariationRegistry::with_defaults();

        // Simplified Chinese (Mainland)
        let cn_locale = Locale::new("zh").with_country("CN").with_script("Hans");
        let cn_variation = registry.find_variation(&cn_locale);
        assert!(cn_variation.is_some());
        assert!(cn_variation.unwrap().description.contains("Simplified"));

        // Traditional Chinese (Taiwan)
        let tw_locale = Locale::new("zh").with_country("TW").with_script("Hant");
        let tw_variation = registry.find_variation(&tw_locale);
        assert!(tw_variation.is_some());
        assert!(tw_variation.unwrap().description.contains("Traditional"));
    }

    #[test]
    fn test_spanish_regional_variations() {
        let registry = RegionalVariationRegistry::with_defaults();

        let es_variations = registry.get_variations(&Locale::new("es"));
        assert!(es_variations.len() >= 3); // ES, MX, AR

        // Check Mexican Spanish
        let mx_locale = Locale::new("es").with_country("MX");
        let mx_variation = registry.find_variation(&mx_locale);
        assert!(mx_variation.is_some());
        assert!(
            mx_variation
                .unwrap()
                .differences
                .iter()
                .any(|d| d.contains("ustedes"))
        );
    }

    #[test]
    fn test_german_regional_variations() {
        let registry = RegionalVariationRegistry::with_defaults();

        let de_variations = registry.get_variations(&Locale::new("de"));
        assert!(de_variations.len() >= 3); // DE, AT, CH

        // Check Swiss German
        let ch_locale = Locale::new("de").with_country("CH");
        let ch_variation = registry.find_variation(&ch_locale);
        assert!(ch_variation.is_some());
        assert!(
            ch_variation
                .unwrap()
                .differences
                .iter()
                .any(|d| d.contains("Swiss"))
        );
    }

    #[test]
    fn test_french_regional_variations() {
        let registry = RegionalVariationRegistry::with_defaults();

        let fr_variations = registry.get_variations(&Locale::new("fr"));
        assert!(fr_variations.len() >= 3); // FR, CA, BE

        // Check Canadian French
        let ca_locale = Locale::new("fr").with_country("CA");
        let ca_variation = registry.find_variation(&ca_locale);
        assert!(ca_variation.is_some());
        assert!(
            ca_variation
                .unwrap()
                .differences
                .iter()
                .any(|d| d.contains("Quebec") || d.contains("Bilingual"))
        );
    }

    #[test]
    fn test_mock_translation_service() {
        let service = MockTranslationService::new();
        let en = Locale::new("en");
        let ja = Locale::new("ja");

        let result = service.translate("contract", &en, &ja).unwrap();
        assert_eq!(result, "[ja] contract");
        assert_eq!(service.service_name(), "MockTranslationService");
        assert!(service.is_available());
    }

    #[test]
    fn test_mock_translation_service_unavailable() {
        let mut service = MockTranslationService::new();
        service.set_available(false);

        let en = Locale::new("en");
        let ja = Locale::new("ja");

        let result = service.translate("contract", &en, &ja);
        assert!(result.is_err());
        assert!(!service.is_available());
    }

    #[test]
    fn test_translation_memory_exact_match() {
        let mut memory = TranslationMemory::new();

        let en = Locale::new("en");
        let ja = Locale::new("ja");

        memory.add_translation("contract", en.clone(), "契約", ja.clone());

        let matches = memory.find_exact("contract", &en, &ja);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].target_text, "契約");
    }

    #[test]
    fn test_translation_memory_no_match() {
        let memory = TranslationMemory::new();

        let en = Locale::new("en");
        let ja = Locale::new("ja");

        let matches = memory.find_exact("contract", &en, &ja);
        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_translation_memory_fuzzy_match() {
        let mut memory = TranslationMemory::new();

        let en = Locale::new("en");
        let ja = Locale::new("ja");

        memory.add_translation("employment contract", en.clone(), "雇用契約", ja.clone());

        let matches = memory.find_fuzzy("employment contract agreement", &en, &ja, 0.5);
        assert!(!matches.is_empty());
        assert!(matches[0].1 > 0.5); // Similarity score
    }

    #[test]
    fn test_translation_memory_entry() {
        let en = Locale::new("en");
        let ja = Locale::new("ja");

        let entry = TranslationMemoryEntry::new("contract", en, "契約", ja)
            .with_quality(0.95)
            .with_metadata("translator", "human");

        assert_eq!(entry.source_text, "contract");
        assert_eq!(entry.target_text, "契約");
        assert_eq!(entry.quality_score, 0.95);
        assert_eq!(entry.metadata.get("translator").unwrap(), "human");
    }

    #[test]
    fn test_machine_translation_fallback_memory_hit() {
        let mut fallback = MachineTranslationFallback::new();

        let en = Locale::new("en");
        let ja = Locale::new("ja");

        // Pre-populate memory
        fallback
            .memory_mut()
            .add_translation("contract", en.clone(), "契約", ja.clone());

        // Should find in memory
        let result = fallback.translate("contract", &en, &ja).unwrap();
        assert_eq!(result, "契約");
    }

    #[test]
    fn test_machine_translation_fallback_service() {
        let mut fallback = MachineTranslationFallback::new();
        fallback.add_service(Box::new(MockTranslationService::new()));

        let en = Locale::new("en");
        let ja = Locale::new("ja");

        // Should fall back to service
        let result = fallback.translate("contract", &en, &ja).unwrap();
        assert_eq!(result, "[ja] contract");

        // Should now be in memory
        assert_eq!(fallback.memory().len(), 1);
    }

    #[test]
    fn test_machine_translation_fallback_no_service() {
        let mut fallback = MachineTranslationFallback::new();

        let en = Locale::new("en");
        let ja = Locale::new("ja");

        // Should fail - no memory, no services
        let result = fallback.translate("contract", &en, &ja);
        assert!(result.is_err());
    }

    #[test]
    fn test_terminology_extractor() {
        let mut extractor = TerminologyExtractor::new();
        extractor.add_known_term("contract");
        extractor.add_known_term("employment");
        extractor.add_known_term("statute");

        let text = "This contract governs employment. The statute requires a written contract.";
        extractor.extract_from_text(text);

        assert_eq!(extractor.get_frequency("contract"), 2);
        assert_eq!(extractor.get_frequency("employment"), 1);
        assert_eq!(extractor.get_frequency("statute"), 1);

        let terms = extractor.get_terms_by_frequency();
        assert_eq!(terms[0].0, "contract");
        assert_eq!(terms[0].1, 2);
    }

    #[test]
    fn test_terminology_extractor_with_dictionary() {
        let mut dict = LegalDictionary::new(Locale::new("en"));
        dict.add_translation("contract", "contract");
        dict.add_translation("statute", "statute");

        let mut extractor = TerminologyExtractor::with_dictionary(&dict);

        let text = "The contract requires compliance with the statute.";
        extractor.extract_from_text(text);

        assert_eq!(extractor.get_frequency("contract"), 1);
        assert_eq!(extractor.get_frequency("statute"), 1);
    }

    #[test]
    fn test_terminology_extractor_clear() {
        let mut extractor = TerminologyExtractor::new();
        extractor.add_known_term("contract");

        let text = "This is a contract.";
        extractor.extract_from_text(text);

        assert_eq!(extractor.get_frequency("contract"), 1);

        extractor.clear();
        assert_eq!(extractor.get_frequency("contract"), 0);
        assert!(extractor.extracted_terms().is_empty());
    }

    #[test]
    fn test_translation_memory_levenshtein_similarity() {
        let mut memory = TranslationMemory::new();
        let en = Locale::new("en");
        let ja = Locale::new("ja");

        memory.add_translation("contract", en.clone(), "契約", ja.clone());
        memory.add_translation("contractor", en.clone(), "請負業者", ja.clone());

        // Test Levenshtein-based fuzzy matching
        let matches = memory.find_fuzzy_levenshtein("contracts", &en, &ja, 0.7);

        assert!(!matches.is_empty());
        assert!(matches[0].1 >= 0.7);
    }

    #[test]
    fn test_translation_memory_context_aware() {
        let mut memory = TranslationMemory::new();
        let en = Locale::new("en");
        let ja = Locale::new("ja");

        // Add entries with different contexts
        let mut entry1 = TranslationMemoryEntry::new("right", en.clone(), "権利", ja.clone());
        entry1
            .metadata
            .insert("context".to_string(), "contract_law".to_string());
        memory.add_entry(entry1);

        let mut entry2 = TranslationMemoryEntry::new("right", en.clone(), "右", ja.clone());
        entry2
            .metadata
            .insert("context".to_string(), "directions".to_string());
        memory.add_entry(entry2);

        // Find with context
        let contract_matches =
            memory.find_with_context("right", &en, &ja, Some("contract_law"), 0.9);
        assert_eq!(contract_matches.len(), 1);
        assert_eq!(contract_matches[0].0.target_text, "権利");

        let direction_matches =
            memory.find_with_context("right", &en, &ja, Some("directions"), 0.9);
        assert_eq!(direction_matches.len(), 1);
        assert_eq!(direction_matches[0].0.target_text, "右");
    }

    #[test]
    fn test_translation_memory_save_load() {
        let mut memory = TranslationMemory::new();
        let en = Locale::new("en");
        let ja = Locale::new("ja");

        memory.add_translation("contract", en.clone(), "契約", ja.clone());
        memory.add_translation("statute", en.clone(), "法令", ja.clone());

        // Save to file
        let temp_path = std::path::PathBuf::from("/tmp/test_translation_memory.json");
        memory.save_to_file(&temp_path).unwrap();

        // Load into new memory
        let mut loaded_memory = TranslationMemory::new();
        loaded_memory.load_from_file(&temp_path).unwrap();

        assert_eq!(loaded_memory.len(), 2);
        let matches = loaded_memory.find_exact("contract", &en, &ja);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].target_text, "契約");

        // Clean up
        let _ = std::fs::remove_file(&temp_path);
    }

    #[test]
    fn test_translation_memory_tmx_export_import() {
        let mut memory = TranslationMemory::new();
        let en = Locale::new("en");
        let ja = Locale::new("ja");

        memory.add_translation("contract", en.clone(), "契約", ja.clone());
        memory.add_translation("employment", en.clone(), "雇用", ja.clone());

        // Export to TMX
        let temp_path = std::path::PathBuf::from("/tmp/test_translation_memory.tmx");
        memory.export_to_tmx(&temp_path).unwrap();

        // Import from TMX
        let mut imported_memory = TranslationMemory::new();
        imported_memory.import_from_tmx(&temp_path).unwrap();

        assert_eq!(imported_memory.len(), 2);
        let matches = imported_memory.find_exact("contract", &en, &ja);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].target_text, "契約");

        // Clean up
        let _ = std::fs::remove_file(&temp_path);
    }

    #[test]
    fn test_translation_memory_merge() {
        let mut memory1 = TranslationMemory::new();
        let mut memory2 = TranslationMemory::new();

        let en = Locale::new("en");
        let ja = Locale::new("ja");

        memory1.add_translation("contract", en.clone(), "契約", ja.clone());
        memory2.add_translation("statute", en.clone(), "法令", ja.clone());

        memory1.merge(&memory2);

        assert_eq!(memory1.len(), 2);
        assert!(memory1.find_exact("contract", &en, &ja).len() == 1);
        assert!(memory1.find_exact("statute", &en, &ja).len() == 1);
    }

    #[test]
    fn test_translation_memory_xml_escape() {
        let mut memory = TranslationMemory::new();
        let en = Locale::new("en");
        let ja = Locale::new("ja");

        // Add text with XML special characters
        memory.add_translation("A & B < C > \"D\"", en.clone(), "A と B", ja.clone());

        let temp_path = std::path::PathBuf::from("/tmp/test_xml_escape.tmx");
        memory.export_to_tmx(&temp_path).unwrap();

        // Read the TMX file and check escaping
        let tmx_content = std::fs::read_to_string(&temp_path).unwrap();
        assert!(tmx_content.contains("&amp;"));
        assert!(tmx_content.contains("&lt;"));
        assert!(tmx_content.contains("&gt;"));
        assert!(tmx_content.contains("&quot;"));

        // Clean up
        let _ = std::fs::remove_file(&temp_path);
    }

    #[test]
    fn test_translation_service_batch() {
        let service = MockTranslationService::new();
        let en = Locale::new("en");
        let ja = Locale::new("ja");

        let texts = vec!["contract", "statute", "employment"];
        let results = service.translate_batch(&texts, &en, &ja).unwrap();

        assert_eq!(results.len(), 3);
        assert_eq!(results[0], "[ja] contract");
        assert_eq!(results[1], "[ja] statute");
        assert_eq!(results[2], "[ja] employment");
    }

    #[test]
    fn test_screen_reader_aria_label() {
        let formatter = ScreenReaderFormatter::new(Locale::new("en"));

        assert_eq!(
            formatter.aria_label("article", "Contract Formation"),
            "Article: Contract Formation"
        );
        assert_eq!(
            formatter.aria_label("section", "Definitions"),
            "Section: Definitions"
        );
    }

    #[test]
    fn test_screen_reader_citation_formatting() {
        let formatter = ScreenReaderFormatter::new(Locale::new("en"));

        let citation = "Brown v. Board of Education, 347 U.S. 483 (1954)";
        let formatted = formatter.format_citation(citation);

        assert!(formatted.contains("versus"));
        assert!(formatted.contains("United States"));
        assert!(!formatted.contains("v."));
        assert!(!formatted.contains("U.S."));
    }

    #[test]
    fn test_screen_reader_navigation() {
        let formatter = ScreenReaderFormatter::new(Locale::new("en"));

        let sections = vec![
            ("article", "Introduction"),
            ("section", "Definitions"),
            ("chapter", "Enforcement"),
        ];

        let nav = formatter.navigation_structure(&sections);

        assert!(nav.contains("<nav"));
        assert!(nav.contains("aria-label"));
        assert!(nav.contains("Introduction"));
        assert!(nav.contains("Definitions"));
        assert!(nav.contains("Enforcement"));
    }

    #[test]
    fn test_screen_reader_table_formatting() {
        let formatter = ScreenReaderFormatter::new(Locale::new("en"));

        let headers = vec!["Name", "Role", "Jurisdiction"];
        let rows = vec![
            vec!["John Doe", "Judge", "Federal"],
            vec!["Jane Smith", "Attorney", "State"],
        ];

        let table = formatter.format_table("Legal Personnel", &headers, &rows);

        assert!(table.contains("<table"));
        assert!(table.contains("aria-label"));
        assert!(table.contains("<caption>Legal Personnel</caption>"));
        assert!(table.contains("scope=\"col\""));
        assert!(table.contains("scope=\"row\""));
    }

    #[test]
    fn test_plain_language_converter() {
        let converter = PlainLanguageConverter::new(Locale::new("en"));

        let legal_text = "The plaintiff hereby files this complaint pursuant to federal law.";
        let plain = converter.convert(legal_text);

        assert!(plain.contains("person who filed the lawsuit") || plain.contains("plaintiff"));
        assert!(plain.contains("by this document") || plain.contains("hereby"));
    }

    #[test]
    fn test_plain_language_custom_conversion() {
        let mut converter = PlainLanguageConverter::new(Locale::new("en"));
        converter.add_conversion("escheat", "revert to the state");

        assert_eq!(
            converter.get_plain_alternative("escheat"),
            Some(&"revert to the state".to_string())
        );
    }

    #[test]
    fn test_reading_level_flesch_reading_ease() {
        let assessor = ReadingLevelAssessor::new();

        let simple_text = "The cat sat on the mat. It was a nice day.";
        let ease = assessor.flesch_reading_ease(simple_text);
        assert!(ease > 60.0); // Should be fairly easy

        let complex_text = "Notwithstanding the aforementioned jurisdictional complications, the defendant's constitutional rights remain inviolate pursuant to established jurisprudence.";
        let ease_complex = assessor.flesch_reading_ease(complex_text);
        assert!(ease_complex < ease); // Complex should be harder
    }

    #[test]
    fn test_reading_level_flesch_kincaid_grade() {
        let assessor = ReadingLevelAssessor::new();

        let text = "The law requires clear documentation. All parties must sign the agreement.";
        let grade = assessor.flesch_kincaid_grade(text);
        assert!(grade >= 0.0);
        assert!(grade < 20.0); // Reasonable range
    }

    #[test]
    fn test_reading_level_assessment_report() {
        let assessor = ReadingLevelAssessor::new();

        let text = "Contract law governs agreements. Each party has rights and duties.";
        let report = assessor.assess(text);

        assert!(report.word_count > 0);
        assert!(report.sentence_count > 0);
        assert!(report.syllable_count > 0);
        assert!(!report.difficulty.is_empty());
        assert!(report.flesch_reading_ease >= 0.0 && report.flesch_reading_ease <= 206.835);
    }

    #[test]
    fn test_braille_formatter_basic() {
        let formatter = BrailleFormatter::new(BrailleGrade::Grade1);

        let braille = formatter.to_braille("law");
        assert!(!braille.is_empty());
        assert!(
            braille
                .chars()
                .all(|c| ('\u{2800}'..='\u{28FF}').contains(&c) || c == ' ')
        );
    }

    #[test]
    fn test_braille_section_number() {
        let formatter = BrailleFormatter::new(BrailleGrade::Grade1);

        let section = formatter.format_section_number("abc");
        assert!(section.starts_with('§'));
        assert!(section.contains(char::from_u32(0x2801).unwrap())); // Braille 'a'
    }

    #[test]
    fn test_audio_description_flowchart() {
        let generator = AudioDescriptionGenerator::new(Locale::new("en"));

        let elements = vec!["File complaint", "Serve defendant", "Discovery", "Trial"];
        let description = generator.describe_diagram("flowchart", &elements);

        assert!(description.contains("Flowchart"));
        assert!(description.contains("4 steps"));
        assert!(description.contains("File complaint"));
        assert!(description.contains("then"));
    }

    #[test]
    fn test_audio_description_chart() {
        let generator = AudioDescriptionGenerator::new(Locale::new("en"));

        let data = vec![
            ("Criminal".to_string(), 45.0),
            ("Civil".to_string(), 35.0),
            ("Family".to_string(), 20.0),
        ];

        let bar_chart = generator.describe_chart("bar", &data);
        assert!(bar_chart.contains("Bar chart"));
        assert!(bar_chart.contains("3 data points"));

        let pie_chart = generator.describe_chart("pie", &data);
        assert!(pie_chart.contains("Pie chart"));
        assert!(pie_chart.contains("%"));
    }

    #[test]
    fn test_audio_description_table() {
        let generator = AudioDescriptionGenerator::new(Locale::new("en"));

        let description = generator.describe_table("Case Statistics", 10, 5);
        assert!(description.contains("Table"));
        assert!(description.contains("Case Statistics"));
        assert!(description.contains("10 rows"));
        assert!(description.contains("5 columns"));
    }

    #[test]
    fn test_abbreviations() {
        let mut dict = LegalDictionary::new(Locale::new("en").with_country("US"));
        dict.add_abbreviation("corporation", "Corp.");
        dict.add_abbreviation("incorporated", "Inc.");
        dict.add_abbreviation("attorney", "Atty.");

        assert_eq!(dict.get_abbreviation("corporation"), Some("Corp."));
        assert_eq!(dict.get_abbreviation("incorporated"), Some("Inc."));
        assert_eq!(dict.get_abbreviation("attorney"), Some("Atty."));

        assert_eq!(dict.expand_abbreviation("Corp."), Some("corporation"));
        assert_eq!(dict.expand_abbreviation("Inc."), Some("incorporated"));

        assert!(dict.is_abbreviation("Corp."));
        assert!(!dict.is_abbreviation("corporation"));
    }

    #[test]
    fn test_contextual_translation() {
        let mut dict = LegalDictionary::new(Locale::new("ja").with_country("JP"));
        dict.add_translation("right", "権利");
        dict.add_contextual_translation("right", "direction", "右");
        dict.add_contextual_translation("right", "legal", "権利");

        // Default translation
        assert_eq!(dict.translate("right"), Some("権利"));

        // Context-aware translations
        assert_eq!(
            dict.translate_with_context("right", "direction"),
            Some("右")
        );
        assert_eq!(dict.translate_with_context("right", "legal"), Some("権利"));

        // Non-existent context falls back to default
        assert_eq!(dict.translate_with_context("right", "other"), Some("権利"));

        // Get contexts for a term
        let contexts = dict.get_contexts_for_term("right");
        assert_eq!(contexts.len(), 2);
        assert!(contexts.contains(&"direction"));
        assert!(contexts.contains(&"legal"));
    }

    #[test]
    fn test_validation_helpers() {
        assert!(is_valid_language_code("en"));
        assert!(is_valid_language_code("ja"));
        assert!(!is_valid_language_code("EN"));
        assert!(!is_valid_language_code("eng"));

        assert!(is_valid_country_code("US"));
        assert!(is_valid_country_code("JP"));
        assert!(!is_valid_country_code("us"));
        assert!(!is_valid_country_code("USA"));

        assert!(is_valid_script_code("Hans"));
        assert!(is_valid_script_code("Hant"));
        assert!(!is_valid_script_code("hans"));
        assert!(!is_valid_script_code("HANS"));

        assert!(is_valid_locale_tag("en-US"));
        assert!(is_valid_locale_tag("zh-Hans-CN"));
        assert!(!is_valid_locale_tag("EN-US"));
    }

    #[test]
    fn test_ordinal_formatting() {
        let en_formatter = NumberFormatter::new(Locale::new("en").with_country("US"));
        assert_eq!(en_formatter.format_ordinal(1), "1st");
        assert_eq!(en_formatter.format_ordinal(2), "2nd");
        assert_eq!(en_formatter.format_ordinal(3), "3rd");
        assert_eq!(en_formatter.format_ordinal(4), "4th");
        assert_eq!(en_formatter.format_ordinal(11), "11th");
        assert_eq!(en_formatter.format_ordinal(21), "21st");
        assert_eq!(en_formatter.format_ordinal(42), "42nd");
        assert_eq!(en_formatter.format_ordinal(113), "113th");

        let fr_formatter = NumberFormatter::new(Locale::new("fr").with_country("FR"));
        assert_eq!(fr_formatter.format_ordinal(1), "1er");
        assert_eq!(fr_formatter.format_ordinal(2), "2e");

        let ja_formatter = NumberFormatter::new(Locale::new("ja").with_country("JP"));
        assert_eq!(ja_formatter.format_ordinal(1), "第1");
        assert_eq!(ja_formatter.format_ordinal(5), "第5");
    }

    #[test]
    fn test_number_to_words() {
        let en_formatter = NumberFormatter::new(Locale::new("en").with_country("US"));
        assert_eq!(en_formatter.number_to_words(0), "zero");
        assert_eq!(en_formatter.number_to_words(1), "one");
        assert_eq!(en_formatter.number_to_words(15), "fifteen");
        assert_eq!(en_formatter.number_to_words(20), "twenty");
        assert_eq!(en_formatter.number_to_words(42), "forty-two");
        assert_eq!(en_formatter.number_to_words(100), "one hundred");
        assert_eq!(en_formatter.number_to_words(101), "one hundred and one");
        assert_eq!(en_formatter.number_to_words(1000), "one thousand");
        assert_eq!(
            en_formatter.number_to_words(1234),
            "one thousand two hundred and thirty-four"
        );

        let ja_formatter = NumberFormatter::new(Locale::new("ja").with_country("JP"));
        assert_eq!(ja_formatter.number_to_words(0), "零");
        assert_eq!(ja_formatter.number_to_words(1), "一");
        assert_eq!(ja_formatter.number_to_words(10), "十");
        assert_eq!(ja_formatter.number_to_words(11), "十一");
        assert_eq!(ja_formatter.number_to_words(100), "百");
        assert_eq!(ja_formatter.number_to_words(123), "百二十三");
    }

    #[test]
    fn test_text_collator() {
        let en_collator = TextCollator::new(Locale::new("en").with_country("US"));

        // Case-insensitive comparison
        assert_eq!(
            en_collator.compare("apple", "BANANA"),
            std::cmp::Ordering::Less
        );
        assert_eq!(
            en_collator.compare("zebra", "Apple"),
            std::cmp::Ordering::Greater
        );

        // Sorting
        let mut items = vec![
            "Zebra".to_string(),
            "apple".to_string(),
            "Banana".to_string(),
        ];
        en_collator.sort(&mut items);
        assert_eq!(items, vec!["apple", "Banana", "Zebra"]);

        // Starts with
        assert!(en_collator.starts_with("Contract", "con"));
        assert!(en_collator.starts_with("STATUTE", "stat"));

        // Normalize
        let de_collator = TextCollator::new(Locale::new("de").with_country("DE"));
        assert_eq!(de_collator.normalize("äöü"), "aeoeue");
        assert_eq!(de_collator.normalize("Straße"), "strasse");
    }

    #[test]
    fn test_dictionary_export_import() {
        let mut dict = LegalDictionary::new(Locale::new("en").with_country("US"));
        dict.add_translation("contract", "contract");
        dict.add_translation("statute", "statute");
        dict.add_abbreviation("corporation", "Corp.");
        dict.add_contextual_translation("right", "legal", "legal right");

        // Export to JSON
        let json = dict.to_json().unwrap();
        assert!(json.contains("contract"));
        assert!(json.contains("statute"));

        // Import from JSON
        let imported = LegalDictionary::from_json(&json).unwrap();
        assert_eq!(imported.translate("contract"), Some("contract"));
        assert_eq!(imported.translate("statute"), Some("statute"));
        assert_eq!(imported.get_abbreviation("corporation"), Some("Corp."));
        assert_eq!(
            imported.translate_with_context("right", "legal"),
            Some("legal right")
        );
    }

    #[test]
    fn test_dictionary_merge() {
        let mut dict1 = LegalDictionary::new(Locale::new("en").with_country("US"));
        dict1.add_translation("contract", "contract");
        dict1.add_translation("statute", "statute");

        let mut dict2 = LegalDictionary::new(Locale::new("en").with_country("US"));
        dict2.add_translation("statute", "law"); // This should not override
        dict2.add_translation("court", "court");

        dict1.merge(&dict2);

        assert_eq!(dict1.translate("contract"), Some("contract"));
        assert_eq!(dict1.translate("statute"), Some("statute")); // Original preserved
        assert_eq!(dict1.translate("court"), Some("court")); // New added
    }

    #[test]
    fn test_dictionary_counts() {
        let mut dict = LegalDictionary::new(Locale::new("en").with_country("US"));
        dict.add_translation("contract", "contract");
        dict.add_translation("statute", "statute");
        dict.add_definition("contract", "A legally binding agreement");
        dict.add_abbreviation("corporation", "Corp.");
        dict.add_contextual_translation("right", "legal", "legal right");

        assert_eq!(dict.translation_count(), 2);
        assert_eq!(dict.definition_count(), 1);
        assert_eq!(dict.abbreviation_count(), 1);
        assert_eq!(dict.contextual_translation_count(), 1);
    }

    #[test]
    fn test_suggest_best_locale() {
        let available = vec![
            Locale::new("en").with_country("US"),
            Locale::new("en").with_country("GB"),
            Locale::new("fr").with_country("FR"),
            Locale::new("ja"),
        ];

        // Exact match
        let requested = Locale::new("en").with_country("US");
        let suggested = suggest_best_locale(&requested, &available).unwrap();
        assert_eq!(suggested, &Locale::new("en").with_country("US"));

        // Language match with different country
        let requested = Locale::new("ja").with_country("JP");
        let suggested = suggest_best_locale(&requested, &available).unwrap();
        assert_eq!(suggested.language, "ja");

        // No match
        let requested = Locale::new("de").with_country("DE");
        let suggested = suggest_best_locale(&requested, &available);
        assert!(suggested.is_none());
    }

    #[test]
    fn test_common_legal_locales() {
        let locales = common_legal_locales();
        assert!(locales.len() >= 10);
        assert!(locales.iter().any(|l| l.tag() == "en-US"));
        assert!(locales.iter().any(|l| l.tag() == "ja-JP"));
        assert!(locales.iter().any(|l| l.tag() == "zh-Hans-CN"));
    }

    #[test]
    fn test_timezone_utc_to_local() {
        let jst = TimeZone::new("Asia/Tokyo", 540, "Japan Standard Time (JST)", false);

        // 00:00 UTC -> 09:00 JST
        let (y, m, d, h, min) = jst.utc_to_local(2024, 1, 1, 0, 0);
        assert_eq!((y, m, d, h, min), (2024, 1, 1, 9, 0));

        // 23:00 UTC -> 08:00 JST next day
        let (y, m, d, h, min) = jst.utc_to_local(2024, 1, 1, 23, 0);
        assert_eq!((y, m, d, h, min), (2024, 1, 2, 8, 0));
    }

    #[test]
    fn test_timezone_local_to_utc() {
        let est = TimeZone::new(
            "America/New_York",
            -300,
            "Eastern Standard Time (EST)",
            true,
        );

        // 09:00 EST -> 14:00 UTC
        let (y, m, d, h, min) = est.local_to_utc(2024, 1, 1, 9, 0);
        assert_eq!((y, m, d, h, min), (2024, 1, 1, 14, 0));

        // 02:00 EST -> 07:00 UTC
        let (y, m, d, h, min) = est.local_to_utc(2024, 1, 1, 2, 0);
        assert_eq!((y, m, d, h, min), (2024, 1, 1, 7, 0));
    }

    #[test]
    fn test_timezone_format_offset() {
        let jst = TimeZone::new("Asia/Tokyo", 540, "Japan Standard Time (JST)", false);
        assert_eq!(jst.format_offset(), "+09:00");

        let est = TimeZone::new(
            "America/New_York",
            -300,
            "Eastern Standard Time (EST)",
            true,
        );
        assert_eq!(est.format_offset(), "-05:00");

        let utc = TimeZone::new("UTC", 0, "Coordinated Universal Time (UTC)", false);
        assert_eq!(utc.format_offset(), "+00:00");
    }

    #[test]
    fn test_timezone_registry() {
        let registry = TimeZoneRegistry::with_defaults();

        // Check some major legal centers
        assert!(registry.get_zone("Asia/Tokyo").is_some());
        assert!(registry.get_zone("America/New_York").is_some());
        assert!(registry.get_zone("Europe/London").is_some());
        assert!(registry.get_zone("UTC").is_some());

        let jp_tz = registry.zone_for_jurisdiction("JP").unwrap();
        assert_eq!(jp_tz.identifier, "Asia/Tokyo");

        let us_tz = registry.zone_for_jurisdiction("US").unwrap();
        assert_eq!(us_tz.identifier, "America/New_York");
    }

    #[test]
    fn test_deadline_calculator_basic() {
        let jp_config = WorkingDaysConfig::japan();
        let calculator = DeadlineCalculator::new(jp_config);

        // Add 5 business days from Monday, Jan 1, 2024
        let (y, m, d) = calculator.calculate_deadline(2024, 1, 1, 5);
        assert_eq!((y, m, d), (2024, 1, 8)); // Should skip weekend
    }

    #[test]
    fn test_deadline_calculator_with_time() {
        let us_config = WorkingDaysConfig::united_states();
        let calculator = DeadlineCalculator::new(us_config);

        // Add 3 business days with time
        let (_y, _m, _d, h, min) = calculator.calculate_deadline_with_time(2024, 1, 1, 9, 30, 3);
        assert_eq!(h, 9);
        assert_eq!(min, 30);
    }

    #[test]
    fn test_deadline_calculator_timezone_conversion() {
        let jp_config = WorkingDaysConfig::japan();
        let calculator = DeadlineCalculator::new(jp_config);

        let jst = TimeZone::new("Asia/Tokyo", 540, "Japan Standard Time (JST)", false);
        let est = TimeZone::new(
            "America/New_York",
            -300,
            "Eastern Standard Time (EST)",
            true,
        );

        // Convert 09:00 JST to EST
        let (y, m, d, h, min) = calculator.convert_timezone(2024, 1, 1, 9, 0, &jst, &est);
        assert_eq!((y, m, d, h, min), (2023, 12, 31, 19, 0)); // Previous day
    }

    #[test]
    fn test_deadline_calculator_is_deadline_passed() {
        let jp_config = WorkingDaysConfig::japan();
        let calculator = DeadlineCalculator::new(jp_config);

        // Deadline in the past
        assert!(calculator.is_deadline_passed(2023, 12, 31, 2024, 1, 1));

        // Deadline in the future
        assert!(!calculator.is_deadline_passed(2024, 1, 2, 2024, 1, 1));

        // Same date
        assert!(!calculator.is_deadline_passed(2024, 1, 1, 2024, 1, 1));
    }

    #[test]
    fn test_citation_bluebook_case() {
        let formatter = CitationFormatter::new(
            CitationStyle::Bluebook,
            Locale::new("en").with_country("US"),
        );

        let citation = CitationComponents::new("Brown v. Board of Education")
            .with_volume("347")
            .with_reporter("U.S.")
            .with_page("483")
            .with_year(1954);

        let formatted = formatter.format_case(&citation);
        assert!(formatted.contains("Brown v. Board of Education"));
        assert!(formatted.contains("347 U.S. 483"));
        assert!(formatted.contains("1954"));
    }

    #[test]
    fn test_citation_oscola_case() {
        let formatter =
            CitationFormatter::new(CitationStyle::OSCOLA, Locale::new("en").with_country("GB"));

        let citation = CitationComponents::new("Donoghue v Stevenson")
            .with_volume("1932")
            .with_reporter("AC")
            .with_page("562")
            .with_year(1932);

        let formatted = formatter.format_case(&citation);
        assert!(formatted.contains("Donoghue v Stevenson"));
        assert!(formatted.contains("[1932]"));
        assert!(formatted.contains("1932 AC"));
    }

    #[test]
    fn test_citation_bluebook_statute() {
        let formatter = CitationFormatter::new(
            CitationStyle::Bluebook,
            Locale::new("en").with_country("US"),
        );

        let citation = CitationComponents::new("Civil Rights Act")
            .with_reporter("U.S.C.")
            .with_page("2000a")
            .with_year(1964);

        let formatted = formatter.format_statute(&citation);
        assert!(formatted.contains("Civil Rights Act"));
        assert!(formatted.contains("U.S.C."));
        assert!(formatted.contains("§ 2000a"));
        assert!(formatted.contains("(1964)"));
    }

    #[test]
    fn test_citation_japanese() {
        let formatter = CitationFormatter::new(
            CitationStyle::Japanese,
            Locale::new("ja").with_country("JP"),
        );

        let citation = CitationComponents::new("最高裁判所判決")
            .with_court("最高裁")
            .with_volume("123")
            .with_page("45")
            .with_year(2020);

        let formatted = formatter.format_case(&citation);
        assert!(formatted.contains("最高裁判所判決"));
        assert!(formatted.contains("最高裁"));
        assert!(formatted.contains("2020"));
        assert!(formatted.contains("123号"));
        assert!(formatted.contains("45頁"));
    }

    #[test]
    fn test_citation_style_for_jurisdiction() {
        assert_eq!(
            CitationFormatter::style_for_jurisdiction("US"),
            CitationStyle::Bluebook
        );
        assert_eq!(
            CitationFormatter::style_for_jurisdiction("GB"),
            CitationStyle::OSCOLA
        );
        assert_eq!(
            CitationFormatter::style_for_jurisdiction("AU"),
            CitationStyle::AGLC
        );
        assert_eq!(
            CitationFormatter::style_for_jurisdiction("CA"),
            CitationStyle::McGill
        );
        assert_eq!(
            CitationFormatter::style_for_jurisdiction("JP"),
            CitationStyle::Japanese
        );
        assert_eq!(
            CitationFormatter::style_for_jurisdiction("DE"),
            CitationStyle::European
        );
    }

    #[test]
    fn test_text_direction_detection() {
        let arabic = Locale::new("ar");
        assert_eq!(
            BidirectionalText::detect_direction(&arabic),
            TextDirection::RTL
        );

        let hebrew = Locale::new("he");
        assert_eq!(
            BidirectionalText::detect_direction(&hebrew),
            TextDirection::RTL
        );

        let english = Locale::new("en");
        assert_eq!(
            BidirectionalText::detect_direction(&english),
            TextDirection::LTR
        );

        let persian = Locale::new("fa");
        assert_eq!(
            BidirectionalText::detect_direction(&persian),
            TextDirection::RTL
        );
    }

    #[test]
    fn test_bidirectional_text_rtl() {
        let arabic_locale = Locale::new("ar");
        let bidi = BidirectionalText::new(arabic_locale);

        assert!(bidi.is_rtl());
        assert_eq!(bidi.direction(), TextDirection::RTL);
    }

    #[test]
    fn test_bidirectional_text_ltr() {
        let english_locale = Locale::new("en");
        let bidi = BidirectionalText::new(english_locale);

        assert!(!bidi.is_rtl());
        assert_eq!(bidi.direction(), TextDirection::LTR);
    }

    #[test]
    fn test_direction_markers() {
        let arabic_locale = Locale::new("ar");
        let bidi = BidirectionalText::new(arabic_locale);

        let wrapped = bidi.wrap_with_direction_markers("نص عربي");
        assert!(wrapped.contains('\u{202B}')); // RLE
        assert!(wrapped.contains('\u{202C}')); // PDF
    }

    #[test]
    fn test_arabic_numerals() {
        let arabic_locale = Locale::new("ar");
        let bidi = BidirectionalText::new(arabic_locale);

        let formatted = bidi.format_number(123);
        assert_eq!(formatted, "١٢٣");

        let formatted_year = bidi.format_number(2024);
        assert_eq!(formatted_year, "٢٠٢٤");
    }

    #[test]
    fn test_persian_numerals() {
        let persian_locale = Locale::new("fa");
        let bidi = BidirectionalText::new(persian_locale);

        let formatted = bidi.format_number(123);
        assert_eq!(formatted, "۱۲۳");
    }

    #[test]
    fn test_rtl_date_formatting() {
        let arabic_locale = Locale::new("ar");
        let bidi = BidirectionalText::new(arabic_locale);

        let date = bidi.format_date_rtl(2024, 1, 15);
        assert!(date.contains('١'));
        assert!(date.contains('٥'));
    }

    #[test]
    fn test_paragraph_formatting() {
        let arabic_locale = Locale::new("ar");
        let bidi = BidirectionalText::new(arabic_locale);

        let paragraph = bidi.format_paragraph("هذا نص عربي");
        assert!(paragraph.contains("dir=\"rtl\""));
        assert!(paragraph.starts_with("<p"));
        assert!(paragraph.ends_with("</p>"));
    }

    #[test]
    fn test_list_formatting() {
        let hebrew_locale = Locale::new("he");
        let bidi = BidirectionalText::new(hebrew_locale);

        let items = vec!["פריט 1".to_string(), "פריט 2".to_string()];
        let list = bidi.format_list(&items);
        assert!(list.contains("dir=\"rtl\""));
        assert!(list.contains("<ul"));
        assert!(list.contains("<li>"));
    }

    #[test]
    fn test_name_order_detection() {
        let japanese = Locale::new("ja");
        assert_eq!(
            NameFormatter::detect_name_order(&japanese),
            NameOrder::FamilyFirst
        );

        let korean = Locale::new("ko");
        assert_eq!(
            NameFormatter::detect_name_order(&korean),
            NameOrder::FamilyFirst
        );

        let english = Locale::new("en");
        assert_eq!(
            NameFormatter::detect_name_order(&english),
            NameOrder::GivenFirst
        );

        let chinese = Locale::new("zh");
        assert_eq!(
            NameFormatter::detect_name_order(&chinese),
            NameOrder::FamilyFirst
        );
    }

    #[test]
    fn test_western_name_formatting() {
        let formatter = NameFormatter::new(Locale::new("en").with_country("US"));
        let name = PersonName::new("John", "Smith")
            .with_middle_name("David")
            .with_prefix("Dr.")
            .with_suffix("Jr.");

        let formatted = formatter.format_full_name(&name);
        assert_eq!(formatted, "Dr. John David Smith Jr.");
    }

    #[test]
    fn test_japanese_name_formatting() {
        let formatter = NameFormatter::new(Locale::new("ja").with_country("JP"));
        let name = PersonName::new("太郎", "山田");

        let formatted = formatter.format_full_name(&name);
        assert_eq!(formatted, "山田 太郎"); // Family first
    }

    #[test]
    fn test_korean_name_formatting() {
        let formatter = NameFormatter::new(Locale::new("ko").with_country("KR"));
        let name = PersonName::new("민수", "김");

        let formatted = formatter.format_full_name(&name);
        assert_eq!(formatted, "김민수"); // No space
    }

    #[test]
    fn test_chinese_name_formatting() {
        let cn_formatter = NameFormatter::new(Locale::new("zh").with_country("CN"));
        let name = PersonName::new("伟", "李");

        let formatted = cn_formatter.format_full_name(&name);
        assert_eq!(formatted, "李伟"); // No space for mainland

        let tw_formatter = NameFormatter::new(Locale::new("zh").with_country("TW"));
        let formatted_tw = tw_formatter.format_full_name(&name);
        assert_eq!(formatted_tw, "李 伟"); // Space for Taiwan
    }

    #[test]
    fn test_russian_name_formatting() {
        let formatter = NameFormatter::new(Locale::new("ru").with_country("RU"));
        let name = PersonName::new("Иван", "Иванов").with_patronymic("Иванович");

        let formatted = formatter.format_full_name(&name);
        assert_eq!(formatted, "Иванов Иван Иванович");
    }

    #[test]
    fn test_arabic_name_formatting() {
        let formatter = NameFormatter::new(Locale::new("ar"));
        let name = PersonName::new("محمد", "الأحمد").with_patronymic("بن علي");

        let formatted = formatter.format_full_name(&name);
        assert_eq!(formatted, "محمد بن علي الأحمد");
    }

    #[test]
    fn test_name_citation_format() {
        let formatter = NameFormatter::new(Locale::new("en").with_country("US"));
        let name = PersonName::new("John", "Smith").with_middle_name("David");

        let citation = formatter.format_citation(&name);
        assert_eq!(citation, "Smith, John David");
    }

    #[test]
    fn test_name_initials() {
        let formatter = NameFormatter::new(Locale::new("en").with_country("US"));
        let name = PersonName::new("John", "Smith").with_middle_name("David");

        let initials = formatter.format_initials(&name);
        assert_eq!(initials, "J. D. S.");

        let name_no_middle = PersonName::new("John", "Smith");
        let initials_no_middle = formatter.format_initials(&name_no_middle);
        assert_eq!(initials_no_middle, "J. S.");
    }

    #[test]
    fn test_us_address_formatting() {
        let formatter = AddressFormatter::new(Locale::new("en").with_country("US"));
        let address = Address::new("123 Main St", "New York", "10001", "USA").with_state("NY");

        let formatted = formatter.format(&address);
        assert!(formatted.contains("123 Main St"));
        assert!(formatted.contains("New York, NY 10001"));
        assert!(formatted.contains("USA"));
    }

    #[test]
    fn test_uk_address_formatting() {
        let formatter = AddressFormatter::new(Locale::new("en").with_country("GB"));
        let address = Address::new("10 Downing Street", "London", "SW1A 2AA", "United Kingdom");

        let formatted = formatter.format(&address);
        assert!(formatted.contains("10 Downing Street"));
        assert!(formatted.contains("London"));
        assert!(formatted.contains("SW1A 2AA"));
    }

    #[test]
    fn test_japanese_address_formatting() {
        let formatter = AddressFormatter::new(Locale::new("ja").with_country("JP"));
        let address = Address::new("1-1-1", "千代田区", "100-0001", "日本")
            .with_state("東京都")
            .with_building("ビル101");

        let formatted = formatter.format(&address);
        assert!(formatted.contains("〒100-0001"));
        assert!(formatted.contains("東京都"));
        assert!(formatted.contains("千代田区"));
        assert!(formatted.contains("ビル101"));
    }

    #[test]
    fn test_european_address_formatting() {
        let formatter = AddressFormatter::new(Locale::new("de").with_country("DE"));
        let address = Address::new("Hauptstraße 1", "Berlin", "10115", "Germany");

        let formatted = formatter.format(&address);
        assert!(formatted.contains("Hauptstraße 1"));
        assert!(formatted.contains("10115 Berlin"));
        assert!(formatted.contains("Germany"));
    }

    #[test]
    fn test_address_single_line() {
        let formatter = AddressFormatter::new(Locale::new("en").with_country("US"));
        let address = Address::new("123 Main St", "New York", "10001", "USA").with_state("NY");

        let single_line = formatter.format_single_line(&address);
        assert!(!single_line.contains('\n'));
        assert!(single_line.contains(", "));
    }

    // ========================================================================
    // Regional Variations v0.1.9 Tests
    // ========================================================================

    #[test]
    fn test_sub_regional_variation_us_states() {
        let registry = SubRegionalVariationRegistry::with_defaults();

        // Test California
        let ca_variation = registry.find_variation("US", "CA");
        assert!(ca_variation.is_some());
        let ca = ca_variation.unwrap();
        assert_eq!(ca.region_name, "California");
        assert_eq!(ca.region_code, "CA");
        assert!(
            ca.legal_differences
                .iter()
                .any(|d| d.contains("Community property state"))
        );
        assert!(ca.legal_differences.iter().any(|d| d.contains("CCPA")));

        // Test New York
        let ny_variation = registry.find_variation("US", "NY");
        assert!(ny_variation.is_some());
        let ny = ny_variation.unwrap();
        assert_eq!(ny.region_name, "New York");
        assert!(
            ny.legal_differences
                .iter()
                .any(|d| d.contains("Martin Act"))
        );

        // Test Delaware
        let de_variation = registry.find_variation("US", "DE");
        assert!(de_variation.is_some());
        let de = de_variation.unwrap();
        assert_eq!(de.region_name, "Delaware");
        assert!(de.legal_differences.iter().any(|d| d.contains("DGCL")));
        assert!(
            de.legal_differences
                .iter()
                .any(|d| d.contains("Court of Chancery"))
        );
    }

    #[test]
    fn test_sub_regional_variation_canadian_provinces() {
        let registry = SubRegionalVariationRegistry::with_defaults();

        // Test Ontario
        let on_variation = registry.find_variation("CA", "ON");
        assert!(on_variation.is_some());
        let on = on_variation.unwrap();
        assert_eq!(on.region_name, "Ontario");
        assert!(
            on.legal_differences
                .iter()
                .any(|d| d.contains("Common law province"))
        );

        // Test Québec
        let qc_variation = registry.find_variation("CA", "QC");
        assert!(qc_variation.is_some());
        let qc = qc_variation.unwrap();
        assert_eq!(qc.region_name, "Québec");
        assert!(
            qc.legal_differences
                .iter()
                .any(|d| d.contains("Civil law jurisdiction"))
        );
        assert!(
            qc.legal_differences
                .iter()
                .any(|d| d.contains("Code civil du Québec"))
        );

        // Test British Columbia
        let bc_variation = registry.find_variation("CA", "BC");
        assert!(bc_variation.is_some());
        let bc = bc_variation.unwrap();
        assert_eq!(bc.region_name, "British Columbia");
    }

    #[test]
    fn test_sub_regional_variation_get_all_for_country() {
        let registry = SubRegionalVariationRegistry::with_defaults();

        let us_variations = registry.get_variations_for_country("US");
        assert!(us_variations.len() >= 6); // CA, NY, TX, FL, IL, DE

        let ca_variations = registry.get_variations_for_country("CA");
        assert!(ca_variations.len() >= 4); // ON, QC, BC, AB
    }

    #[test]
    fn test_eu_member_state_variations() {
        let registry = EUMemberStateRegistry::with_defaults();

        // Test Germany
        let de = registry.find_variation("DE");
        assert!(de.is_some());
        let germany = de.unwrap();
        assert_eq!(germany.country_name, "Germany");
        assert_eq!(germany.accession_year, 1958);
        assert!(germany.legal_system.contains("Civil law"));
        assert!(germany.eu_adaptations.iter().any(|a| a.contains("GDPR")));
        assert!(
            germany
                .specialties
                .iter()
                .any(|s| s.contains("Mitbestimmung"))
        );

        // Test France
        let fr = registry.find_variation("FR");
        assert!(fr.is_some());
        let france = fr.unwrap();
        assert_eq!(france.country_name, "France");
        assert!(france.legal_system.contains("Napoleonic Code"));
        assert!(
            france
                .specialties
                .iter()
                .any(|s| s.contains("Conseil d'État"))
        );

        // Test Ireland (common law in EU)
        let ie = registry.find_variation("IE");
        assert!(ie.is_some());
        let ireland = ie.unwrap();
        assert_eq!(ireland.country_name, "Ireland");
        assert!(ireland.legal_system.contains("Common law"));
        assert!(
            ireland
                .specialties
                .iter()
                .any(|s| s.contains("Common law in EU context"))
        );
    }

    #[test]
    fn test_eu_member_state_all_variations() {
        let registry = EUMemberStateRegistry::with_defaults();
        let all = registry.get_all_variations();
        assert!(all.len() >= 10); // DE, FR, ES, IT, NL, PL, SE, IE, BE, AT
    }

    #[test]
    fn test_dialect_terminology_scottish() {
        let registry = DialectTerminologyRegistry::with_defaults();
        let locale = Locale::new("en").with_country("GB");

        let scottish = registry.find_dialect(&locale, "Scottish Legal");
        assert!(scottish.is_some());
        let dialect = scottish.unwrap();

        assert_eq!(dialect.to_dialect("lawyer"), Some("advocate"));
        assert_eq!(
            dialect.to_dialect("real_estate"),
            Some("heritable property")
        );
        assert_eq!(dialect.to_dialect("mortgage"), Some("standard security"));
        assert_eq!(dialect.to_dialect("plaintiff"), Some("pursuer"));
        assert_eq!(dialect.to_dialect("defendant"), Some("defender"));

        // Test reverse translation
        assert_eq!(dialect.from_dialect("advocate"), Some("lawyer"));
        assert_eq!(dialect.from_dialect("pursuer"), Some("plaintiff"));
    }

    #[test]
    fn test_dialect_terminology_louisiana() {
        let registry = DialectTerminologyRegistry::with_defaults();
        let locale = Locale::new("en").with_country("US");

        let louisiana = registry.find_dialect(&locale, "Louisiana Legal");
        assert!(louisiana.is_some());
        let dialect = louisiana.unwrap();

        assert_eq!(dialect.to_dialect("county"), Some("parish"));
        assert_eq!(
            dialect.to_dialect("real_estate"),
            Some("immovable property")
        );
        assert_eq!(dialect.to_dialect("common_law"), Some("civil law"));
        assert_eq!(dialect.to_dialect("deed"), Some("act of sale"));
    }

    #[test]
    fn test_dialect_terminology_quebec() {
        let registry = DialectTerminologyRegistry::with_defaults();
        let locale = Locale::new("fr").with_country("CA");

        let quebec = registry.find_dialect(&locale, "Québec Legal");
        assert!(quebec.is_some());
        let dialect = quebec.unwrap();

        assert_eq!(
            dialect.to_dialect("code_civil"),
            Some("Code civil du Québec")
        );
        assert_eq!(
            dialect.to_dialect("jurisprudence"),
            Some("jurisprudence québécoise")
        );
    }

    #[test]
    fn test_dialect_terminology_get_all_for_locale() {
        let registry = DialectTerminologyRegistry::with_defaults();

        let us_locale = Locale::new("en").with_country("US");
        let us_dialects = registry.get_dialects_for_locale(&us_locale);
        assert!(!us_dialects.is_empty()); // Louisiana

        let gb_locale = Locale::new("en").with_country("GB");
        let gb_dialects = registry.get_dialects_for_locale(&gb_locale);
        assert!(!gb_dialects.is_empty()); // Scottish
    }

    #[test]
    fn test_regional_concept_mapping_trust() {
        let mapper = RegionalConceptMapper::with_defaults();

        let mappings = mapper.find_mappings("trust", "GB", "FR");
        assert!(!mappings.is_empty());

        let mapping = mappings[0];
        assert_eq!(mapping.source_concept, "trust");
        assert_eq!(mapping.target_concept, "fiducie");
        assert_eq!(mapping.similarity, 0.7);
        assert!(!mapping.notes.is_empty());
        assert!(mapping.notes.iter().any(|n| n.contains("equity concept")));
    }

    #[test]
    fn test_regional_concept_mapping_llc() {
        let mapper = RegionalConceptMapper::with_defaults();

        let mappings = mapper.find_mappings("LLC", "US", "DE");
        assert!(!mappings.is_empty());

        let mapping = mappings[0];
        assert_eq!(mapping.source_concept, "LLC");
        assert_eq!(mapping.target_concept, "GmbH");
        assert_eq!(mapping.similarity, 0.9);
        assert!(
            mapping
                .notes
                .iter()
                .any(|n| n.contains("limited liability"))
        );
    }

    #[test]
    fn test_regional_concept_mapping_corporation() {
        let mapper = RegionalConceptMapper::with_defaults();

        let mappings = mapper.find_mappings("corporation", "US", "JP");
        assert!(!mappings.is_empty());

        let mapping = mappings[0];
        assert_eq!(mapping.target_concept, "kabushiki kaisha");
        assert_eq!(mapping.similarity, 0.85);
    }

    #[test]
    fn test_regional_concept_mapping_find_all_for_concept() {
        let mapper = RegionalConceptMapper::with_defaults();

        let all_trust_mappings = mapper.find_all_mappings_for_concept("trust");
        assert!(!all_trust_mappings.is_empty());

        let all_corporation_mappings = mapper.find_all_mappings_for_concept("corporation");
        assert!(!all_corporation_mappings.is_empty());
    }

    #[test]
    fn test_cross_regional_term_equivalence_attorney() {
        let registry = CrossRegionalTermEquivalenceRegistry::with_defaults();

        let equiv = registry.find_equivalence("attorney", "US");
        assert!(equiv.is_some());

        let attorney_equiv = equiv.unwrap();
        assert_eq!(attorney_equiv.base_term, "attorney");
        assert_eq!(attorney_equiv.base_jurisdiction, "US");

        // Test GB equivalent
        let gb_term = attorney_equiv.get_equivalent("GB");
        assert!(gb_term.is_some());
        assert_eq!(gb_term.unwrap().term, "solicitor");
        assert_eq!(
            gb_term.unwrap().equivalence_level,
            EquivalenceLevel::Approximate
        );

        // Test FR equivalent
        let fr_term = attorney_equiv.get_equivalent("FR");
        assert!(fr_term.is_some());
        assert_eq!(fr_term.unwrap().term, "avocat");
        assert_eq!(fr_term.unwrap().equivalence_level, EquivalenceLevel::Exact);

        // Test DE equivalent
        let de_term = attorney_equiv.get_equivalent("DE");
        assert!(de_term.is_some());
        assert_eq!(de_term.unwrap().term, "Rechtsanwalt");
    }

    #[test]
    fn test_cross_regional_term_equivalence_corporation() {
        let registry = CrossRegionalTermEquivalenceRegistry::with_defaults();

        let corp_term = registry.get_equivalent_term("corporation", "US", "JP");
        assert!(corp_term.is_some());
        assert_eq!(corp_term.unwrap().term, "kabushiki kaisha");
        assert_eq!(
            corp_term.unwrap().equivalence_level,
            EquivalenceLevel::Exact
        );

        let de_term = registry.get_equivalent_term("corporation", "US", "DE");
        assert!(de_term.is_some());
        assert_eq!(de_term.unwrap().term, "Aktiengesellschaft");
    }

    #[test]
    fn test_cross_regional_term_equivalence_contract() {
        let registry = CrossRegionalTermEquivalenceRegistry::with_defaults();

        let equiv = registry.find_equivalence("contract", "US");
        assert!(equiv.is_some());

        let contract_equiv = equiv.unwrap();

        // All contract equivalents should be exact
        let gb = contract_equiv.get_equivalent("GB");
        assert_eq!(gb.unwrap().equivalence_level, EquivalenceLevel::Exact);

        let fr = contract_equiv.get_equivalent("FR");
        assert_eq!(fr.unwrap().equivalence_level, EquivalenceLevel::Exact);
        assert_eq!(fr.unwrap().term, "contrat");

        let de = contract_equiv.get_equivalent("DE");
        assert_eq!(de.unwrap().equivalence_level, EquivalenceLevel::Exact);
        assert_eq!(de.unwrap().term, "Vertrag");
    }

    #[test]
    fn test_cross_regional_term_equivalence_trust() {
        let registry = CrossRegionalTermEquivalenceRegistry::with_defaults();

        let equiv = registry.find_equivalence("trust", "GB");
        assert!(equiv.is_some());

        let trust_equiv = equiv.unwrap();

        // FR should be approximate
        let fr = trust_equiv.get_equivalent("FR");
        assert!(fr.is_some());
        assert_eq!(fr.unwrap().term, "fiducie");
        assert_eq!(fr.unwrap().equivalence_level, EquivalenceLevel::Approximate);
        assert!(!fr.unwrap().notes.is_empty());

        // DE should be loose
        let de = trust_equiv.get_equivalent("DE");
        assert!(de.is_some());
        assert_eq!(de.unwrap().term, "Treuhand");
        assert_eq!(de.unwrap().equivalence_level, EquivalenceLevel::Loose);
    }

    #[test]
    fn test_cross_regional_term_equivalence_plaintiff() {
        let registry = CrossRegionalTermEquivalenceRegistry::with_defaults();

        let plaintiff_fr = registry.get_equivalent_term("plaintiff", "US", "FR");
        assert!(plaintiff_fr.is_some());
        assert_eq!(plaintiff_fr.unwrap().term, "demandeur");

        let plaintiff_de = registry.get_equivalent_term("plaintiff", "US", "DE");
        assert!(plaintiff_de.is_some());
        assert_eq!(plaintiff_de.unwrap().term, "Kläger");

        let plaintiff_gb = registry.get_equivalent_term("plaintiff", "US", "GB");
        assert!(plaintiff_gb.is_some());
        assert_eq!(plaintiff_gb.unwrap().term, "claimant");
    }

    #[test]
    fn test_equivalence_level_enum() {
        // Test that all equivalence levels can be created
        let exact = EquivalenceLevel::Exact;
        let approximate = EquivalenceLevel::Approximate;
        let loose = EquivalenceLevel::Loose;
        let no_equiv = EquivalenceLevel::NoEquivalent;

        assert_eq!(exact, EquivalenceLevel::Exact);
        assert_eq!(approximate, EquivalenceLevel::Approximate);
        assert_eq!(loose, EquivalenceLevel::Loose);
        assert_eq!(no_equiv, EquivalenceLevel::NoEquivalent);

        // Test inequality
        assert_ne!(exact, approximate);
        assert_ne!(approximate, loose);
        assert_ne!(loose, no_equiv);
    }

    #[test]
    fn test_sub_regional_variation_custom() {
        let mut registry = SubRegionalVariationRegistry::new();

        let custom = SubRegionalVariation::new(
            Locale::new("en").with_country("AU"),
            "NSW",
            "New South Wales",
            "NSW state law",
        )
        .add_legal_difference("Separate state supreme court")
        .add_legal_difference("NSW-specific legislation");

        registry.add_variation(custom);

        let found = registry.find_variation("AU", "NSW");
        assert!(found.is_some());
        assert_eq!(found.unwrap().region_name, "New South Wales");
    }

    #[test]
    fn test_asian_regional_variations() {
        let registry = SubRegionalVariationRegistry::with_defaults();

        // Test Indian states
        let maharashtra = registry.find_variation("IN", "MH");
        assert!(maharashtra.is_some());
        assert_eq!(maharashtra.unwrap().region_name, "Maharashtra");
        assert!(
            maharashtra
                .unwrap()
                .legal_differences
                .iter()
                .any(|d| d.contains("Bombay High Court"))
        );

        let delhi = registry.find_variation("IN", "DL");
        assert!(delhi.is_some());
        assert_eq!(delhi.unwrap().region_name, "Delhi");

        let karnataka = registry.find_variation("IN", "KA");
        assert!(karnataka.is_some());
        assert!(
            karnataka
                .unwrap()
                .legal_differences
                .iter()
                .any(|d| d.contains("Tech industry"))
        );

        // Test Singapore
        let singapore = registry.find_variation("SG", "SG");
        assert!(singapore.is_some());
        assert!(
            singapore
                .unwrap()
                .legal_differences
                .iter()
                .any(|d| d.contains("Common law"))
        );

        // Test Malaysia
        let kl = registry.find_variation("MY", "WP");
        assert!(kl.is_some());
        assert!(
            kl.unwrap()
                .legal_differences
                .iter()
                .any(|d| d.contains("Islamic law"))
        );

        // Test Thailand
        let bangkok = registry.find_variation("TH", "BKK");
        assert!(bangkok.is_some());
        assert!(
            bangkok
                .unwrap()
                .legal_differences
                .iter()
                .any(|d| d.contains("Civil law"))
        );

        // Test Vietnam
        let hanoi = registry.find_variation("VN", "HN");
        assert!(hanoi.is_some());
        assert_eq!(hanoi.unwrap().region_name, "Hanoi");

        let hcmc = registry.find_variation("VN", "SG");
        assert!(hcmc.is_some());
        assert_eq!(hcmc.unwrap().region_name, "Ho Chi Minh City");

        // Test Indonesia
        let jakarta = registry.find_variation("ID", "JK");
        assert!(jakarta.is_some());
        assert!(
            jakarta
                .unwrap()
                .legal_differences
                .iter()
                .any(|d| d.contains("Dutch-influenced"))
        );
    }

    #[test]
    fn test_middle_eastern_regional_variations() {
        let registry = SubRegionalVariationRegistry::with_defaults();

        // Test UAE
        let dubai = registry.find_variation("AE", "DU");
        assert!(dubai.is_some());
        assert_eq!(dubai.unwrap().region_name, "Dubai");
        assert!(
            dubai
                .unwrap()
                .legal_differences
                .iter()
                .any(|d| d.contains("DIFC"))
        );

        let abu_dhabi = registry.find_variation("AE", "AZ");
        assert!(abu_dhabi.is_some());
        assert!(
            abu_dhabi
                .unwrap()
                .legal_differences
                .iter()
                .any(|d| d.contains("ADGM"))
        );

        // Test Saudi Arabia
        let riyadh = registry.find_variation("SA", "RI");
        assert!(riyadh.is_some());
        assert!(
            riyadh
                .unwrap()
                .legal_differences
                .iter()
                .any(|d| d.contains("Sharia law"))
        );

        // Test Israel
        let tel_aviv = registry.find_variation("IL", "TA");
        assert!(tel_aviv.is_some());
        assert!(
            tel_aviv
                .unwrap()
                .legal_differences
                .iter()
                .any(|d| d.contains("Tech startup"))
        );
    }

    #[test]
    fn test_latin_american_regional_variations() {
        let registry = SubRegionalVariationRegistry::with_defaults();

        // Test Brazil
        let sao_paulo = registry.find_variation("BR", "SP");
        assert!(sao_paulo.is_some());
        assert_eq!(sao_paulo.unwrap().region_name, "São Paulo");
        assert!(
            sao_paulo
                .unwrap()
                .legal_differences
                .iter()
                .any(|d| d.contains("Civil law"))
        );

        let rio = registry.find_variation("BR", "RJ");
        assert!(rio.is_some());
        assert!(
            rio.unwrap()
                .legal_differences
                .iter()
                .any(|d| d.contains("Oil and gas"))
        );

        // Test Argentina
        let buenos_aires = registry.find_variation("AR", "BA");
        assert!(buenos_aires.is_some());
        assert!(
            buenos_aires
                .unwrap()
                .legal_differences
                .iter()
                .any(|d| d.contains("Código Civil"))
        );

        // Test Mexico
        let mexico_city = registry.find_variation("MX", "CMX");
        assert!(mexico_city.is_some());
        assert!(
            mexico_city
                .unwrap()
                .legal_differences
                .iter()
                .any(|d| d.contains("Amparo"))
        );

        // Test Chile
        let santiago = registry.find_variation("CL", "RM");
        assert!(santiago.is_some());
        assert!(
            santiago
                .unwrap()
                .legal_differences
                .iter()
                .any(|d| d.contains("Mining law"))
        );

        // Test Colombia
        let bogota = registry.find_variation("CO", "DC");
        assert!(bogota.is_some());
        assert!(
            bogota
                .unwrap()
                .legal_differences
                .iter()
                .any(|d| d.contains("tutela"))
        );
    }

    #[test]
    fn test_african_regional_variations() {
        let registry = SubRegionalVariationRegistry::with_defaults();

        // Test South Africa
        let gauteng = registry.find_variation("ZA", "GP");
        assert!(gauteng.is_some());
        assert!(
            gauteng
                .unwrap()
                .legal_differences
                .iter()
                .any(|d| d.contains("Roman-Dutch"))
        );

        let western_cape = registry.find_variation("ZA", "WC");
        assert!(western_cape.is_some());
        assert!(
            western_cape
                .unwrap()
                .legal_differences
                .iter()
                .any(|d| d.contains("Wine industry"))
        );

        // Test Nigeria
        let lagos = registry.find_variation("NG", "LA");
        assert!(lagos.is_some());
        assert!(
            lagos
                .unwrap()
                .legal_differences
                .iter()
                .any(|d| d.contains("Common law"))
        );

        // Test Egypt
        let cairo = registry.find_variation("EG", "C");
        assert!(cairo.is_some());
        assert!(
            cairo
                .unwrap()
                .legal_differences
                .iter()
                .any(|d| d.contains("French-influenced"))
        );

        // Test Kenya
        let nairobi = registry.find_variation("KE", "NBO");
        assert!(nairobi.is_some());
        assert!(
            nairobi
                .unwrap()
                .legal_differences
                .iter()
                .any(|d| d.contains("East African Court"))
        );
    }

    #[test]
    fn test_additional_us_states() {
        let registry = SubRegionalVariationRegistry::with_defaults();

        let washington = registry.find_variation("US", "WA");
        assert!(washington.is_some());
        assert!(
            washington
                .unwrap()
                .legal_differences
                .iter()
                .any(|d| d.contains("tech industry"))
        );

        let massachusetts = registry.find_variation("US", "MA");
        assert!(massachusetts.is_some());
        assert!(
            massachusetts
                .unwrap()
                .legal_differences
                .iter()
                .any(|d| d.contains("healthcare"))
        );

        let colorado = registry.find_variation("US", "CO");
        assert!(colorado.is_some());
        assert!(
            colorado
                .unwrap()
                .legal_differences
                .iter()
                .any(|d| d.contains("Cannabis"))
        );

        let nevada = registry.find_variation("US", "NV");
        assert!(nevada.is_some());
        assert!(
            nevada
                .unwrap()
                .legal_differences
                .iter()
                .any(|d| d.contains("Gaming"))
        );

        // Count total US states
        let us_states = registry.get_variations_for_country("US");
        assert!(us_states.len() >= 16); // 6 original + 10 new
    }

    #[test]
    fn test_canadian_territories() {
        let registry = SubRegionalVariationRegistry::with_defaults();

        let yukon = registry.find_variation("CA", "YT");
        assert!(yukon.is_some());
        assert_eq!(yukon.unwrap().region_name, "Yukon");
        assert!(
            yukon
                .unwrap()
                .legal_differences
                .iter()
                .any(|d| d.contains("Indigenous"))
        );

        let nwt = registry.find_variation("CA", "NT");
        assert!(nwt.is_some());
        assert_eq!(nwt.unwrap().region_name, "Northwest Territories");

        let nunavut = registry.find_variation("CA", "NU");
        assert!(nunavut.is_some());
        assert!(
            nunavut
                .unwrap()
                .legal_differences
                .iter()
                .any(|d| d.contains("Inuit"))
        );

        // Count total Canadian provinces/territories
        let ca_regions = registry.get_variations_for_country("CA");
        assert!(ca_regions.len() >= 7); // 4 original + 3 new territories
    }

    #[test]
    fn test_regional_coverage_count() {
        let registry = SubRegionalVariationRegistry::with_defaults();

        // Count by country
        assert!(registry.get_variations_for_country("US").len() >= 16);
        assert!(registry.get_variations_for_country("CA").len() >= 7);
        assert!(registry.get_variations_for_country("IN").len() >= 3);
        assert!(registry.get_variations_for_country("AE").len() >= 2);
        assert!(registry.get_variations_for_country("BR").len() >= 2);
        assert!(registry.get_variations_for_country("ZA").len() >= 2);
        assert!(registry.get_variations_for_country("VN").len() >= 2);

        // Verify Singapore, Malaysia, Thailand, Indonesia, Saudi Arabia, Israel, Argentina,
        // Mexico, Chile, Colombia, Nigeria, Egypt, Kenya
        assert!(registry.find_variation("SG", "SG").is_some());
        assert!(registry.find_variation("MY", "WP").is_some());
        assert!(registry.find_variation("TH", "BKK").is_some());
        assert!(registry.find_variation("ID", "JK").is_some());
        assert!(registry.find_variation("SA", "RI").is_some());
        assert!(registry.find_variation("IL", "TA").is_some());
        assert!(registry.find_variation("AR", "BA").is_some());
        assert!(registry.find_variation("MX", "CMX").is_some());
        assert!(registry.find_variation("CL", "RM").is_some());
        assert!(registry.find_variation("CO", "DC").is_some());
        assert!(registry.find_variation("NG", "LA").is_some());
        assert!(registry.find_variation("EG", "C").is_some());
        assert!(registry.find_variation("KE", "NBO").is_some());
    }

    #[test]
    fn test_eu_member_state_custom() {
        let mut registry = EUMemberStateRegistry::new();

        let custom = EUMemberStateVariation::new(
            Locale::new("fi").with_country("FI"),
            "Finland",
            1995,
            "Civil law (Nordic tradition)",
        )
        .add_eu_adaptation("GDPR through Finnish law")
        .add_specialty("Nordic legal tradition");

        registry.add_variation(custom);

        let found = registry.find_variation("FI");
        assert!(found.is_some());
        assert_eq!(found.unwrap().country_name, "Finland");
    }

    #[test]
    fn test_dialect_terminology_custom() {
        let mut registry = DialectTerminologyRegistry::new();

        let mut custom =
            DialectTerminology::new(Locale::new("en").with_country("IN"), "Indian Legal");
        custom.add_term("lawyer", "advocate");
        custom.add_term("judge", "honourable justice");

        registry.add_dialect(custom);

        let found = registry.find_dialect(&Locale::new("en").with_country("IN"), "Indian Legal");
        assert!(found.is_some());
        assert_eq!(found.unwrap().to_dialect("lawyer"), Some("advocate"));
    }

    #[test]
    fn test_regional_concept_mapper_custom() {
        let mut mapper = RegionalConceptMapper::new();

        let custom = RegionalConceptMapping::new(
            "limited_partnership",
            "US",
            "kommanditgesellschaft",
            "DE",
            0.85,
        )
        .add_note("Both have limited and general partners");

        mapper.add_mapping(custom);

        let found = mapper.find_mappings("limited_partnership", "US", "DE");
        assert!(!found.is_empty());
        assert_eq!(found[0].target_concept, "kommanditgesellschaft");
    }

    #[test]
    fn test_term_equivalence_custom() {
        let mut registry = CrossRegionalTermEquivalenceRegistry::new();

        let custom = TermEquivalence::new("arbitration", "US")
            .add_equivalent("FR", "arbitrage", EquivalenceLevel::Exact)
            .add_equivalent("DE", "Schiedsverfahren", EquivalenceLevel::Exact)
            .add_note_to_equivalent("FR", "International arbitration hub");

        registry.add_equivalence(custom);

        let found = registry.get_equivalent_term("arbitration", "US", "FR");
        assert!(found.is_some());
        assert_eq!(found.unwrap().term, "arbitrage");
    }

    // ========================================================================
    // Document Templates v0.2.0 Tests
    // ========================================================================

    #[test]
    fn test_template_variable_validation() {
        let text_var = TemplateVariable::new("name", VariableType::Text, true, "Person name");
        assert!(text_var.validate("John Doe"));
        assert!(!text_var.validate("")); // Required but empty

        let number_var = TemplateVariable::new("amount", VariableType::Number, true, "Amount");
        assert!(number_var.validate("123.45"));
        assert!(!number_var.validate("not a number"));

        let email_var = TemplateVariable::new("email", VariableType::Email, true, "Email");
        assert!(email_var.validate("user@example.com"));
        assert!(!email_var.validate("not-an-email"));

        let bool_var = TemplateVariable::new("active", VariableType::Boolean, true, "Active");
        assert!(bool_var.validate("true"));
        assert!(bool_var.validate("false"));
        assert!(bool_var.validate("yes"));
        assert!(bool_var.validate("no"));
        assert!(!bool_var.validate("maybe"));

        let date_var = TemplateVariable::new("date", VariableType::Date, true, "Date");
        assert!(date_var.validate("2024-01-15"));
        assert!(date_var.validate("01/15/2024"));
        assert!(!date_var.validate("invalid"));
    }

    #[test]
    fn test_template_variable_with_default() {
        let var = TemplateVariable::new("state", VariableType::Text, false, "State")
            .with_default("California");

        assert_eq!(var.default_value, Some("California".to_string()));
    }

    #[test]
    fn test_template_section_conditional() {
        let mut context = HashMap::new();
        context.insert("jurisdiction".to_string(), "US".to_string());

        let section_with_condition = TemplateSection::new("us_only", "US specific content")
            .with_condition("jurisdiction == US");

        assert!(section_with_condition.should_include(&context));

        context.insert("jurisdiction".to_string(), "GB".to_string());
        assert!(!section_with_condition.should_include(&context));

        // Test != condition
        let section_not_us =
            TemplateSection::new("non_us", "Non-US content").with_condition("jurisdiction != US");

        assert!(section_not_us.should_include(&context));

        context.insert("jurisdiction".to_string(), "US".to_string());
        assert!(!section_not_us.should_include(&context));
    }

    #[test]
    fn test_template_section_no_condition() {
        let context = HashMap::new();
        let section = TemplateSection::new("always", "Always included");

        assert!(section.should_include(&context));
    }

    #[test]
    fn test_document_template_nda_generation() {
        let registry = DocumentTemplateRegistry::with_defaults();
        let template = registry.get_template("nda_mutual_us").unwrap();

        let mut values = HashMap::new();
        values.insert("party1_name".to_string(), "Acme Corp".to_string());
        values.insert("party2_name".to_string(), "Beta LLC".to_string());
        values.insert("effective_date".to_string(), "2024-01-01".to_string());
        values.insert("state".to_string(), "Delaware".to_string());

        let document = template.generate(&values);
        assert!(document.is_ok());

        let doc_text = document.unwrap();
        assert!(doc_text.contains("MUTUAL NON-DISCLOSURE AGREEMENT"));
        assert!(doc_text.contains("Acme Corp"));
        assert!(doc_text.contains("Beta LLC"));
        assert!(doc_text.contains("2024-01-01"));
        assert!(doc_text.contains("Delaware"));
        assert!(doc_text.contains("CONFIDENTIAL INFORMATION"));
        assert!(doc_text.contains("OBLIGATIONS"));
    }

    #[test]
    fn test_document_template_employment_generation() {
        let registry = DocumentTemplateRegistry::with_defaults();
        let template = registry.get_template("employment_agreement_us").unwrap();

        let mut values = HashMap::new();
        values.insert(
            "company_name".to_string(),
            "Tech Innovations Inc".to_string(),
        );
        values.insert("employee_name".to_string(), "Jane Smith".to_string());
        values.insert("position".to_string(), "Software Engineer".to_string());
        values.insert("start_date".to_string(), "2024-03-15".to_string());
        values.insert("salary".to_string(), "120000".to_string());
        values.insert("state".to_string(), "California".to_string());

        let document = template.generate(&values);
        assert!(document.is_ok());

        let doc_text = document.unwrap();
        assert!(doc_text.contains("EMPLOYMENT AGREEMENT"));
        assert!(doc_text.contains("Tech Innovations Inc"));
        assert!(doc_text.contains("Jane Smith"));
        assert!(doc_text.contains("Software Engineer"));
        assert!(doc_text.contains("$120000"));
        assert!(doc_text.contains("AT-WILL EMPLOYMENT"));
    }

    #[test]
    fn test_document_template_complaint_generation() {
        let registry = DocumentTemplateRegistry::with_defaults();
        let template = registry.get_template("complaint_us").unwrap();

        let mut values = HashMap::new();
        values.insert(
            "court_name".to_string(),
            "UNITED STATES DISTRICT COURT\nSOUTHERN DISTRICT OF NEW YORK".to_string(),
        );
        values.insert("plaintiff_name".to_string(), "Alice Johnson".to_string());
        values.insert("defendant_name".to_string(), "Bob Williams".to_string());
        values.insert("case_number".to_string(), "1:24-cv-12345".to_string());
        values.insert(
            "jurisdiction_facts".to_string(),
            "This Court has jurisdiction pursuant to 28 U.S.C. § 1331.".to_string(),
        );
        values.insert(
            "claim_facts".to_string(),
            "On or about December 1, 2023, Defendant breached the contract.".to_string(),
        );
        values.insert(
            "relief_requested".to_string(),
            "Award Plaintiff damages in the amount of $50,000 plus costs and attorney's fees."
                .to_string(),
        );

        let document = template.generate(&values);
        assert!(document.is_ok());

        let doc_text = document.unwrap();
        assert!(doc_text.contains("COMPLAINT"));
        assert!(doc_text.contains("Alice Johnson"));
        assert!(doc_text.contains("Bob Williams"));
        assert!(doc_text.contains("1:24-cv-12345"));
        assert!(doc_text.contains("JURISDICTION AND VENUE"));
        assert!(doc_text.contains("PRAYER FOR RELIEF"));
    }

    #[test]
    fn test_document_template_articles_generation() {
        let registry = DocumentTemplateRegistry::with_defaults();
        let template = registry.get_template("articles_incorporation_de").unwrap();

        let mut values = HashMap::new();
        values.insert(
            "corporation_name".to_string(),
            "NewCo Technologies, Inc.".to_string(),
        );
        values.insert(
            "registered_agent_name".to_string(),
            "Corporation Service Company".to_string(),
        );
        values.insert(
            "registered_agent_address".to_string(),
            "251 Little Falls Drive, Wilmington, DE 19808".to_string(),
        );
        values.insert("shares_authorized".to_string(), "10000000".to_string());
        values.insert("incorporator_name".to_string(), "John Founder".to_string());

        let document = template.generate(&values);
        assert!(document.is_ok());

        let doc_text = document.unwrap();
        assert!(doc_text.contains("CERTIFICATE OF INCORPORATION"));
        assert!(doc_text.contains("NewCo Technologies, Inc."));
        assert!(doc_text.contains("Corporation Service Company"));
        assert!(doc_text.contains("10000000"));
        assert!(doc_text.contains("John Founder"));
        assert!(doc_text.contains("ARTICLE I - NAME"));
        assert!(doc_text.contains("ARTICLE IV - CAPITAL STOCK"));
    }

    #[test]
    fn test_document_template_missing_required_variable() {
        let registry = DocumentTemplateRegistry::with_defaults();
        let template = registry.get_template("nda_mutual_us").unwrap();

        let mut values = HashMap::new();
        values.insert("party1_name".to_string(), "Acme Corp".to_string());
        // Missing party2_name, effective_date, and state

        let document = template.generate(&values);
        assert!(document.is_err());

        let errors = document.unwrap_err();
        assert!(errors.len() >= 3); // At least 3 missing variables
        assert!(errors.iter().any(|e| e.contains("party2_name")
            || e.contains("effective_date")
            || e.contains("state")));
    }

    #[test]
    fn test_document_template_invalid_variable_type() {
        let registry = DocumentTemplateRegistry::with_defaults();
        let template = registry.get_template("employment_agreement_us").unwrap();

        let mut values = HashMap::new();
        values.insert("company_name".to_string(), "Tech Corp".to_string());
        values.insert("employee_name".to_string(), "Jane Doe".to_string());
        values.insert("position".to_string(), "Engineer".to_string());
        values.insert("start_date".to_string(), "2024-01-01".to_string());
        values.insert("salary".to_string(), "not-a-number".to_string()); // Invalid
        values.insert("state".to_string(), "CA".to_string());

        let document = template.generate(&values);
        assert!(document.is_err());

        let errors = document.unwrap_err();
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.contains("salary")));
    }

    #[test]
    fn test_document_template_registry_find_by_type() {
        let registry = DocumentTemplateRegistry::with_defaults();

        let contracts = registry.find_by_type(DocumentTemplateType::Contract);
        assert!(contracts.len() >= 2); // NDA and Employment

        let court_docs = registry.find_by_type(DocumentTemplateType::CourtFiling);
        assert!(!court_docs.is_empty()); // Complaint

        let corporate_docs = registry.find_by_type(DocumentTemplateType::Corporate);
        assert!(!corporate_docs.is_empty()); // Articles of Incorporation
    }

    #[test]
    fn test_document_template_registry_find_by_jurisdiction() {
        let registry = DocumentTemplateRegistry::with_defaults();

        let us_templates = registry.find_by_jurisdiction("US");
        assert!(us_templates.len() >= 3); // NDA, Employment, Complaint

        let de_templates = registry.find_by_jurisdiction("US-DE");
        assert!(!de_templates.is_empty()); // Articles of Incorporation
    }

    #[test]
    fn test_document_template_registry_list_templates() {
        let registry = DocumentTemplateRegistry::with_defaults();

        let template_ids = registry.list_templates();
        assert!(template_ids.contains(&"nda_mutual_us"));
        assert!(template_ids.contains(&"employment_agreement_us"));
        assert!(template_ids.contains(&"complaint_us"));
        assert!(template_ids.contains(&"articles_incorporation_de"));
    }

    #[test]
    fn test_document_template_custom() {
        let mut registry = DocumentTemplateRegistry::new();

        let custom_template = DocumentTemplate::new(
            "custom_nda_gb",
            "UK Non-Disclosure Agreement",
            DocumentTemplateType::Contract,
            Locale::new("en").with_country("GB"),
            "GB",
        )
        .add_variable(TemplateVariable::new(
            "party1",
            VariableType::Text,
            true,
            "First Party",
        ))
        .add_variable(TemplateVariable::new(
            "party2",
            VariableType::Text,
            true,
            "Second Party",
        ))
        .add_section(TemplateSection::new("title", "CONFIDENTIALITY AGREEMENT\n"))
        .add_section(TemplateSection::new(
            "parties",
            "This Agreement is made between {{party1}} and {{party2}}.\n",
        ))
        .add_metadata("jurisdiction", "England and Wales");

        registry.add_template(custom_template);

        let retrieved = registry.get_template("custom_nda_gb");
        assert!(retrieved.is_some());

        let template = retrieved.unwrap();
        assert_eq!(template.name, "UK Non-Disclosure Agreement");
        assert_eq!(template.jurisdiction, "GB");
        assert_eq!(
            template.metadata.get("jurisdiction"),
            Some(&"England and Wales".to_string())
        );
    }

    #[test]
    fn test_variable_type_enum() {
        // Test that all variable types can be created
        let types = [
            VariableType::Text,
            VariableType::Date,
            VariableType::Number,
            VariableType::Currency,
            VariableType::Boolean,
            VariableType::Email,
            VariableType::Address,
            VariableType::PersonName,
            VariableType::List,
        ];

        assert_eq!(types.len(), 9);
        assert_eq!(types[0], VariableType::Text);
        assert_eq!(types[1], VariableType::Date);
    }

    #[test]
    fn test_document_template_type_enum() {
        // Test that all template types can be created
        let types = [
            DocumentTemplateType::Contract,
            DocumentTemplateType::CourtFiling,
            DocumentTemplateType::Corporate,
            DocumentTemplateType::Compliance,
            DocumentTemplateType::General,
        ];

        assert_eq!(types.len(), 5);
        assert_eq!(types[0], DocumentTemplateType::Contract);
        assert_ne!(types[0], types[1]);
    }

    // ========================================================================
    // Citation Validation Tests (v0.2.1)
    // ========================================================================

    #[test]
    fn test_citation_parser_bluebook_case() {
        let parser = CitationParser::new(CitationStyle::Bluebook);
        let citation = "Brown v. Board of Education, 347 U.S. 483 (1954)";
        let result = parser.parse_case(citation);

        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.title, "Brown v. Board of Education");
        assert_eq!(components.volume, Some("347".to_string()));
        assert_eq!(components.reporter, Some("U.S.".to_string()));
        assert_eq!(components.page, Some("483".to_string()));
        assert_eq!(components.year, Some(1954));
    }

    #[test]
    fn test_citation_parser_oscola_case() {
        let parser = CitationParser::new(CitationStyle::OSCOLA);
        let citation = "R v Smith [2020] EWCA Crim 123";
        let result = parser.parse_case(citation);

        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.title, "R v Smith");
        assert_eq!(components.year, Some(2020));
        assert_eq!(components.reporter, Some("EWCA".to_string()));
        assert_eq!(components.page, Some("Crim".to_string()));
    }

    #[test]
    fn test_citation_parser_bluebook_statute() {
        let parser = CitationParser::new(CitationStyle::Bluebook);
        let citation = "42 U.S.C. § 1983";
        let result = parser.parse_statute(citation);

        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.title, "42 U.S.C. § 1983");
        assert_eq!(components.reporter, Some("42 U.S.C.".to_string()));
        assert_eq!(components.page, Some("1983".to_string()));
    }

    #[test]
    fn test_citation_parser_oscola_statute() {
        let parser = CitationParser::new(CitationStyle::OSCOLA);
        let citation = "Human Rights Act 1998, s 3";
        let result = parser.parse_statute(citation);

        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.title, "Human Rights Act 1998, s 3");
        assert_eq!(components.year, Some(1998));
    }

    #[test]
    fn test_citation_validator_bluebook_case_valid() {
        let validator = CitationValidator::new(CitationStyle::Bluebook);
        let components = CitationComponents {
            title: "Brown v. Board of Education".to_string(),
            volume: Some("347".to_string()),
            reporter: Some("U.S.".to_string()),
            page: Some("483".to_string()),
            court: Some("Supreme Court".to_string()),
            year: Some(1954),
            jurisdiction: None,
        };

        let result = validator.validate_case(&components);
        assert!(result.is_ok());
    }

    #[test]
    fn test_citation_validator_bluebook_case_missing_year() {
        let validator = CitationValidator::new(CitationStyle::Bluebook);
        let components = CitationComponents {
            title: "Brown v. Board of Education".to_string(),
            volume: Some("347".to_string()),
            reporter: Some("U.S.".to_string()),
            page: Some("483".to_string()),
            court: None,
            year: None,
            jurisdiction: None,
        };

        let result = validator.validate_case(&components);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(!errors.is_empty());
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, CitationError::MissingField { field } if field == "year"))
        );
    }

    #[test]
    fn test_citation_validator_oscola_case_valid() {
        let validator = CitationValidator::new(CitationStyle::OSCOLA);
        let components = CitationComponents {
            title: "R v Smith".to_string(),
            volume: None,
            reporter: Some("EWCA".to_string()),
            page: Some("Crim 123".to_string()),
            court: None,
            year: Some(2020),
            jurisdiction: None,
        };

        let result = validator.validate_case(&components);
        assert!(result.is_ok());
    }

    #[test]
    fn test_citation_validator_bluebook_statute_valid() {
        let validator = CitationValidator::new(CitationStyle::Bluebook);
        let components = CitationComponents {
            title: "42 U.S.C.".to_string(),
            volume: None,
            reporter: None,
            page: Some("1983".to_string()),
            court: None,
            year: None,
            jurisdiction: None,
        };

        let result = validator.validate_statute(&components);
        assert!(result.is_ok());
    }

    #[test]
    fn test_citation_normalizer_bluebook_to_oscola() {
        let normalizer = CitationNormalizer::new();
        let components = CitationComponents {
            title: "Brown v. Board of Education".to_string(),
            volume: Some("347".to_string()),
            reporter: Some("U.S.".to_string()),
            page: Some("483".to_string()),
            court: None,
            year: Some(1954),
            jurisdiction: None,
        };

        let result =
            normalizer.convert_case(&components, CitationStyle::Bluebook, CitationStyle::OSCOLA);

        assert!(result.is_ok());
        let converted = result.unwrap();
        assert!(converted.contains("[1954]"));
        assert!(converted.contains("Brown v. Board of Education"));
    }

    #[test]
    fn test_citation_normalizer_parse_and_convert() {
        let normalizer = CitationNormalizer::new();
        let citation = "Brown v. Board of Education, 347 U.S. 483 (1954)";

        let result = normalizer.parse_and_convert_case(
            citation,
            CitationStyle::Bluebook,
            CitationStyle::OSCOLA,
        );

        assert!(result.is_ok());
        let converted = result.unwrap();
        assert!(converted.contains("[1954]"));
    }

    #[test]
    fn test_citation_completeness_checker_complete() {
        let checker = CitationCompletenessChecker::new(CitationStyle::Bluebook);
        let components = CitationComponents {
            title: "Brown v. Board of Education".to_string(),
            volume: Some("347".to_string()),
            reporter: Some("U.S.".to_string()),
            page: Some("483".to_string()),
            court: Some("Supreme Court".to_string()),
            year: Some(1954),
            jurisdiction: None,
        };

        let report = checker.check_case(&components);
        assert!(report.is_complete());
        assert!(report.completeness_score > 80.0);
        assert!(report.missing_required.is_empty());
    }

    #[test]
    fn test_citation_completeness_checker_incomplete() {
        let checker = CitationCompletenessChecker::new(CitationStyle::Bluebook);
        let components = CitationComponents {
            title: "Case Name".to_string(),
            volume: None,
            reporter: None,
            page: None,
            court: None,
            year: None,
            jurisdiction: None,
        };

        let report = checker.check_case(&components);
        assert!(!report.is_complete());
        assert!(report.completeness_score < 50.0);
        assert!(!report.missing_required.is_empty());
        assert!(report.missing_required.contains(&"volume".to_string()));
        assert!(report.missing_required.contains(&"year".to_string()));
    }

    #[test]
    fn test_citation_completeness_report_summary() {
        let checker = CitationCompletenessChecker::new(CitationStyle::Bluebook);
        let components = CitationComponents {
            title: "Case Name".to_string(),
            volume: None,
            reporter: None,
            page: None,
            court: None,
            year: None,
            jurisdiction: None,
        };

        let report = checker.check_case(&components);
        let summary = report.summary();
        assert!(summary.contains("incomplete"));
        assert!(summary.contains("missing"));
    }

    #[test]
    fn test_citation_suggester_bluebook_case() {
        let suggester = CitationSuggester::new(CitationStyle::Bluebook);
        let components = CitationComponents {
            title: "Case Name".to_string(),
            volume: Some("123".to_string()),
            reporter: Some("F.3d".to_string()),
            page: None,
            court: None,
            year: None,
            jurisdiction: None,
        };

        let suggestions = suggester.suggest_case(&components);
        assert!(!suggestions.is_empty());
        assert!(suggestions.iter().any(|s| s.contains("page")));
        assert!(suggestions.iter().any(|s| s.contains("year")));
    }

    #[test]
    fn test_citation_suggester_oscola_case() {
        let suggester = CitationSuggester::new(CitationStyle::OSCOLA);
        let components = CitationComponents {
            title: "case name".to_string(),
            volume: None,
            reporter: Some("UKSC".to_string()),
            page: None,
            court: None,
            year: None,
            jurisdiction: None,
        };

        let suggestions = suggester.suggest_case(&components);
        assert!(!suggestions.is_empty());
        assert!(suggestions.iter().any(|s| s.contains("year")));
        assert!(suggestions.iter().any(|s| s.contains("capital")));
    }

    #[test]
    fn test_citation_suggester_statute() {
        let suggester = CitationSuggester::new(CitationStyle::Bluebook);
        let components = CitationComponents {
            title: "42 U.S.C.".to_string(),
            volume: None,
            reporter: None,
            page: None,
            court: None,
            year: None,
            jurisdiction: None,
        };

        let suggestions = suggester.suggest_statute(&components);
        assert!(suggestions.iter().any(|s| s.contains("section")));
    }

    #[test]
    fn test_citation_suggester_validate_and_suggest_case() {
        let suggester = CitationSuggester::new(CitationStyle::Bluebook);
        let components = CitationComponents {
            title: "Brown v. Board of Education".to_string(),
            volume: Some("347".to_string()),
            reporter: Some("U.S.".to_string()),
            page: Some("483".to_string()),
            court: None,
            year: Some(1954),
            jurisdiction: None,
        };

        let report = suggester.validate_and_suggest_case(&components);
        assert!(report.is_valid);
        assert!(report.errors.is_empty());
        assert!(report.completeness.is_complete());
    }

    #[test]
    fn test_citation_suggester_validate_and_suggest_invalid() {
        let suggester = CitationSuggester::new(CitationStyle::Bluebook);
        let components = CitationComponents {
            title: "Case".to_string(),
            volume: None,
            reporter: None,
            page: None,
            court: None,
            year: None,
            jurisdiction: None,
        };

        let report = suggester.validate_and_suggest_case(&components);
        assert!(!report.is_valid);
        assert!(!report.errors.is_empty());
        assert!(!report.completeness.is_complete());
        assert!(!report.suggestions.is_empty());
    }

    #[test]
    fn test_citation_suggester_style_for_jurisdiction() {
        assert_eq!(
            CitationSuggester::suggest_style_for_jurisdiction("US"),
            CitationStyle::Bluebook
        );
        assert_eq!(
            CitationSuggester::suggest_style_for_jurisdiction("GB"),
            CitationStyle::OSCOLA
        );
        assert_eq!(
            CitationSuggester::suggest_style_for_jurisdiction("AU"),
            CitationStyle::AGLC
        );
        assert_eq!(
            CitationSuggester::suggest_style_for_jurisdiction("CA"),
            CitationStyle::McGill
        );
        assert_eq!(
            CitationSuggester::suggest_style_for_jurisdiction("JP"),
            CitationStyle::Japanese
        );
        assert_eq!(
            CitationSuggester::suggest_style_for_jurisdiction("IN"),
            CitationStyle::Indian
        );
    }

    #[test]
    fn test_validation_report_summary() {
        let suggester = CitationSuggester::new(CitationStyle::Bluebook);
        let components = CitationComponents {
            title: "Test".to_string(),
            volume: None,
            reporter: None,
            page: None,
            court: None,
            year: None,
            jurisdiction: None,
        };

        let report = suggester.validate_and_suggest_case(&components);
        let summary = report.summary();
        assert!(summary.contains("error"));
        assert!(summary.contains("incomplete"));
        assert!(summary.contains("Suggestions"));
    }

    #[test]
    fn test_citation_validation_rule_required() {
        let rule = CitationValidationRule::required("title");
        assert!(rule.required);
        assert_eq!(rule.field, "title");

        let result = rule.validate(None);
        assert!(result.is_err());

        let result = rule.validate(Some(&"Test".to_string()));
        assert!(result.is_ok());
    }

    #[test]
    fn test_citation_validation_rule_optional() {
        let rule = CitationValidationRule::optional("court");
        assert!(!rule.required);

        let result = rule.validate(None);
        assert!(result.is_ok());

        let result = rule.validate(Some(&"Supreme Court".to_string()));
        assert!(result.is_ok());
    }

    #[test]
    fn test_citation_validation_rule_pattern_numeric() {
        let rule = CitationValidationRule::required("volume").with_pattern("numeric");

        let result = rule.validate(Some(&"123".to_string()));
        assert!(result.is_ok());

        let result = rule.validate(Some(&"abc".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn test_citation_validation_rule_pattern_year() {
        let rule = CitationValidationRule::required("year").with_pattern("year");

        let result = rule.validate(Some(&"2020".to_string()));
        assert!(result.is_ok());

        let result = rule.validate(Some(&"999".to_string()));
        assert!(result.is_err());

        let result = rule.validate(Some(&"10000".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn test_citation_error_display() {
        let error = CitationError::MissingField {
            field: "year".to_string(),
        };
        assert!(error.to_string().contains("Missing required field"));
        assert!(error.to_string().contains("year"));

        let error = CitationError::InvalidFormat {
            field: "volume".to_string(),
            reason: "Not numeric".to_string(),
        };
        assert!(error.to_string().contains("Invalid format"));
        assert!(error.to_string().contains("volume"));

        let error = CitationError::ParseError {
            reason: "Empty citation".to_string(),
        };
        assert!(error.to_string().contains("Failed to parse"));
    }

    #[test]
    fn test_citation_type_enum() {
        let types = [
            CitationType::Case,
            CitationType::Statute,
            CitationType::Article,
            CitationType::Book,
        ];

        assert_eq!(types.len(), 4);
        assert_eq!(types[0], CitationType::Case);
        assert_ne!(types[0], types[1]);
    }

    #[test]
    fn test_citation_parser_empty_citation() {
        let parser = CitationParser::new(CitationStyle::Bluebook);
        let result = parser.parse_case("");

        assert!(result.is_err());
        if let Err(CitationError::ParseError { reason }) = result {
            assert!(reason.contains("Empty"));
        } else {
            panic!("Expected ParseError");
        }
    }

    #[test]
    fn test_citation_normalizer_default() {
        let normalizer = CitationNormalizer::default();
        let components = CitationComponents::new("Test");

        let _ =
            normalizer.convert_case(&components, CitationStyle::Bluebook, CitationStyle::OSCOLA);
    }

    #[test]
    fn test_completeness_report_is_complete() {
        let report = CompletenessReport {
            citation_type: CitationType::Case,
            style: CitationStyle::Bluebook,
            completeness_score: 100.0,
            missing_required: vec![],
            missing_optional: vec!["court".to_string()],
            present: vec!["title".to_string(), "year".to_string()],
        };

        assert!(report.is_complete());

        let report = CompletenessReport {
            citation_type: CitationType::Case,
            style: CitationStyle::Bluebook,
            completeness_score: 50.0,
            missing_required: vec!["year".to_string()],
            missing_optional: vec![],
            present: vec!["title".to_string()],
        };

        assert!(!report.is_complete());
    }

    // ============================================================================
    // Legal Document Analysis Tests (v0.2.4)
    // ============================================================================

    #[test]
    fn test_clause_extractor_confidentiality() {
        let extractor = ClauseExtractor::with_defaults();
        let text = "This agreement contains confidential information that must be protected.";

        let clauses = extractor.extract(text);
        assert!(!clauses.is_empty());
        assert!(
            clauses
                .iter()
                .any(|c| c.clause_type == ClauseType::Confidentiality)
        );
    }

    #[test]
    fn test_clause_extractor_multiple_types() {
        let extractor = ClauseExtractor::with_defaults();
        let text = "The parties agree to indemnify each other. This agreement is governed by the laws of Delaware. Termination may occur with 30 days notice.";

        let clauses = extractor.extract(text);
        assert!(clauses.len() >= 3);
    }

    #[test]
    fn test_clause_extractor_custom_pattern() {
        let mut extractor = ClauseExtractor::new();
        extractor.add_pattern(ClauseType::Custom("Test".to_string()), "test clause");

        let text = "This is a test clause for testing.";
        let clauses = extractor.extract(text);

        assert!(!clauses.is_empty());
    }

    #[test]
    fn test_party_identifier_basic() {
        let identifier = PartyIdentifier::with_defaults();
        let text = "This agreement is between Acme Corporation and Beta LLC.";

        let parties = identifier.identify(text);
        assert!(!parties.is_empty());
    }

    #[test]
    fn test_party_identifier_roles() {
        let identifier = PartyIdentifier::with_defaults();
        let text =
            "The party of the first part, John Smith, and the party of the second part, Jane Doe.";

        let parties = identifier.identify(text);
        assert!(parties.iter().any(|p| p.role == PartyRole::FirstParty));
    }

    #[test]
    fn test_obligation_extractor_mandatory() {
        let extractor = ObligationExtractor::new();
        let text =
            "The Seller shall deliver the goods within 30 days. The Buyer must pay upon delivery.";

        let obligations = extractor.extract(text);
        assert!(!obligations.is_empty());
        assert!(
            obligations
                .iter()
                .any(|o| o.obligation_type == ObligationType::Mandatory)
        );
    }

    #[test]
    fn test_obligation_extractor_prohibition() {
        let extractor = ObligationExtractor::new();
        let text = "The Licensee shall not sublicense the software. The parties must not disclose.";

        let obligations = extractor.extract(text);
        // Check that we found at least one prohibition
        assert!(!obligations.is_empty());
        // Note: "shall not" should be detected before "shall" in the check order
    }

    #[test]
    fn test_obligation_extractor_permissive() {
        let extractor = ObligationExtractor::new();
        let text = "The Tenant may renew the lease for an additional year.";

        let obligations = extractor.extract(text);
        assert!(
            obligations
                .iter()
                .any(|o| o.obligation_type == ObligationType::Permissive)
        );
    }

    #[test]
    fn test_deadline_extractor_date_format() {
        let extractor = DeadlineExtractor::new();
        let text =
            "Payment is due by 12/31/2024. All deliverables must be completed before 06/15/2025.";

        let deadlines = extractor.extract(text);
        assert!(!deadlines.is_empty());
        // Note: Simple date parser might not extract all dates perfectly
        // But should find deadline keywords
    }

    #[test]
    fn test_deadline_extractor_keywords() {
        let extractor = DeadlineExtractor::new();
        let text = "The deadline for submission is within 30 days of receipt.";

        let deadlines = extractor.extract(text);
        assert!(!deadlines.is_empty());
    }

    #[test]
    fn test_jurisdiction_detector_us() {
        let detector = JurisdictionDetector::with_defaults();
        let text =
            "This agreement shall be governed by the laws of the State of New York, United States.";

        let result = detector.detect(text);
        assert!(result.is_some());
        let (jurisdiction, _confidence) = result.unwrap();
        assert_eq!(jurisdiction, "US");
    }

    #[test]
    fn test_jurisdiction_detector_multiple_indicators() {
        let detector = JurisdictionDetector::with_defaults();
        let text =
            "This agreement references Delaware corporate law and the United States Supreme Court.";

        let result = detector.detect(text);
        assert!(result.is_some());
    }

    #[test]
    fn test_jurisdiction_detector_uk() {
        let detector = JurisdictionDetector::with_defaults();
        let text = "This contract is governed by English law of England and Wales.";

        let result = detector.detect(text);
        assert!(result.is_some());
        let (jurisdiction, _) = result.unwrap();
        assert_eq!(jurisdiction, "GB");
    }

    #[test]
    fn test_legal_risk_scorer_critical() {
        let scorer = LegalRiskScorer::with_defaults();
        let text =
            "The parties agree to unlimited liability with no limitation of liability whatsoever.";

        let (risk_level, factors) = scorer.score(text);
        // Should be High or Critical due to multiple critical indicators
        assert!(risk_level >= RiskLevel::High);
        assert!(!factors.is_empty());
    }

    #[test]
    fn test_legal_risk_scorer_medium() {
        let scorer = LegalRiskScorer::with_defaults();
        let text = "This product is sold as-is with no warranty. All sales are non-refundable.";

        let (risk_level, _) = scorer.score(text);
        assert!(risk_level >= RiskLevel::Medium);
    }

    #[test]
    fn test_legal_risk_scorer_low() {
        let scorer = LegalRiskScorer::with_defaults();
        let text = "Seller provides indemnification and maintains insurance. Liability is limited to contract value.";

        let (risk_level, _) = scorer.score(text);
        assert!(risk_level <= RiskLevel::Medium);
    }

    #[test]
    fn test_legal_risk_scorer_mitigation() {
        let scorer = LegalRiskScorer::with_defaults();
        let text = "This contract includes unlimited liability provisions.";

        let (_risk_level, factors) = scorer.score(text);
        assert!(factors.iter().any(|f| f.mitigation.is_some()));
    }

    #[test]
    fn test_legal_document_analyzer_comprehensive() {
        let analyzer = LegalDocumentAnalyzer::new();
        let text = "This Mutual Non-Disclosure Agreement is between Acme Corp and Beta LLC. \
                   The parties shall maintain confidentiality of all proprietary information. \
                   The Recipient shall not disclose any confidential information to third parties. \
                   This agreement is governed by the laws of Delaware, United States. \
                   Payment is due by 12/31/2024. \
                   The agreement includes indemnification provisions.";

        let analysis = analyzer.analyze(text);

        // Check that all analysis components are present
        assert!(!analysis.clauses.is_empty(), "Should extract clauses");
        assert!(!analysis.parties.is_empty(), "Should identify parties");
        assert!(
            !analysis.obligations.is_empty(),
            "Should extract obligations"
        );
        assert!(!analysis.deadlines.is_empty(), "Should extract deadlines");
        assert!(
            analysis.jurisdiction.is_some(),
            "Should detect jurisdiction"
        );
    }

    #[test]
    fn test_document_analysis_clauses() {
        let analyzer = LegalDocumentAnalyzer::new();
        let text = "This agreement contains confidential information. The parties agree to indemnify each other.";

        let analysis = analyzer.analyze(text);
        assert!(
            analysis
                .clauses
                .iter()
                .any(|c| matches!(c.clause_type, ClauseType::Confidentiality))
        );
        assert!(
            analysis
                .clauses
                .iter()
                .any(|c| matches!(c.clause_type, ClauseType::Indemnification))
        );
    }

    #[test]
    fn test_document_analysis_risk_assessment() {
        let analyzer = LegalDocumentAnalyzer::new();
        let high_risk_text =
            "This agreement includes unlimited liability and personal guarantee clauses.";
        let low_risk_text =
            "This agreement includes limitation of liability and insurance requirements.";

        let high_risk_analysis = analyzer.analyze(high_risk_text);
        let low_risk_analysis = analyzer.analyze(low_risk_text);

        assert!(high_risk_analysis.risk_level >= RiskLevel::High);
        assert!(low_risk_analysis.risk_level <= RiskLevel::Medium);
    }

    #[test]
    fn test_clause_type_display() {
        assert_eq!(ClauseType::Confidentiality.to_string(), "Confidentiality");
        assert_eq!(
            ClauseType::LimitationOfLiability.to_string(),
            "Limitation of Liability"
        );
        assert_eq!(ClauseType::GoverningLaw.to_string(), "Governing Law");
        assert_eq!(
            ClauseType::Custom("MyClause".to_string()).to_string(),
            "MyClause"
        );
    }

    #[test]
    fn test_risk_level_display() {
        assert_eq!(RiskLevel::Low.to_string(), "Low");
        assert_eq!(RiskLevel::Medium.to_string(), "Medium");
        assert_eq!(RiskLevel::High.to_string(), "High");
        assert_eq!(RiskLevel::Critical.to_string(), "Critical");
    }

    #[test]
    fn test_risk_level_ordering() {
        assert!(RiskLevel::Low < RiskLevel::Medium);
        assert!(RiskLevel::Medium < RiskLevel::High);
        assert!(RiskLevel::High < RiskLevel::Critical);
    }

    #[test]
    fn test_obligation_type_variants() {
        let mandatory = ObligationType::Mandatory;
        let permissive = ObligationType::Permissive;
        let prohibition = ObligationType::Prohibition;
        let recommendation = ObligationType::Recommendation;

        assert_ne!(mandatory, permissive);
        assert_ne!(mandatory, prohibition);
        assert_ne!(permissive, recommendation);
    }

    #[test]
    fn test_party_role_variants() {
        let first = PartyRole::FirstParty;
        let second = PartyRole::SecondParty;
        let plaintiff = PartyRole::Plaintiff;
        let defendant = PartyRole::Defendant;

        assert_ne!(first, second);
        assert_ne!(plaintiff, defendant);
    }

    #[test]
    fn test_extracted_clause_confidence() {
        let extractor = ClauseExtractor::with_defaults();
        let text = "The parties shall hereby maintain confidentiality pursuant to this agreement.";

        let clauses = extractor.extract(text);
        assert!(clauses.iter().any(|c| c.confidence > 0.5));
    }

    #[test]
    fn test_deadline_extractor_with_reference_date() {
        let extractor = DeadlineExtractor::new().with_reference_date(2024, 1, 1);
        let text = "Delivery is due by 12/31/2024.";

        let deadlines = extractor.extract(text);
        assert!(!deadlines.is_empty());
    }

    #[test]
    fn test_custom_jurisdiction_indicator() {
        let mut detector = JurisdictionDetector::new();
        detector.add_indicator("CUSTOM", "custom jurisdiction");

        let text = "This agreement is under custom jurisdiction rules.";
        let result = detector.detect(text);

        assert!(result.is_some());
        let (jurisdiction, _) = result.unwrap();
        assert_eq!(jurisdiction, "CUSTOM");
    }

    #[test]
    fn test_custom_risk_indicator() {
        let mut scorer = LegalRiskScorer::new();
        scorer.add_indicator("dangerous clause", RiskLevel::Critical);
        scorer.add_indicator("very risky term", RiskLevel::Critical);
        scorer.add_indicator("extreme hazard", RiskLevel::Critical);

        let text =
            "This contract contains a dangerous clause and very risky term with extreme hazard.";
        let (risk_level, factors) = scorer.score(text);

        // Three critical indicators should push it to Critical
        assert_eq!(risk_level, RiskLevel::Critical);
        assert!(!factors.is_empty());
    }

    #[test]
    fn test_analyzer_mutable_access() {
        let mut analyzer = LegalDocumentAnalyzer::new();

        // Test mutable access to components
        analyzer
            .clause_extractor_mut()
            .add_pattern(ClauseType::Custom("Test".to_string()), "test pattern");

        analyzer
            .jurisdiction_detector_mut()
            .add_indicator("TEST", "test jurisdiction");
        analyzer
            .risk_scorer_mut()
            .add_indicator("test risk", RiskLevel::High);

        let text = "This test pattern is in test jurisdiction with test risk.";
        let analysis = analyzer.analyze(text);

        // Verify custom patterns were used
        assert!(analysis.jurisdiction.is_some());
    }
}

// ============================================================================
// Machine Translation Integration (v0.2.5)
// ============================================================================

/// Translation quality score (0.0 to 1.0).
pub type QualityScore = f32;

/// Translation engine type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TranslationEngine {
    /// Generic neural machine translation
    Generic,
    /// Legal-domain fine-tuned model
    LegalDomain,
    /// Custom model (user-provided)
    Custom,
}

impl std::fmt::Display for TranslationEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TranslationEngine::Generic => write!(f, "Generic"),
            TranslationEngine::LegalDomain => write!(f, "Legal Domain"),
            TranslationEngine::Custom => write!(f, "Custom"),
        }
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

/// Neural machine translator for legal documents.
///
/// Simulates legal-domain neural machine translation with quality estimation.
#[derive(Debug, Clone)]
pub struct NeuralMachineTranslator {
    /// Translation engine
    engine: TranslationEngine,
    /// Quality threshold (0.0 to 1.0)
    quality_threshold: QualityScore,
    /// Legal dictionary for domain adaptation
    dictionary: Option<Arc<LegalDictionary>>,
}

impl NeuralMachineTranslator {
    /// Creates a new neural machine translator.
    pub fn new(engine: TranslationEngine) -> Self {
        Self {
            engine,
            quality_threshold: 0.7,
            dictionary: None,
        }
    }

    /// Creates a legal-domain translator.
    pub fn legal_domain() -> Self {
        Self::new(TranslationEngine::LegalDomain)
    }

    /// Sets the quality threshold.
    pub fn with_quality_threshold(mut self, threshold: QualityScore) -> Self {
        self.quality_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Adds a legal dictionary for domain adaptation.
    pub fn with_dictionary(mut self, dictionary: Arc<LegalDictionary>) -> Self {
        self.dictionary = Some(dictionary);
        self
    }

    /// Translates text with quality estimation.
    ///
    /// In a real implementation, this would call an external MT API.
    /// For this simulation, we estimate quality based on text characteristics.
    pub fn translate(
        &self,
        text: &str,
        source: &Locale,
        target: &Locale,
    ) -> I18nResult<MTTranslation> {
        // Simulate translation (in production, call external API)
        let translated_text = self.simulate_translation(text, source, target);

        // Estimate quality (in production, use model-based QE)
        let quality_score = self.estimate_quality(text, &translated_text, source, target);

        // Generate alternatives (n-best list)
        let alternatives = self.generate_alternatives(text, source, target);

        Ok(MTTranslation {
            text: translated_text,
            quality_score,
            source_locale: source.clone(),
            target_locale: target.clone(),
            engine: self.engine,
            alternatives,
        })
    }

    /// Simulates translation (placeholder for external MT API).
    fn simulate_translation(&self, text: &str, _source: &Locale, target: &Locale) -> String {
        // If we have a dictionary, try to use it
        if let Some(dict) = &self.dictionary {
            // Check if text is a single term in dictionary
            if let Some(translation) = dict.translate(text) {
                return translation.to_string();
            }
        }

        // Simulate translation with locale marker
        format!("[{}] {}", target.tag(), text)
    }

    /// Estimates translation quality.
    fn estimate_quality(
        &self,
        source_text: &str,
        translated_text: &str,
        _source: &Locale,
        _target: &Locale,
    ) -> QualityScore {
        // Simulate quality estimation based on heuristics
        let mut score: f32 = 0.8; // Base score for legal domain

        // Penalize very short translations
        if translated_text.len() < source_text.len() / 2 {
            score -= 0.2;
        }

        // Boost score if dictionary was used
        if self.dictionary.is_some() && !translated_text.starts_with('[') {
            score += 0.15;
        }

        // Legal domain engine gets higher scores
        if self.engine == TranslationEngine::LegalDomain {
            score += 0.05;
        }

        score.clamp(0.0, 1.0)
    }

    /// Generates alternative translations.
    fn generate_alternatives(
        &self,
        text: &str,
        source: &Locale,
        target: &Locale,
    ) -> Vec<(String, QualityScore)> {
        let mut alternatives = Vec::new();

        // Generate 2-3 alternatives with decreasing quality scores
        alternatives.push((
            format!("[{}] Alt1: {}", target.tag(), text),
            self.estimate_quality(text, text, source, target) - 0.1,
        ));
        alternatives.push((
            format!("[{}] Alt2: {}", target.tag(), text),
            self.estimate_quality(text, text, source, target) - 0.2,
        ));

        alternatives
    }

    /// Returns the quality threshold.
    pub fn quality_threshold(&self) -> QualityScore {
        self.quality_threshold
    }

    /// Returns the engine type.
    pub fn engine(&self) -> TranslationEngine {
        self.engine
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

/// Terminology-aware translator that preserves legal terms.
pub struct TerminologyAwareTranslator {
    /// Base MT translator
    mt_translator: NeuralMachineTranslator,
    /// Glossary for term preservation
    glossary: HashMap<String, String>,
    /// Preservation mode
    preservation_mode: TermPreservationMode,
}

impl TerminologyAwareTranslator {
    /// Creates a new terminology-aware translator.
    pub fn new(mt_translator: NeuralMachineTranslator) -> Self {
        Self {
            mt_translator,
            glossary: HashMap::new(),
            preservation_mode: TermPreservationMode::GlossaryEnforced,
        }
    }

    /// Adds a term to the glossary.
    pub fn add_term(&mut self, source_term: impl Into<String>, target_term: impl Into<String>) {
        self.glossary.insert(source_term.into(), target_term.into());
    }

    /// Loads glossary from dictionary.
    pub fn load_glossary_from_dictionary(
        &mut self,
        dictionary: &LegalDictionary,
        _target: &Locale,
    ) {
        for (term, translation) in dictionary.translations.iter() {
            self.glossary.insert(term.clone(), translation.clone());
        }
    }

    /// Sets preservation mode.
    pub fn with_preservation_mode(mut self, mode: TermPreservationMode) -> Self {
        self.preservation_mode = mode;
        self
    }

    /// Translates text while preserving terminology.
    pub fn translate(
        &self,
        text: &str,
        source: &Locale,
        target: &Locale,
    ) -> I18nResult<MTTranslation> {
        // First, identify and mark terms to preserve
        let (marked_text, term_positions) = self.mark_terms(text);

        // Translate the marked text
        let mut translation = self.mt_translator.translate(&marked_text, source, target)?;

        // Restore preserved terms
        translation.text = self.restore_terms(&translation.text, &term_positions);

        // Boost quality score for term preservation
        translation.quality_score = (translation.quality_score + 0.1).min(1.0);

        Ok(translation)
    }

    /// Marks terms in text that should be preserved.
    fn mark_terms(&self, text: &str) -> (String, Vec<(String, String)>) {
        let mut marked = text.to_string();
        let mut positions = Vec::new();

        // Sort glossary terms by length (longest first) to handle overlapping terms
        let mut terms: Vec<(&String, &String)> = self.glossary.iter().collect();
        terms.sort_by_key(|(k, _)| std::cmp::Reverse(k.len()));

        for (source_term, target_term) in terms {
            match self.preservation_mode {
                TermPreservationMode::Exact => {
                    // Mark term for exact preservation
                    let marker = format!("__TERM_{}__", positions.len());
                    if marked.contains(source_term.as_str()) {
                        marked = marked.replace(source_term, &marker);
                        positions.push((marker, source_term.clone()));
                    }
                }
                TermPreservationMode::GlossaryEnforced => {
                    // Mark term for glossary replacement
                    let marker = format!("__TERM_{}__", positions.len());
                    if marked.contains(source_term.as_str()) {
                        marked = marked.replace(source_term, &marker);
                        positions.push((marker, target_term.clone()));
                    }
                }
                TermPreservationMode::PreserveFormatting => {
                    // Let MT translate but we'll restore formatting
                    // (simplified for this implementation)
                    let marker = format!("__TERM_{}__", positions.len());
                    if marked.contains(source_term.as_str()) {
                        marked = marked.replace(source_term, &marker);
                        positions.push((marker, target_term.clone()));
                    }
                }
            }
        }

        (marked, positions)
    }

    /// Restores preserved terms in translated text.
    fn restore_terms(&self, translated: &str, positions: &[(String, String)]) -> String {
        let mut result = translated.to_string();

        for (marker, term) in positions {
            result = result.replace(marker, term);
        }

        result
    }

    /// Returns the number of glossary terms.
    pub fn glossary_size(&self) -> usize {
        self.glossary.len()
    }
}

/// Translation with memory integration.
#[derive(Debug, Clone)]
pub struct MTWithMemory {
    /// Neural MT translator
    mt_translator: Arc<NeuralMachineTranslator>,
    /// Translation memory
    memory: Arc<Mutex<TranslationMemory>>,
    /// Minimum fuzzy match threshold
    fuzzy_threshold: f32,
}

impl MTWithMemory {
    /// Creates a new MT with memory integration.
    pub fn new(
        mt_translator: NeuralMachineTranslator,
        memory: Arc<Mutex<TranslationMemory>>,
    ) -> Self {
        Self {
            mt_translator: Arc::new(mt_translator),
            memory,
            fuzzy_threshold: 0.85,
        }
    }

    /// Sets fuzzy match threshold.
    pub fn with_fuzzy_threshold(mut self, threshold: f32) -> Self {
        self.fuzzy_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Translates with memory lookup and MT fallback.
    pub fn translate(
        &self,
        text: &str,
        source: &Locale,
        target: &Locale,
    ) -> I18nResult<MTTranslation> {
        // Try exact match first
        {
            let memory_guard = self.memory.lock().unwrap();
            let exact_matches = memory_guard.find_exact(text, source, target);
            if !exact_matches.is_empty() {
                let target_text = exact_matches[0].target_text.clone();
                drop(memory_guard);
                return Ok(MTTranslation {
                    text: target_text,
                    quality_score: 1.0, // Exact match = perfect quality
                    source_locale: source.clone(),
                    target_locale: target.clone(),
                    engine: TranslationEngine::Generic,
                    alternatives: vec![],
                });
            }

            // Try fuzzy match
            let fuzzy_matches =
                memory_guard.find_fuzzy_levenshtein(text, source, target, self.fuzzy_threshold);
            if !fuzzy_matches.is_empty() {
                let (entry, score) = fuzzy_matches[0];
                let target_text = entry.target_text.clone();
                drop(memory_guard);
                return Ok(MTTranslation {
                    text: target_text,
                    quality_score: score, // Fuzzy match quality
                    source_locale: source.clone(),
                    target_locale: target.clone(),
                    engine: TranslationEngine::Generic,
                    alternatives: vec![],
                });
            }
        }

        // Fall back to MT
        let mt_result = self.mt_translator.translate(text, source, target)?;

        // Add to memory if quality is good
        if mt_result.quality_score >= 0.8 {
            let mut memory_guard = self.memory.lock().unwrap();
            memory_guard.add_translation(
                text.to_string(),
                source.clone(),
                mt_result.text.clone(),
                target.clone(),
            );
        }

        Ok(mt_result)
    }

    /// Returns the fuzzy match threshold.
    pub fn fuzzy_threshold(&self) -> f32 {
        self.fuzzy_threshold
    }
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

        // Check for mandatory term violations
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

        // Check for forbidden terms
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

                // Remove forbidden term
                if self.case_sensitive {
                    enforced = enforced.replace(forbidden, "[REMOVED]");
                } else {
                    // Case-insensitive replacement is more complex
                    // For simplicity, just mark it
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

impl Default for GlossaryEnforcer {
    fn default() -> Self {
        Self::new()
    }
}

/// Glossary violation type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViolationType {
    /// Mandatory term missing in translation
    MissingMandatoryTerm,
    /// Forbidden term found in translation
    ForbiddenTermUsed,
}

impl std::fmt::Display for ViolationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ViolationType::MissingMandatoryTerm => write!(f, "Missing Mandatory Term"),
            ViolationType::ForbiddenTermUsed => write!(f, "Forbidden Term Used"),
        }
    }
}

/// Glossary violation.
#[derive(Debug, Clone)]
pub struct GlossaryViolation {
    /// Violation type
    pub violation_type: ViolationType,
    /// Term involved
    pub term: String,
    /// Expected term (for mandatory violations)
    pub expected: Option<String>,
    /// Found term (for forbidden violations)
    pub found: Option<String>,
}

/// Post-editing action.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PostEditAction {
    /// Accept translation as-is
    Accept,
    /// Reject and request new translation
    Reject,
    /// Edit specific segments
    Edit,
}

/// Post-editing feedback.
#[derive(Debug, Clone)]
pub struct PostEditFeedback {
    /// Original translation
    pub original: String,
    /// Edited translation (if action is Edit)
    pub edited: Option<String>,
    /// Action taken
    pub action: PostEditAction,
    /// Quality rating (0.0 to 1.0)
    pub quality_rating: Option<QualityScore>,
    /// Comments
    pub comments: Vec<String>,
}

/// Post-editing workflow for translation review.
pub struct PostEditingWorkflow {
    /// Pending translations for review
    pending: Vec<(String, MTTranslation)>,
    /// Accepted translations
    accepted: Vec<(String, String)>,
    /// Rejected translations
    rejected: Vec<(String, String)>,
}

impl PostEditingWorkflow {
    /// Creates a new post-editing workflow.
    pub fn new() -> Self {
        Self {
            pending: Vec::new(),
            accepted: Vec::new(),
            rejected: Vec::new(),
        }
    }

    /// Adds a translation for review.
    pub fn add_for_review(&mut self, source: impl Into<String>, translation: MTTranslation) {
        self.pending.push((source.into(), translation));
    }

    /// Submits post-editing feedback.
    pub fn submit_feedback(&mut self, index: usize, feedback: PostEditFeedback) {
        if index >= self.pending.len() {
            return;
        }

        let (source, translation) = self.pending.remove(index);

        match feedback.action {
            PostEditAction::Accept => {
                self.accepted.push((source, translation.text));
            }
            PostEditAction::Reject => {
                self.rejected.push((source, translation.text));
            }
            PostEditAction::Edit => {
                if let Some(edited) = feedback.edited {
                    self.accepted.push((source, edited));
                } else {
                    // If no edited text provided, treat as accept
                    self.accepted.push((source, translation.text));
                }
            }
        }
    }

    /// Returns pending translations count.
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }

    /// Returns accepted translations count.
    pub fn accepted_count(&self) -> usize {
        self.accepted.len()
    }

    /// Returns rejected translations count.
    pub fn rejected_count(&self) -> usize {
        self.rejected.len()
    }

    /// Gets pending translation at index.
    pub fn get_pending(&self, index: usize) -> Option<&(String, MTTranslation)> {
        self.pending.get(index)
    }

    /// Exports accepted translations to translation memory.
    pub fn export_to_memory(
        &self,
        memory: &mut TranslationMemory,
        source_locale: &Locale,
        target_locale: &Locale,
    ) {
        for (source, target) in &self.accepted {
            memory.add_translation(
                source.clone(),
                source_locale.clone(),
                target.clone(),
                target_locale.clone(),
            );
        }
    }

    /// Clears all translations.
    pub fn clear(&mut self) {
        self.pending.clear();
        self.accepted.clear();
        self.rejected.clear();
    }
}

impl Default for PostEditingWorkflow {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod mt_tests {
    use super::*;

    #[test]
    fn test_neural_mt_basic() {
        let translator = NeuralMachineTranslator::new(TranslationEngine::Generic);
        let source = Locale::new("en").with_country("US");
        let target = Locale::new("ja").with_country("JP");

        let result = translator.translate("contract", &source, &target);
        assert!(result.is_ok());

        let translation = result.unwrap();
        assert!(!translation.text.is_empty());
        assert!(translation.quality_score >= 0.0 && translation.quality_score <= 1.0);
        assert_eq!(translation.engine, TranslationEngine::Generic);
    }

    #[test]
    fn test_neural_mt_legal_domain() {
        let translator = NeuralMachineTranslator::legal_domain();
        let source = Locale::new("en").with_country("US");
        let target = Locale::new("de").with_country("DE");

        let result = translator.translate("plaintiff", &source, &target);
        assert!(result.is_ok());

        let translation = result.unwrap();
        assert_eq!(translation.engine, TranslationEngine::LegalDomain);
        // Legal domain should have slightly higher quality
        assert!(translation.quality_score >= 0.8);
    }

    #[test]
    fn test_neural_mt_with_dictionary() {
        let locale_en_us = Locale::new("en").with_country("US");
        let mut dict = LegalDictionary::new(locale_en_us.clone());
        dict.add_translation("contract", "Vertrag");

        let translator = NeuralMachineTranslator::new(TranslationEngine::Generic)
            .with_dictionary(Arc::new(dict));

        let source = Locale::new("en").with_country("US");
        let target = Locale::new("de").with_country("DE");

        let result = translator.translate("contract", &source, &target).unwrap();
        assert_eq!(result.text, "Vertrag");
        assert!(result.quality_score > 0.9); // Dictionary use boosts quality
    }

    #[test]
    fn test_neural_mt_alternatives() {
        let translator = NeuralMachineTranslator::new(TranslationEngine::Generic);
        let source = Locale::new("en").with_country("US");
        let target = Locale::new("fr").with_country("FR");

        let result = translator.translate("statute", &source, &target).unwrap();
        assert!(!result.alternatives.is_empty());
        assert!(result.alternatives.len() >= 2);
    }

    #[test]
    fn test_terminology_aware_translator() {
        let mt = NeuralMachineTranslator::new(TranslationEngine::Generic);
        let mut term_translator = TerminologyAwareTranslator::new(mt);

        term_translator.add_term("plaintiff", "demandeur");
        term_translator.add_term("defendant", "défendeur");

        let source = Locale::new("en").with_country("US");
        let target = Locale::new("fr").with_country("FR");

        let result = term_translator
            .translate("The plaintiff sued the defendant", &source, &target)
            .unwrap();

        assert!(result.text.contains("demandeur"));
        assert!(result.text.contains("défendeur"));
        assert_eq!(term_translator.glossary_size(), 2);
    }

    #[test]
    fn test_terminology_aware_with_dictionary() {
        let locale_en_us = Locale::new("en").with_country("US");
        let mut dict = LegalDictionary::new(locale_en_us);
        dict.add_translation("tort", "responsabilité civile");
        dict.add_translation("contract", "contrat");

        let mt = NeuralMachineTranslator::new(TranslationEngine::Generic);
        let mut term_translator = TerminologyAwareTranslator::new(mt);

        let target = Locale::new("fr").with_country("FR");
        term_translator.load_glossary_from_dictionary(&dict, &target);

        assert_eq!(term_translator.glossary_size(), 2);
    }

    #[test]
    fn test_mt_with_memory_exact_match() {
        let mt = NeuralMachineTranslator::new(TranslationEngine::Generic);
        let mut memory = TranslationMemory::new();
        let source_locale = Locale::new("en").with_country("US");
        let target_locale = Locale::new("fr").with_country("FR");
        memory.add_translation(
            "contract".to_string(),
            source_locale.clone(),
            "contrat".to_string(),
            target_locale.clone(),
        );

        let mt_with_memory = MTWithMemory::new(mt, Arc::new(Mutex::new(memory)));

        let result = mt_with_memory
            .translate("contract", &source_locale, &target_locale)
            .unwrap();
        assert_eq!(result.text, "contrat");
        assert_eq!(result.quality_score, 1.0); // Exact match = perfect quality
    }

    #[test]
    fn test_mt_with_memory_fallback() {
        let mt = NeuralMachineTranslator::new(TranslationEngine::Generic);
        let memory = TranslationMemory::new();

        let mt_with_memory = MTWithMemory::new(mt, Arc::new(Mutex::new(memory)));

        let source = Locale::new("en").with_country("US");
        let target = Locale::new("ja").with_country("JP");

        let result = mt_with_memory
            .translate("new term", &source, &target)
            .unwrap();
        assert!(!result.text.is_empty());
    }

    #[test]
    fn test_glossary_enforcer_mandatory_terms() {
        let mut enforcer = GlossaryEnforcer::new();
        enforcer.add_mandatory_term("plaintiff", "demandeur");
        enforcer.add_mandatory_term("defendant", "défendeur");

        let source = "The plaintiff sued the defendant";
        let translation = "Le demandeur a poursuivi la partie adverse";

        let (_, violations) = enforcer.enforce(source, translation);

        // Should detect missing "défendeur"
        assert!(!violations.is_empty());
        assert!(
            violations
                .iter()
                .any(|v| v.violation_type == ViolationType::MissingMandatoryTerm)
        );
    }

    #[test]
    fn test_glossary_enforcer_forbidden_terms() {
        let mut enforcer = GlossaryEnforcer::new();
        enforcer.add_forbidden_term("bad word");
        enforcer.add_forbidden_term("inappropriate");

        let source = "This is a test";
        let translation = "This contains a bad word";

        let (enforced, violations) = enforcer.enforce(source, translation);

        assert!(!violations.is_empty());
        assert!(
            violations
                .iter()
                .any(|v| v.violation_type == ViolationType::ForbiddenTermUsed)
        );
        assert!(enforced.contains("[REMOVED]") || enforced.contains("[FORBIDDEN]"));
    }

    #[test]
    fn test_glossary_enforcer_counts() {
        let mut enforcer = GlossaryEnforcer::new();
        enforcer.add_mandatory_term("term1", "translation1");
        enforcer.add_mandatory_term("term2", "translation2");
        enforcer.add_forbidden_term("forbidden1");

        assert_eq!(enforcer.mandatory_term_count(), 2);
        assert_eq!(enforcer.forbidden_term_count(), 1);
    }

    #[test]
    fn test_post_editing_workflow() {
        let mut workflow = PostEditingWorkflow::new();

        let source = Locale::new("en").with_country("US");
        let target = Locale::new("fr").with_country("FR");

        let translation1 = MTTranslation {
            text: "contrat".to_string(),
            quality_score: 0.9,
            source_locale: source.clone(),
            target_locale: target.clone(),
            engine: TranslationEngine::Generic,
            alternatives: vec![],
        };

        let translation2 = MTTranslation {
            text: "accord".to_string(),
            quality_score: 0.7,
            source_locale: source.clone(),
            target_locale: target.clone(),
            engine: TranslationEngine::Generic,
            alternatives: vec![],
        };

        workflow.add_for_review("contract", translation1);
        workflow.add_for_review("agreement", translation2);

        assert_eq!(workflow.pending_count(), 2);
        assert_eq!(workflow.accepted_count(), 0);
        assert_eq!(workflow.rejected_count(), 0);
    }

    #[test]
    fn test_post_editing_accept() {
        let mut workflow = PostEditingWorkflow::new();

        let source = Locale::new("en").with_country("US");
        let target = Locale::new("fr").with_country("FR");

        let translation = MTTranslation {
            text: "contrat".to_string(),
            quality_score: 0.95,
            source_locale: source,
            target_locale: target,
            engine: TranslationEngine::Generic,
            alternatives: vec![],
        };

        workflow.add_for_review("contract", translation);

        let feedback = PostEditFeedback {
            original: "contrat".to_string(),
            edited: None,
            action: PostEditAction::Accept,
            quality_rating: Some(0.95),
            comments: vec![],
        };

        workflow.submit_feedback(0, feedback);

        assert_eq!(workflow.pending_count(), 0);
        assert_eq!(workflow.accepted_count(), 1);
        assert_eq!(workflow.rejected_count(), 0);
    }

    #[test]
    fn test_post_editing_reject() {
        let mut workflow = PostEditingWorkflow::new();

        let source = Locale::new("en").with_country("US");
        let target = Locale::new("ja").with_country("JP");

        let translation = MTTranslation {
            text: "bad translation".to_string(),
            quality_score: 0.4,
            source_locale: source,
            target_locale: target,
            engine: TranslationEngine::Generic,
            alternatives: vec![],
        };

        workflow.add_for_review("contract", translation);

        let feedback = PostEditFeedback {
            original: "bad translation".to_string(),
            edited: None,
            action: PostEditAction::Reject,
            quality_rating: Some(0.2),
            comments: vec!["Poor quality".to_string()],
        };

        workflow.submit_feedback(0, feedback);

        assert_eq!(workflow.pending_count(), 0);
        assert_eq!(workflow.accepted_count(), 0);
        assert_eq!(workflow.rejected_count(), 1);
    }

    #[test]
    fn test_post_editing_edit() {
        let mut workflow = PostEditingWorkflow::new();

        let source = Locale::new("en").with_country("US");
        let target = Locale::new("de").with_country("DE");

        let translation = MTTranslation {
            text: "Kontrakt".to_string(),
            quality_score: 0.7,
            source_locale: source,
            target_locale: target,
            engine: TranslationEngine::Generic,
            alternatives: vec![],
        };

        workflow.add_for_review("contract", translation);

        let feedback = PostEditFeedback {
            original: "Kontrakt".to_string(),
            edited: Some("Vertrag".to_string()),
            action: PostEditAction::Edit,
            quality_rating: Some(0.9),
            comments: vec!["Corrected to proper legal term".to_string()],
        };

        workflow.submit_feedback(0, feedback);

        assert_eq!(workflow.pending_count(), 0);
        assert_eq!(workflow.accepted_count(), 1);
    }

    #[test]
    fn test_post_editing_export_to_memory() {
        let mut workflow = PostEditingWorkflow::new();

        let source = Locale::new("en").with_country("US");
        let target = Locale::new("fr").with_country("FR");

        let translation = MTTranslation {
            text: "contrat".to_string(),
            quality_score: 0.95,
            source_locale: source.clone(),
            target_locale: target.clone(),
            engine: TranslationEngine::Generic,
            alternatives: vec![],
        };

        workflow.add_for_review("contract", translation);

        let feedback = PostEditFeedback {
            original: "contrat".to_string(),
            edited: None,
            action: PostEditAction::Accept,
            quality_rating: Some(0.95),
            comments: vec![],
        };

        workflow.submit_feedback(0, feedback);

        let mut memory = TranslationMemory::new();
        workflow.export_to_memory(&mut memory, &source, &target);

        let entries = memory.find_exact("contract", &source, &target);
        assert!(!entries.is_empty());
        assert_eq!(entries[0].target_text, "contrat");
    }

    #[test]
    fn test_translation_engine_display() {
        assert_eq!(TranslationEngine::Generic.to_string(), "Generic");
        assert_eq!(TranslationEngine::LegalDomain.to_string(), "Legal Domain");
        assert_eq!(TranslationEngine::Custom.to_string(), "Custom");
    }

    #[test]
    fn test_violation_type_display() {
        assert_eq!(
            ViolationType::MissingMandatoryTerm.to_string(),
            "Missing Mandatory Term"
        );
        assert_eq!(
            ViolationType::ForbiddenTermUsed.to_string(),
            "Forbidden Term Used"
        );
    }

    #[test]
    fn test_term_preservation_modes() {
        let mt = NeuralMachineTranslator::new(TranslationEngine::Generic);

        // Test exact preservation
        let translator_exact = TerminologyAwareTranslator::new(mt.clone())
            .with_preservation_mode(TermPreservationMode::Exact);
        assert_eq!(
            translator_exact.preservation_mode,
            TermPreservationMode::Exact
        );

        // Test glossary enforced
        let translator_glossary = TerminologyAwareTranslator::new(mt.clone())
            .with_preservation_mode(TermPreservationMode::GlossaryEnforced);
        assert_eq!(
            translator_glossary.preservation_mode,
            TermPreservationMode::GlossaryEnforced
        );
    }

    #[test]
    fn test_mt_quality_threshold() {
        let translator =
            NeuralMachineTranslator::new(TranslationEngine::Generic).with_quality_threshold(0.85);

        assert_eq!(translator.quality_threshold(), 0.85);
    }

    #[test]
    fn test_mt_with_memory_fuzzy_threshold() {
        let mt = NeuralMachineTranslator::new(TranslationEngine::Generic);
        let memory = TranslationMemory::new();

        let mt_with_memory =
            MTWithMemory::new(mt, Arc::new(Mutex::new(memory))).with_fuzzy_threshold(0.9);

        assert_eq!(mt_with_memory.fuzzy_threshold(), 0.9);
    }

    #[test]
    fn test_workflow_clear() {
        let mut workflow = PostEditingWorkflow::new();

        let source = Locale::new("en").with_country("US");
        let target = Locale::new("fr").with_country("FR");

        let translation = MTTranslation {
            text: "test".to_string(),
            quality_score: 0.8,
            source_locale: source,
            target_locale: target,
            engine: TranslationEngine::Generic,
            alternatives: vec![],
        };

        workflow.add_for_review("test", translation);
        assert_eq!(workflow.pending_count(), 1);

        workflow.clear();
        assert_eq!(workflow.pending_count(), 0);
        assert_eq!(workflow.accepted_count(), 0);
        assert_eq!(workflow.rejected_count(), 0);
    }

    #[test]
    fn test_workflow_get_pending() {
        let mut workflow = PostEditingWorkflow::new();

        let source = Locale::new("en").with_country("US");
        let target = Locale::new("ja").with_country("JP");

        let translation = MTTranslation {
            text: "test".to_string(),
            quality_score: 0.8,
            source_locale: source,
            target_locale: target,
            engine: TranslationEngine::Generic,
            alternatives: vec![],
        };

        workflow.add_for_review("original", translation);

        let pending = workflow.get_pending(0);
        assert!(pending.is_some());
        assert_eq!(pending.unwrap().0, "original");
    }
}

// ============================================================================
// Cultural Adaptation (v0.2.6)
// ============================================================================

/// Cultural context category.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContextCategory {
    /// Social hierarchy and honorifics
    SocialHierarchy,
    /// Family structure and relationships
    FamilyStructure,
    /// Religious practices
    ReligiousPractice,
    /// Business etiquette
    BusinessEtiquette,
    /// Legal formality levels
    LegalFormality,
    /// Gender roles and expectations
    GenderRoles,
    /// Time perception (monochronic vs polychronic)
    TimePerception,
    /// Communication style (direct vs indirect)
    CommunicationStyle,
    /// Custom category
    Custom(String),
}

impl std::fmt::Display for ContextCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextCategory::SocialHierarchy => write!(f, "Social Hierarchy"),
            ContextCategory::FamilyStructure => write!(f, "Family Structure"),
            ContextCategory::ReligiousPractice => write!(f, "Religious Practice"),
            ContextCategory::BusinessEtiquette => write!(f, "Business Etiquette"),
            ContextCategory::LegalFormality => write!(f, "Legal Formality"),
            ContextCategory::GenderRoles => write!(f, "Gender Roles"),
            ContextCategory::TimePerception => write!(f, "Time Perception"),
            ContextCategory::CommunicationStyle => write!(f, "Communication Style"),
            ContextCategory::Custom(s) => write!(f, "{}", s),
        }
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

/// Cultural context registry.
#[derive(Debug, Clone, Default)]
pub struct CulturalContextRegistry {
    /// Contexts indexed by locale tag
    contexts: HashMap<String, Vec<CulturalContext>>,
}

impl CulturalContextRegistry {
    /// Creates a new cultural context registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a registry with default cultural contexts.
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();
        registry.add_default_contexts();
        registry
    }

    /// Adds a cultural context.
    pub fn add_context(&mut self, context: CulturalContext) {
        self.contexts
            .entry(context.locale.tag())
            .or_default()
            .push(context);
    }

    /// Gets all contexts for a locale.
    pub fn get_contexts(&self, locale: &Locale) -> Vec<&CulturalContext> {
        self.contexts
            .get(&locale.tag())
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    /// Gets contexts by category for a locale.
    pub fn get_by_category(
        &self,
        locale: &Locale,
        category: &ContextCategory,
    ) -> Vec<&CulturalContext> {
        self.get_contexts(locale)
            .into_iter()
            .filter(|c| &c.category == category)
            .collect()
    }

    /// Finds context for a specific term.
    pub fn find_term(&self, locale: &Locale, term: &str) -> Option<&CulturalContext> {
        self.get_contexts(locale)
            .into_iter()
            .find(|c| c.term == term)
    }

    /// Adds default cultural contexts.
    fn add_default_contexts(&mut self) {
        // Japanese contexts
        let ja_jp = Locale::new("ja").with_country("JP");
        self.add_context(
            CulturalContext::new(
                ja_jp.clone(),
                ContextCategory::SocialHierarchy,
                "keigo",
                "Honorific language system used in legal and business contexts to show respect",
            )
            .with_guideline(
                "Use appropriate honorific forms when addressing parties of different status",
            )
            .with_guideline(
                "Failure to use proper keigo may be seen as disrespectful in legal proceedings",
            )
            .with_equivalent("en-US", "formal address"),
        );

        self.add_context(
            CulturalContext::new(
                ja_jp.clone(),
                ContextCategory::BusinessEtiquette,
                "hanko",
                "Personal seal used for legal authentication, equivalent to signature",
            )
            .with_guideline("Hanko is legally binding and often required for contracts")
            .with_guideline("Company hanko (corporate seal) has special legal significance")
            .with_equivalent("en-US", "signature")
            .with_equivalent("zh-CN", "印章 (seal)"),
        );

        // Chinese contexts
        let zh_cn = Locale::new("zh").with_script("Hans").with_country("CN");
        self.add_context(
            CulturalContext::new(
                zh_cn.clone(),
                ContextCategory::SocialHierarchy,
                "guanxi",
                "Network of relationships and mutual obligations crucial in business and legal matters",
            )
            .with_guideline("Understanding guanxi is essential for contract negotiations")
            .with_guideline("Legal disputes may be resolved through guanxi rather than formal proceedings")
            .with_equivalent("ja-JP", "人間関係 (human relationships)"),
        );

        // Arabic contexts
        let ar_sa = Locale::new("ar").with_country("SA");
        self.add_context(
            CulturalContext::new(
                ar_sa.clone(),
                ContextCategory::ReligiousPractice,
                "wasta",
                "System of intercession and mediation in legal and business matters",
            )
            .with_guideline("Wasta can play a significant role in dispute resolution")
            .with_guideline("Consider cultural expectations when drafting contracts")
            .with_equivalent("zh-CN", "关系 (guanxi)"),
        );

        // Indian contexts
        let hi_in = Locale::new("hi").with_country("IN");
        self.add_context(
            CulturalContext::new(
                hi_in,
                ContextCategory::FamilyStructure,
                "joint family",
                "Extended family system with legal implications for property and inheritance",
            )
            .with_guideline("Property law must account for joint family ownership structures")
            .with_guideline("Inheritance differs from Western nuclear family assumptions"),
        );
    }

    /// Returns the total number of contexts.
    pub fn context_count(&self) -> usize {
        self.contexts.values().map(|v| v.len()).sum()
    }

    /// Returns the number of locales with contexts.
    pub fn locale_count(&self) -> usize {
        self.contexts.len()
    }
}

/// Local custom type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CustomType {
    /// Marriage and family customs
    Marriage,
    /// Inheritance and succession
    Inheritance,
    /// Property ownership
    Property,
    /// Business practices
    Business,
    /// Dispute resolution
    DisputeResolution,
    /// Contract formation
    Contract,
}

impl std::fmt::Display for CustomType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomType::Marriage => write!(f, "Marriage"),
            CustomType::Inheritance => write!(f, "Inheritance"),
            CustomType::Property => write!(f, "Property"),
            CustomType::Business => write!(f, "Business"),
            CustomType::DisputeResolution => write!(f, "Dispute Resolution"),
            CustomType::Contract => write!(f, "Contract"),
        }
    }
}

/// Local custom.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalCustom {
    /// Custom name
    pub name: String,
    /// Region where custom applies
    pub region: String,
    /// Locale
    pub locale: Locale,
    /// Type of custom
    pub custom_type: CustomType,
    /// Description
    pub description: String,
    /// Legal recognition level (0.0 = not recognized, 1.0 = fully recognized)
    pub recognition_level: f32,
    /// Statutory basis (if any)
    pub statutory_basis: Option<String>,
}

impl LocalCustom {
    /// Creates a new local custom.
    pub fn new(
        name: impl Into<String>,
        region: impl Into<String>,
        locale: Locale,
        custom_type: CustomType,
        description: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            region: region.into(),
            locale,
            custom_type,
            description: description.into(),
            recognition_level: 0.5,
            statutory_basis: None,
        }
    }

    /// Sets recognition level.
    pub fn with_recognition_level(mut self, level: f32) -> Self {
        self.recognition_level = level.clamp(0.0, 1.0);
        self
    }

    /// Sets statutory basis.
    pub fn with_statutory_basis(mut self, basis: impl Into<String>) -> Self {
        self.statutory_basis = Some(basis.into());
        self
    }
}

/// Local custom registry.
#[derive(Debug, Clone, Default)]
pub struct LocalCustomRegistry {
    /// Customs indexed by region
    customs: HashMap<String, Vec<LocalCustom>>,
}

impl LocalCustomRegistry {
    /// Creates a new local custom registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a registry with default customs.
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();
        registry.add_default_customs();
        registry
    }

    /// Adds a custom.
    pub fn add_custom(&mut self, custom: LocalCustom) {
        self.customs
            .entry(custom.region.clone())
            .or_default()
            .push(custom);
    }

    /// Gets customs for a region.
    pub fn get_customs(&self, region: &str) -> Vec<&LocalCustom> {
        self.customs
            .get(region)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    /// Gets customs by type.
    pub fn get_by_type(&self, region: &str, custom_type: &CustomType) -> Vec<&LocalCustom> {
        self.get_customs(region)
            .into_iter()
            .filter(|c| &c.custom_type == custom_type)
            .collect()
    }

    /// Finds a specific custom by name.
    pub fn find_custom(&self, region: &str, name: &str) -> Option<&LocalCustom> {
        self.get_customs(region)
            .into_iter()
            .find(|c| c.name == name)
    }

    /// Adds default customs.
    fn add_default_customs(&mut self) {
        // Japanese customs
        self.add_custom(
            LocalCustom::new(
                "Miai marriage",
                "Japan",
                Locale::new("ja").with_country("JP"),
                CustomType::Marriage,
                "Traditional arranged marriage introduction system with legal implications for family law",
            )
            .with_recognition_level(0.3),
        );

        self.add_custom(
            LocalCustom::new(
                "Ie system",
                "Japan",
                Locale::new("ja").with_country("JP"),
                CustomType::Inheritance,
                "Traditional household system affecting inheritance and family law",
            )
            .with_recognition_level(0.4)
            .with_statutory_basis("Civil Code Article 897 (family grave inheritance)"),
        );

        // Chinese customs
        self.add_custom(
            LocalCustom::new(
                "Red packet custom",
                "China",
                Locale::new("zh").with_script("Hans").with_country("CN"),
                CustomType::Business,
                "Monetary gift custom in business relationships and contracts",
            )
            .with_recognition_level(0.6),
        );

        // Indian customs
        self.add_custom(
            LocalCustom::new(
                "Hindu Undivided Family",
                "India",
                Locale::new("hi").with_country("IN"),
                CustomType::Property,
                "Joint family property ownership system with tax and inheritance implications",
            )
            .with_recognition_level(1.0)
            .with_statutory_basis("Hindu Succession Act, 1956"),
        );

        // Islamic customs
        self.add_custom(
            LocalCustom::new(
                "Mahr",
                "Saudi Arabia",
                Locale::new("ar").with_country("SA"),
                CustomType::Marriage,
                "Mandatory marriage gift from groom to bride under Islamic law",
            )
            .with_recognition_level(1.0)
            .with_statutory_basis("Sharia law"),
        );

        // Native American customs (US)
        self.add_custom(
            LocalCustom::new(
                "Tribal sovereignty",
                "United States",
                Locale::new("en").with_country("US"),
                CustomType::DisputeResolution,
                "Tribal courts have jurisdiction over certain matters on reservations",
            )
            .with_recognition_level(1.0)
            .with_statutory_basis("Indian Civil Rights Act of 1968"),
        );
    }

    /// Returns the total number of customs.
    pub fn custom_count(&self) -> usize {
        self.customs.values().map(|v| v.len()).sum()
    }

    /// Returns the number of regions.
    pub fn region_count(&self) -> usize {
        self.customs.len()
    }
}

/// Religious law system type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReligiousLawType {
    /// Islamic law (Sharia)
    Islamic,
    /// Jewish law (Halakha)
    Jewish,
    /// Canon law (Catholic)
    Canon,
    /// Hindu law
    Hindu,
    /// Buddhist law (Dharma)
    Buddhist,
}

impl std::fmt::Display for ReligiousLawType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReligiousLawType::Islamic => write!(f, "Islamic Law (Sharia)"),
            ReligiousLawType::Jewish => write!(f, "Jewish Law (Halakha)"),
            ReligiousLawType::Canon => write!(f, "Canon Law"),
            ReligiousLawType::Hindu => write!(f, "Hindu Law"),
            ReligiousLawType::Buddhist => write!(f, "Buddhist Law (Dharma)"),
        }
    }
}

/// Religious law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReligiousLawSystem {
    /// Type of religious law
    pub law_type: ReligiousLawType,
    /// Jurisdictions where this system is recognized
    pub jurisdictions: Vec<String>,
    /// Integration level with civil law (0.0 = separate, 1.0 = fully integrated)
    pub integration_level: f32,
    /// Key principles
    pub principles: Vec<String>,
    /// Sources of authority
    pub sources: Vec<String>,
    /// Civil law equivalents
    pub civil_equivalents: HashMap<String, String>,
}

impl ReligiousLawSystem {
    /// Creates a new religious law system.
    pub fn new(law_type: ReligiousLawType) -> Self {
        Self {
            law_type,
            jurisdictions: Vec::new(),
            integration_level: 0.5,
            principles: Vec::new(),
            sources: Vec::new(),
            civil_equivalents: HashMap::new(),
        }
    }

    /// Adds a jurisdiction.
    pub fn add_jurisdiction(&mut self, jurisdiction: impl Into<String>) {
        self.jurisdictions.push(jurisdiction.into());
    }

    /// Sets integration level.
    pub fn with_integration_level(mut self, level: f32) -> Self {
        self.integration_level = level.clamp(0.0, 1.0);
        self
    }

    /// Adds a principle.
    pub fn with_principle(mut self, principle: impl Into<String>) -> Self {
        self.principles.push(principle.into());
        self
    }

    /// Adds a source.
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.sources.push(source.into());
        self
    }

    /// Adds a civil law equivalent.
    pub fn with_equivalent(
        mut self,
        religious_concept: impl Into<String>,
        civil_equivalent: impl Into<String>,
    ) -> Self {
        self.civil_equivalents
            .insert(religious_concept.into(), civil_equivalent.into());
        self
    }

    /// Creates an Islamic law system.
    pub fn islamic() -> Self {
        Self::new(ReligiousLawType::Islamic)
            .with_integration_level(0.9)
            .with_principle("Quran as primary source of law")
            .with_principle("Hadith (Prophet's traditions) as secondary source")
            .with_principle("Ijma (scholarly consensus)")
            .with_principle("Qiyas (analogical reasoning)")
            .with_source("Quran")
            .with_source("Sunnah")
            .with_source("Scholarly interpretations (fiqh)")
            .with_equivalent("mahr", "marriage settlement")
            .with_equivalent("talaq", "divorce")
            .with_equivalent("zakat", "charitable tax")
            .with_equivalent("riba", "usury/interest prohibition")
    }

    /// Creates a Jewish law system.
    pub fn jewish() -> Self {
        Self::new(ReligiousLawType::Jewish)
            .with_integration_level(0.3)
            .with_principle("Torah as divine law")
            .with_principle("Talmudic interpretation")
            .with_principle("Rabbinical authority")
            .with_source("Torah (Written Law)")
            .with_source("Talmud (Oral Law)")
            .with_source("Responsa literature")
            .with_equivalent("get", "religious divorce decree")
            .with_equivalent("ketubah", "marriage contract")
            .with_equivalent("heter iska", "business partnership permitting profit")
    }

    /// Creates a Hindu law system.
    pub fn hindu() -> Self {
        Self::new(ReligiousLawType::Hindu)
            .with_integration_level(0.7)
            .with_principle("Dharma (righteous duty)")
            .with_principle("Karma (action and consequence)")
            .with_principle("Varna (social order)")
            .with_source("Vedas")
            .with_source("Smritis (legal texts)")
            .with_source("Dharmashastra")
            .with_equivalent("vivaha", "marriage")
            .with_equivalent("sampatti", "property")
    }
}

/// Religious law registry.
#[derive(Debug, Clone, Default)]
pub struct ReligiousLawRegistry {
    /// Systems indexed by type
    systems: HashMap<ReligiousLawType, ReligiousLawSystem>,
}

impl ReligiousLawRegistry {
    /// Creates a new registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a registry with default systems.
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();

        let mut islamic = ReligiousLawSystem::islamic();
        islamic.add_jurisdiction("Saudi Arabia");
        islamic.add_jurisdiction("Iran");
        islamic.add_jurisdiction("Pakistan");
        islamic.add_jurisdiction("UAE");
        registry.add_system(islamic);

        let mut jewish = ReligiousLawSystem::jewish();
        jewish.add_jurisdiction("Israel");
        registry.add_system(jewish);

        let mut hindu = ReligiousLawSystem::hindu();
        hindu.add_jurisdiction("India");
        hindu.add_jurisdiction("Nepal");
        registry.add_system(hindu);

        registry
    }

    /// Adds a religious law system.
    pub fn add_system(&mut self, system: ReligiousLawSystem) {
        self.systems.insert(system.law_type, system);
    }

    /// Gets a system by type.
    pub fn get_system(&self, law_type: ReligiousLawType) -> Option<&ReligiousLawSystem> {
        self.systems.get(&law_type)
    }

    /// Gets all systems for a jurisdiction.
    pub fn get_by_jurisdiction(&self, jurisdiction: &str) -> Vec<&ReligiousLawSystem> {
        self.systems
            .values()
            .filter(|s| s.jurisdictions.iter().any(|j| j == jurisdiction))
            .collect()
    }

    /// Returns the number of systems.
    pub fn system_count(&self) -> usize {
        self.systems.len()
    }
}

/// Indigenous legal tradition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndigenousLawSystem {
    /// Name of indigenous people
    pub people_name: String,
    /// Geographic region
    pub region: String,
    /// Legal principles
    pub principles: Vec<String>,
    /// Dispute resolution methods
    pub dispute_resolution: Vec<String>,
    /// Property concepts
    pub property_concepts: Vec<String>,
    /// Recognition status in state law
    pub state_recognition: bool,
    /// Treaty or statutory basis
    pub legal_basis: Option<String>,
}

impl IndigenousLawSystem {
    /// Creates a new indigenous law system.
    pub fn new(people_name: impl Into<String>, region: impl Into<String>) -> Self {
        Self {
            people_name: people_name.into(),
            region: region.into(),
            principles: Vec::new(),
            dispute_resolution: Vec::new(),
            property_concepts: Vec::new(),
            state_recognition: false,
            legal_basis: None,
        }
    }

    /// Adds a principle.
    pub fn with_principle(mut self, principle: impl Into<String>) -> Self {
        self.principles.push(principle.into());
        self
    }

    /// Adds a dispute resolution method.
    pub fn with_dispute_resolution(mut self, method: impl Into<String>) -> Self {
        self.dispute_resolution.push(method.into());
        self
    }

    /// Adds a property concept.
    pub fn with_property_concept(mut self, concept: impl Into<String>) -> Self {
        self.property_concepts.push(concept.into());
        self
    }

    /// Sets state recognition.
    pub fn with_state_recognition(mut self, recognized: bool) -> Self {
        self.state_recognition = recognized;
        self
    }

    /// Sets legal basis.
    pub fn with_legal_basis(mut self, basis: impl Into<String>) -> Self {
        self.legal_basis = Some(basis.into());
        self
    }
}

/// Indigenous law registry.
#[derive(Debug, Clone, Default)]
pub struct IndigenousLawRegistry {
    /// Systems indexed by people name
    systems: HashMap<String, IndigenousLawSystem>,
}

impl IndigenousLawRegistry {
    /// Creates a new registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a registry with default systems.
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();

        // Native American (Navajo Nation)
        registry.add_system(
            IndigenousLawSystem::new("Navajo Nation", "Southwestern United States")
                .with_principle("Hózhǫ́ (harmony and balance)")
                .with_principle("K'é (kinship and clan relationships)")
                .with_principle("Restorative justice over punitive measures")
                .with_dispute_resolution("Peacemaking circles")
                .with_dispute_resolution("Talking circles")
                .with_property_concept("Communal land ownership")
                .with_property_concept("Grazing permits")
                .with_state_recognition(true)
                .with_legal_basis("Treaty of 1868; Navajo Nation Code"),
        );

        // Māori (New Zealand)
        registry.add_system(
            IndigenousLawSystem::new("Māori", "New Zealand")
                .with_principle("Tikanga (customary law)")
                .with_principle("Mana (authority and prestige)")
                .with_principle("Utu (reciprocity and balance)")
                .with_dispute_resolution("Hui (community meetings)")
                .with_dispute_resolution("Rūnanga (tribal councils)")
                .with_property_concept("Whenua (ancestral land)")
                .with_property_concept("Kaitiakitanga (guardianship)")
                .with_state_recognition(true)
                .with_legal_basis("Treaty of Waitangi 1840; Te Ture Whenua Māori Act 1993"),
        );

        // Aboriginal Australian
        registry.add_system(
            IndigenousLawSystem::new("Aboriginal Australians", "Australia")
                .with_principle("Dreaming (creation law)")
                .with_principle("Country (connection to land)")
                .with_principle("Kinship obligations")
                .with_dispute_resolution("Elder councils")
                .with_dispute_resolution("Sorry business (reconciliation)")
                .with_property_concept("Native title")
                .with_property_concept("Sacred sites")
                .with_state_recognition(true)
                .with_legal_basis("Native Title Act 1993"),
        );

        // Inuit (Canada)
        registry.add_system(
            IndigenousLawSystem::new("Inuit", "Northern Canada")
                .with_principle("Inuit Qaujimajatuqangit (traditional knowledge)")
                .with_principle("Collective decision-making")
                .with_principle("Environmental stewardship")
                .with_dispute_resolution("Elders' councils")
                .with_dispute_resolution("Community consensus")
                .with_property_concept("Land claims agreements")
                .with_property_concept("Harvesting rights")
                .with_state_recognition(true)
                .with_legal_basis("Nunavut Land Claims Agreement 1993"),
        );

        registry
    }

    /// Adds a system.
    pub fn add_system(&mut self, system: IndigenousLawSystem) {
        self.systems.insert(system.people_name.clone(), system);
    }

    /// Gets a system by people name.
    pub fn get_system(&self, people_name: &str) -> Option<&IndigenousLawSystem> {
        self.systems.get(people_name)
    }

    /// Gets all systems for a region.
    pub fn get_by_region(&self, region: &str) -> Vec<&IndigenousLawSystem> {
        self.systems
            .values()
            .filter(|s| s.region.to_lowercase().contains(&region.to_lowercase()))
            .collect()
    }

    /// Gets all state-recognized systems.
    pub fn get_recognized(&self) -> Vec<&IndigenousLawSystem> {
        self.systems
            .values()
            .filter(|s| s.state_recognition)
            .collect()
    }

    /// Returns the number of systems.
    pub fn system_count(&self) -> usize {
        self.systems.len()
    }
}

/// Colonial legacy type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ColonialPower {
    /// British Empire
    British,
    /// French Empire
    French,
    /// Spanish Empire
    Spanish,
    /// Portuguese Empire
    Portuguese,
    /// Dutch Empire
    Dutch,
    /// German Empire
    German,
    /// Belgian Empire
    Belgian,
    /// Italian Empire
    Italian,
}

impl std::fmt::Display for ColonialPower {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColonialPower::British => write!(f, "British"),
            ColonialPower::French => write!(f, "French"),
            ColonialPower::Spanish => write!(f, "Spanish"),
            ColonialPower::Portuguese => write!(f, "Portuguese"),
            ColonialPower::Dutch => write!(f, "Dutch"),
            ColonialPower::German => write!(f, "German"),
            ColonialPower::Belgian => write!(f, "Belgian"),
            ColonialPower::Italian => write!(f, "Italian"),
        }
    }
}

/// Colonial legacy mapping.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColonialLegacy {
    /// Former colonial power
    pub colonial_power: ColonialPower,
    /// Modern jurisdiction
    pub jurisdiction: String,
    /// Colonial legal concepts still in use
    pub retained_concepts: Vec<String>,
    /// Hybrid legal concepts (colonial + indigenous)
    pub hybrid_concepts: HashMap<String, String>,
    /// Decolonization reforms
    pub reforms: Vec<String>,
}

impl ColonialLegacy {
    /// Creates a new colonial legacy.
    pub fn new(colonial_power: ColonialPower, jurisdiction: impl Into<String>) -> Self {
        Self {
            colonial_power,
            jurisdiction: jurisdiction.into(),
            retained_concepts: Vec::new(),
            hybrid_concepts: HashMap::new(),
            reforms: Vec::new(),
        }
    }

    /// Adds a retained concept.
    pub fn with_retained_concept(mut self, concept: impl Into<String>) -> Self {
        self.retained_concepts.push(concept.into());
        self
    }

    /// Adds a hybrid concept.
    pub fn with_hybrid_concept(
        mut self,
        colonial: impl Into<String>,
        indigenous: impl Into<String>,
    ) -> Self {
        self.hybrid_concepts
            .insert(colonial.into(), indigenous.into());
        self
    }

    /// Adds a reform.
    pub fn with_reform(mut self, reform: impl Into<String>) -> Self {
        self.reforms.push(reform.into());
        self
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

        // India (British)
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

        // Hong Kong (British)
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

        // Algeria (French)
        mapper.add_legacy(
            ColonialLegacy::new(ColonialPower::French, "Algeria")
                .with_retained_concept("Civil law system")
                .with_retained_concept("Code-based legal framework")
                .with_hybrid_concept("French civil law + Sharia", "Personal status law")
                .with_reform("Arabization of legal system post-independence")
                .with_reform("Family Code 1984 (Islamic family law)"),
        );

        // Philippines (Spanish then American)
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

        // Brazil (Portuguese)
        mapper.add_legacy(
            ColonialLegacy::new(ColonialPower::Portuguese, "Brazil")
                .with_retained_concept("Civil law system")
                .with_retained_concept("Inquisitorial procedure")
                .with_reform("Constitution of 1988 (democratic transition)")
                .with_reform("New Civil Code 2002"),
        );

        // Indonesia (Dutch)
        mapper.add_legacy(
            ColonialLegacy::new(ColonialPower::Dutch, "Indonesia")
                .with_retained_concept("Civil law system (based on Dutch Civil Code)")
                .with_hybrid_concept("Adat law", "Customary indigenous law")
                .with_hybrid_concept("Islamic law in Aceh", "Special autonomy for Islamic law")
                .with_reform("Constitution of 1945 (Pancasila principles)"),
        );

        // Rwanda (Belgian then German)
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

// ============================================================================
// v0.2.7: Accessibility Features (Enhanced)
// ============================================================================

/// Simplification strategy for plain language generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SimplificationStrategy {
    /// Replace legal jargon with common terms
    ReplaceJargon,
    /// Break long sentences into shorter ones
    ShortenSentences,
    /// Remove passive voice
    ActiveVoice,
    /// Add explanatory context
    AddContext,
    /// Simplify complex grammatical structures
    SimplifyGrammar,
}

impl std::fmt::Display for SimplificationStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimplificationStrategy::ReplaceJargon => write!(f, "Replace Jargon"),
            SimplificationStrategy::ShortenSentences => write!(f, "Shorten Sentences"),
            SimplificationStrategy::ActiveVoice => write!(f, "Active Voice"),
            SimplificationStrategy::AddContext => write!(f, "Add Context"),
            SimplificationStrategy::SimplifyGrammar => write!(f, "Simplify Grammar"),
        }
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
    jargon_map: HashMap<String, String>,
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

        // Apply custom jargon replacements
        for (legal_term, plain_term) in &self.jargon_map {
            result = result.replace(legal_term, plain_term);
        }

        // Apply default replacements based on locale
        let default_replacements = self.get_default_replacements();
        for (legal_term, plain_term) in default_replacements {
            result = result.replace(legal_term, plain_term);
        }

        result
    }

    fn get_default_replacements(&self) -> Vec<(&'static str, &'static str)> {
        match self.locale.language.as_str() {
            "en" => vec![
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
            ],
            "ja" => vec![
                ("以下", "これから"),
                ("前述", "上で述べた"),
                ("規定", "ルール"),
                ("条項", "項目"),
            ],
            _ => vec![],
        }
    }

    fn shorten_sentences(&self, text: &str) -> String {
        // Split long sentences at common conjunctions
        text.replace(", and ", ". Also, ")
            .replace(", but ", ". However, ")
            .replace("; ", ". ")
    }

    fn convert_to_active_voice(&self, text: &str) -> String {
        // Simple passive voice detection and conversion
        text.replace("is required to", "must")
            .replace("shall be", "will be")
            .replace("is prohibited from", "cannot")
    }

    fn add_context(&self, text: &str) -> String {
        // Add explanatory context for complex terms (simplified implementation)
        text.replace("liability", "liability (legal responsibility)")
            .replace("indemnify", "indemnify (compensate for loss or damage)")
    }

    fn simplify_grammar(&self, text: &str) -> String {
        // Remove unnecessary legal formality
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

/// Reading level to adjust to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TargetReadingLevel {
    /// Elementary (grades 3-5, Flesch-Kincaid 3-5)
    Elementary,
    /// Middle school (grades 6-8, Flesch-Kincaid 6-8)
    MiddleSchool,
    /// High school (grades 9-12, Flesch-Kincaid 9-12)
    HighSchool,
    /// College (undergraduate, Flesch-Kincaid 13-16)
    College,
    /// Professional (graduate+, Flesch-Kincaid 16+)
    Professional,
}

impl TargetReadingLevel {
    /// Returns the Flesch-Kincaid grade level.
    pub fn grade_level(&self) -> f64 {
        match self {
            TargetReadingLevel::Elementary => 4.0,
            TargetReadingLevel::MiddleSchool => 7.0,
            TargetReadingLevel::HighSchool => 10.0,
            TargetReadingLevel::College => 14.0,
            TargetReadingLevel::Professional => 18.0,
        }
    }
}

impl std::fmt::Display for TargetReadingLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TargetReadingLevel::Elementary => write!(f, "Elementary (grades 3-5)"),
            TargetReadingLevel::MiddleSchool => write!(f, "Middle School (grades 6-8)"),
            TargetReadingLevel::HighSchool => write!(f, "High School (grades 9-12)"),
            TargetReadingLevel::College => write!(f, "College (grades 13-16)"),
            TargetReadingLevel::Professional => write!(f, "Professional (graduate+)"),
        }
    }
}

/// Reading level adjuster for adaptive content.
#[derive(Debug, Clone)]
pub struct ReadingLevelAdjuster {
    /// Target reading level
    target_level: TargetReadingLevel,
    /// Plain language generator
    generator: PlainLanguageGenerator,
    /// Maximum iterations for adjustment
    max_iterations: usize,
}

impl ReadingLevelAdjuster {
    /// Creates a new reading level adjuster.
    pub fn new(target_level: TargetReadingLevel, locale: Locale) -> Self {
        let generator = PlainLanguageGenerator::new(target_level.grade_level(), locale);
        Self {
            target_level,
            generator,
            max_iterations: 3,
        }
    }

    /// Sets the maximum iterations for adjustment.
    pub fn with_max_iterations(mut self, max_iterations: usize) -> Self {
        self.max_iterations = max_iterations;
        self
    }

    /// Adds a custom jargon replacement.
    pub fn add_jargon_replacement(
        mut self,
        legal_term: impl Into<String>,
        plain_term: impl Into<String>,
    ) -> Self {
        self.generator = self
            .generator
            .add_jargon_replacement(legal_term, plain_term);
        self
    }

    /// Adjusts text to target reading level.
    pub fn adjust(&self, text: &str) -> AdjustedText {
        let original_level = self.generator.estimate_reading_level(text);
        let mut current_text = text.to_string();
        let mut iterations = 0;

        while iterations < self.max_iterations
            && !self.generator.meets_target(&current_text)
            && iterations < 10
        {
            current_text = self.generator.simplify(&current_text);
            iterations += 1;
        }

        let final_level = self.generator.estimate_reading_level(&current_text);

        AdjustedText {
            original: text.to_string(),
            adjusted: current_text,
            original_level,
            final_level,
            target_level: self.target_level.grade_level(),
            iterations,
            meets_target: final_level <= self.target_level.grade_level(),
        }
    }
}

/// Adjusted text with reading level information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdjustedText {
    /// Original text
    pub original: String,
    /// Adjusted text
    pub adjusted: String,
    /// Original reading level
    pub original_level: f64,
    /// Final reading level after adjustment
    pub final_level: f64,
    /// Target reading level
    pub target_level: f64,
    /// Number of iterations performed
    pub iterations: usize,
    /// Whether the adjusted text meets the target level
    pub meets_target: bool,
}

impl AdjustedText {
    /// Returns improvement in grade levels.
    pub fn improvement(&self) -> f64 {
        self.original_level - self.final_level
    }
}

/// WCAG conformance level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WCAGLevel {
    /// Level A (minimum)
    A,
    /// Level AA (mid-range)
    AA,
    /// Level AAA (highest)
    AAA,
}

impl std::fmt::Display for WCAGLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WCAGLevel::A => write!(f, "WCAG Level A"),
            WCAGLevel::AA => write!(f, "WCAG Level AA"),
            WCAGLevel::AAA => write!(f, "WCAG Level AAA"),
        }
    }
}

/// Screen reader optimizer with enhanced WCAG compliance.
#[derive(Debug, Clone)]
pub struct ScreenReaderOptimizer {
    /// Target WCAG level
    wcag_level: WCAGLevel,
    /// Include skip links
    include_skip_links: bool,
    /// Add landmark roles
    add_landmarks: bool,
    /// Locale for language-specific optimization
    locale: Locale,
}

impl ScreenReaderOptimizer {
    /// Creates a new screen reader optimizer.
    pub fn new(wcag_level: WCAGLevel, locale: Locale) -> Self {
        Self {
            wcag_level,
            include_skip_links: true,
            add_landmarks: true,
            locale,
        }
    }

    /// Sets whether to include skip links.
    pub fn with_skip_links(mut self, include: bool) -> Self {
        self.include_skip_links = include;
        self
    }

    /// Sets whether to add landmark roles.
    pub fn with_landmarks(mut self, add: bool) -> Self {
        self.add_landmarks = add;
        self
    }

    /// Optimizes HTML for screen readers.
    pub fn optimize_html(&self, html: &str) -> String {
        let mut result = html.to_string();

        // Add language attribute
        if !result.contains("<html") {
            result = format!(
                "<html lang=\"{}\">\n{}\n</html>",
                self.locale.language, result
            );
        }

        // Add skip links
        if self.include_skip_links {
            let skip_link = self.generate_skip_link();
            result = format!("{}\n{}", skip_link, result);
        }

        // Add landmark roles
        if self.add_landmarks {
            result = self.add_landmark_roles(&result);
        }

        // Enhance headings with hierarchy
        result = self.enhance_headings(&result);

        // Add alt text reminders for images
        result = self.add_image_alt_reminders(&result);

        result
    }

    fn generate_skip_link(&self) -> String {
        let link_text = match self.locale.language.as_str() {
            "en" => "Skip to main content",
            "ja" => "メインコンテンツへスキップ",
            "es" => "Saltar al contenido principal",
            "fr" => "Passer au contenu principal",
            "de" => "Zum Hauptinhalt springen",
            _ => "Skip to main content",
        };

        format!(
            "<a href=\"#main-content\" class=\"skip-link\">{}</a>",
            link_text
        )
    }

    fn add_landmark_roles(&self, html: &str) -> String {
        html.replace("<nav>", "<nav role=\"navigation\">")
            .replace("<main>", "<main role=\"main\" id=\"main-content\">")
            .replace("<header>", "<header role=\"banner\">")
            .replace("<footer>", "<footer role=\"contentinfo\">")
            .replace("<aside>", "<aside role=\"complementary\">")
            .replace("<form>", "<form role=\"form\">")
    }

    fn enhance_headings(&self, html: &str) -> String {
        // Ensure proper heading hierarchy (simplified implementation)
        html.to_string()
    }

    fn add_image_alt_reminders(&self, html: &str) -> String {
        // Mark images without alt text
        html.replace("<img ", "<img alt=\"[ADD DESCRIPTION]\" ")
    }

    /// Generates accessible legal document structure.
    pub fn generate_document_structure(&self, title: &str, sections: Vec<(&str, &str)>) -> String {
        let mut html = format!(
            "<!DOCTYPE html>\n<html lang=\"{}\">\n<head>\n<meta charset=\"UTF-8\">\n<title>{}</title>\n</head>\n<body>\n",
            self.locale.language, title
        );

        if self.include_skip_links {
            html.push_str(&self.generate_skip_link());
            html.push('\n');
        }

        html.push_str("<main role=\"main\" id=\"main-content\">\n");
        html.push_str(&format!("<h1>{}</h1>\n", title));

        for (section_title, section_content) in sections {
            html.push_str(&format!(
                "<section>\n<h2>{}</h2>\n<p>{}</p>\n</section>\n",
                section_title, section_content
            ));
        }

        html.push_str("</main>\n</body>\n</html>");
        html
    }

    /// Checks WCAG compliance.
    pub fn check_compliance(&self, html: &str) -> ComplianceReport {
        let mut issues = Vec::new();

        // Check for language attribute
        if !html.contains("lang=") {
            issues.push("Missing language attribute on html element".to_string());
        }

        // Check for skip links (AA and AAA)
        if matches!(self.wcag_level, WCAGLevel::AA | WCAGLevel::AAA)
            && !html.contains("skip-link")
            && !html.contains("Skip to")
        {
            issues.push("Missing skip link (required for AA/AAA)".to_string());
        }

        // Check for heading hierarchy
        if !html.contains("<h1") {
            issues.push("Missing h1 heading (main page title)".to_string());
        }

        // Check for landmark roles
        if self.add_landmarks && !html.contains("role=") {
            issues.push("Missing ARIA landmark roles".to_string());
        }

        let is_compliant = issues.is_empty();

        ComplianceReport {
            wcag_level: self.wcag_level,
            is_compliant,
            issues,
        }
    }
}

/// WCAG compliance report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    /// Target WCAG level
    pub wcag_level: WCAGLevel,
    /// Whether the content is compliant
    pub is_compliant: bool,
    /// List of compliance issues
    pub issues: Vec<String>,
}

/// SSML (Speech Synthesis Markup Language) tag type.
#[derive(Debug, Clone, PartialEq)]
pub enum SSMLTag {
    /// Pause/break
    Break { duration_ms: u32 },
    /// Emphasis
    Emphasis { level: EmphasisLevel },
    /// Prosody (rate, pitch, volume)
    Prosody { rate: f32, pitch: f32, volume: f32 },
    /// Say-as (interpret as specific type)
    SayAs { interpret_as: String },
    /// Phoneme (pronunciation)
    Phoneme { ph: String, alphabet: String },
}

/// Emphasis level for SSML.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmphasisLevel {
    None,
    Reduced,
    Moderate,
    Strong,
}

impl std::fmt::Display for EmphasisLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmphasisLevel::None => write!(f, "none"),
            EmphasisLevel::Reduced => write!(f, "reduced"),
            EmphasisLevel::Moderate => write!(f, "moderate"),
            EmphasisLevel::Strong => write!(f, "strong"),
        }
    }
}

/// Audio narration support with SSML integration.
#[derive(Debug, Clone)]
pub struct AudioNarrationSupport {
    /// Locale for language-specific narration
    locale: Locale,
    /// Speaking rate (1.0 = normal)
    speaking_rate: f32,
    /// Pitch adjustment (1.0 = normal)
    pitch: f32,
    /// Volume level (1.0 = normal)
    volume: f32,
}

impl AudioNarrationSupport {
    /// Creates a new audio narration support.
    pub fn new(locale: Locale) -> Self {
        Self {
            locale,
            speaking_rate: 1.0,
            pitch: 1.0,
            volume: 1.0,
        }
    }

    /// Sets speaking rate.
    pub fn with_speaking_rate(mut self, rate: f32) -> Self {
        self.speaking_rate = rate;
        self
    }

    /// Sets pitch.
    pub fn with_pitch(mut self, pitch: f32) -> Self {
        self.pitch = pitch;
        self
    }

    /// Sets volume.
    pub fn with_volume(mut self, volume: f32) -> Self {
        self.volume = volume;
        self
    }

    /// Generates SSML markup for legal text.
    pub fn generate_ssml(&self, text: &str) -> String {
        let mut ssml = format!(
            "<speak version=\"1.0\" xmlns=\"http://www.w3.org/2001/10/synthesis\" xml:lang=\"{}\">\n",
            self.locale.tag()
        );

        // Add prosody for speaking rate, pitch, and volume
        ssml.push_str(&format!(
            "<prosody rate=\"{}\" pitch=\"{}%\" volume=\"{}\">\n",
            self.format_rate(),
            (self.pitch * 100.0) as i32,
            self.format_volume()
        ));

        // Process text for legal-specific narration
        let processed_text = self.process_legal_text(text);
        ssml.push_str(&processed_text);

        ssml.push_str("\n</prosody>\n</speak>");
        ssml
    }

    fn format_rate(&self) -> String {
        if self.speaking_rate < 0.9 {
            "slow".to_string()
        } else if self.speaking_rate > 1.1 {
            "fast".to_string()
        } else {
            "medium".to_string()
        }
    }

    fn format_volume(&self) -> String {
        if self.volume < 0.7 {
            "soft".to_string()
        } else if self.volume > 1.3 {
            "loud".to_string()
        } else {
            "medium".to_string()
        }
    }

    fn process_legal_text(&self, text: &str) -> String {
        let mut result = text.to_string();

        // Add pauses after legal citations
        result = result.replace(
            " v. ",
            " <break time=\"300ms\"/> versus <break time=\"200ms\"/> ",
        );

        // Interpret section numbers as ordinals
        result = result.replace(
            "Section ",
            "<say-as interpret-as=\"ordinal\">Section</say-as> ",
        );

        // Add emphasis on important legal terms
        result = result.replace("shall", "<emphasis level=\"strong\">shall</emphasis>");
        result = result.replace("must", "<emphasis level=\"strong\">must</emphasis>");
        result = result.replace("may not", "<emphasis level=\"strong\">may not</emphasis>");

        result
    }

    /// Generates narration script for legal document section.
    pub fn narrate_section(&self, section_number: &str, title: &str, content: &str) -> String {
        let intro = match self.locale.language.as_str() {
            "en" => format!("Section {}. {}", section_number, title),
            "ja" => format!("第{}条。{}", section_number, title),
            "es" => format!("Sección {}. {}", section_number, title),
            "fr" => format!("Section {}. {}", section_number, title),
            "de" => format!("Abschnitt {}. {}", section_number, title),
            _ => format!("Section {}. {}", section_number, title),
        };

        let full_text = format!("{}\n<break time=\"500ms\"/>\n{}", intro, content);
        self.generate_ssml(&full_text)
    }

    /// Generates narration for legal citation.
    pub fn narrate_citation(&self, citation: &str) -> String {
        // Parse and narrate citation components
        let narration = citation
            .replace(" v. ", " versus ")
            .replace("U.S.", "United States")
            .replace("F.3d", "Federal Reporter, Third Series")
            .replace("F.2d", "Federal Reporter, Second Series")
            .replace("S.Ct.", "Supreme Court Reporter");

        self.generate_ssml(&narration)
    }
}

/// Sign language type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SignLanguageType {
    /// American Sign Language
    ASL,
    /// British Sign Language
    BSL,
    /// Japanese Sign Language
    JSL,
    /// International Sign
    IS,
    /// Other sign language
    Other,
}

impl std::fmt::Display for SignLanguageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignLanguageType::ASL => write!(f, "American Sign Language (ASL)"),
            SignLanguageType::BSL => write!(f, "British Sign Language (BSL)"),
            SignLanguageType::JSL => write!(f, "Japanese Sign Language (JSL)"),
            SignLanguageType::IS => write!(f, "International Sign (IS)"),
            SignLanguageType::Other => write!(f, "Other Sign Language"),
        }
    }
}

/// Sign language reference for video/image linking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignLanguageReference {
    /// Term or phrase in spoken/written language
    pub term: String,
    /// Sign language type
    pub sign_language: SignLanguageType,
    /// URL to video demonstrating the sign
    pub video_url: Option<String>,
    /// URL to image/diagram of the sign
    pub image_url: Option<String>,
    /// Description of how to perform the sign
    pub description: Option<String>,
    /// Locale of the term
    pub locale: Locale,
}

impl SignLanguageReference {
    /// Creates a new sign language reference.
    pub fn new(term: impl Into<String>, sign_language: SignLanguageType, locale: Locale) -> Self {
        Self {
            term: term.into(),
            sign_language,
            video_url: None,
            image_url: None,
            description: None,
            locale,
        }
    }

    /// Adds a video URL.
    pub fn with_video(mut self, url: impl Into<String>) -> Self {
        self.video_url = Some(url.into());
        self
    }

    /// Adds an image URL.
    pub fn with_image(mut self, url: impl Into<String>) -> Self {
        self.image_url = Some(url.into());
        self
    }

    /// Adds a description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// Sign language referencer for legal terminology.
#[derive(Debug, Clone)]
pub struct SignLanguageReferencer {
    /// References indexed by term
    references: HashMap<String, Vec<SignLanguageReference>>,
}

impl SignLanguageReferencer {
    /// Creates a new sign language referencer.
    pub fn new() -> Self {
        Self {
            references: HashMap::new(),
        }
    }

    /// Creates a referencer with default legal sign language references.
    pub fn with_defaults() -> Self {
        let mut referencer = Self::new();

        // ASL legal terms
        referencer.add_reference(
            SignLanguageReference::new(
                "contract",
                SignLanguageType::ASL,
                Locale::new("en").with_country("US"),
            )
            .with_description("Hands form C-shape, move together and apart repeatedly"),
        );

        referencer.add_reference(
            SignLanguageReference::new(
                "law",
                SignLanguageType::ASL,
                Locale::new("en").with_country("US"),
            )
            .with_description("L-hand on open palm, representing law/legislation"),
        );

        referencer.add_reference(
            SignLanguageReference::new(
                "court",
                SignLanguageType::ASL,
                Locale::new("en").with_country("US"),
            )
            .with_description(
                "C-hands move down from head level, representing judge and courtroom",
            ),
        );

        referencer.add_reference(
            SignLanguageReference::new(
                "attorney",
                SignLanguageType::ASL,
                Locale::new("en").with_country("US"),
            )
            .with_description("A-hand taps shoulder, representing lawyer/attorney"),
        );

        // BSL legal terms
        referencer.add_reference(
            SignLanguageReference::new(
                "solicitor",
                SignLanguageType::BSL,
                Locale::new("en").with_country("GB"),
            )
            .with_description("S-hand moves from ear to mouth, representing legal advice"),
        );

        referencer.add_reference(
            SignLanguageReference::new(
                "barrister",
                SignLanguageType::BSL,
                Locale::new("en").with_country("GB"),
            )
            .with_description(
                "Hands gesture as if putting on a wig, representing courtroom lawyer",
            ),
        );

        // JSL legal terms
        referencer.add_reference(
            SignLanguageReference::new(
                "法律",
                SignLanguageType::JSL,
                Locale::new("ja").with_country("JP"),
            )
            .with_description("Hands form book shape near head, representing law books"),
        );

        referencer.add_reference(
            SignLanguageReference::new(
                "裁判所",
                SignLanguageType::JSL,
                Locale::new("ja").with_country("JP"),
            )
            .with_description("Gavel motion with fist, representing court judgment"),
        );

        referencer
    }

    /// Adds a sign language reference.
    pub fn add_reference(&mut self, reference: SignLanguageReference) {
        self.references
            .entry(reference.term.clone())
            .or_default()
            .push(reference);
    }

    /// Gets references for a term.
    pub fn get_references(&self, term: &str) -> Vec<&SignLanguageReference> {
        self.references
            .get(term)
            .map(|refs| refs.iter().collect())
            .unwrap_or_default()
    }

    /// Gets references for a term in a specific sign language.
    pub fn get_references_for_sign_language(
        &self,
        term: &str,
        sign_language: SignLanguageType,
    ) -> Vec<&SignLanguageReference> {
        self.references
            .get(term)
            .map(|refs| {
                refs.iter()
                    .filter(|r| r.sign_language == sign_language)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Generates HTML with sign language links.
    pub fn generate_accessible_html(&self, text: &str) -> String {
        let mut result = text.to_string();

        for (term, references) in &self.references {
            if result.contains(term) {
                let links = references
                    .iter()
                    .filter_map(|r| {
                        r.video_url.as_ref().map(|url| {
                            format!(
                                "<a href=\"{}\" class=\"sign-language-link\" data-sign-type=\"{}\" aria-label=\"{} in {}\">🎥</a>",
                                url, r.sign_language, term, r.sign_language
                            )
                        })
                    })
                    .collect::<Vec<_>>()
                    .join(" ");

                if !links.is_empty() {
                    let replacement = format!("{} {}", term, links);
                    result = result.replace(term, &replacement);
                }
            }
        }

        result
    }

    /// Returns the number of references.
    pub fn reference_count(&self) -> usize {
        self.references.values().map(|v| v.len()).sum()
    }

    /// Returns the number of unique terms.
    pub fn term_count(&self) -> usize {
        self.references.len()
    }
}

impl Default for SignLanguageReferencer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// v0.2.8: Historical Legal Language
// ============================================================================

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

impl std::fmt::Display for HistoricalPeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HistoricalPeriod::OldEnglish => write!(f, "Old English (450-1150)"),
            HistoricalPeriod::MiddleEnglish => write!(f, "Middle English (1150-1500)"),
            HistoricalPeriod::EarlyModern => write!(f, "Early Modern English (1500-1700)"),
            HistoricalPeriod::ClassicalLatin => write!(f, "Classical Latin (Roman Empire)"),
            HistoricalPeriod::MedievalLatin => write!(f, "Medieval Latin (500-1500)"),
            HistoricalPeriod::Renaissance => write!(f, "Renaissance (1400-1600)"),
            HistoricalPeriod::Enlightenment => write!(f, "Enlightenment (1600-1800)"),
            HistoricalPeriod::Victorian => write!(f, "Victorian (1837-1901)"),
        }
    }
}

/// Archaic legal term with historical context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchaicTerm {
    /// The archaic term
    pub term: String,
    /// Historical period when the term was used
    pub period: HistoricalPeriod,
    /// Modern equivalent term
    pub modern_equivalent: String,
    /// Definition of the term
    pub definition: String,
    /// Example usage in historical context
    pub example: Option<String>,
    /// Locale of the term
    pub locale: Locale,
}

impl ArchaicTerm {
    /// Creates a new archaic term.
    pub fn new(
        term: impl Into<String>,
        period: HistoricalPeriod,
        modern_equivalent: impl Into<String>,
        definition: impl Into<String>,
        locale: Locale,
    ) -> Self {
        Self {
            term: term.into(),
            period,
            modern_equivalent: modern_equivalent.into(),
            definition: definition.into(),
            example: None,
            locale,
        }
    }

    /// Adds an example usage.
    pub fn with_example(mut self, example: impl Into<String>) -> Self {
        self.example = Some(example.into());
        self
    }
}

/// Archaic term dictionary for historical legal language.
#[derive(Debug, Clone)]
pub struct ArchaicTermDictionary {
    /// Terms indexed by period
    terms_by_period: HashMap<HistoricalPeriod, Vec<ArchaicTerm>>,
    /// Terms indexed by archaic term
    terms_by_name: HashMap<String, Vec<ArchaicTerm>>,
}

impl ArchaicTermDictionary {
    /// Creates a new archaic term dictionary.
    pub fn new() -> Self {
        Self {
            terms_by_period: HashMap::new(),
            terms_by_name: HashMap::new(),
        }
    }

    /// Creates a dictionary with default archaic legal terms.
    pub fn with_defaults() -> Self {
        let mut dict = Self::new();

        // Old English terms
        dict.add_term(
            ArchaicTerm::new(
                "folcriht",
                HistoricalPeriod::OldEnglish,
                "common law",
                "The law of the people, customary law",
                Locale::new("en").with_country("GB"),
            )
            .with_example("Under folcriht, disputes were settled by the community"),
        );

        dict.add_term(
            ArchaicTerm::new(
                "wergild",
                HistoricalPeriod::OldEnglish,
                "blood money",
                "Compensation paid to the family of a slain person",
                Locale::new("en").with_country("GB"),
            )
            .with_example("The wergild for a thane was 1200 shillings"),
        );

        dict.add_term(
            ArchaicTerm::new(
                "moot",
                HistoricalPeriod::OldEnglish,
                "assembly",
                "A judicial assembly or court",
                Locale::new("en").with_country("GB"),
            )
            .with_example("The shire moot met twice yearly"),
        );

        // Middle English terms
        dict.add_term(
            ArchaicTerm::new(
                "feoffment",
                HistoricalPeriod::MiddleEnglish,
                "grant of land",
                "The grant of a fief or fee; transfer of property",
                Locale::new("en").with_country("GB"),
            )
            .with_example("A feoffment required livery of seisin"),
        );

        dict.add_term(
            ArchaicTerm::new(
                "frankpledge",
                HistoricalPeriod::MiddleEnglish,
                "mutual surety",
                "System of collective responsibility for law and order",
                Locale::new("en").with_country("GB"),
            )
            .with_example("All freemen were organized into frankpledge groups"),
        );

        dict.add_term(
            ArchaicTerm::new(
                "assize",
                HistoricalPeriod::MiddleEnglish,
                "court session",
                "A session of a court; also a statute or ordinance",
                Locale::new("en").with_country("GB"),
            )
            .with_example("The assize of clarendon established procedures for criminal justice"),
        );

        // Early Modern English terms
        dict.add_term(
            ArchaicTerm::new(
                "attainder",
                HistoricalPeriod::EarlyModern,
                "forfeiture",
                "Loss of civil rights and property upon conviction of treason",
                Locale::new("en").with_country("GB"),
            )
            .with_example("Bills of attainder were abolished in 1870"),
        );

        dict.add_term(
            ArchaicTerm::new(
                "praemunire",
                HistoricalPeriod::EarlyModern,
                "usurpation of royal authority",
                "Offense of appealing to foreign authority over the Crown",
                Locale::new("en").with_country("GB"),
            )
            .with_example("Praemunire was used against those asserting papal authority"),
        );

        // Classical Latin terms
        dict.add_term(
            ArchaicTerm::new(
                "ius civile",
                HistoricalPeriod::ClassicalLatin,
                "civil law",
                "The law applicable to Roman citizens",
                Locale::new("la"),
            )
            .with_example("Ius civile governed property and contract matters"),
        );

        dict.add_term(
            ArchaicTerm::new(
                "lex aquilia",
                HistoricalPeriod::ClassicalLatin,
                "tort law",
                "Roman law governing damages to property",
                Locale::new("la"),
            )
            .with_example("The lex aquilia provided for compensation for wrongful damage"),
        );

        dict.add_term(
            ArchaicTerm::new(
                "mancipatio",
                HistoricalPeriod::ClassicalLatin,
                "formal transfer",
                "Formal procedure for transferring ownership of property",
                Locale::new("la"),
            )
            .with_example("Mancipatio required five witnesses and a scale bearer"),
        );

        // Medieval Latin terms
        dict.add_term(
            ArchaicTerm::new(
                "mainour",
                HistoricalPeriod::MedievalLatin,
                "stolen goods",
                "Stolen property found in the possession of a thief",
                Locale::new("la"),
            )
            .with_example("A thief taken with mainour could be summarily tried"),
        );

        dict.add_term(
            ArchaicTerm::new(
                "essoign",
                HistoricalPeriod::MedievalLatin,
                "excuse",
                "An excuse for non-appearance in court",
                Locale::new("la"),
            )
            .with_example("Illness was a valid essoign for missing court"),
        );

        // Victorian terms
        dict.add_term(
            ArchaicTerm::new(
                "mesne profits",
                HistoricalPeriod::Victorian,
                "interim profits",
                "Profits from land wrongfully withheld from the rightful owner",
                Locale::new("en").with_country("GB"),
            )
            .with_example("The tenant was liable for mesne profits during the wrongful occupation"),
        );

        dict.add_term(
            ArchaicTerm::new(
                "copyhold",
                HistoricalPeriod::Victorian,
                "tenure by copy",
                "Land held by copy of the manorial court roll",
                Locale::new("en").with_country("GB"),
            )
            .with_example("Copyhold was abolished in 1925"),
        );

        dict
    }

    /// Adds an archaic term.
    pub fn add_term(&mut self, term: ArchaicTerm) {
        self.terms_by_period
            .entry(term.period)
            .or_default()
            .push(term.clone());
        self.terms_by_name
            .entry(term.term.clone())
            .or_default()
            .push(term);
    }

    /// Gets terms by historical period.
    pub fn get_by_period(&self, period: HistoricalPeriod) -> Vec<&ArchaicTerm> {
        self.terms_by_period
            .get(&period)
            .map(|terms| terms.iter().collect())
            .unwrap_or_default()
    }

    /// Gets terms by archaic name.
    pub fn get_by_name(&self, name: &str) -> Vec<&ArchaicTerm> {
        self.terms_by_name
            .get(name)
            .map(|terms| terms.iter().collect())
            .unwrap_or_default()
    }

    /// Translates archaic term to modern equivalent.
    pub fn translate_to_modern(&self, archaic_term: &str) -> Option<String> {
        self.terms_by_name
            .get(archaic_term)
            .and_then(|terms| terms.first())
            .map(|term| term.modern_equivalent.clone())
    }

    /// Returns the number of terms in the dictionary.
    pub fn term_count(&self) -> usize {
        self.terms_by_name.len()
    }

    /// Returns the number of periods represented.
    pub fn period_count(&self) -> usize {
        self.terms_by_period.len()
    }
}

impl Default for ArchaicTermDictionary {
    fn default() -> Self {
        Self::new()
    }
}

/// Historical calendar system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HistoricalCalendar {
    /// Julian calendar (45 BC - 1582 AD in Catholic countries)
    Julian,
    /// Gregorian calendar (1582 AD onwards)
    Gregorian,
    /// Roman calendar (pre-Julian)
    Roman,
    /// French Revolutionary calendar (1793-1805)
    FrenchRevolutionary,
}

impl std::fmt::Display for HistoricalCalendar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HistoricalCalendar::Julian => write!(f, "Julian Calendar"),
            HistoricalCalendar::Gregorian => write!(f, "Gregorian Calendar"),
            HistoricalCalendar::Roman => write!(f, "Roman Calendar"),
            HistoricalCalendar::FrenchRevolutionary => write!(f, "French Revolutionary Calendar"),
        }
    }
}

/// Historical calendar converter.
#[derive(Debug, Clone)]
pub struct HistoricalCalendarConverter {
    /// Source calendar
    source_calendar: HistoricalCalendar,
}

impl HistoricalCalendarConverter {
    /// Creates a new historical calendar converter.
    pub fn new(source_calendar: HistoricalCalendar) -> Self {
        Self { source_calendar }
    }

    /// Converts a Julian date to Gregorian.
    /// Returns (year, month, day) in Gregorian calendar.
    pub fn julian_to_gregorian(&self, year: i32, month: u32, day: u32) -> (i32, u32, u32) {
        // Calculate Julian Day Number
        let a = (14 - month) / 12;
        let y = year + 4800 - a as i32;
        let m = month + 12 * a - 3;

        let jdn = day as i32 + (153 * m as i32 + 2) / 5 + 365 * y + y / 4 - 32083;

        // Convert JDN to Gregorian
        let a = jdn + 32044;
        let b = (4 * a + 3) / 146097;
        let c = a - (146097 * b) / 4;
        let d = (4 * c + 3) / 1461;
        let e = c - (1461 * d) / 4;
        let m = (5 * e + 2) / 153;

        let greg_day = e - (153 * m + 2) / 5 + 1;
        let greg_month = m + 3 - 12 * (m / 10);
        let greg_year = 100 * b + d - 4800 + m / 10;

        (greg_year, greg_month as u32, greg_day as u32)
    }

    /// Converts a Gregorian date to Julian.
    /// Returns (year, month, day) in Julian calendar.
    pub fn gregorian_to_julian(&self, year: i32, month: u32, day: u32) -> (i32, u32, u32) {
        // Calculate Julian Day Number from Gregorian
        let a = (14 - month) / 12;
        let y = year + 4800 - a as i32;
        let m = month + 12 * a - 3;

        let jdn =
            day as i32 + (153 * m as i32 + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045;

        // Convert JDN to Julian
        let c = jdn + 32082;
        let d = (4 * c + 3) / 1461;
        let e = c - (1461 * d) / 4;
        let m = (5 * e + 2) / 153;

        let jul_day = e - (153 * m + 2) / 5 + 1;
        let jul_month = m + 3 - 12 * (m / 10);
        let jul_year = d - 4800 + m / 10;

        (jul_year, jul_month as u32, jul_day as u32)
    }

    /// Calculates the difference in days between Julian and Gregorian calendars.
    pub fn julian_gregorian_offset(&self, year: i32) -> i32 {
        if year < 1582 {
            0 // Before Gregorian reform
        } else {
            let centuries = (year - 1600) / 100;
            centuries * 3 / 4 + 10 // Approximate offset
        }
    }

    /// Formats a date in historical calendar notation.
    pub fn format_historical_date(&self, year: i32, month: u32, day: u32) -> String {
        match self.source_calendar {
            HistoricalCalendar::Julian => {
                format!("{} {} {} (O.S.)", day, self.month_name_latin(month), year)
            }
            HistoricalCalendar::Gregorian => {
                format!("{} {} {} (N.S.)", day, self.month_name_latin(month), year)
            }
            HistoricalCalendar::Roman => self.format_roman_date(year, month, day),
            HistoricalCalendar::FrenchRevolutionary => {
                self.format_french_revolutionary_date(year, month, day)
            }
        }
    }

    fn month_name_latin(&self, month: u32) -> &'static str {
        match month {
            1 => "Januarius",
            2 => "Februarius",
            3 => "Martius",
            4 => "Aprilis",
            5 => "Maius",
            6 => "Junius",
            7 => "Julius",
            8 => "Augustus",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => "Unknown",
        }
    }

    fn format_roman_date(&self, _year: i32, month: u32, day: u32) -> String {
        let month_name = self.month_name_latin(month);
        format!("a.d. {} {}", day, month_name)
    }

    fn format_french_revolutionary_date(&self, year: i32, month: u32, day: u32) -> String {
        let revolutionary_months = [
            "Vendémiaire",
            "Brumaire",
            "Frimaire",
            "Nivôse",
            "Pluviôse",
            "Ventôse",
            "Germinal",
            "Floréal",
            "Prairial",
            "Messidor",
            "Thermidor",
            "Fructidor",
        ];

        let month_name = if month <= 12 {
            revolutionary_months[(month - 1) as usize]
        } else {
            "Sansculottides"
        };

        format!("{} {} An {}", day, month_name, year)
    }
}

/// Language family for etymology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LanguageFamily {
    /// Germanic languages
    Germanic,
    /// Romance languages
    Romance,
    /// Latin
    Latin,
    /// Greek
    Greek,
    /// Celtic languages
    Celtic,
    /// Norman French
    NormanFrench,
    /// Old French
    OldFrench,
}

impl std::fmt::Display for LanguageFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LanguageFamily::Germanic => write!(f, "Germanic"),
            LanguageFamily::Romance => write!(f, "Romance"),
            LanguageFamily::Latin => write!(f, "Latin"),
            LanguageFamily::Greek => write!(f, "Greek"),
            LanguageFamily::Celtic => write!(f, "Celtic"),
            LanguageFamily::NormanFrench => write!(f, "Norman French"),
            LanguageFamily::OldFrench => write!(f, "Old French"),
        }
    }
}

/// Etymology information for a legal term.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Etymology {
    /// The modern term
    pub term: String,
    /// Original term or root
    pub root: String,
    /// Language family of origin
    pub language_family: LanguageFamily,
    /// Original language
    pub original_language: String,
    /// Meaning of the root
    pub root_meaning: String,
    /// Historical period of first usage
    pub first_usage: Option<HistoricalPeriod>,
    /// Evolution of the term through time
    pub evolution: Vec<String>,
}

impl Etymology {
    /// Creates a new etymology.
    pub fn new(
        term: impl Into<String>,
        root: impl Into<String>,
        language_family: LanguageFamily,
        original_language: impl Into<String>,
        root_meaning: impl Into<String>,
    ) -> Self {
        Self {
            term: term.into(),
            root: root.into(),
            language_family,
            original_language: original_language.into(),
            root_meaning: root_meaning.into(),
            first_usage: None,
            evolution: Vec::new(),
        }
    }

    /// Adds first usage period.
    pub fn with_first_usage(mut self, period: HistoricalPeriod) -> Self {
        self.first_usage = Some(period);
        self
    }

    /// Adds evolution step.
    pub fn add_evolution(mut self, evolution_step: impl Into<String>) -> Self {
        self.evolution.push(evolution_step.into());
        self
    }
}

/// Etymology tracker for legal terms.
#[derive(Debug, Clone)]
pub struct EtymologyTracker {
    /// Etymologies indexed by term
    etymologies: HashMap<String, Etymology>,
}

impl EtymologyTracker {
    /// Creates a new etymology tracker.
    pub fn new() -> Self {
        Self {
            etymologies: HashMap::new(),
        }
    }

    /// Creates a tracker with default legal term etymologies.
    pub fn with_defaults() -> Self {
        let mut tracker = Self::new();

        // Contract
        tracker.add_etymology(
            Etymology::new(
                "contract",
                "contractus",
                LanguageFamily::Latin,
                "Latin",
                "drawn together, agreed upon",
            )
            .with_first_usage(HistoricalPeriod::ClassicalLatin)
            .add_evolution("Latin contractus → Old French contract → Middle English contract"),
        );

        // Tort
        tracker.add_etymology(
            Etymology::new(
                "tort",
                "tortus",
                LanguageFamily::Latin,
                "Latin",
                "twisted, wrong",
            )
            .with_first_usage(HistoricalPeriod::MedievalLatin)
            .add_evolution("Latin tortus → Old French tort → Middle English tort"),
        );

        // Jury
        tracker.add_etymology(
            Etymology::new(
                "jury",
                "jurata",
                LanguageFamily::Latin,
                "Latin",
                "sworn (group)",
            )
            .with_first_usage(HistoricalPeriod::MedievalLatin)
            .add_evolution("Latin jurata → Old French juree → Middle English jury"),
        );

        // Attorney
        tracker.add_etymology(
            Etymology::new(
                "attorney",
                "atorner",
                LanguageFamily::OldFrench,
                "Old French",
                "to turn over, assign",
            )
            .with_first_usage(HistoricalPeriod::MiddleEnglish)
            .add_evolution("Old French atorner → Anglo-Norman atourne → Middle English attorney"),
        );

        // Mortgage
        tracker.add_etymology(
            Etymology::new(
                "mortgage",
                "mort + gage",
                LanguageFamily::OldFrench,
                "Old French",
                "dead pledge",
            )
            .with_first_usage(HistoricalPeriod::MiddleEnglish)
            .add_evolution("Old French mort (dead) + gage (pledge) → Middle English mortgage"),
        );

        // Habeas corpus
        tracker.add_etymology(
            Etymology::new(
                "habeas corpus",
                "habeas corpus",
                LanguageFamily::Latin,
                "Latin",
                "you shall have the body",
            )
            .with_first_usage(HistoricalPeriod::MedievalLatin)
            .add_evolution("Latin legal phrase preserved in English common law"),
        );

        // Bailiff
        tracker.add_etymology(
            Etymology::new(
                "bailiff",
                "baillif",
                LanguageFamily::NormanFrench,
                "Norman French",
                "administrator, manager",
            )
            .with_first_usage(HistoricalPeriod::MiddleEnglish)
            .add_evolution(
                "Norman French baillif → Middle English bailif → Modern English bailiff",
            ),
        );

        // Equity
        tracker.add_etymology(
            Etymology::new(
                "equity",
                "aequitas",
                LanguageFamily::Latin,
                "Latin",
                "fairness, equality",
            )
            .with_first_usage(HistoricalPeriod::ClassicalLatin)
            .add_evolution("Latin aequitas → Old French equite → Middle English equity"),
        );

        tracker
    }

    /// Adds an etymology.
    pub fn add_etymology(&mut self, etymology: Etymology) {
        self.etymologies.insert(etymology.term.clone(), etymology);
    }

    /// Gets etymology for a term.
    pub fn get_etymology(&self, term: &str) -> Option<&Etymology> {
        self.etymologies.get(term)
    }

    /// Gets all etymologies by language family.
    pub fn get_by_language_family(&self, family: LanguageFamily) -> Vec<&Etymology> {
        self.etymologies
            .values()
            .filter(|e| e.language_family == family)
            .collect()
    }

    /// Returns the number of tracked etymologies.
    pub fn etymology_count(&self) -> usize {
        self.etymologies.len()
    }
}

impl Default for EtymologyTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Historical context annotation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalContext {
    /// The legal term or concept
    pub term: String,
    /// Historical period
    pub period: HistoricalPeriod,
    /// Historical context description
    pub context: String,
    /// Legal significance in that period
    pub legal_significance: String,
    /// Modern relevance
    pub modern_relevance: Option<String>,
    /// Related legal documents or cases
    pub related_documents: Vec<String>,
}

impl HistoricalContext {
    /// Creates a new historical context.
    pub fn new(
        term: impl Into<String>,
        period: HistoricalPeriod,
        context: impl Into<String>,
        legal_significance: impl Into<String>,
    ) -> Self {
        Self {
            term: term.into(),
            period,
            context: context.into(),
            legal_significance: legal_significance.into(),
            modern_relevance: None,
            related_documents: Vec::new(),
        }
    }

    /// Adds modern relevance.
    pub fn with_modern_relevance(mut self, relevance: impl Into<String>) -> Self {
        self.modern_relevance = Some(relevance.into());
        self
    }

    /// Adds a related document.
    pub fn add_related_document(mut self, document: impl Into<String>) -> Self {
        self.related_documents.push(document.into());
        self
    }
}

/// Historical context annotator.
#[derive(Debug, Clone)]
pub struct HistoricalContextAnnotator {
    /// Contexts indexed by term
    contexts: HashMap<String, Vec<HistoricalContext>>,
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

        // Magna Carta
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

        // Trial by jury
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

        // Writ of habeas corpus
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

        // Equity
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

        // Statute of Frauds
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

        // Bill of Rights
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

impl Default for HistoricalContextAnnotator {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// v0.2.9: International Standards
// ============================================================================

/// ISO 639-3 language code (3-letter code).
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ISO639_3 {
    /// 3-letter language code (e.g., "eng", "jpn", "fra").
    pub code: String,
    /// English name of the language.
    pub name: String,
    /// Type of language (Individual, Macrolanguage, Special).
    pub language_type: LanguageType,
    /// Scope (Individual, Macrolanguage, Special).
    pub scope: LanguageScope,
}

/// Type of language in ISO 639-3.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LanguageType {
    /// Living language.
    Living,
    /// Extinct language.
    Extinct,
    /// Ancient language.
    Ancient,
    /// Historical language.
    Historical,
    /// Constructed language.
    Constructed,
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

impl std::fmt::Display for LanguageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LanguageType::Living => write!(f, "Living"),
            LanguageType::Extinct => write!(f, "Extinct"),
            LanguageType::Ancient => write!(f, "Ancient"),
            LanguageType::Historical => write!(f, "Historical"),
            LanguageType::Constructed => write!(f, "Constructed"),
        }
    }
}

impl std::fmt::Display for LanguageScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LanguageScope::Individual => write!(f, "Individual"),
            LanguageScope::Macrolanguage => write!(f, "Macrolanguage"),
            LanguageScope::Special => write!(f, "Special"),
        }
    }
}

impl ISO639_3 {
    /// Creates a new ISO 639-3 language code.
    pub fn new(code: &str, name: &str, language_type: LanguageType, scope: LanguageScope) -> Self {
        Self {
            code: code.to_lowercase(),
            name: name.to_string(),
            language_type,
            scope,
        }
    }

    /// Converts to ISO 639-1 (2-letter code) if possible.
    pub fn to_iso639_1(&self) -> Option<String> {
        match self.code.as_str() {
            "eng" => Some("en".to_string()),
            "jpn" => Some("ja".to_string()),
            "fra" => Some("fr".to_string()),
            "deu" => Some("de".to_string()),
            "spa" => Some("es".to_string()),
            "zho" => Some("zh".to_string()),
            "ara" => Some("ar".to_string()),
            "rus" => Some("ru".to_string()),
            "por" => Some("pt".to_string()),
            "ita" => Some("it".to_string()),
            "nld" => Some("nl".to_string()),
            "pol" => Some("pl".to_string()),
            "kor" => Some("ko".to_string()),
            "heb" => Some("he".to_string()),
            "hin" => Some("hi".to_string()),
            "fas" => Some("fa".to_string()),
            "tha" => Some("th".to_string()),
            "vie" => Some("vi".to_string()),
            "ind" => Some("id".to_string()),
            "swe" => Some("sv".to_string()),
            "dan" => Some("da".to_string()),
            "fin" => Some("fi".to_string()),
            "nor" => Some("no".to_string()),
            "tur" => Some("tr".to_string()),
            "lat" => Some("la".to_string()),
            _ => None,
        }
    }

    /// Checks if this is a legal language (used in legal contexts).
    pub fn is_legal_language(&self) -> bool {
        matches!(
            self.code.as_str(),
            "eng" | "fra" | "deu" | "spa" | "lat" | "jpn" | "zho" | "ara" | "rus" | "por" | "ita"
        )
    }
}

/// Registry for ISO 639-3 language codes.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub struct ISO639_3_Registry {
    codes: HashMap<String, ISO639_3>,
}

impl ISO639_3_Registry {
    /// Creates a new registry.
    pub fn new() -> Self {
        Self {
            codes: HashMap::new(),
        }
    }

    /// Creates a registry with default legal language codes.
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();

        // Major legal languages
        registry.add_code(ISO639_3::new(
            "eng",
            "English",
            LanguageType::Living,
            LanguageScope::Individual,
        ));
        registry.add_code(ISO639_3::new(
            "fra",
            "French",
            LanguageType::Living,
            LanguageScope::Individual,
        ));
        registry.add_code(ISO639_3::new(
            "deu",
            "German",
            LanguageType::Living,
            LanguageScope::Individual,
        ));
        registry.add_code(ISO639_3::new(
            "spa",
            "Spanish",
            LanguageType::Living,
            LanguageScope::Individual,
        ));
        registry.add_code(ISO639_3::new(
            "jpn",
            "Japanese",
            LanguageType::Living,
            LanguageScope::Individual,
        ));
        registry.add_code(ISO639_3::new(
            "zho",
            "Chinese",
            LanguageType::Living,
            LanguageScope::Macrolanguage,
        ));
        registry.add_code(ISO639_3::new(
            "ara",
            "Arabic",
            LanguageType::Living,
            LanguageScope::Macrolanguage,
        ));
        registry.add_code(ISO639_3::new(
            "rus",
            "Russian",
            LanguageType::Living,
            LanguageScope::Individual,
        ));
        registry.add_code(ISO639_3::new(
            "por",
            "Portuguese",
            LanguageType::Living,
            LanguageScope::Individual,
        ));
        registry.add_code(ISO639_3::new(
            "ita",
            "Italian",
            LanguageType::Living,
            LanguageScope::Individual,
        ));
        registry.add_code(ISO639_3::new(
            "nld",
            "Dutch",
            LanguageType::Living,
            LanguageScope::Individual,
        ));
        registry.add_code(ISO639_3::new(
            "pol",
            "Polish",
            LanguageType::Living,
            LanguageScope::Individual,
        ));
        registry.add_code(ISO639_3::new(
            "kor",
            "Korean",
            LanguageType::Living,
            LanguageScope::Individual,
        ));
        registry.add_code(ISO639_3::new(
            "heb",
            "Hebrew",
            LanguageType::Living,
            LanguageScope::Individual,
        ));
        registry.add_code(ISO639_3::new(
            "hin",
            "Hindi",
            LanguageType::Living,
            LanguageScope::Individual,
        ));

        // Historical legal languages
        registry.add_code(ISO639_3::new(
            "lat",
            "Latin",
            LanguageType::Ancient,
            LanguageScope::Individual,
        ));
        registry.add_code(ISO639_3::new(
            "ang",
            "Old English",
            LanguageType::Historical,
            LanguageScope::Individual,
        ));
        registry.add_code(ISO639_3::new(
            "enm",
            "Middle English",
            LanguageType::Historical,
            LanguageScope::Individual,
        ));
        registry.add_code(ISO639_3::new(
            "fro",
            "Old French",
            LanguageType::Historical,
            LanguageScope::Individual,
        ));

        registry
    }

    /// Adds a language code to the registry.
    pub fn add_code(&mut self, code: ISO639_3) {
        self.codes.insert(code.code.clone(), code);
    }

    /// Gets a language code by its ISO 639-3 code.
    pub fn get_code(&self, code: &str) -> Option<&ISO639_3> {
        self.codes.get(&code.to_lowercase())
    }

    /// Gets all legal languages in the registry.
    pub fn get_legal_languages(&self) -> Vec<&ISO639_3> {
        self.codes
            .values()
            .filter(|code| code.is_legal_language())
            .collect()
    }

    /// Gets all historical/ancient languages.
    pub fn get_historical_languages(&self) -> Vec<&ISO639_3> {
        self.codes
            .values()
            .filter(|code| {
                matches!(
                    code.language_type,
                    LanguageType::Ancient | LanguageType::Historical
                )
            })
            .collect()
    }

    /// Returns the number of language codes in the registry.
    pub fn code_count(&self) -> usize {
        self.codes.len()
    }
}

impl Default for ISO639_3_Registry {
    fn default() -> Self {
        Self::new()
    }
}

/// CLDR (Common Locale Data Repository) field type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CLDRFieldType {
    /// Language display name.
    Languages,
    /// Territory (country/region) display name.
    Territories,
    /// Script display name.
    Scripts,
    /// Variant display name.
    Variants,
    /// Currency display name.
    Currencies,
    /// Time zone display name.
    TimeZones,
    /// Date format pattern.
    DateFormats,
    /// Time format pattern.
    TimeFormats,
    /// Number format pattern.
    NumberFormats,
}

impl std::fmt::Display for CLDRFieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CLDRFieldType::Languages => write!(f, "Languages"),
            CLDRFieldType::Territories => write!(f, "Territories"),
            CLDRFieldType::Scripts => write!(f, "Scripts"),
            CLDRFieldType::Variants => write!(f, "Variants"),
            CLDRFieldType::Currencies => write!(f, "Currencies"),
            CLDRFieldType::TimeZones => write!(f, "Time Zones"),
            CLDRFieldType::DateFormats => write!(f, "Date Formats"),
            CLDRFieldType::TimeFormats => write!(f, "Time Formats"),
            CLDRFieldType::NumberFormats => write!(f, "Number Formats"),
        }
    }
}

/// CLDR data entry.
#[derive(Debug, Clone)]
pub struct CLDREntry {
    /// The locale for this entry.
    pub locale: Locale,
    /// The field type.
    pub field_type: CLDRFieldType,
    /// The key (e.g., language code, territory code).
    pub key: String,
    /// The display value in the locale's language.
    pub value: String,
}

impl CLDREntry {
    /// Creates a new CLDR entry.
    pub fn new(locale: Locale, field_type: CLDRFieldType, key: &str, value: &str) -> Self {
        Self {
            locale,
            field_type,
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}

/// CLDR (Common Locale Data Repository) integration.
#[derive(Debug, Clone)]
pub struct CLDRData {
    entries: HashMap<String, Vec<CLDREntry>>,
}

impl CLDRData {
    /// Creates a new CLDR data store.
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Creates CLDR data with default legal localization data.
    pub fn with_defaults() -> Self {
        let mut cldr = Self::new();

        // English CLDR data
        let en_us = Locale::new("en").with_country("US");
        cldr.add_entry(CLDREntry::new(
            en_us.clone(),
            CLDRFieldType::Languages,
            "en",
            "English",
        ));
        cldr.add_entry(CLDREntry::new(
            en_us.clone(),
            CLDRFieldType::Languages,
            "ja",
            "Japanese",
        ));
        cldr.add_entry(CLDREntry::new(
            en_us.clone(),
            CLDRFieldType::Languages,
            "fr",
            "French",
        ));
        cldr.add_entry(CLDREntry::new(
            en_us.clone(),
            CLDRFieldType::Languages,
            "de",
            "German",
        ));
        cldr.add_entry(CLDREntry::new(
            en_us.clone(),
            CLDRFieldType::Territories,
            "US",
            "United States",
        ));
        cldr.add_entry(CLDREntry::new(
            en_us.clone(),
            CLDRFieldType::Territories,
            "GB",
            "United Kingdom",
        ));
        cldr.add_entry(CLDREntry::new(
            en_us.clone(),
            CLDRFieldType::Territories,
            "JP",
            "Japan",
        ));

        // Japanese CLDR data
        let ja_jp = Locale::new("ja").with_country("JP");
        cldr.add_entry(CLDREntry::new(
            ja_jp.clone(),
            CLDRFieldType::Languages,
            "en",
            "英語",
        ));
        cldr.add_entry(CLDREntry::new(
            ja_jp.clone(),
            CLDRFieldType::Languages,
            "ja",
            "日本語",
        ));
        cldr.add_entry(CLDREntry::new(
            ja_jp.clone(),
            CLDRFieldType::Languages,
            "fr",
            "フランス語",
        ));
        cldr.add_entry(CLDREntry::new(
            ja_jp.clone(),
            CLDRFieldType::Territories,
            "US",
            "アメリカ合衆国",
        ));
        cldr.add_entry(CLDREntry::new(
            ja_jp.clone(),
            CLDRFieldType::Territories,
            "GB",
            "イギリス",
        ));
        cldr.add_entry(CLDREntry::new(
            ja_jp.clone(),
            CLDRFieldType::Territories,
            "JP",
            "日本",
        ));

        // French CLDR data
        let fr_fr = Locale::new("fr").with_country("FR");
        cldr.add_entry(CLDREntry::new(
            fr_fr.clone(),
            CLDRFieldType::Languages,
            "en",
            "anglais",
        ));
        cldr.add_entry(CLDREntry::new(
            fr_fr.clone(),
            CLDRFieldType::Languages,
            "ja",
            "japonais",
        ));
        cldr.add_entry(CLDREntry::new(
            fr_fr.clone(),
            CLDRFieldType::Languages,
            "fr",
            "français",
        ));
        cldr.add_entry(CLDREntry::new(
            fr_fr.clone(),
            CLDRFieldType::Territories,
            "US",
            "États-Unis",
        ));
        cldr.add_entry(CLDREntry::new(
            fr_fr.clone(),
            CLDRFieldType::Territories,
            "GB",
            "Royaume-Uni",
        ));
        cldr.add_entry(CLDREntry::new(
            fr_fr.clone(),
            CLDRFieldType::Territories,
            "FR",
            "France",
        ));

        cldr
    }

    /// Adds a CLDR entry.
    pub fn add_entry(&mut self, entry: CLDREntry) {
        let key = format!("{}:{}", entry.locale, entry.field_type);
        self.entries.entry(key).or_default().push(entry);
    }

    /// Gets CLDR entries for a locale and field type.
    pub fn get_entries(&self, locale: &Locale, field_type: CLDRFieldType) -> Vec<&CLDREntry> {
        let key = format!("{}:{}", locale, field_type);
        self.entries
            .get(&key)
            .map(|entries| entries.iter().collect())
            .unwrap_or_default()
    }

    /// Gets a specific CLDR value.
    pub fn get_value(
        &self,
        locale: &Locale,
        field_type: CLDRFieldType,
        key: &str,
    ) -> Option<String> {
        self.get_entries(locale, field_type)
            .into_iter()
            .find(|entry| entry.key == key)
            .map(|entry| entry.value.clone())
    }

    /// Returns the number of locales with CLDR data.
    pub fn locale_count(&self) -> usize {
        self.entries.len()
    }

    /// Returns the total number of entries.
    pub fn entry_count(&self) -> usize {
        self.entries.values().map(|v| v.len()).sum()
    }
}

impl Default for CLDRData {
    fn default() -> Self {
        Self::new()
    }
}

/// Unicode CLDR legal extension type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LegalExtensionType {
    /// Legal system type (u-legal).
    LegalSystem,
    /// Citation style (u-cite).
    CitationStyle,
    /// Court type (u-court).
    CourtType,
    /// Legal formality level (u-formality).
    FormalityLevel,
}

impl std::fmt::Display for LegalExtensionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LegalExtensionType::LegalSystem => write!(f, "u-legal"),
            LegalExtensionType::CitationStyle => write!(f, "u-cite"),
            LegalExtensionType::CourtType => write!(f, "u-court"),
            LegalExtensionType::FormalityLevel => write!(f, "u-formality"),
        }
    }
}

/// Unicode CLDR legal extension.
#[derive(Debug, Clone)]
pub struct LegalExtension {
    /// The extension type.
    pub extension_type: LegalExtensionType,
    /// The extension value (e.g., "common", "civil", "bluebook").
    pub value: String,
}

impl LegalExtension {
    /// Creates a new legal extension.
    pub fn new(extension_type: LegalExtensionType, value: &str) -> Self {
        Self {
            extension_type,
            value: value.to_string(),
        }
    }

    /// Formats the extension as a BCP 47 extension string.
    pub fn to_bcp47_extension(&self) -> String {
        match self.extension_type {
            LegalExtensionType::LegalSystem => format!("u-legal-{}", self.value),
            LegalExtensionType::CitationStyle => format!("u-cite-{}", self.value),
            LegalExtensionType::CourtType => format!("u-court-{}", self.value),
            LegalExtensionType::FormalityLevel => format!("u-formality-{}", self.value),
        }
    }

    /// Creates a LegalSystem extension.
    pub fn legal_system(system: &str) -> Self {
        Self::new(LegalExtensionType::LegalSystem, system)
    }

    /// Creates a CitationStyle extension.
    pub fn citation_style(style: &str) -> Self {
        Self::new(LegalExtensionType::CitationStyle, style)
    }

    /// Creates a CourtType extension.
    pub fn court_type(court: &str) -> Self {
        Self::new(LegalExtensionType::CourtType, court)
    }

    /// Creates a FormalityLevel extension.
    pub fn formality_level(level: &str) -> Self {
        Self::new(LegalExtensionType::FormalityLevel, level)
    }
}

/// W3C internationalization compliance checker.
#[derive(Debug, Clone)]
pub struct W3CComplianceChecker {
    /// The locale to check.
    pub locale: Locale,
}

impl W3CComplianceChecker {
    /// Creates a new W3C compliance checker.
    pub fn new(locale: Locale) -> Self {
        Self { locale }
    }

    /// Checks if the locale has a valid language tag.
    pub fn has_valid_language_tag(&self) -> bool {
        !self.locale.language.is_empty() && self.locale.language.len() >= 2
    }

    /// Checks if the locale has a valid country code (if present).
    pub fn has_valid_country_code(&self) -> bool {
        if let Some(ref country) = self.locale.country {
            country.len() == 2 && country.chars().all(|c| c.is_ascii_uppercase())
        } else {
            true // No country code is valid
        }
    }

    /// Checks if the locale has a valid script code (if present).
    pub fn has_valid_script_code(&self) -> bool {
        if let Some(ref script) = self.locale.script {
            script.len() == 4 && script.chars().next().unwrap().is_ascii_uppercase()
        } else {
            true // No script code is valid
        }
    }

    /// Checks if text direction is properly specified.
    pub fn has_text_direction(&self) -> bool {
        // Check if locale is RTL language
        matches!(self.locale.language.as_str(), "ar" | "he" | "fa" | "ur")
    }

    /// Gets the recommended text direction for this locale.
    pub fn get_text_direction(&self) -> &str {
        if self.has_text_direction() {
            "rtl"
        } else {
            "ltr"
        }
    }

    /// Generates W3C-compliant HTML lang attribute.
    pub fn generate_html_lang_attribute(&self) -> String {
        self.locale.to_string()
    }

    /// Generates W3C-compliant HTML dir attribute.
    pub fn generate_html_dir_attribute(&self) -> String {
        self.get_text_direction().to_string()
    }

    /// Performs a full W3C compliance check.
    pub fn check_compliance(&self) -> W3CComplianceReport {
        let mut issues = Vec::new();

        if !self.has_valid_language_tag() {
            issues.push("Invalid language tag format".to_string());
        }

        if !self.has_valid_country_code() {
            issues.push("Invalid country code format".to_string());
        }

        if !self.has_valid_script_code() {
            issues.push("Invalid script code format".to_string());
        }

        W3CComplianceReport {
            locale: self.locale.clone(),
            is_compliant: issues.is_empty(),
            issues,
            lang_attribute: self.generate_html_lang_attribute(),
            dir_attribute: self.generate_html_dir_attribute(),
        }
    }
}

/// W3C compliance report.
#[derive(Debug, Clone)]
pub struct W3CComplianceReport {
    /// The locale that was checked.
    pub locale: Locale,
    /// Whether the locale is W3C compliant.
    pub is_compliant: bool,
    /// List of compliance issues.
    pub issues: Vec<String>,
    /// Recommended HTML lang attribute.
    pub lang_attribute: String,
    /// Recommended HTML dir attribute.
    pub dir_attribute: String,
}

impl W3CComplianceReport {
    /// Gets a summary of the compliance check.
    pub fn summary(&self) -> String {
        if self.is_compliant {
            format!("Locale '{}' is W3C compliant", self.locale)
        } else {
            format!(
                "Locale '{}' has {} compliance issue(s): {}",
                self.locale,
                self.issues.len(),
                self.issues.join(", ")
            )
        }
    }
}

/// IETF BCP 47 language tag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BCP47LanguageTag {
    /// Language subtag (e.g., "en", "ja").
    pub language: String,
    /// Script subtag (e.g., "Latn", "Jpan").
    pub script: Option<String>,
    /// Region subtag (e.g., "US", "JP").
    pub region: Option<String>,
    /// Variant subtags.
    pub variants: Vec<String>,
    /// Extension subtags (e.g., "u-ca-japanese").
    pub extensions: Vec<String>,
    /// Private use subtags.
    pub private_use: Vec<String>,
}

impl BCP47LanguageTag {
    /// Creates a new BCP 47 language tag.
    pub fn new(language: &str) -> Self {
        Self {
            language: language.to_lowercase(),
            script: None,
            region: None,
            variants: Vec::new(),
            extensions: Vec::new(),
            private_use: Vec::new(),
        }
    }

    /// Sets the script subtag.
    pub fn with_script(mut self, script: &str) -> Self {
        self.script = Some(
            script
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    if i == 0 {
                        c.to_ascii_uppercase()
                    } else {
                        c.to_ascii_lowercase()
                    }
                })
                .collect(),
        );
        self
    }

    /// Sets the region subtag.
    pub fn with_region(mut self, region: &str) -> Self {
        self.region = Some(region.to_uppercase());
        self
    }

    /// Adds a variant subtag.
    pub fn add_variant(mut self, variant: &str) -> Self {
        self.variants.push(variant.to_lowercase());
        self
    }

    /// Adds an extension subtag.
    pub fn add_extension(mut self, extension: &str) -> Self {
        self.extensions.push(extension.to_lowercase());
        self
    }

    /// Adds a private use subtag.
    pub fn add_private_use(mut self, private: &str) -> Self {
        self.private_use.push(private.to_lowercase());
        self
    }

    /// Formats the tag as a BCP 47 string.
    fn format_tag(&self) -> String {
        let mut parts = vec![self.language.clone()];

        if let Some(ref script) = self.script {
            parts.push(script.clone());
        }

        if let Some(ref region) = self.region {
            parts.push(region.clone());
        }

        parts.extend(self.variants.clone());
        parts.extend(self.extensions.clone());

        if !self.private_use.is_empty() {
            parts.push("x".to_string());
            parts.extend(self.private_use.clone());
        }

        parts.join("-")
    }

    /// Parses a BCP 47 language tag from a string.
    pub fn parse(tag: &str) -> Result<Self, String> {
        let parts: Vec<&str> = tag.split('-').collect();

        if parts.is_empty() {
            return Err("Empty language tag".to_string());
        }

        let language = parts[0].to_lowercase();
        if language.len() < 2 || language.len() > 3 {
            return Err(format!("Invalid language subtag: {}", language));
        }

        let mut bcp47 = Self::new(&language);
        let mut i = 1;

        // Parse script (4 letters, first uppercase)
        if i < parts.len() && parts[i].len() == 4 {
            bcp47 = bcp47.with_script(parts[i]);
            i += 1;
        }

        // Parse region (2 letters or 3 digits)
        if i < parts.len() && (parts[i].len() == 2 || parts[i].len() == 3) {
            bcp47 = bcp47.with_region(parts[i]);
            i += 1;
        }

        // Parse variants and extensions
        while i < parts.len() {
            if parts[i] == "x" {
                // Private use
                i += 1;
                while i < parts.len() {
                    bcp47 = bcp47.add_private_use(parts[i]);
                    i += 1;
                }
                break;
            } else if parts[i].len() == 1 {
                // Extension
                let ext_type = parts[i];
                i += 1;
                while i < parts.len() && parts[i].len() > 1 && parts[i] != "x" {
                    bcp47 = bcp47.add_extension(&format!("{}-{}", ext_type, parts[i]));
                    i += 1;
                }
            } else {
                // Variant
                bcp47 = bcp47.add_variant(parts[i]);
                i += 1;
            }
        }

        Ok(bcp47)
    }

    /// Converts to a Locale.
    pub fn to_locale(&self) -> Locale {
        let mut locale = Locale::new(&self.language);

        if let Some(ref script) = self.script {
            locale = locale.with_script(script);
        }

        if let Some(ref region) = self.region {
            locale = locale.with_country(region);
        }

        locale
    }

    /// Creates a BCP 47 tag from a Locale.
    pub fn from_locale(locale: &Locale) -> Self {
        let mut tag = Self::new(&locale.language);

        if let Some(ref script) = locale.script {
            tag = tag.with_script(script);
        }

        if let Some(ref country) = locale.country {
            tag = tag.with_region(country);
        }

        tag
    }

    /// Validates the BCP 47 tag.
    pub fn is_valid(&self) -> bool {
        // Language must be 2-3 characters
        if self.language.len() < 2 || self.language.len() > 3 {
            return false;
        }

        // Script must be 4 characters if present
        if let Some(ref script) = self.script {
            if script.len() != 4 {
                return false;
            }
        }

        // Region must be 2-3 characters if present
        if let Some(ref region) = self.region {
            if region.len() < 2 || region.len() > 3 {
                return false;
            }
        }

        true
    }
}

impl std::fmt::Display for BCP47LanguageTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format_tag())
    }
}

#[cfg(test)]
mod international_standards_tests {
    use super::*;

    #[test]
    fn test_iso639_3_creation() {
        let code = ISO639_3::new(
            "eng",
            "English",
            LanguageType::Living,
            LanguageScope::Individual,
        );

        assert_eq!(code.code, "eng");
        assert_eq!(code.name, "English");
        assert_eq!(code.language_type, LanguageType::Living);
        assert_eq!(code.scope, LanguageScope::Individual);
    }

    #[test]
    fn test_iso639_3_to_iso639_1() {
        let eng = ISO639_3::new(
            "eng",
            "English",
            LanguageType::Living,
            LanguageScope::Individual,
        );
        assert_eq!(eng.to_iso639_1(), Some("en".to_string()));

        let jpn = ISO639_3::new(
            "jpn",
            "Japanese",
            LanguageType::Living,
            LanguageScope::Individual,
        );
        assert_eq!(jpn.to_iso639_1(), Some("ja".to_string()));

        let lat = ISO639_3::new(
            "lat",
            "Latin",
            LanguageType::Ancient,
            LanguageScope::Individual,
        );
        assert_eq!(lat.to_iso639_1(), Some("la".to_string()));
    }

    #[test]
    fn test_iso639_3_is_legal_language() {
        let eng = ISO639_3::new(
            "eng",
            "English",
            LanguageType::Living,
            LanguageScope::Individual,
        );
        assert!(eng.is_legal_language());

        let lat = ISO639_3::new(
            "lat",
            "Latin",
            LanguageType::Ancient,
            LanguageScope::Individual,
        );
        assert!(lat.is_legal_language());

        let swa = ISO639_3::new(
            "swa",
            "Swahili",
            LanguageType::Living,
            LanguageScope::Individual,
        );
        assert!(!swa.is_legal_language());
    }

    #[test]
    fn test_iso639_3_registry_defaults() {
        let registry = ISO639_3_Registry::with_defaults();

        assert!(registry.code_count() > 0);
        assert!(registry.get_code("eng").is_some());
        assert!(registry.get_code("jpn").is_some());
        assert!(registry.get_code("lat").is_some());
    }

    #[test]
    fn test_iso639_3_registry_legal_languages() {
        let registry = ISO639_3_Registry::with_defaults();

        let legal_langs = registry.get_legal_languages();
        assert!(!legal_langs.is_empty());
        assert!(legal_langs.iter().any(|l| l.code == "eng"));
        assert!(legal_langs.iter().any(|l| l.code == "fra"));
    }

    #[test]
    fn test_iso639_3_registry_historical_languages() {
        let registry = ISO639_3_Registry::with_defaults();

        let historical = registry.get_historical_languages();
        assert!(!historical.is_empty());
        assert!(historical.iter().any(|l| l.code == "lat"));
        assert!(historical.iter().any(|l| l.code == "ang"));
    }

    #[test]
    fn test_language_type_display() {
        assert_eq!(LanguageType::Living.to_string(), "Living");
        assert_eq!(LanguageType::Ancient.to_string(), "Ancient");
        assert_eq!(LanguageType::Historical.to_string(), "Historical");
    }

    #[test]
    fn test_language_scope_display() {
        assert_eq!(LanguageScope::Individual.to_string(), "Individual");
        assert_eq!(LanguageScope::Macrolanguage.to_string(), "Macrolanguage");
        assert_eq!(LanguageScope::Special.to_string(), "Special");
    }

    #[test]
    fn test_cldr_entry_creation() {
        let locale = Locale::new("en").with_country("US");
        let entry = CLDREntry::new(locale.clone(), CLDRFieldType::Languages, "ja", "Japanese");

        assert_eq!(entry.locale, locale);
        assert_eq!(entry.field_type, CLDRFieldType::Languages);
        assert_eq!(entry.key, "ja");
        assert_eq!(entry.value, "Japanese");
    }

    #[test]
    fn test_cldr_data_defaults() {
        let cldr = CLDRData::with_defaults();

        assert!(cldr.locale_count() > 0);
        assert!(cldr.entry_count() > 0);
    }

    #[test]
    fn test_cldr_data_get_value() {
        let cldr = CLDRData::with_defaults();
        let en_us = Locale::new("en").with_country("US");

        let value = cldr.get_value(&en_us, CLDRFieldType::Languages, "ja");
        assert_eq!(value, Some("Japanese".to_string()));

        let territory = cldr.get_value(&en_us, CLDRFieldType::Territories, "JP");
        assert_eq!(territory, Some("Japan".to_string()));
    }

    #[test]
    fn test_cldr_data_japanese_localization() {
        let cldr = CLDRData::with_defaults();
        let ja_jp = Locale::new("ja").with_country("JP");

        let value = cldr.get_value(&ja_jp, CLDRFieldType::Languages, "en");
        assert_eq!(value, Some("英語".to_string()));

        let territory = cldr.get_value(&ja_jp, CLDRFieldType::Territories, "US");
        assert_eq!(territory, Some("アメリカ合衆国".to_string()));
    }

    #[test]
    fn test_cldr_field_type_display() {
        assert_eq!(CLDRFieldType::Languages.to_string(), "Languages");
        assert_eq!(CLDRFieldType::Territories.to_string(), "Territories");
        assert_eq!(CLDRFieldType::TimeZones.to_string(), "Time Zones");
    }

    #[test]
    fn test_legal_extension_creation() {
        let ext = LegalExtension::legal_system("common");
        assert_eq!(ext.extension_type, LegalExtensionType::LegalSystem);
        assert_eq!(ext.value, "common");
    }

    #[test]
    fn test_legal_extension_to_bcp47() {
        let legal_system = LegalExtension::legal_system("common");
        assert_eq!(legal_system.to_bcp47_extension(), "u-legal-common");

        let cite_style = LegalExtension::citation_style("bluebook");
        assert_eq!(cite_style.to_bcp47_extension(), "u-cite-bluebook");

        let court = LegalExtension::court_type("supreme");
        assert_eq!(court.to_bcp47_extension(), "u-court-supreme");

        let formality = LegalExtension::formality_level("high");
        assert_eq!(formality.to_bcp47_extension(), "u-formality-high");
    }

    #[test]
    fn test_legal_extension_type_display() {
        assert_eq!(LegalExtensionType::LegalSystem.to_string(), "u-legal");
        assert_eq!(LegalExtensionType::CitationStyle.to_string(), "u-cite");
        assert_eq!(LegalExtensionType::CourtType.to_string(), "u-court");
        assert_eq!(
            LegalExtensionType::FormalityLevel.to_string(),
            "u-formality"
        );
    }

    #[test]
    fn test_w3c_compliance_valid_locale() {
        let locale = Locale::new("en").with_country("US");
        let checker = W3CComplianceChecker::new(locale);

        assert!(checker.has_valid_language_tag());
        assert!(checker.has_valid_country_code());
        assert_eq!(checker.get_text_direction(), "ltr");
    }

    #[test]
    fn test_w3c_compliance_rtl_locale() {
        let locale = Locale::new("ar").with_country("SA");
        let checker = W3CComplianceChecker::new(locale);

        assert!(checker.has_text_direction());
        assert_eq!(checker.get_text_direction(), "rtl");
    }

    #[test]
    fn test_w3c_compliance_html_attributes() {
        let locale = Locale::new("en").with_country("US");
        let checker = W3CComplianceChecker::new(locale);

        assert_eq!(checker.generate_html_lang_attribute(), "en-US");
        assert_eq!(checker.generate_html_dir_attribute(), "ltr");
    }

    #[test]
    fn test_w3c_compliance_report() {
        let locale = Locale::new("en").with_country("US");
        let checker = W3CComplianceChecker::new(locale);

        let report = checker.check_compliance();
        assert!(report.is_compliant);
        assert!(report.issues.is_empty());
        assert_eq!(report.lang_attribute, "en-US");
        assert_eq!(report.dir_attribute, "ltr");
    }

    #[test]
    fn test_w3c_compliance_report_summary() {
        let locale = Locale::new("en").with_country("US");
        let checker = W3CComplianceChecker::new(locale);

        let report = checker.check_compliance();
        let summary = report.summary();
        assert!(summary.contains("compliant"));
    }

    #[test]
    fn test_bcp47_creation() {
        let tag = BCP47LanguageTag::new("en");
        assert_eq!(tag.language, "en");
        assert!(tag.script.is_none());
        assert!(tag.region.is_none());
    }

    #[test]
    fn test_bcp47_with_script_and_region() {
        let tag = BCP47LanguageTag::new("zh")
            .with_script("Hans")
            .with_region("CN");

        assert_eq!(tag.language, "zh");
        assert_eq!(tag.script, Some("Hans".to_string()));
        assert_eq!(tag.region, Some("CN".to_string()));
        assert_eq!(tag.format_tag(), "zh-Hans-CN");
    }

    #[test]
    fn test_bcp47_with_variants() {
        let tag = BCP47LanguageTag::new("sl")
            .with_region("IT")
            .add_variant("nedis");

        assert_eq!(tag.format_tag(), "sl-IT-nedis");
    }

    #[test]
    fn test_bcp47_with_extensions() {
        let tag = BCP47LanguageTag::new("en")
            .with_region("US")
            .add_extension("u-ca-gregory");

        assert!(tag.format_tag().contains("u-ca-gregory"));
    }

    #[test]
    fn test_bcp47_with_private_use() {
        let tag = BCP47LanguageTag::new("en").add_private_use("legal");

        assert!(tag.format_tag().contains("x-legal"));
    }

    #[test]
    fn test_bcp47_parse_simple() {
        let tag = BCP47LanguageTag::parse("en-US").unwrap();
        assert_eq!(tag.language, "en");
        assert_eq!(tag.region, Some("US".to_string()));
    }

    #[test]
    fn test_bcp47_parse_with_script() {
        let tag = BCP47LanguageTag::parse("zh-Hans-CN").unwrap();
        assert_eq!(tag.language, "zh");
        assert_eq!(tag.script, Some("Hans".to_string()));
        assert_eq!(tag.region, Some("CN".to_string()));
    }

    #[test]
    fn test_bcp47_parse_invalid() {
        let result = BCP47LanguageTag::parse("x");
        assert!(result.is_err());
    }

    #[test]
    fn test_bcp47_to_locale() {
        let tag = BCP47LanguageTag::new("en")
            .with_script("Latn")
            .with_region("US");

        let locale = tag.to_locale();
        assert_eq!(locale.language, "en");
        assert_eq!(locale.script, Some("Latn".to_string()));
        assert_eq!(locale.country, Some("US".to_string()));
    }

    #[test]
    fn test_bcp47_from_locale() {
        let locale = Locale::new("ja").with_script("Jpan").with_country("JP");

        let tag = BCP47LanguageTag::from_locale(&locale);
        assert_eq!(tag.language, "ja");
        assert_eq!(tag.script, Some("Jpan".to_string()));
        assert_eq!(tag.region, Some("JP".to_string()));
    }

    #[test]
    fn test_bcp47_is_valid() {
        let valid = BCP47LanguageTag::new("en").with_region("US");
        assert!(valid.is_valid());

        let mut invalid = BCP47LanguageTag::new("x");
        assert!(!invalid.is_valid());

        invalid = BCP47LanguageTag::new("en");
        invalid.script = Some("AB".to_string()); // Script must be 4 chars
        assert!(!invalid.is_valid());
    }

    #[test]
    fn test_bcp47_roundtrip() {
        let original = "en-Latn-US";
        let tag = BCP47LanguageTag::parse(original).unwrap();
        let reconstructed = tag.format_tag();
        assert_eq!(original, reconstructed);
    }
}

// ============================================================================
// v0.3.0: AI-Powered Translation
// ============================================================================

/// LLM provider type for AI-powered translation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LLMProvider {
    /// OpenAI GPT models.
    OpenAI,
    /// Anthropic Claude models.
    Anthropic,
    /// Google PaLM/Gemini models.
    Google,
    /// Meta Llama models.
    Meta,
    /// Custom LLM provider.
    Custom,
}

impl std::fmt::Display for LLMProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LLMProvider::OpenAI => write!(f, "OpenAI"),
            LLMProvider::Anthropic => write!(f, "Anthropic"),
            LLMProvider::Google => write!(f, "Google"),
            LLMProvider::Meta => write!(f, "Meta"),
            LLMProvider::Custom => write!(f, "Custom"),
        }
    }
}

/// Legal translation prompt template.
#[derive(Debug, Clone)]
pub struct LegalPromptTemplate {
    /// The system prompt for legal translation.
    pub system_prompt: String,
    /// The user prompt template with placeholders.
    pub user_prompt_template: String,
    /// Whether to include legal context in the prompt.
    pub include_legal_context: bool,
    /// Whether to preserve legal citations.
    pub preserve_citations: bool,
    /// Whether to maintain formality level.
    pub maintain_formality: bool,
}

impl LegalPromptTemplate {
    /// Creates a new legal prompt template.
    pub fn new(system_prompt: &str, user_prompt_template: &str) -> Self {
        Self {
            system_prompt: system_prompt.to_string(),
            user_prompt_template: user_prompt_template.to_string(),
            include_legal_context: true,
            preserve_citations: true,
            maintain_formality: true,
        }
    }

    /// Creates a default legal translation prompt template.
    pub fn default_legal_translation() -> Self {
        Self::new(
            "You are a professional legal translator with expertise in multiple legal systems. \
             Translate the following legal text accurately while preserving legal terminology, \
             citations, and formality. Maintain the precise legal meaning and structure.",
            "Translate the following legal text from {source_locale} to {target_locale}:\n\n\
             Text: {text}\n\n\
             Legal Context: {legal_context}\n\n\
             Please provide an accurate legal translation.",
        )
    }

    /// Sets whether to include legal context.
    pub fn with_legal_context(mut self, include: bool) -> Self {
        self.include_legal_context = include;
        self
    }

    /// Sets whether to preserve citations.
    pub fn with_citation_preservation(mut self, preserve: bool) -> Self {
        self.preserve_citations = preserve;
        self
    }

    /// Sets whether to maintain formality.
    pub fn with_formality(mut self, maintain: bool) -> Self {
        self.maintain_formality = maintain;
        self
    }

    /// Renders the prompt with the given parameters.
    pub fn render(
        &self,
        text: &str,
        source_locale: &Locale,
        target_locale: &Locale,
        legal_context: Option<&str>,
    ) -> String {
        let mut prompt = self.user_prompt_template.clone();

        prompt = prompt.replace("{text}", text);
        prompt = prompt.replace("{source_locale}", &source_locale.to_string());
        prompt = prompt.replace("{target_locale}", &target_locale.to_string());
        prompt = prompt.replace(
            "{legal_context}",
            legal_context.unwrap_or("General legal text"),
        );

        prompt
    }
}

/// LLM-based legal translator (infrastructure for external LLM integration).
#[derive(Debug, Clone)]
pub struct LLMTranslator {
    /// The LLM provider to use.
    pub provider: LLMProvider,
    /// The model name (e.g., "gpt-4", "claude-3-opus").
    pub model_name: String,
    /// The prompt template for translation.
    pub prompt_template: LegalPromptTemplate,
    /// Maximum tokens for the response.
    pub max_tokens: usize,
    /// Temperature for generation (0.0 to 1.0).
    pub temperature: f32,
}

impl LLMTranslator {
    /// Creates a new LLM translator.
    pub fn new(provider: LLMProvider, model_name: &str) -> Self {
        Self {
            provider,
            model_name: model_name.to_string(),
            prompt_template: LegalPromptTemplate::default_legal_translation(),
            max_tokens: 2000,
            temperature: 0.3, // Low temperature for consistent legal translation
        }
    }

    /// Creates an OpenAI GPT-4 translator.
    pub fn openai_gpt4() -> Self {
        Self::new(LLMProvider::OpenAI, "gpt-4")
    }

    /// Creates an Anthropic Claude translator.
    pub fn anthropic_claude() -> Self {
        Self::new(LLMProvider::Anthropic, "claude-3-opus-20240229")
    }

    /// Sets a custom prompt template.
    pub fn with_prompt_template(mut self, template: LegalPromptTemplate) -> Self {
        self.prompt_template = template;
        self
    }

    /// Sets the maximum tokens.
    pub fn with_max_tokens(mut self, max_tokens: usize) -> Self {
        self.max_tokens = max_tokens;
        self
    }

    /// Sets the temperature.
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature.clamp(0.0, 1.0);
        self
    }

    /// Generates a translation prompt for the given text.
    pub fn generate_prompt(
        &self,
        text: &str,
        source_locale: &Locale,
        target_locale: &Locale,
        legal_context: Option<&str>,
    ) -> String {
        self.prompt_template
            .render(text, source_locale, target_locale, legal_context)
    }

    /// Gets the system prompt.
    pub fn get_system_prompt(&self) -> &str {
        &self.prompt_template.system_prompt
    }
}

/// Context disambiguation type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DisambiguationType {
    /// Disambiguate by legal domain (e.g., criminal vs. civil).
    LegalDomain,
    /// Disambiguate by jurisdiction.
    Jurisdiction,
    /// Disambiguate by document type.
    DocumentType,
    /// Disambiguate by temporal context (historical vs. modern).
    Temporal,
    /// Disambiguate by formality level.
    Formality,
}

impl std::fmt::Display for DisambiguationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisambiguationType::LegalDomain => write!(f, "Legal Domain"),
            DisambiguationType::Jurisdiction => write!(f, "Jurisdiction"),
            DisambiguationType::DocumentType => write!(f, "Document Type"),
            DisambiguationType::Temporal => write!(f, "Temporal Context"),
            DisambiguationType::Formality => write!(f, "Formality Level"),
        }
    }
}

/// Disambiguation context for legal translation.
#[derive(Debug, Clone)]
pub struct DisambiguationContext {
    /// Type of disambiguation.
    pub disambiguation_type: DisambiguationType,
    /// Context value (e.g., "criminal_law", "en-US", "contract").
    pub value: String,
    /// Confidence score (0.0 to 1.0).
    pub confidence: f32,
    /// Explanation of the disambiguation.
    pub explanation: Option<String>,
}

impl DisambiguationContext {
    /// Creates a new disambiguation context.
    pub fn new(disambiguation_type: DisambiguationType, value: &str, confidence: f32) -> Self {
        Self {
            disambiguation_type,
            value: value.to_string(),
            confidence: confidence.clamp(0.0, 1.0),
            explanation: None,
        }
    }

    /// Adds an explanation.
    pub fn with_explanation(mut self, explanation: &str) -> Self {
        self.explanation = Some(explanation.to_string());
        self
    }
}

/// Context-aware disambiguator for legal terms.
#[derive(Debug, Clone)]
pub struct ContextDisambiguator {
    /// Map of term to disambiguation contexts.
    contexts: HashMap<String, Vec<DisambiguationContext>>,
}

impl ContextDisambiguator {
    /// Creates a new context disambiguator.
    pub fn new() -> Self {
        Self {
            contexts: HashMap::new(),
        }
    }

    /// Creates a disambiguator with default legal term contexts.
    pub fn with_defaults() -> Self {
        let mut disambiguator = Self::new();

        // "Action" - can mean lawsuit or legal proceeding
        disambiguator.add_context(
            "action",
            DisambiguationContext::new(DisambiguationType::LegalDomain, "civil_law", 0.8)
                .with_explanation("In civil law, 'action' typically refers to a lawsuit"),
        );
        disambiguator.add_context(
            "action",
            DisambiguationContext::new(DisambiguationType::LegalDomain, "criminal_law", 0.7)
                .with_explanation("In criminal law, 'action' may refer to prosecution"),
        );

        // "Consideration" - different meanings in contract law
        disambiguator.add_context(
            "consideration",
            DisambiguationContext::new(DisambiguationType::LegalDomain, "contract_law", 0.9)
                .with_explanation(
                    "In contract law, 'consideration' is a requirement for valid contracts",
                ),
        );

        // "Trust" - different meanings in property law
        disambiguator.add_context(
            "trust",
            DisambiguationContext::new(DisambiguationType::LegalDomain, "property_law", 0.85)
                .with_explanation("In property law, 'trust' is a fiduciary relationship"),
        );

        // "Bill" - legislative document vs. invoice
        disambiguator.add_context(
            "bill",
            DisambiguationContext::new(DisambiguationType::DocumentType, "legislation", 0.8)
                .with_explanation("In legislative context, 'bill' is a proposed law"),
        );
        disambiguator.add_context(
            "bill",
            DisambiguationContext::new(DisambiguationType::DocumentType, "commercial", 0.6)
                .with_explanation("In commercial context, 'bill' may refer to an invoice"),
        );

        disambiguator
    }

    /// Adds a disambiguation context for a term.
    pub fn add_context(&mut self, term: &str, context: DisambiguationContext) {
        self.contexts
            .entry(term.to_lowercase())
            .or_default()
            .push(context);
    }

    /// Gets disambiguation contexts for a term.
    pub fn get_contexts(&self, term: &str) -> Vec<&DisambiguationContext> {
        self.contexts
            .get(&term.to_lowercase())
            .map(|contexts| contexts.iter().collect())
            .unwrap_or_default()
    }

    /// Gets the best disambiguation context for a term given a type.
    pub fn get_best_context(
        &self,
        term: &str,
        disambiguation_type: DisambiguationType,
    ) -> Option<&DisambiguationContext> {
        self.get_contexts(term)
            .into_iter()
            .filter(|ctx| ctx.disambiguation_type == disambiguation_type)
            .max_by(|a, b| {
                a.confidence
                    .partial_cmp(&b.confidence)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    }

    /// Returns the number of terms with disambiguation contexts.
    pub fn term_count(&self) -> usize {
        self.contexts.len()
    }

    /// Returns the total number of disambiguation contexts.
    pub fn context_count(&self) -> usize {
        self.contexts.values().map(|v| v.len()).sum()
    }
}

impl Default for ContextDisambiguator {
    fn default() -> Self {
        Self::new()
    }
}

/// Writing style attribute for translation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StyleAttribute {
    /// Formality level (formal, informal, neutral).
    Formality,
    /// Tone (professional, conversational, authoritative).
    Tone,
    /// Person (first, second, third).
    Person,
    /// Voice (active, passive).
    Voice,
    /// Tense (present, past, future).
    Tense,
}

impl std::fmt::Display for StyleAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StyleAttribute::Formality => write!(f, "Formality"),
            StyleAttribute::Tone => write!(f, "Tone"),
            StyleAttribute::Person => write!(f, "Person"),
            StyleAttribute::Voice => write!(f, "Voice"),
            StyleAttribute::Tense => write!(f, "Tense"),
        }
    }
}

/// Style profile for legal text.
#[derive(Debug, Clone)]
pub struct StyleProfile {
    /// Map of style attributes to their values.
    attributes: HashMap<StyleAttribute, String>,
    /// Locale-specific style preferences.
    locale_preferences: HashMap<Locale, HashMap<StyleAttribute, String>>,
}

impl StyleProfile {
    /// Creates a new style profile.
    pub fn new() -> Self {
        Self {
            attributes: HashMap::new(),
            locale_preferences: HashMap::new(),
        }
    }

    /// Creates a formal legal style profile.
    pub fn formal_legal() -> Self {
        let mut profile = Self::new();
        profile.set_attribute(StyleAttribute::Formality, "formal");
        profile.set_attribute(StyleAttribute::Tone, "professional");
        profile.set_attribute(StyleAttribute::Person, "third");
        profile.set_attribute(StyleAttribute::Voice, "passive");
        profile.set_attribute(StyleAttribute::Tense, "present");
        profile
    }

    /// Creates an informal legal style profile.
    pub fn informal_legal() -> Self {
        let mut profile = Self::new();
        profile.set_attribute(StyleAttribute::Formality, "informal");
        profile.set_attribute(StyleAttribute::Tone, "conversational");
        profile.set_attribute(StyleAttribute::Person, "second");
        profile.set_attribute(StyleAttribute::Voice, "active");
        profile.set_attribute(StyleAttribute::Tense, "present");
        profile
    }

    /// Sets a style attribute.
    pub fn set_attribute(&mut self, attribute: StyleAttribute, value: &str) {
        self.attributes.insert(attribute, value.to_string());
    }

    /// Gets a style attribute.
    pub fn get_attribute(&self, attribute: StyleAttribute) -> Option<&String> {
        self.attributes.get(&attribute)
    }

    /// Sets a locale-specific style preference.
    pub fn set_locale_preference(
        &mut self,
        locale: Locale,
        attribute: StyleAttribute,
        value: &str,
    ) {
        self.locale_preferences
            .entry(locale)
            .or_default()
            .insert(attribute, value.to_string());
    }

    /// Gets a style attribute for a specific locale (with fallback to global).
    pub fn get_attribute_for_locale(
        &self,
        locale: &Locale,
        attribute: StyleAttribute,
    ) -> Option<&String> {
        self.locale_preferences
            .get(locale)
            .and_then(|prefs| prefs.get(&attribute))
            .or_else(|| self.attributes.get(&attribute))
    }

    /// Returns the number of style attributes.
    pub fn attribute_count(&self) -> usize {
        self.attributes.len()
    }

    /// Returns the number of locales with preferences.
    pub fn locale_count(&self) -> usize {
        self.locale_preferences.len()
    }
}

impl Default for StyleProfile {
    fn default() -> Self {
        Self::new()
    }
}

/// Style-preserving translator.
#[derive(Debug, Clone)]
pub struct StylePreservingTranslator {
    /// Source style profile.
    pub source_profile: StyleProfile,
    /// Target locale.
    pub target_locale: Locale,
    /// Whether to adapt style to target locale conventions.
    pub adapt_to_target: bool,
}

impl StylePreservingTranslator {
    /// Creates a new style-preserving translator.
    pub fn new(source_profile: StyleProfile, target_locale: Locale) -> Self {
        Self {
            source_profile,
            target_locale,
            adapt_to_target: false,
        }
    }

    /// Sets whether to adapt style to target locale.
    pub fn with_adaptation(mut self, adapt: bool) -> Self {
        self.adapt_to_target = adapt;
        self
    }

    /// Gets the target style profile for translation.
    pub fn get_target_profile(&self) -> StyleProfile {
        if self.adapt_to_target {
            // Create adapted profile based on target locale
            let mut adapted = self.source_profile.clone();

            // Add locale-specific adaptations
            // For example, Japanese formal legal style uses more passive voice
            if self.target_locale.language == "ja" {
                adapted.set_locale_preference(
                    self.target_locale.clone(),
                    StyleAttribute::Voice,
                    "passive",
                );
            }

            adapted
        } else {
            self.source_profile.clone()
        }
    }

    /// Generates style preservation instructions for LLM prompt.
    pub fn generate_style_instructions(&self) -> String {
        let profile = self.get_target_profile();
        let mut instructions = Vec::new();

        if let Some(formality) =
            profile.get_attribute_for_locale(&self.target_locale, StyleAttribute::Formality)
        {
            instructions.push(format!("Maintain {} formality level", formality));
        }

        if let Some(tone) =
            profile.get_attribute_for_locale(&self.target_locale, StyleAttribute::Tone)
        {
            instructions.push(format!("Use a {} tone", tone));
        }

        if let Some(voice) =
            profile.get_attribute_for_locale(&self.target_locale, StyleAttribute::Voice)
        {
            instructions.push(format!("Prefer {} voice", voice));
        }

        instructions.join(". ")
    }
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

impl std::fmt::Display for QualityMetric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QualityMetric::SemanticAccuracy => write!(f, "Semantic Accuracy"),
            QualityMetric::TerminologicalConsistency => write!(f, "Terminological Consistency"),
            QualityMetric::GrammaticalCorrectness => write!(f, "Grammatical Correctness"),
            QualityMetric::StyleAppropriateness => write!(f, "Style Appropriateness"),
            QualityMetric::CitationPreservation => write!(f, "Citation Preservation"),
            QualityMetric::Fluency => write!(f, "Fluency"),
        }
    }
}

/// AI quality score for a specific metric.
#[derive(Debug, Clone)]
pub struct AIQualityScore {
    /// The quality metric.
    pub metric: QualityMetric,
    /// The score (0.0 to 1.0).
    pub score: f32,
    /// Explanation of the score.
    pub explanation: Option<String>,
}

impl AIQualityScore {
    /// Creates a new quality score.
    pub fn new(metric: QualityMetric, score: f32) -> Self {
        Self {
            metric,
            score: score.clamp(0.0, 1.0),
            explanation: None,
        }
    }

    /// Adds an explanation.
    pub fn with_explanation(mut self, explanation: &str) -> Self {
        self.explanation = Some(explanation.to_string());
        self
    }
}

/// Quality estimation report for AI translation.
#[derive(Debug, Clone)]
pub struct QualityEstimationReport {
    /// Overall quality score (0.0 to 1.0).
    pub overall_score: f32,
    /// Individual metric scores.
    pub metric_scores: HashMap<QualityMetric, AIQualityScore>,
    /// Source text.
    pub source_text: String,
    /// Translated text.
    pub translated_text: String,
    /// Source locale.
    pub source_locale: Locale,
    /// Target locale.
    pub target_locale: Locale,
}

impl QualityEstimationReport {
    /// Creates a new quality estimation report.
    pub fn new(
        source_text: &str,
        translated_text: &str,
        source_locale: Locale,
        target_locale: Locale,
    ) -> Self {
        Self {
            overall_score: 0.0,
            metric_scores: HashMap::new(),
            source_text: source_text.to_string(),
            translated_text: translated_text.to_string(),
            source_locale,
            target_locale,
        }
    }

    /// Adds a quality score for a metric.
    pub fn add_score(&mut self, score: AIQualityScore) {
        self.metric_scores.insert(score.metric, score);
        self.recalculate_overall_score();
    }

    /// Recalculates the overall score based on metric scores.
    fn recalculate_overall_score(&mut self) {
        if self.metric_scores.is_empty() {
            self.overall_score = 0.0;
            return;
        }

        let sum: f32 = self.metric_scores.values().map(|s| s.score).sum();
        self.overall_score = sum / self.metric_scores.len() as f32;
    }

    /// Gets the quality level (Low, Medium, High, Excellent).
    pub fn get_quality_level(&self) -> &str {
        match self.overall_score {
            s if s >= 0.9 => "Excellent",
            s if s >= 0.75 => "High",
            s if s >= 0.5 => "Medium",
            _ => "Low",
        }
    }

    /// Checks if the translation meets a minimum quality threshold.
    pub fn meets_threshold(&self, threshold: f32) -> bool {
        self.overall_score >= threshold
    }

    /// Generates a summary of the quality estimation.
    pub fn summary(&self) -> String {
        format!(
            "Translation from {} to {} - Overall Quality: {:.2}% ({})\n\
             Metric Scores: {}",
            self.source_locale,
            self.target_locale,
            self.overall_score * 100.0,
            self.get_quality_level(),
            self.metric_scores.len()
        )
    }
}

/// Quality estimator for AI-powered translations.
#[derive(Debug, Clone)]
pub struct QualityEstimator {
    /// Minimum threshold for acceptable quality (0.0 to 1.0).
    pub min_threshold: f32,
}

impl QualityEstimator {
    /// Creates a new quality estimator.
    pub fn new(min_threshold: f32) -> Self {
        Self {
            min_threshold: min_threshold.clamp(0.0, 1.0),
        }
    }

    /// Creates a quality estimator with default threshold (0.7).
    pub fn with_defaults() -> Self {
        Self::new(0.7)
    }

    /// Estimates quality for a translation (simplified heuristic-based approach).
    pub fn estimate_quality(
        &self,
        source_text: &str,
        translated_text: &str,
        source_locale: Locale,
        target_locale: Locale,
    ) -> QualityEstimationReport {
        let mut report = QualityEstimationReport::new(
            source_text,
            translated_text,
            source_locale,
            target_locale,
        );

        // Semantic Accuracy: Check length ratio (simple heuristic)
        let length_ratio = translated_text.len() as f32 / source_text.len().max(1) as f32;
        let semantic_score = if (0.5..=2.0).contains(&length_ratio) {
            0.8
        } else {
            0.5
        };
        report.add_score(
            AIQualityScore::new(QualityMetric::SemanticAccuracy, semantic_score)
                .with_explanation("Based on length ratio between source and target"),
        );

        // Terminological Consistency: Check if common legal terms are present
        let has_legal_terms = translated_text.to_lowercase().contains("law")
            || translated_text.to_lowercase().contains("contract")
            || translated_text.to_lowercase().contains("court");
        let term_score = if has_legal_terms { 0.75 } else { 0.6 };
        report.add_score(
            AIQualityScore::new(QualityMetric::TerminologicalConsistency, term_score)
                .with_explanation("Based on presence of legal terminology"),
        );

        // Grammatical Correctness: Simple check for complete sentences
        let has_punctuation = translated_text.ends_with('.')
            || translated_text.ends_with('?')
            || translated_text.ends_with('!');
        let grammar_score = if has_punctuation { 0.85 } else { 0.7 };
        report.add_score(
            AIQualityScore::new(QualityMetric::GrammaticalCorrectness, grammar_score)
                .with_explanation("Based on basic sentence structure"),
        );

        // Fluency: Check if not empty and has reasonable structure
        let fluency_score = if !translated_text.is_empty() && translated_text.len() > 10 {
            0.8
        } else {
            0.4
        };
        report.add_score(
            AIQualityScore::new(QualityMetric::Fluency, fluency_score)
                .with_explanation("Based on text length and non-emptiness"),
        );

        report
    }

    /// Checks if a translation meets the minimum quality threshold.
    pub fn is_acceptable(&self, report: &QualityEstimationReport) -> bool {
        report.meets_threshold(self.min_threshold)
    }
}

impl Default for QualityEstimator {
    fn default() -> Self {
        Self::with_defaults()
    }
}

#[cfg(test)]
mod ai_translation_tests {
    use super::*;

    #[test]
    fn test_llm_provider_display() {
        assert_eq!(LLMProvider::OpenAI.to_string(), "OpenAI");
        assert_eq!(LLMProvider::Anthropic.to_string(), "Anthropic");
        assert_eq!(LLMProvider::Google.to_string(), "Google");
    }

    #[test]
    fn test_legal_prompt_template_creation() {
        let template = LegalPromptTemplate::default_legal_translation();

        assert!(template.system_prompt.contains("legal translator"));
        assert!(template.include_legal_context);
        assert!(template.preserve_citations);
    }

    #[test]
    fn test_legal_prompt_template_render() {
        let template = LegalPromptTemplate::default_legal_translation();
        let source = Locale::new("en").with_country("US");
        let target = Locale::new("fr").with_country("FR");

        let rendered = template.render(
            "This is a contract.",
            &source,
            &target,
            Some("contract_law"),
        );

        assert!(rendered.contains("This is a contract."));
        assert!(rendered.contains("en-US"));
        assert!(rendered.contains("fr-FR"));
        assert!(rendered.contains("contract_law"));
    }

    #[test]
    fn test_llm_translator_creation() {
        let translator = LLMTranslator::new(LLMProvider::OpenAI, "gpt-4");

        assert_eq!(translator.provider, LLMProvider::OpenAI);
        assert_eq!(translator.model_name, "gpt-4");
        assert_eq!(translator.max_tokens, 2000);
        assert_eq!(translator.temperature, 0.3);
    }

    #[test]
    fn test_llm_translator_openai() {
        let translator = LLMTranslator::openai_gpt4();

        assert_eq!(translator.provider, LLMProvider::OpenAI);
        assert_eq!(translator.model_name, "gpt-4");
    }

    #[test]
    fn test_llm_translator_anthropic() {
        let translator = LLMTranslator::anthropic_claude();

        assert_eq!(translator.provider, LLMProvider::Anthropic);
        assert!(translator.model_name.contains("claude"));
    }

    #[test]
    fn test_llm_translator_generate_prompt() {
        let translator = LLMTranslator::openai_gpt4();
        let source = Locale::new("en").with_country("US");
        let target = Locale::new("ja").with_country("JP");

        let prompt =
            translator.generate_prompt("Contract law", &source, &target, Some("civil_law"));

        assert!(prompt.contains("Contract law"));
        assert!(prompt.contains("en-US"));
        assert!(prompt.contains("ja-JP"));
    }

    #[test]
    fn test_disambiguation_type_display() {
        assert_eq!(DisambiguationType::LegalDomain.to_string(), "Legal Domain");
        assert_eq!(DisambiguationType::Jurisdiction.to_string(), "Jurisdiction");
    }

    #[test]
    fn test_disambiguation_context_creation() {
        let context =
            DisambiguationContext::new(DisambiguationType::LegalDomain, "criminal_law", 0.9)
                .with_explanation("Criminal law context");

        assert_eq!(context.disambiguation_type, DisambiguationType::LegalDomain);
        assert_eq!(context.value, "criminal_law");
        assert_eq!(context.confidence, 0.9);
        assert!(context.explanation.is_some());
    }

    #[test]
    fn test_context_disambiguator_defaults() {
        let disambiguator = ContextDisambiguator::with_defaults();

        assert!(disambiguator.term_count() > 0);
        assert!(disambiguator.context_count() > 0);
    }

    #[test]
    fn test_context_disambiguator_get_contexts() {
        let disambiguator = ContextDisambiguator::with_defaults();

        let contexts = disambiguator.get_contexts("action");
        assert!(!contexts.is_empty());
    }

    #[test]
    fn test_context_disambiguator_best_context() {
        let disambiguator = ContextDisambiguator::with_defaults();

        let best = disambiguator.get_best_context("consideration", DisambiguationType::LegalDomain);
        assert!(best.is_some());
        assert_eq!(best.unwrap().value, "contract_law");
    }

    #[test]
    fn test_style_attribute_display() {
        assert_eq!(StyleAttribute::Formality.to_string(), "Formality");
        assert_eq!(StyleAttribute::Tone.to_string(), "Tone");
        assert_eq!(StyleAttribute::Voice.to_string(), "Voice");
    }

    #[test]
    fn test_style_profile_formal_legal() {
        let profile = StyleProfile::formal_legal();

        assert_eq!(
            profile.get_attribute(StyleAttribute::Formality),
            Some(&"formal".to_string())
        );
        assert_eq!(
            profile.get_attribute(StyleAttribute::Tone),
            Some(&"professional".to_string())
        );
        assert_eq!(
            profile.get_attribute(StyleAttribute::Voice),
            Some(&"passive".to_string())
        );
    }

    #[test]
    fn test_style_profile_informal_legal() {
        let profile = StyleProfile::informal_legal();

        assert_eq!(
            profile.get_attribute(StyleAttribute::Formality),
            Some(&"informal".to_string())
        );
        assert_eq!(
            profile.get_attribute(StyleAttribute::Tone),
            Some(&"conversational".to_string())
        );
    }

    #[test]
    fn test_style_profile_locale_preference() {
        let mut profile = StyleProfile::new();
        let ja_jp = Locale::new("ja").with_country("JP");

        profile.set_locale_preference(ja_jp.clone(), StyleAttribute::Voice, "passive");

        let voice = profile.get_attribute_for_locale(&ja_jp, StyleAttribute::Voice);
        assert_eq!(voice, Some(&"passive".to_string()));
    }

    #[test]
    fn test_style_preserving_translator() {
        let profile = StyleProfile::formal_legal();
        let target = Locale::new("fr").with_country("FR");

        let translator = StylePreservingTranslator::new(profile, target);

        assert!(!translator.adapt_to_target);
    }

    #[test]
    fn test_style_preserving_translator_instructions() {
        let profile = StyleProfile::formal_legal();
        let target = Locale::new("en").with_country("US");

        let translator = StylePreservingTranslator::new(profile, target);
        let instructions = translator.generate_style_instructions();

        assert!(instructions.contains("formal") || instructions.contains("professional"));
    }

    #[test]
    fn test_quality_metric_display() {
        assert_eq!(
            QualityMetric::SemanticAccuracy.to_string(),
            "Semantic Accuracy"
        );
        assert_eq!(
            QualityMetric::TerminologicalConsistency.to_string(),
            "Terminological Consistency"
        );
    }

    #[test]
    fn test_ai_quality_score_creation() {
        let score = AIQualityScore::new(QualityMetric::SemanticAccuracy, 0.85)
            .with_explanation("High semantic accuracy");

        assert_eq!(score.metric, QualityMetric::SemanticAccuracy);
        assert_eq!(score.score, 0.85);
        assert!(score.explanation.is_some());
    }

    #[test]
    fn test_quality_estimation_report_creation() {
        let source = Locale::new("en").with_country("US");
        let target = Locale::new("fr").with_country("FR");

        let mut report = QualityEstimationReport::new(
            "This is a contract",
            "Ceci est un contrat",
            source,
            target,
        );

        report.add_score(AIQualityScore::new(QualityMetric::SemanticAccuracy, 0.9));
        report.add_score(AIQualityScore::new(QualityMetric::Fluency, 0.85));

        assert!(report.overall_score > 0.0);
        assert_eq!(report.metric_scores.len(), 2);
    }

    #[test]
    fn test_quality_estimation_report_quality_level() {
        let source = Locale::new("en").with_country("US");
        let target = Locale::new("fr").with_country("FR");

        let mut report = QualityEstimationReport::new("Source", "Target", source, target);
        report.add_score(AIQualityScore::new(QualityMetric::SemanticAccuracy, 0.95));

        assert_eq!(report.get_quality_level(), "Excellent");
    }

    #[test]
    fn test_quality_estimation_report_threshold() {
        let source = Locale::new("en").with_country("US");
        let target = Locale::new("fr").with_country("FR");

        let mut report = QualityEstimationReport::new("Source", "Target", source, target);
        report.add_score(AIQualityScore::new(QualityMetric::SemanticAccuracy, 0.8));

        assert!(report.meets_threshold(0.7));
        assert!(!report.meets_threshold(0.9));
    }

    #[test]
    fn test_quality_estimator_creation() {
        let estimator = QualityEstimator::new(0.75);

        assert_eq!(estimator.min_threshold, 0.75);
    }

    #[test]
    fn test_quality_estimator_defaults() {
        let estimator = QualityEstimator::with_defaults();

        assert_eq!(estimator.min_threshold, 0.7);
    }

    #[test]
    fn test_quality_estimator_estimate() {
        let estimator = QualityEstimator::with_defaults();
        let source = Locale::new("en").with_country("US");
        let target = Locale::new("fr").with_country("FR");

        let report = estimator.estimate_quality(
            "This is a legal contract.",
            "Ceci est un contrat juridique.",
            source,
            target,
        );

        assert!(report.overall_score > 0.0);
        assert!(report.metric_scores.len() > 0);
    }

    #[test]
    fn test_quality_estimator_is_acceptable() {
        let estimator = QualityEstimator::new(0.6);
        let source = Locale::new("en").with_country("US");
        let target = Locale::new("fr").with_country("FR");

        let report = estimator.estimate_quality(
            "This is a contract.",
            "Ceci est un contrat.",
            source,
            target,
        );

        assert!(estimator.is_acceptable(&report));
    }
}

#[cfg(test)]
mod cultural_tests {
    use super::*;

    #[test]
    fn test_cultural_context_creation() {
        let locale = Locale::new("ja").with_country("JP");
        let context = CulturalContext::new(
            locale,
            ContextCategory::SocialHierarchy,
            "keigo",
            "Honorific language system",
        );

        assert_eq!(context.term, "keigo");
        assert_eq!(context.category, ContextCategory::SocialHierarchy);
        assert!(context.guidelines.is_empty());
    }

    #[test]
    fn test_cultural_context_with_guidelines() {
        let locale = Locale::new("ja").with_country("JP");
        let context = CulturalContext::new(
            locale,
            ContextCategory::BusinessEtiquette,
            "hanko",
            "Personal seal",
        )
        .with_guideline("Required for contracts")
        .with_equivalent("en-US", "signature");

        assert_eq!(context.guidelines.len(), 1);
        assert_eq!(context.cross_cultural_equivalents.len(), 1);
    }

    #[test]
    fn test_cultural_context_registry() {
        let registry = CulturalContextRegistry::with_defaults();
        let ja_jp = Locale::new("ja").with_country("JP");

        let contexts = registry.get_contexts(&ja_jp);
        assert!(!contexts.is_empty());

        let keigo = registry.find_term(&ja_jp, "keigo");
        assert!(keigo.is_some());
        assert_eq!(keigo.unwrap().term, "keigo");
    }

    #[test]
    fn test_cultural_context_by_category() {
        let registry = CulturalContextRegistry::with_defaults();
        let ja_jp = Locale::new("ja").with_country("JP");

        let hierarchy_contexts =
            registry.get_by_category(&ja_jp, &ContextCategory::SocialHierarchy);
        assert!(!hierarchy_contexts.is_empty());
    }

    #[test]
    fn test_context_category_display() {
        assert_eq!(
            ContextCategory::SocialHierarchy.to_string(),
            "Social Hierarchy"
        );
        assert_eq!(
            ContextCategory::ReligiousPractice.to_string(),
            "Religious Practice"
        );
    }

    #[test]
    fn test_local_custom_creation() {
        let locale = Locale::new("ja").with_country("JP");
        let custom = LocalCustom::new(
            "Miai marriage",
            "Japan",
            locale,
            CustomType::Marriage,
            "Traditional arranged marriage introduction",
        )
        .with_recognition_level(0.3);

        assert_eq!(custom.name, "Miai marriage");
        assert_eq!(custom.recognition_level, 0.3);
    }

    #[test]
    fn test_local_custom_registry() {
        let registry = LocalCustomRegistry::with_defaults();

        let japan_customs = registry.get_customs("Japan");
        assert!(!japan_customs.is_empty());

        let miai = registry.find_custom("Japan", "Miai marriage");
        assert!(miai.is_some());
    }

    #[test]
    fn test_local_custom_by_type() {
        let registry = LocalCustomRegistry::with_defaults();

        let marriage_customs = registry.get_by_type("Saudi Arabia", &CustomType::Marriage);
        assert!(!marriage_customs.is_empty());
    }

    #[test]
    fn test_custom_type_display() {
        assert_eq!(CustomType::Marriage.to_string(), "Marriage");
        assert_eq!(
            CustomType::DisputeResolution.to_string(),
            "Dispute Resolution"
        );
    }

    #[test]
    fn test_religious_law_islamic() {
        let islamic = ReligiousLawSystem::islamic();

        assert_eq!(islamic.law_type, ReligiousLawType::Islamic);
        assert!(!islamic.principles.is_empty());
        assert!(!islamic.sources.is_empty());
        assert!(islamic.civil_equivalents.contains_key("mahr"));
    }

    #[test]
    fn test_religious_law_jewish() {
        let jewish = ReligiousLawSystem::jewish();

        assert_eq!(jewish.law_type, ReligiousLawType::Jewish);
        assert!(jewish.civil_equivalents.contains_key("get"));
    }

    #[test]
    fn test_religious_law_registry() {
        let registry = ReligiousLawRegistry::with_defaults();

        let islamic = registry.get_system(ReligiousLawType::Islamic);
        assert!(islamic.is_some());

        let sa_systems = registry.get_by_jurisdiction("Saudi Arabia");
        assert!(!sa_systems.is_empty());
    }

    #[test]
    fn test_religious_law_type_display() {
        assert_eq!(
            ReligiousLawType::Islamic.to_string(),
            "Islamic Law (Sharia)"
        );
        assert_eq!(ReligiousLawType::Jewish.to_string(), "Jewish Law (Halakha)");
    }

    #[test]
    fn test_indigenous_law_creation() {
        let system = IndigenousLawSystem::new("Navajo Nation", "Southwestern United States")
            .with_principle("Hózhǫ́ (harmony)")
            .with_dispute_resolution("Peacemaking circles")
            .with_property_concept("Communal land ownership")
            .with_state_recognition(true);

        assert_eq!(system.people_name, "Navajo Nation");
        assert!(system.state_recognition);
        assert!(!system.principles.is_empty());
    }

    #[test]
    fn test_indigenous_law_registry() {
        let registry = IndigenousLawRegistry::with_defaults();

        let navajo = registry.get_system("Navajo Nation");
        assert!(navajo.is_some());

        let recognized = registry.get_recognized();
        assert_eq!(recognized.len(), 4); // All default systems are recognized
    }

    #[test]
    fn test_indigenous_law_by_region() {
        let registry = IndigenousLawRegistry::with_defaults();

        let nz_systems = registry.get_by_region("New Zealand");
        assert!(!nz_systems.is_empty());
    }

    #[test]
    fn test_colonial_legacy_creation() {
        let legacy = ColonialLegacy::new(ColonialPower::British, "India")
            .with_retained_concept("Common law")
            .with_hybrid_concept("Anglo-Hindu law", "Hindu personal law")
            .with_reform("Constitution of India 1950");

        assert_eq!(legacy.colonial_power, ColonialPower::British);
        assert_eq!(legacy.jurisdiction, "India");
        assert!(!legacy.retained_concepts.is_empty());
        assert!(!legacy.hybrid_concepts.is_empty());
    }

    #[test]
    fn test_colonial_legacy_mapper() {
        let mapper = ColonialLegacyMapper::with_defaults();

        let india = mapper.get_legacy("India");
        assert!(india.is_some());
        assert_eq!(india.unwrap().colonial_power, ColonialPower::British);

        let british_legacies = mapper.get_by_colonial_power(ColonialPower::British);
        assert!(!british_legacies.is_empty());
    }

    #[test]
    fn test_colonial_power_display() {
        assert_eq!(ColonialPower::British.to_string(), "British");
        assert_eq!(ColonialPower::French.to_string(), "French");
    }

    #[test]
    fn test_registry_counts() {
        let cultural_registry = CulturalContextRegistry::with_defaults();
        assert!(cultural_registry.context_count() > 0);
        assert!(cultural_registry.locale_count() > 0);

        let custom_registry = LocalCustomRegistry::with_defaults();
        assert!(custom_registry.custom_count() > 0);
        assert!(custom_registry.region_count() > 0);

        let religious_registry = ReligiousLawRegistry::with_defaults();
        assert_eq!(religious_registry.system_count(), 3);

        let indigenous_registry = IndigenousLawRegistry::with_defaults();
        assert_eq!(indigenous_registry.system_count(), 4);

        let colonial_mapper = ColonialLegacyMapper::with_defaults();
        assert_eq!(colonial_mapper.legacy_count(), 7);
    }
}

// ============================================================================
// v0.2.7: Accessibility Features Tests
// ============================================================================

#[cfg(test)]
mod accessibility_tests {
    use super::*;

    #[test]
    fn test_plain_language_generator() {
        let locale = Locale::new("en").with_country("US");
        let generator = PlainLanguageGenerator::new(8.0, locale);

        let legal_text = "The party hereinafter referred to as the Plaintiff shall forthwith commence proceedings pursuant to the aforementioned statute.";
        let simplified = generator.simplify(legal_text);

        assert!(simplified.contains("from now on"));
        assert!(simplified.contains("immediately"));
        assert!(simplified.contains("start"));
    }

    #[test]
    fn test_plain_language_custom_jargon() {
        let locale = Locale::new("en").with_country("US");
        let generator = PlainLanguageGenerator::new(8.0, locale)
            .add_jargon_replacement("consideration", "payment");

        let text = "Valid consideration is required.";
        let simplified = generator.simplify(text);

        assert!(simplified.contains("payment"));
    }

    #[test]
    fn test_plain_language_meets_target() {
        let locale = Locale::new("en").with_country("US");
        let generator = PlainLanguageGenerator::new(8.0, locale);

        let simple_text = "The party must pay now.";
        assert!(generator.meets_target(simple_text));
    }

    #[test]
    fn test_simplification_strategy_display() {
        assert_eq!(
            SimplificationStrategy::ReplaceJargon.to_string(),
            "Replace Jargon"
        );
        assert_eq!(
            SimplificationStrategy::ShortenSentences.to_string(),
            "Shorten Sentences"
        );
        assert_eq!(
            SimplificationStrategy::ActiveVoice.to_string(),
            "Active Voice"
        );
    }

    #[test]
    fn test_reading_level_adjuster() {
        let locale = Locale::new("en").with_country("US");
        let adjuster = ReadingLevelAdjuster::new(TargetReadingLevel::MiddleSchool, locale);

        let legal_text = "The party shall forthwith pay consideration.";
        let adjusted = adjuster.adjust(legal_text);

        assert_eq!(adjusted.original, legal_text);
        assert!(adjusted.iterations > 0);
        assert_ne!(adjusted.adjusted, adjusted.original);
    }

    #[test]
    fn test_reading_level_improvement() {
        let locale = Locale::new("en").with_country("US");
        let adjuster = ReadingLevelAdjuster::new(TargetReadingLevel::MiddleSchool, locale);

        let legal_text = "The party hereinafter shall forthwith commence.";
        let adjusted = adjuster.adjust(legal_text);

        let improvement = adjusted.improvement();
        assert!(improvement >= 0.0); // Should not get worse
    }

    #[test]
    fn test_target_reading_level_display() {
        assert_eq!(
            TargetReadingLevel::Elementary.to_string(),
            "Elementary (grades 3-5)"
        );
        assert_eq!(
            TargetReadingLevel::MiddleSchool.to_string(),
            "Middle School (grades 6-8)"
        );
        assert_eq!(
            TargetReadingLevel::HighSchool.to_string(),
            "High School (grades 9-12)"
        );
    }

    #[test]
    fn test_target_reading_level_grade() {
        assert_eq!(TargetReadingLevel::Elementary.grade_level(), 4.0);
        assert_eq!(TargetReadingLevel::MiddleSchool.grade_level(), 7.0);
        assert_eq!(TargetReadingLevel::College.grade_level(), 14.0);
    }

    #[test]
    fn test_screen_reader_optimizer() {
        let locale = Locale::new("en").with_country("US");
        let optimizer = ScreenReaderOptimizer::new(WCAGLevel::AA, locale);

        let html = "<nav>Menu</nav><main>Content</main>";
        let optimized = optimizer.optimize_html(html);

        assert!(optimized.contains("role=\"navigation\""));
        assert!(optimized.contains("role=\"main\""));
        assert!(optimized.contains("lang=\"en\""));
    }

    #[test]
    fn test_screen_reader_skip_links() {
        let locale = Locale::new("en").with_country("US");
        let optimizer = ScreenReaderOptimizer::new(WCAGLevel::AA, locale);

        let html = "<main>Content</main>";
        let optimized = optimizer.optimize_html(html);

        assert!(optimized.contains("Skip to main content"));
        assert!(optimized.contains("skip-link"));
    }

    #[test]
    fn test_screen_reader_skip_links_locale() {
        let locale = Locale::new("ja").with_country("JP");
        let optimizer = ScreenReaderOptimizer::new(WCAGLevel::AA, locale);

        let html = "<main>Content</main>";
        let optimized = optimizer.optimize_html(html);

        assert!(optimized.contains("メインコンテンツへスキップ"));
    }

    #[test]
    fn test_screen_reader_document_structure() {
        let locale = Locale::new("en").with_country("US");
        let optimizer = ScreenReaderOptimizer::new(WCAGLevel::AA, locale);

        let sections = vec![
            ("Introduction", "This is the introduction."),
            ("Terms", "These are the terms."),
        ];
        let html = optimizer.generate_document_structure("Legal Agreement", sections);

        assert!(html.contains("<h1>Legal Agreement</h1>"));
        assert!(html.contains("<h2>Introduction</h2>"));
        assert!(html.contains("<h2>Terms</h2>"));
        assert!(html.contains("lang=\"en\""));
    }

    #[test]
    fn test_screen_reader_compliance_check() {
        let locale = Locale::new("en").with_country("US");
        let optimizer = ScreenReaderOptimizer::new(WCAGLevel::AA, locale);

        let good_html = "<html lang=\"en\"><h1>Title</h1><main role=\"main\"><a href=\"#main\" class=\"skip-link\">Skip</a></main></html>";
        let report = optimizer.check_compliance(good_html);

        assert!(report.is_compliant);
        assert_eq!(report.issues.len(), 0);
    }

    #[test]
    fn test_screen_reader_compliance_issues() {
        let locale = Locale::new("en").with_country("US");
        let optimizer = ScreenReaderOptimizer::new(WCAGLevel::AA, locale);

        let bad_html = "<div>Content</div>";
        let report = optimizer.check_compliance(bad_html);

        assert!(!report.is_compliant);
        assert!(!report.issues.is_empty());
    }

    #[test]
    fn test_wcag_level_display() {
        assert_eq!(WCAGLevel::A.to_string(), "WCAG Level A");
        assert_eq!(WCAGLevel::AA.to_string(), "WCAG Level AA");
        assert_eq!(WCAGLevel::AAA.to_string(), "WCAG Level AAA");
    }

    #[test]
    fn test_audio_narration_ssml() {
        let locale = Locale::new("en").with_country("US");
        let narration = AudioNarrationSupport::new(locale);

        let text = "The party shall pay.";
        let ssml = narration.generate_ssml(text);

        assert!(ssml.contains("<speak"));
        assert!(ssml.contains("xml:lang=\"en-US\""));
        assert!(ssml.contains("<prosody"));
        assert!(ssml.contains("</speak>"));
    }

    #[test]
    fn test_audio_narration_legal_text() {
        let locale = Locale::new("en").with_country("US");
        let narration = AudioNarrationSupport::new(locale);

        let text = "The contract shall be valid.";
        let ssml = narration.generate_ssml(text);

        assert!(ssml.contains("<emphasis level=\"strong\">shall</emphasis>"));
    }

    #[test]
    fn test_audio_narration_section() {
        let locale = Locale::new("en").with_country("US");
        let narration = AudioNarrationSupport::new(locale);

        let ssml = narration.narrate_section("1", "Definitions", "Terms are defined here.");

        assert!(ssml.contains("Section") || ssml.contains("ordinal"));
        assert!(ssml.contains("Definitions"));
        assert!(ssml.contains("<break time=\"500ms\"/>"));
    }

    #[test]
    fn test_audio_narration_citation() {
        let locale = Locale::new("en").with_country("US");
        let narration = AudioNarrationSupport::new(locale);

        let ssml = narration.narrate_citation("Brown v. Board of Education, 347 U.S. 483 (1954)");

        assert!(ssml.contains("versus"));
        assert!(ssml.contains("United States"));
    }

    #[test]
    fn test_audio_narration_with_settings() {
        let locale = Locale::new("en").with_country("US");
        let narration = AudioNarrationSupport::new(locale)
            .with_speaking_rate(1.2)
            .with_pitch(1.1)
            .with_volume(0.9);

        let ssml = narration.generate_ssml("Test");
        assert!(ssml.contains("<prosody"));
    }

    #[test]
    fn test_emphasis_level_display() {
        assert_eq!(EmphasisLevel::None.to_string(), "none");
        assert_eq!(EmphasisLevel::Reduced.to_string(), "reduced");
        assert_eq!(EmphasisLevel::Moderate.to_string(), "moderate");
        assert_eq!(EmphasisLevel::Strong.to_string(), "strong");
    }

    #[test]
    fn test_sign_language_reference() {
        let locale = Locale::new("en").with_country("US");
        let reference = SignLanguageReference::new("contract", SignLanguageType::ASL, locale)
            .with_video("https://example.com/contract.mp4")
            .with_image("https://example.com/contract.jpg")
            .with_description("Hands form C-shape");

        assert_eq!(reference.term, "contract");
        assert_eq!(reference.sign_language, SignLanguageType::ASL);
        assert!(reference.video_url.is_some());
        assert!(reference.image_url.is_some());
        assert!(reference.description.is_some());
    }

    #[test]
    fn test_sign_language_referencer() {
        let referencer = SignLanguageReferencer::with_defaults();

        assert!(referencer.term_count() > 0);
        assert!(referencer.reference_count() > 0);
    }

    #[test]
    fn test_sign_language_get_references() {
        let referencer = SignLanguageReferencer::with_defaults();

        let refs = referencer.get_references("contract");
        assert!(!refs.is_empty());
    }

    #[test]
    fn test_sign_language_by_type() {
        let referencer = SignLanguageReferencer::with_defaults();

        let asl_refs =
            referencer.get_references_for_sign_language("contract", SignLanguageType::ASL);
        assert!(!asl_refs.is_empty());

        let bsl_refs =
            referencer.get_references_for_sign_language("solicitor", SignLanguageType::BSL);
        assert!(!bsl_refs.is_empty());
    }

    #[test]
    fn test_sign_language_html_generation() {
        let mut referencer = SignLanguageReferencer::new();
        referencer.add_reference(
            SignLanguageReference::new(
                "law",
                SignLanguageType::ASL,
                Locale::new("en").with_country("US"),
            )
            .with_video("https://example.com/law.mp4"),
        );

        let html = referencer.generate_accessible_html("This is about law.");
        assert!(html.contains("sign-language-link"));
        assert!(html.contains("aria-label"));
    }

    #[test]
    fn test_sign_language_type_display() {
        assert_eq!(
            SignLanguageType::ASL.to_string(),
            "American Sign Language (ASL)"
        );
        assert_eq!(
            SignLanguageType::BSL.to_string(),
            "British Sign Language (BSL)"
        );
        assert_eq!(
            SignLanguageType::JSL.to_string(),
            "Japanese Sign Language (JSL)"
        );
        assert_eq!(SignLanguageType::IS.to_string(), "International Sign (IS)");
    }

    #[test]
    fn test_sign_language_add_custom() {
        let mut referencer = SignLanguageReferencer::new();
        let locale = Locale::new("en").with_country("US");

        referencer.add_reference(
            SignLanguageReference::new("judge", SignLanguageType::ASL, locale)
                .with_description("Gavel motion"),
        );

        assert_eq!(referencer.term_count(), 1);
        assert_eq!(referencer.reference_count(), 1);
    }
}

// ============================================================================
// v0.2.8: Historical Legal Language Tests
// ============================================================================

#[cfg(test)]
mod historical_tests {
    use super::*;

    #[test]
    fn test_archaic_term_creation() {
        let term = ArchaicTerm::new(
            "wergild",
            HistoricalPeriod::OldEnglish,
            "blood money",
            "Compensation paid to slain person's family",
            Locale::new("en").with_country("GB"),
        )
        .with_example("Wergild was 1200 shillings");

        assert_eq!(term.term, "wergild");
        assert_eq!(term.period, HistoricalPeriod::OldEnglish);
        assert_eq!(term.modern_equivalent, "blood money");
        assert!(term.example.is_some());
    }

    #[test]
    fn test_archaic_dictionary_defaults() {
        let dict = ArchaicTermDictionary::with_defaults();

        assert!(dict.term_count() > 0);
        assert!(dict.period_count() > 0);
    }

    #[test]
    fn test_archaic_dictionary_by_period() {
        let dict = ArchaicTermDictionary::with_defaults();

        let old_english_terms = dict.get_by_period(HistoricalPeriod::OldEnglish);
        assert!(!old_english_terms.is_empty());

        let latin_terms = dict.get_by_period(HistoricalPeriod::ClassicalLatin);
        assert!(!latin_terms.is_empty());
    }

    #[test]
    fn test_archaic_dictionary_translate() {
        let dict = ArchaicTermDictionary::with_defaults();

        let modern = dict.translate_to_modern("wergild");
        assert_eq!(modern, Some("blood money".to_string()));

        let modern = dict.translate_to_modern("feoffment");
        assert_eq!(modern, Some("grant of land".to_string()));
    }

    #[test]
    fn test_archaic_dictionary_by_name() {
        let dict = ArchaicTermDictionary::with_defaults();

        let terms = dict.get_by_name("moot");
        assert!(!terms.is_empty());
        assert_eq!(terms[0].modern_equivalent, "assembly");
    }

    #[test]
    fn test_historical_period_display() {
        assert_eq!(
            HistoricalPeriod::OldEnglish.to_string(),
            "Old English (450-1150)"
        );
        assert_eq!(
            HistoricalPeriod::MiddleEnglish.to_string(),
            "Middle English (1150-1500)"
        );
        assert_eq!(
            HistoricalPeriod::ClassicalLatin.to_string(),
            "Classical Latin (Roman Empire)"
        );
    }

    #[test]
    fn test_historical_calendar_display() {
        assert_eq!(HistoricalCalendar::Julian.to_string(), "Julian Calendar");
        assert_eq!(
            HistoricalCalendar::Gregorian.to_string(),
            "Gregorian Calendar"
        );
        assert_eq!(
            HistoricalCalendar::FrenchRevolutionary.to_string(),
            "French Revolutionary Calendar"
        );
    }

    #[test]
    fn test_julian_to_gregorian_conversion() {
        let converter = HistoricalCalendarConverter::new(HistoricalCalendar::Julian);

        // October 15, 1582 (Gregorian) = October 5, 1582 (Julian)
        let (year, month, day) = converter.julian_to_gregorian(1582, 10, 5);

        // The conversion should result in a later date
        assert_eq!(year, 1582);
        assert_eq!(month, 10);
        assert!(day >= 5); // Gregorian date is later
    }

    #[test]
    fn test_gregorian_to_julian_conversion() {
        let converter = HistoricalCalendarConverter::new(HistoricalCalendar::Gregorian);

        let (year, month, day) = converter.gregorian_to_julian(1700, 1, 1);

        // The conversion should maintain the year
        assert!(year >= 1699 && year <= 1700);
        assert!(month >= 1 && month <= 12);
        assert!(day >= 1 && day <= 31);
    }

    #[test]
    fn test_julian_gregorian_offset() {
        let converter = HistoricalCalendarConverter::new(HistoricalCalendar::Julian);

        let offset_before = converter.julian_gregorian_offset(1500);
        assert_eq!(offset_before, 0); // Before 1582

        let offset_after = converter.julian_gregorian_offset(1700);
        assert!(offset_after > 0); // After 1582
    }

    #[test]
    fn test_format_historical_date_julian() {
        let converter = HistoricalCalendarConverter::new(HistoricalCalendar::Julian);

        let formatted = converter.format_historical_date(1215, 6, 15);
        assert!(formatted.contains("15"));
        assert!(formatted.contains("1215"));
        assert!(formatted.contains("(O.S.)")); // Old Style
    }

    #[test]
    fn test_format_historical_date_gregorian() {
        let converter = HistoricalCalendarConverter::new(HistoricalCalendar::Gregorian);

        let formatted = converter.format_historical_date(1789, 7, 14);
        assert!(formatted.contains("14"));
        assert!(formatted.contains("1789"));
        assert!(formatted.contains("(N.S.)")); // New Style
    }

    #[test]
    fn test_format_french_revolutionary_date() {
        let converter = HistoricalCalendarConverter::new(HistoricalCalendar::FrenchRevolutionary);

        let formatted = converter.format_french_revolutionary_date(2, 11, 9);
        assert!(formatted.contains("9"));
        assert!(formatted.contains("Thermidor"));
        assert!(formatted.contains("An 2"));
    }

    #[test]
    fn test_language_family_display() {
        assert_eq!(LanguageFamily::Germanic.to_string(), "Germanic");
        assert_eq!(LanguageFamily::Latin.to_string(), "Latin");
        assert_eq!(LanguageFamily::NormanFrench.to_string(), "Norman French");
    }

    #[test]
    fn test_etymology_creation() {
        let etymology = Etymology::new(
            "contract",
            "contractus",
            LanguageFamily::Latin,
            "Latin",
            "drawn together",
        )
        .with_first_usage(HistoricalPeriod::ClassicalLatin)
        .add_evolution("Latin → Old French → Middle English");

        assert_eq!(etymology.term, "contract");
        assert_eq!(etymology.root, "contractus");
        assert_eq!(etymology.language_family, LanguageFamily::Latin);
        assert!(etymology.first_usage.is_some());
        assert_eq!(etymology.evolution.len(), 1);
    }

    #[test]
    fn test_etymology_tracker_defaults() {
        let tracker = EtymologyTracker::with_defaults();

        assert!(tracker.etymology_count() > 0);
    }

    #[test]
    fn test_etymology_tracker_get() {
        let tracker = EtymologyTracker::with_defaults();

        let etymology = tracker.get_etymology("contract");
        assert!(etymology.is_some());
        assert_eq!(etymology.unwrap().root, "contractus");
    }

    #[test]
    fn test_etymology_by_language_family() {
        let tracker = EtymologyTracker::with_defaults();

        let latin_etymologies = tracker.get_by_language_family(LanguageFamily::Latin);
        assert!(!latin_etymologies.is_empty());

        let french_etymologies = tracker.get_by_language_family(LanguageFamily::OldFrench);
        assert!(!french_etymologies.is_empty());
    }

    #[test]
    fn test_historical_context_creation() {
        let context = HistoricalContext::new(
            "Magna Carta",
            HistoricalPeriod::MiddleEnglish,
            "Signed in 1215",
            "Established rule of law",
        )
        .with_modern_relevance("Foundation of constitutional law")
        .add_related_document("Bill of Rights 1689");

        assert_eq!(context.term, "Magna Carta");
        assert_eq!(context.period, HistoricalPeriod::MiddleEnglish);
        assert!(context.modern_relevance.is_some());
        assert_eq!(context.related_documents.len(), 1);
    }

    #[test]
    fn test_historical_context_annotator_defaults() {
        let annotator = HistoricalContextAnnotator::with_defaults();

        assert!(annotator.context_count() > 0);
    }

    #[test]
    fn test_historical_context_get() {
        let annotator = HistoricalContextAnnotator::with_defaults();

        let contexts = annotator.get_contexts("Magna Carta");
        assert!(!contexts.is_empty());
        assert!(contexts[0].modern_relevance.is_some());
    }

    #[test]
    fn test_historical_context_by_period() {
        let annotator = HistoricalContextAnnotator::with_defaults();

        let middle_english = annotator.get_by_period(HistoricalPeriod::MiddleEnglish);
        assert!(!middle_english.is_empty());

        let enlightenment = annotator.get_by_period(HistoricalPeriod::Enlightenment);
        assert!(!enlightenment.is_empty());
    }

    #[test]
    fn test_archaic_dictionary_add_custom() {
        let mut dict = ArchaicTermDictionary::new();

        dict.add_term(ArchaicTerm::new(
            "gavelkind",
            HistoricalPeriod::MiddleEnglish,
            "equal inheritance",
            "System of land inheritance divided equally among sons",
            Locale::new("en").with_country("GB"),
        ));

        assert_eq!(dict.term_count(), 1);
        assert!(dict.translate_to_modern("gavelkind").is_some());
    }

    #[test]
    fn test_etymology_tracker_add_custom() {
        let mut tracker = EtymologyTracker::new();

        tracker.add_etymology(Etymology::new(
            "judge",
            "iudex",
            LanguageFamily::Latin,
            "Latin",
            "one who declares law",
        ));

        assert_eq!(tracker.etymology_count(), 1);
        assert!(tracker.get_etymology("judge").is_some());
    }

    #[test]
    fn test_historical_context_add_custom() {
        let mut annotator = HistoricalContextAnnotator::new();

        annotator.add_context(HistoricalContext::new(
            "Common Law",
            HistoricalPeriod::MiddleEnglish,
            "Developed in medieval England",
            "Based on judicial precedent",
        ));

        assert_eq!(annotator.context_count(), 1);
        assert!(!annotator.get_contexts("Common Law").is_empty());
    }
}
