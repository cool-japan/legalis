//! Quality assessment for legal document conversions.
//!
//! This module provides tools to measure and analyze the quality of conversions
//! between different legal document formats, including:
//! - Semantic loss quantification
//! - Structure preservation scoring
//! - Metadata completeness analysis
//! - Round-trip fidelity testing
//! - Conversion confidence calibration

use crate::{ConversionReport, InteropResult, LegalConverter, LegalFormat};
use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Quality assessment for a conversion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityAssessment {
    /// Semantic loss percentage (0-100, where 0 is lossless)
    pub semantic_loss_percent: f64,
    /// Structure preservation score (0-100, where 100 is perfect)
    pub structure_score: f64,
    /// Metadata completeness score (0-100, where 100 is complete)
    pub metadata_score: f64,
    /// Round-trip fidelity score (0-100, where 100 is perfect)
    pub roundtrip_score: f64,
    /// Calibrated conversion confidence (0-100)
    pub confidence: f64,
    /// Detailed breakdown of quality metrics
    pub breakdown: QualityBreakdown,
}

/// Detailed breakdown of quality metrics.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QualityBreakdown {
    /// Preconditions preserved count
    pub preconditions_preserved: usize,
    /// Preconditions total count
    pub preconditions_total: usize,
    /// Effects preserved count
    pub effects_preserved: usize,
    /// Effects total count
    pub effects_total: usize,
    /// Metadata fields preserved
    pub metadata_fields_preserved: usize,
    /// Metadata fields total
    pub metadata_fields_total: usize,
    /// Structural elements preserved
    pub structure_elements_preserved: usize,
    /// Structural elements total
    pub structure_elements_total: usize,
    /// Warnings encountered
    pub warnings: Vec<String>,
    /// Unsupported features
    pub unsupported_features: Vec<String>,
}

/// Quality analyzer for legal document conversions.
pub struct QualityAnalyzer {
    converter: LegalConverter,
}

impl QualityAnalyzer {
    /// Creates a new quality analyzer.
    pub fn new() -> Self {
        Self {
            converter: LegalConverter::new(),
        }
    }

    /// Creates a new quality analyzer with caching enabled.
    pub fn with_cache(cache_size: usize) -> Self {
        Self {
            converter: LegalConverter::with_cache(cache_size),
        }
    }

    /// Analyzes conversion quality between two formats.
    pub fn analyze_conversion(
        &mut self,
        source: &str,
        source_format: LegalFormat,
        target_format: LegalFormat,
    ) -> InteropResult<QualityAssessment> {
        // Import source
        let (source_statutes, import_report) = self.converter.import(source, source_format)?;

        // Export to target
        let (target_output, export_report) =
            self.converter.export(&source_statutes, target_format)?;

        // Re-import from target (for round-trip analysis)
        let (roundtrip_statutes, reimport_report) =
            self.converter.import(&target_output, target_format)?;

        // Calculate quality metrics
        let mut breakdown = QualityBreakdown::default();
        breakdown.warnings.extend(import_report.warnings.clone());
        breakdown.warnings.extend(export_report.warnings.clone());
        breakdown.warnings.extend(reimport_report.warnings.clone());
        breakdown
            .unsupported_features
            .extend(import_report.unsupported_features.clone());
        breakdown
            .unsupported_features
            .extend(export_report.unsupported_features.clone());

        // Structure preservation
        let structure_score =
            self.calculate_structure_score(&source_statutes, &roundtrip_statutes, &mut breakdown);

        // Metadata completeness
        let metadata_score =
            self.calculate_metadata_score(&source_statutes, &roundtrip_statutes, &mut breakdown);

        // Round-trip fidelity
        let roundtrip_score = self.calculate_roundtrip_score(&source_statutes, &roundtrip_statutes);

        // Semantic loss
        let semantic_loss =
            self.calculate_semantic_loss(&breakdown, &import_report, &export_report);

        // Calibrated confidence
        let confidence = self.calibrate_confidence(
            &import_report,
            &export_report,
            structure_score,
            metadata_score,
            roundtrip_score,
        );

        Ok(QualityAssessment {
            semantic_loss_percent: semantic_loss,
            structure_score,
            metadata_score,
            roundtrip_score,
            confidence,
            breakdown,
        })
    }

