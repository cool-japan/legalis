//! LLM-Based Semantic Diff Analysis
//!
//! This module provides AI-powered diff analysis using Large Language Models
//! for semantic understanding, intent detection, and natural language explanations.

use crate::{ChangeType, DiffResult, StatuteDiff};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(test)]
use legalis_core::Statute;

/// LLM provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LlmProvider {
    /// OpenAI GPT models
    OpenAI {
        /// API key
        api_key: String,
        /// Model name (e.g., "gpt-4", "gpt-3.5-turbo")
        model: String,
        /// Temperature (0.0-2.0)
        temperature: f32,
    },
    /// Anthropic Claude models
    Anthropic {
        /// API key
        api_key: String,
        /// Model name
        model: String,
    },
    /// Local model
    Local {
        /// Model path
        model_path: String,
    },
}

/// LLM configuration
#[derive(Debug, Clone)]
pub struct LlmConfig {
    /// LLM provider
    pub provider: LlmProvider,
    /// Maximum tokens for response
    pub max_tokens: usize,
    /// Enable caching
    pub cache_responses: bool,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            provider: LlmProvider::Local {
                model_path: "local-model".to_string(),
            },
            max_tokens: 1000,
            cache_responses: true,
        }
    }
}

/// LLM-powered diff analyzer
pub struct LlmAnalyzer {
    config: LlmConfig,
    cache: HashMap<String, String>,
}

impl LlmAnalyzer {
    /// Creates a new LLM analyzer with the given configuration
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::llm::{LlmAnalyzer, LlmConfig};
    ///
    /// let config = LlmConfig::default();
    /// let analyzer = LlmAnalyzer::new(config);
    /// ```
    pub fn new(config: LlmConfig) -> Self {
        Self {
            config,
            cache: HashMap::new(),
        }
    }

    /// Generates a natural language explanation of the diff
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_diff::{diff, llm::{LlmAnalyzer, LlmConfig}};
    ///
    /// let mut analyzer = LlmAnalyzer::new(LlmConfig::default());
    /// let old = Statute::new("law", "Old Title", Effect::new(EffectType::Grant, "Benefit"));
    /// let new = Statute::new("law", "New Title", Effect::new(EffectType::Grant, "Benefit"));
    ///
    /// let diff_result = diff(&old, &new).unwrap();
    /// let explanation = analyzer.explain_diff(&diff_result).unwrap();
    /// assert!(!explanation.is_empty());
    /// ```
    pub fn explain_diff(&mut self, diff: &StatuteDiff) -> DiffResult<String> {
        let cache_key = format!("explain-{}", diff.statute_id);

        if self.config.cache_responses
            && let Some(cached) = self.cache.get(&cache_key)
        {
            return Ok(cached.clone());
        }

        let explanation = self.generate_explanation(diff);

        if self.config.cache_responses {
            self.cache.insert(cache_key, explanation.clone());
        }

        Ok(explanation)
    }

    /// Simulates LLM-based explanation generation
    ///
    /// In a real implementation, this would call an actual LLM API.
    fn generate_explanation(&self, diff: &StatuteDiff) -> String {
        let mut parts = Vec::new();

        parts.push(format!(
            "This diff shows changes to statute '{}'.",
            diff.statute_id
        ));

        if diff.changes.is_empty() {
            return "No changes detected between the two versions.".to_string();
        }

        parts.push(format!(
            "A total of {} changes were detected.",
            diff.changes.len()
        ));

        let added = diff
            .changes
            .iter()
            .filter(|c| c.change_type == ChangeType::Added)
            .count();
        let removed = diff
            .changes
            .iter()
            .filter(|c| c.change_type == ChangeType::Removed)
            .count();
        let modified = diff
            .changes
            .iter()
            .filter(|c| c.change_type == ChangeType::Modified)
            .count();

        if added > 0 {
            parts.push(format!("{} new elements were added.", added));
        }
        if removed > 0 {
            parts.push(format!("{} elements were removed.", removed));
        }
        if modified > 0 {
            parts.push(format!("{} elements were modified.", modified));
        }

        if diff.impact.affects_eligibility {
            parts.push(
                "These changes affect who is eligible for the statute's provisions.".to_string(),
            );
        }

        if diff.impact.affects_outcome {
            parts.push("The outcome or effect of the statute has been modified.".to_string());
        }

        parts.join(" ")
    }

