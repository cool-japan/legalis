//! Round-Trip Fidelity - Comprehensive fidelity analysis for format conversions.
//!
//! This module provides:
//! - Lossless round-trip verification for format conversions
//! - Detailed fidelity scoring with multiple metrics
//! - Conversion delta tracking to identify changes
//! - Format capability matrices to understand format limitations
//! - Automatic fallback strategies for unsupported features

use crate::{ConversionReport, InteropResult, LegalConverter, LegalFormat};
use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Fidelity score for a conversion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FidelityScore {
    /// Overall fidelity (0.0 - 1.0, 1.0 = perfect)
    pub overall: f64,
    /// Structure preservation score
    pub structure: f64,
    /// Metadata preservation score
    pub metadata: f64,
    /// Semantic preservation score
    pub semantic: f64,
    /// Syntax preservation score
    pub syntax: f64,
    /// Number of statutes preserved
    pub statutes_preserved: usize,
    /// Total number of statutes
    pub total_statutes: usize,
}

impl FidelityScore {
    /// Creates a new fidelity score
    pub fn new() -> Self {
        Self {
            overall: 1.0,
            structure: 1.0,
            metadata: 1.0,
            semantic: 1.0,
            syntax: 1.0,
            statutes_preserved: 0,
            total_statutes: 0,
        }
    }

    /// Calculates overall score from component scores
    pub fn calculate_overall(&mut self) {
        self.overall = (self.structure + self.metadata + self.semantic + self.syntax) / 4.0;
    }

    /// Returns true if the conversion is considered lossless (overall >= 0.95)
    pub fn is_lossless(&self) -> bool {
        self.overall >= 0.95
    }

    /// Returns true if the conversion is high fidelity (overall >= 0.8)
    pub fn is_high_fidelity(&self) -> bool {
        self.overall >= 0.8
    }

    /// Returns a human-readable grade (A+ to F)
    pub fn grade(&self) -> &'static str {
        match self.overall {
            x if x >= 0.97 => "A+",
            x if x >= 0.93 => "A",
            x if x >= 0.90 => "A-",
            x if x >= 0.87 => "B+",
            x if x >= 0.83 => "B",
            x if x >= 0.80 => "B-",
            x if x >= 0.77 => "C+",
            x if x >= 0.73 => "C",
            x if x >= 0.70 => "C-",
            x if x >= 0.60 => "D",
            _ => "F",
        }
    }
}

impl Default for FidelityScore {
    fn default() -> Self {
        Self::new()
    }
}

/// Delta representing changes between original and round-trip
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionDelta {
    /// Statutes added during round-trip
    pub added_statutes: Vec<String>,
    /// Statutes removed during round-trip
    pub removed_statutes: Vec<String>,
    /// Statutes modified during round-trip
    pub modified_statutes: Vec<StatuteModification>,
    /// Metadata changes
    pub metadata_changes: Vec<MetadataChange>,
}

impl ConversionDelta {
    /// Creates a new empty delta
    pub fn new() -> Self {
        Self {
            added_statutes: Vec::new(),
            removed_statutes: Vec::new(),
            modified_statutes: Vec::new(),
            metadata_changes: Vec::new(),
        }
    }

    /// Returns true if there are no changes
    pub fn is_empty(&self) -> bool {
        self.added_statutes.is_empty()
            && self.removed_statutes.is_empty()
            && self.modified_statutes.is_empty()
            && self.metadata_changes.is_empty()
    }

    /// Returns total number of changes
    pub fn total_changes(&self) -> usize {
        self.added_statutes.len()
            + self.removed_statutes.len()
            + self.modified_statutes.len()
            + self.metadata_changes.len()
    }
}

impl Default for ConversionDelta {
    fn default() -> Self {
        Self::new()
    }
}

/// Modification to a statute during conversion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteModification {
    /// Statute ID
    pub statute_id: String,
    /// Field that was modified
    pub field: String,
    /// Original value
    pub original: String,
    /// New value
    pub modified: String,
}

/// Metadata change during conversion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataChange {
    /// Metadata key
    pub key: String,
    /// Change type
    pub change_type: MetadataChangeType,
    /// Original value (if any)
    pub original: Option<String>,
    /// New value (if any)
    pub new: Option<String>,
}

