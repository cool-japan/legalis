//! Automatic format detection for legal documents.
//!
//! This module provides capabilities for:
//! - Automatic format detection based on content analysis
//! - Encoding detection (UTF-8, UTF-16, etc.)
//! - Format version detection
//! - Mixed format handling
//! - Format recommendation based on content characteristics

use crate::{InteropError, InteropResult, LegalFormat};
use std::collections::HashMap;

/// Encoding types supported for detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    /// UTF-8 encoding
    Utf8,
    /// UTF-16 Little Endian
    Utf16Le,
    /// UTF-16 Big Endian
    Utf16Be,
    /// ASCII
    Ascii,
    /// ISO-8859-1 (Latin-1)
    Latin1,
    /// Unknown encoding
    Unknown,
}

impl Encoding {
    /// Returns the name of the encoding.
    pub fn name(&self) -> &'static str {
        match self {
            Encoding::Utf8 => "UTF-8",
            Encoding::Utf16Le => "UTF-16LE",
            Encoding::Utf16Be => "UTF-16BE",
            Encoding::Ascii => "ASCII",
            Encoding::Latin1 => "ISO-8859-1",
            Encoding::Unknown => "Unknown",
        }
    }
}

/// Format detection result.
#[derive(Debug, Clone)]
pub struct DetectionResult {
    /// Detected format
    pub format: LegalFormat,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f64,
    /// Detected encoding
    pub encoding: Encoding,
    /// Detected version (if available)
    pub version: Option<String>,
    /// Alternative formats with their confidence scores
    pub alternatives: Vec<(LegalFormat, f64)>,
}

/// Format detector.
pub struct FormatDetector {
    /// Format signatures and patterns
    signatures: HashMap<LegalFormat, Vec<String>>,
}

impl FormatDetector {
    /// Creates a new format detector with default signatures.
    pub fn new() -> Self {
        let mut signatures = HashMap::new();

        // Catala signatures
        signatures.insert(
            LegalFormat::Catala,
            vec![
                "declaration".to_string(),
                "scope".to_string(),
                "under condition".to_string(),
                "consequence".to_string(),
                "@".to_string(),
            ],
        );

        // Stipula signatures
        signatures.insert(
            LegalFormat::Stipula,
            vec![
                "party".to_string(),
                "asset".to_string(),
                "agreement".to_string(),
                "obligation".to_string(),
                "transfer".to_string(),
            ],
        );

        // L4 signatures
        signatures.insert(
            LegalFormat::L4,
            vec![
                "MUST".to_string(),
                "MAY".to_string(),
                "SHANT".to_string(),
                "DECIDE".to_string(),
                "WHEN".to_string(),
            ],
        );

        // Akoma Ntoso signatures
        signatures.insert(
            LegalFormat::AkomaNtoso,
            vec![
                "<akomaNtoso".to_string(),
                "<act>".to_string(),
                "<bill>".to_string(),
                "<debate>".to_string(),
                "xmlns=\"http://www.akomantoso.org".to_string(),
            ],
        );

        // LegalRuleML signatures
        signatures.insert(
            LegalFormat::LegalRuleML,
            vec![
                "lrml:".to_string(),
                "<LegalRuleML".to_string(),
                "<Rule".to_string(),
                "<Obligation".to_string(),
                "xmlns:lrml=".to_string(),
            ],
        );

        // LegalDocML signatures
        signatures.insert(
            LegalFormat::LegalDocML,
            vec![
                "<LegalDocument".to_string(),
                "<Document".to_string(),
                "LegalDocML".to_string(),
            ],
        );

        // LKIF signatures
        signatures.insert(
            LegalFormat::LKIF,
            vec![
                "<lkif:".to_string(),
                "<sources>".to_string(),
                "<rules>".to_string(),
                "<theory>".to_string(),
            ],
        );

        // LegalCite signatures
        signatures.insert(
            LegalFormat::LegalCite,
            vec![
                "<LegalCiteDocument".to_string(),
                "<legalCite".to_string(),
                "<citations".to_string(),
            ],
        );

        // MetaLex signatures
        signatures.insert(
            LegalFormat::MetaLex,
            vec![
                "<MetaLexDocument".to_string(),
                "<metalex".to_string(),
                "<BibliographicExpression".to_string(),
            ],
        );

        // MPEG-21 REL signatures
        signatures.insert(
            LegalFormat::Mpeg21Rel,
            vec![
                "<Mpeg21RelDocument".to_string(),
                "<license".to_string(),
                "<grant".to_string(),
                "<right".to_string(),
            ],
        );

        // Creative Commons signatures
        signatures.insert(
            LegalFormat::CreativeCommons,
            vec![
                "creativecommons.org".to_string(),
                "<License".to_string(),
                "<permits".to_string(),
                "<requires".to_string(),
            ],
        );

        // SPDX signatures
        signatures.insert(
            LegalFormat::Spdx,
            vec![
                "SPDX-License-Identifier".to_string(),
                " AND ".to_string(),
                " OR ".to_string(),
                " WITH ".to_string(),
            ],
        );

        Self { signatures }
    }

