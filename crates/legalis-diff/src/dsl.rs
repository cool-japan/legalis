//! Domain-Specific Language (DSL) for custom diff rules.
//!
//! This module provides a declarative way to define custom diff analysis rules
//! without writing full Rust code. Rules can be defined in a simple text format
//! and compiled into analyzers.
//!
//! # Example
//!
//! ```
//! use legalis_diff::dsl::{RuleBuilder, Condition, Action};
//!
//! let rule = RuleBuilder::new("check-age-changes")
//!     .description("Detect age-related changes")
//!     .when(Condition::FieldChanged("age".to_string()))
//!     .then(Action::Flag {
//!         severity: "high".to_string(),
//!         message: "Age requirement changed".to_string()
//!     })
//!     .build();
//! ```

use crate::plugins::{AnalysisResult, Finding, FindingSeverity};
use crate::{ChangeType, StatuteDiff};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A compiled diff analysis rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    /// Rule identifier
    pub id: String,
    /// Rule description
    pub description: String,
    /// Conditions that trigger the rule
    pub conditions: Vec<Condition>,
    /// Actions to take when conditions are met
    pub actions: Vec<Action>,
    /// Whether all conditions must match (true) or any (false)
    pub match_all: bool,
}

/// Conditions that can trigger a rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Condition {
    /// Field was changed
    FieldChanged(String),
    /// Change type matches
    ChangeType(ChangeType),
    /// Change count exceeds threshold
    ChangeCountGreaterThan(usize),
    /// Impact severity matches
    ImpactSeverity(String),
    /// Custom predicate (field, operator, value)
    CustomPredicate {
        field: String,
        operator: Operator,
        value: String,
    },
    /// Combined conditions (AND)
    And(Vec<Condition>),
    /// Combined conditions (OR)
    Or(Vec<Condition>),
    /// Negated condition
    Not(Box<Condition>),
}

/// Comparison operators for custom predicates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    Contains,
    Matches,
}

/// Actions to take when rule conditions are met.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    /// Flag the diff with a finding
    Flag { severity: String, message: String },
    /// Add metadata to the analysis
    AddMetadata { key: String, value: String },
    /// Compute a custom metric
    ComputeMetric { name: String, formula: String },
    /// Execute a callback (stored as a name reference)
    Callback(String),
}

/// Builder for creating rules.
pub struct RuleBuilder {
    id: String,
    description: String,
    conditions: Vec<Condition>,
    actions: Vec<Action>,
    match_all: bool,
}

impl RuleBuilder {
    /// Creates a new rule builder.
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            description: String::new(),
            conditions: Vec::new(),
            actions: Vec::new(),
            match_all: true,
        }
    }

    /// Sets the rule description.
    #[must_use]
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    /// Adds a condition to the rule.
    #[must_use]
    pub fn when(mut self, condition: Condition) -> Self {
        self.conditions.push(condition);
        self
    }

    /// Adds an action to the rule.
    #[must_use]
    pub fn then(mut self, action: Action) -> Self {
        self.actions.push(action);
        self
    }

    /// Sets whether all conditions must match (default: true).
    #[must_use]
    pub fn match_all(mut self, match_all: bool) -> Self {
        self.match_all = match_all;
        self
    }

    /// Builds the rule.
    #[must_use]
    pub fn build(self) -> Rule {
        Rule {
            id: self.id,
            description: self.description,
            conditions: self.conditions,
            actions: self.actions,
            match_all: self.match_all,
        }
    }
}

/// Callback function type for rule actions.
type CallbackFn = Box<dyn Fn(&StatuteDiff) -> AnalysisResult + Send + Sync>;

/// A rule engine for evaluating rules against diffs.
pub struct RuleEngine {
    rules: Vec<Rule>,
    callbacks: HashMap<String, CallbackFn>,
}

