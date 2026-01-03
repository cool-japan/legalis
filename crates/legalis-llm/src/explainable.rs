//! Explainable Legal AI (v0.3.3)
//!
//! This module provides natural language explanation generation, counterfactual explanations,
//! feature attribution for decisions, interactive explanation exploration, and layperson-friendly summaries.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Natural language explanation generator.
///
/// Converts AI decisions and reasoning into human-readable explanations.
pub struct ExplanationGenerator {
    templates: Arc<RwLock<HashMap<String, ExplanationTemplate>>>,
    style_config: ExplanationStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationTemplate {
    pub id: String,
    pub template: String,
    pub variables: Vec<String>,
    pub audience: AudienceLevel,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum AudienceLevel {
    Expert,
    Professional,
    General,
    Layperson,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationStyle {
    pub verbosity: Verbosity,
    pub include_citations: bool,
    pub include_confidence: bool,
    pub max_length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Verbosity {
    Concise,
    Normal,
    Detailed,
    Comprehensive,
}

impl Default for ExplanationStyle {
    fn default() -> Self {
        Self {
            verbosity: Verbosity::Normal,
            include_citations: true,
            include_confidence: true,
            max_length: 500,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Explanation {
    pub id: String,
    pub decision_id: String,
    pub text: String,
    pub audience: AudienceLevel,
    pub confidence: f64,
    pub supporting_facts: Vec<String>,
    pub citations: Vec<Citation>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Citation {
    pub source: String,
    pub reference: String,
    pub relevance: f64,
}

impl ExplanationGenerator {
    /// Creates a new explanation generator.
    pub fn new(style: ExplanationStyle) -> Self {
        Self {
            templates: Arc::new(RwLock::new(HashMap::new())),
            style_config: style,
        }
    }

    /// Adds an explanation template.
    pub async fn add_template(&self, template: ExplanationTemplate) -> Result<()> {
        let mut templates = self.templates.write().await;
        templates.insert(template.id.clone(), template);
        Ok(())
    }

    /// Generates an explanation for a decision.
    pub async fn generate_explanation(
        &self,
        decision_id: &str,
        context: &HashMap<String, String>,
        audience: AudienceLevel,
    ) -> Result<Explanation> {
        let templates = self.templates.read().await;

        // Find suitable template
        let template = templates
            .values()
            .find(|t| t.audience == audience)
            .ok_or_else(|| anyhow::anyhow!("No template found for audience level"))?;

        // Generate explanation text
        let mut text = template.template.clone();
        for var in &template.variables {
            if let Some(value) = context.get(var) {
                text = text.replace(&format!("{{{}}}", var), value);
            }
        }

        // Apply style constraints
        if text.len() > self.style_config.max_length {
            text = format!("{}...", &text[..self.style_config.max_length]);
        }

        let explanation = Explanation {
            id: uuid::Uuid::new_v4().to_string(),
            decision_id: decision_id.to_string(),
            text,
            audience,
            confidence: 0.9,
            supporting_facts: vec![],
            citations: vec![],
            timestamp: chrono::Utc::now(),
        };

        Ok(explanation)
    }

    /// Simplifies an explanation for a different audience.
    pub async fn simplify_explanation(
        &self,
        explanation: &Explanation,
        target_audience: AudienceLevel,
    ) -> Result<Explanation> {
        let mut simplified = explanation.clone();
        simplified.id = uuid::Uuid::new_v4().to_string();
        simplified.audience = target_audience;

        // Simplification logic based on target audience
        match target_audience {
            AudienceLevel::Layperson => {
                simplified.text = self.simplify_legal_jargon(&explanation.text);
            }
            AudienceLevel::General => {
                simplified.text = self.reduce_complexity(&explanation.text);
            }
            _ => {}
        }

        Ok(simplified)
    }

    fn simplify_legal_jargon(&self, text: &str) -> String {
        text.replace("hereinafter", "from now on")
            .replace("pursuant to", "according to")
            .replace("aforementioned", "mentioned earlier")
            .replace("notwithstanding", "despite")
            .replace("herein", "in this document")
    }

    fn reduce_complexity(&self, text: &str) -> String {
        // Split into sentences and keep only the most important ones
        let sentences: Vec<&str> = text.split('.').collect();
        if sentences.len() > 3 {
            sentences[..3].join(". ") + "."
        } else {
            text.to_string()
        }
    }
}

impl Default for ExplanationGenerator {
    fn default() -> Self {
        Self::new(ExplanationStyle::default())
    }
}

/// Counterfactual explanation generator.
///
/// Generates "what-if" scenarios to explain why a decision was made.
pub struct CounterfactualGenerator {
    scenarios: Arc<RwLock<Vec<CounterfactualScenario>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterfactualScenario {
    pub id: String,
    pub original_decision: String,
    pub modified_inputs: HashMap<String, String>,
    pub alternative_decision: String,
    pub explanation: String,
    pub probability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterfactualExplanation {
    pub decision_id: String,
    pub original_outcome: String,
    pub scenarios: Vec<CounterfactualScenario>,
    pub key_factors: Vec<String>,
}

impl CounterfactualGenerator {
    /// Creates a new counterfactual generator.
    pub fn new() -> Self {
        Self {
            scenarios: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Generates counterfactual scenarios for a decision.
    pub async fn generate_counterfactuals(
        &self,
        decision: &str,
        inputs: &HashMap<String, String>,
    ) -> Result<CounterfactualExplanation> {
        let mut scenarios = Vec::new();

        // Generate scenarios by modifying each input
        for (key, value) in inputs {
            let mut modified_inputs = inputs.clone();
            modified_inputs.insert(key.clone(), format!("alternative_{}", value));

            let scenario = CounterfactualScenario {
                id: uuid::Uuid::new_v4().to_string(),
                original_decision: decision.to_string(),
                modified_inputs,
                alternative_decision: format!("Alternative decision when {} changes", key),
                explanation: format!(
                    "If {} were different, the outcome would change because...",
                    key
                ),
                probability: 0.7,
            };

            scenarios.push(scenario);
        }

        // Store scenarios
        {
            let mut stored = self.scenarios.write().await;
            stored.extend(scenarios.clone());
        }

        let explanation = CounterfactualExplanation {
            decision_id: uuid::Uuid::new_v4().to_string(),
            original_outcome: decision.to_string(),
            scenarios,
            key_factors: inputs.keys().cloned().collect(),
        };

        Ok(explanation)
    }

    /// Finds the minimal change needed to alter a decision.
    pub async fn find_minimal_change(&self, decision: &str) -> Option<CounterfactualScenario> {
        let scenarios = self.scenarios.read().await;

        scenarios
            .iter()
            .filter(|s| s.original_decision == decision)
            .min_by(|a, b| a.modified_inputs.len().cmp(&b.modified_inputs.len()))
            .cloned()
    }

    /// Gets all scenarios for a decision.
    pub async fn get_scenarios(&self, decision_id: &str) -> Vec<CounterfactualScenario> {
        let scenarios = self.scenarios.read().await;
        scenarios
            .iter()
            .filter(|s| s.original_decision == decision_id)
            .cloned()
            .collect()
    }
}

impl Default for CounterfactualGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Feature attribution analyzer.
///
/// Identifies which features contributed most to a decision.
pub struct FeatureAttributor {
    attributions: Arc<RwLock<Vec<FeatureAttribution>>>,
    attribution_method: AttributionMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AttributionMethod {
    SHAP,
    LIME,
    IntegratedGradients,
    AttentionWeights,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureAttribution {
    pub decision_id: String,
    pub feature_name: String,
    pub importance: f64,
    pub direction: InfluenceDirection,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InfluenceDirection {
    Positive,
    Negative,
    Neutral,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributionReport {
    pub decision_id: String,
    pub top_features: Vec<FeatureAttribution>,
    pub method: AttributionMethod,
    pub total_features: usize,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl FeatureAttributor {
    /// Creates a new feature attributor.
    pub fn new(method: AttributionMethod) -> Self {
        Self {
            attributions: Arc::new(RwLock::new(Vec::new())),
            attribution_method: method,
        }
    }

    /// Analyzes feature importance for a decision.
    pub async fn analyze_features(
        &self,
        decision_id: &str,
        features: &HashMap<String, f64>,
    ) -> Result<AttributionReport> {
        let mut attributions = Vec::new();

        // Calculate importance for each feature
        for (name, value) in features {
            let importance = value.abs();
            let direction = if *value > 0.0 {
                InfluenceDirection::Positive
            } else if *value < 0.0 {
                InfluenceDirection::Negative
            } else {
                InfluenceDirection::Neutral
            };

            let attribution = FeatureAttribution {
                decision_id: decision_id.to_string(),
                feature_name: name.clone(),
                importance,
                direction,
                confidence: 0.85,
            };

            attributions.push(attribution);
        }

        // Sort by importance
        attributions.sort_by(|a, b| {
            b.importance
                .partial_cmp(&a.importance)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Store attributions
        {
            let mut stored = self.attributions.write().await;
            stored.extend(attributions.clone());
        }

        let report = AttributionReport {
            decision_id: decision_id.to_string(),
            top_features: attributions.clone(),
            method: self.attribution_method.clone(),
            total_features: features.len(),
            timestamp: chrono::Utc::now(),
        };

        Ok(report)
    }

    /// Gets the top N most important features for a decision.
    pub async fn get_top_features(&self, decision_id: &str, n: usize) -> Vec<FeatureAttribution> {
        let attributions = self.attributions.read().await;

        let mut relevant: Vec<_> = attributions
            .iter()
            .filter(|a| a.decision_id == decision_id)
            .cloned()
            .collect();

        relevant.sort_by(|a, b| {
            b.importance
                .partial_cmp(&a.importance)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        relevant.into_iter().take(n).collect()
    }

    /// Visualizes feature importance as a simple text representation.
    pub async fn visualize_importance(&self, decision_id: &str) -> String {
        let top_features = self.get_top_features(decision_id, 5).await;

        let mut visualization = format!("Feature Importance for {}\n", decision_id);
        visualization.push_str("=".repeat(50).as_str());
        visualization.push('\n');

        for feature in top_features {
            let bar_length = (feature.importance * 20.0) as usize;
            let bar = "█".repeat(bar_length);
            let direction_symbol = match feature.direction {
                InfluenceDirection::Positive => "+",
                InfluenceDirection::Negative => "-",
                InfluenceDirection::Neutral => "○",
            };

            visualization.push_str(&format!(
                "{} {} {:.2} {}\n",
                direction_symbol, feature.feature_name, feature.importance, bar
            ));
        }

        visualization
    }
}

/// Interactive explanation explorer.
///
/// Provides an interface for users to explore explanations interactively.
pub struct ExplanationExplorer {
    explanations: Arc<RwLock<HashMap<String, Explanation>>>,
    exploration_history: Arc<RwLock<Vec<ExplorationStep>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplorationStep {
    pub step_id: String,
    pub explanation_id: String,
    pub action: ExplorationAction,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExplorationAction {
    ViewExplanation,
    DrillDown { aspect: String },
    ViewCitation { citation_id: String },
    CompareAlternatives,
    AskQuestion { question: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveExplanation {
    pub base_explanation: Explanation,
    pub available_actions: Vec<String>,
    pub related_explanations: Vec<String>,
    pub depth_level: usize,
}

impl ExplanationExplorer {
    /// Creates a new explanation explorer.
    pub fn new() -> Self {
        Self {
            explanations: Arc::new(RwLock::new(HashMap::new())),
            exploration_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Adds an explanation to the explorer.
    pub async fn add_explanation(&self, explanation: Explanation) -> Result<()> {
        let mut explanations = self.explanations.write().await;
        explanations.insert(explanation.id.clone(), explanation);
        Ok(())
    }

    /// Starts an interactive exploration session.
    pub async fn start_exploration(&self, explanation_id: &str) -> Result<InteractiveExplanation> {
        let explanations = self.explanations.read().await;

        let base = explanations
            .get(explanation_id)
            .ok_or_else(|| anyhow::anyhow!("Explanation not found"))?
            .clone();

        let interactive = InteractiveExplanation {
            base_explanation: base,
            available_actions: vec![
                "View details".to_string(),
                "See supporting facts".to_string(),
                "View citations".to_string(),
                "Compare alternatives".to_string(),
            ],
            related_explanations: vec![],
            depth_level: 0,
        };

        // Record exploration step
        self.record_step(explanation_id, ExplorationAction::ViewExplanation)
            .await?;

        Ok(interactive)
    }

    /// Records an exploration step.
    pub async fn record_step(&self, explanation_id: &str, action: ExplorationAction) -> Result<()> {
        let step = ExplorationStep {
            step_id: uuid::Uuid::new_v4().to_string(),
            explanation_id: explanation_id.to_string(),
            action,
            timestamp: chrono::Utc::now(),
        };

        let mut history = self.exploration_history.write().await;
        history.push(step);

        Ok(())
    }

    /// Drills down into a specific aspect of an explanation.
    pub async fn drill_down(&self, explanation_id: &str, aspect: &str) -> Result<String> {
        let explanation = {
            let explanations = self.explanations.read().await;
            explanations
                .get(explanation_id)
                .ok_or_else(|| anyhow::anyhow!("Explanation not found"))?
                .clone()
        };

        // Record drill-down action
        self.record_step(
            explanation_id,
            ExplorationAction::DrillDown {
                aspect: aspect.to_string(),
            },
        )
        .await?;

        // Generate detailed view of the aspect
        let detail = format!(
            "Detailed view of '{}' in explanation:\n{}",
            aspect, explanation.text
        );

        Ok(detail)
    }

    /// Gets the exploration history for an explanation.
    pub async fn get_exploration_history(&self, explanation_id: &str) -> Vec<ExplorationStep> {
        let history = self.exploration_history.read().await;
        history
            .iter()
            .filter(|step| step.explanation_id == explanation_id)
            .cloned()
            .collect()
    }
}

impl Default for ExplanationExplorer {
    fn default() -> Self {
        Self::new()
    }
}

/// Layperson-friendly summary generator.
///
/// Converts complex legal explanations into simple, accessible language.
pub struct LaypersonSummarizer {
    simplification_rules: Arc<RwLock<Vec<SimplificationRule>>>,
    summaries: Arc<RwLock<HashMap<String, SimplifiedSummary>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimplificationRule {
    pub id: String,
    pub complex_term: String,
    pub simple_term: String,
    pub category: SimplificationCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SimplificationCategory {
    LegalJargon,
    TechnicalTerm,
    LatinPhrase,
    ComplexConcept,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimplifiedSummary {
    pub id: String,
    pub original_text: String,
    pub simplified_text: String,
    pub reading_level: ReadingLevel,
    pub key_points: Vec<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReadingLevel {
    Elementary,
    MiddleSchool,
    HighSchool,
    College,
}

impl LaypersonSummarizer {
    /// Creates a new layperson summarizer.
    pub fn new() -> Self {
        let summarizer = Self {
            simplification_rules: Arc::new(RwLock::new(Vec::new())),
            summaries: Arc::new(RwLock::new(HashMap::new())),
        };

        // Initialize with common legal terms
        summarizer
    }

    /// Adds a simplification rule.
    pub async fn add_rule(&self, rule: SimplificationRule) -> Result<()> {
        let mut rules = self.simplification_rules.write().await;
        rules.push(rule);
        Ok(())
    }

    /// Initializes common legal term simplifications.
    pub async fn initialize_common_terms(&self) -> Result<()> {
        let common_rules = vec![
            SimplificationRule {
                id: "1".to_string(),
                complex_term: "plaintiff".to_string(),
                simple_term: "person who filed the lawsuit".to_string(),
                category: SimplificationCategory::LegalJargon,
            },
            SimplificationRule {
                id: "2".to_string(),
                complex_term: "defendant".to_string(),
                simple_term: "person being sued".to_string(),
                category: SimplificationCategory::LegalJargon,
            },
            SimplificationRule {
                id: "3".to_string(),
                complex_term: "pro bono".to_string(),
                simple_term: "free legal work".to_string(),
                category: SimplificationCategory::LatinPhrase,
            },
        ];

        for rule in common_rules {
            self.add_rule(rule).await?;
        }

        Ok(())
    }

    /// Simplifies complex legal text for layperson understanding.
    pub async fn simplify_text(
        &self,
        text: &str,
        target_level: ReadingLevel,
    ) -> Result<SimplifiedSummary> {
        let rules = self.simplification_rules.read().await;

        let mut simplified = text.to_string();

        // Apply simplification rules
        for rule in rules.iter() {
            simplified = simplified.replace(&rule.complex_term, &rule.simple_term);
        }

        // Additional simplifications based on reading level
        simplified = match target_level {
            ReadingLevel::Elementary => self.simplify_to_elementary(&simplified),
            ReadingLevel::MiddleSchool => self.simplify_to_middle_school(&simplified),
            _ => simplified,
        };

        // Extract key points
        let key_points = self.extract_key_points(&simplified);

        let summary = SimplifiedSummary {
            id: uuid::Uuid::new_v4().to_string(),
            original_text: text.to_string(),
            simplified_text: simplified,
            reading_level: target_level,
            key_points,
            timestamp: chrono::Utc::now(),
        };

        // Store summary
        {
            let mut summaries = self.summaries.write().await;
            summaries.insert(summary.id.clone(), summary.clone());
        }

        Ok(summary)
    }

    fn simplify_to_elementary(&self, text: &str) -> String {
        text.split('.')
            .take(2) // Only keep first 2 sentences
            .collect::<Vec<_>>()
            .join(". ")
            + "."
    }

    fn simplify_to_middle_school(&self, text: &str) -> String {
        text.split('.')
            .take(4) // Keep first 4 sentences
            .collect::<Vec<_>>()
            .join(". ")
            + "."
    }

    fn extract_key_points(&self, text: &str) -> Vec<String> {
        // Simple extraction: first sentence of each paragraph
        text.split('\n')
            .filter(|s| !s.is_empty())
            .map(|p| p.split('.').next().unwrap_or("").trim().to_string())
            .filter(|s| !s.is_empty())
            .take(3)
            .collect()
    }

    /// Compares the complexity of original vs. simplified text.
    pub async fn measure_simplification(&self, summary_id: &str) -> Option<SimplificationMetrics> {
        let summaries = self.summaries.read().await;

        summaries.get(summary_id).map(|summary| {
            let original_words = summary.original_text.split_whitespace().count();
            let simplified_words = summary.simplified_text.split_whitespace().count();

            let reduction = if original_words > 0 {
                (original_words - simplified_words) as f64 / original_words as f64
            } else {
                0.0
            };

            SimplificationMetrics {
                original_word_count: original_words,
                simplified_word_count: simplified_words,
                reduction_percentage: reduction * 100.0,
                reading_level: summary.reading_level.clone(),
            }
        })
    }
}

impl Default for LaypersonSummarizer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimplificationMetrics {
    pub original_word_count: usize,
    pub simplified_word_count: usize,
    pub reduction_percentage: f64,
    pub reading_level: ReadingLevel,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_explanation_generator() {
        let generator = ExplanationGenerator::new(ExplanationStyle::default());

        let template = ExplanationTemplate {
            id: "t1".to_string(),
            template: "The decision was made because {reason}".to_string(),
            variables: vec!["reason".to_string()],
            audience: AudienceLevel::General,
        };

        generator.add_template(template).await.unwrap();

        let mut context = HashMap::new();
        context.insert(
            "reason".to_string(),
            "the contract terms were met".to_string(),
        );

        let explanation = generator
            .generate_explanation("d1", &context, AudienceLevel::General)
            .await
            .unwrap();

        assert!(explanation.text.contains("the contract terms were met"));
    }

    #[tokio::test]
    async fn test_counterfactual_generator() {
        let generator = CounterfactualGenerator::new();

        let mut inputs = HashMap::new();
        inputs.insert("contract_value".to_string(), "100000".to_string());

        let explanation = generator
            .generate_counterfactuals("approved", &inputs)
            .await
            .unwrap();

        assert!(!explanation.scenarios.is_empty());
        assert_eq!(explanation.key_factors.len(), 1);
    }

    #[tokio::test]
    async fn test_feature_attributor() {
        let attributor = FeatureAttributor::new(AttributionMethod::SHAP);

        let mut features = HashMap::new();
        features.insert("feature1".to_string(), 0.8);
        features.insert("feature2".to_string(), -0.3);

        let report = attributor.analyze_features("d1", &features).await.unwrap();

        assert_eq!(report.top_features.len(), 2);
        assert_eq!(report.top_features[0].feature_name, "feature1");
    }

    #[tokio::test]
    async fn test_explanation_explorer() {
        let explorer = ExplanationExplorer::new();

        let explanation = Explanation {
            id: "e1".to_string(),
            decision_id: "d1".to_string(),
            text: "Test explanation".to_string(),
            audience: AudienceLevel::General,
            confidence: 0.9,
            supporting_facts: vec![],
            citations: vec![],
            timestamp: chrono::Utc::now(),
        };

        explorer.add_explanation(explanation).await.unwrap();

        let interactive = explorer.start_exploration("e1").await.unwrap();
        assert_eq!(interactive.depth_level, 0);
        assert!(!interactive.available_actions.is_empty());
    }

    #[tokio::test]
    async fn test_layperson_summarizer() {
        let summarizer = LaypersonSummarizer::new();
        summarizer.initialize_common_terms().await.unwrap();

        let complex_text = "The plaintiff filed a lawsuit against the defendant.";
        let summary = summarizer
            .simplify_text(complex_text, ReadingLevel::Elementary)
            .await
            .unwrap();

        assert!(
            summary
                .simplified_text
                .contains("person who filed the lawsuit")
        );
        assert!(summary.simplified_text.contains("person being sued"));
    }

    #[test]
    fn test_explanation_style_default() {
        let style = ExplanationStyle::default();
        assert_eq!(style.verbosity, Verbosity::Normal);
        assert!(style.include_citations);
        assert_eq!(style.max_length, 500);
    }

    #[test]
    fn test_influence_direction() {
        assert_eq!(InfluenceDirection::Positive, InfluenceDirection::Positive);
        assert_ne!(InfluenceDirection::Positive, InfluenceDirection::Negative);
    }
}
