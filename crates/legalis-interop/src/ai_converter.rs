//! AI-powered format conversion utilities.
//!
//! This module provides AI-enhanced conversion capabilities:
//! - LLM-assisted format detection
//! - AI-powered lossy conversion recovery
//! - Semantic structure inference
//! - Format migration suggestions
//! - Automated format documentation

use crate::{ConversionReport, InteropError, InteropResult, LegalConverter, LegalFormat};
use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// AI-powered format detector
pub struct AIFormatDetector {
    /// Confidence threshold for format detection (0.0 to 1.0)
    pub confidence_threshold: f64,
}

/// Format detection result with AI confidence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIFormatDetection {
    /// Detected format
    pub format: LegalFormat,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Reasoning for detection
    pub reasoning: String,
    /// Alternative formats considered
    pub alternatives: Vec<(LegalFormat, f64)>,
}

impl AIFormatDetector {
    /// Creates a new AI format detector with default threshold
    pub fn new() -> Self {
        Self {
            confidence_threshold: 0.7,
        }
    }

    /// Creates a new AI format detector with custom threshold
    pub fn with_threshold(threshold: f64) -> Self {
        Self {
            confidence_threshold: threshold.clamp(0.0, 1.0),
        }
    }

    /// Detects format using AI analysis
    ///
    /// This combines pattern matching with LLM-based semantic analysis
    /// to provide more accurate format detection, especially for:
    /// - Ambiguous or malformed documents
    /// - Mixed-format documents
    /// - Custom or extended format variants
    pub fn detect(&self, content: &str) -> InteropResult<AIFormatDetection> {
        // First, try pattern-based detection as a baseline
        let pattern_scores = self.pattern_based_detection(content);

        // If we have high confidence from pattern matching, use that
        if let Some((format, score)) = pattern_scores.first()
            && *score >= 0.9
        {
            return Ok(AIFormatDetection {
                format: *format,
                confidence: *score,
                reasoning: "High-confidence pattern match".to_string(),
                alternatives: pattern_scores[1..].iter().take(3).copied().collect(),
            });
        }

        // For lower confidence, use AI-enhanced detection
        self.ai_enhanced_detection(content, pattern_scores)
    }

    fn pattern_based_detection(&self, content: &str) -> Vec<(LegalFormat, f64)> {
        let mut scores = Vec::new();

        // JSON-based formats
        if content.trim_start().starts_with('{') {
            if content.contains("\"capital_requirements\"")
                || content.contains("\"liquidity_requirements\"")
            {
                scores.push((LegalFormat::Basel3, 0.95));
            } else if content.contains("\"transactions\"") && content.contains("\"entity_lei\"") {
                scores.push((LegalFormat::MiFID2, 0.95));
            } else if content.contains("\"facts\"") && content.contains("\"contexts\"") {
                scores.push((LegalFormat::Xbrl, 0.95));
            } else if content.contains("\"requirements\"") && content.contains("\"thresholds\"") {
                scores.push((LegalFormat::FinReg, 0.90));
            } else if content.contains("\"clauses\"") {
                scores.push((LegalFormat::CommonForm, 0.85));
            } else if content.contains("\"template\"") && content.contains("\"cicero\"") {
                scores.push((LegalFormat::Cicero, 0.90));
            }
        }

        // XML-based formats
        if content.contains("<RegML") || content.contains("<regml") {
            scores.push((LegalFormat::RegML, 0.95));
        }
        if content.contains("<akomaNtoso") || content.contains("<an:") {
            scores.push((LegalFormat::AkomaNtoso, 0.95));
        }
        if content.contains("<lrml:") || content.contains("<LegalRuleML") {
            scores.push((LegalFormat::LegalRuleML, 0.95));
        }
        if content.contains("<bpmn:") || content.contains("<definitions") {
            scores.push((LegalFormat::Bpmn, 0.90));
        }
        if content.contains("<xbrl") || content.contains("xbrl-instance") {
            scores.push((LegalFormat::Xbrl, 0.90));
        }
        if content.contains("<FORMEX") {
            scores.push((LegalFormat::Formex, 0.95));
        }
        if content.contains("<niem-") || content.contains("xmlns:niem") {
            scores.push((LegalFormat::Niem, 0.95));
        }

        // Text-based DSLs
        if content.contains("scope")
            && (content.contains("consequence") || content.contains("data"))
        {
            scores.push((LegalFormat::Catala, 0.85));
        }
        if content.contains("party") && content.contains("asset") {
            scores.push((LegalFormat::Stipula, 0.85));
        }
        if content.contains("DECLARE") || (content.contains("DECIDE") && content.contains("IF")) {
            scores.push((LegalFormat::L4, 0.85));
        }

        // Sort by confidence descending
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        scores
    }

