//! CMMN (Case Management Model and Notation) import/export - OMG standard for case management.

use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::{Effect, EffectType, Statute};

pub struct CmmnImporter;
impl CmmnImporter {
    pub fn new() -> Self {
        Self
    }
}
impl Default for CmmnImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for CmmnImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Cmmn
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Cmmn, LegalFormat::Legalis);
        let mut statutes = Vec::new();

        if !source.trim_start().starts_with('<')
            || (!source.contains("cmmn") && !source.contains("case"))
        {
            return Err(InteropError::ParseError(
                "Not a valid CMMN document".to_string(),
            ));
        }

        let case_pattern =
            regex_lite::Regex::new(r#"<case[^>]*id="([^"]*)"[^>]*name="([^"]*)"[^>]*>"#)
                .map_err(|e| InteropError::ParseError(e.to_string()))?;

        for cap in case_pattern.captures_iter(source) {
            let id = cap.get(1).map(|m| m.as_str()).unwrap_or("unknown");
            let name = cap.get(2).map(|m| m.as_str()).unwrap_or("Case");
            let effect = Effect::new(EffectType::Grant, format!("manage_{}", id));
            let statute = Statute::new(id, name, effect);
            statutes.push(statute);
        }

        if statutes.is_empty() {
            let statute = Statute::new(
                "cmmn_case",
                "CMMN Case",
                Effect::new(EffectType::Grant, "case_management"),
            );
            statutes.push(statute);
            report.add_warning("No explicit CMMN cases found, created default".to_string());
        }

        report.statutes_converted = statutes.len();
        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        source.trim_start().starts_with('<') && (source.contains("cmmn") || source.contains("case"))
    }
}

pub struct CmmnExporter;
impl CmmnExporter {
    pub fn new() -> Self {
        Self
    }
}
impl Default for CmmnExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for CmmnExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Cmmn
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::Cmmn);
        let mut output = String::from(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<definitions xmlns="http://www.omg.org/spec/CMMN/20151109/MODEL" id="Definitions_1">
"#,
        );

        for statute in statutes {
            output.push_str(&format!(
                r#"  <case id="{}" name="{}">
    <casePlanModel/>
  </case>
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
    fn test_cmmn_import_export() {
        let importer = CmmnImporter::new();
        let exporter = CmmnExporter::new();

        let cmmn = r#"<definitions><case id="test" name="Test"/></definitions>"#;
        let (statutes, _) = importer.import(cmmn).unwrap();
        assert_eq!(statutes.len(), 1);

        let (output, _) = exporter.export(&statutes).unwrap();
        assert!(output.contains("case"));
    }
}
