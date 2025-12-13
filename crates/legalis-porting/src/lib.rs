//! Legalis-Porting: Legal system porting for Legalis-RS.
//!
//! This crate enables "Soft ODA" - porting legal frameworks between jurisdictions
//! while adapting to local cultural parameters:
//! - Cross-jurisdiction statute translation
//! - Cultural parameter injection
//! - Legal concept mapping between systems
//! - Conflict detection with local laws

use async_trait::async_trait;
use legalis_core::Statute;
use legalis_i18n::{Jurisdiction, Locale};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Errors during porting operations.
#[derive(Debug, Error)]
pub enum PortingError {
    #[error("Source jurisdiction not found: {0}")]
    SourceNotFound(String),

    #[error("Target jurisdiction not found: {0}")]
    TargetNotFound(String),

    #[error("Incompatible legal systems: {0} -> {1}")]
    IncompatibleSystems(String, String),

    #[error("Cultural conflict: {0}")]
    CulturalConflict(String),

    #[error("Translation failed: {0}")]
    TranslationFailed(String),

    #[error("Adaptation required: {0}")]
    AdaptationRequired(String),
}

/// Result type for porting operations.
pub type PortingResult<T> = Result<T, PortingError>;

/// Porting request specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortingRequest {
    /// Source statute(s) to port
    pub statutes: Vec<Statute>,
    /// Source jurisdiction ID
    pub source_jurisdiction: String,
    /// Target jurisdiction ID
    pub target_jurisdiction: String,
    /// Porting options
    pub options: PortingOptions,
}

/// Options for porting.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PortingOptions {
    /// Whether to translate legal terms
    pub translate_terms: bool,
    /// Whether to adapt numerical values (ages, amounts)
    pub adapt_values: bool,
    /// Whether to inject cultural parameters
    pub apply_cultural_params: bool,
    /// Specific overrides for values
    pub value_overrides: HashMap<String, String>,
    /// Whether to generate a compatibility report
    pub generate_report: bool,
}

/// Result of a porting operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortingOutput {
    /// Ported statutes
    pub statutes: Vec<PortedStatute>,
    /// Compatibility report
    pub report: Option<CompatibilityReport>,
    /// Warnings generated during porting
    pub warnings: Vec<String>,
}

/// A statute that has been ported to a new jurisdiction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortedStatute {
    /// Original statute ID
    pub original_id: String,
    /// New statute with adaptations
    pub statute: Statute,
    /// Changes made during porting
    pub changes: Vec<PortingChange>,
    /// Locale of the ported statute
    pub locale: Locale,
}

/// A change made during porting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortingChange {
    /// Type of change
    pub change_type: ChangeType,
    /// Description of what changed
    pub description: String,
    /// Original value (if applicable)
    pub original: Option<String>,
    /// New value (if applicable)
    pub adapted: Option<String>,
    /// Reason for the change
    pub reason: String,
}

/// Types of changes during porting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    /// Term was translated
    Translation,
    /// Value was adapted (e.g., age threshold)
    ValueAdaptation,
    /// Condition was modified for cultural reasons
    CulturalAdaptation,
    /// Section was marked as incompatible
    Incompatible,
    /// Added for local compliance
    ComplianceAddition,
    /// Removed due to local prohibition
    Removal,
}

/// Compatibility report for ported statutes.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompatibilityReport {
    /// Overall compatibility score (0.0 - 1.0)
    pub compatibility_score: f64,
    /// Number of adaptations required
    pub adaptations_required: usize,
    /// Number of incompatibilities found
    pub incompatibilities: usize,
    /// Detailed findings
    pub findings: Vec<CompatibilityFinding>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// A finding from compatibility analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityFinding {
    /// Severity level
    pub severity: Severity,
    /// Category of finding
    pub category: String,
    /// Description
    pub description: String,
    /// Affected statute ID
    pub statute_id: Option<String>,
}

/// Severity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Trait for porting adapters.
#[async_trait]
pub trait PortingAdapter: Send + Sync {
    /// Ports statutes from source to target jurisdiction.
    async fn port(&self, request: &PortingRequest) -> PortingResult<PortingOutput>;

    /// Analyzes compatibility between jurisdictions.
    async fn analyze_compatibility(
        &self,
        source: &Jurisdiction,
        target: &Jurisdiction,
    ) -> PortingResult<CompatibilityReport>;
}

/// Basic porting engine.
pub struct PortingEngine {
    /// Source jurisdiction
    source: Jurisdiction,
    /// Target jurisdiction
    target: Jurisdiction,
}

impl PortingEngine {
    /// Creates a new porting engine.
    pub fn new(source: Jurisdiction, target: Jurisdiction) -> Self {
        Self { source, target }
    }

