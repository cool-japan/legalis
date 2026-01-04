//! SAP Legal Module integration format support.
//!
//! SAP Legal Module is an enterprise legal management system that handles:
//! - Legal document management
//! - Contract lifecycle management
//! - Compliance tracking
//! - Legal obligations and deadlines
//! - Clause libraries and templates
//!
//! This module provides bidirectional conversion between SAP Legal Module
//! format and legalis_core::Statute format.

use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// SAP Legal Module document structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SapLegalDocument {
    /// Document metadata
    pub metadata: SapLegalMetadata,
    /// Legal obligations
    pub obligations: Vec<SapLegalObligation>,
    /// Contract clauses
    pub clauses: Vec<SapLegalClause>,
}

/// SAP Legal Module metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SapLegalMetadata {
    /// Document ID (SAP object ID)
    pub document_id: String,
    /// Document title
    pub title: String,
    /// Document type (e.g., "contract", "agreement", "policy")
    pub document_type: String,
    /// Legal entity
    pub legal_entity: String,
    /// Jurisdiction
    pub jurisdiction: String,
    /// Creation date (ISO 8601)
    pub created_date: Option<String>,
    /// Last modified date (ISO 8601)
    pub modified_date: Option<String>,
    /// Status (e.g., "draft", "active", "expired", "terminated")
    pub status: String,
    /// Owner/responsible person
    pub owner: Option<String>,
}

/// SAP Legal obligation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SapLegalObligation {
    /// Obligation ID
    pub obligation_id: String,
    /// Obligation title
    pub title: String,
    /// Obligation type (e.g., "reporting", "payment", "delivery", "notification")
    pub obligation_type: String,
    /// Obligation description
    pub description: String,
    /// Trigger conditions
    pub trigger_conditions: Vec<SapLegalCondition>,
    /// Responsible party
    pub responsible_party: String,
    /// Counterparty (if applicable)
    pub counterparty: Option<String>,
    /// Due date calculation rule
    pub due_date_rule: Option<String>,
    /// Deadline in days (from trigger)
    pub deadline_days: Option<i32>,
    /// Penalty for non-compliance
    pub penalty: Option<String>,
    /// Custom attributes
    pub custom_attributes: HashMap<String, String>,
}

/// SAP Legal clause
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SapLegalClause {
    /// Clause ID
    pub clause_id: String,
    /// Clause type (e.g., "indemnity", "liability", "termination", "warranty")
    pub clause_type: String,
    /// Clause title
    pub title: String,
    /// Clause text
    pub text: String,
    /// Applicability conditions
    pub conditions: Vec<SapLegalCondition>,
    /// Standard/template clause reference
    pub template_ref: Option<String>,
    /// Risk level (e.g., "low", "medium", "high")
    pub risk_level: Option<String>,
}

/// SAP Legal condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SapLegalCondition {
    /// Condition description
    pub description: String,
    /// Field name being evaluated
    pub field: String,
    /// Operator (e.g., "equals", "greater_than", "less_than", "contains")
    pub operator: String,
    /// Value to compare
    pub value: String,
    /// Data type (e.g., "string", "number", "date", "boolean")
    pub data_type: String,
}

/// SAP Legal importer
pub struct SapLegalImporter;

impl SapLegalImporter {
    /// Creates a new SAP Legal importer
    pub fn new() -> Self {
        Self
    }

    fn parse_sap_legal(&self, source: &str) -> InteropResult<SapLegalDocument> {
        serde_json::from_str(source)
            .map_err(|e| InteropError::ParseError(format!("Failed to parse SAP Legal JSON: {}", e)))
    }

    fn convert_condition(&self, condition: &SapLegalCondition) -> Condition {
        match condition.data_type.as_str() {
            "number" | "integer" => {
                if let Ok(value) = condition.value.parse::<i64>() {
                    let op = match condition.operator.as_str() {
                        "greater_than" | "gt" | ">" => ComparisonOp::GreaterThan,
                        "greater_or_equal" | "gte" | ">=" => ComparisonOp::GreaterOrEqual,
                        "less_than" | "lt" | "<" => ComparisonOp::LessThan,
                        "less_or_equal" | "lte" | "<=" => ComparisonOp::LessOrEqual,
                        "equals" | "eq" | "==" | "=" => ComparisonOp::Equal,
                        _ => ComparisonOp::GreaterOrEqual,
                    };

                    // Check if it's an age field
                    if condition.field.to_lowercase().contains("age") {
                        return Condition::Age {
                            operator: op,
                            value: value as u32,
                        };
                    }

                    Condition::Custom {
                        description: format!(
                            "{} {} {}",
                            condition.field, condition.operator, value
                        ),
                    }
                } else {
                    Condition::Custom {
                        description: condition.description.clone(),
                    }
                }
            }
            "boolean" => {
                let value = condition.value.to_lowercase() == "true";
                Condition::Custom {
                    description: format!("{} is {}", condition.field, value),
                }
            }
            "date" => Condition::Custom {
                description: format!(
                    "{} {} {}",
                    condition.field, condition.operator, condition.value
                ),
            },
            _ => Condition::Custom {
                description: condition.description.clone(),
            },
        }
    }

