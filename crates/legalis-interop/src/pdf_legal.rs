//! Adobe PDF legal annotations format support.
//!
//! Adobe PDF supports rich annotation and form field capabilities for legal documents.
//! This module handles:
//! - Text annotations and comments
//! - Form fields and signature fields
//! - Legal stamps and watermarks
//! - Document metadata and properties
//!
//! This module provides bidirectional conversion between PDF legal annotations
//! format and legalis_core::Statute format.

use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::{Effect, EffectType, Statute};
use serde::{Deserialize, Serialize};

/// PDF legal document structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfLegalDocument {
    /// Document metadata
    pub metadata: PdfMetadata,
    /// Annotations (comments, highlights, stamps)
    pub annotations: Vec<PdfAnnotation>,
    /// Form fields (signature, text, checkbox)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_fields: Option<Vec<PdfFormField>>,
    /// Document properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PdfProperties>,
}

/// PDF document metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfMetadata {
    /// Document title
    pub title: String,
    /// Author
    pub author: String,
    /// Subject
    pub subject: String,
    /// Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    /// Creation date (ISO 8601)
    pub created: String,
    /// Modified date (ISO 8601)
    pub modified: String,
}

/// PDF annotation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfAnnotation {
    /// Annotation ID
    pub id: String,
    /// Annotation type (Text, Highlight, Stamp, FreeText)
    pub annotation_type: String,
    /// Page number (1-based)
    pub page: i32,
    /// Author of annotation
    pub author: String,
    /// Annotation content/text
    pub content: String,
    /// Creation date (ISO 8601)
    pub created: String,
    /// Annotation subject/category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Legal significance (Clause, Obligation, Condition, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_category: Option<String>,
}

/// PDF form field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfFormField {
    /// Field name
    pub name: String,
    /// Field type (Signature, Text, CheckBox, RadioButton)
    pub field_type: String,
    /// Field value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Required field
    pub required: bool,
    /// Page number
    pub page: i32,
    /// Field description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// PDF document properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfProperties {
    /// Document is certified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certified: Option<bool>,
    /// Requires signature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_signature: Option<bool>,
    /// Legal status (Draft, Final, Executed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_status: Option<String>,
}

/// PDF Legal importer
pub struct PdfLegalImporter;

impl PdfLegalImporter {
    /// Creates a new PDF Legal importer
    pub fn new() -> Self {
        Self
    }

    fn parse_pdf_legal(&self, source: &str) -> InteropResult<PdfLegalDocument> {
        serde_json::from_str(source)
            .map_err(|e| InteropError::ParseError(format!("Failed to parse PDF legal JSON: {}", e)))
    }

    fn convert_annotation_to_statute(&self, annotation: &PdfAnnotation) -> Statute {
        let effect_type = match annotation.legal_category.as_deref().unwrap_or("General") {
            "Clause" | "Grant" => EffectType::Grant,
            "Obligation" | "Requirement" => EffectType::Obligation,
            "Prohibition" | "Restriction" => EffectType::Prohibition,
            "Termination" => EffectType::Revoke,
            _ => EffectType::Custom,
        };

        let mut effect = Effect::new(effect_type, &annotation.content);
        effect.parameters.insert(
            "annotation_type".to_string(),
            annotation.annotation_type.clone(),
        );
        effect
            .parameters
            .insert("page".to_string(), annotation.page.to_string());
        effect
            .parameters
            .insert("author".to_string(), annotation.author.clone());

        if let Some(subject) = &annotation.subject {
            effect
                .parameters
                .insert("subject".to_string(), subject.clone());
        }

        if let Some(category) = &annotation.legal_category {
            effect
                .parameters
                .insert("legal_category".to_string(), category.clone());
        }

        Statute::new(&annotation.id, &annotation.content, effect)
    }

    fn convert_form_field_to_statute(&self, field: &PdfFormField) -> Statute {
        let effect_type = match field.field_type.as_str() {
            "Signature" => EffectType::Obligation,
            "CheckBox" if field.required => EffectType::Obligation,
            _ => EffectType::Custom,
        };

        let title = field
            .description
            .clone()
            .unwrap_or_else(|| field.name.clone());
        let mut effect = Effect::new(effect_type, &title);
        effect
            .parameters
            .insert("field_type".to_string(), field.field_type.clone());
        effect
            .parameters
            .insert("field_name".to_string(), field.name.clone());
        effect
            .parameters
            .insert("required".to_string(), field.required.to_string());
        effect
            .parameters
            .insert("page".to_string(), field.page.to_string());

        if let Some(value) = &field.value {
            effect.parameters.insert("value".to_string(), value.clone());
        }

        Statute::new(&field.name, &title, effect)
    }
}

impl Default for PdfLegalImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for PdfLegalImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::PdfLegal
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let doc = self.parse_pdf_legal(source)?;
        let mut report = ConversionReport::new(LegalFormat::PdfLegal, LegalFormat::Legalis);
        let mut statutes = Vec::new();

        // Convert annotations to statutes
        for annotation in &doc.annotations {
            statutes.push(self.convert_annotation_to_statute(annotation));
        }

        // Convert form fields to statutes
        if let Some(fields) = &doc.form_fields {
            for field in fields {
                statutes.push(self.convert_form_field_to_statute(field));
            }
        }

        report.statutes_converted = statutes.len();

        if statutes.is_empty() {
            report.add_warning(
                "No annotations or form fields found in PDF legal document".to_string(),
            );
        }

        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        // Try to parse as JSON and check for PDF legal specific fields
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(source)
            && let Some(obj) = value.as_object()
        {
            return obj.contains_key("metadata") && obj.contains_key("annotations");
        }
        false
    }
}

