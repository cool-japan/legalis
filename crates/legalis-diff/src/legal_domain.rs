//! Legal-domain aware diffing for statute analysis.
//!
//! This module provides specialized diff analysis for legal documents,
//! including article/section structure awareness, citation tracking,
//! and defined term propagation analysis.
//!
//! # Examples
//!
//! ```
//! use legalis_diff::legal_domain::{LegalStructure, Citation, DefinedTerm};
//!
//! // Parse legal structure from statute title
//! let structure = LegalStructure::parse("Article 5, Section 2(a)");
//! assert_eq!(structure.article, Some(5));
//! assert_eq!(structure.section, Some("2(a)".to_string()));
//!
//! // Track citations
//! let citation = Citation::new("26 U.S.C. § 501(c)(3)");
//! assert_eq!(citation.title, Some("26 U.S.C.".to_string()));
//! ```

use crate::StatuteDiff;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Legal document structure information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LegalStructure {
    /// Article number (if applicable).
    pub article: Option<u32>,
    /// Section identifier.
    pub section: Option<String>,
    /// Subsection identifier.
    pub subsection: Option<String>,
    /// Paragraph identifier.
    pub paragraph: Option<String>,
    /// Chapter number (if applicable).
    pub chapter: Option<u32>,
    /// Part identifier.
    pub part: Option<String>,
}

impl LegalStructure {
    /// Creates a new empty legal structure.
    pub fn new() -> Self {
        Self {
            article: None,
            section: None,
            subsection: None,
            paragraph: None,
            chapter: None,
            part: None,
        }
    }

    /// Parses legal structure from a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::legal_domain::LegalStructure;
    ///
    /// let structure = LegalStructure::parse("Article 5, Section 2(a)");
    /// assert_eq!(structure.article, Some(5));
    /// ```
    pub fn parse(text: &str) -> Self {
        let mut structure = Self::new();
        let text_lower = text.to_lowercase();

        // Parse article
        if let Some(article_num) = extract_number_after(&text_lower, "article") {
            structure.article = Some(article_num);
        }

        // Parse chapter
        if let Some(chapter_num) = extract_number_after(&text_lower, "chapter") {
            structure.chapter = Some(chapter_num);
        }

        // Parse section (can be alphanumeric)
        if let Some(section_id) = extract_identifier_after(&text_lower, "section") {
            structure.section = Some(section_id);
        }

        // Parse subsection
        if let Some(subsection_id) = extract_identifier_after(&text_lower, "subsection") {
            structure.subsection = Some(subsection_id);
        }

        // Parse paragraph
        if let Some(para_id) = extract_identifier_after(&text_lower, "paragraph") {
            structure.paragraph = Some(para_id);
        }

        // Parse part
        if let Some(part_id) = extract_identifier_after(&text_lower, "part") {
            structure.part = Some(part_id);
        }

        structure
    }

    /// Checks if this structure is more specific than another.
    pub fn is_more_specific_than(&self, other: &Self) -> bool {
        let self_depth = self.depth();
        let other_depth = other.depth();
        self_depth > other_depth
    }

    /// Returns the depth of this structure (how many levels are specified).
    pub fn depth(&self) -> usize {
        let mut depth = 0;
        if self.chapter.is_some() {
            depth += 1;
        }
        if self.article.is_some() {
            depth += 1;
        }
        if self.part.is_some() {
            depth += 1;
        }
        if self.section.is_some() {
            depth += 1;
        }
        if self.subsection.is_some() {
            depth += 1;
        }
        if self.paragraph.is_some() {
            depth += 1;
        }
        depth
    }
}

impl Default for LegalStructure {
    fn default() -> Self {
        Self::new()
    }
}

/// A citation reference in a legal document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Citation {
    /// Full citation text.
    pub full_text: String,
    /// Title or code reference (e.g., "26 U.S.C.").
    pub title: Option<String>,
    /// Section reference.
    pub section: Option<String>,
    /// Type of citation.
    pub citation_type: CitationType,
}

/// Type of legal citation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CitationType {
    /// U.S. Code citation.
    USCode,
    /// Code of Federal Regulations.
    CFR,
    /// Public Law.
    PublicLaw,
    /// Case citation.
    Case,
    /// Internal reference.
    Internal,
    /// Other/Unknown.
    Other,
}

