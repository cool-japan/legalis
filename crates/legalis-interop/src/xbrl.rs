//! XBRL (eXtensible Business Reporting Language) format support.
//!
//! XBRL is an XML-based standard for digital business reporting used worldwide.
//! It supports:
//! - Financial statements
//! - Regulatory reporting
//! - Business performance metrics
//! - Taxonomies for different reporting frameworks (US-GAAP, IFRS, etc.)

use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::{Effect, EffectType, Statute};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// XBRL instance document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XbrlInstance {
    /// Document metadata
    pub metadata: XbrlMetadata,
    /// Context definitions
    pub contexts: Vec<XbrlContext>,
    /// Facts (data points)
    pub facts: Vec<XbrlFact>,
}

/// XBRL metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XbrlMetadata {
    /// Document identifier
    pub document_id: String,
    /// Entity identifier
    pub entity_id: String,
    /// Reporting period
    pub period: String,
    /// Taxonomy schema reference
    pub schema_ref: String,
    /// Currency (e.g., "USD", "EUR")
    pub currency: Option<String>,
}

/// XBRL context (defines the circumstances of a fact)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XbrlContext {
    /// Context ID
    pub id: String,
    /// Entity identifier
    pub entity: String,
    /// Time period
    pub period: XbrlPeriod,
    /// Scenario/segment dimensions
    pub dimensions: HashMap<String, String>,
}

/// XBRL period
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum XbrlPeriod {
    /// Instant in time
    Instant { date: String },
    /// Duration between two dates
    Duration { start: String, end: String },
}

/// XBRL fact (a single data point)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XbrlFact {
    /// Concept name (from taxonomy)
    pub concept: String,
    /// Context reference
    pub context_ref: String,
    /// Value
    pub value: String,
    /// Unit (e.g., "USD", "shares", "pure")
    pub unit: Option<String>,
    /// Decimals precision
    pub decimals: Option<i32>,
}

/// XBRL importer
pub struct XbrlImporter;

impl XbrlImporter {
    /// Creates a new XBRL importer
    pub fn new() -> Self {
        Self
    }

    fn parse_xbrl(&self, source: &str) -> InteropResult<XbrlInstance> {
        // First try JSON format
        if let Ok(instance) = serde_json::from_str::<XbrlInstance>(source) {
            return Ok(instance);
        }

        // Try simple XML parsing
        if source.contains("<xbrl") || source.contains("<XBRL") {
            self.parse_xbrl_xml(source)
        } else {
            Err(InteropError::ParseError(
                "Not a valid XBRL document".to_string(),
            ))
        }
    }

    fn parse_xbrl_xml(&self, source: &str) -> InteropResult<XbrlInstance> {
        // Simple XML parsing for basic XBRL structure
        let mut instance = XbrlInstance {
            metadata: XbrlMetadata {
                document_id: "xbrl-doc".to_string(),
                entity_id: "entity".to_string(),
                period: "2024".to_string(),
                schema_ref: "http://www.xbrl.org/2003/xbrl-instance-2003-12-31.xsd".to_string(),
                currency: Some("USD".to_string()),
            },
            contexts: Vec::new(),
            facts: Vec::new(),
        };

        // Extract entity ID if present
        if let Some(start) = source.find("<identifier")
            && let Some(close) = source[start..].find('>')
            && let Some(end) = source[start + close..].find("</identifier>")
        {
            let content = &source[start + close + 1..start + close + end];
            instance.metadata.entity_id = content.trim().to_string();
        }

        // Create a default context
        instance.contexts.push(XbrlContext {
            id: "default".to_string(),
            entity: instance.metadata.entity_id.clone(),
            period: XbrlPeriod::Instant {
                date: "2024-12-31".to_string(),
            },
            dimensions: HashMap::new(),
        });

        // Parse facts (simplified - look for common XBRL elements)
        let fact_patterns = [
            ("Assets", "us-gaap:Assets"),
            ("Liabilities", "us-gaap:Liabilities"),
            ("Revenue", "us-gaap:Revenue"),
            ("NetIncome", "us-gaap:NetIncomeLoss"),
        ];

        for (simple_name, concept) in &fact_patterns {
            if let Some(start) = source.find(&format!("<{}", simple_name))
                && let Some(close) = source[start..].find('>')
                && let Some(end) = source[start + close..].find(&format!("</{}", simple_name))
            {
                let content = &source[start + close + 1..start + close + end];
                instance.facts.push(XbrlFact {
                    concept: concept.to_string(),
                    context_ref: "default".to_string(),
                    value: content.trim().to_string(),
                    unit: Some("USD".to_string()),
                    decimals: Some(0),
                });
            }
        }

        Ok(instance)
    }
}

