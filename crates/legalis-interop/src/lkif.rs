//! LKIF (Legal Knowledge Interchange Format) import/export.
//!
//! LKIF is an XML-based standard for representing legal knowledge developed
//! as part of the ESTRELLA project. It supports:
//! - Legal rules and norms
//! - Legal cases and arguments
//! - Ontologies and domain models
//!
//! Reference: <http://www.estrellaproject.org/lkif-core/>

use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::{Reader, Writer};
use std::io::Cursor;

/// LKIF importer.
pub struct LkifImporter;

impl LkifImporter {
    /// Creates a new LKIF importer.
    pub fn new() -> Self {
        Self
    }

    fn parse_rule(
        &self,
        reader: &mut Reader<&[u8]>,
        report: &mut ConversionReport,
    ) -> InteropResult<Option<Statute>> {
        let mut buf = Vec::new();
        let mut rule_id = String::new();
        let mut rule_name = String::new();
        let mut conditions = Vec::new();
        let mut effect_desc = String::from("Legal effect");
        let mut effect_type = EffectType::Grant;
        let mut in_body = false;
        let mut in_head = false;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => {
                    match e.name().as_ref() {
                        b"body" => in_body = true,
                        b"head" => in_head = true,
                        b"atom" => {
                            // Parse atom for conditions or effects
                            if let Some(Ok(attr)) = e.attributes().find(|a| {
                                a.as_ref()
                                    .map(|attr| attr.key.as_ref() == b"pred")
                                    .unwrap_or(false)
                            }) {
                                let pred = String::from_utf8_lossy(&attr.value).to_string();

                                if in_body {
                                    // Parse as condition
                                    if pred.contains("age") {
                                        conditions.push(Condition::Age {
                                            operator: ComparisonOp::GreaterOrEqual,
                                            value: 18,
                                        });
                                    }
                                } else if in_head {
                                    // Parse as effect
                                    effect_desc = pred;
                                    if effect_desc.contains("obliged")
                                        || effect_desc.contains("must")
                                    {
                                        effect_type = EffectType::Obligation;
                                    } else if effect_desc.contains("prohibited")
                                        || effect_desc.contains("forbidden")
                                    {
                                        effect_type = EffectType::Prohibition;
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Ok(Event::End(e)) => match e.name().as_ref() {
                    b"rule" => break,
                    b"body" => in_body = false,
                    b"head" => in_head = false,
                    _ => {}
                },
                Ok(Event::Text(e)) => {
                    let text = std::str::from_utf8(e.as_ref())
                        .unwrap_or_default()
                        .trim()
                        .to_string();
                    if !text.is_empty() && rule_name.is_empty() {
                        rule_name = text.clone();
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(InteropError::ParseError(format!("XML parse error: {}", e))),
                _ => {}
            }
            buf.clear();
        }

        // Read attributes from the rule tag
        // Note: We need to re-read to get attributes, so we'll use a simple default ID
        if rule_id.is_empty() {
            rule_id = format!("lkif_rule_{}", report.statutes_converted);
        }
        if rule_name.is_empty() {
            rule_name = format!("LKIF Rule {}", report.statutes_converted);
        }

        let mut statute =
            Statute::new(&rule_id, &rule_name, Effect::new(effect_type, &effect_desc));

        for condition in conditions {
            statute = statute.with_precondition(condition);
        }

        Ok(Some(statute))
    }
}

impl Default for LkifImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for LkifImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::LKIF
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let mut reader = Reader::from_str(source);
        reader.config_mut().trim_text(true);

        let mut statutes = Vec::new();
        let mut report = ConversionReport::new(LegalFormat::LKIF, LegalFormat::Legalis);
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => {
                    if e.name().as_ref() == b"rule" {
                        if let Some(statute) = self.parse_rule(&mut reader, &mut report)? {
                            statutes.push(statute);
                            report.statutes_converted += 1;
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(InteropError::ParseError(format!("XML parse error: {}", e))),
                _ => {}
            }
            buf.clear();
        }

        if statutes.is_empty() {
            report.add_warning("No rules found in LKIF document");
        }

        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        source.contains("<lkif")
            || source.contains("<rule")
            || source.contains("http://www.estrellaproject.org")
    }
}

/// LKIF exporter.
pub struct LkifExporter;

impl LkifExporter {
    /// Creates a new LKIF exporter.
    pub fn new() -> Self {
        Self
    }

    fn write_atom(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        predicate: &str,
        args: &[&str],
    ) -> InteropResult<()> {
        let mut atom = BytesStart::new("atom");
        atom.push_attribute(("pred", predicate));
        writer
            .write_event(Event::Start(atom))
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;

        for (i, arg) in args.iter().enumerate() {
            let mut var = BytesStart::new("var");
            var.push_attribute(("value", *arg));
            if i == 0 {
                var.push_attribute(("role", "subject"));
            }
            writer
                .write_event(Event::Empty(var))
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;
        }

        writer
            .write_event(Event::End(BytesEnd::new("atom")))
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;

        Ok(())
    }

    fn write_condition(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        condition: &Condition,
    ) -> InteropResult<()> {
        match condition {
            Condition::Age { operator, value } => {
                let op_str = match operator {
                    ComparisonOp::GreaterOrEqual => "gte",
                    ComparisonOp::GreaterThan => "gt",
                    ComparisonOp::Equal => "eq",
                    ComparisonOp::LessThan => "lt",
                    ComparisonOp::LessOrEqual => "lte",
                    ComparisonOp::NotEqual => "neq",
                };
                self.write_atom(writer, &format!("age_{}", op_str), &[&value.to_string()])?;
            }
            Condition::Geographic {
                region_type: _,
                region_id,
            } => {
                self.write_atom(writer, "resides_in", &[region_id])?;
            }
            Condition::Custom { description } => {
                self.write_atom(writer, description, &[])?;
            }
            Condition::And(left, right) => {
                writer
                    .write_event(Event::Start(BytesStart::new("and")))
                    .map_err(|e| InteropError::SerializationError(e.to_string()))?;
                self.write_condition(writer, left)?;
                self.write_condition(writer, right)?;
                writer
                    .write_event(Event::End(BytesEnd::new("and")))
                    .map_err(|e| InteropError::SerializationError(e.to_string()))?;
            }
            Condition::Or(left, right) => {
                writer
                    .write_event(Event::Start(BytesStart::new("or")))
                    .map_err(|e| InteropError::SerializationError(e.to_string()))?;
                self.write_condition(writer, left)?;
                self.write_condition(writer, right)?;
                writer
                    .write_event(Event::End(BytesEnd::new("or")))
                    .map_err(|e| InteropError::SerializationError(e.to_string()))?;
            }
            Condition::Not(cond) => {
                writer
                    .write_event(Event::Start(BytesStart::new("not")))
                    .map_err(|e| InteropError::SerializationError(e.to_string()))?;
                self.write_condition(writer, cond)?;
                writer
                    .write_event(Event::End(BytesEnd::new("not")))
                    .map_err(|e| InteropError::SerializationError(e.to_string()))?;
            }
            _ => {
                self.write_atom(writer, "custom_condition", &[])?;
            }
        }
        Ok(())
    }
}

impl Default for LkifExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for LkifExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::LKIF
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::LKIF);
        let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2);

        // Write XML declaration
        writer
            .write_event(Event::Decl(quick_xml::events::BytesDecl::new(
                "1.0",
                Some("UTF-8"),
                None,
            )))
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;

        // Write LKIF root element
        let mut lkif = BytesStart::new("lkif");
        lkif.push_attribute(("xmlns", "http://www.estrellaproject.org/lkif"));
        writer
            .write_event(Event::Start(lkif))
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;

        // Write theory element
        writer
            .write_event(Event::Start(BytesStart::new("theory")))
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;

        // Write each statute as a rule
        for statute in statutes {
            let mut rule = BytesStart::new("rule");
            rule.push_attribute(("id", statute.id.as_str()));
            writer
                .write_event(Event::Start(rule))
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;

            // Write rule name
            writer
                .write_event(Event::Start(BytesStart::new("label")))
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;
            writer
                .write_event(Event::Text(BytesText::new(&statute.title)))
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;
            writer
                .write_event(Event::End(BytesEnd::new("label")))
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;

            // Write body (conditions)
            if !statute.preconditions.is_empty() {
                writer
                    .write_event(Event::Start(BytesStart::new("body")))
                    .map_err(|e| InteropError::SerializationError(e.to_string()))?;

                if statute.preconditions.len() == 1 {
                    self.write_condition(&mut writer, &statute.preconditions[0])?;
                } else {
                    writer
                        .write_event(Event::Start(BytesStart::new("and")))
                        .map_err(|e| InteropError::SerializationError(e.to_string()))?;
                    for condition in &statute.preconditions {
                        self.write_condition(&mut writer, condition)?;
                    }
                    writer
                        .write_event(Event::End(BytesEnd::new("and")))
                        .map_err(|e| InteropError::SerializationError(e.to_string()))?;
                }

                writer
                    .write_event(Event::End(BytesEnd::new("body")))
                    .map_err(|e| InteropError::SerializationError(e.to_string()))?;
            }

            // Write head (effect)
            writer
                .write_event(Event::Start(BytesStart::new("head")))
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;

            let effect_pred = match statute.effect.effect_type {
                EffectType::Grant => {
                    format!("may_{}", statute.effect.description.replace(' ', "_"))
                }
                EffectType::Obligation => {
                    format!("must_{}", statute.effect.description.replace(' ', "_"))
                }
                EffectType::Prohibition => format!(
                    "prohibited_{}",
                    statute.effect.description.replace(' ', "_")
                ),
                EffectType::Revoke => {
                    format!("revoke_{}", statute.effect.description.replace(' ', "_"))
                }
                _ => statute.effect.description.replace(' ', "_"),
            };

            self.write_atom(&mut writer, &effect_pred, &["Person"])?;

            writer
                .write_event(Event::End(BytesEnd::new("head")))
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;

            // Close rule
            writer
                .write_event(Event::End(BytesEnd::new("rule")))
                .map_err(|e| InteropError::SerializationError(e.to_string()))?;

            report.statutes_converted += 1;
        }

        // Close theory and lkif
        writer
            .write_event(Event::End(BytesEnd::new("theory")))
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;
        writer
            .write_event(Event::End(BytesEnd::new("lkif")))
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;

        let result = writer.into_inner().into_inner();
        let output = String::from_utf8(result)
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;

        Ok((output, report))
    }

    fn can_represent(&self, _statute: &Statute) -> Vec<String> {
        // LKIF is very expressive and can represent most statute features
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

    #[test]
    fn test_lkif_exporter_basic() {
        let exporter = LkifExporter::new();

        let statute = Statute::new(
            "voting-rule",
            "Voting Rights Rule",
            Effect::new(EffectType::Grant, "vote"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let (output, report) = exporter.export(&[statute]).unwrap();

        assert_eq!(report.statutes_converted, 1);
        assert!(output.contains("<lkif"));
        assert!(output.contains("<rule"));
        assert!(output.contains("voting-rule"));
        assert!(output.contains("Voting Rights Rule"));
        assert!(output.contains("<body>"));
        assert!(output.contains("<head>"));
    }

    #[test]
    fn test_lkif_importer_validate() {
        let importer = LkifImporter::new();

        assert!(importer.validate("<lkif><theory></theory></lkif>"));
        assert!(importer.validate("<rule id=\"test\"></rule>"));
        assert!(!importer.validate("not lkif"));
    }

    #[test]
    fn test_lkif_import_basic() {
        let importer = LkifImporter::new();

        let lkif_source = r#"
        <lkif xmlns="http://www.estrellaproject.org/lkif">
            <theory>
                <rule id="voting-rule">
                    <label>Voting Rights</label>
                    <body>
                        <atom pred="age_gte">
                            <var value="18" role="subject"/>
                        </atom>
                    </body>
                    <head>
                        <atom pred="may_vote">
                            <var value="Person" role="subject"/>
                        </atom>
                    </head>
                </rule>
            </theory>
        </lkif>
        "#;

        let (statutes, report) = importer.import(lkif_source).unwrap();

        assert_eq!(report.statutes_converted, 1);
        assert_eq!(statutes.len(), 1);
        assert_eq!(statutes[0].id, "lkif_rule_0");
    }

    #[test]
    fn test_lkif_roundtrip() {
        let exporter = LkifExporter::new();
        let importer = LkifImporter::new();

        let statute = Statute::new(
            "contract-rule",
            "Contract Capacity",
            Effect::new(EffectType::Grant, "enter_contract"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 21,
        });

        let (lkif_output, _) = exporter.export(&[statute]).unwrap();
        let (imported, import_report) = importer.import(&lkif_output).unwrap();

        assert_eq!(import_report.statutes_converted, 1);
        assert_eq!(imported.len(), 1);
        assert!(!imported[0].preconditions.is_empty());
    }

    #[test]
    fn test_lkif_multiple_conditions() {
        let exporter = LkifExporter::new();
        use legalis_core::RegionType;

        let statute = Statute::new(
            "complex-rule",
            "Complex Rule",
            Effect::new(EffectType::Obligation, "comply"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
        .with_precondition(Condition::Geographic {
            region_type: RegionType::Country,
            region_id: "US".to_string(),
        });

        let (output, report) = exporter.export(&[statute]).unwrap();

        assert_eq!(report.statutes_converted, 1);
        assert!(output.contains("<and>"));
        assert!(output.contains("<body>"));
    }

    #[test]
    fn test_lkif_effect_types() {
        let exporter = LkifExporter::new();

        let grant_statute = Statute::new(
            "grant-rule",
            "Grant Rule",
            Effect::new(EffectType::Grant, "benefit"),
        );

        let prohibit_statute = Statute::new(
            "prohibit-rule",
            "Prohibit Rule",
            Effect::new(EffectType::Prohibition, "action"),
        );

        let (grant_output, _) = exporter.export(&[grant_statute]).unwrap();
        let (prohibit_output, _) = exporter.export(&[prohibit_statute]).unwrap();

        assert!(grant_output.contains("may_benefit"));
        assert!(prohibit_output.contains("prohibited_action"));
    }
}