    /// Detects the format of the given content.
    pub fn detect(&self, content: &str) -> InteropResult<DetectionResult> {
        // Detect encoding first
        let encoding = self.detect_encoding(content.as_bytes());

        // Calculate confidence scores for each format
        let mut scores: Vec<(LegalFormat, f64)> = self
            .signatures
            .iter()
            .map(|(format, patterns)| {
                let score = self.calculate_format_score(content, patterns);
                (*format, score)
            })
            .collect();

        // Sort by confidence (highest first)
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        if scores.is_empty() || scores[0].1 < 0.1 {
            return Err(InteropError::UnsupportedFormat(
                "Unable to detect format".to_string(),
            ));
        }

        let (format, confidence) = scores[0];
        let alternatives = scores[1..].to_vec();

        // Try to detect version
        let version = self.detect_version(content, format);

        Ok(DetectionResult {
            format,
            confidence,
            encoding,
            version,
            alternatives,
        })
    }

    /// Detects encoding from byte content.
    pub fn detect_encoding(&self, bytes: &[u8]) -> Encoding {
        // Check for BOM (Byte Order Mark)
        if bytes.len() >= 3 && bytes[0] == 0xEF && bytes[1] == 0xBB && bytes[2] == 0xBF {
            return Encoding::Utf8;
        }

        if bytes.len() >= 2 {
            if bytes[0] == 0xFF && bytes[1] == 0xFE {
                return Encoding::Utf16Le;
            }
            if bytes[0] == 0xFE && bytes[1] == 0xFF {
                return Encoding::Utf16Be;
            }
        }

        // Check if valid ASCII (all bytes < 128)
        if bytes.iter().all(|&b| b < 128) {
            return Encoding::Ascii;
        }

        // Check if valid UTF-8
        if std::str::from_utf8(bytes).is_ok() {
            return Encoding::Utf8;
        }

        Encoding::Unknown
    }

    fn calculate_format_score(&self, content: &str, patterns: &[String]) -> f64 {
        let mut matches = 0;
        for pattern in patterns {
            if content.contains(pattern) {
                matches += 1;
            }
        }

        if patterns.is_empty() {
            0.0
        } else {
            matches as f64 / patterns.len() as f64
        }
    }

