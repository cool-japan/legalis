//! FinReg (Financial Regulatory) format support.
//!
//! FinReg is a format for representing financial regulatory requirements,
//! compliance rules, and reporting obligations. It supports:
//! - Regulatory compliance rules
//! - Financial reporting requirements
//! - Risk assessment rules
//! - Capital adequacy requirements
//! - Liquidity coverage requirements

use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// FinReg document structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinRegDocument {
    /// Document metadata
    pub metadata: FinRegMetadata,
    /// Regulatory requirements
    pub requirements: Vec<FinRegRequirement>,
}

/// FinReg metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinRegMetadata {
    /// Document ID
    pub id: String,
    /// Title
    pub title: String,
    /// Regulatory framework (e.g., "Basel III", "MiFID II", "Dodd-Frank")
    pub framework: String,
    /// Jurisdiction
    pub jurisdiction: String,
    /// Version
    pub version: String,
    /// Effective date
    pub effective_date: Option<String>,
}

/// FinReg requirement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinRegRequirement {
    /// Requirement ID
    pub id: String,
    /// Title/description
    pub title: String,
    /// Requirement type (e.g., "capital", "liquidity", "reporting", "risk")
    pub requirement_type: String,
    /// Conditions that trigger this requirement
    pub conditions: Vec<FinRegCondition>,
    /// Required action or obligation
    pub obligation: String,
    /// Threshold values (e.g., minimum capital ratio)
    pub thresholds: HashMap<String, f64>,
    /// Formula for calculation (if applicable)
    pub formula: Option<String>,
    /// Penalties for non-compliance
    pub penalties: Vec<String>,
}

/// FinReg condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinRegCondition {
    /// Condition description
    pub description: String,
    /// Parameter being checked
    pub parameter: String,
    /// Operator (e.g., ">=", "<=", "==")
    pub operator: String,
    /// Value to compare against
    pub value: String,
}

/// FinReg importer
pub struct FinRegImporter;

impl FinRegImporter {
    /// Creates a new FinReg importer
    pub fn new() -> Self {
        Self
    }

    fn parse_finreg(&self, source: &str) -> InteropResult<FinRegDocument> {
        serde_json::from_str(source)
            .map_err(|e| InteropError::ParseError(format!("Failed to parse FinReg JSON: {}", e)))
    }

    fn convert_condition(&self, condition: &FinRegCondition) -> Condition {
        // Try to parse as numeric comparison
        if let Ok(value) = condition.value.parse::<i64>() {
            let op = match condition.operator.as_str() {
                ">=" => ComparisonOp::GreaterOrEqual,
                ">" => ComparisonOp::GreaterThan,
                "<=" => ComparisonOp::LessOrEqual,
                "<" => ComparisonOp::LessThan,
                "==" | "=" => ComparisonOp::Equal,
                "!=" => ComparisonOp::NotEqual,
                _ => ComparisonOp::Equal,
            };

            match condition.parameter.to_lowercase().as_str() {
                "age" => Condition::Age {
                    operator: op,
                    value: value as u32,
                },
                "income" => Condition::Income {
                    operator: op,
                    value: value as u64,
                },
                "duration" => Condition::Duration {
                    operator: op,
                    value: value as u32,
                    unit: legalis_core::DurationUnit::Years,
                },
                _ => Condition::Custom {
                    description: format!(
                        "{} {} {}",
                        condition.parameter,
                        FinRegExporter::comparison_op_to_str(&op),
                        condition.value
                    ),
                },
            }
        } else {
            // String-based condition
            Condition::Custom {
                description: format!("{} == {}", condition.parameter, condition.value),
            }
        }
    }
}

impl Default for FinRegImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for FinRegImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::FinReg
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let doc = self.parse_finreg(source)?;
        let mut statutes = Vec::new();
        let mut report = ConversionReport::new(LegalFormat::FinReg, LegalFormat::Legalis);

        for req in &doc.requirements {
            let effect_type = match req.requirement_type.to_lowercase().as_str() {
                "reporting" => EffectType::Obligation,
                "prohibition" | "restriction" => EffectType::Prohibition,
                _ => EffectType::Grant,
            };

            let mut effect = Effect::new(effect_type, &req.obligation);

            // Add thresholds as parameters
            for (key, value) in &req.thresholds {
                effect.parameters.insert(key.clone(), value.to_string());
            }

            if let Some(formula) = &req.formula {
                effect
                    .parameters
                    .insert("formula".to_string(), formula.clone());
            }

            let mut statute = Statute::new(&req.id, &req.title, effect);

            // Add conditions
            for condition in &req.conditions {
                statute = statute.with_precondition(self.convert_condition(condition));
            }

            // Add metadata to parameters
            statute
                .effect
                .parameters
                .insert("framework".to_string(), doc.metadata.framework.clone());
            statute
                .effect
                .parameters
                .insert("requirement_type".to_string(), req.requirement_type.clone());
            statute
                .effect
                .parameters
                .insert("version".to_string(), doc.metadata.version.clone());

            if let Some(effective_date) = &doc.metadata.effective_date {
                statute
                    .effect
                    .parameters
                    .insert("effective_date".to_string(), effective_date.clone());
            }

            statute = statute.with_jurisdiction(&doc.metadata.jurisdiction);

            statutes.push(statute);
        }

        report.statutes_converted = statutes.len();
        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(source) {
            value.get("metadata").is_some()
                && value.get("requirements").is_some()
                && value
                    .get("metadata")
                    .and_then(|m| m.get("framework"))
                    .is_some()
        } else {
            false
        }
    }
}

