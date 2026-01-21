//! ContractExpress format support.
//!
//! ContractExpress is a document automation platform for creating legal documents.
//! It uses templates with merge fields, conditional logic, and data sources.
//!
//! Format characteristics:
//! - Word-based templates with merge fields
//! - XML-based template definitions
//! - Conditional sections and logic
//! - Data questionnaires
//! - Answer files (XML)
//!
//! Reference: <https://www.contractexpress.com/>

use crate::{ConversionReport, FormatExporter, FormatImporter, InteropResult, LegalFormat};
use legalis_core::{Condition, Effect, EffectType, Statute};
use regex_lite::Regex;

/// ContractExpress template structure (simplified XML representation).
#[allow(dead_code)]
#[derive(Debug, Clone)]
struct ContractExpressTemplate {
    name: String,
    description: String,
    fields: Vec<MergeField>,
    sections: Vec<TemplateSection>,
    conditions: Vec<TemplateCondition>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct MergeField {
    name: String,
    field_type: String,
    default_value: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct TemplateSection {
    title: String,
    content: String,
    condition: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct TemplateCondition {
    field: String,
    operator: String,
    value: String,
}

/// ContractExpress format importer.
pub struct ContractExpressImporter;

impl ContractExpressImporter {
    /// Creates a new ContractExpress importer.
    pub fn new() -> Self {
        Self
    }

    fn parse_merge_fields(&self, text: &str) -> Vec<String> {
        // Merge fields in ContractExpress are typically «Field_Name»
        let re = Regex::new(r"«([^»]+)»").unwrap();
        re.captures_iter(text)
            .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
            .collect()
    }

    fn parse_conditionals(&self, text: &str) -> Vec<Condition> {
        let mut conditions = Vec::new();

        // Look for IF statements
        let re = Regex::new(r"IF\s+([^:]+):").unwrap();
        for cap in re.captures_iter(text) {
            if let Some(condition_text) = cap.get(1) {
                let cond_str = condition_text.as_str().trim();

                // Parse age conditions
                if cond_str.to_lowercase().contains("age") {
                    let age_re = Regex::new(r"age\s*>=\s*(\d+)").unwrap();
                    if let Some(age_cap) = age_re.captures(cond_str)
                        && let Some(age_str) = age_cap.get(1)
                        && let Ok(age_val) = age_str.as_str().parse::<u32>()
                    {
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

    fn extract_template_sections(&self, text: &str) -> Vec<(String, String)> {
        let mut sections = Vec::new();

        // Split by section markers manually (regex_lite doesn't support look-ahead)
        let parts: Vec<&str> = text.split("SECTION:").collect();

        for part in parts.iter().skip(1) {
            // Skip first empty part
            if let Some(newline_pos) = part.find('\n') {
                let title = part[..newline_pos].trim().to_string();
                let content = part[newline_pos + 1..].trim().to_string();
                sections.push((title, content));
            }
        }

        // If no explicit sections, treat entire text as one section
        if sections.is_empty() {
            sections.push(("Main".to_string(), text.to_string()));
        }

        sections
    }

    fn extract_statute_from_template(&self, text: &str, index: usize) -> Statute {
        let fields = self.parse_merge_fields(text);
        let conditions = self.parse_conditionals(text);
        let sections = self.extract_template_sections(text);

        // Extract title from TEMPLATE: line if present, otherwise from first section
        let title = {
            let mut found_title = None;
            for line in text.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("TEMPLATE:") {
                    let extracted = trimmed
                        .strip_prefix("TEMPLATE:")
                        .unwrap_or("")
                        .trim()
                        .to_string();
                    found_title = Some(extracted);
                    break;
                }
            }

            found_title.unwrap_or_else(|| {
                sections
                    .first()
                    .map(|(t, _)| t.clone())
                    .unwrap_or_else(|| format!("ContractExpress Template {}", index + 1))
            })
        };

        let id = title
            .to_lowercase()
            .replace(|c: char| !c.is_alphanumeric(), "-")
            .trim_matches('-')
            .to_string();

        // Determine effect type from content
        let all_content: String = sections
            .iter()
            .map(|(_, c)| c.clone())
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

        let mut effect = Effect::new(effect_type, "contract provision");

        // Store merge fields as parameters
        for (i, field) in fields.iter().enumerate() {
            effect
                .parameters
                .insert(format!("field_{}", i), field.clone());
        }

        let mut statute = Statute::new(&id, &title, effect);

        for condition in conditions {
            statute = statute.with_precondition(condition);
        }

        statute
    }
}

impl Default for ContractExpressImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for ContractExpressImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::ContractExpress
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::ContractExpress, LegalFormat::Legalis);

        // Split by template separators if present
        let templates: Vec<String> = if source.contains("TEMPLATE:") {
            source
                .split("TEMPLATE:")
                .skip(1)
                .map(|s| format!("TEMPLATE:{}", s.trim())) // Re-add the TEMPLATE: prefix
                .filter(|s| s.len() > "TEMPLATE:".len())
                .collect()
        } else {
            vec![source.to_string()]
        };

        let mut statutes = Vec::new();

        for (i, template) in templates.iter().enumerate() {
            let statute = self.extract_statute_from_template(template.as_str(), i);
            statutes.push(statute);
        }

        report.statutes_converted = statutes.len();

        if statutes.is_empty() {
            report.add_warning("No valid ContractExpress templates found");
        }

        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        // ContractExpress templates use «field» syntax or specific keywords
        source.contains('«') && source.contains('»')
            || source.contains("TEMPLATE:")
            || source.contains("SECTION:")
    }
}

/// ContractExpress format exporter.
pub struct ContractExpressExporter;

impl ContractExpressExporter {
    /// Creates a new ContractExpress exporter.
    pub fn new() -> Self {
        Self
    }

    fn statute_to_template(&self, statute: &Statute) -> String {
        let mut template = String::new();

        // Add template header
        template.push_str(&format!("TEMPLATE: {}\n\n", statute.title));

        // Note: Statute description not available in current structure

        // Add merge fields section
        let fields: Vec<String> = statute
            .effect
            .parameters
            .iter()
            .filter(|(k, _)| k.starts_with("field_"))
            .map(|(_, v)| v.clone())
            .collect();

        if !fields.is_empty() {
            template.push_str("MERGE FIELDS:\n");
            for field in &fields {
                template.push_str(&format!("- «{}»\n", field));
            }
            template.push('\n');
        }

        // Add conditions as IF statements
        if !statute.preconditions.is_empty() {
            template.push_str("CONDITIONS:\n");

            for condition in &statute.preconditions {
                match condition {
                    Condition::Age { operator, value } => {
                        let op_str = match operator {
                            legalis_core::ComparisonOp::GreaterOrEqual => ">=",
                            legalis_core::ComparisonOp::LessThan => "<",
                            legalis_core::ComparisonOp::Equal => "=",
                            legalis_core::ComparisonOp::GreaterThan => ">",
                            legalis_core::ComparisonOp::LessOrEqual => "<=",
                            legalis_core::ComparisonOp::NotEqual => "<>",
                        };
                        template.push_str(&format!("IF age {} {}:\n", op_str, value));
                        template.push_str("  Age requirement is satisfied.\n");
                    }
                    Condition::AttributeEquals { key, value } if key == "citizenship" => {
                        template.push_str(&format!("IF citizenship = \"{}\":\n", value));
                        template.push_str("  Citizenship requirement is satisfied.\n");
                    }
                    _ => {
                        template.push_str("IF custom_condition:\n");
                        template.push_str("  Custom condition is satisfied.\n");
                    }
                }
            }
            template.push('\n');
        }

        // Add main section
        template.push_str("SECTION: Main Provision\n\n");

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
            "The party {} {}.\n",
            effect_verb, statute.effect.description
        ));

        template
    }
}

impl Default for ContractExpressExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for ContractExpressExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::ContractExpress
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::ContractExpress);
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
        // ContractExpress can represent most features
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::ComparisonOp;

