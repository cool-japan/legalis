//! Legal document processing and structure recognition.
//!
//! This module provides tools for parsing, analyzing, and extracting information
//! from legal documents including:
//! - Hierarchical document structure (sections, articles, clauses)
//! - Clause-level parsing
//! - Cross-statute reference resolution
//! - Legal named entity recognition
//! - Entity extraction framework

use chrono::NaiveDate;
use std::collections::HashMap;
use std::fmt;

/// Type of legal document section.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum SectionType {
    /// Title of the document.
    Title,
    /// Chapter division.
    Chapter,
    /// Article within a chapter.
    Article,
    /// Section within an article.
    Section,
    /// Subsection.
    Subsection,
    /// Paragraph.
    Paragraph,
    /// Clause.
    Clause,
    /// Subclause.
    Subclause,
    /// Preamble or introduction.
    Preamble,
    /// Definitions section.
    Definitions,
    /// Appendix or schedule.
    Appendix,
}

impl fmt::Display for SectionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SectionType::Title => write!(f, "Title"),
            SectionType::Chapter => write!(f, "Chapter"),
            SectionType::Article => write!(f, "Article"),
            SectionType::Section => write!(f, "Section"),
            SectionType::Subsection => write!(f, "Subsection"),
            SectionType::Paragraph => write!(f, "Paragraph"),
            SectionType::Clause => write!(f, "Clause"),
            SectionType::Subclause => write!(f, "Subclause"),
            SectionType::Preamble => write!(f, "Preamble"),
            SectionType::Definitions => write!(f, "Definitions"),
            SectionType::Appendix => write!(f, "Appendix"),
        }
    }
}

/// A section in a legal document.
///
/// # Examples
///
/// ```
/// use legalis_core::document_processing::{DocumentSection, SectionType};
///
/// let section = DocumentSection::new(SectionType::Article, "1", "Purpose")
///     .with_content("This Act establishes the framework for...");
///
/// assert_eq!(section.number, "1");
/// assert_eq!(section.section_type, SectionType::Article);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DocumentSection {
    /// Type of section.
    pub section_type: SectionType,
    /// Section number (e.g., "1", "2.1", "3(a)").
    pub number: String,
    /// Section title or heading.
    pub title: String,
    /// Section content/text.
    pub content: String,
    /// Child sections.
    pub children: Vec<DocumentSection>,
    /// Cross-references to other sections.
    pub references: Vec<String>,
}

impl DocumentSection {
    /// Creates a new document section.
    pub fn new(
        section_type: SectionType,
        number: impl Into<String>,
        title: impl Into<String>,
    ) -> Self {
        Self {
            section_type,
            number: number.into(),
            title: title.into(),
            content: String::new(),
            children: Vec::new(),
            references: Vec::new(),
        }
    }

    /// Sets the content.
    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    /// Adds a child section.
    pub fn with_child(mut self, child: DocumentSection) -> Self {
        self.children.push(child);
        self
    }

    /// Adds a reference.
    pub fn with_reference(mut self, reference: impl Into<String>) -> Self {
        self.references.push(reference.into());
        self
    }

    /// Returns the full section number path (e.g., "1.2.3").
    pub fn full_path(&self) -> String {
        self.number.clone()
    }

    /// Checks if this section has children.
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    /// Returns the number of child sections.
    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    /// Recursively counts all descendant sections.
    pub fn total_sections(&self) -> usize {
        1 + self
            .children
            .iter()
            .map(|c| c.total_sections())
            .sum::<usize>()
    }

    /// Finds a child section by number.
    pub fn find_child(&self, number: &str) -> Option<&DocumentSection> {
        self.children.iter().find(|c| c.number == number)
    }

    /// Finds a section by path (e.g., "1.2.3").
    pub fn find_by_path(&self, path: &str) -> Option<&DocumentSection> {
        let parts: Vec<&str> = path.split('.').collect();
        if parts.is_empty() {
            return None;
        }

        if parts[0] == self.number {
            if parts.len() == 1 {
                return Some(self);
            }
            let remaining = parts[1..].join(".");
            return self
                .children
                .iter()
                .find_map(|c| c.find_by_path(&remaining));
        }

        None
    }
}

