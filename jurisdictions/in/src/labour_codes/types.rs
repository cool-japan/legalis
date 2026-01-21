//! Labour Codes 2020 Types
//!
//! Types for India's four consolidated Labour Codes enacted in 2019-2020

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

// ====================
// Code on Wages, 2019
// ====================

/// Wage definition under Code on Wages (Section 2(y))
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wage {
    /// Basic pay
    pub basic: f64,
    /// Dearness allowance
    pub da: f64,
    /// Retaining allowance
    pub retaining_allowance: f64,
    /// Total wages
    pub total: f64,
    /// Excludes special allowances
    pub exclusions: Vec<WageExclusion>,
}

impl Wage {
    /// Calculate minimum wage compliance
    pub fn is_above_minimum(&self, minimum_wage: f64) -> bool {
        self.basic + self.da >= minimum_wage
    }

    /// Calculate basic as percentage of total (must be >= 50%)
    pub fn basic_percentage(&self) -> f64 {
        if self.total > 0.0 {
            (self.basic + self.da) / self.total * 100.0
        } else {
            0.0
        }
    }
}

/// Wage exclusions (Section 2(y))
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WageExclusion {
    /// House rent allowance
    Hra,
    /// Statutory bonus
    StatutoryBonus,
    /// Overtime
    Overtime,
    /// Conveyance allowance
    Conveyance,
    /// Gratuity
    Gratuity,
    /// Retrenchment compensation
    Retrenchment,
    /// Contribution to PF
    PfContribution,
    /// Encashment of leave
    LeaveEncashment,
    /// Commission
    Commission,
}

/// Minimum wage floor (Section 9)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MinimumWageFloor {
    /// Floor wage (national minimum)
    pub floor_wage: f64,
    /// Geographical area
    pub area: GeographicalArea,
    /// Skill level
    pub skill_level: SkillLevel,
    /// Effective date
    pub effective_date: NaiveDate,
}

/// Geographical area for wage determination
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeographicalArea {
    /// Metropolitan area
    Metropolitan,
    /// Non-metropolitan urban
    Urban,
    /// Rural
    Rural,
}

/// Skill level classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SkillLevel {
    /// Unskilled worker
    Unskilled,
    /// Semi-skilled worker
    SemiSkilled,
    /// Skilled worker
    Skilled,
    /// Highly skilled worker
    HighlySkilled,
}

impl SkillLevel {
    /// Get typical wage multiplier over unskilled
    pub fn wage_multiplier(&self) -> f64 {
        match self {
            Self::Unskilled => 1.0,
            Self::SemiSkilled => 1.15,
            Self::Skilled => 1.30,
            Self::HighlySkilled => 1.50,
        }
    }
}

/// Payment of wages (Section 15-19)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WagePayment {
    /// Payment period
    pub period: PaymentPeriod,
    /// Payment mode
    pub mode: PaymentMode,
    /// Payment date
    pub date: NaiveDate,
    /// Gross amount
    pub gross: f64,
    /// Deductions
    pub deductions: Vec<WageDeduction>,
    /// Net amount
    pub net: f64,
}

/// Payment period
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PaymentPeriod {
    /// Daily wages
    Daily,
    /// Weekly wages
    Weekly,
    /// Fortnightly wages
    Fortnightly,
    /// Monthly wages
    Monthly,
}

impl PaymentPeriod {
    /// Get payment deadline (days after period end)
    pub fn payment_deadline(&self, employee_count: u32) -> u32 {
        match self {
            Self::Daily | Self::Weekly => 2,
            Self::Fortnightly => 4,
            Self::Monthly => {
                if employee_count < 1000 {
                    7
                } else {
                    10
                }
            }
        }
    }
}

/// Payment mode (Section 15)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PaymentMode {
    /// Cash
    Cash,
    /// Cheque
    Cheque,
    /// Bank transfer
    BankTransfer,
    /// Digital payment
    DigitalPayment,
}

/// Wage deduction (Section 18)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WageDeduction {
    /// Deduction type
    pub deduction_type: DeductionType,
    /// Amount
    pub amount: f64,
    /// Section reference
    pub section: String,
}

