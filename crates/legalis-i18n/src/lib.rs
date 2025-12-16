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

    /// Checks if this locale matches another locale (considering regional variations).
    /// Returns true if the locales match exactly or if they share the same language/country.
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

/// Currency formatter for monetary values.
#[derive(Debug, Clone)]
pub struct CurrencyFormatter {
    locale: Locale,
}

impl CurrencyFormatter {
    /// Creates a new currency formatter.
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
            CalendarSystem::Japanese => {
                // Simplified conversion to Japanese era
                self.to_japanese_calendar(year, month, day)
            }
            CalendarSystem::Buddhist => {
                // Buddhist era = Gregorian year + 543
                CalendarDate::new(system, year + 543, month, day)
            }
            CalendarSystem::Islamic => {
                // Simplified Islamic calendar conversion
                // Note: Real conversion is complex due to lunar calendar
                self.to_islamic_approximate(year, month, day)
            }
            _ => CalendarDate::new(CalendarSystem::Gregorian, year, month, day),
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

    #[allow(dead_code)]
    fn to_islamic_approximate(&self, year: i32, _month: u32, _day: u32) -> CalendarDate {
        // Very simplified approximation: Islamic year ≈ (Gregorian year - 622) * 1.03
        // Note: This is not accurate for precise date conversion
        let islamic_year = ((year - 622) as f64 * 1.03) as i32;
        CalendarDate::new(CalendarSystem::Islamic, islamic_year, 1, 1)
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
                format!("{} AH", date.year)
            }
            _ => {
                format!("{}-{:02}-{:02}", date.year, date.month, date.day)
            }
        }
    }
}

/// Number formatter for locale-specific number formatting.
#[derive(Debug, Clone)]
pub struct NumberFormatter {
    locale: Locale,
}

impl NumberFormatter {
    /// Creates a new number formatter.
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
}
