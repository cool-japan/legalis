//! LegalRuleML format support.
//!
//! LegalRuleML is an OASIS standard for representing legal rules and norms in XML.
//! It provides a standardized way to express legal reasoning, precedents, and rules.
//!
//! Key features:
//! - Rule-based representation with premises and conclusions
//! - Support for defeasible and strict rules
//! - Legal authority and source tracking
//! - Temporal validity expressions

use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::{Reader, Writer};
use std::io::Cursor;

/// Rule type in LegalRuleML.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleType {
    /// Strict rule (always applies)
    Strict,
    /// Defeasible rule (can be overridden)
    Defeasible,
    /// Constitutive rule (defines concepts)
    Constitutive,
}

impl RuleType {
    /// Converts to LegalRuleML attribute value.
    pub fn to_lrml(&self) -> &'static str {
        match self {
            RuleType::Strict => "strict",
            RuleType::Defeasible => "defeasible",
            RuleType::Constitutive => "constitutive",
        }
    }

    /// Parses from LegalRuleML attribute.
    pub fn from_lrml(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "strict" => Some(RuleType::Strict),
            "defeasible" => Some(RuleType::Defeasible),
            "constitutive" => Some(RuleType::Constitutive),
            _ => None,
        }
    }
}

/// LegalRuleML format importer.
pub struct LegalRuleMLImporter {
    /// Whether to preserve metadata
    preserve_metadata: bool,
}

impl LegalRuleMLImporter {
    /// Creates a new LegalRuleML importer.
    pub fn new() -> Self {
        Self {
            preserve_metadata: true,
        }
    }

    /// Sets whether to preserve metadata.
    pub fn with_metadata(mut self, preserve: bool) -> Self {
        self.preserve_metadata = preserve;
        self
    }

    /// Parses a LegalRuleML document and extracts statutes.
    fn parse_document(&self, source: &str, report: &mut ConversionReport) -> Vec<Statute> {
        let mut statutes = Vec::new();
        let mut reader = Reader::from_str(source);
        reader.config_mut().trim_text(true);

        let mut current_id = String::new();
        let mut current_name = String::new();
        let mut current_premises: Vec<String> = Vec::new();
        let mut current_conclusion = String::new();
        let mut in_rule = false;
        let mut in_name = false;
        let mut in_premise = false;
        let mut in_conclusion = false;
        let mut premise_text = String::new();
        let mut conclusion_text = String::new();

        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) => {
                    let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    match name.as_str() {
                        "Rule" | "LegalRule" => {
                            in_rule = true;
                            // Extract rule ID
                            for attr in e.attributes().flatten() {
                                if attr.key.as_ref() == b"key" || attr.key.as_ref() == b"id" {
                                    current_id = String::from_utf8_lossy(&attr.value).to_string();
                                }
                            }
                        }
                        "Name" => {
                            in_name = true;
                        }
                        "Premise" | "if" => {
                            in_premise = true;
                            premise_text.clear();
                        }
                        "Conclusion" | "then" => {
                            in_conclusion = true;
                            conclusion_text.clear();
                        }
                        _ => {}
                    }
                }
                Ok(Event::End(ref e)) => {
                    let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    match name.as_str() {
                        "Rule" | "LegalRule" => {
                            if in_rule {
                                // Create statute from collected data
                                let id = if current_id.is_empty() {
                                    format!("lrml-{}", statutes.len() + 1)
                                } else {
                                    current_id.to_lowercase().replace(['_', ' ', ':'], "-")
                                };

                                let title = if current_name.is_empty() {
                                    format!("Rule {}", statutes.len() + 1)
                                } else {
                                    current_name.clone()
                                };

                                let effect_desc = if current_conclusion.is_empty() {
                                    title.clone()
                                } else {
                                    current_conclusion.clone()
                                };

                                let mut statute = Statute::new(
                                    &id,
                                    &title,
                                    Effect::new(EffectType::Grant, &effect_desc),
                                );

                                // Add premises as conditions
                                for premise in &current_premises {
                                    if let Some(cond) = Self::parse_premise(premise, report) {
                                        statute.preconditions.push(cond);
                                    }
                                }

                                statutes.push(statute);

                                // Reset state
                                current_id.clear();
                                current_name.clear();
                                current_premises.clear();
                                current_conclusion.clear();
                                in_rule = false;
                            }
                        }
                        "Name" => {
                            in_name = false;
                        }
                        "Premise" => {
                            if !premise_text.trim().is_empty() {
                                current_premises.push(premise_text.clone());
                            }
                            in_premise = false;
                        }
                        "if" => {
                            // Container element, no action needed
                        }
                        "Conclusion" | "then" => {
                            current_conclusion = conclusion_text.clone();
                            in_conclusion = false;
                        }
                        _ => {}
                    }
                }
                Ok(Event::Text(e)) => {
                    let text = e.unescape().unwrap_or_default().trim().to_string();
                    if !text.is_empty() {
                        if in_name {
                            current_name = text;
                        } else if in_premise {
                            if !premise_text.is_empty() {
                                premise_text.push(' ');
                            }
                            premise_text.push_str(&text);
                        } else if in_conclusion {
                            if !conclusion_text.is_empty() {
                                conclusion_text.push(' ');
                            }
                            conclusion_text.push_str(&text);
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    report.add_warning(format!("XML parse error: {}", e));
                    break;
                }
                _ => {}
            }
        }

