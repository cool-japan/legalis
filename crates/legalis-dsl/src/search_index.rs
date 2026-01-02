//! Search index generation for legal documents.
//!
//! This module provides utilities to generate search indices that enable
//! fast full-text search across legal documents and statutes.

use crate::ast::{ConditionNode, ConditionValue, LegalDocument, StatuteNode};
use std::collections::{HashMap, HashSet};

/// A search index for legal documents.
#[derive(Debug, Clone)]
pub struct SearchIndex {
    /// Inverted index: term -> set of statute IDs
    term_to_statutes: HashMap<String, HashSet<String>>,
    /// Statute ID to document mapping
    statute_metadata: HashMap<String, StatuteMetadata>,
    /// Full-text index for statute content
    content_index: HashMap<String, String>,
}

/// Metadata about a statute for search purposes.
#[derive(Debug, Clone)]
pub struct StatuteMetadata {
    /// Statute ID
    pub id: String,
    /// Statute title
    pub title: String,
    /// Statute visibility
    pub visibility: String,
    /// Number of conditions
    pub condition_count: usize,
    /// Number of effects
    pub effect_count: usize,
    /// Required statute IDs
    pub requires: Vec<String>,
    /// Superseded statute IDs
    pub supersedes: Vec<String>,
}

/// Search result for a query.
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// Statute ID
    pub statute_id: String,
    /// Statute title
    pub title: String,
    /// Relevance score (0.0 to 1.0)
    pub score: f64,
    /// Matching terms
    pub matching_terms: Vec<String>,
}

impl SearchIndex {
    /// Creates a new empty search index.
    pub fn new() -> Self {
        Self {
            term_to_statutes: HashMap::new(),
            statute_metadata: HashMap::new(),
            content_index: HashMap::new(),
        }
    }

    /// Builds a search index from a legal document.
    pub fn from_document(doc: &LegalDocument) -> Self {
        let mut index = Self::new();
        index.index_document(doc);
        index
    }

    /// Indexes a legal document.
    pub fn index_document(&mut self, doc: &LegalDocument) {
        for statute in &doc.statutes {
            self.index_statute(statute);
        }
    }

    /// Indexes a single statute.
    fn index_statute(&mut self, statute: &StatuteNode) {
        // Create metadata
        let metadata = StatuteMetadata {
            id: statute.id.clone(),
            title: statute.title.clone(),
            visibility: format!("{:?}", statute.visibility),
            condition_count: statute.conditions.len(),
            effect_count: statute.effects.len(),
            requires: statute.requires.clone(),
            supersedes: statute.supersedes.clone(),
        };
        self.statute_metadata.insert(statute.id.clone(), metadata);

        // Build full content for this statute
        let mut content = String::new();
        content.push_str(&statute.id);
        content.push(' ');
        content.push_str(&statute.title);
        content.push(' ');

        // Add condition fields and values
        for condition in &statute.conditions {
            content.push_str(&self.extract_condition_text(condition));
            content.push(' ');
        }

        // Add effect descriptions
        for effect in &statute.effects {
            content.push_str(&effect.effect_type);
            content.push(' ');
            content.push_str(&effect.description);
            content.push(' ');
        }

        // Add discretion
        if let Some(discretion) = &statute.discretion {
            content.push_str(discretion);
            content.push(' ');
        }

        // Add exception descriptions
        for exception in &statute.exceptions {
            content.push_str(&exception.description);
            content.push(' ');
        }

        // Store full content
        self.content_index
            .insert(statute.id.clone(), content.clone());

        // Tokenize and index
        let tokens = self.tokenize(&content);
        for token in tokens {
            self.term_to_statutes
                .entry(token)
                .or_insert_with(HashSet::new)
                .insert(statute.id.clone());
        }
    }

