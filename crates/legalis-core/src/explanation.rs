//! Legal Explanation Generation
//!
//! This module provides natural language explanations for legal reasoning results,
//! including counterfactual analysis ("why not?"), contrastive explanations, and
//! interactive drill-down capabilities.
//!
//! # Features
//!
//! - **Natural Language Generation**: Convert evaluation results to human-readable explanations
//! - **Counterfactual Explanation**: Explain why conditions failed and what would need to change
//! - **Contrastive Analysis**: Compare and explain differences between statutes
//! - **Interactive Drill-Down**: Explore complex nested evaluations step-by-step
//! - **Confidence Reporting**: Provide confidence scores for explanations
//!
//! # Examples
//!
//! ```
//! use legalis_core::*;
//! use legalis_core::explanation::*;
//!
//! // Create a condition
//! let condition = Condition::age(ComparisonOp::GreaterOrEqual, 65);
//!
//! // Generate natural language explanation
//! let explainer = NaturalLanguageExplainer::new();
//! let explanation = explainer.explain_condition(&condition);
//! assert!(explanation.contains("age"));
//! assert!(explanation.contains("65"));
//! ```

use crate::{ComparisonOp, Condition, Effect, EffectType, Statute};
use std::collections::HashMap;
use std::fmt;

/// Natural language explanation for a condition or evaluation
#[derive(Debug, Clone, PartialEq)]
pub struct Explanation {
    /// Human-readable explanation text
    pub text: String,

    /// Confidence score (0.0-1.0) for this explanation
    pub confidence: f64,

    /// Supporting details and evidence
    pub details: Vec<String>,

    /// Metadata about the explanation
    pub metadata: HashMap<String, String>,
}

impl Explanation {
    /// Create a new explanation
    pub fn new(text: String) -> Self {
        Self {
            text,
            confidence: 1.0,
            details: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Set confidence score
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    /// Add a detail
    pub fn with_detail(mut self, detail: String) -> Self {
        self.details.push(detail);
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Check if this is a high-confidence explanation
    pub fn is_high_confidence(&self) -> bool {
        self.confidence >= 0.8
    }

    /// Get confidence level as text
    pub fn confidence_level(&self) -> &'static str {
        if self.confidence >= 0.9 {
            "Very High"
        } else if self.confidence >= 0.7 {
            "High"
        } else if self.confidence >= 0.5 {
            "Moderate"
        } else if self.confidence >= 0.3 {
            "Low"
        } else {
            "Very Low"
        }
    }
}

impl fmt::Display for Explanation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.text)?;
        if !self.details.is_empty() {
            writeln!(f, "\nDetails:")?;
            for detail in &self.details {
                writeln!(f, "  • {}", detail)?;
            }
        }
        if self.confidence < 1.0 {
            writeln!(
                f,
                "\nConfidence: {} ({:.0}%)",
                self.confidence_level(),
                self.confidence * 100.0
            )?;
        }
        Ok(())
    }
}

/// Natural language explainer for conditions and evaluations
///
/// # Example
///
/// ```
/// use legalis_core::*;
/// use legalis_core::explanation::NaturalLanguageExplainer;
///
/// let explainer = NaturalLanguageExplainer::new();
/// let condition = Condition::income(ComparisonOp::GreaterThan, 50000);
/// let explanation = explainer.explain_condition(&condition);
/// assert!(explanation.contains("income"));
/// ```
#[derive(Debug, Clone)]
pub struct NaturalLanguageExplainer {
    /// Verbosity level (1-3)
    verbosity: u8,
}

impl NaturalLanguageExplainer {
    /// Create a new explainer with default verbosity (2)
    pub fn new() -> Self {
        Self { verbosity: 2 }
    }

    /// Set verbosity level (1=terse, 2=normal, 3=verbose)
    pub fn with_verbosity(mut self, verbosity: u8) -> Self {
        self.verbosity = verbosity.clamp(1, 3);
        self
    }

