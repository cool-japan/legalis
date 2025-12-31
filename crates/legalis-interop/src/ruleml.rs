//! RuleML (Rule Markup Language) import/export.

use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::{Effect, EffectType, Statute};

pub struct RuleMLImporter;
impl RuleMLImporter {
    pub fn new() -> Self {
        Self
    }
}
impl Default for RuleMLImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for RuleMLImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::RuleML
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::RuleML, LegalFormat::Legalis);
        let mut statutes = Vec::new();

        if !source.trim_start().starts_with('<') || !source.contains("RuleML") {
            return Err(InteropError::ParseError(
                "Not a valid RuleML document".to_string(),
            ));
        }

        let rule_pattern = regex_lite::Regex::new(r#"<Implies[^>]*>"#)
            .map_err(|e| InteropError::ParseError(e.to_string()))?;

        for (idx, _) in rule_pattern.find_iter(source).enumerate() {
            let id = format!("rule_{}", idx + 1);
            let name = format!("Rule {}", idx + 1);
            let effect = Effect::new(EffectType::Grant, "apply_rule");
            let statute = Statute::new(&id, &name, effect);
            statutes.push(statute);
        }

        if statutes.is_empty() {
            let statute = Statute::new(
                "ruleml_rule",
                "RuleML Rule",
                Effect::new(EffectType::Grant, "rule"),
            );
            statutes.push(statute);
            report.add_warning("No explicit RuleML rules found, created default".to_string());
        }

        report.statutes_converted = statutes.len();
        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        source.trim_start().starts_with('<') && source.contains("RuleML")
    }
}

pub struct RuleMLExporter;
impl RuleMLExporter {
    pub fn new() -> Self {
        Self
    }
}
impl Default for RuleMLExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for RuleMLExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::RuleML
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::RuleML);
        let mut output = String::from(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<RuleML xmlns="http://ruleml.org/spec">
"#,
        );

        for statute in statutes {
            output.push_str(&format!(
                r#"  <Implies>
    <head><Atom><Rel>{}</Rel></Atom></head>
    <body><Atom><Rel>precondition</Rel></Atom></body>
  </Implies>
"#,
                statute.title
            ));
        }

        output.push_str("</RuleML>\n");
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
    fn test_ruleml_import_export() {
        let importer = RuleMLImporter::new();
        let exporter = RuleMLExporter::new();

        let ruleml = r#"<RuleML><Implies></Implies></RuleML>"#;
        let (statutes, _) = importer.import(ruleml).unwrap();
        assert_eq!(statutes.len(), 1);

        let (output, _) = exporter.export(&statutes).unwrap();
        assert!(output.contains("RuleML"));
    }
}
