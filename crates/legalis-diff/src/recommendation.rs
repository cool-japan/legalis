//! Change recommendation system for statute amendments.
//!
//! This module analyzes patterns in statute changes and provides
//! recommendations for potential amendments based on historical patterns,
//! common practices, and detected inconsistencies.
//!
//! # Example
//!
//! ```
//! use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
//! use legalis_diff::{diff, recommendation::analyze_and_recommend};
//!
//! let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"))
//!     .with_precondition(Condition::Age {
//!         operator: ComparisonOp::GreaterOrEqual,
//!         value: 65,
//!     });
//!
//! let mut new = old.clone();
//! new.preconditions[0] = Condition::Age {
//!     operator: ComparisonOp::GreaterOrEqual,
//!     value: 60,
//! };
//!
//! let diff_result = diff(&old, &new).unwrap();
//! let recommendations = analyze_and_recommend(&diff_result, &[]);
//! ```

use crate::{ChangeTarget, ChangeType, StatuteDiff};
use serde::{Deserialize, Serialize};

/// Priority level for a recommendation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RecommendationPriority {
    /// Low priority suggestion.
    Low,
    /// Medium priority suggestion.
    Medium,
    /// High priority suggestion.
    High,
    /// Critical issue that should be addressed.
    Critical,
}

/// Category of recommendation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecommendationCategory {
    /// Consistency improvement.
    Consistency,
    /// Clarity improvement.
    Clarity,
    /// Legal compliance.
    Compliance,
    /// Best practice.
    BestPractice,
    /// Potential error.
    PotentialError,
    /// Performance optimization.
    Performance,
}

/// A recommendation for a statute change.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    /// Priority level.
    pub priority: RecommendationPriority,
    /// Category.
    pub category: RecommendationCategory,
    /// Title of the recommendation.
    pub title: String,
    /// Detailed description.
    pub description: String,
    /// Rationale for the recommendation.
    pub rationale: String,
    /// Suggested action.
    pub suggested_action: Option<String>,
    /// Confidence score (0.0 to 1.0).
    pub confidence: f64,
    /// Related changes.
    pub related_changes: Vec<String>,
}

impl Recommendation {
    /// Creates a new recommendation.
    pub fn new(
        priority: RecommendationPriority,
        category: RecommendationCategory,
        title: String,
        description: String,
    ) -> Self {
        Self {
            priority,
            category,
            title,
            description,
            rationale: String::new(),
            suggested_action: None,
            confidence: 1.0,
            related_changes: Vec::new(),
        }
    }

    /// Sets the rationale for this recommendation.
    pub fn with_rationale(mut self, rationale: String) -> Self {
        self.rationale = rationale;
        self
    }

    /// Sets the suggested action.
    pub fn with_suggested_action(mut self, action: String) -> Self {
        self.suggested_action = Some(action);
        self
    }

    /// Sets the confidence score.
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    /// Adds a related change reference.
    pub fn with_related_change(mut self, change_desc: String) -> Self {
        self.related_changes.push(change_desc);
        self
    }
}

/// Analyzes a diff and generates recommendations.
///
/// Takes into account historical patterns if provided.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, recommendation::analyze_and_recommend};
///
/// let old = Statute::new("law", "Old Title", Effect::new(EffectType::Grant, "Benefit"));
/// let mut new = old.clone();
/// new.title = "New Title".to_string();
///
/// let diff_result = diff(&old, &new).unwrap();
/// let recommendations = analyze_and_recommend(&diff_result, &[]);
/// ```
pub fn analyze_and_recommend(
    diff: &StatuteDiff,
    historical_diffs: &[StatuteDiff],
) -> Vec<Recommendation> {
    let mut recommendations = Vec::new();

    // Check for incomplete changes
    recommendations.extend(check_incomplete_changes(diff));

    // Check for consistency issues
    recommendations.extend(check_consistency_issues(diff));

    // Check for breaking changes without documentation
    recommendations.extend(check_breaking_changes(diff));

    // Analyze historical patterns
    if !historical_diffs.is_empty() {
        recommendations.extend(analyze_historical_patterns(diff, historical_diffs));
    }

    // Check for common pitfalls
    recommendations.extend(check_common_pitfalls(diff));

    recommendations
}

/// Checks for incomplete or partial changes.
fn check_incomplete_changes(diff: &StatuteDiff) -> Vec<Recommendation> {
    let mut recommendations = Vec::new();

    // Check if title changed but metadata wasn't updated
    let has_title_change = diff
        .changes
        .iter()
        .any(|c| matches!(c.target, ChangeTarget::Title));

    let has_metadata_update = diff
        .changes
        .iter()
        .any(|c| matches!(c.target, ChangeTarget::Metadata { .. }));

    if has_title_change && !has_metadata_update {
        recommendations.push(
            Recommendation::new(
                RecommendationPriority::Medium,
                RecommendationCategory::BestPractice,
                "Consider updating metadata".to_string(),
                "The statute title changed but metadata was not updated.".to_string(),
            )
            .with_rationale(
                "When changing a statute title, it's good practice to update relevant metadata such as amendment date and version.".to_string()
            )
            .with_suggested_action(
                "Add metadata updates to reflect the title change.".to_string()
            )
            .with_confidence(0.8),
        );
    }

    recommendations
}