    /// Calculates structure preservation score (0-100).
    fn calculate_structure_score(
        &self,
        original: &[Statute],
        roundtrip: &[Statute],
        breakdown: &mut QualityBreakdown,
    ) -> f64 {
        if original.is_empty() {
            return 100.0;
        }

        let mut preserved = 0;
        let mut total = 0;

        // Compare preconditions
        for (orig, rt) in original.iter().zip(roundtrip.iter()) {
            breakdown.preconditions_total += orig.preconditions.len();
            breakdown.effects_total += 1;
            total += orig.preconditions.len() + 1;

            // Count preserved preconditions
            let preserved_preconditions = orig.preconditions.len().min(rt.preconditions.len());
            breakdown.preconditions_preserved += preserved_preconditions;
            preserved += preserved_preconditions;

            // Check effect preservation
            if orig.effect.effect_type == rt.effect.effect_type {
                breakdown.effects_preserved += 1;
                preserved += 1;
            }
        }

        breakdown.structure_elements_total = total;
        breakdown.structure_elements_preserved = preserved;

        if total == 0 {
            100.0
        } else {
            (preserved as f64 / total as f64) * 100.0
        }
    }

    /// Calculates metadata completeness score (0-100).
    fn calculate_metadata_score(
        &self,
        original: &[Statute],
        roundtrip: &[Statute],
        breakdown: &mut QualityBreakdown,
    ) -> f64 {
        if original.is_empty() {
            return 100.0;
        }

        let mut total_fields = 0;
        let mut preserved_fields = 0;

        for (orig, rt) in original.iter().zip(roundtrip.iter()) {
            // Check jurisdiction
            if orig.jurisdiction.is_some() {
                total_fields += 1;
                if orig.jurisdiction == rt.jurisdiction {
                    preserved_fields += 1;
                }
            }

            // Check version
            total_fields += 1;
            if orig.version == rt.version {
                preserved_fields += 1;
            }

            // Check temporal validity
            total_fields += 1;
            if orig.temporal_validity.effective_date == rt.temporal_validity.effective_date {
                preserved_fields += 1;
            }
        }

        breakdown.metadata_fields_total = total_fields;
        breakdown.metadata_fields_preserved = preserved_fields;

        if total_fields == 0 {
            100.0
        } else {
            (preserved_fields as f64 / total_fields as f64) * 100.0
        }
    }

    /// Calculates round-trip fidelity score (0-100).
    fn calculate_roundtrip_score(&self, original: &[Statute], roundtrip: &[Statute]) -> f64 {
        if original.is_empty() && roundtrip.is_empty() {
            return 100.0;
        }

        // Check statute count preservation
        let count_match = if original.len() == roundtrip.len() {
            20.0
        } else {
            20.0 * (roundtrip.len().min(original.len()) as f64 / original.len().max(1) as f64)
        };

        // Check ID preservation
        let mut id_matches = 0;
        for (orig, rt) in original.iter().zip(roundtrip.iter()) {
            if orig.id == rt.id || orig.id.to_lowercase() == rt.id.to_lowercase() {
                id_matches += 1;
            }
        }
        let id_score = if !original.is_empty() {
            20.0 * (id_matches as f64 / original.len() as f64)
        } else {
            20.0
        };

        // Check title preservation
        let mut title_matches = 0;
        for (orig, rt) in original.iter().zip(roundtrip.iter()) {
            if orig.title == rt.title {
                title_matches += 1;
            }
        }
        let title_score = if !original.is_empty() {
            20.0 * (title_matches as f64 / original.len() as f64)
        } else {
            20.0
        };

        // Check effect type preservation
        let mut effect_type_matches = 0;
        for (orig, rt) in original.iter().zip(roundtrip.iter()) {
            if orig.effect.effect_type == rt.effect.effect_type {
                effect_type_matches += 1;
            }
        }
        let effect_score = if !original.is_empty() {
            20.0 * (effect_type_matches as f64 / original.len() as f64)
        } else {
            20.0
        };

        // Check precondition count preservation
        let mut precondition_matches = 0;
        for (orig, rt) in original.iter().zip(roundtrip.iter()) {
            if orig.preconditions.len() == rt.preconditions.len() {
                precondition_matches += 1;
            }
        }
        let precondition_score = if !original.is_empty() {
            20.0 * (precondition_matches as f64 / original.len() as f64)
        } else {
            20.0
        };

        count_match + id_score + title_score + effect_score + precondition_score
    }

    /// Calculates semantic loss percentage (0-100).
    fn calculate_semantic_loss(
        &self,
        breakdown: &QualityBreakdown,
        import_report: &ConversionReport,
        export_report: &ConversionReport,
    ) -> f64 {
        let mut loss = 0.0;

        // Loss from unsupported features
        let total_unsupported = breakdown.unsupported_features.len();
        loss += (total_unsupported as f64) * 10.0;

        // Loss from warnings
        let total_warnings = breakdown.warnings.len();
        loss += (total_warnings as f64) * 5.0;

        // Loss from confidence degradation
        let avg_confidence = (import_report.confidence + export_report.confidence) / 2.0;
        loss += (1.0 - avg_confidence) * 30.0;

        // Loss from structure preservation
        if breakdown.structure_elements_total > 0 {
            let structure_loss_rate = 1.0
                - (breakdown.structure_elements_preserved as f64
                    / breakdown.structure_elements_total as f64);
            loss += structure_loss_rate * 25.0;
        }

        // Loss from metadata preservation
        if breakdown.metadata_fields_total > 0 {
            let metadata_loss_rate = 1.0
                - (breakdown.metadata_fields_preserved as f64
                    / breakdown.metadata_fields_total as f64);
            loss += metadata_loss_rate * 10.0;
        }

        loss.min(100.0)
    }