impl RuleEngine {
    /// Creates a new rule engine.
    #[must_use]
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            callbacks: HashMap::new(),
        }
    }

    /// Adds a rule to the engine.
    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    /// Registers a callback function.
    pub fn register_callback<F>(&mut self, name: String, callback: F)
    where
        F: Fn(&StatuteDiff) -> AnalysisResult + Send + Sync + 'static,
    {
        self.callbacks.insert(name, Box::new(callback));
    }

    /// Evaluates all rules against a diff.
    pub fn evaluate(&self, diff: &StatuteDiff) -> Vec<AnalysisResult> {
        let mut results = Vec::new();

        for rule in &self.rules {
            if self.matches_conditions(diff, &rule.conditions, rule.match_all) {
                if let Some(result) = self.execute_actions(diff, rule) {
                    results.push(result);
                }
            }
        }

        results
    }

    /// Checks if conditions match the diff.
    fn matches_conditions(
        &self,
        diff: &StatuteDiff,
        conditions: &[Condition],
        match_all: bool,
    ) -> bool {
        if conditions.is_empty() {
            return true;
        }

        if match_all {
            conditions
                .iter()
                .all(|cond| self.evaluate_condition(diff, cond))
        } else {
            conditions
                .iter()
                .any(|cond| self.evaluate_condition(diff, cond))
        }
    }

    /// Evaluates a single condition.
    fn evaluate_condition(&self, diff: &StatuteDiff, condition: &Condition) -> bool {
        match condition {
            Condition::FieldChanged(field) => {
                // Check if any change involves this field
                diff.changes.iter().any(|change| {
                    change
                        .description
                        .to_lowercase()
                        .contains(&field.to_lowercase())
                })
            }
            Condition::ChangeType(change_type) => {
                diff.changes.iter().any(|change| match change_type {
                    ChangeType::Added => matches!(change.change_type, ChangeType::Added),
                    ChangeType::Removed => matches!(change.change_type, ChangeType::Removed),
                    ChangeType::Modified => matches!(change.change_type, ChangeType::Modified),
                    ChangeType::Reordered => matches!(change.change_type, ChangeType::Reordered),
                })
            }
            Condition::ChangeCountGreaterThan(threshold) => diff.changes.len() > *threshold,
            Condition::ImpactSeverity(severity) => {
                format!("{:?}", diff.impact.severity).to_lowercase() == severity.to_lowercase()
            }
            Condition::CustomPredicate {
                field,
                operator,
                value,
            } => self.evaluate_predicate(diff, field, operator, value),
            Condition::And(conditions) => self.matches_conditions(diff, conditions, true),
            Condition::Or(conditions) => self.matches_conditions(diff, conditions, false),
            Condition::Not(condition) => !self.evaluate_condition(diff, condition),
        }
    }

    /// Evaluates a custom predicate.
    fn evaluate_predicate(
        &self,
        diff: &StatuteDiff,
        field: &str,
        operator: &Operator,
        value: &str,
    ) -> bool {
        // Extract field value from diff
        let field_value = match field {
            "change_count" => Some(diff.changes.len().to_string()),
            "statute_id" => Some(diff.statute_id.clone()),
            "old_version" => diff
                .version_info
                .as_ref()
                .and_then(|v| v.old_version.map(|ver| ver.to_string())),
            "new_version" => diff
                .version_info
                .as_ref()
                .and_then(|v| v.new_version.map(|ver| ver.to_string())),
            _ => None,
        };

        if let Some(field_value) = field_value {
            match operator {
                Operator::Equals => field_value == value,
                Operator::NotEquals => field_value != value,
                Operator::GreaterThan => field_value
                    .parse::<f64>()
                    .and_then(|fv| value.parse::<f64>().map(|v| fv > v))
                    .unwrap_or(false),
                Operator::LessThan => field_value
                    .parse::<f64>()
                    .and_then(|fv| value.parse::<f64>().map(|v| fv < v))
                    .unwrap_or(false),
                Operator::Contains => field_value.contains(value),
                Operator::Matches => {
                    // Simple pattern matching (could be enhanced with regex)
                    field_value.contains(value)
                }
            }
        } else {
            false
        }
    }

    /// Executes rule actions.
    fn execute_actions(&self, diff: &StatuteDiff, rule: &Rule) -> Option<AnalysisResult> {
        let mut findings = Vec::new();
        let mut metadata = HashMap::new();

        for action in &rule.actions {
            match action {
                Action::Flag { severity, message } => {
                    let finding_severity = match severity.to_lowercase().as_str() {
                        "critical" => FindingSeverity::Critical,
                        "high" => FindingSeverity::High,
                        "medium" => FindingSeverity::Medium,
                        "low" => FindingSeverity::Low,
                        _ => FindingSeverity::Info,
                    };

                    findings.push(Finding {
                        severity: finding_severity,
                        category: rule.id.clone(),
                        message: message.clone(),
                        location: None,
                        suggestion: None,
                    });
                }
                Action::AddMetadata { key, value } => {
                    metadata.insert(key.clone(), value.clone());
                }
                Action::ComputeMetric { name, formula } => {
                    // Basic formula evaluation (could be enhanced)
                    let computed_value = self.evaluate_formula(diff, formula);
                    metadata.insert(name.clone(), computed_value);
                }
                Action::Callback(name) => {
                    if let Some(callback) = self.callbacks.get(name) {
                        return Some(callback(diff));
                    }
                }
            }
        }

        if !findings.is_empty() || !metadata.is_empty() {
            Some(AnalysisResult {
                plugin_name: format!("dsl-rule:{}", rule.id),
                findings,
                confidence: 0.9,
                metadata,
            })
        } else {
            None
        }
    }

    /// Evaluates a formula (basic implementation).
    #[allow(dead_code)]
    fn evaluate_formula(&self, diff: &StatuteDiff, formula: &str) -> String {
        // Basic formula evaluation - could be enhanced with a proper expression parser
        match formula {
            "change_count" => diff.changes.len().to_string(),
            "added_count" => diff
                .changes
                .iter()
                .filter(|c| matches!(c.change_type, ChangeType::Added))
                .count()
                .to_string(),
            "removed_count" => diff
                .changes
                .iter()
                .filter(|c| matches!(c.change_type, ChangeType::Removed))
                .count()
                .to_string(),
            "modified_count" => diff
                .changes
                .iter()
                .filter(|c| matches!(c.change_type, ChangeType::Modified))
                .count()
                .to_string(),
            _ => "0".to_string(),
        }
    }
}