impl fmt::Display for DocumentSection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}: {}", self.section_type, self.number, self.title)
    }
}

/// A legal document with hierarchical structure.
///
/// # Examples
///
/// ```
/// use legalis_core::document_processing::{LegalDocument, DocumentSection, SectionType};
///
/// let mut doc = LegalDocument::new("Act-2025-123", "Data Protection Act 2025");
/// doc.set_jurisdiction("US");
///
/// let section = DocumentSection::new(SectionType::Article, "1", "Definitions")
///     .with_content("In this Act...");
/// doc.add_section(section);
///
/// assert_eq!(doc.section_count(), 1);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LegalDocument {
    /// Document ID.
    pub id: String,
    /// Document title.
    pub title: String,
    /// Jurisdiction.
    pub jurisdiction: Option<String>,
    /// Enactment date.
    pub enacted_date: Option<NaiveDate>,
    /// Root sections.
    pub sections: Vec<DocumentSection>,
    /// Document metadata.
    pub metadata: HashMap<String, String>,
}

impl LegalDocument {
    /// Creates a new legal document.
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            jurisdiction: None,
            enacted_date: None,
            sections: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Sets the jurisdiction.
    pub fn set_jurisdiction(&mut self, jurisdiction: impl Into<String>) {
        self.jurisdiction = Some(jurisdiction.into());
    }

    /// Sets the enactment date.
    pub fn set_enacted_date(&mut self, date: NaiveDate) {
        self.enacted_date = Some(date);
    }

    /// Adds a section.
    pub fn add_section(&mut self, section: DocumentSection) {
        self.sections.push(section);
    }

    /// Adds metadata.
    pub fn add_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }

    /// Returns the number of root sections.
    pub fn section_count(&self) -> usize {
        self.sections.len()
    }

    /// Returns the total number of sections (including nested).
    pub fn total_sections(&self) -> usize {
        self.sections.iter().map(|s| s.total_sections()).sum()
    }

    /// Finds a section by path.
    pub fn find_section(&self, path: &str) -> Option<&DocumentSection> {
        self.sections.iter().find_map(|s| s.find_by_path(path))
    }

    /// Extracts all references from the document.
    pub fn all_references(&self) -> Vec<String> {
        let mut refs = Vec::new();
        fn collect_refs(section: &DocumentSection, refs: &mut Vec<String>) {
            refs.extend(section.references.clone());
            for child in &section.children {
                collect_refs(child, refs);
            }
        }
        for section in &self.sections {
            collect_refs(section, &mut refs);
        }
        refs
    }
}

/// Type of legal reference.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum ReferenceType {
    /// Reference to a statute.
    Statute,
    /// Reference to a case.
    Case,
    /// Reference to a regulation.
    Regulation,
    /// Reference to a treaty.
    Treaty,
    /// Internal reference (within same document).
    Internal,
    /// Reference to a definition.
    Definition,
}

impl fmt::Display for ReferenceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReferenceType::Statute => write!(f, "Statute"),
            ReferenceType::Case => write!(f, "Case"),
            ReferenceType::Regulation => write!(f, "Regulation"),
            ReferenceType::Treaty => write!(f, "Treaty"),
            ReferenceType::Internal => write!(f, "Internal"),
            ReferenceType::Definition => write!(f, "Definition"),
        }
    }
}

/// A parsed legal reference.
///
/// # Examples
///
/// ```
/// use legalis_core::document_processing::{LegalReference, ReferenceType};
///
/// let reference = LegalReference::new(
///     "Section 123 of the Data Protection Act 2025",
///     ReferenceType::Statute,
/// )
/// .with_target_id("DPA-2025")
/// .with_section("123");
///
/// assert_eq!(reference.reference_type, ReferenceType::Statute);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LegalReference {
    /// Raw reference text.
    pub text: String,
    /// Type of reference.
    pub reference_type: ReferenceType,
    /// Target document ID (if resolved).
    pub target_id: Option<String>,
    /// Target section (if applicable).
    pub target_section: Option<String>,
    /// Confidence score (0.0 to 1.0).
    pub confidence: f64,
}