impl Citation {
    /// Creates a new citation.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::legal_domain::Citation;
    ///
    /// let citation = Citation::new("26 U.S.C. § 501(c)(3)");
    /// assert!(!citation.full_text.is_empty());
    /// ```
    pub fn new(text: &str) -> Self {
        let citation_type = Self::detect_type(text);
        let (title, section) = Self::parse_citation(text, citation_type);

        Self {
            full_text: text.to_string(),
            title,
            section,
            citation_type,
        }
    }

    /// Detects the type of citation.
    fn detect_type(text: &str) -> CitationType {
        let text_upper = text.to_uppercase();
        if text_upper.contains("U.S.C.") {
            CitationType::USCode
        } else if text_upper.contains("C.F.R.") {
            CitationType::CFR
        } else if text_upper.contains("PUB.") && text_upper.contains("L.") {
            CitationType::PublicLaw
        } else if text_upper.contains(" V. ") || text_upper.contains(" VS. ") {
            CitationType::Case
        } else if text.starts_with("§") || text.contains("Section") {
            CitationType::Internal
        } else {
            CitationType::Other
        }
    }

    /// Parses citation into components.
    fn parse_citation(text: &str, citation_type: CitationType) -> (Option<String>, Option<String>) {
        match citation_type {
            CitationType::USCode | CitationType::CFR => {
                // Format: "26 U.S.C. § 501"
                if let Some(section_idx) = text.find('§') {
                    let title = text[..section_idx].trim().to_string();
                    let section = text[section_idx..].trim().to_string();
                    (Some(title), Some(section))
                } else {
                    (Some(text.to_string()), None)
                }
            }
            CitationType::Internal => {
                if let Some(section_idx) = text.find('§') {
                    (None, Some(text[section_idx..].trim().to_string()))
                } else {
                    (None, Some(text.to_string()))
                }
            }
            _ => (Some(text.to_string()), None),
        }
    }
}

/// A defined term in a legal document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DefinedTerm {
    /// The term being defined.
    pub term: String,
    /// The definition.
    pub definition: String,
    /// Location where it's defined.
    pub defined_in: Option<LegalStructure>,
    /// References to this term in the document.
    pub references: Vec<String>,
}

impl DefinedTerm {
    /// Creates a new defined term.
    pub fn new(term: &str, definition: &str) -> Self {
        Self {
            term: term.to_string(),
            definition: definition.to_string(),
            defined_in: None,
            references: Vec::new(),
        }
    }

    /// Adds a reference location for this term.
    pub fn add_reference(&mut self, location: String) {
        if !self.references.contains(&location) {
            self.references.push(location);
        }
    }
}

/// Analysis of citation impacts in a diff.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationImpactAnalysis {
    /// Citations added.
    pub added_citations: Vec<Citation>,
    /// Citations removed.
    pub removed_citations: Vec<Citation>,
    /// Citations that may be affected by the changes.
    pub affected_citations: Vec<Citation>,
    /// Summary of citation impacts.
    pub impact_summary: String,
}

/// Analysis of defined term impacts in a diff.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefinedTermImpactAnalysis {
    /// Terms added.
    pub added_terms: Vec<DefinedTerm>,
    /// Terms removed.
    pub removed_terms: Vec<DefinedTerm>,
    /// Terms whose definitions changed.
    pub modified_terms: Vec<(DefinedTerm, DefinedTerm)>,
    /// Terms whose usage may be affected.
    pub affected_terms: Vec<DefinedTerm>,
    /// Summary of term impacts.
    pub impact_summary: String,
}