    /// Extracts searchable text from a condition.
    fn extract_condition_text(&self, condition: &ConditionNode) -> String {
        match condition {
            ConditionNode::Comparison {
                field,
                operator,
                value,
            } => {
                format!("{} {} {}", field, operator, self.format_value(value))
            }
            ConditionNode::HasAttribute { key } => format!("has {}", key),
            ConditionNode::Between { field, min, max } => {
                format!(
                    "{} between {} and {}",
                    field,
                    self.format_value(min),
                    self.format_value(max)
                )
            }
            ConditionNode::In { field, values } => {
                let vals: Vec<_> = values.iter().map(|v| self.format_value(v)).collect();
                format!("{} in {}", field, vals.join(" "))
            }
            ConditionNode::Like { field, pattern } => {
                format!("{} like {}", field, pattern)
            }
            ConditionNode::Matches {
                field,
                regex_pattern,
            } => {
                format!("{} matches {}", field, regex_pattern)
            }
            ConditionNode::InRange {
                field, min, max, ..
            } => {
                format!(
                    "{} in range {} to {}",
                    field,
                    self.format_value(min),
                    self.format_value(max)
                )
            }
            ConditionNode::NotInRange {
                field, min, max, ..
            } => {
                format!(
                    "{} not in range {} to {}",
                    field,
                    self.format_value(min),
                    self.format_value(max)
                )
            }
            ConditionNode::TemporalComparison {
                field,
                operator,
                value,
            } => {
                format!("{:?} {} {}", field, operator, self.format_value(value))
            }
            ConditionNode::And(left, right) => {
                format!(
                    "{} and {}",
                    self.extract_condition_text(left),
                    self.extract_condition_text(right)
                )
            }
            ConditionNode::Or(left, right) => {
                format!(
                    "{} or {}",
                    self.extract_condition_text(left),
                    self.extract_condition_text(right)
                )
            }
            ConditionNode::Not(inner) => {
                format!("not {}", self.extract_condition_text(inner))
            }
        }
    }

    /// Formats a value for indexing.
    fn format_value(&self, value: &ConditionValue) -> String {
        match value {
            ConditionValue::Number(n) => n.to_string(),
            ConditionValue::String(s) => s.clone(),
            ConditionValue::Boolean(b) => b.to_string(),
            ConditionValue::Date(d) => d.clone(),
            ConditionValue::SetExpr(_) => String::new(),
        }
    }

    /// Tokenizes text into searchable terms.
    fn tokenize(&self, text: &str) -> Vec<String> {
        text.to_lowercase()
            .split(|c: char| !c.is_alphanumeric() && c != '-')
            .filter(|s| !s.is_empty())
            .filter(|s| s.len() >= 2) // Filter out single-character tokens
            .map(|s| s.to_string())
            .collect()
    }

    /// Searches for statutes matching the given query.
    pub fn search(&self, query: &str) -> Vec<SearchResult> {
        let query_terms = self.tokenize(query);
        if query_terms.is_empty() {
            return Vec::new();
        }

        // Find all statutes that match at least one term
        let mut statute_scores: HashMap<String, (f64, HashSet<String>)> = HashMap::new();

        for term in &query_terms {
            if let Some(statute_ids) = self.term_to_statutes.get(term) {
                for statute_id in statute_ids {
                    let entry = statute_scores
                        .entry(statute_id.clone())
                        .or_insert((0.0, HashSet::new()));
                    entry.0 += 1.0;
                    entry.1.insert(term.clone());
                }
            }
        }

        // Calculate final scores (normalize by query length)
        let mut results: Vec<SearchResult> = statute_scores
            .into_iter()
            .filter_map(|(statute_id, (score, matching_terms))| {
                let metadata = self.statute_metadata.get(&statute_id)?;
                Some(SearchResult {
                    statute_id: statute_id.clone(),
                    title: metadata.title.clone(),
                    score: score / query_terms.len() as f64,
                    matching_terms: matching_terms.into_iter().collect(),
                })
            })
            .collect();

        // Sort by score descending
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        results
    }

    /// Searches for an exact statute ID.
    pub fn find_by_id(&self, statute_id: &str) -> Option<&StatuteMetadata> {
        self.statute_metadata.get(statute_id)
    }

    /// Searches for statutes by title (case-insensitive substring match).
    pub fn search_by_title(&self, title_query: &str) -> Vec<&StatuteMetadata> {
        let query_lower = title_query.to_lowercase();
        self.statute_metadata
            .values()
            .filter(|meta| meta.title.to_lowercase().contains(&query_lower))
            .collect()
    }

    /// Returns all indexed statute IDs.
    pub fn all_statute_ids(&self) -> Vec<String> {
        self.statute_metadata.keys().cloned().collect()
    }

    /// Returns the number of indexed statutes.
    pub fn statute_count(&self) -> usize {
        self.statute_metadata.len()
    }

    /// Returns statistics about the index.
    pub fn index_stats(&self) -> IndexStats {
        let total_terms = self.term_to_statutes.len();
        let total_statutes = self.statute_metadata.len();
        let avg_terms_per_statute = if total_statutes > 0 {
            self.term_to_statutes
                .values()
                .map(|s| s.len())
                .sum::<usize>() as f64
                / total_terms as f64
        } else {
            0.0
        };

        IndexStats {
            total_terms,
            total_statutes,
            avg_terms_per_statute,
        }
    }
}

impl Default for SearchIndex {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about the search index.
#[derive(Debug, Clone)]
pub struct IndexStats {
    /// Total number of unique terms
    pub total_terms: usize,
    /// Total number of indexed statutes
    pub total_statutes: usize,
    /// Average number of terms per statute
    pub avg_terms_per_statute: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ConditionNode, ConditionValue, EffectNode};

