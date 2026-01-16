//! Universal Legal Format (ULF) - Canonical interchange format.
//!
//! ULF is a comprehensive, lossless interchange format designed to represent
//! legal documents and statutes from any legal DSL or format. It serves as
//! a canonical intermediate representation for conversions between formats.
//!
//! Key features:
//! - Lossless representation of legal concepts from all supported formats
//! - Versioned schema with backward/forward compatibility
//! - Rich metadata and provenance tracking
//! - Support for format-specific extensions
//! - JSON and binary serialization

use crate::{InteropError, InteropResult, LegalFormat};
use chrono::{DateTime, Utc};
use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Universal Legal Format version
pub const ULF_VERSION: &str = "1.0.0";

/// Supported ULF versions
pub const SUPPORTED_ULF_VERSIONS: &[&str] = &["1.0.0"];

/// Minimum compatible ULF version
pub const MIN_COMPATIBLE_VERSION: &str = "1.0.0";

/// ULF version information
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct UlfVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl UlfVersion {
    /// Parses a version string (e.g., "1.2.3")
    pub fn parse(version: &str) -> InteropResult<Self> {
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() != 3 {
            return Err(InteropError::ParseError(format!(
                "Invalid version format: {}. Expected major.minor.patch",
                version
            )));
        }

        let major = parts[0].parse().map_err(|_| {
            InteropError::ParseError(format!("Invalid major version: {}", parts[0]))
        })?;
        let minor = parts[1].parse().map_err(|_| {
            InteropError::ParseError(format!("Invalid minor version: {}", parts[1]))
        })?;
        let patch = parts[2].parse().map_err(|_| {
            InteropError::ParseError(format!("Invalid patch version: {}", parts[2]))
        })?;

        Ok(UlfVersion {
            major,
            minor,
            patch,
        })
    }

    /// Creates from components
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Checks if this version is compatible with another
    pub fn is_compatible_with(&self, other: &UlfVersion) -> bool {
        // Major version must match for compatibility
        // Minor/patch versions are backward compatible
        self.major == other.major && self.minor >= other.minor
    }

    /// Checks if migration is needed
    pub fn needs_migration(&self, target: &UlfVersion) -> bool {
        self != target
    }
}

impl fmt::Display for UlfVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Feature flags for ULF versions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UlfFeatures {
    /// Supports temporal validity
    pub temporal_validity: bool,
    /// Supports cross-references
    pub cross_references: bool,
    /// Supports provenance tracking
    pub provenance: bool,
    /// Supports format-specific extensions
    pub extensions: bool,
    /// Supports document structure
    pub document_structure: bool,
}

impl UlfFeatures {
    /// Returns features for a specific ULF version
    pub fn for_version(version: &UlfVersion) -> Self {
        match (version.major, version.minor) {
            (1, 0) => Self {
                temporal_validity: true,
                cross_references: true,
                provenance: true,
                extensions: true,
                document_structure: true,
            },
            // Future versions would add more features
            _ => Self {
                temporal_validity: false,
                cross_references: false,
                provenance: false,
                extensions: false,
                document_structure: false,
            },
        }
    }

    /// Checks if a feature is supported
    pub fn supports_temporal_validity(&self) -> bool {
        self.temporal_validity
    }

    pub fn supports_cross_references(&self) -> bool {
        self.cross_references
    }

    pub fn supports_provenance(&self) -> bool {
        self.provenance
    }

    pub fn supports_extensions(&self) -> bool {
        self.extensions
    }

    pub fn supports_document_structure(&self) -> bool {
        self.document_structure
    }
}

/// Version migration manager
pub struct VersionMigrator;

impl VersionMigrator {
    /// Migrates a ULF document to a target version
    pub fn migrate(
        doc: &mut UniversalLegalDocument,
        target_version: &str,
    ) -> InteropResult<Vec<String>> {
        let current = UlfVersion::parse(&doc.ulf_version)?;
        let target = UlfVersion::parse(target_version)?;

        if !current.needs_migration(&target) {
            return Ok(vec!["No migration needed".to_string()]);
        }

        let mut warnings = Vec::new();

        // Check compatibility
        if !target.is_compatible_with(&current) && !current.is_compatible_with(&target) {
            warnings.push(format!(
                "Warning: Migrating between incompatible versions {} -> {}",
                current, target
            ));
        }

        // Perform version-specific migrations
        Self::migrate_1_0_to_target(doc, &current, &target, &mut warnings)?;

        // Update version
        doc.ulf_version = target_version.to_string();

        Ok(warnings)
    }

