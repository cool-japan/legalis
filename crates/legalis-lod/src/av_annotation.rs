//! Audio/video annotation for legal multimedia content.
//!
//! This module provides functionality for annotating audio and video content
//! with temporal segments, transcripts, and semantic tags.

use crate::{RdfValue, Triple};
use std::collections::HashMap;

/// Represents an audio or video document.
#[derive(Debug, Clone)]
pub struct MediaDocument {
    /// Document identifier
    pub id: String,
    /// Media source (path or URL)
    pub source: String,
    /// Media type (audio or video)
    pub media_type: MediaType,
    /// Duration in seconds
    pub duration: Option<f64>,
    /// Format (MP3, MP4, WAV, etc.)
    pub format: Option<String>,
    /// Language
    pub language: Option<String>,
}

/// Type of media.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaType {
    /// Audio only
    Audio,
    /// Video with audio
    Video,
}

impl MediaType {
    /// Converts to RDF class URI.
    pub fn to_rdf_class(&self) -> &'static str {
        match self {
            MediaType::Audio => "legalis:AudioDocument",
            MediaType::Video => "legalis:VideoDocument",
        }
    }
}

/// Temporal annotation for a media segment.
#[derive(Debug, Clone, PartialEq)]
pub struct TemporalAnnotation {
    /// Start time in seconds
    pub start_time: f64,
    /// End time in seconds
    pub end_time: f64,
    /// Annotation type
    pub annotation_type: AnnotationType,
    /// Content/description
    pub content: String,
    /// Speaker (if applicable)
    pub speaker: Option<String>,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Additional tags
    pub tags: Vec<String>,
}

impl TemporalAnnotation {
    /// Gets the duration of this annotation.
    pub fn duration(&self) -> f64 {
        self.end_time - self.start_time
    }

    /// Checks if this annotation overlaps with another.
    pub fn overlaps(&self, other: &TemporalAnnotation) -> bool {
        !(self.end_time <= other.start_time || other.end_time <= self.start_time)
    }
}

/// Type of annotation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AnnotationType {
    /// Transcript/speech
    Transcript,
    /// Scene description
    Scene,
    /// Speaker identification
    Speaker,
    /// Topic or subject
    Topic,
    /// Legal argument
    Argument,
    /// Citation or reference
    Citation,
    /// Procedural event
    ProceduralEvent,
    /// Custom annotation
    Custom(String),
}

impl AnnotationType {
    /// Converts to RDF class URI.
    pub fn to_rdf_class(&self) -> String {
        match self {
            AnnotationType::Transcript => "legalis:TranscriptAnnotation".to_string(),
            AnnotationType::Scene => "legalis:SceneAnnotation".to_string(),
            AnnotationType::Speaker => "legalis:SpeakerAnnotation".to_string(),
            AnnotationType::Topic => "legalis:TopicAnnotation".to_string(),
            AnnotationType::Argument => "legalis:ArgumentAnnotation".to_string(),
            AnnotationType::Citation => "legalis:CitationAnnotation".to_string(),
            AnnotationType::ProceduralEvent => "legalis:ProceduralEventAnnotation".to_string(),
            AnnotationType::Custom(s) => format!("legalis:{}Annotation", s.replace(' ', "")),
        }
    }
}

/// Media-to-RDF converter.
pub struct MediaToRdfConverter {
    /// Base URI for generated resources
    base_uri: String,
    /// Minimum confidence threshold
    confidence_threshold: f64,
}

impl MediaToRdfConverter {
    /// Creates a new media-to-RDF converter.
    pub fn new(base_uri: impl Into<String>) -> Self {
        Self {
            base_uri: base_uri.into(),
            confidence_threshold: 0.5,
        }
    }

