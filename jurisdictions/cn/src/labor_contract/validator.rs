//! Labor Contract Law Validation
//!
//! # 劳动合同法合规验证

#![allow(missing_docs)]

use super::error::{LaborContractError, LaborContractResult};
use super::types::*;
use crate::common::currency::CnyAmount;
use crate::i18n::BilingualText;
use chrono::NaiveDate;

/// Labor contract compliance report
#[derive(Debug, Clone)]
pub struct LaborComplianceReport {
    pub compliant: bool,
    pub violations: Vec<LaborContractError>,
    pub warnings: Vec<BilingualText>,
    pub severance_due: Option<SeveranceCalculation>,
}

impl Default for LaborComplianceReport {
    fn default() -> Self {
        Self {
            compliant: true,
            violations: Vec::new(),
            warnings: Vec::new(),
            severance_due: None,
        }
    }
}

/// Validate labor contract compliance
pub fn validate_contract(
    contract: &LaborContract,
    has_written_contract: bool,
    days_since_start: u32,
) -> LaborComplianceReport {
    let mut report = LaborComplianceReport::default();

    // Check written contract requirement (Article 10)
    if !has_written_contract && days_since_start > 30 {
        report
            .violations
            .push(LaborContractError::NoWrittenContract);
        report.compliant = false;
    }

    // Check probation period (Article 19)
    if let Some(probation_end) = contract.probation_end_date {
        let probation_days = (probation_end - contract.start_date).num_days() as u32;
        let max_probation = contract.max_probation();

        if let Some(max_days) = max_probation.max_days()
            && probation_days > max_days
        {
            report
                .violations
                .push(LaborContractError::ProbationExceedsLimit {
                    actual_days: probation_days,
                    max_days,
                });
            report.compliant = false;
        }

        if matches!(max_probation, ProbationLimit::NotAllowed) && probation_days > 0 {
            report
                .violations
                .push(LaborContractError::ProbationNotAllowed);
            report.compliant = false;
        }
    }

    // Check social insurance (Article 17)
    if !contract.social_insurance.is_complete() {
        let missing = contract.social_insurance.missing().join(", ");
        report
            .violations
            .push(LaborContractError::SocialInsuranceIncomplete { missing });
        report.compliant = false;
    }

    // Check housing fund (warning if not enrolled)
    if !contract.housing_fund.enrolled {
        report.warnings.push(BilingualText::new(
            "建议缴纳住房公积金",
            "Housing provident fund enrollment recommended",
        ));
    }

    report
}

/// Validate termination compliance
pub fn validate_termination(
    contract: &LaborContract,
    reason: TerminationReason,
    notice_days: u32,
    protected_status: Option<ProtectedCategory>,
    local_average_wage: CnyAmount,
) -> LaborComplianceReport {
    let mut report = LaborComplianceReport::default();

    // Check protected employee status (Article 42)
    if let Some(protected) = protected_status {
        // Can only terminate for Article 39 reasons (serious violations)
        if !reason.immediate_termination() {
            report
                .violations
                .push(LaborContractError::ProtectedEmployeeTermination {
                    category: protected.name_zh().to_string(),
                });
            report.compliant = false;
        }
    }

    // Check notice requirement (Article 40)
    if matches!(
        reason,
        TerminationReason::MedicalInability
            | TerminationReason::Incompetence
            | TerminationReason::ObjectiveChange
    ) && notice_days < 30
    {
        report
            .violations
            .push(LaborContractError::TerminationWithoutNotice);
        report.compliant = false;
    }

    // Calculate severance if required
    if reason.requires_severance()
        && let Some(end_date) = contract.end_date
    {
        let years = calculate_service_years(contract.start_date, end_date);
        report.severance_due = Some(calculate_severance(
            years,
            contract.monthly_salary,
            local_average_wage,
        ));
    }

    report
}

