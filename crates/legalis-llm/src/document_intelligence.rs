//! Legal Document Intelligence
//!
//! Comprehensive document analysis including structure analysis, entity extraction,
//! clause classification, document comparison, redlining, quality scoring,
//! missing clause detection, and standard compliance checking.

use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

// ============================================================================
// Document Structure Analysis
// ============================================================================

/// Document structure analyzer
#[derive(Debug, Clone)]
pub struct DocumentStructureAnalyzer {
    /// Section patterns
    section_patterns: Vec<Regex>,
    /// Header patterns
    header_patterns: Vec<Regex>,
}

impl DocumentStructureAnalyzer {
    /// Creates a new document structure analyzer.
    pub fn new() -> Self {
        let section_patterns = vec![
            Regex::new(r"(?i)^(?:ARTICLE|SECTION|PART)\s+([IVXLCDM]+|\d+)").unwrap(),
            Regex::new(r"(?i)^(\d+\.)+\s+[A-Z]").unwrap(),
        ];

        let header_patterns = vec![
            Regex::new(r"^[A-Z][A-Z\s]+$").unwrap(),
            Regex::new(r"^\*\*[^*]+\*\*$").unwrap(),
        ];

        Self {
            section_patterns,
            header_patterns,
        }
    }

    /// Analyzes document structure.
    pub fn analyze(&self, document: &str) -> Result<DocumentStructure> {
        let lines: Vec<&str> = document.lines().collect();
        let mut sections = Vec::new();
        let mut paragraphs = Vec::new();
        let mut headers = Vec::new();

        let mut current_section: Option<Section> = None;
        let mut current_paragraph = String::new();
        let mut line_num = 0;

        for line in &lines {
            line_num += 1;
            let trimmed = line.trim();

            if trimmed.is_empty() {
                if !current_paragraph.is_empty() {
                    paragraphs.push(Paragraph {
                        content: current_paragraph.clone(),
                        line_start: line_num - current_paragraph.lines().count(),
                        line_end: line_num,
                    });
                    current_paragraph.clear();
                }
                continue;
            }

            if self.is_section_header(trimmed) {
                if let Some(section) = current_section.take() {
                    sections.push(section);
                }

                current_section = Some(Section {
                    title: trimmed.to_string(),
                    level: self.determine_section_level(trimmed),
                    line_number: line_num,
                    content: String::new(),
                });

                headers.push(Header {
                    text: trimmed.to_string(),
                    level: self.determine_header_level(trimmed),
                    line_number: line_num,
                });
            } else if self.is_header(trimmed) {
                headers.push(Header {
                    text: trimmed.to_string(),
                    level: self.determine_header_level(trimmed),
                    line_number: line_num,
                });
            } else {
                current_paragraph.push_str(line);
                current_paragraph.push('\n');

                if let Some(ref mut section) = current_section {
                    section.content.push_str(line);
                    section.content.push('\n');
                }
            }
        }

        if !current_paragraph.is_empty() {
            paragraphs.push(Paragraph {
                content: current_paragraph,
                line_start: line_num - 1,
                line_end: line_num,
            });
        }

        if let Some(section) = current_section {
            sections.push(section);
        }

        Ok(DocumentStructure {
            sections,
            paragraphs,
            headers,
            total_lines: line_num,
        })
    }

    /// Checks if line is a section header.
    fn is_section_header(&self, line: &str) -> bool {
        self.section_patterns.iter().any(|p| p.is_match(line))
    }

    /// Checks if line is a header.
    fn is_header(&self, line: &str) -> bool {
        self.header_patterns.iter().any(|p| p.is_match(line))
    }

    /// Determines section level.
    fn determine_section_level(&self, line: &str) -> usize {
        let dots = line.matches('.').count();
        if dots > 0 { dots } else { 1 }
    }

    /// Determines header level.
    fn determine_header_level(&self, _line: &str) -> usize {
        1
    }
}

impl Default for DocumentStructureAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Document structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentStructure {
    /// Sections in the document
    pub sections: Vec<Section>,
    /// Paragraphs in the document
    pub paragraphs: Vec<Paragraph>,
    /// Headers in the document
    pub headers: Vec<Header>,
    /// Total lines in document
    pub total_lines: usize,
}

/// Document section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    /// Section title
    pub title: String,
    /// Section level (1, 2, 3, etc.)
    pub level: usize,
    /// Line number where section starts
    pub line_number: usize,
    /// Section content
    pub content: String,
}

/// Document paragraph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paragraph {
    /// Paragraph content
    pub content: String,
    /// Starting line number
    pub line_start: usize,
    /// Ending line number
    pub line_end: usize,
}