    /// Explain a condition in natural language
    pub fn explain_condition(&self, condition: &Condition) -> String {
        match condition {
            Condition::Age { operator, value } => {
                format!(
                    "The person's age must be {} {} years",
                    self.explain_operator(operator),
                    value
                )
            }
            Condition::Income { operator, value } => {
                format!(
                    "The person's income must be {} ${}",
                    self.explain_operator(operator),
                    value
                )
            }
            Condition::HasAttribute { key } => {
                format!("The entity must have the '{}' attribute", key)
            }
            Condition::AttributeEquals { key, value } => {
                format!("The '{}' attribute must equal '{}'", key, value)
            }
            Condition::And(left, right) => {
                let left_text = self.explain_condition(left);
                let right_text = self.explain_condition(right);
                format!(
                    "Both of the following must be true:\n  1. {}\n  2. {}",
                    left_text, right_text
                )
            }
            Condition::Or(left, right) => {
                let left_text = self.explain_condition(left);
                let right_text = self.explain_condition(right);
                format!(
                    "At least one of the following must be true:\n  1. {}\n  2. {}",
                    left_text, right_text
                )
            }
            Condition::Not(inner) => {
                format!(
                    "The following must NOT be true: {}",
                    self.explain_condition(inner)
                )
            }
            Condition::Percentage {
                operator,
                value,
                context,
            } => {
                format!(
                    "The percentage ({}) must be {} {}%",
                    context,
                    self.explain_operator(operator),
                    value
                )
            }
            Condition::Duration {
                operator,
                value,
                unit,
            } => {
                format!(
                    "The duration must be {} {} {:?}",
                    self.explain_operator(operator),
                    value,
                    unit
                )
            }
            Condition::Custom { description } => {
                format!("Custom condition: {}", description)
            }
            _ => format!("{}", condition),
        }
    }

    /// Explain a comparison operator
    fn explain_operator(&self, op: &ComparisonOp) -> &'static str {
        match op {
            ComparisonOp::Equal => "exactly",
            ComparisonOp::NotEqual => "not",
            ComparisonOp::GreaterThan => "greater than",
            ComparisonOp::LessThan => "less than",
            ComparisonOp::GreaterOrEqual => "at least",
            ComparisonOp::LessOrEqual => "at most",
        }
    }

    /// Explain a statute
    pub fn explain_statute(&self, statute: &Statute) -> Explanation {
        let mut text = format!("Statute: {}\n\n", statute.title);

        text.push_str(&format!(
            "Effect: {}\n\n",
            self.explain_effect(&statute.effect)
        ));

        if !statute.preconditions.is_empty() {
            text.push_str("Conditions:\n");
            for (i, condition) in statute.preconditions.iter().enumerate() {
                text.push_str(&format!(
                    "  {}. {}\n",
                    i + 1,
                    self.explain_condition(condition)
                ));
            }
        } else {
            text.push_str("This statute has no conditions and applies to everyone.\n");
        }

        Explanation::new(text).with_confidence(1.0)
    }

    /// Explain an effect
    fn explain_effect(&self, effect: &Effect) -> String {
        match effect.effect_type {
            EffectType::Grant => format!("Grants {}", effect.description),
            EffectType::Revoke => format!("Revokes {}", effect.description),
            EffectType::Obligation => format!("Creates an obligation to {}", effect.description),
            EffectType::Prohibition => format!("Prohibits {}", effect.description),
            EffectType::MonetaryTransfer => format!("Monetary transfer: {}", effect.description),
            EffectType::StatusChange => format!("Changes status: {}", effect.description),
            EffectType::Custom => effect.description.clone(),
        }
    }
}

impl Default for NaturalLanguageExplainer {
    fn default() -> Self {
        Self::new()
    }
}

/// Counterfactual explanation - explains why something didn't happen and what would need to change
///
/// # Example
///
/// ```
/// use legalis_core::*;
/// use legalis_core::explanation::CounterfactualExplainer;
///
/// let condition = Condition::age(ComparisonOp::GreaterOrEqual, 65);
/// let explainer = CounterfactualExplainer::new();
///
/// // Explain why age 60 doesn't satisfy the condition
/// let explanation = explainer.explain_failure(&condition, "age", "60");
/// assert!(explanation.text.contains("not met"));
/// ```
#[derive(Debug, Clone)]
pub struct CounterfactualExplainer {
    max_suggestions: usize,
}

impl CounterfactualExplainer {
    /// Create a new counterfactual explainer
    pub fn new() -> Self {
        Self { max_suggestions: 3 }
    }

    /// Set maximum number of suggestions
    pub fn with_max_suggestions(mut self, max: usize) -> Self {
        self.max_suggestions = max;
        self
    }

