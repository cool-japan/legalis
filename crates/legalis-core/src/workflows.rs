//! Workflow helpers for common legal processes.
//!
//! This module provides pre-built workflows and helpers for common legal
//! operations, making it easier to implement standard legal processes.
//!
//! # Common Workflows
//!
//! - **Eligibility Checking**: Determine if an entity qualifies for a benefit
//! - **Compliance Verification**: Check if actions comply with regulations
//! - **Decision Trees**: Navigate complex legal decision pathways
//! - **Application Processing**: Handle multi-step application workflows
//!
//! # Examples
//!
//! ```
//! use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
//! use legalis_core::workflows::{EligibilityChecker, WorkflowContext};
//!
//! let statute = Statute::new(
//!     "benefit-1",
//!     "Low Income Benefit",
//!     Effect::new(EffectType::Grant, "Monthly benefit of $500")
//! )
//! .with_precondition(Condition::Age {
//!     operator: ComparisonOp::GreaterOrEqual,
//!     value: 18
//! })
//! .with_precondition(Condition::Income {
//!     operator: ComparisonOp::LessThan,
//!     value: 30000
//! });
//!
//! let mut context = WorkflowContext::new();
//! context.set_age(25);
//! context.set_income(25000);
//!
//! let checker = EligibilityChecker::new(vec![statute]);
//! let results = checker.check_eligibility(&context);
//!
//! assert_eq!(results.len(), 1);
//! assert!(results[0].is_eligible);
//! ```

use crate::{BasicEntity, Condition, Effect, EffectType, EvaluationContext, LegalEntity, Statute};
use chrono::NaiveDate;
use std::collections::HashMap;

/// Context for workflow execution.
///
/// Stores all the information needed to evaluate legal conditions
/// and make determinations during a workflow.
#[derive(Debug, Clone)]
pub struct WorkflowContext {
    entity: BasicEntity,
    metadata: HashMap<String, String>,
}

impl WorkflowContext {
    /// Create a new workflow context.
    pub fn new() -> Self {
        Self {
            entity: BasicEntity::new(),
            metadata: HashMap::new(),
        }
    }

    /// Create a context from an entity.
    pub fn from_entity(entity: BasicEntity) -> Self {
        Self {
            entity,
            metadata: HashMap::new(),
        }
    }

    /// Set age attribute.
    pub fn set_age(&mut self, age: u32) {
        self.entity.set_attribute("age", age.to_string());
    }

    /// Set income attribute.
    pub fn set_income(&mut self, income: u64) {
        self.entity.set_attribute("income", income.to_string());
    }

    /// Set a custom attribute.
    pub fn set_attribute(&mut self, key: &str, value: impl ToString) {
        self.entity.set_attribute(key, value.to_string());
    }

    /// Get an attribute value.
    pub fn get_attribute(&self, key: &str) -> Option<String> {
        self.entity.get_attribute(key).map(|s| s.to_string())
    }

    /// Set metadata (workflow-specific data).
    pub fn set_metadata(&mut self, key: impl ToString, value: impl ToString) {
        self.metadata.insert(key.to_string(), value.to_string());
    }

    /// Get metadata value.
    pub fn get_metadata(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(|s| s.as_str())
    }

    /// Get the underlying entity.
    pub fn entity(&self) -> &BasicEntity {
        &self.entity
    }

    /// Get mutable reference to the underlying entity.
    pub fn entity_mut(&mut self) -> &mut BasicEntity {
        &mut self.entity
    }
}

impl Default for WorkflowContext {
    fn default() -> Self {
        Self::new()
    }
}

impl EvaluationContext for WorkflowContext {
    fn get_attribute(&self, key: &str) -> Option<String> {
        self.entity.get_attribute(key).map(|s| s.to_string())
    }

    fn get_age(&self) -> Option<u32> {
        self.get_attribute("age")?.parse().ok()
    }

    fn get_income(&self) -> Option<u64> {
        self.get_attribute("income")?.parse().ok()
    }

    fn get_current_date(&self) -> Option<NaiveDate> {
        None
    }

    fn check_geographic(&self, _region_type: crate::RegionType, _region_id: &str) -> bool {
        false
    }

