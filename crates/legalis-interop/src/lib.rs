//! Legalis-Interop: Interoperability layer for legal DSL formats.
//!
//! This crate enables Legalis-RS to import from and export to other legal DSL formats:
//! - **Catala**: French legal DSL for tax and benefits legislation (Inria)
//! - **Stipula**: Italian legal DSL for smart contracts (University of Bologna)
//! - **L4**: Singapore's legal DSL with deontic logic support
//! - **Akoma Ntoso**: XML standard for legislative documents (OASIS)
//! - **LegalRuleML**: XML standard for legal rules
//! - **LegalDocML**: OASIS legal document markup standard
//! - **LKIF**: Legal Knowledge Interchange Format (ESTRELLA)
//! - **LegalCite**: OASIS standard for legal citation (TC LegalCiteM)
//! - **MetaLex**: CEN standard for legal document metadata (CWA 15710)
//! - **MPEG-21 REL**: ISO standard for rights expression (ISO/IEC 21000-5)
//! - **Creative Commons**: CC license format (RDF/XML)
//! - **SPDX**: Software Package Data Exchange license expressions (ISO/IEC 5962:2021)

pub mod ai_converter;
pub mod akoma_ntoso;
#[cfg(feature = "async")]
pub mod async_converter;
pub mod basel3;
#[cfg(feature = "batch")]
pub mod batch;
pub mod bpmn;
pub mod cache;
pub mod catala;
pub mod cicero;
pub mod clauseio;
pub mod cli;
pub mod cmmn;
pub mod commonform;
pub mod compatibility;
pub mod contractexpress;
pub mod coverage;
pub mod creative_commons;
pub mod dmn;
pub mod dms;
#[cfg(test)]
mod edge_cases_tests;
pub mod enhanced;
pub mod error_handling;
pub mod errors;
pub mod fidelity;
pub mod finreg;
pub mod format_detection;
pub mod format_validation;
pub mod formex;
pub mod incremental;
pub mod l4;
pub mod legalcite;
pub mod legaldocml;
pub mod legalruleml;
pub mod lkif;
pub mod metalex;
pub mod metrics;
pub mod mifid2;
pub mod mpeg21_rel;
pub mod niem;
pub mod openlaw;
pub mod optimizations;
pub mod performance;
pub mod quality;
pub mod regml;
pub mod rest_api;
pub mod ruleml;
pub mod sbvr;
pub mod schema;
pub mod spdx;
pub mod stipula;
pub mod streaming;
pub mod streaming_v2;
pub mod transformation;
pub mod validation;
pub mod webhooks;
pub mod xbrl;

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

    #[error("Validation error: {0}")]
    ValidationError(String),
}

/// Result type for interop operations.
pub type InteropResult<T> = Result<T, InteropError>;