/// Checks for consistency issues in changes.
fn check_consistency_issues(diff: &StatuteDiff) -> Vec<Recommendation> {
    let mut recommendations = Vec::new();

    // Check for multiple precondition modifications without clear pattern
    let precondition_changes: Vec<_> = diff
        .changes
        .iter()
        .filter(|c| matches!(c.target, ChangeTarget::Precondition { .. }))
        .collect();

    if precondition_changes.len() > 3 {
        recommendations.push(
            Recommendation::new(
                RecommendationPriority::Medium,
                RecommendationCategory::Clarity,
                "Multiple precondition changes detected".to_string(),
                format!(
                    "{} preconditions were modified. Consider reviewing for consistency.",
                    precondition_changes.len()
                ),
            )
            .with_rationale(
                "Multiple simultaneous precondition changes can indicate a significant policy shift and should be carefully reviewed.".to_string()
            )
            .with_confidence(0.7),
        );
    }

    recommendations
}

/// Checks for breaking changes and suggests documentation.
fn check_breaking_changes(diff: &StatuteDiff) -> Vec<Recommendation> {
    let mut recommendations = Vec::new();

    if diff.impact.affects_outcome || diff.impact.affects_eligibility {
        recommendations.push(
            Recommendation::new(
                RecommendationPriority::High,
                RecommendationCategory::BestPractice,
                "Breaking change detected".to_string(),
                "This change affects eligibility or outcomes and should be well-documented.".to_string(),
            )
            .with_rationale(
                "Changes that affect who is eligible or what outcomes occur are significant and require clear documentation for affected parties.".to_string()
            )
            .with_suggested_action(
                "Document the change impact, affected population, and transition plan.".to_string()
            )
            .with_confidence(0.95),
        );
    }

    if diff.impact.discretion_changed {
        recommendations.push(
            Recommendation::new(
                RecommendationPriority::High,
                RecommendationCategory::Compliance,
                "Discretion requirements changed".to_string(),
                "Changes to discretion requirements affect decision-making processes.".to_string(),
            )
            .with_rationale(
                "Modifications to discretionary judgment requirements impact how decisions are made and should include guidance for decision-makers.".to_string()
            )
            .with_suggested_action(
                "Provide updated guidance or training materials for decision-makers.".to_string()
            )
            .with_confidence(0.9),
        );
    }

    recommendations
}

/// Analyzes historical patterns to suggest improvements.
fn analyze_historical_patterns(
    diff: &StatuteDiff,
    historical_diffs: &[StatuteDiff],
) -> Vec<Recommendation> {
    let mut recommendations = Vec::new();

    // Count common change types in history
    let mut change_type_counts = std::collections::HashMap::new();
    for hist_diff in historical_diffs {
        for change in &hist_diff.changes {
            *change_type_counts.entry(change.change_type).or_insert(0) += 1;
        }
    }

    // Check if current changes align with historical patterns
    for change in &diff.changes {
        if let Some(&count) = change_type_counts.get(&change.change_type) {
            if count > 5 {
                // Frequent change type
                recommendations.push(
                    Recommendation::new(
                        RecommendationPriority::Low,
                        RecommendationCategory::BestPractice,
                        "Common change pattern detected".to_string(),
                        format!(
                            "This type of change ({:?}) is common in this statute's history.",
                            change.change_type
                        ),
                    )
                    .with_rationale(
                        "Historical data shows this is a frequently modified area. Consider standardizing this change pattern.".to_string()
                    )
                    .with_confidence(0.6),
                );
            }
        }
    }

    recommendations
}

