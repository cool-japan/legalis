//! Employment Act (Cap. 91) - Type Definitions
//!
//! This module provides type-safe representations of Singapore employment relationships,
//! including contracts, working hours, CPF contributions, and leave entitlements.
//!
//! ## Key Types
//!
//! - [`EmploymentContract`]: Complete employment contract structure
//! - [`CpfContribution`]: CPF calculation by age brackets
//! - [`LeaveEntitlement`]: Annual, sick, maternity leave tracking
//! - [`WorkingHours`]: Working hours and overtime tracking
//! - [`TerminationNotice`]: Notice period requirements
//!
//! ## Examples
//!
//! ```
//! use legalis_sg::employment::types::*;
//! use chrono::Utc;
//!
//! // Create employment contract
//! let contract = EmploymentContract {
//!     employee_name: "John Tan".to_string(),
//!     employer_name: "Tech Innovations Pte Ltd".to_string(),
//!     contract_type: ContractType::Indefinite,
//!     start_date: Utc::now(),
//!     end_date: None,
//!     basic_salary_cents: 500_000, // SGD 5,000/month
//!     allowances: vec![],
//!     working_hours: WorkingHours::standard(),
//!     leave_entitlement: LeaveEntitlement::new(0), // 0 years service = 7 days
//!     cpf_applicable: true,
//!     covered_by_ea: true, // Earning ≤ SGD 4,500
//! };
//! ```

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Employment contract
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmploymentContract {
    /// Employee name
    pub employee_name: String,

    /// Employer name
    pub employer_name: String,

    /// Contract type (indefinite, fixed-term, part-time)
    pub contract_type: ContractType,

    /// Contract start date
    pub start_date: DateTime<Utc>,

    /// Contract end date (None for indefinite contracts)
    pub end_date: Option<DateTime<Utc>>,

    /// Basic monthly salary in cents (SGD)
    pub basic_salary_cents: u64,

    /// Allowances (transport, meal, housing, etc.)
    pub allowances: Vec<Allowance>,

    /// Working hours arrangement
    pub working_hours: WorkingHours,

    /// Leave entitlement
    pub leave_entitlement: LeaveEntitlement,

    /// Whether CPF contributions apply (citizens/PRs only)
    pub cpf_applicable: bool,

    /// Whether covered by Employment Act
    ///
    /// EA covers:
    /// - Workmen earning ≤ SGD 4,500/month
    /// - Non-workmen earning ≤ SGD 2,600/month
    pub covered_by_ea: bool,
}

impl EmploymentContract {
    /// Returns total monthly salary (basic + allowances) in cents
    pub fn total_monthly_salary_cents(&self) -> u64 {
        let allowances_total: u64 = self.allowances.iter().map(|a| a.amount_cents).sum();
        self.basic_salary_cents + allowances_total
    }

    /// Returns total monthly salary in SGD
    pub fn total_monthly_salary_sgd(&self) -> f64 {
        self.total_monthly_salary_cents() as f64 / 100.0
    }

    /// Returns basic salary in SGD
    pub fn basic_salary_sgd(&self) -> f64 {
        self.basic_salary_cents as f64 / 100.0
    }

    /// Returns whether employee qualifies for EA coverage
    pub fn is_covered_by_ea(&self) -> bool {
        self.covered_by_ea
    }

    /// Returns years of service from start date
    pub fn years_of_service(&self) -> u32 {
        let now = Utc::now();
        let duration = now.signed_duration_since(self.start_date);
        (duration.num_days() / 365) as u32
    }
}

/// Contract type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContractType {
    /// Indefinite/permanent contract
    Indefinite,

    /// Fixed-term contract (with end date)
    FixedTerm,

    /// Part-time contract (< 35 hours/week)
    PartTime,

    /// Temporary/casual contract
    Temporary,

    /// Contract work/freelance
    Contract,
}

/// Allowance (additional compensation beyond basic salary)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Allowance {
    /// Allowance type (e.g., "Transport", "Meal", "Housing")
    pub allowance_type: String,

    /// Amount in cents per month
    pub amount_cents: u64,

    /// Whether allowance is part of Ordinary Wage (for CPF)
    pub is_ordinary_wage: bool,
}

impl Allowance {
    /// Creates a new allowance
    pub fn new(
        allowance_type: impl Into<String>,
        amount_cents: u64,
        is_ordinary_wage: bool,
    ) -> Self {
        Self {
            allowance_type: allowance_type.into(),
            amount_cents,
            is_ordinary_wage,
        }
    }

