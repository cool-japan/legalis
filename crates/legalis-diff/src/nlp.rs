//! Natural Language Processing module for generating human-readable summaries.
//!
//! This module provides functions to convert diff results into natural language
//! summaries that are easy to understand without technical jargon.
//!
//! # Examples
//!
//! ```
//! use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
//! use legalis_diff::{diff, nlp};
//!
//! let old = Statute::new(
//!     "tax-credit",
//!     "Senior Tax Credit",
//!     Effect::new(EffectType::Grant, "Tax credit granted"),
//! ).with_precondition(Condition::Age {
//!     operator: ComparisonOp::GreaterOrEqual,
//!     value: 65,
//! });
//!
//! let mut new = old.clone();
//! new.preconditions[0] = Condition::Age {
//!     operator: ComparisonOp::GreaterOrEqual,
//!     value: 60,
//! };
//!
//! let diff_result = diff(&old, &new).unwrap();
//! let summary = nlp::generate_natural_language_summary(&diff_result);
//! let text = nlp::format_summary_text(&summary);
//!
//! println!("{}", text);
//! // Output: "The statute 'tax-credit' has been modified with moderate impact..."
//! ```

use crate::{Change, ChangeTarget, ChangeType, Severity, StatuteDiff};
use serde::{Deserialize, Serialize};

/// Natural language summary of a diff.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NaturalLanguageSummary {
    /// The statute ID.
    pub statute_id: String,
    /// Overall summary sentence.
    pub summary: String,
    /// Detailed explanation paragraphs.
    pub explanation: Vec<String>,
    /// Key impacts in plain language.
    pub impacts: Vec<String>,
    /// Recommendations or notes.
    pub recommendations: Vec<String>,
}

/// Generates a natural language summary of a diff.
///
/// This function converts technical diff information into easy-to-understand
/// natural language descriptions suitable for non-technical stakeholders.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, nlp::generate_natural_language_summary};
///
/// let old = Statute::new("law", "Old Title", Effect::new(EffectType::Grant, "Benefit"));
/// let mut new = old.clone();
/// new.title = "New Title".to_string();
///
/// let diff_result = diff(&old, &new).unwrap();
/// let summary = generate_natural_language_summary(&diff_result);
///
/// assert!(summary.summary.contains("law"));
/// ```
pub fn generate_natural_language_summary(diff: &StatuteDiff) -> NaturalLanguageSummary {
    let mut explanation = Vec::new();
    let mut impacts = Vec::new();
    let mut recommendations = Vec::new();

    // Generate summary sentence
    let severity_desc = match diff.impact.severity {
        Severity::None => "no significant impact",
        Severity::Minor => "minor impact",
        Severity::Moderate => "moderate impact",
        Severity::Major => "major impact",
        Severity::Breaking => "breaking changes",
    };

    let change_count = diff.changes.len();
    let change_word = if change_count == 1 {
        "change"
    } else {
        "changes"
    };

    let summary = if change_count == 0 {
        format!(
            "The statute '{}' has not been modified. No changes were detected.",
            diff.statute_id
        )
    } else {
        format!(
            "The statute '{}' has been modified with {} ({} {}).",
            diff.statute_id, severity_desc, change_count, change_word
        )
    };

    // Generate detailed explanations
    if change_count > 0 {
        explanation.push(format!(
            "This analysis identified {} {} to the statute:",
            change_count, change_word
        ));

        // Group changes by type
        let added_changes: Vec<_> = diff
            .changes
            .iter()
            .filter(|c| c.change_type == ChangeType::Added)
            .collect();
        let removed_changes: Vec<_> = diff
            .changes
            .iter()
            .filter(|c| c.change_type == ChangeType::Removed)
            .collect();
        let modified_changes: Vec<_> = diff
            .changes
            .iter()
            .filter(|c| c.change_type == ChangeType::Modified)
            .collect();

        if !added_changes.is_empty() {
            let desc = generate_changes_description(&added_changes, "added");
            explanation.push(desc);
        }

        if !removed_changes.is_empty() {
            let desc = generate_changes_description(&removed_changes, "removed");
            explanation.push(desc);
        }

        if !modified_changes.is_empty() {
            let desc = generate_changes_description(&modified_changes, "modified");
            explanation.push(desc);
        }
    }

    // Generate impact statements
    if diff.impact.affects_eligibility {
        impacts.push(
            "Who is eligible: The requirements for being eligible under this statute have changed. \
             This may affect the number of people or entities that qualify."
                .to_string(),
        );
    }

    if diff.impact.affects_outcome {
        impacts.push(
            "What happens: The outcome or effect of the statute has been modified. \
             This changes what happens when the statute applies."
                .to_string(),
        );
    }

    if diff.impact.discretion_changed {
        impacts.push(
            "Decision-making: The requirements for human judgment or discretion have changed. \
             This affects how the statute is applied in practice."
                .to_string(),
        );
    }

    // Generate recommendations based on severity
    match diff.impact.severity {
        Severity::None => {
            recommendations.push(
                "No action required. The statute remains functionally unchanged.".to_string(),
            );
        }
        Severity::Minor => {
            recommendations.push(
                "Review recommended. While changes are minor, it's good practice to review \
                 the modifications to ensure they align with expectations."
                    .to_string(),
            );
        }
        Severity::Moderate => {
            recommendations.push(
                "Careful review required. These changes may affect how the statute is applied. \
                 Stakeholders should be informed of the modifications."
                    .to_string(),
            );
        }
        Severity::Major => {
            recommendations.push(
                "Immediate attention required. These are significant changes that will affect \
                 statute application. All stakeholders must be notified and systems may need updates."
                    .to_string(),
            );
            if diff.impact.affects_eligibility {
                recommendations.push(
                    "Update eligibility criteria in all relevant systems and documentation."
                        .to_string(),
                );
            }
            if diff.impact.affects_outcome {
                recommendations.push(
                    "Review and update all outcome-related processes and expectations.".to_string(),
                );
            }
        }
        Severity::Breaking => {
            recommendations.push(
                "Critical: Breaking changes detected. This statute has been fundamentally altered. \
                 Comprehensive review, system updates, and stakeholder communication are essential."
                    .to_string(),
            );
            recommendations.push(
                "Create a migration plan to transition from the old version to the new version."
                    .to_string(),
            );
            recommendations.push(
                "Consider maintaining backward compatibility or providing a transition period."
                    .to_string(),
            );
        }
    }

    NaturalLanguageSummary {
        statute_id: diff.statute_id.clone(),
        summary,
        explanation,
        impacts,
        recommendations,
    }
}

