//! Legal reasoning engine for Singapore law.
//!
//! This module provides the main reasoning engine that evaluates entities
//! against Singapore statutes using `legalis-core`.

use legalis_core::{EvaluationContext, Statute, StatuteRegistry};

use super::context::SingaporeEvaluationContext;
use super::error::ReasoningResult;
use super::statute_adapter::{
    all_singapore_statutes, companies_act_statutes, employment_act_statutes,
};
use super::types::{
    ComplianceStatus, LegalAnalysis, ReasoningStep, Remedy, RemedyType, RiskLevel, Violation,
    ViolationSeverity,
};
use crate::employment::types::EmploymentContract;

/// Legal reasoning engine for Singapore law
pub struct LegalReasoningEngine {
    registry: StatuteRegistry,
}

impl Default for LegalReasoningEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl LegalReasoningEngine {
    /// Creates a new reasoning engine with all Singapore statutes loaded
    #[must_use]
    pub fn new() -> Self {
        let mut registry = StatuteRegistry::new();
        for statute in all_singapore_statutes() {
            registry.add(statute);
        }
        Self { registry }
    }

    /// Creates a reasoning engine with only employment law statutes
    #[must_use]
    pub fn employment_only() -> Self {
        let mut registry = StatuteRegistry::new();
        for statute in employment_act_statutes() {
            registry.add(statute);
        }
        Self { registry }
    }

    /// Creates a reasoning engine with only company law statutes
    #[must_use]
    pub fn companies_only() -> Self {
        let mut registry = StatuteRegistry::new();
        for statute in companies_act_statutes() {
            registry.add(statute);
        }
        Self { registry }
    }

    /// Gets the statute registry
    #[must_use]
    pub fn registry(&self) -> &StatuteRegistry {
        &self.registry
    }

    /// Analyzes an employment contract against Employment Act provisions
    pub fn analyze_employment_contract(
        &self,
        contract: &EmploymentContract,
    ) -> ReasoningResult<LegalAnalysis> {
        let ctx = SingaporeEvaluationContext::new(contract);
        let mut analysis = LegalAnalysis::compliant("EmploymentContract");
        let mut violations = Vec::new();
        let mut step_num = 0u32;

        // Check Employment Act coverage
        let covered_by_ea = contract.covered_by_ea;

        // Evaluate each applicable statute
        for statute in self.registry.iter() {
            if !statute.id.starts_with("EA_") {
                continue;
            }

            analysis.applicable_statutes.push(statute.id.clone());
            step_num += 1;

            // If not covered by EA, skip EA-specific checks
            if statute.id != "EA_s10_11" && !covered_by_ea {
                analysis.reasoning_steps.push(ReasoningStep {
                    step: step_num,
                    statute_id: statute.id.clone(),
                    condition_description: "Employment Act coverage check".to_string(),
                    result: true,
                    explanation: "Contract not covered by EA (salary above threshold), statute not applicable".to_string(),
                });
                continue;
            }

            // Evaluate statute preconditions
            // A failed precondition means the statute is not applicable, not a violation
            let (result, explanation) = self.evaluate_statute(&ctx, statute);

            analysis.reasoning_steps.push(ReasoningStep {
                step: step_num,
                statute_id: statute.id.clone(),
                condition_description: statute.title.clone(),
                result,
                explanation: explanation.clone(),
            });

            // Note: Precondition failures indicate non-applicability, not violations
            // Violations are detected separately via check_working_hours_compliance
        }

        // Check working hours compliance
        if covered_by_ea {
            let hours_violation = self.check_working_hours_compliance(contract);
            if let Some(v) = hours_violation {
                violations.push(v);
            }
        }

        // Update analysis with violations
        if violations.is_empty() {
            analysis.compliance_status = ComplianceStatus::Compliant;
            analysis.risk_level = RiskLevel::None;
        } else {
            analysis.risk_level = violations
                .iter()
                .map(|v| match v.severity {
                    ViolationSeverity::Critical => RiskLevel::Critical,
                    ViolationSeverity::Major => RiskLevel::High,
                    ViolationSeverity::Minor => RiskLevel::Medium,
                    ViolationSeverity::Advisory => RiskLevel::Low,
                })
                .max()
                .unwrap_or(RiskLevel::None);

            analysis.compliance_status = ComplianceStatus::NonCompliant { violations };
        }

        Ok(analysis)
    }

    /// Evaluates a statute against a context
    fn evaluate_statute<C: EvaluationContext>(&self, ctx: &C, statute: &Statute) -> (bool, String) {
        // Check all preconditions
        for precondition in &statute.preconditions {
            match precondition.evaluate(ctx) {
                Ok(true) => continue,
                Ok(false) => {
                    return (
                        false,
                        format!("Precondition failed for {}: {:?}", statute.id, precondition),
                    );
                }
                Err(e) => {
                    return (
                        true, // Assume compliant if we can't evaluate
                        format!("Could not evaluate precondition: {:?}", e),
                    );
                }
            }
        }

        (
            true,
            format!("All preconditions satisfied for {}", statute.id),
        )
    }