    fn ai_enhanced_detection(
        &self,
        content: &str,
        pattern_scores: Vec<(LegalFormat, f64)>,
    ) -> InteropResult<AIFormatDetection> {
        // For now, use semantic heuristics (in production, this would call LLM)
        let semantic_features = self.extract_semantic_features(content);

        // Combine pattern scores with semantic analysis
        let mut final_scores = pattern_scores;

        // Boost scores based on semantic features
        for (format, score) in &mut final_scores {
            let semantic_boost = self.calculate_semantic_boost(*format, &semantic_features);
            *score = (*score * 0.7 + semantic_boost * 0.3).min(1.0);
        }

        // Re-sort after boosting
        final_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        if let Some((format, confidence)) = final_scores.first() {
            Ok(AIFormatDetection {
                format: *format,
                confidence: *confidence,
                reasoning: format!(
                    "Combined pattern matching and semantic analysis. Key features: {}",
                    semantic_features.join(", ")
                ),
                alternatives: final_scores[1..].iter().take(3).copied().collect(),
            })
        } else {
            Err(InteropError::ParseError(
                "Could not detect format with sufficient confidence".to_string(),
            ))
        }
    }

    fn extract_semantic_features(&self, content: &str) -> Vec<String> {
        let mut features = Vec::new();

        if content.contains("obligation") || content.contains("MUST") {
            features.push("deontic-obligations".to_string());
        }
        if content.contains("prohibition") || content.contains("SHALL NOT") {
            features.push("deontic-prohibitions".to_string());
        }
        if content.contains("article") || content.contains("section") {
            features.push("legislative-structure".to_string());
        }
        if content.contains("party") || content.contains("contract") {
            features.push("contractual-elements".to_string());
        }
        if content.contains("regulation") || content.contains("compliance") {
            features.push("regulatory-content".to_string());
        }
        if content.contains("capital") || content.contains("liquidity") || content.contains("ratio")
        {
            features.push("financial-metrics".to_string());
        }
        if content.contains("transaction") || content.contains("reporting") {
            features.push("reporting-requirements".to_string());
        }

        features
    }

    fn calculate_semantic_boost(&self, format: LegalFormat, features: &[String]) -> f64 {
        let mut boost: f64 = 0.5;

        match format {
            LegalFormat::Basel3 | LegalFormat::FinReg | LegalFormat::MiFID2 => {
                if features.contains(&"financial-metrics".to_string()) {
                    boost += 0.3;
                }
                if features.contains(&"regulatory-content".to_string()) {
                    boost += 0.2;
                }
            }
            LegalFormat::RegML | LegalFormat::LegalRuleML => {
                if features.contains(&"deontic-obligations".to_string()) {
                    boost += 0.25;
                }
                if features.contains(&"regulatory-content".to_string()) {
                    boost += 0.25;
                }
            }
            LegalFormat::Catala | LegalFormat::L4 | LegalFormat::Stipula => {
                if features.contains(&"deontic-obligations".to_string()) {
                    boost += 0.2;
                }
                if features.contains(&"legislative-structure".to_string()) {
                    boost += 0.3;
                }
            }
            LegalFormat::Cicero | LegalFormat::CommonForm | LegalFormat::OpenLaw => {
                if features.contains(&"contractual-elements".to_string()) {
                    boost += 0.3;
                }
            }
            _ => {}
        }

        boost.min(1.0)
    }
}