    /// Sets the confidence threshold.
    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.confidence_threshold = threshold;
        self
    }

    /// Converts media document with annotations to RDF triples.
    pub fn convert(
        &self,
        media: &MediaDocument,
        annotations: &[TemporalAnnotation],
    ) -> Vec<Triple> {
        let mut triples = Vec::new();

        // Media document metadata
        let media_uri = format!("{}media/{}", self.base_uri, media.id);

        triples.push(Triple {
            subject: media_uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri(media.media_type.to_rdf_class().to_string()),
        });

        triples.push(Triple {
            subject: media_uri.clone(),
            predicate: "dcterms:identifier".to_string(),
            object: RdfValue::string(&media.id),
        });

        triples.push(Triple {
            subject: media_uri.clone(),
            predicate: "legalis:source".to_string(),
            object: RdfValue::string(&media.source),
        });

        if let Some(duration) = media.duration {
            triples.push(Triple {
                subject: media_uri.clone(),
                predicate: "legalis:duration".to_string(),
                object: RdfValue::TypedLiteral(duration.to_string(), "xsd:double".to_string()),
            });
        }

        if let Some(ref format) = media.format {
            triples.push(Triple {
                subject: media_uri.clone(),
                predicate: "dcterms:format".to_string(),
                object: RdfValue::string(format),
            });
        }

        if let Some(ref language) = media.language {
            triples.push(Triple {
                subject: media_uri.clone(),
                predicate: "dcterms:language".to_string(),
                object: RdfValue::string(language),
            });
        }

        // Temporal annotations
        for (idx, annotation) in annotations.iter().enumerate() {
            if annotation.confidence < self.confidence_threshold {
                continue;
            }

            let annotation_uri = format!("{}annotation/{}/{}", self.base_uri, media.id, idx);

            // Annotation type
            triples.push(Triple {
                subject: annotation_uri.clone(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri(annotation.annotation_type.to_rdf_class()),
            });

            // Link to parent media
            triples.push(Triple {
                subject: annotation_uri.clone(),
                predicate: "legalis:annotates".to_string(),
                object: RdfValue::Uri(media_uri.clone()),
            });

            // Temporal bounds
            triples.push(Triple {
                subject: annotation_uri.clone(),
                predicate: "legalis:startTime".to_string(),
                object: RdfValue::TypedLiteral(
                    annotation.start_time.to_string(),
                    "xsd:double".to_string(),
                ),
            });

            triples.push(Triple {
                subject: annotation_uri.clone(),
                predicate: "legalis:endTime".to_string(),
                object: RdfValue::TypedLiteral(
                    annotation.end_time.to_string(),
                    "xsd:double".to_string(),
                ),
            });

            // Content
            triples.push(Triple {
                subject: annotation_uri.clone(),
                predicate: "legalis:content".to_string(),
                object: RdfValue::string(&annotation.content),
            });

            // Speaker
            if let Some(ref speaker) = annotation.speaker {
                triples.push(Triple {
                    subject: annotation_uri.clone(),
                    predicate: "legalis:speaker".to_string(),
                    object: RdfValue::string(speaker),
                });
            }

            // Confidence
            triples.push(Triple {
                subject: annotation_uri.clone(),
                predicate: "legalis:confidence".to_string(),
                object: RdfValue::TypedLiteral(
                    annotation.confidence.to_string(),
                    "xsd:double".to_string(),
                ),
            });

            // Tags
            for tag in &annotation.tags {
                triples.push(Triple {
                    subject: annotation_uri.clone(),
                    predicate: "legalis:tag".to_string(),
                    object: RdfValue::string(tag),
                });
            }
        }

        // Add annotation count
        triples.push(Triple {
            subject: media_uri,
            predicate: "legalis:annotationCount".to_string(),
            object: RdfValue::integer(
                annotations
                    .iter()
                    .filter(|a| a.confidence >= self.confidence_threshold)
                    .count() as i64,
            ),
        });

        triples
    }

    /// Creates a full transcript from transcript annotations.
    pub fn generate_transcript(&self, annotations: &[TemporalAnnotation]) -> String {
        let mut transcript_annotations: Vec<&TemporalAnnotation> = annotations
            .iter()
            .filter(|a| a.annotation_type == AnnotationType::Transcript)
            .collect();

        transcript_annotations.sort_by(|a, b| {
            a.start_time
                .partial_cmp(&b.start_time)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        transcript_annotations
            .iter()
            .map(|a| {
                if let Some(ref speaker) = a.speaker {
                    format!(
                        "[{:.1}s - {:.1}s] {}: {}",
                        a.start_time, a.end_time, speaker, a.content
                    )
                } else {
                    format!("[{:.1}s - {:.1}s] {}", a.start_time, a.end_time, a.content)
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Statistics about media annotations.
#[derive(Debug, Clone)]
pub struct AnnotationStats {
    /// Total number of media documents
    pub total_media: usize,
    /// Total annotations
    pub total_annotations: usize,
    /// Annotations by type
    pub by_type: HashMap<AnnotationType, usize>,
    /// Average confidence
    pub avg_confidence: f64,
    /// Total annotated duration
    pub total_duration: f64,
}

impl AnnotationStats {
    /// Creates statistics from annotations.
    pub fn from_annotations(media_count: usize, annotations: &[TemporalAnnotation]) -> Self {
        let mut by_type: HashMap<AnnotationType, usize> = HashMap::new();
        let mut total_confidence = 0.0;
        let mut total_duration = 0.0;

        for annotation in annotations {
            *by_type
                .entry(annotation.annotation_type.clone())
                .or_insert(0) += 1;
            total_confidence += annotation.confidence;
            total_duration += annotation.duration();
        }

        let avg_confidence = if annotations.is_empty() {
            0.0
        } else {
            total_confidence / annotations.len() as f64
        };

        Self {
            total_media: media_count,
            total_annotations: annotations.len(),
            by_type,
            avg_confidence,
            total_duration,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_media() -> MediaDocument {
        MediaDocument {
            id: "test-video-001".to_string(),
            source: "/path/to/video.mp4".to_string(),
            media_type: MediaType::Video,
            duration: Some(120.0),
            format: Some("MP4".to_string()),
            language: Some("en".to_string()),
        }
    }

    fn create_test_annotation() -> TemporalAnnotation {
        TemporalAnnotation {
            start_time: 0.0,
            end_time: 10.0,
            annotation_type: AnnotationType::Transcript,
            content: "Test transcript".to_string(),
            speaker: Some("Speaker A".to_string()),
            confidence: 0.95,
            tags: vec!["legal".to_string()],
        }
    }

    #[test]
    fn test_media_document_creation() {
        let media = create_test_media();
        assert_eq!(media.id, "test-video-001");
        assert_eq!(media.media_type, MediaType::Video);
        assert_eq!(media.duration, Some(120.0));
    }

    #[test]
    fn test_media_type_to_rdf() {
        assert_eq!(MediaType::Audio.to_rdf_class(), "legalis:AudioDocument");
        assert_eq!(MediaType::Video.to_rdf_class(), "legalis:VideoDocument");
    }

    #[test]
    fn test_annotation_duration() {
        let annotation = TemporalAnnotation {
            start_time: 5.0,
            end_time: 15.0,
            annotation_type: AnnotationType::Transcript,
            content: "Test".to_string(),
            speaker: None,
            confidence: 0.9,
            tags: Vec::new(),
        };

        assert_eq!(annotation.duration(), 10.0);
    }

    #[test]
    fn test_annotation_overlap() {
        let ann1 = TemporalAnnotation {
            start_time: 0.0,
            end_time: 10.0,
            annotation_type: AnnotationType::Transcript,
            content: "A".to_string(),
            speaker: None,
            confidence: 0.9,
            tags: Vec::new(),
        };

        let ann2 = TemporalAnnotation {
            start_time: 5.0,
            end_time: 15.0,
            annotation_type: AnnotationType::Transcript,
            content: "B".to_string(),
            speaker: None,
            confidence: 0.9,
            tags: Vec::new(),
        };

        let ann3 = TemporalAnnotation {
            start_time: 20.0,
            end_time: 30.0,
            annotation_type: AnnotationType::Transcript,
            content: "C".to_string(),
            speaker: None,
            confidence: 0.9,
            tags: Vec::new(),
        };

        assert!(ann1.overlaps(&ann2));
        assert!(!ann1.overlaps(&ann3));
    }

    #[test]
    fn test_annotation_type_to_rdf() {
        assert_eq!(
            AnnotationType::Transcript.to_rdf_class(),
            "legalis:TranscriptAnnotation"
        );
        assert_eq!(
            AnnotationType::Scene.to_rdf_class(),
            "legalis:SceneAnnotation"
        );
        assert_eq!(
            AnnotationType::Citation.to_rdf_class(),
            "legalis:CitationAnnotation"
        );
    }

    #[test]
    fn test_converter_creation() {
        let converter = MediaToRdfConverter::new("http://example.org/");
        assert_eq!(converter.base_uri, "http://example.org/");
        assert_eq!(converter.confidence_threshold, 0.5);
    }

    #[test]
    fn test_converter_with_threshold() {
        let converter = MediaToRdfConverter::new("http://example.org/").with_threshold(0.8);
        assert_eq!(converter.confidence_threshold, 0.8);
    }

    #[test]
    fn test_convert_to_rdf() {
        let converter = MediaToRdfConverter::new("http://example.org/");
        let media = create_test_media();
        let annotations = vec![create_test_annotation()];

        let triples = converter.convert(&media, &annotations);

        assert!(!triples.is_empty());
        assert!(triples.iter().any(|t| t.predicate == "rdf:type"
            && matches!(&t.object, RdfValue::Uri(u) if u == "legalis:VideoDocument")));
        assert!(triples.iter().any(|t| t.predicate == "dcterms:identifier"));
        assert!(triples.iter().any(|t| t.predicate == "legalis:duration"));
    }

    #[test]
    fn test_annotation_conversion() {
        let converter = MediaToRdfConverter::new("http://example.org/");
        let media = create_test_media();
        let annotations = vec![create_test_annotation()];

        let triples = converter.convert(&media, &annotations);

        assert!(triples.iter().any(|t| t.predicate == "legalis:annotates"));
        assert!(triples.iter().any(|t| t.predicate == "legalis:startTime"));
        assert!(triples.iter().any(|t| t.predicate == "legalis:endTime"));
        assert!(triples.iter().any(|t| t.predicate == "legalis:content"));
        assert!(triples.iter().any(|t| t.predicate == "legalis:speaker"));
    }

    #[test]
    fn test_confidence_filtering() {
        let converter = MediaToRdfConverter::new("http://example.org/").with_threshold(0.96);
        let media = create_test_media();
        let annotations = vec![
            TemporalAnnotation {
                start_time: 0.0,
                end_time: 10.0,
                annotation_type: AnnotationType::Transcript,
                content: "Low confidence".to_string(),
                speaker: None,
                confidence: 0.5,
                tags: Vec::new(),
            },
            TemporalAnnotation {
                start_time: 10.0,
                end_time: 20.0,
                annotation_type: AnnotationType::Transcript,
                content: "High confidence".to_string(),
                speaker: None,
                confidence: 0.99,
                tags: Vec::new(),
            },
        ];

        let triples = converter.convert(&media, &annotations);

        // Should only include high-confidence annotation
        let content_count = triples
            .iter()
            .filter(|t| t.predicate == "legalis:content")
            .count();
        assert_eq!(content_count, 1);
    }

    #[test]
    fn test_generate_transcript() {
        let converter = MediaToRdfConverter::new("http://example.org/");
        let annotations = vec![
            TemporalAnnotation {
                start_time: 0.0,
                end_time: 5.0,
                annotation_type: AnnotationType::Transcript,
                content: "Hello".to_string(),
                speaker: Some("Alice".to_string()),
                confidence: 0.95,
                tags: Vec::new(),
            },
            TemporalAnnotation {
                start_time: 5.0,
                end_time: 10.0,
                annotation_type: AnnotationType::Transcript,
                content: "Hi there".to_string(),
                speaker: Some("Bob".to_string()),
                confidence: 0.9,
                tags: Vec::new(),
            },
        ];

        let transcript = converter.generate_transcript(&annotations);

        assert!(transcript.contains("Alice: Hello"));
        assert!(transcript.contains("Bob: Hi there"));
        assert!(transcript.contains("[0.0s - 5.0s]"));
    }

    #[test]
    fn test_annotation_stats() {
        let annotations = vec![
            TemporalAnnotation {
                start_time: 0.0,
                end_time: 10.0,
                annotation_type: AnnotationType::Transcript,
                content: "A".to_string(),
                speaker: None,
                confidence: 0.8,
                tags: Vec::new(),
            },
            TemporalAnnotation {
                start_time: 10.0,
                end_time: 20.0,
                annotation_type: AnnotationType::Scene,
                content: "B".to_string(),
                speaker: None,
                confidence: 0.9,
                tags: Vec::new(),
            },
        ];

        let stats = AnnotationStats::from_annotations(1, &annotations);

        assert_eq!(stats.total_media, 1);
        assert_eq!(stats.total_annotations, 2);
        assert!((stats.avg_confidence - 0.85).abs() < 0.001);
        assert_eq!(stats.total_duration, 20.0);
        assert_eq!(stats.by_type.len(), 2);
    }

    #[test]
    fn test_custom_annotation_type() {
        let custom = AnnotationType::Custom("Objection".to_string());
        assert_eq!(custom.to_rdf_class(), "legalis:ObjectionAnnotation");
    }

    #[test]
    fn test_annotation_tags() {
        let converter = MediaToRdfConverter::new("http://example.org/");
        let media = create_test_media();
        let mut annotation = create_test_annotation();
        annotation.tags = vec![
            "important".to_string(),
            "evidence".to_string(),
            "procedural".to_string(),
        ];

        let triples = converter.convert(&media, &[annotation]);

        let tag_count = triples
            .iter()
            .filter(|t| t.predicate == "legalis:tag")
            .count();
        assert_eq!(tag_count, 3);
    }

    #[test]
    fn test_audio_media() {
        let converter = MediaToRdfConverter::new("http://example.org/");
        let media = MediaDocument {
            id: "audio-001".to_string(),
            source: "/path/to/audio.mp3".to_string(),
            media_type: MediaType::Audio,
            duration: Some(60.0),
            format: Some("MP3".to_string()),
            language: Some("en".to_string()),
        };

        let triples = converter.convert(&media, &[]);

        assert!(
            triples
                .iter()
                .any(|t| matches!(&t.object, RdfValue::Uri(u) if u == "legalis:AudioDocument"))
        );
    }

    #[test]
    fn test_multiple_speakers() {
        let converter = MediaToRdfConverter::new("http://example.org/");
        let annotations = vec![
            TemporalAnnotation {
                start_time: 0.0,
                end_time: 5.0,
                annotation_type: AnnotationType::Transcript,
                content: "First statement".to_string(),
                speaker: Some("Judge".to_string()),
                confidence: 0.95,
                tags: Vec::new(),
            },
            TemporalAnnotation {
                start_time: 5.0,
                end_time: 10.0,
                annotation_type: AnnotationType::Transcript,
                content: "Response".to_string(),
                speaker: Some("Attorney".to_string()),
                confidence: 0.9,
                tags: Vec::new(),
            },
        ];

        let transcript = converter.generate_transcript(&annotations);

        assert!(transcript.contains("Judge:"));
        assert!(transcript.contains("Attorney:"));
    }

    #[test]
    fn test_empty_annotations() {
        let converter = MediaToRdfConverter::new("http://example.org/");
        let media = create_test_media();
        let triples = converter.convert(&media, &[]);

        // Should still have media metadata
        assert!(
            triples
                .iter()
                .any(|t| matches!(&t.object, RdfValue::Uri(u) if u == "legalis:VideoDocument"))
        );

        // Annotation count should be 0
        let count_triple = triples
            .iter()
            .find(|t| t.predicate == "legalis:annotationCount");
        assert!(count_triple.is_some());
    }

    #[test]
    fn test_transcript_sorting() {
        let converter = MediaToRdfConverter::new("http://example.org/");
        let annotations = vec![
            TemporalAnnotation {
                start_time: 10.0,
                end_time: 20.0,
                annotation_type: AnnotationType::Transcript,
                content: "Second".to_string(),
                speaker: None,
                confidence: 0.9,
                tags: Vec::new(),
            },
            TemporalAnnotation {
                start_time: 0.0,
                end_time: 10.0,
                annotation_type: AnnotationType::Transcript,
                content: "First".to_string(),
                speaker: None,
                confidence: 0.9,
                tags: Vec::new(),
            },
        ];

        let transcript = converter.generate_transcript(&annotations);

        // Should be sorted by start time
        assert!(transcript.find("First").unwrap() < transcript.find("Second").unwrap());
    }
}
