//! Types for German Labor Law (Arbeitsrecht)
//!
//! This module provides type-safe representations of German labor law concepts
//! including employment contracts, dismissal protection, working hours, and leave.

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::gmbhg::Capital;

/// Employment contract (Arbeitsvertrag)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmploymentContract {
    pub employee: Employee,
    pub employer: Employer,
    pub contract_type: ContractType,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>, // None for unlimited, Some for fixed-term
    pub probation_period_months: Option<u8>, // Max 6 months
    pub salary: Salary,
    pub working_hours: WorkingHours,
    pub duties: String,
    pub written: bool, // §2 NachwG requires written documentation
}

/// Employee (Arbeitnehmer)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Employee {
    pub name: String,
    pub date_of_birth: NaiveDate,
    pub address: String,
    pub social_security_number: Option<String>,
}

impl Employee {
    pub fn age_at(&self, date: NaiveDate) -> u32 {
        let years = date.year() - self.date_of_birth.year();
        if date.month() < self.date_of_birth.month()
            || (date.month() == self.date_of_birth.month() && date.day() < self.date_of_birth.day())
        {
            (years - 1) as u32
        } else {
            years as u32
        }
    }
}

/// Employer (Arbeitgeber)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Employer {
    pub name: String,
    pub address: String,
    pub company_size: CompanySize,
}

/// Company size for legal thresholds
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompanySize {
    /// Less than 10 employees (no KSchG protection)
    Small,
    /// 10+ employees (KSchG applies)
    Medium,
    /// 20+ employees (full co-determination rights)
    Large,
}

impl CompanySize {
    pub fn from_employee_count(count: u32) -> Self {
        if count < 10 {
            CompanySize::Small
        } else if count < 20 {
            CompanySize::Medium
        } else {
            CompanySize::Large
        }
    }

    pub fn has_dismissal_protection(&self) -> bool {
        !matches!(self, CompanySize::Small)
    }
}

/// Type of employment contract
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractType {
    /// Unlimited contract (Unbefristeter Arbeitsvertrag)
    Unlimited,
    /// Fixed-term contract (Befristeter Arbeitsvertrag) - TzBfG
    FixedTerm { reason: FixedTermReason },
    /// Part-time (Teilzeit) - TzBfG
    PartTime { hours_per_week: u8 },
    /// Temporary agency work (Zeitarbeit) - AÜG
    TemporaryAgency,
}

/// Reason for fixed-term contract (§14 TzBfG)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FixedTermReason {
    /// Temporary need (vorübergehender Bedarf)
    TemporaryNeed,
    /// Trial basis (Erprobung)
    Trial,
    /// Employee request (auf Wunsch des Arbeitnehmers)
    EmployeeRequest,
    /// No reason needed (first 2 years, max 2 years total)
    NoReasonNeeded,
}

/// Salary structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Salary {
    pub gross_monthly: Capital,
    pub payment_day: u8, // Day of month (1-31)
    pub includes_overtime: bool,
}

impl Salary {
    pub fn gross_annual(&self) -> Capital {
        Capital::from_cents(self.gross_monthly.amount_cents * 12)
    }
}

/// Working hours (Arbeitszeitgesetz - ArbZG)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkingHours {
    pub hours_per_week: u8,
    pub days_per_week: u8,
    pub overtime_allowed: bool,
}

impl WorkingHours {
    /// Check if within ArbZG limits (§3 - max 8 hours/day, 10 with compensation)
    pub fn complies_with_arbzg(&self) -> bool {
        if self.days_per_week == 0 {
            return false;
        }
        let hours_per_day = self.hours_per_week as f32 / self.days_per_week as f32;
        hours_per_day <= 10.0
    }
}

/// Dismissal (Kündigung)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dismissal {
    pub dismissed_by: DismissalParty,
    pub employee_name: String,
    pub dismissal_date: NaiveDate,
    pub dismissal_type: DismissalType,
    pub grounds: DismissalGrounds,
    pub notice_period_weeks: u8,
    pub effective_date: NaiveDate,
    pub written: bool,                 // §623 BGB requires written form
    pub works_council_consulted: bool, // §102 BetrVG if works council exists
}

/// Party initiating dismissal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DismissalParty {
    Employer,
    Employee,
}

/// Type of dismissal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DismissalType {
    /// Ordinary dismissal with notice (Ordentliche Kündigung)
    Ordinary,
    /// Extraordinary dismissal without notice (Außerordentliche Kündigung) - §626 BGB
    Extraordinary,
}