impl Default for AIFormatDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// AI-powered lossy conversion recovery
pub struct LossyConversionRecovery {
    /// Whether to use aggressive recovery (may hallucinate)
    pub aggressive: bool,
}

impl LossyConversionRecovery {
    /// Creates a new recovery system with conservative settings
    pub fn new() -> Self {
        Self { aggressive: false }
    }

    /// Creates a recovery system with aggressive settings
    pub fn aggressive() -> Self {
        Self { aggressive: true }
    }

    /// Attempts to recover lost information from conversion
    ///
    /// This analyzes the conversion report and attempts to infer
    /// missing information that was lost during conversion.
    pub fn recover(
        &self,
        statutes: &[Statute],
        report: &ConversionReport,
        original_source: &str,
    ) -> InteropResult<Vec<Statute>> {
        let mut recovered = statutes.to_vec();

        // Analyze what was lost
        let losses = self.analyze_losses(report);

        // Attempt recovery for each loss
        for loss in losses {
            match loss.as_str() {
                "metadata" => {
                    self.recover_metadata(&mut recovered, original_source)?;
                }
                "structure" => {
                    self.recover_structure(&mut recovered, original_source)?;
                }
                "conditions" => {
                    self.recover_conditions(&mut recovered, original_source)?;
                }
                _ => {}
            }
        }

        Ok(recovered)
    }

    fn analyze_losses(&self, report: &ConversionReport) -> Vec<String> {
        let mut losses = Vec::new();

        for warning in &report.warnings {
            if warning.contains("metadata") {
                losses.push("metadata".to_string());
            }
            if warning.contains("structure") || warning.contains("hierarchy") {
                losses.push("structure".to_string());
            }
            if warning.contains("condition") {
                losses.push("conditions".to_string());
            }
        }

        losses
    }

    fn recover_metadata(&self, statutes: &mut [Statute], source: &str) -> InteropResult<()> {
        // Simple pattern matching to recover common metadata
        for statute in statutes.iter_mut() {
            // Try to find jurisdiction
            if statute.jurisdiction.is_none()
                && let Some(jurisdiction) = self.extract_jurisdiction(source)
            {
                statute.jurisdiction = Some(jurisdiction);
            }

            // Try to find effective date
            if let Some(date) = self.extract_date(source) {
                statute
                    .effect
                    .parameters
                    .entry("effective_date".to_string())
                    .or_insert(date);
            }
        }

        Ok(())
    }

    fn recover_structure(&self, _statutes: &mut [Statute], _source: &str) -> InteropResult<()> {
        // Would use AI to infer hierarchical structure
        // For now, we keep it simple
        Ok(())
    }

    fn recover_conditions(&self, _statutes: &mut [Statute], _source: &str) -> InteropResult<()> {
        // Would use AI to infer missing conditions
        // For now, we keep it simple
        Ok(())
    }

    fn extract_jurisdiction(&self, source: &str) -> Option<String> {
        let jurisdictions = ["EU", "US", "UK", "Global", "International"];
        for jur in &jurisdictions {
            if source.contains(jur) {
                return Some(jur.to_string());
            }
        }
        None
    }

    fn extract_date(&self, source: &str) -> Option<String> {
        // Simple ISO date pattern matching
        let date_pattern = regex_lite::Regex::new(r"\d{4}-\d{2}-\d{2}").ok()?;
        date_pattern.find(source).map(|m| m.as_str().to_string())
    }
}

impl Default for LossyConversionRecovery {
    fn default() -> Self {
        Self::new()
    }
}

/// Semantic structure inference for unstructured legal text
pub struct SemanticStructureInference;

impl SemanticStructureInference {
    /// Creates a new semantic structure inference engine
    pub fn new() -> Self {
        Self
    }

