//! Legal reasoning engine for French law.
//!
//! This module provides the core engine for automated legal analysis,
//! orchestrating statute evaluation and violation detection.

use legalis_core::{Condition, EvaluationContext, Statute, StatuteRegistry};

use super::error::{ReasoningError, ReasoningResult};
use super::statute_adapter::{
    all_french_statutes, company_law_statutes, contract_law_statutes, labor_law_statutes,
};
use super::types::{
    ComplianceStatus, EntityType, LegalAnalysis, LegalOpinion, ReasoningStep, RiskLevel, Violation,
    ViolationSeverity,
};

/// Core legal reasoning engine for French law analysis.
///
/// This engine orchestrates the evaluation of French legal statutes against
/// various entity types (contracts, employment contracts, articles of incorporation, etc.),
/// detecting violations and generating comprehensive legal analysis.
///
/// # Examples
///
/// ```rust,ignore
/// use legalis_fr::reasoning::LegalReasoningEngine;
/// use legalis_fr::contract::Contract;
///
/// let engine = LegalReasoningEngine::new();
/// let contract = Contract::new()
///     .with_consent(true)
///     .with_good_faith(true);
///
/// let analysis = engine.analyze_contract(&contract)?;
/// println!("Compliance: {:?}", analysis.compliance_status);
/// ```
pub struct LegalReasoningEngine {
    /// Registry containing all French law statutes
    registry: StatuteRegistry,

    /// Contract law statutes (Code civil)
    contract_statutes: Vec<Statute>,

    /// Labor law statutes (Code du travail)
    labor_statutes: Vec<Statute>,

    /// Company law statutes (Code de commerce)
    company_statutes: Vec<Statute>,
}

impl LegalReasoningEngine {
    /// Create a new legal reasoning engine.
    ///
    /// Initializes the statute registry with all French law statutes.
    #[must_use]
    pub fn new() -> Self {
        let contract_statutes = contract_law_statutes();
        let labor_statutes = labor_law_statutes();
        let company_statutes = company_law_statutes();

        let mut registry = StatuteRegistry::new();

        // Register all statutes
        for statute in &contract_statutes {
            registry.add(statute.clone());
        }
        for statute in &labor_statutes {
            registry.add(statute.clone());
        }
        for statute in &company_statutes {
            registry.add(statute.clone());
        }

        Self {
            registry,
            contract_statutes,
            labor_statutes,
            company_statutes,
        }
    }

    /// Get all registered statutes.
    ///
    /// Returns a vector containing all French law statutes in the registry.
    #[must_use]
    pub fn all_statutes(&self) -> Vec<Statute> {
        all_french_statutes()
    }

    /// Get the statute registry.
    #[must_use]
    pub const fn registry(&self) -> &StatuteRegistry {
        &self.registry
    }