/// Generates a description for a group of changes.
#[allow(dead_code)]
fn generate_changes_description(changes: &[&Change], action: &str) -> String {
    let count = changes.len();
    let word = if count == 1 { "item" } else { "items" };

    let mut parts = Vec::new();
    for change in changes {
        let part = match &change.target {
            ChangeTarget::Title => "the title".to_string(),
            ChangeTarget::Precondition { index } => {
                format!("eligibility requirement #{}", index + 1)
            }
            ChangeTarget::Effect => "the outcome/effect".to_string(),
            ChangeTarget::DiscretionLogic => "the discretion requirements".to_string(),
            ChangeTarget::Metadata { key } => format!("metadata field '{}'", key),
        };
        parts.push(part);
    }

    format!("{} {} were {}: {}", count, word, action, parts.join(", "))
}

/// Explains a single change in natural language.
///
/// # Examples
///
/// ```
/// use legalis_diff::{Change, ChangeType, ChangeTarget, nlp::explain_change};
///
/// let change = Change {
///     change_type: ChangeType::Modified,
///     target: ChangeTarget::Title,
///     description: "Title was modified".to_string(),
///     old_value: Some("Old Title".to_string()),
///     new_value: Some("New Title".to_string()),
/// };
///
/// let explanation = explain_change(&change);
/// assert!(explanation.contains("title"));
/// ```
pub fn explain_change(change: &Change) -> String {
    let action = match change.change_type {
        ChangeType::Added => "was added",
        ChangeType::Removed => "was removed",
        ChangeType::Modified => "was changed",
        ChangeType::Reordered => "was reordered",
    };

    let target_desc = match &change.target {
        ChangeTarget::Title => "The statute's title".to_string(),
        ChangeTarget::Precondition { index } => {
            format!("Eligibility requirement #{}", index + 1)
        }
        ChangeTarget::Effect => "The statute's outcome".to_string(),
        ChangeTarget::DiscretionLogic => "The discretion requirements".to_string(),
        ChangeTarget::Metadata { key } => format!("The metadata field '{}'", key),
    };

    let mut explanation = format!("{} {}.", target_desc, action);

    // Add before/after context if available
    match (&change.old_value, &change.new_value) {
        (Some(old), Some(new)) => {
            explanation.push_str(&format!(" It changed from '{}' to '{}'.", old, new));
        }
        (None, Some(new)) => {
            explanation.push_str(&format!(" The new value is '{}'.", new));
        }
        (Some(old), None) => {
            explanation.push_str(&format!(" The previous value was '{}'.", old));
        }
        (None, None) => {}
    }

    explanation
}