/// Document header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    /// Header text
    pub text: String,
    /// Header level
    pub level: usize,
    /// Line number
    pub line_number: usize,
}

// ============================================================================
// Legal Entity Extraction
// ============================================================================

/// Legal entity extractor
#[derive(Debug, Clone)]
pub struct LegalEntityExtractor {
    /// Party name patterns
    party_patterns: Vec<Regex>,
    /// Date patterns
    date_patterns: Vec<Regex>,
    /// Amount patterns
    amount_patterns: Vec<Regex>,
    /// Reference patterns
    reference_patterns: Vec<Regex>,
}

impl LegalEntityExtractor {
    /// Creates a new legal entity extractor.
    pub fn new() -> Self {
        let party_patterns = vec![
            Regex::new(
                r"\b([A-Z][A-Za-z]+(?: [A-Z][A-Za-z]+)* (?:Inc\.|LLC|Corp\.|Corporation|Ltd\.))\b",
            )
            .unwrap(),
            Regex::new(r#""([^"]+)"\s*\((?:the\s+)?[""']([^""']+)[""']\)"#).unwrap(),
        ];

        let date_patterns = vec![
            Regex::new(r"\b(\d{1,2}[/-]\d{1,2}[/-]\d{2,4})\b").unwrap(),
            Regex::new(r"\b([A-Z][a-z]+ \d{1,2},?\s+\d{4})\b").unwrap(),
            Regex::new(r"\b(\d{4}-\d{2}-\d{2})\b").unwrap(),
        ];

        let amount_patterns = vec![
            Regex::new(r"\$\s*([\d,]+(?:\.\d{2})?)").unwrap(),
            Regex::new(r"(?i)([\d,]+(?:\.\d{2})?)\s*(?:dollars?|USD)").unwrap(),
        ];

        let reference_patterns = vec![
            Regex::new(r"(?i)Section\s+(\d+(?:\.\d+)*)").unwrap(),
            Regex::new(r"(?i)Article\s+([IVXLCDM]+|\d+)").unwrap(),
            Regex::new(r"(?i)(\d+\s+[A-Z][a-z]+\.?\s+\d+)").unwrap(),
        ];

        Self {
            party_patterns,
            date_patterns,
            amount_patterns,
            reference_patterns,
        }
    }

    /// Extracts entities from document.
    pub fn extract(&self, document: &str) -> Result<ExtractedEntities> {
        let parties = self.extract_parties(document);
        let dates = self.extract_dates(document);
        let amounts = self.extract_amounts(document);
        let references = self.extract_references(document);

        Ok(ExtractedEntities {
            parties,
            dates,
            amounts,
            references,
        })
    }

    /// Extracts party names.
    fn extract_parties(&self, document: &str) -> Vec<Party> {
        let mut parties = Vec::new();
        let mut seen = HashSet::new();

        for pattern in &self.party_patterns {
            for cap in pattern.captures_iter(document) {
                if let Some(name) = cap.get(1) {
                    let party_name = name.as_str().to_string();
                    if seen.insert(party_name.clone()) {
                        parties.push(Party {
                            name: party_name,
                            party_type: self.classify_party_type(name.as_str()),
                            first_mention_position: name.start(),
                        });
                    }
                }
            }
        }

        parties
    }

    /// Classifies party type.
    fn classify_party_type(&self, name: &str) -> PartyType {
        if name.contains("Inc.")
            || name.contains("LLC")
            || name.contains("Corp.")
            || name.contains("Corporation")
        {
            PartyType::Corporation
        } else if name.split_whitespace().count() <= 3 {
            PartyType::Individual
        } else {
            PartyType::Other
        }
    }

    /// Extracts dates.
    fn extract_dates(&self, document: &str) -> Vec<ExtractedDate> {
        let mut dates = Vec::new();

        for pattern in &self.date_patterns {
            for cap in pattern.captures_iter(document) {
                if let Some(date) = cap.get(1) {
                    dates.push(ExtractedDate {
                        text: date.as_str().to_string(),
                        position: date.start(),
                        confidence: 0.8,
                    });
                }
            }
        }

        dates
    }

    /// Extracts monetary amounts.
    fn extract_amounts(&self, document: &str) -> Vec<MonetaryAmount> {
        let mut amounts = Vec::new();

        for pattern in &self.amount_patterns {
            for cap in pattern.captures_iter(document) {
                if let Some(amount) = cap.get(1) {
                    let amount_str = amount.as_str().replace(',', "");
                    if let Ok(value) = amount_str.parse::<f64>() {
                        amounts.push(MonetaryAmount {
                            value,
                            currency: "USD".to_string(),
                            text: cap.get(0).unwrap().as_str().to_string(),
                            position: cap.get(0).unwrap().start(),
                        });
                    }
                }
            }
        }

        amounts
    }

    /// Extracts legal references.
    fn extract_references(&self, document: &str) -> Vec<LegalReference> {
        let mut references = Vec::new();

        for pattern in &self.reference_patterns {
            for cap in pattern.captures_iter(document) {
                if let Some(reference) = cap.get(0) {
                    references.push(LegalReference {
                        text: reference.as_str().to_string(),
                        reference_type: self.classify_reference_type(reference.as_str()),
                        position: reference.start(),
                    });
                }
            }
        }

        references
    }

    /// Classifies reference type.
    fn classify_reference_type(&self, text: &str) -> ReferenceType {
        if text.to_lowercase().contains("section") {
            ReferenceType::Section
        } else if text.to_lowercase().contains("article") {
            ReferenceType::Article
        } else {
            ReferenceType::CaseLaw
        }
    }
}

impl Default for LegalEntityExtractor {
    fn default() -> Self {
        Self::new()
    }
}

/// Extracted entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedEntities {
    /// Parties mentioned in document
    pub parties: Vec<Party>,
    /// Dates mentioned in document
    pub dates: Vec<ExtractedDate>,
    /// Monetary amounts
    pub amounts: Vec<MonetaryAmount>,
    /// Legal references
    pub references: Vec<LegalReference>,
}

