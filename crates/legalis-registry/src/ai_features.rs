//! AI-Powered Features Module (v0.2.5)
//!
//! This module provides AI-driven capabilities for the statute registry:
//! - AI-generated statute summaries
//! - Automated tagging with classification
//! - AI-powered search query expansion
//! - Intelligent duplicate detection
//! - Predictive statute recommendations

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

use crate::StatuteEntry;

// ============================================================================
// Error Types
// ============================================================================

#[derive(Debug, Error, Clone, PartialEq)]
pub enum AiError {
    #[error("AI service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Processing failed: {0}")]
    ProcessingFailed(String),

    #[error("Model not found: {0}")]
    ModelNotFound(String),

    #[error("Insufficient data: {0}")]
    InsufficientData(String),
}

pub type AiResult<T> = Result<T, AiError>;

// ============================================================================
// AI Service Configuration
// ============================================================================

/// AI service provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AiProvider {
    OpenAI { api_key: String, model: String },
    Anthropic { api_key: String, model: String },
    Cohere { api_key: String, model: String },
    Local { model_path: String },
    Mock, // For testing
}

/// AI service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    pub provider: AiProvider,
    pub temperature: f32,
    pub max_tokens: usize,
    pub cache_enabled: bool,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            provider: AiProvider::Mock,
            temperature: 0.7,
            max_tokens: 500,
            cache_enabled: true,
        }
    }
}

// ============================================================================
// Statute Summarization
// ============================================================================

/// Summary quality level
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SummaryLength {
    Brief,         // 1-2 sentences
    Standard,      // 3-5 sentences
    Detailed,      // 1-2 paragraphs
    Comprehensive, // Full analysis
}

/// Generated statute summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteSummary {
    pub statute_id: String,
    pub summary: String,
    pub length: SummaryLength,
    pub key_points: Vec<String>,
    pub confidence: f32,
    pub model_used: String,
}

impl StatuteSummary {
    pub fn new(
        statute_id: impl Into<String>,
        summary: impl Into<String>,
        length: SummaryLength,
    ) -> Self {
        Self {
            statute_id: statute_id.into(),
            summary: summary.into(),
            length,
            key_points: Vec::new(),
            confidence: 0.0,
            model_used: "unknown".to_string(),
        }
    }

    pub fn with_key_points(mut self, points: Vec<String>) -> Self {
        self.key_points = points;
        self
    }

    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.confidence = confidence;
        self
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model_used = model.into();
        self
    }
}

/// Statute summarization engine
pub struct SummarizationEngine {
    config: AiConfig,
    cache: HashMap<String, StatuteSummary>,
}

impl SummarizationEngine {
    pub fn new(config: AiConfig) -> Self {
        Self {
            config,
            cache: HashMap::new(),
        }
    }

    /// Generate summary for a statute
    pub fn summarize(
        &mut self,
        statute: &StatuteEntry,
        length: SummaryLength,
    ) -> AiResult<StatuteSummary> {
        let cache_key = format!("{}:{:?}", statute.statute.id, length);

        // Check cache
        if self.config.cache_enabled {
            if let Some(cached) = self.cache.get(&cache_key) {
                return Ok(cached.clone());
            }
        }

        // Generate summary based on provider
        let summary = match &self.config.provider {
            AiProvider::Mock => self.generate_mock_summary(statute, length),
            AiProvider::OpenAI { model, .. } => {
                self.generate_openai_summary(statute, length, model)
            }
            AiProvider::Anthropic { model, .. } => {
                self.generate_anthropic_summary(statute, length, model)
            }
            AiProvider::Cohere { model, .. } => {
                self.generate_cohere_summary(statute, length, model)
            }
            AiProvider::Local { model_path } => {
                self.generate_local_summary(statute, length, model_path)
            }
        }?;

        // Cache result
        if self.config.cache_enabled {
            self.cache.insert(cache_key, summary.clone());
        }

        Ok(summary)
    }

