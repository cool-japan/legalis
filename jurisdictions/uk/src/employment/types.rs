//! Types for UK Employment Law
//!
//! This module provides type-safe representations of UK employment law concepts under:
//! - Employment Rights Act 1996 (ERA 1996)
//! - Working Time Regulations 1998 (WTR 1998)
//! - National Minimum Wage Act 1998 (NMWA 1998)
//!
//! # Key Concepts

#![allow(missing_docs)]
//!
//! ## ERA 1996 Written Particulars (s.1)
//! Employers must provide written particulars of employment within 2 months of start date
//!
//! ## ERA 1996 Notice Periods (s.86)
//! Statutory minimum notice:
//! - Less than 1 month: No notice required
//! - 1 month to 2 years: 1 week
//! - 2+ years: 1 week per year of service (max 12 weeks)
//!
//! ## ERA 1996 Unfair Dismissal (s.98)
//! - Qualifying period: 2 years continuous employment
//! - Fair reasons: Capability, Conduct, Redundancy, Statutory restriction, SOSR
//! - Automatically unfair: Pregnancy, whistleblowing, etc. (no qualifying period)
//!
//! ## ERA 1996 Redundancy (s.162)
//! Age-based multipliers:
//! - Under 22: 0.5 week's pay per year
//! - 22-40: 1.0 week's pay per year
//! - 41+: 1.5 weeks' pay per year
//! - Maximum: 20 years counted, £700/week cap (April 2024)
//!
//! ## WTR 1998 (48-hour week)
//! - Maximum 48 hours per week (averaged over 17 weeks)
//! - Can opt out in writing
//! - 20-minute rest break if working 6+ hours
//! - 5.6 weeks annual leave (28 days for 5-day week)
//!
//! ## NMWA 1998 (Age-based rates as of April 2024)
//! - 21+: £11.44/hour (National Living Wage)
//! - 18-20: £8.60/hour
//! - Under 18: £6.40/hour
//! - Apprentice: £6.40/hour

use chrono::{Datelike, Duration, NaiveDate};
use serde::{Deserialize, Serialize};

/// Employment contract under ERA 1996
///
/// # ERA 1996 s.1 Written Particulars
/// Must include: employee/employer names, start date, pay, hours, holidays, notice periods
///
/// # Example
/// ```ignore
/// let contract = EmploymentContract {
///     employee: Employee { /* ... */ },
///     employer: Employer { /* ... */ },
///     contract_type: ContractType::Permanent,
///     start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
///     written_particulars_provided: true,
///     // ...
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmploymentContract {
    /// Employee details
    pub employee: Employee,

    /// Employer details
    pub employer: Employer,

    /// Type of contract (Permanent, Fixed-Term, Zero-Hours, Part-Time)
    pub contract_type: ContractType,

    /// Start date of employment
    pub start_date: NaiveDate,

    /// End date (Some for fixed-term, None for permanent)
    pub end_date: Option<NaiveDate>,

    /// Probation period in months (typically 3-6 months)
    pub probation_period_months: Option<u8>,

    /// Salary details
    pub salary: Salary,

    /// Working hours per week
    pub working_hours: WorkingHours,

    /// Job duties/description
    pub duties: String,

    /// Notice period requirements
    pub notice_period: NoticePeriod,

    /// Written particulars provided? (ERA 1996 s.1 - required within 2 months)
    pub written_particulars_provided: bool,

    /// Pension scheme details (auto-enrolment required since 2012)
    pub pension_scheme: Option<PensionScheme>,
}

impl EmploymentContract {
    /// Create a new employment contract builder
    pub fn builder() -> EmploymentContractBuilder {
        EmploymentContractBuilder::default()
    }

    /// Calculate years of continuous service at a given date
    pub fn years_of_service(&self, at_date: NaiveDate) -> u8 {
        let duration = at_date.signed_duration_since(self.start_date);
        (duration.num_days() / 365) as u8
    }

    /// Is the employee still in probation period?
    pub fn in_probation(&self, at_date: NaiveDate) -> bool {
        if let Some(months) = self.probation_period_months {
            let probation_end = self.start_date + Duration::days((months as i64) * 30);
            at_date < probation_end
        } else {
            false
        }
    }
}

impl Default for EmploymentContract {
    fn default() -> Self {
        Self {
            employee: Employee::default(),
            employer: Employer::default(),
            contract_type: ContractType::Permanent,
            start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            end_date: None,
            probation_period_months: None,
            salary: Salary::default(),
            working_hours: WorkingHours::default(),
            duties: String::new(),
            notice_period: NoticePeriod::default(),
            written_particulars_provided: false,
            pension_scheme: None,
        }
    }
}