/// Supported legal DSL formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
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
    /// LegalDocML - OASIS legal document markup standard
    LegalDocML,
    /// LKIF - Legal Knowledge Interchange Format
    LKIF,
    /// LegalCite - OASIS standard for legal citation
    LegalCite,
    /// MetaLex - CEN standard for legal document metadata
    MetaLex,
    /// MPEG-21 REL - ISO standard for rights expression
    Mpeg21Rel,
    /// Creative Commons license format
    CreativeCommons,
    /// SPDX license expression format
    Spdx,
    /// Native Legalis DSL format
    Legalis,
    /// BPMN - Business Process Model and Notation (OMG)
    Bpmn,
    /// DMN - Decision Model and Notation (OMG)
    Dmn,
    /// CMMN - Case Management Model and Notation (OMG)
    Cmmn,
    /// RuleML - Rule Markup Language
    RuleML,
    /// SBVR - Semantics of Business Vocabulary and Business Rules
    Sbvr,
    /// OpenLaw - Protocol for creating and executing legal agreements
    OpenLaw,
    /// Cicero - Accord Project smart legal contract templates
    Cicero,
    /// CommonForm - Format for legal forms and contracts (JSON)
    CommonForm,
    /// Clause.io - Contract automation platform templates
    ClauseIo,
    /// ContractExpress - Document automation platform
    ContractExpress,
    /// FORMEX - EU Official Journal format
    Formex,
    /// NIEM - National Information Exchange Model
    Niem,
    /// FinReg - Financial Regulatory format
    FinReg,
    /// XBRL - eXtensible Business Reporting Language
    Xbrl,
    /// RegML - Regulation Markup Language
    RegML,
    /// MiFID II - Markets in Financial Instruments Directive II
    MiFID2,
    /// Basel III - International regulatory framework for banks
    Basel3,
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
            LegalFormat::LegalDocML => "xml",
            LegalFormat::LKIF => "xml",
            LegalFormat::LegalCite => "xml",
            LegalFormat::MetaLex => "xml",
            LegalFormat::Mpeg21Rel => "xml",
            LegalFormat::CreativeCommons => "rdf",
            LegalFormat::Spdx => "spdx",
            LegalFormat::Legalis => "legal",
            LegalFormat::Bpmn => "bpmn",
            LegalFormat::Dmn => "dmn",
            LegalFormat::Cmmn => "cmmn",
            LegalFormat::RuleML => "ruleml",
            LegalFormat::Sbvr => "sbvr",
            LegalFormat::OpenLaw => "openlaw",
            LegalFormat::Cicero => "cicero",
            LegalFormat::CommonForm => "json",
            LegalFormat::ClauseIo => "json",
            LegalFormat::ContractExpress => "docx",
            LegalFormat::Formex => "xml",
            LegalFormat::Niem => "xml",
            LegalFormat::FinReg => "json",
            LegalFormat::Xbrl => "xbrl",
            LegalFormat::RegML => "xml",
            LegalFormat::MiFID2 => "json",
            LegalFormat::Basel3 => "json",
        }
    }

    /// Attempts to detect format from file extension.
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "catala_en" | "catala_fr" | "catala" => Some(LegalFormat::Catala),
            "stipula" => Some(LegalFormat::Stipula),
            "l4" => Some(LegalFormat::L4),
            "lkif" => Some(LegalFormat::LKIF),
            "rdf" => Some(LegalFormat::CreativeCommons),
            "spdx" => Some(LegalFormat::Spdx),
            "legal" => Some(LegalFormat::Legalis),
            "bpmn" => Some(LegalFormat::Bpmn),
            "dmn" => Some(LegalFormat::Dmn),
            "cmmn" => Some(LegalFormat::Cmmn),
            "ruleml" => Some(LegalFormat::RuleML),
            "sbvr" => Some(LegalFormat::Sbvr),
            "openlaw" => Some(LegalFormat::OpenLaw),
            "cicero" => Some(LegalFormat::Cicero),
            "commonform" | "commonform.json" => Some(LegalFormat::CommonForm),
            "clauseio" | "clauseio.json" => Some(LegalFormat::ClauseIo),
            "contractexpress" | "docx" => Some(LegalFormat::ContractExpress),
            "formex" => Some(LegalFormat::Formex),
            "niem" => Some(LegalFormat::Niem),
            "finreg" | "finreg.json" => Some(LegalFormat::FinReg),
            "xbrl" => Some(LegalFormat::Xbrl),
            "regml" | "regml.xml" => Some(LegalFormat::RegML),
            "mifid2" | "mifid2.json" => Some(LegalFormat::MiFID2),
            "basel3" | "basel3.json" => Some(LegalFormat::Basel3),
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

    /// Returns true if the conversion is considered high quality (confidence >= 0.8).
    pub fn is_high_quality(&self) -> bool {
        self.confidence >= 0.8
    }

    /// Returns true if the conversion is lossless (confidence == 1.0 and no warnings).
    pub fn is_lossless(&self) -> bool {
        self.confidence >= 1.0 && self.unsupported_features.is_empty() && self.warnings.is_empty()
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
    cache: Option<cache::ConversionCache>,
}

impl Default for LegalConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl LegalConverter {
    /// Creates a new converter with default importers/exporters (without caching).
    pub fn new() -> Self {
        Self {
            importers: vec![
                Box::new(catala::CatalaImporter::new()),
                Box::new(stipula::StipulaImporter::new()),
                Box::new(l4::L4Importer::new()),
                Box::new(akoma_ntoso::AkomaNtosoImporter::new()),
                Box::new(legalruleml::LegalRuleMLImporter::new()),
                Box::new(legaldocml::LegalDocMLImporter::new()),
                Box::new(lkif::LkifImporter::new()),
                Box::new(legalcite::LegalCiteImporter::new()),
                Box::new(metalex::MetaLexImporter::new()),
                Box::new(mpeg21_rel::Mpeg21RelImporter::new()),
                Box::new(creative_commons::CreativeCommonsImporter::new()),
                Box::new(spdx::SpdxImporter::new()),
                Box::new(bpmn::BpmnImporter::new()),
                Box::new(dmn::DmnImporter::new()),
                Box::new(cmmn::CmmnImporter::new()),
                Box::new(ruleml::RuleMLImporter::new()),
                Box::new(sbvr::SbvrImporter::new()),
                Box::new(openlaw::OpenLawImporter::new()),
                Box::new(cicero::CiceroImporter::new()),
                Box::new(commonform::CommonFormImporter::new()),
                Box::new(clauseio::ClauseIoImporter::new()),
                Box::new(contractexpress::ContractExpressImporter::new()),
                Box::new(formex::FormexImporter::new()),
                Box::new(niem::NiemImporter::new()),
                Box::new(finreg::FinRegImporter::new()),
                Box::new(xbrl::XbrlImporter::new()),
                Box::new(regml::RegMLImporter::new()),
                Box::new(mifid2::MiFID2Importer::new()),
                Box::new(basel3::Basel3Importer::new()),
            ],
            exporters: vec![
                Box::new(catala::CatalaExporter::new()),
                Box::new(stipula::StipulaExporter::new()),
                Box::new(l4::L4Exporter::new()),
                Box::new(akoma_ntoso::AkomaNtosoExporter::new()),
                Box::new(legalruleml::LegalRuleMLExporter::new()),
                Box::new(legaldocml::LegalDocMLExporter::new()),
                Box::new(lkif::LkifExporter::new()),
                Box::new(legalcite::LegalCiteExporter::new()),
                Box::new(metalex::MetaLexExporter::new()),
                Box::new(mpeg21_rel::Mpeg21RelExporter::new()),
                Box::new(creative_commons::CreativeCommonsExporter::new()),
                Box::new(spdx::SpdxExporter::new()),
                Box::new(bpmn::BpmnExporter::new()),
                Box::new(dmn::DmnExporter::new()),
                Box::new(cmmn::CmmnExporter::new()),
                Box::new(ruleml::RuleMLExporter::new()),
                Box::new(sbvr::SbvrExporter::new()),
                Box::new(openlaw::OpenLawExporter::new()),
                Box::new(cicero::CiceroExporter::new()),
                Box::new(commonform::CommonFormExporter::new()),
                Box::new(clauseio::ClauseIoExporter::new()),
                Box::new(contractexpress::ContractExpressExporter::new()),
                Box::new(formex::FormexExporter::new()),
                Box::new(niem::NiemExporter::new()),
                Box::new(finreg::FinRegExporter::new()),
                Box::new(xbrl::XbrlExporter::new()),
                Box::new(regml::RegMLExporter::new()),
                Box::new(mifid2::MiFID2Exporter::new()),
                Box::new(basel3::Basel3Exporter::new()),
            ],
            cache: None,
        }
    }

