//! DMN (Decision Model and Notation) import/export - OMG standard for decision modeling.

use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::{Effect, EffectType, Statute};

pub struct DmnImporter;
impl DmnImporter {
    pub fn new() -> Self {
        Self
    }
}
impl Default for DmnImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for DmnImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Dmn
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Dmn, LegalFormat::Legalis);
        let mut statutes = Vec::new();

        if !source.trim_start().starts_with('<')
            || (!source.contains("dmn") && !source.contains("decision"))
        {
            return Err(InteropError::ParseError(
                "Not a valid DMN document".to_string(),
            ));
        }

        let decision_pattern =
            regex_lite::Regex::new(r#"<decision[^>]*id="([^"]*)"[^>]*name="([^"]*)"[^>]*>"#)
                .map_err(|e| InteropError::ParseError(e.to_string()))?;

        for cap in decision_pattern.captures_iter(source) {
            let id = cap.get(1).map(|m| m.as_str()).unwrap_or("unknown");
            let name = cap.get(2).map(|m| m.as_str()).unwrap_or("Decision");
            let effect = Effect::new(EffectType::Grant, format!("decide_{}", id));
            let statute = Statute::new(id, name, effect);
            statutes.push(statute);
        }

        if statutes.is_empty() {
            let statute = Statute::new(
                "dmn_decision",
                "DMN Decision",
                Effect::new(EffectType::Grant, "decision"),
            );
            statutes.push(statute);
            report.add_warning("No explicit DMN decisions found, created default".to_string());
        }

        report.statutes_converted = statutes.len();
        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        source.trim_start().starts_with('<')
            && (source.contains("dmn") || source.contains("decision"))
    }
}

pub struct DmnExporter;
impl DmnExporter {
    pub fn new() -> Self {
        Self
    }
}
impl Default for DmnExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for DmnExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Dmn
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::Dmn);
        let mut output = String::from(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<definitions xmlns="http://www.omg.org/spec/DMN/20151101/dmn.xsd" id="Definitions_1">
"#,
        );

        for statute in statutes {
            output.push_str(&format!(
                r#"  <decision id="{}" name="{}">
    <decisionTable>
      <output name="result"/>
    </decisionTable>
  </decision>
"#,
                statute.id, statute.title
            ));
        }

        output.push_str("</definitions>\n");
        report.statutes_converted = statutes.len();
        Ok((output, report))
    }

    fn can_represent(&self, _statute: &Statute) -> Vec<String> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dmn_import_export() {
        let importer = DmnImporter::new();
        let exporter = DmnExporter::new();

        let dmn = r#"<definitions><decision id="test" name="Test"/></definitions>"#;
        let (statutes, _) = importer.import(dmn).unwrap();
        assert_eq!(statutes.len(), 1);

        let (output, _) = exporter.export(&statutes).unwrap();
        assert!(output.contains("decision"));
    }
}