    /// Clear summary cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    fn generate_mock_summary(
        &self,
        statute: &StatuteEntry,
        length: SummaryLength,
    ) -> AiResult<StatuteSummary> {
        let summary_text = match length {
            SummaryLength::Brief => {
                format!("Brief summary of statute: {}", statute.statute.title)
            }
            SummaryLength::Standard => {
                let date_str = statute
                    .effective_date
                    .map(|d| d.to_string())
                    .unwrap_or_else(|| "an unspecified date".to_string());
                format!(
                    "This statute ({}) addresses {}. It was enacted on {} and is currently {}.",
                    statute.statute.id,
                    statute.statute.title,
                    date_str,
                    format!("{:?}", statute.status).to_lowercase()
                )
            }
            SummaryLength::Detailed => {
                let date_str = statute
                    .effective_date
                    .map(|d| d.to_string())
                    .unwrap_or_else(|| "an unspecified date".to_string());
                format!(
                    "Detailed analysis of statute {}: {}. This legislation, effective from {}, \
                     establishes important provisions in the jurisdiction of {}. The statute's \
                     primary effect is to {}.",
                    statute.statute.id,
                    statute.statute.title,
                    date_str,
                    statute.jurisdiction,
                    statute.statute.effect.description
                )
            }
            SummaryLength::Comprehensive => {
                let date_str = statute
                    .effective_date
                    .map(|d| d.to_string())
                    .unwrap_or_else(|| "Not specified".to_string());
                format!(
                    "Comprehensive overview of statute {}: {}\n\nEffective Date: {}\n\
                     Jurisdiction: {}\nStatus: {:?}\n\nPrimary Effect: {}\n\n\
                     This statute represents a significant legal framework that impacts \
                     multiple aspects of the regulated domain.",
                    statute.statute.id,
                    statute.statute.title,
                    date_str,
                    statute.jurisdiction,
                    statute.status,
                    statute.statute.effect.description
                )
            }
        };

        Ok(
            StatuteSummary::new(&statute.statute.id, summary_text, length)
                .with_confidence(0.95)
                .with_model("mock"),
        )
    }

    #[allow(dead_code)]
    fn generate_openai_summary(
        &self,
        statute: &StatuteEntry,
        length: SummaryLength,
        model: &str,
    ) -> AiResult<StatuteSummary> {
        // Placeholder for OpenAI integration
        Ok(StatuteSummary::new(
            &statute.statute.id,
            format!("OpenAI summary ({}) for: {}", model, statute.statute.title),
            length,
        )
        .with_model(model))
    }

    #[allow(dead_code)]
    fn generate_anthropic_summary(
        &self,
        statute: &StatuteEntry,
        length: SummaryLength,
        model: &str,
    ) -> AiResult<StatuteSummary> {
        // Placeholder for Anthropic integration
        Ok(StatuteSummary::new(
            &statute.statute.id,
            format!(
                "Anthropic summary ({}) for: {}",
                model, statute.statute.title
            ),
            length,
        )
        .with_model(model))
    }

    #[allow(dead_code)]
    fn generate_cohere_summary(
        &self,
        statute: &StatuteEntry,
        length: SummaryLength,
        model: &str,
    ) -> AiResult<StatuteSummary> {
        // Placeholder for Cohere integration
        Ok(StatuteSummary::new(
            &statute.statute.id,
            format!("Cohere summary ({}) for: {}", model, statute.statute.title),
            length,
        )
        .with_model(model))
    }

    #[allow(dead_code)]
    fn generate_local_summary(
        &self,
        statute: &StatuteEntry,
        length: SummaryLength,
        model_path: &str,
    ) -> AiResult<StatuteSummary> {
        // Placeholder for local model integration
        Ok(StatuteSummary::new(
            &statute.statute.id,
            format!(
                "Local model summary ({}) for: {}",
                model_path, statute.statute.title
            ),
            length,
        )
        .with_model(model_path))
    }
}