/// Validate economic layoff compliance (Article 41)
pub fn validate_economic_layoff(layoff: &EconomicLayoff) -> LaborComplianceReport {
    let mut report = LaborComplianceReport::default();

    if layoff.is_mass_layoff() {
        // Check 30-day union notice
        if !layoff.union_notified {
            report
                .violations
                .push(LaborContractError::LayoffProcedureViolation {
                    missing_step: "未提前三十日向工会说明情况".to_string(),
                });
            report.compliant = false;
        }

        // Check employee notification
        if !layoff.employees_notified {
            report
                .violations
                .push(LaborContractError::LayoffProcedureViolation {
                    missing_step: "未向全体职工说明情况".to_string(),
                });
            report.compliant = false;
        }

        // Check labor bureau report
        if !layoff.labor_bureau_reported {
            report
                .violations
                .push(LaborContractError::LayoffProcedureViolation {
                    missing_step: "未向劳动行政部门报告裁减人员方案".to_string(),
                });
            report.compliant = false;
        }

        // Check priority retention
        if !layoff.priority_retention_applied {
            report.warnings.push(BilingualText::new(
                "应当优先留用：较长期限固定期限合同、无固定期限合同、家庭无其他就业人员的职工",
                "Priority retention required: long-term contracts, open-ended contracts, sole breadwinners",
            ));
        }
    }

    report
}

/// Validate non-compete agreement (Article 23-24)
pub fn validate_non_compete(agreement: &NonCompeteAgreement) -> LaborContractResult<()> {
    // Check duration (max 2 years)
    if agreement.duration_months > 24 {
        return Err(LaborContractError::NonCompetePeriodExceeds {
            months: agreement.duration_months,
        });
    }

    // Check compensation exists
    if agreement.monthly_compensation.yuan() <= 0.0 {
        return Err(LaborContractError::NonCompeteCompensationNotPaid);
    }

    Ok(())
}

/// Validate labor dispatch compliance
pub fn validate_dispatch(dispatch: &LaborDispatch) -> LaborComplianceReport {
    let mut report = LaborComplianceReport::default();

    // Check 10% ratio limit
    if !dispatch.is_ratio_compliant() {
        let actual_pct =
            (dispatch.dispatched_count as f64 / dispatch.host_workforce as f64) * 100.0;
        report
            .violations
            .push(LaborContractError::DispatchRatioExceeded { actual_pct });
        report.compliant = false;
    }

    // Check position type for temporary positions
    if matches!(dispatch.position_type, DispatchPositionType::Temporary)
        && dispatch.duration_months > 6
    {
        report.warnings.push(BilingualText::new(
            "临时性岗位一般不超过六个月",
            "Temporary positions generally should not exceed 6 months",
        ));
    }

    report
}

/// Validate overtime pay
pub fn validate_overtime_pay(
    hours: f64,
    overtime_type: OvertimeType,
    hourly_rate: f64,
    amount_paid: f64,
) -> LaborContractResult<()> {
    let required_rate = overtime_type.rate_multiplier();
    let minimum_pay = hours * hourly_rate * required_rate;

    if amount_paid < minimum_pay * 0.99 {
        // 1% tolerance for rounding
        return Err(LaborContractError::OvertimePayIncorrect {
            overtime_type: overtime_type.name_zh().to_string(),
            required_rate,
        });
    }

    Ok(())
}

/// Validate annual leave entitlement
pub fn validate_annual_leave(
    cumulative_years: u32,
    days_granted: u32,
    days_taken: u32,
) -> LaborContractResult<()> {
    let entitlement = AnnualLeaveEntitlement::from_years(cumulative_years);

    if days_granted < entitlement.days {
        return Err(LaborContractError::AnnualLeaveNotGranted {
            days: entitlement.days,
        });
    }

    // Cannot carry forward more than one year's worth
    let unused = days_granted.saturating_sub(days_taken);
    if unused > entitlement.days {
        // Warning only, not an error
    }

    Ok(())
}

/// Calculate service years (rounded up for 6+ months)
pub fn calculate_service_years(start: NaiveDate, end: NaiveDate) -> f64 {
    let days = (end - start).num_days() as f64;
    let years = days / 365.0;
    let fractional = years.fract();

    if fractional >= 0.5 {
        years.floor() + 1.0
    } else if fractional > 0.0 {
        years.floor() + 0.5
    } else {
        years
    }
}