/// Type of metadata change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetadataChangeType {
    /// Metadata was added
    Added,
    /// Metadata was removed
    Removed,
    /// Metadata was modified
    Modified,
}

/// Round-trip verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundTripVerification {
    /// Source format
    pub source_format: LegalFormat,
    /// Target format
    pub target_format: LegalFormat,
    /// Fidelity score
    pub fidelity: FidelityScore,
    /// Conversion delta
    pub delta: ConversionDelta,
    /// Import report
    pub import_report: ConversionReport,
    /// Export report
    pub export_report: ConversionReport,
    /// Re-import report
    pub reimport_report: ConversionReport,
    /// Whether the round-trip was successful
    pub success: bool,
}

/// Format capability matrix
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatCapabilityMatrix {
    capabilities: HashMap<LegalFormat, FormatCapabilities>,
}

impl FormatCapabilityMatrix {
    /// Creates a new capability matrix
    pub fn new() -> Self {
        let mut matrix = Self {
            capabilities: HashMap::new(),
        };
        matrix.initialize_capabilities();
        matrix
    }

    /// Initializes capabilities for all formats
    fn initialize_capabilities(&mut self) {
        // Define capabilities for each format
        let formats = vec![
            (
                LegalFormat::Catala,
                vec!["scopes", "exceptions", "literate", "functions"],
            ),
            (
                LegalFormat::Stipula,
                vec!["parties", "assets", "state_machines", "temporal"],
            ),
            (
                LegalFormat::L4,
                vec!["deontic_logic", "temporal_operators", "decision_tables"],
            ),
            (
                LegalFormat::AkomaNtoso,
                vec!["xml_structure", "hierarchical", "amendments"],
            ),
            (
                LegalFormat::LegalRuleML,
                vec!["rules", "premises", "conclusions", "defeasibility"],
            ),
            (
                LegalFormat::LKIF,
                vec!["arguments", "sources", "rules", "theories"],
            ),
            (
                LegalFormat::Bpmn,
                vec!["processes", "tasks", "gateways", "events"],
            ),
            (
                LegalFormat::Dmn,
                vec!["decisions", "decision_tables", "hit_policies"],
            ),
            (
                LegalFormat::OpenLaw,
                vec!["templates", "variables", "conditionals"],
            ),
            (
                LegalFormat::Cicero,
                vec!["smart_contracts", "templates", "accordproject"],
            ),
        ];

        for (format, features) in formats {
            let caps = FormatCapabilities {
                format,
                supported_features: features.iter().map(|s| s.to_string()).collect(),
                limitations: Vec::new(),
            };
            self.capabilities.insert(format, caps);
        }
    }

    /// Gets capabilities for a format
    pub fn get_capabilities(&self, format: LegalFormat) -> Option<&FormatCapabilities> {
        self.capabilities.get(&format)
    }

    /// Checks if a feature is supported by a format
    pub fn supports_feature(&self, format: LegalFormat, feature: &str) -> bool {
        self.capabilities
            .get(&format)
            .map(|caps| caps.supported_features.contains(&feature.to_string()))
            .unwrap_or(false)
    }

    /// Gets incompatible features between two formats
    pub fn get_incompatibilities(&self, source: LegalFormat, target: LegalFormat) -> Vec<String> {
        let source_caps = match self.capabilities.get(&source) {
            Some(caps) => caps,
            None => return Vec::new(),
        };

        let target_caps = match self.capabilities.get(&target) {
            Some(caps) => caps,
            None => return source_caps.supported_features.clone(),
        };

        source_caps
            .supported_features
            .iter()
            .filter(|feature| !target_caps.supported_features.contains(feature))
            .cloned()
            .collect()
    }

    /// Recommends best target format for a source format
    pub fn recommend_target(
        &self,
        source: LegalFormat,
        required_features: &[String],
    ) -> Option<LegalFormat> {
        let mut best_match: Option<(LegalFormat, usize)> = None;

        for (format, caps) in &self.capabilities {
            if *format == source {
                continue;
            }

            let matching_features = required_features
                .iter()
                .filter(|feature| caps.supported_features.contains(feature))
                .count();

            match best_match {
                None => best_match = Some((*format, matching_features)),
                Some((_, count)) if matching_features > count => {
                    best_match = Some((*format, matching_features));
                }
                _ => {}
            }
        }

        best_match.map(|(format, _)| format)
    }
}

