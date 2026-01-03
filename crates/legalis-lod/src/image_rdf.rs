//! Image-to-RDF extraction for legal documents.
//!
//! This module provides functionality for extracting structured information
//! from images (diagrams, charts, scanned documents) and converting to RDF.

use crate::{RdfValue, Triple};
use std::collections::HashMap;

/// Represents an image with metadata.
#[derive(Debug, Clone)]
pub struct ImageDocument {
    /// Image identifier
    pub id: String,
    /// Image path or URL
    pub source: String,
    /// Image width in pixels
    pub width: Option<u32>,
    /// Image height in pixels
    pub height: Option<u32>,
    /// Image format (PNG, JPEG, etc.)
    pub format: Option<String>,
    /// Creation date
    pub created: Option<String>,
}

/// Extracted visual element from an image.
#[derive(Debug, Clone, PartialEq)]
pub struct VisualElement {
    /// Element type (text, diagram, table, signature, etc.)
    pub element_type: ElementType,
    /// Bounding box (x, y, width, height)
    pub bbox: BoundingBox,
    /// Extracted content/text
    pub content: Option<String>,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Additional properties
    pub properties: HashMap<String, String>,
}

/// Type of visual element.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ElementType {
    /// Text block
    Text,
    /// Heading or title
    Heading,
    /// Table
    Table,
    /// Diagram or chart
    Diagram,
    /// Signature
    Signature,
    /// Seal or stamp
    Seal,
    /// Logo
    Logo,
    /// Image or photo
    Image,
    /// Form field
    FormField,
    /// Custom type
    Custom(String),
}

impl ElementType {
    /// Converts to RDF class URI.
    pub fn to_rdf_class(&self) -> String {
        match self {
            ElementType::Text => "legalis:TextElement".to_string(),
            ElementType::Heading => "legalis:HeadingElement".to_string(),
            ElementType::Table => "legalis:TableElement".to_string(),
            ElementType::Diagram => "legalis:DiagramElement".to_string(),
            ElementType::Signature => "legalis:SignatureElement".to_string(),
            ElementType::Seal => "legalis:SealElement".to_string(),
            ElementType::Logo => "legalis:LogoElement".to_string(),
            ElementType::Image => "legalis:ImageElement".to_string(),
            ElementType::FormField => "legalis:FormFieldElement".to_string(),
            ElementType::Custom(s) => format!("legalis:{}", s.replace(' ', "")),
        }
    }
}

/// Bounding box for visual elements.
#[derive(Debug, Clone, PartialEq)]
pub struct BoundingBox {
    /// X coordinate (top-left)
    pub x: f64,
    /// Y coordinate (top-left)
    pub y: f64,
    /// Width
    pub width: f64,
    /// Height
    pub height: f64,
}

impl BoundingBox {
    /// Creates a new bounding box.
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Calculates the area of the bounding box.
    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    /// Checks if this box overlaps with another.
    pub fn overlaps(&self, other: &BoundingBox) -> bool {
        !(self.x + self.width < other.x
            || other.x + other.width < self.x
            || self.y + self.height < other.y
            || other.y + other.height < self.y)
    }

    /// Calculates intersection-over-union (IoU) with another box.
    pub fn iou(&self, other: &BoundingBox) -> f64 {
        if !self.overlaps(other) {
            return 0.0;
        }

        let x_left = self.x.max(other.x);
        let y_top = self.y.max(other.y);
        let x_right = (self.x + self.width).min(other.x + other.width);
        let y_bottom = (self.y + self.height).min(other.y + other.height);

        let intersection = (x_right - x_left) * (y_bottom - y_top);
        let union = self.area() + other.area() - intersection;

        intersection / union
    }
}

/// Image-to-RDF converter.
pub struct ImageToRdfConverter {
    /// Base URI for generated resources
    base_uri: String,
    /// Minimum confidence threshold
    confidence_threshold: f64,
}

impl ImageToRdfConverter {
    /// Creates a new image-to-RDF converter.
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