/// Types of permissible deductions (Section 18)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeductionType {
    /// Fines
    Fine,
    /// Absence from duty
    Absence,
    /// Damage/loss to employer
    DamageLoss,
    /// Services/amenities provided
    Services,
    /// House accommodation
    Accommodation,
    /// Advances/loans
    Advances,
    /// Tax at source
    TaxAtSource,
    /// Court order
    CourtOrder,
    /// Provident Fund
    ProvidentFund,
    /// Insurance premium
    Insurance,
    /// Union dues
    UnionDues,
}

impl DeductionType {
    /// Get maximum deduction percentage
    pub fn max_percentage(&self) -> Option<f64> {
        match self {
            Self::Fine => Some(3.0),
            Self::Accommodation => Some(5.0),
            Self::Services => Some(3.0),
            _ => None, // Variable or no limit
        }
    }
}

/// Bonus under Code on Wages (Chapter III)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bonus {
    /// Accounting year
    pub year: String,
    /// Allocable surplus
    pub allocable_surplus: f64,
    /// Available surplus
    pub available_surplus: f64,
    /// Minimum bonus (8.33%)
    pub minimum: f64,
    /// Maximum bonus (20%)
    pub maximum: f64,
    /// Actual bonus payable
    pub actual: f64,
    /// Set-on from previous year
    pub set_on: f64,
    /// Set-off against future
    pub set_off: f64,
}

impl Bonus {
    /// Calculate minimum bonus (8.33% of salary)
    pub fn calculate_minimum(salary: f64) -> f64 {
        salary * 8.33 / 100.0
    }

    /// Calculate maximum bonus (20% of salary)
    pub fn calculate_maximum(salary: f64) -> f64 {
        salary * 20.0 / 100.0
    }

    /// Check eligibility (>= 30 days worked)
    pub fn is_eligible(days_worked: u32) -> bool {
        days_worked >= 30
    }

    /// Wage ceiling for bonus calculation
    pub fn wage_ceiling() -> f64 {
        21_000.0 // Rs. 21,000 per month
    }
}

// ================================
// Code on Social Security, 2020
// ================================

/// Social security scheme type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SocialSecurityScheme {
    /// Employees' Provident Fund
    Epf,
    /// Employees' Pension Scheme
    Eps,
    /// Employees' Deposit Linked Insurance
    Edli,
    /// Employees' State Insurance
    Esi,
    /// Gratuity
    Gratuity,
    /// Maternity Benefit
    MaternityBenefit,
    /// Building Workers Welfare
    BuildingWorkers,
    /// Unorganized Workers
    UnorganizedWorkers,
    /// Gig Workers
    GigWorkers,
    /// Platform Workers
    PlatformWorkers,
}

/// EPF contribution details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EpfContribution {
    /// Employee contribution (12%)
    pub employee: f64,
    /// Employer contribution (12%)
    pub employer: f64,
    /// Of employer, to EPS (8.33%)
    pub eps_portion: f64,
    /// Of employer, to EPF (3.67%)
    pub epf_portion: f64,
    /// Basic wage for calculation
    pub basic_wage: f64,
    /// Month
    pub month: String,
}

impl EpfContribution {
    /// EPF wage ceiling
    pub fn wage_ceiling() -> f64 {
        15_000.0 // Rs. 15,000 per month
    }

    /// Employee contribution rate
    pub fn employee_rate() -> f64 {
        12.0
    }

    /// Employer contribution rate
    pub fn employer_rate() -> f64 {
        12.0
    }

    /// EPS contribution rate (from employer)
    pub fn eps_rate() -> f64 {
        8.33
    }

    /// Calculate contributions
    pub fn calculate(basic_wage: f64) -> Self {
        let capped_wage = basic_wage.min(Self::wage_ceiling());
        let employee = capped_wage * Self::employee_rate() / 100.0;
        let employer = capped_wage * Self::employer_rate() / 100.0;
        let eps_portion = capped_wage * Self::eps_rate() / 100.0;
        let epf_portion = employer - eps_portion;

        Self {
            employee,
            employer,
            eps_portion,
            epf_portion,
            basic_wage,
            month: String::new(),
        }
    }
}