/// Grounds for dismissal (§1 KSchG)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DismissalGrounds {
    /// Conduct-related (Verhaltensbedingt) - Employee misconduct
    Conduct { description: String, warnings: u8 },
    /// Personal reasons (Personenbedingt) - Employee incapacity
    Personal { description: String },
    /// Operational reasons (Betriebsbedingt) - Business necessity
    Operational { description: String },
    /// Extraordinary cause (Wichtiger Grund) - §626 BGB
    ExtraordinaryCause { description: String },
}

/// Annual leave entitlement (Bundesurlaubsgesetz - BUrlG)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeaveEntitlement {
    pub employee_name: String,
    pub year: u16,
    pub days_per_week: u8,    // Working days per week
    pub minimum_days: u8,     // §3 BUrlG: 24 days for 6-day week (4 weeks)
    pub contractual_days: u8, // Can be more than minimum
    pub days_taken: u8,
    pub days_carried_over: u8, // From previous year
}

impl LeaveEntitlement {
    /// Calculate minimum leave per BUrlG (24 days for 6-day week, proportional for others)
    pub fn calculate_minimum(days_per_week: u8) -> u8 {
        match days_per_week {
            6 => 24,
            5 => 20, // 4 weeks × 5 days
            4 => 16,
            3 => 12,
            2 => 8,
            1 => 4,
            _ => 0,
        }
    }

    pub fn remaining_days(&self) -> u8 {
        self.contractual_days + self.days_carried_over - self.days_taken
    }
}

/// Sick leave and continued remuneration (Entgeltfortzahlungsgesetz - EFZG)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SickLeave {
    pub employee_name: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,        // None if ongoing
    pub medical_certificate_provided: bool, // §5 EFZG - required after 3 days
    pub notification_timely: bool,          // Must notify employer immediately
}

impl SickLeave {
    /// Calculate duration in days
    pub fn duration_days(&self, current_date: NaiveDate) -> u32 {
        let end = self.end_date.unwrap_or(current_date);
        (end - self.start_date).num_days().max(0) as u32
    }

    /// Check if entitled to continued remuneration (§3 EFZG - 6 weeks)
    pub fn entitled_to_continued_pay(&self, current_date: NaiveDate) -> bool {
        self.duration_days(current_date) <= 42 // 6 weeks
    }
}

/// Maternity protection (Mutterschutzgesetz - MuSchG)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaternityProtection {
    pub employee_name: String,
    pub expected_due_date: NaiveDate,
    pub maternity_leave_start: NaiveDate, // 6 weeks before due date
    pub maternity_leave_end: NaiveDate,   // 8 weeks after birth (12 for multiples)
    pub multiple_birth: bool,
}

impl MaternityProtection {
    /// Calculate maternity leave period (§3 MuSchG)
    pub fn new(employee_name: String, due_date: NaiveDate, multiple_birth: bool) -> Self {
        let leave_start = due_date - chrono::Duration::weeks(6);
        let leave_weeks = if multiple_birth { 12 } else { 8 };
        let leave_end = due_date + chrono::Duration::weeks(leave_weeks);

        MaternityProtection {
            employee_name,
            expected_due_date: due_date,
            maternity_leave_start: leave_start,
            maternity_leave_end: leave_end,
            multiple_birth,
        }
    }
}

/// Parental leave (Elternzeit - BEEG)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParentalLeave {
    pub employee_name: String,
    pub child_birth_date: NaiveDate,
    pub leave_start: NaiveDate,
    pub leave_end: NaiveDate,
    pub notice_given_weeks: u8, // Must be at least 7 weeks before start
}

impl ParentalLeave {
    /// Maximum parental leave per child (§15 BEEG - 3 years)
    pub const MAX_YEARS: u8 = 3;

    pub fn duration_days(&self) -> u32 {
        (self.leave_end - self.leave_start).num_days().max(0) as u32
    }

    pub fn duration_years(&self) -> f32 {
        self.duration_days() as f32 / 365.25
    }
}

/// Notice period (Kündigungsfrist)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoticePeriod {
    pub weeks: u8,
    pub months: Option<u8>,
}