impl Default for RuleEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Parses a rule from a simple text format.
///
/// # Format
///
/// ```text
/// RULE rule_id
/// DESCRIPTION "Rule description"
/// WHEN FieldChanged("field_name")
/// THEN Flag(severity="high", message="Message")
/// ```
pub fn parse_rule(text: &str) -> Result<Rule, String> {
    let mut id = String::new();
    let mut description = String::new();
    let mut conditions = Vec::new();
    let mut actions = Vec::new();

    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let line = line.trim();

        if line.starts_with("RULE ") {
            id = line.strip_prefix("RULE ").unwrap().trim().to_string();
        } else if line.starts_with("DESCRIPTION ") {
            description = line
                .strip_prefix("DESCRIPTION ")
                .unwrap()
                .trim()
                .trim_matches('"')
                .to_string();
        } else if line.starts_with("WHEN ") {
            let condition_str = line.strip_prefix("WHEN ").unwrap().trim();
            if let Some(cond) = parse_condition(condition_str) {
                conditions.push(cond);
            }
        } else if line.starts_with("THEN ") {
            let action_str = line.strip_prefix("THEN ").unwrap().trim();
            if let Some(action) = parse_action(action_str) {
                actions.push(action);
            }
        }
    }

    if id.is_empty() {
        return Err("Rule ID is required".to_string());
    }

    Ok(Rule {
        id,
        description,
        conditions,
        actions,
        match_all: true,
    })
}