    /// Migrates from version 1.0 to target
    fn migrate_1_0_to_target(
        _doc: &mut UniversalLegalDocument,
        current: &UlfVersion,
        target: &UlfVersion,
        warnings: &mut Vec<String>,
    ) -> InteropResult<()> {
        if current.major == 1 && current.minor == 0 {
            match (target.major, target.minor) {
                (1, 0) => {
                    // Same version, no migration needed
                    Ok(())
                }
                (1, 1) => {
                    // Future: migrate to 1.1
                    warnings.push("Migration to 1.1 not yet implemented".to_string());
                    Ok(())
                }
                (2, _) => {
                    // Future: migrate to 2.x
                    warnings.push("Migration to 2.x not yet implemented".to_string());
                    Ok(())
                }
                _ => Err(InteropError::UnsupportedFormat(format!(
                    "Unknown target version: {}",
                    target
                ))),
            }
        } else {
            // Document is from a future version
            warnings.push(format!(
                "Downgrading from {} to {} may lose features",
                current, target
            ));
            Ok(())
        }
    }

    /// Validates a ULF document version
    pub fn validate_version(doc: &UniversalLegalDocument) -> InteropResult<Vec<String>> {
        let version = UlfVersion::parse(&doc.ulf_version)?;
        let mut warnings = Vec::new();

        // Check if version is supported
        if !SUPPORTED_ULF_VERSIONS.contains(&doc.ulf_version.as_str()) {
            warnings.push(format!(
                "ULF version {} is not officially supported. Supported versions: {}",
                doc.ulf_version,
                SUPPORTED_ULF_VERSIONS.join(", ")
            ));
        }

        // Check minimum compatibility
        let min_version = UlfVersion::parse(MIN_COMPATIBLE_VERSION)?;
        if !version.is_compatible_with(&min_version) {
            warnings.push(format!(
                "ULF version {} may not be compatible with minimum required version {}",
                doc.ulf_version, MIN_COMPATIBLE_VERSION
            ));
        }

        // Check feature usage
        let features = UlfFeatures::for_version(&version);

        if doc.structure.is_some() && !features.supports_document_structure() {
            warnings.push("Document uses structure but version doesn't support it".to_string());
        }

        if !doc.cross_references.is_empty() && !features.supports_cross_references() {
            warnings.push(
                "Document uses cross-references but version doesn't support them".to_string(),
            );
        }

        if !doc.extensions.is_empty() && !features.supports_extensions() {
            warnings.push("Document uses extensions but version doesn't support them".to_string());
        }

        Ok(warnings)
    }
}

/// Universal Legal Document - top-level container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalLegalDocument {
    /// ULF version
    pub ulf_version: String,
    /// Document metadata
    pub metadata: DocumentMetadata,
    /// Legal provisions (statutes, rules, clauses)
    pub provisions: Vec<LegalProvision>,
    /// Document structure (sections, articles, chapters)
    pub structure: Option<DocumentStructure>,
    /// Cross-references between provisions
    pub cross_references: Vec<CrossReference>,
    /// Document annotations
    pub annotations: Vec<Annotation>,
    /// Format-specific extensions
    pub extensions: HashMap<String, serde_json::Value>,
}

/// Document metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    /// Document identifier
    pub id: String,
    /// Document title
    pub title: String,
    /// Source format
    pub source_format: String,
    /// Jurisdiction
    pub jurisdiction: Option<String>,
    /// Document type (statute, regulation, contract, etc.)
    pub document_type: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modified timestamp
    pub modified_at: DateTime<Utc>,
    /// Authors
    pub authors: Vec<String>,
    /// Version number
    pub version: String,
    /// Language (ISO 639-1 code)
    pub language: String,
    /// Custom metadata
    pub custom: HashMap<String, String>,
}

/// Legal provision - represents a single legal rule or clause
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalProvision {
    /// Unique identifier
    pub id: String,
    /// Title or heading
    pub title: String,
    /// Provision type
    pub provision_type: ProvisionType,
    /// Conditions (IF part)
    pub conditions: Vec<Condition>,
    /// Effects (THEN part)
    pub effects: Vec<Effect>,
    /// Exceptions (UNLESS part)
    pub exceptions: Vec<Condition>,
    /// Discretion logic (MAY/MIGHT part)
    pub discretion: Option<String>,
    /// Temporal validity
    pub temporal: Option<TemporalValidity>,
    /// Applicability scope
    pub scope: Option<ApplicabilityScope>,
    /// Provenance (where this came from)
    pub provenance: Option<Provenance>,
    /// Format-specific data
    pub format_specific: HashMap<String, serde_json::Value>,
}

/// Type of legal provision
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProvisionType {
    /// Grant of right or permission
    Grant,
    /// Imposition of obligation
    Obligation,
    /// Prohibition
    Prohibition,
    /// Definition
    Definition,
    /// Procedural rule
    Procedural,
    /// Interpretive rule
    Interpretive,
    /// Exception or exclusion
    Exception,
    /// Custom type
    Custom(String),
}

/// Condition in a legal provision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    /// Condition type
    pub condition_type: ConditionType,
    /// Description
    pub description: String,
    /// Logical operator (for compound conditions)
    pub operator: Option<LogicalOperator>,
    /// Sub-conditions (for compound conditions)
    pub sub_conditions: Vec<Condition>,
    /// Parameters
    pub parameters: HashMap<String, String>,
}

