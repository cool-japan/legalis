//! Multi-Language Legal Support (v0.2.8)
//!
//! This module provides comprehensive multilingual support for legal analysis,
//! including cross-lingual analysis, terminology translation, statute comparison,
//! jurisdiction-aware translation, and legal jargon normalization.

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Supported legal language codes (ISO 639-1)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LanguageCode {
    /// English
    EN,
    /// Spanish
    ES,
    /// French
    FR,
    /// German
    DE,
    /// Italian
    IT,
    /// Portuguese
    PT,
    /// Dutch
    NL,
    /// Polish
    PL,
    /// Russian
    RU,
    /// Chinese (Simplified)
    ZH,
    /// Japanese
    JA,
    /// Korean
    KO,
    /// Arabic
    AR,
}

impl LanguageCode {
    /// Returns the language name in English
    pub fn name(&self) -> &'static str {
        match self {
            Self::EN => "English",
            Self::ES => "Spanish",
            Self::FR => "French",
            Self::DE => "German",
            Self::IT => "Italian",
            Self::PT => "Portuguese",
            Self::NL => "Dutch",
            Self::PL => "Polish",
            Self::RU => "Russian",
            Self::ZH => "Chinese",
            Self::JA => "Japanese",
            Self::KO => "Korean",
            Self::AR => "Arabic",
        }
    }

    /// Returns the ISO 639-1 code
    pub fn code(&self) -> &'static str {
        match self {
            Self::EN => "en",
            Self::ES => "es",
            Self::FR => "fr",
            Self::DE => "de",
            Self::IT => "it",
            Self::PT => "pt",
            Self::NL => "nl",
            Self::PL => "pl",
            Self::RU => "ru",
            Self::ZH => "zh",
            Self::JA => "ja",
            Self::KO => "ko",
            Self::AR => "ar",
        }
    }
}

/// Legal jurisdiction information with language and system metadata
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct JurisdictionInfo {
    /// Country code (ISO 3166-1 alpha-2)
    pub country: String,
    /// State or province (optional)
    pub state: Option<String>,
    /// Primary language
    pub language: LanguageCode,
    /// Legal system type
    pub legal_system: LegalSystemType,
}

/// Type of legal system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LegalSystemType {
    /// Common law (e.g., US, UK)
    CommonLaw,
    /// Civil law (e.g., France, Germany)
    CivilLaw,
    /// Religious law (e.g., Sharia)
    ReligiousLaw,
    /// Customary law
    CustomaryLaw,
    /// Mixed/Hybrid system
    Mixed,
}

/// Cross-lingual legal analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossLingualAnalysis {
    /// Source language
    pub source_language: LanguageCode,
    /// Target language
    pub target_language: LanguageCode,
    /// Original text
    pub original_text: String,
    /// Analyzed text in target language
    pub analyzed_text: String,
    /// Key legal concepts identified
    pub concepts: Vec<LegalConcept>,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f64,
    /// Jurisdiction context
    pub jurisdiction: Option<JurisdictionInfo>,
}

/// Legal concept with multilingual support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalConcept {
    /// Concept identifier
    pub id: String,
    /// Term in source language
    pub source_term: String,
    /// Term in target language
    pub target_term: String,
    /// Definition in target language
    pub definition: String,
    /// Related concepts
    pub related: Vec<String>,
}

/// Cross-lingual legal analyzer
pub struct CrossLingualAnalyzer {
    /// Terminology database
    terminology: HashMap<(LanguageCode, LanguageCode), HashMap<String, String>>,
    /// Legal concept database
    #[allow(dead_code)]
    concepts: HashMap<String, LegalConcept>,
}

impl CrossLingualAnalyzer {
    /// Creates a new cross-lingual analyzer
    pub fn new() -> Self {
        let mut analyzer = Self {
            terminology: HashMap::new(),
            concepts: HashMap::new(),
        };
        analyzer.init_terminology();
        analyzer
    }

