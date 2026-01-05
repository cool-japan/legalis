//! Entity linking for legal knowledge graphs.
//!
//! This module provides functionality for linking entity mentions in text
//! to entities in a knowledge graph, with support for LLM-based disambiguation.

use crate::{RdfValue, Triple};
use std::collections::HashMap;

/// An entity mention in text.
#[derive(Debug, Clone, PartialEq)]
pub struct EntityMention {
    /// The text of the mention
    pub text: String,
    /// Start position in the source text
    pub start: usize,
    /// End position in the source text
    pub end: usize,
    /// Optional entity type hint
    pub entity_type: Option<String>,
}

/// A candidate entity for linking.
#[derive(Debug, Clone, PartialEq)]
pub struct EntityCandidate {
    /// Entity URI in the knowledge graph
    pub uri: String,
    /// Entity label/name
    pub label: String,
    /// Entity type
    pub entity_type: Option<String>,
    /// Similarity score to the mention (0.0 to 1.0)
    pub similarity: f64,
    /// Context information
    pub context: Option<String>,
}

/// A linked entity (mention resolved to KB entity).
#[derive(Debug, Clone, PartialEq)]
pub struct LinkedEntity {
    /// The original mention
    pub mention: EntityMention,
    /// The linked entity
    pub entity: EntityCandidate,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Method used for linking
    pub method: LinkingMethod,
}

/// Method used for entity linking.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkingMethod {
    /// Exact string match
    ExactMatch,
    /// Fuzzy string matching
    FuzzyMatch,
    /// Context-based disambiguation
    ContextBased,
    /// LLM-based linking
    LLMBased,
    /// Hybrid approach
    Hybrid,
}

/// Entity linker using multiple strategies.
pub struct EntityLinker {
    /// Knowledge graph entities
    entities: HashMap<String, EntityCandidate>,
    /// Alias/alternative names index
    aliases: HashMap<String, Vec<String>>,
    /// Minimum confidence threshold
    confidence_threshold: f64,
}

impl Default for EntityLinker {
    fn default() -> Self {
        Self::new()
    }
}

impl EntityLinker {
    /// Creates a new entity linker.
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            aliases: HashMap::new(),
            confidence_threshold: 0.5,
        }
    }

    /// Sets the confidence threshold.
    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.confidence_threshold = threshold;
        self
    }

    /// Adds an entity to the knowledge base.
    pub fn add_entity(&mut self, entity: EntityCandidate) {
        self.entities.insert(entity.uri.clone(), entity);
    }

    /// Adds an alias for an entity.
    pub fn add_alias(&mut self, alias: impl Into<String>, entity_uri: impl Into<String>) {
        self.aliases
            .entry(alias.into())
            .or_default()
            .push(entity_uri.into());
    }

    /// Indexes entities from RDF triples.
    pub fn index_from_triples(&mut self, triples: &[Triple]) {
        for triple in triples {
            // Index entities with labels
            if triple.predicate == "rdfs:label" || triple.predicate == "skos:prefLabel" {
                if let RdfValue::Literal(ref label, _) = triple.object {
                    let entity = EntityCandidate {
                        uri: triple.subject.clone(),
                        label: label.clone(),
                        entity_type: None,
                        similarity: 1.0,
                        context: None,
                    };
                    self.add_entity(entity);
                    self.add_alias(label.clone(), triple.subject.clone());
                }
            }

            // Index alternative labels
            if triple.predicate == "skos:altLabel" {
                if let RdfValue::Literal(ref label, _) = triple.object {
                    self.add_alias(label.clone(), triple.subject.clone());
                }
            }
        }
    }

    /// Links entity mentions in text.
    pub fn link(&self, mentions: &[EntityMention]) -> Vec<LinkedEntity> {
        let mut linked = Vec::new();

        for mention in mentions {
            if let Some(entity) = self.link_mention(mention) {
                linked.push(entity);
            }
        }

        linked
    }

    fn link_mention(&self, mention: &EntityMention) -> Option<LinkedEntity> {
        // Try exact match first
        if let Some(entity) = self.exact_match(mention) {
            return Some(LinkedEntity {
                mention: mention.clone(),
                entity,
                confidence: 0.95,
                method: LinkingMethod::ExactMatch,
            });
        }

        // Try fuzzy match
        if let Some(entity) = self.fuzzy_match(mention) {
            return Some(LinkedEntity {
                mention: mention.clone(),
                entity,
                confidence: 0.7,
                method: LinkingMethod::FuzzyMatch,
            });
        }

        None
    }

    fn exact_match(&self, mention: &EntityMention) -> Option<EntityCandidate> {
        // Look up in aliases
        if let Some(uris) = self.aliases.get(&mention.text) {
            if let Some(uri) = uris.first() {
                return self.entities.get(uri).cloned();
            }
        }

        // Direct entity lookup
        self.entities.get(&mention.text).cloned()
    }

    fn fuzzy_match(&self, mention: &EntityMention) -> Option<EntityCandidate> {
        let mention_lower = mention.text.to_lowercase();

        // Find best matching entity
        let mut best_match: Option<(EntityCandidate, f64)> = None;

        for (alias, uris) in &self.aliases {
            let similarity = self.string_similarity(&mention_lower, &alias.to_lowercase());

            if similarity > 0.7 {
                // Threshold for fuzzy matching
                if let Some(uri) = uris.first() {
                    if let Some(entity) = self.entities.get(uri) {
                        if best_match.is_none() || similarity > best_match.as_ref().unwrap().1 {
                            best_match = Some((entity.clone(), similarity));
                        }
                    }
                }
            }
        }

        best_match.map(|(entity, _)| entity)
    }

    fn string_similarity(&self, s1: &str, s2: &str) -> f64 {
        // Simple Jaccard similarity on character trigrams
        if s1 == s2 {
            return 1.0;
        }

        let trigrams1 = self.get_trigrams(s1);
        let trigrams2 = self.get_trigrams(s2);

        if trigrams1.is_empty() && trigrams2.is_empty() {
            return 1.0;
        }

        if trigrams1.is_empty() || trigrams2.is_empty() {
            return 0.0;
        }

        let intersection: usize = trigrams1.iter().filter(|t| trigrams2.contains(t)).count();
        let union = trigrams1.len() + trigrams2.len() - intersection;

        intersection as f64 / union as f64
    }

    fn get_trigrams(&self, s: &str) -> Vec<String> {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() < 3 {
            return vec![s.to_string()];
        }

        chars
            .windows(3)
            .map(|w| w.iter().collect::<String>())
            .collect()
    }

    /// Gets entity statistics.
    pub fn stats(&self) -> EntityLinkerStats {
        EntityLinkerStats {
            total_entities: self.entities.len(),
            total_aliases: self.aliases.values().map(|v| v.len()).sum(),
        }
    }
}