/// Parses a condition from text.
fn parse_condition(text: &str) -> Option<Condition> {
    if text.starts_with("FieldChanged(") {
        let field = text
            .strip_prefix("FieldChanged(")?
            .strip_suffix(')')?
            .trim_matches('"');
        Some(Condition::FieldChanged(field.to_string()))
    } else if text.starts_with("ChangeType(") {
        let change_type_str = text
            .strip_prefix("ChangeType(")?
            .strip_suffix(')')?
            .trim_matches('"');
        let change_type = match change_type_str {
            "Added" => ChangeType::Added,
            "Removed" => ChangeType::Removed,
            "Modified" => ChangeType::Modified,
            _ => return None,
        };
        Some(Condition::ChangeType(change_type))
    } else if text.starts_with("ChangeCountGreaterThan(") {
        let count = text
            .strip_prefix("ChangeCountGreaterThan(")?
            .strip_suffix(')')?
            .parse()
            .ok()?;
        Some(Condition::ChangeCountGreaterThan(count))
    } else {
        None
    }
}

/// Parses an action from text.
fn parse_action(text: &str) -> Option<Action> {
    if text.starts_with("Flag(") {
        // Parse Flag(severity="high", message="Message")
        let content = text.strip_prefix("Flag(")?.strip_suffix(')')?;
        let mut severity = String::new();
        let mut message = String::new();

        for part in content.split(',') {
            let part = part.trim();
            if let Some(value) = part.strip_prefix("severity=") {
                severity = value.trim_matches('"').to_string();
            } else if let Some(value) = part.strip_prefix("message=") {
                message = value.trim_matches('"').to_string();
            }
        }

        Some(Action::Flag { severity, message })
    } else if text.starts_with("AddMetadata(") {
        let content = text.strip_prefix("AddMetadata(")?.strip_suffix(')')?;
        let mut key = String::new();
        let mut value = String::new();

        for part in content.split(',') {
            let part = part.trim();
            if let Some(k) = part.strip_prefix("key=") {
                key = k.trim_matches('"').to_string();
            } else if let Some(v) = part.strip_prefix("value=") {
                value = v.trim_matches('"').to_string();
            }
        }

        Some(Action::AddMetadata { key, value })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Change, ChangeTarget, ImpactAssessment, Severity};

    #[test]
    fn test_rule_builder() {
        let rule = RuleBuilder::new("test-rule")
            .description("Test rule")
            .when(Condition::FieldChanged("age".to_string()))
            .then(Action::Flag {
                severity: "high".to_string(),
                message: "Age changed".to_string(),
            })
            .build();

        assert_eq!(rule.id, "test-rule");
        assert_eq!(rule.description, "Test rule");
        assert_eq!(rule.conditions.len(), 1);
        assert_eq!(rule.actions.len(), 1);
    }

    #[test]
    fn test_rule_engine() {
        let mut engine = RuleEngine::new();

        let rule = RuleBuilder::new("age-change-detector")
            .description("Detect age changes")
            .when(Condition::FieldChanged("age".to_string()))
            .then(Action::Flag {
                severity: "high".to_string(),
                message: "Age requirement changed".to_string(),
            })
            .build();

        engine.add_rule(rule);

        let diff = StatuteDiff {
            statute_id: "test-123".to_string(),
            version_info: None,
            changes: vec![Change {
                change_type: ChangeType::Modified,
                target: ChangeTarget::Precondition { index: 0 },
                description: "Age requirement changed from 65 to 60".to_string(),
                old_value: Some("65".to_string()),
                new_value: Some("60".to_string()),
            }],
            impact: ImpactAssessment {
                severity: Severity::Moderate,
                affects_eligibility: true,
                affects_outcome: false,
                discretion_changed: false,
                notes: vec!["Test impact".to_string()],
            },
        };

        let results = engine.evaluate(&diff);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].findings.len(), 1);
        assert_eq!(results[0].findings[0].message, "Age requirement changed");
    }

    #[test]
    fn test_parse_rule() {
        let rule_text = r#"
RULE age-change-detector
DESCRIPTION "Detect age changes"
WHEN FieldChanged("age")
THEN Flag(severity="high", message="Age changed")
"#;

        let rule = parse_rule(rule_text).unwrap();
        assert_eq!(rule.id, "age-change-detector");
        assert_eq!(rule.description, "Detect age changes");
        assert_eq!(rule.conditions.len(), 1);
        assert_eq!(rule.actions.len(), 1);
    }
}