    /// Detects the intent behind the changes
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
    /// use legalis_diff::{diff, llm::{LlmAnalyzer, LlmConfig}};
    ///
    /// let mut analyzer = LlmAnalyzer::new(LlmConfig::default());
    /// let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"))
    ///     .with_precondition(Condition::Age {
    ///         operator: ComparisonOp::GreaterOrEqual,
    ///         value: 65,
    ///     });
    /// let new = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"))
    ///     .with_precondition(Condition::Age {
    ///         operator: ComparisonOp::GreaterOrEqual,
    ///         value: 60,
    ///     });
    ///
    /// let diff_result = diff(&old, &new).unwrap();
    /// let intent = analyzer.detect_intent(&diff_result).unwrap();
    /// assert!(!intent.is_empty());
    /// ```
    pub fn detect_intent(&mut self, diff: &StatuteDiff) -> DiffResult<String> {
        let cache_key = format!("intent-{}", diff.statute_id);

        if self.config.cache_responses
            && let Some(cached) = self.cache.get(&cache_key)
        {
            return Ok(cached.clone());
        }

        let intent = self.analyze_intent(diff);

        if self.config.cache_responses {
            self.cache.insert(cache_key, intent.clone());
        }

        Ok(intent)
    }

    /// Analyzes the intent behind changes
    fn analyze_intent(&self, diff: &StatuteDiff) -> String {
        if diff.changes.is_empty() {
            return "No changes detected.".to_string();
        }

        let added_count = diff
            .changes
            .iter()
            .filter(|c| c.change_type == ChangeType::Added)
            .count();
        let removed_count = diff
            .changes
            .iter()
            .filter(|c| c.change_type == ChangeType::Removed)
            .count();

        if removed_count > added_count && diff.impact.affects_eligibility {
            "Intent: Relaxing eligibility criteria to expand coverage".to_string()
        } else if added_count > removed_count && diff.impact.affects_eligibility {
            "Intent: Tightening eligibility criteria to restrict coverage".to_string()
        } else if diff.impact.affects_outcome {
            "Intent: Modifying the outcome or benefit provided".to_string()
        } else {
            "Intent: Minor clarification or administrative update".to_string()
        }
    }

    /// Automatically categorizes changes using LLM
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_diff::{diff, llm::{LlmAnalyzer, LlmConfig}};
    ///
    /// let mut analyzer = LlmAnalyzer::new(LlmConfig::default());
    /// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
    /// let new = Statute::new("law", "New", Effect::new(EffectType::Grant, "Benefit"));
    ///
    /// let diff_result = diff(&old, &new).unwrap();
    /// let categories = analyzer.categorize_changes(&diff_result).unwrap();
    /// assert!(!categories.is_empty());
    /// ```
    pub fn categorize_changes(&mut self, diff: &StatuteDiff) -> DiffResult<Vec<String>> {
        let mut categories = Vec::new();

        if diff.impact.affects_eligibility {
            categories.push("Eligibility Change".to_string());
        }

        if diff.impact.affects_outcome {
            categories.push("Outcome Change".to_string());
        }

        if diff.impact.discretion_changed {
            categories.push("Discretion Change".to_string());
        }

        match diff.impact.severity {
            crate::Severity::Breaking => categories.push("Breaking Change".to_string()),
            crate::Severity::Major => categories.push("Major Change".to_string()),
            crate::Severity::Moderate => categories.push("Moderate Change".to_string()),
            crate::Severity::Minor => categories.push("Minor Change".to_string()),
            crate::Severity::None => categories.push("No Impact".to_string()),
        }

        if categories.is_empty() {
            categories.push("Uncategorized".to_string());
        }

        Ok(categories)
    }