/// Builder for EmploymentContract
#[derive(Debug, Clone, Default)]
pub struct EmploymentContractBuilder {
    contract: EmploymentContract,
}

impl EmploymentContractBuilder {
    pub fn with_employee(mut self, employee: Employee) -> Self {
        self.contract.employee = employee;
        self
    }

    pub fn with_employer(mut self, employer: Employer) -> Self {
        self.contract.employer = employer;
        self
    }

    pub fn with_contract_type(mut self, contract_type: ContractType) -> Self {
        self.contract.contract_type = contract_type;
        self
    }

    pub fn with_start_date(mut self, start_date: NaiveDate) -> Self {
        self.contract.start_date = start_date;
        self
    }

    pub fn with_end_date(mut self, end_date: NaiveDate) -> Self {
        self.contract.end_date = Some(end_date);
        self
    }

    pub fn with_probation_period_months(mut self, months: u8) -> Self {
        self.contract.probation_period_months = Some(months);
        self
    }

    pub fn with_salary(mut self, salary: Salary) -> Self {
        self.contract.salary = salary;
        self
    }

    pub fn with_working_hours(mut self, working_hours: WorkingHours) -> Self {
        self.contract.working_hours = working_hours;
        self
    }

    pub fn with_duties(mut self, duties: impl Into<String>) -> Self {
        self.contract.duties = duties.into();
        self
    }

    pub fn with_notice_period(mut self, notice_period: NoticePeriod) -> Self {
        self.contract.notice_period = notice_period;
        self
    }

    pub fn with_written_particulars(mut self, provided: bool) -> Self {
        self.contract.written_particulars_provided = provided;
        self
    }

    pub fn with_pension_scheme(mut self, pension_scheme: PensionScheme) -> Self {
        self.contract.pension_scheme = Some(pension_scheme);
        self
    }

    pub fn build(self) -> EmploymentContract {
        self.contract
    }
}

/// Employee details
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Employee {
    /// Full name
    pub name: String,

    /// Date of birth (for redundancy/minimum wage calculations)
    pub date_of_birth: NaiveDate,

    /// Address
    pub address: String,

    /// National Insurance number
    pub national_insurance_number: Option<String>,
}

impl Employee {
    /// Calculate age at a specific date
    pub fn age_at(&self, date: NaiveDate) -> u8 {
        let years = date.year() - self.date_of_birth.year();
        if date.month() < self.date_of_birth.month()
            || (date.month() == self.date_of_birth.month() && date.day() < self.date_of_birth.day())
        {
            (years - 1) as u8
        } else {
            years as u8
        }
    }
}

/// Employer details
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Employer {
    /// Company/organization name
    pub name: String,

    /// Registered address
    pub address: String,

    /// Number of employees (for certain thresholds)
    pub employee_count: Option<u32>,
}

/// Type of employment contract
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractType {
    /// Permanent/unlimited contract (most common)
    Permanent,

    /// Fixed-term contract
    /// Fixed-Term Employees (Prevention of Less Favourable Treatment) Regulations 2002
    FixedTerm {
        /// Reason for fixed-term
        reason: FixedTermReason,

        /// Treated less favourably than comparable permanent employee?
        less_favourable: bool,
    },

    /// Zero-hours contract
    /// Exclusivity clauses banned since 2015
    ZeroHours {
        /// Has illegal exclusivity clause? (banned since 2015)
        exclusivity_clause: bool,
    },

    /// Part-time contract
    /// Part-Time Workers (Prevention of Less Favourable Treatment) Regulations 2000
    PartTime {
        /// Hours per week
        hours_per_week: u8,

        /// Treated less favourably than comparable full-time employee?
        less_favourable: bool,
    },
}

/// Reason for fixed-term contract
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FixedTermReason {
    /// Temporary project/need
    TemporaryProject,

    /// Covering maternity/sick leave
    CoverAbsence,

    /// Seasonal work
    Seasonal,

    /// Probationary period
    Probation,

    /// Other specific reason
    Other,
}

/// Salary details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Salary {
    /// Gross annual salary in GBP
    pub gross_annual_gbp: f64,

    /// Payment frequency
    pub payment_frequency: PaymentFrequency,

    /// Payment day of month/week
    pub payment_day: u8,
}