    /// Returns amount in SGD
    pub fn amount_sgd(&self) -> f64 {
        self.amount_cents as f64 / 100.0
    }
}

/// Working hours arrangement
///
/// ## Section 38: Hours of Work
///
/// - Max 44 hours/week for non-shift workers
/// - Max 48 hours/week for shift workers
/// - Max 12 hours/day (inclusive of overtime)
/// - Overtime paid at 1.5x for first 2 hours on normal day, 2x thereafter
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkingHours {
    /// Normal hours per day
    pub hours_per_day: f64,

    /// Normal hours per week
    pub hours_per_week: f64,

    /// Whether shift work arrangement
    pub is_shift_work: bool,

    /// Number of rest days per week (minimum 1)
    pub rest_days_per_week: u32,

    /// Whether eligible for overtime pay
    pub overtime_eligible: bool,

    /// Working days per week
    pub working_days_per_week: u32,
}

impl WorkingHours {
    /// Creates standard working hours (8h/day, 44h/week, 5-day week)
    pub fn standard() -> Self {
        Self {
            hours_per_day: 8.0,
            hours_per_week: 44.0,
            is_shift_work: false,
            rest_days_per_week: 2, // Weekend
            overtime_eligible: true,
            working_days_per_week: 5,
        }
    }

    /// Creates shift work hours (max 48h/week)
    pub fn shift_work(hours_per_week: f64) -> Self {
        Self {
            hours_per_day: hours_per_week / 6.0, // 6-day work week typical for shift
            hours_per_week,
            is_shift_work: true,
            rest_days_per_week: 1,
            overtime_eligible: true,
            working_days_per_week: 6,
        }
    }

    /// Returns maximum weekly hours allowed (44 for non-shift, 48 for shift)
    pub fn max_weekly_hours(&self) -> f64 {
        if self.is_shift_work { 48.0 } else { 44.0 }
    }

    /// Calculates overtime hours per week (if over standard)
    pub fn overtime_hours(&self) -> f64 {
        let max = self.max_weekly_hours();
        if self.hours_per_week > max {
            self.hours_per_week - max
        } else {
            0.0
        }
    }
}

/// Leave entitlement
///
/// ## Sections 43, 89: Annual and Sick Leave
///
/// **Annual Leave** (s. 43):
/// - Year 1: 7 days
/// - Year 2: 8 days
/// - Year 3-4: 9 days
/// - Year 5-6: 11 days
/// - Year 7-8: 12 days
/// - Year 8+: 14 days
///
/// **Sick Leave** (s. 89):
/// - 14 days outpatient
/// - 60 days hospitalization (after 3 months service)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeaveEntitlement {
    /// Annual leave days per year
    pub annual_leave_days: u32,

    /// Sick leave - outpatient days per year
    pub sick_leave_outpatient_days: u32,

    /// Sick leave - hospitalization days per year
    pub sick_leave_hospitalization_days: u32,

    /// Maternity leave weeks (16 weeks for citizens)
    pub maternity_leave_weeks: Option<u32>,

    /// Paternity leave days (2 weeks for citizens)
    pub paternity_leave_days: Option<u32>,

    /// Childcare leave days per year (6 days for citizens)
    pub childcare_leave_days: Option<u32>,

    /// Shared parental leave days (citizens only)
    pub shared_parental_leave_days: Option<u32>,
}

impl LeaveEntitlement {
    /// Creates leave entitlement based on years of service (s. 43)
    pub fn new(years_of_service: u32) -> Self {
        let annual_leave_days = match years_of_service {
            0 => 7,
            1 => 8,
            2..=3 => 9,
            4..=5 => 11,
            6..=7 => 12,
            _ => 14, // 8+ years
        };

        Self {
            annual_leave_days,
            sick_leave_outpatient_days: 14,
            sick_leave_hospitalization_days: 60,
            maternity_leave_weeks: None,
            paternity_leave_days: None,
            childcare_leave_days: None,
            shared_parental_leave_days: None,
        }
    }

    /// Adds maternity leave (16 weeks for citizens)
    pub fn with_maternity_leave(mut self, weeks: u32) -> Self {
        self.maternity_leave_weeks = Some(weeks);
        self
    }