    /// Check working hours compliance
    fn check_working_hours_compliance(&self, contract: &EmploymentContract) -> Option<Violation> {
        let max_hours = if contract.working_hours.is_shift_work {
            48.0
        } else {
            44.0
        };

        if contract.working_hours.hours_per_week > max_hours {
            Some(Violation {
                statute_id: "EA_s38_1".to_string(),
                description: format!(
                    "Working hours ({:.1} hrs/week) exceed maximum ({:.1} hrs/week)",
                    contract.working_hours.hours_per_week, max_hours
                ),
                severity: ViolationSeverity::Major,
                remedies: vec![
                    Remedy {
                        remedy_type: RemedyType::Corrective,
                        description: "Reduce weekly working hours to statutory maximum".to_string(),
                        effort: Some("Immediate".to_string()),
                    },
                    Remedy {
                        remedy_type: RemedyType::Financial,
                        description: "Pay overtime at 1.5x rate for hours exceeding limit"
                            .to_string(),
                        effort: Some("Per payroll".to_string()),
                    },
                ],
                legal_reference: "Employment Act s. 38(1)".to_string(),
            })
        } else {
            None
        }
    }

    /// Creates a violation from a statute
    #[allow(dead_code)]
    fn create_violation(&self, statute: &Statute, explanation: &str) -> Violation {
        Violation {
            statute_id: statute.id.clone(),
            description: explanation.to_string(),
            severity: ViolationSeverity::Major,
            remedies: vec![Remedy {
                remedy_type: RemedyType::Corrective,
                description: format!("Address compliance issue with {}", statute.title),
                effort: None,
            }],
            legal_reference: statute.title.clone(),
        }
    }

    /// Get statute by ID
    pub fn get_statute(&self, id: &str) -> Option<&Statute> {
        self.registry.get(id)
    }

    /// Get all statute IDs
    pub fn statute_ids(&self) -> Vec<&str> {
        self.registry.iter().map(|s| s.id.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::employment::types::{ContractType, LeaveEntitlement, WorkingHours};
    use chrono::Utc;

    fn sample_contract(covered_by_ea: bool, hours_per_week: f64) -> EmploymentContract {
        EmploymentContract {
            employee_name: "John Tan".to_string(),
            employer_name: "Tech Innovations Pte Ltd".to_string(),
            contract_type: ContractType::Indefinite,
            start_date: Utc::now(),
            end_date: None,
            basic_salary_cents: if covered_by_ea { 250_000 } else { 500_000 },
            allowances: vec![],
            working_hours: WorkingHours {
                hours_per_day: hours_per_week / 5.0,
                hours_per_week,
                is_shift_work: false,
                rest_days_per_week: 2,
                overtime_eligible: covered_by_ea,
                working_days_per_week: 5,
            },
            leave_entitlement: LeaveEntitlement::new(0),
            cpf_applicable: true,
            covered_by_ea,
        }
    }

    #[test]
    fn test_engine_creation() {
        let engine = LegalReasoningEngine::new();
        assert!(!engine.statute_ids().is_empty());
    }

    #[test]
    fn test_compliant_contract() {
        let engine = LegalReasoningEngine::new();
        let contract = sample_contract(true, 44.0);

        let analysis = engine
            .analyze_employment_contract(&contract)
            .expect("Analysis should succeed");
        assert!(analysis.compliance_status.is_compliant());
    }

    #[test]
    fn test_excessive_hours_violation() {
        let engine = LegalReasoningEngine::new();
        let contract = sample_contract(true, 50.0); // Exceeds 44 hours

        let analysis = engine
            .analyze_employment_contract(&contract)
            .expect("Analysis should succeed");
        assert!(analysis.compliance_status.is_non_compliant());

        if let ComplianceStatus::NonCompliant { violations } = &analysis.compliance_status {
            assert!(!violations.is_empty());
            assert!(violations.iter().any(|v| v.statute_id == "EA_s38_1"));
        }
    }

    #[test]
    fn test_non_ea_covered_contract() {
        let engine = LegalReasoningEngine::new();
        let contract = sample_contract(false, 50.0); // High salary, not covered by EA

        let analysis = engine
            .analyze_employment_contract(&contract)
            .expect("Analysis should succeed");
        // Should be compliant as EA doesn't apply
        assert!(analysis.compliance_status.is_compliant());
    }

    #[test]
    fn test_get_statute() {
        let engine = LegalReasoningEngine::new();
        let statute = engine.get_statute("EA_s38_1");
        assert!(statute.is_some());
        assert_eq!(statute.map(|s| &s.id), Some(&"EA_s38_1".to_string()));
    }
}