    fn convert_obligation_to_statute(&self, obligation: &SapLegalObligation) -> Statute {
        // Determine effect type based on obligation type
        let effect_type = match obligation.obligation_type.as_str() {
            "reporting" | "notification" => EffectType::Grant,
            "payment" => EffectType::Obligation,
            "delivery" => EffectType::Grant,
            "prohibition" => EffectType::Prohibition,
            _ => EffectType::Grant,
        };

        let mut effect = Effect::new(effect_type, &obligation.title);
        effect
            .parameters
            .insert("description".to_string(), obligation.description.clone());
        effect.parameters.insert(
            "responsible_party".to_string(),
            obligation.responsible_party.clone(),
        );

        if let Some(counterparty) = &obligation.counterparty {
            effect
                .parameters
                .insert("counterparty".to_string(), counterparty.clone());
        }

        if let Some(deadline_days) = obligation.deadline_days {
            effect
                .parameters
                .insert("deadline_days".to_string(), deadline_days.to_string());
        }

        if let Some(penalty) = &obligation.penalty {
            effect
                .parameters
                .insert("penalty".to_string(), penalty.clone());
        }

        // Add custom attributes to effect parameters
        for (key, value) in &obligation.custom_attributes {
            effect.parameters.insert(key.clone(), value.clone());
        }

        let mut statute = Statute::new(&obligation.obligation_id, &obligation.title, effect);

        // Add trigger conditions as preconditions
        for condition in &obligation.trigger_conditions {
            statute = statute.with_precondition(self.convert_condition(condition));
        }

        statute
    }

    fn convert_clause_to_statute(&self, clause: &SapLegalClause) -> Statute {
        // Determine effect type based on clause type
        let effect_type = match clause.clause_type.as_str() {
            "indemnity" | "warranty" => EffectType::Grant,
            "liability" | "limitation" => EffectType::Prohibition,
            "termination" => EffectType::Revoke,
            _ => EffectType::Grant,
        };

        let mut effect = Effect::new(effect_type, &clause.title);
        effect
            .parameters
            .insert("clause_text".to_string(), clause.text.clone());
        effect
            .parameters
            .insert("clause_type".to_string(), clause.clause_type.clone());

        if let Some(template_ref) = &clause.template_ref {
            effect
                .parameters
                .insert("template_ref".to_string(), template_ref.clone());
        }

        if let Some(risk_level) = &clause.risk_level {
            effect
                .parameters
                .insert("risk_level".to_string(), risk_level.clone());
        }

        let mut statute = Statute::new(&clause.clause_id, &clause.title, effect);

        // Add applicability conditions as preconditions
        for condition in &clause.conditions {
            statute = statute.with_precondition(self.convert_condition(condition));
        }

        statute
    }
}

impl Default for SapLegalImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for SapLegalImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::SapLegal
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let doc = self.parse_sap_legal(source)?;
        let mut report = ConversionReport::new(LegalFormat::SapLegal, LegalFormat::Legalis);
        let mut statutes = Vec::new();

        // Convert obligations to statutes
        for obligation in &doc.obligations {
            statutes.push(self.convert_obligation_to_statute(obligation));
        }

        // Convert clauses to statutes
        for clause in &doc.clauses {
            statutes.push(self.convert_clause_to_statute(clause));
        }

        report.statutes_converted = statutes.len();

        if !doc.obligations.is_empty() {
            report.add_warning(format!(
                "Converted {} SAP Legal obligations to statutes",
                doc.obligations.len()
            ));
        }

        if !doc.clauses.is_empty() {
            report.add_warning(format!(
                "Converted {} SAP Legal clauses to statutes",
                doc.clauses.len()
            ));
        }

        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        // Try to parse as JSON and check for SAP Legal specific fields
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(source) {
            if let Some(obj) = value.as_object() {
                return obj.contains_key("metadata")
                    && obj.contains_key("obligations")
                    && obj.contains_key("clauses")
                    && obj
                        .get("metadata")
                        .and_then(|m| m.get("document_id"))
                        .is_some();
            }
        }
        false
    }
}

