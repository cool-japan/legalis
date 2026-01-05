//! NIEM format support.
//!
//! NIEM (National Information Exchange Model) is a U.S. standard for information exchange
//! between government agencies and organizations.
//!
//! Format characteristics:
//! - XML-based information exchange
//! - Standardized data components
//! - Domain-specific extensions
//! - Justice, emergency management, and other domains
//! - Interoperability focus
//!
//! Reference: https://www.niem.gov/

use crate::{ConversionReport, FormatExporter, FormatImporter, InteropResult, LegalFormat};
use legalis_core::{Effect, EffectType, Statute};
use serde::{Deserialize, Serialize};

/// NIEM document structure (simplified for justice domain).
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct NiemDocument {
    #[serde(rename = "ExchangeContent")]
    content: NiemExchangeContent,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct NiemExchangeContent {
    #[serde(rename = "Case", default)]
    cases: Vec<NiemCase>,
    #[serde(rename = "Statute", default)]
    statutes: Vec<NiemStatute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
struct NiemCase {
    #[serde(rename = "CaseTrackingID")]
    tracking_id: String,
    #[serde(rename = "CaseTitleText", skip_serializing_if = "Option::is_none")]
    title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NiemStatute {
    #[serde(rename = "StatuteCodeIdentification")]
    code_id: String,
    #[serde(rename = "StatuteDescriptionText")]
    description: String,
    #[serde(
        rename = "StatuteJurisdictionText",
        skip_serializing_if = "Option::is_none"
    )]
    jurisdiction: Option<String>,
}

/// NIEM format importer.
pub struct NiemImporter;

impl NiemImporter {
    /// Creates a new NIEM importer.
    pub fn new() -> Self {
        Self
    }

    fn parse_niem_xml(&self, source: &str) -> InteropResult<Vec<NiemStatute>> {
        // Simple manual XML parsing for NIEM statutes
        let mut statutes = Vec::new();

        // Split by Statute tags
        let parts: Vec<&str> = source.split("<Statute>").collect();

        for part in parts.iter().skip(1) {
            if let Some(end_pos) = part.find("</Statute>") {
                let statute_content = &part[..end_pos];

                // Extract code
                let code = if let (Some(start), Some(end)) = (
                    statute_content.find("<StatuteCodeIdentification>"),
                    statute_content.find("</StatuteCodeIdentification>"),
                ) {
                    statute_content[start + 27..end].trim().to_string()
                } else {
                    continue;
                };

                // Extract description
                let description = if let (Some(start), Some(end)) = (
                    statute_content.find("<StatuteDescriptionText>"),
                    statute_content.find("</StatuteDescriptionText>"),
                ) {
                    statute_content[start + 24..end].trim().to_string()
                } else {
                    continue;
                };

                // Extract jurisdiction (optional)
                let jurisdiction = if let (Some(start), Some(end)) = (
                    statute_content.find("<StatuteJurisdictionText>"),
                    statute_content.find("</StatuteJurisdictionText>"),
                ) {
                    Some(statute_content[start + 25..end].trim().to_string())
                } else {
                    None
                };

                statutes.push(NiemStatute {
                    code_id: code,
                    description,
                    jurisdiction,
                });
            }
        }

        Ok(statutes)
    }

    fn niem_statute_to_statute(&self, niem: &NiemStatute) -> Statute {
        let id = niem
            .code_id
            .to_lowercase()
            .replace(|c: char| !c.is_alphanumeric(), "-")
            .trim_matches('-')
            .to_string();

        let title = format!("Statute {}", niem.code_id);

        // Determine effect type from description
        let effect_type = if niem.description.to_lowercase().contains("shall")
            || niem.description.to_lowercase().contains("must")
            || niem.description.to_lowercase().contains("required")
        {
            EffectType::Obligation
        } else if niem.description.to_lowercase().contains("may")
            || niem.description.to_lowercase().contains("permitted")
        {
            EffectType::Grant
        } else if niem.description.to_lowercase().contains("prohibited")
            || niem.description.to_lowercase().contains("shall not")
        {
            EffectType::Prohibition
        } else {
            EffectType::Grant
        };

        let mut effect = Effect::new(effect_type, &niem.description);
        effect
            .parameters
            .insert("niem_code".to_string(), niem.code_id.clone());

        let mut statute = Statute::new(&id, &title, effect);

        if let Some(jurisdiction) = &niem.jurisdiction {
            statute.jurisdiction = Some(jurisdiction.clone());
        }

        statute
    }
}

impl Default for NiemImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for NiemImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Niem
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Niem, LegalFormat::Legalis);

        let niem_statutes = self.parse_niem_xml(source)?;

        let mut statutes = Vec::new();

        for niem_statute in niem_statutes {
            let statute = self.niem_statute_to_statute(&niem_statute);
            statutes.push(statute);
        }

        report.statutes_converted = statutes.len();

        if statutes.is_empty() {
            report.add_warning("No valid NIEM statutes found");
        }

        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        // NIEM documents contain specific XML namespaces and elements
        source.contains("niem")
            || source.contains("ExchangeContent")
            || source.contains("StatuteCodeIdentification")
    }
}