    /// Initialize basic legal terminology translations
    fn init_terminology(&mut self) {
        // English to Spanish
        let mut en_es = HashMap::new();
        en_es.insert("contract".to_string(), "contrato".to_string());
        en_es.insert("law".to_string(), "ley".to_string());
        en_es.insert("statute".to_string(), "estatuto".to_string());
        en_es.insert("regulation".to_string(), "reglamento".to_string());
        en_es.insert("jurisdiction".to_string(), "jurisdicción".to_string());
        en_es.insert("plaintiff".to_string(), "demandante".to_string());
        en_es.insert("defendant".to_string(), "demandado".to_string());
        en_es.insert("court".to_string(), "tribunal".to_string());
        self.terminology
            .insert((LanguageCode::EN, LanguageCode::ES), en_es);

        // English to French
        let mut en_fr = HashMap::new();
        en_fr.insert("contract".to_string(), "contrat".to_string());
        en_fr.insert("law".to_string(), "loi".to_string());
        en_fr.insert("statute".to_string(), "statut".to_string());
        en_fr.insert("regulation".to_string(), "règlement".to_string());
        en_fr.insert("jurisdiction".to_string(), "juridiction".to_string());
        en_fr.insert("plaintiff".to_string(), "demandeur".to_string());
        en_fr.insert("defendant".to_string(), "défendeur".to_string());
        en_fr.insert("court".to_string(), "tribunal".to_string());
        self.terminology
            .insert((LanguageCode::EN, LanguageCode::FR), en_fr);

        // English to German
        let mut en_de = HashMap::new();
        en_de.insert("contract".to_string(), "Vertrag".to_string());
        en_de.insert("law".to_string(), "Gesetz".to_string());
        en_de.insert("statute".to_string(), "Satzung".to_string());
        en_de.insert("regulation".to_string(), "Verordnung".to_string());
        en_de.insert("jurisdiction".to_string(), "Gerichtsbarkeit".to_string());
        en_de.insert("plaintiff".to_string(), "Kläger".to_string());
        en_de.insert("defendant".to_string(), "Beklagter".to_string());
        en_de.insert("court".to_string(), "Gericht".to_string());
        self.terminology
            .insert((LanguageCode::EN, LanguageCode::DE), en_de);
    }

    /// Analyzes legal text across languages
    pub fn analyze(
        &self,
        text: &str,
        source: LanguageCode,
        target: LanguageCode,
        jurisdiction: Option<JurisdictionInfo>,
    ) -> Result<CrossLingualAnalysis> {
        // Extract legal concepts from the text
        let concepts = self.extract_concepts(text, source, target)?;

        // Translate the text (simplified - in production would use LLM)
        let analyzed_text = self.translate_legal_text(text, source, target)?;

        // Calculate confidence based on concept coverage
        let confidence = if concepts.is_empty() {
            0.5
        } else {
            0.8 // High confidence if we found concepts
        };

        Ok(CrossLingualAnalysis {
            source_language: source,
            target_language: target,
            original_text: text.to_string(),
            analyzed_text,
            concepts,
            confidence,
            jurisdiction,
        })
    }

    /// Extracts legal concepts from text
    fn extract_concepts(
        &self,
        text: &str,
        source: LanguageCode,
        target: LanguageCode,
    ) -> Result<Vec<LegalConcept>> {
        let mut concepts = Vec::new();
        let text_lower = text.to_lowercase();

        // Get terminology for this language pair
        if let Some(term_map) = self.terminology.get(&(source, target)) {
            for (source_term, target_term) in term_map {
                if text_lower.contains(&source_term.to_lowercase()) {
                    concepts.push(LegalConcept {
                        id: source_term.clone(),
                        source_term: source_term.clone(),
                        target_term: target_term.clone(),
                        definition: format!("Legal term: {}", target_term),
                        related: Vec::new(),
                    });
                }
            }
        }

        Ok(concepts)
    }

    /// Translates legal text (simplified version)
    fn translate_legal_text(
        &self,
        text: &str,
        source: LanguageCode,
        target: LanguageCode,
    ) -> Result<String> {
        let mut result = text.to_string();

        if let Some(term_map) = self.terminology.get(&(source, target)) {
            for (source_term, target_term) in term_map {
                // Case-insensitive replacement
                let pattern =
                    regex::Regex::new(&format!(r"(?i)\b{}\b", regex::escape(source_term)))
                        .map_err(|e| anyhow!("Regex error: {}", e))?;
                result = pattern
                    .replace_all(&result, target_term.as_str())
                    .to_string();
            }
        }

        Ok(result)
    }

    /// Adds a new terminology mapping
    pub fn add_terminology(
        &mut self,
        source: LanguageCode,
        target: LanguageCode,
        source_term: String,
        target_term: String,
    ) {
        self.terminology
            .entry((source, target))
            .or_insert_with(HashMap::new)
            .insert(source_term, target_term);
    }
}

impl Default for CrossLingualAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Legal terminology translator
pub struct TerminologyTranslator {
    /// Translation database
    translations: HashMap<(LanguageCode, LanguageCode), HashMap<String, Translation>>,
}

/// Translation with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Translation {
    /// Translated term
    pub term: String,
    /// Definition in target language
    pub definition: Option<String>,
    /// Usage examples
    pub examples: Vec<String>,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f64,
    /// Jurisdiction-specific variations
    pub jurisdiction_variations: HashMap<String, String>,
}