impl Salary {
    /// Calculate gross monthly salary
    pub fn gross_monthly(&self) -> f64 {
        match self.payment_frequency {
            PaymentFrequency::Monthly => self.gross_annual_gbp / 12.0,
            PaymentFrequency::Fortnightly => (self.gross_annual_gbp / 52.0) * 2.0,
            PaymentFrequency::Weekly => self.gross_annual_gbp / 52.0,
        }
    }

    /// Calculate gross hourly rate
    pub fn gross_hourly(&self, hours_per_week: u8) -> f64 {
        self.gross_annual_gbp / (52.0 * hours_per_week as f64)
    }
}

/// Payment frequency
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PaymentFrequency {
    /// Monthly (most common)
    #[default]
    Monthly,

    /// Fortnightly (every 2 weeks)
    Fortnightly,

    /// Weekly
    Weekly,
}

/// Working hours under WTR 1998
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct WorkingHours {
    /// Contracted hours per week
    pub hours_per_week: u8,

    /// Days per week
    pub days_per_week: u8,

    /// Opted out of 48-hour limit? (WTR Reg 4)
    pub opted_out_of_48h_limit: bool,

    /// Night work hours (if applicable)
    pub night_work_hours: Option<u8>,
}

impl WorkingHours {
    /// Does this comply with 48-hour week limit?
    pub fn complies_with_48h_limit(&self) -> bool {
        self.hours_per_week <= 48 || self.opted_out_of_48h_limit
    }

    /// Entitled to 20-minute break? (WTR Reg 12 - if working 6+ hours)
    pub fn entitled_to_20min_break(&self, daily_hours: u8) -> bool {
        daily_hours >= 6
    }
}

/// Notice period under ERA 1996 s.86
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct NoticePeriod {
    /// Notice period in weeks given by employer
    pub employer_notice_weeks: u8,

    /// Notice period in weeks given by employee
    pub employee_notice_weeks: u8,
}

impl NoticePeriod {
    /// Calculate statutory minimum notice for employer (ERA 1996 s.86)
    ///
    /// - Less than 1 month service: None
    /// - 1 month to 2 years: 1 week
    /// - 2+ years: 1 week per year (max 12 weeks)
    pub fn statutory_minimum_employer(years_service: u8) -> u8 {
        match years_service {
            0 => 0,
            1 => 1,
            2..=11 => years_service,
            _ => 12, // Maximum 12 weeks
        }
    }

    /// Calculate statutory minimum notice for employee (ERA 1996 s.86)
    ///
    /// Employee must give at least 1 week notice (if 1+ month service)
    pub fn statutory_minimum_employee(years_service: u8) -> u8 {
        if years_service >= 1 { 1 } else { 0 }
    }
}

/// Pension scheme (auto-enrolment since 2012)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PensionScheme {
    /// Scheme name
    pub scheme_name: String,

    /// Employee contribution percentage
    pub employee_contribution_pct: f64,

    /// Employer contribution percentage (minimum 3%)
    pub employer_contribution_pct: f64,

    /// Auto-enrolled?
    pub auto_enrolled: bool,
}

/// Dismissal details under ERA 1996 s.98
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dismissal {
    /// Type of dismissal
    pub dismissal_type: DismissalType,

    /// Reason for dismissal (ERA 1996 s.98 fair reasons)
    pub reason: DismissalReason,

    /// Years of continuous service
    pub years_of_service: u8,

    /// Date of dismissal
    pub dismissal_date: NaiveDate,

    /// Written reasons provided? (ERA 1996 s.92 - must provide if requested)
    pub written_reasons_provided: bool,

    /// Notice period given (in weeks)
    pub notice_given_weeks: Option<u8>,
}

impl Dismissal {
    /// Is employee protected from unfair dismissal?
    /// (Requires 2 years continuous service, unless automatically unfair)
    pub fn has_unfair_dismissal_protection(&self) -> bool {
        self.years_of_service >= 2 || self.reason.is_automatically_unfair()
    }
}

/// Type of dismissal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DismissalType {
    /// Ordinary dismissal with notice
    Ordinary,

    /// Summary dismissal (without notice - for gross misconduct)
    Summary,

    /// Constructive dismissal (employee resigns due to employer breach)
    Constructive,
}

