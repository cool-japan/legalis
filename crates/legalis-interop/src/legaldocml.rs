//! LegalDocML format support.
//!
//! LegalDocML is an OASIS standard for legal document markup,
//! focusing on document structure and metadata.
//!
//! Key features:
//! - Document structure markup
//! - Metadata and provenance
//! - Cross-references and citations
//! - Version control and amendments

use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

/// LegalDocML format importer.
pub struct LegalDocMLImporter {
    /// Whether to preserve document metadata
    preserve_metadata: bool,
}

impl LegalDocMLImporter {
    /// Creates a new LegalDocML importer.
    pub fn new() -> Self {
        Self {
            preserve_metadata: true,
        }
    }

    /// Sets whether to preserve document metadata.
    pub fn with_metadata_preservation(mut self, preserve: bool) -> Self {
        self.preserve_metadata = preserve;
        self
    }

    /// Parses a LegalDocML document element into a statute.
    fn parse_document(&self, content: &str, report: &mut ConversionReport) -> Option<Statute> {
        // Look for section or article elements
        let section_re = regex_lite::Regex::new(
            r#"<(?:section|article)\s+(?:id|eId)="([^"]+)"[^>]*>(.*?)</(?:section|article)>"#,
        )
        .ok()?;

        let captures = section_re.captures(content)?;
        let id = captures.get(1)?.as_str();
        let body = captures.get(2)?.as_str();

        // Extract heading/title
        let heading_re = regex_lite::Regex::new(r"<heading>([^<]+)</heading>").ok()?;
        let title = heading_re
            .captures(body)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str())
            .unwrap_or(id);

        // Extract content text
        let content_re = regex_lite::Regex::new(r"<content>(.*?)</content>").ok()?;
        let content_text = content_re
            .captures(body)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str())
            .unwrap_or("");

        // Create statute
        let mut statute = Statute::new(
            id.to_lowercase().replace(' ', "-"),
            title,
            Effect::new(EffectType::Grant, "Document provision"),
        );

        // Parse conditions from content
        if let Some(cond) = self.parse_conditions_from_text(content_text, report) {
            statute.preconditions.push(cond);
        }

        // Track metadata if enabled
        if self.preserve_metadata {
            self.extract_metadata(body, report);
        }

        Some(statute)
    }

    /// Parses conditions from natural language text.
    fn parse_conditions_from_text(
        &self,
        text: &str,
        _report: &mut ConversionReport,
    ) -> Option<Condition> {
        // Simple heuristic-based parsing
        let text_lower = text.to_lowercase();

        // Look for age conditions
        let age_re = regex_lite::Regex::new(r"(?:age|years?)\s+(?:of\s+)?(\d+)").ok()?;
        if let Some(cap) = age_re.captures(&text_lower) {
            if let Ok(age) = cap.get(1)?.as_str().parse::<u32>() {
                return Some(Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: age,
                });
            }
        }

        None
    }

    /// Extracts metadata from document.
    fn extract_metadata(&self, content: &str, report: &mut ConversionReport) {
        // Look for metadata elements
        let meta_re = regex_lite::Regex::new(r#"<meta\s+name="([^"]+)"\s+content="([^"]+)""#).ok();

        if let Some(re) = meta_re {
            let count = re.captures_iter(content).count();
            if count > 0 {
                report.add_warning(format!("Preserved {} metadata element(s)", count));
            }
        }
    }
}

impl Default for LegalDocMLImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for LegalDocMLImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::LegalDocML
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::LegalDocML, LegalFormat::Legalis);
        let mut statutes = Vec::new();

        // Try to parse document sections
        if let Some(statute) = self.parse_document(source, &mut report) {
            statutes.push(statute);
        }

        if statutes.is_empty() {
            return Err(InteropError::ParseError(
                "No valid LegalDocML elements found".to_string(),
            ));
        }

        // Note unsupported features
        if source.contains("<amendment") {
            report.add_unsupported("LegalDocML amendments");
        }
        if source.contains("<quotedStructure") {
            report.add_unsupported("LegalDocML quoted structures");
        }

        report.statutes_converted = statutes.len();
        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        source.contains("<legalDoc") || (source.contains("<section") && source.contains("eId="))
    }
}

/// LegalDocML format exporter.
pub struct LegalDocMLExporter {
    /// Document language
    language: String,
    /// Include metadata
    include_metadata: bool,
}

impl LegalDocMLExporter {
    /// Creates a new LegalDocML exporter.
    pub fn new() -> Self {
        Self {
            language: "en".to_string(),
            include_metadata: true,
        }
    }

    /// Sets the document language.
    pub fn with_language(mut self, lang: impl Into<String>) -> Self {
        self.language = lang.into();
        self
    }

    /// Sets whether to include metadata.
    pub fn with_metadata(mut self, include: bool) -> Self {
        self.include_metadata = include;
        self
    }

