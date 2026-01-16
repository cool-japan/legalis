//! Legal Reasoning Engine for UK Employment Law.
//!
//! Provides automated compliance analysis and violation detection.

use legalis_core::{EvaluationContext, StatuteRegistry};

use super::context::UkEvaluationContext;
use super::error::ReasoningResult;
use super::statute_adapter::all_employment_statutes;
use super::types::{ComplianceStatus, LegalAnalysis, RiskLevel, Violation, ViolationSeverity};

use crate::employment::types::{EmploymentContract, MinimumWageAssessment, WorkingHours};

/// Legal Reasoning Engine for UK Employment Law
pub struct LegalReasoningEngine {
    registry: StatuteRegistry,
}

impl Default for LegalReasoningEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl LegalReasoningEngine {
    /// Create a new reasoning engine with all UK employment statutes
    #[must_use]
    pub fn new() -> Self {
        let mut registry = StatuteRegistry::new();
        for statute in all_employment_statutes() {
            registry.add(statute);
        }
        Self { registry }
    }

    /// Create with custom registry
    #[must_use]
    pub fn with_registry(registry: StatuteRegistry) -> Self {
        Self { registry }
    }

    /// Analyze an employment contract for compliance
    pub fn analyze_employment_contract(
        &self,
        contract: &EmploymentContract,
    ) -> ReasoningResult<LegalAnalysis> {
        let ctx = UkEvaluationContext::new(contract);
        let mut analysis = LegalAnalysis::new(
            ComplianceStatus::Compliant,
            RiskLevel::None,
            "UK Employment Contract Analysis",
        );

        // Check written particulars (ERA 1996 s.1)
        if !contract.written_particulars_provided {
            if let Some(months) = ctx.get_duration(legalis_core::DurationUnit::Months) {
                if months >= 2 {
                    analysis.add_violation(
                        Violation::new(
                            "ERA_s1",
                            "Written Particulars of Employment",
                            "Written particulars not provided within 2 months of start",
                            ViolationSeverity::Moderate,
                        )
                        .with_legal_reference("ERA 1996 s.1")
                        .with_remediation("Provide written statement of employment particulars"),
                    );
                }
            }
        }

        // Check 48-hour limit compliance (WTR 1998 Reg 4)
        if !contract.working_hours.complies_with_48h_limit() {
            analysis.add_violation(
                Violation::new(
                    "WTR_Reg4",
                    "Maximum Working Week",
                    format!(
                        "Working {} hours/week exceeds 48-hour limit without opt-out",
                        contract.working_hours.hours_per_week
                    ),
                    ViolationSeverity::Major,
                )
                .with_legal_reference("WTR 1998 Reg 4")
                .with_remediation("Obtain written opt-out or reduce hours to 48 or below"),
            );
        }

        // Check pension auto-enrolment
        if contract.salary.gross_annual_gbp >= 10000.0 && contract.pension_scheme.is_none() {
            analysis.add_violation(
                Violation::new(
                    "PENSION_AE",
                    "Workplace Pension Auto-Enrolment",
                    "Eligible worker not enrolled in workplace pension",
                    ViolationSeverity::Moderate,
                )
                .with_remediation("Enrol worker in qualifying workplace pension scheme"),
            );
        }

        // Update overall status
        if !analysis.violations.is_empty() {
            let has_major = analysis
                .violations
                .iter()
                .any(|v| v.severity >= ViolationSeverity::Major);

            analysis.status = if has_major {
                ComplianceStatus::NonCompliant
            } else {
                ComplianceStatus::PartiallyCompliant
            };
        }

        Ok(analysis)
    }

    /// Analyze working hours for compliance
    pub fn analyze_working_hours(&self, hours: &WorkingHours) -> ReasoningResult<LegalAnalysis> {
        let _ctx = UkEvaluationContext::new(hours);
        let mut analysis = LegalAnalysis::new(
            ComplianceStatus::Compliant,
            RiskLevel::None,
            "UK Working Time Analysis",
        );

        // Check 48-hour limit
        if !hours.complies_with_48h_limit() {
            analysis.add_violation(
                Violation::new(
                    "WTR_Reg4",
                    "Maximum Working Week",
                    format!(
                        "Working {} hours/week exceeds 48-hour limit",
                        hours.hours_per_week
                    ),
                    ViolationSeverity::Major,
                )
                .with_legal_reference("WTR 1998 Reg 4"),
            );
            analysis.status = ComplianceStatus::NonCompliant;
        }

        // Check night work hours if applicable
        if let Some(night_hours) = hours.night_work_hours {
            if night_hours > 8 {
                analysis.add_violation(
                    Violation::new(
                        "WTR_Reg6",
                        "Night Work Limits",
                        format!("Night work of {} hours exceeds 8-hour limit", night_hours),
                        ViolationSeverity::Moderate,
                    )
                    .with_legal_reference("WTR 1998 Reg 6"),
                );
                if analysis.status == ComplianceStatus::Compliant {
                    analysis.status = ComplianceStatus::PartiallyCompliant;
                }
            }
        }

        Ok(analysis)
    }