/// Type of condition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConditionType {
    /// Age comparison
    Age,
    /// Date/time condition
    Temporal,
    /// Status check
    Status,
    /// Jurisdiction check
    Jurisdiction,
    /// Role or capacity
    Role,
    /// Compound condition (AND/OR/NOT)
    Compound,
    /// Custom condition
    Custom,
}

/// Logical operator for compound conditions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum LogicalOperator {
    And,
    Or,
    Not,
    Xor,
}

/// Effect of a legal provision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effect {
    /// Effect type
    pub effect_type: EffectType,
    /// Description
    pub description: String,
    /// Target (who/what is affected)
    pub target: Option<String>,
    /// Action or change
    pub action: String,
    /// Parameters
    pub parameters: HashMap<String, String>,
}

/// Type of legal effect
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EffectType {
    /// Grant right or permission
    Grant,
    /// Revoke right or permission
    Revoke,
    /// Impose obligation
    Obligation,
    /// Prohibit action
    Prohibition,
    /// Transfer (money, property, etc.)
    Transfer,
    /// Change status
    StatusChange,
    /// Create entity
    Creation,
    /// Terminate entity
    Termination,
    /// Custom effect
    Custom(String),
}

/// Temporal validity period
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalValidity {
    /// Start date/time
    pub start: Option<DateTime<Utc>>,
    /// End date/time
    pub end: Option<DateTime<Utc>>,
    /// Duration description
    pub duration: Option<String>,
    /// Renewal rules
    pub renewal: Option<String>,
}

/// Applicability scope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicabilityScope {
    /// Geographic scope
    pub geographic: Vec<String>,
    /// Subject matter scope
    pub subject_matter: Vec<String>,
    /// Applicable entities
    pub entities: Vec<String>,
    /// Custom scope criteria
    pub custom: HashMap<String, String>,
}

/// Provenance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provenance {
    /// Source document
    pub source: String,
    /// Source section/article
    pub source_section: Option<String>,
    /// Original text
    pub original_text: Option<String>,
    /// Transformation history
    pub transformations: Vec<Transformation>,
}

/// Transformation in conversion history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transformation {
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// From format
    pub from_format: String,
    /// To format
    pub to_format: String,
    /// Converter version
    pub converter_version: String,
    /// Information loss description
    pub loss_description: Option<String>,
}

/// Document structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentStructure {
    /// Root sections
    pub sections: Vec<Section>,
    /// Table of contents
    pub toc: Option<Vec<TocEntry>>,
}

/// Document section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    /// Section identifier
    pub id: String,
    /// Section number or label
    pub number: String,
    /// Section title
    pub title: String,
    /// Section type (chapter, article, section, etc.)
    pub section_type: String,
    /// Provision IDs in this section
    pub provisions: Vec<String>,
    /// Subsections
    pub subsections: Vec<Section>,
}

/// Table of contents entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TocEntry {
    /// Entry identifier
    pub id: String,
    /// Entry title
    pub title: String,
    /// Level (depth in hierarchy)
    pub level: u32,
    /// Page or provision reference
    pub reference: String,
}

/// Cross-reference between provisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossReference {
    /// Source provision ID
    pub from: String,
    /// Target provision ID
    pub to: String,
    /// Reference type
    pub reference_type: ReferenceType,
    /// Description
    pub description: Option<String>,
}

/// Type of cross-reference
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReferenceType {
    /// Defines or elaborates
    Defines,
    /// Depends on
    DependsOn,
    /// Modifies or amends
    Modifies,
    /// Supersedes
    Supersedes,
    /// Conflicts with
    ConflictsWith,
    /// Complements
    Complements,
    /// Custom reference type
    Custom(String),
}

/// Annotation on the document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    /// Annotation ID
    pub id: String,
    /// Target provision ID
    pub target: String,
    /// Annotation type
    pub annotation_type: String,
    /// Content
    pub content: String,
    /// Author
    pub author: Option<String>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