/// ESI contribution details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EsiContribution {
    /// Employee contribution (0.75%)
    pub employee: f64,
    /// Employer contribution (3.25%)
    pub employer: f64,
    /// Gross wages
    pub gross_wages: f64,
    /// Month
    pub month: String,
}

impl EsiContribution {
    /// ESI wage ceiling
    pub fn wage_ceiling() -> f64 {
        21_000.0 // Rs. 21,000 per month
    }

    /// Employee contribution rate
    pub fn employee_rate() -> f64 {
        0.75
    }

    /// Employer contribution rate
    pub fn employer_rate() -> f64 {
        3.25
    }

    /// Calculate contributions
    pub fn calculate(gross_wages: f64) -> Self {
        let employee = gross_wages * Self::employee_rate() / 100.0;
        let employer = gross_wages * Self::employer_rate() / 100.0;

        Self {
            employee,
            employer,
            gross_wages,
            month: String::new(),
        }
    }
}

/// Gratuity calculation (Section 53)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gratuity {
    /// Years of service
    pub years_of_service: f64,
    /// Last drawn salary (basic + DA)
    pub last_salary: f64,
    /// Gratuity amount
    pub amount: f64,
    /// Taxable amount (above exemption)
    pub taxable_amount: f64,
}

impl Gratuity {
    /// Minimum years for eligibility
    pub fn minimum_years() -> f64 {
        5.0
    }

    /// Maximum gratuity amount
    pub fn maximum_amount() -> f64 {
        2_000_000.0 // Rs. 20 lakhs
    }

    /// Tax exemption limit
    pub fn tax_exemption_limit() -> f64 {
        2_000_000.0 // Rs. 20 lakhs
    }

    /// Calculate gratuity
    pub fn calculate(years: f64, last_salary: f64) -> Self {
        let rounded_years = (years * 2.0).round() / 2.0; // Round to nearest 0.5
        let amount = (last_salary * 15.0 * rounded_years / 26.0).min(Self::maximum_amount());
        let taxable = (amount - Self::tax_exemption_limit()).max(0.0);

        Self {
            years_of_service: years,
            last_salary,
            amount,
            taxable_amount: taxable,
        }
    }

    /// Check eligibility
    pub fn is_eligible(years: f64) -> bool {
        years >= Self::minimum_years()
    }
}

/// Maternity benefit (Section 60-64)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaternityBenefit {
    /// Type of benefit
    pub benefit_type: MaternityBenefitType,
    /// Duration in weeks
    pub weeks: u32,
    /// Daily wage
    pub daily_wage: f64,
    /// Total benefit
    pub total_benefit: f64,
}

/// Maternity benefit type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MaternityBenefitType {
    /// First two children delivery
    NormalDelivery,
    /// Third child onwards
    ThirdChildOnwards,
    /// Miscarriage
    Miscarriage,
    /// Tubectomy operation
    Tubectomy,
    /// Illness from pregnancy
    IllnessFromPregnancy,
    /// Commissioning mother
    CommissioningMother,
    /// Adopting mother
    AdoptingMother,
}

impl MaternityBenefitType {
    /// Get weeks of leave
    pub fn leave_weeks(&self) -> u32 {
        match self {
            Self::NormalDelivery => 26,
            Self::ThirdChildOnwards => 12,
            Self::Miscarriage => 6,
            Self::Tubectomy => 2,
            Self::IllnessFromPregnancy => 4, // Additional
            Self::CommissioningMother | Self::AdoptingMother => 12,
        }
    }
}

// ==================================
// Industrial Relations Code, 2020
// ==================================

/// Worker type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorkerType {
    /// Permanent worker
    Permanent,
    /// Probationer
    Probationer,
    /// Temporary worker
    Temporary,
    /// Casual worker
    Casual,
    /// Contract worker
    Contract,
    /// Apprentice
    Apprentice,
    /// Fixed term employee
    FixedTerm,
}

impl WorkerType {
    /// Check if covered under IR Code
    pub fn covered_under_ir_code(&self) -> bool {
        !matches!(self, Self::Apprentice)
    }
}