    fn detect_version(&self, content: &str, format: LegalFormat) -> Option<String> {
        match format {
            LegalFormat::AkomaNtoso => {
                // Look for version in XML namespace
                if content.contains("akomantoso.org/3.0") {
                    Some("3.0".to_string())
                } else if content.contains("akomantoso.org/2.0") {
                    Some("2.0".to_string())
                } else {
                    None
                }
            }
            LegalFormat::LegalRuleML => {
                if content.contains("legalruleml.org/1.0") {
                    Some("1.0".to_string())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Recommends the best format for given content characteristics.
    pub fn recommend_format(&self, characteristics: &ContentCharacteristics) -> LegalFormat {
        // Rule-based recommendation
        if characteristics.has_deontic_logic {
            return LegalFormat::L4;
        }

        if characteristics.has_scope_declarations {
            return LegalFormat::Catala;
        }

        if characteristics.has_party_declarations {
            return LegalFormat::Stipula;
        }

        if characteristics.has_xml_structure {
            if characteristics.is_legislative_document {
                return LegalFormat::AkomaNtoso;
            } else {
                return LegalFormat::LegalDocML;
            }
        }

        // Default to Legalis
        LegalFormat::Legalis
    }
}

impl Default for FormatDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Content characteristics for format recommendation.
#[derive(Debug, Clone, Default)]
pub struct ContentCharacteristics {
    /// Whether content contains deontic logic (MUST, MAY, SHANT)
    pub has_deontic_logic: bool,
    /// Whether content has scope declarations
    pub has_scope_declarations: bool,
    /// Whether content has party declarations
    pub has_party_declarations: bool,
    /// Whether content is XML-structured
    pub has_xml_structure: bool,
    /// Whether content is a legislative document
    pub is_legislative_document: bool,
    /// Whether content has temporal logic
    pub has_temporal_logic: bool,
    /// Whether content has preconditions
    pub has_preconditions: bool,
}

impl ContentCharacteristics {
    /// Analyzes content to extract characteristics.
    pub fn analyze(content: &str) -> Self {
        Self {
            has_deontic_logic: content.contains("MUST")
                || content.contains("MAY")
                || content.contains("SHANT"),
            has_scope_declarations: content.contains("scope") || content.contains("declaration"),
            has_party_declarations: content.contains("party") || content.contains("agreement"),
            has_xml_structure: content.trim_start().starts_with('<'),
            is_legislative_document: content.contains("<act>")
                || content.contains("<bill>")
                || content.contains("legislation"),
            has_temporal_logic: content.contains("WHEN")
                || content.contains("UNTIL")
                || content.contains("AFTER"),
            has_preconditions: content.contains("IF")
                || content.contains("condition")
                || content.contains("GIVEN"),
        }
    }
}

/// Mixed format handler for documents containing multiple formats.
pub struct MixedFormatHandler;

impl MixedFormatHandler {
    /// Creates a new mixed format handler.
    pub fn new() -> Self {
        Self
    }

    /// Detects if content contains mixed formats.
    pub fn detect_mixed(&self, content: &str) -> Vec<(LegalFormat, usize, usize)> {
        let mut formats = Vec::new();
        let detector = FormatDetector::new();

        // Split content into sections and detect format for each
        let sections = self.split_into_sections(content);

        for (start, end, section) in sections {
            if let Ok(result) = detector.detect(section) {
                if result.confidence > 0.3 {
                    formats.push((result.format, start, end));
                }
            }
        }

        formats
    }

    fn split_into_sections<'a>(&self, content: &'a str) -> Vec<(usize, usize, &'a str)> {
        let mut sections = Vec::new();
        let mut start = 0;

        // Simple splitting by empty lines or format markers
        for (idx, line) in content.lines().enumerate() {
            if line.trim().is_empty() || line.starts_with("---") {
                if start < idx {
                    let section_start = content.lines().take(start).map(|l| l.len() + 1).sum();
                    let section_end = content.lines().take(idx).map(|l| l.len() + 1).sum();
                    if let Some(section) = content.get(section_start..section_end) {
                        sections.push((section_start, section_end, section));
                    }
                }
                start = idx + 1;
            }
        }

        // Add final section
        if start < content.lines().count() {
            let section_start = content.lines().take(start).map(|l| l.len() + 1).sum();
            if let Some(section) = content.get(section_start..) {
                sections.push((section_start, content.len(), section));
            }
        }

        sections
    }
}

impl Default for MixedFormatHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoding_detection_utf8() {
        let detector = FormatDetector::new();
        let content = "Hello World";
        let encoding = detector.detect_encoding(content.as_bytes());
        assert_eq!(encoding, Encoding::Ascii);
    }

    #[test]
    fn test_encoding_detection_utf8_bom() {
        let detector = FormatDetector::new();
        let bytes = vec![0xEF, 0xBB, 0xBF, b'H', b'i'];
        let encoding = detector.detect_encoding(&bytes);
        assert_eq!(encoding, Encoding::Utf8);
    }

    #[test]
    fn test_format_detection_catala() {
        let detector = FormatDetector::new();
        let content = r#"
            declaration scope TaxCalculation:
                input income content money
                output tax content money

            scope TaxCalculation:
                definition tax under condition income >= 50000
                consequence equals income * 0.3
        "#;

        let result = detector.detect(content).unwrap();
        assert_eq!(result.format, LegalFormat::Catala);
        assert!(result.confidence > 0.5);
    }

    #[test]
    fn test_format_detection_l4() {
        let detector = FormatDetector::new();
        let content = r#"
            Person MUST register
            WHEN age >= 18
            Company MAY apply for exemption
        "#;

        let result = detector.detect(content).unwrap();
        assert_eq!(result.format, LegalFormat::L4);
        assert!(result.confidence > 0.5);
    }

    #[test]
    fn test_format_detection_akoma_ntoso() {
        let detector = FormatDetector::new();
        let content = r#"<?xml version="1.0"?>
            <akomaNtoso xmlns="http://www.akomantoso.org/3.0">
                <act>
                    <body>
                        <section>Test</section>
                    </body>
                </act>
            </akomaNtoso>
        "#;

        let result = detector.detect(content).unwrap();
        assert_eq!(result.format, LegalFormat::AkomaNtoso);
        assert!(result.confidence > 0.5);
        assert_eq!(result.version, Some("3.0".to_string()));
    }

    #[test]
    fn test_content_characteristics() {
        let content = "Person MUST register WHEN age >= 18";
        let chars = ContentCharacteristics::analyze(content);

        assert!(chars.has_deontic_logic);
        assert!(chars.has_temporal_logic);
    }

    #[test]
    fn test_format_recommendation() {
        let detector = FormatDetector::new();

        let chars = ContentCharacteristics {
            has_deontic_logic: true,
            ..Default::default()
        };
        assert_eq!(detector.recommend_format(&chars), LegalFormat::L4);

        let chars2 = ContentCharacteristics {
            has_scope_declarations: true,
            ..Default::default()
        };
        assert_eq!(detector.recommend_format(&chars2), LegalFormat::Catala);
    }

    #[test]
    fn test_mixed_format_detection() {
        let handler = MixedFormatHandler::new();
        let content = r#"
            declaration scope Tax:
                input income content money

            ---

            Person MUST register
            WHEN age >= 18
        "#;

        let formats = handler.detect_mixed(content);
        assert!(!formats.is_empty());
    }
}
