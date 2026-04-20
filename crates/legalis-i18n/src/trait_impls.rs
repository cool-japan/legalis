//! # AdoptionStatus - Trait Implementations
//!
//! This module contains trait implementations for `AdoptionStatus`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//! - `Default`
//! - `Display`
//! - `Display`
//! - `Default`
//! - `Display`
//! - `Display`
//! - `Default`
//! - `Display`
//! - `Display`
//! - `Default`
//! - `Display`
//! - `Display`
//! - `Default`
//! - `Display`
//! - `Default`
//! - `Display`
//! - `Default`
//! - `Display`
//! - `Display`
//! - `Display`
//! - `Default`
//! - `Display`
//! - `Display`
//! - `Default`
//! - `Default`
//! - `Display`
//! - `Display`
//! - `Display`
//! - `Default`
//! - `Default`
//! - `Default`
//! - `Display`
//! - `Default`
//! - `Display`
//! - `Default`
//! - `Display`
//! - `Default`
//! - `Display`
//! - `Display`
//! - `Display`
//! - `Display`
//! - `Debug`
//! - `Serialize`
//! - `Deserialize`
//! - `Default`
//! - `Default`
//! - `Display`
//! - `Display`
//! - `Display`
//! - `Display`
//! - `Default`
//! - `Default`
//! - `Display`
//! - `Display`
//! - `Default`
//! - `Default`
//! - `Default`
//! - `TranslationService`
//! - `Display`
//! - `Display`
//! - `Default`
//! - `Default`
//! - `Display`
//! - `Default`
//! - `Display`
//! - `Display`
//! - `Default`
//! - `Display`
//! - `Display`
//! - `Default`
//! - `Display`
//! - `Display`
//! - `Default`
//! - `Display`
//! - `Display`
//! - `Default`
//! - `Display`
//! - `Display`
//! - `Display`
//! - `Display`
//! - `Default`
//! - `Default`
//! - `Display`
//! - `Display`
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::functions::{I18nResult, TranslationService};
use super::types::{
    ClauseType, EURegulationType, EtymologyTracker, LegalEntityType, RegulatoryDomain,
    SimplificationStrategy,
};
use super::types_3::{
    AdoptionStatus, AudioQuality, CitationNormalizer, CourtParticipantRole, EURegulationAligner,
    PluralCategory, SignLanguageReferencer, StandardAdoptionTracker, StyleProfile,
    SubtitlePosition, TextDirection,
};
use super::types_4::{
    DayOfWeek, GlossaryEnforcer, HistoricalContextAnnotator, HistoricalPeriod, LanguageScope,
    LazyDictionary, LegalEntityRecognizer, QualityMetric, RegulatoryEquivalenceLevel, StandardType,
    TreatyType,
};
use super::types_5::{
    ArchaicTermDictionary, ContributionWorkflow, DialectHandler, KeyTermExtractor,
    TranslationManager, TreatyStandardizer,
};
use super::types_6::{
    CitationStyle, ClauseClass, ColonialPower, LanguageFamily, LegalSystem, LowResourceSupport,
    PostEditingWorkflow,
};
use super::types_7::LegalTopicModeler;
use super::types_8::{BCP47LanguageTag, DialectType, LegalDictionary, RiskLevel, SignLanguageType};
use super::types_9::{
    CLDRData, CalendarSystem, DisambiguationType, EmphasisLevel, ISO639_3_Registry,
    MockTranslationService, StyleAttribute, WCAGLevel,
};
use super::types_10::{
    ClauseClassifier, ComplianceNormalizer, LegalDocumentAnalyzer, Locale, LowResourceStrategy,
    RegulatoryEquivalenceMapper, ReligiousLawType, ViolationType,
};
use super::types_11::{
    CLDRFieldType, DocumentSimilarityCalculator, EmbeddingModel, ExtendedLanguageRegistry,
    HistoricalCalendar, I18nError, InterpretationMode, LanguageType, MachineTranslationFallback,
    TranslationEngine,
};
use super::types_12::{
    ContextCategory, ContextDisambiguator, ContributionStatus, LLMProvider, LegalSpeechDomain,
    LocalLawDatabase, NormalizationLevel, QualityEstimator, ReadingLevelAssessor,
    TargetReadingLevel,
};
use super::types_13::{ContextualTranslationEntry, CustomType, LegalExtensionType};

impl std::fmt::Display for AdoptionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AdoptionStatus::FullyAdopted => write!(f, "Fully Adopted"),
            AdoptionStatus::PartiallyAdopted => write!(f, "Partially Adopted"),
            AdoptionStatus::InProgress => write!(f, "In Progress"),
            AdoptionStatus::NotAdopted => write!(f, "Not Adopted"),
        }
    }
}

