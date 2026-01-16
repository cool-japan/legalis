//! MPEG-21 REL (Rights Expression Language) format import/export.
//!
//! MPEG-21 REL is an ISO standard (ISO/IEC 21000-5) for expressing rights
//! and permissions for digital content. It uses XML to describe rights holders,
//! permissions, conditions, and obligations.

use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::{Effect, EffectType, Statute};
use serde::{Deserialize, Serialize};

/// MPEG-21 REL license structure (simplified).
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Mpeg21RelDocument {
    #[serde(rename = "license")]
    license: Mpeg21License,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Mpeg21License {
    #[serde(rename = "grant", default)]
    grants: Vec<Mpeg21Grant>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
struct Mpeg21Grant {
    #[serde(rename = "@grantId", skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename = "principal", skip_serializing_if = "Option::is_none")]
    principal: Option<String>,
    #[serde(rename = "right")]
    right: Mpeg21Right,
    #[serde(rename = "resource", skip_serializing_if = "Option::is_none")]
    resource: Option<String>,
    #[serde(rename = "condition", skip_serializing_if = "Option::is_none")]
    condition: Option<Mpeg21Condition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Mpeg21Right {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Mpeg21Condition {
    #[serde(rename = "$value")]
    value: String,
}

/// Importer for MPEG-21 REL format.
pub struct Mpeg21RelImporter;

impl Mpeg21RelImporter {
    /// Creates a new MPEG-21 REL importer.
    pub fn new() -> Self {
        Self
    }

    fn parse_grant(&self, grant: &Mpeg21Grant, index: usize) -> InteropResult<Statute> {
        let id = grant
            .id
            .as_ref()
            .unwrap_or(&format!("grant_{}", index))
            .to_lowercase()
            .replace([' ', '-'], "_");

        let title = format!("Grant: {}", grant.right.value);

        // Create statute from grant
        let effect = Effect::new(EffectType::Grant, &grant.right.value);
        let statute = Statute::new(&id, &title, effect);

        Ok(statute)
    }
}

impl Default for Mpeg21RelImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for Mpeg21RelImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Mpeg21Rel
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Mpeg21Rel, LegalFormat::Legalis);

        // Parse XML
        let doc: Mpeg21RelDocument = quick_xml::de::from_str(source).map_err(|e| {
            InteropError::ParseError(format!("Failed to parse MPEG-21 REL XML: {}", e))
        })?;

        let mut statutes = Vec::new();

        for (index, grant) in doc.license.grants.iter().enumerate() {
            match self.parse_grant(grant, index) {
                Ok(statute) => statutes.push(statute),
                Err(e) => {
                    report.add_warning(format!("Failed to parse grant {}: {}", index, e));
                }
            }
        }

        report.statutes_converted = statutes.len();
        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        (source.contains("<license>") || source.contains("r:license"))
            && (source.contains("<grant>") || source.contains("r:grant"))
            || source.contains("MPEG-21")
    }
}

/// Exporter for MPEG-21 REL format.
pub struct Mpeg21RelExporter;

impl Mpeg21RelExporter {
    /// Creates a new MPEG-21 REL exporter.
    pub fn new() -> Self {
        Self
    }

    fn statute_to_grant(&self, statute: &Statute) -> Mpeg21Grant {
        Mpeg21Grant {
            id: Some(statute.id.clone()),
            principal: Some("holder".to_string()),
            right: Mpeg21Right {
                value: statute.effect.description.clone(),
            },
            resource: Some("content".to_string()),
            condition: None,
        }
    }
}

impl Default for Mpeg21RelExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for Mpeg21RelExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Mpeg21Rel
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::Mpeg21Rel);

        let grants: Vec<Mpeg21Grant> = statutes.iter().map(|s| self.statute_to_grant(s)).collect();

        let doc = Mpeg21RelDocument {
            license: Mpeg21License { grants },
        };

        let output = quick_xml::se::to_string(&doc).map_err(|e| {
            InteropError::SerializationError(format!("Failed to serialize MPEG-21 REL: {}", e))
        })?;

        report.statutes_converted = statutes.len();
        report.add_warning(
            "MPEG-21 REL is designed for digital rights, legal semantics may be simplified",
        );

        Ok((output, report))
    }

    fn can_represent(&self, _statute: &Statute) -> Vec<String> {
        vec![
            "Complex legal preconditions not supported".to_string(),
            "Designed for digital rights, not general legal rules".to_string(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mpeg21_rel_export() {
        let exporter = Mpeg21RelExporter::new();
        let statute = Statute::new(
            "play-right",
            "Play Right",
            Effect::new(EffectType::Grant, "play"),
        );

        let (output, report) = exporter.export(&[statute]).unwrap();

        assert!(output.contains("r:license") || output.contains("license"));
        assert!(output.contains("play"));
        assert_eq!(report.statutes_converted, 1);
    }

    #[test]
    fn test_mpeg21_rel_import() {
        let importer = Mpeg21RelImporter::new();
        let source = r#"<Mpeg21RelDocument>
            <license>
                <grant grantId="grant-1">
                    <principal>User</principal>
                    <right>play</right>
                    <resource>video.mp4</resource>
                </grant>
            </license>
        </Mpeg21RelDocument>"#;

        let (statutes, report) = importer.import(source).unwrap();

        assert_eq!(statutes.len(), 1);
        assert!(statutes[0].title.contains("play"));
        assert_eq!(report.statutes_converted, 1);
    }

    #[test]
    fn test_mpeg21_rel_validate() {
        let importer = Mpeg21RelImporter::new();
        assert!(importer.validate("<license><grant></grant></license>"));
        assert!(importer.validate("<r:license><r:grant></r:grant></r:license>"));
        assert!(!importer.validate("not mpeg21"));
    }

    #[test]
    fn test_mpeg21_rel_roundtrip() {
        let exporter = Mpeg21RelExporter::new();
        let importer = Mpeg21RelImporter::new();

        let original = Statute::new(
            "copy-right",
            "Copy Right",
            Effect::new(EffectType::Grant, "copy"),
        );

        let (exported, _) = exporter.export(std::slice::from_ref(&original)).unwrap();
        let (imported, _) = importer.import(&exported).unwrap();

        assert_eq!(imported.len(), 1);
        assert!(imported[0].effect.description.contains("copy"));
    }
}
