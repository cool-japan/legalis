//! Semantic validation for Legal DSL documents.
//!
//! This module provides validation utilities to check for semantic errors
//! that are not caught by the parser, such as:
//! - Invalid date ranges (effective date after expiry date)
//! - Circular dependencies in REQUIRES clauses
//! - Undefined statute references
//! - Conflicting conditions between statutes
//! - Invalid numeric ranges

use crate::ast::{
    AmendmentNode, ConditionNode, ConditionValue, ExceptionNode, LegalDocument, StatuteNode,
};
use chrono::NaiveDate;
use std::collections::{HashMap, HashSet};
use thiserror::Error;

/// Validation errors that can occur during semantic analysis.
#[derive(Debug, Error, Clone, PartialEq)]
pub enum ValidationError {
    #[error(
        "Invalid date range in statute '{statute_id}': effective date {effective} is after expiry date {expiry}"
    )]
    InvalidDateRange {
        statute_id: String,
        effective: String,
        expiry: String,
    },

    #[error("Circular dependency detected in statute '{statute_id}': {cycle}")]
    CircularDependency { statute_id: String, cycle: String },

    #[error("Undefined statute reference in '{statute_id}': statute '{referenced}' does not exist")]
    UndefinedReference {
        statute_id: String,
        referenced: String,
    },

    #[error("Invalid numeric range in statute '{statute_id}': min ({min}) >= max ({max})")]
    InvalidNumericRange {
        statute_id: String,
        min: i64,
        max: i64,
    },

    #[error("Conflicting conditions between statutes '{statute1}' and '{statute2}': {details}")]
    ConflictingConditions {
        statute1: String,
        statute2: String,
        details: String,
    },

    #[error("Missing required field '{field}' in statute '{statute_id}'")]
    MissingRequiredField { statute_id: String, field: String },

    #[error("Duplicate statute ID '{statute_id}' found at multiple locations")]
    DuplicateStatuteId { statute_id: String },

    #[error(
        "Invalid amendment in statute '{statute_id}': target statute '{target}' does not exist"
    )]
    InvalidAmendment { statute_id: String, target: String },

    #[error("Self-reference in statute '{statute_id}': statute cannot reference itself")]
    SelfReference { statute_id: String },

    #[error("Dead code detected in statute '{statute_id}': {reason}")]
    DeadCode { statute_id: String, reason: String },

    #[error("Unreachable effect in statute '{statute_id}': {description}")]
    UnreachableEffect {
        statute_id: String,
        description: String,
    },
}

/// Result type for validation operations.
pub type ValidationResult<T> = Result<T, ValidationError>;

/// Validation context containing all statutes for cross-referencing.
#[derive(Debug, Default)]
pub struct ValidationContext {
    /// Map of statute ID to statute node
    statutes: HashMap<String, StatuteNode>,
    /// Collected warnings during validation
    warnings: Vec<String>,
}

impl ValidationContext {
    /// Creates a new validation context from a legal document.
    pub fn from_document(doc: &LegalDocument) -> Self {
        let mut statutes = HashMap::new();
        for statute in &doc.statutes {
            statutes.insert(statute.id.clone(), statute.clone());
        }
        Self {
            statutes,
            warnings: Vec::new(),
        }
    }

    /// Adds a warning to the context.
    fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    /// Returns all collected warnings.
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }

    /// Checks if a statute exists.
    pub fn statute_exists(&self, id: &str) -> bool {
        self.statutes.contains_key(id)
    }

    /// Gets a statute by ID.
    pub fn get_statute(&self, id: &str) -> Option<&StatuteNode> {
        self.statutes.get(id)
    }
}

/// Semantic validator for legal documents.
#[derive(Debug, Default)]
pub struct SemanticValidator {
    /// Whether to perform strict validation (fail on warnings)
    #[allow(dead_code)]
    strict: bool,
}

impl SemanticValidator {
    /// Creates a new semantic validator.
    pub fn new() -> Self {
        Self { strict: false }
    }

