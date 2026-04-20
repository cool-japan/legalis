//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::types_3::{PluralCategory, PostEditFeedback, TextDirection};
use super::types_4::{
    CulturalContext, InterpretedSegment, MTTranslation, SimultaneousInterpreter, TreatyType,
};
use super::types_5::{CitationError, LegalConceptMapping};
use super::types_7::{CulturalParams, TranslationMemory};
use super::types_8::{CourtParticipant, DialectType, Jurisdiction, RiskLevel};
use super::types_9::{FootnoteStyle, TranscriptionSegment};
use super::types_10::{Locale, LowResourceConfig, LowResourceStrategy};
use super::types_11::ExtendedLanguageRegistry;
use super::types_12::{ContextCategory, LegalSpeechDomain, PostEditAction, SemanticEmbedding};

/// Court proceeding live translation system.
#[derive(Debug, Clone)]
pub struct CourtProceedingTranslator {
    /// Court language (official language of the court).
    pub court_language: Locale,
    /// Participants in the proceeding.
    pub participants: Vec<CourtParticipant>,
    /// Active interpreters by target language.
    pub interpreters: HashMap<String, SimultaneousInterpreter>,
    /// Whether to record original and translated audio.
    pub record_audio: bool,
    /// Whether to generate real-time transcripts.
    pub real_time_transcripts: bool,
}
impl CourtProceedingTranslator {
    /// Creates a new court proceeding translator.
    pub fn new(court_language: Locale) -> Self {
        Self {
            court_language,
            participants: Vec::new(),
            interpreters: HashMap::new(),
            record_audio: true,
            real_time_transcripts: true,
        }
    }
    /// Adds a participant to the proceeding.
    pub fn add_participant(&mut self, participant: CourtParticipant) {
        if participant.requires_interpretation {
            let target_locale = participant.primary_language.clone();
            let locale_key = target_locale.tag();
            if !self.interpreters.contains_key(&locale_key) {
                let interpreter = SimultaneousInterpreter::for_court_proceedings(
                    self.court_language.clone(),
                    target_locale,
                );
                self.interpreters.insert(locale_key, interpreter);
            }
        }
        self.participants.push(participant);
    }
    /// Gets the number of languages being interpreted.
    pub fn language_count(&self) -> usize {
        self.interpreters.len() + 1
    }
    /// Processes a spoken utterance and distributes translations.
    pub fn process_utterance(
        &self,
        _speaker_name: &str,
        segment: TranscriptionSegment,
    ) -> HashMap<String, InterpretedSegment> {
        let mut translations = HashMap::new();
        for (locale_key, interpreter) in &self.interpreters {
            let interpreted = interpreter.interpret_segment(segment.clone());
            translations.insert(locale_key.clone(), interpreted);
        }
        translations
    }
    /// Enables or disables audio recording.
    pub fn set_recording(mut self, enable: bool) -> Self {
        self.record_audio = enable;
        self
    }
    /// Enables or disables real-time transcripts.
    pub fn set_transcripts(mut self, enable: bool) -> Self {
        self.real_time_transcripts = enable;
        self
    }
}
/// Low-resource language support manager.
#[derive(Debug, Clone)]
pub struct LowResourceSupport {
    /// Configurations by language code.
    configs: HashMap<String, LowResourceConfig>,
    /// Language registry.
    registry: ExtendedLanguageRegistry,
}
impl LowResourceSupport {
    /// Creates a new support manager.
    pub fn new(registry: ExtendedLanguageRegistry) -> Self {
        Self {
            configs: HashMap::new(),
            registry,
        }
    }
    /// Creates a manager with default configurations.
    pub fn with_defaults() -> Self {
        let registry = ExtendedLanguageRegistry::with_extended_set();
        let mut support = Self::new(registry);
        support.add_config(
            LowResourceConfig::new("ha", LowResourceStrategy::FallbackToRelated)
                .add_fallback("sw")
                .add_fallback("en")
                .with_transfer_from("sw"),
        );
        support.add_config(
            LowResourceConfig::new("yo", LowResourceStrategy::TransferLearning)
                .add_fallback("en")
                .with_transfer_from("sw"),
        );
        support.add_config(
            LowResourceConfig::new("ig", LowResourceStrategy::CommunityDriven)
                .add_fallback("en")
                .with_min_confidence(0.5),
        );
        support.add_config(
            LowResourceConfig::new("kk", LowResourceStrategy::FallbackToRelated)
                .add_fallback("ru")
                .add_fallback("tr")
                .with_transfer_from("ru"),
        );
        support.add_config(
            LowResourceConfig::new("uz", LowResourceStrategy::TransferLearning)
                .add_fallback("tr")
                .add_fallback("ru")
                .with_transfer_from("tr"),
        );
        support.add_config(
            LowResourceConfig::new("ne", LowResourceStrategy::FallbackToRelated)
                .add_fallback("hi")
                .add_fallback("en")
                .with_transfer_from("hi"),
        );
        support.add_config(
            LowResourceConfig::new("si", LowResourceStrategy::MultilingualModel)
                .add_fallback("ta")
                .add_fallback("en"),
        );
        support
    }
    /// Adds a configuration.
    pub fn add_config(&mut self, config: LowResourceConfig) {
        self.configs.insert(config.language_code.clone(), config);
    }
    /// Gets configuration for a language.
    pub fn get_config(&self, language_code: &str) -> Option<&LowResourceConfig> {
        self.configs.get(language_code)
    }
    /// Gets fallback chain for a language.
    pub fn get_fallback_chain(&self, language_code: &str) -> Vec<String> {
        self.configs
            .get(language_code)
            .map(|c| c.fallback_chain.clone())
            .unwrap_or_default()
    }
    /// Checks if language is low-resource.
    pub fn is_low_resource(&self, language_code: &str) -> bool {
        self.registry
            .get_language(language_code)
            .map(|l| l.low_resource)
            .unwrap_or(false)
    }
    /// Gets total config count.
    pub fn config_count(&self) -> usize {
        self.configs.len()
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
        if self.required && value.is_none() {
            return Err(CitationError::MissingField {
                field: self.field.clone(),
            });
        }
        if let Some(val) = value {
            if let Some(pattern) = &self.pattern
                && !Self::matches_pattern(val, pattern)
            {
                return Err(CitationError::InvalidFormat {
                    field: self.field.clone(),
                    reason: format!("Does not match pattern: {}", pattern),
                });
            }
            if let Some(validator) = self.validator
                && let Err(msg) = validator(val)
            {
                return Err(CitationError::InvalidFormat {
                    field: self.field.clone(),
                    reason: msg,
                });
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
            _ => true,
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
        while minutes < 0 {
            minutes += 24 * 60;
            let (y, m, d) = self.previous_day(current_year, current_month, current_day);
            current_year = y;
            current_month = m;
            current_day = d;
        }
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
            Jurisdiction::new("GB", "United Kingdom", Locale::new("en").with_country("GB"))
                .with_legal_system(LegalSystem::CommonLaw)
                .with_cultural_params(CulturalParams::for_country("GB")),
        );
        registry.register(
            Jurisdiction::new("DE", "Germany", Locale::new("de").with_country("DE"))
                .with_legal_system(LegalSystem::CivilLaw)
                .with_cultural_params(CulturalParams::for_country("DE")),
        );
        registry.register(
            Jurisdiction::new("FR", "France", Locale::new("fr").with_country("FR"))
                .with_legal_system(LegalSystem::CivilLaw)
                .with_cultural_params(CulturalParams::for_country("FR")),
        );
        registry.register(
            Jurisdiction::new("ES", "Spain", Locale::new("es").with_country("ES"))
                .with_legal_system(LegalSystem::CivilLaw)
                .with_cultural_params(CulturalParams::for_country("ES")),
        );
        registry.register(
            Jurisdiction::new("IT", "Italy", Locale::new("it").with_country("IT"))
                .with_legal_system(LegalSystem::CivilLaw)
                .with_cultural_params(CulturalParams::for_country("IT")),
        );
        registry.register(
            Jurisdiction::new("CN", "China", Locale::new("zh").with_country("CN"))
                .with_legal_system(LegalSystem::CivilLaw)
                .with_cultural_params(CulturalParams::for_country("CN")),
        );
        registry.register(
            Jurisdiction::new("TW", "Taiwan", Locale::new("zh").with_country("TW"))
                .with_legal_system(LegalSystem::CivilLaw)
                .with_cultural_params(CulturalParams::for_country("TW")),
        );
        registry.register(
            Jurisdiction::new("KR", "South Korea", Locale::new("ko").with_country("KR"))
                .with_legal_system(LegalSystem::CivilLaw)
                .with_cultural_params(CulturalParams::for_country("KR")),
        );
        registry.register(
            Jurisdiction::new("CA", "Canada", Locale::new("en").with_country("CA"))
                .with_legal_system(LegalSystem::Mixed)
                .with_cultural_params(CulturalParams::for_country("CA")),
        );
        registry.register(
            Jurisdiction::new("AU", "Australia", Locale::new("en").with_country("AU"))
                .with_legal_system(LegalSystem::CommonLaw)
                .with_cultural_params(CulturalParams::for_country("AU")),
        );
        registry.register(
            Jurisdiction::new("IN", "India", Locale::new("en").with_country("IN"))
                .with_legal_system(LegalSystem::CommonLaw)
                .with_cultural_params(CulturalParams::for_country("IN")),
        );
        registry.register(
            Jurisdiction::new("BR", "Brazil", Locale::new("pt").with_country("BR"))
                .with_legal_system(LegalSystem::CivilLaw)
                .with_cultural_params(CulturalParams::for_country("BR")),
        );
        registry.register(
            Jurisdiction::new("RU", "Russia", Locale::new("ru").with_country("RU"))
                .with_legal_system(LegalSystem::CivilLaw)
                .with_cultural_params(CulturalParams::for_country("RU")),
        );
        registry.register(
            Jurisdiction::new("SA", "Saudi Arabia", Locale::new("ar").with_country("SA"))
                .with_legal_system(LegalSystem::ReligiousLaw)
                .with_cultural_params(CulturalParams::for_country("SA")),
        );
        registry.register(
            Jurisdiction::new("NL", "Netherlands", Locale::new("nl").with_country("NL"))
                .with_legal_system(LegalSystem::CivilLaw)
                .with_cultural_params(CulturalParams::for_country("NL")),
        );
        registry.register(
            Jurisdiction::new("CH", "Switzerland", Locale::new("de").with_country("CH"))
                .with_legal_system(LegalSystem::CivilLaw)
                .with_cultural_params(CulturalParams::for_country("CH")),
        );
        registry.register(
            Jurisdiction::new("MX", "Mexico", Locale::new("es").with_country("MX"))
                .with_legal_system(LegalSystem::CivilLaw)
                .with_cultural_params(CulturalParams::for_country("MX")),
        );
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
/// Registry of legal concept mappings between different legal systems.
#[derive(Debug, Default)]
pub struct LegalConceptRegistry {
    pub(super) mappings: Vec<LegalConceptMapping>,
}
impl LegalConceptRegistry {
    /// Creates a new registry.
    pub fn new() -> Self {
        Self::default()
    }
    /// Creates a registry with standard mappings.
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();
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
/// Extended language information for emerging markets.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtendedLanguage {
    /// ISO 639-1 or 639-3 code.
    pub code: String,
    /// Language name in English.
    pub name: String,
    /// Native language name.
    pub native_name: String,
    /// Language family.
    pub family: String,
    /// Script system (Latin, Cyrillic, Arabic, etc.).
    pub script: String,
    /// Is this a low-resource language?
    pub low_resource: bool,
    /// Number of native speakers (millions).
    pub speakers_millions: f32,
    /// Countries where this language is official.
    pub official_in: Vec<String>,
}
impl ExtendedLanguage {
    /// Creates a new extended language.
    pub fn new(
        code: impl Into<String>,
        name: impl Into<String>,
        native_name: impl Into<String>,
        family: impl Into<String>,
        script: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            name: name.into(),
            native_name: native_name.into(),
            family: family.into(),
            script: script.into(),
            low_resource: false,
            speakers_millions: 0.0,
            official_in: Vec::new(),
        }
    }
    /// Marks as low-resource language.
    pub fn as_low_resource(mut self) -> Self {
        self.low_resource = true;
        self
    }
    /// Sets speaker count.
    pub fn with_speakers(mut self, millions: f32) -> Self {
        self.speakers_millions = millions;
        self
    }
    /// Adds an official country.
    pub fn add_official_country(mut self, country: impl Into<String>) -> Self {
        self.official_in.push(country.into());
        self
    }
}
/// Footnote or endnote formatter.
#[derive(Debug, Clone)]
pub struct FootnoteFormatter {
    style: FootnoteStyle,
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
/// Dialect variation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dialect {
    /// Dialect identifier.
    pub dialect_id: String,
    /// Base language code.
    pub base_language: String,
    /// Dialect name.
    pub name: String,
    /// Dialect type.
    pub dialect_type: DialectType,
    /// Region or area.
    pub region: Option<String>,
    /// Term variations (standard term -> dialect variant).
    pub variations: HashMap<String, String>,
}
impl Dialect {
    /// Creates a new dialect.
    pub fn new(
        dialect_id: impl Into<String>,
        base_language: impl Into<String>,
        name: impl Into<String>,
        dialect_type: DialectType,
    ) -> Self {
        Self {
            dialect_id: dialect_id.into(),
            base_language: base_language.into(),
            name: name.into(),
            dialect_type,
            region: None,
            variations: HashMap::new(),
        }
    }
    /// Sets the region.
    pub fn with_region(mut self, region: impl Into<String>) -> Self {
        self.region = Some(region.into());
        self
    }
    /// Adds a variation.
    pub fn add_variation(
        mut self,
        standard: impl Into<String>,
        variant: impl Into<String>,
    ) -> Self {
        self.variations.insert(standard.into(), variant.into());
        self
    }
    /// Converts from standard to dialect.
    pub fn to_dialect(&self, standard_term: &str) -> Option<String> {
        self.variations.get(standard_term).cloned()
    }
    /// Converts from dialect to standard.
    pub fn to_standard(&self, dialect_term: &str) -> Option<String> {
        self.variations
            .iter()
            .find(|(_, v)| *v == dialect_term)
            .map(|(k, _)| k.clone())
    }
}
/// Contract clause classification types.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClauseClass {
    /// Payment terms
    Payment,
    /// Termination conditions
    Termination,
    /// Confidentiality provisions
    Confidentiality,
    /// Liability limitations
    LiabilityLimitation,
    /// Indemnification
    Indemnification,
    /// Force majeure
    ForceMajeure,
    /// Dispute resolution
    DisputeResolution,
    /// Intellectual property
    IntellectualProperty,
    /// Governing law
    GoverningLaw,
    /// Warranties and representations
    Warranties,
    /// Assignment rights
    Assignment,
    /// Severability
    Severability,
    /// Custom class
    Custom(String),
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
        match self.locale.language.as_str() {
            "ja" | "zh" | "ko" | "vi" | "th" => PluralCategory::Other,
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
                if n == 1 {
                    PluralCategory::One
                } else {
                    PluralCategory::Other
                }
            }
        }
    }
}
/// RTL (Right-to-Left) text handler for Arabic and Hebrew legal documents.
#[derive(Debug, Clone)]
pub struct BidirectionalText {
    pub(super) locale: Locale,
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
            TextDirection::RTL => format!("\u{202B}{}\u{202C}", text),
            TextDirection::LTR => format!("\u{202A}{}\u{202C}", text),
        }
    }
    /// Adds Right-to-Left Mark (RLM) for RTL languages.
    /// Useful for maintaining RTL directionality in mixed content.
    pub fn add_direction_mark(&self, text: &str) -> String {
        match self.direction {
            TextDirection::RTL => format!("{}\u{200F}", text),
            TextDirection::LTR => format!("{}\u{200E}", text),
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
            TextDirection::RTL => format!("{} \u{202A}{}\u{202C}", rtl_text, ltr_text),
            TextDirection::LTR => format!("{} \u{202B}{}\u{202C}", ltr_text, rtl_text),
        }
    }
    /// Formats a number for RTL context (e.g., Arabic numerals vs Eastern Arabic numerals).
    pub fn format_number(&self, number: i64) -> String {
        match self.locale.language.as_str() {
            "ar" => {
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
            _ => number.to_string(),
        }
    }
    /// Formats a date for RTL context.
    pub fn format_date_rtl(&self, year: i32, month: u32, day: u32) -> String {
        match self.locale.language.as_str() {
            "ar" => {
                let day_str = self.format_number(day as i64);
                let month_str = self.format_number(month as i64);
                let year_str = self.format_number(year as i64);
                format!("{}/{}/{}", day_str, month_str, year_str)
            }
            "he" => format!("{}.{}.{}", day, month, year),
            _ => format!("{}-{:02}-{:02}", year, month, day),
        }
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
/// Legal concept for cross-lingual mapping.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LegalConcept {
    /// Concept identifier.
    pub concept_id: String,
    /// Concept name in English (canonical).
    pub canonical_name: String,
    /// Localized names by locale.
    pub localized_names: HashMap<String, String>,
    /// Concept definition.
    pub definition: String,
    /// Legal domain.
    pub domain: Option<LegalSpeechDomain>,
    /// Semantic embedding.
    pub embedding: Option<SemanticEmbedding>,
}
impl LegalConcept {
    /// Creates a new legal concept.
    pub fn new(
        concept_id: impl Into<String>,
        canonical_name: impl Into<String>,
        definition: impl Into<String>,
    ) -> Self {
        Self {
            concept_id: concept_id.into(),
            canonical_name: canonical_name.into(),
            localized_names: HashMap::new(),
            definition: definition.into(),
            domain: None,
            embedding: None,
        }
    }
    /// Adds a localized name for a locale.
    pub fn add_localized_name(mut self, locale: Locale, name: impl Into<String>) -> Self {
        self.localized_names.insert(locale.tag(), name.into());
        self
    }
    /// Sets the legal domain.
    pub fn with_domain(mut self, domain: LegalSpeechDomain) -> Self {
        self.domain = Some(domain);
        self
    }
    /// Sets the semantic embedding.
    pub fn with_embedding(mut self, embedding: SemanticEmbedding) -> Self {
        self.embedding = Some(embedding);
        self
    }
    /// Gets the name in a specific locale, or canonical name if not available.
    pub fn get_name(&self, locale: &Locale) -> String {
        self.localized_names
            .get(&locale.tag())
            .cloned()
            .unwrap_or_else(|| self.canonical_name.clone())
    }
}
/// Standardized treaty term.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TreatyTerm {
    /// Treaty name or identifier.
    pub treaty_name: String,
    /// Treaty type.
    pub treaty_type: TreatyType,
    /// Canonical term in English.
    pub canonical_term: String,
    /// Standardized translations in treaty languages.
    pub translations: HashMap<String, String>,
    /// Article or section reference.
    pub article_ref: Option<String>,
    /// Ratifying countries.
    pub ratifying_countries: Vec<String>,
}
impl TreatyTerm {
    /// Creates a new treaty term.
    pub fn new(
        treaty_name: impl Into<String>,
        treaty_type: TreatyType,
        canonical_term: impl Into<String>,
    ) -> Self {
        Self {
            treaty_name: treaty_name.into(),
            treaty_type,
            canonical_term: canonical_term.into(),
            translations: HashMap::new(),
            article_ref: None,
            ratifying_countries: Vec::new(),
        }
    }
    /// Adds a translation in a specific language.
    pub fn add_translation(mut self, language: impl Into<String>, term: impl Into<String>) -> Self {
        self.translations.insert(language.into(), term.into());
        self
    }
    /// Sets the article reference.
    pub fn with_article(mut self, article: impl Into<String>) -> Self {
        self.article_ref = Some(article.into());
        self
    }
    /// Adds a ratifying country.
    pub fn add_country(mut self, country: impl Into<String>) -> Self {
        self.ratifying_countries.push(country.into());
        self
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
/// Post-editing workflow for translation review.
pub struct PostEditingWorkflow {
    /// Pending translations for review
    pub(super) pending: Vec<(String, MTTranslation)>,
    /// Accepted translations
    pub(super) accepted: Vec<(String, String)>,
    /// Rejected translations
    pub(super) rejected: Vec<(String, String)>,
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
/// Cultural context registry.
#[derive(Debug, Clone, Default)]
pub struct CulturalContextRegistry {
    /// Contexts indexed by locale tag
    pub(super) contexts: HashMap<String, Vec<CulturalContext>>,
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
        let zh_cn = Locale::new("zh").with_script("Hans").with_country("CN");
        self.add_context(
            CulturalContext::new(
                    zh_cn.clone(),
                    ContextCategory::SocialHierarchy,
                    "guanxi",
                    "Network of relationships and mutual obligations crucial in business and legal matters",
                )
                .with_guideline(
                    "Understanding guanxi is essential for contract negotiations",
                )
                .with_guideline(
                    "Legal disputes may be resolved through guanxi rather than formal proceedings",
                )
                .with_equivalent("ja-JP", "人間関係 (human relationships)"),
        );
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