impl LegalReference {
    /// Creates a new legal reference.
    pub fn new(text: impl Into<String>, reference_type: ReferenceType) -> Self {
        Self {
            text: text.into(),
            reference_type,
            target_id: None,
            target_section: None,
            confidence: 1.0,
        }
    }

    /// Sets the target ID.
    pub fn with_target_id(mut self, target_id: impl Into<String>) -> Self {
        self.target_id = Some(target_id.into());
        self
    }

    /// Sets the target section.
    pub fn with_section(mut self, section: impl Into<String>) -> Self {
        self.target_section = Some(section.into());
        self
    }

    /// Sets the confidence score.
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    /// Checks if this reference is resolved.
    pub fn is_resolved(&self) -> bool {
        self.target_id.is_some()
    }
}

/// Reference resolver for cross-statute citations.
///
/// # Examples
///
/// ```
/// use legalis_core::document_processing::ReferenceResolver;
///
/// let mut resolver = ReferenceResolver::new();
/// resolver.register_statute("DPA-2025", "Data Protection Act 2025");
/// resolver.register_statute("GDPR", "General Data Protection Regulation");
///
/// let references = resolver.resolve_text("See Section 5 of the Data Protection Act 2025");
/// assert!(!references.is_empty());
/// ```
#[derive(Debug, Clone, Default)]
pub struct ReferenceResolver {
    /// Known statutes (ID -> title).
    statutes: HashMap<String, String>,
    /// Known cases (ID -> name).
    cases: HashMap<String, String>,
}

impl ReferenceResolver {
    /// Creates a new reference resolver.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a statute.
    pub fn register_statute(&mut self, id: impl Into<String>, title: impl Into<String>) {
        self.statutes.insert(id.into(), title.into());
    }

    /// Registers a case.
    pub fn register_case(&mut self, id: impl Into<String>, name: impl Into<String>) {
        self.cases.insert(id.into(), name.into());
    }

    /// Resolves references in text.
    pub fn resolve_text(&self, text: &str) -> Vec<LegalReference> {
        let mut references = Vec::new();

        // Simple pattern matching for statute references
        for (id, title) in &self.statutes {
            if text.contains(title) {
                let reference = LegalReference::new(title.clone(), ReferenceType::Statute)
                    .with_target_id(id.clone())
                    .with_confidence(0.9);
                references.push(reference);
            }
        }

        // Pattern for section references (e.g., "Section 123")
        let section_pattern = regex::Regex::new(r"Section\s+(\d+)").ok();
        if let Some(re) = section_pattern {
            for cap in re.captures_iter(text) {
                if let Some(section_num) = cap.get(1) {
                    let reference =
                        LegalReference::new(cap.get(0).unwrap().as_str(), ReferenceType::Internal)
                            .with_section(section_num.as_str())
                            .with_confidence(0.8);
                    references.push(reference);
                }
            }
        }

        references
    }

    /// Finds the target statute for a reference.
    pub fn find_target(&self, reference: &LegalReference) -> Option<String> {
        reference.target_id.clone()
    }
}

/// Type of legal entity in text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum EntityType {
    /// Person name.
    Person,
    /// Organization name.
    Organization,
    /// Court name.
    Court,
    /// Date.
    Date,
    /// Location/jurisdiction.
    Location,
    /// Statute or law name.
    Statute,
    /// Case name.
    Case,
    /// Legal term or concept.
    LegalTerm,
    /// Monetary amount.
    Money,
    /// Time period.
    Duration,
}

impl fmt::Display for EntityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EntityType::Person => write!(f, "Person"),
            EntityType::Organization => write!(f, "Organization"),
            EntityType::Court => write!(f, "Court"),
            EntityType::Date => write!(f, "Date"),
            EntityType::Location => write!(f, "Location"),
            EntityType::Statute => write!(f, "Statute"),
            EntityType::Case => write!(f, "Case"),
            EntityType::LegalTerm => write!(f, "LegalTerm"),
            EntityType::Money => write!(f, "Money"),
            EntityType::Duration => write!(f, "Duration"),
        }
    }
}