/// Reason for dismissal under ERA 1996 s.98
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DismissalReason {
    /// Capability or qualifications (s.98(2)(a))
    Capability {
        /// Description of capability issue
        description: String,

        /// Performance warnings given?
        warnings_given: bool,
    },

    /// Conduct (s.98(2)(b))
    Conduct {
        /// Description of misconduct
        description: String,

        /// Number of warnings given
        warnings_given: u8,

        /// Is gross misconduct? (immediate dismissal)
        gross_misconduct: bool,
    },

    /// Redundancy (s.98(2)(c))
    Redundancy {
        /// Description of redundancy situation
        description: String,

        /// Fair selection process followed?
        fair_selection: bool,

        /// Consultation carried out?
        consultation: bool,
    },

    /// Statutory restriction (s.98(2)(d))
    /// e.g., loss of driving license for driver
    StatutoryRestriction {
        /// Description of restriction
        description: String,
    },

    /// Some Other Substantial Reason (s.98(1)(b))
    SomeOtherSubstantialReason {
        /// Description of reason
        description: String,
    },

    /// Automatically unfair reasons (no qualifying period required)
    AutomaticallyUnfair {
        /// Reason
        reason: AutomaticallyUnfairReason,
    },
}

impl DismissalReason {
    /// Is this an automatically unfair reason? (no qualifying period required)
    pub fn is_automatically_unfair(&self) -> bool {
        matches!(self, DismissalReason::AutomaticallyUnfair { .. })
    }
}

/// Automatically unfair dismissal reasons (no 2-year qualifying period)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AutomaticallyUnfairReason {
    /// Pregnancy or maternity-related
    Pregnancy,

    /// Trade union membership or activities
    TradeUnion,

    /// Whistleblowing (protected disclosure)
    Whistleblowing,

    /// Asserting statutory right
    AssertingStatutoryRight,

    /// Health and safety complaint
    HealthAndSafety,

    /// Requesting flexible working
    FlexibleWorking,

    /// Discrimination (protected characteristics)
    Discrimination,
}

/// Redundancy payment calculation under ERA 1996 s.162
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RedundancyPayment {
    /// Employee age at redundancy date
    pub age: u8,

    /// Years of continuous service (max 20 counted)
    pub years_of_service: u8,

    /// Weekly pay in GBP (capped at £700 as of April 2024)
    pub weekly_pay_gbp: f64,
}

impl RedundancyPayment {
    /// Calculate statutory redundancy payment (ERA 1996 s.162)
    ///
    /// Age-based multipliers:
    /// - Under 22: 0.5 week's pay per year
    /// - 22-40: 1.0 week's pay per year
    /// - 41+: 1.5 weeks' pay per year
    ///
    /// Maximum: 20 years counted, £700/week cap (April 2024)
    pub fn calculate_statutory_payment(&self) -> f64 {
        // Cap weekly pay at £700
        let capped_weekly_pay = self.weekly_pay_gbp.min(700.0);

        // Cap years at 20
        let capped_years = self.years_of_service.min(20) as f64;

        // Age-based multiplier
        let multiplier = if self.age < 22 {
            0.5
        } else if self.age <= 40 {
            1.0
        } else {
            1.5
        };

        capped_weekly_pay * capped_years * multiplier
    }
}

/// Minimum wage assessment under NMWA 1998
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MinimumWageAssessment {
    /// Employee age
    pub age: u8,

    /// Hourly rate in GBP
    pub hourly_rate_gbp: f64,

    /// Is apprentice? (first year of apprenticeship)
    pub apprentice: bool,
}

impl MinimumWageAssessment {
    /// Minimum wage rates as of April 2024
    pub const NATIONAL_LIVING_WAGE_21_PLUS: f64 = 11.44;
    pub const NMW_18_TO_20: f64 = 8.60;
    pub const NMW_UNDER_18: f64 = 6.40;
    pub const APPRENTICE_RATE: f64 = 6.40;

    /// Get applicable minimum wage rate
    pub fn applicable_minimum_wage(&self) -> f64 {
        if self.apprentice {
            Self::APPRENTICE_RATE
        } else if self.age >= 21 {
            Self::NATIONAL_LIVING_WAGE_21_PLUS
        } else if self.age >= 18 {
            Self::NMW_18_TO_20
        } else {
            Self::NMW_UNDER_18
        }
    }

    /// Is hourly rate compliant with minimum wage?
    pub fn is_compliant(&self) -> bool {
        self.hourly_rate_gbp >= self.applicable_minimum_wage()
    }
}

/// Annual leave entitlement under WTR 1998 Reg 13
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnnualLeaveEntitlement {
    /// Days worked per week
    pub days_per_week: u8,

    /// Start date of leave year
    pub leave_year_start: NaiveDate,
}