/// FinReg exporter
pub struct FinRegExporter;

impl FinRegExporter {
    /// Creates a new FinReg exporter
    pub fn new() -> Self {
        Self
    }

    fn statute_to_requirement(&self, statute: &Statute) -> FinRegRequirement {
        let mut conditions = Vec::new();

        for precondition in &statute.preconditions {
            let (parameter, operator, value) = match precondition {
                Condition::Age { operator, value } => (
                    "age".to_string(),
                    Self::comparison_op_to_str(operator),
                    value.to_string(),
                ),
                Condition::Income { operator, value } => (
                    "income".to_string(),
                    Self::comparison_op_to_str(operator),
                    value.to_string(),
                ),
                Condition::Duration {
                    operator, value, ..
                } => (
                    "duration".to_string(),
                    Self::comparison_op_to_str(operator),
                    value.to_string(),
                ),
                Condition::Custom { description } => {
                    // Parse description like "param op value"
                    let parts: Vec<&str> = description.split_whitespace().collect();
                    if parts.len() >= 3 {
                        (
                            parts[0].to_string(),
                            parts[1].to_string(),
                            parts[2..].join(" "),
                        )
                    } else {
                        (
                            "condition".to_string(),
                            "==".to_string(),
                            description.clone(),
                        )
                    }
                }
                _ => (
                    "condition".to_string(),
                    "==".to_string(),
                    "true".to_string(),
                ),
            };

            conditions.push(FinRegCondition {
                description: format!("{} {} {}", parameter, operator, value),
                parameter,
                operator,
                value,
            });
        }

        let requirement_type = statute
            .effect
            .parameters
            .get("requirement_type")
            .map(|s| s.to_string())
            .unwrap_or_else(|| match statute.effect.effect_type {
                EffectType::Obligation => "reporting".to_string(),
                EffectType::Prohibition => "prohibition".to_string(),
                _ => "compliance".to_string(),
            });

        let mut thresholds = HashMap::new();
        for (key, value) in &statute.effect.parameters {
            if key != "formula"
                && key != "framework"
                && key != "requirement_type"
                && key != "version"
                && key != "effective_date"
                && let Ok(num) = value.parse::<f64>()
            {
                thresholds.insert(key.clone(), num);
            }
        }

        let formula = statute.effect.parameters.get("formula").cloned();

        FinRegRequirement {
            id: statute.id.clone(),
            title: statute.title.clone(),
            requirement_type,
            conditions,
            obligation: statute.effect.description.clone(),
            thresholds,
            formula,
            penalties: Vec::new(), // Not directly mapped from Statute
        }
    }

    fn comparison_op_to_str(op: &ComparisonOp) -> String {
        match op {
            ComparisonOp::Equal => "==",
            ComparisonOp::NotEqual => "!=",
            ComparisonOp::GreaterThan => ">",
            ComparisonOp::GreaterOrEqual => ">=",
            ComparisonOp::LessThan => "<",
            ComparisonOp::LessOrEqual => "<=",
        }
        .to_string()
    }
}