        statutes
    }

    /// Parses a premise text into a condition.
    fn parse_premise(premise: &str, report: &mut ConversionReport) -> Option<Condition> {
        let premise = premise.trim();

        // Try to parse common patterns
        // Pattern: "age >= 18"
        if let Some(pos) = premise.find(">=") {
            let field = premise[..pos].trim();
            let value = premise[pos + 2..].trim();
            if field.to_lowercase().contains("age") {
                if let Ok(v) = value.parse::<u32>() {
                    return Some(Condition::Age {
                        operator: ComparisonOp::GreaterOrEqual,
                        value: v,
                    });
                }
            }
        }

        // Pattern: "income < 50000"
        if let Some(pos) = premise.find('<') {
            let field = premise[..pos].trim();
            let value = premise[pos + 1..].trim();
            if field.to_lowercase().contains("income") {
                if let Ok(v) = value.parse::<u64>() {
                    return Some(Condition::Income {
                        operator: ComparisonOp::LessThan,
                        value: v,
                    });
                }
            }
        }

        // Generic attribute equals
        if let Some(pos) = premise.find('=') {
            let key = premise[..pos].trim();
            let value = premise[pos + 1..].trim();
            return Some(Condition::AttributeEquals {
                key: key.to_string(),
                value: value.to_string(),
            });
        }

        report.add_warning(format!("Could not parse premise: {}", premise));
        None
    }
}

impl Default for LegalRuleMLImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for LegalRuleMLImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::LegalRuleML
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::LegalRuleML, LegalFormat::Legalis);

        let statutes = self.parse_document(source, &mut report);

        if statutes.is_empty() {
            return Err(InteropError::ParseError(
                "No valid LegalRuleML rules found".to_string(),
            ));
        }

        // Note unsupported features
        if source.contains("<override") || source.contains("<overrides") {
            report.add_unsupported("LegalRuleML override relationships");
        }
        if source.contains("<authority") {
            report.add_unsupported("LegalRuleML authority metadata");
        }
        if source.contains("<temporal") {
            report.add_unsupported("LegalRuleML temporal validity");
        }

        report.statutes_converted = statutes.len();
        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        source.contains("<LegalRuleML")
            || source.contains("<legalruleml")
            || (source.contains("<Rule") && source.contains("<Premise"))
    }
}

/// LegalRuleML format exporter.
pub struct LegalRuleMLExporter {
    /// Default rule type
    default_rule_type: RuleType,
    /// Include metadata
    include_metadata: bool,
}

impl LegalRuleMLExporter {
    /// Creates a new LegalRuleML exporter.
    pub fn new() -> Self {
        Self {
            default_rule_type: RuleType::Strict,
            include_metadata: true,
        }
    }

    /// Sets the default rule type.
    pub fn with_rule_type(mut self, rule_type: RuleType) -> Self {
        self.default_rule_type = rule_type;
        self
    }

    /// Sets whether to include metadata.
    pub fn with_metadata(mut self, include: bool) -> Self {
        self.include_metadata = include;
        self
    }

