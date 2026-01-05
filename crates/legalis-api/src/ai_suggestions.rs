//! AI-powered statute suggestion module.
//!
//! This module provides intelligent statute suggestions using rule-based analysis
//! to help users find relevant statutes based on natural language queries.
//!
//! Future versions will integrate LLM-powered suggestions.

use legalis_core::Statute;
use serde::{Deserialize, Serialize};

/// Request for statute suggestions.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SuggestionRequest {
    /// Natural language query describing the legal scenario
    pub query: String,
    /// Optional jurisdiction filter
    pub jurisdiction: Option<String>,
    /// Optional domain filter
    pub domain: Option<String>,
    /// Maximum number of suggestions to return
    #[serde(default = "default_max_suggestions")]
    pub max_suggestions: usize,
    /// Whether to include reasoning
    #[serde(default)]
    pub include_reasoning: bool,
}

fn default_max_suggestions() -> usize {
    5
}

/// Suggested statute with confidence score.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteSuggestion {
    /// Suggested statute ID
    pub statute_id: String,
    /// Statute title
    pub title: String,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f64,
    /// Reasoning for the suggestion
    pub reasoning: String,
    /// Relevant excerpts or highlights
    pub highlights: Vec<String>,
    /// Tags or categories
    pub tags: Vec<String>,
}

/// Response containing statute suggestions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestionResponse {
    /// List of suggested statutes
    pub suggestions: Vec<StatuteSuggestion>,
    /// Original query
    pub query: String,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
    /// Metadata about the suggestion process
    pub metadata: serde_json::Value,
}

/// Statute suggestion engine using rule-based matching.
#[derive(Default)]
pub struct SuggestionEngine {}

impl SuggestionEngine {
    /// Creates a new suggestion engine.
    pub fn new() -> Self {
        Self {}
    }

    /// Generates statute suggestions based on a natural language query.
    pub async fn suggest(
        &self,
        request: SuggestionRequest,
        available_statutes: &[Statute],
    ) -> Result<SuggestionResponse, String> {
        let start_time = std::time::Instant::now();

        // Use rule-based suggestions
        let suggestions = self.rule_based_suggest(request.clone(), available_statutes)?;

        let processing_time_ms = start_time.elapsed().as_millis() as u64;

        Ok(SuggestionResponse {
            suggestions,
            query: request.query,
            processing_time_ms,
            metadata: serde_json::json!({
                "method": "rule-based",
                "statutes_analyzed": available_statutes.len(),
            }),
        })
    }

    /// Rule-based suggestion generation.
    fn rule_based_suggest(
        &self,
        request: SuggestionRequest,
        available_statutes: &[Statute],
    ) -> Result<Vec<StatuteSuggestion>, String> {
        let query_lower = request.query.to_lowercase();
        let mut suggestions = Vec::new();

        // Simple keyword-based matching
        for statute in available_statutes.iter().take(100) {
            let title_lower = statute.title.to_lowercase();

            // Calculate simple relevance score based on keyword overlap
            let relevance = if title_lower.contains(&query_lower) {
                0.9
            } else {
                // Count matching words
                let query_words: Vec<&str> = query_lower.split_whitespace().collect();
                let matching_words = query_words
                    .iter()
                    .filter(|word| title_lower.contains(*word))
                    .count();

                if matching_words > 0 {
                    (matching_words as f64 / query_words.len() as f64) * 0.7
                } else {
                    0.0
                }
            };

            if relevance > 0.3 {
                suggestions.push(StatuteSuggestion {
                    statute_id: statute.id.clone(),
                    title: statute.title.clone(),
                    confidence: relevance,
                    reasoning: format!("Matched based on keyword similarity ({})", relevance),
                    highlights: vec![statute.title.clone()],
                    tags: vec!["keyword-match".to_string()],
                });
            }
        }

        // Sort by confidence and take top N
        suggestions.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        suggestions.truncate(request.max_suggestions);

        Ok(suggestions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType, Statute};

    #[tokio::test]
    async fn test_rule_based_suggestion() {
        let engine = SuggestionEngine::new();

        let statutes = vec![
            Statute::new(
                "test-1",
                "Contract Formation Rules",
                Effect::new(EffectType::Grant, "contract_formation"),
            ),
            Statute::new(
                "test-2",
                "Tort Liability Standards",
                Effect::new(EffectType::Grant, "tort_liability"),
            ),
        ];

        let request = SuggestionRequest {
            query: "contract".to_string(),
            jurisdiction: None,
            domain: None,
            max_suggestions: 5,
            include_reasoning: true,
        };

        let response = engine.suggest(request, &statutes).await.unwrap();

        assert!(!response.suggestions.is_empty());
        assert_eq!(response.query, "contract");
        // processing_time_ms is u64, always >= 0, so no need to check
    }

    #[test]
    fn test_suggestion_request_default() {
        let request = SuggestionRequest {
            query: "test".to_string(),
            jurisdiction: None,
            domain: None,
            max_suggestions: default_max_suggestions(),
            include_reasoning: false,
        };

        assert_eq!(request.max_suggestions, 5);
    }
}
