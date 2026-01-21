//! OpenLaw format support.
//!
//! OpenLaw is a protocol for creating and executing legal agreements.
//! It uses a template-based system with variables, conditionals, and smart contract integration.
//!
//! Format characteristics:
//! - Markdown-like syntax with special markup
//! - Variables: [[Variable Name]] or [[Variable Name: Type]]
//! - Conditionals: <% if condition %>...<% else %>...<% endif %>
//! - Smart contract integration capabilities
//!
//! Reference: <https://docs.openlaw.io/>

use crate::{ConversionReport, FormatExporter, FormatImporter, InteropResult, LegalFormat};
use legalis_core::{Condition, Effect, EffectType, Statute};
use regex_lite::Regex;

/// OpenLaw format importer.
pub struct OpenLawImporter;

impl OpenLawImporter {
    /// Creates a new OpenLaw importer.
    pub fn new() -> Self {
        Self
    }

    fn parse_variables(&self, text: &str) -> Vec<String> {
        let re = Regex::new(r"\[\[([^\]]+)\]\]").unwrap();
        re.captures_iter(text)
            .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
            .collect()
    }

    fn parse_conditionals(&self, text: &str) -> Vec<Condition> {
        let mut conditions = Vec::new();
        let re = Regex::new(r"<%\s*if\s+([^%>]+)\s*%>").unwrap();

        for cap in re.captures_iter(text) {
            if let Some(condition_text) = cap.get(1) {
                let condition_str = condition_text.as_str().trim();

                // Parse simple conditions
                if condition_str.contains(">=") {
                    let parts: Vec<&str> = condition_str.split(">=").collect();
                    if parts.len() == 2 {
                        let var_name = parts[0].trim();
                        let value = parts[1].trim();

                        if var_name.to_lowercase().contains("age")
                            && let Ok(age_val) = value.parse::<u32>()
                        {
                            conditions.push(Condition::Age {
                                operator: legalis_core::ComparisonOp::GreaterOrEqual,
                                value: age_val,
                            });
                        }
                    }
                }
            }
        }

        conditions
    }

    fn extract_statute_from_template(&self, template: &str, index: usize) -> Statute {
        let variables = self.parse_variables(template);
        let conditions = self.parse_conditionals(template);

        // Extract title from first heading if present
        let title = if template.contains('#') {
            let re = Regex::new(r"^#\s*(.+)$").unwrap();
            template
                .lines()
                .find_map(|line| {
                    re.captures(line)
                        .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
                })
                .unwrap_or_else(|| format!("OpenLaw Template {}", index + 1))
        } else {
            format!("OpenLaw Template {}", index + 1)
        };

        // Generate statute ID from title
        let id = title
            .to_lowercase()
            .replace(|c: char| !c.is_alphanumeric(), "-")
            .trim_matches('-')
            .to_string();

        // Determine effect type based on content
        let effect_type = if template.to_lowercase().contains("grant")
            || template.to_lowercase().contains("permit")
        {
            EffectType::Grant
        } else if template.to_lowercase().contains("require")
            || template.to_lowercase().contains("must")
        {
            EffectType::Obligation
        } else {
            EffectType::Grant
        };

        let mut effect = Effect::new(effect_type, "contract");

        // Store variables as parameters
        for (i, var) in variables.iter().enumerate() {
            effect.parameters.insert(format!("var_{}", i), var.clone());
        }

        let mut statute = Statute::new(&id, &title, effect);

        // Add conditions
        for condition in conditions {
            statute = statute.with_precondition(condition);
        }

        statute
    }
}

impl Default for OpenLawImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for OpenLawImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::OpenLaw
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::OpenLaw, LegalFormat::Legalis);

        // Split by template sections (typically separated by double newlines or sections)
        let templates: Vec<&str> = if source.contains("<%=") || source.contains("[[") {
            vec![source]
        } else {
            source
                .split("\n\n\n")
                .filter(|s| !s.trim().is_empty())
                .collect()
        };

        let mut statutes = Vec::new();

        for (i, template) in templates.iter().enumerate() {
            let statute = self.extract_statute_from_template(template, i);
            statutes.push(statute);
        }

        report.statutes_converted = statutes.len();

        if statutes.is_empty() {
            report.add_warning("No valid OpenLaw templates found");
        }

        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        // OpenLaw templates typically contain [[ ]] for variables or <% %> for logic
        source.contains("[[") && source.contains("]]")
            || (source.contains("<%") && source.contains("%>"))
    }
}

/// OpenLaw format exporter.
pub struct OpenLawExporter;

impl OpenLawExporter {
    /// Creates a new OpenLaw exporter.
    pub fn new() -> Self {
        Self
    }

