//! Akoma Ntoso format support.
//!
//! Akoma Ntoso (Architecture for Knowledge-Oriented Management of African
//! Normative Texts using Open Standards and Ontologies) is an XML standard
//! for legislative and parliamentary documents.
//!
//! Key features:
//! - Hierarchical document structure (act, bill, section, article)
//! - Semantic markup for legal concepts
//! - Metadata and identification system
//! - Multi-language support

use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::{Condition, Effect, EffectType, Statute};
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::{Reader, Writer};
use std::io::Cursor;

/// Akoma Ntoso format importer.
pub struct AkomaNtosoImporter {
    /// Whether to preserve metadata
    preserve_metadata: bool,
}

impl AkomaNtosoImporter {
    /// Creates a new Akoma Ntoso importer.
    pub fn new() -> Self {
        Self {
            preserve_metadata: true,
        }
    }

    /// Sets whether to preserve metadata.
    pub fn with_metadata(mut self, preserve: bool) -> Self {
        self.preserve_metadata = preserve;
        self
    }

    /// Parses an Akoma Ntoso document and extracts statutes.
    fn parse_document(&self, source: &str, report: &mut ConversionReport) -> Vec<Statute> {
        let mut statutes = Vec::new();
        let mut reader = Reader::from_str(source);
        reader.config_mut().trim_text(true);

        let mut current_id = String::new();
        let mut current_title = String::new();
        let mut current_content = String::new();
        let mut in_article = false;
        let mut in_heading = false;
        let mut in_content = false;
        let mut in_num = false;
        let mut depth = 0;

        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) => {
                    let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    match name.as_str() {
                        "article" | "section" | "rule" | "paragraph" => {
                            in_article = true;
                            depth += 1;
                            // Extract eId attribute if present
                            for attr in e.attributes().flatten() {
                                if attr.key.as_ref() == b"eId" || attr.key.as_ref() == b"GUID" {
                                    current_id = String::from_utf8_lossy(&attr.value).to_string();
                                }
                            }
                        }
                        "heading" | "title" => {
                            in_heading = true;
                        }
                        "content" | "p" | "block" => {
                            in_content = true;
                        }
                        "num" => {
                            in_num = true;
                        }
                        _ => {}
                    }
                }
                Ok(Event::End(ref e)) => {
                    let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    match name.as_str() {
                        "article" | "section" | "rule" | "paragraph" => {
                            depth -= 1;
                            if depth == 0 && in_article {
                                // Create statute from collected data
                                if !current_title.is_empty() || !current_content.is_empty() {
                                    let id = if current_id.is_empty() {
                                        format!("akn-{}", statutes.len() + 1)
                                    } else {
                                        current_id.to_lowercase().replace(['_', ' '], "-")
                                    };

                                    let title = if current_title.is_empty() {
                                        format!("Article {}", statutes.len() + 1)
                                    } else {
                                        current_title.clone()
                                    };

                                    let effect_desc = if current_content.is_empty() {
                                        title.clone()
                                    } else {
                                        current_content.clone()
                                    };

                                    let statute = Statute::new(
                                        &id,
                                        &title,
                                        Effect::new(EffectType::Grant, &effect_desc),
                                    );

                                    statutes.push(statute);
                                }

                                // Reset state
                                current_id.clear();
                                current_title.clear();
                                current_content.clear();
                                in_article = false;
                            }
                        }
                        "heading" | "title" => {
                            in_heading = false;
                        }
                        "content" | "p" | "block" => {
                            in_content = false;
                        }
                        "num" => {
                            in_num = false;
                        }
                        _ => {}
                    }
                }
                Ok(Event::Text(e)) => {
                    let text = e.unescape().unwrap_or_default().trim().to_string();
                    if !text.is_empty() {
                        if in_heading {
                            if !current_title.is_empty() {
                                current_title.push(' ');
                            }
                            current_title.push_str(&text);
                        } else if in_content {
                            if !current_content.is_empty() {
                                current_content.push(' ');
                            }
                            current_content.push_str(&text);
                        } else if in_num && current_id.is_empty() {
                            current_id = text.to_lowercase().replace(['.', ' '], "-");
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    report.add_warning(format!("XML parse error: {}", e));
                    break;
                }
                _ => {}
            }
        }

        statutes
    }
}