/// Checks for common pitfalls in statute amendments.
fn check_common_pitfalls(diff: &StatuteDiff) -> Vec<Recommendation> {
    let mut recommendations = Vec::new();

    // Check for effect changes without precondition review
    let has_effect_change = diff
        .changes
        .iter()
        .any(|c| matches!(c.target, ChangeTarget::Effect));

    let has_precondition_review = diff
        .changes
        .iter()
        .any(|c| matches!(c.target, ChangeTarget::Precondition { .. }));

    if has_effect_change && !has_precondition_review {
        recommendations.push(
            Recommendation::new(
                RecommendationPriority::Medium,
                RecommendationCategory::PotentialError,
                "Effect changed without precondition review".to_string(),
                "The effect was modified, but preconditions were not reviewed.".to_string(),
            )
            .with_rationale(
                "When changing the effect of a statute, it's important to review whether the preconditions still make sense.".to_string()
            )
            .with_suggested_action(
                "Review preconditions to ensure they align with the new effect.".to_string()
            )
            .with_confidence(0.75),
        );
    }

    // Check for removed preconditions that might be too permissive
    let removed_preconditions = diff
        .changes
        .iter()
        .filter(|c| {
            matches!(c.change_type, ChangeType::Removed)
                && matches!(c.target, ChangeTarget::Precondition { .. })
        })
        .count();

    if removed_preconditions > 1 {
        recommendations.push(
            Recommendation::new(
                RecommendationPriority::High,
                RecommendationCategory::PotentialError,
                "Multiple preconditions removed".to_string(),
                format!(
                    "{} preconditions were removed. This significantly broadens eligibility.",
                    removed_preconditions
                ),
            )
            .with_rationale(
                "Removing multiple preconditions at once can have unintended consequences and should be carefully reviewed.".to_string()
            )
            .with_suggested_action(
                "Verify that the broader eligibility is intentional and document the rationale.".to_string()
            )
            .with_confidence(0.85),
        );
    }

    recommendations
}

/// Filters recommendations by priority.
pub fn filter_by_priority(
    recommendations: &[Recommendation],
    min_priority: RecommendationPriority,
) -> Vec<Recommendation> {
    recommendations
        .iter()
        .filter(|r| r.priority >= min_priority)
        .cloned()
        .collect()
}

/// Filters recommendations by category.
pub fn filter_by_category(
    recommendations: &[Recommendation],
    category: RecommendationCategory,
) -> Vec<Recommendation> {
    recommendations
        .iter()
        .filter(|r| r.category == category)
        .cloned()
        .collect()
}

/// Sorts recommendations by priority (highest first).
pub fn sort_by_priority(recommendations: &mut [Recommendation]) {
    recommendations.sort_by(|a, b| b.priority.cmp(&a.priority));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diff;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

    #[test]
    fn test_analyze_and_recommend_basic() {
        let old = Statute::new(
            "law",
            "Old Title",
            Effect::new(EffectType::Grant, "Benefit"),
        );
        let mut new = old.clone();
        new.title = "New Title".to_string();

        let diff_result = diff(&old, &new).unwrap();
        let recommendations = analyze_and_recommend(&diff_result, &[]);

        assert!(!recommendations.is_empty());
    }

    #[test]
    fn test_breaking_change_recommendation() {
        let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
        let mut new = old.clone();
        new.effect = Effect::new(EffectType::Revoke, "Revoke benefit");

        let diff_result = diff(&old, &new).unwrap();
        let recommendations = analyze_and_recommend(&diff_result, &[]);

        let has_breaking_warning = recommendations
            .iter()
            .any(|r| r.priority == RecommendationPriority::High);
        assert!(has_breaking_warning);
    }

    #[test]
    fn test_filter_by_priority() {
        let recs = vec![
            Recommendation::new(
                RecommendationPriority::Low,
                RecommendationCategory::BestPractice,
                "Low".to_string(),
                "Test".to_string(),
            ),
            Recommendation::new(
                RecommendationPriority::High,
                RecommendationCategory::Compliance,
                "High".to_string(),
                "Test".to_string(),
            ),
        ];

        let high_priority = filter_by_priority(&recs, RecommendationPriority::High);
        assert_eq!(high_priority.len(), 1);
        assert_eq!(high_priority[0].priority, RecommendationPriority::High);
    }

    #[test]
    fn test_sort_by_priority() {
        let mut recs = vec![
            Recommendation::new(
                RecommendationPriority::Low,
                RecommendationCategory::BestPractice,
                "Low".to_string(),
                "Test".to_string(),
            ),
            Recommendation::new(
                RecommendationPriority::Critical,
                RecommendationCategory::Compliance,
                "Critical".to_string(),
                "Test".to_string(),
            ),
            Recommendation::new(
                RecommendationPriority::Medium,
                RecommendationCategory::Clarity,
                "Medium".to_string(),
                "Test".to_string(),
            ),
        ];

        sort_by_priority(&mut recs);
        assert_eq!(recs[0].priority, RecommendationPriority::Critical);
        assert_eq!(recs[2].priority, RecommendationPriority::Low);
    }

    #[test]
    fn test_multiple_precondition_removal_warning() {
        let mut old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
        old.preconditions.push(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });
        old.preconditions.push(Condition::Income {
            operator: ComparisonOp::LessOrEqual,
            value: 50000,
        });

        let new = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));

        let diff_result = diff(&old, &new).unwrap();
        let recommendations = analyze_and_recommend(&diff_result, &[]);

        let has_warning = recommendations
            .iter()
            .any(|r| r.title.contains("Multiple preconditions removed"));
        assert!(has_warning);
    }
}