    /// Creates a new converter with caching enabled.
    pub fn with_cache(cache_size: usize) -> Self {
        let mut converter = Self::new();
        converter.cache = Some(cache::ConversionCache::with_capacity(cache_size));
        converter
    }

    /// Enables caching with the specified capacity.
    pub fn enable_cache(&mut self, cache_size: usize) {
        self.cache = Some(cache::ConversionCache::with_capacity(cache_size));
    }

    /// Disables caching.
    pub fn disable_cache(&mut self) {
        self.cache = None;
    }

    /// Clears the cache if enabled.
    pub fn clear_cache(&mut self) {
        if let Some(cache) = &mut self.cache {
            cache.clear();
        }
    }

    /// Returns cache statistics if caching is enabled.
    pub fn cache_stats(&self) -> Option<cache::CacheStats> {
        self.cache.as_ref().map(|c| c.stats())
    }

    /// Imports from a specific format.
    pub fn import(
        &mut self,
        source: &str,
        format: LegalFormat,
    ) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        // Check cache first
        if let Some(cache) = &mut self.cache {
            if let Some(cached) = cache.get_import(source, format) {
                return Ok(cached);
            }
        }

        let importer = self
            .importers
            .iter()
            .find(|i| i.format() == format)
            .ok_or_else(|| InteropError::UnsupportedFormat(format!("{:?}", format)))?;

        let result = importer.import(source)?;

        // Store in cache
        if let Some(cache) = &mut self.cache {
            cache.put_import(source, format, result.0.clone(), result.1.clone());
        }

