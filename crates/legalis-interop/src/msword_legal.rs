//! Microsoft Word Legal Add-in format support.
//!
//! Microsoft Word legal add-ins (such as ContractExpress, HotDocs, etc.) use
//! structured XML formats for legal document automation. This module handles:
//! - Document structure with clauses and sections
//! - Field definitions and conditional content
//! - Metadata and document properties
//! - Clause libraries and templates
//!
//! This module provides bidirectional conversion between MS Word legal add-in
//! format and legalis_core::Statute format.

use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::{Condition, Effect, EffectType, Statute};
use serde::{Deserialize, Serialize};

/// Microsoft Word legal document structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsWordLegalDocument {
    /// Document metadata
    pub metadata: MsWordMetadata,
    /// Document sections
    pub sections: Vec<MsWordSection>,
    /// Clause library references
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clause_library: Option<Vec<MsWordClause>>,
    /// Field definitions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<MsWordField>>,
}

/// MS Word document metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsWordMetadata {
    /// Document title
    pub title: String,
    /// Document type (Agreement, Policy, Notice, etc.)
    pub document_type: String,
    /// Version
    pub version: String,
    /// Author
    pub author: String,
    /// Creation date (ISO 8601)
    pub created: String,
    /// Last modified date (ISO 8601)
    pub modified: String,
    /// Jurisdiction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<String>,
}

/// MS Word document section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsWordSection {
    /// Section ID
    pub id: String,
    /// Section title
    pub title: String,
    /// Section number
    pub number: String,
    /// Section content/text
    pub content: String,
    /// Clauses in this section
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clauses: Option<Vec<MsWordClause>>,
}

/// MS Word clause
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsWordClause {
    /// Clause ID
    pub id: String,
    /// Clause type (Indemnity, Warranty, Termination, etc.)
    pub clause_type: String,
    /// Clause title
    pub title: String,
    /// Clause text
    pub text: String,
    /// Optional/Required
    pub required: bool,
    /// Conditional logic
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    /// Risk level (Low, Medium, High)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_level: Option<String>,
}

/// MS Word field definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsWordField {
    /// Field name
    pub name: String,
    /// Field type (Text, Date, Number, Boolean)
    pub field_type: String,
    /// Default value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// Required field
    pub required: bool,
    /// Description/help text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// MS Word Legal importer
pub struct MsWordLegalImporter;

impl MsWordLegalImporter {
    /// Creates a new MS Word Legal importer
    pub fn new() -> Self {
        Self
    }

    fn parse_msword_legal(&self, source: &str) -> InteropResult<MsWordLegalDocument> {
        serde_json::from_str(source).map_err(|e| {
            InteropError::ParseError(format!("Failed to parse MS Word legal JSON: {}", e))
        })
    }

    fn convert_clause_to_statute(&self, clause: &MsWordClause) -> Statute {
        let effect_type = match clause.clause_type.as_str() {
            "Indemnity" | "Warranty" | "Grant" => EffectType::Grant,
            "Limitation" | "Prohibition" => EffectType::Prohibition,
            "Termination" => EffectType::Revoke,
            "Obligation" | "Covenant" => EffectType::Obligation,
            _ => EffectType::Grant,
        };

        let mut effect = Effect::new(effect_type, &clause.title);
        effect
            .parameters
            .insert("clause_text".to_string(), clause.text.clone());
        effect
            .parameters
            .insert("clause_type".to_string(), clause.clause_type.clone());
        effect
            .parameters
            .insert("required".to_string(), clause.required.to_string());

        if let Some(risk_level) = &clause.risk_level {
            effect
                .parameters
                .insert("risk_level".to_string(), risk_level.clone());
        }

        let mut statute = Statute::new(&clause.id, &clause.title, effect);

        // Add conditional logic as precondition
        if let Some(condition_text) = &clause.condition {
            statute = statute.with_precondition(Condition::Custom {
                description: condition_text.clone(),
            });
        }

        statute
    }
}

impl Default for MsWordLegalImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for MsWordLegalImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::MsWordLegal
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let doc = self.parse_msword_legal(source)?;
        let mut report = ConversionReport::new(LegalFormat::MsWordLegal, LegalFormat::Legalis);
        let mut statutes = Vec::new();

        // Convert clauses from clause library
        if let Some(clause_lib) = &doc.clause_library {
            for clause in clause_lib {
                statutes.push(self.convert_clause_to_statute(clause));
            }
        }

        // Convert clauses from sections
        for section in &doc.sections {
            if let Some(clauses) = &section.clauses {
                for clause in clauses {
                    statutes.push(self.convert_clause_to_statute(clause));
                }
            }
        }

        report.statutes_converted = statutes.len();

        if statutes.is_empty() {
            report.add_warning("No clauses found in MS Word legal document".to_string());
        }

        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        // Try to parse as JSON and check for MS Word legal specific fields
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(source) {
            if let Some(obj) = value.as_object() {
                return obj.contains_key("metadata")
                    && obj.contains_key("sections")
                    && obj
                        .get("metadata")
                        .and_then(|m| m.get("document_type"))
                        .is_some();
            }
        }
        false
    }
}

/// MS Word Legal exporter
pub struct MsWordLegalExporter;

impl MsWordLegalExporter {
    /// Creates a new MS Word Legal exporter
    pub fn new() -> Self {
        Self
    }