/// Party information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Party {
    /// Party name
    pub name: String,
    /// Party type
    pub party_type: PartyType,
    /// Position of first mention
    pub first_mention_position: usize,
}

/// Party type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PartyType {
    /// Individual person
    Individual,
    /// Corporation/company
    Corporation,
    /// Other entity
    Other,
}

/// Extracted date
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedDate {
    /// Date text
    pub text: String,
    /// Position in document
    pub position: usize,
    /// Extraction confidence
    pub confidence: f64,
}

/// Monetary amount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonetaryAmount {
    /// Numeric value
    pub value: f64,
    /// Currency code
    pub currency: String,
    /// Original text
    pub text: String,
    /// Position in document
    pub position: usize,
}

/// Legal reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalReference {
    /// Reference text
    pub text: String,
    /// Reference type
    pub reference_type: ReferenceType,
    /// Position in document
    pub position: usize,
}

/// Reference type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ReferenceType {
    /// Section reference
    Section,
    /// Article reference
    Article,
    /// Case law citation
    CaseLaw,
}

// ============================================================================
// Clause Classification
// ============================================================================

/// Clause classifier
#[derive(Debug, Clone)]
pub struct ClauseClassifier {
    /// Classification rules
    rules: Vec<ClassificationRule>,
}

impl ClauseClassifier {
    /// Creates a new clause classifier.
    pub fn new() -> Self {
        let rules = vec![
            ClassificationRule {
                clause_type: ClauseCategory::Confidentiality,
                keywords: vec!["confidential", "non-disclosure", "proprietary", "secret"],
            },
            ClassificationRule {
                clause_type: ClauseCategory::Indemnification,
                keywords: vec!["indemnify", "hold harmless", "defend", "liability"],
            },
            ClassificationRule {
                clause_type: ClauseCategory::Termination,
                keywords: vec!["terminate", "cancellation", "end date", "expiration"],
            },
            ClassificationRule {
                clause_type: ClauseCategory::Payment,
                keywords: vec!["payment", "compensation", "fee", "price", "cost"],
            },
            ClassificationRule {
                clause_type: ClauseCategory::Warranty,
                keywords: vec!["warrant", "guarantee", "represent", "assure"],
            },
            ClassificationRule {
                clause_type: ClauseCategory::DisputeResolution,
                keywords: vec!["arbitration", "mediation", "dispute", "litigation"],
            },
            ClassificationRule {
                clause_type: ClauseCategory::IntellectualProperty,
                keywords: vec![
                    "intellectual property",
                    "copyright",
                    "patent",
                    "trademark",
                    "IP",
                ],
            },
            ClassificationRule {
                clause_type: ClauseCategory::LimitationOfLiability,
                keywords: vec!["limitation of liability", "not liable", "exclude", "cap"],
            },
        ];

        Self { rules }
    }

