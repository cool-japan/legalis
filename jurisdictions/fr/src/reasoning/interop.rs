//! French Interoperability Module.
//!
//! This module provides integration with the Catala DSL, a French legal DSL
//! developed by INRIA for expressing complex tax and benefits legislation.
//!
//! ## Catala Integration
//!
//! Catala (<https://catala-lang.org/>) is specifically designed for French legislation
//! including:
//! - Code général des impôts (Tax Code)
//! - Code de la sécurité sociale (Social Security Code)
//! - Code civil (Civil Code)
//!
//! This module enables:
//! - Import: Parse Catala source code into Legalis statutes
//! - Export: Convert Legalis statutes to Catala format
//! - Roundtrip: Validate semantic preservation through conversion

use legalis_core::Statute;
use legalis_interop::{
    ConversionReport, InteropResult, LegalConverter, LegalFormat, SemanticValidation,
};

// ============================================================================
// French Catala Integration
// ============================================================================

/// French-specific Catala converter with enhanced support for French legal codes
pub struct FrenchCatalaConverter {
    converter: LegalConverter,
    french_language: FrenchLanguageMode,
}

/// French language mode for Catala
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FrenchLanguageMode {
    /// French language Catala (.catala_fr)
    #[default]
    French,
    /// English language Catala (.catala_en)
    English,
    /// Bilingual mode
    Bilingual,
}

impl Default for FrenchCatalaConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl FrenchCatalaConverter {
    /// Creates a new French Catala converter
    pub fn new() -> Self {
        Self {
            converter: LegalConverter::with_cache(100),
            french_language: FrenchLanguageMode::default(),
        }
    }

    /// Creates a converter with specific language mode
    pub fn with_language(language: FrenchLanguageMode) -> Self {
        Self {
            converter: LegalConverter::with_cache(100),
            french_language: language,
        }
    }

    /// Returns the current language mode
    pub fn language_mode(&self) -> FrenchLanguageMode {
        self.french_language
    }

    /// Imports Catala source code into Legalis statutes
    pub fn import_catala(
        &mut self,
        source: &str,
    ) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let (mut statutes, mut report) = self.converter.import(source, LegalFormat::Catala)?;

        // Add French jurisdiction to all imported statutes
        for statute in &mut statutes {
            if statute.jurisdiction.is_none() {
                statute.jurisdiction = Some("FR".to_string());
            }
        }

        // Add French-specific metadata
        report
            .warnings
            .push(format!("Imported in {:?} mode", self.french_language));

        Ok((statutes, report))
    }

    /// Exports Legalis statutes to Catala format
    pub fn export_to_catala(
        &mut self,
        statutes: &[Statute],
    ) -> InteropResult<(String, ConversionReport)> {
        let (mut output, report) = self.converter.export(statutes, LegalFormat::Catala)?;

        // Add French language header if in French mode
        if self.french_language == FrenchLanguageMode::French {
            let header = format!(
                "# Généré par Legalis-RS\n# Juridiction: France\n# Mode: {:?}\n\n",
                self.french_language
            );
            output = header + output.as_str();
        }

        Ok((output, report))
    }

    /// Converts Catala to other legal DSL formats
    pub fn convert_from_catala(
        &mut self,
        source: &str,
        target: LegalFormat,
    ) -> InteropResult<(String, ConversionReport)> {
        self.converter.convert(source, LegalFormat::Catala, target)
    }

    /// Converts other legal DSL formats to Catala
    pub fn convert_to_catala(
        &mut self,
        source: &str,
        from: LegalFormat,
    ) -> InteropResult<(String, ConversionReport)> {
        self.converter.convert(source, from, LegalFormat::Catala)
    }

    /// Validates roundtrip conversion fidelity
    pub fn validate_roundtrip(
        &mut self,
        source: &str,
        target: LegalFormat,
    ) -> InteropResult<SemanticValidation> {
        self.converter
            .validate_roundtrip(source, LegalFormat::Catala, target)
    }
}

// ============================================================================
// Code Civil Article Converter
// ============================================================================

/// Specialized converter for Code civil articles
pub struct CodeCivilConverter {
    converter: FrenchCatalaConverter,
}

impl Default for CodeCivilConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeCivilConverter {
    /// Creates a new Code civil converter
    pub fn new() -> Self {
        Self {
            converter: FrenchCatalaConverter::new(),
        }
    }