impl NoticePeriod {
    /// Calculate notice period per §622 BGB
    pub fn for_employee_tenure(years_of_service: u8) -> Self {
        match years_of_service {
            0..=1 => NoticePeriod {
                weeks: 4,
                months: None,
            },
            2 => NoticePeriod {
                weeks: 0,
                months: Some(1),
            },
            5 => NoticePeriod {
                weeks: 0,
                months: Some(2),
            },
            8 => NoticePeriod {
                weeks: 0,
                months: Some(3),
            },
            10 => NoticePeriod {
                weeks: 0,
                months: Some(4),
            },
            12 => NoticePeriod {
                weeks: 0,
                months: Some(5),
            },
            15 => NoticePeriod {
                weeks: 0,
                months: Some(6),
            },
            20 => NoticePeriod {
                weeks: 0,
                months: Some(7),
            },
            _ => NoticePeriod {
                weeks: 4,
                months: None,
            },
        }
    }
}

/// Works council (Betriebsrat - BetrVG)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorksCouncil {
    pub company_name: String,
    pub employee_count: u32,
    pub council_members: Vec<CouncilMember>,
    pub formation_date: NaiveDate,
}

impl WorksCouncil {
    /// Check if company requires works council (§1 BetrVG - 5+ employees)
    pub fn is_required(employee_count: u32) -> bool {
        employee_count >= 5
    }

    /// Calculate required council size per §9 BetrVG
    pub fn required_size(employee_count: u32) -> u8 {
        match employee_count {
            5..=20 => 1,
            21..=50 => 3,
            51..=100 => 5,
            101..=200 => 7,
            201..=400 => 9,
            401..=700 => 11,
            701..=1000 => 13,
            1001..=1500 => 15,
            _ => 15 + ((employee_count - 1500) / 500) as u8,
        }
    }
}

/// Works council member (Betriebsratsmitglied)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CouncilMember {
    pub name: String,
    pub position: CouncilPosition,
    pub term_start: NaiveDate,
}

/// Position in works council
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CouncilPosition {
    Chairperson,
    DeputyChairperson,
    Member,
}

// ===== Collective Labor Law (Kollektives Arbeitsrecht) =====

/// Collective bargaining agreement (Tarifvertrag) - TVG
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectiveBargainingAgreement {
    pub agreement_name: String,
    pub parties: BargainingParties,
    pub effective_date: NaiveDate,
    pub expiry_date: Option<NaiveDate>, // None for unlimited
    pub agreement_type: AgreementType,
    pub coverage: AgreementCoverage,
    pub normative_provisions: Vec<NormativeProvision>, // §1 TVG
    pub obligational_provisions: Vec<String>,          // Peace obligation, etc.
    pub registered: bool,                              // Registration with labor authority
}

impl CollectiveBargainingAgreement {
    /// Check if agreement is currently valid
    pub fn is_valid(&self, date: NaiveDate) -> bool {
        if date < self.effective_date {
            return false;
        }
        if let Some(expiry) = self.expiry_date {
            date <= expiry
        } else {
            true // Unlimited duration
        }
    }

    /// Check after-effect (Nachwirkung) per §4 Abs. 5 TVG
    /// Provisions continue to apply after expiry until new agreement
    pub fn has_after_effect(&self, date: NaiveDate) -> bool {
        if let Some(expiry) = self.expiry_date {
            date > expiry // After-effect applies after expiration
        } else {
            false
        }
    }
}

/// Parties to collective bargaining
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BargainingParties {
    pub union: Union,
    pub employer_association: Option<EmployerAssociation>, // Or individual employer
    pub individual_employer: Option<String>,               // For single-employer agreements
}

/// Labor union (Gewerkschaft)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Union {
    pub name: String,
    pub registered: bool,
    pub member_count: u32,
}

/// Employer association (Arbeitgeberverband)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmployerAssociation {
    pub name: String,
    pub member_companies: u32,
}

/// Type of collective agreement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgreementType {
    /// Industry-wide agreement (Flächentarifvertrag)
    IndustryWide,
    /// Company agreement (Firmentarifvertrag)
    CompanyLevel,
    /// Framework agreement (Manteltarifvertrag) - general conditions
    Framework,
    /// Wage agreement (Lohntarifvertrag) - wages and salaries
    Wage,
}

/// Coverage scope
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgreementCoverage {
    /// Covers specific industry/sector
    Industry { sector: String },
    /// Covers specific region
    Regional { region: String },
    /// Covers single company
    Company { company_name: String },
    /// National coverage
    National,
}

/// Normative provision (Normativer Teil) - §1 TVG
/// Direct and mandatory effect on employment relationships
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NormativeProvision {
    pub provision_type: ProvisionType,
    pub description: String,
    pub details: ProvisionDetails,
}

/// Type of normative provision
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProvisionType {
    /// Wages and salaries (Vergütung)
    Compensation,
    /// Working hours (Arbeitszeit)
    WorkingHours,
    /// Leave entitlement (Urlaub)
    Leave,
    /// Notice periods (Kündigungsfristen)
    NoticePeriods,
    /// Other working conditions
    OtherConditions,
}