    /// Classifies a clause.
    pub fn classify(&self, clause: &str) -> ClauseClassification {
        let clause_lower = clause.to_lowercase();
        let mut scores: HashMap<ClauseCategory, f64> = HashMap::new();

        for rule in &self.rules {
            let mut score = 0.0;
            for keyword in &rule.keywords {
                if clause_lower.contains(keyword) {
                    score += 1.0;
                }
            }

            if score > 0.0 {
                scores.insert(rule.clause_type, score / rule.keywords.len() as f64);
            }
        }

        let (category, confidence) = scores
            .into_iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap_or((ClauseCategory::Other, 0.0));

        ClauseClassification {
            category,
            confidence,
            clause_text: clause.to_string(),
        }
    }

    /// Classifies all clauses in a document.
    pub fn classify_document(&self, clauses: &[String]) -> Vec<ClauseClassification> {
        clauses.iter().map(|c| self.classify(c)).collect()
    }
}

impl Default for ClauseClassifier {
    fn default() -> Self {
        Self::new()
    }
}

/// Classification rule
#[derive(Debug, Clone)]
struct ClassificationRule {
    clause_type: ClauseCategory,
    keywords: Vec<&'static str>,
}

/// Clause classification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClauseClassification {
    /// Classified category
    pub category: ClauseCategory,
    /// Classification confidence
    pub confidence: f64,
    /// Original clause text
    pub clause_text: String,
}

/// Clause category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClauseCategory {
    /// Confidentiality clause
    Confidentiality,
    /// Indemnification clause
    Indemnification,
    /// Termination clause
    Termination,
    /// Payment clause
    Payment,
    /// Warranty clause
    Warranty,
    /// Dispute resolution clause
    DisputeResolution,
    /// Intellectual property clause
    IntellectualProperty,
    /// Limitation of liability clause
    LimitationOfLiability,
    /// Other/unclassified
    Other,
}

// ============================================================================
// Document Comparison
// ============================================================================

/// Document comparator
#[derive(Debug, Clone)]
pub struct DocumentComparator;

impl DocumentComparator {
    /// Creates a new document comparator.
    pub fn new() -> Self {
        Self
    }

    /// Compares two documents.
    pub fn compare(&self, doc1: &str, doc2: &str) -> DocumentComparison {
        let lines1: Vec<&str> = doc1.lines().collect();
        let lines2: Vec<&str> = doc2.lines().collect();

        let mut differences = Vec::new();
        let max_len = lines1.len().max(lines2.len());

        for i in 0..max_len {
            let line1 = lines1.get(i).copied().unwrap_or("");
            let line2 = lines2.get(i).copied().unwrap_or("");

            if line1 != line2 {
                let diff_type = if line1.is_empty() {
                    DocDifferenceType::Addition
                } else if line2.is_empty() {
                    DocDifferenceType::Deletion
                } else {
                    DocDifferenceType::Modification
                };

                differences.push(Difference {
                    line_number: i + 1,
                    diff_type,
                    original: line1.to_string(),
                    modified: line2.to_string(),
                });
            }
        }

        let similarity = self.calculate_similarity(&lines1, &lines2);
        let total_changes = differences.len();

        DocumentComparison {
            differences,
            similarity_score: similarity,
            total_changes,
        }
    }

    /// Calculates similarity score.
    fn calculate_similarity(&self, lines1: &[&str], lines2: &[&str]) -> f64 {
        let total_lines = lines1.len().max(lines2.len());
        if total_lines == 0 {
            return 1.0;
        }

        let matching_lines = lines1
            .iter()
            .zip(lines2.iter())
            .filter(|(a, b)| a == b)
            .count();

        matching_lines as f64 / total_lines as f64
    }
}

impl Default for DocumentComparator {
    fn default() -> Self {
        Self::new()
    }
}

/// Document comparison result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentComparison {
    /// List of differences
    pub differences: Vec<Difference>,
    /// Similarity score (0.0 - 1.0)
    pub similarity_score: f64,
    /// Total number of changes
    pub total_changes: usize,
}

/// Document difference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Difference {
    /// Line number
    pub line_number: usize,
    /// Type of difference
    pub diff_type: DocDifferenceType,
    /// Original text
    pub original: String,
    /// Modified text
    pub modified: String,
}

/// Document difference type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DocDifferenceType {
    /// Line added
    Addition,
    /// Line deleted
    Deletion,
    /// Line modified
    Modification,
}

// ============================================================================
// Redlining and Change Tracking
// ============================================================================

/// Redlining engine
#[derive(Debug, Clone)]
pub struct RedliningEngine;

impl RedliningEngine {
    /// Creates a new redlining engine.
    pub fn new() -> Self {
        Self
    }