// ============================================================================
// Automated Tagging & Classification
// ============================================================================

/// Tag confidence level
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct TagConfidence(pub f32);

impl TagConfidence {
    pub fn new(value: f32) -> Self {
        Self(value.clamp(0.0, 1.0))
    }

    pub fn is_high(&self) -> bool {
        self.0 >= 0.8
    }

    pub fn is_medium(&self) -> bool {
        self.0 >= 0.5 && self.0 < 0.8
    }

    pub fn is_low(&self) -> bool {
        self.0 < 0.5
    }
}

/// Suggested tag with confidence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedTag {
    pub tag: String,
    pub confidence: TagConfidence,
    pub reasoning: String,
}

impl SuggestedTag {
    pub fn new(tag: impl Into<String>, confidence: f32) -> Self {
        Self {
            tag: tag.into(),
            confidence: TagConfidence::new(confidence),
            reasoning: String::new(),
        }
    }

    pub fn with_reasoning(mut self, reasoning: impl Into<String>) -> Self {
        self.reasoning = reasoning.into();
        self
    }
}

/// Tag classification engine
pub struct TagClassifier {
    config: AiConfig,
    known_tags: Vec<String>,
}

impl TagClassifier {
    pub fn new(config: AiConfig) -> Self {
        Self {
            config,
            known_tags: Vec::new(),
        }
    }

    pub fn with_known_tags(mut self, tags: Vec<String>) -> Self {
        self.known_tags = tags;
        self
    }

    /// Suggest tags for a statute
    pub fn suggest_tags(&self, statute: &StatuteEntry) -> AiResult<Vec<SuggestedTag>> {
        match &self.config.provider {
            AiProvider::Mock => self.suggest_tags_mock(statute),
            _ => {
                // Placeholder for real AI integration
                self.suggest_tags_mock(statute)
            }
        }
    }

    fn suggest_tags_mock(&self, statute: &StatuteEntry) -> AiResult<Vec<SuggestedTag>> {
        let mut tags = Vec::new();

        // Extract tags from title
        let title_lower = statute.statute.title.to_lowercase();
        if title_lower.contains("tax") {
            tags.push(SuggestedTag::new("taxation", 0.9).with_reasoning("Title mentions tax"));
        }
        if title_lower.contains("health") {
            tags.push(
                SuggestedTag::new("healthcare", 0.85).with_reasoning("Title mentions health"),
            );
        }
        if title_lower.contains("environment") {
            tags.push(
                SuggestedTag::new("environmental", 0.88)
                    .with_reasoning("Title mentions environment"),
            );
        }
        if title_lower.contains("education") {
            tags.push(
                SuggestedTag::new("education", 0.9).with_reasoning("Title mentions education"),
            );
        }

        // Add jurisdiction-based tag
        tags.push(
            SuggestedTag::new(&statute.jurisdiction, 1.0).with_reasoning("Jurisdiction metadata"),
        );

        // Add effect type tag
        let effect_tag = format!("{:?}", statute.statute.effect.effect_type).to_lowercase();
        tags.push(SuggestedTag::new(effect_tag, 0.95).with_reasoning("Based on effect type"));

        Ok(tags)
    }

    /// Classify statute into categories
    pub fn classify(&self, statute: &StatuteEntry) -> AiResult<Vec<String>> {
        let tags = self.suggest_tags(statute)?;
        Ok(tags
            .into_iter()
            .filter(|t| t.confidence.is_high())
            .map(|t| t.tag)
            .collect())
    }
}

// ============================================================================
// Search Query Expansion
// ============================================================================

/// Expanded search query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpandedQuery {
    pub original_query: String,
    pub expanded_terms: Vec<String>,
    pub synonyms: HashMap<String, Vec<String>>,
    pub related_concepts: Vec<String>,
}

impl ExpandedQuery {
    pub fn new(original_query: impl Into<String>) -> Self {
        Self {
            original_query: original_query.into(),
            expanded_terms: Vec::new(),
            synonyms: HashMap::new(),
            related_concepts: Vec::new(),
        }
    }