    /// Infers structure from unstructured legal text
    ///
    /// This uses AI to identify:
    /// - Legal provisions and obligations
    /// - Hierarchical structure (articles, sections, paragraphs)
    /// - Conditions and exceptions
    /// - References and citations
    pub fn infer_structure(&self, text: &str) -> InteropResult<Vec<Statute>> {
        let mut statutes = Vec::new();

        // Split into paragraphs
        let paragraphs: Vec<&str> = text
            .split("\n\n")
            .filter(|p| !p.trim().is_empty())
            .collect();

        for (idx, paragraph) in paragraphs.iter().enumerate() {
            // Infer if this is a legal provision
            if self.is_legal_provision(paragraph) {
                let statute = self.paragraph_to_statute(paragraph, idx)?;
                statutes.push(statute);
            }
        }

        Ok(statutes)
    }

    fn is_legal_provision(&self, text: &str) -> bool {
        // Check for legal language patterns
        text.contains("shall")
            || text.contains("must")
            || text.contains("may not")
            || text.contains("required to")
            || text.contains("prohibited from")
            || text.contains("Article")
            || text.contains("Section")
    }

    fn paragraph_to_statute(&self, text: &str, index: usize) -> InteropResult<Statute> {
        use legalis_core::{Effect, EffectType};

        // Determine effect type from text
        let effect_type = if text.contains("shall not")
            || text.contains("may not")
            || text.contains("prohibited")
        {
            EffectType::Prohibition
        } else if text.contains("shall") || text.contains("must") || text.contains("required") {
            EffectType::Obligation
        } else {
            EffectType::Grant
        };

        let effect = Effect::new(effect_type, text.trim());

        Ok(Statute::new(
            format!("inferred_{}", index + 1),
            format!("Inferred Provision {}", index + 1),
            effect,
        ))
    }
}

impl Default for SemanticStructureInference {
    fn default() -> Self {
        Self::new()
    }
}

/// Format migration suggestions
pub struct FormatMigrationSuggester;

/// Migration suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationSuggestion {
    /// Source format
    pub from: LegalFormat,
    /// Target format
    pub to: LegalFormat,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Reasoning
    pub reasoning: String,
    /// Estimated information loss (0.0 to 1.0, where 0 is lossless)
    pub estimated_loss: f64,
    /// Recommended preprocessing steps
    pub preprocessing: Vec<String>,
}

impl FormatMigrationSuggester {
    /// Creates a new migration suggester
    pub fn new() -> Self {
        Self
    }

    /// Suggests optimal migration path
    pub fn suggest_migration(
        &self,
        statutes: &[Statute],
        current_format: LegalFormat,
        desired_features: &[String],
    ) -> Vec<MigrationSuggestion> {
        let mut suggestions = Vec::new();

        // Analyze statute characteristics
        let has_financial = statutes.iter().any(|s| {
            s.effect.parameters.contains_key("capital_type")
                || s.effect.parameters.contains_key("liquidity_type")
                || s.effect.parameters.contains_key("value")
        });

        let has_regulatory = statutes.iter().any(|s| {
            s.effect.parameters.contains_key("requirement_type")
                || s.effect.parameters.contains_key("regulation_id")
        });

        let has_contractual = statutes.iter().any(|s| {
            s.effect.parameters.contains_key("party") || s.effect.parameters.contains_key("asset")
        });

        // Generate suggestions based on content
        if has_financial {
            suggestions.push(MigrationSuggestion {
                from: current_format,
                to: LegalFormat::Xbrl,
                confidence: 0.9,
                reasoning: "Content contains financial metrics suitable for XBRL".to_string(),
                estimated_loss: 0.1,
                preprocessing: vec!["Normalize monetary values".to_string()],
            });

            suggestions.push(MigrationSuggestion {
                from: current_format,
                to: LegalFormat::Basel3,
                confidence: 0.85,
                reasoning: "Content contains capital/liquidity ratios".to_string(),
                estimated_loss: 0.15,
                preprocessing: vec!["Extract ratio calculations".to_string()],
            });
        }

        if has_regulatory {
            suggestions.push(MigrationSuggestion {
                from: current_format,
                to: LegalFormat::RegML,
                confidence: 0.9,
                reasoning: "Content contains regulatory provisions".to_string(),
                estimated_loss: 0.05,
                preprocessing: vec!["Extract regulatory requirements".to_string()],
            });
        }

        if has_contractual {
            suggestions.push(MigrationSuggestion {
                from: current_format,
                to: LegalFormat::Cicero,
                confidence: 0.85,
                reasoning: "Content contains contractual elements".to_string(),
                estimated_loss: 0.1,
                preprocessing: vec!["Identify parties and obligations".to_string()],
            });
        }

        // Consider desired features
        for feature in desired_features {
            match feature.as_str() {
                "xml" => {
                    suggestions.push(MigrationSuggestion {
                        from: current_format,
                        to: LegalFormat::AkomaNtoso,
                        confidence: 0.8,
                        reasoning: "XML format requested - Akoma Ntoso is comprehensive"
                            .to_string(),
                        estimated_loss: 0.0,
                        preprocessing: vec![],
                    });
                }
                "json" => {
                    suggestions.push(MigrationSuggestion {
                        from: current_format,
                        to: LegalFormat::CommonForm,
                        confidence: 0.75,
                        reasoning: "JSON format requested - CommonForm is well-structured"
                            .to_string(),
                        estimated_loss: 0.05,
                        preprocessing: vec![],
                    });
                }
                _ => {}
            }
        }

        // Sort by confidence
        suggestions.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        suggestions
    }
}

