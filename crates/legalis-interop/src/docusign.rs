//! DocuSign envelope format support.
//!
//! DocuSign is an electronic signature and digital transaction management platform.
//! This module handles:
//! - Envelope structure and recipients
//! - Document tabs and fields
//! - Routing and signing workflow
//! - Envelope status and events
//!
//! This module provides bidirectional conversion between DocuSign envelope
//! format and legalis_core::Statute format.

use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::{Effect, EffectType, Statute};
use serde::{Deserialize, Serialize};

/// DocuSign envelope structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocuSignEnvelope {
    /// Envelope ID
    #[serde(rename = "envelopeId")]
    pub envelope_id: String,
    /// Envelope status (created, sent, delivered, signed, completed, declined, voided)
    pub status: String,
    /// Email subject
    #[serde(rename = "emailSubject")]
    pub email_subject: String,
    /// Documents in the envelope
    pub documents: Vec<DocuSignDocument>,
    /// Recipients (signers, carbon copies, etc.)
    pub recipients: DocuSignRecipients,
    /// Custom fields
    #[serde(rename = "customFields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<DocuSignCustomFields>,
}

/// DocuSign document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocuSignDocument {
    /// Document ID
    #[serde(rename = "documentId")]
    pub document_id: String,
    /// Document name
    pub name: String,
    /// File extension
    #[serde(rename = "fileExtension")]
    pub file_extension: String,
    /// Document order
    pub order: String,
    /// Pages count
    pub pages: Option<String>,
}

/// DocuSign recipients container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocuSignRecipients {
    /// Signers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signers: Option<Vec<DocuSignSigner>>,
    /// Carbon copy recipients
    #[serde(rename = "carbonCopies", skip_serializing_if = "Option::is_none")]
    pub carbon_copies: Option<Vec<DocuSignCarbonCopy>>,
}

/// DocuSign signer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocuSignSigner {
    /// Recipient ID
    #[serde(rename = "recipientId")]
    pub recipient_id: String,
    /// Email address
    pub email: String,
    /// Name
    pub name: String,
    /// Routing order
    #[serde(rename = "routingOrder")]
    pub routing_order: String,
    /// Status
    pub status: Option<String>,
    /// Tabs (signature fields, text fields, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tabs: Option<DocuSignTabs>,
}

/// DocuSign carbon copy recipient
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocuSignCarbonCopy {
    /// Recipient ID
    #[serde(rename = "recipientId")]
    pub recipient_id: String,
    /// Email address
    pub email: String,
    /// Name
    pub name: String,
    /// Routing order
    #[serde(rename = "routingOrder")]
    pub routing_order: String,
}

/// DocuSign tabs (fields)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocuSignTabs {
    /// Signature tabs
    #[serde(rename = "signHereTabs", skip_serializing_if = "Option::is_none")]
    pub sign_here_tabs: Option<Vec<DocuSignTab>>,
    /// Text tabs
    #[serde(rename = "textTabs", skip_serializing_if = "Option::is_none")]
    pub text_tabs: Option<Vec<DocuSignTab>>,
    /// Date tabs
    #[serde(rename = "dateSignedTabs", skip_serializing_if = "Option::is_none")]
    pub date_signed_tabs: Option<Vec<DocuSignTab>>,
}

/// DocuSign tab (field)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocuSignTab {
    /// Tab label
    #[serde(rename = "tabLabel")]
    pub tab_label: String,
    /// Document ID
    #[serde(rename = "documentId")]
    pub document_id: String,
    /// Page number
    #[serde(rename = "pageNumber", skip_serializing_if = "Option::is_none")]
    pub page_number: Option<String>,
    /// Required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<String>,
}

/// DocuSign custom fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocuSignCustomFields {
    /// Text custom fields
    #[serde(rename = "textCustomFields", skip_serializing_if = "Option::is_none")]
    pub text_custom_fields: Option<Vec<DocuSignTextField>>,
}

/// DocuSign text field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocuSignTextField {
    /// Field ID
    #[serde(rename = "fieldId")]
    pub field_id: String,
    /// Name
    pub name: String,
    /// Value
    pub value: String,
}

/// DocuSign importer
pub struct DocuSignImporter;

impl DocuSignImporter {
    /// Creates a new DocuSign importer
    pub fn new() -> Self {
        Self
    }

    fn parse_docusign(&self, source: &str) -> InteropResult<DocuSignEnvelope> {
        serde_json::from_str(source).map_err(|e| {
            InteropError::ParseError(format!("Failed to parse DocuSign envelope JSON: {}", e))
        })
    }

    fn convert_signer_to_statute(&self, signer: &DocuSignSigner) -> Statute {
        let mut effect = Effect::new(EffectType::Obligation, "Sign document");
        effect
            .parameters
            .insert("signer_email".to_string(), signer.email.clone());
        effect
            .parameters
            .insert("signer_name".to_string(), signer.name.clone());
        effect
            .parameters
            .insert("routing_order".to_string(), signer.routing_order.clone());

        if let Some(status) = &signer.status {
            effect
                .parameters
                .insert("status".to_string(), status.clone());
        }

        // Add tab information
        if let Some(tabs) = &signer.tabs {
            if let Some(sign_tabs) = &tabs.sign_here_tabs {
                effect
                    .parameters
                    .insert("signature_tabs".to_string(), sign_tabs.len().to_string());
            }
            if let Some(text_tabs) = &tabs.text_tabs {
                effect
                    .parameters
                    .insert("text_tabs".to_string(), text_tabs.len().to_string());
            }
        }

        Statute::new(&signer.recipient_id, &signer.name, effect)
    }
}

