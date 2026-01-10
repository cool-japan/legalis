//! Full-text search across audit records.
//!
//! This module provides full-text search capabilities for querying audit trails:
//! - Search across decision context, attributes, and metadata
//! - Boolean queries (AND, OR, NOT)
//! - Phrase matching
//! - Fuzzy matching with edit distance
//! - Relevance scoring

use crate::{AuditRecord, AuditResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Search query.
#[derive(Debug, Clone)]
pub struct SearchQuery {
    /// Terms to search for
    terms: Vec<SearchTerm>,
    /// Minimum score threshold (0.0 to 1.0)
    min_score: f64,
    /// Maximum results to return
    limit: usize,
}

/// A single search term.
#[derive(Debug, Clone)]
pub enum SearchTerm {
    /// Exact text match
    Exact(String),
    /// Fuzzy match with max edit distance
    Fuzzy { text: String, max_distance: usize },
    /// Phrase match
    Phrase(Vec<String>),
    /// AND combination
    And(Vec<SearchTerm>),
    /// OR combination
    Or(Vec<SearchTerm>),
    /// NOT (negation)
    Not(Box<SearchTerm>),
}

impl SearchQuery {
    /// Creates a new search query.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            terms: vec![SearchTerm::Exact(text.into())],
            min_score: 0.0,
            limit: 100,
        }
    }

    /// Creates a fuzzy search query.
    pub fn fuzzy(text: impl Into<String>, max_distance: usize) -> Self {
        Self {
            terms: vec![SearchTerm::Fuzzy {
                text: text.into(),
                max_distance,
            }],
            min_score: 0.0,
            limit: 100,
        }
    }

    /// Creates a phrase search query.
    pub fn phrase(words: Vec<String>) -> Self {
        Self {
            terms: vec![SearchTerm::Phrase(words)],
            min_score: 0.0,
            limit: 100,
        }
    }

    /// Sets the minimum score threshold.
    pub fn min_score(mut self, score: f64) -> Self {
        self.min_score = score.clamp(0.0, 1.0);
        self
    }

    /// Sets the maximum number of results.
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    /// Adds an AND term.
    pub fn and(mut self, other: SearchQuery) -> Self {
        let new_term = SearchTerm::And(vec![
            SearchTerm::And(self.terms),
            SearchTerm::And(other.terms),
        ]);
        self.terms = vec![new_term];
        self
    }

    /// Adds an OR term.
    pub fn or(mut self, other: SearchQuery) -> Self {
        let new_term = SearchTerm::Or(vec![
            SearchTerm::Or(self.terms),
            SearchTerm::Or(other.terms),
        ]);
        self.terms = vec![new_term];
        self
    }

    /// Negates the query.
    #[allow(clippy::should_implement_trait)]
    pub fn not(mut self) -> Self {
        let existing = if self.terms.len() == 1 {
            self.terms.into_iter().next().unwrap()
        } else {
            SearchTerm::And(self.terms)
        };
        self.terms = vec![SearchTerm::Not(Box::new(existing))];
        self
    }

    /// Executes the search query.
    pub fn execute(&self, records: &[AuditRecord]) -> AuditResult<Vec<SearchResult>> {
        let mut results: Vec<SearchResult> = records
            .iter()
            .filter_map(|record| {
                let score = self.score_record(record);
                if score > 0.0 && score >= self.min_score {
                    Some(SearchResult {
                        record_id: record.id,
                        score,
                        matches: self.find_matches(record),
                    })
                } else {
                    None
                }
            })
            .collect();

        // Sort by score descending
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        // Limit results
        results.truncate(self.limit);

        Ok(results)
    }

    /// Scores a record against the query.
    fn score_record(&self, record: &AuditRecord) -> f64 {
        let text = self.record_to_searchable_text(record);
        let mut total_score = 0.0;

        for term in &self.terms {
            total_score += self.score_term(term, &text);
        }

        // Normalize to 0.0-1.0
        (total_score / self.terms.len() as f64).min(1.0)
    }

    /// Scores a search term against text.
    #[allow(clippy::only_used_in_recursion)]
    fn score_term(&self, term: &SearchTerm, text: &str) -> f64 {
        match term {
            SearchTerm::Exact(query) => {
                if text.to_lowercase().contains(&query.to_lowercase()) {
                    1.0
                } else {
                    0.0
                }
            }
            SearchTerm::Fuzzy {
                text: query,
                max_distance,
            } => {
                let words: Vec<&str> = text.split_whitespace().collect();
                let mut best_score: f64 = 0.0;

                for word in words {
                    let distance = levenshtein_distance(word, query);
                    if distance <= *max_distance {
                        let score = 1.0 - (distance as f64 / query.len().max(1) as f64);
                        best_score = best_score.max(score);
                    }
                }

                best_score
            }
            SearchTerm::Phrase(words) => {
                let phrase = words.join(" ").to_lowercase();
                if text.to_lowercase().contains(&phrase) {
                    1.0
                } else {
                    0.0
                }
            }
            SearchTerm::And(terms) => {
                let scores: Vec<f64> = terms.iter().map(|t| self.score_term(t, text)).collect();
                if scores.iter().all(|&s| s > 0.0) {
                    scores.iter().sum::<f64>() / scores.len() as f64
                } else {
                    0.0
                }
            }
            SearchTerm::Or(terms) => terms
                .iter()
                .map(|t| self.score_term(t, text))
                .fold(0.0, f64::max),
            SearchTerm::Not(term) => {
                let score = self.score_term(term, text);
                if score == 0.0 { 1.0 } else { 0.0 }
            }
        }
    }

    /// Finds matching snippets in a record.
    fn find_matches(&self, record: &AuditRecord) -> Vec<String> {
        let text = self.record_to_searchable_text(record);
        let mut matches = Vec::new();

        for term in &self.terms {
            if let Some(snippet) = self.find_match_snippet(term, &text) {
                matches.push(snippet);
            }
        }

        matches
    }

    /// Finds a matching snippet for a term.
    fn find_match_snippet(&self, term: &SearchTerm, text: &str) -> Option<String> {
        match term {
            SearchTerm::Exact(query) => {
                let lower_text = text.to_lowercase();
                let lower_query = query.to_lowercase();
                if let Some(pos) = lower_text.find(&lower_query) {
                    let start = pos.saturating_sub(20);
                    let end = (pos + query.len() + 20).min(text.len());
                    Some(text[start..end].to_string())
                } else {
                    None
                }
            }
            SearchTerm::Phrase(words) => {
                let phrase = words.join(" ");
                let lower_text = text.to_lowercase();
                let lower_phrase = phrase.to_lowercase();
                if let Some(pos) = lower_text.find(&lower_phrase) {
                    let start = pos.saturating_sub(20);
                    let end = (pos + phrase.len() + 20).min(text.len());
                    Some(text[start..end].to_string())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Converts a record to searchable text.
    fn record_to_searchable_text(&self, record: &AuditRecord) -> String {
        let mut parts = Vec::new();

        // Include statute ID
        parts.push(record.statute_id.clone());

        // Include subject ID
        parts.push(record.subject_id.to_string());

        // Include actor information
        match &record.actor {
            crate::Actor::System { component } => parts.push(component.clone()),
            crate::Actor::User { user_id, role } => {
                parts.push(user_id.clone());
                parts.push(role.clone());
            }
            crate::Actor::External { system_id } => parts.push(system_id.clone()),
        }

        // Include context attributes
        for (key, value) in &record.context.attributes {
            parts.push(key.clone());
            parts.push(value.clone());
        }

        // Include context metadata
        for (key, value) in &record.context.metadata {
            parts.push(key.clone());
            parts.push(value.clone());
        }

        // Include result information
        match &record.result {
            crate::DecisionResult::Deterministic {
                effect_applied,
                parameters,
            } => {
                parts.push(effect_applied.clone());
                for (key, value) in parameters {
                    parts.push(key.clone());
                    parts.push(value.clone());
                }
            }
            crate::DecisionResult::RequiresDiscretion {
                issue,
                narrative_hint,
                ..
            } => {
                parts.push(issue.clone());
                if let Some(hint) = narrative_hint {
                    parts.push(hint.clone());
                }
            }
            crate::DecisionResult::Void { reason } => {
                parts.push(reason.clone());
            }
            crate::DecisionResult::Overridden { justification, .. } => {
                parts.push(justification.clone());
            }
        }

        parts.join(" ")
    }
}

/// A search result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Record ID
    pub record_id: Uuid,
    /// Relevance score (0.0 to 1.0)
    pub score: f64,
    /// Matching snippets
    pub matches: Vec<String>,
}

/// Calculates Levenshtein distance between two strings.
fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_len = a.chars().count();
    let b_len = b.chars().count();

    if a_len == 0 {
        return b_len;
    }
    if b_len == 0 {
        return a_len;
    }

    let mut matrix = vec![vec![0; b_len + 1]; a_len + 1];

    for (i, row) in matrix.iter_mut().enumerate().take(a_len + 1) {
        row[0] = i;
    }
    for (j, cell) in matrix[0].iter_mut().enumerate().take(b_len + 1) {
        *cell = j;
    }

    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();

    for (i, a_char) in a_chars.iter().enumerate() {
        for (j, b_char) in b_chars.iter().enumerate() {
            let cost = if a_char == b_char { 0 } else { 1 };
            matrix[i + 1][j + 1] = (matrix[i][j + 1] + 1)
                .min(matrix[i + 1][j] + 1)
                .min(matrix[i][j] + cost);
        }
    }

    matrix[a_len][b_len]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap as StdHashMap;

    fn create_test_record(statute: &str, component: &str) -> AuditRecord {
        let mut context = DecisionContext::default();
        context
            .attributes
            .insert("key1".to_string(), "value1".to_string());
        context.attributes.insert(
            "description".to_string(),
            "test description with important keywords".to_string(),
        );

        AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: component.to_string(),
            },
            statute.to_string(),
            Uuid::new_v4(),
            context,
            DecisionResult::Deterministic {
                effect_applied: "approved".to_string(),
                parameters: StdHashMap::new(),
            },
            None,
        )
    }

    #[test]
    fn test_exact_search() {
        let records = vec![
            create_test_record("statute-1", "engine-alpha"),
            create_test_record("statute-2", "engine-beta"),
        ];

        let query = SearchQuery::new("engine-alpha");
        let results = query.execute(&records).unwrap();

        assert_eq!(results.len(), 1);
        assert!(results[0].score > 0.0);
    }

    #[test]
    fn test_fuzzy_search() {
        let records = vec![create_test_record("statute-1", "engine")];

        let query = SearchQuery::fuzzy("engin", 1); // 1 char difference
        let results = query.execute(&records).unwrap();

        assert!(!results.is_empty());
        assert!(results[0].score > 0.0);
    }

    #[test]
    fn test_phrase_search() {
        let records = vec![create_test_record("statute-1", "test-component")];

        let query = SearchQuery::phrase(vec!["test".to_string(), "description".to_string()]);
        let results = query.execute(&records).unwrap();

        assert!(!results.is_empty());
    }

    #[test]
    fn test_min_score_filter() {
        let records = vec![
            create_test_record("statute-1", "engine-alpha"),
            create_test_record("statute-2", "other-component"),
        ];

        let query = SearchQuery::new("engine-alpha").min_score(0.5);
        let results = query.execute(&records).unwrap();

        assert!(results.iter().all(|r| r.score >= 0.5));
    }

    #[test]
    fn test_limit_results() {
        let records = vec![
            create_test_record("statute-1", "test"),
            create_test_record("statute-2", "test"),
            create_test_record("statute-3", "test"),
        ];

        let query = SearchQuery::new("test").limit(2);
        let results = query.execute(&records).unwrap();

        assert!(results.len() <= 2);
    }

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
        assert_eq!(levenshtein_distance("hello", "hello"), 0);
        assert_eq!(levenshtein_distance("abc", "def"), 3);
    }

    #[test]
    fn test_context_search() {
        let records = vec![create_test_record("statute-1", "component")];

        let query = SearchQuery::new("important keywords");
        let results = query.execute(&records).unwrap();

        assert!(!results.is_empty());
        assert!(results[0].score > 0.0);
    }
}