impl Default for ArchaicTermDictionary {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for AudioQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioQuality::Low => write!(f, "Low (8kHz)"),
            AudioQuality::Medium => write!(f, "Medium (16kHz)"),
            AudioQuality::High => write!(f, "High (44.1kHz)"),
            AudioQuality::Studio => write!(f, "Studio (48kHz+)"),
        }
    }
}

impl std::fmt::Display for BCP47LanguageTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format_tag())
    }
}

impl Default for CLDRData {
    fn default() -> Self {
        Self::new()
    }
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

impl Default for CitationNormalizer {
    fn default() -> Self {
        Self::new()
    }
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

impl std::fmt::Display for ClauseClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClauseClass::Payment => write!(f, "Payment"),
            ClauseClass::Termination => write!(f, "Termination"),
            ClauseClass::Confidentiality => write!(f, "Confidentiality"),
            ClauseClass::LiabilityLimitation => write!(f, "Liability Limitation"),
            ClauseClass::Indemnification => write!(f, "Indemnification"),
            ClauseClass::ForceMajeure => write!(f, "Force Majeure"),
            ClauseClass::DisputeResolution => write!(f, "Dispute Resolution"),
            ClauseClass::IntellectualProperty => write!(f, "Intellectual Property"),
            ClauseClass::GoverningLaw => write!(f, "Governing Law"),
            ClauseClass::Warranties => write!(f, "Warranties"),
            ClauseClass::Assignment => write!(f, "Assignment"),
            ClauseClass::Severability => write!(f, "Severability"),
            ClauseClass::Custom(s) => write!(f, "{}", s),
        }
    }
}

impl Default for ClauseClassifier {
    fn default() -> Self {
        Self::new()
    }
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

impl Default for ComplianceNormalizer {
    fn default() -> Self {
        Self::with_defaults()
    }
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

impl Default for ContextDisambiguator {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ContributionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContributionStatus::Pending => write!(f, "Pending"),
            ContributionStatus::InReview => write!(f, "In Review"),
            ContributionStatus::Approved => write!(f, "Approved"),
            ContributionStatus::Rejected => write!(f, "Rejected"),
        }
    }
}

impl Default for ContributionWorkflow {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for CourtParticipantRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CourtParticipantRole::Judge => write!(f, "Judge"),
            CourtParticipantRole::Prosecutor => write!(f, "Prosecutor"),
            CourtParticipantRole::DefenseAttorney => write!(f, "Defense Attorney"),
            CourtParticipantRole::PlaintiffAttorney => write!(f, "Plaintiff's Attorney"),
            CourtParticipantRole::DefendantAttorney => write!(f, "Defendant's Attorney"),
            CourtParticipantRole::Witness => write!(f, "Witness"),
            CourtParticipantRole::Defendant => write!(f, "Defendant"),
            CourtParticipantRole::Plaintiff => write!(f, "Plaintiff"),
            CourtParticipantRole::CourtReporter => write!(f, "Court Reporter"),
            CourtParticipantRole::Interpreter => write!(f, "Interpreter"),
            CourtParticipantRole::Juror => write!(f, "Juror"),
        }
    }
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

impl Default for DialectHandler {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl std::fmt::Display for DialectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DialectType::Regional => write!(f, "Regional"),
            DialectType::Social => write!(f, "Social"),
            DialectType::Occupational => write!(f, "Occupational"),
            DialectType::Historical => write!(f, "Historical"),
        }
    }
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

impl Default for DocumentSimilarityCalculator {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for EURegulationAligner {
    fn default() -> Self {
        Self::with_gdpr_defaults()
    }
}

impl std::fmt::Display for EURegulationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EURegulationType::GDPR => write!(f, "GDPR"),
            EURegulationType::MiFIDII => write!(f, "MiFID II"),
            EURegulationType::REACH => write!(f, "REACH"),
            EURegulationType::EUDataAct => write!(f, "EU Data Act"),
            EURegulationType::DigitalMarketsAct => write!(f, "Digital Markets Act"),
            EURegulationType::DigitalServicesAct => write!(f, "Digital Services Act"),
            EURegulationType::AIAct => write!(f, "AI Act"),
            EURegulationType::Custom => write!(f, "Custom"),
        }
    }
}

