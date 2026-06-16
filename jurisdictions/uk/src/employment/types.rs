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
            start_date: NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date constant"),
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

/// Statutory cap on a "week's pay" for redundancy and basic-award purposes (ERA 1996 s.227).
///
/// £700 from 6 April 2024 (Employment Rights (Increase of Limits) Order 2023, SI 2023/1191).
pub const WEEKLY_PAY_CAP_GBP: f64 = 700.0;

/// Maximum number of years of continuous service that may be reckoned.
///
/// Service in excess of 20 years is disregarded for the statutory redundancy payment
/// (ERA 1996 s.162(3)) and, by extension, for the unfair-dismissal basic award
/// (ERA 1996 s.119(2)).
pub const MAX_RECKONABLE_YEARS: u8 = 20;

/// Age band of the lower threshold (22) for the statutory reckoning (ERA 1996 s.162(2)(b)).
const REDUNDANCY_LOWER_AGE_BAND: u8 = 22;

/// Age band of the upper threshold (41) for the statutory reckoning (ERA 1996 s.162(2)(a)).
const REDUNDANCY_UPPER_AGE_BAND: u8 = 41;

/// Age-banded breakdown of reckonable service under ERA 1996 s.162(2).
///
/// The statute requires the period of continuous employment to be reckoned **backwards**
/// from the end of employment, allowing an appropriate number of weeks' pay for each
/// complete year according to the employee's age during that year. This same reckoning is
/// applied to the unfair-dismissal basic award by ERA 1996 s.119.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceReckoning {
    /// Complete years reckoned at 1.5 weeks' pay (employee aged 41 or over) — s.162(2)(a).
    pub years_at_one_and_half: u8,

    /// Complete years reckoned at 1 week's pay (aged 22 to 40 inclusive) — s.162(2)(b).
    pub years_at_one: u8,

    /// Complete years reckoned at 0.5 week's pay (aged under 22) — s.162(2)(c).
    pub years_at_half: u8,
}

impl ServiceReckoning {
    /// Reckon service backwards from the end of employment (ERA 1996 s.162(1)-(3)).
    ///
    /// For each complete year, reckoning backwards from the dismissal date, the age band is
    /// fixed by the employee's age at the start of that year (so a year only attracts the
    /// 1.5-week rate where the employee was 41 or over for the whole of it). Service beyond
    /// [`MAX_RECKONABLE_YEARS`] is disregarded (s.162(3)).
    ///
    /// This is the correct statutory method: applying a single multiplier based on the age at
    /// the dismissal date to every year would over-state the entitlement of any employee who
    /// crossed an age band during their employment.
    pub fn reckon(age_at_dismissal: u8, complete_years: u8) -> Self {
        let reckonable_years = complete_years.min(MAX_RECKONABLE_YEARS);
        let mut years_at_one_and_half = 0u8;
        let mut years_at_one = 0u8;
        let mut years_at_half = 0u8;

        for year_back in 1..=reckonable_years {
            let age_during_year = age_at_dismissal.saturating_sub(year_back);
            if age_during_year >= REDUNDANCY_UPPER_AGE_BAND {
                years_at_one_and_half += 1;
            } else if age_during_year >= REDUNDANCY_LOWER_AGE_BAND {
                years_at_one += 1;
            } else {
                years_at_half += 1;
            }
        }

        Self {
            years_at_one_and_half,
            years_at_one,
            years_at_half,
        }
    }

    /// Total number of weeks' pay due for the reckoned service (ERA 1996 s.162(2)).
    pub fn weeks_due(&self) -> f64 {
        let upper = f64::from(self.years_at_one_and_half) * 1.5;
        let middle = f64::from(self.years_at_one);
        let lower = f64::from(self.years_at_half) * 0.5;
        upper + middle + lower
    }

    /// Total number of complete years reckoned (capped at [`MAX_RECKONABLE_YEARS`]).
    pub fn total_years(&self) -> u8 {
        self.years_at_one_and_half + self.years_at_one + self.years_at_half
    }
}

/// Number of weeks' pay due for `complete_years` of service ending at `age_at_dismissal`
/// (ERA 1996 s.162). Convenience wrapper over [`ServiceReckoning::reckon`].
pub fn statutory_weeks_due(age_at_dismissal: u8, complete_years: u8) -> f64 {
    ServiceReckoning::reckon(age_at_dismissal, complete_years).weeks_due()
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
    /// Age-banded reckoning of the employee's service (ERA 1996 s.162(1)-(3)).
    pub fn reckoning(&self) -> ServiceReckoning {
        ServiceReckoning::reckon(self.age, self.years_of_service)
    }

    /// A "week's pay" subject to the statutory cap (ERA 1996 s.227).
    pub fn capped_weekly_pay(&self) -> f64 {
        self.weekly_pay_gbp.min(WEEKLY_PAY_CAP_GBP)
    }

    /// Calculate the statutory redundancy payment (ERA 1996 s.162).
    ///
    /// The payment is the number of weeks' pay due under the age-banded reckoning
    /// (s.162(2)) multiplied by a week's pay capped at [`WEEKLY_PAY_CAP_GBP`] (s.227):
    /// - 1.5 weeks' pay for each complete year aged 41 or over,
    /// - 1 week's pay for each complete year aged 22 to 40,
    /// - 0.5 week's pay for each complete year aged under 22,
    ///
    /// reckoned backwards from the dismissal date over at most 20 years.
    pub fn calculate_statutory_payment(&self) -> f64 {
        self.reckoning().weeks_due() * self.capped_weekly_pay()
    }
}

