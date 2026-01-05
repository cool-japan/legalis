//! Multi-modal entity alignment for legal knowledge graphs.
//!
//! This module provides functionality for aligning entities across different
//! modalities (text, images, audio, video, document layout).

use crate::{RdfValue, Triple};
use std::collections::HashMap;

/// Represents an entity across multiple modalities.
#[derive(Debug, Clone, PartialEq)]
pub struct MultiModalEntity {
    /// Entity identifier
    pub id: String,
    /// Entity label
    pub label: String,
    /// Entity type
    pub entity_type: String,
    /// References to entity in different modalities
    pub modality_refs: Vec<ModalityReference>,
    /// Alignment confidence
    pub confidence: f64,
}

/// Reference to an entity in a specific modality.
#[derive(Debug, Clone, PartialEq)]
pub struct ModalityReference {
    /// Modality type
    pub modality: Modality,
    /// Reference URI
    pub reference_uri: String,
    /// Bounding information (if applicable)
    pub bounds: Option<ModalityBounds>,
    /// Confidence of this reference
    pub confidence: f64,
}

/// Type of modality.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Modality {
    /// Text document
    Text,
    /// Image
    Image,
    /// Audio
    Audio,
    /// Video
    Video,
    /// Document layout
    Layout,
    /// Custom modality
    Custom(String),
}

impl Modality {
    /// Converts to RDF class URI.
    pub fn to_rdf_class(&self) -> String {
        match self {
            Modality::Text => "legalis:TextModality".to_string(),
            Modality::Image => "legalis:ImageModality".to_string(),
            Modality::Audio => "legalis:AudioModality".to_string(),
            Modality::Video => "legalis:VideoModality".to_string(),
            Modality::Layout => "legalis:LayoutModality".to_string(),
            Modality::Custom(s) => format!("legalis:{}Modality", s.replace(' ', "")),
        }
    }
}

/// Bounds for a modality reference.
#[derive(Debug, Clone, PartialEq)]
pub enum ModalityBounds {
    /// Spatial bounds (x, y, width, height) for images/layout
    Spatial(f64, f64, f64, f64),
    /// Temporal bounds (start, end) for audio/video
    Temporal(f64, f64),
    /// Text span (start offset, end offset)
    TextSpan(usize, usize),
}

/// Multi-modal alignment algorithm.
pub struct MultiModalAligner {
    /// Base URI for generated resources
    base_uri: String,
    /// Minimum confidence threshold
    confidence_threshold: f64,
    /// Entity index by type
    entity_index: HashMap<String, Vec<MultiModalEntity>>,
}

impl MultiModalAligner {
    /// Creates a new multi-modal aligner.
    pub fn new(base_uri: impl Into<String>) -> Self {
        Self {
            base_uri: base_uri.into(),
            confidence_threshold: 0.5,
            entity_index: HashMap::new(),
        }
    }