    /// Analyze minimum wage compliance
    pub fn analyze_minimum_wage(
        &self,
        assessment: &MinimumWageAssessment,
    ) -> ReasoningResult<LegalAnalysis> {
        let mut analysis = LegalAnalysis::new(
            ComplianceStatus::Compliant,
            RiskLevel::None,
            "UK Minimum Wage Analysis",
        );

        if !assessment.is_compliant() {
            let shortfall = assessment.applicable_minimum_wage() - assessment.hourly_rate_gbp;
            analysis.add_violation(
                Violation::new(
                    "NMWA_1998",
                    "National Minimum Wage",
                    format!(
                        "Hourly rate £{:.2} below minimum wage £{:.2} (shortfall: £{:.2})",
                        assessment.hourly_rate_gbp,
                        assessment.applicable_minimum_wage(),
                        shortfall
                    ),
                    ViolationSeverity::Critical,
                )
                .with_legal_reference("NMWA 1998")
                .with_remediation(format!(
                    "Increase hourly rate to at least £{:.2}",
                    assessment.applicable_minimum_wage()
                )),
            );
            analysis.status = ComplianceStatus::NonCompliant;
        }

        Ok(analysis)
    }

    /// Get the statute registry
    #[must_use]
    pub fn registry(&self) -> &StatuteRegistry {
        &self.registry
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn create_test_contract() -> EmploymentContract {
        EmploymentContract {
            employee: crate::employment::types::Employee {
                name: "Test Employee".to_string(),
                date_of_birth: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
                address: "London".to_string(),
                national_insurance_number: None,
            },
            employer: crate::employment::types::Employer {
                name: "Test Ltd".to_string(),
                address: "London".to_string(),
                employee_count: Some(50),
            },
            contract_type: crate::employment::types::ContractType::Permanent,
            start_date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            end_date: None,
            probation_period_months: None,
            salary: crate::employment::types::Salary {
                gross_annual_gbp: 35000.0,
                payment_frequency: crate::employment::types::PaymentFrequency::Monthly,
                payment_day: 25,
            },
            working_hours: WorkingHours {
                hours_per_week: 40,
                days_per_week: 5,
                opted_out_of_48h_limit: false,
                night_work_hours: None,
            },
            duties: "Software Developer".to_string(),
            notice_period: crate::employment::types::NoticePeriod {
                employer_notice_weeks: 4,
                employee_notice_weeks: 4,
            },
            written_particulars_provided: true,
            pension_scheme: Some(crate::employment::types::PensionScheme {
                scheme_name: "NEST".to_string(),
                employee_contribution_pct: 5.0,
                employer_contribution_pct: 3.0,
                auto_enrolled: true,
            }),
        }
    }

    #[test]
    fn test_compliant_contract() {
        let engine = LegalReasoningEngine::new();
        let contract = create_test_contract();
        let analysis = engine.analyze_employment_contract(&contract).unwrap();

        assert_eq!(analysis.status, ComplianceStatus::Compliant);
        assert!(analysis.violations.is_empty());
    }

    #[test]
    fn test_working_hours_violation() {
        let engine = LegalReasoningEngine::new();
        let hours = WorkingHours {
            hours_per_week: 55,
            days_per_week: 5,
            opted_out_of_48h_limit: false,
            night_work_hours: None,
        };

        let analysis = engine.analyze_working_hours(&hours).unwrap();
        assert_eq!(analysis.status, ComplianceStatus::NonCompliant);
        assert!(!analysis.violations.is_empty());
    }

    #[test]
    fn test_minimum_wage_violation() {
        let engine = LegalReasoningEngine::new();
        let assessment = MinimumWageAssessment {
            age: 25,
            hourly_rate_gbp: 10.00, // Below NLW of £11.44
            apprentice: false,
        };

        let analysis = engine.analyze_minimum_wage(&assessment).unwrap();
        assert_eq!(analysis.status, ComplianceStatus::NonCompliant);
        assert!(!analysis.violations.is_empty());
    }
}