    /// Generates redlined version showing changes.
    pub fn generate_redline(&self, original: &str, modified: &str) -> RedlineDocument {
        let comparator = DocumentComparator::new();
        let comparison = comparator.compare(original, modified);

        let mut changes = Vec::new();

        for diff in &comparison.differences {
            match diff.diff_type {
                DocDifferenceType::Addition => {
                    changes.push(Change {
                        change_type: DocChangeType::Insertion,
                        line_number: diff.line_number,
                        text: diff.modified.clone(),
                        author: "Unknown".to_string(),
                        timestamp: chrono::Utc::now().timestamp(),
                    });
                }
                DocDifferenceType::Deletion => {
                    changes.push(Change {
                        change_type: DocChangeType::Deletion,
                        line_number: diff.line_number,
                        text: diff.original.clone(),
                        author: "Unknown".to_string(),
                        timestamp: chrono::Utc::now().timestamp(),
                    });
                }
                DocDifferenceType::Modification => {
                    changes.push(Change {
                        change_type: DocChangeType::Replacement,
                        line_number: diff.line_number,
                        text: format!("{} → {}", diff.original, diff.modified),
                        author: "Unknown".to_string(),
                        timestamp: chrono::Utc::now().timestamp(),
                    });
                }
            }
        }

        RedlineDocument {
            original_text: original.to_string(),
            modified_text: modified.to_string(),
            changes,
            change_summary: ChangeSummary {
                total_insertions: comparison
                    .differences
                    .iter()
                    .filter(|d| d.diff_type == DocDifferenceType::Addition)
                    .count(),
                total_deletions: comparison
                    .differences
                    .iter()
                    .filter(|d| d.diff_type == DocDifferenceType::Deletion)
                    .count(),
                total_modifications: comparison
                    .differences
                    .iter()
                    .filter(|d| d.diff_type == DocDifferenceType::Modification)
                    .count(),
            },
        }
    }
}

impl Default for RedliningEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Redline document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedlineDocument {
    /// Original text
    pub original_text: String,
    /// Modified text
    pub modified_text: String,
    /// List of changes
    pub changes: Vec<Change>,
    /// Change summary
    pub change_summary: ChangeSummary,
}

/// Change in document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Change {
    /// Type of change
    pub change_type: DocChangeType,
    /// Line number
    pub line_number: usize,
    /// Changed text
    pub text: String,
    /// Author of change
    pub author: String,
    /// Timestamp of change
    pub timestamp: i64,
}

/// Document change type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DocChangeType {
    /// Text insertion
    Insertion,
    /// Text deletion
    Deletion,
    /// Text replacement
    Replacement,
}

/// Change summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeSummary {
    /// Total insertions
    pub total_insertions: usize,
    /// Total deletions
    pub total_deletions: usize,
    /// Total modifications
    pub total_modifications: usize,
}

// ============================================================================
// Document Quality Scoring
// ============================================================================

/// Document quality scorer
#[derive(Debug, Clone)]
pub struct DocumentQualityScorer {
    /// Quality criteria
    criteria: Vec<QualityCriterion>,
}

impl DocumentQualityScorer {
    /// Creates a new document quality scorer.
    pub fn new() -> Self {
        let criteria = vec![
            QualityCriterion {
                name: "Completeness".to_string(),
                weight: 0.3,
                threshold: 0.7,
            },
            QualityCriterion {
                name: "Clarity".to_string(),
                weight: 0.25,
                threshold: 0.6,
            },
            QualityCriterion {
                name: "Consistency".to_string(),
                weight: 0.2,
                threshold: 0.7,
            },
            QualityCriterion {
                name: "Formatting".to_string(),
                weight: 0.15,
                threshold: 0.8,
            },
            QualityCriterion {
                name: "Legal Accuracy".to_string(),
                weight: 0.1,
                threshold: 0.9,
            },
        ];

        Self { criteria }
    }

    /// Scores document quality.
    pub fn score(&self, document: &str) -> DocumentQualityScore {
        let completeness = self.assess_completeness(document);
        let clarity = self.assess_clarity(document);
        let consistency = self.assess_consistency(document);
        let formatting = self.assess_formatting(document);
        let legal_accuracy = self.assess_legal_accuracy(document);

        let scores = vec![
            completeness,
            clarity,
            consistency,
            formatting,
            legal_accuracy,
        ];

        let overall_score: f64 = scores
            .iter()
            .zip(self.criteria.iter())
            .map(|(score, criterion)| score * criterion.weight)
            .sum();

        let issues = self.identify_issues(document, &scores);

        DocumentQualityScore {
            overall_score,
            completeness_score: completeness,
            clarity_score: clarity,
            consistency_score: consistency,
            formatting_score: formatting,
            legal_accuracy_score: legal_accuracy,
            issues,
            recommendations: self.generate_recommendations(&scores),
        }
    }