impl std::fmt::Display for EmbeddingModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmbeddingModel::MultilinguralBERT => write!(f, "Multilingual BERT"),
            EmbeddingModel::XLMRoBERTa => write!(f, "XLM-RoBERTa"),
            EmbeddingModel::LaBSE => write!(f, "LaBSE"),
            EmbeddingModel::LegalMultilingual => write!(f, "Legal Multilingual"),
            EmbeddingModel::Custom => write!(f, "Custom"),
        }
    }
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

impl Default for EtymologyTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ExtendedLanguageRegistry {
    fn default() -> Self {
        Self::with_extended_set()
    }
}

impl Default for GlossaryEnforcer {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for HistoricalCalendar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HistoricalCalendar::Julian => write!(f, "Julian Calendar"),
            HistoricalCalendar::Gregorian => write!(f, "Gregorian Calendar"),
            HistoricalCalendar::Roman => write!(f, "Roman Calendar"),
            HistoricalCalendar::FrenchRevolutionary => {
                write!(f, "French Revolutionary Calendar")
            }
        }
    }
}

impl Default for HistoricalContextAnnotator {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for HistoricalPeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HistoricalPeriod::OldEnglish => write!(f, "Old English (450-1150)"),
            HistoricalPeriod::MiddleEnglish => write!(f, "Middle English (1150-1500)"),
            HistoricalPeriod::EarlyModern => {
                write!(f, "Early Modern English (1500-1700)")
            }
            HistoricalPeriod::ClassicalLatin => {
                write!(f, "Classical Latin (Roman Empire)")
            }
            HistoricalPeriod::MedievalLatin => write!(f, "Medieval Latin (500-1500)"),
            HistoricalPeriod::Renaissance => write!(f, "Renaissance (1400-1600)"),
            HistoricalPeriod::Enlightenment => write!(f, "Enlightenment (1600-1800)"),
            HistoricalPeriod::Victorian => write!(f, "Victorian (1837-1901)"),
        }
    }
}

impl Default for ISO639_3_Registry {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for InterpretationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpretationMode::Consecutive => write!(f, "Consecutive"),
            InterpretationMode::Simultaneous => write!(f, "Simultaneous"),
            InterpretationMode::Whispered => write!(f, "Whispered (Chuchotage)"),
        }
    }
}

impl Default for KeyTermExtractor {
    fn default() -> Self {
        Self::new()
    }
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

impl std::fmt::Display for LanguageScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LanguageScope::Individual => write!(f, "Individual"),
            LanguageScope::Macrolanguage => write!(f, "Macrolanguage"),
            LanguageScope::Special => write!(f, "Special"),
        }
    }
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

impl std::fmt::Debug for LazyDictionary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LazyDictionary")
            .field("locale", &self.locale)
            .field("is_loaded", &self.is_loaded())
            .finish()
    }
}

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

impl Default for LegalDocumentAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for LegalEntityRecognizer {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for LegalEntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LegalEntityType::Court => write!(f, "Court"),
            LegalEntityType::Company => write!(f, "Company"),
            LegalEntityType::Statute => write!(f, "Statute"),
            LegalEntityType::Person => write!(f, "Person"),
            LegalEntityType::GovernmentAgency => write!(f, "Government Agency"),
            LegalEntityType::LawFirm => write!(f, "Law Firm"),
            LegalEntityType::Other(s) => write!(f, "{}", s),
        }
    }
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

impl std::fmt::Display for LegalSpeechDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LegalSpeechDomain::CourtProceedings => write!(f, "Court Proceedings"),
            LegalSpeechDomain::Depositions => write!(f, "Depositions"),
            LegalSpeechDomain::Consultations => write!(f, "Legal Consultations"),
            LegalSpeechDomain::ContractNegotiations => write!(f, "Contract Negotiations"),
            LegalSpeechDomain::ArbitrationMediation => write!(f, "Arbitration/Mediation"),
            LegalSpeechDomain::General => write!(f, "General Legal"),
        }
    }
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

impl Default for LegalTopicModeler {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for LocalLawDatabase {
    fn default() -> Self {
        Self::with_samples()
    }
}

impl std::fmt::Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.tag())
    }
}

impl std::fmt::Display for LowResourceStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LowResourceStrategy::FallbackToRelated => {
                write!(f, "Fallback to Related Language")
            }
            LowResourceStrategy::TransferLearning => write!(f, "Transfer Learning"),
            LowResourceStrategy::MultilingualModel => write!(f, "Multilingual Model"),
            LowResourceStrategy::CommunityDriven => write!(f, "Community-Driven"),
        }
    }
}

