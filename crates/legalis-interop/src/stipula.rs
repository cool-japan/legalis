//! Stipula format support.
//!
//! Stipula is a domain-specific language for smart legal contracts,
//! developed at the University of Bologna, Italy.
//!
//! Key features:
//! - Party-based agreements
//! - Asset management
//! - State machine semantics
//! - Temporal obligations

use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

/// Stipula format importer.
pub struct StipulaImporter {
    /// Whether to preserve party definitions
    _preserve_parties: bool,
}

impl StipulaImporter {
    /// Creates a new Stipula importer.
    pub fn new() -> Self {
        Self {
            _preserve_parties: true,
        }
    }

    /// Parses a Stipula agreement.
    fn parse_agreement(&self, content: &str, report: &mut ConversionReport) -> Option<Statute> {
        // Look for agreement declaration: "agreement AgreementName(parties) {"
        let agreement_re = regex_lite::Regex::new(r"agreement\s+(\w+)\s*\(([^)]*)\)\s*\{").ok()?;
        let captures = agreement_re.captures(content)?;
        let agreement_name = captures.get(1)?.as_str();
        let parties_str = captures.get(2).map(|m| m.as_str()).unwrap_or("");

        // Parse parties
        let parties: Vec<&str> = parties_str.split(',').map(|s| s.trim()).collect();

        // Create statute from agreement
        let mut statute = Statute::new(
            agreement_name.to_lowercase().replace(' ', "-"),
            agreement_name,
            Effect::new(EffectType::Grant, "Agreement execution"),
        );

        // Add parties as entity conditions
        for party in &parties {
            if !party.is_empty() {
                statute.preconditions.push(Condition::AttributeEquals {
                    key: "party".to_string(),
                    value: party.to_string(),
                });
            }
        }

        // Parse asset declarations
        let asset_re = regex_lite::Regex::new(r"asset\s+(\w+)\s*:\s*(\w+)").ok()?;
        for cap in asset_re.captures_iter(content) {
            let asset_name = cap.get(1).map(|m| m.as_str()).unwrap_or("asset");
            let asset_type = cap.get(2).map(|m| m.as_str()).unwrap_or("unknown");
            report.add_warning(format!(
                "Asset '{}' of type '{}' noted as custom field",
                asset_name, asset_type
            ));
        }

        // Parse when clauses (conditions)
        let when_re = regex_lite::Regex::new(r"when\s+([^{]+)\s*\{").ok()?;
        for cap in when_re.captures_iter(content) {
            let condition_str = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            if let Some(cond) = self.parse_condition(condition_str, report) {
                statute.preconditions.push(cond);
            }
        }

        Some(statute)
    }

    /// Parses a Stipula condition expression.
    fn parse_condition(&self, expr: &str, report: &mut ConversionReport) -> Option<Condition> {
        let expr = expr.trim();

        // Check for comparison operators
        if let Some(pos) = expr.find(">=") {
            let field = expr[..pos].trim();
            let value = expr[pos + 2..].trim();
            if field.to_lowercase().contains("age") {
                if let Ok(v) = value.parse::<u32>() {
                    return Some(Condition::Age {
                        operator: ComparisonOp::GreaterOrEqual,
                        value: v,
                    });
                }
            }
            return Some(Condition::AttributeEquals {
                key: field.to_string(),
                value: format!(">= {}", value),
            });
        } else if let Some(pos) = expr.find("<=") {
            let field = expr[..pos].trim();
            let value = expr[pos + 2..].trim();
            if field.to_lowercase().contains("age") {
                if let Ok(v) = value.parse::<u32>() {
                    return Some(Condition::Age {
                        operator: ComparisonOp::LessOrEqual,
                        value: v,
                    });
                }
            }
            return Some(Condition::AttributeEquals {
                key: field.to_string(),
                value: format!("<= {}", value),
            });
        } else if let Some(pos) = expr.find("==") {
            let field = expr[..pos].trim();
            let value = expr[pos + 2..].trim();
            return Some(Condition::AttributeEquals {
                key: field.to_string(),
                value: value.to_string(),
            });
        }

        report.add_warning(format!("Could not parse condition: {}", expr));
        None
    }
}

impl Default for StipulaImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for StipulaImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Stipula
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Stipula, LegalFormat::Legalis);
        let mut statutes = Vec::new();

        // Split by agreement declarations
        let sections: Vec<&str> = source.split("agreement ").collect();

        for (i, section) in sections.iter().enumerate() {
            if i == 0 && !section.contains('{') {
                continue; // Skip preamble
            }

            let full_section = format!("agreement {}", section);
            if let Some(statute) = self.parse_agreement(&full_section, &mut report) {
                statutes.push(statute);
            }
        }

        if statutes.is_empty() {
            return Err(InteropError::ParseError(
                "No valid Stipula agreements found".to_string(),
            ));
        }

        // Note unsupported features
        if source.contains('@') {
            report.add_unsupported("Stipula event handlers (@)");
        }
        if source.contains("now") {
            report.add_unsupported("Stipula temporal expressions (now)");
        }
        if source.contains("->") {
            report.add_unsupported("Stipula state transitions (->)");
        }

        report.statutes_converted = statutes.len();
        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        source.contains("agreement ") && source.contains('{')
    }
}