impl Default for FinRegExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for FinRegExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::FinReg
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::FinReg);

        let first_statute = statutes.first();
        let metadata = FinRegMetadata {
            id: first_statute
                .and_then(|s| s.effect.parameters.get("document_id"))
                .unwrap_or(&"finreg-doc".to_string())
                .clone(),
            title: first_statute
                .map(|s| s.title.clone())
                .unwrap_or_else(|| "Financial Regulatory Requirements".to_string()),
            framework: first_statute
                .and_then(|s| s.effect.parameters.get("framework"))
                .unwrap_or(&"General".to_string())
                .clone(),
            jurisdiction: first_statute
                .and_then(|s| s.jurisdiction.as_ref())
                .unwrap_or(&"Global".to_string())
                .clone(),
            version: first_statute
                .and_then(|s| s.effect.parameters.get("version"))
                .unwrap_or(&"1.0".to_string())
                .clone(),
            effective_date: first_statute
                .and_then(|s| s.effect.parameters.get("effective_date"))
                .cloned(),
        };

        let requirements: Vec<FinRegRequirement> = statutes
            .iter()
            .map(|statute| self.statute_to_requirement(statute))
            .collect();

        let doc = FinRegDocument {
            metadata,
            requirements,
        };

        let json = serde_json::to_string_pretty(&doc).map_err(|e| {
            InteropError::SerializationError(format!("Failed to serialize FinReg: {}", e))
        })?;

        report.statutes_converted = statutes.len();
        Ok((json, report))
    }

    fn can_represent(&self, _statute: &Statute) -> Vec<String> {
        // FinReg can represent most regulatory requirements
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finreg_import() {
        let source = r#"{
            "metadata": {
                "id": "basel-iii-001",
                "title": "Basel III Capital Requirements",
                "framework": "Basel III",
                "jurisdiction": "Global",
                "version": "1.0",
                "effective_date": "2013-01-01"
            },
            "requirements": [
                {
                    "id": "cet1-ratio",
                    "title": "Common Equity Tier 1 Capital Ratio",
                    "requirement_type": "capital",
                    "conditions": [
                        {
                            "description": "Risk-weighted assets >= 0",
                            "parameter": "risk_weighted_assets",
                            "operator": ">=",
                            "value": "0"
                        }
                    ],
                    "obligation": "Maintain minimum CET1 ratio",
                    "thresholds": {
                        "minimum_ratio": 4.5
                    },
                    "formula": "CET1 / Risk-Weighted Assets >= 4.5%",
                    "penalties": ["Supervisory action", "Capital conservation measures"]
                }
            ]
        }"#;

        let importer = FinRegImporter::new();
        let (statutes, report) = importer.import(source).unwrap();

        assert_eq!(report.statutes_converted, 1);
        assert_eq!(statutes.len(), 1);
        assert_eq!(statutes[0].id, "cet1-ratio");
        assert_eq!(statutes[0].title, "Common Equity Tier 1 Capital Ratio");
        assert!(
            statutes[0]
                .jurisdiction
                .as_ref()
                .unwrap()
                .contains("Global")
        );
    }

    #[test]
    fn test_finreg_export() {
        let mut effect = Effect::new(EffectType::Obligation, "Maintain minimum capital ratio");
        effect
            .parameters
            .insert("minimum_ratio".to_string(), "8.0".to_string());
        effect
            .parameters
            .insert("formula".to_string(), "Capital / RWA >= 8%".to_string());

        let mut statute = Statute::new("capital-req", "Capital Requirement", effect)
            .with_jurisdiction("EU")
            .with_precondition(Condition::Income {
                operator: ComparisonOp::GreaterOrEqual,
                value: 1000000,
            });

        statute
            .effect
            .parameters
            .insert("framework".to_string(), "Basel III".to_string());
        statute
            .effect
            .parameters
            .insert("requirement_type".to_string(), "capital".to_string());
        statute
            .effect
            .parameters
            .insert("version".to_string(), "1.0".to_string());

        let exporter = FinRegExporter::new();
        let (output, report) = exporter.export(&[statute]).unwrap();

        assert_eq!(report.statutes_converted, 1);
        assert!(output.contains("Basel III"));
        assert!(output.contains("capital"));
        assert!(output.contains("Capital Requirement"));
    }

    #[test]
    fn test_finreg_roundtrip() {
        let source = r#"{
            "metadata": {
                "id": "test-reg",
                "title": "Test Regulation",
                "framework": "Test Framework",
                "jurisdiction": "US",
                "version": "1.0"
            },
            "requirements": [
                {
                    "id": "test-req",
                    "title": "Test Requirement",
                    "requirement_type": "reporting",
                    "conditions": [],
                    "obligation": "Submit quarterly report",
                    "thresholds": {},
                    "formula": null,
                    "penalties": []
                }
            ]
        }"#;

        let importer = FinRegImporter::new();
        let exporter = FinRegExporter::new();

        let (statutes, _) = importer.import(source).unwrap();
        let (output, _) = exporter.export(&statutes).unwrap();

        let (roundtrip_statutes, _) = importer.import(&output).unwrap();

        assert_eq!(statutes.len(), roundtrip_statutes.len());
        assert_eq!(statutes[0].id, roundtrip_statutes[0].id);
    }

    #[test]
    fn test_finreg_validate() {
        let importer = FinRegImporter::new();

        let valid = r#"{
            "metadata": {"framework": "Test"},
            "requirements": []
        }"#;
        assert!(importer.validate(valid));

        let invalid = r#"{"random": "data"}"#;
        assert!(!importer.validate(invalid));
    }
}
