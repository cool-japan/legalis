//! Document layout representation for knowledge graphs.
//!
//! This module provides functionality for extracting and representing
//! document layout structure in RDF format.

use crate::{RdfValue, Triple};
use std::collections::HashMap;

/// Represents a document with layout information.
#[derive(Debug, Clone)]
pub struct LayoutDocument {
    /// Document identifier
    pub id: String,
    /// Document source
    pub source: String,
    /// Number of pages
    pub page_count: usize,
    /// Pages with layout information
    pub pages: Vec<Page>,
}

/// Represents a page in a document.
#[derive(Debug, Clone)]
pub struct Page {
    /// Page number (1-indexed)
    pub page_number: usize,
    /// Page width
    pub width: f64,
    /// Page height
    pub height: f64,
    /// Layout zones/regions
    pub zones: Vec<LayoutZone>,
}

/// A layout zone or region on a page.
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutZone {
    /// Zone type
    pub zone_type: ZoneType,
    /// Bounding box (x, y, width, height)
    pub bbox: (f64, f64, f64, f64),
    /// Reading order (lower numbers come first)
    pub reading_order: Option<usize>,
    /// Text content (if extracted)
    pub content: Option<String>,
    /// Child zones (for hierarchical layouts)
    pub children: Vec<LayoutZone>,
}

/// Type of layout zone.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ZoneType {
    /// Header
    Header,
    /// Footer
    Footer,
    /// Main content body
    Body,
    /// Sidebar
    Sidebar,
    /// Title or heading
    Title,
    /// Paragraph
    Paragraph,
    /// List
    List,
    /// Table
    Table,
    /// Figure or image
    Figure,
    /// Caption
    Caption,
    /// Footnote
    Footnote,
    /// Page number
    PageNumber,
    /// Custom zone type
    Custom(String),
}

impl ZoneType {
    /// Converts to RDF class URI.
    pub fn to_rdf_class(&self) -> String {
        match self {
            ZoneType::Header => "legalis:HeaderZone".to_string(),
            ZoneType::Footer => "legalis:FooterZone".to_string(),
            ZoneType::Body => "legalis:BodyZone".to_string(),
            ZoneType::Sidebar => "legalis:SidebarZone".to_string(),
            ZoneType::Title => "legalis:TitleZone".to_string(),
            ZoneType::Paragraph => "legalis:ParagraphZone".to_string(),
            ZoneType::List => "legalis:ListZone".to_string(),
            ZoneType::Table => "legalis:TableZone".to_string(),
            ZoneType::Figure => "legalis:FigureZone".to_string(),
            ZoneType::Caption => "legalis:CaptionZone".to_string(),
            ZoneType::Footnote => "legalis:FootnoteZone".to_string(),
            ZoneType::PageNumber => "legalis:PageNumberZone".to_string(),
            ZoneType::Custom(s) => format!("legalis:{}Zone", s.replace(' ', "")),
        }
    }
}

/// Layout-to-RDF converter.
pub struct LayoutToRdfConverter {
    /// Base URI for generated resources
    base_uri: String,
}

impl LayoutToRdfConverter {
    /// Creates a new layout-to-RDF converter.
    pub fn new(base_uri: impl Into<String>) -> Self {
        Self {
            base_uri: base_uri.into(),
        }
    }