impl TerminologyTranslator {
    /// Creates a new terminology translator
    pub fn new() -> Self {
        let mut translator = Self {
            translations: HashMap::new(),
        };
        translator.init_translations();
        translator
    }

    /// Initialize basic legal terminology translations with metadata
    fn init_translations(&mut self) {
        // English to Spanish legal terms
        let mut en_es = HashMap::new();
        en_es.insert(
            "contract".to_string(),
            Translation {
                term: "contrato".to_string(),
                definition: Some("Acuerdo legal vinculante entre dos o más partes".to_string()),
                examples: vec!["contrato de arrendamiento".to_string()],
                confidence: 1.0,
                jurisdiction_variations: HashMap::new(),
            },
        );
        en_es.insert(
            "negligence".to_string(),
            Translation {
                term: "negligencia".to_string(),
                definition: Some("Falta de cuidado razonable en una situación".to_string()),
                examples: vec!["negligencia médica".to_string()],
                confidence: 1.0,
                jurisdiction_variations: HashMap::new(),
            },
        );
        self.translations
            .insert((LanguageCode::EN, LanguageCode::ES), en_es);
    }

    /// Translates a legal term
    pub fn translate(
        &self,
        term: &str,
        source: LanguageCode,
        target: LanguageCode,
    ) -> Result<Translation> {
        self.translations
            .get(&(source, target))
            .and_then(|map| map.get(term))
            .cloned()
            .ok_or_else(|| anyhow!("Translation not found for term: {}", term))
    }

    /// Adds a new translation
    pub fn add_translation(
        &mut self,
        source: LanguageCode,
        target: LanguageCode,
        term: String,
        translation: Translation,
    ) {
        self.translations
            .entry((source, target))
            .or_insert_with(HashMap::new)
            .insert(term, translation);
    }

    /// Translates multiple terms in batch
    pub fn translate_batch(
        &self,
        terms: &[String],
        source: LanguageCode,
        target: LanguageCode,
    ) -> Vec<Result<Translation>> {
        terms
            .iter()
            .map(|term| self.translate(term, source, target))
            .collect()
    }
}

impl Default for TerminologyTranslator {
    fn default() -> Self {
        Self::new()
    }
}

/// Multilingual statute comparison result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteComparison {
    /// Source statute
    pub source_statute: Statute,
    /// Target statute
    pub target_statute: Statute,
    /// Similarity score (0.0 - 1.0)
    pub similarity: f64,
    /// Differences found
    pub differences: Vec<StatuteDifference>,
    /// Common concepts
    pub common_concepts: Vec<String>,
}

/// Statute in a specific language
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statute {
    /// Statute identifier
    pub id: String,
    /// Title
    pub title: String,
    /// Content
    pub content: String,
    /// Language
    pub language: LanguageCode,
    /// Jurisdiction
    pub jurisdiction: JurisdictionInfo,
}

/// Difference between statutes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteDifference {
    /// Type of difference
    pub diff_type: DifferenceType,
    /// Description
    pub description: String,
    /// Source excerpt
    pub source_excerpt: String,
    /// Target excerpt
    pub target_excerpt: String,
}

/// Type of statute difference
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DifferenceType {
    /// Terminology difference
    Terminology,
    /// Structural difference
    Structure,
    /// Scope difference
    Scope,
    /// Penalty difference
    Penalty,
    /// Procedural difference
    Procedure,
}

/// Multilingual statute comparator
pub struct StatuteComparator {
    analyzer: CrossLingualAnalyzer,
}

impl StatuteComparator {
    /// Creates a new statute comparator
    pub fn new() -> Self {
        Self {
            analyzer: CrossLingualAnalyzer::new(),
        }
    }

    /// Compares two statutes across languages
    pub fn compare(&self, source: &Statute, target: &Statute) -> Result<StatuteComparison> {
        // Analyze source statute in context of target language
        let analysis = self.analyzer.analyze(
            &source.content,
            source.language,
            target.language,
            Some(target.jurisdiction.clone()),
        )?;

        // Calculate similarity (simplified)
        let similarity = self.calculate_similarity(&source.content, &target.content);

        // Find differences (simplified)
        let differences = self.find_differences(source, target, &analysis);

        // Extract common concepts
        let common_concepts = analysis
            .concepts
            .iter()
            .map(|c| c.target_term.clone())
            .collect();

        Ok(StatuteComparison {
            source_statute: source.clone(),
            target_statute: target.clone(),
            similarity,
            differences,
            common_concepts,
        })
    }

