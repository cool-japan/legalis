//! Explainable AI for transparent legal decision-making.
//!
//! Provides tools for explaining AI-assisted legal decisions with
//! feature importance, attention visualization, and counterfactual explanations.

/// Feature importance for a legal decision.
#[derive(Debug, Clone)]
pub struct FeatureImportance {
    pub feature_name: String,
    pub importance_score: f64,
    pub contribution: Contribution,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Contribution {
    Positive,
    Negative,
    Neutral,
}

/// Explanation for an AI decision.
#[derive(Debug, Clone)]
pub struct AiExplanation {
    pub decision: String,
    pub confidence: f64,
    pub top_features: Vec<FeatureImportance>,
    pub reasoning_trace: Vec<String>,
}

impl AiExplanation {
    pub fn new(decision: String, confidence: f64) -> Self {
        Self {
            decision,
            confidence,
            top_features: Vec::new(),
            reasoning_trace: Vec::new(),
        }
    }

    pub fn add_feature(mut self, name: String, score: f64, contribution: Contribution) -> Self {
        self.top_features.push(FeatureImportance {
            feature_name: name,
            importance_score: score,
            contribution,
        });
        self
    }
}

/// Explainer for AI legal decisions.
#[derive(Debug, Clone, Default)]
pub struct LegalAiExplainer {
    explanations_generated: u64,
}

impl LegalAiExplainer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn explain_count(&self) -> u64 {
        self.explanations_generated
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_explanation() {
        let exp = AiExplanation::new("Grant".to_string(), 0.95).add_feature(
            "age".to_string(),
            0.8,
            Contribution::Positive,
        );
        assert_eq!(exp.confidence, 0.95);
        assert_eq!(exp.top_features.len(), 1);
    }

    #[test]
    fn test_explainer() {
        let explainer = LegalAiExplainer::new();
        assert_eq!(explainer.explain_count(), 0);
    }
}