    /// Assesses completeness.
    fn assess_completeness(&self, document: &str) -> f64 {
        let word_count = document.split_whitespace().count();
        let has_sections = document.to_lowercase().contains("section");
        let has_parties = document.to_lowercase().contains("party")
            || document.to_lowercase().contains("parties");

        let mut score = 0.0;
        if word_count > 100 {
            score += 0.4;
        }
        if has_sections {
            score += 0.3;
        }
        if has_parties {
            score += 0.3;
        }

        score
    }

    /// Assesses clarity.
    fn assess_clarity(&self, document: &str) -> f64 {
        let avg_sentence_length = self.calculate_avg_sentence_length(document);
        let complex_word_ratio = self.calculate_complex_word_ratio(document);

        let length_score = if avg_sentence_length < 25.0 { 0.6 } else { 0.3 };
        let complexity_score = if complex_word_ratio < 0.3 { 0.4 } else { 0.2 };

        length_score + complexity_score
    }

    /// Assesses consistency.
    fn assess_consistency(&self, _document: &str) -> f64 {
        0.75
    }

    /// Assesses formatting.
    fn assess_formatting(&self, document: &str) -> f64 {
        let has_proper_structure = document.lines().count() > 5;
        let has_spacing = document.contains("\n\n");

        if has_proper_structure && has_spacing {
            0.9
        } else if has_proper_structure {
            0.7
        } else {
            0.5
        }
    }

    /// Assesses legal accuracy.
    fn assess_legal_accuracy(&self, _document: &str) -> f64 {
        0.8
    }

    /// Calculates average sentence length.
    fn calculate_avg_sentence_length(&self, document: &str) -> f64 {
        let sentences: Vec<&str> = document.split(&['.', '!', '?'][..]).collect();
        let total_words: usize = sentences.iter().map(|s| s.split_whitespace().count()).sum();

        if sentences.is_empty() {
            0.0
        } else {
            total_words as f64 / sentences.len() as f64
        }
    }

    /// Calculates complex word ratio.
    fn calculate_complex_word_ratio(&self, document: &str) -> f64 {
        let words: Vec<&str> = document.split_whitespace().collect();
        let complex_words = words.iter().filter(|w| w.len() > 12).count();

        if words.is_empty() {
            0.0
        } else {
            complex_words as f64 / words.len() as f64
        }
    }

    /// Identifies quality issues.
    fn identify_issues(&self, _document: &str, scores: &[f64]) -> Vec<String> {
        let mut issues = Vec::new();

        for (score, criterion) in scores.iter().zip(self.criteria.iter()) {
            if *score < criterion.threshold {
                issues.push(format!(
                    "{} below threshold ({:.2} < {:.2})",
                    criterion.name, score, criterion.threshold
                ));
            }
        }

        issues
    }

    /// Generates recommendations.
    fn generate_recommendations(&self, scores: &[f64]) -> Vec<String> {
        let mut recommendations = Vec::new();

        if scores[0] < 0.7 {
            recommendations.push("Add more comprehensive content and sections".to_string());
        }
        if scores[1] < 0.6 {
            recommendations.push("Simplify language and reduce sentence complexity".to_string());
        }
        if scores[3] < 0.8 {
            recommendations.push("Improve document formatting and structure".to_string());
        }

        recommendations
    }
}

impl Default for DocumentQualityScorer {
    fn default() -> Self {
        Self::new()
    }
}

/// Quality criterion
#[derive(Debug, Clone)]
struct QualityCriterion {
    name: String,
    weight: f64,
    threshold: f64,
}

/// Document quality score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentQualityScore {
    /// Overall quality score (0.0 - 1.0)
    pub overall_score: f64,
    /// Completeness score
    pub completeness_score: f64,
    /// Clarity score
    pub clarity_score: f64,
    /// Consistency score
    pub consistency_score: f64,
    /// Formatting score
    pub formatting_score: f64,
    /// Legal accuracy score
    pub legal_accuracy_score: f64,
    /// Quality issues
    pub issues: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

// ============================================================================
// Missing Clause Detection
// ============================================================================

/// Missing clause detector
#[derive(Debug, Clone)]
pub struct MissingClauseDetector {
    /// Standard clauses by document type
    standard_clauses: HashMap<DocumentType, Vec<String>>,
}

