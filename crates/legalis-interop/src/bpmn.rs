//! BPMN (Business Process Model and Notation) import/export.
//!
//! BPMN is an OMG standard for business process modeling.
//! This module provides conversion between BPMN process models and Legalis statutes.

use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::{Condition, Effect, EffectType, Statute};

#[cfg(test)]
use legalis_core::ComparisonOp;

/// BPMN format importer.
pub struct BpmnImporter;

impl BpmnImporter {
    /// Creates a new BPMN importer.
    pub fn new() -> Self {
        Self
    }
}

impl Default for BpmnImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for BpmnImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Bpmn
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Bpmn, LegalFormat::Legalis);
        let mut statutes = Vec::new();

        // Parse BPMN XML
        if !source.trim_start().starts_with('<') {
            return Err(InteropError::ParseError(
                "BPMN must be XML format".to_string(),
            ));
        }

        // Check for BPMN namespace
        if !source.contains("bpmn") && !source.contains("definitions") {
            return Err(InteropError::ParseError(
                "Not a valid BPMN document".to_string(),
            ));
        }

        // Extract process definitions
        // Look for <process> elements
        let process_pattern =
            regex_lite::Regex::new(r#"<process[^>]*id="([^"]*)"[^>]*name="([^"]*)"[^>]*>"#)
                .map_err(|e| InteropError::ParseError(e.to_string()))?;

        for cap in process_pattern.captures_iter(source) {
            let id = cap.get(1).map(|m| m.as_str()).unwrap_or("unknown");
            let name = cap.get(2).map(|m| m.as_str()).unwrap_or("Process");

            // Create a statute for each process
            let effect = Effect::new(EffectType::Grant, format!("execute_{}", id));
            let mut statute = Statute::new(id, name, effect);

            // Look for start events, tasks, and gateways within this process
            // This is a simplified extraction - a full parser would use proper XML parsing
            if source.contains("startEvent") {
                report.add_warning("BPMN start events mapped to statute preconditions".to_string());
            }

            if source.contains("userTask") || source.contains("serviceTask") {
                statute = statute.with_precondition(Condition::Custom {
                    description: "task_available".to_string(),
                });
            }

            if source.contains("exclusiveGateway") {
                report.add_warning("BPMN exclusive gateways require manual review".to_string());
            }

            statutes.push(statute);
        }

        if statutes.is_empty() {
            // Create a default statute if no processes found
            let effect = Effect::new(EffectType::Grant, "process_execution");
            let statute = Statute::new("bpmn_process", "BPMN Process", effect);
            statutes.push(statute);
            report.add_warning("No explicit BPMN processes found, created default".to_string());
        }

        report.statutes_converted = statutes.len();
        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        source.trim_start().starts_with('<')
            && (source.contains("bpmn") || source.contains("definitions"))
    }
}

/// BPMN format exporter.
pub struct BpmnExporter;

impl BpmnExporter {
    /// Creates a new BPMN exporter.
    pub fn new() -> Self {
        Self
    }
}