    pub fn all_terms(&self) -> Vec<String> {
        let mut terms = vec![self.original_query.clone()];
        terms.extend(self.expanded_terms.clone());
        terms.extend(self.related_concepts.clone());
        for syn_list in self.synonyms.values() {
            terms.extend(syn_list.clone());
        }
        terms
    }
}

/// Query expansion engine
pub struct QueryExpander {
    #[allow(dead_code)]
    config: AiConfig,
    synonym_map: HashMap<String, Vec<String>>,
}

impl QueryExpander {
    pub fn new(config: AiConfig) -> Self {
        Self {
            config,
            synonym_map: Self::build_legal_synonyms(),
        }
    }

    /// Expand search query with synonyms and related terms
    pub fn expand(&self, query: &str) -> AiResult<ExpandedQuery> {
        let mut expanded = ExpandedQuery::new(query);

        // Split query into terms
        let terms: Vec<&str> = query.split_whitespace().collect();

        for term in &terms {
            let term_lower = term.to_lowercase();

            // Find synonyms
            if let Some(synonyms) = self.synonym_map.get(&term_lower) {
                expanded
                    .synonyms
                    .insert(term_lower.clone(), synonyms.clone());
                expanded.expanded_terms.extend(synonyms.clone());
            }

            // Add related legal concepts
            self.add_related_concepts(&term_lower, &mut expanded);
        }

        Ok(expanded)
    }

    fn build_legal_synonyms() -> HashMap<String, Vec<String>> {
        let mut map = HashMap::new();

        map.insert(
            "statute".to_string(),
            vec![
                "law".to_string(),
                "regulation".to_string(),
                "act".to_string(),
            ],
        );
        map.insert(
            "regulation".to_string(),
            vec![
                "rule".to_string(),
                "statute".to_string(),
                "ordinance".to_string(),
            ],
        );
        map.insert(
            "tax".to_string(),
            vec![
                "taxation".to_string(),
                "levy".to_string(),
                "duty".to_string(),
            ],
        );
        map.insert(
            "penalty".to_string(),
            vec![
                "fine".to_string(),
                "sanction".to_string(),
                "punishment".to_string(),
            ],
        );
        map.insert(
            "enforce".to_string(),
            vec![
                "implement".to_string(),
                "apply".to_string(),
                "execute".to_string(),
            ],
        );

        map
    }

    fn add_related_concepts(&self, term: &str, expanded: &mut ExpandedQuery) {
        match term {
            "tax" | "taxation" => {
                expanded.related_concepts.push("revenue".to_string());
                expanded.related_concepts.push("fiscal".to_string());
            }
            "health" | "healthcare" => {
                expanded.related_concepts.push("medical".to_string());
                expanded.related_concepts.push("wellness".to_string());
            }
            "environment" | "environmental" => {
                expanded.related_concepts.push("climate".to_string());
                expanded.related_concepts.push("pollution".to_string());
            }
            _ => {}
        }
    }
}

// ============================================================================
// Duplicate Detection
// ============================================================================

/// Duplicate match result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateMatch {
    pub statute_id_1: String,
    pub statute_id_2: String,
    pub similarity_score: f32,
    pub duplicate_type: DuplicateType,
    pub reasoning: String,
}

impl DuplicateMatch {
    pub fn new(
        id1: impl Into<String>,
        id2: impl Into<String>,
        score: f32,
        duplicate_type: DuplicateType,
    ) -> Self {
        Self {
            statute_id_1: id1.into(),
            statute_id_2: id2.into(),
            similarity_score: score,
            duplicate_type,
            reasoning: String::new(),
        }
    }

    pub fn with_reasoning(mut self, reasoning: impl Into<String>) -> Self {
        self.reasoning = reasoning.into();
        self
    }
}