    #[test]
    fn test_contractexpress_import_simple() {
        let importer = ContractExpressImporter::new();

        let source = r#"
TEMPLATE: Employment Agreement

SECTION: Payment Terms

The employer shall pay «Employee_Name» the amount of «Salary» per year.

IF age >= 18:
  Employee must be an adult.
"#;

        let (statutes, report) = importer.import(source).unwrap();
        assert_eq!(report.statutes_converted, 1);
        assert_eq!(statutes.len(), 1);
        assert!(statutes[0].title.contains("Employment Agreement"));
    }

    #[test]
    fn test_contractexpress_export_simple() {
        let exporter = ContractExpressExporter::new();

        let mut effect = Effect::new(EffectType::Obligation, "pay salary");
        effect
            .parameters
            .insert("field_0".to_string(), "Employee_Name".to_string());
        effect
            .parameters
            .insert("field_1".to_string(), "Salary".to_string());

        let statute = Statute::new("employment", "Employment Agreement", effect).with_precondition(
            Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            },
        );

        let (output, report) = exporter.export(&[statute]).unwrap();
        assert_eq!(report.statutes_converted, 1);
        assert!(output.contains("TEMPLATE: Employment Agreement"));
        assert!(output.contains("«Employee_Name»"));
        assert!(output.contains("IF age >= 18"));
    }

    #[test]
    fn test_contractexpress_validate() {
        let importer = ContractExpressImporter::new();

        assert!(importer.validate("Text with «Field» merge field"));
        assert!(importer.validate("TEMPLATE: Name"));
        assert!(importer.validate("SECTION: Title"));
        assert!(!importer.validate("plain text"));
    }

    #[test]
    fn test_contractexpress_roundtrip() {
        let importer = ContractExpressImporter::new();
        let exporter = ContractExpressExporter::new();

        let mut effect = Effect::new(EffectType::Grant, "execute");
        effect
            .parameters
            .insert("field_0".to_string(), "Party_A".to_string());

        let statute = Statute::new("agreement", "Service Agreement", effect);

        let (exported, _) = exporter.export(&[statute]).unwrap();
        let (imported, _) = importer.import(&exported).unwrap();

        assert_eq!(imported.len(), 1);
        assert_eq!(imported[0].title, "Service Agreement");
    }

    #[test]
    fn test_contractexpress_multiple_templates() {
        let importer = ContractExpressImporter::new();

        let source = r#"
TEMPLATE: First Agreement

SECTION: Main
Content for first agreement.

TEMPLATE: Second Agreement

SECTION: Main
Content for second agreement.
"#;

        let (statutes, report) = importer.import(source).unwrap();
        assert_eq!(report.statutes_converted, 2);
        assert_eq!(statutes.len(), 2);
    }
}
