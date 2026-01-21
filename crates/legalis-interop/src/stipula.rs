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

/// Stipula state machine transition.
#[derive(Debug, Clone)]
pub struct StateMachineTransition {
    /// From state
    pub from: String,
    /// To state
    pub to: String,
    /// Trigger event
    pub trigger: Option<String>,
    /// Guard condition
    pub guard: Option<String>,
}

/// Stipula asset transfer.
#[derive(Debug, Clone)]
pub struct AssetTransfer {
    /// Asset name
    pub asset: String,
    /// Asset type
    pub asset_type: String,
    /// From party
    pub from: String,
    /// To party
    pub to: String,
    /// Amount or quantity
    pub amount: Option<String>,
}

/// Stipula temporal obligation.
#[derive(Debug, Clone)]
pub struct TemporalObligation {
    /// Obligation description
    pub description: String,
    /// Deadline expression (e.g., "now + 30 days")
    pub deadline: Option<String>,
    /// Recurring pattern
    pub recurring: Option<String>,
}

/// Stipula format importer.
pub struct StipulaImporter {
    /// Whether to preserve party definitions
    _preserve_parties: bool,
    /// Whether to convert state machines
    convert_state_machines: bool,
    /// Whether to track temporal obligations
    track_temporal: bool,
    /// Whether to track asset transfers
    track_assets: bool,
}

impl StipulaImporter {
    /// Creates a new Stipula importer.
    pub fn new() -> Self {
        Self {
            _preserve_parties: true,
            convert_state_machines: true,
            track_temporal: true,
            track_assets: true,
        }
    }

    /// Sets whether to convert state machines.
    pub fn with_state_machine_conversion(mut self, convert: bool) -> Self {
        self.convert_state_machines = convert;
        self
    }

    /// Sets whether to track temporal obligations.
    pub fn with_temporal_tracking(mut self, track: bool) -> Self {
        self.track_temporal = track;
        self
    }

    /// Sets whether to track asset transfers.
    pub fn with_asset_tracking(mut self, track: bool) -> Self {
        self.track_assets = track;
        self
    }

    /// Extracts state machine transitions from Stipula code.
    fn extract_state_transitions(&self, content: &str) -> Vec<StateMachineTransition> {
        let mut transitions = Vec::new();

        // Look for state transitions: "state -> newState"
        let transition_re = regex_lite::Regex::new(r"(?m)(\w+)\s*->\s*(\w+)").ok();

        if let Some(re) = transition_re {
            for cap in re.captures_iter(content) {
                if let (Some(from), Some(to)) = (cap.get(1), cap.get(2)) {
                    transitions.push(StateMachineTransition {
                        from: from.as_str().to_string(),
                        to: to.as_str().to_string(),
                        trigger: None,
                        guard: None,
                    });
                }
            }
        }

        transitions
    }

    /// Extracts temporal obligations from Stipula code.
    fn extract_temporal_obligations(&self, content: &str) -> Vec<TemporalObligation> {
        let mut obligations = Vec::new();

        // Look for "before", "after", "within" temporal keywords
        let temporal_re = regex_lite::Regex::new(r"(?i)(before|after|within)\s+(.+?)(?:\{|$)").ok();

        if let Some(re) = temporal_re {
            for cap in re.captures_iter(content) {
                if let (Some(keyword), Some(expr)) = (cap.get(1), cap.get(2)) {
                    obligations.push(TemporalObligation {
                        description: format!("{} {}", keyword.as_str(), expr.as_str().trim()),
                        deadline: Some(expr.as_str().trim().to_string()),
                        recurring: None,
                    });
                }
            }
        }

        // Look for "now" expressions
        let now_re =
            regex_lite::Regex::new(r"now\s*([+\-])\s*(\d+)\s*(days?|hours?|months?|years?)").ok();

        if let Some(re) = now_re {
            for cap in re.captures_iter(content) {
                if let (Some(op), Some(num), Some(unit)) = (cap.get(1), cap.get(2), cap.get(3)) {
                    obligations.push(TemporalObligation {
                        description: format!(
                            "Deadline: now {} {} {}",
                            op.as_str(),
                            num.as_str(),
                            unit.as_str()
                        ),
                        deadline: Some(format!(
                            "now {} {} {}",
                            op.as_str(),
                            num.as_str(),
                            unit.as_str()
                        )),
                        recurring: None,
                    });
                }
            }
        }

        obligations
    }

    /// Extracts asset transfers from Stipula code.
    fn extract_asset_transfers(&self, content: &str) -> Vec<AssetTransfer> {
        let mut transfers = Vec::new();

        // Look for "transfer" or "send" operations
        let transfer_re =
            regex_lite::Regex::new(r"(?i)(transfer|send)\s+(\w+)\s+from\s+(\w+)\s+to\s+(\w+)").ok();

        if let Some(re) = transfer_re {
            for cap in re.captures_iter(content) {
                if let (Some(asset), Some(from), Some(to)) = (cap.get(2), cap.get(3), cap.get(4)) {
                    transfers.push(AssetTransfer {
                        asset: asset.as_str().to_string(),
                        asset_type: "unknown".to_string(),
                        from: from.as_str().to_string(),
                        to: to.as_str().to_string(),
                        amount: None,
                    });
                }
            }
        }

        // Also look for asset declarations
        let asset_decl_re =
            regex_lite::Regex::new(r"asset\s+(\w+)\s*:\s*(\w+)(?:\s*=\s*(.+?))?").ok();

        if let Some(re) = asset_decl_re {
            for cap in re.captures_iter(content) {
                if let (Some(name), Some(typ)) = (cap.get(1), cap.get(2)) {
                    let amount = cap.get(3).map(|m| m.as_str().to_string());
                    // Create a dummy transfer to track the asset
                    transfers.push(AssetTransfer {
                        asset: name.as_str().to_string(),
                        asset_type: typ.as_str().to_string(),
                        from: "".to_string(),
                        to: "".to_string(),
                        amount,
                    });
                }
            }
        }

        transfers
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

        // Extract state transitions if enabled
        if self.convert_state_machines {
            let transitions = self.extract_state_transitions(content);
            if !transitions.is_empty() {
                report.add_warning(format!(
                    "Converted {} state transition(s) to conditional logic",
                    transitions.len()
                ));
            }
        }

        // Extract temporal obligations if enabled
        if self.track_temporal {
            let obligations = self.extract_temporal_obligations(content);
            if !obligations.is_empty() {
                report.add_warning(format!(
                    "Found {} temporal obligation(s): {}",
                    obligations.len(),
                    obligations
                        .iter()
                        .map(|o| o.description.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
        }

        // Extract asset transfers if enabled
        if self.track_assets {
            let transfers = self.extract_asset_transfers(content);
            if !transfers.is_empty() {
                report.add_warning(format!(
                    "Tracked {} asset declaration(s) and transfer(s)",
                    transfers.len()
                ));
            }
        }

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

        // Parse asset declarations (keeping original logic for backward compatibility)
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
            if field.to_lowercase().contains("age")
                && let Ok(v) = value.parse::<u32>()
            {
                return Some(Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: v,
                });
            }
            return Some(Condition::AttributeEquals {
                key: field.to_string(),
                value: format!(">= {}", value),
            });
        } else if let Some(pos) = expr.find("<=") {
            let field = expr[..pos].trim();
            let value = expr[pos + 2..].trim();
            if field.to_lowercase().contains("age")
                && let Ok(v) = value.parse::<u32>()
            {
                return Some(Condition::Age {
                    operator: ComparisonOp::LessOrEqual,
                    value: v,
                });
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