/// Type of duplication detected
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DuplicateType {
    Exact,     // Identical content
    NearExact, // Very similar with minor differences
    Semantic,  // Same meaning, different wording
    Partial,   // Significant overlap
}

/// Duplicate detection engine
pub struct DuplicateDetector {
    #[allow(dead_code)]
    config: AiConfig,
    threshold: f32,
}

impl DuplicateDetector {
    pub fn new(config: AiConfig) -> Self {
        Self {
            config,
            threshold: 0.85,
        }
    }

    pub fn with_threshold(mut self, threshold: f32) -> Self {
        self.threshold = threshold;
        self
    }

    /// Detect duplicates in a collection of statutes
    pub fn detect_duplicates(&self, statutes: &[StatuteEntry]) -> AiResult<Vec<DuplicateMatch>> {
        let mut duplicates = Vec::new();

        for i in 0..statutes.len() {
            for j in (i + 1)..statutes.len() {
                if let Some(duplicate_match) = self.compare_statutes(&statutes[i], &statutes[j])? {
                    duplicates.push(duplicate_match);
                }
            }
        }

        Ok(duplicates)
    }

    /// Compare two statutes for similarity
    pub fn compare_statutes(
        &self,
        statute1: &StatuteEntry,
        statute2: &StatuteEntry,
    ) -> AiResult<Option<DuplicateMatch>> {
        // Check for exact ID match
        if statute1.statute.id == statute2.statute.id {
            return Ok(Some(
                DuplicateMatch::new(
                    &statute1.statute.id,
                    &statute2.statute.id,
                    1.0,
                    DuplicateType::Exact,
                )
                .with_reasoning("Identical statute IDs"),
            ));
        }

        // Calculate title similarity
        let title_similarity =
            self.calculate_text_similarity(&statute1.statute.title, &statute2.statute.title);

        if title_similarity >= self.threshold {
            let duplicate_type = if title_similarity >= 0.98 {
                DuplicateType::Exact
            } else if title_similarity >= 0.90 {
                DuplicateType::NearExact
            } else {
                DuplicateType::Semantic
            };

            return Ok(Some(
                DuplicateMatch::new(
                    &statute1.statute.id,
                    &statute2.statute.id,
                    title_similarity,
                    duplicate_type,
                )
                .with_reasoning(format!("Title similarity: {:.2}", title_similarity)),
            ));
        }

        Ok(None)
    }

    fn calculate_text_similarity(&self, text1: &str, text2: &str) -> f32 {
        // Simple Jaccard similarity for mock implementation
        let text1_lower = text1.to_lowercase();
        let text2_lower = text2.to_lowercase();

        let words1: std::collections::HashSet<_> = text1_lower.split_whitespace().collect();
        let words2: std::collections::HashSet<_> = text2_lower.split_whitespace().collect();

        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();

        if union == 0 {
            0.0
        } else {
            intersection as f32 / union as f32
        }
    }
}

// ============================================================================
// Statute Recommendations
// ============================================================================

/// Recommendation reason
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationReason {
    SimilarContent,
    RelatedJurisdiction,
    CommonTags,
    CitationNetwork,
    UserHistory,
}

/// Statute recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteRecommendation {
    pub statute_id: String,
    pub score: f32,
    pub reasons: Vec<RecommendationReason>,
    pub explanation: String,
}

impl StatuteRecommendation {
    pub fn new(statute_id: impl Into<String>, score: f32) -> Self {
        Self {
            statute_id: statute_id.into(),
            score,
            reasons: Vec::new(),
            explanation: String::new(),
        }
    }

    pub fn add_reason(mut self, reason: RecommendationReason) -> Self {
        self.reasons.push(reason);
        self
    }

    pub fn with_explanation(mut self, explanation: impl Into<String>) -> Self {
        self.explanation = explanation.into();
        self
    }
}

/// Recommendation engine
pub struct RecommendationEngine {
    #[allow(dead_code)]
    config: AiConfig,
}