    fn check_relationship(
        &self,
        _relationship_type: crate::RelationshipType,
        _target_id: Option<&str>,
    ) -> bool {
        false
    }

    fn get_residency_months(&self) -> Option<u32> {
        None
    }

    fn get_duration(&self, _unit: crate::DurationUnit) -> Option<u32> {
        None
    }

    fn get_percentage(&self, _context: &str) -> Option<u32> {
        None
    }

    fn evaluate_formula(&self, _formula: &str) -> Option<f64> {
        None
    }
}

/// Result of an eligibility check.
#[derive(Debug, Clone, PartialEq)]
pub struct EligibilityResult {
    /// The statute that was checked.
    pub statute_id: String,
    /// Whether the entity is eligible.
    pub is_eligible: bool,
    /// The effect that would apply if eligible.
    pub effect: Option<Effect>,
    /// Reason for ineligibility (if not eligible).
    pub reason: Option<String>,
    /// Missing attributes that prevented determination.
    pub missing_attributes: Vec<String>,
}

/// Eligibility checker for benefits and programs.
///
/// Checks whether an entity meets the requirements for one or more
/// legal benefits or programs.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
/// use legalis_core::workflows::{EligibilityChecker, WorkflowContext};
///
/// let statute = Statute::new(
///     "senior-benefit",
///     "Senior Citizen Benefit",
///     Effect::new(EffectType::Grant, "Free transit pass")
/// )
/// .with_precondition(Condition::Age {
///     operator: ComparisonOp::GreaterOrEqual,
///     value: 65
/// });
///
/// let mut context = WorkflowContext::new();
/// context.set_age(70);
///
/// let checker = EligibilityChecker::new(vec![statute]);
/// let results = checker.check_eligibility(&context);
///
/// assert_eq!(results[0].is_eligible, true);
/// ```
pub struct EligibilityChecker {
    statutes: Vec<Statute>,
}

impl EligibilityChecker {
    /// Create a new eligibility checker with the given statutes.
    pub fn new(statutes: Vec<Statute>) -> Self {
        Self { statutes }
    }

    /// Check eligibility for all statutes.
    pub fn check_eligibility(&self, context: &WorkflowContext) -> Vec<EligibilityResult> {
        self.statutes
            .iter()
            .map(|statute| self.check_statute(statute, context))
            .collect()
    }

    /// Check eligibility for a single statute.
    pub fn check_statute(&self, statute: &Statute, context: &WorkflowContext) -> EligibilityResult {
        let missing = Vec::new();
        let mut is_eligible = true;
        let mut reason = None;

        // Convert to AttributeBasedContext
        let mut attributes = HashMap::new();
        if let Some(age) = context.get_age() {
            attributes.insert("age".to_string(), age.to_string());
        }
        if let Some(income) = context.get_income() {
            attributes.insert("income".to_string(), income.to_string());
        }
        let attr_context = crate::AttributeBasedContext::new(attributes);

        // Check each precondition
        for condition in &statute.preconditions {
            match condition.evaluate_simple(&attr_context) {
                Ok(result) => {
                    if !result {
                        is_eligible = false;
                        reason = Some(format!("Condition not met: {}", condition));
                        break;
                    }
                }
                Err(e) => {
                    is_eligible = false;
                    reason = Some(format!("Error evaluating condition: {}", e));
                    break;
                }
            }
        }

        EligibilityResult {
            statute_id: statute.id.clone(),
            is_eligible,
            effect: if is_eligible {
                Some(statute.effect.clone())
            } else {
                None
            },
            reason,
            missing_attributes: missing,
        }
    }

    /// Get all statutes this checker evaluates.
    pub fn statutes(&self) -> &[Statute] {
        &self.statutes
    }

    /// Filter results to only eligible statutes.
    pub fn filter_eligible(&self, results: Vec<EligibilityResult>) -> Vec<EligibilityResult> {
        results.into_iter().filter(|r| r.is_eligible).collect()
    }
}

/// Compliance verification result.
#[derive(Debug, Clone, PartialEq)]
pub struct ComplianceResult {
    /// Whether the action is compliant.
    pub is_compliant: bool,
    /// Violated statutes (if not compliant).
    pub violations: Vec<String>,
    /// Warnings (non-critical issues).
    pub warnings: Vec<String>,
}

