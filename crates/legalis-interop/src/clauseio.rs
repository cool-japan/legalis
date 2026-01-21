//! Clause.io template format support.
//!
//! Clause.io is a contract automation platform that uses structured templates
//! with variables, logic, and data models.
//!
//! Format characteristics:
//! - JSON-based template structure
//! - Variable definitions with types
//! - Conditional logic blocks
//! - Repeating sections
//! - Template metadata and versioning
//!
//! Reference: <https://clause.io/>

use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::{Condition, Effect, EffectType, Statute};
use serde::{Deserialize, Serialize};

/// Clause.io template structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ClauseTemplate {
    #[serde(default)]
    name: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    version: String,
    #[serde(default)]
    variables: Vec<ClauseVariable>,
    #[serde(default)]
    sections: Vec<ClauseSection>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ClauseVariable {
    name: String,
    #[serde(rename = "type")]
    var_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    default: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ClauseSection {
    #[serde(default)]
    title: String,
    #[serde(default)]
    content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    condition: Option<String>,
    #[serde(default)]
    subsections: Vec<ClauseSection>,
}

/// Clause.io format importer.
pub struct ClauseIoImporter;

impl ClauseIoImporter {
    /// Creates a new Clause.io importer.
    pub fn new() -> Self {
        Self
    }

    fn extract_conditions(&self, template: &ClauseTemplate) -> Vec<Condition> {
        let mut conditions = Vec::new();

        // Extract conditions from section conditions
        for section in &template.sections {
            if let Some(cond) = &section.condition
                && cond.contains("age")
            {
                let re = regex_lite::Regex::new(r"age\s*>=\s*(\d+)").unwrap();
                if let Some(cap) = re.captures(cond)
                    && let Some(age_str) = cap.get(1)
                    && let Ok(age_val) = age_str.as_str().parse::<u32>()
                {
                    conditions.push(Condition::Age {
                        operator: legalis_core::ComparisonOp::GreaterOrEqual,
                        value: age_val,
                    });
                }
            }
        }

        conditions
    }

    fn template_to_statute(&self, template: &ClauseTemplate) -> Statute {
        let conditions = self.extract_conditions(template);

        let title = if template.name.is_empty() {
            "Clause.io Template".to_string()
        } else {
            template.name.clone()
        };

        let id = title
            .to_lowercase()
            .replace(|c: char| !c.is_alphanumeric(), "-")
            .trim_matches('-')
            .to_string();

        // Combine all section content to determine effect type
        let all_content: String = template
            .sections
            .iter()
            .map(|s| s.content.clone())
            .collect::<Vec<_>>()
            .join(" ");

        let effect_type = if all_content.to_lowercase().contains("shall")
            || all_content.to_lowercase().contains("must")
            || all_content.to_lowercase().contains("required")
        {
            EffectType::Obligation
        } else if all_content.to_lowercase().contains("may")
            || all_content.to_lowercase().contains("permitted")
        {
            EffectType::Grant
        } else if all_content.to_lowercase().contains("prohibited")
            || all_content.to_lowercase().contains("shall not")
        {
            EffectType::Prohibition
        } else {
            EffectType::Grant
        };

        let mut effect = Effect::new(effect_type, "clause");

        // Store variables as parameters
        for var in &template.variables {
            effect
                .parameters
                .insert(format!("var_{}", var.name), var.var_type.clone());
        }

        if !template.version.is_empty() {
            effect
                .parameters
                .insert("version".to_string(), template.version.clone());
        }

        let mut statute = Statute::new(&id, &title, effect);

        // Note: Statute description field not available in current structure
        // Template description is stored but not used: template.description

        for condition in conditions {
            statute = statute.with_precondition(condition);
        }

        statute
    }
}

impl Default for ClauseIoImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for ClauseIoImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::ClauseIo
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::ClauseIo, LegalFormat::Legalis);

        let template: ClauseTemplate = serde_json::from_str(source).map_err(|e| {
            InteropError::ParseError(format!("Failed to parse Clause.io template: {}", e))
        })?;

        let statute = self.template_to_statute(&template);

        report.statutes_converted = 1;

        Ok((vec![statute], report))
    }

    fn validate(&self, source: &str) -> bool {
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(source) {
            // Check for Clause.io-specific fields
            value.get("name").is_some()
                || value.get("variables").is_some()
                || value.get("sections").is_some()
        } else {
            false
        }
    }
}

/// Clause.io format exporter.
pub struct ClauseIoExporter;

impl ClauseIoExporter {
    /// Creates a new Clause.io exporter.
    pub fn new() -> Self {
        Self
    }

