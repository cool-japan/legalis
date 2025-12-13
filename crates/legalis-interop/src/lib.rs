//! Legalis-Interop: Interoperability layer for legal DSL formats.
//!
//! This crate enables Legalis-RS to import from and export to other legal DSL formats:
//! - **Catala**: French legal DSL for tax and benefits legislation (Inria)
//! - **Stipula**: Italian legal DSL for smart contracts (University of Bologna)
//! - **L4**: Singapore's legal DSL with deontic logic support
//! - **Akoma Ntoso**: XML standard for legislative documents (OASIS)
//! - **LegalRuleML**: XML standard for legal rules

pub mod akoma_ntoso;
pub mod catala;
pub mod l4;
pub mod stipula;

use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors during interop operations.
#[derive(Debug, Error)]
pub enum InteropError {
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("Conversion error: {0}")]
    ConversionError(String),

    #[error("Feature not supported in target format: {0}")]
    UnsupportedFeature(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(String),
}

/// Result type for interop operations.
pub type InteropResult<T> = Result<T, InteropError>;

/// Supported legal DSL formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LegalFormat {
    /// Catala - French legal DSL (Inria)
    Catala,
    /// Stipula - Italian smart contract DSL (Bologna)
    Stipula,
    /// L4 - Singapore legal DSL with deontic logic
    L4,
    /// Akoma Ntoso XML standard
    AkomaNtoso,
    /// LegalRuleML XML standard
    LegalRuleML,
    /// Native Legalis DSL format
    Legalis,
}

impl LegalFormat {
    /// Returns the typical file extension for this format.
    pub fn extension(&self) -> &'static str {
        match self {
            LegalFormat::Catala => "catala_en",
            LegalFormat::Stipula => "stipula",
            LegalFormat::L4 => "l4",
            LegalFormat::AkomaNtoso => "xml",
            LegalFormat::LegalRuleML => "xml",
            LegalFormat::Legalis => "legal",
        }
    }

    /// Attempts to detect format from file extension.
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "catala_en" | "catala_fr" | "catala" => Some(LegalFormat::Catala),
            "stipula" => Some(LegalFormat::Stipula),
            "l4" => Some(LegalFormat::L4),
            "legal" => Some(LegalFormat::Legalis),
            _ => None,
        }
    }
}

/// Report of conversion quality and potential data loss.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConversionReport {
    /// Source format
    pub source_format: Option<LegalFormat>,
    /// Target format
    pub target_format: Option<LegalFormat>,
    /// Features that could not be converted
    pub unsupported_features: Vec<String>,
    /// Warnings about potential semantic changes
    pub warnings: Vec<String>,
    /// Conversion confidence score (0.0 - 1.0)
    pub confidence: f64,
    /// Number of statutes converted
    pub statutes_converted: usize,
}

impl ConversionReport {
    /// Creates a new report.
    pub fn new(source: LegalFormat, target: LegalFormat) -> Self {
        Self {
            source_format: Some(source),
            target_format: Some(target),
            confidence: 1.0,
            ..Default::default()
        }
    }

    /// Adds an unsupported feature warning.
    pub fn add_unsupported(&mut self, feature: impl Into<String>) {
        self.unsupported_features.push(feature.into());
        self.confidence = (self.confidence - 0.1).max(0.0);
    }

    /// Adds a warning.
    pub fn add_warning(&mut self, warning: impl Into<String>) {
        self.warnings.push(warning.into());
        self.confidence = (self.confidence - 0.05).max(0.0);
    }
}

/// Trait for importing from external formats.
pub trait FormatImporter: Send + Sync {
    /// Returns the format this importer handles.
    fn format(&self) -> LegalFormat;

    /// Parses source code into statutes.
    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)>;

    /// Validates that the source is in the expected format.
    fn validate(&self, source: &str) -> bool;
}

/// Trait for exporting to external formats.
pub trait FormatExporter: Send + Sync {
    /// Returns the format this exporter produces.
    fn format(&self) -> LegalFormat;

    /// Exports statutes to the target format.
    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)>;

    /// Checks if a statute can be fully represented in this format.
    fn can_represent(&self, statute: &Statute) -> Vec<String>;
}

/// Universal converter between legal DSL formats.
pub struct LegalConverter {
    importers: Vec<Box<dyn FormatImporter>>,
    exporters: Vec<Box<dyn FormatExporter>>,
}

impl Default for LegalConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl LegalConverter {
    /// Creates a new converter with default importers/exporters.
    pub fn new() -> Self {
        Self {
            importers: vec![
                Box::new(catala::CatalaImporter::new()),
                Box::new(stipula::StipulaImporter::new()),
                Box::new(l4::L4Importer::new()),
                Box::new(akoma_ntoso::AkomaNtosoImporter::new()),
            ],
            exporters: vec![
                Box::new(catala::CatalaExporter::new()),
                Box::new(stipula::StipulaExporter::new()),
                Box::new(l4::L4Exporter::new()),
                Box::new(akoma_ntoso::AkomaNtosoExporter::new()),
            ],
        }
    }