    /// Converts a condition to natural language text.
    fn condition_to_text(condition: &Condition, report: &mut ConversionReport) -> String {
        match condition {
            Condition::Age { operator, value } => {
                let op_text = match operator {
                    ComparisonOp::GreaterOrEqual => "at least",
                    ComparisonOp::GreaterThan => "more than",
                    ComparisonOp::LessOrEqual => "at most",
                    ComparisonOp::LessThan => "less than",
                    ComparisonOp::Equal => "exactly",
                    ComparisonOp::NotEqual => "not",
                };
                format!("age {} {} years", op_text, value)
            }
            Condition::Income { operator, value } => {
                let op_text = match operator {
                    ComparisonOp::GreaterOrEqual => "at least",
                    ComparisonOp::GreaterThan => "more than",
                    ComparisonOp::LessOrEqual => "at most",
                    ComparisonOp::LessThan => "less than",
                    ComparisonOp::Equal => "exactly",
                    ComparisonOp::NotEqual => "not",
                };
                format!("income {} ${}", op_text, value)
            }
            Condition::And(left, right) => {
                let l = Self::condition_to_text(left, report);
                let r = Self::condition_to_text(right, report);
                format!("{} and {}", l, r)
            }
            Condition::Or(left, right) => {
                let l = Self::condition_to_text(left, report);
                let r = Self::condition_to_text(right, report);
                format!("{} or {}", l, r)
            }
            Condition::Not(inner) => {
                let i = Self::condition_to_text(inner, report);
                format!("not {}", i)
            }
            _ => {
                report.add_unsupported(format!("Condition type: {:?}", condition));
                "condition met".to_string()
            }
        }
    }
}

impl Default for LegalDocMLExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for LegalDocMLExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::LegalDocML
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::LegalDocML);
        let mut output = String::new();

        // LegalDocML header
        output.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        output.push_str(&format!(
            "<legalDoc xmlns=\"http://docs.oasis-open.org/legaldocml/ns/akn/3.0\" xml:lang=\"{}\">\n",
            self.language
        ));

        // Metadata
        if self.include_metadata {
            output.push_str("  <meta>\n");
            output.push_str("    <identification source=\"#legalis-rs\">\n");
            output.push_str("      <FRBRWork>\n");
            output.push_str("        <FRBRthis value=\"/legalis/document\"/>\n");
            output.push_str("        <FRBRdate name=\"generation\" date=\"");
            output.push_str(&chrono::Utc::now().format("%Y-%m-%d").to_string());
            output.push_str("\"/>\n");
            output.push_str("      </FRBRWork>\n");
            output.push_str("    </identification>\n");
            output.push_str("  </meta>\n");
        }

        // Body
        output.push_str("  <body>\n");

        for statute in statutes {
            output.push_str(&format!("    <section eId=\"{}\">\n", statute.id));
            output.push_str(&format!("      <heading>{}</heading>\n", statute.title));
            output.push_str("      <content>\n");

            // Build content text
            let mut content_parts = Vec::new();

            if !statute.preconditions.is_empty() {
                let conditions: Vec<String> = statute
                    .preconditions
                    .iter()
                    .map(|c| Self::condition_to_text(c, &mut report))
                    .collect();

                content_parts.push(format!("Where {}", conditions.join(" and ")));
            }

            content_parts.push(format!("then {}", statute.effect.description));

            output.push_str(&format!("        <p>{}</p>\n", content_parts.join(", ")));
            output.push_str("      </content>\n");
            output.push_str("    </section>\n");

            report.statutes_converted += 1;
        }

        output.push_str("  </body>\n");
        output.push_str("</legalDoc>\n");

        Ok((output, report))
    }

    fn can_represent(&self, _statute: &Statute) -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legaldocml_importer_validate() {
        let importer = LegalDocMLImporter::new();
        assert!(importer.validate("<legalDoc><section eId=\"test\"></section></legalDoc>"));
        assert!(importer.validate("<section eId=\"art1\"><heading>Test</heading></section>"));
        assert!(!importer.validate("RULE Test WHEN age >= 18"));
    }

    #[test]
    fn test_legaldocml_exporter_basic() {
        let exporter = LegalDocMLExporter::new();
        let statute = Statute::new(
            "voting-rights",
            "Voting Rights",
            Effect::new(EffectType::Grant, "may vote"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let (output, report) = exporter.export(&[statute]).unwrap();

        assert!(output.contains("<legalDoc"));
        assert!(output.contains("<section eId=\"voting-rights\">"));
        assert!(output.contains("<heading>Voting Rights</heading>"));
        assert!(output.contains("age at least 18"));
        assert_eq!(report.statutes_converted, 1);
    }

    #[test]
    fn test_legaldocml_with_metadata() {
        let exporter = LegalDocMLExporter::new().with_metadata(true);
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "test"));

        let (output, _) = exporter.export(&[statute]).unwrap();

        assert!(output.contains("<meta>"));
        assert!(output.contains("<FRBRWork>"));
    }

    #[test]
    fn test_legaldocml_without_metadata() {
        let exporter = LegalDocMLExporter::new().with_metadata(false);
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "test"));

        let (output, _) = exporter.export(&[statute]).unwrap();

        assert!(!output.contains("<meta>"));
    }
}
