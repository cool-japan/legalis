//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

use super::functions::I18nResult;
use super::functions_3::QualityScore;
use super::types::{CitationType, EURegulationType, W3CComplianceReport};
use super::types_4::{EquivalentTerm, MTTranslation, StandardType, VariableType};
use super::types_5::{
    CitationError, CitationValidator, MultilingualEmbedder, SignLanguageReference,
};
use super::types_6::{CitationComponents, CitationStyle, ColonialPower};
use super::types_7::StandardAdoption;
use super::types_8::{EURegulationTerm, LegalDictionary, SignLanguageType};
use super::types_9::{ExtractedObligation, StyleAttribute};
use super::types_10::{CitationFormatter, KnowledgeGraphEdge, Locale, ViolationType};
use super::types_11::TranslationEngine;
use super::types_12::{EquivalenceLevel, KnowledgeGraphNode, PostEditAction};

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
            "US" => Self::new("US", 10, 1),
            "GB" | "UK" => Self::new("GB", 4, 6),
            "JP" => Self::new("JP", 4, 1),
            "AU" => Self::new("AU", 7, 1),
            "CA" => Self::new("CA", 4, 1),
            "IN" => Self::new("IN", 4, 1),
            "DE" | "FR" | "IT" | "ES" | "NL" | "PT" | "PL" => Self::new(jurisdiction, 1, 1),
            _ => Self::new(jurisdiction, 1, 1),
        }
    }
    /// Calculates the fiscal year for a given Gregorian date.
    /// Returns the fiscal year number.
    pub fn get_fiscal_year(&self, year: i32, month: u32, day: u32) -> i32 {
        if month > self.start_month || (month == self.start_month && day >= self.start_day) {
            if self.start_month == 1 && self.start_day == 1 {
                year
            } else {
                year + 1
            }
        } else {
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
        let (next_year, next_month, next_day) = if start_month == 12 {
            (start_year + 1, 1, start_day)
        } else {
            (start_year, start_month + 1, start_day)
        };
        if next_day > 1 {
            (next_year, next_month, next_day - 1)
        } else {
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
/// Multilingual knowledge graph for legal concepts.
#[derive(Debug, Clone)]
pub struct MultilingualKnowledgeGraph {
    /// Graph nodes.
    pub nodes: HashMap<String, KnowledgeGraphNode>,
    /// Graph edges.
    pub edges: Vec<KnowledgeGraphEdge>,
    /// Multilingual embedder.
    pub embedder: MultilingualEmbedder,
}
impl MultilingualKnowledgeGraph {
    /// Creates a new multilingual knowledge graph.
    pub fn new(embedder: MultilingualEmbedder) -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
            embedder,
        }
    }
    /// Adds a node to the graph.
    pub fn add_node(&mut self, mut node: KnowledgeGraphNode) {
        if node.embedding.is_none() {
            let embedding = self.embedder.embed(&node.label, node.locale.clone());
            node.embedding = Some(embedding);
        }
        self.nodes.insert(node.node_id.clone(), node);
    }
    /// Adds an edge to the graph.
    pub fn add_edge(&mut self, edge: KnowledgeGraphEdge) {
        self.edges.push(edge);
    }
    /// Gets a node by ID.
    pub fn get_node(&self, node_id: &str) -> Option<&KnowledgeGraphNode> {
        self.nodes.get(node_id)
    }
    /// Finds nodes by type.
    pub fn find_nodes_by_type(&self, node_type: &str) -> Vec<&KnowledgeGraphNode> {
        self.nodes
            .values()
            .filter(|node| node.node_type == node_type)
            .collect()
    }
    /// Finds outgoing edges from a node.
    pub fn find_outgoing_edges(&self, node_id: &str) -> Vec<&KnowledgeGraphEdge> {
        self.edges
            .iter()
            .filter(|edge| edge.from_node == node_id)
            .collect()
    }
    /// Finds incoming edges to a node.
    pub fn find_incoming_edges(&self, node_id: &str) -> Vec<&KnowledgeGraphEdge> {
        self.edges
            .iter()
            .filter(|edge| edge.to_node == node_id)
            .collect()
    }
    /// Finds similar nodes using semantic search.
    pub fn find_similar_nodes(
        &self,
        query: &str,
        locale: Locale,
        max_results: usize,
    ) -> Vec<(String, f32)> {
        let query_embedding = self.embedder.embed(query, locale);
        let mut results: Vec<(String, f32)> = self
            .nodes
            .values()
            .filter_map(|node| {
                node.embedding.as_ref().map(|emb| {
                    let similarity = query_embedding.cosine_similarity(emb);
                    (node.node_id.clone(), similarity)
                })
            })
            .collect();
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        results.into_iter().take(max_results).collect()
    }
    /// Returns graph statistics.
    pub fn stats(&self) -> (usize, usize) {
        (self.nodes.len(), self.edges.len())
    }
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
/// Neural machine translator for legal documents.
///
/// Simulates legal-domain neural machine translation with quality estimation.
#[derive(Debug, Clone)]
pub struct NeuralMachineTranslator {
    /// Translation engine
    pub(super) engine: TranslationEngine,
    /// Quality threshold (0.0 to 1.0)
    quality_threshold: QualityScore,
    /// Legal dictionary for domain adaptation
    pub(super) dictionary: Option<Arc<LegalDictionary>>,
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
        let translated_text = self.simulate_translation(text, source, target);
        let quality_score = self.estimate_quality(text, &translated_text, source, target);
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
        if let Some(dict) = &self.dictionary
            && let Some(translation) = dict.translate(text)
        {
            return translation.to_string();
        }
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
        let mut score: f32 = 0.8;
        if translated_text.len() < source_text.len() / 2 {
            score -= 0.2;
        }
        if self.dictionary.is_some() && !translated_text.starts_with('[') {
            score += 0.15;
        }
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
/// Sign language referencer for legal terminology.
#[derive(Debug, Clone)]
pub struct SignLanguageReferencer {
    /// References indexed by term
    pub(super) references: HashMap<String, Vec<SignLanguageReference>>,
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
                        r.video_url
                            .as_ref()
                            .map(|url| {
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
            true
        }
    }
    /// Checks if the locale has a valid script code (if present).
    pub fn has_valid_script_code(&self) -> bool {
        if let Some(ref script) = self.locale.script {
            script.len() == 4
                && script
                    .chars()
                    .next()
                    .expect("invariant: script has length 4 so first char exists")
                    .is_ascii_uppercase()
        } else {
            true
        }
    }
    /// Checks if text direction is properly specified.
    pub fn has_text_direction(&self) -> bool {
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
/// Style profile for legal text.
#[derive(Debug, Clone)]
pub struct StyleProfile {
    /// Map of style attributes to their values.
    attributes: HashMap<StyleAttribute, String>,
    /// Locale-specific style preferences.
    pub(super) locale_preferences: HashMap<Locale, HashMap<StyleAttribute, String>>,
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
/// International standard adoption status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AdoptionStatus {
    /// Fully adopted in national law.
    FullyAdopted,
    /// Partially adopted.
    PartiallyAdopted,
    /// In progress (drafting or review).
    InProgress,
    /// Not adopted.
    NotAdopted,
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
/// Name order convention for different cultures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NameOrder {
    /// Given name first, family name last (Western style)
    GivenFirst,
    /// Family name first, given name last (East Asian style)
    FamilyFirst,
}
/// Subtitle position on screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubtitlePosition {
    /// Bottom center (default).
    BottomCenter,
    /// Top center.
    TopCenter,
    /// Bottom left.
    BottomLeft,
    /// Bottom right.
    BottomRight,
    /// Top left.
    TopLeft,
    /// Top right.
    TopRight,
}
/// Court proceeding participant role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CourtParticipantRole {
    /// Judge or magistrate.
    Judge,
    /// Prosecutor or district attorney.
    Prosecutor,
    /// Defense attorney.
    DefenseAttorney,
    /// Plaintiff's attorney.
    PlaintiffAttorney,
    /// Defendant's attorney.
    DefendantAttorney,
    /// Witness.
    Witness,
    /// Defendant.
    Defendant,
    /// Plaintiff.
    Plaintiff,
    /// Court reporter.
    CourtReporter,
    /// Interpreter.
    Interpreter,
    /// Jury member.
    Juror,
}
/// Audio quality level for speech recognition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AudioQuality {
    /// Low quality (8kHz, telephony).
    Low,
    /// Medium quality (16kHz, standard recording).
    Medium,
    /// High quality (44.1kHz, professional recording).
    High,
    /// Studio quality (48kHz+, court recording systems).
    Studio,
}
/// Text direction for layout and display.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextDirection {
    /// Left-to-Right (e.g., English, French, German)
    LTR,
    /// Right-to-Left (e.g., Arabic, Hebrew)
    RTL,
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
        let sentences: Vec<&str> = text.split(&['.', ';', '!'][..]).collect();
        for (i, sentence) in sentences.iter().enumerate() {
            let sentence_lower = sentence.to_lowercase();
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
        let words: Vec<&str> = sentence.split_whitespace().collect();
        for (i, word) in words.iter().enumerate() {
            let word_lower = word.to_lowercase();
            if word_lower.contains("shall")
                || word_lower.contains("must")
                || word_lower.contains("may")
            {
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
            let mut adapted = self.source_profile.clone();
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
            VariableType::Date => value.contains('-') || value.contains('/'),
            VariableType::List => true,
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
/// EU regulation language aligner.
#[derive(Debug, Clone)]
pub struct EURegulationAligner {
    /// Terms indexed by regulation type.
    pub(super) terms: HashMap<EURegulationType, Vec<EURegulationTerm>>,
    /// Reverse index from canonical term to regulation term.
    pub(super) term_index: HashMap<String, EURegulationTerm>,
}
impl EURegulationAligner {
    /// Creates a new EU regulation aligner.
    pub fn new() -> Self {
        Self {
            terms: HashMap::new(),
            term_index: HashMap::new(),
        }
    }
    /// Creates an aligner with default GDPR terms.
    pub fn with_gdpr_defaults() -> Self {
        let mut aligner = Self::new();
        aligner.add_term(
            EURegulationTerm::new(
                EURegulationType::GDPR,
                "personal data",
                "Any information relating to an identified or identifiable natural person",
            )
            .add_translation("de", "personenbezogene Daten")
            .add_translation("fr", "données à caractère personnel")
            .add_translation("es", "datos personales")
            .add_translation("it", "dati personali")
            .with_article("Article 4(1)"),
        );
        aligner.add_term(
            EURegulationTerm::new(
                EURegulationType::GDPR,
                "data controller",
                "Natural or legal person which determines the purposes and means of the processing",
            )
            .add_translation("de", "Verantwortlicher")
            .add_translation("fr", "responsable du traitement")
            .add_translation("es", "responsable del tratamiento")
            .add_translation("it", "titolare del trattamento")
            .with_article("Article 4(7)"),
        );
        aligner.add_term(
            EURegulationTerm::new(
                EURegulationType::GDPR,
                "data processor",
                "Natural or legal person which processes data on behalf of the controller",
            )
            .add_translation("de", "Auftragsverarbeiter")
            .add_translation("fr", "sous-traitant")
            .add_translation("es", "encargado del tratamiento")
            .add_translation("it", "responsabile del trattamento")
            .with_article("Article 4(8)"),
        );
        aligner
            .add_term(
                EURegulationTerm::new(
                        EURegulationType::GDPR,
                        "consent",
                        "Freely given, specific, informed and unambiguous indication of the data subject's wishes",
                    )
                    .add_translation("de", "Einwilligung")
                    .add_translation("fr", "consentement")
                    .add_translation("es", "consentimiento")
                    .add_translation("it", "consenso")
                    .with_article("Article 4(11)"),
            );
        aligner
    }
    /// Adds a term to the aligner.
    pub fn add_term(&mut self, term: EURegulationTerm) {
        self.term_index
            .insert(term.canonical_term.clone(), term.clone());
        self.terms.entry(term.regulation).or_default().push(term);
    }
    /// Gets all terms for a specific regulation.
    pub fn get_terms(&self, regulation: EURegulationType) -> Vec<&EURegulationTerm> {
        self.terms
            .get(&regulation)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }
    /// Translates a canonical term to a target language.
    pub fn translate_term(&self, canonical_term: &str, target_language: &str) -> Option<String> {
        self.term_index
            .get(canonical_term)
            .and_then(|term| term.get_translation(target_language).cloned())
    }
    /// Gets the total number of terms.
    pub fn term_count(&self) -> usize {
        self.term_index.len()
    }
    /// Gets all supported regulations.
    pub fn supported_regulations(&self) -> Vec<EURegulationType> {
        self.terms.keys().copied().collect()
    }
}
/// International standard adoption tracker.
#[derive(Debug, Clone)]
pub struct StandardAdoptionTracker {
    /// Adoptions indexed by standard ID.
    adoptions: HashMap<String, Vec<StandardAdoption>>,
    /// Adoptions indexed by jurisdiction.
    by_jurisdiction: HashMap<String, Vec<StandardAdoption>>,
}
impl StandardAdoptionTracker {
    /// Creates a new adoption tracker.
    pub fn new() -> Self {
        Self {
            adoptions: HashMap::new(),
            by_jurisdiction: HashMap::new(),
        }
    }
    /// Creates a tracker with default adoptions.
    pub fn with_defaults() -> Self {
        let mut tracker = Self::new();
        tracker.add_adoption(
            StandardAdoption::new(
                "ISO 27001",
                StandardType::ISO,
                "US",
                AdoptionStatus::FullyAdopted,
            )
            .with_date("2013-10-01")
            .with_law("NIST SP 800-53"),
        );
        tracker.add_adoption(
            StandardAdoption::new(
                "ISO 27001",
                StandardType::ISO,
                "GB",
                AdoptionStatus::FullyAdopted,
            )
            .with_date("2005-11-01")
            .with_law("BS 7799"),
        );
        tracker.add_adoption(
            StandardAdoption::new(
                "UNCITRAL Model Law",
                StandardType::UNCITRAL,
                "US",
                AdoptionStatus::PartiallyAdopted,
            )
            .with_date("2000-07-07")
            .with_law("UETA")
            .add_deviation("State-level adoption varies"),
        );
        tracker.add_adoption(
            StandardAdoption::new(
                "Hague Convention 2005",
                StandardType::HagueConference,
                "US",
                AdoptionStatus::InProgress,
            )
            .with_date("2007-01-01"),
        );
        tracker.add_adoption(
            StandardAdoption::new(
                "RFC 2616",
                StandardType::IETF,
                "global",
                AdoptionStatus::FullyAdopted,
            )
            .with_date("1999-06-01"),
        );
        tracker
    }
    /// Adds an adoption record.
    pub fn add_adoption(&mut self, adoption: StandardAdoption) {
        self.adoptions
            .entry(adoption.standard_id.clone())
            .or_default()
            .push(adoption.clone());
        self.by_jurisdiction
            .entry(adoption.jurisdiction.clone())
            .or_default()
            .push(adoption);
    }
    /// Gets adoption records for a specific standard.
    pub fn get_standard_adoptions(&self, standard_id: &str) -> Vec<&StandardAdoption> {
        self.adoptions
            .get(standard_id)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }
    /// Gets adoption records for a specific jurisdiction.
    pub fn get_jurisdiction_adoptions(&self, jurisdiction: &str) -> Vec<&StandardAdoption> {
        self.by_jurisdiction
            .get(jurisdiction)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }
    /// Checks if a standard is fully adopted in a jurisdiction.
    pub fn is_fully_adopted(&self, standard_id: &str, jurisdiction: &str) -> bool {
        self.adoptions
            .get(standard_id)
            .map(|adoptions| {
                adoptions.iter().any(|a| {
                    a.jurisdiction == jurisdiction && a.status == AdoptionStatus::FullyAdopted
                })
            })
            .unwrap_or(false)
    }
    /// Gets the total number of tracked standards.
    pub fn standard_count(&self) -> usize {
        self.adoptions.len()
    }
    /// Gets the total number of adoption records.
    pub fn adoption_count(&self) -> usize {
        self.adoptions.values().map(|v| v.len()).sum()
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
    fn parse_bluebook_case(&self, citation: &str) -> Result<CitationComponents, CitationError> {
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
        if let Some(paren_start) = citation.rfind('(')
            && let Some(paren_end) = citation.rfind(')')
        {
            let paren_content = &citation[paren_start + 1..paren_end];
            let paren_parts: Vec<&str> = paren_content.split_whitespace().collect();
            if let Some(year_str) = paren_parts.last()
                && let Ok(year) = year_str.parse::<i32>()
            {
                components.year = Some(year);
            }
            if paren_parts.len() > 1 {
                components.court = Some(paren_parts[..paren_parts.len() - 1].join(" "));
            }
        }
        Ok(components)
    }
    fn parse_oscola_case(&self, citation: &str) -> Result<CitationComponents, CitationError> {
        let parts: Vec<&str> = citation.split('[').collect();
        if parts.is_empty() {
            return Err(CitationError::ParseError {
                reason: "Empty citation".to_string(),
            });
        }
        let title = parts[0].trim().to_string();
        let mut components = CitationComponents::new(title);
        if parts.len() > 1
            && let Some(year_end) = parts[1].find(']')
        {
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
        Ok(components)
    }
    fn parse_aglc_case(&self, citation: &str) -> Result<CitationComponents, CitationError> {
        self.parse_oscola_case(citation)
    }
    fn parse_mcgill_case(&self, citation: &str) -> Result<CitationComponents, CitationError> {
        self.parse_bluebook_case(citation)
    }
    fn parse_european_case(&self, citation: &str) -> Result<CitationComponents, CitationError> {
        Ok(CitationComponents::new(citation.trim()))
    }
    fn parse_japanese_case(&self, citation: &str) -> Result<CitationComponents, CitationError> {
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
        self.parse_bluebook_case(citation)
    }
    fn parse_custom_case(&self, citation: &str) -> Result<CitationComponents, CitationError> {
        Ok(CitationComponents::new(citation.trim()))
    }
    fn parse_bluebook_statute(&self, citation: &str) -> Result<CitationComponents, CitationError> {
        let mut components = CitationComponents::new(citation.trim());
        let parts: Vec<&str> = citation.split('§').collect();
        if parts.len() == 2 {
            components.reporter = Some(parts[0].trim().to_string());
            components.page = Some(parts[1].trim().to_string());
        }
        Ok(components)
    }
    fn parse_oscola_statute(&self, citation: &str) -> Result<CitationComponents, CitationError> {
        let mut components = CitationComponents::new(citation.trim());
        let words: Vec<&str> = citation.split_whitespace().collect();
        for word in &words {
            let cleaned_word = word.trim_matches(|c: char| !c.is_numeric());
            if let Ok(year) = cleaned_word.parse::<i32>()
                && (1000..=9999).contains(&year)
            {
                components.year = Some(year);
                break;
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
        let validator = CitationValidator::new(from_style.clone());
        if let Err(errors) = validator.validate_case(components) {
            return Err(CitationError::StyleViolation {
                style: format!("{}", from_style),
                reason: format!("{} validation errors", errors.len()),
            });
        }
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
        let validator = CitationValidator::new(from_style.clone());
        if let Err(errors) = validator.validate_statute(components) {
            return Err(CitationError::StyleViolation {
                style: format!("{}", from_style),
                reason: format!("{} validation errors", errors.len()),
            });
        }
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
        for (legal_term, plain_term) in &self.conversions {
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