    fn sample_document() -> LegalDocument {
        LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![
                StatuteNode {
                    id: "voting-rights-1".to_string(),
                    title: "Voting Rights Act".to_string(),
                    visibility: crate::module_system::Visibility::Private,
                    conditions: vec![
                        ConditionNode::Comparison {
                            field: "age".to_string(),
                            operator: ">=".to_string(),
                            value: ConditionValue::Number(18),
                        },
                        ConditionNode::HasAttribute {
                            key: "citizen".to_string(),
                        },
                    ],
                    effects: vec![EffectNode {
                        effect_type: "GRANT".to_string(),
                        description: "Right to vote in elections".to_string(),
                        parameters: vec![],
                    }],
                    discretion: Some("Consider residency requirements".to_string()),
                    exceptions: vec![],
                    amendments: vec![],
                    supersedes: vec![],
                    defaults: vec![],
                    requires: vec![],
                    delegates: vec![],
                    scope: None,
                    constraints: vec![],
                    priority: None,
                },
                StatuteNode {
                    id: "tax-exemption-1".to_string(),
                    title: "Tax Exemption for Low Income".to_string(),
                    visibility: crate::module_system::Visibility::Private,
                    conditions: vec![ConditionNode::Comparison {
                        field: "income".to_string(),
                        operator: "<".to_string(),
                        value: ConditionValue::Number(20000),
                    }],
                    effects: vec![EffectNode {
                        effect_type: "EXEMPT".to_string(),
                        description: "Exempt from income tax".to_string(),
                        parameters: vec![],
                    }],
                    discretion: None,
                    exceptions: vec![],
                    amendments: vec![],
                    supersedes: vec![],
                    defaults: vec![],
                    requires: vec![],
                    delegates: vec![],
                    scope: None,
                    constraints: vec![],
                    priority: None,
                },
            ],
        }
    }

    #[test]
    fn test_search_index_creation() {
        let doc = sample_document();
        let index = SearchIndex::from_document(&doc);

        assert_eq!(index.statute_count(), 2);
        assert!(
            index
                .all_statute_ids()
                .contains(&"voting-rights-1".to_string())
        );
        assert!(
            index
                .all_statute_ids()
                .contains(&"tax-exemption-1".to_string())
        );
    }

    #[test]
    fn test_search_by_term() {
        let doc = sample_document();
        let index = SearchIndex::from_document(&doc);

        let results = index.search("voting");
        assert!(!results.is_empty());
        assert!(results.iter().any(|r| r.statute_id == "voting-rights-1"));
    }

    #[test]
    fn test_search_by_multiple_terms() {
        let doc = sample_document();
        let index = SearchIndex::from_document(&doc);

        let results = index.search("voting rights");
        assert!(!results.is_empty());
        assert!(results[0].statute_id == "voting-rights-1");
        assert!(results[0].score > 0.0);
    }

    #[test]
    fn test_search_no_results() {
        let doc = sample_document();
        let index = SearchIndex::from_document(&doc);

        let results = index.search("nonexistent");
        assert!(results.is_empty());
    }

    #[test]
    fn test_find_by_id() {
        let doc = sample_document();
        let index = SearchIndex::from_document(&doc);

        let metadata = index.find_by_id("voting-rights-1");
        assert!(metadata.is_some());
        assert_eq!(metadata.unwrap().title, "Voting Rights Act");
    }

    #[test]
    fn test_search_by_title() {
        let doc = sample_document();
        let index = SearchIndex::from_document(&doc);

        let results = index.search_by_title("tax");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "tax-exemption-1");
    }

    #[test]
    fn test_index_stats() {
        let doc = sample_document();
        let index = SearchIndex::from_document(&doc);

        let stats = index.index_stats();
        assert_eq!(stats.total_statutes, 2);
        assert!(stats.total_terms > 0);
        assert!(stats.avg_terms_per_statute > 0.0);
    }

    #[test]
    fn test_tokenization() {
        let index = SearchIndex::new();
        let tokens = index.tokenize("Hello, World! This is a test-case.");

        assert!(tokens.contains(&"hello".to_string()));
        assert!(tokens.contains(&"world".to_string()));
        assert!(tokens.contains(&"test-case".to_string()));
        assert!(!tokens.contains(&"a".to_string())); // Single-char filtered out
    }

    #[test]
    fn test_search_case_insensitive() {
        let doc = sample_document();
        let index = SearchIndex::from_document(&doc);

        let results1 = index.search("VOTING");
        let results2 = index.search("voting");
        let results3 = index.search("VoTiNg");

        assert_eq!(results1.len(), results2.len());
        assert_eq!(results2.len(), results3.len());
    }
}