    /// Explain why a condition failed and what would need to change
    pub fn explain_failure(
        &self,
        condition: &Condition,
        attribute_name: &str,
        current_value: &str,
    ) -> Explanation {
        match condition {
            Condition::Age { operator, value } => {
                let suggestion = self.suggest_age_change(operator, *value, current_value);
                Explanation::new(format!(
                    "The age requirement is not met. Current age is {}, but it needs to be {}.",
                    current_value, suggestion
                ))
                .with_detail(format!(
                    "Requirement: age {} {}",
                    self.operator_text(operator),
                    value
                ))
                .with_confidence(0.9)
            }
            Condition::Income { operator, value } => {
                let suggestion = self.suggest_income_change(operator, *value, current_value);
                Explanation::new(format!(
                    "The income requirement is not met. Current income is ${}, but it needs to be {}.",
                    current_value, suggestion
                ))
                .with_detail(format!("Requirement: income {} ${}", self.operator_text(operator), value))
                .with_confidence(0.9)
            }
            Condition::HasAttribute { key } => Explanation::new(format!(
                "The required attribute '{}' is missing. You would need to add this attribute.",
                key
            ))
            .with_detail(format!("Add the '{}' attribute to your entity", key))
            .with_confidence(1.0),
            Condition::AttributeEquals { key, value } => Explanation::new(format!(
                "The '{}' attribute has value '{}', but it needs to be '{}'.",
                key, current_value, value
            ))
            .with_detail(format!(
                "Change '{}' from '{}' to '{}'",
                key, current_value, value
            ))
            .with_confidence(1.0),
            Condition::And(_left, _right) => Explanation::new(
                "This compound condition requires ALL parts to be satisfied.".to_string(),
            )
            .with_detail("Left condition may have failed".to_string())
            .with_detail("Right condition may have failed".to_string())
            .with_detail("Check each part individually".to_string())
            .with_confidence(0.6),
            Condition::Or(_left, _right) => Explanation::new(
                "This compound condition requires AT LEAST ONE part to be satisfied.".to_string(),
            )
            .with_detail("Neither left nor right condition was satisfied".to_string())
            .with_detail("Satisfy at least one of the alternatives".to_string())
            .with_confidence(0.6),
            Condition::Not(_inner) => Explanation::new(
                "This is a negation - the inner condition must be FALSE.".to_string(),
            )
            .with_detail("The inner condition is currently TRUE, but needs to be FALSE".to_string())
            .with_confidence(0.8),
            _ => Explanation::new(format!(
                "The condition '{}' is not satisfied with current value '{}'.",
                attribute_name, current_value
            ))
            .with_confidence(0.5),
        }
    }

    fn suggest_age_change(&self, op: &ComparisonOp, threshold: u32, _current: &str) -> String {
        match op {
            ComparisonOp::GreaterThan => format!("greater than {}", threshold),
            ComparisonOp::GreaterOrEqual => format!("at least {}", threshold),
            ComparisonOp::LessThan => format!("less than {}", threshold),
            ComparisonOp::LessOrEqual => format!("at most {}", threshold),
            ComparisonOp::Equal => format!("exactly {}", threshold),
            ComparisonOp::NotEqual => format!("anything except {}", threshold),
        }
    }

    fn suggest_income_change(&self, op: &ComparisonOp, threshold: u64, _current: &str) -> String {
        match op {
            ComparisonOp::GreaterThan => format!("greater than ${}", threshold),
            ComparisonOp::GreaterOrEqual => format!("at least ${}", threshold),
            ComparisonOp::LessThan => format!("less than ${}", threshold),
            ComparisonOp::LessOrEqual => format!("at most ${}", threshold),
            ComparisonOp::Equal => format!("exactly ${}", threshold),
            ComparisonOp::NotEqual => format!("anything except ${}", threshold),
        }
    }

    fn operator_text(&self, op: &ComparisonOp) -> &'static str {
        match op {
            ComparisonOp::Equal => "=",
            ComparisonOp::NotEqual => "≠",
            ComparisonOp::GreaterThan => ">",
            ComparisonOp::LessThan => "<",
            ComparisonOp::GreaterOrEqual => "≥",
            ComparisonOp::LessOrEqual => "≤",
        }
    }
}

impl Default for CounterfactualExplainer {
    fn default() -> Self {
        Self::new()
    }
}