    /// Evaluate a single statute against a context.
    ///
    /// # Arguments
    ///
    /// * `statute` - The statute to evaluate
    /// * `context` - The evaluation context providing entity attributes
    ///
    /// # Returns
    ///
    /// True if all preconditions are satisfied, false otherwise
    pub fn evaluate_statute(
        &self,
        statute: &Statute,
        context: &dyn EvaluationContext,
    ) -> ReasoningResult<bool> {
        let preconditions = &statute.preconditions;

        // If no preconditions, statute applies
        if preconditions.is_empty() {
            return Ok(true);
        }

        // Evaluate all preconditions
        for condition in preconditions {
            if !self.evaluate_condition(condition, context)? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Evaluate a single condition against a context.
    ///
    /// # Arguments
    ///
    /// * `condition` - The condition to evaluate
    /// * `context` - The evaluation context
    ///
    /// # Returns
    ///
    /// True if the condition is satisfied, false otherwise
    #[allow(clippy::only_used_in_recursion)]
    fn evaluate_condition(
        &self,
        condition: &Condition,
        context: &dyn EvaluationContext,
    ) -> ReasoningResult<bool> {
        match condition {
            Condition::AttributeEquals { key, value } => {
                let actual = context.get_attribute(key);
                Ok(actual.as_deref() == Some(value.as_str()))
            }
            Condition::And(left, right) => {
                let left_result = self.evaluate_condition(left, context)?;
                let right_result = self.evaluate_condition(right, context)?;
                Ok(left_result && right_result)
            }
            Condition::Or(left, right) => {
                let left_result = self.evaluate_condition(left, context)?;
                let right_result = self.evaluate_condition(right, context)?;
                Ok(left_result || right_result)
            }
            Condition::Age { operator, value } => {
                let actual_age =
                    context
                        .get_age()
                        .ok_or_else(|| ReasoningError::MissingContextData {
                            description: "Age information not available".to_string(),
                        })?;

                Ok(match operator {
                    legalis_core::ComparisonOp::GreaterThan => actual_age > *value,
                    legalis_core::ComparisonOp::GreaterOrEqual => actual_age >= *value,
                    legalis_core::ComparisonOp::LessThan => actual_age < *value,
                    legalis_core::ComparisonOp::LessOrEqual => actual_age <= *value,
                    legalis_core::ComparisonOp::Equal => actual_age == *value,
                    legalis_core::ComparisonOp::NotEqual => actual_age != *value,
                })
            }
            Condition::Threshold {
                attributes,
                operator,
                value,
            } => {
                // Calculate weighted sum of attributes
                let mut sum = 0.0;
                for (attr_name, weight) in attributes {
                    let attr_value = context
                        .get_attribute(attr_name)
                        .and_then(|v| v.parse::<f64>().ok())
                        .ok_or_else(|| ReasoningError::MissingContextData {
                            description: format!(
                                "Attribute '{}' not available or not numeric",
                                attr_name
                            ),
                        })?;
                    sum += attr_value * weight;
                }

                Ok(match operator {
                    legalis_core::ComparisonOp::GreaterThan => sum > *value,
                    legalis_core::ComparisonOp::GreaterOrEqual => sum >= *value,
                    legalis_core::ComparisonOp::LessThan => sum < *value,
                    legalis_core::ComparisonOp::LessOrEqual => sum <= *value,
                    legalis_core::ComparisonOp::Equal => (sum - value).abs() < f64::EPSILON,
                    legalis_core::ComparisonOp::NotEqual => (sum - value).abs() >= f64::EPSILON,
                })
            }
            Condition::Pattern {
                attribute,
                pattern,
                negated,
            } => {
                let matches = context
                    .get_attribute(attribute)
                    .is_some_and(|v| v.contains(pattern));
                Ok(if *negated { !matches } else { matches })
            }
            Condition::Duration {
                operator,
                value,
                unit,
            } => {
                let duration = context.get_duration(*unit).ok_or_else(|| {
                    ReasoningError::MissingContextData {
                        description: format!("Duration information ({:?}) not available", unit),
                    }
                })?;

                Ok(match operator {
                    legalis_core::ComparisonOp::GreaterThan => duration > *value,
                    legalis_core::ComparisonOp::GreaterOrEqual => duration >= *value,
                    legalis_core::ComparisonOp::LessThan => duration < *value,
                    legalis_core::ComparisonOp::LessOrEqual => duration <= *value,
                    legalis_core::ComparisonOp::Equal => duration == *value,
                    legalis_core::ComparisonOp::NotEqual => duration != *value,
                })
            }
            _ => Err(ReasoningError::ConditionEvaluationFailed {
                reason: format!("Unsupported condition type: {:?}", condition),
            }),
        }
    }

    /// Analyze contract law compliance.
    ///
    /// Evaluates a contract against French contract law statutes.
    ///
    /// # Arguments
    ///
    /// * `context` - Entity providing contract attributes via `EvaluationContext`
    ///
    /// # Returns
    ///
    /// Comprehensive legal analysis with violations, compliance status, and recommendations
    pub fn analyze_contract(
        &self,
        context: &dyn EvaluationContext,
    ) -> ReasoningResult<LegalAnalysis> {
        let mut analysis = LegalAnalysis::new(EntityType::Contract);

        // Evaluate all contract law statutes
        for statute in &self.contract_statutes {
            analysis.add_statute(statute.id.to_string());

            let satisfied = self.evaluate_statute(statute, context)?;

            // Add reasoning step
            let step = ReasoningStep::new(
                analysis.reasoning_chain.len() + 1,
                &statute.id,
                statute.title.clone(),
                satisfied,
                format!(
                    "Évaluation de {} - {}",
                    &statute.id,
                    if satisfied {
                        "conforme"
                    } else {
                        "non conforme"
                    }
                ),
                format!(
                    "Evaluation of {} - {}",
                    &statute.id,
                    if satisfied {
                        "compliant"
                    } else {
                        "non-compliant"
                    }
                ),
            );
            analysis.add_reasoning_step(step);

            // If not satisfied, create violation
            if !satisfied {
                let severity = self.determine_severity(statute);
                let violation = Violation::new(
                    &statute.id,
                    severity,
                    format!("Violation de {}", &statute.id),
                    format!("Violation of {}", &statute.id),
                )
                .with_titles(statute.title.clone(), statute.title.clone());

                analysis.add_violation(violation);
            }
        }

        // Determine overall compliance status
        analysis.compliance_status = self.determine_compliance_status(&analysis);

        // Generate legal opinion
        analysis.legal_opinion = self.generate_legal_opinion(&analysis);

        // Set confidence level
        analysis.confidence = self.calculate_confidence(&analysis);

        Ok(analysis)
    }

    /// Analyze employment contract compliance.
    ///
    /// Evaluates an employment contract against French labor law statutes.
    pub fn analyze_employment(
        &self,
        context: &dyn EvaluationContext,
    ) -> ReasoningResult<LegalAnalysis> {
        let mut analysis = LegalAnalysis::new(EntityType::EmploymentContract);

        // Evaluate all labor law statutes
        for statute in &self.labor_statutes {
            analysis.add_statute(statute.id.to_string());

            let satisfied = self.evaluate_statute(statute, context)?;

            let step = ReasoningStep::new(
                analysis.reasoning_chain.len() + 1,
                &statute.id,
                statute.title.clone(),
                satisfied,
                format!(
                    "Évaluation de {} - {}",
                    &statute.id,
                    if satisfied {
                        "conforme"
                    } else {
                        "non conforme"
                    }
                ),
                format!(
                    "Evaluation of {} - {}",
                    &statute.id,
                    if satisfied {
                        "compliant"
                    } else {
                        "non-compliant"
                    }
                ),
            );
            analysis.add_reasoning_step(step);

            if !satisfied {
                let severity = self.determine_severity(statute);
                let violation = Violation::new(
                    &statute.id,
                    severity,
                    format!("Violation de {}", &statute.id),
                    format!("Violation of {}", &statute.id),
                )
                .with_titles(statute.title.clone(), statute.title.clone());

                analysis.add_violation(violation);
            }
        }

        analysis.compliance_status = self.determine_compliance_status(&analysis);
        analysis.legal_opinion = self.generate_legal_opinion(&analysis);
        analysis.confidence = self.calculate_confidence(&analysis);

        Ok(analysis)
    }

    /// Analyze articles of incorporation compliance.
    ///
    /// Evaluates articles of incorporation against French company law statutes.
    pub fn analyze_articles(
        &self,
        context: &dyn EvaluationContext,
    ) -> ReasoningResult<LegalAnalysis> {
        let mut analysis = LegalAnalysis::new(EntityType::ArticlesOfIncorporation);

        // Evaluate all company law statutes
        for statute in &self.company_statutes {
            analysis.add_statute(statute.id.to_string());

            let satisfied = self.evaluate_statute(statute, context)?;

            let step = ReasoningStep::new(
                analysis.reasoning_chain.len() + 1,
                &statute.id,
                statute.title.clone(),
                satisfied,
                format!(
                    "Évaluation de {} - {}",
                    &statute.id,
                    if satisfied {
                        "conforme"
                    } else {
                        "non conforme"
                    }
                ),
                format!(
                    "Evaluation of {} - {}",
                    &statute.id,
                    if satisfied {
                        "compliant"
                    } else {
                        "non-compliant"
                    }
                ),
            );
            analysis.add_reasoning_step(step);

            if !satisfied {
                let severity = self.determine_severity(statute);
                let violation = Violation::new(
                    &statute.id,
                    severity,
                    format!("Violation de {}", &statute.id),
                    format!("Violation of {}", &statute.id),
                )
                .with_titles(statute.title.clone(), statute.title.clone());

                analysis.add_violation(violation);
            }
        }

        analysis.compliance_status = self.determine_compliance_status(&analysis);
        analysis.legal_opinion = self.generate_legal_opinion(&analysis);
        analysis.confidence = self.calculate_confidence(&analysis);

        Ok(analysis)
    }

    /// Determine violation severity based on statute characteristics.
    fn determine_severity(&self, statute: &Statute) -> ViolationSeverity {
        let statute_id = &statute.id;

        // Critical: Validity requirements (contract nullity)
        if statute_id.contains("1128") {
            return ViolationSeverity::Critical;
        }

        // High: Breach remedies, company formation
        if statute_id.contains("1231")
            || statute_id.contains("1217")
            || statute_id.contains("l225-1")
        {
            return ViolationSeverity::High;
        }

        // Medium: Working hours, CDD duration
        if statute_id.contains("l3121") || statute_id.contains("l1242") {
            return ViolationSeverity::Medium;
        }

        // Default to medium
        ViolationSeverity::Medium
    }

    /// Determine overall compliance status from violations.
    fn determine_compliance_status(&self, analysis: &LegalAnalysis) -> ComplianceStatus {
        if analysis.violations.is_empty() {
            return ComplianceStatus::Compliant;
        }

        let critical_count = analysis.critical_violation_count();
        let high_count = analysis
            .violations
            .iter()
            .filter(|v| v.severity == ViolationSeverity::High)
            .count();

        if critical_count > 0 {
            ComplianceStatus::Invalid
        } else if high_count > 0 {
            let issues: Vec<String> = analysis
                .violations
                .iter()
                .filter(|v| v.severity >= ViolationSeverity::High)
                .map(|v| v.article_id.clone())
                .collect();
            ComplianceStatus::MajorViolations(issues)
        } else {
            let issues: Vec<String> = analysis
                .violations
                .iter()
                .map(|v| v.article_id.clone())
                .collect();
            ComplianceStatus::MinorIssues(issues)
        }
    }

    /// Generate legal opinion with recommendations.
    fn generate_legal_opinion(&self, analysis: &LegalAnalysis) -> LegalOpinion {
        let risk_level = match &analysis.compliance_status {
            ComplianceStatus::Compliant => RiskLevel::Low,
            ComplianceStatus::MinorIssues(_) => RiskLevel::Medium,
            ComplianceStatus::MajorViolations(_) => RiskLevel::High,
            ComplianceStatus::Invalid => RiskLevel::Critical,
        };

        let (summary_fr, summary_en) = match &analysis.compliance_status {
            ComplianceStatus::Compliant => (
                format!(
                    "{} est conforme au droit français",
                    analysis.entity_type.french_name()
                ),
                format!(
                    "{} is compliant with French law",
                    analysis.entity_type.english_name()
                ),
            ),
            ComplianceStatus::MinorIssues(issues) => (
                format!(
                    "{} présente {} problème(s) mineur(s)",
                    analysis.entity_type.french_name(),
                    issues.len()
                ),
                format!(
                    "{} has {} minor issue(s)",
                    analysis.entity_type.english_name(),
                    issues.len()
                ),
            ),
            ComplianceStatus::MajorViolations(violations) => (
                format!(
                    "{} présente {} violation(s) majeure(s)",
                    analysis.entity_type.french_name(),
                    violations.len()
                ),
                format!(
                    "{} has {} major violation(s)",
                    analysis.entity_type.english_name(),
                    violations.len()
                ),
            ),
            ComplianceStatus::Invalid => (
                format!("{} est invalide", analysis.entity_type.french_name()),
                format!("{} is invalid", analysis.entity_type.english_name()),
            ),
        };

        let mut opinion = LegalOpinion::new(summary_fr, summary_en, risk_level);

        // Add recommendations based on violations
        if !analysis.violations.is_empty() {
            for violation in &analysis.violations {
                opinion.add_recommendation(
                    format!("Corriger la violation de {}", violation.article_id),
                    format!("Correct the violation of {}", violation.article_id),
                );
            }
        }

        opinion
    }

    /// Calculate confidence level for the analysis.
    fn calculate_confidence(&self, analysis: &LegalAnalysis) -> f64 {
        // High confidence if we have reasoning steps for all applicable statutes
        let statute_count = analysis.applicable_statutes.len();
        let reasoning_count = analysis.reasoning_chain.len();

        if statute_count == 0 {
            return 0.0;
        }

        let base_confidence = reasoning_count as f64 / statute_count as f64;

        // Reduce confidence if there are many violations
        let violation_penalty = (analysis.violations.len() as f64 * 0.05).min(0.3);

        (base_confidence - violation_penalty).clamp(0.0, 1.0)
    }
}

impl Default for LegalReasoningEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    /// Mock evaluation context for testing
    struct MockContext {
        attributes: HashMap<String, String>,
        age: Option<u32>,
    }

    impl MockContext {
        fn new() -> Self {
            Self {
                attributes: HashMap::new(),
                age: None,
            }
        }

        fn with_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
            self.attributes.insert(key.into(), value.into());
            self
        }

        fn with_age(mut self, age: u32) -> Self {
            self.age = Some(age);
            self
        }
    }