impl AnnualLeaveEntitlement {
    /// Calculate statutory minimum annual leave (WTR 1998 Reg 13)
    ///
    /// 5.6 weeks per year
    /// = 28 days for 5-day week
    /// = 22.4 days for 4-day week
    pub fn statutory_minimum_days(&self) -> f64 {
        5.6 * self.days_per_week as f64
    }
}

/// Rest entitlement under WTR 1998
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestEntitlement {
    /// Daily working hours
    pub daily_hours: u8,

    /// Days worked per week
    pub days_per_week: u8,
}

impl RestEntitlement {
    /// Rest break entitlement (WTR Reg 12)
    ///
    /// 20 minutes if working 6+ hours per day
    pub fn rest_break_minutes(&self) -> u8 {
        if self.daily_hours >= 6 { 20 } else { 0 }
    }

    /// Daily rest entitlement (WTR Reg 10)
    ///
    /// 11 consecutive hours between working days
    pub fn daily_rest_hours(&self) -> u8 {
        11
    }

    /// Weekly rest entitlement (WTR Reg 11)
    ///
    /// 24 hours (or 48 hours per fortnight)
    pub fn weekly_rest_hours(&self) -> u8 {
        24
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statutory_notice_period() {
        assert_eq!(NoticePeriod::statutory_minimum_employer(0), 0);
        assert_eq!(NoticePeriod::statutory_minimum_employer(1), 1);
        assert_eq!(NoticePeriod::statutory_minimum_employer(2), 2);
        assert_eq!(NoticePeriod::statutory_minimum_employer(5), 5);
        assert_eq!(NoticePeriod::statutory_minimum_employer(12), 12);
        assert_eq!(NoticePeriod::statutory_minimum_employer(20), 12); // Capped at 12
    }

    #[test]
    fn test_redundancy_payment_under_22() {
        let payment = RedundancyPayment {
            age: 21,
            years_of_service: 3,
            weekly_pay_gbp: 600.0,
        };
        // 3 years × 0.5 × £600 = £900
        assert_eq!(payment.calculate_statutory_payment(), 900.0);
    }

    #[test]
    fn test_redundancy_payment_22_to_40() {
        let payment = RedundancyPayment {
            age: 30,
            years_of_service: 8,
            weekly_pay_gbp: 650.0,
        };
        // 8 years × 1.0 × £650 = £5,200
        assert_eq!(payment.calculate_statutory_payment(), 5200.0);
    }

    #[test]
    fn test_redundancy_payment_41_plus() {
        let payment = RedundancyPayment {
            age: 45,
            years_of_service: 10,
            weekly_pay_gbp: 800.0, // Above £700 cap
        };
        // 10 years × 1.5 × £700 (capped) = £10,500
        assert_eq!(payment.calculate_statutory_payment(), 10500.0);
    }

    #[test]
    fn test_minimum_wage_national_living_wage() {
        let assessment = MinimumWageAssessment {
            age: 25,
            hourly_rate_gbp: 11.50,
            apprentice: false,
        };
        assert_eq!(assessment.applicable_minimum_wage(), 11.44);
        assert!(assessment.is_compliant());
    }

    #[test]
    fn test_minimum_wage_below_minimum() {
        let assessment = MinimumWageAssessment {
            age: 19,
            hourly_rate_gbp: 8.00,
            apprentice: false,
        };
        assert_eq!(assessment.applicable_minimum_wage(), 8.60);
        assert!(!assessment.is_compliant());
    }

    #[test]
    fn test_annual_leave_5_day_week() {
        let entitlement = AnnualLeaveEntitlement {
            days_per_week: 5,
            leave_year_start: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        };
        // 5.6 × 5 = 28 days
        assert_eq!(entitlement.statutory_minimum_days(), 28.0);
    }

    #[test]
    fn test_working_hours_48h_limit_compliant() {
        let hours = WorkingHours {
            hours_per_week: 40,
            days_per_week: 5,
            opted_out_of_48h_limit: false,
            night_work_hours: None,
        };
        assert!(hours.complies_with_48h_limit());
    }

    #[test]
    fn test_working_hours_48h_limit_with_opt_out() {
        let hours = WorkingHours {
            hours_per_week: 55,
            days_per_week: 6,
            opted_out_of_48h_limit: true,
            night_work_hours: None,
        };
        assert!(hours.complies_with_48h_limit());
    }
}
