//! L4 format support.
//!
//! L4 is a legal DSL developed in Singapore for expressing legal rules
//! with deontic logic (obligations, permissions, prohibitions).
//!
//! Key features:
//! - Deontic operators (MUST, MAY, SHANT)
//! - Rule-based reasoning
//! - Decision tables
//! - Natural language integration

use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

/// Deontic modality in L4.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeonticModality {
    /// Obligation - MUST
    Must,
    /// Permission - MAY
    May,
    /// Prohibition - SHANT
    Shant,
}

impl DeonticModality {
    /// Converts to L4 keyword.
    pub fn to_l4(&self) -> &'static str {
        match self {
            DeonticModality::Must => "MUST",
            DeonticModality::May => "MAY",
            DeonticModality::Shant => "SHANT",
        }
    }

    /// Parses from L4 keyword.
    pub fn from_l4(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "MUST" | "SHALL" | "OBLIGED" => Some(DeonticModality::Must),
            "MAY" | "PERMITTED" | "CAN" => Some(DeonticModality::May),
            "SHANT" | "SHALL NOT" | "MUST NOT" | "FORBIDDEN" => Some(DeonticModality::Shant),
            _ => None,
        }
    }

    /// Maps to Legalis effect type.
    pub fn to_effect_type(&self) -> EffectType {
        match self {
            DeonticModality::Must => EffectType::Obligation,
            DeonticModality::May => EffectType::Grant,
            DeonticModality::Shant => EffectType::Prohibition,
        }
    }
}

/// L4 format importer.
pub struct L4Importer {
    /// Whether to preserve decision tables
    _preserve_tables: bool,
}

impl L4Importer {
    /// Creates a new L4 importer.
    pub fn new() -> Self {
        Self {
            _preserve_tables: true,
        }
    }

    /// Parses an L4 rule.
    fn parse_rule(&self, content: &str, report: &mut ConversionReport) -> Option<Statute> {
        // Look for rule declaration patterns
        // L4 has various syntaxes, we support the common ones

        // Pattern 1: "RULE <name> WHEN <condition> THEN <party> <modality> <action>"
        let rule_re = regex_lite::Regex::new(
            r"(?i)RULE\s+(\w+)\s+WHEN\s+(.+?)\s+THEN\s+(\w+)\s+(MUST|MAY|SHANT|SHALL|SHALL NOT)\s+(.+)",
        )
        .ok()?;

        if let Some(captures) = rule_re.captures(content) {
            let rule_name = captures.get(1)?.as_str();
            let condition_str = captures.get(2)?.as_str();
            let _party = captures.get(3)?.as_str();
            let modality_str = captures.get(4)?.as_str();
            let action = captures.get(5)?.as_str();

            let modality = DeonticModality::from_l4(modality_str)?;
            let effect_type = modality.to_effect_type();

            let mut statute = Statute::new(
                rule_name.to_lowercase().replace(' ', "-"),
                rule_name,
                Effect::new(effect_type, action.trim()),
            );

            // Parse condition
            if let Some(cond) = Self::parse_l4_condition(condition_str, report) {
                statute.preconditions.push(cond);
            }

            return Some(statute);
        }

        // Pattern 2: Simple "GIVEN <conditions> <party> <modality> <action>"
        let given_re =
            regex_lite::Regex::new(r"(?i)GIVEN\s+(.+?)\s+(\w+)\s+(MUST|MAY|SHANT)\s+(.+)").ok()?;

        if let Some(captures) = given_re.captures(content) {
            let condition_str = captures.get(1)?.as_str();
            let _party = captures.get(2)?.as_str();
            let modality_str = captures.get(3)?.as_str();
            let action = captures.get(4)?.as_str();

            let modality = DeonticModality::from_l4(modality_str)?;
            let effect_type = modality.to_effect_type();

            let mut statute = Statute::new(
                "l4-rule",
                "L4 Rule",
                Effect::new(effect_type, action.trim()),
            );

            if let Some(cond) = Self::parse_l4_condition(condition_str, report) {
                statute.preconditions.push(cond);
            }

            return Some(statute);
        }

        None
    }