    /// Converts a layout document to RDF triples.
    pub fn convert(&self, document: &LayoutDocument) -> Vec<Triple> {
        let mut triples = Vec::new();

        // Document metadata
        let doc_uri = format!("{}document/{}", self.base_uri, document.id);

        triples.push(Triple {
            subject: doc_uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("legalis:LayoutDocument".to_string()),
        });

        triples.push(Triple {
            subject: doc_uri.clone(),
            predicate: "dcterms:identifier".to_string(),
            object: RdfValue::string(&document.id),
        });

        triples.push(Triple {
            subject: doc_uri.clone(),
            predicate: "legalis:source".to_string(),
            object: RdfValue::string(&document.source),
        });

        triples.push(Triple {
            subject: doc_uri.clone(),
            predicate: "legalis:pageCount".to_string(),
            object: RdfValue::integer(document.page_count as i64),
        });

        // Pages
        for page in &document.pages {
            let page_uri = format!("{}page/{}/{}", self.base_uri, document.id, page.page_number);

            triples.push(Triple {
                subject: page_uri.clone(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri("legalis:Page".to_string()),
            });

            triples.push(Triple {
                subject: page_uri.clone(),
                predicate: "legalis:pageNumber".to_string(),
                object: RdfValue::integer(page.page_number as i64),
            });

            triples.push(Triple {
                subject: page_uri.clone(),
                predicate: "legalis:width".to_string(),
                object: RdfValue::TypedLiteral(page.width.to_string(), "xsd:double".to_string()),
            });

            triples.push(Triple {
                subject: page_uri.clone(),
                predicate: "legalis:height".to_string(),
                object: RdfValue::TypedLiteral(page.height.to_string(), "xsd:double".to_string()),
            });

            triples.push(Triple {
                subject: doc_uri.clone(),
                predicate: "legalis:hasPage".to_string(),
                object: RdfValue::Uri(page_uri.clone()),
            });

            // Zones
            for (zone_idx, zone) in page.zones.iter().enumerate() {
                self.convert_zone(
                    &mut triples,
                    zone,
                    &page_uri,
                    &document.id,
                    page.page_number,
                    zone_idx,
                    None,
                );
            }
        }

        triples
    }

    #[allow(clippy::too_many_arguments)]
    fn convert_zone(
        &self,
        triples: &mut Vec<Triple>,
        zone: &LayoutZone,
        page_uri: &str,
        doc_id: &str,
        page_num: usize,
        zone_idx: usize,
        parent_idx: Option<usize>,
    ) {
        let zone_uri = if let Some(parent) = parent_idx {
            format!(
                "{}zone/{}/{}/{}/{}",
                self.base_uri, doc_id, page_num, parent, zone_idx
            )
        } else {
            format!("{}zone/{}/{}/{}", self.base_uri, doc_id, page_num, zone_idx)
        };

        triples.push(Triple {
            subject: zone_uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri(zone.zone_type.to_rdf_class()),
        });

        triples.push(Triple {
            subject: zone_uri.clone(),
            predicate: "legalis:inPage".to_string(),
            object: RdfValue::Uri(page_uri.to_string()),
        });

        // Bounding box
        triples.push(Triple {
            subject: zone_uri.clone(),
            predicate: "legalis:boundingBoxX".to_string(),
            object: RdfValue::TypedLiteral(zone.bbox.0.to_string(), "xsd:double".to_string()),
        });

        triples.push(Triple {
            subject: zone_uri.clone(),
            predicate: "legalis:boundingBoxY".to_string(),
            object: RdfValue::TypedLiteral(zone.bbox.1.to_string(), "xsd:double".to_string()),
        });

        triples.push(Triple {
            subject: zone_uri.clone(),
            predicate: "legalis:boundingBoxWidth".to_string(),
            object: RdfValue::TypedLiteral(zone.bbox.2.to_string(), "xsd:double".to_string()),
        });

        triples.push(Triple {
            subject: zone_uri.clone(),
            predicate: "legalis:boundingBoxHeight".to_string(),
            object: RdfValue::TypedLiteral(zone.bbox.3.to_string(), "xsd:double".to_string()),
        });

        // Reading order
        if let Some(order) = zone.reading_order {
            triples.push(Triple {
                subject: zone_uri.clone(),
                predicate: "legalis:readingOrder".to_string(),
                object: RdfValue::integer(order as i64),
            });
        }

        // Content
        if let Some(ref content) = zone.content {
            triples.push(Triple {
                subject: zone_uri.clone(),
                predicate: "legalis:textContent".to_string(),
                object: RdfValue::string(content),
            });
        }

        // Children
        for (child_idx, child) in zone.children.iter().enumerate() {
            self.convert_zone(
                triples,
                child,
                page_uri,
                doc_id,
                page_num,
                child_idx,
                Some(zone_idx),
            );

            let child_uri = format!(
                "{}zone/{}/{}/{}/{}",
                self.base_uri, doc_id, page_num, zone_idx, child_idx
            );

            triples.push(Triple {
                subject: zone_uri.clone(),
                predicate: "legalis:hasChildZone".to_string(),
                object: RdfValue::Uri(child_uri),
            });
        }
    }

    /// Extracts text in reading order from a document.
    pub fn extract_reading_order_text(&self, document: &LayoutDocument) -> String {
        let mut text_parts = Vec::new();

        for page in &document.pages {
            let mut zones_with_order: Vec<(&LayoutZone, usize)> = page
                .zones
                .iter()
                .filter_map(|z| z.reading_order.map(|o| (z, o)))
                .collect();

            zones_with_order.sort_by_key(|(_, order)| *order);

            for (zone, _) in zones_with_order {
                if let Some(ref content) = zone.content {
                    text_parts.push(content.clone());
                }
                self.collect_child_text(zone, &mut text_parts);
            }
        }

        text_parts.join("\n")
    }

    fn collect_child_text(&self, zone: &LayoutZone, text_parts: &mut Vec<String>) {
        for child in &zone.children {
            if let Some(ref content) = child.content {
                text_parts.push(content.clone());
            }
            self.collect_child_text(child, text_parts);
        }
    }
}

/// Statistics about document layout.
#[derive(Debug, Clone)]
pub struct LayoutStats {
    /// Total number of documents
    pub total_documents: usize,
    /// Total number of pages
    pub total_pages: usize,
    /// Total number of zones
    pub total_zones: usize,
    /// Zones by type
    pub by_type: HashMap<ZoneType, usize>,
}

impl LayoutStats {
    /// Creates statistics from layout documents.
    pub fn from_documents(documents: &[LayoutDocument]) -> Self {
        let mut total_pages = 0;
        let mut total_zones = 0;
        let mut by_type: HashMap<ZoneType, usize> = HashMap::new();

        for doc in documents {
            total_pages += doc.pages.len();
            for page in &doc.pages {
                for zone in &page.zones {
                    Self::count_zones(zone, &mut total_zones, &mut by_type);
                }
            }
        }

        Self {
            total_documents: documents.len(),
            total_pages,
            total_zones,
            by_type,
        }
    }