    /// Calibrates conversion confidence (0-100).
    fn calibrate_confidence(
        &self,
        import_report: &ConversionReport,
        export_report: &ConversionReport,
        structure_score: f64,
        metadata_score: f64,
        roundtrip_score: f64,
    ) -> f64 {
        // Base confidence from reports
        let report_confidence = (import_report.confidence + export_report.confidence) / 2.0 * 100.0;

        // Weight different factors
        let weighted_score = report_confidence * 0.3
            + structure_score * 0.25
            + metadata_score * 0.15
            + roundtrip_score * 0.30;

        weighted_score.clamp(0.0, 100.0)
    }

    /// Generates a quality report for multiple format pairs.
    pub fn generate_quality_matrix(
        &mut self,
        source: &str,
        source_format: LegalFormat,
        target_formats: &[LegalFormat],
    ) -> InteropResult<QualityMatrix> {
        let mut matrix = QualityMatrix {
            source_format,
            assessments: HashMap::new(),
        };

        for &target_format in target_formats {
            if source_format != target_format {
                match self.analyze_conversion(source, source_format, target_format) {
                    Ok(assessment) => {
                        matrix.assessments.insert(target_format, assessment);
                    }
                    Err(e) => {
                        // Create a failed assessment
                        let mut breakdown = QualityBreakdown::default();
                        breakdown.warnings.push(format!("Conversion failed: {}", e));
                        matrix.assessments.insert(
                            target_format,
                            QualityAssessment {
                                semantic_loss_percent: 100.0,
                                structure_score: 0.0,
                                metadata_score: 0.0,
                                roundtrip_score: 0.0,
                                confidence: 0.0,
                                breakdown,
                            },
                        );
                    }
                }
            }
        }

        Ok(matrix)
    }
}

impl Default for QualityAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Quality matrix showing assessments for multiple target formats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMatrix {
    /// Source format
    pub source_format: LegalFormat,
    /// Assessments for each target format
    pub assessments: HashMap<LegalFormat, QualityAssessment>,
}