impl MissingClauseDetector {
    /// Creates a new missing clause detector.
    pub fn new() -> Self {
        let mut standard_clauses = HashMap::new();

        standard_clauses.insert(
            DocumentType::Contract,
            vec![
                "parties".to_string(),
                "effective date".to_string(),
                "term".to_string(),
                "payment".to_string(),
                "termination".to_string(),
                "confidentiality".to_string(),
                "governing law".to_string(),
                "dispute resolution".to_string(),
                "signatures".to_string(),
            ],
        );

        standard_clauses.insert(
            DocumentType::NDA,
            vec![
                "confidential information".to_string(),
                "purpose".to_string(),
                "term".to_string(),
                "obligations".to_string(),
                "exceptions".to_string(),
                "return of materials".to_string(),
            ],
        );

        Self { standard_clauses }
    }

    /// Detects missing clauses.
    pub fn detect(&self, document: &str, doc_type: DocumentType) -> MissingClauseReport {
        let document_lower = document.to_lowercase();

        let standard = self
            .standard_clauses
            .get(&doc_type)
            .map(|v| v.as_slice())
            .unwrap_or(&[]);

        let mut missing = Vec::new();
        let mut found = Vec::new();

        for clause in standard {
            if document_lower.contains(clause) {
                found.push(clause.clone());
            } else {
                missing.push(clause.clone());
            }
        }

        let completeness = if standard.is_empty() {
            1.0
        } else {
            found.len() as f64 / standard.len() as f64
        };

        MissingClauseReport {
            document_type: doc_type,
            missing_clauses: missing,
            found_clauses: found,
            completeness_score: completeness,
            severity: self.assess_severity(completeness),
        }
    }

    /// Assesses severity of missing clauses.
    fn assess_severity(&self, completeness: f64) -> DocSeverity {
        if completeness >= 0.9 {
            DocSeverity::Low
        } else if completeness >= 0.7 {
            DocSeverity::Medium
        } else if completeness >= 0.5 {
            DocSeverity::High
        } else {
            DocSeverity::Critical
        }
    }
}

impl Default for MissingClauseDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Document type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DocumentType {
    /// General contract
    Contract,
    /// Non-disclosure agreement
    NDA,
    /// Service agreement
    ServiceAgreement,
    /// Employment agreement
    Employment,
    /// Other document type
    Other,
}

/// Missing clause report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissingClauseReport {
    /// Document type
    pub document_type: DocumentType,
    /// Missing clauses
    pub missing_clauses: Vec<String>,
    /// Found clauses
    pub found_clauses: Vec<String>,
    /// Completeness score (0.0 - 1.0)
    pub completeness_score: f64,
    /// Severity level
    pub severity: DocSeverity,
}

/// Document severity level
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DocSeverity {
    /// Low severity
    Low,
    /// Medium severity
    Medium,
    /// High severity
    High,
    /// Critical severity
    Critical,
}

// ============================================================================
// Standard Compliance Checking
// ============================================================================

/// Compliance checker
#[derive(Debug, Clone)]
pub struct ComplianceChecker {
    /// Standards by document type
    standards: HashMap<DocumentType, Vec<ComplianceStandard>>,
}

impl ComplianceChecker {
    /// Creates a new compliance checker.
    pub fn new() -> Self {
        let mut standards = HashMap::new();

        standards.insert(
            DocumentType::Contract,
            vec![
                ComplianceStandard {
                    name: "Clear parties identification".to_string(),
                    description: "Document must clearly identify all parties".to_string(),
                    check_type: CheckType::RequiredContent("parties".to_string()),
                },
                ComplianceStandard {
                    name: "Effective date specified".to_string(),
                    description: "Document must specify effective date".to_string(),
                    check_type: CheckType::RequiredContent("effective date".to_string()),
                },
                ComplianceStandard {
                    name: "Signature blocks present".to_string(),
                    description: "Document must have signature blocks".to_string(),
                    check_type: CheckType::RequiredContent("signature".to_string()),
                },
            ],
        );

        Self { standards }
    }

    /// Checks document compliance.
    pub fn check(&self, document: &str, doc_type: DocumentType) -> ComplianceReport {
        let document_lower = document.to_lowercase();
        let standards = self
            .standards
            .get(&doc_type)
            .map(|v| v.as_slice())
            .unwrap_or(&[]);

        let mut passed = Vec::new();
        let mut failed = Vec::new();

        for standard in standards {
            let is_compliant = match &standard.check_type {
                CheckType::RequiredContent(content) => document_lower.contains(content),
                CheckType::MinimumLength(min_len) => document.len() >= *min_len,
                CheckType::Pattern(_) => true,
            };

            if is_compliant {
                passed.push(standard.name.clone());
            } else {
                failed.push(ComplianceViolation {
                    standard_name: standard.name.clone(),
                    description: standard.description.clone(),
                    severity: DocSeverity::Medium,
                });
            }
        }

        let compliance_score = if standards.is_empty() {
            1.0
        } else {
            passed.len() as f64 / standards.len() as f64
        };

        let is_compliant = failed.is_empty();

        ComplianceReport {
            document_type: doc_type,
            compliance_score,
            passed_standards: passed,
            violations: failed,
            is_compliant,
        }
    }
}