/// Contrastive explainer - compares two statutes and explains their differences
///
/// # Example
///
/// ```
/// use legalis_core::*;
/// use legalis_core::explanation::ContrastiveExplainer;
///
/// let statute1 = Statute::new("S1", "Senior Discount", Effect::grant("10% discount"))
///     .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 65));
///
/// let statute2 = Statute::new("S2", "Student Discount", Effect::grant("15% discount"))
///     .with_precondition(Condition::has_attribute("student_id"));
///
/// let explainer = ContrastiveExplainer::new();
/// let comparison = explainer.compare(&statute1, &statute2);
/// assert!(comparison.text.contains("difference"));
/// ```
#[derive(Debug, Clone)]
pub struct ContrastiveExplainer;

impl ContrastiveExplainer {
    /// Create a new contrastive explainer
    pub fn new() -> Self {
        Self
    }

    /// Compare two statutes and explain their differences
    pub fn compare(&self, statute1: &Statute, statute2: &Statute) -> Explanation {
        let mut differences = Vec::new();
        let mut text = format!("Comparing '{}' vs '{}'\n\n", statute1.title, statute2.title);

        // Compare effects
        if statute1.effect.effect_type != statute2.effect.effect_type {
            differences.push(format!(
                "Different effect types: {:?} vs {:?}",
                statute1.effect.effect_type, statute2.effect.effect_type
            ));
        }

        if statute1.effect.description != statute2.effect.description {
            differences.push(format!(
                "Different effect descriptions: '{}' vs '{}'",
                statute1.effect.description, statute2.effect.description
            ));
        }

        // Compare number of conditions
        if statute1.preconditions.len() != statute2.preconditions.len() {
            differences.push(format!(
                "Different number of conditions: {} vs {}",
                statute1.preconditions.len(),
                statute2.preconditions.len()
            ));
        }

        // Compare jurisdictions
        if statute1.jurisdiction != statute2.jurisdiction {
            differences.push(format!(
                "Different jurisdictions: {:?} vs {:?}",
                statute1.jurisdiction, statute2.jurisdiction
            ));
        }

        if differences.is_empty() {
            text.push_str("These statutes are very similar with no major differences detected.");
        } else {
            text.push_str("Key differences:\n");
            for (i, diff) in differences.iter().enumerate() {
                text.push_str(&format!("  {}. {}\n", i + 1, diff));
            }
        }

        let confidence = if differences.is_empty() { 0.9 } else { 1.0 };

        Explanation::new(text)
            .with_confidence(confidence)
            .with_metadata("statute1_id".to_string(), statute1.id.clone())
            .with_metadata("statute2_id".to_string(), statute2.id.clone())
    }

    /// Explain why one statute applies but another doesn't
    pub fn explain_application_difference(
        &self,
        applies: &Statute,
        does_not_apply: &Statute,
    ) -> Explanation {
        Explanation::new(format!(
            "'{}' applies because its conditions are satisfied, \
             while '{}' does not apply because its conditions are not met.",
            applies.title, does_not_apply.title
        ))
        .with_detail(format!("Applying statute: {}", applies.id))
        .with_detail(format!("Non-applying statute: {}", does_not_apply.id))
        .with_confidence(0.9)
    }
}

impl Default for ContrastiveExplainer {
    fn default() -> Self {
        Self::new()
    }
}

/// Interactive explanation drill-down for complex nested evaluations
///
/// # Example
///
/// ```
/// use legalis_core::*;
/// use legalis_core::explanation::ExplanationDrillDown;
///
/// let condition = Condition::And(
///     Box::new(Condition::age(ComparisonOp::GreaterOrEqual, 65)),
///     Box::new(Condition::income(ComparisonOp::LessThan, 30000)),
/// );
///
/// let drill_down = ExplanationDrillDown::new(&condition);
/// let layers = drill_down.get_layers();
/// assert!(layers.len() > 0);
/// ```
#[derive(Debug, Clone)]
pub struct ExplanationDrillDown {
    /// Hierarchical layers of explanation
    layers: Vec<ExplanationLayer>,
}

/// A single layer in the drill-down hierarchy
#[derive(Debug, Clone)]
pub struct ExplanationLayer {
    /// Depth level (0 = top)
    pub depth: usize,

    /// Description of this layer
    pub description: String,

    /// Child layers
    pub children: Vec<ExplanationLayer>,
}

impl ExplanationDrillDown {
    /// Create a new drill-down explanation for a condition
    pub fn new(condition: &Condition) -> Self {
        let layers = vec![Self::build_layer(condition, 0)];
        Self { layers }
    }