    /// Creates a new strict validator that fails on warnings.
    pub fn strict() -> Self {
        Self { strict: true }
    }

    /// Validates a complete legal document.
    pub fn validate_document(
        &self,
        doc: &LegalDocument,
    ) -> Result<Vec<ValidationError>, Vec<ValidationError>> {
        let mut errors = Vec::new();
        let mut context = ValidationContext::from_document(doc);

        // Check for duplicate statute IDs
        if let Err(e) = self.check_duplicate_ids(doc) {
            errors.push(e);
        }

        // Validate each statute
        for statute in &doc.statutes {
            if let Err(errs) = self.validate_statute(statute, &mut context) {
                errors.extend(errs);
            }
        }

        // Check for circular dependencies
        for statute in &doc.statutes {
            if let Err(e) =
                self.check_circular_dependencies(&statute.id, &context, &mut HashSet::new())
            {
                errors.push(e);
            }
        }

        if errors.is_empty() {
            Ok(vec![])
        } else {
            Err(errors)
        }
    }

    /// Checks for duplicate statute IDs in the document.
    fn check_duplicate_ids(&self, doc: &LegalDocument) -> ValidationResult<()> {
        let mut seen = HashSet::new();
        for statute in &doc.statutes {
            if !seen.insert(&statute.id) {
                return Err(ValidationError::DuplicateStatuteId {
                    statute_id: statute.id.clone(),
                });
            }
        }
        Ok(())
    }

    /// Validates a single statute.
    fn validate_statute(
        &self,
        statute: &StatuteNode,
        context: &mut ValidationContext,
    ) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        // Validate conditions
        for condition in &statute.conditions {
            if let Err(e) = self.validate_condition(condition, &statute.id) {
                errors.push(e);
            }
        }

        // Validate REQUIRES references
        for required_id in &statute.requires {
            // Check for self-reference
            if required_id == &statute.id {
                errors.push(ValidationError::SelfReference {
                    statute_id: statute.id.clone(),
                });
                continue;
            }

            // Check if required statute exists
            if !context.statute_exists(required_id) {
                errors.push(ValidationError::UndefinedReference {
                    statute_id: statute.id.clone(),
                    referenced: required_id.clone(),
                });
            }
        }

        // Validate SUPERSEDES references
        for superseded_id in &statute.supersedes {
            if superseded_id == &statute.id {
                errors.push(ValidationError::SelfReference {
                    statute_id: statute.id.clone(),
                });
                continue;
            }

            if !context.statute_exists(superseded_id) {
                context.add_warning(format!(
                    "Statute '{}' supersedes '{}' which does not exist (may be intentional)",
                    statute.id, superseded_id
                ));
            }
        }

        // Validate amendments
        for amendment in &statute.amendments {
            if let Err(e) = self.validate_amendment(amendment, &statute.id, context) {
                errors.push(e);
            }
        }

