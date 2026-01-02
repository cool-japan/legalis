//! RegML (Regulation Markup Language) format support.
//!
//! RegML is an XML-based format for representing regulatory requirements
//! and compliance rules. It supports:
//! - Regulatory texts and provisions
//! - Compliance obligations
//! - Reporting requirements
//! - Enforcement rules

use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::{Condition, Effect, EffectType, Statute};
use serde::{Deserialize, Serialize};

/// RegML document structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegMLDocument {
    /// Document metadata
    pub metadata: RegMLMetadata,
    /// Regulatory provisions
    pub provisions: Vec<RegMLProvision>,
}

/// RegML metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegMLMetadata {
    /// Regulation ID
    pub regulation_id: String,
    /// Title
    pub title: String,
    /// Issuing authority
    pub authority: String,
    /// Jurisdiction
    pub jurisdiction: String,
    /// Publication date
    pub publication_date: Option<String>,
    /// Effective date
    pub effective_date: Option<String>,
}

/// RegML provision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegMLProvision {
    /// Provision ID
    pub id: String,
    /// Provision title
    pub title: String,
    /// Provision text
    pub text: String,
    /// Provision type (e.g., "obligation", "prohibition", "permission")
    pub provision_type: String,
    /// Applicability conditions
    pub conditions: Vec<RegMLCondition>,
    /// Sanctions for non-compliance
    pub sanctions: Vec<String>,
}

/// RegML condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegMLCondition {
    /// Condition description
    pub description: String,
    /// Condition type
    pub condition_type: String,
    /// Required value or threshold
    pub threshold: Option<String>,
}

/// RegML importer
pub struct RegMLImporter;

impl RegMLImporter {
    /// Creates a new RegML importer
    pub fn new() -> Self {
        Self
    }

    fn parse_regml(&self, source: &str) -> InteropResult<RegMLDocument> {
        // Try JSON format first
        if let Ok(doc) = serde_json::from_str::<RegMLDocument>(source) {
            return Ok(doc);
        }

        // Try XML parsing
        if source.contains("<RegML") || source.contains("<regml") {
            self.parse_regml_xml(source)
        } else {
            Err(InteropError::ParseError("Invalid RegML format".to_string()))
        }
    }

    fn parse_regml_xml(&self, source: &str) -> InteropResult<RegMLDocument> {
        // Simple XML parsing
        let mut doc = RegMLDocument {
            metadata: RegMLMetadata {
                regulation_id: "reg-001".to_string(),
                title: "Regulation".to_string(),
                authority: "Regulatory Authority".to_string(),
                jurisdiction: "Global".to_string(),
                publication_date: None,
                effective_date: None,
            },
            provisions: Vec::new(),
        };

        // Extract title
        if let Some(start) = source.find("<title>") {
            if let Some(end) = source[start..].find("</title>") {
                doc.metadata.title = source[start + 7..start + end].trim().to_string();
            }
        }

        // Extract provisions
        let mut search_start = 0;
        while let Some(prov_start) = source[search_start..].find("<provision") {
            let abs_start = search_start + prov_start;
            if let Some(prov_end) = source[abs_start..].find("</provision>") {
                let prov_content = &source[abs_start..abs_start + prov_end];

                let mut provision = RegMLProvision {
                    id: "prov".to_string(),
                    title: "Provision".to_string(),
                    text: "".to_string(),
                    provision_type: "obligation".to_string(),
                    conditions: Vec::new(),
                    sanctions: Vec::new(),
                };

                // Extract ID
                if let Some(id_start) = prov_content.find("id=\"") {
                    if let Some(id_end) = prov_content[id_start + 4..].find('"') {
                        provision.id =
                            prov_content[id_start + 4..id_start + 4 + id_end].to_string();
                    }
                }

                // Extract text
                if let Some(text_start) = prov_content.find("<text>") {
                    if let Some(text_end) = prov_content[text_start..].find("</text>") {
                        provision.text = prov_content[text_start + 6..text_start + text_end]
                            .trim()
                            .to_string();
                    }
                }

                doc.provisions.push(provision);
                search_start = abs_start + prov_end + 12;
            } else {
                break;
            }
        }

        Ok(doc)
    }
}

impl Default for RegMLImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for RegMLImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::RegML
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let doc = self.parse_regml(source)?;
        let mut statutes = Vec::new();
        let mut report = ConversionReport::new(LegalFormat::RegML, LegalFormat::Legalis);

        for provision in &doc.provisions {
            let effect_type = match provision.provision_type.to_lowercase().as_str() {
                "prohibition" => EffectType::Prohibition,
                "permission" => EffectType::Grant,
                "obligation" | "requirement" => EffectType::Obligation,
                _ => EffectType::Obligation,
            };

            let effect = Effect::new(effect_type, &provision.text);

            let mut statute = Statute::new(&provision.id, &provision.title, effect);

            // Add conditions
            for condition in &provision.conditions {
                let desc = if let Some(threshold) = &condition.threshold {
                    format!("{} >= {}", condition.condition_type, threshold)
                } else {
                    condition.description.clone()
                };
                statute = statute.with_precondition(Condition::Custom { description: desc });
            }

            // Add metadata to parameters
            statute.effect.parameters.insert(
                "regulation_id".to_string(),
                doc.metadata.regulation_id.clone(),
            );
            statute
                .effect
                .parameters
                .insert("authority".to_string(), doc.metadata.authority.clone());
            statute.effect.parameters.insert(
                "provision_type".to_string(),
                provision.provision_type.clone(),
            );

            if let Some(pub_date) = &doc.metadata.publication_date {
                statute
                    .effect
                    .parameters
                    .insert("publication_date".to_string(), pub_date.clone());
            }

            if let Some(eff_date) = &doc.metadata.effective_date {
                statute
                    .effect
                    .parameters
                    .insert("effective_date".to_string(), eff_date.clone());
            }

            statute = statute.with_jurisdiction(&doc.metadata.jurisdiction);

            statutes.push(statute);
        }

        report.statutes_converted = statutes.len();
        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        source.contains("<RegML")
            || source.contains("<regml")
            || (source.contains("\"metadata\"") && source.contains("\"provisions\""))
    }
}