    /// Imports from a specific format.
    pub fn import(
        &self,
        source: &str,
        format: LegalFormat,
    ) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let importer = self
            .importers
            .iter()
            .find(|i| i.format() == format)
            .ok_or_else(|| InteropError::UnsupportedFormat(format!("{:?}", format)))?;

        importer.import(source)
    }

    /// Exports to a specific format.
    pub fn export(
        &self,
        statutes: &[Statute],
        format: LegalFormat,
    ) -> InteropResult<(String, ConversionReport)> {
        let exporter = self
            .exporters
            .iter()
            .find(|e| e.format() == format)
            .ok_or_else(|| InteropError::UnsupportedFormat(format!("{:?}", format)))?;

        exporter.export(statutes)
    }

    /// Converts between formats.
    pub fn convert(
        &self,
        source: &str,
        from: LegalFormat,
        to: LegalFormat,
    ) -> InteropResult<(String, ConversionReport)> {
        let (statutes, mut import_report) = self.import(source, from)?;
        let (output, export_report) = self.export(&statutes, to)?;

        // Merge reports
        import_report.target_format = Some(to);
        import_report
            .unsupported_features
            .extend(export_report.unsupported_features);
        import_report.warnings.extend(export_report.warnings);
        import_report.confidence = (import_report.confidence * export_report.confidence).max(0.0);

        Ok((output, import_report))
    }

    /// Auto-detects format and imports.
    pub fn auto_import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        for importer in &self.importers {
            if importer.validate(source) {
                return importer.import(source);
            }
        }
        Err(InteropError::UnsupportedFormat(
            "Could not auto-detect format".to_string(),
        ))
    }

    /// Returns supported import formats.
    pub fn supported_imports(&self) -> Vec<LegalFormat> {
        self.importers.iter().map(|i| i.format()).collect()
    }

    /// Returns supported export formats.
    pub fn supported_exports(&self) -> Vec<LegalFormat> {
        self.exporters.iter().map(|e| e.format()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

    #[test]
    fn test_format_extension() {
        assert_eq!(LegalFormat::Catala.extension(), "catala_en");
        assert_eq!(LegalFormat::Stipula.extension(), "stipula");
        assert_eq!(LegalFormat::L4.extension(), "l4");
    }

    #[test]
    fn test_format_from_extension() {
        assert_eq!(
            LegalFormat::from_extension("catala_en"),
            Some(LegalFormat::Catala)
        );
        assert_eq!(
            LegalFormat::from_extension("stipula"),
            Some(LegalFormat::Stipula)
        );
        assert_eq!(LegalFormat::from_extension("l4"), Some(LegalFormat::L4));
        assert_eq!(LegalFormat::from_extension("unknown"), None);
    }

    #[test]
    fn test_conversion_report() {
        let mut report = ConversionReport::new(LegalFormat::Catala, LegalFormat::Legalis);
        assert_eq!(report.confidence, 1.0);

        report.add_unsupported("scopes");
        assert!(report.confidence < 1.0);

        report.add_warning("Date format normalized");
        assert!(report.unsupported_features.contains(&"scopes".to_string()));
    }

    #[test]
    fn test_converter_supported_formats() {
        let converter = LegalConverter::new();
        let imports = converter.supported_imports();
        let exports = converter.supported_exports();

        assert!(imports.contains(&LegalFormat::Catala));
        assert!(imports.contains(&LegalFormat::Stipula));
        assert!(imports.contains(&LegalFormat::L4));
        assert!(imports.contains(&LegalFormat::AkomaNtoso));

        assert!(exports.contains(&LegalFormat::Catala));
        assert!(exports.contains(&LegalFormat::Stipula));
        assert!(exports.contains(&LegalFormat::L4));
        assert!(exports.contains(&LegalFormat::AkomaNtoso));
    }

    #[test]
    fn test_catala_export_import_roundtrip() {
        let converter = LegalConverter::new();

        // Create a statute
        let statute = Statute::new(
            "voting-rights",
            "Voting Rights",
            Effect::new(EffectType::Grant, "vote"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        // Export to Catala
        let (catala_output, export_report) =
            converter.export(&[statute], LegalFormat::Catala).unwrap();
        assert_eq!(export_report.statutes_converted, 1);
        assert!(catala_output.contains("declaration scope VotingRights"));
        assert!(catala_output.contains("input.age >= 18"));

        // Import from Catala
        let (imported, import_report) = converter
            .import(&catala_output, LegalFormat::Catala)
            .unwrap();
        assert_eq!(import_report.statutes_converted, 1);
        assert_eq!(imported.len(), 1);
        assert_eq!(imported[0].id, "votingrights");
    }

    #[test]
    fn test_stipula_export_import_roundtrip() {
        let converter = LegalConverter::new();

        // Create a statute
        let statute = Statute::new(
            "simple-contract",
            "Simple Contract",
            Effect::new(EffectType::Grant, "execute"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 21,
        });

        // Export to Stipula
        let (stipula_output, export_report) =
            converter.export(&[statute], LegalFormat::Stipula).unwrap();
        assert_eq!(export_report.statutes_converted, 1);
        assert!(stipula_output.contains("agreement SimpleContract"));
        assert!(stipula_output.contains("age >= 21"));

        // Import from Stipula
        let (imported, import_report) = converter
            .import(&stipula_output, LegalFormat::Stipula)
            .unwrap();
        assert_eq!(import_report.statutes_converted, 1);
        assert_eq!(imported.len(), 1);
        assert_eq!(imported[0].id, "simplecontract");
    }

    #[test]
    fn test_l4_export_import_roundtrip() {
        let converter = LegalConverter::new();

        // Create a statute
        let statute = Statute::new(
            "adult-rights",
            "Adult Rights",
            Effect::new(EffectType::Grant, "full_capacity"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        // Export to L4
        let (l4_output, export_report) = converter.export(&[statute], LegalFormat::L4).unwrap();
        assert_eq!(export_report.statutes_converted, 1);
        assert!(l4_output.contains("RULE AdultRights"));
        assert!(l4_output.contains("age >= 18"));
        assert!(l4_output.contains("MAY"));

        // Import from L4
        let (imported, import_report) = converter.import(&l4_output, LegalFormat::L4).unwrap();
        assert_eq!(import_report.statutes_converted, 1);
        assert_eq!(imported.len(), 1);
    }

    #[test]
    fn test_catala_to_l4_conversion() {
        let converter = LegalConverter::new();

        let catala_source = r#"
```catala
declaration scope TaxBenefit:
  context input content Input
  context output content Output
```

```catala
scope TaxBenefit:
  definition output.eligible equals
    input.age >= 65
```
"#;

        // Convert Catala to L4
        let (l4_output, report) = converter
            .convert(catala_source, LegalFormat::Catala, LegalFormat::L4)
            .unwrap();

        assert!(report.statutes_converted >= 1);
        assert!(l4_output.contains("RULE"));
    }

    #[test]
    fn test_auto_detect_catala() {
        let converter = LegalConverter::new();

        let catala_source = r#"
declaration scope Test:
  context input content integer
"#;

        let (statutes, report) = converter.auto_import(catala_source).unwrap();
        assert_eq!(report.source_format, Some(LegalFormat::Catala));
        assert!(!statutes.is_empty());
    }

    #[test]
    fn test_auto_detect_stipula() {
        let converter = LegalConverter::new();

        let stipula_source = "agreement TestContract(Alice, Bob) { }";

        let (statutes, report) = converter.auto_import(stipula_source).unwrap();
        assert_eq!(report.source_format, Some(LegalFormat::Stipula));
        assert!(!statutes.is_empty());
    }

    #[test]
    fn test_auto_detect_l4() {
        let converter = LegalConverter::new();

        let l4_source = "RULE TestRule WHEN age >= 18 THEN Person MAY vote";

        let (statutes, report) = converter.auto_import(l4_source).unwrap();
        assert_eq!(report.source_format, Some(LegalFormat::L4));
        assert!(!statutes.is_empty());
    }

    #[test]
    fn test_akoma_ntoso_export_import_roundtrip() {
        let converter = LegalConverter::new();

        // Create a statute
        let statute = Statute::new(
            "adult-capacity",
            "Adult Capacity Act",
            Effect::new(EffectType::Grant, "Full legal capacity"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        // Export to Akoma Ntoso
        let (akn_output, export_report) = converter
            .export(&[statute], LegalFormat::AkomaNtoso)
            .unwrap();
        assert_eq!(export_report.statutes_converted, 1);
        assert!(akn_output.contains("<akomaNtoso"));
        assert!(akn_output.contains("Adult Capacity Act"));

        // Import from Akoma Ntoso
        let (imported, import_report) = converter
            .import(&akn_output, LegalFormat::AkomaNtoso)
            .unwrap();
        assert_eq!(import_report.statutes_converted, 1);
        assert_eq!(imported.len(), 1);
        assert_eq!(imported[0].title, "Adult Capacity Act");
    }

    #[test]
    fn test_auto_detect_akoma_ntoso() {
        let converter = LegalConverter::new();

        let akn_source = r#"
        <akomaNtoso>
            <act>
                <body>
                    <article eId="art_1">
                        <heading>Test Article</heading>
                    </article>
                </body>
            </act>
        </akomaNtoso>
        "#;

        let (statutes, report) = converter.auto_import(akn_source).unwrap();
        assert_eq!(report.source_format, Some(LegalFormat::AkomaNtoso));
        assert!(!statutes.is_empty());
    }

    #[test]
    fn test_catala_to_akoma_ntoso_conversion() {
        let converter = LegalConverter::new();

        let catala_source = r#"
declaration scope AdultRights:
  context input content integer
"#;

        // Convert Catala to Akoma Ntoso
        let (akn_output, report) = converter
            .convert(catala_source, LegalFormat::Catala, LegalFormat::AkomaNtoso)
            .unwrap();

        assert!(report.statutes_converted >= 1);
        assert!(akn_output.contains("<akomaNtoso"));
        assert!(akn_output.contains("<article"));
    }
}