/// A named entity extracted from legal text.
///
/// # Examples
///
/// ```
/// use legalis_core::document_processing::{NamedEntity, EntityType};
///
/// let entity = NamedEntity::new("Supreme Court", EntityType::Court, 0, 13)
///     .with_confidence(0.95);
///
/// assert_eq!(entity.entity_type, EntityType::Court);
/// assert_eq!(entity.text, "Supreme Court");
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NamedEntity {
    /// Entity text.
    pub text: String,
    /// Entity type.
    pub entity_type: EntityType,
    /// Start position in source text.
    pub start: usize,
    /// End position in source text.
    pub end: usize,
    /// Confidence score (0.0 to 1.0).
    pub confidence: f64,
    /// Additional metadata.
    pub metadata: HashMap<String, String>,
}

impl NamedEntity {
    /// Creates a new named entity.
    pub fn new(text: impl Into<String>, entity_type: EntityType, start: usize, end: usize) -> Self {
        Self {
            text: text.into(),
            entity_type,
            start,
            end,
            confidence: 1.0,
            metadata: HashMap::new(),
        }
    }

    /// Sets the confidence score.
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    /// Adds metadata.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Returns the length of the entity text.
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Checks if the entity is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Legal named entity recognizer (NER).
///
/// # Examples
///
/// ```
/// use legalis_core::document_processing::{LegalNER, EntityType};
///
/// let mut ner = LegalNER::new();
/// ner.add_pattern(EntityType::Court, "Supreme Court");
/// ner.add_pattern(EntityType::Court, "Court of Appeals");
///
/// let text = "The Supreme Court ruled that...";
/// let entities = ner.extract(text);
///
/// assert_eq!(entities.len(), 1);
/// assert_eq!(entities[0].entity_type, EntityType::Court);
/// ```
#[derive(Debug, Clone, Default)]
pub struct LegalNER {
    /// Known patterns for each entity type.
    patterns: HashMap<EntityType, Vec<String>>,
}

impl LegalNER {
    /// Creates a new legal NER.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a NER with default legal patterns.
    pub fn with_defaults() -> Self {
        let mut ner = Self::new();

        // Courts
        ner.add_pattern(EntityType::Court, "Supreme Court");
        ner.add_pattern(EntityType::Court, "Court of Appeals");
        ner.add_pattern(EntityType::Court, "District Court");
        ner.add_pattern(EntityType::Court, "High Court");

        // Legal terms
        ner.add_pattern(EntityType::LegalTerm, "plaintiff");
        ner.add_pattern(EntityType::LegalTerm, "defendant");
        ner.add_pattern(EntityType::LegalTerm, "jurisdiction");
        ner.add_pattern(EntityType::LegalTerm, "statute");
        ner.add_pattern(EntityType::LegalTerm, "precedent");

        ner
    }

    /// Adds a pattern for an entity type.
    pub fn add_pattern(&mut self, entity_type: EntityType, pattern: impl Into<String>) {
        self.patterns
            .entry(entity_type)
            .or_default()
            .push(pattern.into());
    }

    /// Extracts entities from text.
    pub fn extract(&self, text: &str) -> Vec<NamedEntity> {
        let mut entities = Vec::new();

        for (entity_type, patterns) in &self.patterns {
            for pattern in patterns {
                let mut start = 0;
                while let Some(pos) = text[start..].find(pattern) {
                    let abs_start = start + pos;
                    let abs_end = abs_start + pattern.len();

                    let entity =
                        NamedEntity::new(pattern.clone(), *entity_type, abs_start, abs_end)
                            .with_confidence(0.8);
                    entities.push(entity);

                    start = abs_end;
                }
            }
        }

        // Sort by start position
        entities.sort_by_key(|e| e.start);

        entities
    }

    /// Extracts entities of a specific type.
    pub fn extract_type(&self, text: &str, entity_type: EntityType) -> Vec<NamedEntity> {
        self.extract(text)
            .into_iter()
            .filter(|e| e.entity_type == entity_type)
            .collect()
    }