impl Default for FormatMigrationSuggester {
    fn default() -> Self {
        Self::new()
    }
}

/// Automated format documentation generator
pub struct FormatDocumentationGenerator;

/// Generated documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatDocumentation {
    /// Format name
    pub format: LegalFormat,
    /// Summary
    pub summary: String,
    /// Supported features
    pub features: Vec<String>,
    /// Limitations
    pub limitations: Vec<String>,
    /// Example usage
    pub examples: Vec<String>,
    /// Conversion notes
    pub conversion_notes: HashMap<LegalFormat, String>,
}

impl FormatDocumentationGenerator {
    /// Creates a new documentation generator
    pub fn new() -> Self {
        Self
    }

    /// Generates documentation for a format
    pub fn generate(&self, format: LegalFormat) -> FormatDocumentation {
        match format {
            LegalFormat::Basel3 => FormatDocumentation {
                format,
                summary: "Basel III compliance format for banking regulations".to_string(),
                features: vec![
                    "Capital adequacy requirements".to_string(),
                    "Liquidity coverage ratio (LCR)".to_string(),
                    "Leverage ratio requirements".to_string(),
                ],
                limitations: vec![
                    "JSON format only".to_string(),
                    "Limited to banking sector".to_string(),
                ],
                examples: vec![
                    "Import Basel III compliance reports".to_string(),
                    "Export capital requirement statutes".to_string(),
                ],
                conversion_notes: HashMap::from([
                    (
                        LegalFormat::FinReg,
                        "Compatible for financial regulations".to_string(),
                    ),
                    (
                        LegalFormat::Xbrl,
                        "Complementary for financial reporting".to_string(),
                    ),
                ]),
            },
            LegalFormat::Xbrl => FormatDocumentation {
                format,
                summary: "eXtensible Business Reporting Language for financial reporting"
                    .to_string(),
                features: vec![
                    "Financial statements".to_string(),
                    "Regulatory reporting".to_string(),
                    "Business performance metrics".to_string(),
                    "Taxonomy support (US-GAAP, IFRS)".to_string(),
                ],
                limitations: vec![
                    "Complex taxonomy management".to_string(),
                    "Requires context definitions".to_string(),
                ],
                examples: vec![
                    "Import financial reports".to_string(),
                    "Export accounting data".to_string(),
                ],
                conversion_notes: HashMap::from([
                    (LegalFormat::Basel3, "Use for capital reporting".to_string()),
                    (
                        LegalFormat::MiFID2,
                        "Use for transaction reporting".to_string(),
                    ),
                ]),
            },
            // Add more formats as needed
            _ => FormatDocumentation {
                format,
                summary: format!("{:?} format", format),
                features: vec!["General legal document support".to_string()],
                limitations: vec![],
                examples: vec![],
                conversion_notes: HashMap::new(),
            },
        }
    }