    /// Calculates similarity between two statute texts
    fn calculate_similarity(&self, source: &str, target: &str) -> f64 {
        // Simplified Jaccard similarity
        let source_lower = source.to_lowercase();
        let source_words: std::collections::HashSet<_> = source_lower.split_whitespace().collect();
        let target_lower = target.to_lowercase();
        let target_words: std::collections::HashSet<_> = target_lower.split_whitespace().collect();

        let intersection = source_words.intersection(&target_words).count();
        let union = source_words.union(&target_words).count();

        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }

    /// Finds differences between statutes
    fn find_differences(
        &self,
        source: &Statute,
        target: &Statute,
        _analysis: &CrossLingualAnalysis,
    ) -> Vec<StatuteDifference> {
        let mut differences = Vec::new();

        // Check for length differences (structural)
        if source.content.len() != target.content.len() {
            differences.push(StatuteDifference {
                diff_type: DifferenceType::Structure,
                description: "Content length differs significantly".to_string(),
                source_excerpt: format!("{} characters", source.content.len()),
                target_excerpt: format!("{} characters", target.content.len()),
            });
        }

        differences
    }
}

impl Default for StatuteComparator {
    fn default() -> Self {
        Self::new()
    }
}

/// Jurisdiction-aware translator
pub struct JurisdictionTranslator {
    base_translator: TerminologyTranslator,
    jurisdiction_rules: HashMap<String, JurisdictionRules>,
}

/// Translation rules specific to a jurisdiction
#[derive(Debug, Clone)]
pub struct JurisdictionRules {
    /// Country code
    pub country: String,
    /// Term overrides
    pub term_overrides: HashMap<String, String>,
    /// Cultural adaptations
    pub cultural_adaptations: Vec<String>,
}

impl JurisdictionTranslator {
    /// Creates a new jurisdiction-aware translator
    pub fn new() -> Self {
        Self {
            base_translator: TerminologyTranslator::new(),
            jurisdiction_rules: HashMap::new(),
        }
    }

    /// Translates with jurisdiction awareness
    pub fn translate(
        &self,
        term: &str,
        source: LanguageCode,
        target: LanguageCode,
        jurisdiction: &JurisdictionInfo,
    ) -> Result<Translation> {
        // Get base translation
        let mut translation = self.base_translator.translate(term, source, target)?;

        // Apply jurisdiction-specific overrides
        if let Some(rules) = self.jurisdiction_rules.get(&jurisdiction.country) {
            if let Some(override_term) = rules.term_overrides.get(term) {
                translation.term = override_term.clone();
                translation
                    .jurisdiction_variations
                    .insert(jurisdiction.country.clone(), override_term.clone());
            }
        }

        Ok(translation)
    }

    /// Adds jurisdiction-specific rules
    pub fn add_jurisdiction_rules(&mut self, country: String, rules: JurisdictionRules) {
        self.jurisdiction_rules.insert(country, rules);
    }
}

impl Default for JurisdictionTranslator {
    fn default() -> Self {
        Self::new()
    }
}

/// Legal jargon normalizer
pub struct JargonNormalizer {
    /// Normalization rules
    rules: HashMap<LanguageCode, Vec<NormalizationRule>>,
}

/// Normalization rule for legal jargon
#[derive(Debug, Clone)]
pub struct NormalizationRule {
    /// Pattern to match (regex)
    pub pattern: String,
    /// Replacement (plain language)
    pub replacement: String,
    /// Description of the normalization
    pub description: String,
}

impl JargonNormalizer {
    /// Creates a new jargon normalizer
    pub fn new() -> Self {
        let mut normalizer = Self {
            rules: HashMap::new(),
        };
        normalizer.init_rules();
        normalizer
    }

    /// Initialize default normalization rules
    fn init_rules(&mut self) {
        let mut en_rules = Vec::new();

        // English legal jargon to plain language
        en_rules.push(NormalizationRule {
            pattern: r"\bhereinafter\b".to_string(),
            replacement: "from now on".to_string(),
            description: "Simplify temporal reference".to_string(),
        });
        en_rules.push(NormalizationRule {
            pattern: r"\bwhereas\b".to_string(),
            replacement: "because".to_string(),
            description: "Simplify causal conjunction".to_string(),
        });
        en_rules.push(NormalizationRule {
            pattern: r"\bforthwith\b".to_string(),
            replacement: "immediately".to_string(),
            description: "Simplify temporal adverb".to_string(),
        });
        en_rules.push(NormalizationRule {
            pattern: r"\binter alia\b".to_string(),
            replacement: "among other things".to_string(),
            description: "Replace Latin phrase".to_string(),
        });

        self.rules.insert(LanguageCode::EN, en_rules);
    }