/// Trade union registration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradeUnion {
    /// Registration number
    pub registration_number: String,
    /// Name of union
    pub name: String,
    /// Registration date
    pub registration_date: NaiveDate,
    /// Number of members
    pub members: u32,
    /// Establishment covered
    pub establishment: String,
    /// Industry/sector
    pub industry: String,
    /// Is negotiating union
    pub is_negotiating_union: bool,
}

impl TradeUnion {
    /// Minimum members for registration (10% or 100, whichever is less)
    pub fn minimum_members(total_workers: u32) -> u32 {
        (total_workers * 10 / 100).clamp(7, 100)
    }

    /// Threshold for sole negotiating agent (75% membership)
    pub fn sole_negotiating_threshold() -> f64 {
        75.0
    }

    /// Threshold for negotiating council (51% combined)
    pub fn negotiating_council_threshold() -> f64 {
        51.0
    }
}

/// Standing orders
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StandingOrders {
    /// Classification of workers
    pub worker_classification: Vec<WorkerType>,
    /// Working hours
    pub working_hours: WorkingHours,
    /// Leave provisions
    pub leave_provisions: LeaveProvisions,
    /// Disciplinary procedure
    pub disciplinary_procedure: bool,
    /// Grievance redressal
    pub grievance_mechanism: bool,
}

/// Industrial dispute
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndustrialDispute {
    /// Dispute type
    pub dispute_type: DisputeType,
    /// Parties involved
    pub parties: Vec<String>,
    /// Date of dispute
    pub dispute_date: NaiveDate,
    /// Subject matter
    pub subject: String,
    /// Current stage
    pub stage: DisputeStage,
}

/// Type of industrial dispute
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DisputeType {
    /// Individual dispute
    Individual,
    /// Collective dispute
    Collective,
    /// Rights dispute
    Rights,
    /// Interest dispute
    Interest,
    /// Unfair labour practice
    UnfairLabourPractice,
}

/// Dispute resolution stage
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DisputeStage {
    /// Bipartite negotiation
    BipartiteNegotiation,
    /// Conciliation
    Conciliation,
    /// Arbitration
    Arbitration,
    /// Industrial Tribunal
    Tribunal,
    /// Labour Court
    LabourCourt,
    /// National Industrial Tribunal
    NationalTribunal,
    /// Resolved
    Resolved,
}

/// Layoff details (Section 70)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Layoff {
    /// Number of workers laid off
    pub workers_affected: u32,
    /// Start date
    pub start_date: NaiveDate,
    /// Expected end date
    pub expected_end_date: Option<NaiveDate>,
    /// Reason
    pub reason: LayoffReason,
    /// Compensation rate
    pub compensation_rate: f64,
    /// Government permission required
    pub permission_required: bool,
}

/// Reason for layoff
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LayoffReason {
    /// Shortage of raw material
    MaterialShortage,
    /// Power failure
    PowerFailure,
    /// Accumulation of stocks
    StockAccumulation,
    /// Breakdown of machinery
    MachineryBreakdown,
    /// Natural calamity
    NaturalCalamity,
    /// Other
    Other,
}

impl Layoff {
    /// Compensation rate (50% of wages)
    pub fn compensation_percentage() -> f64 {
        50.0
    }

    /// Threshold for government permission
    pub fn permission_threshold() -> u32 {
        300 // 300 workers
    }

    /// Maximum layoff days without permission (45 days)
    pub fn max_days_without_permission() -> u32 {
        45
    }
}

/// Retrenchment (Section 71)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Retrenchment {
    /// Worker details
    pub worker_name: String,
    /// Years of service
    pub years_of_service: f64,
    /// Last drawn wages
    pub last_wages: f64,
    /// Compensation amount
    pub compensation: f64,
    /// Notice period (days)
    pub notice_period: u32,
    /// Notice pay in lieu
    pub notice_pay: Option<f64>,
}

impl Retrenchment {
    /// Minimum continuous service
    pub fn minimum_service_years() -> f64 {
        1.0
    }

    /// Notice period days
    pub fn notice_period_days() -> u32 {
        30
    }

