//! SBVR (Semantics of Business Vocabulary and Business Rules) import/export - OMG standard.

use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::{Effect, EffectType, Statute};

pub struct SbvrImporter;
impl SbvrImporter {
    pub fn new() -> Self {
        Self
    }
}
impl Default for SbvrImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for SbvrImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Sbvr
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Sbvr, LegalFormat::Legalis);
        let mut statutes = Vec::new();

        // SBVR can be text or XML based
        let is_xml = source.trim_start().starts_with('<');

        if is_xml && !source.contains("sbvr") {
            return Err(InteropError::ParseError(
                "Not a valid SBVR document".to_string(),
            ));
        }

        // Parse SBVR rules (simplified - looks for "It is obligatory that" or similar patterns)
        let rule_patterns = [
            regex_lite::Regex::new(r"It is obligatory that ([^\n.]+)").ok(),
            regex_lite::Regex::new(r"It is necessary that ([^\n.]+)").ok(),
            regex_lite::Regex::new(r"<rule[^>]*>([^<]+)</rule>").ok(),
        ];

        for (idx, pattern) in rule_patterns.iter().flatten().enumerate() {
            for (rule_idx, cap) in pattern.captures_iter(source).enumerate() {
                let rule_text = cap.get(1).map(|m| m.as_str()).unwrap_or("rule");
                let id = format!("sbvr_rule_{}_{}", idx, rule_idx + 1);
                let name = rule_text.chars().take(50).collect::<String>();
                let effect = Effect::new(EffectType::Grant, rule_text);
                let statute = Statute::new(&id, &name, effect);
                statutes.push(statute);
            }
        }

        if statutes.is_empty() {
            let statute = Statute::new(
                "sbvr_rule",
                "SBVR Business Rule",
                Effect::new(EffectType::Grant, "business_rule"),
            );
            statutes.push(statute);
            report.add_warning("No explicit SBVR rules found, created default".to_string());
        }

        report.statutes_converted = statutes.len();
        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        source.contains("sbvr")
            || source.contains("It is obligatory that")
            || source.contains("It is necessary that")
    }
}

pub struct SbvrExporter;
impl SbvrExporter {
    pub fn new() -> Self {
        Self
    }
}
impl Default for SbvrExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for SbvrExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Sbvr
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::Sbvr);
        let mut output = String::from("SBVR Business Vocabulary and Rules\n\n");

        for statute in statutes {
            let rule_type = "It is obligatory that";

            output.push_str(&format!("{} {}\n\n", rule_type, statute.title));
        }

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
    fn test_sbvr_import_export() {
        let importer = SbvrImporter::new();
        let exporter = SbvrExporter::new();

        let sbvr = "It is obligatory that users verify their identity";
        let (statutes, _) = importer.import(sbvr).unwrap();
        assert_eq!(statutes.len(), 1);

        let (output, _) = exporter.export(&statutes).unwrap();
        assert!(output.contains("SBVR"));
    }
}