/// RegML exporter
pub struct RegMLExporter;

impl RegMLExporter {
    /// Creates a new RegML exporter
    pub fn new() -> Self {
        Self
    }
}

impl Default for RegMLExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for RegMLExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::RegML
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::RegML);

        let first_statute = statutes.first();

        let metadata = RegMLMetadata {
            regulation_id: first_statute
                .and_then(|s| s.effect.parameters.get("regulation_id"))
                .unwrap_or(&"reg-001".to_string())
                .clone(),
            title: first_statute
                .map(|s| s.title.clone())
                .unwrap_or_else(|| "Regulation".to_string()),
            authority: first_statute
                .and_then(|s| s.effect.parameters.get("authority"))
                .unwrap_or(&"Regulatory Authority".to_string())
                .clone(),
            jurisdiction: first_statute
                .and_then(|s| s.jurisdiction.as_ref())
                .unwrap_or(&"Global".to_string())
                .clone(),
            publication_date: first_statute
                .and_then(|s| s.effect.parameters.get("publication_date"))
                .cloned(),
            effective_date: first_statute
                .and_then(|s| s.effect.parameters.get("effective_date"))
                .cloned(),
        };

        let provisions: Vec<RegMLProvision> = statutes
            .iter()
            .map(|statute| {
                let provision_type = statute
                    .effect
                    .parameters
                    .get("provision_type")
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| match statute.effect.effect_type {
                        EffectType::Prohibition => "prohibition".to_string(),
                        EffectType::Grant => "permission".to_string(),
                        EffectType::Obligation => "obligation".to_string(),
                        _ => "provision".to_string(),
                    });

                RegMLProvision {
                    id: statute.id.clone(),
                    title: statute.title.clone(),
                    text: statute.effect.description.clone(),
                    provision_type,
                    conditions: Vec::new(),
                    sanctions: Vec::new(),
                }
            })
            .collect();

        let doc = RegMLDocument {
            metadata,
            provisions,
        };

        let json = serde_json::to_string_pretty(&doc).map_err(|e| {
            InteropError::SerializationError(format!("Failed to serialize RegML: {}", e))
        })?;

        report.statutes_converted = statutes.len();
        Ok((json, report))
    }

    fn can_represent(&self, _statute: &Statute) -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regml_json_import() {
        let source = r#"{
            "metadata": {
                "regulation_id": "REG-2024-001",
                "title": "Data Protection Regulation",
                "authority": "Data Protection Authority",
                "jurisdiction": "EU",
                "publication_date": "2024-01-01",
                "effective_date": "2024-03-01"
            },
            "provisions": [
                {
                    "id": "art-5",
                    "title": "Consent Requirement",
                    "text": "Organizations must obtain explicit consent before processing personal data",
                    "provision_type": "obligation",
                    "conditions": [],
                    "sanctions": ["Fine up to 4% of annual revenue"]
                }
            ]
        }"#;

        let importer = RegMLImporter::new();
        let (statutes, report) = importer.import(source).unwrap();

        assert_eq!(report.statutes_converted, 1);
        assert_eq!(statutes[0].id, "art-5");
        assert_eq!(statutes[0].effect.effect_type, EffectType::Obligation);
    }

    #[test]
    fn test_regml_export() {
        let effect = Effect::new(
            EffectType::Prohibition,
            "Unauthorized data processing is prohibited",
        );

        let mut statute = Statute::new("art-6", "Prohibition of Unauthorized Processing", effect)
            .with_jurisdiction("EU");

        statute
            .effect
            .parameters
            .insert("regulation_id".to_string(), "REG-2024-001".to_string());
        statute
            .effect
            .parameters
            .insert("authority".to_string(), "DPA".to_string());
        statute
            .effect
            .parameters
            .insert("provision_type".to_string(), "prohibition".to_string());

        let exporter = RegMLExporter::new();
        let (output, report) = exporter.export(&[statute]).unwrap();

        assert_eq!(report.statutes_converted, 1);
        assert!(output.contains("art-6"));
        assert!(output.contains("prohibition"));
    }

    #[test]
    fn test_regml_xml_import() {
        let source = r#"
        <RegML>
            <metadata>
                <title>Test Regulation</title>
            </metadata>
            <provision id="p1">
                <text>Must comply with requirements</text>
            </provision>
        </RegML>
        "#;

        let importer = RegMLImporter::new();
        let (statutes, _) = importer.import(source).unwrap();

        assert!(!statutes.is_empty());
    }

    #[test]
    fn test_regml_validate() {
        let importer = RegMLImporter::new();

        assert!(importer.validate(r#"<RegML></RegML>"#));
        assert!(importer.validate(r#"{"metadata": {}, "provisions": []}"#));
        assert!(!importer.validate("not regml"));
    }
}
