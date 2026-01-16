//! CEN MetaLex format import/export.
//!
//! MetaLex is a CEN Workshop Agreement (CWA 15710) for legal document metadata
//! and structure. It provides a standardized way to represent legal documents
//! with their metadata, structure, and temporal aspects.

use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::{Effect, EffectType, Statute};
use serde::{Deserialize, Serialize};

/// MetaLex document structure (simplified).
#[derive(Debug, Clone, Serialize, Deserialize)]
struct MetaLexDocument {
    #[serde(rename = "metalex")]
    metalex: MetaLexContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MetaLexContent {
    #[serde(
        rename = "BibliographicExpression",
        skip_serializing_if = "Option::is_none"
    )]
    bibliographic: Option<BibliographicExpression>,
    #[serde(rename = "Body")]
    body: MetaLexBody,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
struct BibliographicExpression {
    #[serde(rename = "Title", skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(rename = "Date", skip_serializing_if = "Option::is_none")]
    date: Option<String>,
    #[serde(rename = "Jurisdiction", skip_serializing_if = "Option::is_none")]
    jurisdiction: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MetaLexBody {
    #[serde(rename = "Article", default)]
    articles: Vec<MetaLexArticle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
struct MetaLexArticle {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "Title", skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(rename = "Content", skip_serializing_if = "Option::is_none")]
    content: Option<String>,
}

/// Importer for CEN MetaLex format.
pub struct MetaLexImporter;

impl MetaLexImporter {
    /// Creates a new MetaLex importer.
    pub fn new() -> Self {
        Self
    }

    fn parse_article(&self, article: &MetaLexArticle) -> InteropResult<Statute> {
        let id = article.id.to_lowercase().replace([' ', '-'], "_");
        let title = article.title.as_deref().unwrap_or("Untitled Article");

        // Create basic statute from article
        let effect = Effect::new(EffectType::Grant, "article_provision");
        let statute = Statute::new(&id, title, effect);

        Ok(statute)
    }
}

impl Default for MetaLexImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for MetaLexImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::MetaLex
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::MetaLex, LegalFormat::Legalis);

        // Parse XML
        let doc: MetaLexDocument = quick_xml::de::from_str(source)
            .map_err(|e| InteropError::ParseError(format!("Failed to parse MetaLex XML: {}", e)))?;

        let mut statutes = Vec::new();

        for article in &doc.metalex.body.articles {
            match self.parse_article(article) {
                Ok(statute) => statutes.push(statute),
                Err(e) => {
                    report.add_warning(format!("Failed to parse article '{}': {}", article.id, e));
                }
            }
        }

        report.statutes_converted = statutes.len();
        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        source.contains("<metalex") || source.contains("MetaLex")
    }
}

/// Exporter for CEN MetaLex format.
pub struct MetaLexExporter;

impl MetaLexExporter {
    /// Creates a new MetaLex exporter.
    pub fn new() -> Self {
        Self
    }

    fn statute_to_article(&self, statute: &Statute) -> MetaLexArticle {
        MetaLexArticle {
            id: statute.id.clone(),
            title: Some(statute.title.clone()),
            content: Some(format!("Effect: {:?}", statute.effect.effect_type)),
        }
    }
}

impl Default for MetaLexExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for MetaLexExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::MetaLex
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::MetaLex);

        let articles: Vec<MetaLexArticle> = statutes
            .iter()
            .map(|s| self.statute_to_article(s))
            .collect();

        let doc = MetaLexDocument {
            metalex: MetaLexContent {
                bibliographic: Some(BibliographicExpression {
                    title: Some("Legal Document".to_string()),
                    date: None,
                    jurisdiction: None,
                }),
                body: MetaLexBody { articles },
            },
        };

        let output = quick_xml::se::to_string(&doc).map_err(|e| {
            InteropError::SerializationError(format!("Failed to serialize MetaLex: {}", e))
        })?;

        report.statutes_converted = statutes.len();
        report.add_warning(
            "MetaLex format focuses on document structure, semantic details may be lost",
        );

        Ok((output, report))
    }

    fn can_represent(&self, _statute: &Statute) -> Vec<String> {
        vec![
            "Complex preconditions not fully supported".to_string(),
            "Temporal aspects simplified".to_string(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metalex_export() {
        let exporter = MetaLexExporter::new();
        let statute = Statute::new(
            "article-1",
            "Article 1",
            Effect::new(EffectType::Grant, "provision"),
        );

        let (output, report) = exporter.export(&[statute]).unwrap();

        assert!(output.contains("metalex"));
        assert!(output.contains("Article 1"));
        assert_eq!(report.statutes_converted, 1);
    }

    #[test]
    fn test_metalex_import() {
        let importer = MetaLexImporter::new();
        let source = r#"<MetaLexDocument>
            <metalex>
                <Body>
                    <Article id="art-1">
                        <Title>Test Article</Title>
                        <Content>Test content</Content>
                    </Article>
                </Body>
            </metalex>
        </MetaLexDocument>"#;

        let (statutes, report) = importer.import(source).unwrap();

        assert_eq!(statutes.len(), 1);
        assert_eq!(statutes[0].title, "Test Article");
        assert_eq!(report.statutes_converted, 1);
    }

    #[test]
    fn test_metalex_validate() {
        let importer = MetaLexImporter::new();
        assert!(importer.validate("<metalex><Body></Body></metalex>"));
        assert!(!importer.validate("not metalex"));
    }

    #[test]
    fn test_metalex_roundtrip() {
        let exporter = MetaLexExporter::new();
        let importer = MetaLexImporter::new();

        let original = Statute::new(
            "test-article",
            "Test Article",
            Effect::new(EffectType::Grant, "provision"),
        );

        let (exported, _) = exporter.export(std::slice::from_ref(&original)).unwrap();
        let (imported, _) = importer.import(&exported).unwrap();

        assert_eq!(imported.len(), 1);
        assert_eq!(imported[0].title, original.title);
    }
}
