//! FORMEX format support.
//!
//! FORMEX (Formalized Exchange of Electronic Publications) is the EU's standard
//! for publishing in the Official Journal of the European Union.
//!
//! Format characteristics:
//! - XML-based structure for EU official documents
//! - Structured sections, articles, and paragraphs
//! - Multilingual support
//! - Publication metadata
//! - Citation and reference system
//!
//! Reference: https://op.europa.eu/en/web/eu-vocabularies/formex

use crate::{ConversionReport, FormatExporter, FormatImporter, InteropResult, LegalFormat};
use legalis_core::{Condition, Effect, EffectType, Statute};
use regex_lite::Regex;

/// FORMEX format importer.
pub struct FormexImporter;

impl FormexImporter {
    /// Creates a new FORMEX importer.
    pub fn new() -> Self {
        Self
    }

    fn extract_articles(&self, text: &str) -> Vec<(String, String, String)> {
        let mut articles = Vec::new();

        // Simple manual parsing since regex_lite doesn't support look-ahead/behind
        // Split by ARTICLE tags
        let parts: Vec<&str> = text.split("<ARTICLE").collect();

        for part in parts.iter().skip(1) {
            if let Some(end_pos) = part.find("</ARTICLE>") {
                let article_content = &part[..end_pos];

                // Extract NO.P (number)
                let number = if let (Some(start), Some(end)) = (
                    article_content.find("<NO.P>"),
                    article_content.find("</NO.P>"),
                ) {
                    article_content[start + 6..end].trim().to_string()
                } else {
                    "1".to_string()
                };

                // Extract TITRE (title)
                let title = if let (Some(start), Some(end)) = (
                    article_content.find("<TITRE>"),
                    article_content.find("</TITRE>"),
                ) {
                    article_content[start + 7..end].trim().to_string()
                } else {
                    String::new()
                };

                // Extract ALINEA (content)
                let content = if let (Some(start), Some(end)) = (
                    article_content.find("<ALINEA>"),
                    article_content.find("</ALINEA>"),
                ) {
                    article_content[start + 8..end].trim().to_string()
                } else {
                    String::new()
                };

                articles.push((number, title, content));
            }
        }

        articles
    }

    fn parse_conditions(&self, text: &str) -> Vec<Condition> {
        let mut conditions = Vec::new();

        // Look for condition patterns in content
        if text.contains("age") || text.contains("years") {
            let re = Regex::new(r"(?:age|years)\s*(?:of\s*)?(?:at least|minimum|>=|greater than or equal to)\s*(\d+)").unwrap();
            if let Some(cap) = re.captures(text) {
                if let Some(age_str) = cap.get(1) {
                    if let Ok(age_val) = age_str.as_str().parse::<u32>() {
                        conditions.push(Condition::Age {
                            operator: legalis_core::ComparisonOp::GreaterOrEqual,
                            value: age_val,
                        });
                    }
                }
            }
        }

        conditions
    }

    fn article_to_statute(&self, number: &str, title: &str, content: &str) -> Statute {
        let conditions = self.parse_conditions(content);

        let statute_id = format!(
            "article-{}",
            number
                .to_lowercase()
                .replace(|c: char| !c.is_alphanumeric(), "-")
        );

        let statute_title = if title.is_empty() {
            format!("Article {}", number)
        } else {
            format!("Article {} - {}", number, title)
        };

        // Determine effect type from content
        let effect_type = if content.to_lowercase().contains("shall")
            || content.to_lowercase().contains("must")
            || content.to_lowercase().contains("required")
        {
            EffectType::Obligation
        } else if content.to_lowercase().contains("may")
            || content.to_lowercase().contains("permitted")
        {
            EffectType::Grant
        } else if content.to_lowercase().contains("prohibited")
            || content.to_lowercase().contains("shall not")
        {
            EffectType::Prohibition
        } else {
            EffectType::Grant
        };

        let mut effect = Effect::new(effect_type, "eu regulation");
        effect
            .parameters
            .insert("content".to_string(), content.to_string());
        effect
            .parameters
            .insert("article_number".to_string(), number.to_string());

        let mut statute = Statute::new(&statute_id, &statute_title, effect);

        for condition in conditions {
            statute = statute.with_precondition(condition);
        }

        statute
    }
}