impl Default for FormatCapabilityMatrix {
    fn default() -> Self {
        Self::new()
    }
}

/// Capabilities of a specific format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatCapabilities {
    /// Format
    pub format: LegalFormat,
    /// Supported features
    pub supported_features: Vec<String>,
    /// Known limitations
    pub limitations: Vec<String>,
}

/// Fallback strategy for unsupported features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FallbackStrategy {
    /// Skip the unsupported feature
    Skip,
    /// Use a default value
    UseDefault(String),
    /// Convert to closest equivalent
    Approximate,
    /// Store as metadata/comment
    PreserveAsMetadata,
    /// Fail the conversion
    Fail,
}

/// Fallback configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FallbackConfig {
    /// Default strategy
    pub default_strategy: FallbackStrategy,
    /// Feature-specific strategies
    pub feature_strategies: HashMap<String, FallbackStrategy>,
}

impl FallbackConfig {
    /// Creates a new fallback config with sensible defaults
    pub fn new() -> Self {
        Self {
            default_strategy: FallbackStrategy::PreserveAsMetadata,
            feature_strategies: HashMap::new(),
        }
    }

    /// Sets strategy for a specific feature
    pub fn set_strategy(&mut self, feature: String, strategy: FallbackStrategy) {
        self.feature_strategies.insert(feature, strategy);
    }

    /// Gets strategy for a feature
    pub fn get_strategy(&self, feature: &str) -> &FallbackStrategy {
        self.feature_strategies
            .get(feature)
            .unwrap_or(&self.default_strategy)
    }
}

impl Default for FallbackConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Round-trip fidelity analyzer
pub struct FidelityAnalyzer {
    converter: LegalConverter,
    capability_matrix: FormatCapabilityMatrix,
    fallback_config: FallbackConfig,
}

impl FidelityAnalyzer {
    /// Creates a new fidelity analyzer
    pub fn new() -> Self {
        Self {
            converter: LegalConverter::new(),
            capability_matrix: FormatCapabilityMatrix::new(),
            fallback_config: FallbackConfig::new(),
        }
    }

    /// Sets fallback configuration
    pub fn set_fallback_config(&mut self, config: FallbackConfig) {
        self.fallback_config = config;
    }

    /// Performs round-trip verification
    pub fn verify_roundtrip(
        &mut self,
        source: &str,
        source_format: LegalFormat,
        target_format: LegalFormat,
    ) -> InteropResult<RoundTripVerification> {
        // Import from source
        let (original_statutes, import_report) = self.converter.import(source, source_format)?;

        // Export to target
        let (target_output, export_report) =
            self.converter.export(&original_statutes, target_format)?;

        // Re-import from target
        let (roundtrip_statutes, reimport_report) =
            self.converter.import(&target_output, target_format)?;

        // Calculate fidelity
        let fidelity = self.calculate_fidelity(&original_statutes, &roundtrip_statutes);

        // Calculate delta
        let delta = self.calculate_delta(&original_statutes, &roundtrip_statutes);

        // Determine success
        let success = fidelity.is_high_fidelity() && delta.total_changes() < 10;

        Ok(RoundTripVerification {
            source_format,
            target_format,
            fidelity,
            delta,
            import_report,
            export_report,
            reimport_report,
            success,
        })
    }