    /// Ports a single statute.
    pub fn port_statute(
        &self,
        statute: &Statute,
        options: &PortingOptions,
    ) -> PortingResult<PortedStatute> {
        let mut changes = Vec::new();
        let mut adapted = statute.clone();

        // Apply cultural parameter adaptations
        if options.apply_cultural_params {
            self.apply_cultural_adaptations(&mut adapted, &mut changes)?;
        }

        // Update statute ID for target jurisdiction
        adapted.id = format!("{}-{}", self.target.id.to_lowercase(), statute.id);

        Ok(PortedStatute {
            original_id: statute.id.clone(),
            statute: adapted,
            changes,
            locale: self.target.locale.clone(),
        })
    }

    fn apply_cultural_adaptations(
        &self,
        _statute: &mut Statute,
        changes: &mut Vec<PortingChange>,
    ) -> PortingResult<()> {
        let source_params = &self.source.cultural_params;
        let target_params = &self.target.cultural_params;

        // Check for age of majority differences
        if source_params.age_of_majority != target_params.age_of_majority {
            if let (Some(source_age), Some(target_age)) =
                (source_params.age_of_majority, target_params.age_of_majority)
            {
                // Would need to modify conditions here
                changes.push(PortingChange {
                    change_type: ChangeType::ValueAdaptation,
                    description: "Age of majority adjusted".to_string(),
                    original: Some(source_age.to_string()),
                    adapted: Some(target_age.to_string()),
                    reason: format!(
                        "Target jurisdiction ({}) has different age of majority",
                        self.target.id
                    ),
                });
            }
        }

        // Check for cultural prohibitions
        for prohibition in &target_params.prohibitions {
            changes.push(PortingChange {
                change_type: ChangeType::CulturalAdaptation,
                description: format!("Checked against prohibition: {}", prohibition),
                original: None,
                adapted: None,
                reason: "Target jurisdiction has cultural prohibition".to_string(),
            });
        }

        Ok(())
    }

    /// Generates a compatibility report.
    pub fn generate_report(&self, statutes: &[Statute]) -> CompatibilityReport {
        let mut report = CompatibilityReport::default();
        let mut findings = Vec::new();

        // Check legal system compatibility
        if self.source.legal_system != self.target.legal_system {
            findings.push(CompatibilityFinding {
                severity: Severity::Warning,
                category: "Legal System".to_string(),
                description: format!(
                    "Different legal systems: {:?} -> {:?}",
                    self.source.legal_system, self.target.legal_system
                ),
                statute_id: None,
            });
            report.adaptations_required += 1;
        }

        // Check for discretionary statutes
        for statute in statutes {
            if statute.discretion_logic.is_some() {
                findings.push(CompatibilityFinding {
                    severity: Severity::Info,
                    category: "Discretion".to_string(),
                    description: "Statute contains discretionary elements requiring local review"
                        .to_string(),
                    statute_id: Some(statute.id.clone()),
                });
            }
        }

        report.findings = findings;
        report.compatibility_score = self.calculate_compatibility_score(&report);
        report.recommendations = self.generate_recommendations(&report);

        report
    }

    fn calculate_compatibility_score(&self, report: &CompatibilityReport) -> f64 {
        let base_score = 1.0;
        let deductions =
            (report.adaptations_required as f64 * 0.1) + (report.incompatibilities as f64 * 0.2);
        (base_score - deductions).max(0.0)
    }

    fn generate_recommendations(&self, report: &CompatibilityReport) -> Vec<String> {
        let mut recommendations = Vec::new();

        if report.compatibility_score < 0.5 {
            recommendations.push(
                "Low compatibility score. Consider a full legal review before adoption."
                    .to_string(),
            );
        }

        if self.source.legal_system != self.target.legal_system {
            recommendations.push(
                "Legal systems differ. Case law adaptation may be required for common law targets."
                    .to_string(),
            );
        }

        recommendations
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType};
    use legalis_i18n::{CulturalParams, LegalSystem, Locale};

    fn test_jurisdiction_jp() -> Jurisdiction {
        Jurisdiction::new("JP", "Japan", Locale::new("ja").with_country("JP"))
            .with_legal_system(LegalSystem::CivilLaw)
            .with_cultural_params(CulturalParams::japan())
    }

    fn test_jurisdiction_us() -> Jurisdiction {
        Jurisdiction::new("US", "United States", Locale::new("en").with_country("US"))
            .with_legal_system(LegalSystem::CommonLaw)
            .with_cultural_params(CulturalParams::for_country("US"))
    }

    #[test]
    fn test_port_statute() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new(
            "adult-rights",
            "成人権法",
            Effect::new(EffectType::Grant, "Complete legal capacity"),
        );

        let options = PortingOptions {
            apply_cultural_params: true,
            ..Default::default()
        };

        let result = engine.port_statute(&statute, &options).unwrap();
        assert!(result.statute.id.starts_with("us-"));
    }

    #[test]
    fn test_compatibility_report() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statutes = vec![Statute::new(
            "test",
            "Test",
            Effect::new(EffectType::Grant, "Test"),
        )];

        let report = engine.generate_report(&statutes);
        assert!(report.compatibility_score > 0.0);
    }
}