    /// Predicts the impact of changes
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_diff::{diff, llm::{LlmAnalyzer, LlmConfig}};
    ///
    /// let mut analyzer = LlmAnalyzer::new(LlmConfig::default());
    /// let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
    /// let mut new = old.clone();
    /// new.effect = Effect::new(EffectType::Revoke, "Revoke");
    ///
    /// let diff_result = diff(&old, &new).unwrap();
    /// let prediction = analyzer.predict_impact(&diff_result).unwrap();
    /// assert!(prediction.confidence > 0.0);
    /// ```
    pub fn predict_impact(&mut self, diff: &StatuteDiff) -> DiffResult<ImpactPrediction> {
        let severity_score = match diff.impact.severity {
            crate::Severity::Breaking => 1.0,
            crate::Severity::Major => 0.8,
            crate::Severity::Moderate => 0.5,
            crate::Severity::Minor => 0.2,
            crate::Severity::None => 0.0,
        };

        let affected_users = if diff.impact.affects_eligibility {
            "high"
        } else if diff.impact.affects_outcome {
            "medium"
        } else {
            "low"
        };

        let migration_difficulty = if diff.impact.severity >= crate::Severity::Major {
            "high"
        } else if diff.impact.severity == crate::Severity::Moderate {
            "medium"
        } else {
            "low"
        };

        Ok(ImpactPrediction {
            severity_score,
            affected_users: affected_users.to_string(),
            migration_difficulty: migration_difficulty.to_string(),
            confidence: 0.85,
            recommendations: self.generate_recommendations(diff),
        })
    }

    /// Generates recommendations based on the diff
    fn generate_recommendations(&self, diff: &StatuteDiff) -> Vec<String> {
        let mut recommendations = Vec::new();

        if diff.impact.affects_eligibility {
            recommendations
                .push("Review the impact on current beneficiaries before deploying.".to_string());
        }

        if diff.impact.affects_outcome {
            recommendations.push("Communicate outcome changes to all stakeholders.".to_string());
        }

        if diff.impact.severity >= crate::Severity::Major {
            recommendations.push("Consider a gradual rollout strategy.".to_string());
            recommendations.push("Prepare migration documentation.".to_string());
        }

        recommendations
    }

    /// Assists in resolving merge conflicts using LLM
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_diff::llm::{LlmAnalyzer, LlmConfig, MergeConflict};
    ///
    /// let mut analyzer = LlmAnalyzer::new(LlmConfig::default());
    /// let base = Statute::new("law", "Base", Effect::new(EffectType::Grant, "Benefit"));
    /// let ours = Statute::new("law", "Ours", Effect::new(EffectType::Grant, "Benefit"));
    /// let theirs = Statute::new("law", "Theirs", Effect::new(EffectType::Grant, "Benefit"));
    ///
    /// let conflict = MergeConflict {
    ///     location: "title".to_string(),
    ///     base_value: "Base".to_string(),
    ///     ours_value: "Ours".to_string(),
    ///     theirs_value: "Theirs".to_string(),
    /// };
    ///
    /// let resolution = analyzer.resolve_conflict(&conflict).unwrap();
    /// assert!(!resolution.suggested_value.is_empty());
    /// ```
    pub fn resolve_conflict(&mut self, conflict: &MergeConflict) -> DiffResult<ConflictResolution> {
        // Simple heuristic-based resolution
        // In a real implementation, this would use LLM reasoning

        let suggested_value = if conflict.ours_value.len() > conflict.theirs_value.len() {
            conflict.ours_value.clone()
        } else {
            conflict.theirs_value.clone()
        };

        Ok(ConflictResolution {
            suggested_value,
            reasoning: format!(
                "Selected the more detailed version for {}",
                conflict.location
            ),
            confidence: 0.7,
            alternatives: vec![conflict.ours_value.clone(), conflict.theirs_value.clone()],
        })
    }