/// NIEM format exporter.
pub struct NiemExporter;

impl NiemExporter {
    /// Creates a new NIEM exporter.
    pub fn new() -> Self {
        Self
    }

    fn statute_to_niem(&self, statute: &Statute) -> String {
        let mut xml = String::new();

        xml.push_str("  <Statute>\n");

        // Code ID from effect parameters or generate from ID
        let code_id = statute
            .effect
            .parameters
            .get("niem_code")
            .cloned()
            .unwrap_or_else(|| statute.id.to_uppercase());

        xml.push_str(&format!(
            "    <StatuteCodeIdentification>{}</StatuteCodeIdentification>\n",
            code_id
        ));
        xml.push_str(&format!(
            "    <StatuteDescriptionText>{}</StatuteDescriptionText>\n",
            statute.effect.description
        ));

        if let Some(jurisdiction) = &statute.jurisdiction {
            xml.push_str(&format!(
                "    <StatuteJurisdictionText>{}</StatuteJurisdictionText>\n",
                jurisdiction
            ));
        }

        xml.push_str("  </Statute>\n");

        xml
    }
}

impl Default for NiemExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for NiemExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Niem
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::Niem);
        let mut output = String::new();

        // NIEM document structure
        output.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        output.push_str("<ExchangeContent xmlns:niem=\"http://niem.gov/niem/niem-core/2.0\">\n");

        for statute in statutes {
            output.push_str(&self.statute_to_niem(statute));
        }

        output.push_str("</ExchangeContent>\n");

        report.statutes_converted = statutes.len();
        Ok((output, report))
    }

    fn can_represent(&self, _statute: &Statute) -> Vec<String> {
        // NIEM can represent most statute information
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_niem_import_simple() {
        let importer = NiemImporter::new();

        let source = r#"<ExchangeContent>
  <Statute>
    <StatuteCodeIdentification>USC-18-1001</StatuteCodeIdentification>
    <StatuteDescriptionText>False statements to federal agents</StatuteDescriptionText>
    <StatuteJurisdictionText>Federal</StatuteJurisdictionText>
  </Statute>
</ExchangeContent>"#;

        let (statutes, report) = importer.import(source).unwrap();
        assert_eq!(report.statutes_converted, 1);
        assert_eq!(statutes.len(), 1);
        assert_eq!(statutes[0].jurisdiction, Some("Federal".to_string()));
    }

    #[test]
    fn test_niem_export_simple() {
        let exporter = NiemExporter::new();

        let mut effect = Effect::new(EffectType::Prohibition, "False statements");
        effect
            .parameters
            .insert("niem_code".to_string(), "USC-18-1001".to_string());

        let mut statute = Statute::new("usc-18-1001", "Statute USC-18-1001", effect);
        statute.jurisdiction = Some("Federal".to_string());

        let (output, report) = exporter.export(&[statute]).unwrap();
        assert_eq!(report.statutes_converted, 1);
        assert!(output.contains("<ExchangeContent"));
        assert!(output.contains("<Statute>"));
        assert!(output.contains("USC-18-1001"));
    }

    #[test]
    fn test_niem_validate() {
        let importer = NiemImporter::new();

        assert!(importer.validate("<ExchangeContent><Statute></Statute></ExchangeContent>"));
        assert!(importer.validate("<StatuteCodeIdentification>123</StatuteCodeIdentification>"));
        assert!(importer.validate("xmlns:niem=\"http://niem.gov\""));
        assert!(!importer.validate("plain text"));
    }

    #[test]
    fn test_niem_roundtrip() {
        let importer = NiemImporter::new();
        let exporter = NiemExporter::new();

        let effect = Effect::new(EffectType::Grant, "Legal authority");
        let statute = Statute::new("statute-1", "Statute 1", effect);

        let (exported, _) = exporter.export(&[statute]).unwrap();
        let (imported, _) = importer.import(&exported).unwrap();

        assert_eq!(imported.len(), 1);
    }
}