/// Basic award for unfair dismissal (ERA 1996 s.119).
///
/// The basic award is calculated in the same way as a statutory redundancy payment: the
/// age-banded reckoning of s.162 applied to a week's pay capped at [`WEEKLY_PAY_CAP_GBP`],
/// over at most 20 years (s.119(2), s.227). It is reduced by any redundancy payment already
/// made in respect of the same dismissal (s.122(4)); that interaction is left to the caller.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasicAward {
    /// Employee age at the effective date of termination.
    pub age: u8,

    /// Complete years of continuous service (max 20 counted).
    pub years_of_service: u8,

    /// Gross weekly pay in GBP (capped at [`WEEKLY_PAY_CAP_GBP`]).
    pub weekly_pay_gbp: f64,
}

impl BasicAward {
    /// Age-banded reckoning of the employee's service (ERA 1996 s.162 as applied by s.119).
    pub fn reckoning(&self) -> ServiceReckoning {
        ServiceReckoning::reckon(self.age, self.years_of_service)
    }

    /// A "week's pay" subject to the statutory cap (ERA 1996 s.227).
    pub fn capped_weekly_pay(&self) -> f64 {
        self.weekly_pay_gbp.min(WEEKLY_PAY_CAP_GBP)
    }

    /// Calculate the basic award (ERA 1996 s.119).
    pub fn calculate(&self) -> f64 {
        self.reckoning().weeks_due() * self.capped_weekly_pay()
    }

    /// The statutory maximum basic award (20 years × 1.5 × the week's-pay cap).
    ///
    /// £21,000 from 6 April 2024.
    pub fn statutory_maximum() -> f64 {
        f64::from(MAX_RECKONABLE_YEARS) * 1.5 * WEEKLY_PAY_CAP_GBP
    }
}

/// Compensatory award for unfair dismissal (ERA 1996 ss.123-124).
///
/// The tribunal first assesses the claimant's financial loss flowing from the dismissal
/// (s.123). The award is then limited to the lower of 52 weeks' gross pay or the statutory
/// maximum (s.124(1), (1ZA)). Unlike the basic award, the 52-week limit uses the claimant's
/// *actual* (uncapped) gross weekly pay.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompensatoryAward {
    /// Tribunal-assessed financial loss attributable to the dismissal (ERA 1996 s.123).
    pub assessed_loss_gbp: f64,

    /// Claimant's actual gross weekly pay (uncapped; used for the 52-week limit).
    pub gross_weekly_pay_gbp: f64,
}

impl CompensatoryAward {
    /// Statutory maximum compensatory award: £115,115 from 6 April 2024 (ERA 1996 s.124(1)).
    pub const STATUTORY_MAXIMUM_GBP: f64 = 115_115.0;

    /// Alternative limit of 52 weeks' gross pay (ERA 1996 s.124(1ZA)).
    pub const WEEKS_LIMIT: f64 = 52.0;

    /// The applicable statutory cap: the lower of 52 weeks' pay or the statutory maximum.
    pub fn statutory_cap(&self) -> f64 {
        (Self::WEEKS_LIMIT * self.gross_weekly_pay_gbp).min(Self::STATUTORY_MAXIMUM_GBP)
    }

    /// The compensatory award actually payable: the assessed loss, limited by the cap.
    pub fn award(&self) -> f64 {
        self.assessed_loss_gbp.min(self.statutory_cap())
    }
}

/// Combined monetary award for unfair dismissal (ERA 1996 ss.118-124).
///
/// Comprises the basic award (s.119) and the compensatory award (s.124).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnfairDismissalAward {
    /// Basic award component (ERA 1996 s.119).
    pub basic_award: BasicAward,

    /// Compensatory award component (ERA 1996 s.124).
    pub compensatory_award: CompensatoryAward,
}

impl UnfairDismissalAward {
    /// The basic award payable (ERA 1996 s.119).
    pub fn basic(&self) -> f64 {
        self.basic_award.calculate()
    }

    /// The compensatory award payable (ERA 1996 s.124).
    pub fn compensatory(&self) -> f64 {
        self.compensatory_award.award()
    }