impl UniversalLegalDocument {
    /// Creates a new empty ULF document
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        source_format: LegalFormat,
    ) -> Self {
        let now = Utc::now();
        Self {
            ulf_version: ULF_VERSION.to_string(),
            metadata: DocumentMetadata {
                id: id.into(),
                title: title.into(),
                source_format: format!("{:?}", source_format),
                jurisdiction: None,
                document_type: "statute".to_string(),
                created_at: now,
                modified_at: now,
                authors: Vec::new(),
                version: "1.0".to_string(),
                language: "en".to_string(),
                custom: HashMap::new(),
            },
            provisions: Vec::new(),
            structure: None,
            cross_references: Vec::new(),
            annotations: Vec::new(),
            extensions: HashMap::new(),
        }
    }

    /// Converts from legalis_core::Statute to ULF
    pub fn from_statute(statute: &Statute, source_format: LegalFormat) -> Self {
        let mut doc = Self::new(&statute.id, &statute.title, source_format);

        // Convert statute to provision
        let provision = LegalProvision {
            id: statute.id.clone(),
            title: statute.title.clone(),
            provision_type: Self::map_effect_type(&statute.effect.effect_type),
            conditions: statute
                .preconditions
                .iter()
                .map(Self::map_condition)
                .collect(),
            effects: vec![Self::map_effect(&statute.effect)],
            exceptions: statute
                .exceptions
                .iter()
                .map(Self::map_exception_to_condition)
                .collect(),
            discretion: statute.discretion_logic.clone(),
            temporal: Some(Self::map_temporal(&statute.temporal_validity)),
            scope: Some(Self::map_scope(statute)),
            provenance: None,
            format_specific: HashMap::new(),
        };

        doc.provisions.push(provision);
        doc.metadata.jurisdiction = statute.jurisdiction.clone();

        doc
    }

    /// Converts ULF back to legalis_core::Statute
    pub fn to_statute(&self) -> InteropResult<Statute> {
        if self.provisions.is_empty() {
            return Err(InteropError::ConversionError(
                "No provisions in ULF document".to_string(),
            ));
        }

        let provision = &self.provisions[0];

        // Map back to core types
        let effect_type = Self::unmap_effect_type(&provision.provision_type)?;
        let effect_description = provision
            .effects
            .first()
            .map(|e| e.description.clone())
            .unwrap_or_default();

        let effect = legalis_core::Effect::new(effect_type, effect_description);

        let mut statute = Statute::new(&provision.id, &provision.title, effect);

        // Add preconditions
        for condition in &provision.conditions {
            if let Ok(cond) = Self::unmap_condition(condition) {
                statute = statute.with_precondition(cond);
            }
        }

        // Add jurisdiction
        statute.jurisdiction = self.metadata.jurisdiction.clone();

        Ok(statute)
    }

    // Helper mapping functions
    fn map_effect_type(effect_type: &legalis_core::EffectType) -> ProvisionType {
        match effect_type {
            legalis_core::EffectType::Grant => ProvisionType::Grant,
            legalis_core::EffectType::Obligation => ProvisionType::Obligation,
            legalis_core::EffectType::Prohibition => ProvisionType::Prohibition,
            _ => ProvisionType::Grant,
        }
    }

    fn unmap_effect_type(
        provision_type: &ProvisionType,
    ) -> InteropResult<legalis_core::EffectType> {
        Ok(match provision_type {
            ProvisionType::Grant => legalis_core::EffectType::Grant,
            ProvisionType::Obligation => legalis_core::EffectType::Obligation,
            ProvisionType::Prohibition => legalis_core::EffectType::Prohibition,
            _ => legalis_core::EffectType::Custom,
        })
    }

    fn map_condition(condition: &legalis_core::Condition) -> Condition {
        match condition {
            legalis_core::Condition::Age { operator, value } => Condition {
                condition_type: ConditionType::Age,
                description: format!("age {} {}", Self::format_operator(operator), value),
                operator: None,
                sub_conditions: Vec::new(),
                parameters: [
                    ("operator".to_string(), format!("{:?}", operator)),
                    ("value".to_string(), value.to_string()),
                ]
                .iter()
                .cloned()
                .collect(),
            },
            legalis_core::Condition::Custom { description } => Condition {
                condition_type: ConditionType::Custom,
                description: description.clone(),
                operator: None,
                sub_conditions: Vec::new(),
                parameters: HashMap::new(),
            },
            _ => Condition {
                condition_type: ConditionType::Custom,
                description: format!("{:?}", condition),
                operator: None,
                sub_conditions: Vec::new(),
                parameters: HashMap::new(),
            },
        }
    }

    fn unmap_condition(condition: &Condition) -> InteropResult<legalis_core::Condition> {
        Ok(match condition.condition_type {
            ConditionType::Age => {
                let operator = condition
                    .parameters
                    .get("operator")
                    .and_then(|s| match s.as_str() {
                        "Equal" => Some(legalis_core::ComparisonOp::Equal),
                        "GreaterThan" => Some(legalis_core::ComparisonOp::GreaterThan),
                        "GreaterOrEqual" => Some(legalis_core::ComparisonOp::GreaterOrEqual),
                        "LessThan" => Some(legalis_core::ComparisonOp::LessThan),
                        "LessOrEqual" => Some(legalis_core::ComparisonOp::LessOrEqual),
                        _ => None,
                    })
                    .unwrap_or(legalis_core::ComparisonOp::GreaterOrEqual);

                let value = condition
                    .parameters
                    .get("value")
                    .and_then(|s| s.parse::<u32>().ok())
                    .unwrap_or(18);

                legalis_core::Condition::Age { operator, value }
            }
            _ => legalis_core::Condition::Custom {
                description: condition.description.clone(),
            },
        })
    }

    fn format_operator(op: &legalis_core::ComparisonOp) -> &'static str {
        match op {
            legalis_core::ComparisonOp::Equal => "==",
            legalis_core::ComparisonOp::NotEqual => "!=",
            legalis_core::ComparisonOp::GreaterThan => ">",
            legalis_core::ComparisonOp::GreaterOrEqual => ">=",
            legalis_core::ComparisonOp::LessThan => "<",
            legalis_core::ComparisonOp::LessOrEqual => "<=",
        }
    }

    fn map_effect(effect: &legalis_core::Effect) -> Effect {
        Effect {
            effect_type: match effect.effect_type {
                legalis_core::EffectType::Grant => EffectType::Grant,
                legalis_core::EffectType::Revoke => EffectType::Revoke,
                legalis_core::EffectType::Obligation => EffectType::Obligation,
                legalis_core::EffectType::Prohibition => EffectType::Prohibition,
                legalis_core::EffectType::MonetaryTransfer => EffectType::Transfer,
                legalis_core::EffectType::StatusChange => EffectType::StatusChange,
                legalis_core::EffectType::Custom => EffectType::Custom("custom".to_string()),
            },
            description: effect.description.clone(),
            target: None,
            action: effect.description.clone(),
            parameters: effect.parameters.clone(),
        }
    }

    fn map_exception_to_condition(exception: &legalis_core::StatuteException) -> Condition {
        Condition {
            condition_type: ConditionType::Custom,
            description: exception.description.clone(),
            operator: None,
            sub_conditions: Vec::new(),
            parameters: HashMap::new(),
        }
    }

    fn map_temporal(temporal: &legalis_core::TemporalValidity) -> TemporalValidity {
        TemporalValidity {
            start: temporal
                .effective_date
                .map(|d| d.and_hms_opt(0, 0, 0).unwrap().and_utc()),
            end: temporal
                .expiry_date
                .map(|d| d.and_hms_opt(23, 59, 59).unwrap().and_utc()),
            duration: None,
            renewal: None,
        }
    }

    fn map_scope(statute: &Statute) -> ApplicabilityScope {
        ApplicabilityScope {
            geographic: statute.jurisdiction.iter().cloned().collect(),
            subject_matter: Vec::new(),
            entities: statute.applies_to.clone(),
            custom: HashMap::new(),
        }
    }

    /// Serializes to JSON
    pub fn to_json(&self) -> InteropResult<String> {
        serde_json::to_string_pretty(self).map_err(|e| {
            InteropError::SerializationError(format!("JSON serialization failed: {}", e))
        })
    }

    /// Deserializes from JSON
    pub fn from_json(json: &str) -> InteropResult<Self> {
        serde_json::from_str(json)
            .map_err(|e| InteropError::ParseError(format!("JSON parsing failed: {}", e)))
    }
}

