//! Accord Project Cicero format support.
//!
//! Cicero is a template-based system for smart legal contracts from the Accord Project.
//! It uses CiceroMark (CommonMark + special syntax) for templates and JSON for data models.
//!
//! Format characteristics:
//! - CiceroMark template syntax (Markdown + {{variable}} placeholders)
//! - JSON data models (CTO - Concerto models)
//! - Clause templates with embedded logic
//! - Smart contract integration (Ergo logic language)
//!
//! Reference: https://accordproject.org/

use crate::{ConversionReport, FormatExporter, FormatImporter, InteropResult, LegalFormat};
use legalis_core::{Condition, Effect, EffectType, Statute};
use regex_lite::Regex;

/// Cicero format importer.
pub struct CiceroImporter;

impl CiceroImporter {
    /// Creates a new Cicero importer.
    pub fn new() -> Self {
        Self
    }

    fn parse_variables(&self, text: &str) -> Vec<String> {
        let re = Regex::new(r"\{\{([^}]+)\}\}").unwrap();
        re.captures_iter(text)
            .filter_map(|cap| cap.get(1).map(|m| m.as_str().trim().to_string()))
            .collect()
    }

    fn parse_clauses(&self, text: &str) -> Vec<String> {
        // Look for clause blocks
        let re = Regex::new(r"```cicero(?:mark)?\n(.*?)\n```").unwrap();
        re.captures_iter(text)
            .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
            .collect()
    }

    fn extract_conditions(&self, text: &str) -> Vec<Condition> {
        let mut conditions = Vec::new();

        // Parse condition patterns from text
        if text.contains("age") || text.contains("Age") {
            let re = Regex::new(r"(?:age|Age)\s*(?:>=|≥|at least|minimum of)\s*(\d+)").unwrap();
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

    fn extract_statute_from_template(&self, template: &str, index: usize) -> Statute {
        let variables = self.parse_variables(template);
        let conditions = self.extract_conditions(template);

        // Extract title from first heading or use clause name
        let title = if template.contains('#') {
            let re = Regex::new(r"^#\s*(.+)$").unwrap();
            template
                .lines()
                .find_map(|line| {
                    re.captures(line)
                        .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
                })
                .unwrap_or_else(|| format!("Cicero Clause {}", index + 1))
        } else {
            format!("Cicero Clause {}", index + 1)
        };

        let id = title
            .to_lowercase()
            .replace(|c: char| !c.is_alphanumeric(), "-")
            .trim_matches('-')
            .to_string();

        // Determine effect type
        let effect_type = if template.to_lowercase().contains("shall")
            || template.to_lowercase().contains("must")
            || template.to_lowercase().contains("required")
        {
            EffectType::Obligation
        } else if template.to_lowercase().contains("may")
            || template.to_lowercase().contains("permitted")
        {
            EffectType::Grant
        } else if template.to_lowercase().contains("shall not")
            || template.to_lowercase().contains("prohibited")
        {
            EffectType::Prohibition
        } else {
            EffectType::Grant
        };

        let mut effect = Effect::new(effect_type, "clause");

        // Store variables
        for (i, var) in variables.iter().enumerate() {
            effect.parameters.insert(format!("var_{}", i), var.clone());
        }

        let mut statute = Statute::new(&id, &title, effect);

        for condition in conditions {
            statute = statute.with_precondition(condition);
        }

        statute
    }
}

impl Default for CiceroImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for CiceroImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Cicero
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Cicero, LegalFormat::Legalis);

        let clauses = self.parse_clauses(source);

        let templates: Vec<&str> = if clauses.is_empty() {
            vec![source]
        } else {
            clauses.iter().map(|s| s.as_str()).collect()
        };

        let mut statutes = Vec::new();

        for (i, template) in templates.iter().enumerate() {
            let statute = self.extract_statute_from_template(template, i);
            statutes.push(statute);
        }

        report.statutes_converted = statutes.len();

        if statutes.is_empty() {
            report.add_warning("No valid Cicero templates found");
        }

        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        // Cicero templates use {{variable}} syntax or ciceromark code blocks
        source.contains("{{") && source.contains("}}")
            || source.contains("```cicero")
            || source.contains("namespace org.accordproject")
    }
}

/// Cicero format exporter.
pub struct CiceroExporter;

impl CiceroExporter {
    /// Creates a new Cicero exporter.
    pub fn new() -> Self {
        Self
    }