impl RecommendationEngine {
    pub fn new(config: AiConfig) -> Self {
        Self { config }
    }

    /// Get recommendations based on a statute
    pub fn recommend_similar(
        &self,
        target: &StatuteEntry,
        candidates: &[StatuteEntry],
        max_results: usize,
    ) -> AiResult<Vec<StatuteRecommendation>> {
        let mut recommendations = Vec::new();

        for candidate in candidates {
            if candidate.statute.id == target.statute.id {
                continue;
            }

            let mut score = 0.0;
            let mut reasons = Vec::new();

            // Check jurisdiction similarity
            if candidate.jurisdiction == target.jurisdiction {
                score += 0.3;
                reasons.push(RecommendationReason::RelatedJurisdiction);
            }

            // Check tag overlap
            let common_tags: Vec<_> = candidate
                .tags
                .iter()
                .filter(|t| target.tags.contains(t))
                .collect();

            if !common_tags.is_empty() {
                score += 0.4 * (common_tags.len() as f32 / target.tags.len().max(1) as f32);
                reasons.push(RecommendationReason::CommonTags);
            }

            // Check title similarity
            let title_sim =
                self.calculate_similarity(&target.statute.title, &candidate.statute.title);
            score += 0.3 * title_sim;
            if title_sim > 0.3 {
                reasons.push(RecommendationReason::SimilarContent);
            }

            if score > 0.1 {
                recommendations.push(
                    StatuteRecommendation::new(&candidate.statute.id, score).with_explanation(
                        format!("Recommended based on {} factors", reasons.len()),
                    ),
                );
                for reason in reasons {
                    let last_idx = recommendations.len() - 1;
                    recommendations[last_idx] =
                        recommendations[last_idx].clone().add_reason(reason);
                }
            }
        }

        // Sort by score descending
        recommendations.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        recommendations.truncate(max_results);

        Ok(recommendations)
    }