    fn convert_statute_to_clause(&self, statute: &Statute) -> MsWordClause {
        let clause_type = match statute.effect.effect_type {
            EffectType::Grant => "Grant",
            EffectType::Obligation => "Obligation",
            EffectType::Prohibition => "Limitation",
            EffectType::Revoke => "Termination",
            _ => "General",
        };

        let text = statute
            .effect
            .parameters
            .get("clause_text")
            .cloned()
            .unwrap_or_else(|| statute.title.clone());

        let required = statute
            .effect
            .parameters
            .get("required")
            .and_then(|s| s.parse::<bool>().ok())
            .unwrap_or(true);

        let risk_level = statute.effect.parameters.get("risk_level").cloned();

        let condition = if !statute.preconditions.is_empty() {
            Some(format!("{:?}", statute.preconditions))
        } else {
            None
        };

        MsWordClause {
            id: statute.id.clone(),
            clause_type: clause_type.to_string(),
            title: statute.title.clone(),
            text,
            required,
            condition,
            risk_level,
        }
    }
}

impl Default for MsWordLegalExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for MsWordLegalExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::MsWordLegal
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::MsWordLegal);

        let clauses: Vec<MsWordClause> = statutes
            .iter()
            .map(|s| self.convert_statute_to_clause(s))
            .collect();

        let doc = MsWordLegalDocument {
            metadata: MsWordMetadata {
                title: "Legal Document".to_string(),
                document_type: "Agreement".to_string(),
                version: "1.0".to_string(),
                author: "System".to_string(),
                created: chrono::Utc::now().to_rfc3339(),
                modified: chrono::Utc::now().to_rfc3339(),
                jurisdiction: statutes.first().and_then(|s| s.jurisdiction.clone()),
            },
            sections: vec![MsWordSection {
                id: "section-1".to_string(),
                title: "Terms and Conditions".to_string(),
                number: "1".to_string(),
                content: "Legal provisions".to_string(),
                clauses: Some(clauses),
            }],
            clause_library: None,
            fields: None,
        };

        let json = serde_json::to_string_pretty(&doc).map_err(|e| {
            InteropError::SerializationError(format!("JSON serialization failed: {}", e))
        })?;

        report.statutes_converted = statutes.len();

        Ok((json, report))
    }

    fn can_represent(&self, _statute: &Statute) -> Vec<String> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType, Statute};

    #[test]
    fn test_msword_legal_validate() {
        let importer = MsWordLegalImporter::new();

        let valid_json = r#"{
            "metadata": {
                "title": "Test Agreement",
                "document_type": "Agreement",
                "version": "1.0",
                "author": "Test",
                "created": "2024-01-01T00:00:00Z",
                "modified": "2024-01-01T00:00:00Z"
            },
            "sections": []
        }"#;

        assert!(importer.validate(valid_json));

        let invalid_json = r#"{"random": "data"}"#;
        assert!(!importer.validate(invalid_json));
    }

    #[test]
    fn test_msword_legal_import() {
        let importer = MsWordLegalImporter::new();

        let json = r#"{
            "metadata": {
                "title": "Employment Agreement",
                "document_type": "Agreement",
                "version": "1.0",
                "author": "HR Dept",
                "created": "2024-01-01T00:00:00Z",
                "modified": "2024-01-01T00:00:00Z"
            },
            "sections": [
                {
                    "id": "sec-1",
                    "title": "Terms",
                    "number": "1",
                    "content": "Employment terms",
                    "clauses": [
                        {
                            "id": "clause-1",
                            "clause_type": "Obligation",
                            "title": "Non-Compete",
                            "text": "Employee agrees not to compete",
                            "required": true
                        }
                    ]
                }
            ]
        }"#;

        let (statutes, report) = importer.import(json).unwrap();
        assert_eq!(statutes.len(), 1);
        assert_eq!(report.statutes_converted, 1);
        assert_eq!(statutes[0].id, "clause-1");
    }

    #[test]
    fn test_msword_legal_export() {
        let exporter = MsWordLegalExporter::new();

        let statute = Statute::new(
            "clause-1",
            "Confidentiality",
            Effect::new(EffectType::Obligation, "Keep information confidential"),
        );

        let (output, report) = exporter.export(&[statute]).unwrap();
        assert_eq!(report.statutes_converted, 1);
        assert!(output.contains("Legal Document"));
        assert!(output.contains("Confidentiality"));
    }

    #[test]
    fn test_msword_legal_roundtrip() {
        let importer = MsWordLegalImporter::new();
        let exporter = MsWordLegalExporter::new();

        let original_json = r#"{
            "metadata": {
                "title": "Test Agreement",
                "document_type": "Agreement",
                "version": "1.0",
                "author": "Test",
                "created": "2024-01-01T00:00:00Z",
                "modified": "2024-01-01T00:00:00Z"
            },
            "sections": [],
            "clause_library": [
                {
                    "id": "warranty-1",
                    "clause_type": "Warranty",
                    "title": "Product Warranty",
                    "text": "Warranty terms",
                    "required": true
                }
            ]
        }"#;

        let (statutes, _) = importer.import(original_json).unwrap();
        let (output, _) = exporter.export(&statutes).unwrap();

        // Verify the exported JSON is valid
        let doc: MsWordLegalDocument = serde_json::from_str(&output).unwrap();
        assert_eq!(doc.sections.len(), 1);
    }
}