    /// Build a layer from a condition
    fn build_layer(condition: &Condition, depth: usize) -> ExplanationLayer {
        match condition {
            Condition::And(left, right) => ExplanationLayer {
                depth,
                description: "AND: Both conditions must be true".to_string(),
                children: vec![
                    Self::build_layer(left, depth + 1),
                    Self::build_layer(right, depth + 1),
                ],
            },
            Condition::Or(left, right) => ExplanationLayer {
                depth,
                description: "OR: At least one condition must be true".to_string(),
                children: vec![
                    Self::build_layer(left, depth + 1),
                    Self::build_layer(right, depth + 1),
                ],
            },
            Condition::Not(inner) => ExplanationLayer {
                depth,
                description: "NOT: The following must be false".to_string(),
                children: vec![Self::build_layer(inner, depth + 1)],
            },
            other => ExplanationLayer {
                depth,
                description: format!("{}", other),
                children: Vec::new(),
            },
        }
    }

    /// Get all layers
    pub fn get_layers(&self) -> &[ExplanationLayer] {
        &self.layers
    }

    /// Get maximum depth
    pub fn max_depth(&self) -> usize {
        self.layers
            .iter()
            .map(Self::layer_max_depth)
            .max()
            .unwrap_or(0)
    }

    fn layer_max_depth(layer: &ExplanationLayer) -> usize {
        if layer.children.is_empty() {
            layer.depth
        } else {
            layer
                .children
                .iter()
                .map(Self::layer_max_depth)
                .max()
                .unwrap_or(layer.depth)
        }
    }

    /// Render as indented text
    pub fn render(&self) -> String {
        let mut result = String::new();
        for layer in &self.layers {
            Self::render_layer(layer, &mut result);
        }
        result
    }

    fn render_layer(layer: &ExplanationLayer, output: &mut String) {
        let indent = "  ".repeat(layer.depth);
        output.push_str(&format!("{}{}\n", indent, layer.description));
        for child in &layer.children {
            Self::render_layer(child, output);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explanation_confidence_levels() {
        assert_eq!(
            Explanation::new("test".to_string())
                .with_confidence(0.95)
                .confidence_level(),
            "Very High"
        );
        assert_eq!(
            Explanation::new("test".to_string())
                .with_confidence(0.75)
                .confidence_level(),
            "High"
        );
        assert_eq!(
            Explanation::new("test".to_string())
                .with_confidence(0.55)
                .confidence_level(),
            "Moderate"
        );
        assert_eq!(
            Explanation::new("test".to_string())
                .with_confidence(0.35)
                .confidence_level(),
            "Low"
        );
        assert_eq!(
            Explanation::new("test".to_string())
                .with_confidence(0.15)
                .confidence_level(),
            "Very Low"
        );
    }

    #[test]
    fn test_natural_language_explainer() {
        let explainer = NaturalLanguageExplainer::new();
        let condition = Condition::age(ComparisonOp::GreaterOrEqual, 65);
        let explanation = explainer.explain_condition(&condition);
        assert!(explanation.contains("age"));
        assert!(explanation.contains("65"));
    }

    #[test]
    fn test_counterfactual_explainer() {
        let explainer = CounterfactualExplainer::new();
        let condition = Condition::age(ComparisonOp::GreaterOrEqual, 65);
        let explanation = explainer.explain_failure(&condition, "age", "60");
        assert!(explanation.text.contains("not met"));
    }

    #[test]
    fn test_contrastive_explainer() {
        let explainer = ContrastiveExplainer::new();
        let statute1 = Statute::new("S1", "Test 1", Effect::grant("benefit"));
        let statute2 = Statute::new("S2", "Test 2", Effect::revoke("benefit"));
        let comparison = explainer.compare(&statute1, &statute2);
        assert!(comparison.text.contains("Comparing"));
    }

    #[test]
    fn test_drill_down() {
        let condition = Condition::And(
            Box::new(Condition::age(ComparisonOp::GreaterOrEqual, 65)),
            Box::new(Condition::income(ComparisonOp::LessThan, 30000)),
        );
        let drill_down = ExplanationDrillDown::new(&condition);
        assert!(drill_down.max_depth() > 0);
        let rendered = drill_down.render();
        assert!(rendered.contains("AND"));
    }
}
