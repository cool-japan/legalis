//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::functions::I18nResult;
use super::types::{AnalysisResult, RegulatoryDomain};
use super::types_3::{AudioQuality, MultilingualKnowledgeGraph, ObligationExtractor};
use super::types_4::RegulatoryEquivalenceLevel;
use super::types_5::{ConceptMapper, MultilingualEmbedder};
use super::types_6::{CitationComponents, CitationStyle, ClauseClass, TimeZone};
use super::types_7::{JurisdictionDetector, RegulatoryEquivalence};
use super::types_8::{LegalDictionary, PartyIdentifier};
use super::types_9::TranscriptionSegment;
use super::types_11::I18nError;
use super::types_12::{
    ClauseExtractor, DeadlineExtractor, DocumentAnalysis, LegalSpeechDomain, NormalizationLevel,
    SemanticEmbedding,
};
use super::types_13::{CrossLingualCaseSearch, CustomType, LegalRiskScorer};

/// Legal case metadata for cross-lingual search.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LegalCase {
    /// Case identifier.
    pub case_id: String,
    /// Case title/name.
    pub title: String,
    /// Jurisdiction.
    pub jurisdiction: String,
    /// Case summary or holding.
    pub summary: String,
    /// Case text locale.
    pub locale: Locale,
    /// Case year.
    pub year: u32,
    /// Legal domain.
    pub domain: Option<LegalSpeechDomain>,
    /// Semantic embedding of the case.
    pub embedding: Option<SemanticEmbedding>,
}
impl LegalCase {
    /// Creates a new legal case.
    pub fn new(
        case_id: impl Into<String>,
        title: impl Into<String>,
        jurisdiction: impl Into<String>,
        summary: impl Into<String>,
        locale: Locale,
        year: u32,
    ) -> Self {
        Self {
            case_id: case_id.into(),
            title: title.into(),
            jurisdiction: jurisdiction.into(),
            summary: summary.into(),
            locale,
            year,
            domain: None,
            embedding: None,
        }
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
}
/// Low-resource language configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LowResourceConfig {
    /// Language code.
    pub language_code: String,
    /// Fallback language codes (in priority order).
    pub fallback_chain: Vec<String>,
    /// Strategy to use.
    pub strategy: LowResourceStrategy,
    /// Source language for transfer learning.
    pub transfer_from: Option<String>,
    /// Minimum confidence threshold.
    pub min_confidence: f32,
}
impl LowResourceConfig {
    /// Creates a new low-resource configuration.
    pub fn new(language_code: impl Into<String>, strategy: LowResourceStrategy) -> Self {
        Self {
            language_code: language_code.into(),
            fallback_chain: Vec::new(),
            strategy,
            transfer_from: None,
            min_confidence: 0.6,
        }
    }
    /// Adds a fallback language.
    pub fn add_fallback(mut self, lang_code: impl Into<String>) -> Self {
        self.fallback_chain.push(lang_code.into());
        self
    }
    /// Sets transfer learning source.
    pub fn with_transfer_from(mut self, source: impl Into<String>) -> Self {
        self.transfer_from = Some(source.into());
        self
    }
    /// Sets minimum confidence threshold.
    pub fn with_min_confidence(mut self, threshold: f32) -> Self {
        self.min_confidence = threshold;
        self
    }
}
/// Compliance term with normalized variants.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplianceTerm {
    /// Canonical normalized term.
    pub canonical: String,
    /// Accepted variants.
    pub variants: Vec<String>,
    /// Regulatory domain.
    pub domain: RegulatoryDomain,
    /// Definition.
    pub definition: String,
    /// Normalization level required.
    pub normalization_level: NormalizationLevel,
}
impl ComplianceTerm {
    /// Creates a new compliance term.
    pub fn new(
        canonical: impl Into<String>,
        domain: RegulatoryDomain,
        definition: impl Into<String>,
        level: NormalizationLevel,
    ) -> Self {
        Self {
            canonical: canonical.into(),
            variants: Vec::new(),
            domain,
            definition: definition.into(),
            normalization_level: level,
        }
    }
    /// Adds an accepted variant.
    pub fn add_variant(mut self, variant: impl Into<String>) -> Self {
        self.variants.push(variant.into());
        self
    }
    /// Checks if a term matches this compliance term.
    pub fn matches(&self, term: &str) -> bool {
        if self.canonical.eq_ignore_ascii_case(term) {
            return true;
        }
        self.variants.iter().any(|v| v.eq_ignore_ascii_case(term))
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
/// Compliance language normalizer.
#[derive(Debug, Clone)]
pub struct ComplianceNormalizer {
    /// Terms indexed by canonical form.
    pub(super) terms: HashMap<String, ComplianceTerm>,
    /// Terms indexed by domain.
    pub(super) by_domain: HashMap<RegulatoryDomain, Vec<ComplianceTerm>>,
    /// Default normalization level.
    _default_level: NormalizationLevel,
}
impl ComplianceNormalizer {
    /// Creates a new compliance normalizer.
    pub fn new(default_level: NormalizationLevel) -> Self {
        Self {
            terms: HashMap::new(),
            by_domain: HashMap::new(),
            _default_level: default_level,
        }
    }
    /// Creates a normalizer with default compliance terms.
    pub fn with_defaults() -> Self {
        let mut normalizer = Self::new(NormalizationLevel::Standard);
        normalizer.add_term(
            ComplianceTerm::new(
                "data controller",
                RegulatoryDomain::DataProtection,
                "Entity that determines purposes and means of processing",
                NormalizationLevel::Strict,
            )
            .add_variant("controller")
            .add_variant("data owner"),
        );
        normalizer.add_term(
            ComplianceTerm::new(
                "data processor",
                RegulatoryDomain::DataProtection,
                "Entity that processes data on behalf of controller",
                NormalizationLevel::Strict,
            )
            .add_variant("processor")
            .add_variant("service provider"),
        );
        normalizer.add_term(
            ComplianceTerm::new(
                "personal data",
                RegulatoryDomain::DataProtection,
                "Information relating to an identified or identifiable person",
                NormalizationLevel::Strict,
            )
            .add_variant("personally identifiable information")
            .add_variant("PII")
            .add_variant("personal information"),
        );
        normalizer.add_term(
            ComplianceTerm::new(
                "consent",
                RegulatoryDomain::DataProtection,
                "Freely given, specific, informed indication of wishes",
                NormalizationLevel::Standard,
            )
            .add_variant("user consent")
            .add_variant("data subject consent")
            .add_variant("authorization"),
        );
        normalizer.add_term(
            ComplianceTerm::new(
                "prudential requirements",
                RegulatoryDomain::FinancialServices,
                "Financial soundness and risk management standards",
                NormalizationLevel::Standard,
            )
            .add_variant("capital requirements")
            .add_variant("liquidity requirements"),
        );
        normalizer.add_term(
            ComplianceTerm::new(
                "emissions trading",
                RegulatoryDomain::Environmental,
                "Market-based approach to controlling pollution",
                NormalizationLevel::Flexible,
            )
            .add_variant("cap and trade")
            .add_variant("carbon trading")
            .add_variant("emissions market"),
        );
        normalizer
    }
    /// Adds a compliance term.
    pub fn add_term(&mut self, term: ComplianceTerm) {
        self.by_domain
            .entry(term.domain)
            .or_default()
            .push(term.clone());
        self.terms.insert(term.canonical.clone(), term);
    }
    /// Normalizes a term to its canonical form.
    pub fn normalize(&self, term: &str) -> Option<String> {
        for compliance_term in self.terms.values() {
            if compliance_term.matches(term) {
                return Some(compliance_term.canonical.clone());
            }
        }
        None
    }
    /// Normalizes a term within a specific domain.
    pub fn normalize_in_domain(&self, term: &str, domain: RegulatoryDomain) -> Option<String> {
        self.by_domain.get(&domain).and_then(|terms| {
            terms
                .iter()
                .find(|t| t.matches(term))
                .map(|t| t.canonical.clone())
        })
    }
    /// Validates if a term is correctly normalized.
    pub fn is_normalized(&self, term: &str) -> bool {
        self.terms.contains_key(term)
    }
    /// Gets all accepted variants for a canonical term.
    pub fn get_variants(&self, canonical: &str) -> Vec<String> {
        self.terms
            .get(canonical)
            .map(|t| t.variants.clone())
            .unwrap_or_default()
    }
    /// Gets the total number of compliance terms.
    pub fn term_count(&self) -> usize {
        self.terms.len()
    }
    /// Gets terms by domain.
    pub fn get_by_domain(&self, domain: RegulatoryDomain) -> Vec<&ComplianceTerm> {
        self.by_domain
            .get(&domain)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }
}
/// Contract clause classifier.
pub struct ClauseClassifier {
    /// Classification patterns (keywords -> class)
    pub(super) patterns: HashMap<ClauseClass, Vec<String>>,
    /// Minimum confidence threshold
    pub(super) threshold: f64,
}
impl ClauseClassifier {
    /// Creates a new clause classifier with default patterns.
    pub fn new() -> Self {
        let mut patterns = HashMap::new();
        patterns.insert(
            ClauseClass::Payment,
            vec![
                "payment".to_string(),
                "pay".to_string(),
                "fee".to_string(),
                "compensation".to_string(),
                "invoice".to_string(),
            ],
        );
        patterns.insert(
            ClauseClass::Termination,
            vec![
                "termination".to_string(),
                "terminate".to_string(),
                "cancellation".to_string(),
                "cancel".to_string(),
            ],
        );
        patterns.insert(
            ClauseClass::Confidentiality,
            vec![
                "confidential".to_string(),
                "confidentiality".to_string(),
                "non-disclosure".to_string(),
                "proprietary".to_string(),
            ],
        );
        patterns.insert(
            ClauseClass::LiabilityLimitation,
            vec![
                "liability".to_string(),
                "limitation of liability".to_string(),
                "limited to".to_string(),
                "no liability".to_string(),
            ],
        );
        patterns.insert(
            ClauseClass::Indemnification,
            vec![
                "indemnify".to_string(),
                "indemnification".to_string(),
                "hold harmless".to_string(),
            ],
        );
        patterns.insert(
            ClauseClass::ForceMajeure,
            vec![
                "force majeure".to_string(),
                "act of god".to_string(),
                "beyond control".to_string(),
            ],
        );
        patterns.insert(
            ClauseClass::DisputeResolution,
            vec![
                "dispute".to_string(),
                "arbitration".to_string(),
                "mediation".to_string(),
                "litigation".to_string(),
            ],
        );
        patterns.insert(
            ClauseClass::IntellectualProperty,
            vec![
                "intellectual property".to_string(),
                "copyright".to_string(),
                "patent".to_string(),
                "trademark".to_string(),
            ],
        );
        patterns.insert(
            ClauseClass::GoverningLaw,
            vec![
                "governing law".to_string(),
                "applicable law".to_string(),
                "jurisdiction".to_string(),
            ],
        );
        patterns.insert(
            ClauseClass::Warranties,
            vec![
                "warranty".to_string(),
                "warrants".to_string(),
                "represent".to_string(),
                "representation".to_string(),
            ],
        );
        Self {
            patterns,
            threshold: 0.5,
        }
    }
    /// Classifies a clause.
    pub fn classify(&self, clause: &str) -> Option<ClassifiedClause> {
        let clause_lower = clause.to_lowercase();
        let mut scores: Vec<(ClauseClass, f64)> = Vec::new();
        for (class, keywords) in &self.patterns {
            let mut score = 0.0;
            for keyword in keywords {
                if clause_lower.contains(keyword) {
                    score += 1.0;
                }
            }
            if score > 0.0 {
                let confidence = (score / keywords.len() as f64).min(1.0);
                scores.push((class.clone(), confidence));
            }
        }
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        if let Some((class, confidence)) = scores.first()
            && *confidence >= self.threshold
        {
            let mut result = ClassifiedClause::new(clause, class.clone(), *confidence);
            for (alt_class, alt_conf) in scores.iter().skip(1).take(2) {
                result = result.add_alternative(alt_class.clone(), *alt_conf);
            }
            return Some(result);
        }
        None
    }
    /// Sets the confidence threshold.
    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.threshold = threshold.clamp(0.0, 1.0);
        self
    }
    /// Adds a custom pattern.
    pub fn add_pattern(&mut self, class: ClauseClass, keywords: Vec<String>) {
        self.patterns.insert(class, keywords);
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
/// Classified clause with confidence score.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassifiedClause {
    /// Original clause text
    pub text: String,
    /// Predicted class
    pub class: ClauseClass,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Alternative classifications
    pub alternatives: Vec<(ClauseClass, f64)>,
}
impl ClassifiedClause {
    /// Creates a new classified clause.
    pub fn new(text: impl Into<String>, class: ClauseClass, confidence: f64) -> Self {
        Self {
            text: text.into(),
            class,
            confidence: confidence.clamp(0.0, 1.0),
            alternatives: Vec::new(),
        }
    }
    /// Adds an alternative classification.
    pub fn add_alternative(mut self, class: ClauseClass, confidence: f64) -> Self {
        self.alternatives.push((class, confidence.clamp(0.0, 1.0)));
        self
    }
}
/// Speech-to-text legal transcription engine.
#[derive(Debug, Clone)]
pub struct LegalSpeechTranscriber {
    /// The locale for transcription.
    pub locale: Locale,
    /// Audio quality expectation.
    pub audio_quality: AudioQuality,
    /// Legal domain specialization.
    pub domain: LegalSpeechDomain,
    /// Whether to enable speaker diarization.
    pub speaker_diarization: bool,
    /// Whether to use legal vocabulary boost.
    pub legal_vocabulary_boost: bool,
    /// Legal dictionary for vocabulary boosting.
    pub dictionary: Option<LegalDictionary>,
}
impl LegalSpeechTranscriber {
    /// Creates a new legal speech transcriber.
    pub fn new(locale: Locale, domain: LegalSpeechDomain) -> Self {
        Self {
            locale,
            audio_quality: AudioQuality::Medium,
            domain,
            speaker_diarization: false,
            legal_vocabulary_boost: false,
            dictionary: None,
        }
    }
    /// Creates a transcriber for court proceedings.
    pub fn for_court_proceedings(locale: Locale) -> Self {
        Self::new(locale, LegalSpeechDomain::CourtProceedings)
            .with_speaker_diarization(true)
            .with_audio_quality(AudioQuality::Studio)
            .with_legal_vocabulary_boost(true)
    }
    /// Creates a transcriber for depositions.
    pub fn for_depositions(locale: Locale) -> Self {
        Self::new(locale, LegalSpeechDomain::Depositions)
            .with_speaker_diarization(true)
            .with_audio_quality(AudioQuality::High)
            .with_legal_vocabulary_boost(true)
    }
    /// Sets the audio quality.
    pub fn with_audio_quality(mut self, quality: AudioQuality) -> Self {
        self.audio_quality = quality;
        self
    }
    /// Enables speaker diarization.
    pub fn with_speaker_diarization(mut self, enable: bool) -> Self {
        self.speaker_diarization = enable;
        self
    }
    /// Enables legal vocabulary boosting.
    pub fn with_legal_vocabulary_boost(mut self, enable: bool) -> Self {
        self.legal_vocabulary_boost = enable;
        self
    }
    /// Sets the legal dictionary for vocabulary boosting.
    pub fn with_dictionary(mut self, dictionary: LegalDictionary) -> Self {
        self.dictionary = Some(dictionary);
        self
    }
    /// Transcribes audio (placeholder - would integrate with actual ASR engine).
    /// In production, this would call services like Google Speech-to-Text,
    /// Azure Speech Services, or custom models.
    pub fn transcribe(&self, _audio_data: &[u8]) -> Vec<TranscriptionSegment> {
        vec![]
    }
    /// Transcribes a single utterance and returns text with confidence.
    pub fn transcribe_utterance(&self, _audio_data: &[u8]) -> (String, f64) {
        ("".to_string(), 0.0)
    }
    /// Gets legal vocabulary hints for the transcription engine.
    pub fn get_vocabulary_hints(&self) -> Vec<String> {
        let mut hints = Vec::new();
        if self.legal_vocabulary_boost {
            if let Some(ref dict) = self.dictionary {
                for (key, _value) in &dict.translations {
                    hints.push(key.clone());
                }
                for (_term, abbrev) in &dict.abbreviations {
                    hints.push(abbrev.clone());
                }
            }
            match self.domain {
                LegalSpeechDomain::CourtProceedings => {
                    hints.extend(vec![
                        "Your Honor".to_string(),
                        "objection".to_string(),
                        "sustained".to_string(),
                        "overruled".to_string(),
                        "counsel".to_string(),
                        "witness".to_string(),
                        "testimony".to_string(),
                        "evidence".to_string(),
                    ]);
                }
                LegalSpeechDomain::Depositions => {
                    hints.extend(vec![
                        "deposition".to_string(),
                        "reporter".to_string(),
                        "exhibit".to_string(),
                        "marked".to_string(),
                    ]);
                }
                LegalSpeechDomain::ContractNegotiations => {
                    hints.extend(vec![
                        "clause".to_string(),
                        "provision".to_string(),
                        "amendment".to_string(),
                        "consideration".to_string(),
                    ]);
                }
                _ => {}
            }
        }
        hints
    }
}
/// Language-agnostic legal reasoning engine.
#[derive(Debug, Clone)]
pub struct LegalReasoningEngine {
    /// Knowledge graph.
    pub knowledge_graph: MultilingualKnowledgeGraph,
    /// Concept mapper.
    pub concept_mapper: ConceptMapper,
    /// Case search engine.
    pub case_search: CrossLingualCaseSearch,
}
impl LegalReasoningEngine {
    /// Creates a new legal reasoning engine.
    pub fn new(embedder: MultilingualEmbedder) -> Self {
        Self {
            knowledge_graph: MultilingualKnowledgeGraph::new(embedder.clone()),
            concept_mapper: ConceptMapper::with_defaults(embedder.clone()),
            case_search: CrossLingualCaseSearch::new(embedder),
        }
    }
    /// Analyzes a legal query across languages.
    pub fn analyze_query(&self, query: &str, locale: Locale) -> AnalysisResult {
        let concept = self.concept_mapper.find_concept(query, locale.clone());
        let cases = self.case_search.search(query, locale.clone(), 5);
        let similar_nodes = self
            .knowledge_graph
            .find_similar_nodes(query, locale.clone(), 5);
        AnalysisResult {
            query: query.to_string(),
            locale,
            matched_concept: concept,
            similar_cases: cases,
            related_nodes: similar_nodes,
        }
    }
    /// Finds equivalent legal concepts across jurisdictions.
    pub fn find_cross_jurisdictional_equivalents(
        &self,
        term: &str,
        source_locale: Locale,
    ) -> HashMap<String, String> {
        self.concept_mapper
            .map_term_across_languages(term, source_locale)
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
/// Regulatory equivalence mapper.
#[derive(Debug, Clone)]
pub struct RegulatoryEquivalenceMapper {
    /// Equivalences indexed by source jurisdiction.
    equivalences: HashMap<String, Vec<RegulatoryEquivalence>>,
    /// Equivalences indexed by domain.
    by_domain: HashMap<RegulatoryDomain, Vec<RegulatoryEquivalence>>,
}
impl RegulatoryEquivalenceMapper {
    /// Creates a new equivalence mapper.
    pub fn new() -> Self {
        Self {
            equivalences: HashMap::new(),
            by_domain: HashMap::new(),
        }
    }
    /// Creates a mapper with default equivalences.
    pub fn with_defaults() -> Self {
        let mut mapper = Self::new();
        mapper.add_equivalence(
            RegulatoryEquivalence::new(
                "EU",
                "US",
                RegulatoryDomain::DataProtection,
                RegulatoryEquivalenceLevel::Conditional,
            )
            .with_basis("EU-US Privacy Shield (invalidated), DPF")
            .add_condition("Must use Standard Contractual Clauses")
            .add_condition("Adequacy decision required")
            .with_review_date("2023-07-10"),
        );
        mapper.add_equivalence(
            RegulatoryEquivalence::new(
                "EU",
                "GB",
                RegulatoryDomain::DataProtection,
                RegulatoryEquivalenceLevel::Full,
            )
            .with_basis("EU-UK Trade and Cooperation Agreement")
            .with_review_date("2021-06-28"),
        );
        mapper.add_equivalence(
            RegulatoryEquivalence::new(
                "US",
                "EU",
                RegulatoryDomain::FinancialServices,
                RegulatoryEquivalenceLevel::Partial,
            )
            .with_basis("SEC-ESMA MoU")
            .add_condition("Limited to securities trading")
            .with_review_date("2020-01-01"),
        );
        mapper.add_equivalence(
            RegulatoryEquivalence::new(
                "AU",
                "NZ",
                RegulatoryDomain::ProfessionalQualifications,
                RegulatoryEquivalenceLevel::Full,
            )
            .with_basis("Trans-Tasman Mutual Recognition Arrangement")
            .with_review_date("1997-05-01"),
        );
        mapper.add_equivalence(
            RegulatoryEquivalence::new(
                "CA",
                "US",
                RegulatoryDomain::ProductSafety,
                RegulatoryEquivalenceLevel::Conditional,
            )
            .with_basis("USMCA (formerly NAFTA)")
            .add_condition("Compliance with USMCA standards")
            .with_review_date("2020-07-01"),
        );
        mapper
    }
    /// Adds an equivalence mapping.
    pub fn add_equivalence(&mut self, equivalence: RegulatoryEquivalence) {
        self.equivalences
            .entry(equivalence.source_jurisdiction.clone())
            .or_default()
            .push(equivalence.clone());
        self.by_domain
            .entry(equivalence.domain)
            .or_default()
            .push(equivalence);
    }
    /// Gets equivalences for a source jurisdiction.
    pub fn get_equivalences(&self, source_jurisdiction: &str) -> Vec<&RegulatoryEquivalence> {
        self.equivalences
            .get(source_jurisdiction)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }
    /// Gets equivalences for a specific domain.
    pub fn get_by_domain(&self, domain: RegulatoryDomain) -> Vec<&RegulatoryEquivalence> {
        self.by_domain
            .get(&domain)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }
    /// Checks if two jurisdictions have equivalence in a domain.
    pub fn has_equivalence(
        &self,
        source: &str,
        target: &str,
        domain: RegulatoryDomain,
    ) -> Option<RegulatoryEquivalenceLevel> {
        self.equivalences.get(source).and_then(|eqs| {
            eqs.iter()
                .find(|eq| eq.target_jurisdiction == target && eq.domain == domain)
                .map(|eq| eq.level)
        })
    }
    /// Gets the total number of equivalences.
    pub fn equivalence_count(&self) -> usize {
        self.equivalences.values().map(|v| v.len()).sum()
    }
    /// Gets the number of tracked source jurisdictions.
    pub fn jurisdiction_count(&self) -> usize {
        self.equivalences.len()
    }
}
/// Low-resource language support strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LowResourceStrategy {
    /// Use related high-resource language as fallback.
    FallbackToRelated,
    /// Use transfer learning from similar language.
    TransferLearning,
    /// Use multilingual model.
    MultilingualModel,
    /// Use community contributions.
    CommunityDriven,
}
/// Glossary violation type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViolationType {
    /// Mandatory term missing in translation
    MissingMandatoryTerm,
    /// Forbidden term found in translation
    ForbiddenTermUsed,
}
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
        match (&self.country, &other.country) {
            (Some(c1), Some(c2)) => c1 == c2,
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
/// Comprehensive legal document analyzer.
pub struct LegalDocumentAnalyzer {
    clause_extractor: ClauseExtractor,
    party_identifier: PartyIdentifier,
    obligation_extractor: ObligationExtractor,
    deadline_extractor: DeadlineExtractor,
    jurisdiction_detector: JurisdictionDetector,
    risk_scorer: LegalRiskScorer,
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
/// Currency formatter for monetary values.
#[derive(Debug, Clone)]
pub struct CurrencyFormatter {
    pub(super) locale: Locale,
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
                let formatted = format!("{:.prec$}", amount, prec = decimal_places);
                formatted.replace('.', ",")
            }
            _ => format!("{:.prec$}", amount, prec = decimal_places),
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
        self.format_oscola_case(c)
    }
    fn format_mcgill_case(&self, c: &CitationComponents) -> String {
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
        let mut result = c.title.clone();
        if let Some(year) = c.year {
            result.push_str(&format!("（{}年）", year));
        }
        if let Some(page) = &c.page {
            result.push_str(&format!(" 第{}条", page));
        }
        result
    }
    fn format_harvard_case(&self, c: &CitationComponents) -> String {
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
        let mut result = c.title.clone();
        if let Some(year) = c.year {
            result.push_str(&format!(" {}", year));
        }
        result
    }
    fn format_apa_case(&self, c: &CitationComponents) -> String {
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
        let mut result = c.title.clone();
        if let Some(year) = c.year {
            result.push_str(&format!(" ({})", year));
        }
        result
    }
    fn format_chicago_case(&self, c: &CitationComponents) -> String {
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
        let mut result = c.title.clone();
        if let Some(year) = c.year {
            result.push_str(&format!(" ({})", year));
        }
        result
    }
    fn format_indian_case(&self, c: &CitationComponents) -> String {
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
        let mut result = c.title.clone();
        if let Some(year) = c.year {
            result.push_str(&format!(", {}", year));
        }
        result
    }
    fn format_custom_case(&self, c: &CitationComponents, template: &str) -> String {
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
            _ => CitationStyle::Bluebook,
        }
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
            created_at: 0,
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
/// Knowledge graph edge representing a relationship.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeGraphEdge {
    /// Source node ID.
    pub from_node: String,
    /// Target node ID.
    pub to_node: String,
    /// Relationship type.
    pub relationship: String,
    /// Edge properties.
    pub properties: HashMap<String, String>,
}
impl KnowledgeGraphEdge {
    /// Creates a new knowledge graph edge.
    pub fn new(
        from_node: impl Into<String>,
        to_node: impl Into<String>,
        relationship: impl Into<String>,
    ) -> Self {
        Self {
            from_node: from_node.into(),
            to_node: to_node.into(),
            relationship: relationship.into(),
            properties: HashMap::new(),
        }
    }
    /// Adds a property to the edge.
    pub fn with_property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }
}