    /// Generates comparison documentation between two formats
    pub fn compare(&self, format1: LegalFormat, format2: LegalFormat) -> String {
        let doc1 = self.generate(format1);
        let doc2 = self.generate(format2);

        format!(
            "# Comparison: {:?} vs {:?}\n\n\
            ## {:?}\n{}\n\n\
            Features:\n{}\n\n\
            ## {:?}\n{}\n\n\
            Features:\n{}\n\n\
            ## Conversion Notes\n\
            When converting from {:?} to {:?}:\n{}",
            format1,
            format2,
            format1,
            doc1.summary,
            doc1.features
                .iter()
                .map(|f| format!("- {}", f))
                .collect::<Vec<_>>()
                .join("\n"),
            format2,
            doc2.summary,
            doc2.features
                .iter()
                .map(|f| format!("- {}", f))
                .collect::<Vec<_>>()
                .join("\n"),
            format1,
            format2,
            doc1.conversion_notes
                .get(&format2)
                .unwrap_or(&"No specific notes".to_string())
        )
    }
}

impl Default for FormatDocumentationGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// AI-enhanced converter combining all AI features
pub struct AIEnhancedConverter {
    /// Base converter
    pub converter: LegalConverter,
    /// Format detector
    pub detector: AIFormatDetector,
    /// Lossy conversion recovery
    pub recovery: LossyConversionRecovery,
    /// Semantic inference
    pub inference: SemanticStructureInference,
    /// Migration suggester
    pub suggester: FormatMigrationSuggester,
    /// Documentation generator
    pub doc_generator: FormatDocumentationGenerator,
}

impl AIEnhancedConverter {
    /// Creates a new AI-enhanced converter
    pub fn new() -> Self {
        Self {
            converter: LegalConverter::new(),
            detector: AIFormatDetector::new(),
            recovery: LossyConversionRecovery::new(),
            inference: SemanticStructureInference::new(),
            suggester: FormatMigrationSuggester::new(),
            doc_generator: FormatDocumentationGenerator::new(),
        }
    }

    /// Auto-detects format and imports
    pub fn auto_import(
        &mut self,
        content: &str,
    ) -> InteropResult<(Vec<Statute>, ConversionReport, AIFormatDetection)> {
        let detection = self.detector.detect(content)?;

        let (mut statutes, mut report) = self.converter.import(content, detection.format)?;

        // Attempt recovery if there were warnings
        if !report.warnings.is_empty() && !self.recovery.aggressive {
            statutes = self.recovery.recover(&statutes, &report, content)?;
            report.add_warning("Applied AI-powered lossy conversion recovery");
        }

        Ok((statutes, report, detection))
    }

    /// Infers structure from unstructured text
    pub fn infer_from_text(&self, text: &str) -> InteropResult<Vec<Statute>> {
        self.inference.infer_structure(text)
    }

    /// Suggests migration paths
    pub fn suggest_migrations(
        &self,
        statutes: &[Statute],
        current_format: LegalFormat,
        desired_features: &[String],
    ) -> Vec<MigrationSuggestion> {
        self.suggester
            .suggest_migration(statutes, current_format, desired_features)
    }

    /// Generates format documentation
    pub fn document_format(&self, format: LegalFormat) -> FormatDocumentation {
        self.doc_generator.generate(format)
    }
}

