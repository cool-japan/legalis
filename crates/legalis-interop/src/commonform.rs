//! CommonForm format support.
//!
//! CommonForm is a format for legal forms and contracts using structured JSON.
//! It focuses on reusable components, definitions, and structured document composition.
//!
//! Format characteristics:
//! - JSON-based structure
//! - Nested content arrays
//! - Definitions and references
//! - Form composition from reusable components
//!
//! Reference: https://commonform.org/

use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::{Condition, Effect, EffectType, Statute};
use serde::{Deserialize, Serialize};

/// CommonForm JSON structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum CommonFormContent {
    Text(String),
    Definition {
        definition: String,
    },
    Use {
        #[serde(rename = "use")]
        use_term: String,
    },
    Blank {
        blank: String,
    },
    Reference {
        reference: String,
    },
    Form {
        form: CommonForm,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CommonForm {
    #[serde(default)]
    content: Vec<CommonFormContent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    conspicuous: Option<String>,
}

/// CommonForm format importer.
pub struct CommonFormImporter;

impl CommonFormImporter {
    /// Creates a new CommonForm importer.
    pub fn new() -> Self {
        Self
    }

    #[allow(clippy::only_used_in_recursion)]
    fn extract_text_from_content(&self, content: &[CommonFormContent]) -> String {
        let mut text = String::new();

        for item in content {
            match item {
                CommonFormContent::Text(s) => {
                    text.push_str(s);
                    text.push(' ');
                }
                CommonFormContent::Definition { definition } => {
                    text.push_str(definition);
                    text.push(' ');
                }
                CommonFormContent::Use { use_term } => {
                    text.push_str(use_term);
                    text.push(' ');
                }
                CommonFormContent::Blank { blank } => {
                    text.push_str(&format!("[{}]", blank));
                    text.push(' ');
                }
                CommonFormContent::Reference { reference } => {
                    text.push_str(&format!("({})", reference));
                    text.push(' ');
                }
                CommonFormContent::Form { form } => {
                    text.push_str(&self.extract_text_from_content(&form.content));
                    text.push(' ');
                }
            }
        }

        text.trim().to_string()
    }

    fn extract_conditions(&self, text: &str) -> Vec<Condition> {
        let mut conditions = Vec::new();

        // Look for age conditions
        if text.contains("age") || text.contains("years") {
            let re = regex_lite::Regex::new(
                r"(?:age|years)\s*(?:of\s*)?(?:at least|minimum|>=)\s*(\d+)",
            )
            .unwrap();
            if let Some(cap) = re.captures(text) {
                if let Some(age_str) = cap.get(1) {
                    if let Ok(age_val) = age_str.as_str().parse::<u32>() {
                        conditions.push(Condition::Age {
                            operator: legalis_core::ComparisonOp::GreaterOrEqual,
                            value: age_val,
                        });
                    }
                }
            }
        }

        conditions
    }

    fn form_to_statute(&self, form: &CommonForm, index: usize) -> Statute {
        let text = self.extract_text_from_content(&form.content);
        let conditions = self.extract_conditions(&text);

        // Generate title from first few words
        let title = text
            .split_whitespace()
            .take(5)
            .collect::<Vec<_>>()
            .join(" ");

        let title = if title.is_empty() {
            format!("CommonForm Clause {}", index + 1)
        } else {
            title
        };

        let id = title
            .to_lowercase()
            .replace(|c: char| !c.is_alphanumeric(), "-")
            .trim_matches('-')
            .to_string();

        // Determine effect type
        let effect_type =
            if text.to_lowercase().contains("shall") || text.to_lowercase().contains("must") {
                EffectType::Obligation
            } else if text.to_lowercase().contains("may") {
                EffectType::Grant
            } else if text.to_lowercase().contains("shall not")
                || text.to_lowercase().contains("prohibited")
            {
                EffectType::Prohibition
            } else {
                EffectType::Grant
            };

        let mut effect = Effect::new(effect_type, "provision");
        effect.parameters.insert("text".to_string(), text.clone());

        if let Some(conspicuous) = &form.conspicuous {
            effect
                .parameters
                .insert("conspicuous".to_string(), conspicuous.clone());
        }

        let mut statute = Statute::new(&id, &title, effect);

        for condition in conditions {
            statute = statute.with_precondition(condition);
        }

        statute
    }
}

impl Default for CommonFormImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for CommonFormImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::CommonForm
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::CommonForm, LegalFormat::Legalis);

        // Try to parse as CommonForm JSON
        let form: CommonForm = serde_json::from_str(source).map_err(|e| {
            InteropError::ParseError(format!("Failed to parse CommonForm JSON: {}", e))
        })?;

        let mut statutes = Vec::new();
        let statute = self.form_to_statute(&form, 0);
        statutes.push(statute);

        report.statutes_converted = statutes.len();

        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        // Check if it's valid JSON with CommonForm structure
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(source) {
            value.get("content").is_some()
        } else {
            false
        }
    }
}

/// CommonForm format exporter.
pub struct CommonFormExporter;

impl CommonFormExporter {
    /// Creates a new CommonForm exporter.
    pub fn new() -> Self {
        Self
    }