/// Conversion path recommendation
#[derive(Debug, Clone, PartialEq)]
pub enum ConversionPath {
    /// Direct conversion between formats
    Direct,
    /// Convert via ULF intermediate format
    ViaULF,
    /// Multiple hops through intermediate formats
    MultiHop(Vec<LegalFormat>),
}

/// Format negotiation result
#[derive(Debug, Clone)]
pub struct NegotiationResult {
    /// Recommended conversion path
    pub path: ConversionPath,
    /// Estimated conversion confidence (0.0 - 1.0)
    pub confidence: f64,
    /// Reason for the recommendation
    pub reasoning: String,
    /// Potential data loss warnings
    pub warnings: Vec<String>,
    /// Alternative paths (if any)
    pub alternatives: Vec<(ConversionPath, f64)>,
}

/// Format negotiator - determines optimal conversion strategy
pub struct FormatNegotiator;

impl FormatNegotiator {
    /// Creates a new format negotiator
    pub fn new() -> Self {
        Self
    }

    /// Analyzes and recommends conversion strategy
    pub fn negotiate(&self, source: LegalFormat, target: LegalFormat) -> NegotiationResult {
        // Same format - no conversion needed
        if source == target {
            return NegotiationResult {
                path: ConversionPath::Direct,
                confidence: 1.0,
                reasoning: "Source and target formats are identical".to_string(),
                warnings: Vec::new(),
                alternatives: Vec::new(),
            };
        }

        // Calculate format compatibility
        let compatibility = self.calculate_compatibility(source, target);

        // Determine optimal path
        let (path, confidence, reasoning) = if compatibility > 0.7 {
            // High compatibility - use direct conversion
            (
                ConversionPath::Direct,
                compatibility,
                format!(
                    "Direct conversion supported with {:.0}% compatibility",
                    compatibility * 100.0
                ),
            )
        } else {
            // Lower compatibility - use ULF as intermediate
            let ulf_confidence = self.calculate_ulf_confidence(source, target);
            (
                ConversionPath::ViaULF,
                ulf_confidence,
                format!(
                    "Using ULF intermediate format for lossless conversion ({:.0}% confidence)",
                    ulf_confidence * 100.0
                ),
            )
        };

        // Identify potential warnings
        let warnings = self.identify_warnings(source, target);

        // Generate alternatives
        let alternatives = self.generate_alternatives(source, target, &path);

        NegotiationResult {
            path,
            confidence,
            reasoning,
            warnings,
            alternatives,
        }
    }