    fn count_zones(zone: &LayoutZone, total: &mut usize, by_type: &mut HashMap<ZoneType, usize>) {
        *total += 1;
        *by_type.entry(zone.zone_type.clone()).or_insert(0) += 1;

        for child in &zone.children {
            Self::count_zones(child, total, by_type);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_document() -> LayoutDocument {
        LayoutDocument {
            id: "test-doc-001".to_string(),
            source: "/path/to/document.pdf".to_string(),
            page_count: 2,
            pages: vec![
                Page {
                    page_number: 1,
                    width: 612.0,
                    height: 792.0,
                    zones: vec![LayoutZone {
                        zone_type: ZoneType::Title,
                        bbox: (50.0, 50.0, 512.0, 50.0),
                        reading_order: Some(1),
                        content: Some("Document Title".to_string()),
                        children: Vec::new(),
                    }],
                },
                Page {
                    page_number: 2,
                    width: 612.0,
                    height: 792.0,
                    zones: Vec::new(),
                },
            ],
        }
    }

    #[test]
    fn test_layout_document_creation() {
        let doc = create_test_document();
        assert_eq!(doc.id, "test-doc-001");
        assert_eq!(doc.page_count, 2);
        assert_eq!(doc.pages.len(), 2);
    }

    #[test]
    fn test_zone_type_to_rdf() {
        assert_eq!(ZoneType::Header.to_rdf_class(), "legalis:HeaderZone");
        assert_eq!(ZoneType::Title.to_rdf_class(), "legalis:TitleZone");
        assert_eq!(ZoneType::Table.to_rdf_class(), "legalis:TableZone");
    }

    #[test]
    fn test_converter_creation() {
        let converter = LayoutToRdfConverter::new("http://example.org/");
        assert_eq!(converter.base_uri, "http://example.org/");
    }

    #[test]
    fn test_convert_to_rdf() {
        let converter = LayoutToRdfConverter::new("http://example.org/");
        let document = create_test_document();

        let triples = converter.convert(&document);

        assert!(!triples.is_empty());
        assert!(triples.iter().any(|t| t.predicate == "rdf:type"
            && matches!(&t.object, RdfValue::Uri(u) if u == "legalis:LayoutDocument")));
        assert!(triples.iter().any(|t| t.predicate == "legalis:pageCount"));
    }

    #[test]
    fn test_page_conversion() {
        let converter = LayoutToRdfConverter::new("http://example.org/");
        let document = create_test_document();

        let triples = converter.convert(&document);

        assert!(triples.iter().any(|t| t.predicate == "legalis:hasPage"));
        assert!(triples.iter().any(|t| t.predicate == "legalis:pageNumber"));
        assert!(triples.iter().any(|t| t.predicate == "legalis:width"));
        assert!(triples.iter().any(|t| t.predicate == "legalis:height"));
    }

    #[test]
    fn test_zone_conversion() {
        let converter = LayoutToRdfConverter::new("http://example.org/");
        let document = create_test_document();

        let triples = converter.convert(&document);

        assert!(triples.iter().any(|t| t.predicate == "legalis:inPage"));
        assert!(
            triples
                .iter()
                .any(|t| t.predicate == "legalis:boundingBoxX")
        );
        assert!(
            triples
                .iter()
                .any(|t| t.predicate == "legalis:readingOrder")
        );
        assert!(triples.iter().any(|t| t.predicate == "legalis:textContent"));
    }

    #[test]
    fn test_hierarchical_zones() {
        let converter = LayoutToRdfConverter::new("http://example.org/");
        let document = LayoutDocument {
            id: "test-doc".to_string(),
            source: "/path/to/doc.pdf".to_string(),
            page_count: 1,
            pages: vec![Page {
                page_number: 1,
                width: 612.0,
                height: 792.0,
                zones: vec![LayoutZone {
                    zone_type: ZoneType::Body,
                    bbox: (0.0, 0.0, 612.0, 792.0),
                    reading_order: Some(1),
                    content: None,
                    children: vec![LayoutZone {
                        zone_type: ZoneType::Paragraph,
                        bbox: (50.0, 50.0, 512.0, 100.0),
                        reading_order: Some(2),
                        content: Some("Paragraph text".to_string()),
                        children: Vec::new(),
                    }],
                }],
            }],
        };

        let triples = converter.convert(&document);

        assert!(
            triples
                .iter()
                .any(|t| t.predicate == "legalis:hasChildZone")
        );
    }

    #[test]
    fn test_extract_reading_order_text() {
        let converter = LayoutToRdfConverter::new("http://example.org/");
        let document = LayoutDocument {
            id: "test-doc".to_string(),
            source: "/path/to/doc.pdf".to_string(),
            page_count: 1,
            pages: vec![Page {
                page_number: 1,
                width: 612.0,
                height: 792.0,
                zones: vec![
                    LayoutZone {
                        zone_type: ZoneType::Title,
                        bbox: (0.0, 0.0, 100.0, 20.0),
                        reading_order: Some(1),
                        content: Some("Title".to_string()),
                        children: Vec::new(),
                    },
                    LayoutZone {
                        zone_type: ZoneType::Paragraph,
                        bbox: (0.0, 30.0, 100.0, 50.0),
                        reading_order: Some(2),
                        content: Some("Body".to_string()),
                        children: Vec::new(),
                    },
                ],
            }],
        };

        let text = converter.extract_reading_order_text(&document);

        assert!(text.contains("Title"));
        assert!(text.contains("Body"));
        assert!(text.find("Title").unwrap() < text.find("Body").unwrap());
    }

    #[test]
    fn test_layout_stats() {
        let documents = vec![create_test_document()];
        let stats = LayoutStats::from_documents(&documents);

        assert_eq!(stats.total_documents, 1);
        assert_eq!(stats.total_pages, 2);
        assert_eq!(stats.total_zones, 1);
        assert_eq!(stats.by_type.len(), 1);
    }

    #[test]
    fn test_custom_zone_type() {
        let custom = ZoneType::Custom("Signature".to_string());
        assert_eq!(custom.to_rdf_class(), "legalis:SignatureZone");
    }

    #[test]
    fn test_multiple_zones_per_page() {
        let converter = LayoutToRdfConverter::new("http://example.org/");
        let document = LayoutDocument {
            id: "test-doc".to_string(),
            source: "/path/to/doc.pdf".to_string(),
            page_count: 1,
            pages: vec![Page {
                page_number: 1,
                width: 612.0,
                height: 792.0,
                zones: vec![
                    LayoutZone {
                        zone_type: ZoneType::Header,
                        bbox: (0.0, 0.0, 612.0, 50.0),
                        reading_order: Some(1),
                        content: Some("Header".to_string()),
                        children: Vec::new(),
                    },
                    LayoutZone {
                        zone_type: ZoneType::Body,
                        bbox: (0.0, 50.0, 612.0, 692.0),
                        reading_order: Some(2),
                        content: Some("Body".to_string()),
                        children: Vec::new(),
                    },
                    LayoutZone {
                        zone_type: ZoneType::Footer,
                        bbox: (0.0, 742.0, 612.0, 50.0),
                        reading_order: Some(3),
                        content: Some("Footer".to_string()),
                        children: Vec::new(),
                    },
                ],
            }],
        };

        let triples = converter.convert(&document);

        // Should have triples for all three zones
        let zone_count = triples
            .iter()
            .filter(|t| t.predicate == "legalis:inPage")
            .count();
        assert_eq!(zone_count, 3);
    }

    #[test]
    fn test_zone_without_content() {
        let converter = LayoutToRdfConverter::new("http://example.org/");
        let document = LayoutDocument {
            id: "test-doc".to_string(),
            source: "/path/to/doc.pdf".to_string(),
            page_count: 1,
            pages: vec![Page {
                page_number: 1,
                width: 612.0,
                height: 792.0,
                zones: vec![LayoutZone {
                    zone_type: ZoneType::Figure,
                    bbox: (0.0, 0.0, 100.0, 100.0),
                    reading_order: Some(1),
                    content: None, // No text content for figures
                    children: Vec::new(),
                }],
            }],
        };

        let triples = converter.convert(&document);

        // Should not have textContent triple
        assert!(!triples.iter().any(|t| t.predicate == "legalis:textContent"));
    }
}