    fn statute_to_template(&self, statute: &Statute) -> String {
        let mut template = String::new();

        // Add title
        template.push_str(&format!("# {}\n\n", statute.title));

        // Add CiceroMark block
        template.push_str("```ciceromark\n");

        // Note: Statute description not available in current structure

        // Add variables
        let var_names: Vec<String> = statute
            .effect
            .parameters
            .iter()
            .filter(|(k, _)| k.starts_with("var_"))
            .map(|(_, v)| v.clone())
            .collect();

        if !var_names.is_empty() {
            template.push_str("Variables:\n");
            for var in &var_names {
                template.push_str(&format!("- {{{{{}}}}} \n", var));
            }
            template.push('\n');
        }

        // Add conditions as prose
        for condition in &statute.preconditions {
            match condition {
                Condition::Age { operator, value } => {
                    let op_text = match operator {
                        legalis_core::ComparisonOp::GreaterOrEqual => "at least",
                        legalis_core::ComparisonOp::LessThan => "less than",
                        legalis_core::ComparisonOp::Equal => "exactly",
                        legalis_core::ComparisonOp::GreaterThan => "greater than",
                        legalis_core::ComparisonOp::LessOrEqual => "at most",
                        legalis_core::ComparisonOp::NotEqual => "not equal to",
                    };
                    template.push_str(&format!(
                        "The party must be {} {} years of age.\n",
                        op_text, value
                    ));
                }
                Condition::AttributeEquals { key, value } if key == "citizenship" => {
                    template.push_str(&format!("The party must be a citizen of {}.\n", value));
                }
                _ => {
                    template.push_str("Additional conditions apply.\n");
                }
            }
        }

        // Add effect
        let effect_verb = match statute.effect.effect_type {
            EffectType::Grant => "MAY",
            EffectType::Obligation => "SHALL",
            EffectType::Prohibition => "SHALL NOT",
            EffectType::Revoke => "VOID",
            EffectType::MonetaryTransfer => "SHALL TRANSFER",
            EffectType::StatusChange => "SHALL CHANGE",
            EffectType::Custom => "SHALL APPLY",
        };

        template.push_str(&format!(
            "\nThe party {} {}.\n",
            effect_verb, statute.effect.description
        ));

        template.push_str("```\n");

        template
    }
}

impl Default for CiceroExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for CiceroExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Cicero
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::Cicero);
        let mut output = String::new();

        for (i, statute) in statutes.iter().enumerate() {
            if i > 0 {
                output.push_str("\n\n");
            }
            output.push_str(&self.statute_to_template(statute));
        }

        report.statutes_converted = statutes.len();
        Ok((output, report))
    }

    fn can_represent(&self, _statute: &Statute) -> Vec<String> {
        // Cicero can represent most features through CiceroMark
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::ComparisonOp;

    #[test]
    fn test_cicero_import_simple() {
        let importer = CiceroImporter::new();

        let source = r#"
# Late Delivery Clause

```ciceromark
Late Delivery and Penalty. In case of delayed delivery, the Seller shall pay to the Buyer a penalty of {{penalty}}% of the total value for every {{unit}} of delay.

The buyer must be at least 18 years of age.
```
"#;

        let (statutes, report) = importer.import(source).unwrap();
        assert_eq!(report.statutes_converted, 1);
        assert_eq!(statutes.len(), 1);
        assert!(statutes[0].title.contains("Late Delivery"));
    }

    #[test]
    fn test_cicero_export_simple() {
        let exporter = CiceroExporter::new();

        let mut effect = Effect::new(EffectType::Obligation, "pay penalty");
        effect
            .parameters
            .insert("var_0".to_string(), "penalty".to_string());
        effect
            .parameters
            .insert("var_1".to_string(), "unit".to_string());

        let statute = Statute::new("late-delivery", "Late Delivery Clause", effect)
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            });

        let (output, report) = exporter.export(&[statute]).unwrap();
        assert_eq!(report.statutes_converted, 1);
        assert!(output.contains("# Late Delivery Clause"));
        assert!(output.contains("```ciceromark"));
        assert!(output.contains("{{penalty}}"));
        assert!(output.contains("at least 18"));
    }

    #[test]
    fn test_cicero_validate() {
        let importer = CiceroImporter::new();

        assert!(importer.validate("Text with {{variable}} placeholder"));
        assert!(importer.validate("```ciceromark\ncontent\n```"));
        assert!(importer.validate("namespace org.accordproject.example"));
        assert!(!importer.validate("plain text without markup"));
    }

    #[test]
    fn test_cicero_roundtrip() {
        let importer = CiceroImporter::new();
        let exporter = CiceroExporter::new();

        let mut effect = Effect::new(EffectType::Grant, "execute agreement");
        effect
            .parameters
            .insert("var_0".to_string(), "party_name".to_string());

        let statute = Statute::new("agreement", "Service Agreement", effect);

        let (exported, _) = exporter.export(&[statute]).unwrap();
        let (imported, _) = importer.import(&exported).unwrap();

        assert_eq!(imported.len(), 1);
        assert_eq!(imported[0].title, "Service Agreement");
    }
}