    /// Calculates compatibility score between two formats
    fn calculate_compatibility(&self, source: LegalFormat, target: LegalFormat) -> f64 {
        use LegalFormat::*;

        // Perfect compatibility groups
        let xml_formats = vec![
            AkomaNtoso,
            LegalRuleML,
            LegalDocML,
            LKIF,
            LegalCite,
            MetaLex,
            Formex,
            Niem,
            RegML,
        ];
        let json_formats = vec![
            CommonForm,
            ClauseIo,
            FinReg,
            MiFID2,
            Basel3,
            SapLegal,
            SalesforceContract,
            DocuSign,
            MsWordLegal,
            PdfLegal,
        ];
        let dsl_formats = [Catala, Stipula, L4];
        let blockchain_formats = [Solidity, Vyper, Cadence, Move];
        let process_formats = [Bpmn, Dmn, Cmmn];
        let license_formats = [CreativeCommons, Spdx, Mpeg21Rel];

        // Same category = high compatibility
        if (xml_formats.contains(&source) && xml_formats.contains(&target))
            || (json_formats.contains(&source) && json_formats.contains(&target))
            || (dsl_formats.contains(&source) && dsl_formats.contains(&target))
            || (blockchain_formats.contains(&source) && blockchain_formats.contains(&target))
            || (process_formats.contains(&source) && process_formats.contains(&target))
            || (license_formats.contains(&source) && license_formats.contains(&target))
        {
            return 0.85;
        }

        // Cross-category compatibility
        match (source, target) {
            // DSL to blockchain - moderate compatibility
            (Catala | Stipula | L4, Solidity | Vyper | Cadence | Move) => 0.6,
            (Solidity | Vyper | Cadence | Move, Catala | Stipula | L4) => 0.6,

            // Process models to DSL - moderate
            (Bpmn | Dmn | Cmmn, Catala | Stipula | L4) => 0.5,

            // XML to JSON - structural similarity
            _ if xml_formats.contains(&source) && json_formats.contains(&target) => 0.55,
            _ if json_formats.contains(&source) && xml_formats.contains(&target) => 0.55,

            // Default - use ULF
            _ => 0.4,
        }
    }

    /// Calculates confidence when using ULF as intermediate
    fn calculate_ulf_confidence(&self, _source: LegalFormat, _target: LegalFormat) -> f64 {
        // ULF is designed for lossless conversion
        0.95
    }

    /// Identifies potential data loss warnings
    fn identify_warnings(&self, source: LegalFormat, target: LegalFormat) -> Vec<String> {
        use LegalFormat::*;
        let mut warnings = Vec::new();

        // Blockchain to non-blockchain conversions
        match (source, target) {
            (Solidity | Vyper | Cadence | Move, t)
                if !matches!(t, Solidity | Vyper | Cadence | Move) =>
            {
                warnings.push(
                    "Smart contract-specific features may not be fully represented".to_string(),
                );
            }
            (_, Solidity | Vyper | Cadence | Move) => {
                warnings.push("Target format expects blockchain-specific constructs".to_string());
            }
            _ => {}
        }

        // Process models have temporal semantics
        match source {
            Bpmn | Dmn | Cmmn => {
                if !matches!(target, Bpmn | Dmn | Cmmn) {
                    warnings
                        .push("Process flow and temporal semantics may be simplified".to_string());
                }
            }
            _ => {}
        }

        // License formats are highly specialized
        match (source, target) {
            (CreativeCommons | Spdx | Mpeg21Rel, t)
                if !matches!(t, CreativeCommons | Spdx | Mpeg21Rel) =>
            {
                warnings.push(
                    "License-specific rights expressions may not translate precisely".to_string(),
                );
            }
            _ => {}
        }

        warnings
    }

    /// Generates alternative conversion paths
    fn generate_alternatives(
        &self,
        source: LegalFormat,
        target: LegalFormat,
        current_path: &ConversionPath,
    ) -> Vec<(ConversionPath, f64)> {
        let mut alternatives = Vec::new();

        // If current recommendation is Direct, offer ULF as alternative
        if matches!(current_path, ConversionPath::Direct) {
            let ulf_confidence = self.calculate_ulf_confidence(source, target);
            alternatives.push((ConversionPath::ViaULF, ulf_confidence));
        }

        // If current recommendation is ULF, offer Direct as alternative (if compatible)
        if matches!(current_path, ConversionPath::ViaULF) {
            let direct_confidence = self.calculate_compatibility(source, target);
            if direct_confidence > 0.5 {
                alternatives.push((ConversionPath::Direct, direct_confidence));
            }
        }

        alternatives
    }