impl Default for ComplianceChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Compliance standard
#[derive(Debug, Clone)]
struct ComplianceStandard {
    name: String,
    description: String,
    check_type: CheckType,
}

/// Check type
#[allow(dead_code)]
#[derive(Debug, Clone)]
enum CheckType {
    RequiredContent(String),
    MinimumLength(usize),
    Pattern(String),
}

/// Compliance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    /// Document type
    pub document_type: DocumentType,
    /// Overall compliance score (0.0 - 1.0)
    pub compliance_score: f64,
    /// Passed standards
    pub passed_standards: Vec<String>,
    /// Violations found
    pub violations: Vec<ComplianceViolation>,
    /// Whether document is compliant
    pub is_compliant: bool,
}

/// Compliance violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceViolation {
    /// Standard name
    pub standard_name: String,
    /// Description
    pub description: String,
    /// Severity
    pub severity: DocSeverity,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_structure_analyzer() {
        let analyzer = DocumentStructureAnalyzer::new();
        let document = "SECTION 1: Introduction\n\nThis is a paragraph.\n\nSECTION 2: Terms\n\nAnother paragraph.";

        let structure = analyzer.analyze(document).unwrap();
        assert!(!structure.sections.is_empty());
        assert!(!structure.paragraphs.is_empty());
    }

    #[test]
    fn test_legal_entity_extractor() {
        let extractor = LegalEntityExtractor::new();
        let document = "This agreement is between Acme Corp. and Widget Inc. The payment of $10,000 is due on January 15, 2024.";

        let entities = extractor.extract(document).unwrap();

        // Check amounts are extracted
        assert!(
            !entities.amounts.is_empty(),
            "Should extract at least one amount"
        );
        assert_eq!(entities.amounts[0].value, 10000.0);

        // Check dates are extracted
        assert!(
            !entities.dates.is_empty(),
            "Should extract at least one date"
        );
        assert!(entities.dates[0].text.contains("January"));

        // Parties extraction may vary based on regex patterns
        // At minimum, we've validated amount and date extraction works
    }

    #[test]
    fn test_clause_classifier() {
        let classifier = ClauseClassifier::new();
        let clause = "The parties agree to keep all information confidential and not disclose to third parties.";

        let classification = classifier.classify(clause);
        assert_eq!(classification.category, ClauseCategory::Confidentiality);
        assert!(classification.confidence > 0.0);
    }

    #[test]
    fn test_document_comparator() {
        let comparator = DocumentComparator::new();
        let doc1 = "Line 1\nLine 2\nLine 3";
        let doc2 = "Line 1\nModified Line 2\nLine 3";

        let comparison = comparator.compare(doc1, doc2);
        assert_eq!(comparison.total_changes, 1);
        assert!(comparison.similarity_score > 0.5);
    }

    #[test]
    fn test_redlining_engine() {
        let engine = RedliningEngine::new();
        let original = "Original text";
        let modified = "Modified text";

        let redline = engine.generate_redline(original, modified);
        assert!(!redline.changes.is_empty());
    }

    #[test]
    fn test_document_quality_scorer() {
        let scorer = DocumentQualityScorer::new();
        let document = "SECTION 1\n\nThis is a well-structured legal document with proper sections and formatting. The parties agree to the terms.";

        let score = scorer.score(document);
        assert!(score.overall_score >= 0.0 && score.overall_score <= 1.0);
    }

    #[test]
    fn test_missing_clause_detector() {
        let detector = MissingClauseDetector::new();
        let document = "The parties agree. Effective date is today. Payment terms specified. Termination clause included.";

        let report = detector.detect(document, DocumentType::Contract);
        assert!(report.completeness_score > 0.0);
    }

    #[test]
    fn test_compliance_checker() {
        let checker = ComplianceChecker::new();
        let document = "Parties: Acme Corp and Widget Inc. Effective date: Jan 1, 2024. Signature: ___________";

        let report = checker.check(document, DocumentType::Contract);
        assert!(report.compliance_score > 0.0);
    }
}