impl QualityMatrix {
    /// Returns the best target format based on overall quality.
    pub fn best_target(&self) -> Option<(LegalFormat, &QualityAssessment)> {
        self.assessments
            .iter()
            .max_by(|(_, a), (_, b)| {
                a.confidence
                    .partial_cmp(&b.confidence)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(format, assessment)| (*format, assessment))
    }

    /// Returns formats with high quality (confidence >= 80).
    pub fn high_quality_targets(&self) -> Vec<(LegalFormat, &QualityAssessment)> {
        self.assessments
            .iter()
            .filter(|(_, assessment)| assessment.confidence >= 80.0)
            .map(|(format, assessment)| (*format, assessment))
            .collect()
    }

    /// Generates a markdown report of the quality matrix.
    pub fn to_markdown(&self) -> String {
        let mut md = format!("# Quality Matrix for {:?}\n\n", self.source_format);

        md.push_str(
            "| Target Format | Confidence | Semantic Loss | Structure | Metadata | Round-trip |\n",
        );
        md.push_str(
            "|---------------|------------|---------------|-----------|----------|------------|\n",
        );

        let mut formats: Vec<_> = self.assessments.keys().collect();
        formats.sort_by_key(|f| format!("{:?}", f));

        for format in formats {
            if let Some(assessment) = self.assessments.get(format) {
                md.push_str(&format!(
                    "| {:?} | {:.1}% | {:.1}% | {:.1}% | {:.1}% | {:.1}% |\n",
                    format,
                    assessment.confidence,
                    assessment.semantic_loss_percent,
                    assessment.structure_score,
                    assessment.metadata_score,
                    assessment.roundtrip_score
                ));
            }
        }

        md.push_str("\n## Best Target\n\n");
        if let Some((format, assessment)) = self.best_target() {
            md.push_str(&format!(
                "**{:?}** with {:.1}% confidence\n\n",
                format, assessment.confidence
            ));
        } else {
            md.push_str("No suitable target format found.\n\n");
        }

        md
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

    #[test]
    fn test_quality_analyzer_creation() {
        let _analyzer = QualityAnalyzer::new();
    }

    #[test]
    fn test_quality_analyzer_with_cache() {
        let _analyzer = QualityAnalyzer::with_cache(10);
    }

    #[test]
    fn test_structure_score_calculation() {
        let analyzer = QualityAnalyzer::new();
        let mut breakdown = QualityBreakdown::default();

        let original = vec![
            Statute::new("test", "Test", Effect::new(EffectType::Grant, "test")).with_precondition(
                Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 18,
                },
            ),
        ];

        let roundtrip = original.clone();

        let score = analyzer.calculate_structure_score(&original, &roundtrip, &mut breakdown);
        assert_eq!(score, 100.0);
        assert_eq!(breakdown.structure_elements_preserved, 2); // 1 precondition + 1 effect
    }

    #[test]
    fn test_roundtrip_score_perfect() {
        let analyzer = QualityAnalyzer::new();
        let statutes = vec![Statute::new(
            "test",
            "Test",
            Effect::new(EffectType::Grant, "test"),
        )];

        let score = analyzer.calculate_roundtrip_score(&statutes, &statutes);
        assert_eq!(score, 100.0);
    }

    #[test]
    fn test_semantic_loss_lossless() {
        let analyzer = QualityAnalyzer::new();
        let breakdown = QualityBreakdown::default();
        let import_report = ConversionReport {
            confidence: 1.0,
            ..Default::default()
        };
        let export_report = ConversionReport {
            confidence: 1.0,
            ..Default::default()
        };

        let loss = analyzer.calculate_semantic_loss(&breakdown, &import_report, &export_report);
        assert_eq!(loss, 0.0);
    }

    #[test]
    fn test_confidence_calibration() {
        let analyzer = QualityAnalyzer::new();
        let import_report = ConversionReport {
            confidence: 1.0,
            ..Default::default()
        };
        let export_report = ConversionReport {
            confidence: 1.0,
            ..Default::default()
        };

        let confidence =
            analyzer.calibrate_confidence(&import_report, &export_report, 100.0, 100.0, 100.0);
        assert_eq!(confidence, 100.0);
    }

    #[test]
    fn test_quality_matrix_best_target() {
        let mut matrix = QualityMatrix {
            source_format: LegalFormat::Catala,
            assessments: HashMap::new(),
        };

        matrix.assessments.insert(
            LegalFormat::L4,
            QualityAssessment {
                confidence: 90.0,
                semantic_loss_percent: 10.0,
                structure_score: 95.0,
                metadata_score: 85.0,
                roundtrip_score: 92.0,
                breakdown: QualityBreakdown::default(),
            },
        );

        matrix.assessments.insert(
            LegalFormat::Stipula,
            QualityAssessment {
                confidence: 75.0,
                semantic_loss_percent: 25.0,
                structure_score: 80.0,
                metadata_score: 70.0,
                roundtrip_score: 75.0,
                breakdown: QualityBreakdown::default(),
            },
        );

        let best = matrix.best_target();
        assert!(best.is_some());
        let (format, assessment) = best.unwrap();
        assert_eq!(format, LegalFormat::L4);
        assert_eq!(assessment.confidence, 90.0);
    }

    #[test]
    fn test_quality_matrix_high_quality_targets() {
        let mut matrix = QualityMatrix {
            source_format: LegalFormat::Catala,
            assessments: HashMap::new(),
        };

        matrix.assessments.insert(
            LegalFormat::L4,
            QualityAssessment {
                confidence: 90.0,
                semantic_loss_percent: 10.0,
                structure_score: 95.0,
                metadata_score: 85.0,
                roundtrip_score: 92.0,
                breakdown: QualityBreakdown::default(),
            },
        );

        matrix.assessments.insert(
            LegalFormat::Stipula,
            QualityAssessment {
                confidence: 75.0,
                semantic_loss_percent: 25.0,
                structure_score: 80.0,
                metadata_score: 70.0,
                roundtrip_score: 75.0,
                breakdown: QualityBreakdown::default(),
            },
        );

        let high_quality = matrix.high_quality_targets();
        assert_eq!(high_quality.len(), 1);
        assert_eq!(high_quality[0].0, LegalFormat::L4);
    }

    #[test]
    fn test_quality_matrix_markdown() {
        let mut matrix = QualityMatrix {
            source_format: LegalFormat::Catala,
            assessments: HashMap::new(),
        };

        matrix.assessments.insert(
            LegalFormat::L4,
            QualityAssessment {
                confidence: 90.0,
                semantic_loss_percent: 10.0,
                structure_score: 95.0,
                metadata_score: 85.0,
                roundtrip_score: 92.0,
                breakdown: QualityBreakdown::default(),
            },
        );

        let md = matrix.to_markdown();
        assert!(md.contains("Quality Matrix"));
        assert!(md.contains("L4"));
        assert!(md.contains("90.0%"));
    }
}