impl Default for FormexImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for FormexImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Formex
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Formex, LegalFormat::Legalis);

        let articles = self.extract_articles(source);

        let mut statutes = Vec::new();

        for (number, title, content) in articles {
            let statute = self.article_to_statute(&number, &title, &content);
            statutes.push(statute);
        }

        report.statutes_converted = statutes.len();

        if statutes.is_empty() {
            report.add_warning("No valid FORMEX articles found");
        }

        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        // FORMEX documents contain specific XML tags
        source.contains("<ARTICLE") || source.contains("<ACT") || source.contains("<FORMEX")
    }
}

/// FORMEX format exporter.
pub struct FormexExporter;

impl FormexExporter {
    /// Creates a new FORMEX exporter.
    pub fn new() -> Self {
        Self
    }

    fn statute_to_article(&self, statute: &Statute, number: usize) -> String {
        let mut xml = String::new();

        xml.push_str("<ARTICLE>\n");

        // Article number
        xml.push_str(&format!("  <NO.P>{}</NO.P>\n", number));

        // Title
        xml.push_str(&format!("  <TITRE>{}</TITRE>\n", statute.title));

        // Content
        let content = statute
            .effect
            .parameters
            .get("content")
            .cloned()
            .unwrap_or_else(|| {
                let effect_verb = match statute.effect.effect_type {
                    EffectType::Grant => "may",
                    EffectType::Obligation => "shall",
                    EffectType::Prohibition => "shall not",
                    EffectType::Revoke => "void",
                    EffectType::MonetaryTransfer => "shall transfer",
                    EffectType::StatusChange => "shall change",
                    EffectType::Custom => "shall apply",
                };
                format!("The party {} {}.", effect_verb, statute.effect.description)
            });

        xml.push_str(&format!("  <ALINEA>{}</ALINEA>\n", content));

        xml.push_str("</ARTICLE>\n");

        xml
    }
}

impl Default for FormexExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for FormexExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Formex
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::Formex);
        let mut output = String::new();

        // FORMEX document structure
        output.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        output.push_str("<FORMEX>\n");
        output.push_str("  <ACT>\n");
        output.push_str("    <ENACTING.TERMS>\n");

        for (i, statute) in statutes.iter().enumerate() {
            output.push_str(&format!(
                "      {}",
                self.statute_to_article(statute, i + 1)
            ));
        }

        output.push_str("    </ENACTING.TERMS>\n");
        output.push_str("  </ACT>\n");
        output.push_str("</FORMEX>\n");

        report.statutes_converted = statutes.len();
        Ok((output, report))
    }

    fn can_represent(&self, _statute: &Statute) -> Vec<String> {
        // FORMEX can represent most legal provisions
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formex_import_simple() {
        let importer = FormexImporter::new();

        let source = r#"<FORMEX>
  <ACT>
    <ENACTING.TERMS>
      <ARTICLE>
        <NO.P>1</NO.P>
        <TITRE>Scope</TITRE>
        <ALINEA>This regulation shall apply to all Member States.</ALINEA>
      </ARTICLE>
    </ENACTING.TERMS>
  </ACT>
</FORMEX>"#;

        let (statutes, report) = importer.import(source).unwrap();
        assert_eq!(report.statutes_converted, 1);
        assert_eq!(statutes.len(), 1);
        assert!(statutes[0].title.contains("Article 1"));
    }

    #[test]
    fn test_formex_export_simple() {
        let exporter = FormexExporter::new();

        let mut effect = Effect::new(EffectType::Obligation, "comply with regulation");
        effect.parameters.insert(
            "content".to_string(),
            "Member States shall ensure compliance.".to_string(),
        );

        let statute = Statute::new("article-1", "Article 1 - Compliance", effect);

        let (output, report) = exporter.export(&[statute]).unwrap();
        assert_eq!(report.statutes_converted, 1);
        assert!(output.contains("<FORMEX>"));
        assert!(output.contains("<ARTICLE>"));
        assert!(output.contains("<NO.P>1</NO.P>"));
    }

    #[test]
    fn test_formex_validate() {
        let importer = FormexImporter::new();

        assert!(importer.validate("<FORMEX><ACT></ACT></FORMEX>"));
        assert!(importer.validate("<ARTICLE><NO.P>1</NO.P></ARTICLE>"));
        assert!(!importer.validate("plain text"));
    }

    #[test]
    fn test_formex_roundtrip() {
        let importer = FormexImporter::new();
        let exporter = FormexExporter::new();

        let effect = Effect::new(EffectType::Grant, "access rights");
        let statute = Statute::new("article-1", "Article 1", effect);

        let (exported, _) = exporter.export(&[statute]).unwrap();
        let (imported, _) = importer.import(&exported).unwrap();

        assert_eq!(imported.len(), 1);
        assert!(imported[0].title.contains("Article 1"));
    }
}
