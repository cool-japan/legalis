//! Validation functions for Vietnamese Labor Code 2019

use super::error::{LaborCodeError, LaborCodeResult};
use super::types::*;
use crate::common::WageRegion;
use serde::{Deserialize, Serialize};

/// Validate working hours - Article 105
pub fn validate_working_hours(hours: &WorkingHours) -> LaborCodeResult<()> {
    let weekly_hours = hours.total_weekly_hours();

    // Regular hours max 8/day, 48/week
    if weekly_hours > 48 {
        return Err(LaborCodeError::WorkingHoursExceeded {
            hours: weekly_hours,
        });
    }

    // Overtime max 40 hours/month
    if hours.overtime_per_month > 40 {
        return Err(LaborCodeError::OvertimeExceeded {
            hours: hours.overtime_per_month,
        });
    }

    Ok(())
}

/// Validate minimum wage - Article 90-91
pub fn validate_minimum_wage(monthly_wage: i64, region: &WageRegion) -> LaborCodeResult<()> {
    let minimum = region.minimum_wage_2024();

    if monthly_wage < minimum.amount() {
        return Err(LaborCodeError::MinimumWageViolation {
            actual: monthly_wage,
            minimum: minimum.amount(),
        });
    }

    Ok(())
}

/// Validate employment contract - Article 14, 20, 25
pub fn validate_contract(contract: &EmploymentContract) -> LaborCodeResult<()> {
    // Check language requirement - Article 14
    if !contract.in_vietnamese {
        return Err(LaborCodeError::ContractNotInVietnamese);
    }

    // Check written requirement - Article 14
    if !contract.is_written {
        return Err(LaborCodeError::ContractNotWritten);
    }

    // Check contract duration
    if !contract.contract_type.is_valid_duration() {
        let months = match &contract.contract_type {
            ContractType::FixedTerm { duration_months } => *duration_months,
            ContractType::Seasonal { duration_months } => *duration_months,
            _ => 0,
        };
        return Err(LaborCodeError::InvalidContractDuration { months });
    }

    // Check probation period - Article 25
    if let Some(probation_days) = contract.probation_days {
        let max_days = match &contract.contract_type {
            ContractType::IndefiniteTerm => 60, // General workers
            ContractType::FixedTerm { .. } => 60,
            ContractType::Seasonal { .. } => 6, // Very short for seasonal
        };

        if probation_days > max_days {
            return Err(LaborCodeError::ProbationTooLong {
                days: probation_days,
                max_days,
            });
        }
    }

    // Validate minimum wage
    let region = match contract.wage_region.to_lowercase().as_str() {
        "vung i" | "vùng i" | "region 1" | "1" => WageRegion::Region1,
        "vung ii" | "vùng ii" | "region 2" | "2" => WageRegion::Region2,
        "vung iii" | "vùng iii" | "region 3" | "3" => WageRegion::Region3,
        "vung iv" | "vùng iv" | "region 4" | "4" => WageRegion::Region4,
        _ => WageRegion::Region3, // Default
    };

    validate_minimum_wage(contract.monthly_salary, &region)?;

    // Validate working hours
    validate_working_hours(&contract.working_hours)?;

    Ok(())
}

/// Calculate severance payment
pub fn calculate_severance(
    years_of_service: u32,
    monthly_salary: i64,
    termination_type: TerminationType,
) -> Severance {
    Severance::calculate(years_of_service, monthly_salary, termination_type)
}

/// Calculate social insurance contributions
pub fn calculate_social_insurance(monthly_salary: i64) -> SocialInsurance {
    SocialInsurance::calculate(monthly_salary)
}

/// Labor compliance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaborCompliance {
    /// Overall compliance status
    pub compliant: bool,
    /// Working hours compliant
    pub working_hours_compliant: bool,
    /// Minimum wage met
    pub minimum_wage_met: bool,
    /// Contract valid
    pub contract_valid: bool,
    /// Social insurance registered
    pub social_insurance_registered: bool,
    /// Annual leave provided
    pub annual_leave_provided: bool,
    /// Issues found
    pub issues: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Comprehensive labor compliance check
