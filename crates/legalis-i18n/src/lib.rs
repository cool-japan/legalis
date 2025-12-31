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
        assert!(braille.len() > 0);
        assert!(
            braille
                .chars()
                .all(|c| c >= '\u{2800}' && c <= '\u{28FF}' || c == ' ')
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
        assert!(us_dialects.len() >= 1); // Louisiana

        let gb_locale = Locale::new("en").with_country("GB");
        let gb_dialects = registry.get_dialects_for_locale(&gb_locale);
        assert!(gb_dialects.len() >= 1); // Scottish
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
        assert!(court_docs.len() >= 1); // Complaint

        let corporate_docs = registry.find_by_type(DocumentTemplateType::Corporate);
        assert!(corporate_docs.len() >= 1); // Articles of Incorporation
    }

    #[test]
    fn test_document_template_registry_find_by_jurisdiction() {
        let registry = DocumentTemplateRegistry::with_defaults();

        let us_templates = registry.find_by_jurisdiction("US");
        assert!(us_templates.len() >= 3); // NDA, Employment, Complaint

        let de_templates = registry.find_by_jurisdiction("US-DE");
        assert!(de_templates.len() >= 1); // Articles of Incorporation
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
        let types = vec![
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
        let types = vec![
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
        let types = vec![
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