    impl EvaluationContext for MockContext {
        fn get_attribute(&self, key: &str) -> Option<String> {
            self.attributes.get(key).cloned()
        }

        fn get_age(&self) -> Option<u32> {
            self.age
        }

        fn get_income(&self) -> Option<u64> {
            None
        }

        fn get_current_date(&self) -> Option<chrono::NaiveDate> {
            None
        }

        fn check_geographic(
            &self,
            _region_type: legalis_core::RegionType,
            _region_id: &str,
        ) -> bool {
            false
        }

        fn check_relationship(
            &self,
            _relationship_type: legalis_core::RelationshipType,
            _target_id: Option<&str>,
        ) -> bool {
            false
        }

        fn get_residency_months(&self) -> Option<u32> {
            None
        }

        fn get_duration(&self, _unit: legalis_core::DurationUnit) -> Option<u32> {
            None
        }

        fn get_percentage(&self, _context: &str) -> Option<u32> {
            None
        }

        fn evaluate_formula(&self, _formula: &str) -> Option<f64> {
            None
        }
    }

    #[test]
    fn test_engine_creation() {
        let engine = LegalReasoningEngine::new();
        assert!(!engine.contract_statutes.is_empty());
        assert!(!engine.labor_statutes.is_empty());
        assert!(!engine.company_statutes.is_empty());
    }