    /// Imports Code civil provisions from Catala
    pub fn import_code_civil(
        &mut self,
        source: &str,
    ) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let (mut statutes, mut report) = self.converter.import_catala(source)?;

        // Tag all statutes as Code civil
        for statute in &mut statutes {
            statute
                .effect
                .parameters
                .insert("source_code".to_string(), "Code civil".to_string());
        }

        report.warnings.push("Imported from Code civil".to_string());

        Ok((statutes, report))
    }

    /// Exports to Catala format with Code civil header
    pub fn export_code_civil(
        &mut self,
        statutes: &[Statute],
    ) -> InteropResult<(String, ConversionReport)> {
        let (mut output, report) = self.converter.export_to_catala(statutes)?;

        // Add Code civil reference header
        output = format!(
            "# Code civil\n# République française\n# Source: https://legifrance.gouv.fr\n\n{}",
            output
        );

        Ok((output, report))
    }
}

// ============================================================================
// Code du Travail (Labor Code) Converter
// ============================================================================

/// Specialized converter for Code du travail articles
pub struct CodeTravailConverter {
    converter: FrenchCatalaConverter,
}

impl Default for CodeTravailConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeTravailConverter {
    /// Creates a new Code du travail converter
    pub fn new() -> Self {
        Self {
            converter: FrenchCatalaConverter::new(),
        }
    }

    /// Imports labor law provisions from Catala
    pub fn import_code_travail(
        &mut self,
        source: &str,
    ) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let (mut statutes, mut report) = self.converter.import_catala(source)?;

        // Tag all statutes as Code du travail
        for statute in &mut statutes {
            statute
                .effect
                .parameters
                .insert("source_code".to_string(), "Code du travail".to_string());
            statute
                .effect
                .parameters
                .insert("domain".to_string(), "employment".to_string());
        }

        report
            .warnings
            .push("Imported from Code du travail".to_string());

        Ok((statutes, report))
    }

    /// Exports to Catala format with labor law header
    pub fn export_code_travail(
        &mut self,
        statutes: &[Statute],
    ) -> InteropResult<(String, ConversionReport)> {
        let (mut output, report) = self.converter.export_to_catala(statutes)?;

        // Add Code du travail reference header
        output = format!(
            "# Code du travail\n# République française\n# Source: https://legifrance.gouv.fr\n\n{}",
            output
        );

        Ok((output, report))
    }
}

// ============================================================================
// Code Général des Impôts (Tax Code) Converter
// ============================================================================

/// Specialized converter for Code général des impôts articles
pub struct CodeImpotsConverter {
    converter: FrenchCatalaConverter,
}

impl Default for CodeImpotsConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeImpotsConverter {
    /// Creates a new CGI converter
    pub fn new() -> Self {
        Self {
            converter: FrenchCatalaConverter::new(),
        }
    }

    /// Imports tax code provisions from Catala
    pub fn import_cgi(&mut self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let (mut statutes, mut report) = self.converter.import_catala(source)?;

        // Tag all statutes as CGI
        for statute in &mut statutes {
            statute.effect.parameters.insert(
                "source_code".to_string(),
                "Code général des impôts".to_string(),
            );
            statute
                .effect
                .parameters
                .insert("domain".to_string(), "tax".to_string());
        }

        report
            .warnings
            .push("Imported from Code général des impôts".to_string());

        Ok((statutes, report))
    }

    /// Exports to Catala format with tax code header
    pub fn export_cgi(
        &mut self,
        statutes: &[Statute],
    ) -> InteropResult<(String, ConversionReport)> {
        let (mut output, report) = self.converter.export_to_catala(statutes)?;

        // Add CGI reference header
        output = format!(
            "# Code général des impôts\n# République française\n# Source: https://legifrance.gouv.fr\n\n{}",
            output
        );

        Ok((output, report))
    }
}

// ============================================================================
// Code de la Sécurité Sociale Converter
// ============================================================================

/// Specialized converter for Code de la sécurité sociale articles
pub struct CodeSecuriteSocialeConverter {
    converter: FrenchCatalaConverter,
}

impl Default for CodeSecuriteSocialeConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeSecuriteSocialeConverter {
    /// Creates a new CSS converter
    pub fn new() -> Self {
        Self {
            converter: FrenchCatalaConverter::new(),
        }
    }