/// Extracts the intent or purpose of a change from its description.
///
/// This analyzes the nature of changes to infer why they were made.
///
/// # Examples
///
/// ```
/// use legalis_diff::{StatuteDiff, nlp::extract_intent};
///
/// // Create a sample diff (would normally come from diff function)
/// let diff = StatuteDiff {
///     statute_id: "tax-law".to_string(),
///     version_info: None,
///     changes: vec![],
///     impact: Default::default(),
/// };
///
/// let intent = extract_intent(&diff);
/// assert!(!intent.is_empty());
/// ```
pub fn extract_intent(diff: &StatuteDiff) -> String {
    if diff.changes.is_empty() {
        return "No changes were made to this statute.".to_string();
    }

    let mut intents = Vec::new();

    // Analyze the pattern of changes
    let has_precondition_relaxation = diff.changes.iter().any(|c| {
        matches!(&c.target, ChangeTarget::Precondition { .. })
            && c.change_type == ChangeType::Removed
    });

    let has_precondition_tightening = diff.changes.iter().any(|c| {
        matches!(&c.target, ChangeTarget::Precondition { .. }) && c.change_type == ChangeType::Added
    });

    let has_effect_change = diff
        .changes
        .iter()
        .any(|c| matches!(&c.target, ChangeTarget::Effect));

    let has_title_change = diff
        .changes
        .iter()
        .any(|c| matches!(&c.target, ChangeTarget::Title));

    let has_discretion_change = diff
        .changes
        .iter()
        .any(|c| matches!(&c.target, ChangeTarget::DiscretionLogic));

    // Infer intent from change patterns
    if has_precondition_relaxation {
        intents.push("expand eligibility to include more people or entities".to_string());
    }

    if has_precondition_tightening {
        intents.push("restrict eligibility to a more specific group".to_string());
    }

    if has_effect_change {
        intents.push("modify what happens when the statute applies".to_string());
    }

    if has_title_change
        && !has_effect_change
        && !has_precondition_relaxation
        && !has_precondition_tightening
    {
        intents.push("clarify or update the statute's name without functional changes".to_string());
    }

    if has_discretion_change && diff.impact.discretion_changed {
        intents.push("change how human judgment is applied in decision-making".to_string());
    }

    if intents.is_empty() {
        "make technical or administrative updates".to_string()
    } else if intents.len() == 1 {
        format!("The intent appears to be to {}.", intents[0])
    } else {
        format!(
            "The intent appears to be to: {}.",
            intents.join(", and to ")
        )
    }
}

/// Compares semantic similarity between two change descriptions.
///
/// Returns a similarity score between 0.0 (completely different) and 1.0 (identical).
///
/// # Examples
///
/// ```
/// use legalis_diff::nlp::semantic_similarity;
///
/// let similarity = semantic_similarity(
///     "Age requirement changed from 65 to 60",
///     "Age requirement changed from 65 to 60"
/// );
/// assert!(similarity > 0.9);
/// ```
pub fn semantic_similarity(text1: &str, text2: &str) -> f64 {
    // Basic similarity using word overlap (Jaccard similarity)
    let text1_lower = text1.to_lowercase();
    let text2_lower = text2.to_lowercase();

    let words1: std::collections::HashSet<_> = text1_lower.split_whitespace().collect();
    let words2: std::collections::HashSet<_> = text2_lower.split_whitespace().collect();

    let intersection = words1.intersection(&words2).count();
    let union = words1.union(&words2).count();

    if union == 0 {
        0.0
    } else {
        intersection as f64 / union as f64
    }
}

