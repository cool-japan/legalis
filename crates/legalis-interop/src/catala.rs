//! Catala format support.
//!
//! Catala is a domain-specific language for deriving faithful-by-construction
//! implementations from legislative texts. Developed by Inria, France.
//!
//! Key features:
//! - Literate programming with legal text annotations
//! - Scope-based organization
//! - Strong typing with dates, money, durations
//! - Default logic for legal reasoning

use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

/// Catala format importer.
pub struct CatalaImporter {
    /// Whether to preserve legal text comments
    preserve_comments: bool,
}

impl CatalaImporter {
    /// Creates a new Catala importer.
    pub fn new() -> Self {
        Self {
            preserve_comments: true,
        }
    }

    /// Sets whether to preserve legal text comments.
    pub fn with_comments(mut self, preserve: bool) -> Self {
        self.preserve_comments = preserve;
        self
    }

    /// Parses a Catala scope declaration.
    fn parse_scope(&self, content: &str, report: &mut ConversionReport) -> Option<Statute> {
        // Look for scope declaration: "declaration scope ScopeName:"
        let scope_re = regex_lite::Regex::new(r"declaration\s+scope\s+(\w+):").ok()?;
        let captures = scope_re.captures(content)?;
        let scope_name = captures.get(1)?.as_str();

        // Create statute from scope
        let mut statute = Statute::new(
            scope_name.to_lowercase().replace(' ', "-"),
            scope_name,
            Effect::new(EffectType::Grant, "Scope output"),
        );

        // Parse context variables as conditions
        let context_re = regex_lite::Regex::new(r"context\s+(\w+)\s+content\s+(\w+)").ok()?;
        for cap in context_re.captures_iter(content) {
            let var_name = cap.get(1).map(|m| m.as_str()).unwrap_or("unknown");
            let var_type = cap.get(2).map(|m| m.as_str()).unwrap_or("unknown");

            // Map Catala types to conditions
            match var_type {
                "integer" | "decimal" => {
                    if var_name.to_lowercase().contains("age") {
                        statute.preconditions.push(Condition::Age {
                            operator: ComparisonOp::GreaterOrEqual,
                            value: 0,
                        });
                    }
                }
                "money" => {
                    if var_name.to_lowercase().contains("income") {
                        statute.preconditions.push(Condition::Income {
                            operator: ComparisonOp::GreaterOrEqual,
                            value: 0,
                        });
                    }
                }
                "boolean" => {
                    statute.preconditions.push(Condition::AttributeEquals {
                        key: var_name.to_string(),
                        value: "true".to_string(),
                    });
                }
                _ => {
                    report.add_warning(format!("Unknown Catala type: {}", var_type));
                }
            }
        }

        // Check for rule definitions
        let rule_re = regex_lite::Regex::new(r"rule\s+(\w+)\s+under\s+condition").ok()?;
        for cap in rule_re.captures_iter(content) {
            let rule_name = cap.get(1).map(|m| m.as_str()).unwrap_or("rule");
            report.add_warning(format!("Rule '{}' converted to condition", rule_name));
        }

        Some(statute)
    }
}

impl Default for CatalaImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for CatalaImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Catala
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Catala, LegalFormat::Legalis);
        let mut statutes = Vec::new();

        // Split by scope declarations
        let sections: Vec<&str> = source.split("declaration scope").collect();

        for (i, section) in sections.iter().enumerate() {
            if i == 0 && !section.contains("scope") {
                continue; // Skip preamble
            }

            let full_section = if i > 0 {
                format!("declaration scope{}", section)
            } else {
                section.to_string()
            };

            if let Some(statute) = self.parse_scope(&full_section, &mut report) {
                statutes.push(statute);
            }
        }

        if statutes.is_empty() {
            return Err(InteropError::ParseError(
                "No valid Catala scopes found".to_string(),
            ));
        }

        // Note unsupported features
        if source.contains("exception") {
            report.add_unsupported("Catala exceptions (default logic)");
        }
        if source.contains("assertion") {
            report.add_unsupported("Catala assertions");
        }
        if source.contains("definition") && source.contains("state") {
            report.add_unsupported("Catala state definitions");
        }

        report.statutes_converted = statutes.len();
        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        // Check for Catala markers
        source.contains("declaration scope")
            || source.contains("```catala")
            || source.contains("# Catala")
    }
}

/// Catala format exporter.
pub struct CatalaExporter {
    /// Language variant (en, fr)
    language: String,
}

impl CatalaExporter {
    /// Creates a new Catala exporter.
    pub fn new() -> Self {
        Self {
            language: "en".to_string(),
        }
    }

    /// Sets the output language.
    pub fn with_language(mut self, lang: impl Into<String>) -> Self {
        self.language = lang.into();
        self
    }

