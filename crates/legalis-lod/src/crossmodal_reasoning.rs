//! Cross-modal reasoning for legal knowledge graphs.
//!
//! This module provides functionality for reasoning across different modalities
//! to infer new knowledge and validate consistency.

use crate::{RdfValue, Triple};
use std::collections::{HashMap, HashSet};

/// A cross-modal inference rule.
#[derive(Debug, Clone)]
pub struct CrossModalRule {
    /// Rule identifier
    pub id: String,
    /// Rule name
    pub name: String,
    /// Source modalities
    pub source_modalities: Vec<String>,
    /// Target modality
    pub target_modality: String,
    /// Rule pattern
    pub pattern: RulePattern,
    /// Confidence of inferences
    pub confidence: f64,
}

/// Pattern for cross-modal inference.
#[derive(Debug, Clone)]
pub enum RulePattern {
    /// If entity appears in text and image, infer layout presence
    TextImageToLayout,
    /// If entity in audio transcript matches document, create link
    AudioTextMatch,
    /// If visual element matches text reference, create alignment
    VisualTextAlignment,
    /// Custom pattern with description
    Custom(String),
}

/// Inferred fact from cross-modal reasoning.
#[derive(Debug, Clone, PartialEq)]
pub struct InferredFact {
    /// Subject
    pub subject: String,
    /// Predicate
    pub predicate: String,
    /// Object
    pub object: String,
    /// Confidence score
    pub confidence: f64,
    /// Source rule
    pub source_rule: String,
    /// Supporting evidence (URIs)
    pub evidence: Vec<String>,
}

impl InferredFact {
    /// Converts to an RDF triple.
    pub fn to_triple(&self) -> Triple {
        Triple {
            subject: self.subject.clone(),
            predicate: self.predicate.clone(),
            object: RdfValue::Uri(self.object.clone()),
        }
    }
}

/// Cross-modal reasoner.
pub struct CrossModalReasoner {
    /// Base URI
    base_uri: String,
    /// Inference rules
    rules: Vec<CrossModalRule>,
    /// Minimum confidence threshold
    confidence_threshold: f64,
    /// Knowledge base (modality -> entities)
    knowledge_base: HashMap<String, HashSet<String>>,
}

impl CrossModalReasoner {
    /// Creates a new cross-modal reasoner.
    pub fn new(base_uri: impl Into<String>) -> Self {
        let mut reasoner = Self {
            base_uri: base_uri.into(),
            rules: Vec::new(),
            confidence_threshold: 0.5,
            knowledge_base: HashMap::new(),
        };
        reasoner.add_default_rules();
        reasoner
    }