    /// Imports social security provisions from Catala
    pub fn import_css(&mut self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let (mut statutes, mut report) = self.converter.import_catala(source)?;

        // Tag all statutes as CSS
        for statute in &mut statutes {
            statute.effect.parameters.insert(
                "source_code".to_string(),
                "Code de la sécurité sociale".to_string(),
            );
            statute
                .effect
                .parameters
                .insert("domain".to_string(), "social_security".to_string());
        }

        report
            .warnings
            .push("Imported from Code de la sécurité sociale".to_string());

        Ok((statutes, report))
    }

    /// Exports to Catala format with social security code header
    pub fn export_css(
        &mut self,
        statutes: &[Statute],
    ) -> InteropResult<(String, ConversionReport)> {
        let (mut output, report) = self.converter.export_to_catala(statutes)?;

        // Add CSS reference header
        output = format!(
            "# Code de la sécurité sociale\n# République française\n# Source: https://legifrance.gouv.fr\n\n{}",
            output
        );

        Ok((output, report))
    }
}

// ============================================================================
// Batch Conversion Utilities
// ============================================================================

/// Batch converts multiple Catala sources to Legalis statutes
pub fn batch_import_catala(
    sources: &[String],
) -> InteropResult<Vec<(Vec<Statute>, ConversionReport)>> {
    let mut converter = FrenchCatalaConverter::new();
    let mut results = Vec::with_capacity(sources.len());

    for source in sources {
        match converter.import_catala(source) {
            Ok(result) => results.push(result),
            Err(e) => {
                let mut report = ConversionReport::new(LegalFormat::Catala, LegalFormat::Legalis);
                report.add_warning(format!("Import failed: {}", e));
                report.confidence = 0.0;
                results.push((Vec::new(), report));
            }
        }
    }

    Ok(results)
}

/// Batch exports Legalis statutes to Catala format
pub fn batch_export_catala(
    statute_groups: &[Vec<Statute>],
) -> InteropResult<Vec<(String, ConversionReport)>> {
    let mut converter = FrenchCatalaConverter::new();
    let mut results = Vec::with_capacity(statute_groups.len());

    for statutes in statute_groups {
        match converter.export_to_catala(statutes) {
            Ok(result) => results.push(result),
            Err(e) => {
                let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::Catala);
                report.add_warning(format!("Export failed: {}", e));
                report.confidence = 0.0;
                results.push((String::new(), report));
            }
        }
    }

    Ok(results)
}

// ============================================================================
// Catala Template Generators
// ============================================================================

/// Generates a Catala scope declaration template
pub fn generate_catala_scope(name: &str, inputs: &[(&str, &str)], output_name: &str) -> String {
    let mut template = String::new();

    template.push_str("```catala\n");
    template.push_str(&format!(
        "declaration scope {}:\n",
        to_catala_identifier(name)
    ));

    for (input_name, input_type) in inputs {
        template.push_str(&format!(
            "  context input {} content {}\n",
            input_name, input_type
        ));
    }

    template.push_str(&format!(
        "  context output {} content boolean\n",
        output_name
    ));
    template.push_str("```\n");

    template
}

/// Generates a Catala rule definition template
pub fn generate_catala_rule(
    scope_name: &str,
    condition: &str,
    output_name: &str,
    value: &str,
) -> String {
    let mut template = String::new();

    template.push_str("```catala\n");
    template.push_str(&format!("scope {}:\n", to_catala_identifier(scope_name)));
    template.push_str(&format!("  definition {} equals\n", output_name));
    template.push_str(&format!("    if {} then {}\n", condition, value));
    template.push_str("    else false\n");
    template.push_str("```\n");

    template
}