/// PDF Legal exporter
pub struct PdfLegalExporter;

impl PdfLegalExporter {
    /// Creates a new PDF Legal exporter
    pub fn new() -> Self {
        Self
    }

    fn convert_statute_to_annotation(&self, statute: &Statute, page: i32) -> PdfAnnotation {
        let annotation_type = statute
            .effect
            .parameters
            .get("annotation_type")
            .cloned()
            .unwrap_or_else(|| "Text".to_string());

        let author = statute
            .effect
            .parameters
            .get("author")
            .cloned()
            .unwrap_or_else(|| "System".to_string());

        let subject = statute.effect.parameters.get("subject").cloned();

        let legal_category = match statute.effect.effect_type {
            EffectType::Grant => Some("Clause".to_string()),
            EffectType::Obligation => Some("Obligation".to_string()),
            EffectType::Prohibition => Some("Prohibition".to_string()),
            EffectType::Revoke => Some("Termination".to_string()),
            _ => None,
        };

        PdfAnnotation {
            id: statute.id.clone(),
            annotation_type,
            page,
            author,
            content: statute.title.clone(),
            created: chrono::Utc::now().to_rfc3339(),
            subject,
            legal_category,
        }
    }
}

impl Default for PdfLegalExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for PdfLegalExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::PdfLegal
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::PdfLegal);

        let annotations: Vec<PdfAnnotation> = statutes
            .iter()
            .enumerate()
            .map(|(i, s)| self.convert_statute_to_annotation(s, (i / 10 + 1) as i32))
            .collect();

        let doc = PdfLegalDocument {
            metadata: PdfMetadata {
                title: "Legal Document".to_string(),
                author: "System".to_string(),
                subject: "Legal Annotations".to_string(),
                keywords: Some(vec!["legal".to_string(), "contract".to_string()]),
                created: chrono::Utc::now().to_rfc3339(),
                modified: chrono::Utc::now().to_rfc3339(),
            },
            annotations,
            form_fields: None,
            properties: Some(PdfProperties {
                certified: Some(false),
                requires_signature: Some(true),
                legal_status: Some("Draft".to_string()),
            }),
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
    fn test_pdf_legal_validate() {
        let importer = PdfLegalImporter::new();

        let valid_json = r#"{
            "metadata": {
                "title": "Test Document",
                "author": "Test Author",
                "subject": "Legal",
                "created": "2024-01-01T00:00:00Z",
                "modified": "2024-01-01T00:00:00Z"
            },
            "annotations": []
        }"#;

        assert!(importer.validate(valid_json));

        let invalid_json = r#"{"random": "data"}"#;
        assert!(!importer.validate(invalid_json));
    }

    #[test]
    fn test_pdf_legal_import() {
        let importer = PdfLegalImporter::new();

        let json = r#"{
            "metadata": {
                "title": "Contract",
                "author": "Legal Team",
                "subject": "Agreement",
                "created": "2024-01-01T00:00:00Z",
                "modified": "2024-01-01T00:00:00Z"
            },
            "annotations": [
                {
                    "id": "annot-1",
                    "annotation_type": "Highlight",
                    "page": 1,
                    "author": "Reviewer",
                    "content": "Payment obligation",
                    "created": "2024-01-01T00:00:00Z",
                    "legal_category": "Obligation"
                }
            ]
        }"#;

        let (statutes, report) = importer.import(json).unwrap();
        assert_eq!(statutes.len(), 1);
        assert_eq!(report.statutes_converted, 1);
        assert_eq!(statutes[0].id, "annot-1");
    }

    #[test]
    fn test_pdf_legal_export() {
        let exporter = PdfLegalExporter::new();

        let statute = Statute::new(
            "obligation-1",
            "Payment obligation",
            Effect::new(EffectType::Obligation, "Pay within 30 days"),
        );

        let (output, report) = exporter.export(&[statute]).unwrap();
        assert_eq!(report.statutes_converted, 1);
        assert!(output.contains("Legal Document"));
        assert!(output.contains("Payment obligation"));
    }

    #[test]
    fn test_pdf_legal_roundtrip() {
        let importer = PdfLegalImporter::new();
        let exporter = PdfLegalExporter::new();

        let original_json = r#"{
            "metadata": {
                "title": "Test",
                "author": "Test",
                "subject": "Test",
                "created": "2024-01-01T00:00:00Z",
                "modified": "2024-01-01T00:00:00Z"
            },
            "annotations": [
                {
                    "id": "test-1",
                    "annotation_type": "Text",
                    "page": 1,
                    "author": "Test",
                    "content": "Test clause",
                    "created": "2024-01-01T00:00:00Z",
                    "legal_category": "Clause"
                }
            ]
        }"#;

        let (statutes, _) = importer.import(original_json).unwrap();
        let (output, _) = exporter.export(&statutes).unwrap();

        // Verify the exported JSON is valid
        let doc: PdfLegalDocument = serde_json::from_str(&output).unwrap();
        assert_eq!(doc.annotations.len(), 1);
    }
}