impl Default for XbrlImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for XbrlImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Xbrl
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let instance = self.parse_xbrl(source)?;
        let mut statutes = Vec::new();
        let mut report = ConversionReport::new(LegalFormat::Xbrl, LegalFormat::Legalis);

        // Convert facts to statutes
        for fact in &instance.facts {
            let context = instance.contexts.iter().find(|c| c.id == fact.context_ref);

            let mut effect =
                Effect::new(EffectType::Obligation, format!("Report {}", fact.concept));

            effect
                .parameters
                .insert("concept".to_string(), fact.concept.clone());
            effect
                .parameters
                .insert("value".to_string(), fact.value.clone());

            if let Some(unit) = &fact.unit {
                effect.parameters.insert("unit".to_string(), unit.clone());
            }

            if let Some(decimals) = fact.decimals {
                effect
                    .parameters
                    .insert("decimals".to_string(), decimals.to_string());
            }

            let mut statute = Statute::new(
                fact.concept.replace(':', "_"),
                format!("{} Reporting", fact.concept),
                effect,
            );

            // Add metadata to parameters
            statute
                .effect
                .parameters
                .insert("entity".to_string(), instance.metadata.entity_id.clone());
            statute
                .effect
                .parameters
                .insert("period".to_string(), instance.metadata.period.clone());
            statute
                .effect
                .parameters
                .insert("schema".to_string(), instance.metadata.schema_ref.clone());

            if let Some(currency) = &instance.metadata.currency {
                statute
                    .effect
                    .parameters
                    .insert("currency".to_string(), currency.clone());
            }

            if let Some(ctx) = context {
                statute
                    .effect
                    .parameters
                    .insert("context_id".to_string(), ctx.id.clone());

                // Add period as a condition
                match &ctx.period {
                    XbrlPeriod::Instant { date } => {
                        statute
                            .effect
                            .parameters
                            .insert("instant_date".to_string(), date.clone());
                    }
                    XbrlPeriod::Duration { start, end } => {
                        statute
                            .effect
                            .parameters
                            .insert("start_date".to_string(), start.clone());
                        statute
                            .effect
                            .parameters
                            .insert("end_date".to_string(), end.clone());
                    }
                }
            }

            statutes.push(statute);
        }

        report.statutes_converted = statutes.len();

        if instance.facts.is_empty() {
            report.add_warning("No XBRL facts found in document");
        }

        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        // Check for XBRL-specific markers
        source.contains("<xbrl")
            || source.contains("\"schema_ref\"")
            || source.contains("\"facts\"")
    }
}

/// XBRL exporter
pub struct XbrlExporter;

impl XbrlExporter {
    /// Creates a new XBRL exporter
    pub fn new() -> Self {
        Self
    }
}