    fn statute_to_template(&self, statute: &Statute) -> ClauseTemplate {
        let mut variables = Vec::new();

        // Extract variables from effect parameters
        for (key, value) in &statute.effect.parameters {
            if key.starts_with("var_") {
                let var_name = key.strip_prefix("var_").unwrap_or(key);
                variables.push(ClauseVariable {
                    name: var_name.to_string(),
                    var_type: value.clone(),
                    description: None,
                    default: None,
                });
            }
        }

        let mut sections = Vec::new();

        // Add main content section
        let effect_verb = match statute.effect.effect_type {
            EffectType::Grant => "MAY",
            EffectType::Obligation => "SHALL",
            EffectType::Prohibition => "SHALL NOT",
            EffectType::Revoke => "VOID",
            EffectType::MonetaryTransfer => "SHALL TRANSFER",
            EffectType::StatusChange => "SHALL CHANGE",
            EffectType::Custom => "SHALL APPLY",
        };

        let main_content = format!("The party {} {}.", effect_verb, statute.effect.description);

        sections.push(ClauseSection {
            title: "Main Provision".to_string(),
            content: main_content,
            condition: None,
            subsections: Vec::new(),
        });

        // Add condition sections
        for (i, condition) in statute.preconditions.iter().enumerate() {
            let (title, content, cond) = match condition {
                Condition::Age { operator, value } => {
                    let op_str = match operator {
                        legalis_core::ComparisonOp::GreaterOrEqual => ">=",
                        legalis_core::ComparisonOp::LessThan => "<",
                        legalis_core::ComparisonOp::Equal => "==",
                        legalis_core::ComparisonOp::GreaterThan => ">",
                        legalis_core::ComparisonOp::LessOrEqual => "<=",
                        legalis_core::ComparisonOp::NotEqual => "!=",
                    };
                    (
                        format!("Age Requirement {}", i + 1),
                        format!(
                            "The party must meet the age requirement of {} years.",
                            value
                        ),
                        Some(format!("age {} {}", op_str, value)),
                    )
                }
                Condition::AttributeEquals { key, value } if key == "citizenship" => (
                    format!("Citizenship Requirement {}", i + 1),
                    format!("The party must be a citizen of {}.", value),
                    Some(format!("citizenship == \"{}\"", value)),
                ),
                _ => (
                    format!("Condition {}", i + 1),
                    "Additional condition applies.".to_string(),
                    Some("custom_condition".to_string()),
                ),
            };

            sections.push(ClauseSection {
                title,
                content,
                condition: cond,
                subsections: Vec::new(),
            });
        }

        let version = statute
            .effect
            .parameters
            .get("version")
            .cloned()
            .unwrap_or_else(|| "1.0.0".to_string());

        ClauseTemplate {
            name: statute.title.clone(),
            description: String::new(), // Statute description not available in current structure
            version,
            variables,
            sections,
            metadata: None,
        }
    }
}

impl Default for ClauseIoExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for ClauseIoExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::ClauseIo
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::ClauseIo);

        if statutes.is_empty() {
            return Err(InteropError::ConversionError(
                "No statutes to export".to_string(),
            ));
        }

        // For multiple statutes, export as array
        if statutes.len() == 1 {
            let template = self.statute_to_template(&statutes[0]);
            let output = serde_json::to_string_pretty(&template)
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;
            report.statutes_converted = 1;
            return Ok((output, report));
        }

        // Multiple statutes as array
        let templates: Vec<ClauseTemplate> = statutes
            .iter()
            .map(|s| self.statute_to_template(s))
            .collect();

        let output = serde_json::to_string_pretty(&templates)
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;

        report.statutes_converted = statutes.len();
        Ok((output, report))
    }

    fn can_represent(&self, _statute: &Statute) -> Vec<String> {
        // Clause.io can represent most features
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clauseio_import_simple() {
        let importer = ClauseIoImporter::new();

        let source = r#"{
  "name": "Payment Clause",
  "description": "Standard payment terms",
  "version": "1.0.0",
  "variables": [
    {
      "name": "amount",
      "type": "Number"
    }
  ],
  "sections": [
    {
      "title": "Payment",
      "content": "The buyer shall pay the amount."
    }
  ]
}"#;

        let (statutes, report) = importer.import(source).unwrap();
        assert_eq!(report.statutes_converted, 1);
        assert_eq!(statutes.len(), 1);
        assert_eq!(statutes[0].title, "Payment Clause");
    }

    #[test]
    fn test_clauseio_export_simple() {
        let exporter = ClauseIoExporter::new();

        let mut effect = Effect::new(EffectType::Obligation, "pay amount");
        effect
            .parameters
            .insert("var_amount".to_string(), "Number".to_string());

        let statute = Statute::new("payment", "Payment Clause", effect);

        let (output, report) = exporter.export(&[statute]).unwrap();
        assert_eq!(report.statutes_converted, 1);
        assert!(output.contains("Payment Clause"));
        assert!(output.contains("variables"));
        assert!(output.contains("amount"));
    }

    #[test]
    fn test_clauseio_validate() {
        let importer = ClauseIoImporter::new();

        assert!(importer.validate(r#"{"name": "Test"}"#));
        assert!(importer.validate(r#"{"variables": []}"#));
        assert!(importer.validate(r#"{"sections": []}"#));
        assert!(!importer.validate("not json"));
    }

    #[test]
    fn test_clauseio_roundtrip() {
        let importer = ClauseIoImporter::new();
        let exporter = ClauseIoExporter::new();

        let mut effect = Effect::new(EffectType::Grant, "execute");
        effect
            .parameters
            .insert("var_party".to_string(), "String".to_string());

        let statute = Statute::new("agreement", "Service Agreement", effect);

        let (exported, _) = exporter.export(&[statute]).unwrap();
        let (imported, _) = importer.import(&exported).unwrap();

        assert_eq!(imported.len(), 1);
        assert_eq!(imported[0].title, "Service Agreement");
    }

    #[test]
    fn test_clauseio_with_conditions() {
        let importer = ClauseIoImporter::new();

        let source = r#"{
  "name": "Adult Agreement",
  "sections": [
    {
      "title": "Main",
      "content": "Agreement text",
      "condition": "age >= 18"
    }
  ]
}"#;

        let (statutes, report) = importer.import(source).unwrap();
        assert_eq!(report.statutes_converted, 1);
        assert_eq!(statutes[0].preconditions.len(), 1);
    }
}