/// Analyzes citation impacts in a diff.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, legal_domain::analyze_citation_impact};
///
/// let old = Statute::new("law", "Refers to 26 U.S.C. § 501", Effect::new(EffectType::Grant, "Benefit"));
/// let new = Statute::new("law", "Refers to 42 U.S.C. § 1983", Effect::new(EffectType::Grant, "Benefit"));
///
/// let diff_result = diff(&old, &new).unwrap();
/// let citation_impact = analyze_citation_impact(&diff_result);
/// ```
pub fn analyze_citation_impact(diff: &StatuteDiff) -> CitationImpactAnalysis {
    let mut added_citations = Vec::new();
    let mut removed_citations = Vec::new();

    for change in &diff.changes {
        // Extract citations from old values
        if let Some(old_value) = &change.old_value {
            let citations = extract_citations(old_value);
            removed_citations.extend(citations);
        }

        // Extract citations from new values
        if let Some(new_value) = &change.new_value {
            let citations = extract_citations(new_value);
            added_citations.extend(citations);
        }
    }

    // Remove duplicates
    let removed_set: HashSet<_> = removed_citations
        .iter()
        .map(|c| c.full_text.clone())
        .collect();
    let added_set: HashSet<_> = added_citations
        .iter()
        .map(|c| c.full_text.clone())
        .collect();

    // Keep only truly added/removed citations
    added_citations.retain(|c| !removed_set.contains(&c.full_text));
    removed_citations.retain(|c| !added_set.contains(&c.full_text));

    let impact_summary = format!(
        "{} citation(s) added, {} citation(s) removed",
        added_citations.len(),
        removed_citations.len()
    );

    CitationImpactAnalysis {
        added_citations,
        removed_citations,
        affected_citations: Vec::new(),
        impact_summary,
    }
}

/// Analyzes defined term impacts in a diff.
pub fn analyze_defined_terms(diff: &StatuteDiff) -> DefinedTermImpactAnalysis {
    let mut added_terms = Vec::new();
    let mut removed_terms = Vec::new();
    let mut modified_terms = Vec::new();

    let mut old_terms: HashMap<String, DefinedTerm> = HashMap::new();
    let mut new_terms: HashMap<String, DefinedTerm> = HashMap::new();

    for change in &diff.changes {
        // Extract defined terms from old values
        if let Some(old_value) = &change.old_value {
            for term in extract_defined_terms(old_value) {
                old_terms.insert(term.term.clone(), term);
            }
        }

        // Extract defined terms from new values
        if let Some(new_value) = &change.new_value {
            for term in extract_defined_terms(new_value) {
                new_terms.insert(term.term.clone(), term);
            }
        }
    }

    // Find added, removed, and modified terms
    for (term_name, new_term) in &new_terms {
        if let Some(old_term) = old_terms.get(term_name) {
            if old_term.definition != new_term.definition {
                modified_terms.push((old_term.clone(), new_term.clone()));
            }
        } else {
            added_terms.push(new_term.clone());
        }
    }

    for (term_name, old_term) in &old_terms {
        if !new_terms.contains_key(term_name) {
            removed_terms.push(old_term.clone());
        }
    }

    let impact_summary = format!(
        "{} term(s) added, {} removed, {} modified",
        added_terms.len(),
        removed_terms.len(),
        modified_terms.len()
    );

    DefinedTermImpactAnalysis {
        added_terms,
        removed_terms,
        modified_terms,
        affected_terms: Vec::new(),
        impact_summary,
    }
}

/// Extracts citations from text.
#[allow(dead_code)]
fn extract_citations(text: &str) -> Vec<Citation> {
    let mut citations = Vec::new();

    // Pattern for U.S. Code citations
    if text.contains("U.S.C.") {
        // Simple extraction - in a real implementation, use regex
        let parts: Vec<&str> = text.split("U.S.C.").collect();
        for i in 0..parts.len() - 1 {
            let before = parts[i].split_whitespace().last().unwrap_or("");
            let after = parts[i + 1].split_whitespace().next().unwrap_or("");
            let citation_text = format!("{} U.S.C. {}", before, after);
            citations.push(Citation::new(&citation_text));
        }
    }

    // Pattern for CFR citations
    if text.contains("C.F.R.") {
        let parts: Vec<&str> = text.split("C.F.R.").collect();
        for i in 0..parts.len() - 1 {
            let before = parts[i].split_whitespace().last().unwrap_or("");
            let after = parts[i + 1].split_whitespace().next().unwrap_or("");
            let citation_text = format!("{} C.F.R. {}", before, after);
            citations.push(Citation::new(&citation_text));
        }
    }

    // Pattern for section symbols
    if text.contains('§') {
        let parts: Vec<&str> = text.split('§').collect();
        for part in parts.iter().skip(1) {
            let section = part.split_whitespace().next().unwrap_or("");
            if !section.is_empty() {
                citations.push(Citation::new(&format!("§ {}", section)));
            }
        }
    }

    citations
}