impl Default for AkomaNtosoImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for AkomaNtosoImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::AkomaNtoso
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::AkomaNtoso, LegalFormat::Legalis);

        let statutes = self.parse_document(source, &mut report);

        if statutes.is_empty() {
            return Err(InteropError::ParseError(
                "No valid Akoma Ntoso articles/sections found".to_string(),
            ));
        }

        // Note unsupported features
        if source.contains("<condition") {
            report.add_unsupported("Akoma Ntoso condition elements");
        }
        if source.contains("<temporal") {
            report.add_unsupported("Akoma Ntoso temporal groups");
        }
        if source.contains("<lifecycle") {
            report.add_unsupported("Akoma Ntoso lifecycle events");
        }

        report.statutes_converted = statutes.len();
        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        source.contains("<akomaNtoso") || source.contains("<act") || source.contains("<bill")
    }
}

/// Akoma Ntoso format exporter.
pub struct AkomaNtosoExporter {
    /// Document type (act, bill, etc.)
    doc_type: String,
    /// Country code for the document
    country: String,
}

impl AkomaNtosoExporter {
    /// Creates a new Akoma Ntoso exporter.
    pub fn new() -> Self {
        Self {
            doc_type: "act".to_string(),
            country: "un".to_string(),
        }
    }

    /// Sets the document type.
    pub fn with_doc_type(mut self, doc_type: impl Into<String>) -> Self {
        self.doc_type = doc_type.into();
        self
    }

    /// Sets the country code.
    pub fn with_country(mut self, country: impl Into<String>) -> Self {
        self.country = country.into();
        self
    }

    /// Converts a condition to Akoma Ntoso comment format.
    fn condition_to_comment(condition: &Condition) -> String {
        match condition {
            Condition::Age { operator, value } => {
                format!("Age {} {}", operator, value)
            }
            Condition::Income { operator, value } => {
                format!("Income {} {}", operator, value)
            }
            Condition::And(left, right) => {
                let l = Self::condition_to_comment(left);
                let r = Self::condition_to_comment(right);
                format!("{} AND {}", l, r)
            }
            Condition::Or(left, right) => {
                let l = Self::condition_to_comment(left);
                let r = Self::condition_to_comment(right);
                format!("{} OR {}", l, r)
            }
            Condition::Not(inner) => {
                let i = Self::condition_to_comment(inner);
                format!("NOT {}", i)
            }
            Condition::AttributeEquals { key, value } => {
                format!("{} = {}", key, value)
            }
            Condition::HasAttribute { key } => {
                format!("{} exists", key)
            }
            _ => format!("{:?}", condition),
        }
    }
}