/// Stipula format exporter.
pub struct StipulaExporter {
    /// Default party names
    default_parties: Vec<String>,
}

impl StipulaExporter {
    /// Creates a new Stipula exporter.
    pub fn new() -> Self {
        Self {
            default_parties: vec!["PartyA".to_string(), "PartyB".to_string()],
        }
    }

    /// Sets default parties for export.
    pub fn with_parties(mut self, parties: Vec<String>) -> Self {
        self.default_parties = parties;
        self
    }

    /// Converts a condition to Stipula syntax.
    fn condition_to_stipula(condition: &Condition, report: &mut ConversionReport) -> String {
        match condition {
            Condition::Age { operator, value } => {
                let op = Self::operator_to_stipula(operator);
                format!("age {} {}", op, value)
            }
            Condition::Income { operator, value } => {
                let op = Self::operator_to_stipula(operator);
                format!("amount {} {}", op, value)
            }
            Condition::And(left, right) => {
                let l = Self::condition_to_stipula(left, report);
                let r = Self::condition_to_stipula(right, report);
                format!("{} && {}", l, r)
            }
            Condition::Or(left, right) => {
                let l = Self::condition_to_stipula(left, report);
                let r = Self::condition_to_stipula(right, report);
                format!("{} || {}", l, r)
            }
            Condition::Not(inner) => {
                let i = Self::condition_to_stipula(inner, report);
                format!("!{}", i)
            }
            Condition::AttributeEquals { key, value } => {
                format!("{} == {}", key, value)
            }
            Condition::HasAttribute { key } => {
                format!("{} exists", key)
            }
            _ => {
                report.add_unsupported(format!("Condition type: {:?}", condition));
                "true".to_string()
            }
        }
    }

    fn operator_to_stipula(op: &ComparisonOp) -> &'static str {
        match op {
            ComparisonOp::Equal => "==",
            ComparisonOp::NotEqual => "!=",
            ComparisonOp::GreaterThan => ">",
            ComparisonOp::GreaterOrEqual => ">=",
            ComparisonOp::LessThan => "<",
            ComparisonOp::LessOrEqual => "<=",
        }
    }
}

impl Default for StipulaExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for StipulaExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Stipula
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::Stipula);
        let mut output = String::new();

        // Stipula header
        output.push_str("// Generated by Legalis-RS\n\n");

        for statute in statutes {
            // Convert statute ID to CamelCase for agreement name
            let agreement_name: String = statute
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

            // Parties
            let parties = self.default_parties.join(", ");

            // Agreement declaration
            output.push_str(&format!("agreement {}({}) {{\n", agreement_name, parties));

            // Fields section
            output.push_str("  fields {\n");
            output.push_str("    // Auto-generated fields\n");
            output.push_str("  }\n\n");

            // Init clause
            output.push_str("  init {\n");
            output.push_str(&format!("    // {}\n", statute.title));
            output.push_str("  }\n\n");

            // When clause for conditions
            if !statute.preconditions.is_empty() {
                let conditions: Vec<String> = statute
                    .preconditions
                    .iter()
                    .map(|c| Self::condition_to_stipula(c, &mut report))
                    .collect();

                output.push_str(&format!("  when {} {{\n", conditions.join(" && ")));
                output.push_str(&format!("    // Effect: {}\n", statute.effect.description));
                output.push_str("  }\n");
            }

            // Handle discretion
            if let Some(ref discretion) = statute.discretion_logic {
                report.add_warning(format!("Discretion '{}' converted to comment", discretion));
                output.push_str(&format!("\n  // DISCRETION: {}\n", discretion));
            }

            output.push_str("}\n\n");
            report.statutes_converted += 1;
        }

        Ok((output, report))
    }

    fn can_represent(&self, statute: &Statute) -> Vec<String> {
        let mut issues = Vec::new();

        if statute.discretion_logic.is_some() {
            issues.push("Discretionary logic will be converted to comments".to_string());
        }

        // Stipula is contract-focused, regulatory statutes may not map well
        if statute.effect.effect_type == EffectType::Prohibition {
            issues.push(
                "Prohibition effects may need manual adjustment for contract context".to_string(),
            );
        }

        issues
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stipula_importer_validate() {
        let importer = StipulaImporter::new();
        assert!(importer.validate("agreement Test(A, B) { }"));
        assert!(!importer.validate("declaration scope Test:"));
    }

    #[test]
    fn test_stipula_exporter_basic() {
        let exporter = StipulaExporter::new();
        let statute = Statute::new(
            "simple-contract",
            "Simple Contract",
            Effect::new(EffectType::Grant, "Contract execution"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let (output, report) = exporter.export(&[statute]).unwrap();

        assert!(output.contains("agreement SimpleContract"));
        assert!(output.contains("age >= 18"));
        assert_eq!(report.statutes_converted, 1);
    }

    #[test]
    fn test_stipula_with_multiple_conditions() {
        let exporter = StipulaExporter::new();
        let statute = Statute::new(
            "multi-cond",
            "Multi Condition",
            Effect::new(EffectType::Grant, "Grant"),
        )
        .with_precondition(Condition::And(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            }),
            Box::new(Condition::AttributeEquals {
                key: "verified".to_string(),
                value: "true".to_string(),
            }),
        ));

        let (output, _) = exporter.export(&[statute]).unwrap();
        assert!(output.contains("&&"));
    }
}