    /// Normalizes legal jargon to plain language
    pub fn normalize(&self, text: &str, language: LanguageCode) -> Result<String> {
        let mut result = text.to_string();

        if let Some(rules) = self.rules.get(&language) {
            for rule in rules {
                let pattern = regex::Regex::new(&format!(r"(?i){}", rule.pattern))
                    .map_err(|e| anyhow!("Regex error: {}", e))?;
                result = pattern
                    .replace_all(&result, rule.replacement.as_str())
                    .to_string();
            }
        }

        Ok(result)
    }

    /// Adds a normalization rule
    pub fn add_rule(&mut self, language: LanguageCode, rule: NormalizationRule) {
        self.rules
            .entry(language)
            .or_insert_with(Vec::new)
            .push(rule);
    }

    /// Returns the number of normalizations that would be applied
    pub fn count_normalizations(&self, text: &str, language: LanguageCode) -> usize {
        let mut count = 0;

        if let Some(rules) = self.rules.get(&language) {
            for rule in rules {
                if let Ok(pattern) = regex::Regex::new(&format!(r"(?i){}", rule.pattern)) {
                    count += pattern.find_iter(text).count();
                }
            }
        }

        count
    }
}

impl Default for JargonNormalizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_code() {
        assert_eq!(LanguageCode::EN.name(), "English");
        assert_eq!(LanguageCode::ES.code(), "es");
    }

    #[test]
    fn test_cross_lingual_analyzer() {
        let analyzer = CrossLingualAnalyzer::new();
        let text = "This contract defines the law and statute.";

        let result = analyzer
            .analyze(text, LanguageCode::EN, LanguageCode::ES, None)
            .unwrap();

        assert_eq!(result.source_language, LanguageCode::EN);
        assert_eq!(result.target_language, LanguageCode::ES);
        assert!(!result.concepts.is_empty());
        assert!(result.confidence > 0.0);
    }

    #[test]
    fn test_terminology_translator() {
        let translator = TerminologyTranslator::new();

        let result = translator
            .translate("contract", LanguageCode::EN, LanguageCode::ES)
            .unwrap();

        assert_eq!(result.term, "contrato");
        assert!(result.confidence > 0.0);
    }

    #[test]
    fn test_statute_comparator() {
        let comparator = StatuteComparator::new();

        let source = Statute {
            id: "src-1".to_string(),
            title: "Test Statute".to_string(),
            content: "This is a test statute about contracts.".to_string(),
            language: LanguageCode::EN,
            jurisdiction: JurisdictionInfo {
                country: "US".to_string(),
                state: Some("CA".to_string()),
                language: LanguageCode::EN,
                legal_system: LegalSystemType::CommonLaw,
            },
        };

        let target = Statute {
            id: "tgt-1".to_string(),
            title: "Estatuto de Prueba".to_string(),
            content: "Este es un estatuto de prueba sobre contratos.".to_string(),
            language: LanguageCode::ES,
            jurisdiction: JurisdictionInfo {
                country: "ES".to_string(),
                state: None,
                language: LanguageCode::ES,
                legal_system: LegalSystemType::CivilLaw,
            },
        };

        let comparison = comparator.compare(&source, &target).unwrap();
        assert!(comparison.similarity >= 0.0 && comparison.similarity <= 1.0);
    }

    #[test]
    fn test_jurisdiction_translator() {
        let translator = JurisdictionTranslator::new();

        let jurisdiction = JurisdictionInfo {
            country: "US".to_string(),
            state: Some("CA".to_string()),
            language: LanguageCode::EN,
            legal_system: LegalSystemType::CommonLaw,
        };

        let result = translator
            .translate(
                "contract",
                LanguageCode::EN,
                LanguageCode::ES,
                &jurisdiction,
            )
            .unwrap();

        assert_eq!(result.term, "contrato");
    }

    #[test]
    fn test_jargon_normalizer() {
        let normalizer = JargonNormalizer::new();

        let text = "The parties hereinafter shall forthwith comply whereas the statute requires.";
        let normalized = normalizer.normalize(text, LanguageCode::EN).unwrap();

        assert!(normalized.contains("from now on"));
        assert!(normalized.contains("immediately"));
        assert!(normalized.contains("because"));
        assert!(!normalized.contains("hereinafter"));
        assert!(!normalized.contains("forthwith"));
    }

    #[test]
    fn test_jargon_normalizer_count() {
        let normalizer = JargonNormalizer::new();

        let text = "hereinafter and forthwith";
        let count = normalizer.count_normalizations(text, LanguageCode::EN);

        assert_eq!(count, 2);
    }
}