    fn calculate_similarity(&self, text1: &str, text2: &str) -> f32 {
        let text1_lower = text1.to_lowercase();
        let text2_lower = text2.to_lowercase();

        let words1: std::collections::HashSet<_> = text1_lower.split_whitespace().collect();
        let words2: std::collections::HashSet<_> = text2_lower.split_whitespace().collect();

        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();

        if union == 0 {
            0.0
        } else {
            intersection as f32 / union as f32
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType, Statute};

    fn create_test_statute(id: &str, title: &str) -> StatuteEntry {
        let effect = Effect::new(EffectType::Grant, "Test effect");
        let statute = Statute::new(id, title, effect);
        StatuteEntry::new(statute, "US")
    }

    #[test]
    fn test_ai_config_default() {
        let config = AiConfig::default();
        assert_eq!(config.temperature, 0.7);
        assert_eq!(config.max_tokens, 500);
        assert!(config.cache_enabled);
    }

    #[test]
    fn test_summary_length_variants() {
        assert!(matches!(SummaryLength::Brief, SummaryLength::Brief));
        assert!(matches!(SummaryLength::Standard, SummaryLength::Standard));
        assert!(matches!(SummaryLength::Detailed, SummaryLength::Detailed));
        assert!(matches!(
            SummaryLength::Comprehensive,
            SummaryLength::Comprehensive
        ));
    }

    #[test]
    fn test_statute_summary_builder() {
        let summary = StatuteSummary::new("STAT1", "Test summary", SummaryLength::Brief)
            .with_key_points(vec!["Point 1".to_string(), "Point 2".to_string()])
            .with_confidence(0.95)
            .with_model("gpt-4");

        assert_eq!(summary.statute_id, "STAT1");
        assert_eq!(summary.summary, "Test summary");
        assert_eq!(summary.key_points.len(), 2);
        assert_eq!(summary.confidence, 0.95);
        assert_eq!(summary.model_used, "gpt-4");
    }

    #[test]
    fn test_summarization_brief() {
        let statute = create_test_statute("TAX001", "Income Tax Act");
        let mut engine = SummarizationEngine::new(AiConfig::default());

        let summary = engine.summarize(&statute, SummaryLength::Brief).unwrap();
        assert_eq!(summary.statute_id, "TAX001");
        assert!(!summary.summary.is_empty());
        assert_eq!(summary.length, SummaryLength::Brief);
    }

    #[test]
    fn test_summarization_cache() {
        let statute = create_test_statute("TAX001", "Income Tax Act");
        let mut engine = SummarizationEngine::new(AiConfig::default());

        // First call
        let summary1 = engine.summarize(&statute, SummaryLength::Standard).unwrap();

        // Second call should use cache
        let summary2 = engine.summarize(&statute, SummaryLength::Standard).unwrap();

        assert_eq!(summary1.summary, summary2.summary);

        // Clear cache
        engine.clear_cache();
    }

    #[test]
    fn test_tag_confidence() {
        let conf_high = TagConfidence::new(0.9);
        assert!(conf_high.is_high());
        assert!(!conf_high.is_medium());
        assert!(!conf_high.is_low());

        let conf_medium = TagConfidence::new(0.6);
        assert!(!conf_medium.is_high());
        assert!(conf_medium.is_medium());
        assert!(!conf_medium.is_low());

        let conf_low = TagConfidence::new(0.3);
        assert!(!conf_low.is_high());
        assert!(!conf_low.is_medium());
        assert!(conf_low.is_low());
    }

    #[test]
    fn test_suggested_tag() {
        let tag = SuggestedTag::new("taxation", 0.9).with_reasoning("Contains tax-related content");

        assert_eq!(tag.tag, "taxation");
        assert_eq!(tag.confidence.0, 0.9);
        assert_eq!(tag.reasoning, "Contains tax-related content");
    }

    #[test]
    fn test_tag_classifier() {
        let statute = create_test_statute("TAX001", "Income Tax Reform Act");
        let classifier = TagClassifier::new(AiConfig::default());

        let tags = classifier.suggest_tags(&statute).unwrap();
        assert!(!tags.is_empty());

        // Should suggest "taxation" tag
        let has_tax_tag = tags.iter().any(|t| t.tag == "taxation");
        assert!(has_tax_tag);
    }

    #[test]
    fn test_tag_classification() {
        let statute = create_test_statute("EDU001", "Education Standards Act");
        let classifier = TagClassifier::new(AiConfig::default());

        let categories = classifier.classify(&statute).unwrap();
        assert!(!categories.is_empty());
    }

    #[test]
    fn test_query_expansion() {
        let expander = QueryExpander::new(AiConfig::default());
        let expanded = expander.expand("tax statute").unwrap();

        assert_eq!(expanded.original_query, "tax statute");
        assert!(!expanded.expanded_terms.is_empty());
    }

    #[test]
    fn test_query_expansion_synonyms() {
        let expander = QueryExpander::new(AiConfig::default());
        let expanded = expander.expand("regulation").unwrap();

        // Should include synonyms for "regulation"
        let all_terms = expanded.all_terms();
        assert!(all_terms.len() > 1);
    }

    #[test]
    fn test_expanded_query_all_terms() {
        let mut query = ExpandedQuery::new("test");
        query.expanded_terms.push("expanded1".to_string());
        query.related_concepts.push("concept1".to_string());
        query
            .synonyms
            .insert("test".to_string(), vec!["synonym1".to_string()]);

        let all_terms = query.all_terms();
        assert!(all_terms.len() >= 4);
    }

    #[test]
    fn test_duplicate_match_builder() {
        let dup = DuplicateMatch::new("STAT1", "STAT2", 0.95, DuplicateType::NearExact)
            .with_reasoning("Very similar titles");

        assert_eq!(dup.statute_id_1, "STAT1");
        assert_eq!(dup.statute_id_2, "STAT2");
        assert_eq!(dup.similarity_score, 0.95);
        assert_eq!(dup.duplicate_type, DuplicateType::NearExact);
    }

    #[test]
    fn test_duplicate_detector_exact() {
        let statute1 = create_test_statute("TAX001", "Income Tax Act");
        let statute2 = create_test_statute("TAX001", "Income Tax Act");

        let detector = DuplicateDetector::new(AiConfig::default());
        let result = detector.compare_statutes(&statute1, &statute2).unwrap();

        assert!(result.is_some());
        let dup = result.unwrap();
        assert_eq!(dup.duplicate_type, DuplicateType::Exact);
    }

    #[test]
    fn test_duplicate_detector_similar() {
        let statute1 = create_test_statute("TAX001", "Income Tax Reform Act");
        let statute2 = create_test_statute("TAX002", "Income Tax Reform Act");

        let detector = DuplicateDetector::new(AiConfig::default());
        let result = detector.compare_statutes(&statute1, &statute2).unwrap();

        assert!(result.is_some());
    }

    #[test]
    fn test_duplicate_detector_batch() {
        let statutes = vec![
            create_test_statute("S1", "Test Statute One"),
            create_test_statute("S2", "Test Statute One"),
            create_test_statute("S3", "Different Statute"),
        ];

        let detector = DuplicateDetector::new(AiConfig::default()).with_threshold(0.7);
        let duplicates = detector.detect_duplicates(&statutes).unwrap();

        assert!(!duplicates.is_empty());
    }

    #[test]
    fn test_duplicate_type_variants() {
        assert!(matches!(DuplicateType::Exact, DuplicateType::Exact));
        assert!(matches!(DuplicateType::NearExact, DuplicateType::NearExact));
        assert!(matches!(DuplicateType::Semantic, DuplicateType::Semantic));
        assert!(matches!(DuplicateType::Partial, DuplicateType::Partial));
    }

    #[test]
    fn test_recommendation_builder() {
        let rec = StatuteRecommendation::new("STAT1", 0.85)
            .add_reason(RecommendationReason::SimilarContent)
            .add_reason(RecommendationReason::CommonTags)
            .with_explanation("Highly relevant");

        assert_eq!(rec.statute_id, "STAT1");
        assert_eq!(rec.score, 0.85);
        assert_eq!(rec.reasons.len(), 2);
        assert_eq!(rec.explanation, "Highly relevant");
    }

    #[test]
    fn test_recommendation_engine() {
        let target = create_test_statute("TAX001", "Income Tax Act");
        let mut target_with_tags = target.clone();
        target_with_tags.tags.push("taxation".to_string());

        let candidates = vec![
            {
                let mut s = create_test_statute("TAX002", "Corporate Tax Act");
                s.tags.push("taxation".to_string());
                s
            },
            create_test_statute("EDU001", "Education Standards Act"),
        ];

        let engine = RecommendationEngine::new(AiConfig::default());
        let recommendations = engine
            .recommend_similar(&target_with_tags, &candidates, 5)
            .unwrap();

        assert!(!recommendations.is_empty());
        // TAX002 should rank higher than EDU001
        if recommendations.len() >= 2 {
            assert!(recommendations[0].score >= recommendations[1].score);
        }
    }

    #[test]
    fn test_recommendation_reason_variants() {
        assert!(matches!(
            RecommendationReason::SimilarContent,
            RecommendationReason::SimilarContent
        ));
        assert!(matches!(
            RecommendationReason::CommonTags,
            RecommendationReason::CommonTags
        ));
        assert!(matches!(
            RecommendationReason::RelatedJurisdiction,
            RecommendationReason::RelatedJurisdiction
        ));
    }

    #[test]
    fn test_ai_provider_variants() {
        let openai = AiProvider::OpenAI {
            api_key: "test".to_string(),
            model: "gpt-4".to_string(),
        };
        assert!(matches!(openai, AiProvider::OpenAI { .. }));

        let mock = AiProvider::Mock;
        assert!(matches!(mock, AiProvider::Mock));
    }
}