    /// Adds paternity leave (2 weeks for citizens)
    pub fn with_paternity_leave(mut self, days: u32) -> Self {
        self.paternity_leave_days = Some(days);
        self
    }

    /// Adds childcare leave (6 days/year for citizens)
    pub fn with_childcare_leave(mut self, days: u32) -> Self {
        self.childcare_leave_days = Some(days);
        self
    }
}

/// Leave type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LeaveType {
    /// Annual leave (s. 43)
    Annual,

    /// Sick leave - outpatient (s. 89)
    SickOutpatient,

    /// Sick leave - hospitalization (s. 89)
    SickHospitalization,

    /// Maternity leave (16 weeks)
    Maternity,

    /// Paternity leave (2 weeks)
    Paternity,

    /// Childcare leave (6 days/year)
    Childcare,

    /// Shared parental leave
    SharedParental,

    /// Unpaid leave
    Unpaid,
}

/// CPF (Central Provident Fund) contribution
///
/// ## CPF Contribution Rates (2024)
///
/// | Age | Employer | Employee | Total |
/// |-----|----------|----------|-------|
/// | ≤55 | 17%      | 20%      | 37%   |
/// | 55-60 | 15.5%  | 15%      | 30.5% |
/// | 60-65 | 11.5%  | 9.5%     | 21%   |
/// | 65-70 | 9%     | 7.5%     | 16.5% |
/// | >70 | 7.5%     | 5%       | 12.5% |
///
/// **Wage Ceiling**: SGD 6,000/month (Ordinary Wage)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CpfContribution {
    /// Employee age
    pub employee_age: u32,

    /// Monthly wage in cents (capped at SGD 6,000)
    pub monthly_wage_cents: u64,

    /// Employer contribution rate (in basis points, e.g., 1700 = 17%)
    pub employer_rate_bps: u32,

    /// Employee contribution rate (in basis points)
    pub employee_rate_bps: u32,

    /// Ordinary wage ceiling in cents (SGD 6,000 = 600,000 cents)
    pub wage_ceiling_cents: u64,
}

impl CpfContribution {
    /// Ordinary wage ceiling (SGD 6,000/month in 2024)
    pub const ORDINARY_WAGE_CEILING_CENTS: u64 = 600_000; // SGD 6,000

    /// Creates CPF contribution with age-based rates
    pub fn new(employee_age: u32, monthly_wage_cents: u64) -> Self {
        let (employer_rate_bps, employee_rate_bps) = Self::rates_by_age(employee_age);

        Self {
            employee_age,
            monthly_wage_cents,
            employer_rate_bps,
            employee_rate_bps,
            wage_ceiling_cents: Self::ORDINARY_WAGE_CEILING_CENTS,
        }
    }

    /// Returns CPF contribution rates by age (in basis points)
    pub fn rates_by_age(age: u32) -> (u32, u32) {
        match age {
            0..=55 => (1700, 2000),  // 17%, 20%
            56..=60 => (1550, 1500), // 15.5%, 15%
            61..=65 => (1150, 950),  // 11.5%, 9.5%
            66..=70 => (900, 750),   // 9%, 7.5%
            _ => (750, 500),         // 7.5%, 5%
        }
    }

    /// Returns capped wage subject to CPF
    pub fn cpf_subject_wage_cents(&self) -> u64 {
        self.monthly_wage_cents.min(self.wage_ceiling_cents)
    }

    /// Calculates employer CPF contribution in cents
    pub fn employer_contribution_cents(&self) -> u64 {
        let subject_wage = self.cpf_subject_wage_cents();
        (subject_wage * self.employer_rate_bps as u64) / 10_000
    }

    /// Calculates employee CPF contribution in cents
    pub fn employee_contribution_cents(&self) -> u64 {
        let subject_wage = self.cpf_subject_wage_cents();
        (subject_wage * self.employee_rate_bps as u64) / 10_000
    }

    /// Returns total CPF contribution in cents
    pub fn total_contribution_cents(&self) -> u64 {
        self.employer_contribution_cents() + self.employee_contribution_cents()
    }

    /// Returns employer contribution in SGD
    pub fn employer_contribution_sgd(&self) -> f64 {
        self.employer_contribution_cents() as f64 / 100.0
    }

    /// Returns employee contribution in SGD
    pub fn employee_contribution_sgd(&self) -> f64 {
        self.employee_contribution_cents() as f64 / 100.0
    }

