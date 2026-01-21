//! Types for Vietnamese Labor Code 2019

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// Type of employment contract - Article 20
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContractType {
    /// Hợp đồng lao động không xác định thời hạn (Indefinite-term)
    IndefiniteTerm,
    /// Hợp đồng lao động xác định thời hạn (Fixed-term) - max 36 months
    FixedTerm {
        /// Duration in months (12-36)
        duration_months: u32,
    },
    /// Hợp đồng lao động theo mùa vụ (Seasonal) - max 12 months
    Seasonal {
        /// Duration in months (max 12)
        duration_months: u32,
    },
}

impl ContractType {
    /// Check if this is a permanent contract
    pub fn is_indefinite(&self) -> bool {
        matches!(self, Self::IndefiniteTerm)
    }

    /// Get maximum duration for fixed-term contracts
    pub fn max_fixed_term_months() -> u32 {
        36
    }

    /// Get maximum duration for seasonal contracts
    pub fn max_seasonal_months() -> u32 {
        12
    }

    /// Check if contract duration is valid
    pub fn is_valid_duration(&self) -> bool {
        match self {
            Self::IndefiniteTerm => true,
            Self::FixedTerm { duration_months } => {
                *duration_months >= 12 && *duration_months <= Self::max_fixed_term_months()
            }
            Self::Seasonal { duration_months } => *duration_months <= Self::max_seasonal_months(),
        }
    }
}

/// Employment contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmploymentContract {
    /// Contract type
    pub contract_type: ContractType,
    /// Employee name
    pub employee_name: String,
    /// Employee ID (CCCD/CMND)
    pub employee_id: Option<String>,
    /// Employer/company name
    pub employer_name: String,
    /// Job position
    pub position: String,
    /// Workplace location
    pub workplace: String,
    /// Contract start date
    pub start_date: NaiveDate,
    /// Contract end date (None for indefinite)
    pub end_date: Option<NaiveDate>,
    /// Monthly salary (VND)
    pub monthly_salary: i64,
    /// Wage region for minimum wage calculation
    pub wage_region: String,
    /// Working hours schedule
    pub working_hours: WorkingHours,
    /// Whether contract is in Vietnamese
    pub in_vietnamese: bool,
    /// Whether contract is written
    pub is_written: bool,
    /// Probation period in days (max 180 for managers, 60 for others)
    pub probation_days: Option<u32>,
    /// Date contract was signed
    pub signed_date: DateTime<Utc>,
}

/// Working hours configuration - Article 105
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingHours {
    /// Hours per day (max 8, or 10 for special sectors)
    pub hours_per_day: u32,
    /// Days per week
    pub days_per_week: u32,
    /// Overtime hours per day
    pub overtime_per_day: u32,
    /// Overtime hours per month
    pub overtime_per_month: u32,
}

impl WorkingHours {
    /// Standard 6-day work week (8 hours/day, 48 hours/week)
    pub fn standard() -> Self {
        Self {
            hours_per_day: 8,
            days_per_week: 6,
            overtime_per_day: 0,
            overtime_per_month: 0,
        }
    }

    /// 5-day work week (8 hours/day, 40 hours/week)
    pub fn five_day_week() -> Self {
        Self {
            hours_per_day: 8,
            days_per_week: 5,
            overtime_per_day: 0,
            overtime_per_month: 0,
        }
    }

    /// Calculate total weekly hours
    pub fn total_weekly_hours(&self) -> u32 {
        self.hours_per_day * self.days_per_week
    }

    /// Check if working hours are within legal limits - Article 105
    pub fn is_within_limits(&self) -> bool {
        // Regular hours max 8/day, 48/week
        if self.hours_per_day > 8 || self.total_weekly_hours() > 48 {
            return false;
        }

        // Overtime max 4 hours/day (Article 107)
        if self.overtime_per_day > 4 {
            return false;
        }

        // Overtime max 40 hours/month, 200 hours/year
        if self.overtime_per_month > 40 {
            return false;
        }

        true
    }
}

/// Termination type - Article 34-36
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TerminationType {
    /// Hết hạn hợp đồng (Contract expiry)
    ContractExpiry,
    /// Thỏa thuận chấm dứt (Mutual agreement)
    MutualAgreement,
    /// Người lao động đơn phương chấm dứt (Employee resignation)
    EmployeeResignation,
    /// Người sử dụng lao động đơn phương chấm dứt (Employer termination)
    EmployerTermination(EmployerTerminationReason),
    /// Sa thải (Dismissal for serious violation)
    Dismissal,
    /// Nghỉ hưu (Retirement)
    Retirement,
    /// Chết (Death)
    Death,
}