/// Formats a natural language summary as plain text.
pub fn format_summary_text(summary: &NaturalLanguageSummary) -> String {
    let mut output = String::new();

    output.push_str("=".repeat(70).as_str());
    output.push('\n');
    output.push_str(&format!("STATUTE CHANGE SUMMARY: {}\n", summary.statute_id));
    output.push_str("=".repeat(70).as_str());
    output.push_str("\n\n");

    output.push_str("OVERVIEW\n");
    output.push_str("-".repeat(70).as_str());
    output.push('\n');
    output.push_str(&summary.summary);
    output.push_str("\n\n");

    if !summary.explanation.is_empty() {
        output.push_str("DETAILED CHANGES\n");
        output.push_str("-".repeat(70).as_str());
        output.push('\n');
        for explanation in &summary.explanation {
            output.push_str(explanation);
            output.push('\n');
        }
        output.push('\n');
    }

    if !summary.impacts.is_empty() {
        output.push_str("KEY IMPACTS\n");
        output.push_str("-".repeat(70).as_str());
        output.push('\n');
        for impact in &summary.impacts {
            output.push_str("• ");
            output.push_str(impact);
            output.push('\n');
        }
        output.push('\n');
    }

    if !summary.recommendations.is_empty() {
        output.push_str("RECOMMENDATIONS\n");
        output.push_str("-".repeat(70).as_str());
        output.push('\n');
        for (i, rec) in summary.recommendations.iter().enumerate() {
            output.push_str(&format!("{}. ", i + 1));
            output.push_str(rec);
            output.push('\n');
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ImpactAssessment, diff};
    use legalis_core::{Effect, EffectType, Statute};

    #[test]
    fn test_generate_natural_language_summary_no_changes() {
        let statute = Statute::new(
            "test-law",
            "Test Law",
            Effect::new(EffectType::Grant, "Benefit"),
        );

        let diff_result = diff(&statute, &statute).unwrap();
        let summary = generate_natural_language_summary(&diff_result);

        assert!(summary.summary.contains("not been modified"));
        assert_eq!(summary.statute_id, "test-law");
    }

    #[test]
    fn test_generate_natural_language_summary_with_changes() {
        let old = Statute::new(
            "test-law",
            "Old Title",
            Effect::new(EffectType::Grant, "Benefit"),
        );
        let mut new = old.clone();
        new.title = "New Title".to_string();

        let diff_result = diff(&old, &new).unwrap();
        let summary = generate_natural_language_summary(&diff_result);

        assert!(summary.summary.contains("modified"));
        assert!(summary.summary.contains("1 change"));
    }

    #[test]
    fn test_explain_change() {
        let change = Change {
            change_type: ChangeType::Modified,
            target: ChangeTarget::Title,
            description: "Title changed".to_string(),
            old_value: Some("Old".to_string()),
            new_value: Some("New".to_string()),
        };

        let explanation = explain_change(&change);
        assert!(explanation.contains("title"));
        assert!(explanation.contains("changed"));
    }

    #[test]
    fn test_extract_intent_no_changes() {
        let diff = StatuteDiff {
            statute_id: "test".to_string(),
            version_info: None,
            changes: vec![],
            impact: ImpactAssessment::default(),
        };

        let intent = extract_intent(&diff);
        assert!(intent.contains("No changes"));
    }

    #[test]
    fn test_extract_intent_precondition_relaxation() {
        let diff = StatuteDiff {
            statute_id: "test".to_string(),
            version_info: None,
            changes: vec![Change {
                change_type: ChangeType::Removed,
                target: ChangeTarget::Precondition { index: 0 },
                description: "Removed precondition".to_string(),
                old_value: Some("Old".to_string()),
                new_value: None,
            }],
            impact: ImpactAssessment::default(),
        };

        let intent = extract_intent(&diff);
        assert!(intent.contains("expand eligibility"));
    }

    #[test]
    fn test_semantic_similarity_identical() {
        let sim = semantic_similarity("hello world", "hello world");
        assert_eq!(sim, 1.0);
    }

    #[test]
    fn test_semantic_similarity_different() {
        let sim = semantic_similarity("hello world", "foo bar");
        assert_eq!(sim, 0.0);
    }

    #[test]
    fn test_semantic_similarity_partial() {
        let sim = semantic_similarity("hello world foo", "hello world bar");
        assert!(sim > 0.4 && sim < 0.8);
    }

    #[test]
    fn test_format_summary_text() {
        let summary = NaturalLanguageSummary {
            statute_id: "test-law".to_string(),
            summary: "Test summary".to_string(),
            explanation: vec!["Explanation 1".to_string()],
            impacts: vec!["Impact 1".to_string()],
            recommendations: vec!["Recommendation 1".to_string()],
        };

        let text = format_summary_text(&summary);
        assert!(text.contains("test-law"));
        assert!(text.contains("Test summary"));
        assert!(text.contains("Explanation 1"));
    }
}
