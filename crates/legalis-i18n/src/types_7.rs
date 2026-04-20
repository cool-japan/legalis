//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::functions::I18nResult;
use super::types::RegulatoryDomain;
use super::types_3::AdoptionStatus;
use super::types_4::{RegulatoryEquivalenceLevel, StandardType};
use super::types_10::{Locale, TranslationMemoryEntry};
use super::types_11::{I18nError, SubRegionalVariation, TocEntry};
use super::types_12::LegalTopic;

/// Translation memory for caching and reusing translations.
#[derive(Debug, Default)]
pub struct TranslationMemory {
    /// Stored translation entries
    pub(super) entries: Vec<TranslationMemoryEntry>,
    /// Index for fast lookup: (source_text, source_locale, target_locale) -> entry index
    pub(super) index: HashMap<(String, String, String), Vec<usize>>,
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
        for (j, cell) in matrix[0].iter_mut().enumerate().take(len2 + 1) {
            *cell = j;
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
            if !entry.metadata.is_empty() {
                for (key, value) in &entry.metadata {
                    tmx.push_str(&format!(
                        "      <prop type=\"{}\">{}</prop>\n",
                        Self::escape_xml(key),
                        Self::escape_xml(value)
                    ));
                }
            }
            tmx.push_str(&format!(
                "      <tuv xml:lang=\"{}\">\n",
                entry.source_locale.tag()
            ));
            tmx.push_str(&format!(
                "        <seg>{}</seg>\n",
                Self::escape_xml(&entry.source_text)
            ));
            tmx.push_str("      </tuv>\n");
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
        let mut pos = 0;
        while let Some(tu_start) = tmx_content[pos..].find("<tu>") {
            let tu_start_abs = pos + tu_start;
            if let Some(tu_end) = tmx_content[tu_start_abs..].find("</tu>") {
                let tu_end_abs = tu_start_abs + tu_end + 5;
                let tu_content = &tmx_content[tu_start_abs..tu_end_abs];
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
                if tuvs.len() >= 2
                    && let (Ok(source_locale), Ok(target_locale)) =
                        (Locale::parse(&tuvs[0].0), Locale::parse(&tuvs[1].0))
                {
                    self.add_translation(
                        tuvs[0].1.clone(),
                        source_locale,
                        tuvs[1].1.clone(),
                        target_locale,
                    );
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
/// Jurisdiction detector for legal documents.
#[derive(Debug, Default)]
pub struct JurisdictionDetector {
    /// Known jurisdictions and their indicators
    pub(super) indicators: HashMap<String, Vec<String>>,
}
impl JurisdictionDetector {
    /// Creates a new jurisdiction detector.
    pub fn new() -> Self {
        Self::default()
    }
    /// Creates a jurisdiction detector with default indicators.
    pub fn with_defaults() -> Self {
        let mut detector = Self::new();
        detector.add_indicator("US", "United States");
        detector.add_indicator("US", "New York");
        detector.add_indicator("US", "Delaware");
        detector.add_indicator("US", "California");
        detector.add_indicator("US", "Supreme Court");
        detector.add_indicator("GB", "United Kingdom");
        detector.add_indicator("GB", "England and Wales");
        detector.add_indicator("GB", "English law");
        detector.add_indicator("JP", "Japan");
        detector.add_indicator("JP", "Japanese law");
        detector.add_indicator("JP", "Tokyo");
        detector.add_indicator("DE", "Germany");
        detector.add_indicator("DE", "German law");
        detector.add_indicator("DE", "BGB");
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
        scores
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(j, s)| (j.clone(), (s / 3.0).min(1.0)))
    }
}
/// Legal topic modeler for extracting topics from legal documents.
pub struct LegalTopicModeler {
    /// Predefined topics
    topics: Vec<LegalTopic>,
}
impl LegalTopicModeler {
    /// Creates a new topic modeler with default legal topics.
    pub fn new() -> Self {
        let topics = vec![
            LegalTopic::new("contract", "Contract Law")
                .add_term("contract")
                .add_term("agreement")
                .add_term("party")
                .add_term("obligation"),
            LegalTopic::new("tort", "Tort Law")
                .add_term("negligence")
                .add_term("liability")
                .add_term("damages")
                .add_term("injury"),
            LegalTopic::new("property", "Property Law")
                .add_term("property")
                .add_term("ownership")
                .add_term("title")
                .add_term("deed"),
            LegalTopic::new("criminal", "Criminal Law")
                .add_term("crime")
                .add_term("offense")
                .add_term("prosecution")
                .add_term("defendant"),
            LegalTopic::new("corporate", "Corporate Law")
                .add_term("corporation")
                .add_term("shareholder")
                .add_term("board")
                .add_term("merger"),
            LegalTopic::new("ip", "Intellectual Property")
                .add_term("patent")
                .add_term("copyright")
                .add_term("trademark")
                .add_term("license"),
        ];
        Self { topics }
    }
    /// Extracts topics from text.
    pub fn extract_topics(&self, text: &str) -> Vec<LegalTopic> {
        let text_lower = text.to_lowercase();
        let mut results = Vec::new();
        for topic in &self.topics {
            let mut matches = 0;
            for term in &topic.key_terms {
                if text_lower.contains(term) {
                    matches += 1;
                }
            }
            if matches > 0 {
                let weight = matches as f64 / topic.key_terms.len() as f64;
                results.push(LegalTopic::new(&topic.id, &topic.name).with_weight(weight));
            }
        }
        results.sort_by(|a, b| {
            b.weight
                .partial_cmp(&a.weight)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        results
    }
    /// Adds a custom topic.
    pub fn add_topic(&mut self, topic: LegalTopic) {
        self.topics.push(topic);
    }
    /// Gets all topics.
    pub fn topic_count(&self) -> usize {
        self.topics.len()
    }
}
/// Regulatory equivalence mapping between two jurisdictions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegulatoryEquivalence {
    /// Source jurisdiction.
    pub source_jurisdiction: String,
    /// Target jurisdiction.
    pub target_jurisdiction: String,
    /// Regulatory domain.
    pub domain: RegulatoryDomain,
    /// Equivalence level.
    pub level: RegulatoryEquivalenceLevel,
    /// Recognition agreement or treaty basis.
    pub basis: Option<String>,
    /// Conditions for equivalence (if conditional).
    pub conditions: Vec<String>,
    /// Last review date.
    pub last_review: Option<String>,
}
impl RegulatoryEquivalence {
    /// Creates a new regulatory equivalence mapping.
    pub fn new(
        source: impl Into<String>,
        target: impl Into<String>,
        domain: RegulatoryDomain,
        level: RegulatoryEquivalenceLevel,
    ) -> Self {
        Self {
            source_jurisdiction: source.into(),
            target_jurisdiction: target.into(),
            domain,
            level,
            basis: None,
            conditions: Vec::new(),
            last_review: None,
        }
    }
    /// Sets the legal basis for equivalence.
    pub fn with_basis(mut self, basis: impl Into<String>) -> Self {
        self.basis = Some(basis.into());
        self
    }
    /// Adds a condition for equivalence.
    pub fn add_condition(mut self, condition: impl Into<String>) -> Self {
        self.conditions.push(condition.into());
        self
    }
    /// Sets the last review date.
    pub fn with_review_date(mut self, date: impl Into<String>) -> Self {
        self.last_review = Some(date.into());
        self
    }
    /// Checks if this is a bidirectional equivalence.
    pub fn is_mutual(&self) -> bool {
        matches!(self.level, RegulatoryEquivalenceLevel::Full)
    }
}
/// Table of contents generator.
#[derive(Debug, Clone)]
pub struct TableOfContents {
    pub(super) entries: Vec<TocEntry>,
    pub(super) locale: Locale,
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
/// Standard adoption record for a jurisdiction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StandardAdoption {
    /// Standard identifier (e.g., "ISO 27001", "RFC 2616").
    pub standard_id: String,
    /// Standard type.
    pub standard_type: StandardType,
    /// Jurisdiction (country code).
    pub jurisdiction: String,
    /// Adoption status.
    pub status: AdoptionStatus,
    /// Date of adoption (YYYY-MM-DD).
    pub adoption_date: Option<String>,
    /// National law reference implementing the standard.
    pub implementing_law: Option<String>,
    /// Deviations or modifications from the standard.
    pub deviations: Vec<String>,
}
impl StandardAdoption {
    /// Creates a new standard adoption record.
    pub fn new(
        standard_id: impl Into<String>,
        standard_type: StandardType,
        jurisdiction: impl Into<String>,
        status: AdoptionStatus,
    ) -> Self {
        Self {
            standard_id: standard_id.into(),
            standard_type,
            jurisdiction: jurisdiction.into(),
            status,
            adoption_date: None,
            implementing_law: None,
            deviations: Vec::new(),
        }
    }
    /// Sets the adoption date.
    pub fn with_date(mut self, date: impl Into<String>) -> Self {
        self.adoption_date = Some(date.into());
        self
    }
    /// Sets the implementing law.
    pub fn with_law(mut self, law: impl Into<String>) -> Self {
        self.implementing_law = Some(law.into());
        self
    }
    /// Adds a deviation from the standard.
    pub fn add_deviation(mut self, deviation: impl Into<String>) -> Self {
        self.deviations.push(deviation.into());
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
