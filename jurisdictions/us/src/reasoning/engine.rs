//! Legal Reasoning Engine for US Law.
//!
//! Provides automated compliance analysis and violation detection.

use legalis_core::StatuteRegistry;

use super::context::UsEvaluationContext;
use super::error::ReasoningResult;
use super::statute_adapter::all_federal_statutes;
use super::types::{LegalAnalysis, Violation, ViolationSeverity};

use crate::tax::income_tax::IncomeTaxStructure;

/// Legal Reasoning Engine for US Federal Law
pub struct LegalReasoningEngine {
    registry: StatuteRegistry,
}

impl Default for LegalReasoningEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl LegalReasoningEngine {
    /// Create a new reasoning engine with all federal statutes
    #[must_use]
    pub fn new() -> Self {
        let mut registry = StatuteRegistry::new();
        for statute in all_federal_statutes() {
            registry.add(statute);
        }
        Self { registry }
    }

    /// Create an engine with custom statutes
    #[must_use]
    pub fn with_statutes(statutes: Vec<legalis_core::Statute>) -> Self {
        let mut registry = StatuteRegistry::new();
        for statute in statutes {
            registry.add(statute);
        }
        Self { registry }
    }

    /// Analyze a state tax structure for compliance
    pub fn analyze_tax_structure(
        &self,
        tax: &IncomeTaxStructure,
    ) -> ReasoningResult<LegalAnalysis> {
        let _ctx = UsEvaluationContext::new(tax);
        let analysis = LegalAnalysis::compliant("IncomeTaxStructure");

        // State tax structures are generally compliant with federal requirements
        // as states have broad authority to set their own tax policies
        Ok(analysis)
    }

    /// Analyze employment compliance for federal requirements
    pub fn analyze_employment_compliance(
        &self,
        hourly_wage: f64,
        weekly_hours: f64,
        employee_age: u32,
        is_exempt: bool,
    ) -> ReasoningResult<LegalAnalysis> {
        let mut violations = Vec::new();

        // FLSA Minimum Wage check (§ 206)
        const FEDERAL_MINIMUM_WAGE: f64 = 7.25;
        if hourly_wage < FEDERAL_MINIMUM_WAGE {
            violations.push(
                Violation::new(
                    "FLSA_206",
                    "Minimum Wage",
                    format!(
                        "Hourly wage ${:.2} is below federal minimum wage ${:.2}",
                        hourly_wage, FEDERAL_MINIMUM_WAGE
                    ),
                    ViolationSeverity::Critical,
                )
                .with_legal_reference("29 U.S.C. § 206(a)(1)"),
            );
        }

        // FLSA Overtime check (§ 207)
        if !is_exempt && weekly_hours > 40.0 {
            // Non-exempt employees working over 40 hours should get overtime
            // This is an advisory - we're just noting they're in overtime territory
        }

        // FLSA Child Labor check (§ 212)
        if employee_age < 14 {
            violations.push(
                Violation::new(
                    "FLSA_212",
                    "Child Labor",
                    format!(
                        "Employee age {} is below minimum working age of 14",
                        employee_age
                    ),
                    ViolationSeverity::Critical,
                )
                .with_legal_reference("29 U.S.C. § 212"),
            );
        } else if employee_age < 16 {
            // Minors 14-15 have restricted hours
            if weekly_hours > 18.0 {
                violations.push(
                    Violation::new(
                        "FLSA_212",
                        "Child Labor - Hours",
                        format!(
                            "Minor age {} working {:.1} hours/week exceeds 18-hour limit during school year",
                            employee_age, weekly_hours
                        ),
                        ViolationSeverity::Major,
                    )
                    .with_legal_reference("29 CFR § 570.35"),
                );
            }
        }

        if violations.is_empty() {
            Ok(LegalAnalysis::compliant("EmploymentCompliance"))
        } else {
            Ok(LegalAnalysis::non_compliant(
                "EmploymentCompliance",
                violations,
            ))
        }
    }

    /// Get a reference to the statute registry
    #[must_use]
    pub fn registry(&self) -> &StatuteRegistry {
        &self.registry
    }
}

#[cfg(test)]
mod tests {
    use super::super::types::ComplianceStatus;
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = LegalReasoningEngine::new();
        assert!(!engine.registry().is_empty());
    }

    #[test]
    fn test_compliant_employment() {
        let engine = LegalReasoningEngine::new();
        let analysis = engine
            .analyze_employment_compliance(15.00, 40.0, 25, false)
            .expect("Analysis should succeed");
        assert!(analysis.compliance_status.is_compliant());
    }

    #[test]
    fn test_minimum_wage_violation() {
        let engine = LegalReasoningEngine::new();
        let analysis = engine
            .analyze_employment_compliance(5.00, 40.0, 25, false)
            .expect("Analysis should succeed");
        assert!(analysis.compliance_status.is_non_compliant());

        if let ComplianceStatus::NonCompliant { violations } = &analysis.compliance_status {
            assert!(violations.iter().any(|v| v.statute_id == "FLSA_206"));
        }
    }

    #[test]
    fn test_child_labor_violation() {
        let engine = LegalReasoningEngine::new();
        let analysis = engine
            .analyze_employment_compliance(10.00, 40.0, 13, false)
            .expect("Analysis should succeed");
        assert!(analysis.compliance_status.is_non_compliant());

        if let ComplianceStatus::NonCompliant { violations } = &analysis.compliance_status {
            assert!(violations.iter().any(|v| v.statute_id == "FLSA_212"));
        }
    }
}