    /// Total compensation: basic award plus compensatory award.
    pub fn total(&self) -> f64 {
        self.basic() + self.compensatory()
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
        // ERA 1996 s.162: reckoning backwards from 45 over 10 years covers ages 35-44.
        // Years aged 41+ (44,43,42,41): 4 × 1.5 = 6.0 weeks.
        // Years aged 22-40 (40..35):   6 × 1.0 = 6.0 weeks.
        // Total 12 weeks × £700 (capped) = £8,400.
        assert_eq!(payment.calculate_statutory_payment(), 8400.0);
    }

    #[test]
    fn test_service_reckoning_band_crossing() {
        // Employee aged 45 with 10 complete years started at 35: only 4 years at the 41+ rate.
        let reckoning = ServiceReckoning::reckon(45, 10);
        assert_eq!(reckoning.years_at_one_and_half, 4);
        assert_eq!(reckoning.years_at_one, 6);
        assert_eq!(reckoning.years_at_half, 0);
        assert_eq!(reckoning.total_years(), 10);
        assert_eq!(reckoning.weeks_due(), 12.0);
    }

    #[test]
    fn test_service_reckoning_crossing_22() {
        // Aged 23 with 5 years: the year ending at 22 is at 1.0; the earlier 4 are at 0.5.
        let reckoning = ServiceReckoning::reckon(23, 5);
        assert_eq!(reckoning.years_at_one_and_half, 0);
        assert_eq!(reckoning.years_at_one, 1);
        assert_eq!(reckoning.years_at_half, 4);
        assert_eq!(reckoning.weeks_due(), 3.0);
    }

    #[test]
    fn test_service_reckoning_caps_at_twenty_years() {
        // Aged 55 with 25 years: only the most recent 20 are reckoned (ages 35-54).
        let reckoning = ServiceReckoning::reckon(55, 25);
        assert_eq!(reckoning.total_years(), 20);
        assert_eq!(reckoning.years_at_one_and_half, 14); // ages 54..41
        assert_eq!(reckoning.years_at_one, 6); // ages 40..35
        assert_eq!(reckoning.years_at_half, 0);
        assert_eq!(reckoning.weeks_due(), 27.0);
    }

    #[test]
    fn test_statutory_weeks_due_helper_matches_reckoning() {
        assert_eq!(statutory_weeks_due(45, 10), 12.0);
        assert_eq!(statutory_weeks_due(21, 3), 1.5);
        assert_eq!(statutory_weeks_due(30, 8), 8.0);
    }

    #[test]
    fn test_basic_award_matches_redundancy_method() {
        // ERA 1996 s.119: basic award uses the same age-banded reckoning as redundancy.
        let award = BasicAward {
            age: 45,
            years_of_service: 10,
            weekly_pay_gbp: 600.0,
        };
        // 12 weeks × £600 = £7,200.
        assert_eq!(award.calculate(), 7200.0);
    }

    #[test]
    fn test_basic_award_statutory_maximum() {
        // 20 years × 1.5 × £700 = £21,000 (April 2024).
        assert_eq!(BasicAward::statutory_maximum(), 21000.0);
    }

    #[test]
    fn test_compensatory_award_capped_by_statutory_maximum() {
        // High earner: 52 weeks' pay exceeds the statutory maximum, so the cap is £115,115.
        let award = CompensatoryAward {
            assessed_loss_gbp: 200_000.0,
            gross_weekly_pay_gbp: 5_000.0,
        };
        assert_eq!(award.statutory_cap(), 115_115.0);
        assert_eq!(award.award(), 115_115.0);
    }

    #[test]
    fn test_compensatory_award_capped_by_fifty_two_weeks() {
        // Lower earner: 52 × £1,000 = £52,000 is below the statutory maximum and binds.
        let award = CompensatoryAward {
            assessed_loss_gbp: 100_000.0,
            gross_weekly_pay_gbp: 1_000.0,
        };
        assert_eq!(award.statutory_cap(), 52_000.0);
        assert_eq!(award.award(), 52_000.0);
    }

    #[test]
    fn test_compensatory_award_below_cap_pays_assessed_loss() {
        let award = CompensatoryAward {
            assessed_loss_gbp: 30_000.0,
            gross_weekly_pay_gbp: 2_000.0,
        };
        // Cap is min(52 × 2,000, 115,115) = 104,000; assessed loss is lower, so it is paid in full.
        assert_eq!(award.award(), 30_000.0);
    }

    #[test]
    fn test_unfair_dismissal_award_total() {
        let award = UnfairDismissalAward {
            basic_award: BasicAward {
                age: 45,
                years_of_service: 10,
                weekly_pay_gbp: 600.0,
            },
            compensatory_award: CompensatoryAward {
                assessed_loss_gbp: 30_000.0,
                gross_weekly_pay_gbp: 2_000.0,
            },
        };
        assert_eq!(award.basic(), 7200.0);
        assert_eq!(award.compensatory(), 30_000.0);
        assert_eq!(award.total(), 37_200.0);
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