    /// Parses an L4 condition expression.
    fn parse_l4_condition(expr: &str, report: &mut ConversionReport) -> Option<Condition> {
        let expr = expr.trim();

        // Check for AND/OR
        if let Some(pos) = expr.to_uppercase().find(" AND ") {
            let left = &expr[..pos];
            let right = &expr[pos + 5..];
            let left_cond = Self::parse_l4_condition(left, report)?;
            let right_cond = Self::parse_l4_condition(right, report)?;
            return Some(Condition::And(Box::new(left_cond), Box::new(right_cond)));
        }

        if let Some(pos) = expr.to_uppercase().find(" OR ") {
            let left = &expr[..pos];
            let right = &expr[pos + 4..];
            let left_cond = Self::parse_l4_condition(left, report)?;
            let right_cond = Self::parse_l4_condition(right, report)?;
            return Some(Condition::Or(Box::new(left_cond), Box::new(right_cond)));
        }

        // Check for comparison: "field IS value" or "field >= value"
        if let Some(pos) = expr.to_uppercase().find(" IS ") {
            let field = expr[..pos].trim();
            let value = expr[pos + 4..].trim();
            return Some(Condition::AttributeEquals {
                key: field.to_string(),
                value: value.to_string(),
            });
        }

        // Numeric comparisons
        for (op_str, op) in [
            (">=", ComparisonOp::GreaterOrEqual),
            ("<=", ComparisonOp::LessOrEqual),
            (">", ComparisonOp::GreaterThan),
            ("<", ComparisonOp::LessThan),
            ("=", ComparisonOp::Equal),
        ] {
            if let Some(pos) = expr.find(op_str) {
                let field = expr[..pos].trim();
                let value = expr[pos + op_str.len()..].trim();

                // Check if it's an age field
                if field.to_lowercase().contains("age") {
                    if let Ok(v) = value.parse::<u32>() {
                        return Some(Condition::Age {
                            operator: op,
                            value: v,
                        });
                    }
                }

                return Some(Condition::AttributeEquals {
                    key: field.to_string(),
                    value: format!("{} {}", op_str, value),
                });
            }
        }

        report.add_warning(format!("Could not parse L4 condition: {}", expr));
        None
    }
}

impl Default for L4Importer {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for L4Importer {
    fn format(&self) -> LegalFormat {
        LegalFormat::L4
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::L4, LegalFormat::Legalis);
        let mut statutes = Vec::new();

        // Try to parse rules
        for line in source.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") || line.starts_with('#') {
                continue;
            }

            if let Some(statute) = self.parse_rule(line, &mut report) {
                statutes.push(statute);
            }
        }

        // Also try multi-line rules
        if statutes.is_empty() {
            // Try parsing as a single multi-line rule
            if let Some(statute) = self.parse_rule(source, &mut report) {
                statutes.push(statute);
            }
        }

        if statutes.is_empty() {
            return Err(InteropError::ParseError(
                "No valid L4 rules found".to_string(),
            ));
        }

        // Note unsupported features
        if source.to_uppercase().contains("DECIDE") {
            report.add_unsupported("L4 DECIDE blocks (decision tables)");
        }
        if source.to_uppercase().contains("UNLESS") {
            report.add_unsupported("L4 UNLESS exceptions");
        }
        if source.to_uppercase().contains("WITHIN") {
            report.add_unsupported("L4 WITHIN temporal constraints");
        }

        report.statutes_converted = statutes.len();
        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        let upper = source.to_uppercase();
        (upper.contains("RULE ") || upper.contains("GIVEN "))
            && (upper.contains("MUST") || upper.contains("MAY") || upper.contains("SHANT"))
    }
}

/// L4 format exporter.
pub struct L4Exporter {
    /// Default party name
    default_party: String,
}

impl L4Exporter {
    /// Creates a new L4 exporter.
    pub fn new() -> Self {
        Self {
            default_party: "Party".to_string(),
        }
    }

    /// Sets the default party name.
    pub fn with_party(mut self, party: impl Into<String>) -> Self {
        self.default_party = party.into();
        self
    }

    /// Converts a condition to L4 syntax.
    fn condition_to_l4(condition: &Condition, report: &mut ConversionReport) -> String {
        match condition {
            Condition::Age { operator, value } => {
                let op = Self::operator_to_l4(operator);
                format!("age {} {}", op, value)
            }
            Condition::Income { operator, value } => {
                let op = Self::operator_to_l4(operator);
                format!("income {} {}", op, value)
            }
            Condition::And(left, right) => {
                let l = Self::condition_to_l4(left, report);
                let r = Self::condition_to_l4(right, report);
                format!("{} AND {}", l, r)
            }
            Condition::Or(left, right) => {
                let l = Self::condition_to_l4(left, report);
                let r = Self::condition_to_l4(right, report);
                format!("{} OR {}", l, r)
            }
            Condition::Not(inner) => {
                let i = Self::condition_to_l4(inner, report);
                format!("NOT {}", i)
            }
            Condition::AttributeEquals { key, value } => {
                format!("{} IS {}", key, value)
            }
            Condition::HasAttribute { key } => {
                format!("{} EXISTS", key)
            }
            Condition::ResidencyDuration { operator, months } => {
                let op = Self::operator_to_l4(operator);
                format!("residency_months {} {}", op, months)
            }
            _ => {
                report.add_unsupported(format!("Condition type: {:?}", condition));
                "TRUE".to_string()
            }
        }
    }