    /// Compensation rate (15 days wages per year)
    pub fn compensation_per_year() -> f64 {
        15.0
    }

    /// Calculate compensation
    pub fn calculate_compensation(years: f64, daily_wage: f64) -> f64 {
        years * Self::compensation_per_year() * daily_wage
    }
}

/// Strike/Lockout
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StrikeLockout {
    /// Type
    pub action_type: IndustrialActionType,
    /// Notice given date
    pub notice_date: NaiveDate,
    /// Start date
    pub start_date: NaiveDate,
    /// End date
    pub end_date: Option<NaiveDate>,
    /// Is in essential service
    pub essential_service: bool,
    /// Is legal
    pub is_legal: bool,
}

/// Industrial action type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IndustrialActionType {
    /// Strike by workers
    Strike,
    /// Lockout by employer
    Lockout,
    /// Work-to-rule
    WorkToRule,
    /// Go-slow
    GoSlow,
    /// Gherao
    Gherao,
}

impl StrikeLockout {
    /// Notice period (days)
    pub fn notice_period_days() -> u32 {
        14 // 14 days advance notice
    }

    /// Notice validity period (days)
    pub fn notice_validity_days() -> u32 {
        60 // Valid for 60 days
    }

    /// Cooling off period after notice (days)
    pub fn cooling_off_period() -> u32 {
        14
    }
}

// =====================================================
// Occupational Safety, Health and Working Conditions Code, 2020
// =====================================================

/// Working hours (Section 25-31)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkingHours {
    /// Daily hours
    pub daily: f64,
    /// Weekly hours
    pub weekly: f64,
    /// Spread over (max hours between start and end)
    pub spread_over: f64,
    /// Rest interval
    pub rest_interval_minutes: u32,
    /// Weekly off
    pub weekly_off: WeeklyOff,
}

impl Default for WorkingHours {
    fn default() -> Self {
        Self {
            daily: 8.0,
            weekly: 48.0,
            spread_over: 10.5,
            rest_interval_minutes: 30,
            weekly_off: WeeklyOff::Sunday,
        }
    }
}

impl WorkingHours {
    /// Maximum daily hours
    pub fn max_daily() -> f64 {
        8.0
    }

    /// Maximum weekly hours
    pub fn max_weekly() -> f64 {
        48.0
    }

    /// Maximum spread over
    pub fn max_spread_over() -> f64 {
        10.5
    }

    /// Minimum rest interval
    pub fn min_rest_interval() -> u32 {
        30 // 30 minutes
    }

    /// Maximum overtime (Section 27)
    pub fn max_overtime_weekly() -> f64 {
        12.0 // Can work up to 60 hours with overtime
    }
}

/// Weekly off day
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeeklyOff {
    /// Sunday
    Sunday,
    /// Any other fixed day
    OtherFixed,
    /// Rotating
    Rotating,
}

/// Leave provisions (Section 32-35)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeaveProvisions {
    /// Earned leave per year
    pub earned_leave: u32,
    /// Casual leave
    pub casual_leave: u32,
    /// Sick leave
    pub sick_leave: u32,
    /// Accumulation limit
    pub accumulation_limit: u32,
    /// Encashment allowed
    pub encashment_allowed: bool,
}

impl Default for LeaveProvisions {
    fn default() -> Self {
        Self {
            earned_leave: 15, // 1 day per 20 days worked
            casual_leave: 12,
            sick_leave: 12,
            accumulation_limit: 30,
            encashment_allowed: true,
        }
    }
}

impl LeaveProvisions {
    /// Calculate earned leave (1 day per 20 days worked)
    pub fn calculate_earned_leave(days_worked: u32) -> u32 {
        days_worked / 20
    }

    /// Annual leave minimum
    pub fn annual_leave_minimum() -> u32 {
        15
    }
}

/// Overtime (Section 27)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Overtime {
    /// Hours of overtime
    pub hours: f64,
    /// Ordinary wage rate
    pub ordinary_rate: f64,
    /// Overtime rate (2x)
    pub overtime_rate: f64,
    /// Amount payable
    pub amount: f64,
}