/// Extracts defined terms from text.
#[allow(dead_code)]
fn extract_defined_terms(text: &str) -> Vec<DefinedTerm> {
    let mut terms = Vec::new();

    // Look for patterns like "X means Y" or "X is defined as Y"
    let text_lower = text.to_lowercase();

    if let Some(means_idx) = text_lower.find(" means ") {
        let before = text[..means_idx].trim();
        let after = text[means_idx + 7..].trim();
        if let Some(term_start) = before.rfind('"') {
            let term = before[term_start + 1..].trim();
            terms.push(DefinedTerm::new(term, after));
        }
    }

    if let Some(defined_idx) = text_lower.find(" is defined as ") {
        let before = text[..defined_idx].trim();
        let after = text[defined_idx + 15..].trim();
        if let Some(term_start) = before.rfind('"') {
            let term = before[term_start + 1..].trim();
            terms.push(DefinedTerm::new(term, after));
        }
    }

    terms
}

/// Helper function to extract a number after a keyword.
#[allow(dead_code)]
fn extract_number_after(text: &str, keyword: &str) -> Option<u32> {
    if let Some(idx) = text.find(keyword) {
        let after = &text[idx + keyword.len()..];
        for word in after.split_whitespace() {
            if let Ok(num) = word.trim_matches(|c: char| !c.is_numeric()).parse::<u32>() {
                return Some(num);
            }
        }
    }
    None
}

/// Helper function to extract an identifier after a keyword.
#[allow(dead_code)]
fn extract_identifier_after(text: &str, keyword: &str) -> Option<String> {
    if let Some(idx) = text.find(keyword) {
        let after = &text[idx + keyword.len()..];
        for word in after.split_whitespace() {
            let cleaned = word.trim_matches(|c: char| c == ',' || c == '.' || c == ';');
            if !cleaned.is_empty() {
                return Some(cleaned.to_string());
            }
        }
    }
    None
}

/// Analyzes cross-reference impacts when a statute section is modified.
pub fn analyze_cross_reference_impact(
    _diff: &StatuteDiff,
    _all_statutes: &[crate::StatuteDiff],
) -> Vec<String> {
    // This would analyze other statutes that reference the modified statute
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legal_structure_parse() {
        let structure = LegalStructure::parse("Article 5, Section 2(a)");
        assert_eq!(structure.article, Some(5));
        assert_eq!(structure.section, Some("2(a)".to_string()));
    }

    #[test]
    fn test_legal_structure_depth() {
        let mut structure = LegalStructure::new();
        assert_eq!(structure.depth(), 0);

        structure.article = Some(1);
        assert_eq!(structure.depth(), 1);

        structure.section = Some("2".to_string());
        assert_eq!(structure.depth(), 2);
    }

    #[test]
    fn test_citation_usc() {
        let citation = Citation::new("26 U.S.C. § 501(c)(3)");
        assert_eq!(citation.citation_type, CitationType::USCode);
        assert!(citation.title.is_some());
    }

    #[test]
    fn test_citation_cfr() {
        let citation = Citation::new("26 C.F.R. § 1.501");
        assert_eq!(citation.citation_type, CitationType::CFR);
    }

    #[test]
    fn test_citation_internal() {
        let citation = Citation::new("§ 123");
        assert_eq!(citation.citation_type, CitationType::Internal);
    }

    #[test]
    fn test_defined_term() {
        let mut term = DefinedTerm::new(
            "Qualified Benefit",
            "A benefit meeting criteria X, Y, and Z",
        );
        assert_eq!(term.term, "Qualified Benefit");
        assert_eq!(term.references.len(), 0);

        term.add_reference("Section 5".to_string());
        assert_eq!(term.references.len(), 1);

        term.add_reference("Section 5".to_string());
        assert_eq!(term.references.len(), 1); // Should not duplicate
    }

    #[test]
    fn test_extract_citations_usc() {
        let text = "This refers to 26 U.S.C. § 501";
        let citations = extract_citations(text);
        assert!(!citations.is_empty());
    }

    #[test]
    fn test_extract_citations_section() {
        let text = "See § 123 for details";
        let citations = extract_citations(text);
        assert!(!citations.is_empty());
    }
}
