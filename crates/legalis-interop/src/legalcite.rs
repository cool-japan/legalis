//! OASIS LegalCite format import/export.
//!
//! LegalCite is an OASIS standard for citation of legal resources
//! (TC LegalCiteM). It provides structured citations for legal documents,
//! cases, statutes, and regulations.

use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::{Effect, EffectType, Statute};
use serde::{Deserialize, Serialize};

/// LegalCite citation structure (simplified).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
struct LegalCitation {
    /// Citation identifier
    id: String,
    /// Citation title
    title: String,
    /// Citation type (statute, case, regulation, etc.)
    citation_type: String,
    /// Jurisdiction
    #[serde(skip_serializing_if = "Option::is_none")]
    jurisdiction: Option<String>,
    /// Year
    #[serde(skip_serializing_if = "Option::is_none")]
    year: Option<u32>,
    /// Volume
    #[serde(skip_serializing_if = "Option::is_none")]
    volume: Option<String>,
    /// Reporter
    #[serde(skip_serializing_if = "Option::is_none")]
    reporter: Option<String>,
    /// Page
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<String>,
    /// URI
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<String>,
}

/// LegalCite document wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct LegalCiteDocument {
    #[serde(rename = "legalCite")]
    legal_cite: LegalCiteContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LegalCiteContent {
    citations: Vec<LegalCitation>,
}

/// Importer for OASIS LegalCite format.
pub struct LegalCiteImporter;

impl LegalCiteImporter {
    /// Creates a new LegalCite importer.
    pub fn new() -> Self {
        Self
    }

    fn parse_citation(&self, citation: &LegalCitation) -> InteropResult<Statute> {
        let id = citation.id.to_lowercase().replace([' ', '-'], "_");
        let title = &citation.title;

        // Create basic statute from citation
        let effect = Effect::new(EffectType::Grant, "legal_reference");
        let mut statute = Statute::new(&id, title, effect);

        // Add jurisdiction if present
        if let Some(jurisdiction) = &citation.jurisdiction {
            statute = statute.with_jurisdiction(jurisdiction);
        }

        Ok(statute)
    }
}

impl Default for LegalCiteImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for LegalCiteImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::LegalCite
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::LegalCite, LegalFormat::Legalis);

        // Parse XML
        let doc: LegalCiteDocument = quick_xml::de::from_str(source).map_err(|e| {
            InteropError::ParseError(format!("Failed to parse LegalCite XML: {}", e))
        })?;

        let mut statutes = Vec::new();

        for citation in &doc.legal_cite.citations {
            match self.parse_citation(citation) {
                Ok(statute) => statutes.push(statute),
                Err(e) => {
                    report
                        .add_warning(format!("Failed to parse citation '{}': {}", citation.id, e));
                }
            }
        }

        report.statutes_converted = statutes.len();
        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        source.contains("<legalCite") || source.contains("legalCite")
    }
}

/// Exporter for OASIS LegalCite format.
pub struct LegalCiteExporter;

impl LegalCiteExporter {
    /// Creates a new LegalCite exporter.
    pub fn new() -> Self {
        Self
    }

    fn statute_to_citation(&self, statute: &Statute) -> LegalCitation {
        LegalCitation {
            id: statute.id.clone(),
            title: statute.title.clone(),
            citation_type: "statute".to_string(),
            jurisdiction: statute.jurisdiction.clone(),
            year: None,
            volume: None,
            reporter: None,
            page: None,
            uri: None,
        }
    }
}

impl Default for LegalCiteExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for LegalCiteExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::LegalCite
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::LegalCite);

        let citations: Vec<LegalCitation> = statutes
            .iter()
            .map(|s| self.statute_to_citation(s))
            .collect();

        let doc = LegalCiteDocument {
            legal_cite: LegalCiteContent { citations },
        };

        let output = quick_xml::se::to_string(&doc).map_err(|e| {
            InteropError::SerializationError(format!("Failed to serialize LegalCite: {}", e))
        })?;

        report.statutes_converted = statutes.len();
        report.add_warning("LegalCite format has limited semantic expressiveness");

        Ok((output, report))
    }

    fn can_represent(&self, _statute: &Statute) -> Vec<String> {
        vec![
            "Complex preconditions not supported".to_string(),
            "Effect semantics reduced to citations".to_string(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legalcite_export() {
        let exporter = LegalCiteExporter::new();
        let statute = Statute::new(
            "test-statute",
            "Test Statute",
            Effect::new(EffectType::Grant, "legal_reference"),
        )
        .with_jurisdiction("US");

        let (output, report) = exporter.export(&[statute]).unwrap();

        assert!(output.contains("legalCite"));
        assert!(output.contains("Test Statute"));
        assert!(output.contains("US"));
        assert_eq!(report.statutes_converted, 1);
    }

    #[test]
    fn test_legalcite_import() {
        let importer = LegalCiteImporter::new();
        let source = r#"<LegalCiteDocument>
            <legalCite>
                <citations>
                    <id>statute-1</id>
                    <title>Sample Statute</title>
                    <citation_type>statute</citation_type>
                    <jurisdiction>US</jurisdiction>
                    <year>2024</year>
                </citations>
            </legalCite>
        </LegalCiteDocument>"#;

        let (statutes, report) = importer.import(source).unwrap();

        assert_eq!(statutes.len(), 1);
        assert_eq!(statutes[0].title, "Sample Statute");
        assert_eq!(statutes[0].jurisdiction, Some("US".to_string()));
        assert_eq!(report.statutes_converted, 1);
    }

    #[test]
    fn test_legalcite_validate() {
        let importer = LegalCiteImporter::new();
        assert!(importer.validate("<legalCite><citations></citations></legalCite>"));
        assert!(!importer.validate("not legalcite"));
    }

    #[test]
    fn test_legalcite_roundtrip() {
        let exporter = LegalCiteExporter::new();
        let importer = LegalCiteImporter::new();

        let original = Statute::new(
            "test-law",
            "Test Law",
            Effect::new(EffectType::Grant, "reference"),
        )
        .with_jurisdiction("UK");

        let (exported, _) = exporter.export(&[original.clone()]).unwrap();
        let (imported, _) = importer.import(&exported).unwrap();

        assert_eq!(imported.len(), 1);
        assert_eq!(imported[0].title, original.title);
    }
}