    fn operator_to_l4(op: &ComparisonOp) -> &'static str {
        match op {
            ComparisonOp::Equal => "IS",
            ComparisonOp::NotEqual => "IS NOT",
            ComparisonOp::GreaterThan => ">",
            ComparisonOp::GreaterOrEqual => ">=",
            ComparisonOp::LessThan => "<",
            ComparisonOp::LessOrEqual => "<=",
        }
    }

    /// Maps Legalis effect type to L4 deontic modality.
    fn effect_to_modality(effect_type: &EffectType) -> DeonticModality {
        match effect_type {
            EffectType::Grant => DeonticModality::May,
            EffectType::Prohibition => DeonticModality::Shant,
            EffectType::Obligation => DeonticModality::Must,
            _ => DeonticModality::May,
        }
    }
}

impl Default for L4Exporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for L4Exporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::L4
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::L4);
        let mut output = String::new();

        // L4 header
        output.push_str("// Generated by Legalis-RS\n");
        output.push_str("// L4 Legal Rules\n\n");

        for statute in statutes {
            // Convert statute to rule name
            let rule_name: String = statute
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

            let modality = Self::effect_to_modality(&statute.effect.effect_type);

            // Build condition string
            let condition_str = if statute.preconditions.is_empty() {
                "TRUE".to_string()
            } else if statute.preconditions.len() == 1 {
                Self::condition_to_l4(&statute.preconditions[0], &mut report)
            } else {
                statute
                    .preconditions
                    .iter()
                    .map(|c| Self::condition_to_l4(c, &mut report))
                    .collect::<Vec<_>>()
                    .join(" AND ")
            };

            // Output rule
            output.push_str(&format!("RULE {}\n", rule_name));
            output.push_str(&format!("  WHEN {}\n", condition_str));
            output.push_str(&format!(
                "  THEN {} {} {}\n",
                self.default_party,
                modality.to_l4(),
                statute.effect.description
            ));

            // Handle discretion
            if let Some(ref discretion) = statute.discretion_logic {
                report.add_warning(format!("Discretion '{}' converted to comment", discretion));
                output.push_str(&format!("  // DISCRETION: {}\n", discretion));
            }

            output.push('\n');
            report.statutes_converted += 1;
        }

        Ok((output, report))
    }

    fn can_represent(&self, statute: &Statute) -> Vec<String> {
        let mut issues = Vec::new();

        if statute.discretion_logic.is_some() {
            issues.push("Discretionary logic will be converted to comments".to_string());
        }

        // Check for conditions that need special handling
        for condition in &statute.preconditions {
            match condition {
                Condition::Geographic { .. } => {
                    issues.push("Geographic conditions need manual review".to_string());
                }
                Condition::EntityRelationship { .. } => {
                    issues.push("Entity relationship conditions need manual review".to_string());
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
    fn test_deontic_modality() {
        assert_eq!(
            DeonticModality::from_l4("MUST"),
            Some(DeonticModality::Must)
        );
        assert_eq!(DeonticModality::from_l4("MAY"), Some(DeonticModality::May));
        assert_eq!(
            DeonticModality::from_l4("SHANT"),
            Some(DeonticModality::Shant)
        );
        assert_eq!(DeonticModality::from_l4("invalid"), None);
    }

    #[test]
    fn test_l4_importer_validate() {
        let importer = L4Importer::new();
        assert!(importer.validate("RULE Test WHEN age >= 18 THEN Person MUST vote"));
        assert!(importer.validate("GIVEN something Party MAY do action"));
        assert!(!importer.validate("declaration scope Test:"));
    }

    #[test]
    fn test_l4_importer_parse() {
        let importer = L4Importer::new();
        let source = "RULE AdultRights WHEN age >= 18 THEN Citizen MUST have_capacity";

        let (statutes, report) = importer.import(source).unwrap();
        assert_eq!(statutes.len(), 1);
        assert_eq!(report.statutes_converted, 1);
    }

    #[test]
    fn test_l4_exporter_basic() {
        let exporter = L4Exporter::new();
        let statute = Statute::new(
            "voting-rights",
            "Voting Rights",
            Effect::new(EffectType::Grant, "vote"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let (output, report) = exporter.export(&[statute]).unwrap();

        assert!(output.contains("RULE VotingRights"));
        assert!(output.contains("age >= 18"));
        assert!(output.contains("MAY vote"));
        assert_eq!(report.statutes_converted, 1);
    }

    #[test]
    fn test_l4_with_and_conditions() {
        let exporter = L4Exporter::new();
        let statute = Statute::new(
            "test-rule",
            "Test Rule",
            Effect::new(EffectType::Obligation, "comply"),
        )
        .with_precondition(Condition::And(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            }),
            Box::new(Condition::ResidencyDuration {
                operator: ComparisonOp::GreaterOrEqual,
                months: 12,
            }),
        ));

        let (output, report) = exporter.export(&[statute]).unwrap();

        assert!(output.contains("AND"));
        assert!(output.contains("MUST"));
        assert_eq!(report.statutes_converted, 1);
    }
}