    /// Sets the confidence threshold.
    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.confidence_threshold = threshold;
        self
    }

    /// Adds a multi-modal entity.
    pub fn add_entity(&mut self, entity: MultiModalEntity) {
        self.entity_index
            .entry(entity.entity_type.clone())
            .or_default()
            .push(entity);
    }

    /// Aligns entities across modalities based on similarity.
    pub fn align(&self) -> Vec<EntityAlignment> {
        let mut alignments = Vec::new();

        // For each entity type, find cross-modal alignments
        for entities in self.entity_index.values() {
            for entity in entities {
                if entity.modality_refs.len() >= 2 {
                    // Entity already has multi-modal references
                    let alignment = EntityAlignment {
                        entity_id: entity.id.clone(),
                        entity_label: entity.label.clone(),
                        aligned_modalities: entity
                            .modality_refs
                            .iter()
                            .map(|r| r.modality.clone())
                            .collect(),
                        confidence: entity.confidence,
                        references: entity.modality_refs.clone(),
                    };
                    alignments.push(alignment);
                }
            }
        }

        // Filter by confidence
        alignments.retain(|a| a.confidence >= self.confidence_threshold);

        alignments
    }

    /// Converts alignments to RDF triples.
    pub fn to_triples(&self, alignments: &[EntityAlignment]) -> Vec<Triple> {
        let mut triples = Vec::new();

        for alignment in alignments {
            let entity_uri = format!("{}multimodal/{}", self.base_uri, alignment.entity_id);

            // Entity type
            triples.push(Triple {
                subject: entity_uri.clone(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri("legalis:MultiModalEntity".to_string()),
            });

            // Label
            triples.push(Triple {
                subject: entity_uri.clone(),
                predicate: "rdfs:label".to_string(),
                object: RdfValue::string(&alignment.entity_label),
            });

            // Confidence
            triples.push(Triple {
                subject: entity_uri.clone(),
                predicate: "legalis:alignmentConfidence".to_string(),
                object: RdfValue::TypedLiteral(
                    alignment.confidence.to_string(),
                    "xsd:double".to_string(),
                ),
            });

            // References
            for (idx, reference) in alignment.references.iter().enumerate() {
                let ref_uri = format!(
                    "{}multimodal/{}/ref/{}",
                    self.base_uri, alignment.entity_id, idx
                );

                triples.push(Triple {
                    subject: entity_uri.clone(),
                    predicate: "legalis:hasModalityReference".to_string(),
                    object: RdfValue::Uri(ref_uri.clone()),
                });

                triples.push(Triple {
                    subject: ref_uri.clone(),
                    predicate: "rdf:type".to_string(),
                    object: RdfValue::Uri(reference.modality.to_rdf_class()),
                });

                triples.push(Triple {
                    subject: ref_uri.clone(),
                    predicate: "legalis:references".to_string(),
                    object: RdfValue::Uri(reference.reference_uri.clone()),
                });

                triples.push(Triple {
                    subject: ref_uri.clone(),
                    predicate: "legalis:confidence".to_string(),
                    object: RdfValue::TypedLiteral(
                        reference.confidence.to_string(),
                        "xsd:double".to_string(),
                    ),
                });

                // Bounds
                if let Some(ref bounds) = reference.bounds {
                    match bounds {
                        ModalityBounds::Spatial(x, y, w, h) => {
                            triples.push(Triple {
                                subject: ref_uri.clone(),
                                predicate: "legalis:spatialX".to_string(),
                                object: RdfValue::TypedLiteral(
                                    x.to_string(),
                                    "xsd:double".to_string(),
                                ),
                            });
                            triples.push(Triple {
                                subject: ref_uri.clone(),
                                predicate: "legalis:spatialY".to_string(),
                                object: RdfValue::TypedLiteral(
                                    y.to_string(),
                                    "xsd:double".to_string(),
                                ),
                            });
                            triples.push(Triple {
                                subject: ref_uri.clone(),
                                predicate: "legalis:spatialWidth".to_string(),
                                object: RdfValue::TypedLiteral(
                                    w.to_string(),
                                    "xsd:double".to_string(),
                                ),
                            });
                            triples.push(Triple {
                                subject: ref_uri.clone(),
                                predicate: "legalis:spatialHeight".to_string(),
                                object: RdfValue::TypedLiteral(
                                    h.to_string(),
                                    "xsd:double".to_string(),
                                ),
                            });
                        }
                        ModalityBounds::Temporal(start, end) => {
                            triples.push(Triple {
                                subject: ref_uri.clone(),
                                predicate: "legalis:temporalStart".to_string(),
                                object: RdfValue::TypedLiteral(
                                    start.to_string(),
                                    "xsd:double".to_string(),
                                ),
                            });
                            triples.push(Triple {
                                subject: ref_uri.clone(),
                                predicate: "legalis:temporalEnd".to_string(),
                                object: RdfValue::TypedLiteral(
                                    end.to_string(),
                                    "xsd:double".to_string(),
                                ),
                            });
                        }
                        ModalityBounds::TextSpan(start, end) => {
                            triples.push(Triple {
                                subject: ref_uri.clone(),
                                predicate: "legalis:textSpanStart".to_string(),
                                object: RdfValue::integer(*start as i64),
                            });
                            triples.push(Triple {
                                subject: ref_uri.clone(),
                                predicate: "legalis:textSpanEnd".to_string(),
                                object: RdfValue::integer(*end as i64),
                            });
                        }
                    }
                }
            }
        }

        triples
    }

    /// Gets statistics about alignments.
    pub fn stats(&self) -> AlignmentStats {
        let alignments = self.align();

        let mut by_modality_count: HashMap<Modality, usize> = HashMap::new();
        let mut modality_pairs: HashMap<(Modality, Modality), usize> = HashMap::new();

        for alignment in &alignments {
            for modality in &alignment.aligned_modalities {
                *by_modality_count.entry(modality.clone()).or_insert(0) += 1;
            }

            // Count pairs
            for i in 0..alignment.aligned_modalities.len() {
                for j in (i + 1)..alignment.aligned_modalities.len() {
                    let pair = (
                        alignment.aligned_modalities[i].clone(),
                        alignment.aligned_modalities[j].clone(),
                    );
                    *modality_pairs.entry(pair).or_insert(0) += 1;
                }
            }
        }

        AlignmentStats {
            total_alignments: alignments.len(),
            avg_confidence: if alignments.is_empty() {
                0.0
            } else {
                alignments.iter().map(|a| a.confidence).sum::<f64>() / alignments.len() as f64
            },
            by_modality: by_modality_count,
            modality_pairs,
        }
    }
}