    fn statute_to_form(&self, statute: &Statute) -> CommonForm {
        let mut content = Vec::new();

        // Add title as text
        content.push(CommonFormContent::Text(statute.title.clone()));

        // Note: Statute description not available in current structure

        // Add conditions as text
        for condition in &statute.preconditions {
            let condition_text = match condition {
                Condition::Age { operator, value } => {
                    let op_text = match operator {
                        legalis_core::ComparisonOp::GreaterOrEqual => "at least",
                        legalis_core::ComparisonOp::LessThan => "less than",
                        legalis_core::ComparisonOp::Equal => "exactly",
                        legalis_core::ComparisonOp::GreaterThan => "greater than",
                        legalis_core::ComparisonOp::LessOrEqual => "at most",
                        legalis_core::ComparisonOp::NotEqual => "not equal to",
                    };
                    format!("The party must be {} {} years of age.", op_text, value)
                }
                Condition::AttributeEquals { key, value } if key == "citizenship" => {
                    format!("The party must be a citizen of {}.", value)
                }
                _ => "Additional conditions apply.".to_string(),
            };

            content.push(CommonFormContent::Text(condition_text));
        }

        // Add effect
        let effect_verb = match statute.effect.effect_type {
            EffectType::Grant => "may",
            EffectType::Obligation => "shall",
            EffectType::Prohibition => "shall not",
            EffectType::Revoke => "void",
            EffectType::MonetaryTransfer => "shall transfer",
            EffectType::StatusChange => "shall change",
            EffectType::Custom => "shall apply",
        };

        content.push(CommonFormContent::Text(format!(
            "The party {} {}.",
            effect_verb, statute.effect.description
        )));

        // Add blanks for parameters
        for (key, value) in &statute.effect.parameters {
            if key.starts_with("var_") {
                content.push(CommonFormContent::Blank {
                    blank: value.clone(),
                });
            }
        }

        let conspicuous = statute.effect.parameters.get("conspicuous").cloned();

        CommonForm {
            content,
            conspicuous,
        }
    }
}

impl Default for CommonFormExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for CommonFormExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::CommonForm
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::CommonForm);

        // For multiple statutes, we'll export as an array
        if statutes.is_empty() {
            return Ok((
                serde_json::to_string_pretty(&CommonForm {
                    content: Vec::new(),
                    conspicuous: None,
                })
                .unwrap(),
                report,
            ));
        }

        // For single statute, export as single form
        if statutes.len() == 1 {
            let form = self.statute_to_form(&statutes[0]);
            let output = serde_json::to_string_pretty(&form)
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;
            report.statutes_converted = 1;
            return Ok((output, report));
        }

        // For multiple statutes, create a composite form
        let mut content = Vec::new();
        for statute in statutes {
            let form = self.statute_to_form(statute);
            content.push(CommonFormContent::Form { form });
        }

        let composite = CommonForm {
            content,
            conspicuous: None,
        };

        let output = serde_json::to_string_pretty(&composite)
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;

        report.statutes_converted = statutes.len();
        Ok((output, report))
    }

    fn can_represent(&self, _statute: &Statute) -> Vec<String> {
        // CommonForm can represent most features
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commonform_import_simple() {
        let importer = CommonFormImporter::new();

        let source = r#"{
  "content": [
    "The party shall pay",
    { "blank": "amount" },
    "upon signing."
  ]
}"#;

        let (statutes, report) = importer.import(source).unwrap();
        assert_eq!(report.statutes_converted, 1);
        assert_eq!(statutes.len(), 1);
    }

    #[test]
    fn test_commonform_export_simple() {
        let exporter = CommonFormExporter::new();

        let mut effect = Effect::new(EffectType::Obligation, "pay amount");
        effect
            .parameters
            .insert("var_0".to_string(), "amount".to_string());

        let statute = Statute::new("payment", "Payment Clause", effect);

        let (output, report) = exporter.export(&[statute]).unwrap();
        assert_eq!(report.statutes_converted, 1);
        assert!(output.contains("content"));
        assert!(output.contains("Payment Clause"));
    }

    #[test]
    fn test_commonform_validate() {
        let importer = CommonFormImporter::new();

        assert!(importer.validate(r#"{"content": []}"#));
        assert!(importer.validate(r#"{"content": ["text"]}"#));
        assert!(!importer.validate("not json"));
        assert!(!importer.validate(r#"{"no_content": []}"#));
    }

    #[test]
    fn test_commonform_roundtrip() {
        let importer = CommonFormImporter::new();
        let exporter = CommonFormExporter::new();

        let effect = Effect::new(EffectType::Grant, "execute");
        let statute = Statute::new("clause", "Test Clause", effect);

        let (exported, _) = exporter.export(&[statute]).unwrap();
        let (imported, _) = importer.import(&exported).unwrap();

        assert_eq!(imported.len(), 1);
    }

    #[test]
    fn test_commonform_with_definition() {
        let importer = CommonFormImporter::new();

        let source = r#"{
  "content": [
    "The",
    { "definition": "Seller" },
    "shall deliver the goods."
  ]
}"#;

        let (statutes, report) = importer.import(source).unwrap();
        assert_eq!(report.statutes_converted, 1);
        assert_eq!(statutes.len(), 1);
    }
}