/// Compliance verifier for regulatory requirements.
///
/// Checks whether a proposed action complies with applicable statutes.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
/// use legalis_core::workflows::{ComplianceVerifier, WorkflowContext};
///
/// let prohibition = Statute::new(
///     "age-restriction",
///     "Alcohol Purchase Restriction",
///     Effect::new(EffectType::Prohibition, "Cannot purchase alcohol")
/// )
/// .with_precondition(Condition::Age {
///     operator: ComparisonOp::LessThan,
///     value: 21
/// });
///
/// let mut context = WorkflowContext::new();
/// context.set_age(18);
///
/// let verifier = ComplianceVerifier::new(vec![prohibition]);
/// let result = verifier.verify_action("purchase_alcohol", &context);
///
/// assert!(!result.is_compliant);
/// assert!(!result.violations.is_empty());
/// ```
pub struct ComplianceVerifier {
    statutes: Vec<Statute>,
}

impl ComplianceVerifier {
    /// Create a new compliance verifier.
    pub fn new(statutes: Vec<Statute>) -> Self {
        Self { statutes }
    }

    /// Verify if an action is compliant.
    pub fn verify_action(&self, _action: &str, context: &WorkflowContext) -> ComplianceResult {
        let mut violations = Vec::new();
        let mut warnings = Vec::new();

        // Convert to AttributeBasedContext
        let mut attributes = HashMap::new();
        if let Some(age) = context.get_age() {
            attributes.insert("age".to_string(), age.to_string());
        }
        if let Some(income) = context.get_income() {
            attributes.insert("income".to_string(), income.to_string());
        }
        let attr_context = crate::AttributeBasedContext::new(attributes);

        for statute in &self.statutes {
            // Check if statute applies (preconditions are met)
            let mut all_conditions_met = true;
            for condition in &statute.preconditions {
                match condition.evaluate_simple(&attr_context) {
                    Ok(false) => {
                        all_conditions_met = false;
                        break;
                    }
                    Ok(true) => continue,
                    Err(_) => {
                        all_conditions_met = false;
                        break;
                    }
                }
            }

            if all_conditions_met {
                match statute.effect.effect_type {
                    EffectType::Prohibition => {
                        violations.push(format!("{}: {}", statute.id, statute.title));
                    }
                    EffectType::Obligation => {
                        warnings.push(format!("Required: {}", statute.title));
                    }
                    _ => {}
                }
            }
        }

        ComplianceResult {
            is_compliant: violations.is_empty(),
            violations,
            warnings,
        }
    }

    /// Get all statutes this verifier checks.
    pub fn statutes(&self) -> &[Statute] {
        &self.statutes
    }
}

/// Decision tree node for multi-step legal decisions.
#[derive(Debug, Clone)]
pub enum DecisionNode {
    /// A condition to evaluate.
    Condition {
        condition: Condition,
        on_true: Box<DecisionNode>,
        on_false: Box<DecisionNode>,
    },
    /// A final decision/outcome.
    Outcome {
        description: String,
        effect: Option<EffectType>,
    },
}

impl DecisionNode {
    /// Create a condition node.
    pub fn condition(condition: Condition, on_true: DecisionNode, on_false: DecisionNode) -> Self {
        Self::Condition {
            condition,
            on_true: Box::new(on_true),
            on_false: Box::new(on_false),
        }
    }

    /// Create an outcome node.
    pub fn outcome(description: impl ToString, effect: Option<EffectType>) -> Self {
        Self::Outcome {
            description: description.to_string(),
            effect,
        }
    }