    /// Calculates fidelity score
    fn calculate_fidelity(&self, original: &[Statute], roundtrip: &[Statute]) -> FidelityScore {
        let mut score = FidelityScore::new();
        score.total_statutes = original.len();
        score.statutes_preserved = roundtrip.len();

        // Structure score: how many statutes preserved
        score.structure = if original.is_empty() {
            1.0
        } else {
            roundtrip.len() as f64 / original.len() as f64
        };

        // Metadata score: check jurisdiction preservation
        let mut metadata_matches = 0;
        let mut total_metadata = 0;
        for (orig, rt) in original.iter().zip(roundtrip.iter()) {
            total_metadata += 1; // jurisdiction
            if orig.jurisdiction == rt.jurisdiction {
                metadata_matches += 1;
            }
        }
        score.metadata = if total_metadata == 0 {
            1.0
        } else {
            metadata_matches as f64 / total_metadata as f64
        };

        // Semantic score: check effect types and preconditions
        let mut semantic_matches = 0;
        let mut total_semantic = 0;
        for (orig, rt) in original.iter().zip(roundtrip.iter()) {
            total_semantic += 2; // effect type + precondition count
            if orig.effect.effect_type == rt.effect.effect_type {
                semantic_matches += 1;
            }
            if orig.preconditions.len() == rt.preconditions.len() {
                semantic_matches += 1;
            }
        }
        score.semantic = if total_semantic == 0 {
            1.0
        } else {
            semantic_matches as f64 / total_semantic as f64
        };

        // Syntax score: based on title and ID preservation
        let mut syntax_matches = 0;
        let mut total_syntax = 0;
        for (orig, rt) in original.iter().zip(roundtrip.iter()) {
            total_syntax += 2;
            if orig.id == rt.id || orig.id.to_lowercase() == rt.id.to_lowercase() {
                syntax_matches += 1;
            }
            if orig.title == rt.title {
                syntax_matches += 1;
            }
        }
        score.syntax = if total_syntax == 0 {
            1.0
        } else {
            syntax_matches as f64 / total_syntax as f64
        };

        score.calculate_overall();
        score
    }

    /// Calculates conversion delta
    fn calculate_delta(&self, original: &[Statute], roundtrip: &[Statute]) -> ConversionDelta {
        let mut delta = ConversionDelta::new();

        // Build ID sets
        let original_ids: HashSet<_> = original.iter().map(|s| &s.id).collect();
        let roundtrip_ids: HashSet<_> = roundtrip.iter().map(|s| &s.id).collect();

        // Find added statutes
        for id in roundtrip_ids.difference(&original_ids) {
            delta.added_statutes.push((*id).clone());
        }

        // Find removed statutes
        for id in original_ids.difference(&roundtrip_ids) {
            delta.removed_statutes.push((*id).clone());
        }

        // Find modified statutes
        for (orig, rt) in original.iter().zip(roundtrip.iter()) {
            if orig.title != rt.title {
                delta.modified_statutes.push(StatuteModification {
                    statute_id: orig.id.clone(),
                    field: "title".to_string(),
                    original: orig.title.clone(),
                    modified: rt.title.clone(),
                });
            }

            if orig.effect.description != rt.effect.description {
                delta.modified_statutes.push(StatuteModification {
                    statute_id: orig.id.clone(),
                    field: "effect.description".to_string(),
                    original: orig.effect.description.clone(),
                    modified: rt.effect.description.clone(),
                });
            }
        }

        delta
    }

    /// Gets capability matrix
    pub fn capability_matrix(&self) -> &FormatCapabilityMatrix {
        &self.capability_matrix
    }

    /// Checks if conversion is recommended
    pub fn is_conversion_recommended(&self, source: LegalFormat, target: LegalFormat) -> bool {
        let incompatibilities = self.capability_matrix.get_incompatibilities(source, target);
        incompatibilities.len() < 5 // Recommend if fewer than 5 incompatibilities
    }

    /// Gets recommended formats for a source
    pub fn get_recommended_targets(&self, source: LegalFormat) -> Vec<LegalFormat> {
        let all_formats = vec![
            LegalFormat::Catala,
            LegalFormat::Stipula,
            LegalFormat::L4,
            LegalFormat::AkomaNtoso,
            LegalFormat::LegalRuleML,
        ];

        all_formats
            .into_iter()
            .filter(|&target| target != source && self.is_conversion_recommended(source, target))
            .collect()
    }
}