    /// Converts a condition to LegalRuleML premise text.
    fn condition_to_premise(condition: &Condition, report: &mut ConversionReport) -> String {
        match condition {
            Condition::Age { operator, value } => {
                format!("age {} {}", Self::operator_symbol(operator), value)
            }
            Condition::Income { operator, value } => {
                format!("income {} {}", Self::operator_symbol(operator), value)
            }
            Condition::And(left, right) => {
                let l = Self::condition_to_premise(left, report);
                let r = Self::condition_to_premise(right, report);
                format!("({}) AND ({})", l, r)
            }
            Condition::Or(left, right) => {
                let l = Self::condition_to_premise(left, report);
                let r = Self::condition_to_premise(right, report);
                format!("({}) OR ({})", l, r)
            }
            Condition::Not(inner) => {
                let i = Self::condition_to_premise(inner, report);
                format!("NOT ({})", i)
            }
            Condition::AttributeEquals { key, value } => {
                format!("{} = {}", key, value)
            }
            Condition::HasAttribute { key } => {
                format!("{} exists", key)
            }
            Condition::ResidencyDuration { operator, months } => {
                format!(
                    "residency_months {} {}",
                    Self::operator_symbol(operator),
                    months
                )
            }
            _ => {
                report.add_unsupported(format!("Condition type: {:?}", condition));
                "true".to_string()
            }
        }
    }