impl Default for AkomaNtosoExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for AkomaNtosoExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::AkomaNtoso
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::AkomaNtoso);
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        // XML declaration
        writer
            .write_event(Event::Decl(quick_xml::events::BytesDecl::new(
                "1.0",
                Some("UTF-8"),
                None,
            )))
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;

        // Root element
        let mut root = BytesStart::new("akomaNtoso");
        root.push_attribute(("xmlns", "http://docs.oasis-open.org/legaldocml/ns/akn/3.0"));
        writer
            .write_event(Event::Start(root))
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;

        // Document type element (act, bill, etc.)
        let doc_start = BytesStart::new(self.doc_type.as_str());
        writer
            .write_event(Event::Start(doc_start))
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;

        // Meta section
        writer
            .write_event(Event::Start(BytesStart::new("meta")))
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;

        // Identification
        let mut ident = BytesStart::new("identification");
        ident.push_attribute(("source", "#legalis"));
        writer
            .write_event(Event::Start(ident))
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;

        // FRBRWork
        writer
            .write_event(Event::Start(BytesStart::new("FRBRWork")))
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;

        let mut frbrthis = BytesStart::new("FRBRthis");
        frbrthis.push_attribute((
            "value",
            format!("/akn/{}/{}/main", self.country, self.doc_type).as_str(),
        ));
        writer
            .write_event(Event::Empty(frbrthis))
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;

        let mut frbrcountry = BytesStart::new("FRBRcountry");
        frbrcountry.push_attribute(("value", self.country.as_str()));
        writer
            .write_event(Event::Empty(frbrcountry))
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;

        writer
            .write_event(Event::End(BytesEnd::new("FRBRWork")))
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;

        writer
            .write_event(Event::End(BytesEnd::new("identification")))
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;

        writer
            .write_event(Event::End(BytesEnd::new("meta")))
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;

        // Body section
        writer
            .write_event(Event::Start(BytesStart::new("body")))
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;

        // Convert each statute to an article
        for (i, statute) in statutes.iter().enumerate() {
            let mut article = BytesStart::new("article");
            article.push_attribute(("eId", statute.id.as_str()));
            writer
                .write_event(Event::Start(article))
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;

            // Article number
            writer
                .write_event(Event::Start(BytesStart::new("num")))
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;
            writer
                .write_event(Event::Text(BytesText::new(&format!("Article {}", i + 1))))
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;
            writer
                .write_event(Event::End(BytesEnd::new("num")))
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;

            // Heading (title)
            writer
                .write_event(Event::Start(BytesStart::new("heading")))
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;
            writer
                .write_event(Event::Text(BytesText::new(&statute.title)))
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;
            writer
                .write_event(Event::End(BytesEnd::new("heading")))
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;

            // Content paragraph with conditions
            if !statute.preconditions.is_empty() {
                let mut para = BytesStart::new("paragraph");
                para.push_attribute(("eId", format!("{}_para1", statute.id).as_str()));
                writer
                    .write_event(Event::Start(para))
                    .map_err(|e| InteropError::SerializationError(e.to_string()))?;

                writer
                    .write_event(Event::Start(BytesStart::new("content")))
                    .map_err(|e| InteropError::SerializationError(e.to_string()))?;

                writer
                    .write_event(Event::Start(BytesStart::new("p")))
                    .map_err(|e| InteropError::SerializationError(e.to_string()))?;

                let conditions: Vec<String> = statute
                    .preconditions
                    .iter()
                    .map(Self::condition_to_comment)
                    .collect();

                let condition_text = format!("When: {}", conditions.join(" AND "));
                writer
                    .write_event(Event::Text(BytesText::new(&condition_text)))
                    .map_err(|e| InteropError::SerializationError(e.to_string()))?;

                writer
                    .write_event(Event::End(BytesEnd::new("p")))
                    .map_err(|e| InteropError::SerializationError(e.to_string()))?;

                writer
                    .write_event(Event::End(BytesEnd::new("content")))
                    .map_err(|e| InteropError::SerializationError(e.to_string()))?;

                writer
                    .write_event(Event::End(BytesEnd::new("paragraph")))
                    .map_err(|e| InteropError::SerializationError(e.to_string()))?;
            }

            // Effect paragraph
            let mut effect_para = BytesStart::new("paragraph");
            effect_para.push_attribute(("eId", format!("{}_effect", statute.id).as_str()));
            writer
                .write_event(Event::Start(effect_para))
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;

            writer
                .write_event(Event::Start(BytesStart::new("content")))
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;

            writer
                .write_event(Event::Start(BytesStart::new("p")))
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;

            let effect_text = format!(
                "Effect: {} - {}",
                statute.effect.effect_type, statute.effect.description
            );
            writer
                .write_event(Event::Text(BytesText::new(&effect_text)))
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;

            writer
                .write_event(Event::End(BytesEnd::new("p")))
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;

            writer
                .write_event(Event::End(BytesEnd::new("content")))
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;

            writer
                .write_event(Event::End(BytesEnd::new("paragraph")))
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;

            // Discretion note if present
            if let Some(ref discretion) = statute.discretion_logic {
                report.add_warning(format!(
                    "Discretion '{}' added as remark element",
                    discretion
                ));

                let mut remark = BytesStart::new("remark");
                remark.push_attribute(("type", "discretion"));
                writer
                    .write_event(Event::Start(remark))
                    .map_err(|e| InteropError::SerializationError(e.to_string()))?;
                writer
                    .write_event(Event::Text(BytesText::new(discretion)))
                    .map_err(|e| InteropError::SerializationError(e.to_string()))?;
                writer
                    .write_event(Event::End(BytesEnd::new("remark")))
                    .map_err(|e| InteropError::SerializationError(e.to_string()))?;
            }

            writer
                .write_event(Event::End(BytesEnd::new("article")))
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;

            report.statutes_converted += 1;
        }

        // Close body
        writer
            .write_event(Event::End(BytesEnd::new("body")))
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;

        // Close document type
        writer
            .write_event(Event::End(BytesEnd::new(&self.doc_type)))
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;

        // Close root
        writer
            .write_event(Event::End(BytesEnd::new("akomaNtoso")))
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;

        let output = String::from_utf8(writer.into_inner().into_inner())
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;

        Ok((output, report))
    }

    fn can_represent(&self, statute: &Statute) -> Vec<String> {
        let mut issues = Vec::new();

        if statute.discretion_logic.is_some() {
            issues.push("Discretionary logic will be added as remark element".to_string());
        }

        // Complex conditions may lose semantic meaning
        for condition in &statute.preconditions {
            if matches!(condition, Condition::Custom { .. }) {
                issues.push("Custom conditions may lose semantic meaning in XML".to_string());
                break;
            }
        }

        issues
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::ComparisonOp;

    #[test]
    fn test_akoma_ntoso_importer_validate() {
        let importer = AkomaNtosoImporter::new();
        assert!(importer.validate("<akomaNtoso><act></act></akomaNtoso>"));
        assert!(importer.validate("<act name=\"test\"></act>"));
        assert!(!importer.validate("STATUTE foo: \"bar\" {}"));
    }

    #[test]
    fn test_akoma_ntoso_import_basic() {
        let importer = AkomaNtosoImporter::new();
        let source = r#"
        <akomaNtoso xmlns="http://docs.oasis-open.org/legaldocml/ns/akn/3.0">
            <act>
                <body>
                    <article eId="art_1">
                        <num>Article 1</num>
                        <heading>Adult Rights</heading>
                        <paragraph>
                            <content>
                                <p>Any person aged 18 or older has full legal capacity.</p>
                            </content>
                        </paragraph>
                    </article>
                </body>
            </act>
        </akomaNtoso>
        "#;

        let (statutes, report) = importer.import(source).unwrap();
        assert_eq!(statutes.len(), 1);
        assert_eq!(report.statutes_converted, 1);
        assert_eq!(statutes[0].id, "art-1");
        assert_eq!(statutes[0].title, "Adult Rights");
    }

    #[test]
    fn test_akoma_ntoso_import_multiple_articles() {
        let importer = AkomaNtosoImporter::new();
        let source = r#"
        <akomaNtoso>
            <act>
                <body>
                    <article eId="art_1">
                        <heading>First Article</heading>
                    </article>
                    <article eId="art_2">
                        <heading>Second Article</heading>
                    </article>
                </body>
            </act>
        </akomaNtoso>
        "#;

        let (statutes, report) = importer.import(source).unwrap();
        assert_eq!(statutes.len(), 2);
        assert_eq!(report.statutes_converted, 2);
    }

    #[test]
    fn test_akoma_ntoso_exporter_basic() {
        let exporter = AkomaNtosoExporter::new();
        let statute = Statute::new(
            "voting-rights",
            "Voting Rights Act",
            Effect::new(EffectType::Grant, "Right to vote"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let (output, report) = exporter.export(&[statute]).unwrap();

        assert!(output.contains("<akomaNtoso"));
        assert!(output.contains("Voting Rights Act"));
        // XML escaping: >= becomes &gt;= in XML text
        assert!(output.contains("Age") && output.contains("18"));
        assert!(output.contains("Right to vote"));
        assert_eq!(report.statutes_converted, 1);
    }

    #[test]
    fn test_akoma_ntoso_roundtrip() {
        let exporter = AkomaNtosoExporter::new();
        let importer = AkomaNtosoImporter::new();

        let statute = Statute::new(
            "test-statute",
            "Test Statute Title",
            Effect::new(EffectType::Grant, "Test effect"),
        );

        // Export
        let (xml_output, _) = exporter.export(&[statute]).unwrap();

        // Import back
        let (imported, _) = importer.import(&xml_output).unwrap();

        assert_eq!(imported.len(), 1);
        assert_eq!(imported[0].title, "Test Statute Title");
    }

    #[test]
    fn test_akoma_ntoso_with_country() {
        let exporter = AkomaNtosoExporter::new()
            .with_country("jp")
            .with_doc_type("bill");

        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "test"));

        let (output, _) = exporter.export(&[statute]).unwrap();

        assert!(output.contains("<bill"));
        assert!(output.contains("/akn/jp/bill"));
    }
}