impl Default for FidelityAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

    #[test]
    fn test_fidelity_score_new() {
        let score = FidelityScore::new();
        assert_eq!(score.overall, 1.0);
        assert_eq!(score.structure, 1.0);
        assert_eq!(score.metadata, 1.0);
        assert_eq!(score.semantic, 1.0);
        assert_eq!(score.syntax, 1.0);
    }

    #[test]
    fn test_fidelity_score_calculate_overall() {
        let mut score = FidelityScore::new();
        score.structure = 0.9;
        score.metadata = 0.8;
        score.semantic = 0.95;
        score.syntax = 0.85;
        score.calculate_overall();
        assert!((score.overall - 0.875).abs() < 0.001);
    }

    #[test]
    fn test_fidelity_score_is_lossless() {
        let mut score = FidelityScore::new();
        assert!(score.is_lossless());

        score.overall = 0.94;
        assert!(!score.is_lossless());

        score.overall = 0.96;
        assert!(score.is_lossless());
    }

    #[test]
    fn test_fidelity_score_grade() {
        let mut score = FidelityScore::new();
        score.overall = 0.98;
        assert_eq!(score.grade(), "A+");

        score.overall = 0.94;
        assert_eq!(score.grade(), "A");

        score.overall = 0.85;
        assert_eq!(score.grade(), "B");

        score.overall = 0.50;
        assert_eq!(score.grade(), "F");
    }

    #[test]
    fn test_conversion_delta_new() {
        let delta = ConversionDelta::new();
        assert!(delta.is_empty());
        assert_eq!(delta.total_changes(), 0);
    }

    #[test]
    fn test_conversion_delta_changes() {
        let mut delta = ConversionDelta::new();
        delta.added_statutes.push("new-statute".to_string());
        delta.removed_statutes.push("old-statute".to_string());

        assert!(!delta.is_empty());
        assert_eq!(delta.total_changes(), 2);
    }

    #[test]
    fn test_format_capability_matrix_new() {
        let matrix = FormatCapabilityMatrix::new();
        assert!(!matrix.capabilities.is_empty());
    }

    #[test]
    fn test_format_capability_matrix_supports_feature() {
        let matrix = FormatCapabilityMatrix::new();
        assert!(matrix.supports_feature(LegalFormat::Catala, "scopes"));
        assert!(matrix.supports_feature(LegalFormat::Stipula, "parties"));
        assert!(!matrix.supports_feature(LegalFormat::Catala, "nonexistent"));
    }

    #[test]
    fn test_format_capability_matrix_incompatibilities() {
        let matrix = FormatCapabilityMatrix::new();
        let incompatibilities =
            matrix.get_incompatibilities(LegalFormat::Catala, LegalFormat::Stipula);
        assert!(!incompatibilities.is_empty());
    }

    #[test]
    fn test_fallback_config_new() {
        let config = FallbackConfig::new();
        matches!(
            config.default_strategy,
            FallbackStrategy::PreserveAsMetadata
        );
    }

    #[test]
    fn test_fallback_config_set_strategy() {
        let mut config = FallbackConfig::new();
        config.set_strategy("scopes".to_string(), FallbackStrategy::Skip);

        matches!(config.get_strategy("scopes"), FallbackStrategy::Skip);
    }

    #[test]
    fn test_fidelity_analyzer_new() {
        let analyzer = FidelityAnalyzer::new();
        assert!(!analyzer.capability_matrix.capabilities.is_empty());
    }

    #[test]
    fn test_fidelity_analyzer_verify_roundtrip() {
        let mut analyzer = FidelityAnalyzer::new();

        let l4_source = "RULE TestRule WHEN age >= 18 THEN Person MAY vote";

        let result = analyzer.verify_roundtrip(l4_source, LegalFormat::L4, LegalFormat::Catala);

        assert!(result.is_ok());
        let verification = result.unwrap();
        assert_eq!(verification.source_format, LegalFormat::L4);
        assert_eq!(verification.target_format, LegalFormat::Catala);
    }

    #[test]
    fn test_fidelity_analyzer_calculate_fidelity() {
        let analyzer = FidelityAnalyzer::new();

        let original = vec![Statute::new(
            "test",
            "Test Statute",
            Effect::new(EffectType::Grant, "test"),
        )];

        let roundtrip = vec![Statute::new(
            "test",
            "Test Statute",
            Effect::new(EffectType::Grant, "test"),
        )];

        let fidelity = analyzer.calculate_fidelity(&original, &roundtrip);
        assert!(fidelity.is_lossless());
        assert_eq!(fidelity.grade(), "A+");
    }

    #[test]
    fn test_fidelity_analyzer_calculate_delta() {
        let analyzer = FidelityAnalyzer::new();

        let original = vec![Statute::new(
            "test1",
            "Test 1",
            Effect::new(EffectType::Grant, "test"),
        )];

        let mut roundtrip = vec![Statute::new(
            "test1",
            "Test One",
            Effect::new(EffectType::Grant, "test"),
        )];

        let delta = analyzer.calculate_delta(&original, &roundtrip);
        assert_eq!(delta.modified_statutes.len(), 1);
        assert_eq!(delta.modified_statutes[0].field, "title");

        // Test added/removed
        roundtrip.push(Statute::new(
            "test2",
            "Test 2",
            Effect::new(EffectType::Grant, "test2"),
        ));

        let delta2 = analyzer.calculate_delta(&original, &roundtrip);
        assert_eq!(delta2.added_statutes.len(), 1);
    }

    #[test]
    fn test_fidelity_analyzer_is_conversion_recommended() {
        let analyzer = FidelityAnalyzer::new();

        let recommended = analyzer.is_conversion_recommended(LegalFormat::Catala, LegalFormat::L4);
        assert!(recommended); // Should be true since both support legal logic
    }

    #[test]
    fn test_fidelity_analyzer_get_recommended_targets() {
        let analyzer = FidelityAnalyzer::new();
        let targets = analyzer.get_recommended_targets(LegalFormat::Catala);
        assert!(!targets.is_empty());
        assert!(!targets.contains(&LegalFormat::Catala)); // Should not recommend itself
    }

    #[test]
    fn test_format_capabilities_serialization() {
        let caps = FormatCapabilities {
            format: LegalFormat::Catala,
            supported_features: vec!["scopes".to_string(), "exceptions".to_string()],
            limitations: vec!["no_blockchain".to_string()],
        };

        let json = serde_json::to_string(&caps).unwrap();
        let deserialized: FormatCapabilities = serde_json::from_str(&json).unwrap();

        assert_eq!(caps.format, deserialized.format);
        assert_eq!(
            caps.supported_features.len(),
            deserialized.supported_features.len()
        );
    }

    #[test]
    fn test_statute_modification() {
        let modification = StatuteModification {
            statute_id: "test-1".to_string(),
            field: "title".to_string(),
            original: "Original Title".to_string(),
            modified: "Modified Title".to_string(),
        };

        assert_eq!(modification.statute_id, "test-1");
        assert_eq!(modification.field, "title");
    }

    #[test]
    fn test_metadata_change() {
        let change = MetadataChange {
            key: "author".to_string(),
            change_type: MetadataChangeType::Modified,
            original: Some("John".to_string()),
            new: Some("Jane".to_string()),
        };

        assert_eq!(change.key, "author");
        matches!(change.change_type, MetadataChangeType::Modified);
    }

    #[test]
    fn test_fallback_strategy_skip() {
        let strategy = FallbackStrategy::Skip;
        let json = serde_json::to_string(&strategy).unwrap();
        assert!(json.contains("Skip"));
    }

    #[test]
    fn test_format_capability_matrix_recommend_target() {
        let matrix = FormatCapabilityMatrix::new();
        let required = vec!["scopes".to_string(), "exceptions".to_string()];

        let recommended = matrix.recommend_target(LegalFormat::Stipula, &required);
        assert!(recommended.is_some());
    }

    #[test]
    fn test_fidelity_score_high_fidelity() {
        let mut score = FidelityScore::new();
        score.overall = 0.85;
        assert!(score.is_high_fidelity());

        score.overall = 0.75;
        assert!(!score.is_high_fidelity());
    }

    #[test]
    fn test_perfect_fidelity() {
        let analyzer = FidelityAnalyzer::new();

        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "test"))
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            });

        let fidelity = analyzer.calculate_fidelity(&[statute.clone()], &[statute]);
        assert_eq!(fidelity.overall, 1.0);
        assert!(fidelity.is_lossless());
        assert_eq!(fidelity.grade(), "A+");
    }
}