/// Calculate severance payment (Article 47)
pub fn calculate_severance(
    years: f64,
    monthly_salary: CnyAmount,
    local_average_wage: CnyAmount,
) -> SeveranceCalculation {
    // Calculate months (1 month per year, 0.5 for less than 6 months)
    let months_count = if years < 0.5 { 0.5 } else { years.ceil() };

    // Cap at 3x local average wage
    let cap_amount = local_average_wage * 3.0;
    let is_capped = monthly_salary.yuan() > cap_amount.yuan();

    let monthly_base = if is_capped {
        cap_amount
    } else {
        monthly_salary
    };

    // Cap at 12 months if salary exceeds 3x average
    let effective_months = if is_capped {
        months_count.min(12.0)
    } else {
        months_count
    };

    let total_amount = monthly_base * effective_months;

    SeveranceCalculation {
        years_of_service: years,
        months_count: effective_months,
        monthly_base,
        is_capped,
        cap_amount: if is_capped { Some(cap_amount) } else { None },
        total_amount,
        legal_basis: BilingualText::new(
            "《劳动合同法》第四十七条",
            "Labor Contract Law Article 47",
        ),
    }
}

/// Check if employee should be offered open-ended contract
pub fn should_offer_open_ended(renewal_count: u32, years_of_service: f64) -> Option<BilingualText> {
    if renewal_count >= 2 {
        return Some(BilingualText::new(
            "连续订立二次固定期限劳动合同后，应当订立无固定期限劳动合同",
            "After two consecutive fixed-term contracts, open-ended contract must be offered",
        ));
    }

    if years_of_service >= 10.0 {
        return Some(BilingualText::new(
            "劳动者在该用人单位连续工作满十年，应当订立无固定期限劳动合同",
            "After 10 years continuous service, open-ended contract must be offered",
        ));
    }

    None
}

/// Validate minimum wage compliance
pub fn validate_minimum_wage(
    actual_wage: CnyAmount,
    local_minimum: CnyAmount,
) -> LaborContractResult<()> {
    if actual_wage.yuan() < local_minimum.yuan() {
        return Err(LaborContractError::WageBelowMinimum {
            actual: actual_wage.yuan(),
            minimum: local_minimum.yuan(),
        });
    }
    Ok(())
}

/// Calculate double wages penalty for no written contract
pub fn calculate_double_wages_penalty(
    monthly_salary: CnyAmount,
    months_without_contract: u32,
) -> CnyAmount {
    // Double wages from 2nd month to 12th month (max 11 months)
    let penalty_months = months_without_contract.saturating_sub(1).min(11) as f64;
    monthly_salary * penalty_months
}