impl Default for LowResourceSupport {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl Default for MachineTranslationFallback {
    fn default() -> Self {
        Self::new()
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

impl std::fmt::Display for NormalizationLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NormalizationLevel::Strict => write!(f, "Strict"),
            NormalizationLevel::Standard => write!(f, "Standard"),
            NormalizationLevel::Flexible => write!(f, "Flexible"),
        }
    }
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

impl Default for PostEditingWorkflow {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for QualityEstimator {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl std::fmt::Display for QualityMetric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QualityMetric::SemanticAccuracy => write!(f, "Semantic Accuracy"),
            QualityMetric::TerminologicalConsistency => {
                write!(f, "Terminological Consistency")
            }
            QualityMetric::GrammaticalCorrectness => write!(f, "Grammatical Correctness"),
            QualityMetric::StyleAppropriateness => write!(f, "Style Appropriateness"),
            QualityMetric::CitationPreservation => write!(f, "Citation Preservation"),
            QualityMetric::Fluency => write!(f, "Fluency"),
        }
    }
}

impl Default for ReadingLevelAssessor {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for RegulatoryDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegulatoryDomain::DataProtection => write!(f, "Data Protection"),
            RegulatoryDomain::FinancialServices => write!(f, "Financial Services"),
            RegulatoryDomain::Environmental => write!(f, "Environmental"),
            RegulatoryDomain::ConsumerProtection => write!(f, "Consumer Protection"),
            RegulatoryDomain::ProfessionalQualifications => {
                write!(f, "Professional Qualifications")
            }
            RegulatoryDomain::ProductSafety => write!(f, "Product Safety"),
            RegulatoryDomain::Telecommunications => write!(f, "Telecommunications"),
        }
    }
}

impl std::fmt::Display for RegulatoryEquivalenceLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegulatoryEquivalenceLevel::Full => write!(f, "Full Equivalence"),
            RegulatoryEquivalenceLevel::Conditional => {
                write!(f, "Conditional Equivalence")
            }
            RegulatoryEquivalenceLevel::Partial => write!(f, "Partial Equivalence"),
            RegulatoryEquivalenceLevel::NoEquivalence => write!(f, "No Equivalence"),
        }
    }
}

impl Default for RegulatoryEquivalenceMapper {
    fn default() -> Self {
        Self::with_defaults()
    }
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

impl Default for SignLanguageReferencer {
    fn default() -> Self {
        Self::new()
    }
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

impl Default for StandardAdoptionTracker {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl std::fmt::Display for StandardType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StandardType::ISO => write!(f, "ISO"),
            StandardType::IEC => write!(f, "IEC"),
            StandardType::ITU => write!(f, "ITU"),
            StandardType::IETF => write!(f, "IETF"),
            StandardType::W3C => write!(f, "W3C"),
            StandardType::UNCITRAL => write!(f, "UNCITRAL"),
            StandardType::HagueConference => write!(f, "Hague Conference"),
        }
    }
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

impl Default for StyleProfile {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for SubtitlePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubtitlePosition::BottomCenter => write!(f, "Bottom Center"),
            SubtitlePosition::TopCenter => write!(f, "Top Center"),
            SubtitlePosition::BottomLeft => write!(f, "Bottom Left"),
            SubtitlePosition::BottomRight => write!(f, "Bottom Right"),
            SubtitlePosition::TopLeft => write!(f, "Top Left"),
            SubtitlePosition::TopRight => write!(f, "Top Right"),
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

impl std::fmt::Display for TextDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextDirection::LTR => write!(f, "LTR"),
            TextDirection::RTL => write!(f, "RTL"),
        }
    }
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

impl Default for TranslationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for TreatyStandardizer {
    fn default() -> Self {
        Self::with_un_defaults()
    }
}

impl std::fmt::Display for TreatyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TreatyType::Bilateral => write!(f, "Bilateral Treaty"),
            TreatyType::Multilateral => write!(f, "Multilateral Treaty"),
            TreatyType::UNTreaty => write!(f, "UN Treaty"),
            TreatyType::Regional => write!(f, "Regional Treaty"),
            TreatyType::TradeAgreement => write!(f, "Trade Agreement"),
            TreatyType::HumanRights => write!(f, "Human Rights Treaty"),
            TreatyType::Environmental => write!(f, "Environmental Treaty"),
        }
    }
}

impl std::fmt::Display for ViolationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ViolationType::MissingMandatoryTerm => write!(f, "Missing Mandatory Term"),
            ViolationType::ForbiddenTermUsed => write!(f, "Forbidden Term Used"),
        }
    }
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