    /// Returns the number of patterns registered.
    pub fn pattern_count(&self) -> usize {
        self.patterns.values().map(|v| v.len()).sum()
    }
}

/// Clause parser for legal documents.
///
/// # Examples
///
/// ```
/// use legalis_core::document_processing::ClauseParser;
///
/// let parser = ClauseParser::new();
/// let clauses = parser.parse_clauses(
///     "The party shall provide notice; The party shall maintain records; The party shall comply."
/// );
///
/// assert!(clauses.len() >= 3);
/// ```
#[derive(Debug, Clone, Default)]
pub struct ClauseParser {
    /// Minimum clause length.
    min_length: usize,
}

impl ClauseParser {
    /// Creates a new clause parser.
    pub fn new() -> Self {
        Self { min_length: 10 }
    }

    /// Sets the minimum clause length.
    pub fn with_min_length(mut self, length: usize) -> Self {
        self.min_length = length;
        self
    }

    /// Parses clauses from text.
    pub fn parse_clauses(&self, text: &str) -> Vec<String> {
        let mut clauses = Vec::new();

        // Split by semicolons and periods
        let parts: Vec<&str> = text.split([';', '.']).collect();

        for part in parts {
            let trimmed = part.trim();
            if trimmed.len() >= self.min_length {
                clauses.push(trimmed.to_string());
            }
        }

        // Also try to extract enumerated clauses (a), (b), (c)
        let enum_pattern = regex::Regex::new(r"\(([a-z])\)\s*([^(;.]+)").ok();
        if let Some(re) = enum_pattern {
            for cap in re.captures_iter(text) {
                if let Some(clause_text) = cap.get(2) {
                    let trimmed = clause_text.as_str().trim();
                    if trimmed.len() >= self.min_length {
                        clauses.push(trimmed.to_string());
                    }
                }
            }
        }

        // Remove duplicates while preserving order
        let mut seen = std::collections::HashSet::new();
        clauses.retain(|c| seen.insert(c.clone()));

        clauses
    }

    /// Parses numbered sections (1., 2., 3., etc.).
    pub fn parse_numbered_sections(&self, text: &str) -> Vec<(usize, String)> {
        let mut sections = Vec::new();

        let pattern = regex::Regex::new(r"(\d+)\.\s+([^0-9]+(?:[0-9]+[^0-9]+)*)").ok();
        if let Some(re) = pattern {
            for cap in re.captures_iter(text) {
                if let (Some(num), Some(content)) = (cap.get(1), cap.get(2)) {
                    if let Ok(number) = num.as_str().parse::<usize>() {
                        let trimmed = content.as_str().trim();
                        if trimmed.len() >= self.min_length {
                            sections.push((number, trimmed.to_string()));
                        }
                    }
                }
            }
        }

        sections
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_section() {
        let section = DocumentSection::new(SectionType::Article, "1", "Purpose")
            .with_content("This establishes...");

        assert_eq!(section.section_type, SectionType::Article);
        assert_eq!(section.number, "1");
        assert!(!section.content.is_empty());
    }

    #[test]
    fn test_legal_document() {
        let mut doc = LegalDocument::new("Act-2025", "Test Act");
        doc.add_section(DocumentSection::new(SectionType::Article, "1", "Test"));

        assert_eq!(doc.section_count(), 1);
        assert_eq!(doc.total_sections(), 1);
    }

    #[test]
    fn test_reference_resolver() {
        let mut resolver = ReferenceResolver::new();
        resolver.register_statute("DPA", "Data Protection Act");

        let refs = resolver.resolve_text("See the Data Protection Act");
        assert!(!refs.is_empty());
    }

    #[test]
    fn test_legal_ner() {
        let mut ner = LegalNER::new();
        ner.add_pattern(EntityType::Court, "Supreme Court");

        let entities = ner.extract("The Supreme Court ruled...");
        assert_eq!(entities.len(), 1);
        assert_eq!(entities[0].entity_type, EntityType::Court);
    }

    #[test]
    fn test_clause_parser() {
        let parser = ClauseParser::new();
        let clauses = parser.parse_clauses("Clause one; Clause two; Clause three.");

        assert_eq!(clauses.len(), 3);
    }
}