    #[test]
    fn test_all_statutes() {
        let engine = LegalReasoningEngine::new();
        let statutes = engine.all_statutes();
        assert!(statutes.len() >= 6); // At least the 6 we've implemented
    }

    #[test]
    fn test_evaluate_attribute_equals() {
        let engine = LegalReasoningEngine::new();
        let context = MockContext::new().with_attribute("consent_given", "true");

        let condition = Condition::AttributeEquals {
            key: "consent_given".to_string(),
            value: "true".to_string(),
        };

        let result = engine.evaluate_condition(&condition, &context).unwrap();
        assert!(result);
    }

    #[test]
    fn test_evaluate_age_condition() {
        let engine = LegalReasoningEngine::new();
        let context = MockContext::new().with_age(25);

        let condition = Condition::Age {
            operator: legalis_core::ComparisonOp::GreaterOrEqual,
            value: 18,
        };

        let result = engine.evaluate_condition(&condition, &context).unwrap();
        assert!(result);
    }

    #[test]
    fn test_evaluate_and_condition() {
        let engine = LegalReasoningEngine::new();
        let context = MockContext::new()
            .with_attribute("consent_given", "true")
            .with_attribute("good_faith", "true");

        let condition = Condition::And(
            Box::new(Condition::AttributeEquals {
                key: "consent_given".to_string(),
                value: "true".to_string(),
            }),
            Box::new(Condition::AttributeEquals {
                key: "good_faith".to_string(),
                value: "true".to_string(),
            }),
        );

        let result = engine.evaluate_condition(&condition, &context).unwrap();
        assert!(result);
    }