    fn operator_symbol(op: &ComparisonOp) -> &'static str {
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

impl Default for LegalRuleMLExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for LegalRuleMLExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::LegalRuleML
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::LegalRuleML);
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        // XML declaration
        writer
            .write_event(Event::Decl(quick_xml::events::BytesDecl::new(
                "1.0",
                Some("UTF-8"),
                None,
            )))
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;

        // Root element
        let mut root = BytesStart::new("legalruleml");
        root.push_attribute(("xmlns", "http://docs.oasis-open.org/legalruleml/ns/v1.0/"));
        root.push_attribute(("xmlns:ruleml", "http://ruleml.org/spec"));
        writer
            .write_event(Event::Start(root))
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;

        // Statements section
        writer
            .write_event(Event::Start(BytesStart::new("Statements")))
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;

        // Convert each statute to a rule
        for statute in statutes {
            let mut rule = BytesStart::new("LegalRule");
            rule.push_attribute(("key", statute.id.as_str()));
            rule.push_attribute(("type", self.default_rule_type.to_lrml()));
            writer
                .write_event(Event::Start(rule))
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;

            // Rule name
            writer
                .write_event(Event::Start(BytesStart::new("Name")))
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;
            writer
                .write_event(Event::Text(BytesText::new(&statute.title)))
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;
            writer
                .write_event(Event::End(BytesEnd::new("Name")))
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;

            // Premises (conditions)
            if !statute.preconditions.is_empty() {
                writer
                    .write_event(Event::Start(BytesStart::new("if")))
                    .map_err(|e| InteropError::SerializationError(e.to_string()))?;

                for condition in &statute.preconditions {
                    writer
                        .write_event(Event::Start(BytesStart::new("Premise")))
                        .map_err(|e| InteropError::SerializationError(e.to_string()))?;

                    let premise_text = Self::condition_to_premise(condition, &mut report);
                    writer
                        .write_event(Event::Text(BytesText::new(&premise_text)))
                        .map_err(|e| InteropError::SerializationError(e.to_string()))?;

                    writer
                        .write_event(Event::End(BytesEnd::new("Premise")))
                        .map_err(|e| InteropError::SerializationError(e.to_string()))?;
                }

                writer
                    .write_event(Event::End(BytesEnd::new("if")))
                    .map_err(|e| InteropError::SerializationError(e.to_string()))?;
            }

            // Conclusion (effect)
            writer
                .write_event(Event::Start(BytesStart::new("then")))
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;

            writer
                .write_event(Event::Start(BytesStart::new("Conclusion")))
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;

            let conclusion_text = format!(
                "{}: {}",
                statute.effect.effect_type, statute.effect.description
            );
            writer
                .write_event(Event::Text(BytesText::new(&conclusion_text)))
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;

            writer
                .write_event(Event::End(BytesEnd::new("Conclusion")))
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;

            writer
                .write_event(Event::End(BytesEnd::new("then")))
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;

            // Discretion as metadata
            if let Some(ref discretion) = statute.discretion_logic {
                report.add_warning(format!(
                    "Discretion '{}' added as remark element",
                    discretion
                ));

                writer
                    .write_event(Event::Start(BytesStart::new("Remark")))
                    .map_err(|e| InteropError::SerializationError(e.to_string()))?;
                writer
                    .write_event(Event::Text(BytesText::new(&format!(
                        "DISCRETION: {}",
                        discretion
                    ))))
                    .map_err(|e| InteropError::SerializationError(e.to_string()))?;
                writer
                    .write_event(Event::End(BytesEnd::new("Remark")))
                    .map_err(|e| InteropError::SerializationError(e.to_string()))?;
            }

            writer
                .write_event(Event::End(BytesEnd::new("LegalRule")))
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;

            report.statutes_converted += 1;
        }

        // Close Statements
        writer
            .write_event(Event::End(BytesEnd::new("Statements")))
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;

        // Close root
        writer
            .write_event(Event::End(BytesEnd::new("legalruleml")))
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;

        let output = String::from_utf8(writer.into_inner().into_inner())
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;

        Ok((output, report))
    }

    fn can_represent(&self, statute: &Statute) -> Vec<String> {
        let mut issues = Vec::new();

        if statute.discretion_logic.is_some() {
            issues.push("Discretionary logic will be added as remark element".to_string());
        }

        // Check for complex conditions
        for condition in &statute.preconditions {
            if matches!(condition, Condition::Custom { .. }) {
                issues
                    .push("Custom conditions may lose semantic meaning in LegalRuleML".to_string());
                break;
            }
        }

        issues
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_type_conversion() {
        assert_eq!(RuleType::Strict.to_lrml(), "strict");
        assert_eq!(RuleType::Defeasible.to_lrml(), "defeasible");
        assert_eq!(RuleType::from_lrml("strict"), Some(RuleType::Strict));
        assert_eq!(
            RuleType::from_lrml("defeasible"),
            Some(RuleType::Defeasible)
        );
    }

    #[test]
    fn test_legalruleml_importer_validate() {
        let importer = LegalRuleMLImporter::new();
        assert!(importer.validate("<LegalRuleML><Rule></Rule></LegalRuleML>"));
        assert!(importer.validate("<Rule><Premise>test</Premise></Rule>"));
        assert!(!importer.validate("STATUTE foo: \"bar\" {}"));
    }

    #[test]
    fn test_legalruleml_import_basic() {
        let importer = LegalRuleMLImporter::new();
        let source = r#"
        <legalruleml xmlns="http://docs.oasis-open.org/legalruleml/ns/v1.0/">
            <Statements>
                <LegalRule key="rule1" type="strict">
                    <Name>Adult Rights Rule</Name>
                    <if>
                        <Premise>age >= 18</Premise>
                    </if>
                    <then>
                        <Conclusion>Person has full legal capacity</Conclusion>
                    </then>
                </LegalRule>
            </Statements>
        </legalruleml>
        "#;

        let (statutes, report) = importer.import(source).unwrap();
        assert_eq!(statutes.len(), 1);
        assert_eq!(report.statutes_converted, 1);
        assert_eq!(statutes[0].id, "rule1");
        assert_eq!(statutes[0].title, "Adult Rights Rule");
    }

    #[test]
    fn test_legalruleml_exporter_basic() {
        let exporter = LegalRuleMLExporter::new();
        let statute = Statute::new(
            "voting-rights",
            "Voting Rights Rule",
            Effect::new(EffectType::Grant, "Right to vote"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let (output, report) = exporter.export(&[statute]).unwrap();

        assert!(output.contains("<legalruleml"));
        assert!(output.contains("Voting Rights Rule"));
        assert!(output.contains("age"));
        assert!(output.contains("18"));
        assert_eq!(report.statutes_converted, 1);
    }

    #[test]
    fn test_legalruleml_roundtrip() {
        let exporter = LegalRuleMLExporter::new();
        let importer = LegalRuleMLImporter::new();

        let statute = Statute::new(
            "test-rule",
            "Test Legal Rule",
            Effect::new(EffectType::Grant, "Test effect"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 21,
        });

        // Export
        let (xml_output, _) = exporter.export(&[statute]).unwrap();

        // Import back
        let (imported, _) = importer.import(&xml_output).unwrap();

        assert_eq!(imported.len(), 1);
        assert_eq!(imported[0].title, "Test Legal Rule");
        assert_eq!(imported[0].preconditions.len(), 1);
    }

    #[test]
    fn test_legalruleml_multiple_premises() {
        let exporter = LegalRuleMLExporter::new();
        let statute = Statute::new(
            "multi-cond",
            "Multi Condition Rule",
            Effect::new(EffectType::Grant, "Grant"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
        .with_precondition(Condition::Income {
            operator: ComparisonOp::LessThan,
            value: 50000,
        });

        let (output, report) = exporter.export(&[statute]).unwrap();
        assert_eq!(report.statutes_converted, 1);
        assert!(output.contains("<Premise>age"));
        assert!(output.contains("<Premise>income"));
    }
}