    /// Recommends best target format for a given source
    pub fn recommend_target_format(&self, source: LegalFormat) -> Vec<(LegalFormat, f64)> {
        use LegalFormat::*;

        let all_formats = vec![
            Catala,
            Stipula,
            L4,
            AkomaNtoso,
            LegalRuleML,
            LegalDocML,
            LKIF,
            LegalCite,
            MetaLex,
            Mpeg21Rel,
            CreativeCommons,
            Spdx,
            Legalis,
            Bpmn,
            Dmn,
            Cmmn,
            RuleML,
            Sbvr,
            OpenLaw,
            Cicero,
            CommonForm,
            ClauseIo,
            ContractExpress,
            Formex,
            Niem,
            FinReg,
            Xbrl,
            RegML,
            MiFID2,
            Basel3,
            SapLegal,
            SalesforceContract,
            DocuSign,
            MsWordLegal,
            PdfLegal,
            Solidity,
            Vyper,
            Cadence,
            Move,
        ];

        let mut recommendations: Vec<(LegalFormat, f64)> = all_formats
            .into_iter()
            .filter(|&f| f != source)
            .map(|f| (f, self.calculate_compatibility(source, f)))
            .collect();

        // Sort by compatibility score
        recommendations.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Return top 5
        recommendations.truncate(5);
        recommendations
    }
}