    #[test]
    fn test_analyze_contract_compliant() {
        let engine = LegalReasoningEngine::new();
        let context = MockContext::new()
            .with_attribute("consent_given", "true")
            .with_attribute("not_under_guardianship", "true")
            .with_attribute("content_lawful", "true")
            .with_attribute("content_certain", "true")
            .with_attribute("non_performance", "false")
            .with_attribute("good_faith", "true")
            .with_age(25);

        let analysis = engine.analyze_contract(&context).unwrap();
        assert_eq!(analysis.entity_type, EntityType::Contract);
        assert!(!analysis.applicable_statutes.is_empty());
    }

    #[test]
    fn test_determine_severity() {
        let engine = LegalReasoningEngine::new();

        // Get Article 1128 (validity)
        let statute_1128 = engine
            .contract_statutes
            .iter()
            .find(|s| s.id.contains("1128"))
            .unwrap();

        let severity = engine.determine_severity(statute_1128);
        assert_eq!(severity, ViolationSeverity::Critical);
    }

    #[test]
    fn test_calculate_confidence() {
        let engine = LegalReasoningEngine::new();
        let mut analysis = LegalAnalysis::new(EntityType::Contract);

        analysis.add_statute("statute1");
        analysis.add_statute("statute2");

        // Add reasoning steps
        analysis.add_reasoning_step(ReasoningStep::new(
            1, "statute1", "Test", true, "Test", "Test",
        ));
        analysis.add_reasoning_step(ReasoningStep::new(
            2, "statute2", "Test", true, "Test", "Test",
        ));

        let confidence = engine.calculate_confidence(&analysis);
        assert!(confidence > 0.9); // High confidence with all statutes evaluated
    }
}