pub fn validate_labor_compliance(
    contract: &EmploymentContract,
    social_insurance_registered: bool,
    annual_leave_days: u32,
) -> LaborCompliance {
    let mut compliance = LaborCompliance {
        compliant: true,
        working_hours_compliant: true,
        minimum_wage_met: true,
        contract_valid: true,
        social_insurance_registered,
        annual_leave_provided: true,
        issues: Vec::new(),
        recommendations: Vec::new(),
    };

    // Validate working hours
    if let Err(e) = validate_working_hours(&contract.working_hours) {
        compliance.compliant = false;
        compliance.working_hours_compliant = false;
        compliance.issues.push(format!("{}", e));
        compliance
            .recommendations
            .push("Điều chỉnh giờ làm việc theo Điều 105 BLLĐ".to_string());
    }

    // Validate contract
    if let Err(e) = validate_contract(contract) {
        compliance.compliant = false;
        compliance.contract_valid = false;
        compliance.issues.push(format!("{}", e));
    }

    // Check social insurance
    if !social_insurance_registered {
        compliance.compliant = false;
        compliance
            .issues
            .push("Chưa đăng ký bảo hiểm xã hội bắt buộc".to_string());
        compliance
            .recommendations
            .push("Đăng ký BHXH, BHYT, BHTN theo Điều 168 BLLĐ".to_string());
    }

    // Check annual leave
    let required_leave = LeaveType::annual_leave_by_seniority(0, false);
    if annual_leave_days < required_leave {
        compliance.compliant = false;
        compliance.annual_leave_provided = false;
        compliance.issues.push(format!(
            "Nghỉ phép năm: {} ngày (tối thiểu {} ngày)",
            annual_leave_days, required_leave
        ));
        compliance
            .recommendations
            .push("Đảm bảo quyền nghỉ phép năm theo Điều 113 BLLĐ".to_string());
    }

    compliance
}

/// Get labor law compliance checklist
pub fn get_labor_checklist() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "Hợp đồng lao động bằng văn bản",
            "Written employment contract",
            "Điều 14",
        ),
        (
            "Hợp đồng bằng tiếng Việt",
            "Contract in Vietnamese",
            "Điều 14",
        ),
        (
            "Lương tối thiểu vùng",
            "Regional minimum wage",
            "Điều 90-91",
        ),
        (
            "Giờ làm việc tối đa 48 giờ/tuần",
            "Max 48 hours/week",
            "Điều 105",
        ),
        (
            "Làm thêm tối đa 40 giờ/tháng",
            "Max 40 hours overtime/month",
            "Điều 107",
        ),
        (
            "Nghỉ ít nhất 24 giờ liên tục/tuần",
            "24 hours weekly rest",
            "Điều 110",
        ),
        (
            "Nghỉ phép năm 12+ ngày",
            "12+ days annual leave",
            "Điều 113",
        ),
        (
            "Nghỉ thai sản 6 tháng",
            "6 months maternity leave",
            "Điều 139",
        ),
        (
            "Đăng ký BHXH, BHYT, BHTN",
            "Social insurance registration",
            "Điều 168",
        ),
        (
            "Công đoàn được thành lập",
            "Trade union established",
            "Điều 170",
        ),
        (
            "Thử việc tối đa 60 ngày",
            "Max 60 days probation",
            "Điều 25",
        ),
        (
            "Trợ cấp thôi việc 0.5 tháng/năm",
            "Severance 0.5 months/year",
            "Điều 46",
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn sample_contract() -> EmploymentContract {
        EmploymentContract {
            contract_type: ContractType::IndefiniteTerm,
            employee_name: "Nguyễn Văn A".to_string(),
            employee_id: Some("012345678901".to_string()),
            employer_name: "Công ty ABC".to_string(),
            position: "Nhân viên".to_string(),
            workplace: "Hà Nội".to_string(),
            start_date: chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            end_date: None,
            monthly_salary: 10_000_000,
            wage_region: "Vùng I".to_string(),
            working_hours: WorkingHours::standard(),
            in_vietnamese: true,
            is_written: true,
            probation_days: Some(60),
            signed_date: Utc::now(),
        }
    }

    #[test]
    fn test_validate_working_hours_ok() {
        let hours = WorkingHours::standard();
        assert!(validate_working_hours(&hours).is_ok());
    }

    #[test]
    fn test_validate_working_hours_exceeded() {
        let hours = WorkingHours {
            hours_per_day: 10,
            days_per_week: 6,
            overtime_per_day: 0,
            overtime_per_month: 0,
        };
        assert!(validate_working_hours(&hours).is_err());
    }

    #[test]
    fn test_validate_minimum_wage_ok() {
        let result = validate_minimum_wage(10_000_000, &WageRegion::Region1);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_minimum_wage_violation() {
        let result = validate_minimum_wage(3_000_000, &WageRegion::Region1);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_contract_valid() {
        let contract = sample_contract();
        assert!(validate_contract(&contract).is_ok());
    }

    #[test]
    fn test_validate_contract_not_vietnamese() {
        let mut contract = sample_contract();
        contract.in_vietnamese = false;
        let result = validate_contract(&contract);
        assert!(matches!(
            result,
            Err(LaborCodeError::ContractNotInVietnamese)
        ));
    }

    #[test]
    fn test_labor_compliance_check() {
        let contract = sample_contract();
        let compliance = validate_labor_compliance(&contract, true, 12);
        assert!(compliance.compliant);
        assert!(compliance.issues.is_empty());
    }

    #[test]
    fn test_labor_checklist() {
        let checklist = get_labor_checklist();
        assert!(!checklist.is_empty());
        assert!(checklist.iter().any(|(vi, _, _)| vi.contains("BHXH")));
    }
}