    /// Converts an image document with extracted elements to RDF triples.
    pub fn convert(&self, image: &ImageDocument, elements: &[VisualElement]) -> Vec<Triple> {
        let mut triples = Vec::new();

        // Image document metadata
        let image_uri = format!("{}image/{}", self.base_uri, image.id);

        triples.push(Triple {
            subject: image_uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("legalis:ImageDocument".to_string()),
        });

        triples.push(Triple {
            subject: image_uri.clone(),
            predicate: "dcterms:identifier".to_string(),
            object: RdfValue::string(&image.id),
        });

        triples.push(Triple {
            subject: image_uri.clone(),
            predicate: "legalis:source".to_string(),
            object: RdfValue::string(&image.source),
        });

        if let Some(width) = image.width {
            triples.push(Triple {
                subject: image_uri.clone(),
                predicate: "legalis:width".to_string(),
                object: RdfValue::integer(width as i64),
            });
        }

        if let Some(height) = image.height {
            triples.push(Triple {
                subject: image_uri.clone(),
                predicate: "legalis:height".to_string(),
                object: RdfValue::integer(height as i64),
            });
        }

        if let Some(ref format) = image.format {
            triples.push(Triple {
                subject: image_uri.clone(),
                predicate: "dcterms:format".to_string(),
                object: RdfValue::string(format),
            });
        }

        // Visual elements
        for (idx, element) in elements.iter().enumerate() {
            if element.confidence < self.confidence_threshold {
                continue;
            }

            let element_uri = format!("{}element/{}/{}", self.base_uri, image.id, idx);

            // Element type
            triples.push(Triple {
                subject: element_uri.clone(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri(element.element_type.to_rdf_class()),
            });

            // Link to parent image
            triples.push(Triple {
                subject: element_uri.clone(),
                predicate: "legalis:partOfImage".to_string(),
                object: RdfValue::Uri(image_uri.clone()),
            });

            // Bounding box
            triples.push(Triple {
                subject: element_uri.clone(),
                predicate: "legalis:boundingBoxX".to_string(),
                object: RdfValue::TypedLiteral(
                    element.bbox.x.to_string(),
                    "xsd:double".to_string(),
                ),
            });

            triples.push(Triple {
                subject: element_uri.clone(),
                predicate: "legalis:boundingBoxY".to_string(),
                object: RdfValue::TypedLiteral(
                    element.bbox.y.to_string(),
                    "xsd:double".to_string(),
                ),
            });

            triples.push(Triple {
                subject: element_uri.clone(),
                predicate: "legalis:boundingBoxWidth".to_string(),
                object: RdfValue::TypedLiteral(
                    element.bbox.width.to_string(),
                    "xsd:double".to_string(),
                ),
            });

            triples.push(Triple {
                subject: element_uri.clone(),
                predicate: "legalis:boundingBoxHeight".to_string(),
                object: RdfValue::TypedLiteral(
                    element.bbox.height.to_string(),
                    "xsd:double".to_string(),
                ),
            });

            // Content
            if let Some(ref content) = element.content {
                triples.push(Triple {
                    subject: element_uri.clone(),
                    predicate: "legalis:extractedText".to_string(),
                    object: RdfValue::string(content),
                });
            }

            // Confidence
            triples.push(Triple {
                subject: element_uri.clone(),
                predicate: "legalis:confidence".to_string(),
                object: RdfValue::TypedLiteral(
                    element.confidence.to_string(),
                    "xsd:double".to_string(),
                ),
            });

            // Additional properties
            for (key, value) in &element.properties {
                triples.push(Triple {
                    subject: element_uri.clone(),
                    predicate: format!("legalis:{}", key),
                    object: RdfValue::string(value),
                });
            }
        }

        // Add reverse link from image to elements
        triples.push(Triple {
            subject: image_uri,
            predicate: "legalis:elementCount".to_string(),
            object: RdfValue::integer(
                elements
                    .iter()
                    .filter(|e| e.confidence >= self.confidence_threshold)
                    .count() as i64,
            ),
        });

        triples
    }

    /// Extracts visual elements from an image (placeholder for actual CV/OCR).
    pub fn extract_elements(&self, _image: &ImageDocument) -> Vec<VisualElement> {
        // In a real implementation, this would:
        // 1. Use OCR for text extraction
        // 2. Use object detection for visual elements
        // 3. Use layout analysis for structure
        // For now, return placeholder elements
        Vec::new()
    }
}

/// Statistics about image extraction.
#[derive(Debug, Clone)]
pub struct ExtractionStats {
    /// Total number of images processed
    pub total_images: usize,
    /// Total elements extracted
    pub total_elements: usize,
    /// Elements by type
    pub by_type: HashMap<ElementType, usize>,
    /// Average confidence
    pub avg_confidence: f64,
}

impl ExtractionStats {
    /// Creates statistics from extracted elements.
    pub fn from_elements(images_count: usize, elements: &[VisualElement]) -> Self {
        let mut by_type: HashMap<ElementType, usize> = HashMap::new();
        let mut total_confidence = 0.0;

        for element in elements {
            *by_type.entry(element.element_type.clone()).or_insert(0) += 1;
            total_confidence += element.confidence;
        }

        let avg_confidence = if elements.is_empty() {
            0.0
        } else {
            total_confidence / elements.len() as f64
        };

        Self {
            total_images: images_count,
            total_elements: elements.len(),
            by_type,
            avg_confidence,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_image() -> ImageDocument {
        ImageDocument {
            id: "test-img-001".to_string(),
            source: "/path/to/image.png".to_string(),
            width: Some(800),
            height: Some(600),
            format: Some("PNG".to_string()),
            created: Some("2024-01-01".to_string()),
        }
    }

    fn create_test_element() -> VisualElement {
        VisualElement {
            element_type: ElementType::Text,
            bbox: BoundingBox::new(10.0, 20.0, 100.0, 50.0),
            content: Some("Test content".to_string()),
            confidence: 0.9,
            properties: HashMap::new(),
        }
    }

    #[test]
    fn test_image_document_creation() {
        let img = create_test_image();
        assert_eq!(img.id, "test-img-001");
        assert_eq!(img.width, Some(800));
        assert_eq!(img.height, Some(600));
    }

    #[test]
    fn test_element_type_to_rdf() {
        assert_eq!(ElementType::Text.to_rdf_class(), "legalis:TextElement");
        assert_eq!(ElementType::Table.to_rdf_class(), "legalis:TableElement");
        assert_eq!(
            ElementType::Signature.to_rdf_class(),
            "legalis:SignatureElement"
        );
    }

    #[test]
    fn test_bounding_box_area() {
        let bbox = BoundingBox::new(0.0, 0.0, 100.0, 50.0);
        assert_eq!(bbox.area(), 5000.0);
    }

    #[test]
    fn test_bounding_box_overlap() {
        let bbox1 = BoundingBox::new(0.0, 0.0, 100.0, 100.0);
        let bbox2 = BoundingBox::new(50.0, 50.0, 100.0, 100.0);
        let bbox3 = BoundingBox::new(200.0, 200.0, 100.0, 100.0);

        assert!(bbox1.overlaps(&bbox2));
        assert!(!bbox1.overlaps(&bbox3));
    }

    #[test]
    fn test_bounding_box_iou() {
        let bbox1 = BoundingBox::new(0.0, 0.0, 100.0, 100.0);
        let bbox2 = BoundingBox::new(50.0, 50.0, 100.0, 100.0);

        let iou = bbox1.iou(&bbox2);
        assert!(iou > 0.0 && iou < 1.0);

        let bbox3 = BoundingBox::new(0.0, 0.0, 100.0, 100.0);
        let iou_same = bbox1.iou(&bbox3);
        assert!((iou_same - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_converter_creation() {
        let converter = ImageToRdfConverter::new("http://example.org/");
        assert_eq!(converter.base_uri, "http://example.org/");
        assert_eq!(converter.confidence_threshold, 0.5);
    }

    #[test]
    fn test_converter_with_threshold() {
        let converter = ImageToRdfConverter::new("http://example.org/").with_threshold(0.8);
        assert_eq!(converter.confidence_threshold, 0.8);
    }

    #[test]
    fn test_convert_to_rdf() {
        let converter = ImageToRdfConverter::new("http://example.org/");
        let image = create_test_image();
        let elements = vec![create_test_element()];

        let triples = converter.convert(&image, &elements);

        assert!(!triples.is_empty());
        assert!(triples.iter().any(|t| t.predicate == "rdf:type"
            && matches!(&t.object, RdfValue::Uri(u) if u == "legalis:ImageDocument")));
        assert!(triples.iter().any(|t| t.predicate == "dcterms:identifier"));
        assert!(triples.iter().any(|t| t.predicate == "legalis:width"));
    }

    #[test]
    fn test_element_conversion() {
        let converter = ImageToRdfConverter::new("http://example.org/");
        let image = create_test_image();
        let elements = vec![create_test_element()];

        let triples = converter.convert(&image, &elements);

        assert!(triples.iter().any(|t| t.predicate == "legalis:partOfImage"));
        assert!(
            triples
                .iter()
                .any(|t| t.predicate == "legalis:boundingBoxX")
        );
        assert!(
            triples
                .iter()
                .any(|t| t.predicate == "legalis:extractedText")
        );
        assert!(triples.iter().any(|t| t.predicate == "legalis:confidence"));
    }

    #[test]
    fn test_confidence_filtering() {
        let converter = ImageToRdfConverter::new("http://example.org/").with_threshold(0.95);
        let image = create_test_image();
        let elements = vec![
            VisualElement {
                element_type: ElementType::Text,
                bbox: BoundingBox::new(0.0, 0.0, 10.0, 10.0),
                content: Some("Low confidence".to_string()),
                confidence: 0.5,
                properties: HashMap::new(),
            },
            VisualElement {
                element_type: ElementType::Text,
                bbox: BoundingBox::new(0.0, 0.0, 10.0, 10.0),
                content: Some("High confidence".to_string()),
                confidence: 0.99,
                properties: HashMap::new(),
            },
        ];

        let triples = converter.convert(&image, &elements);

        // Should only include high-confidence element
        let element_count = triples
            .iter()
            .filter(|t| t.predicate == "legalis:extractedText")
            .count();
        assert_eq!(element_count, 1);
    }

    #[test]
    fn test_extraction_stats() {
        let elements = vec![
            VisualElement {
                element_type: ElementType::Text,
                bbox: BoundingBox::new(0.0, 0.0, 10.0, 10.0),
                content: None,
                confidence: 0.8,
                properties: HashMap::new(),
            },
            VisualElement {
                element_type: ElementType::Table,
                bbox: BoundingBox::new(0.0, 0.0, 10.0, 10.0),
                content: None,
                confidence: 0.9,
                properties: HashMap::new(),
            },
        ];

        let stats = ExtractionStats::from_elements(1, &elements);

        assert_eq!(stats.total_images, 1);
        assert_eq!(stats.total_elements, 2);
        assert!((stats.avg_confidence - 0.85).abs() < 0.001);
        assert_eq!(stats.by_type.len(), 2);
    }

    #[test]
    fn test_visual_element_with_properties() {
        let mut element = create_test_element();
        element
            .properties
            .insert("language".to_string(), "en".to_string());
        element
            .properties
            .insert("font".to_string(), "Arial".to_string());

        let converter = ImageToRdfConverter::new("http://example.org/");
        let image = create_test_image();
        let triples = converter.convert(&image, &[element]);

        assert!(triples.iter().any(|t| t.predicate == "legalis:language"));
        assert!(triples.iter().any(|t| t.predicate == "legalis:font"));
    }

    #[test]
    fn test_multiple_elements() {
        let converter = ImageToRdfConverter::new("http://example.org/");
        let image = create_test_image();
        let elements = vec![
            VisualElement {
                element_type: ElementType::Heading,
                bbox: BoundingBox::new(0.0, 0.0, 200.0, 50.0),
                content: Some("Title".to_string()),
                confidence: 0.95,
                properties: HashMap::new(),
            },
            VisualElement {
                element_type: ElementType::Text,
                bbox: BoundingBox::new(0.0, 60.0, 200.0, 100.0),
                content: Some("Body text".to_string()),
                confidence: 0.9,
                properties: HashMap::new(),
            },
        ];

        let triples = converter.convert(&image, &elements);

        // Check element count
        let count_triple = triples
            .iter()
            .find(|t| t.predicate == "legalis:elementCount");
        assert!(count_triple.is_some());
    }

    #[test]
    fn test_custom_element_type() {
        let custom_type = ElementType::Custom("Annotation".to_string());
        assert_eq!(custom_type.to_rdf_class(), "legalis:Annotation");
    }

    #[test]
    fn test_empty_elements() {
        let converter = ImageToRdfConverter::new("http://example.org/");
        let image = create_test_image();
        let triples = converter.convert(&image, &[]);

        // Should still have image metadata
        assert!(
            triples
                .iter()
                .any(|t| matches!(&t.object, RdfValue::Uri(u) if u == "legalis:ImageDocument"))
        );

        // Element count should be 0
        let count_triple = triples
            .iter()
            .find(|t| t.predicate == "legalis:elementCount");
        assert!(count_triple.is_some());
    }

    #[test]
    fn test_extract_elements_placeholder() {
        let converter = ImageToRdfConverter::new("http://example.org/");
        let image = create_test_image();
        let elements = converter.extract_elements(&image);

        // Placeholder returns empty
        assert!(elements.is_empty());
    }
}