/// Alignment of an entity across modalities.
#[derive(Debug, Clone, PartialEq)]
pub struct EntityAlignment {
    /// Entity identifier
    pub entity_id: String,
    /// Entity label
    pub entity_label: String,
    /// Modalities in which this entity appears
    pub aligned_modalities: Vec<Modality>,
    /// Alignment confidence
    pub confidence: f64,
    /// References to each modality
    pub references: Vec<ModalityReference>,
}

/// Statistics about multi-modal alignments.
#[derive(Debug, Clone)]
pub struct AlignmentStats {
    /// Total number of alignments
    pub total_alignments: usize,
    /// Average alignment confidence
    pub avg_confidence: f64,
    /// Number of alignments by modality
    pub by_modality: HashMap<Modality, usize>,
    /// Number of cross-modal pairs
    pub modality_pairs: HashMap<(Modality, Modality), usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_entity() -> MultiModalEntity {
        MultiModalEntity {
            id: "entity-001".to_string(),
            label: "Article 5".to_string(),
            entity_type: "LegalProvision".to_string(),
            modality_refs: vec![
                ModalityReference {
                    modality: Modality::Text,
                    reference_uri: "http://example.org/text/1".to_string(),
                    bounds: Some(ModalityBounds::TextSpan(100, 200)),
                    confidence: 0.95,
                },
                ModalityReference {
                    modality: Modality::Image,
                    reference_uri: "http://example.org/image/1".to_string(),
                    bounds: Some(ModalityBounds::Spatial(10.0, 20.0, 100.0, 50.0)),
                    confidence: 0.9,
                },
            ],
            confidence: 0.92,
        }
    }

    #[test]
    fn test_multimodal_entity_creation() {
        let entity = create_test_entity();
        assert_eq!(entity.id, "entity-001");
        assert_eq!(entity.modality_refs.len(), 2);
        assert_eq!(entity.confidence, 0.92);
    }

    #[test]
    fn test_modality_to_rdf() {
        assert_eq!(Modality::Text.to_rdf_class(), "legalis:TextModality");
        assert_eq!(Modality::Image.to_rdf_class(), "legalis:ImageModality");
        assert_eq!(Modality::Audio.to_rdf_class(), "legalis:AudioModality");
        assert_eq!(Modality::Video.to_rdf_class(), "legalis:VideoModality");
    }

    #[test]
    fn test_aligner_creation() {
        let aligner = MultiModalAligner::new("http://example.org/");
        assert_eq!(aligner.base_uri, "http://example.org/");
        assert_eq!(aligner.confidence_threshold, 0.5);
    }

    #[test]
    fn test_add_entity() {
        let mut aligner = MultiModalAligner::new("http://example.org/");
        let entity = create_test_entity();

        aligner.add_entity(entity);
        assert_eq!(aligner.entity_index.len(), 1);
    }

    #[test]
    fn test_align() {
        let mut aligner = MultiModalAligner::new("http://example.org/");
        let entity = create_test_entity();

        aligner.add_entity(entity);
        let alignments = aligner.align();

        assert_eq!(alignments.len(), 1);
        assert_eq!(alignments[0].aligned_modalities.len(), 2);
    }

    #[test]
    fn test_to_triples() {
        let mut aligner = MultiModalAligner::new("http://example.org/");
        let entity = create_test_entity();

        aligner.add_entity(entity);
        let alignments = aligner.align();
        let triples = aligner.to_triples(&alignments);

        assert!(!triples.is_empty());
        assert!(triples.iter().any(|t| t.predicate == "rdf:type"
            && matches!(&t.object, RdfValue::Uri(u) if u == "legalis:MultiModalEntity")));
        assert!(
            triples
                .iter()
                .any(|t| t.predicate == "legalis:hasModalityReference")
        );
    }

    #[test]
    fn test_spatial_bounds() {
        let mut aligner = MultiModalAligner::new("http://example.org/");
        let entity = create_test_entity();

        aligner.add_entity(entity);
        let alignments = aligner.align();
        let triples = aligner.to_triples(&alignments);

        assert!(triples.iter().any(|t| t.predicate == "legalis:spatialX"));
        assert!(triples.iter().any(|t| t.predicate == "legalis:spatialY"));
    }