    /// Sets the confidence threshold.
    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.confidence_threshold = threshold;
        self
    }

    /// Adds a rule.
    pub fn add_rule(&mut self, rule: CrossModalRule) {
        self.rules.push(rule);
    }

    /// Adds default cross-modal rules.
    fn add_default_rules(&mut self) {
        // Rule: Text + Image → Layout inference
        self.rules.push(CrossModalRule {
            id: "rule-001".to_string(),
            name: "Text-Image to Layout".to_string(),
            source_modalities: vec!["text".to_string(), "image".to_string()],
            target_modality: "layout".to_string(),
            pattern: RulePattern::TextImageToLayout,
            confidence: 0.8,
        });

        // Rule: Audio transcript + Text document → Alignment
        self.rules.push(CrossModalRule {
            id: "rule-002".to_string(),
            name: "Audio-Text Matching".to_string(),
            source_modalities: vec!["audio".to_string(), "text".to_string()],
            target_modality: "alignment".to_string(),
            pattern: RulePattern::AudioTextMatch,
            confidence: 0.75,
        });

        // Rule: Visual element + Text reference → Alignment
        self.rules.push(CrossModalRule {
            id: "rule-003".to_string(),
            name: "Visual-Text Alignment".to_string(),
            source_modalities: vec!["image".to_string(), "text".to_string()],
            target_modality: "alignment".to_string(),
            pattern: RulePattern::VisualTextAlignment,
            confidence: 0.7,
        });
    }

    /// Adds entities to the knowledge base.
    pub fn add_entities(&mut self, modality: impl Into<String>, entities: Vec<String>) {
        self.knowledge_base
            .entry(modality.into())
            .or_default()
            .extend(entities);
    }

    /// Performs cross-modal reasoning.
    pub fn reason(&self) -> Vec<InferredFact> {
        let mut inferences = Vec::new();

        for rule in &self.rules {
            inferences.extend(self.apply_rule(rule));
        }

        // Filter by confidence
        inferences.retain(|inf| inf.confidence >= self.confidence_threshold);

        inferences
    }

    fn apply_rule(&self, rule: &CrossModalRule) -> Vec<InferredFact> {
        let mut inferences = Vec::new();

        match rule.pattern {
            RulePattern::TextImageToLayout => {
                inferences.extend(self.apply_text_image_to_layout(rule));
            }
            RulePattern::AudioTextMatch => {
                inferences.extend(self.apply_audio_text_match(rule));
            }
            RulePattern::VisualTextAlignment => {
                inferences.extend(self.apply_visual_text_alignment(rule));
            }
            RulePattern::Custom(_) => {
                // Custom patterns would be handled by plugins
            }
        }

        inferences
    }

    fn apply_text_image_to_layout(&self, rule: &CrossModalRule) -> Vec<InferredFact> {
        let mut inferences = Vec::new();

        // Find entities that appear in both text and image
        if let (Some(text_entities), Some(image_entities)) = (
            self.knowledge_base.get("text"),
            self.knowledge_base.get("image"),
        ) {
            for entity in text_entities.intersection(image_entities) {
                inferences.push(InferredFact {
                    subject: entity.clone(),
                    predicate: "legalis:hasLayoutRepresentation".to_string(),
                    object: format!("{}layout/{}", self.base_uri, entity),
                    confidence: rule.confidence,
                    source_rule: rule.id.clone(),
                    evidence: vec![
                        format!("{}text/{}", self.base_uri, entity),
                        format!("{}image/{}", self.base_uri, entity),
                    ],
                });
            }
        }

        inferences
    }

    fn apply_audio_text_match(&self, rule: &CrossModalRule) -> Vec<InferredFact> {
        let mut inferences = Vec::new();

        // Find entities in both audio and text
        if let (Some(audio_entities), Some(text_entities)) = (
            self.knowledge_base.get("audio"),
            self.knowledge_base.get("text"),
        ) {
            for entity in audio_entities.intersection(text_entities) {
                inferences.push(InferredFact {
                    subject: format!("{}audio/{}", self.base_uri, entity),
                    predicate: "legalis:correspondsWith".to_string(),
                    object: format!("{}text/{}", self.base_uri, entity),
                    confidence: rule.confidence,
                    source_rule: rule.id.clone(),
                    evidence: vec![
                        format!("{}audio/{}", self.base_uri, entity),
                        format!("{}text/{}", self.base_uri, entity),
                    ],
                });
            }
        }

        inferences
    }

    fn apply_visual_text_alignment(&self, rule: &CrossModalRule) -> Vec<InferredFact> {
        let mut inferences = Vec::new();

        // Find entities in both image and text
        if let (Some(image_entities), Some(text_entities)) = (
            self.knowledge_base.get("image"),
            self.knowledge_base.get("text"),
        ) {
            for entity in image_entities.intersection(text_entities) {
                inferences.push(InferredFact {
                    subject: format!("{}image/{}", self.base_uri, entity),
                    predicate: "legalis:alignedWith".to_string(),
                    object: format!("{}text/{}", self.base_uri, entity),
                    confidence: rule.confidence,
                    source_rule: rule.id.clone(),
                    evidence: vec![
                        format!("{}image/{}", self.base_uri, entity),
                        format!("{}text/{}", self.base_uri, entity),
                    ],
                });
            }
        }

        inferences
    }

    /// Converts inferences to RDF triples.
    pub fn to_triples(&self, inferences: &[InferredFact]) -> Vec<Triple> {
        let mut triples = Vec::new();

        for inference in inferences {
            let inference_uri = format!(
                "{}inference/{}",
                self.base_uri,
                self.generate_inference_id(inference)
            );

            // Main triple
            triples.push(inference.to_triple());

            // Inference metadata
            triples.push(Triple {
                subject: inference_uri.clone(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri("legalis:InferredFact".to_string()),
            });

            triples.push(Triple {
                subject: inference_uri.clone(),
                predicate: "legalis:inferredSubject".to_string(),
                object: RdfValue::Uri(inference.subject.clone()),
            });

            triples.push(Triple {
                subject: inference_uri.clone(),
                predicate: "legalis:inferredPredicate".to_string(),
                object: RdfValue::string(&inference.predicate),
            });

            triples.push(Triple {
                subject: inference_uri.clone(),
                predicate: "legalis:inferredObject".to_string(),
                object: RdfValue::Uri(inference.object.clone()),
            });

            triples.push(Triple {
                subject: inference_uri.clone(),
                predicate: "legalis:confidence".to_string(),
                object: RdfValue::TypedLiteral(
                    inference.confidence.to_string(),
                    "xsd:double".to_string(),
                ),
            });

            triples.push(Triple {
                subject: inference_uri.clone(),
                predicate: "legalis:sourceRule".to_string(),
                object: RdfValue::string(&inference.source_rule),
            });

            // Evidence
            for evidence in &inference.evidence {
                triples.push(Triple {
                    subject: inference_uri.clone(),
                    predicate: "legalis:evidence".to_string(),
                    object: RdfValue::Uri(evidence.clone()),
                });
            }
        }

        triples
    }

    fn generate_inference_id(&self, inference: &InferredFact) -> String {
        // Simple hash-like ID generation
        format!(
            "{}_{}",
            inference.source_rule,
            inference.subject.chars().take(8).collect::<String>()
        )
    }

    /// Gets statistics about reasoning.
    pub fn stats(&self) -> ReasoningStats {
        let inferences = self.reason();

        let mut by_rule: HashMap<String, usize> = HashMap::new();
        for inference in &inferences {
            *by_rule.entry(inference.source_rule.clone()).or_insert(0) += 1;
        }

        ReasoningStats {
            total_inferences: inferences.len(),
            avg_confidence: if inferences.is_empty() {
                0.0
            } else {
                inferences.iter().map(|i| i.confidence).sum::<f64>() / inferences.len() as f64
            },
            by_rule,
            total_rules: self.rules.len(),
        }
    }
}