/// Statistics about the entity linker.
#[derive(Debug, Clone)]
pub struct EntityLinkerStats {
    /// Total number of entities
    pub total_entities: usize,
    /// Total number of aliases
    pub total_aliases: usize,
}

/// LLM-based entity linking (placeholder for actual LLM integration).
pub struct LLMEntityLinker {
    /// Base linker for fallback
    base_linker: EntityLinker,
    /// Context window size
    context_window: usize,
}

impl LLMEntityLinker {
    /// Creates a new LLM-based entity linker.
    pub fn new(base_linker: EntityLinker) -> Self {
        Self {
            base_linker,
            context_window: 100,
        }
    }

    /// Sets the context window size.
    pub fn with_context_window(mut self, size: usize) -> Self {
        self.context_window = size;
        self
    }

    /// Links entities using LLM (placeholder - would call actual LLM API).
    pub fn link_with_llm(&self, text: &str, mentions: &[EntityMention]) -> Vec<LinkedEntity> {
        let mut linked = Vec::new();

        // In a real implementation, this would:
        // 1. Extract context around each mention
        // 2. Call LLM API with context and candidates
        // 3. Parse LLM response to get entity links

        // For now, fall back to base linker
        linked.extend(self.base_linker.link(mentions));

        // Add LLM-specific post-processing
        for entity in &mut linked {
            // Adjust confidence based on context
            if let Some(context) = self.extract_context(text, &entity.mention) {
                entity.confidence =
                    self.adjust_confidence_with_context(entity.confidence, &context);
                entity.method = LinkingMethod::LLMBased;
            }
        }

        linked
    }

    fn extract_context(&self, text: &str, mention: &EntityMention) -> Option<String> {
        let start = mention.start.saturating_sub(self.context_window);
        let end = (mention.end + self.context_window).min(text.len());

        Some(text[start..end].to_string())
    }

    fn adjust_confidence_with_context(&self, base_confidence: f64, _context: &str) -> f64 {
        // Placeholder: In real implementation, would use LLM to assess context match
        // For now, slightly boost confidence
        (base_confidence * 1.1).min(1.0)
    }