/// SAP Legal exporter
pub struct SapLegalExporter;

impl SapLegalExporter {
    /// Creates a new SAP Legal exporter
    pub fn new() -> Self {
        Self
    }

    fn convert_condition_from_statute(&self, condition: &Condition) -> SapLegalCondition {
        match condition {
            Condition::Age { operator, value } => SapLegalCondition {
                description: format!(
                    "Age must be {} {}",
                    Self::operator_to_string(operator),
                    value
                ),
                field: "age".to_string(),
                operator: Self::operator_to_sap_operator(operator),
                value: value.to_string(),
                data_type: "number".to_string(),
            },
            Condition::Custom { description } => SapLegalCondition {
                description: description.clone(),
                field: "custom".to_string(),
                operator: "equals".to_string(),
                value: "true".to_string(),
                data_type: "boolean".to_string(),
            },
            _ => SapLegalCondition {
                description: format!("{:?}", condition),
                field: "custom".to_string(),
                operator: "equals".to_string(),
                value: "true".to_string(),
                data_type: "boolean".to_string(),
            },
        }
    }

    fn operator_to_string(op: &ComparisonOp) -> &'static str {
        match op {
            ComparisonOp::GreaterThan => "greater than",
            ComparisonOp::GreaterOrEqual => "greater than or equal to",
            ComparisonOp::LessThan => "less than",
            ComparisonOp::LessOrEqual => "less than or equal to",
            ComparisonOp::Equal => "equal to",
            ComparisonOp::NotEqual => "not equal to",
        }
    }

    fn operator_to_sap_operator(op: &ComparisonOp) -> String {
        match op {
            ComparisonOp::GreaterThan => "greater_than".to_string(),
            ComparisonOp::GreaterOrEqual => "greater_or_equal".to_string(),
            ComparisonOp::LessThan => "less_than".to_string(),
            ComparisonOp::LessOrEqual => "less_or_equal".to_string(),
            ComparisonOp::Equal => "equals".to_string(),
            ComparisonOp::NotEqual => "not_equals".to_string(),
        }
    }

    fn convert_statute_to_obligation(&self, statute: &Statute) -> SapLegalObligation {
        let obligation_type = match statute.effect.effect_type {
            EffectType::Grant => "delivery",
            EffectType::Obligation => "payment",
            EffectType::Prohibition => "prohibition",
            EffectType::Revoke => "termination",
            _ => "reporting",
        };

        let trigger_conditions: Vec<SapLegalCondition> = statute
            .preconditions
            .iter()
            .map(|c| self.convert_condition_from_statute(c))
            .collect();

        let description = statute
            .effect
            .parameters
            .get("description")
            .cloned()
            .unwrap_or_else(|| statute.title.clone());

        let responsible_party = statute
            .effect
            .parameters
            .get("responsible_party")
            .cloned()
            .unwrap_or_else(|| "DefaultParty".to_string());

        let counterparty = statute.effect.parameters.get("counterparty").cloned();

        let deadline_days = statute
            .effect
            .parameters
            .get("deadline_days")
            .and_then(|s| s.parse::<i32>().ok());

        let penalty = statute.effect.parameters.get("penalty").cloned();

        let mut custom_attributes = HashMap::new();
        for (key, value) in &statute.effect.parameters {
            if !matches!(
                key.as_str(),
                "description" | "responsible_party" | "counterparty" | "deadline_days" | "penalty"
            ) {
                custom_attributes.insert(key.clone(), value.clone());
            }
        }

        SapLegalObligation {
            obligation_id: statute.id.clone(),
            title: statute.title.clone(),
            obligation_type: obligation_type.to_string(),
            description,
            trigger_conditions,
            responsible_party,
            counterparty,
            due_date_rule: None,
            deadline_days,
            penalty,
            custom_attributes,
        }
    }
}