    /// Evaluate the decision tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Condition, ComparisonOp, EffectType};
    /// use legalis_core::workflows::{DecisionNode, WorkflowContext};
    ///
    /// let tree = DecisionNode::condition(
    ///     Condition::Age {
    ///         operator: ComparisonOp::GreaterOrEqual,
    ///         value: 18
    ///     },
    ///     DecisionNode::outcome("Adult", Some(EffectType::Grant)),
    ///     DecisionNode::outcome("Minor", None)
    /// );
    ///
    /// let mut context = WorkflowContext::new();
    /// context.set_age(20);
    ///
    /// let outcome = tree.evaluate(&context).unwrap();
    /// assert_eq!(outcome, "Adult");
    /// ```
    pub fn evaluate(&self, context: &WorkflowContext) -> Result<String, String> {
        match self {
            DecisionNode::Condition {
                condition,
                on_true,
                on_false,
            } => {
                // Convert to AttributeBasedContext
                let mut attributes = HashMap::new();
                if let Some(age) = context.get_age() {
                    attributes.insert("age".to_string(), age.to_string());
                }
                if let Some(income) = context.get_income() {
                    attributes.insert("income".to_string(), income.to_string());
                }
                let attr_context = crate::AttributeBasedContext::new(attributes);

                let result = condition
                    .evaluate_simple(&attr_context)
                    .map_err(|e| e.to_string())?;
                if result {
                    on_true.evaluate(context)
                } else {
                    on_false.evaluate(context)
                }
            }
            DecisionNode::Outcome { description, .. } => Ok(description.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ComparisonOp;

    #[test]
    fn test_workflow_context_basic() {
        let mut context = WorkflowContext::new();
        context.set_age(25);
        context.set_income(50000);

        assert_eq!(context.get_age().unwrap(), 25);
        assert_eq!(context.get_income().unwrap(), 50000);
    }

    #[test]
    fn test_eligibility_checker_eligible() {
        let statute = Statute::new(
            "test-benefit",
            "Test Benefit",
            Effect::new(EffectType::Grant, "Benefit granted"),
        )
        .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 18))
        .with_precondition(Condition::income(ComparisonOp::LessThan, 30000));

        let mut context = WorkflowContext::new();
        context.set_age(25);
        context.set_income(25000);

        let checker = EligibilityChecker::new(vec![statute]);
        let results = checker.check_eligibility(&context);

        assert_eq!(results.len(), 1);
        assert!(results[0].is_eligible);
    }

    #[test]
    fn test_eligibility_checker_not_eligible() {
        let statute = Statute::new(
            "test-benefit",
            "Test Benefit",
            Effect::new(EffectType::Grant, "Benefit granted"),
        )
        .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 65));

        let mut context = WorkflowContext::new();
        context.set_age(25);

        let checker = EligibilityChecker::new(vec![statute]);
        let results = checker.check_eligibility(&context);

        assert_eq!(results.len(), 1);
        assert!(!results[0].is_eligible);
    }

    #[test]
    fn test_compliance_verifier_compliant() {
        let statute = Statute::new(
            "test-prohibition",
            "Test Prohibition",
            Effect::new(EffectType::Prohibition, "Action prohibited"),
        )
        .with_precondition(Condition::age(ComparisonOp::LessThan, 18));

        let mut context = WorkflowContext::new();
        context.set_age(25); // Above threshold, so prohibition doesn't apply

        let verifier = ComplianceVerifier::new(vec![statute]);
        let result = verifier.verify_action("test_action", &context);

        assert!(result.is_compliant);
        assert!(result.violations.is_empty());
    }

    #[test]
    fn test_compliance_verifier_violation() {
        let statute = Statute::new(
            "test-prohibition",
            "Test Prohibition",
            Effect::new(EffectType::Prohibition, "Action prohibited"),
        )
        .with_precondition(Condition::age(ComparisonOp::LessThan, 21));

        let mut context = WorkflowContext::new();
        context.set_age(18); // Below threshold, prohibition applies

        let verifier = ComplianceVerifier::new(vec![statute]);
        let result = verifier.verify_action("test_action", &context);

        assert!(!result.is_compliant);
        assert_eq!(result.violations.len(), 1);
    }

    #[test]
    fn test_decision_tree() {
        let tree = DecisionNode::condition(
            Condition::age(ComparisonOp::GreaterOrEqual, 18),
            DecisionNode::outcome("Adult", Some(EffectType::Grant)),
            DecisionNode::outcome("Minor", None),
        );

        let mut context = WorkflowContext::new();
        context.set_age(20);

        let outcome = tree.evaluate(&context).unwrap();
        assert_eq!(outcome, "Adult");
    }
}