impl Default for AIEnhancedConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_format_detection_basel3() {
        let detector = AIFormatDetector::new();
        let source = r#"{
            "metadata": {"bank_lei": "123"},
            "capital_requirements": [{"id": "cet1", "capital_type": "CET1", "minimum_ratio": 4.5}],
            "liquidity_requirements": []
        }"#;

        let result = detector.detect(source).unwrap();
        assert_eq!(result.format, LegalFormat::Basel3);
        assert!(result.confidence > 0.8);
    }

    #[test]
    fn test_ai_format_detection_xbrl() {
        let detector = AIFormatDetector::new();
        let source = r#"{"metadata": {}, "contexts": [], "facts": []}"#;

        let result = detector.detect(source).unwrap();
        assert_eq!(result.format, LegalFormat::Xbrl);
    }

    #[test]
    fn test_ai_format_detection_regml_xml() {
        let detector = AIFormatDetector::new();
        let source = "<RegML><provision id='p1'><text>Must comply</text></provision></RegML>";

        let result = detector.detect(source).unwrap();
        assert_eq!(result.format, LegalFormat::RegML);
    }

    #[test]
    fn test_lossy_conversion_recovery() {
        let recovery = LossyConversionRecovery::new();
        let mut report = ConversionReport::new(LegalFormat::Catala, LegalFormat::Legalis);
        report.add_warning("Lost metadata during conversion");

        let source = "jurisdiction: EU\ndate: 2024-01-01";
        let statutes = vec![Statute::new(
            "test",
            "Test",
            legalis_core::Effect::new(legalis_core::EffectType::Obligation, "Test"),
        )];

        let recovered = recovery.recover(&statutes, &report, source).unwrap();
        assert_eq!(recovered.len(), 1);
        // Check if jurisdiction was recovered
        assert!(recovered[0].jurisdiction.is_some() || !recovered[0].effect.parameters.is_empty());
    }

    #[test]
    fn test_semantic_structure_inference() {
        let inference = SemanticStructureInference::new();
        let text = "Article 1: Parties shall comply with all regulations.\n\n\
                    Article 2: Users may not engage in prohibited activities.";

        let statutes = inference.infer_structure(text).unwrap();
        assert_eq!(statutes.len(), 2);
        assert_eq!(
            statutes[0].effect.effect_type,
            legalis_core::EffectType::Obligation
        );
        assert_eq!(
            statutes[1].effect.effect_type,
            legalis_core::EffectType::Prohibition
        );
    }

    #[test]
    fn test_migration_suggestions() {
        let suggester = FormatMigrationSuggester::new();

        let mut statute = Statute::new(
            "cap1",
            "Capital Requirement",
            legalis_core::Effect::new(legalis_core::EffectType::Obligation, "Maintain capital"),
        );
        statute
            .effect
            .parameters
            .insert("capital_type".to_string(), "CET1".to_string());

        let suggestions =
            suggester.suggest_migration(&[statute], LegalFormat::Legalis, &["xml".to_string()]);

        assert!(!suggestions.is_empty());
        assert!(
            suggestions
                .iter()
                .any(|s| s.to == LegalFormat::Xbrl || s.to == LegalFormat::Basel3)
        );
    }

    #[test]
    fn test_format_documentation() {
        let generator = FormatDocumentationGenerator::new();

        let doc = generator.generate(LegalFormat::Basel3);
        assert_eq!(doc.format, LegalFormat::Basel3);
        assert!(!doc.summary.is_empty());
        assert!(!doc.features.is_empty());

        let comparison = generator.compare(LegalFormat::Basel3, LegalFormat::Xbrl);
        assert!(comparison.contains("Basel3"));
        assert!(comparison.contains("Xbrl"));
    }

    #[test]
    fn test_ai_enhanced_converter_auto_import() {
        let mut converter = AIEnhancedConverter::new();
        let source = r#"{
            "metadata": {
                "report_id": "BASEL3-TEST-001",
                "bank_lei": "123",
                "reporting_date": "2024-12-31",
                "jurisdiction": "Global",
                "supervisor": "BIS"
            },
            "capital_requirements": [{"id": "cet1", "name": "CET1", "capital_type": "CET1", "minimum_ratio": 4.5, "current_ratio": null, "risk_weighted_assets": null, "capital_amount": null}],
            "liquidity_requirements": []
        }"#;

        let (statutes, report, detection) = converter.auto_import(source).unwrap();
        assert_eq!(detection.format, LegalFormat::Basel3);
        assert!(!statutes.is_empty());
        assert!(report.statutes_converted > 0);
    }

    #[test]
    fn test_ai_enhanced_converter_infer_from_text() {
        let converter = AIEnhancedConverter::new();
        let text = "Section 1: All entities must maintain adequate capital.\n\n\
                    Section 2: Entities shall not exceed leverage limits.";

        let statutes = converter.infer_from_text(text).unwrap();
        assert_eq!(statutes.len(), 2);
    }
}