impl Default for SapLegalExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for SapLegalExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::SapLegal
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::SapLegal);

        let metadata = SapLegalMetadata {
            document_id: "SAP-DOC-001".to_string(),
            title: "Exported Legal Document".to_string(),
            document_type: "contract".to_string(),
            legal_entity: "Default Entity".to_string(),
            jurisdiction: statutes
                .first()
                .and_then(|s| s.jurisdiction.clone())
                .unwrap_or_else(|| "US".to_string()),
            created_date: Some(chrono::Utc::now().to_rfc3339()),
            modified_date: Some(chrono::Utc::now().to_rfc3339()),
            status: "active".to_string(),
            owner: None,
        };

        let obligations: Vec<SapLegalObligation> = statutes
            .iter()
            .map(|s| self.convert_statute_to_obligation(s))
            .collect();

        let doc = SapLegalDocument {
            metadata,
            obligations,
            clauses: Vec::new(), // Could be extended to separate obligations and clauses
        };

        let json = serde_json::to_string_pretty(&doc).map_err(|e| {
            InteropError::SerializationError(format!("JSON serialization failed: {}", e))
        })?;

        report.statutes_converted = statutes.len();

        if statutes.len() > 10 {
            report.add_warning(format!(
                "Exported {} statutes as SAP Legal obligations",
                statutes.len()
            ));
        }

        Ok((json, report))
    }

    fn can_represent(&self, _statute: &Statute) -> Vec<String> {
        // SAP Legal can represent most legal concepts
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

    #[test]
    fn test_sap_legal_validate() {
        let importer = SapLegalImporter::new();

        let valid_json = r#"{
            "metadata": {
                "document_id": "SAP-001",
                "title": "Test Contract",
                "document_type": "contract",
                "legal_entity": "Test Corp",
                "jurisdiction": "US",
                "status": "active"
            },
            "obligations": [],
            "clauses": []
        }"#;

        assert!(importer.validate(valid_json));

        let invalid_json = r#"{"random": "data"}"#;
        assert!(!importer.validate(invalid_json));
    }

    #[test]
    fn test_sap_legal_import() {
        let importer = SapLegalImporter::new();

        let json = r#"{
            "metadata": {
                "document_id": "SAP-001",
                "title": "Employment Contract",
                "document_type": "contract",
                "legal_entity": "Test Corp",
                "jurisdiction": "US",
                "status": "active"
            },
            "obligations": [
                {
                    "obligation_id": "OBL-001",
                    "title": "Payment Obligation",
                    "obligation_type": "payment",
                    "description": "Monthly salary payment",
                    "trigger_conditions": [
                        {
                            "description": "Age must be at least 18",
                            "field": "employee_age",
                            "operator": "greater_or_equal",
                            "value": "18",
                            "data_type": "number"
                        }
                    ],
                    "responsible_party": "Employer",
                    "counterparty": "Employee",
                    "due_date_rule": "End of month",
                    "deadline_days": 30,
                    "penalty": "Interest on late payment",
                    "custom_attributes": {}
                }
            ],
            "clauses": []
        }"#;

        let (statutes, report) = importer.import(json).unwrap();
        assert_eq!(statutes.len(), 1);
        assert_eq!(report.statutes_converted, 1);
        assert_eq!(statutes[0].id, "OBL-001");
        assert_eq!(statutes[0].title, "Payment Obligation");
    }

    #[test]
    fn test_sap_legal_export() {
        let exporter = SapLegalExporter::new();

        let statute = Statute::new(
            "test-obligation",
            "Test Obligation",
            Effect::new(EffectType::Grant, "Test Action"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 21,
        });

        let (output, report) = exporter.export(&[statute]).unwrap();
        assert_eq!(report.statutes_converted, 1);
        assert!(output.contains("SAP-DOC-001"));
        assert!(output.contains("Test Obligation"));
    }

    #[test]
    fn test_sap_legal_roundtrip() {
        let importer = SapLegalImporter::new();
        let exporter = SapLegalExporter::new();

        let original_json = r#"{
            "metadata": {
                "document_id": "SAP-001",
                "title": "Test Contract",
                "document_type": "contract",
                "legal_entity": "Test Corp",
                "jurisdiction": "US",
                "status": "active"
            },
            "obligations": [
                {
                    "obligation_id": "OBL-001",
                    "title": "Reporting Obligation",
                    "obligation_type": "reporting",
                    "description": "Quarterly reports",
                    "trigger_conditions": [],
                    "responsible_party": "Company",
                    "counterparty": null,
                    "due_date_rule": null,
                    "deadline_days": 90,
                    "penalty": null,
                    "custom_attributes": {}
                }
            ],
            "clauses": []
        }"#;

        let (statutes, _) = importer.import(original_json).unwrap();
        let (output, _) = exporter.export(&statutes).unwrap();

        // Verify the exported JSON is valid
        let doc: SapLegalDocument = serde_json::from_str(&output).unwrap();
        assert_eq!(doc.obligations.len(), 1);
        assert_eq!(doc.obligations[0].title, "Reporting Obligation");
    }
}