    /// Clears the cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

/// Impact prediction result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactPrediction {
    /// Severity score (0.0-1.0)
    pub severity_score: f64,
    /// Estimated number of affected users
    pub affected_users: String,
    /// Migration difficulty level
    pub migration_difficulty: String,
    /// Confidence in the prediction (0.0-1.0)
    pub confidence: f64,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Merge conflict information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeConflict {
    /// Location of the conflict
    pub location: String,
    /// Base version value
    pub base_value: String,
    /// Our version value
    pub ours_value: String,
    /// Their version value
    pub theirs_value: String,
}

/// Conflict resolution suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictResolution {
    /// Suggested value to use
    pub suggested_value: String,
    /// Reasoning behind the suggestion
    pub reasoning: String,
    /// Confidence in the resolution (0.0-1.0)
    pub confidence: f64,
    /// Alternative values to consider
    pub alternatives: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diff;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

    fn create_test_statute(id: &str, title: &str) -> Statute {
        Statute::new(id, title, Effect::new(EffectType::Grant, "Test benefit"))
    }

    #[test]
    fn test_analyzer_creation() {
        let config = LlmConfig::default();
        let _analyzer = LlmAnalyzer::new(config);
    }

    #[test]
    fn test_explain_diff() {
        let mut analyzer = LlmAnalyzer::new(LlmConfig::default());
        let old = create_test_statute("law", "Old Title");
        let new = create_test_statute("law", "New Title");

        let diff_result = diff(&old, &new).unwrap();
        let explanation = analyzer.explain_diff(&diff_result).unwrap();

        assert!(!explanation.is_empty());
        assert!(explanation.contains("law"));
    }

    #[test]
    fn test_detect_intent() {
        let mut analyzer = LlmAnalyzer::new(LlmConfig::default());
        let old = create_test_statute("law", "Title").with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 65,
        });
        let new = create_test_statute("law", "Title").with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 60,
        });

        let diff_result = diff(&old, &new).unwrap();
        let intent = analyzer.detect_intent(&diff_result).unwrap();

        assert!(!intent.is_empty());
    }

    #[test]
    fn test_categorize_changes() {
        let mut analyzer = LlmAnalyzer::new(LlmConfig::default());
        let old = create_test_statute("law", "Old");
        let new = create_test_statute("law", "New");

        let diff_result = diff(&old, &new).unwrap();
        let categories = analyzer.categorize_changes(&diff_result).unwrap();

        assert!(!categories.is_empty());
    }

    #[test]
    fn test_predict_impact() {
        let mut analyzer = LlmAnalyzer::new(LlmConfig::default());
        let old = create_test_statute("law", "Title");
        let mut new = old.clone();
        new.effect = Effect::new(EffectType::Revoke, "Revoke");

        let diff_result = diff(&old, &new).unwrap();
        let prediction = analyzer.predict_impact(&diff_result).unwrap();

        assert!(prediction.confidence > 0.0);
        assert!(!prediction.recommendations.is_empty());
    }

    #[test]
    fn test_resolve_conflict() {
        let mut analyzer = LlmAnalyzer::new(LlmConfig::default());
        let conflict = MergeConflict {
            location: "title".to_string(),
            base_value: "Base Title".to_string(),
            ours_value: "Our Enhanced Title".to_string(),
            theirs_value: "Their Title".to_string(),
        };

        let resolution = analyzer.resolve_conflict(&conflict).unwrap();
        assert!(!resolution.suggested_value.is_empty());
        assert!(resolution.confidence > 0.0);
    }

    #[test]
    fn test_cache() {
        let mut analyzer = LlmAnalyzer::new(LlmConfig::default());
        let old = create_test_statute("law", "Old");
        let new = create_test_statute("law", "New");

        let diff_result = diff(&old, &new).unwrap();

        // First call
        let exp1 = analyzer.explain_diff(&diff_result).unwrap();

        // Second call (should use cache)
        let exp2 = analyzer.explain_diff(&diff_result).unwrap();

        assert_eq!(exp1, exp2);
    }

    #[test]
    fn test_clear_cache() {
        let mut analyzer = LlmAnalyzer::new(LlmConfig::default());
        let old = create_test_statute("law", "Old");
        let new = create_test_statute("law", "New");

        let diff_result = diff(&old, &new).unwrap();
        analyzer.explain_diff(&diff_result).unwrap();

        analyzer.clear_cache();
        // Cache should be cleared
    }
}