    fn statute_to_template(&self, statute: &Statute) -> String {
        let mut template = String::new();

        // Add title as heading
        template.push_str(&format!("# {}\n\n", statute.title));

        // Note: Statute description not available in current structure

        // Add variables from effect parameters
        let var_names: Vec<String> = statute
            .effect
            .parameters
            .iter()
            .filter(|(k, _)| k.starts_with("var_"))
            .map(|(_, v)| v.clone())
            .collect();

        if !var_names.is_empty() {
            template.push_str("## Variables\n\n");
            for var in &var_names {
                template.push_str(&format!("[[{}]]\n", var));
            }
            template.push('\n');
        }

        // Add conditions
        if !statute.preconditions.is_empty() {
            template.push_str("## Conditions\n\n");

            for condition in &statute.preconditions {
                match condition {
                    Condition::Age { operator, value } => {
                        let op_str = match operator {
                            legalis_core::ComparisonOp::GreaterOrEqual => ">=",
                            legalis_core::ComparisonOp::LessThan => "<",
                            legalis_core::ComparisonOp::Equal => "==",
                            legalis_core::ComparisonOp::GreaterThan => ">",
                            legalis_core::ComparisonOp::LessOrEqual => "<=",
                            legalis_core::ComparisonOp::NotEqual => "!=",
                        };
                        template.push_str(&format!("<% if age {} {} %>\n", op_str, value));
                        template.push_str("Condition met: Age requirement satisfied\n");
                        template.push_str("<% endif %>\n\n");
                    }
                    Condition::AttributeEquals { key, value } if key == "citizenship" => {
                        template.push_str(&format!("<% if citizenship == \"{}\" %>\n", value));
                        template.push_str("Condition met: Citizenship requirement satisfied\n");
                        template.push_str("<% endif %>\n\n");
                    }
                    _ => {
                        template.push_str("<% if custom_condition %>\n");
                        template.push_str("Custom condition check\n");
                        template.push_str("<% endif %>\n\n");
                    }
                }
            }
        }

        // Add effect description
        template.push_str("## Effect\n\n");
        let effect_verb = match statute.effect.effect_type {
            EffectType::Grant => "GRANTS",
            EffectType::Obligation => "REQUIRES",
            EffectType::Prohibition => "PROHIBITS",
            EffectType::Revoke => "VOIDS",
            EffectType::MonetaryTransfer => "TRANSFERS",
            EffectType::StatusChange => "CHANGES",
            EffectType::Custom => "AFFECTS",
        };
        template.push_str(&format!(
            "This agreement {} {}\n",
            effect_verb, statute.effect.description
        ));

        template
    }
}

impl Default for OpenLawExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for OpenLawExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::OpenLaw
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::OpenLaw);
        let mut output = String::new();

        for (i, statute) in statutes.iter().enumerate() {
            if i > 0 {
                output.push_str("\n\n---\n\n");
            }
            output.push_str(&self.statute_to_template(statute));
        }

        report.statutes_converted = statutes.len();
        Ok((output, report))
    }

    fn can_represent(&self, _statute: &Statute) -> Vec<String> {
        // OpenLaw can represent most statute features through its template system
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::ComparisonOp;

    #[test]
    fn test_openlaw_import_simple() {
        let importer = OpenLawImporter::new();

        let source = r#"
# Employment Agreement

[[Employee Name]]
[[Start Date: Date]]
[[Salary: Number]]

<% if age >= 18 %>
The employee must be at least 18 years old.
<% endif %>
"#;

        let (statutes, report) = importer.import(source).unwrap();
        assert_eq!(report.statutes_converted, 1);
        assert_eq!(statutes.len(), 1);
        assert!(statutes[0].title.contains("Employment Agreement"));
    }

    #[test]
    fn test_openlaw_export_simple() {
        let exporter = OpenLawExporter::new();

        let mut effect = Effect::new(EffectType::Grant, "employment");
        effect
            .parameters
            .insert("var_0".to_string(), "Employee Name".to_string());
        effect
            .parameters
            .insert("var_1".to_string(), "Salary: Number".to_string());

        let statute = Statute::new("employment", "Employment Agreement", effect).with_precondition(
            Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            },
        );

        let (output, report) = exporter.export(&[statute]).unwrap();
        assert_eq!(report.statutes_converted, 1);
        assert!(output.contains("# Employment Agreement"));
        assert!(output.contains("[[Employee Name]]"));
        assert!(output.contains("<% if age >= 18 %>"));
    }

    #[test]
    fn test_openlaw_validate() {
        let importer = OpenLawImporter::new();

        assert!(importer.validate("[[Variable Name]]"));
        assert!(importer.validate("<% if condition %>content<% endif %>"));
        assert!(!importer.validate("plain text without markup"));
    }

    #[test]
    fn test_openlaw_roundtrip() {
        let importer = OpenLawImporter::new();
        let exporter = OpenLawExporter::new();

        let mut effect = Effect::new(EffectType::Grant, "contract rights");
        effect
            .parameters
            .insert("var_0".to_string(), "Party A".to_string());

        let statute = Statute::new("contract", "Service Agreement", effect);

        let (exported, _) = exporter.export(&[statute]).unwrap();
        let (imported, _) = importer.import(&exported).unwrap();

        assert_eq!(imported.len(), 1);
        assert_eq!(imported[0].title, "Service Agreement");
    }
}
