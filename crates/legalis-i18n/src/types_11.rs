//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

use super::functions::{I18nResult, TranslationService};
use super::types_3::{NeuralMachineTranslator, RegionalConceptMapping};
use super::types_4::{HistoricalPeriod, MTTranslation, TermPreservationMode};
use super::types_5::SimilarityScore;
use super::types_6::ExtendedLanguage;
use super::types_7::TranslationMemory;
use super::types_8::LegalDictionary;
use super::types_9::{CalendarSystem, DisambiguationType, TranscriptionSegment};
use super::types_10::Locale;
use super::types_13::SubtitleCue;

/// Extended language registry with 50+ languages.
#[derive(Debug, Clone)]
pub struct ExtendedLanguageRegistry {
    /// Languages indexed by code.
    pub(super) languages: HashMap<String, ExtendedLanguage>,
    /// Languages by family.
    pub(super) by_family: HashMap<String, Vec<String>>,
    /// Low-resource languages.
    pub(super) low_resource: Vec<String>,
}
impl ExtendedLanguageRegistry {
    /// Creates a new registry.
    pub fn new() -> Self {
        Self {
            languages: HashMap::new(),
            by_family: HashMap::new(),
            low_resource: Vec::new(),
        }
    }
    /// Creates a registry with 50+ languages.
    #[allow(clippy::too_many_lines)]
    pub fn with_extended_set() -> Self {
        let mut registry = Self::new();
        registry.add_language(
            ExtendedLanguage::new("en", "English", "English", "Germanic", "Latin")
                .with_speakers(1500.0)
                .add_official_country("US")
                .add_official_country("GB")
                .add_official_country("AU"),
        );
        registry.add_language(
            ExtendedLanguage::new("zh", "Chinese", "中文", "Sino-Tibetan", "Han")
                .with_speakers(1100.0)
                .add_official_country("CN")
                .add_official_country("TW"),
        );
        registry.add_language(
            ExtendedLanguage::new("hi", "Hindi", "हिन्दी", "Indo-Aryan", "Devanagari")
                .with_speakers(600.0)
                .add_official_country("IN"),
        );
        registry.add_language(
            ExtendedLanguage::new("es", "Spanish", "Español", "Romance", "Latin")
                .with_speakers(500.0)
                .add_official_country("ES")
                .add_official_country("MX"),
        );
        registry.add_language(
            ExtendedLanguage::new("fr", "French", "Français", "Romance", "Latin")
                .with_speakers(280.0)
                .add_official_country("FR")
                .add_official_country("CA"),
        );
        registry.add_language(
            ExtendedLanguage::new("ar", "Arabic", "العربية", "Semitic", "Arabic")
                .with_speakers(310.0)
                .add_official_country("SA")
                .add_official_country("EG"),
        );
        registry.add_language(
            ExtendedLanguage::new("bn", "Bengali", "বাংলা", "Indo-Aryan", "Bengali")
                .with_speakers(265.0)
                .add_official_country("BD")
                .add_official_country("IN"),
        );
        registry.add_language(
            ExtendedLanguage::new("pt", "Portuguese", "Português", "Romance", "Latin")
                .with_speakers(260.0)
                .add_official_country("BR")
                .add_official_country("PT"),
        );
        registry.add_language(
            ExtendedLanguage::new("ru", "Russian", "Русский", "Slavic", "Cyrillic")
                .with_speakers(258.0)
                .add_official_country("RU"),
        );
        registry.add_language(
            ExtendedLanguage::new("ja", "Japanese", "日本語", "Japonic", "Kanji/Kana")
                .with_speakers(125.0)
                .add_official_country("JP"),
        );
        registry.add_language(
            ExtendedLanguage::new("de", "German", "Deutsch", "Germanic", "Latin")
                .with_speakers(134.0)
                .add_official_country("DE")
                .add_official_country("AT"),
        );
        registry.add_language(
            ExtendedLanguage::new("ko", "Korean", "한국어", "Koreanic", "Hangul")
                .with_speakers(81.0)
                .add_official_country("KR")
                .add_official_country("KP"),
        );
        registry.add_language(
            ExtendedLanguage::new("vi", "Vietnamese", "Tiếng Việt", "Austroasiatic", "Latin")
                .with_speakers(85.0)
                .add_official_country("VN"),
        );
        registry.add_language(
            ExtendedLanguage::new("it", "Italian", "Italiano", "Romance", "Latin")
                .with_speakers(85.0)
                .add_official_country("IT"),
        );
        registry.add_language(
            ExtendedLanguage::new("tr", "Turkish", "Türkçe", "Turkic", "Latin")
                .with_speakers(88.0)
                .add_official_country("TR"),
        );
        registry.add_language(
            ExtendedLanguage::new("sw", "Swahili", "Kiswahili", "Bantu", "Latin")
                .with_speakers(200.0)
                .add_official_country("TZ")
                .add_official_country("KE"),
        );
        registry.add_language(
            ExtendedLanguage::new("mr", "Marathi", "मराठी", "Indo-Aryan", "Devanagari")
                .with_speakers(95.0)
                .add_official_country("IN"),
        );
        registry.add_language(
            ExtendedLanguage::new("ta", "Tamil", "தமிழ்", "Dravidian", "Tamil")
                .with_speakers(85.0)
                .add_official_country("IN")
                .add_official_country("LK"),
        );
        registry.add_language(
            ExtendedLanguage::new("te", "Telugu", "తెలుగు", "Dravidian", "Telugu")
                .with_speakers(95.0)
                .add_official_country("IN"),
        );
        registry.add_language(
            ExtendedLanguage::new("ur", "Urdu", "اردو", "Indo-Aryan", "Arabic")
                .with_speakers(232.0)
                .add_official_country("PK")
                .add_official_country("IN"),
        );
        registry.add_language(
            ExtendedLanguage::new(
                "id",
                "Indonesian",
                "Bahasa Indonesia",
                "Austronesian",
                "Latin",
            )
            .with_speakers(199.0)
            .add_official_country("ID"),
        );
        registry.add_language(
            ExtendedLanguage::new("th", "Thai", "ไทย", "Tai-Kadai", "Thai")
                .with_speakers(69.0)
                .add_official_country("TH"),
        );
        registry.add_language(
            ExtendedLanguage::new("pl", "Polish", "Polski", "Slavic", "Latin")
                .with_speakers(45.0)
                .add_official_country("PL"),
        );
        registry.add_language(
            ExtendedLanguage::new("uk", "Ukrainian", "Українська", "Slavic", "Cyrillic")
                .with_speakers(41.0)
                .add_official_country("UA"),
        );
        registry.add_language(
            ExtendedLanguage::new("fa", "Persian", "فارسی", "Iranian", "Arabic")
                .with_speakers(110.0)
                .add_official_country("IR"),
        );
        registry.add_language(
            ExtendedLanguage::new("my", "Burmese", "မြန်မာဘာသာ", "Sino-Tibetan", "Burmese")
                .with_speakers(43.0)
                .add_official_country("MM"),
        );
        registry.add_language(
            ExtendedLanguage::new("km", "Khmer", "ភាសាខ្មែរ", "Austroasiatic", "Khmer")
                .with_speakers(16.0)
                .add_official_country("KH"),
        );
        registry.add_language(
            ExtendedLanguage::new("lo", "Lao", "ພາສາລາວ", "Tai-Kadai", "Lao")
                .with_speakers(30.0)
                .add_official_country("LA"),
        );
        registry.add_language(
            ExtendedLanguage::new("tl", "Tagalog", "Tagalog", "Austronesian", "Latin")
                .with_speakers(82.0)
                .add_official_country("PH"),
        );
        registry.add_language(
            ExtendedLanguage::new("ha", "Hausa", "Hausa", "Afro-Asiatic", "Latin")
                .with_speakers(85.0)
                .add_official_country("NG")
                .as_low_resource(),
        );
        registry.add_language(
            ExtendedLanguage::new("yo", "Yoruba", "Yorùbá", "Niger-Congo", "Latin")
                .with_speakers(45.0)
                .add_official_country("NG")
                .as_low_resource(),
        );
        registry.add_language(
            ExtendedLanguage::new("ig", "Igbo", "Igbo", "Niger-Congo", "Latin")
                .with_speakers(44.0)
                .add_official_country("NG")
                .as_low_resource(),
        );
        registry.add_language(
            ExtendedLanguage::new("am", "Amharic", "አማርኛ", "Semitic", "Ethiopic")
                .with_speakers(57.0)
                .add_official_country("ET")
                .as_low_resource(),
        );
        registry.add_language(
            ExtendedLanguage::new("zu", "Zulu", "isiZulu", "Bantu", "Latin")
                .with_speakers(27.0)
                .add_official_country("ZA")
                .as_low_resource(),
        );
        registry.add_language(
            ExtendedLanguage::new("xh", "Xhosa", "isiXhosa", "Bantu", "Latin")
                .with_speakers(19.0)
                .add_official_country("ZA")
                .as_low_resource(),
        );
        registry.add_language(
            ExtendedLanguage::new("kk", "Kazakh", "Қазақша", "Turkic", "Cyrillic")
                .with_speakers(18.0)
                .add_official_country("KZ")
                .as_low_resource(),
        );
        registry.add_language(
            ExtendedLanguage::new("uz", "Uzbek", "Oʻzbekcha", "Turkic", "Latin")
                .with_speakers(34.0)
                .add_official_country("UZ")
                .as_low_resource(),
        );
        registry.add_language(
            ExtendedLanguage::new("ky", "Kyrgyz", "Кыргызча", "Turkic", "Cyrillic")
                .with_speakers(5.0)
                .add_official_country("KG")
                .as_low_resource(),
        );
        registry.add_language(
            ExtendedLanguage::new("he", "Hebrew", "עברית", "Semitic", "Hebrew")
                .with_speakers(9.0)
                .add_official_country("IL"),
        );
        registry.add_language(
            ExtendedLanguage::new("ps", "Pashto", "پښتو", "Iranian", "Arabic")
                .with_speakers(60.0)
                .add_official_country("AF")
                .as_low_resource(),
        );
        registry.add_language(
            ExtendedLanguage::new("ku", "Kurdish", "Kurdî", "Iranian", "Latin/Arabic")
                .with_speakers(30.0)
                .as_low_resource(),
        );
        registry.add_language(
            ExtendedLanguage::new("gu", "Gujarati", "ગુજરાતી", "Indo-Aryan", "Gujarati")
                .with_speakers(60.0)
                .add_official_country("IN"),
        );
        registry.add_language(
            ExtendedLanguage::new("kn", "Kannada", "ಕನ್ನಡ", "Dravidian", "Kannada")
                .with_speakers(56.0)
                .add_official_country("IN"),
        );
        registry.add_language(
            ExtendedLanguage::new("ml", "Malayalam", "മലയാളം", "Dravidian", "Malayalam")
                .with_speakers(38.0)
                .add_official_country("IN"),
        );
        registry.add_language(
            ExtendedLanguage::new("pa", "Punjabi", "ਪੰਜਾਬੀ", "Indo-Aryan", "Gurmukhi")
                .with_speakers(125.0)
                .add_official_country("IN")
                .add_official_country("PK"),
        );
        registry.add_language(
            ExtendedLanguage::new("si", "Sinhala", "සිංහල", "Indo-Aryan", "Sinhala")
                .with_speakers(19.0)
                .add_official_country("LK")
                .as_low_resource(),
        );
        registry.add_language(
            ExtendedLanguage::new("ne", "Nepali", "नेपाली", "Indo-Aryan", "Devanagari")
                .with_speakers(16.0)
                .add_official_country("NP")
                .as_low_resource(),
        );
        registry.add_language(
            ExtendedLanguage::new("nl", "Dutch", "Nederlands", "Germanic", "Latin")
                .with_speakers(24.0)
                .add_official_country("NL")
                .add_official_country("BE"),
        );
        registry.add_language(
            ExtendedLanguage::new("ro", "Romanian", "Română", "Romance", "Latin")
                .with_speakers(26.0)
                .add_official_country("RO"),
        );
        registry.add_language(
            ExtendedLanguage::new("cs", "Czech", "Čeština", "Slavic", "Latin")
                .with_speakers(13.0)
                .add_official_country("CZ"),
        );
        registry.add_language(
            ExtendedLanguage::new("hu", "Hungarian", "Magyar", "Uralic", "Latin")
                .with_speakers(13.0)
                .add_official_country("HU"),
        );
        registry.add_language(
            ExtendedLanguage::new("el", "Greek", "Ελληνικά", "Hellenic", "Greek")
                .with_speakers(13.0)
                .add_official_country("GR"),
        );
        registry.add_language(
            ExtendedLanguage::new("sv", "Swedish", "Svenska", "Germanic", "Latin")
                .with_speakers(13.0)
                .add_official_country("SE"),
        );
        registry.add_language(
            ExtendedLanguage::new("bg", "Bulgarian", "Български", "Slavic", "Cyrillic")
                .with_speakers(8.0)
                .add_official_country("BG")
                .as_low_resource(),
        );
        registry.add_language(
            ExtendedLanguage::new("ms", "Malay", "Bahasa Melayu", "Austronesian", "Latin")
                .with_speakers(290.0)
                .add_official_country("MY")
                .add_official_country("SG"),
        );
        registry.add_language(
            ExtendedLanguage::new("tl", "Filipino", "Filipino", "Austronesian", "Latin")
                .with_speakers(82.0)
                .add_official_country("PH"),
        );
        registry.add_language(
            ExtendedLanguage::new("mg", "Malagasy", "Malagasy", "Austronesian", "Latin")
                .with_speakers(25.0)
                .add_official_country("MG")
                .as_low_resource(),
        );
        registry.add_language(
            ExtendedLanguage::new("rw", "Kinyarwanda", "Kinyarwanda", "Bantu", "Latin")
                .with_speakers(12.0)
                .add_official_country("RW")
                .as_low_resource(),
        );
        registry.add_language(
            ExtendedLanguage::new("sn", "Shona", "chiShona", "Bantu", "Latin")
                .with_speakers(14.0)
                .add_official_country("ZW")
                .as_low_resource(),
        );
        registry.add_language(
            ExtendedLanguage::new("so", "Somali", "Soomaali", "Cushitic", "Latin")
                .with_speakers(21.0)
                .add_official_country("SO")
                .as_low_resource(),
        );
        registry.add_language(
            ExtendedLanguage::new("sd", "Sindhi", "سنڌي", "Indo-Aryan", "Arabic")
                .with_speakers(31.0)
                .add_official_country("PK")
                .as_low_resource(),
        );
        registry
    }
    /// Adds a language to the registry.
    pub fn add_language(&mut self, lang: ExtendedLanguage) {
        if lang.low_resource {
            self.low_resource.push(lang.code.clone());
        }
        self.by_family
            .entry(lang.family.clone())
            .or_default()
            .push(lang.code.clone());
        self.languages.insert(lang.code.clone(), lang);
    }
    /// Gets a language by code.
    pub fn get_language(&self, code: &str) -> Option<&ExtendedLanguage> {
        self.languages.get(code)
    }
    /// Gets all languages in a family.
    pub fn get_by_family(&self, family: &str) -> Vec<&ExtendedLanguage> {
        self.by_family
            .get(family)
            .map(|codes| codes.iter().filter_map(|c| self.languages.get(c)).collect())
            .unwrap_or_default()
    }
    /// Gets all low-resource languages.
    pub fn get_low_resource_languages(&self) -> Vec<&ExtendedLanguage> {
        self.low_resource
            .iter()
            .filter_map(|c| self.languages.get(c))
            .collect()
    }
    /// Gets total language count.
    pub fn language_count(&self) -> usize {
        self.languages.len()
    }
    /// Gets all language codes.
    pub fn all_codes(&self) -> Vec<String> {
        self.languages.keys().cloned().collect()
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
        ssml.push_str(&format!(
            "<prosody rate=\"{}\" pitch=\"{}%\" volume=\"{}\">\n",
            self.format_rate(),
            (self.pitch * 100.0) as i32,
            self.format_volume()
        ));
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
        result = result.replace(
            " v. ",
            " <break time=\"300ms\"/> versus <break time=\"200ms\"/> ",
        );
        result = result.replace(
            "Section ",
            "<say-as interpret-as=\"ordinal\">Section</say-as> ",
        );
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
        let narration = citation
            .replace(" v. ", " versus ")
            .replace("U.S.", "United States")
            .replace("F.3d", "Federal Reporter, Third Series")
            .replace("F.2d", "Federal Reporter, Second Series")
            .replace("S.Ct.", "Supreme Court Reporter");
        self.generate_ssml(&narration)
    }
}
/// Interpretation mode for real-time translation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InterpretationMode {
    /// Consecutive interpretation (speaker pauses for interpretation).
    Consecutive,
    /// Simultaneous interpretation (interpreter speaks concurrently).
    Simultaneous,
    /// Whispered interpretation (chuchotage).
    Whispered,
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
        let exact_matches = self.memory.find_exact(text, source_locale, target_locale);
        if let Some(entry) = exact_matches.first() {
            return Ok(entry.target_text.clone());
        }
        let fuzzy_matches = self
            .memory
            .find_fuzzy(text, source_locale, target_locale, 0.9);
        if let Some((entry, _)) = fuzzy_matches.first() {
            return Ok(entry.target_text.clone());
        }
        for service in &self.services {
            if !service.is_available() {
                continue;
            }
            match service.translate(text, source_locale, target_locale) {
                Ok(translation) => {
                    self.memory.add_translation(
                        text.to_string(),
                        source_locale.clone(),
                        translation.clone(),
                        target_locale.clone(),
                    );
                    return Ok(translation);
                }
                Err(_) => {
                    continue;
                }
            }
        }
        Err(I18nError::TranslationMissing {
            key: text.to_string(),
            locale: target_locale.tag(),
        })
    }
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
/// Semantic embedding model for multilingual legal text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmbeddingModel {
    /// Multilingual BERT (mBERT).
    MultilinguralBERT,
    /// XLM-RoBERTa for cross-lingual understanding.
    XLMRoBERTa,
    /// LaBSE (Language-agnostic BERT Sentence Encoder).
    LaBSE,
    /// Legal-domain fine-tuned multilingual model.
    LegalMultilingual,
    /// Custom embedding model.
    Custom,
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
/// Accessibility subtitle generator for legal proceedings.
#[derive(Debug, Clone)]
pub struct AccessibilitySubtitleGenerator {
    /// Primary language locale.
    pub primary_locale: Locale,
    /// Whether to include speaker labels.
    pub include_speakers: bool,
    /// Whether to include sound descriptions.
    pub include_sound_descriptions: bool,
    /// Maximum characters per line.
    pub max_chars_per_line: usize,
    /// Whether to generate multi-language subtitles.
    pub multilingual: bool,
    /// Secondary languages for multilingual subtitles.
    pub secondary_locales: Vec<Locale>,
}
impl AccessibilitySubtitleGenerator {
    /// Creates a new accessibility subtitle generator.
    pub fn new(primary_locale: Locale) -> Self {
        Self {
            primary_locale,
            include_speakers: true,
            include_sound_descriptions: true,
            max_chars_per_line: 42,
            multilingual: false,
            secondary_locales: Vec::new(),
        }
    }
    /// Creates a generator for multilingual court proceedings.
    pub fn for_multilingual_court(primary_locale: Locale, secondary_locales: Vec<Locale>) -> Self {
        Self {
            primary_locale,
            include_speakers: true,
            include_sound_descriptions: true,
            max_chars_per_line: 42,
            multilingual: true,
            secondary_locales,
        }
    }
    /// Enables or disables speaker labels.
    pub fn with_speaker_labels(mut self, enable: bool) -> Self {
        self.include_speakers = enable;
        self
    }
    /// Enables or disables sound descriptions.
    pub fn with_sound_descriptions(mut self, enable: bool) -> Self {
        self.include_sound_descriptions = enable;
        self
    }
    /// Sets maximum characters per line.
    pub fn with_max_chars(mut self, max_chars: usize) -> Self {
        self.max_chars_per_line = max_chars;
        self
    }
    /// Generates subtitle cues from transcription segments.
    pub fn generate_cues(&self, segments: &[TranscriptionSegment]) -> Vec<SubtitleCue> {
        let mut cues = Vec::new();
        for segment in segments {
            let lines = self.split_text(&segment.text);
            let duration_per_line = segment.duration_ms() / lines.len() as u64;
            for (i, line) in lines.iter().enumerate() {
                let start = segment.start_ms + (i as u64 * duration_per_line);
                let end = start + duration_per_line;
                let mut cue = SubtitleCue::new(line.clone(), start, end, segment.locale.clone());
                if self.include_speakers
                    && let Some(ref speaker) = segment.speaker
                {
                    cue = cue.with_speaker(speaker.clone());
                }
                cues.push(cue);
            }
        }
        cues
    }
    /// Generates WebVTT format subtitles.
    pub fn generate_webvtt(&self, segments: &[TranscriptionSegment]) -> String {
        let mut webvtt = String::from("WEBVTT\n\n");
        let cues = self.generate_cues(segments);
        for cue in cues {
            webvtt.push_str(&cue.to_webvtt());
            webvtt.push('\n');
        }
        webvtt
    }
    /// Generates SRT format subtitles.
    pub fn generate_srt(&self, segments: &[TranscriptionSegment]) -> String {
        let mut srt = String::new();
        let cues = self.generate_cues(segments);
        for (i, cue) in cues.iter().enumerate() {
            srt.push_str(&cue.to_srt((i + 1) as u32));
        }
        srt
    }
    /// Splits text into lines respecting max characters per line.
    fn split_text(&self, text: &str) -> Vec<String> {
        let mut lines = Vec::new();
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut current_line = String::new();
        for word in words {
            let test_line = if current_line.is_empty() {
                word.to_string()
            } else {
                format!("{} {}", current_line, word)
            };
            if test_line.len() <= self.max_chars_per_line {
                current_line = test_line;
            } else {
                if !current_line.is_empty() {
                    lines.push(current_line.clone());
                }
                current_line = word.to_string();
            }
        }
        if !current_line.is_empty() {
            lines.push(current_line);
        }
        if lines.is_empty() {
            lines.push(text.to_string());
        }
        lines
    }
    /// Adds a sound description cue.
    pub fn add_sound_description(
        &self,
        cues: &mut Vec<SubtitleCue>,
        description: &str,
        start_ms: u64,
        end_ms: u64,
    ) {
        if self.include_sound_descriptions {
            let cue = SubtitleCue::new(
                format!("[{}]", description),
                start_ms,
                end_ms,
                self.primary_locale.clone(),
            );
            cues.push(cue);
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
/// Document similarity calculator for legal documents.
pub struct DocumentSimilarityCalculator {
    /// Similarity threshold
    pub(crate) threshold: f64,
}
impl DocumentSimilarityCalculator {
    /// Creates a new similarity calculator.
    pub fn new() -> Self {
        Self { threshold: 0.5 }
    }
    /// Calculates Jaccard similarity between two documents.
    pub fn jaccard_similarity(&self, doc1: &str, doc2: &str) -> f64 {
        let words1: std::collections::HashSet<&str> = doc1.split_whitespace().collect();
        let words2: std::collections::HashSet<&str> = doc2.split_whitespace().collect();
        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();
        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }
    /// Calculates cosine similarity based on term frequency.
    pub fn cosine_similarity(&self, doc1: &str, doc2: &str) -> f64 {
        let words1: Vec<&str> = doc1.split_whitespace().collect();
        let words2: Vec<&str> = doc2.split_whitespace().collect();
        let mut all_terms = std::collections::HashSet::new();
        for word in words1.iter().chain(words2.iter()) {
            all_terms.insert(*word);
        }
        let mut vec1 = Vec::new();
        let mut vec2 = Vec::new();
        for term in &all_terms {
            vec1.push(words1.iter().filter(|w| *w == term).count() as f64);
            vec2.push(words2.iter().filter(|w| *w == term).count() as f64);
        }
        let dot_product: f64 = vec1.iter().zip(vec2.iter()).map(|(a, b)| a * b).sum();
        let mag1: f64 = vec1.iter().map(|x| x * x).sum::<f64>().sqrt();
        let mag2: f64 = vec2.iter().map(|x| x * x).sum::<f64>().sqrt();
        if mag1 == 0.0 || mag2 == 0.0 {
            0.0
        } else {
            dot_product / (mag1 * mag2)
        }
    }
    /// Compares two documents and returns similarity score.
    pub fn compare(
        &self,
        doc1_id: impl Into<String>,
        doc1_text: &str,
        doc2_id: impl Into<String>,
        doc2_text: &str,
    ) -> SimilarityScore {
        let score = self.cosine_similarity(doc1_text, doc2_text);
        SimilarityScore::new(doc1_id, doc2_id, score, "cosine")
    }
    /// Sets the similarity threshold.
    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.threshold = threshold.clamp(0.0, 1.0);
        self
    }
}
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
        match self.locale.language.as_str() {
            "ja" | "zh" => a.cmp(b),
            _ => {
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
            "de" => text
                .to_lowercase()
                .replace('ä', "ae")
                .replace('ö', "oe")
                .replace('ü', "ue")
                .replace('ß', "ss"),
            "fr" | "es" => text
                .to_lowercase()
                .replace(['é', 'è', 'ê', 'ë'], "e")
                .replace(['à', 'â'], "a")
                .replace('ñ', "n")
                .replace('ç', "c"),
            _ => text.to_lowercase(),
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
/// Terminology-aware translator that preserves legal terms.
pub struct TerminologyAwareTranslator {
    /// Base MT translator
    mt_translator: NeuralMachineTranslator,
    /// Glossary for term preservation
    pub(super) glossary: HashMap<String, String>,
    /// Preservation mode
    pub(crate) preservation_mode: TermPreservationMode,
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
        let (marked_text, term_positions) = self.mark_terms(text);
        let mut translation = self.mt_translator.translate(&marked_text, source, target)?;
        translation.text = self.restore_terms(&translation.text, &term_positions);
        translation.quality_score = (translation.quality_score + 0.1).min(1.0);
        Ok(translation)
    }
    /// Marks terms in text that should be preserved.
    fn mark_terms(&self, text: &str) -> (String, Vec<(String, String)>) {
        let mut marked = text.to_string();
        let mut positions = Vec::new();
        let mut terms: Vec<(&String, &String)> = self.glossary.iter().collect();
        terms.sort_by_key(|(k, _)| std::cmp::Reverse(k.len()));
        for (source_term, target_term) in terms {
            match self.preservation_mode {
                TermPreservationMode::Exact => {
                    let marker = format!("__TERM_{}__", positions.len());
                    if marked.contains(source_term.as_str()) {
                        marked = marked.replace(source_term, &marker);
                        positions.push((marker, source_term.clone()));
                    }
                }
                TermPreservationMode::GlossaryEnforced => {
                    let marker = format!("__TERM_{}__", positions.len());
                    if marked.contains(source_term.as_str()) {
                        marked = marked.replace(source_term, &marker);
                        positions.push((marker, target_term.clone()));
                    }
                }
                TermPreservationMode::PreserveFormatting => {
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