/// Reason for employer-initiated termination - Article 36
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EmployerTerminationReason {
    /// Repeated failure to perform duties
    RepeatedFailure,
    /// Prolonged illness (12 months indefinite, 6 months fixed-term)
    ProlongedIllness,
    /// Force majeure
    ForceMajeure,
    /// Business restructuring
    Restructuring,
    /// Redundancy
    Redundancy,
}

impl TerminationType {
    /// Check if termination entitles employee to severance - Article 46
    pub fn entitles_severance(&self) -> bool {
        matches!(
            self,
            Self::ContractExpiry
                | Self::MutualAgreement
                | Self::EmployerTermination(_)
                | Self::Retirement
        )
    }

    /// Get notice period in days - Article 35-36
    pub fn notice_period_days(&self, contract_type: &ContractType) -> u32 {
        match self {
            Self::EmployeeResignation => match contract_type {
                ContractType::IndefiniteTerm => 45,
                ContractType::FixedTerm { .. } => 30,
                ContractType::Seasonal { .. } => 3,
            },
            Self::EmployerTermination(_) => match contract_type {
                ContractType::IndefiniteTerm => 45,
                ContractType::FixedTerm { .. } => 30,
                ContractType::Seasonal { .. } => 3,
            },
            _ => 0,
        }
    }
}

/// Severance calculation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Severance {
    /// Years of service
    pub years_of_service: u32,
    /// Monthly salary for calculation
    pub monthly_salary: i64,
    /// Severance months (0.5 per year)
    pub severance_months: f64,
    /// Total severance amount
    pub total_amount: i64,
    /// Termination type
    pub termination_type: TerminationType,
}

impl Severance {
    /// Calculate severance based on tenure - Article 46
    /// 0.5 months salary per year of service
    pub fn calculate(
        years_of_service: u32,
        monthly_salary: i64,
        termination_type: TerminationType,
    ) -> Self {
        let severance_months = if termination_type.entitles_severance() {
            years_of_service as f64 * 0.5
        } else {
            0.0
        };

        let total_amount = (monthly_salary as f64 * severance_months) as i64;

        Self {
            years_of_service,
            monthly_salary,
            severance_months,
            total_amount,
            termination_type,
        }
    }
}

/// Types of leave - Article 112-116
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LeaveType {
    /// Nghỉ phép năm (Annual leave) - 12+ days
    Annual,
    /// Nghỉ ốm (Sick leave) - per social insurance
    Sick,
    /// Thai sản (Maternity) - 6 months
    Maternity,
    /// Nghỉ việc riêng (Personal leave) - 3 days
    Personal,
    /// Đám cưới bản thân (Own wedding) - 3 days
    OwnWedding,
    /// Đám cưới con (Child's wedding) - 1 day
    ChildWedding,
    /// Tang lễ (Funeral) - 3 days for parents/spouse/children
    Funeral,
    /// Nghỉ lễ, Tết (Public holidays)
    PublicHoliday,
}

impl LeaveType {
    /// Get default days for this leave type
    pub fn default_days(&self) -> u32 {
        match self {
            Self::Annual => 12,        // Base, increases with seniority
            Self::Sick => 30,          // Varies by social insurance contribution
            Self::Maternity => 180,    // 6 months
            Self::Personal => 3,       // When permitted by employer
            Self::OwnWedding => 3,     // Paid leave
            Self::ChildWedding => 1,   // Paid leave
            Self::Funeral => 3,        // For close relatives
            Self::PublicHoliday => 11, // National holidays
        }
    }

    /// Check if leave is paid
    pub fn is_paid(&self) -> bool {
        match self {
            Self::Annual => true,
            Self::Sick => true,      // Paid by social insurance
            Self::Maternity => true, // Paid by social insurance
            Self::Personal => false, // Unpaid
            Self::OwnWedding => true,
            Self::ChildWedding => true,
            Self::Funeral => true,
            Self::PublicHoliday => true,
        }
    }

    /// Get additional annual leave days based on seniority - Article 113
    pub fn annual_leave_by_seniority(years_of_service: u32, hazardous_work: bool) -> u32 {
        let base = if hazardous_work { 14 } else { 12 };
        // Additional 1 day per 5 years of service
        base + (years_of_service / 5)
    }
}