/// Statistics about cross-modal reasoning.
#[derive(Debug, Clone)]
pub struct ReasoningStats {
    /// Total number of inferences
    pub total_inferences: usize,
    /// Average confidence
    pub avg_confidence: f64,
    /// Inferences by rule
    pub by_rule: HashMap<String, usize>,
    /// Total number of rules
    pub total_rules: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reasoner_creation() {
        let reasoner = CrossModalReasoner::new("http://example.org/");
        assert_eq!(reasoner.base_uri, "http://example.org/");
        assert!(!reasoner.rules.is_empty()); // Has default rules
    }

    #[test]
    fn test_add_entities() {
        let mut reasoner = CrossModalReasoner::new("http://example.org/");
        reasoner.add_entities("text", vec!["entity1".to_string(), "entity2".to_string()]);

        assert!(reasoner.knowledge_base.contains_key("text"));
        assert_eq!(reasoner.knowledge_base.get("text").unwrap().len(), 2);
    }

    #[test]
    fn test_text_image_to_layout() {
        let mut reasoner = CrossModalReasoner::new("http://example.org/");

        reasoner.add_entities("text", vec!["Article5".to_string()]);
        reasoner.add_entities("image", vec!["Article5".to_string()]);

        let inferences = reasoner.reason();

        assert!(!inferences.is_empty());
        assert!(
            inferences
                .iter()
                .any(|i| i.predicate == "legalis:hasLayoutRepresentation")
        );
    }

    #[test]
    fn test_audio_text_match() {
        let mut reasoner = CrossModalReasoner::new("http://example.org/");

        reasoner.add_entities("audio", vec!["testimony1".to_string()]);
        reasoner.add_entities("text", vec!["testimony1".to_string()]);

        let inferences = reasoner.reason();

        assert!(
            inferences
                .iter()
                .any(|i| i.predicate == "legalis:correspondsWith")
        );
    }

    #[test]
    fn test_visual_text_alignment() {
        let mut reasoner = CrossModalReasoner::new("http://example.org/");

        reasoner.add_entities("image", vec!["diagram1".to_string()]);
        reasoner.add_entities("text", vec!["diagram1".to_string()]);

        let inferences = reasoner.reason();

        assert!(
            inferences
                .iter()
                .any(|i| i.predicate == "legalis:alignedWith")
        );
    }

    #[test]
    fn test_confidence_filtering() {
        let mut reasoner = CrossModalReasoner::new("http://example.org/").with_threshold(0.9);

        reasoner.add_entities("text", vec!["entity1".to_string()]);
        reasoner.add_entities("image", vec!["entity1".to_string()]);

        let inferences = reasoner.reason();

        // Default rule confidence is 0.8, should be filtered
        assert!(inferences.is_empty() || inferences.iter().all(|i| i.confidence >= 0.9));
    }

    #[test]
    fn test_inferred_fact_to_triple() {
        let fact = InferredFact {
            subject: "http://example.org/subject".to_string(),
            predicate: "http://example.org/predicate".to_string(),
            object: "http://example.org/object".to_string(),
            confidence: 0.9,
            source_rule: "rule-001".to_string(),
            evidence: Vec::new(),
        };

        let triple = fact.to_triple();
        assert_eq!(triple.subject, "http://example.org/subject");
        assert_eq!(triple.predicate, "http://example.org/predicate");
    }