impl Default for DocuSignImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for DocuSignImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::DocuSign
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let envelope = self.parse_docusign(source)?;
        let mut report = ConversionReport::new(LegalFormat::DocuSign, LegalFormat::Legalis);
        let mut statutes = Vec::new();

        // Convert signers to statutes (signing obligations)
        if let Some(signers) = envelope.recipients.signers {
            for signer in &signers {
                statutes.push(self.convert_signer_to_statute(signer));
            }
        }

        report.statutes_converted = statutes.len();

        if statutes.is_empty() {
            report.add_warning("No signers found in DocuSign envelope".to_string());
        }

        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        // Try to parse as JSON and check for DocuSign specific fields
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(source) {
            if let Some(obj) = value.as_object() {
                return obj.contains_key("envelopeId")
                    && obj.contains_key("status")
                    && obj.contains_key("recipients");
            }
        }
        false
    }
}

/// DocuSign exporter
pub struct DocuSignExporter;

impl DocuSignExporter {
    /// Creates a new DocuSign exporter
    pub fn new() -> Self {
        Self
    }

    fn convert_statute_to_signer(&self, statute: &Statute, routing_order: usize) -> DocuSignSigner {
        let email = statute
            .effect
            .parameters
            .get("signer_email")
            .cloned()
            .unwrap_or_else(|| format!("signer{}@example.com", routing_order));

        let name = statute
            .effect
            .parameters
            .get("signer_name")
            .cloned()
            .unwrap_or_else(|| statute.title.clone());

        DocuSignSigner {
            recipient_id: statute.id.clone(),
            email,
            name,
            routing_order: routing_order.to_string(),
            status: statute.effect.parameters.get("status").cloned(),
            tabs: None,
        }
    }
}

impl Default for DocuSignExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for DocuSignExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::DocuSign
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::DocuSign);

        let signers: Vec<DocuSignSigner> = statutes
            .iter()
            .enumerate()
            .map(|(i, s)| self.convert_statute_to_signer(s, i + 1))
            .collect();

        let envelope = DocuSignEnvelope {
            envelope_id: "ENV-001".to_string(),
            status: "created".to_string(),
            email_subject: "Please sign this document".to_string(),
            documents: vec![DocuSignDocument {
                document_id: "1".to_string(),
                name: "Legal Document".to_string(),
                file_extension: "pdf".to_string(),
                order: "1".to_string(),
                pages: None,
            }],
            recipients: DocuSignRecipients {
                signers: Some(signers),
                carbon_copies: None,
            },
            custom_fields: None,
        };

        let json = serde_json::to_string_pretty(&envelope).map_err(|e| {
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
    fn test_docusign_validate() {
        let importer = DocuSignImporter::new();

        let valid_json = r#"{
            "envelopeId": "abc123",
            "status": "sent",
            "emailSubject": "Please sign",
            "documents": [],
            "recipients": {
                "signers": []
            }
        }"#;

        assert!(importer.validate(valid_json));

        let invalid_json = r#"{"random": "data"}"#;
        assert!(!importer.validate(invalid_json));
    }

    #[test]
    fn test_docusign_import() {
        let importer = DocuSignImporter::new();

        let json = r#"{
            "envelopeId": "abc123",
            "status": "sent",
            "emailSubject": "Please sign this contract",
            "documents": [
                {
                    "documentId": "1",
                    "name": "Contract.pdf",
                    "fileExtension": "pdf",
                    "order": "1"
                }
            ],
            "recipients": {
                "signers": [
                    {
                        "recipientId": "1",
                        "email": "john@example.com",
                        "name": "John Doe",
                        "routingOrder": "1",
                        "status": "sent"
                    }
                ]
            }
        }"#;

        let (statutes, report) = importer.import(json).unwrap();
        assert_eq!(statutes.len(), 1);
        assert_eq!(report.statutes_converted, 1);
        assert_eq!(statutes[0].id, "1");
    }

    #[test]
    fn test_docusign_export() {
        let exporter = DocuSignExporter::new();

        let statute = Statute::new(
            "signer-1",
            "John Doe",
            Effect::new(EffectType::Obligation, "Sign document"),
        );

        let (output, report) = exporter.export(&[statute]).unwrap();
        assert_eq!(report.statutes_converted, 1);
        assert!(output.contains("ENV-001"));
        assert!(output.contains("John Doe"));
    }

    #[test]
    fn test_docusign_roundtrip() {
        let importer = DocuSignImporter::new();
        let exporter = DocuSignExporter::new();

        let original_json = r#"{
            "envelopeId": "abc123",
            "status": "sent",
            "emailSubject": "Sign contract",
            "documents": [],
            "recipients": {
                "signers": [
                    {
                        "recipientId": "1",
                        "email": "alice@example.com",
                        "name": "Alice Smith",
                        "routingOrder": "1"
                    }
                ]
            }
        }"#;

        let (statutes, _) = importer.import(original_json).unwrap();
        let (output, _) = exporter.export(&statutes).unwrap();

        // Verify the exported JSON is valid
        let envelope: DocuSignEnvelope = serde_json::from_str(&output).unwrap();
        assert!(envelope.recipients.signers.is_some());
        assert_eq!(envelope.recipients.signers.unwrap().len(), 1);
    }
}