impl Overtime {
    /// Overtime rate multiplier
    pub fn rate_multiplier() -> f64 {
        2.0 // Double the ordinary rate
    }

    /// Calculate overtime pay
    pub fn calculate(hours: f64, ordinary_rate: f64) -> Self {
        let overtime_rate = ordinary_rate * Self::rate_multiplier();
        let amount = hours * overtime_rate;

        Self {
            hours,
            ordinary_rate,
            overtime_rate,
            amount,
        }
    }
}

/// Establishment type under OSH Code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EstablishmentType {
    /// Factory
    Factory,
    /// Mine
    Mine,
    /// Plantation
    Plantation,
    /// Motor transport
    MotorTransport,
    /// Beedi/cigar establishment
    Beedi,
    /// Building construction
    BuildingConstruction,
    /// Shop
    Shop,
    /// Commercial establishment
    Commercial,
    /// Other
    Other,
}

impl EstablishmentType {
    /// Get applicability threshold
    pub fn applicability_threshold(&self) -> u32 {
        match self {
            Self::Factory | Self::Mine => 10,
            Self::Plantation => 50,
            Self::BuildingConstruction => 10,
            Self::Shop | Self::Commercial => 10,
            _ => 10,
        }
    }

    /// Get registration requirement
    pub fn requires_registration(&self) -> bool {
        !matches!(self, Self::Other)
    }
}

/// Safety committee requirement (Section 22)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SafetyCommittee {
    /// Employer representatives
    pub employer_reps: u32,
    /// Worker representatives
    pub worker_reps: u32,
    /// Safety officer
    pub has_safety_officer: bool,
    /// Meeting frequency per quarter
    pub quarterly_meetings: u32,
}

impl SafetyCommittee {
    /// Worker threshold for committee
    pub fn threshold() -> u32 {
        250
    }

    /// Check if required
    pub fn is_required(worker_count: u32, is_hazardous: bool) -> bool {
        worker_count >= Self::threshold() || is_hazardous
    }
}

/// Contract labour (Chapter XI)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractLabour {
    /// Principal employer
    pub principal_employer: String,
    /// Contractor
    pub contractor: String,
    /// Number of workers
    pub worker_count: u32,
    /// Registration number
    pub registration_number: Option<String>,
    /// License number
    pub license_number: Option<String>,
    /// Work nature
    pub work_nature: ContractWorkNature,
}

/// Contract work nature
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContractWorkNature {
    /// Core activity
    Core,
    /// Perennial nature
    Perennial,
    /// Temporary/seasonal
    Temporary,
    /// Support services
    SupportServices,
}

impl ContractLabour {
    /// Threshold for registration
    pub fn registration_threshold() -> u32 {
        50
    }

    /// Threshold for license
    pub fn license_threshold() -> u32 {
        50
    }

    /// Check if core activity (prohibited)
    pub fn is_core_prohibited(work_nature: ContractWorkNature) -> bool {
        matches!(work_nature, ContractWorkNature::Core)
    }
}

/// Inter-state migrant worker (Chapter XII)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterStateMigrantWorker {
    /// Name
    pub name: String,
    /// Native state
    pub native_state: String,
    /// Employment state
    pub employment_state: String,
    /// Contractor (if through contractor)
    pub contractor: Option<String>,
    /// Passbook issued
    pub passbook_issued: bool,
    /// Journey allowance paid
    pub journey_allowance_paid: bool,
}

impl InterStateMigrantWorker {
    /// Threshold for applicability
    pub fn threshold() -> u32 {
        5
    }

    /// Journey allowance (to and fro)
    pub fn journey_allowance_entitlement() -> bool {
        true
    }

    /// Displacement allowance (50% of monthly wage)
    pub fn displacement_allowance_percentage() -> f64 {
        50.0
    }
}

/// Gig worker definition (Section 2(35))
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GigWorker {
    /// Name
    pub name: String,
    /// Platform/aggregator
    pub platform: String,
    /// Work type
    pub work_type: String,
    /// Registered with social security
    pub ss_registered: bool,
    /// Unique worker ID
    pub worker_id: Option<String>,
}