/// Social insurance contribution breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialInsurance {
    /// BHXH - Bảo hiểm xã hội (Social Insurance)
    /// Employer: 17.5%, Employee: 8%
    pub bhxh_employer: i64,
    pub bhxh_employee: i64,
    /// BHYT - Bảo hiểm y tế (Health Insurance)
    /// Employer: 3%, Employee: 1.5%
    pub bhyt_employer: i64,
    pub bhyt_employee: i64,
    /// BHTN - Bảo hiểm thất nghiệp (Unemployment Insurance)
    /// Employer: 1%, Employee: 1%
    pub bhtn_employer: i64,
    pub bhtn_employee: i64,
    /// Total employer contribution
    pub total_employer: i64,
    /// Total employee contribution
    pub total_employee: i64,
    /// Grand total
    pub total: i64,
}

impl SocialInsurance {
    /// Calculate social insurance contributions based on salary
    pub fn calculate(monthly_salary: i64) -> Self {
        // Capped at 20x base salary (approximately 29.8M VND in 2024)
        let cap = 29_800_000i64;
        let contribution_base = monthly_salary.min(cap);

        // BHXH: 17.5% employer, 8% employee
        let bhxh_employer = (contribution_base as f64 * 0.175) as i64;
        let bhxh_employee = (contribution_base as f64 * 0.08) as i64;

        // BHYT: 3% employer, 1.5% employee
        let bhyt_employer = (contribution_base as f64 * 0.03) as i64;
        let bhyt_employee = (contribution_base as f64 * 0.015) as i64;

        // BHTN: 1% employer, 1% employee
        let bhtn_employer = (contribution_base as f64 * 0.01) as i64;
        let bhtn_employee = (contribution_base as f64 * 0.01) as i64;

        let total_employer = bhxh_employer + bhyt_employer + bhtn_employer;
        let total_employee = bhxh_employee + bhyt_employee + bhtn_employee;

        Self {
            bhxh_employer,
            bhxh_employee,
            bhyt_employer,
            bhyt_employee,
            bhtn_employer,
            bhtn_employee,
            total_employer,
            total_employee,
            total: total_employer + total_employee,
        }
    }

    /// Get employer contribution rate (total)
    pub fn employer_rate() -> f64 {
        0.215 // 21.5%
    }

    /// Get employee contribution rate (total)
    pub fn employee_rate() -> f64 {
        0.105 // 10.5%
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_working_hours_limits() {
        let standard = WorkingHours::standard();
        assert!(standard.is_within_limits());
        assert_eq!(standard.total_weekly_hours(), 48);

        let excessive = WorkingHours {
            hours_per_day: 10,
            days_per_week: 6,
            overtime_per_day: 0,
            overtime_per_month: 0,
        };
        assert!(!excessive.is_within_limits());
    }

    #[test]
    fn test_severance_calculation() {
        let severance = Severance::calculate(10, 10_000_000, TerminationType::ContractExpiry);

        assert_eq!(severance.severance_months, 5.0); // 10 years * 0.5
        assert_eq!(severance.total_amount, 50_000_000);
    }

    #[test]
    fn test_severance_resignation() {
        let severance = Severance::calculate(5, 10_000_000, TerminationType::EmployeeResignation);
        assert_eq!(severance.total_amount, 0); // No severance for resignation
    }

    #[test]
    fn test_social_insurance() {
        let insurance = SocialInsurance::calculate(10_000_000);

        assert!(insurance.total_employer > 0);
        assert!(insurance.total_employee > 0);
        assert_eq!(
            insurance.total,
            insurance.total_employer + insurance.total_employee
        );
    }

    #[test]
    fn test_leave_types() {
        assert_eq!(LeaveType::Annual.default_days(), 12);
        assert_eq!(LeaveType::Maternity.default_days(), 180);
        assert!(LeaveType::Annual.is_paid());
        assert!(!LeaveType::Personal.is_paid());
    }

    #[test]
    fn test_annual_leave_seniority() {
        assert_eq!(LeaveType::annual_leave_by_seniority(0, false), 12);
        assert_eq!(LeaveType::annual_leave_by_seniority(5, false), 13);
        assert_eq!(LeaveType::annual_leave_by_seniority(10, false), 14);
        assert_eq!(LeaveType::annual_leave_by_seniority(5, true), 15); // Hazardous
    }

    #[test]
    fn test_contract_type_duration() {
        let valid_fixed = ContractType::FixedTerm {
            duration_months: 24,
        };
        assert!(valid_fixed.is_valid_duration());

        let invalid_fixed = ContractType::FixedTerm {
            duration_months: 48,
        };
        assert!(!invalid_fixed.is_valid_duration());
    }
}