/// Detailed provision content
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProvisionDetails {
    /// Wage scale with grades
    WageScale { grades: Vec<WageGrade> },
    /// Working hours specification
    WorkingHoursSpec {
        hours_per_week: u8,
        days_per_week: u8,
    },
    /// Leave days
    LeaveSpec { days_per_year: u8 },
    /// Notice period specification
    NoticeSpec { weeks: u8 },
    /// Generic provision
    Other { details: String },
}

/// Wage grade (Lohngruppe)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WageGrade {
    pub grade: u8,
    pub description: String,
    pub monthly_wage: Capital,
}

/// Co-determination types (Mitbestimmung) - MitbestG
/// Supervisory board (Aufsichtsrat) with employee representation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupervisoryBoard {
    pub company_name: String,
    pub employee_count: u32,
    pub codetermination_type: CodeterminationType,
    pub total_members: u8,
    pub employee_representatives: u8,
    pub shareholder_representatives: u8,
    pub members: Vec<BoardMember>,
}

impl SupervisoryBoard {
    /// Determine required co-determination type based on employee count
    pub fn required_codetermination(employee_count: u32) -> CodeterminationType {
        if employee_count >= 2000 {
            CodeterminationType::Full // MitbestG - 50% employee representation
        } else if employee_count >= 500 {
            CodeterminationType::OneThird // DrittelbG - 1/3 employee representation
        } else {
            CodeterminationType::None
        }
    }

    /// Calculate required board size per MitbestG
    pub fn required_size(employee_count: u32) -> u8 {
        if employee_count >= 20_000 {
            20 // Large companies
        } else if employee_count >= 10_000 {
            16
        } else if employee_count >= 2_000 {
            12 // Standard for MitbestG
        } else if employee_count >= 500 {
            9 // DrittelbG minimum
        } else {
            6 // Minimum for smaller AGs
        }
    }
}

/// Type of co-determination
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CodeterminationType {
    /// No co-determination (< 500 employees)
    None,
    /// One-third representation (500-1,999 employees) - DrittelbG
    OneThird,
    /// Full parity co-determination (2,000+ employees) - MitbestG
    Full,
    /// Coal and steel industry (Montan-Mitbestimmung) - special regime
    MontanMitbestimmung,
}

/// Supervisory board member
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoardMember {
    pub name: String,
    pub representative_type: RepresentativeType,
    pub position: Option<BoardPosition>,
}

/// Type of board representative
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RepresentativeType {
    /// Employee representative (Arbeitnehmervertreter)
    Employee,
    /// Shareholder representative (Anteilseignervertreter)
    Shareholder,
    /// Trade union representative (Gewerkschaftsvertreter)
    Union,
}

/// Position on supervisory board
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BoardPosition {
    /// Chairperson (Aufsichtsratsvorsitzender)
    Chairperson,
    /// Deputy chairperson (Stellvertretender Vorsitzender)
    DeputyChairperson,
    /// Regular member
    Member,
}

/// Works council co-determination rights (BetrVG §87)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeterminationRights {
    pub company_name: String,
    pub employee_count: u32,
    pub rights: Vec<CodeterminationRight>,
}

/// Specific co-determination right
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeterminationRight {
    pub right_type: CodeterminationRightType,
    pub description: String,
    pub legal_basis: String, // e.g., "§87 Abs. 1 Nr. 1 BetrVG"
}

/// Type of co-determination right
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CodeterminationRightType {
    /// Working hours (§87 Abs. 1 Nr. 2 BetrVG)
    WorkingHours,
    /// Overtime (§87 Abs. 1 Nr. 3 BetrVG)
    Overtime,
    /// Payment methods (§87 Abs. 1 Nr. 4 BetrVG)
    PaymentMethods,
    /// Leave scheduling (§87 Abs. 1 Nr. 5 BetrVG)
    LeaveScheduling,
    /// Technical monitoring (§87 Abs. 1 Nr. 6 BetrVG)
    TechnicalMonitoring,
    /// Health and safety (§87 Abs. 1 Nr. 7 BetrVG)
    HealthAndSafety,
    /// Social facilities (§87 Abs. 1 Nr. 8 BetrVG)
    SocialFacilities,
    /// Personnel selection (§99 BetrVG)
    PersonnelSelection,
    /// Vocational training (§98 BetrVG)
    VocationalTraining,
}