    /// Returns total contribution in SGD
    pub fn total_contribution_sgd(&self) -> f64 {
        self.total_contribution_cents() as f64 / 100.0
    }
}

/// Termination notice period
///
/// ## Section 10/11: Notice of Termination
///
/// | Service Length | Notice Period |
/// |---------------|---------------|
/// | < 26 weeks    | 1 day         |
/// | 26 weeks - 2 years | 1 week   |
/// | 2-5 years     | 2 weeks       |
/// | 5+ years      | 4 weeks       |
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminationNotice {
    /// Party terminating (employer or employee)
    pub terminating_party: TerminatingParty,

    /// Notice period in days
    pub notice_days: u32,

    /// Whether payment in lieu of notice
    pub payment_in_lieu: bool,

    /// Reason for termination (if employer)
    pub reason: Option<String>,

    /// Notice date
    pub notice_date: DateTime<Utc>,

    /// Effective termination date
    pub effective_date: DateTime<Utc>,
}

impl TerminationNotice {
    /// Calculates required notice period based on service length (s. 10/11)
    pub fn required_notice_days(service_weeks: u32) -> u32 {
        match service_weeks {
            0..=25 => 1,     // < 26 weeks = 1 day
            26..=103 => 7,   // 26 weeks - 2 years = 1 week
            104..=259 => 14, // 2-5 years = 2 weeks
            _ => 28,         // 5+ years = 4 weeks
        }
    }

    /// Returns required notice in weeks
    pub fn required_notice_weeks(service_weeks: u32) -> u32 {
        Self::required_notice_days(service_weeks) / 7
    }
}

/// Party terminating employment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TerminatingParty {
    /// Employer terminates employment
    Employer,

    /// Employee resigns
    Employee,

    /// Mutual agreement
    Mutual,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpf_rates_by_age() {
        let (emp, ee) = CpfContribution::rates_by_age(30);
        assert_eq!(emp, 1700); // 17%
        assert_eq!(ee, 2000); // 20%

        let (emp2, ee2) = CpfContribution::rates_by_age(58);
        assert_eq!(emp2, 1550); // 15.5%
        assert_eq!(ee2, 1500); // 15%
    }

    #[test]
    fn test_cpf_contribution_calculation() {
        let cpf = CpfContribution::new(30, 500_000); // Age 30, SGD 5,000
        assert_eq!(cpf.employer_contribution_sgd(), 850.0); // 17% of 5,000
        assert_eq!(cpf.employee_contribution_sgd(), 1000.0); // 20% of 5,000
        assert_eq!(cpf.total_contribution_sgd(), 1850.0);
    }

    #[test]
    fn test_cpf_wage_ceiling() {
        let cpf = CpfContribution::new(30, 800_000); // Age 30, SGD 8,000 (above ceiling)
        assert_eq!(cpf.cpf_subject_wage_cents(), 600_000); // Capped at SGD 6,000
        assert_eq!(cpf.employer_contribution_sgd(), 1020.0); // 17% of 6,000
    }

    #[test]
    fn test_leave_entitlement_by_years() {
        let leave0 = LeaveEntitlement::new(0);
        assert_eq!(leave0.annual_leave_days, 7);

        let leave5 = LeaveEntitlement::new(5);
        assert_eq!(leave5.annual_leave_days, 11);

        let leave10 = LeaveEntitlement::new(10);
        assert_eq!(leave10.annual_leave_days, 14);
    }

    #[test]
    fn test_termination_notice_calculation() {
        assert_eq!(TerminationNotice::required_notice_days(10), 1); // < 26 weeks
        assert_eq!(TerminationNotice::required_notice_days(50), 7); // 26 weeks - 2 years
        assert_eq!(TerminationNotice::required_notice_days(150), 14); // 2-5 years
        assert_eq!(TerminationNotice::required_notice_days(300), 28); // 5+ years
    }

    #[test]
    fn test_working_hours_standard() {
        let hours = WorkingHours::standard();
        assert_eq!(hours.hours_per_week, 44.0);
        assert_eq!(hours.max_weekly_hours(), 44.0);
        assert!(!hours.is_shift_work);
    }

    #[test]
    fn test_working_hours_overtime() {
        let mut hours = WorkingHours::standard();
        hours.hours_per_week = 50.0;
        assert_eq!(hours.overtime_hours(), 6.0); // 50 - 44 = 6 hours overtime
    }
}