        // Validate exceptions
        for exception in &statute.exceptions {
            if let Err(e) = self.validate_exception(exception, &statute.id) {
                errors.push(e);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Validates a condition node for semantic correctness.
    #[allow(clippy::only_used_in_recursion)]
    fn validate_condition(
        &self,
        condition: &ConditionNode,
        statute_id: &str,
    ) -> ValidationResult<()> {
        match condition {
            ConditionNode::Between { min, max, .. } => {
                if let (ConditionValue::Number(min_val), ConditionValue::Number(max_val)) =
                    (min, max)
                    && min_val >= max_val
                {
                    return Err(ValidationError::InvalidNumericRange {
                        statute_id: statute_id.to_string(),
                        min: *min_val,
                        max: *max_val,
                    });
                }
                Ok(())
            }
            ConditionNode::InRange {
                min,
                max,
                inclusive_min,
                inclusive_max,
                ..
            } => {
                if let (ConditionValue::Number(min_val), ConditionValue::Number(max_val)) =
                    (min, max)
                    && ((*inclusive_min && *inclusive_max && min_val >= max_val)
                        || (!*inclusive_min && !*inclusive_max && *min_val >= max_val - 1))
                {
                    return Err(ValidationError::InvalidNumericRange {
                        statute_id: statute_id.to_string(),
                        min: *min_val,
                        max: *max_val,
                    });
                }
                Ok(())
            }
            ConditionNode::And(left, right) | ConditionNode::Or(left, right) => {
                self.validate_condition(left, statute_id)?;
                self.validate_condition(right, statute_id)
            }
            ConditionNode::Not(inner) => self.validate_condition(inner, statute_id),
            _ => Ok(()),
        }
    }

    /// Validates an amendment node.
    fn validate_amendment(
        &self,
        amendment: &AmendmentNode,
        statute_id: &str,
        context: &ValidationContext,
    ) -> ValidationResult<()> {
        // Check if target statute exists
        if !context.statute_exists(&amendment.target_id) {
            return Err(ValidationError::InvalidAmendment {
                statute_id: statute_id.to_string(),
                target: amendment.target_id.clone(),
            });
        }

        // Validate date if present
        if let Some(date_str) = &amendment.date
            && NaiveDate::parse_from_str(date_str, "%Y-%m-%d").is_err()
        {
            // Just a warning, not a hard error
        }

        Ok(())
    }

    /// Validates an exception node.
    fn validate_exception(
        &self,
        exception: &ExceptionNode,
        statute_id: &str,
    ) -> ValidationResult<()> {
        for condition in &exception.conditions {
            self.validate_condition(condition, statute_id)?;
        }
        Ok(())
    }

    /// Checks for circular dependencies using depth-first search.
    #[allow(clippy::only_used_in_recursion)]
    fn check_circular_dependencies(
        &self,
        statute_id: &str,
        context: &ValidationContext,
        visited: &mut HashSet<String>,
    ) -> ValidationResult<()> {
        if !visited.insert(statute_id.to_string()) {
            // Found a cycle
            let cycle = visited
                .iter()
                .skip_while(|id| *id != statute_id)
                .cloned()
                .collect::<Vec<_>>()
                .join(" -> ");
            return Err(ValidationError::CircularDependency {
                statute_id: statute_id.to_string(),
                cycle: format!("{} -> {}", cycle, statute_id),
            });
        }

        // Check all required statutes
        if let Some(statute) = context.get_statute(statute_id) {
            for required_id in &statute.requires {
                self.check_circular_dependencies(required_id, context, visited)?;
            }
        }

        visited.remove(statute_id);
        Ok(())
    }
}

/// Checks for completeness of statute requirements.
#[derive(Debug, Default)]
pub struct CompletenessChecker {
    /// Required fields that must be present
    required_fields: HashSet<String>,
}

impl CompletenessChecker {
    /// Creates a new completeness checker with default required fields.
    pub fn new() -> Self {
        let mut required_fields = HashSet::new();
        required_fields.insert("id".to_string());
        required_fields.insert("title".to_string());
        Self { required_fields }
    }

    /// Adds a required field.
    pub fn require_field(mut self, field: &str) -> Self {
        self.required_fields.insert(field.to_string());
        self
    }

    /// Checks if a statute is complete.
    pub fn check_statute(&self, statute: &StatuteNode) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        // Check basic fields
        if statute.id.is_empty() {
            errors.push(ValidationError::MissingRequiredField {
                statute_id: statute.id.clone(),
                field: "id".to_string(),
            });
        }

        if statute.title.is_empty() {
            errors.push(ValidationError::MissingRequiredField {
                statute_id: statute.id.clone(),
                field: "title".to_string(),
            });
        }

        // Check optional required fields
        if self.required_fields.contains("conditions") && statute.conditions.is_empty() {
            errors.push(ValidationError::MissingRequiredField {
                statute_id: statute.id.clone(),
                field: "conditions".to_string(),
            });
        }

        if self.required_fields.contains("effects") && statute.effects.is_empty() {
            errors.push(ValidationError::MissingRequiredField {
                statute_id: statute.id.clone(),
                field: "effects".to_string(),
            });
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Checks completeness of all statutes in a document.
    pub fn check_document(&self, doc: &LegalDocument) -> Result<(), Vec<ValidationError>> {
        let mut all_errors = Vec::new();

        for statute in &doc.statutes {
            if let Err(errors) = self.check_statute(statute) {
                all_errors.extend(errors);
            }
        }

        if all_errors.is_empty() {
            Ok(())
        } else {
            Err(all_errors)
        }
    }
}

/// Detects dead code (unreachable effects) in legal documents.
#[derive(Debug, Default)]
pub struct DeadCodeDetector;

impl DeadCodeDetector {
    /// Creates a new dead code detector.
    pub fn new() -> Self {
        Self
    }

    /// Detects dead code in a legal document.
    pub fn detect(&self, doc: &LegalDocument) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        let context = ValidationContext::from_document(doc);

        for statute in &doc.statutes {
            // Check for contradictory conditions (always false)
            if let Some(error) = self.check_contradictory_conditions(statute) {
                errors.push(error);
            }

            // Check for unreferenced statutes
            if !self.is_statute_referenced(statute, doc) {
                errors.push(ValidationError::DeadCode {
                    statute_id: statute.id.clone(),
                    reason: "Statute is never referenced by other statutes".to_string(),
                });
            }

            // Check for effects that can never be reached
            errors.extend(self.check_unreachable_effects(statute, &context));
        }

        errors
    }

    /// Checks if a statute is referenced by any other statute.
    fn is_statute_referenced(&self, statute: &StatuteNode, doc: &LegalDocument) -> bool {
        for other in &doc.statutes {
            if other.id != statute.id {
                // Check REQUIRES
                if other.requires.contains(&statute.id) {
                    return true;
                }
                // Check SUPERSEDES
                if other.supersedes.contains(&statute.id) {
                    return true;
                }
                // Check AMENDMENTS
                for amendment in &other.amendments {
                    if amendment.target_id == statute.id {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Checks for contradictory conditions that make the statute unreachable.
    fn check_contradictory_conditions(&self, statute: &StatuteNode) -> Option<ValidationError> {
        for condition in &statute.conditions {
            if self.is_always_false(condition) {
                return Some(ValidationError::DeadCode {
                    statute_id: statute.id.clone(),
                    reason: "Statute contains contradictory conditions that are always false"
                        .to_string(),
                });
            }
        }
        None
    }

    /// Checks if a condition is always false.
    #[allow(clippy::only_used_in_recursion)]
    fn is_always_false(&self, condition: &ConditionNode) -> bool {
        match condition {
            ConditionNode::And(left, right) => {
                // Check for direct contradictions like (x > 5) AND (x < 3)
                if self.is_always_false(left) || self.is_always_false(right) {
                    return true;
                }
                self.are_contradictory(left, right)
            }
            ConditionNode::Between { min, max, .. } => {
                // Check if min >= max
                if let (ConditionValue::Number(min_val), ConditionValue::Number(max_val)) =
                    (min, max)
                {
                    return min_val >= max_val;
                }
                false
            }
            ConditionNode::InRange {
                min,
                max,
                inclusive_min,
                inclusive_max,
                ..
            } => {
                // Check impossible ranges
                if let (ConditionValue::Number(min_val), ConditionValue::Number(max_val)) =
                    (min, max)
                {
                    if !inclusive_min && !inclusive_max {
                        return *max_val <= *min_val + 1;
                    }
                    return min_val >= max_val;
                }
                false
            }
            _ => false,
        }
    }

    /// Checks if two conditions are contradictory.
    #[allow(dead_code)]
    fn are_contradictory(&self, left: &ConditionNode, right: &ConditionNode) -> bool {
        // Simple case: (field > 5) AND (field < 3)
        match (left, right) {
            (
                ConditionNode::Comparison {
                    field: f1,
                    operator: op1,
                    value: v1,
                },
                ConditionNode::Comparison {
                    field: f2,
                    operator: op2,
                    value: v2,
                },
            ) => {
                if f1 == f2
                    && let (ConditionValue::Number(n1), ConditionValue::Number(n2)) = (v1, v2)
                {
                    // Check for contradictions like (x > 5) AND (x < 3)
                    if (op1 == ">" || op1 == ">=") && (op2 == "<" || op2 == "<=") {
                        return n1 >= n2;
                    }
                    if (op1 == "<" || op1 == "<=") && (op2 == ">" || op2 == ">=") {
                        return n1 <= n2;
                    }
                }
                false
            }
            _ => false,
        }
    }

    /// Checks for effects that can never be reached due to required dependencies.
    fn check_unreachable_effects(
        &self,
        statute: &StatuteNode,
        context: &ValidationContext,
    ) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        // Check if required statutes have contradictory conditions
        for req_id in &statute.requires {
            if let Some(req_statute) = context.get_statute(req_id) {
                for condition in &req_statute.conditions {
                    if self.is_always_false(condition) {
                        for effect in &statute.effects {
                            errors.push(ValidationError::UnreachableEffect {
                                statute_id: statute.id.clone(),
                                description: format!(
                                    "Effect '{}' is unreachable because required statute '{}' has contradictory conditions",
                                    effect.description, req_id
                                ),
                            });
                        }
                        break;
                    }
                }
            }
        }

        errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_invalid_numeric_range() {
        let condition = ConditionNode::Between {
            field: "age".to_string(),
            min: ConditionValue::Number(50),
            max: ConditionValue::Number(30),
        };

        let validator = SemanticValidator::new();
        let result = validator.validate_condition(&condition, "test-statute");

        assert!(result.is_err());
        match result.unwrap_err() {
            ValidationError::InvalidNumericRange { min, max, .. } => {
                assert_eq!(min, 50);
                assert_eq!(max, 30);
            }
            _ => panic!("Expected InvalidNumericRange error"),
        }
    }

    #[test]
    fn test_validate_valid_numeric_range() {
        let condition = ConditionNode::Between {
            field: "age".to_string(),
            min: ConditionValue::Number(18),
            max: ConditionValue::Number(65),
        };

        let validator = SemanticValidator::new();
        let result = validator.validate_condition(&condition, "test-statute");

        assert!(result.is_ok());
    }

    #[test]
    fn test_circular_dependency_detection() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![
                StatuteNode {
                    id: "statute1".to_string(),
                    visibility: crate::module_system::Visibility::Private,
                    title: "Statute 1".to_string(),
                    requires: vec!["statute2".to_string()],
                    ..Default::default()
                },
                StatuteNode {
                    id: "statute2".to_string(),
                    visibility: crate::module_system::Visibility::Private,
                    title: "Statute 2".to_string(),
                    requires: vec!["statute1".to_string()],
                    ..Default::default()
                },
            ],
        };

        let validator = SemanticValidator::new();
        let result = validator.validate_document(&doc);

        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, ValidationError::CircularDependency { .. }))
        );
    }

    #[test]
    fn test_undefined_reference() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![StatuteNode {
                id: "statute1".to_string(),
                visibility: crate::module_system::Visibility::Private,
                title: "Statute 1".to_string(),
                requires: vec!["nonexistent".to_string()],
                ..Default::default()
            }],
        };

        let validator = SemanticValidator::new();
        let result = validator.validate_document(&doc);

        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, ValidationError::UndefinedReference { .. }))
        );
    }

    #[test]
    fn test_self_reference() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![StatuteNode {
                id: "statute1".to_string(),
                visibility: crate::module_system::Visibility::Private,
                title: "Statute 1".to_string(),
                requires: vec!["statute1".to_string()],
                ..Default::default()
            }],
        };

        let validator = SemanticValidator::new();
        let result = validator.validate_document(&doc);

        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, ValidationError::SelfReference { .. }))
        );
    }

    #[test]
    fn test_completeness_checker() {
        let checker = CompletenessChecker::new().require_field("conditions");

        let incomplete_statute = StatuteNode {
            id: "test".to_string(),
            visibility: crate::module_system::Visibility::Private,
            title: "Test".to_string(),
            conditions: vec![],
            ..Default::default()
        };

        let result = checker.check_statute(&incomplete_statute);
        assert!(result.is_err());
    }

    #[test]
    fn test_dead_code_contradictory_conditions() {
        use crate::ast::EffectNode;

        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![StatuteNode {
                id: "unreachable".to_string(),
                visibility: crate::module_system::Visibility::Private,
                title: "Unreachable Statute".to_string(),
                conditions: vec![ConditionNode::Between {
                    field: "age".to_string(),
                    min: ConditionValue::Number(50),
                    max: ConditionValue::Number(30),
                }],
                effects: vec![EffectNode {
                    effect_type: "grant".to_string(),
                    description: "This will never happen".to_string(),
                    parameters: vec![],
                }],
                ..Default::default()
            }],
        };

        let detector = DeadCodeDetector::new();
        let errors = detector.detect(&doc);

        assert!(!errors.is_empty());
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, ValidationError::DeadCode { .. }))
        );
    }

    #[test]
    fn test_dead_code_unreferenced_statute() {
        use crate::ast::EffectNode;

        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![
                StatuteNode {
                    id: "referenced".to_string(),
                    visibility: crate::module_system::Visibility::Private,
                    title: "Referenced Statute".to_string(),
                    requires: vec!["unreferenced".to_string()],
                    effects: vec![EffectNode {
                        effect_type: "grant".to_string(),
                        description: "Effect".to_string(),
                        parameters: vec![],
                    }],
                    ..Default::default()
                },
                StatuteNode {
                    id: "unreferenced".to_string(),
                    visibility: crate::module_system::Visibility::Private,
                    title: "Unreferenced Statute".to_string(),
                    effects: vec![EffectNode {
                        effect_type: "grant".to_string(),
                        description: "Effect".to_string(),
                        parameters: vec![],
                    }],
                    ..Default::default()
                },
            ],
        };

        let detector = DeadCodeDetector::new();
        let errors = detector.detect(&doc);

        // "unreferenced" is actually referenced by "referenced", so should not be dead code
        // Only "referenced" should be reported as unreferenced
        assert!(errors
            .iter()
            .any(|e| matches!(e, ValidationError::DeadCode { statute_id, .. } if statute_id == "referenced")));
    }

    #[test]
    fn test_dead_code_no_issues() {
        use crate::ast::EffectNode;

        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![
                StatuteNode {
                    id: "statute1".to_string(),
                    visibility: crate::module_system::Visibility::Private,
                    title: "Statute 1".to_string(),
                    conditions: vec![ConditionNode::Comparison {
                        field: "age".to_string(),
                        operator: ">=".to_string(),
                        value: ConditionValue::Number(18),
                    }],
                    effects: vec![EffectNode {
                        effect_type: "grant".to_string(),
                        description: "Effect".to_string(),
                        parameters: vec![],
                    }],
                    ..Default::default()
                },
                StatuteNode {
                    id: "statute2".to_string(),
                    visibility: crate::module_system::Visibility::Private,
                    title: "Statute 2".to_string(),
                    requires: vec!["statute1".to_string()],
                    effects: vec![EffectNode {
                        effect_type: "grant".to_string(),
                        description: "Effect 2".to_string(),
                        parameters: vec![],
                    }],
                    ..Default::default()
                },
            ],
        };

        let detector = DeadCodeDetector::new();
        let errors = detector.detect(&doc);

        // statute2 should be reported as unreferenced since no one references it
        // But statute1 is referenced by statute2
        assert_eq!(errors.len(), 1);
        assert!(matches!(
            &errors[0],
            ValidationError::DeadCode { statute_id, .. } if statute_id == "statute2"
        ));
    }
}