        Ok(result)
    }

    /// Exports to a specific format.
    pub fn export(
        &mut self,
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
        &mut self,
        source: &str,
        from: LegalFormat,
        to: LegalFormat,
    ) -> InteropResult<(String, ConversionReport)> {
        // Check cache first
        if let Some(cache) = &mut self.cache {
            if let Some(cached) = cache.get_export(source, from, to) {
                return Ok(cached);
            }
        }

        let (statutes, mut import_report) = self.import(source, from)?;
        let (output, export_report) = self.export(&statutes, to)?;

        // Merge reports
        import_report.target_format = Some(to);
        import_report
            .unsupported_features
            .extend(export_report.unsupported_features);
        import_report.warnings.extend(export_report.warnings);
        import_report.confidence = (import_report.confidence * export_report.confidence).max(0.0);

        // Store in cache
        if let Some(cache) = &mut self.cache {
            cache.put_export(source, from, to, output.clone(), import_report.clone());
        }

        Ok((output, import_report))
    }

    /// Auto-detects format and imports.
    pub fn auto_import(&mut self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        for importer in &self.importers {
            if importer.validate(source) {
                let format = importer.format();
                return self.import(source, format);
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

    /// Batch converts multiple source documents.
    ///
    /// # Arguments
    /// * `sources` - Vector of (source_text, source_format) tuples
    /// * `target_format` - Target format for all conversions
    ///
    /// # Returns
    /// Vector of (converted_text, report) tuples, one for each source
    pub fn batch_convert(
        &mut self,
        sources: &[(String, LegalFormat)],
        target_format: LegalFormat,
    ) -> InteropResult<Vec<(String, ConversionReport)>> {
        let mut results = Vec::with_capacity(sources.len());

        for (source_text, source_format) in sources {
            match self.convert(source_text, *source_format, target_format) {
                Ok(result) => results.push(result),
                Err(e) => {
                    // Create error report for failed conversion
                    let mut report = ConversionReport::new(*source_format, target_format);
                    report.add_warning(format!("Conversion failed: {}", e));
                    report.confidence = 0.0;
                    results.push((String::new(), report));
                }
            }
        }

        Ok(results)
    }

    /// Batch imports multiple source documents.
    ///
    /// # Arguments
    /// * `sources` - Vector of (source_text, source_format) tuples
    ///
    /// # Returns
    /// Vector of (statutes, report) tuples, one for each source
    pub fn batch_import(
        &mut self,
        sources: &[(String, LegalFormat)],
    ) -> InteropResult<Vec<(Vec<Statute>, ConversionReport)>> {
        let mut results = Vec::with_capacity(sources.len());

        for (source_text, source_format) in sources {
            match self.import(source_text, *source_format) {
                Ok(result) => results.push(result),
                Err(e) => {
                    // Create error report for failed import
                    let mut report = ConversionReport::new(*source_format, LegalFormat::Legalis);
                    report.add_warning(format!("Import failed: {}", e));
                    report.confidence = 0.0;
                    results.push((Vec::new(), report));
                }
            }
        }

        Ok(results)
    }

    /// Batch exports statutes to multiple formats.
    ///
    /// # Arguments
    /// * `statutes` - Statutes to export
    /// * `target_formats` - Vector of target formats
    ///
    /// # Returns
    /// Vector of (format, converted_text, report) tuples
    pub fn batch_export(
        &mut self,
        statutes: &[Statute],
        target_formats: &[LegalFormat],
    ) -> InteropResult<Vec<(LegalFormat, String, ConversionReport)>> {
        let mut results = Vec::with_capacity(target_formats.len());

        for &format in target_formats {
            match self.export(statutes, format) {
                Ok((output, report)) => results.push((format, output, report)),
                Err(e) => {
                    // Create error report for failed export
                    let mut report = ConversionReport::new(LegalFormat::Legalis, format);
                    report.add_warning(format!("Export failed: {}", e));
                    report.confidence = 0.0;
                    results.push((format, String::new(), report));
                }
            }
        }

        Ok(results)
    }

    /// Parallel batch converts multiple source documents.
    ///
    /// Uses rayon for parallel processing to speed up conversion of multiple documents.
    /// Note: This method requires mutable self but processes items in parallel safely.
    ///
    /// # Arguments
    /// * `sources` - Vector of (source_text, source_format) tuples
    /// * `target_format` - Target format for all conversions
    ///
    /// # Returns
    /// Vector of (converted_text, report) tuples, one for each source
    #[cfg(feature = "parallel")]
    pub fn batch_convert_parallel(
        sources: &[(String, LegalFormat)],
        target_format: LegalFormat,
    ) -> InteropResult<Vec<(String, ConversionReport)>> {
        use rayon::prelude::*;

        let results: Vec<_> = sources
            .par_iter()
            .map(|(source_text, source_format)| {
                let mut converter = Self::new();
                match converter.convert(source_text, *source_format, target_format) {
                    Ok(result) => result,
                    Err(e) => {
                        let mut report = ConversionReport::new(*source_format, target_format);
                        report.add_warning(format!("Conversion failed: {}", e));
                        report.confidence = 0.0;
                        (String::new(), report)
                    }
                }
            })
            .collect();

        Ok(results)
    }

    /// Parallel batch imports multiple source documents.
    ///
    /// Uses rayon for parallel processing to speed up importing of multiple documents.
    ///
    /// # Arguments
    /// * `sources` - Vector of (source_text, source_format) tuples
    ///
    /// # Returns
    /// Vector of (statutes, report) tuples, one for each source
    #[cfg(feature = "parallel")]
    pub fn batch_import_parallel(
        sources: &[(String, LegalFormat)],
    ) -> InteropResult<Vec<(Vec<Statute>, ConversionReport)>> {
        use rayon::prelude::*;

        let results: Vec<_> = sources
            .par_iter()
            .map(|(source_text, source_format)| {
                let mut converter = Self::new();
                match converter.import(source_text, *source_format) {
                    Ok(result) => result,
                    Err(e) => {
                        let mut report =
                            ConversionReport::new(*source_format, LegalFormat::Legalis);
                        report.add_warning(format!("Import failed: {}", e));
                        report.confidence = 0.0;
                        (Vec::new(), report)
                    }
                }
            })
            .collect();

        Ok(results)
    }

    /// Parallel batch exports statutes to multiple formats.
    ///
    /// Uses rayon for parallel processing to speed up exporting to multiple formats.
    ///
    /// # Arguments
    /// * `statutes` - Statutes to export
    /// * `target_formats` - Vector of target formats
    ///
    /// # Returns
    /// Vector of (format, converted_text, report) tuples
    #[cfg(feature = "parallel")]
    pub fn batch_export_parallel(
        statutes: &[Statute],
        target_formats: &[LegalFormat],
    ) -> InteropResult<Vec<(LegalFormat, String, ConversionReport)>> {
        use rayon::prelude::*;

        let results: Vec<_> = target_formats
            .par_iter()
            .map(|&format| {
                let mut converter = Self::new();
                match converter.export(statutes, format) {
                    Ok((output, report)) => (format, output, report),
                    Err(e) => {
                        let mut report = ConversionReport::new(LegalFormat::Legalis, format);
                        report.add_warning(format!("Export failed: {}", e));
                        report.confidence = 0.0;
                        (format, String::new(), report)
                    }
                }
            })
            .collect();

        Ok(results)
    }

    /// Validates semantic preservation through roundtrip conversion.
    ///
    /// Converts to target format and back, then compares statute counts and structure.
    ///
    /// # Arguments
    /// * `source` - Source text
    /// * `source_format` - Source format
    /// * `target_format` - Target format to test
    ///
    /// # Returns
    /// Validation report with findings
    pub fn validate_roundtrip(
        &mut self,
        source: &str,
        source_format: LegalFormat,
        target_format: LegalFormat,
    ) -> InteropResult<SemanticValidation> {
        // Import original
        let (original_statutes, import_report) = self.import(source, source_format)?;

        // Convert to target format
        let (target_output, convert_report) = self.export(&original_statutes, target_format)?;

        // Convert back to source format
        let (roundtrip_statutes, reimport_report) = self.import(&target_output, target_format)?;

        // Compare
        let mut validation = SemanticValidation::new(source_format, target_format);

        // Check statute count preservation
        if original_statutes.len() != roundtrip_statutes.len() {
            validation.add_issue(format!(
                "Statute count changed: {} -> {}",
                original_statutes.len(),
                roundtrip_statutes.len()
            ));
        }

        // Check individual statutes
        for (i, (original, roundtrip)) in original_statutes
            .iter()
            .zip(roundtrip_statutes.iter())
            .enumerate()
        {
            // Compare precondition counts
            if original.preconditions.len() != roundtrip.preconditions.len() {
                validation.add_issue(format!(
                    "Statute {}: Precondition count changed: {} -> {}",
                    i,
                    original.preconditions.len(),
                    roundtrip.preconditions.len()
                ));
            }

            // Compare effect types
            if original.effect.effect_type != roundtrip.effect.effect_type {
                validation.add_issue(format!(
                    "Statute {}: Effect type changed: {:?} -> {:?}",
                    i, original.effect.effect_type, roundtrip.effect.effect_type
                ));
            }
        }

        // Aggregate confidence from all reports
        validation.confidence =
            (import_report.confidence * convert_report.confidence * reimport_report.confidence)
                .max(0.0);

        validation.import_report = import_report;
        validation.convert_report = convert_report;
        validation.reimport_report = reimport_report;

        Ok(validation)
    }
}

/// Semantic preservation validation result.
#[derive(Debug, Clone)]
pub struct SemanticValidation {
    /// Source format
    pub source_format: LegalFormat,
    /// Target format tested
    pub target_format: LegalFormat,
    /// Issues found during validation
    pub issues: Vec<String>,
    /// Overall confidence in semantic preservation (0.0 - 1.0)
    pub confidence: f64,
    /// Import report
    pub import_report: ConversionReport,
    /// Conversion report
    pub convert_report: ConversionReport,
    /// Re-import report
    pub reimport_report: ConversionReport,
}

impl SemanticValidation {
    /// Creates a new validation result.
    pub fn new(source: LegalFormat, target: LegalFormat) -> Self {
        Self {
            source_format: source,
            target_format: target,
            issues: Vec::new(),
            confidence: 1.0,
            import_report: ConversionReport::new(source, LegalFormat::Legalis),
            convert_report: ConversionReport::new(LegalFormat::Legalis, target),
            reimport_report: ConversionReport::new(target, LegalFormat::Legalis),
        }
    }

    /// Adds a validation issue.
    pub fn add_issue(&mut self, issue: impl Into<String>) {
        self.issues.push(issue.into());
        self.confidence = (self.confidence - 0.1).max(0.0);
    }

    /// Returns true if validation passed (no issues and high confidence).
    pub fn passed(&self) -> bool {
        self.issues.is_empty() && self.confidence >= 0.8
    }

    /// Returns true if semantic preservation is perfect (no issues, confidence 1.0).
    pub fn is_perfect(&self) -> bool {
        self.issues.is_empty() && self.confidence >= 1.0
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
        let mut converter = LegalConverter::new();

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
        let mut converter = LegalConverter::new();

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
        let mut converter = LegalConverter::new();

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
        let mut converter = LegalConverter::new();

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
        let mut converter = LegalConverter::new();

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
        let mut converter = LegalConverter::new();

        let stipula_source = "agreement TestContract(Alice, Bob) { }";

        let (statutes, report) = converter.auto_import(stipula_source).unwrap();
        assert_eq!(report.source_format, Some(LegalFormat::Stipula));
        assert!(!statutes.is_empty());
    }

    #[test]
    fn test_auto_detect_l4() {
        let mut converter = LegalConverter::new();

        let l4_source = "RULE TestRule WHEN age >= 18 THEN Person MAY vote";

        let (statutes, report) = converter.auto_import(l4_source).unwrap();
        assert_eq!(report.source_format, Some(LegalFormat::L4));
        assert!(!statutes.is_empty());
    }

    #[test]
    fn test_akoma_ntoso_export_import_roundtrip() {
        let mut converter = LegalConverter::new();

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
        let mut converter = LegalConverter::new();

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
        let mut converter = LegalConverter::new();

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

    #[test]
    fn test_legalruleml_export_import_roundtrip() {
        let mut converter = LegalConverter::new();

        // Create a statute
        let statute = Statute::new(
            "legal-rule",
            "Legal Rule Example",
            Effect::new(EffectType::Grant, "Legal capacity"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        // Export to LegalRuleML
        let (lrml_output, export_report) = converter
            .export(&[statute], LegalFormat::LegalRuleML)
            .unwrap();
        assert_eq!(export_report.statutes_converted, 1);
        assert!(lrml_output.contains("<legalruleml"));
        assert!(lrml_output.contains("Legal Rule Example"));

        // Import from LegalRuleML
        let (imported, import_report) = converter
            .import(&lrml_output, LegalFormat::LegalRuleML)
            .unwrap();
        assert_eq!(import_report.statutes_converted, 1);
        assert_eq!(imported.len(), 1);
        assert_eq!(imported[0].title, "Legal Rule Example");
    }

    #[test]
    fn test_auto_detect_legalruleml() {
        let mut converter = LegalConverter::new();

        let lrml_source = r#"
        <legalruleml>
            <Statements>
                <LegalRule key="test">
                    <Name>Test</Name>
                    <if><Premise>age >= 18</Premise></if>
                    <then><Conclusion>Grant</Conclusion></then>
                </LegalRule>
            </Statements>
        </legalruleml>
        "#;

        let (statutes, report) = converter.auto_import(lrml_source).unwrap();
        assert_eq!(report.source_format, Some(LegalFormat::LegalRuleML));
        assert!(!statutes.is_empty());
    }

    #[test]
    fn test_catala_to_legalruleml_conversion() {
        let mut converter = LegalConverter::new();

        let catala_source = r#"
declaration scope TaxRule:
  context input content integer
"#;

        // Convert Catala to LegalRuleML
        let (lrml_output, report) = converter
            .convert(catala_source, LegalFormat::Catala, LegalFormat::LegalRuleML)
            .unwrap();

        assert!(report.statutes_converted >= 1);
        assert!(lrml_output.contains("<legalruleml"));
        assert!(lrml_output.contains("<LegalRule"));
    }

    #[test]
    fn test_batch_convert() {
        let mut converter = LegalConverter::new();

        let sources = vec![
            (
                "declaration scope Test1:\n  context input content integer".to_string(),
                LegalFormat::Catala,
            ),
            (
                "agreement Test2(A, B) { }".to_string(),
                LegalFormat::Stipula,
            ),
        ];

        let results = converter.batch_convert(&sources, LegalFormat::L4).unwrap();

        assert_eq!(results.len(), 2);
        assert!(results[0].0.contains("RULE"));
        assert!(results[1].0.contains("RULE"));
    }

    #[test]
    fn test_batch_import() {
        let mut converter = LegalConverter::new();

        let sources = vec![
            (
                "declaration scope Test1:\n  context input content integer".to_string(),
                LegalFormat::Catala,
            ),
            (
                "agreement Test2(A, B) { }".to_string(),
                LegalFormat::Stipula,
            ),
        ];

        let results = converter.batch_import(&sources).unwrap();

        assert_eq!(results.len(), 2);
        assert!(!results[0].0.is_empty());
        assert!(!results[1].0.is_empty());
    }

    #[test]
    fn test_batch_export() {
        let mut converter = LegalConverter::new();

        let statute = Statute::new(
            "test",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test"),
        );

        let formats = vec![LegalFormat::Catala, LegalFormat::L4, LegalFormat::Stipula];

        let results = converter.batch_export(&[statute], &formats).unwrap();

        assert_eq!(results.len(), 3);
        assert_eq!(results[0].0, LegalFormat::Catala);
        assert_eq!(results[1].0, LegalFormat::L4);
        assert_eq!(results[2].0, LegalFormat::Stipula);
    }

    #[test]
    fn test_conversion_caching() {
        let mut converter = LegalConverter::with_cache(10);

        let catala_source = "declaration scope Test:\n  context input content integer";

        // First conversion - cache miss
        let (output1, report1) = converter
            .convert(catala_source, LegalFormat::Catala, LegalFormat::L4)
            .unwrap();

        // Second conversion - cache hit
        let (output2, report2) = converter
            .convert(catala_source, LegalFormat::Catala, LegalFormat::L4)
            .unwrap();

        // Results should be identical
        assert_eq!(output1, output2);
        assert_eq!(report1.statutes_converted, report2.statutes_converted);

        // Check cache stats
        // Note: We cache both import and conversion, so first run creates 2 entries
        // Second run is a cache hit on conversion
        let stats = converter.cache_stats().unwrap();
        assert_eq!(stats.entries, 2); // import + conversion cached
        assert!(stats.access_count >= 3); // Multiple puts and gets
    }

    #[test]
    fn test_cache_enable_disable() {
        let mut converter = LegalConverter::new();

        // Initially no cache
        assert!(converter.cache_stats().is_none());

        // Enable cache
        converter.enable_cache(5);
        assert!(converter.cache_stats().is_some());

        // Disable cache
        converter.disable_cache();
        assert!(converter.cache_stats().is_none());
    }

    #[test]
    fn test_semantic_validation_roundtrip() {
        let mut converter = LegalConverter::new();

        let l4_source = "RULE VotingAge WHEN age >= 18 THEN Person MAY vote";

        let validation = converter
            .validate_roundtrip(l4_source, LegalFormat::L4, LegalFormat::Catala)
            .unwrap();

        // Should preserve basic structure
        assert!(validation.confidence > 0.0);
        assert!(!validation.issues.is_empty() || validation.passed());
    }

    #[test]
    fn test_conversion_report_quality() {
        let mut report = ConversionReport::new(LegalFormat::Catala, LegalFormat::L4);

        assert!(report.is_lossless());
        assert!(report.is_high_quality());

        report.add_warning("Minor issue");
        assert!(!report.is_lossless());
        assert!(report.is_high_quality());

        report.add_unsupported("Major feature");
        report.add_unsupported("Another feature");
        report.add_unsupported("Yet another");
        assert!(!report.is_high_quality());
    }

    #[test]
    fn test_semantic_validation_structure() {
        let mut converter = LegalConverter::new();

        let catala_source = r#"
declaration scope AdultRights:
  context input content integer
"#;

        let validation = converter
            .validate_roundtrip(catala_source, LegalFormat::Catala, LegalFormat::L4)
            .unwrap();

        // Validation structure should be populated
        assert_eq!(validation.source_format, LegalFormat::Catala);
        assert_eq!(validation.target_format, LegalFormat::L4);
        assert!(validation.confidence <= 1.0);
    }

    // Tests for new formats (v0.1.1)

    #[test]
    fn test_legalcite_export_import_roundtrip() {
        let mut converter = LegalConverter::new();

        let statute = Statute::new(
            "test-statute",
            "Test Statute",
            Effect::new(EffectType::Grant, "legal_reference"),
        )
        .with_jurisdiction("US");

        let (legalcite_output, export_report) = converter
            .export(&[statute], LegalFormat::LegalCite)
            .unwrap();
        assert_eq!(export_report.statutes_converted, 1);
        assert!(legalcite_output.contains("legalCite"));

        let (imported, import_report) = converter
            .import(&legalcite_output, LegalFormat::LegalCite)
            .unwrap();
        assert_eq!(import_report.statutes_converted, 1);
        assert_eq!(imported.len(), 1);
    }

    #[test]
    fn test_metalex_export_import_roundtrip() {
        let mut converter = LegalConverter::new();

        let statute = Statute::new(
            "article-1",
            "Article 1",
            Effect::new(EffectType::Grant, "provision"),
        );

        let (metalex_output, export_report) =
            converter.export(&[statute], LegalFormat::MetaLex).unwrap();
        assert_eq!(export_report.statutes_converted, 1);
        assert!(metalex_output.contains("metalex"));

        let (imported, import_report) = converter
            .import(&metalex_output, LegalFormat::MetaLex)
            .unwrap();
        assert_eq!(import_report.statutes_converted, 1);
        assert_eq!(imported.len(), 1);
    }

    #[test]
    fn test_mpeg21_rel_export_import_roundtrip() {
        let mut converter = LegalConverter::new();

        let statute = Statute::new(
            "play-right",
            "Play Right",
            Effect::new(EffectType::Grant, "play"),
        );

        let (mpeg21_output, export_report) = converter
            .export(&[statute], LegalFormat::Mpeg21Rel)
            .unwrap();
        assert_eq!(export_report.statutes_converted, 1);

        let (imported, import_report) = converter
            .import(&mpeg21_output, LegalFormat::Mpeg21Rel)
            .unwrap();
        assert_eq!(import_report.statutes_converted, 1);
        assert_eq!(imported.len(), 1);
    }

    #[test]
    fn test_creative_commons_export_import_roundtrip() {
        let mut converter = LegalConverter::new();

        let statute = Statute::new(
            "cc-permit",
            "Permit Reproduction",
            Effect::new(EffectType::Grant, "Reproduction"),
        );

        let (cc_output, export_report) = converter
            .export(&[statute], LegalFormat::CreativeCommons)
            .unwrap();
        assert_eq!(export_report.statutes_converted, 1);

        let (imported, import_report) = converter
            .import(&cc_output, LegalFormat::CreativeCommons)
            .unwrap();
        assert_eq!(import_report.statutes_converted, 1);
        assert!(!imported.is_empty());
    }

    #[test]
    fn test_spdx_export_import_roundtrip() {
        let mut converter = LegalConverter::new();

        let mut effect = Effect::new(EffectType::Grant, "use");
        effect
            .parameters
            .insert("spdx_id".to_string(), "MIT".to_string());
        let statute = Statute::new("mit_license", "License: MIT", effect);

        let (spdx_output, export_report) = converter.export(&[statute], LegalFormat::Spdx).unwrap();
        assert_eq!(export_report.statutes_converted, 1);
        assert_eq!(spdx_output, "MIT");

        let (imported, import_report) = converter.import(&spdx_output, LegalFormat::Spdx).unwrap();
        assert_eq!(import_report.statutes_converted, 1);
        assert_eq!(imported.len(), 1);
    }

    #[test]
    fn test_auto_detect_legalcite() {
        let mut converter = LegalConverter::new();

        let legalcite_source = r#"<LegalCiteDocument>
            <legalCite>
                <citations>
                    <id>test-1</id>
                    <title>Test Citation</title>
                    <citation_type>statute</citation_type>
                </citations>
            </legalCite>
        </LegalCiteDocument>"#;

        let (statutes, report) = converter.auto_import(legalcite_source).unwrap();
        assert_eq!(report.source_format, Some(LegalFormat::LegalCite));
        assert!(!statutes.is_empty());
    }

    #[test]
    fn test_auto_detect_metalex() {
        let mut converter = LegalConverter::new();

        let metalex_source = r#"<MetaLexDocument>
            <metalex>
                <Body>
                    <Article id="art-1">
                        <Title>Test Article</Title>
                    </Article>
                </Body>
            </metalex>
        </MetaLexDocument>"#;

        let (statutes, report) = converter.auto_import(metalex_source).unwrap();
        assert_eq!(report.source_format, Some(LegalFormat::MetaLex));
        assert!(!statutes.is_empty());
    }

    #[test]
    fn test_auto_detect_mpeg21_rel() {
        let mut converter = LegalConverter::new();

        let mpeg21_source = r#"<Mpeg21RelDocument>
            <license>
                <grant>
                    <right>play</right>
                </grant>
            </license>
        </Mpeg21RelDocument>"#;

        let (statutes, report) = converter.auto_import(mpeg21_source).unwrap();
        assert_eq!(report.source_format, Some(LegalFormat::Mpeg21Rel));
        assert!(!statutes.is_empty());
    }

    #[test]
    fn test_auto_detect_creative_commons() {
        let mut converter = LegalConverter::new();

        let cc_source = "https://creativecommons.org/licenses/by/4.0/";

        let (statutes, report) = converter.auto_import(cc_source).unwrap();
        assert_eq!(report.source_format, Some(LegalFormat::CreativeCommons));
        assert!(!statutes.is_empty());
    }

    #[test]
    fn test_auto_detect_spdx() {
        let mut converter = LegalConverter::new();

        let spdx_source = "MIT AND Apache-2.0";

        let (statutes, report) = converter.auto_import(spdx_source).unwrap();
        assert_eq!(report.source_format, Some(LegalFormat::Spdx));
        assert!(!statutes.is_empty());
    }

    #[test]
    fn test_new_formats_in_converter() {
        let converter = LegalConverter::new();
        let imports = converter.supported_imports();
        let exports = converter.supported_exports();

        // Check all new formats are registered
        assert!(imports.contains(&LegalFormat::LegalCite));
        assert!(imports.contains(&LegalFormat::MetaLex));
        assert!(imports.contains(&LegalFormat::Mpeg21Rel));
        assert!(imports.contains(&LegalFormat::CreativeCommons));
        assert!(imports.contains(&LegalFormat::Spdx));

        assert!(exports.contains(&LegalFormat::LegalCite));
        assert!(exports.contains(&LegalFormat::MetaLex));
        assert!(exports.contains(&LegalFormat::Mpeg21Rel));
        assert!(exports.contains(&LegalFormat::CreativeCommons));
        assert!(exports.contains(&LegalFormat::Spdx));
    }
}