/// Converts a name to a valid Catala identifier (PascalCase)
fn to_catala_identifier(name: &str) -> String {
    name.split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                None => String::new(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

    #[test]
    fn test_french_catala_converter_creation() {
        let converter = FrenchCatalaConverter::new();
        assert_eq!(converter.language_mode(), FrenchLanguageMode::French);
    }

    #[test]
    fn test_french_catala_converter_with_language() {
        let converter = FrenchCatalaConverter::with_language(FrenchLanguageMode::English);
        assert_eq!(converter.language_mode(), FrenchLanguageMode::English);
    }

    #[test]
    fn test_catala_import() {
        let mut converter = FrenchCatalaConverter::new();

        let catala_source = r#"
declaration scope VotingRights:
  context input age content integer
  context output eligible content boolean
"#;

        let result = converter.import_catala(catala_source);
        assert!(result.is_ok());

        let (statutes, report) = result.unwrap();
        assert!(!statutes.is_empty());
        assert!(report.statutes_converted >= 1);
    }

    #[test]
    fn test_catala_export() {
        let mut converter = FrenchCatalaConverter::new();

        let statute = Statute::new(
            "art_1240",
            "Responsabilité délictuelle",
            Effect::new(EffectType::Grant, "Droit à réparation"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
        .with_jurisdiction("FR");

        let result = converter.export_to_catala(&[statute]);
        assert!(result.is_ok());

        let (output, report) = result.unwrap();
        assert!(output.contains("declaration scope"));
        assert!(report.statutes_converted >= 1);
    }

    #[test]
    fn test_code_civil_converter() {
        let mut converter = CodeCivilConverter::new();

        let catala_source =
            "declaration scope ContractValidity:\n  context input age content integer";

        let result = converter.import_code_civil(catala_source);
        assert!(result.is_ok());

        let (statutes, _) = result.unwrap();
        for statute in statutes {
            assert_eq!(
                statute.effect.parameters.get("source_code"),
                Some(&"Code civil".to_string())
            );
        }
    }

    #[test]
    fn test_code_travail_converter() {
        let mut converter = CodeTravailConverter::new();

        let catala_source = "declaration scope MinimumWage:\n  context input hours content integer";

        let result = converter.import_code_travail(catala_source);
        assert!(result.is_ok());

        let (statutes, _) = result.unwrap();
        for statute in statutes {
            assert_eq!(
                statute.effect.parameters.get("source_code"),
                Some(&"Code du travail".to_string())
            );
        }
    }

    #[test]
    fn test_cgi_converter() {
        let mut converter = CodeImpotsConverter::new();

        let catala_source = "declaration scope IncomeTax:\n  context input income content money";

        let result = converter.import_cgi(catala_source);
        assert!(result.is_ok());

        let (statutes, _) = result.unwrap();
        for statute in statutes {
            assert_eq!(
                statute.effect.parameters.get("domain"),
                Some(&"tax".to_string())
            );
        }
    }

    #[test]
    fn test_css_converter() {
        let mut converter = CodeSecuriteSocialeConverter::new();

        let catala_source =
            "declaration scope RetirementBenefits:\n  context input years content integer";

        let result = converter.import_css(catala_source);
        assert!(result.is_ok());

        let (statutes, _) = result.unwrap();
        for statute in statutes {
            assert_eq!(
                statute.effect.parameters.get("domain"),
                Some(&"social_security".to_string())
            );
        }
    }

    #[test]
    fn test_generate_catala_scope() {
        let scope = generate_catala_scope(
            "voting_eligibility",
            &[("age", "integer"), ("citizenship", "boolean")],
            "eligible",
        );

        assert!(scope.contains("declaration scope VotingEligibility"));
        assert!(scope.contains("context input age content integer"));
        assert!(scope.contains("context output eligible content boolean"));
    }

    #[test]
    fn test_generate_catala_rule() {
        let rule = generate_catala_rule(
            "voting_eligibility",
            "input.age >= 18 and input.citizenship",
            "eligible",
            "true",
        );

        assert!(rule.contains("scope VotingEligibility"));
        assert!(rule.contains("definition eligible equals"));
    }

    #[test]
    fn test_to_catala_identifier() {
        assert_eq!(to_catala_identifier("voting_rights"), "VotingRights");
        assert_eq!(
            to_catala_identifier("code-civil-art1240"),
            "CodeCivilArt1240"
        );
        assert_eq!(to_catala_identifier("TestCase"), "TestCase");
    }

    #[test]
    fn test_batch_import_catala() {
        let sources = vec![
            "declaration scope Test1:\n  context input age content integer".to_string(),
            "declaration scope Test2:\n  context input name content text".to_string(),
        ];

        let result = batch_import_catala(&sources);
        assert!(result.is_ok());

        let results = result.unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_convert_from_catala() {
        let mut converter = FrenchCatalaConverter::new();

        let catala_source = "declaration scope Test:\n  context input age content integer";

        let result = converter.convert_from_catala(catala_source, LegalFormat::L4);
        assert!(result.is_ok());

        let (output, _) = result.unwrap();
        assert!(output.contains("RULE"));
    }
}