    /// Gets candidates for a mention using LLM ranking.
    pub fn get_candidates(
        &self,
        mention: &EntityMention,
        max_candidates: usize,
    ) -> Vec<EntityCandidate> {
        // In real implementation: use LLM to generate and rank candidates
        // For now: return top candidates from base linker

        let mut candidates: Vec<EntityCandidate> =
            self.base_linker.entities.values().cloned().collect();

        // Sort by similarity to mention text
        let mention_lower = mention.text.to_lowercase();
        candidates.sort_by(|a, b| {
            let sim_a = self
                .base_linker
                .string_similarity(&mention_lower, &a.label.to_lowercase());
            let sim_b = self
                .base_linker
                .string_similarity(&mention_lower, &b.label.to_lowercase());
            sim_b
                .partial_cmp(&sim_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        candidates.truncate(max_candidates);
        candidates
    }
}

/// Extracts entity mentions from text (simple pattern-based).
pub struct MentionExtractor {
    /// Entity type patterns
    patterns: HashMap<String, Vec<String>>,
}

impl Default for MentionExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl MentionExtractor {
    /// Creates a new mention extractor.
    pub fn new() -> Self {
        let mut extractor = Self {
            patterns: HashMap::new(),
        };
        extractor.add_default_patterns();
        extractor
    }

    fn add_default_patterns(&mut self) {
        // Legal document patterns
        self.add_pattern("LegalDocument", "Article");
        self.add_pattern("LegalDocument", "Section");
        self.add_pattern("LegalDocument", "Paragraph");
        self.add_pattern("LegalDocument", "Chapter");

        // Act names
        self.add_pattern("Act", "Act");
        self.add_pattern("Act", "Law");
        self.add_pattern("Act", "Statute");
    }

    fn add_pattern(&mut self, entity_type: &str, pattern: &str) {
        self.patterns
            .entry(entity_type.to_string())
            .or_default()
            .push(pattern.to_string());
    }

    /// Extracts entity mentions from text.
    pub fn extract(&self, text: &str) -> Vec<EntityMention> {
        let mut mentions = Vec::new();

        // Simple pattern matching
        for (entity_type, patterns) in &self.patterns {
            for pattern in patterns {
                let mut start = 0;
                while let Some(pos) = text[start..].find(pattern) {
                    let abs_pos = start + pos;
                    let end = abs_pos + pattern.len();

                    // Extract the full entity (pattern + following number/text)
                    let remaining = &text[end..];
                    let entity_end = remaining
                        .find(|c: char| !c.is_alphanumeric() && c != ' ' && c != '-')
                        .unwrap_or(remaining.len());

                    let full_mention = &text[abs_pos..end + entity_end];

                    mentions.push(EntityMention {
                        text: full_mention.trim().to_string(),
                        start: abs_pos,
                        end: end + entity_end,
                        entity_type: Some(entity_type.clone()),
                    });

                    start = abs_pos + 1;
                }
            }
        }

        // Remove duplicates and overlaps
        mentions.sort_by_key(|m| m.start);
        mentions.dedup_by(|a, b| a.start == b.start && a.end == b.end);

        mentions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_mention_creation() {
        let mention = EntityMention {
            text: "Article 5".to_string(),
            start: 10,
            end: 19,
            entity_type: Some("LegalDocument".to_string()),
        };

        assert_eq!(mention.text, "Article 5");
        assert_eq!(mention.start, 10);
        assert_eq!(mention.end, 19);
    }

    #[test]
    fn test_entity_linker_creation() {
        let linker = EntityLinker::new();
        assert_eq!(linker.entities.len(), 0);
        assert_eq!(linker.confidence_threshold, 0.5);
    }

    #[test]
    fn test_add_entity() {
        let mut linker = EntityLinker::new();
        let entity = EntityCandidate {
            uri: "http://example.org/article5".to_string(),
            label: "Article 5".to_string(),
            entity_type: Some("LegalDocument".to_string()),
            similarity: 1.0,
            context: None,
        };

        linker.add_entity(entity);
        assert_eq!(linker.entities.len(), 1);
    }

    #[test]
    fn test_add_alias() {
        let mut linker = EntityLinker::new();
        linker.add_alias("Art. 5", "http://example.org/article5");

        assert_eq!(linker.aliases.len(), 1);
        assert_eq!(linker.aliases["Art. 5"][0], "http://example.org/article5");
    }

    #[test]
    fn test_exact_match() {
        let mut linker = EntityLinker::new();
        let entity = EntityCandidate {
            uri: "http://example.org/article5".to_string(),
            label: "Article 5".to_string(),
            entity_type: Some("LegalDocument".to_string()),
            similarity: 1.0,
            context: None,
        };

        linker.add_entity(entity);
        linker.add_alias("Article 5", "http://example.org/article5");

        let mention = EntityMention {
            text: "Article 5".to_string(),
            start: 0,
            end: 9,
            entity_type: None,
        };

        let linked = linker.link(&[mention]);
        assert_eq!(linked.len(), 1);
        assert_eq!(linked[0].method, LinkingMethod::ExactMatch);
    }

    #[test]
    fn test_fuzzy_match() {
        let mut linker = EntityLinker::new();
        let entity = EntityCandidate {
            uri: "http://example.org/article5".to_string(),
            label: "Article 5".to_string(),
            entity_type: Some("LegalDocument".to_string()),
            similarity: 1.0,
            context: None,
        };

        linker.add_entity(entity);
        linker.add_alias("Article 5", "http://example.org/article5");

        let mention = EntityMention {
            text: "article 5".to_string(), // lowercase
            start: 0,
            end: 9,
            entity_type: None,
        };

        let linked = linker.link(&[mention]);
        assert!(!linked.is_empty());
    }

    #[test]
    fn test_string_similarity() {
        let linker = EntityLinker::new();
        assert_eq!(linker.string_similarity("test", "test"), 1.0);
        assert!(linker.string_similarity("test", "tests") > 0.5);
        assert!(linker.string_similarity("hello", "world") < 0.5);
    }

    #[test]
    fn test_trigrams() {
        let linker = EntityLinker::new();
        let trigrams = linker.get_trigrams("test");

        assert_eq!(trigrams.len(), 2);
        assert!(trigrams.contains(&"tes".to_string()));
        assert!(trigrams.contains(&"est".to_string()));
    }

    #[test]
    fn test_mention_extractor() {
        let extractor = MentionExtractor::new();
        let text = "Article 5 of the Constitution states that Section 10 applies.";

        let mentions = extractor.extract(text);
        assert!(!mentions.is_empty());
        assert!(mentions.iter().any(|m| m.text.contains("Article")));
        assert!(mentions.iter().any(|m| m.text.contains("Section")));
    }

    #[test]
    fn test_llm_entity_linker() {
        let base_linker = EntityLinker::new();
        let llm_linker = LLMEntityLinker::new(base_linker);

        assert_eq!(llm_linker.context_window, 100);
    }

    #[test]
    fn test_extract_context() {
        let base_linker = EntityLinker::new();
        let llm_linker = LLMEntityLinker::new(base_linker).with_context_window(10);

        let text = "This is a test document with Article 5 mentioned.";
        let mention = EntityMention {
            text: "Article 5".to_string(),
            start: 29,
            end: 38,
            entity_type: None,
        };

        let context = llm_linker.extract_context(text, &mention);
        assert!(context.is_some());
        assert!(context.unwrap().contains("Article 5"));
    }

    #[test]
    fn test_index_from_triples() {
        let mut linker = EntityLinker::new();

        let triples = vec![
            Triple {
                subject: "http://example.org/article5".to_string(),
                predicate: "rdfs:label".to_string(),
                object: RdfValue::Literal("Article 5".to_string(), None),
            },
            Triple {
                subject: "http://example.org/article5".to_string(),
                predicate: "skos:altLabel".to_string(),
                object: RdfValue::Literal("Art. 5".to_string(), None),
            },
        ];

        linker.index_from_triples(&triples);

        assert_eq!(linker.entities.len(), 1);
        assert!(linker.aliases.contains_key("Article 5"));
        assert!(linker.aliases.contains_key("Art. 5"));
    }

    #[test]
    fn test_linker_stats() {
        let mut linker = EntityLinker::new();

        linker.add_entity(EntityCandidate {
            uri: "http://example.org/1".to_string(),
            label: "Entity 1".to_string(),
            entity_type: None,
            similarity: 1.0,
            context: None,
        });

        linker.add_alias("E1", "http://example.org/1");
        linker.add_alias("Entity One", "http://example.org/1");

        let stats = linker.stats();
        assert_eq!(stats.total_entities, 1);
        assert_eq!(stats.total_aliases, 2);
    }

    #[test]
    fn test_get_candidates() {
        let mut linker = EntityLinker::new();

        for i in 0..10 {
            linker.add_entity(EntityCandidate {
                uri: format!("http://example.org/{}", i),
                label: format!("Entity {}", i),
                entity_type: None,
                similarity: 1.0,
                context: None,
            });
        }

        let llm_linker = LLMEntityLinker::new(linker);
        let mention = EntityMention {
            text: "Entity 5".to_string(),
            start: 0,
            end: 8,
            entity_type: None,
        };

        let candidates = llm_linker.get_candidates(&mention, 5);
        assert_eq!(candidates.len(), 5);
    }

    #[test]
    fn test_linking_method() {
        assert_eq!(LinkingMethod::ExactMatch, LinkingMethod::ExactMatch);
        assert_ne!(LinkingMethod::ExactMatch, LinkingMethod::FuzzyMatch);
    }

    #[test]
    fn test_confidence_threshold() {
        let linker = EntityLinker::new().with_threshold(0.8);
        assert_eq!(linker.confidence_threshold, 0.8);
    }
}