/// Calculate illegal termination compensation
pub fn calculate_illegal_termination_compensation(severance: &SeveranceCalculation) -> CnyAmount {
    // 2x severance for illegal termination
    severance.total_amount * 2.0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_contract() -> LaborContract {
        LaborContract {
            id: "LC-2024-001".to_string(),
            contract_type: ContractType::FixedTerm,
            employer: BilingualText::new("测试公司", "Test Company"),
            employee_name: "张三".to_string(),
            employee_id: "110101199001011234".to_string(),
            position: BilingualText::new("软件工程师", "Software Engineer"),
            work_location: "北京市".to_string(),
            start_date: NaiveDate::from_ymd_opt(2022, 1, 1).expect("valid date"),
            end_date: Some(NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date")),
            probation_end_date: Some(NaiveDate::from_ymd_opt(2022, 7, 1).expect("valid date")),
            monthly_salary: CnyAmount::from_yuan(15000.0),
            working_hours: WorkingHoursType::Standard,
            social_insurance: SocialInsuranceStatus {
                pension: true,
                medical: true,
                unemployment: true,
                work_injury: true,
                maternity: true,
            },
            housing_fund: HousingFundStatus {
                enrolled: true,
                contribution_base: Some(CnyAmount::from_yuan(15000.0)),
                contribution_rate: Some(0.12),
            },
            status: EmploymentStatus::Regular,
            renewal_count: 0,
        }
    }

    #[test]
    fn test_validate_contract_no_written() {
        let contract = create_test_contract();
        let report = validate_contract(&contract, false, 60);
        assert!(!report.compliant);
        assert!(
            report
                .violations
                .iter()
                .any(|e| matches!(e, LaborContractError::NoWrittenContract))
        );
    }

    #[test]
    fn test_validate_contract_probation_exceeds() {
        let mut contract = create_test_contract();
        // Set very long probation (7 months for 3-year contract, max is 6)
        contract.probation_end_date =
            Some(NaiveDate::from_ymd_opt(2022, 8, 1).expect("valid date"));

        let report = validate_contract(&contract, true, 30);
        assert!(!report.compliant);
        assert!(
            report
                .violations
                .iter()
                .any(|e| matches!(e, LaborContractError::ProbationExceedsLimit { .. }))
        );
    }

    #[test]
    fn test_validate_contract_missing_insurance() {
        let mut contract = create_test_contract();
        contract.social_insurance.unemployment = false;

        let report = validate_contract(&contract, true, 30);
        assert!(!report.compliant);
        assert!(
            report
                .violations
                .iter()
                .any(|e| matches!(e, LaborContractError::SocialInsuranceIncomplete { .. }))
        );
    }

    #[test]
    fn test_calculate_severance_normal() {
        let start = NaiveDate::from_ymd_opt(2020, 1, 1).expect("valid date");
        let end = NaiveDate::from_ymd_opt(2023, 6, 15).expect("valid date");
        let years = calculate_service_years(start, end);

        let severance = calculate_severance(
            years,
            CnyAmount::from_yuan(10000.0),
            CnyAmount::from_yuan(12000.0),
        );

        // 3.5 years → 4 months severance
        assert!(!severance.is_capped);
        assert_eq!(severance.total_amount.yuan(), 40000.0);
    }

    #[test]
    fn test_calculate_severance_capped() {
        let severance = calculate_severance(
            5.0,
            CnyAmount::from_yuan(50000.0), // High salary
            CnyAmount::from_yuan(10000.0), // Local average
        );

        // Should be capped at 3x average (30000)
        assert!(severance.is_capped);
        assert_eq!(severance.monthly_base.yuan(), 30000.0);
    }

    #[test]
    fn test_non_compete_validation() {
        let valid = NonCompeteAgreement {
            duration_months: 24,
            geographic_scope: "全国".to_string(),
            industry_scope: "互联网".to_string(),
            monthly_compensation: CnyAmount::from_yuan(5000.0),
            breach_penalty: None,
        };
        assert!(validate_non_compete(&valid).is_ok());

        let invalid_duration = NonCompeteAgreement {
            duration_months: 36,
            geographic_scope: "全国".to_string(),
            industry_scope: "互联网".to_string(),
            monthly_compensation: CnyAmount::from_yuan(5000.0),
            breach_penalty: None,
        };
        assert!(matches!(
            validate_non_compete(&invalid_duration),
            Err(LaborContractError::NonCompetePeriodExceeds { .. })
        ));
    }

    #[test]
    fn test_overtime_pay_validation() {
        // Valid overtime pay
        assert!(
            validate_overtime_pay(
                10.0,                        // 10 hours
                OvertimeType::ExtendedHours, // 1.5x
                100.0,                       // hourly rate
                1500.0,                      // paid amount
            )
            .is_ok()
        );

        // Underpaid overtime
        assert!(matches!(
            validate_overtime_pay(
                10.0,
                OvertimeType::StatutoryHoliday, // 3x
                100.0,
                2000.0, // Should be 3000
            ),
            Err(LaborContractError::OvertimePayIncorrect { .. })
        ));
    }

    #[test]
    fn test_should_offer_open_ended() {
        // After 2 renewals
        assert!(should_offer_open_ended(2, 3.0).is_some());

        // After 10 years
        assert!(should_offer_open_ended(0, 10.5).is_some());

        // Not eligible
        assert!(should_offer_open_ended(1, 5.0).is_none());
    }

    #[test]
    fn test_double_wages_penalty() {
        let salary = CnyAmount::from_yuan(10000.0);

        // 3 months without contract: penalty for months 2-3 = 2 months
        let penalty = calculate_double_wages_penalty(salary, 3);
        assert_eq!(penalty.yuan(), 20000.0);

        // 15 months without contract: capped at 11 months penalty
        let penalty = calculate_double_wages_penalty(salary, 15);
        assert_eq!(penalty.yuan(), 110000.0);
    }

    #[test]
    fn test_illegal_termination_compensation() {
        let severance = SeveranceCalculation {
            years_of_service: 5.0,
            months_count: 5.0,
            monthly_base: CnyAmount::from_yuan(10000.0),
            is_capped: false,
            cap_amount: None,
            total_amount: CnyAmount::from_yuan(50000.0),
            legal_basis: BilingualText::new("第47条", "Article 47"),
        };

        let compensation = calculate_illegal_termination_compensation(&severance);
        assert_eq!(compensation.yuan(), 100000.0); // 2x severance
    }
}