/// Platform worker definition (Section 2(61))
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlatformWorker {
    /// Name
    pub name: String,
    /// Platform/aggregator
    pub platform: String,
    /// Algorithm-based work assignment
    pub algorithm_assigned: bool,
    /// Social security registration
    pub ss_registered: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wage_basic_percentage() {
        let wage = Wage {
            basic: 10000.0,
            da: 5000.0,
            retaining_allowance: 0.0,
            total: 25000.0,
            exclusions: vec![],
        };
        assert_eq!(wage.basic_percentage(), 60.0);
    }

    #[test]
    fn test_minimum_bonus() {
        assert_eq!(Bonus::calculate_minimum(10000.0), 833.0);
    }

    #[test]
    fn test_maximum_bonus() {
        assert_eq!(Bonus::calculate_maximum(10000.0), 2000.0);
    }

    #[test]
    fn test_epf_contribution() {
        let contribution = EpfContribution::calculate(15000.0);
        assert_eq!(contribution.employee, 1800.0);
        assert_eq!(contribution.employer, 1800.0);
        assert!((contribution.eps_portion - 1249.5).abs() < 1.0);
    }

    #[test]
    fn test_epf_contribution_above_ceiling() {
        let contribution = EpfContribution::calculate(25000.0);
        // Should cap at 15000
        assert_eq!(contribution.employee, 1800.0);
    }

    #[test]
    fn test_esi_contribution() {
        let contribution = EsiContribution::calculate(20000.0);
        assert_eq!(contribution.employee, 150.0);
        assert_eq!(contribution.employer, 650.0);
    }

    #[test]
    fn test_gratuity_calculation() {
        let gratuity = Gratuity::calculate(10.0, 30000.0);
        // (30000 * 15 * 10) / 26 = 173076.92
        assert!((gratuity.amount - 173076.92).abs() < 1.0);
    }

    #[test]
    fn test_gratuity_eligibility() {
        assert!(!Gratuity::is_eligible(4.5));
        assert!(Gratuity::is_eligible(5.0));
    }

    #[test]
    fn test_maternity_leave_weeks() {
        assert_eq!(MaternityBenefitType::NormalDelivery.leave_weeks(), 26);
        assert_eq!(MaternityBenefitType::ThirdChildOnwards.leave_weeks(), 12);
    }

    #[test]
    fn test_retrenchment_compensation() {
        let comp = Retrenchment::calculate_compensation(5.0, 1000.0);
        // 5 * 15 * 1000 = 75000
        assert_eq!(comp, 75000.0);
    }

    #[test]
    fn test_overtime_calculation() {
        let ot = Overtime::calculate(2.0, 100.0);
        assert_eq!(ot.overtime_rate, 200.0);
        assert_eq!(ot.amount, 400.0);
    }

    #[test]
    fn test_working_hours_default() {
        let hours = WorkingHours::default();
        assert_eq!(hours.daily, 8.0);
        assert_eq!(hours.weekly, 48.0);
    }

    #[test]
    fn test_earned_leave_calculation() {
        assert_eq!(LeaveProvisions::calculate_earned_leave(240), 12);
    }

    #[test]
    fn test_trade_union_minimum_members() {
        assert_eq!(TradeUnion::minimum_members(500), 50);
        assert_eq!(TradeUnion::minimum_members(2000), 100);
        assert_eq!(TradeUnion::minimum_members(50), 7);
    }

    #[test]
    fn test_safety_committee_requirement() {
        assert!(SafetyCommittee::is_required(300, false));
        assert!(SafetyCommittee::is_required(100, true));
        assert!(!SafetyCommittee::is_required(100, false));
    }

    #[test]
    fn test_layoff_threshold() {
        assert_eq!(Layoff::permission_threshold(), 300);
    }

    #[test]
    fn test_strike_notice_period() {
        assert_eq!(StrikeLockout::notice_period_days(), 14);
    }

    #[test]
    fn test_skill_level_multiplier() {
        assert_eq!(SkillLevel::Unskilled.wage_multiplier(), 1.0);
        assert_eq!(SkillLevel::Skilled.wage_multiplier(), 1.30);
    }
}