impl Default for FormatNegotiator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::ComparisonOp;

    #[test]
    fn test_create_ulf_document() {
        let doc = UniversalLegalDocument::new("test-1", "Test Document", LegalFormat::Legalis);
        assert_eq!(doc.ulf_version, ULF_VERSION);
        assert_eq!(doc.metadata.id, "test-1");
        assert_eq!(doc.metadata.title, "Test Document");
    }

    #[test]
    fn test_statute_to_ulf_conversion() {
        let effect = legalis_core::Effect::new(legalis_core::EffectType::Grant, "vote");
        let statute = Statute::new("voting-rights", "Voting Rights", effect).with_precondition(
            legalis_core::Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            },
        );

        let doc = UniversalLegalDocument::from_statute(&statute, LegalFormat::Legalis);

        assert_eq!(doc.provisions.len(), 1);
        assert_eq!(doc.provisions[0].id, "voting-rights");
        assert_eq!(doc.provisions[0].title, "Voting Rights");
        assert_eq!(doc.provisions[0].conditions.len(), 1);
    }

    #[test]
    fn test_ulf_to_statute_conversion() {
        let effect = legalis_core::Effect::new(legalis_core::EffectType::Grant, "vote");
        let statute = Statute::new("voting-rights", "Voting Rights", effect);

        let doc = UniversalLegalDocument::from_statute(&statute, LegalFormat::Legalis);
        let converted = doc.to_statute().unwrap();

        assert_eq!(converted.id, "voting-rights");
        assert_eq!(converted.title, "Voting Rights");
    }

    #[test]
    fn test_json_serialization() {
        let doc = UniversalLegalDocument::new("test-1", "Test", LegalFormat::Legalis);
        let json = doc.to_json().unwrap();
        assert!(json.contains("test-1"));
        assert!(json.contains("Test"));
    }

    #[test]
    fn test_json_deserialization() {
        let doc = UniversalLegalDocument::new("test-1", "Test", LegalFormat::Legalis);
        let json = doc.to_json().unwrap();
        let loaded = UniversalLegalDocument::from_json(&json).unwrap();
        assert_eq!(loaded.metadata.id, "test-1");
    }

    #[test]
    fn test_roundtrip_conversion() {
        let effect = legalis_core::Effect::new(legalis_core::EffectType::Grant, "vote");
        let original = Statute::new("test", "Test Statute", effect);

        let doc = UniversalLegalDocument::from_statute(&original, LegalFormat::Legalis);
        let converted = doc.to_statute().unwrap();

        assert_eq!(original.id, converted.id);
        assert_eq!(original.title, converted.title);
    }

    #[test]
    fn test_negotiator_same_format() {
        let negotiator = FormatNegotiator::new();
        let result = negotiator.negotiate(LegalFormat::Catala, LegalFormat::Catala);

        assert_eq!(result.path, ConversionPath::Direct);
        assert_eq!(result.confidence, 1.0);
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn test_negotiator_high_compatibility() {
        let negotiator = FormatNegotiator::new();
        let result = negotiator.negotiate(LegalFormat::AkomaNtoso, LegalFormat::LegalRuleML);

        assert_eq!(result.path, ConversionPath::Direct);
        assert!(result.confidence > 0.7);
        assert!(!result.alternatives.is_empty());
    }

    #[test]
    fn test_negotiator_low_compatibility() {
        let negotiator = FormatNegotiator::new();
        let result = negotiator.negotiate(LegalFormat::Solidity, LegalFormat::CreativeCommons);

        assert_eq!(result.path, ConversionPath::ViaULF);
        assert!(result.confidence >= 0.9);
        assert!(!result.warnings.is_empty());
    }

    #[test]
    fn test_negotiator_blockchain_warnings() {
        let negotiator = FormatNegotiator::new();
        let result = negotiator.negotiate(LegalFormat::Solidity, LegalFormat::Catala);

        assert!(!result.warnings.is_empty());
        assert!(result.warnings.iter().any(|w| w.contains("Smart contract")));
    }

    #[test]
    fn test_negotiator_recommendations() {
        let negotiator = FormatNegotiator::new();
        let recommendations = negotiator.recommend_target_format(LegalFormat::Solidity);

        assert_eq!(recommendations.len(), 5);
        assert!(recommendations[0].1 > recommendations[4].1); // Sorted by compatibility
    }

    #[test]
    fn test_negotiator_blockchain_compatibility() {
        let negotiator = FormatNegotiator::new();

        // Same blockchain category should have high compatibility
        let result = negotiator.negotiate(LegalFormat::Solidity, LegalFormat::Vyper);
        assert!(result.confidence > 0.8);
    }

    #[test]
    fn test_negotiator_alternatives() {
        let negotiator = FormatNegotiator::new();
        let result = negotiator.negotiate(LegalFormat::Catala, LegalFormat::L4);

        assert!(!result.alternatives.is_empty());
        // Should have both Direct and ViaULF as options
        assert!(
            result
                .alternatives
                .iter()
                .any(|(path, _)| matches!(path, ConversionPath::ViaULF))
        );
    }

    #[test]
    fn test_version_parsing() {
        let version = UlfVersion::parse("1.2.3").unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
        assert_eq!(version.to_string(), "1.2.3");
    }

    #[test]
    fn test_version_parsing_invalid() {
        assert!(UlfVersion::parse("1.2").is_err());
        assert!(UlfVersion::parse("1.2.3.4").is_err());
        assert!(UlfVersion::parse("invalid").is_err());
    }

    #[test]
    fn test_version_compatibility() {
        let v1_0_0 = UlfVersion::new(1, 0, 0);
        let v1_1_0 = UlfVersion::new(1, 1, 0);
        let v1_2_0 = UlfVersion::new(1, 2, 0);
        let v2_0_0 = UlfVersion::new(2, 0, 0);

        // Same major, higher minor is compatible
        assert!(v1_2_0.is_compatible_with(&v1_0_0));
        assert!(v1_1_0.is_compatible_with(&v1_0_0));

        // Lower minor is not compatible
        assert!(!v1_0_0.is_compatible_with(&v1_1_0));

        // Different major is not compatible
        assert!(!v2_0_0.is_compatible_with(&v1_0_0));
        assert!(!v1_0_0.is_compatible_with(&v2_0_0));
    }

    #[test]
    fn test_version_needs_migration() {
        let v1_0_0 = UlfVersion::new(1, 0, 0);
        let v1_1_0 = UlfVersion::new(1, 1, 0);

        assert!(v1_0_0.needs_migration(&v1_1_0));
        assert!(!v1_0_0.needs_migration(&v1_0_0));
    }

    #[test]
    fn test_ulf_features() {
        let v1_0 = UlfVersion::new(1, 0, 0);
        let features = UlfFeatures::for_version(&v1_0);

        assert!(features.supports_temporal_validity());
        assert!(features.supports_cross_references());
        assert!(features.supports_provenance());
        assert!(features.supports_extensions());
        assert!(features.supports_document_structure());
    }

    #[test]
    fn test_version_migration_same_version() {
        let mut doc = UniversalLegalDocument::new("test", "Test", LegalFormat::Legalis);
        doc.ulf_version = "1.0.0".to_string();

        let warnings = VersionMigrator::migrate(&mut doc, "1.0.0").unwrap();
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("No migration needed"));
    }

    #[test]
    fn test_version_validation() {
        let doc = UniversalLegalDocument::new("test", "Test", LegalFormat::Legalis);
        let warnings = VersionMigrator::validate_version(&doc).unwrap();

        // Should not have warnings for current version
        assert!(
            warnings.is_empty()
                || warnings
                    .iter()
                    .all(|w| !w.contains("not officially supported"))
        );
    }

    #[test]
    fn test_version_validation_unsupported() {
        let mut doc = UniversalLegalDocument::new("test", "Test", LegalFormat::Legalis);
        doc.ulf_version = "99.0.0".to_string();

        let warnings = VersionMigrator::validate_version(&doc).unwrap();
        assert!(!warnings.is_empty());
        assert!(
            warnings
                .iter()
                .any(|w| w.contains("not officially supported"))
        );
    }
}
