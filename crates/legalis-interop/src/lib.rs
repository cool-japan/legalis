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
//! - **AI-native formats**: LLM-native, embedding, neural-document, attention
//!   markup, and semantic-chunk representations (see [`formats_nextgen`])
//! - **Cross-reality formats**: VR/AR annotation, 3D document, holographic
//!   display, spatial markup, and metaverse-native representations (see
//!   [`cross_reality`])

pub mod ai_converter;
#[cfg(test)]
mod ai_native_tests;
pub mod akoma_ntoso;
#[cfg(feature = "async")]
pub mod async_converter;
pub mod basel3;
#[cfg(feature = "batch")]
pub mod batch;
pub mod blockchain_docs;
pub mod bpmn;
pub mod cache;
pub mod cadence;
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
pub mod cross_reality;
pub mod dmn;
pub mod dms;
pub mod docusign;
#[cfg(test)]
mod edge_cases_tests;
pub mod enhanced;
pub mod error_handling;
pub mod errors;
pub mod fidelity;
pub mod finreg;
pub mod format_detection;
pub mod format_validation;
pub mod formats_nextgen;
pub mod formex;
pub mod future_proof;
pub mod incremental;
pub mod l4;
pub mod legalcite;
pub mod legaldocml;
pub mod legalruleml;
pub mod lkif;
pub mod metalex;
pub mod metrics;
pub mod mifid2;
pub mod move_lang;
pub mod mpeg21_rel;
pub mod msword_legal;
pub mod niem;
pub mod openlaw;
pub mod optimizations;
pub mod pdf_legal;
pub mod performance;
pub mod quality;
pub mod realtime;
pub mod regml;
pub mod rest_api;
pub mod ruleml;
pub mod salesforce_contract;
pub mod sap_legal;
pub mod sbvr;
pub mod schema;
pub mod solidity;
pub mod spdx;
pub mod stipula;
pub mod streaming;
pub mod streaming_v2;
pub mod transformation;
pub mod universal_format;
pub mod validation;
pub mod vyper;
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
    /// SAP Legal Module - Enterprise legal management system
    SapLegal,
    /// Salesforce Contract - Salesforce CPQ contract management
    SalesforceContract,
    /// DocuSign - Electronic signature and digital transaction platform
    DocuSign,
    /// MS Word Legal - Microsoft Word legal add-in format
    MsWordLegal,
    /// PDF Legal - Adobe PDF legal annotations and form fields
    PdfLegal,
    /// Solidity - Ethereum smart contract language
    Solidity,
    /// Vyper - Pythonic Ethereum smart contract language
    Vyper,
    /// Cadence - Flow blockchain smart contract language
    Cadence,
    /// Move - Aptos/Sui blockchain smart contract language
    Move,
    /// LLM-native legal format (prompt-context Markdown/JSON with provenance)
    LlmNative,
    /// Embedding-based legal format (text chunks plus float embedding vectors)
    Embedding,
    /// Neural legal document format (activation graph with PageRank salience)
    NeuralDocument,
    /// Attention-aware legal markup (span-level softmax attention weights)
    AttentionMarkup,
    /// Semantic chunk format (overlap-controlled RAG chunking)
    SemanticChunk,
    /// Long-term preservation archive (BagIt-like; fixity, migration metadata,
    /// post-quantum hash-based signatures, cryptographic agility)
    PreservationArchive,
    /// VR/AR legal annotation format (spatially-anchored annotations)
    VrArAnnotation,
    /// 3D legal document format (scene graph of statute panels; X3D-renderable)
    SpatialDocument3D,
    /// Holographic legal display format (depth-layered light field)
    Holographic,
    /// Spatial legal markup (`SLM`; parseable textual spatial DSL)
    SpatialMarkup,
    /// Metaverse-native legal format (interactive scene graph with portals)
    MetaverseLegal,
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
            LegalFormat::SapLegal => "json",
            LegalFormat::SalesforceContract => "json",
            LegalFormat::DocuSign => "json",
            LegalFormat::MsWordLegal => "json",
            LegalFormat::PdfLegal => "json",
            LegalFormat::Solidity => "sol",
            LegalFormat::Vyper => "vy",
            LegalFormat::Cadence => "cdc",
            LegalFormat::Move => "move",
            LegalFormat::LlmNative => "llm.json",
            LegalFormat::Embedding => "emb.json",
            LegalFormat::NeuralDocument => "neural.json",
            LegalFormat::AttentionMarkup => "attn.json",
            LegalFormat::SemanticChunk => "chunks.json",
            LegalFormat::PreservationArchive => "lpa.json",
            LegalFormat::VrArAnnotation => "var.json",
            LegalFormat::SpatialDocument3D => "l3d.json",
            LegalFormat::Holographic => "holo.json",
            LegalFormat::SpatialMarkup => "slm",
            LegalFormat::MetaverseLegal => "mvl.json",
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
            "saplegal" | "sap.json" | "sap-legal.json" => Some(LegalFormat::SapLegal),
            "salesforce" | "sfdc.json" | "salesforce-contract.json" => {
                Some(LegalFormat::SalesforceContract)
            }
            "docusign" | "docusign.json" | "envelope.json" => Some(LegalFormat::DocuSign),
            "msword" | "word-legal.json" | "msword-legal.json" => Some(LegalFormat::MsWordLegal),
            "pdf-legal" | "pdf-annotations.json" | "pdf-legal.json" => Some(LegalFormat::PdfLegal),
            "sol" | "solidity" => Some(LegalFormat::Solidity),
            "vy" | "vyper" => Some(LegalFormat::Vyper),
            "cdc" | "cadence" => Some(LegalFormat::Cadence),
            "move" => Some(LegalFormat::Move),
            "llm" | "llmjson" | "llm.json" | "llm-native.json" => Some(LegalFormat::LlmNative),
            "emb" | "embjson" | "emb.json" | "embeddings.json" => Some(LegalFormat::Embedding),
            "neural" | "neuraljson" | "neural.json" => Some(LegalFormat::NeuralDocument),
            "attn" | "attnjson" | "attn.json" | "attention.json" => {
                Some(LegalFormat::AttentionMarkup)
            }
            "chunks" | "chunksjson" | "chunks.json" => Some(LegalFormat::SemanticChunk),
            "lpa" | "lpajson" | "lpa.json" | "archive.json" => {
                Some(LegalFormat::PreservationArchive)
            }
            "var" | "varjson" | "var.json" | "vr-ar.json" | "vrar.json" => {
                Some(LegalFormat::VrArAnnotation)
            }
            "l3d" | "l3djson" | "l3d.json" | "doc3d.json" => Some(LegalFormat::SpatialDocument3D),
            "holo" | "holojson" | "holo.json" | "hologram.json" => Some(LegalFormat::Holographic),
            "slm" | "slm.txt" | "spatial.slm" => Some(LegalFormat::SpatialMarkup),
            "mvl" | "mvljson" | "mvl.json" | "metaverse.json" => Some(LegalFormat::MetaverseLegal),
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
                Box::new(sap_legal::SapLegalImporter::new()),
                Box::new(salesforce_contract::SalesforceContractImporter::new()),
                Box::new(docusign::DocuSignImporter::new()),
                Box::new(msword_legal::MsWordLegalImporter::new()),
                Box::new(pdf_legal::PdfLegalImporter::new()),
                Box::new(solidity::SolidityImporter::new()),
                Box::new(vyper::VyperImporter::new()),
                Box::new(cadence::CadenceImporter::new()),
                Box::new(move_lang::MoveImporter::new()),
                Box::new(formats_nextgen::llm_native::LlmNativeImporter::new()),
                Box::new(formats_nextgen::embedding::EmbeddingImporter::new()),
                Box::new(formats_nextgen::neural::NeuralDocumentImporter::new()),
                Box::new(formats_nextgen::attention::AttentionMarkupImporter::new()),
                Box::new(formats_nextgen::semantic_chunk::SemanticChunkImporter::new()),
                Box::new(future_proof::archive::PreservationArchiveImporter::new()),
                Box::new(cross_reality::vr_ar::VrArAnnotationImporter::new()),
                Box::new(cross_reality::document_3d::Document3DImporter::new()),
                Box::new(cross_reality::holographic::HolographicImporter::new()),
                Box::new(cross_reality::spatial_markup::SpatialMarkupImporter::new()),
                Box::new(cross_reality::metaverse::MetaverseLegalImporter::new()),
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
                Box::new(sap_legal::SapLegalExporter::new()),
                Box::new(salesforce_contract::SalesforceContractExporter::new()),
                Box::new(docusign::DocuSignExporter::new()),
                Box::new(msword_legal::MsWordLegalExporter::new()),
                Box::new(pdf_legal::PdfLegalExporter::new()),
                Box::new(solidity::SolidityExporter::new()),
                Box::new(vyper::VyperExporter::new()),
                Box::new(cadence::CadenceExporter::new()),
                Box::new(move_lang::MoveExporter::new()),
                Box::new(formats_nextgen::llm_native::LlmNativeExporter::new()),
                Box::new(formats_nextgen::embedding::EmbeddingExporter::new()),
                Box::new(formats_nextgen::neural::NeuralDocumentExporter::new()),
                Box::new(formats_nextgen::attention::AttentionMarkupExporter::new()),
                Box::new(formats_nextgen::semantic_chunk::SemanticChunkExporter::new()),
                Box::new(future_proof::archive::PreservationArchiveExporter::new()),
                Box::new(cross_reality::vr_ar::VrArAnnotationExporter::new()),
                Box::new(cross_reality::document_3d::Document3DExporter::new()),
                Box::new(cross_reality::holographic::HolographicExporter::new()),
                Box::new(cross_reality::spatial_markup::SpatialMarkupExporter::new()),
                Box::new(cross_reality::metaverse::MetaverseLegalExporter::new()),
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
        if let Some(cache) = &mut self.cache
            && let Some(cached) = cache.get_import(source, format)
        {
            return Ok(cached);
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
        if let Some(cache) = &mut self.cache
            && let Some(cached) = cache.get_export(source, from, to)
        {
            return Ok(cached);
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
mod conversion_tests;