    #[test]
    fn test_temporal_bounds() {
        let mut aligner = MultiModalAligner::new("http://example.org/");
        let entity = MultiModalEntity {
            id: "entity-002".to_string(),
            label: "Judge Statement".to_string(),
            entity_type: "Speech".to_string(),
            modality_refs: vec![
                ModalityReference {
                    modality: Modality::Audio,
                    reference_uri: "http://example.org/audio/1".to_string(),
                    bounds: Some(ModalityBounds::Temporal(10.0, 20.0)),
                    confidence: 0.9,
                },
                ModalityReference {
                    modality: Modality::Text,
                    reference_uri: "http://example.org/transcript/1".to_string(),
                    bounds: None,
                    confidence: 0.85,
                },
            ],
            confidence: 0.87,
        };

        aligner.add_entity(entity);
        let alignments = aligner.align();
        let triples = aligner.to_triples(&alignments);

        assert!(
            triples
                .iter()
                .any(|t| t.predicate == "legalis:temporalStart")
        );
        assert!(triples.iter().any(|t| t.predicate == "legalis:temporalEnd"));
    }

    #[test]
    fn test_text_span_bounds() {
        let mut aligner = MultiModalAligner::new("http://example.org/");
        let entity = create_test_entity();

        aligner.add_entity(entity);
        let alignments = aligner.align();
        let triples = aligner.to_triples(&alignments);

        assert!(
            triples
                .iter()
                .any(|t| t.predicate == "legalis:textSpanStart")
        );
        assert!(triples.iter().any(|t| t.predicate == "legalis:textSpanEnd"));
    }

    #[test]
    fn test_confidence_filtering() {
        let mut aligner = MultiModalAligner::new("http://example.org/").with_threshold(0.95);
        let entity = create_test_entity(); // confidence 0.92

        aligner.add_entity(entity);
        let alignments = aligner.align();

        // Should be filtered out
        assert_eq!(alignments.len(), 0);
    }

    #[test]
    fn test_alignment_stats() {
        let mut aligner = MultiModalAligner::new("http://example.org/");
        aligner.add_entity(create_test_entity());

        let stats = aligner.stats();

        assert_eq!(stats.total_alignments, 1);
        assert_eq!(stats.avg_confidence, 0.92);
        assert!(!stats.by_modality.is_empty());
        assert!(!stats.modality_pairs.is_empty());
    }

    #[test]
    fn test_custom_modality() {
        let custom = Modality::Custom("3D".to_string());
        assert_eq!(custom.to_rdf_class(), "legalis:3DModality");
    }

    #[test]
    fn test_single_modality_entity() {
        let mut aligner = MultiModalAligner::new("http://example.org/");
        let entity = MultiModalEntity {
            id: "entity-003".to_string(),
            label: "Single Modality".to_string(),
            entity_type: "Test".to_string(),
            modality_refs: vec![ModalityReference {
                modality: Modality::Text,
                reference_uri: "http://example.org/text/1".to_string(),
                bounds: None,
                confidence: 0.9,
            }],
            confidence: 0.9,
        };

        aligner.add_entity(entity);
        let alignments = aligner.align();

        // Should not be included (requires >= 2 modalities)
        assert_eq!(alignments.len(), 0);
    }

    #[test]
    fn test_multiple_entities() {
        let mut aligner = MultiModalAligner::new("http://example.org/");

        for i in 0..3 {
            let entity = MultiModalEntity {
                id: format!("entity-{:03}", i),
                label: format!("Entity {}", i),
                entity_type: "Test".to_string(),
                modality_refs: vec![
                    ModalityReference {
                        modality: Modality::Text,
                        reference_uri: format!("http://example.org/text/{}", i),
                        bounds: None,
                        confidence: 0.9,
                    },
                    ModalityReference {
                        modality: Modality::Image,
                        reference_uri: format!("http://example.org/image/{}", i),
                        bounds: None,
                        confidence: 0.85,
                    },
                ],
                confidence: 0.87,
            };
            aligner.add_entity(entity);
        }

        let alignments = aligner.align();
        assert_eq!(alignments.len(), 3);
    }

    #[test]
    fn test_modality_reference_without_bounds() {
        let mut aligner = MultiModalAligner::new("http://example.org/");
        let entity = MultiModalEntity {
            id: "entity-004".to_string(),
            label: "No Bounds".to_string(),
            entity_type: "Test".to_string(),
            modality_refs: vec![
                ModalityReference {
                    modality: Modality::Text,
                    reference_uri: "http://example.org/text/1".to_string(),
                    bounds: None,
                    confidence: 0.9,
                },
                ModalityReference {
                    modality: Modality::Layout,
                    reference_uri: "http://example.org/layout/1".to_string(),
                    bounds: None,
                    confidence: 0.85,
                },
            ],
            confidence: 0.87,
        };

        aligner.add_entity(entity);
        let alignments = aligner.align();
        let triples = aligner.to_triples(&alignments);

        // Should not have bounds-related triples
        assert!(
            !triples
                .iter()
                .any(|t| t.predicate.contains("Spatial") || t.predicate.contains("Temporal"))
        );
    }
}