impl Default for BpmnExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for BpmnExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Bpmn
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::Bpmn);
        let mut output = String::new();

        // BPMN XML header
        output.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
        output.push('\n');
        output.push_str(r#"<definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL""#);
        output.push('\n');
        output.push_str(r#"             xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance""#);
        output.push('\n');
        output.push_str(r#"             id="Definitions_1">"#);
        output.push('\n');

        // Convert each statute to a BPMN process
        for statute in statutes {
            output.push_str(&format!(
                r#"  <process id="{}" name="{}" isExecutable="true">"#,
                statute.id, statute.title
            ));
            output.push('\n');

            // Add start event
            output.push_str(&format!(
                r#"    <startEvent id="start_{}" name="Start"/>"#,
                statute.id
            ));
            output.push('\n');

            // Add a task for the effect
            let task_id = format!("task_{}", statute.id);
            output.push_str(&format!(
                r#"    <userTask id="{}" name="{}"/>"#,
                task_id, statute.effect.description
            ));
            output.push('\n');

            // Add end event
            output.push_str(&format!(
                r#"    <endEvent id="end_{}" name="End"/>"#,
                statute.id
            ));
            output.push('\n');

            // Add sequence flows
            output.push_str(&format!(
                r#"    <sequenceFlow id="flow1_{}" sourceRef="start_{}" targetRef="{}"/>"#,
                statute.id, statute.id, task_id
            ));
            output.push('\n');

            output.push_str(&format!(
                r#"    <sequenceFlow id="flow2_{}" sourceRef="{}" targetRef="end_{}"/>"#,
                statute.id, task_id, statute.id
            ));
            output.push('\n');

            output.push_str("  </process>\n");

            if !statute.preconditions.is_empty() {
                report.add_warning(format!(
                    "Statute {} has {} preconditions that were simplified in BPMN",
                    statute.id,
                    statute.preconditions.len()
                ));
            }
        }

        output.push_str("</definitions>\n");

        report.statutes_converted = statutes.len();
        Ok((output, report))
    }

    fn can_represent(&self, statute: &Statute) -> Vec<String> {
        let mut limitations = Vec::new();

        if statute.preconditions.len() > 3 {
            limitations.push(format!(
                "Complex preconditions ({}) may require gateways",
                statute.preconditions.len()
            ));
        }

        // Note: metadata is handled through the statute's general metadata mechanism
        // No specific field check needed here

        limitations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bpmn_importer_validate() {
        let importer = BpmnImporter::new();

        let valid_bpmn = r#"<?xml version="1.0"?>
<definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL">
  <process id="proc1" name="Test Process"/>
</definitions>"#;

        assert!(importer.validate(valid_bpmn));
        assert!(!importer.validate("not xml"));
    }

    #[test]
    fn test_bpmn_import() {
        let importer = BpmnImporter::new();

        let bpmn = r#"<?xml version="1.0"?>
<definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL">
  <process id="approval_process" name="Document Approval">
    <startEvent id="start"/>
    <userTask id="review" name="Review Document"/>
    <endEvent id="end"/>
  </process>
</definitions>"#;

        let (statutes, report) = importer.import(bpmn).unwrap();

        assert_eq!(report.source_format, Some(LegalFormat::Bpmn));
        assert!(!statutes.is_empty());
        assert_eq!(statutes[0].id, "approval_process");
        assert_eq!(statutes[0].title, "Document Approval");
    }

    #[test]
    fn test_bpmn_export() {
        let exporter = BpmnExporter::new();

        let statute = Statute::new(
            "test_process",
            "Test Process",
            Effect::new(EffectType::Grant, "execute"),
        );

        let (output, report) = exporter.export(&[statute]).unwrap();

        assert!(output.contains("<?xml"));
        assert!(output.contains("definitions"));
        assert!(output.contains("process"));
        assert!(output.contains(r#"id="test_process""#));
        assert!(output.contains(r#"name="Test Process""#));
        assert!(output.contains("startEvent"));
        assert!(output.contains("userTask"));
        assert!(output.contains("endEvent"));
        assert_eq!(report.statutes_converted, 1);
    }

    #[test]
    fn test_bpmn_roundtrip() {
        let exporter = BpmnExporter::new();
        let importer = BpmnImporter::new();

        let statute = Statute::new(
            "roundtrip_test",
            "Roundtrip Test",
            Effect::new(EffectType::Grant, "process"),
        );

        let (bpmn_output, _) = exporter.export(&[statute]).unwrap();
        let (imported_statutes, _) = importer.import(&bpmn_output).unwrap();

        assert_eq!(imported_statutes.len(), 1);
        assert_eq!(imported_statutes[0].id, "roundtrip_test");
    }

    #[test]
    fn test_bpmn_can_represent() {
        let exporter = BpmnExporter::new();

        let simple_statute = Statute::new("simple", "Simple", Effect::new(EffectType::Grant, "x"));
        let limitations = exporter.can_represent(&simple_statute);
        assert!(limitations.is_empty() || limitations.len() <= 1);

        let complex_statute =
            Statute::new("complex", "Complex", Effect::new(EffectType::Grant, "x"))
                .with_precondition(Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 18,
                })
                .with_precondition(Condition::Custom {
                    description: "cond1".to_string(),
                })
                .with_precondition(Condition::Custom {
                    description: "cond2".to_string(),
                })
                .with_precondition(Condition::Custom {
                    description: "cond3".to_string(),
                });

        let limitations = exporter.can_represent(&complex_statute);
        assert!(!limitations.is_empty());
    }
}