    #[test]
    fn test_to_triples() {
        let mut reasoner = CrossModalReasoner::new("http://example.org/");

        reasoner.add_entities("text", vec!["entity1".to_string()]);
        reasoner.add_entities("image", vec!["entity1".to_string()]);

        let inferences = reasoner.reason();
        let triples = reasoner.to_triples(&inferences);

        assert!(!triples.is_empty());
        assert!(triples.iter().any(|t| t.predicate == "rdf:type"
            && matches!(&t.object, RdfValue::Uri(u) if u == "legalis:InferredFact")));
        assert!(triples.iter().any(|t| t.predicate == "legalis:confidence"));
        assert!(triples.iter().any(|t| t.predicate == "legalis:sourceRule"));
    }

    #[test]
    fn test_evidence_tracking() {
        let mut reasoner = CrossModalReasoner::new("http://example.org/");

        reasoner.add_entities("text", vec!["entity1".to_string()]);
        reasoner.add_entities("image", vec!["entity1".to_string()]);

        let inferences = reasoner.reason();

        assert!(!inferences.is_empty());
        assert!(inferences.iter().all(|i| !i.evidence.is_empty()));
    }

    #[test]
    fn test_reasoning_stats() {
        let mut reasoner = CrossModalReasoner::new("http://example.org/");

        reasoner.add_entities("text", vec!["entity1".to_string(), "entity2".to_string()]);
        reasoner.add_entities("image", vec!["entity1".to_string()]);

        let stats = reasoner.stats();

        assert!(stats.total_inferences > 0);
        assert!(stats.avg_confidence > 0.0);
        assert!(!stats.by_rule.is_empty());
        assert!(stats.total_rules > 0);
    }

    #[test]
    fn test_add_custom_rule() {
        let mut reasoner = CrossModalReasoner::new("http://example.org/");
        let initial_count = reasoner.rules.len();

        reasoner.add_rule(CrossModalRule {
            id: "custom-001".to_string(),
            name: "Custom Rule".to_string(),
            source_modalities: vec!["video".to_string(), "text".to_string()],
            target_modality: "alignment".to_string(),
            pattern: RulePattern::Custom("Video-Text alignment".to_string()),
            confidence: 0.85,
        });

        assert_eq!(reasoner.rules.len(), initial_count + 1);
    }

    #[test]
    fn test_multiple_entity_types() {
        let mut reasoner = CrossModalReasoner::new("http://example.org/");

        reasoner.add_entities(
            "text",
            vec![
                "entity1".to_string(),
                "entity2".to_string(),
                "entity3".to_string(),
            ],
        );
        reasoner.add_entities("image", vec!["entity1".to_string(), "entity2".to_string()]);
        reasoner.add_entities("audio", vec!["entity1".to_string()]);

        let inferences = reasoner.reason();

        // Should have inferences for entity1 and entity2
        assert!(inferences.len() > 1);
    }

    #[test]
    fn test_no_overlap_no_inference() {
        let mut reasoner = CrossModalReasoner::new("http://example.org/");

        reasoner.add_entities("text", vec!["entity1".to_string()]);
        reasoner.add_entities("image", vec!["entity2".to_string()]);

        let inferences = reasoner.reason();

        // No overlapping entities, so no inferences
        assert!(inferences.is_empty());
    }

    #[test]
    fn test_default_rules_loaded() {
        let reasoner = CrossModalReasoner::new("http://example.org/");

        assert!(reasoner.rules.len() >= 3); // At least 3 default rules
        assert!(reasoner.rules.iter().any(|r| r.id == "rule-001"));
        assert!(reasoner.rules.iter().any(|r| r.id == "rule-002"));
        assert!(reasoner.rules.iter().any(|r| r.id == "rule-003"));
    }

    #[test]
    fn test_confidence_threshold() {
        let reasoner = CrossModalReasoner::new("http://example.org/").with_threshold(0.75);
        assert_eq!(reasoner.confidence_threshold, 0.75);
    }

    #[test]
    fn test_generate_inference_id() {
        let reasoner = CrossModalReasoner::new("http://example.org/");
        let inference = InferredFact {
            subject: "http://example.org/subject123".to_string(),
            predicate: "test".to_string(),
            object: "test".to_string(),
            confidence: 0.9,
            source_rule: "rule-001".to_string(),
            evidence: Vec::new(),
        };

        let id = reasoner.generate_inference_id(&inference);
        assert!(id.starts_with("rule-001"));
    }
}