    /// Converts a condition to Catala syntax.
    fn condition_to_catala(condition: &Condition, report: &mut ConversionReport) -> String {
        match condition {
            Condition::Age { operator, value } => {
                let op = Self::operator_to_catala(operator);
                format!("input.age {} {}", op, value)
            }
            Condition::Income { operator, value } => {
                let op = Self::operator_to_catala(operator);
                format!("input.income {} ${}", op, value)
            }
            Condition::And(left, right) => {
                let l = Self::condition_to_catala(left, report);
                let r = Self::condition_to_catala(right, report);
                format!("({}) and ({})", l, r)
            }
            Condition::Or(left, right) => {
                let l = Self::condition_to_catala(left, report);
                let r = Self::condition_to_catala(right, report);
                format!("({}) or ({})", l, r)
            }
            Condition::Not(inner) => {
                let i = Self::condition_to_catala(inner, report);
                format!("not ({})", i)
            }
            Condition::AttributeEquals { key, value } => {
                format!("input.{} = {}", key, value)
            }
            Condition::HasAttribute { key } => {
                format!("input.{} exists", key)
            }
            _ => {
                report.add_unsupported(format!("Condition type: {:?}", condition));
                "true".to_string()
            }
        }
    }

    fn operator_to_catala(op: &ComparisonOp) -> &'static str {
        match op {
            ComparisonOp::Equal => "=",
            ComparisonOp::NotEqual => "!=",
            ComparisonOp::GreaterThan => ">",
            ComparisonOp::GreaterOrEqual => ">=",
            ComparisonOp::LessThan => "<",
            ComparisonOp::LessOrEqual => "<=",
        }
    }
}

impl Default for CatalaExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for CatalaExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Catala
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::Catala);
        let mut output = String::new();

        // Catala header
        output.push_str("# Generated by Legalis-RS\n");
        output.push_str(&format!("# Language: {}\n\n", self.language));

        for statute in statutes {
            // Convert statute ID to CamelCase for scope name
            let scope_name: String = statute
                .id
                .split('-')
                .map(|s| {
                    let mut c = s.chars();
                    match c.next() {
                        None => String::new(),
                        Some(f) => f.to_uppercase().chain(c).collect(),
                    }
                })
                .collect();

            // Scope declaration
            output.push_str(&format!("```catala\ndeclaration scope {}:\n", scope_name));

            // Input context
            output.push_str("  context input content Input\n");
            output.push_str("  context output content Output\n");
            output.push_str("```\n\n");

            // Scope definition with rules
            output.push_str(&format!("```catala\nscope {}:\n", scope_name));

            // Convert preconditions to rules
            if !statute.preconditions.is_empty() {
                output.push_str("  definition output.eligible equals\n");

                let conditions: Vec<String> = statute
                    .preconditions
                    .iter()
                    .map(|c| Self::condition_to_catala(c, &mut report))
                    .collect();

                if conditions.len() == 1 {
                    output.push_str(&format!("    {}\n", conditions[0]));
                } else {
                    output.push_str("    ");
                    output.push_str(&conditions.join(" and\n    "));
                    output.push('\n');
                }
            }

            // Handle discretion
            if let Some(ref discretion) = statute.discretion_logic {
                report.add_warning(format!(
                    "Discretion '{}' converted to Catala comment",
                    discretion
                ));
                output.push_str(&format!("  # DISCRETION: {}\n", discretion));
            }

            output.push_str("```\n\n");

            report.statutes_converted += 1;
        }

        Ok((output, report))
    }

    fn can_represent(&self, statute: &Statute) -> Vec<String> {
        let mut issues = Vec::new();

        // Check for features Catala doesn't support well
        if statute.discretion_logic.is_some() {
            issues.push("Discretionary logic will be converted to comments".to_string());
        }

        for condition in &statute.preconditions {
            match condition {
                Condition::ResidencyDuration { .. } => {
                    issues.push("Residency conditions need manual mapping".to_string());
                }
                Condition::Geographic { .. } => {
                    issues.push("Geographic conditions need manual mapping".to_string());
                }
                _ => {}
            }
        }

        issues
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catala_importer_validate() {
        let importer = CatalaImporter::new();
        assert!(importer.validate("declaration scope Test:"));
        assert!(importer.validate("```catala\ncode\n```"));
        assert!(!importer.validate("STATUTE foo: \"bar\" {}"));
    }

    #[test]
    fn test_catala_exporter_basic() {
        let exporter = CatalaExporter::new();
        let statute = Statute::new(
            "adult-rights",
            "Adult Rights",
            Effect::new(EffectType::Grant, "Full legal capacity"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let (output, report) = exporter.export(&[statute]).unwrap();

        assert!(output.contains("declaration scope AdultRights"));
        assert!(output.contains("input.age >= 18"));
        assert_eq!(report.statutes_converted, 1);
    }

    #[test]
    fn test_catala_roundtrip_concepts() {
        // Test that we can export and the output is valid Catala-like syntax
        let statute = Statute::new(
            "tax-benefit",
            "Tax Benefit Rule",
            Effect::new(EffectType::Grant, "Tax reduction"),
        )
        .with_precondition(Condition::And(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 65,
            }),
            Box::new(Condition::Income {
                operator: ComparisonOp::LessThan,
                value: 50000,
            }),
        ));

        let exporter = CatalaExporter::new();
        let (output, _) = exporter.export(&[statute]).unwrap();

        assert!(output.contains("input.age >= 65"));
        assert!(output.contains("and"));
        assert!(output.contains("input.income < $50000"));
    }
}