impl Default for XbrlExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for XbrlExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Xbrl
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::Xbrl);

        let first_statute = statutes.first();

        let metadata = XbrlMetadata {
            document_id: first_statute
                .and_then(|s| s.effect.parameters.get("document_id"))
                .unwrap_or(&"xbrl-instance".to_string())
                .clone(),
            entity_id: first_statute
                .and_then(|s| s.effect.parameters.get("entity"))
                .unwrap_or(&"ENTITY".to_string())
                .clone(),
            period: first_statute
                .and_then(|s| s.effect.parameters.get("period"))
                .unwrap_or(&"2024".to_string())
                .clone(),
            schema_ref: first_statute
                .and_then(|s| s.effect.parameters.get("schema"))
                .unwrap_or(&"http://www.xbrl.org/2003/xbrl-instance-2003-12-31.xsd".to_string())
                .clone(),
            currency: first_statute
                .and_then(|s| s.effect.parameters.get("currency"))
                .cloned(),
        };

        let mut contexts = Vec::new();
        let mut facts = Vec::new();

        // Create contexts and facts from statutes
        for statute in statutes {
            let context_id = statute
                .effect
                .parameters
                .get("context_id")
                .unwrap_or(&"ctx-default".to_string())
                .clone();

            // Check if context already exists
            if !contexts.iter().any(|c: &XbrlContext| c.id == context_id) {
                let period = if let Some(instant) = statute.effect.parameters.get("instant_date") {
                    XbrlPeriod::Instant {
                        date: instant.clone(),
                    }
                } else if let (Some(start), Some(end)) = (
                    statute.effect.parameters.get("start_date"),
                    statute.effect.parameters.get("end_date"),
                ) {
                    XbrlPeriod::Duration {
                        start: start.clone(),
                        end: end.clone(),
                    }
                } else {
                    XbrlPeriod::Instant {
                        date: "2024-12-31".to_string(),
                    }
                };

                contexts.push(XbrlContext {
                    id: context_id.clone(),
                    entity: metadata.entity_id.clone(),
                    period,
                    dimensions: HashMap::new(),
                });
            }

            // Create fact
            let concept = statute
                .effect
                .parameters
                .get("concept")
                .unwrap_or(&statute.id)
                .clone();

            let value = statute
                .effect
                .parameters
                .get("value")
                .unwrap_or(&"0".to_string())
                .clone();

            facts.push(XbrlFact {
                concept,
                context_ref: context_id,
                value,
                unit: statute.effect.parameters.get("unit").cloned(),
                decimals: statute
                    .effect
                    .parameters
                    .get("decimals")
                    .and_then(|d| d.parse().ok()),
            });
        }

        let instance = XbrlInstance {
            metadata,
            contexts,
            facts,
        };

        let json = serde_json::to_string_pretty(&instance).map_err(|e| {
            InteropError::SerializationError(format!("Failed to serialize XBRL: {}", e))
        })?;

        report.statutes_converted = statutes.len();
        Ok((json, report))
    }

    fn can_represent(&self, _statute: &Statute) -> Vec<String> {
        // XBRL can represent most financial reporting requirements
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xbrl_json_import() {
        let source = r#"{
            "metadata": {
                "document_id": "test-xbrl",
                "entity_id": "COMPANY-001",
                "period": "2024",
                "schema_ref": "http://www.xbrl.org/2003/xbrl-instance-2003-12-31.xsd",
                "currency": "USD"
            },
            "contexts": [
                {
                    "id": "ctx-2024",
                    "entity": "COMPANY-001",
                    "period": {
                        "type": "Instant",
                        "date": "2024-12-31"
                    },
                    "dimensions": {}
                }
            ],
            "facts": [
                {
                    "concept": "us-gaap:Assets",
                    "context_ref": "ctx-2024",
                    "value": "1000000",
                    "unit": "USD",
                    "decimals": 0
                }
            ]
        }"#;

        let importer = XbrlImporter::new();
        let (statutes, report) = importer.import(source).unwrap();

        assert_eq!(report.statutes_converted, 1);
        assert_eq!(statutes.len(), 1);
        assert!(statutes[0].id.contains("Assets"));
    }

    #[test]
    fn test_xbrl_export() {
        let mut effect = Effect::new(EffectType::Obligation, "Report Assets");
        effect
            .parameters
            .insert("concept".to_string(), "us-gaap:Assets".to_string());
        effect
            .parameters
            .insert("value".to_string(), "5000000".to_string());
        effect
            .parameters
            .insert("unit".to_string(), "USD".to_string());

        let mut statute = Statute::new("assets", "Assets Reporting", effect);

        statute
            .effect
            .parameters
            .insert("entity".to_string(), "COMPANY-001".to_string());
        statute
            .effect
            .parameters
            .insert("period".to_string(), "2024".to_string());
        statute
            .effect
            .parameters
            .insert("currency".to_string(), "USD".to_string());
        statute
            .effect
            .parameters
            .insert("context_id".to_string(), "ctx-2024".to_string());
        statute
            .effect
            .parameters
            .insert("instant_date".to_string(), "2024-12-31".to_string());

        let exporter = XbrlExporter::new();
        let (output, report) = exporter.export(&[statute]).unwrap();

        assert_eq!(report.statutes_converted, 1);
        assert!(output.contains("us-gaap:Assets"));
        assert!(output.contains("5000000"));
        assert!(output.contains("COMPANY-001"));
    }

    #[test]
    fn test_xbrl_xml_import() {
        let source = r#"
        <xbrl xmlns="http://www.xbrl.org/2003/instance">
            <context id="ctx1">
                <entity>
                    <identifier scheme="http://www.sec.gov/CIK">0001234567</identifier>
                </entity>
            </context>
            <Assets contextRef="ctx1" unitRef="USD" decimals="0">10000000</Assets>
        </xbrl>
        "#;

        let importer = XbrlImporter::new();
        let (statutes, _report) = importer.import(source).unwrap();

        assert!(!statutes.is_empty());
    }

    #[test]
    fn test_xbrl_validate() {
        let importer = XbrlImporter::new();

        let valid_json = r#"{"metadata": {}, "facts": []}"#;
        assert!(importer.validate(valid_json));

        let valid_xml = r#"<xbrl xmlns="http://www.xbrl.org/2003/instance"></xbrl>"#;
        assert!(importer.validate(valid_xml));

        let invalid = "not xbrl";
        assert!(!importer.validate(invalid));
    }
}
