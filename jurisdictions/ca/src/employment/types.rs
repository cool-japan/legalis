//! Canada Employment Law - Types
//!
//! Core types for Canadian employment law (federal and provincial).

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use crate::common::{CaseCitation, Province};

// ============================================================================
// Employment Status
// ============================================================================

/// Employment status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmploymentStatus {
    /// Employee (common law relationship)
    Employee,
    /// Independent contractor
    IndependentContractor,
    /// Dependent contractor
    DependentContractor,
    /// Agency/temporary worker
    AgencyWorker,
    /// Probationary employee
    Probationary { months: u32 },
    /// Fixed-term employee
    FixedTerm { end_date: String },
}

/// Factors for employee vs contractor analysis (Sagaz)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SagazFactor {
    /// Control over work
    Control,
    /// Ownership of tools
    OwnershipOfTools,
    /// Chance of profit
    ChanceOfProfit,
    /// Risk of loss
    RiskOfLoss,
    /// Integration into business
    Integration,
    /// Exclusive relationship
    Exclusivity,
}

// ============================================================================
// Jurisdiction
// ============================================================================

/// Employment jurisdiction
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmploymentJurisdiction {
    /// Federal (Canada Labour Code)
    Federal,
    /// Provincial (Employment Standards Act)
    Provincial { province: Province },
}

/// Federally regulated industries (s.91 industries)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FederalIndustry {
    /// Banks and banking
    Banking,
    /// Interprovincial/international transportation
    Transportation,
    /// Telecommunications
    Telecommunications,
    /// Broadcasting
    Broadcasting,
    /// Airlines
    Airlines,
    /// Railways
    Railways,
    /// Shipping
    Shipping,
    /// Postal services
    PostalService,
    /// Grain handling
    GrainHandling,
    /// Uranium mining
    UraniumMining,
    /// Crown corporations
    CrownCorporation,
    /// First Nations
    FirstNations,
    /// Other federal work/undertaking
    Other { description: String },
}

// ============================================================================
// Employment Standards
// ============================================================================

/// Minimum employment standards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmploymentStandards {
    /// Minimum wage (cents per hour)
    pub minimum_wage_cents: i64,
    /// Regular hours per week
    pub regular_hours_per_week: f64,
    /// Overtime threshold
    pub overtime_threshold_hours: f64,
    /// Overtime premium rate
    pub overtime_rate: f64,
    /// Vacation entitlement (weeks)
    pub vacation_weeks: f64,
    /// Vacation pay percentage
    pub vacation_pay_percent: f64,
    /// Public holidays per year
    pub public_holidays: u32,
    /// Minimum notice weeks per year of service
    pub notice_per_year_weeks: f64,
}

impl Default for EmploymentStandards {
    fn default() -> Self {
        Self {
            minimum_wage_cents: 1565, // $15.65 (Ontario 2024)
            regular_hours_per_week: 44.0,
            overtime_threshold_hours: 44.0,
            overtime_rate: 1.5,
            vacation_weeks: 2.0,
            vacation_pay_percent: 4.0,
            public_holidays: 9,
            notice_per_year_weeks: 1.0,
        }
    }
}

/// Employment standard type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StandardType {
    /// Minimum wage
    MinimumWage,
    /// Hours of work
    HoursOfWork,
    /// Overtime
    Overtime,
    /// Vacation
    Vacation,
    /// Public holidays
    PublicHolidays,
    /// Sick leave
    SickLeave,
    /// Parental leave
    ParentalLeave,
    /// Termination notice/pay
    TerminationNotice,
    /// Severance pay
    SeverancePay,
    /// Equal pay
    EqualPay,
}

// ============================================================================
// Termination
// ============================================================================

/// Type of termination
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerminationType {
    /// Resignation
    Resignation,
    /// Termination without cause
    WithoutCause,
    /// Termination for cause
    ForCause,
    /// Constructive dismissal
    ConstructiveDismissal,
    /// Unjust dismissal (federal)
    UnjustDismissal,
    /// Mass termination
    MassTermination { number: u32 },
    /// Layoff
    Layoff,
    /// End of fixed term
    EndOfFixedTerm,
    /// Frustration of contract
    Frustration,
}

/// Just cause grounds
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JustCauseGround {
    /// Dishonesty/fraud
    Dishonesty,
    /// Theft
    Theft,
    /// Insubordination
    Insubordination,
    /// Incompetence
    Incompetence,
    /// Habitual neglect of duty
    HabitualNeglect,
    /// Conflict of interest
    ConflictOfInterest,
    /// Harassment
    Harassment,
    /// Violence
    Violence,
    /// Intoxication at work
    IntoxicationAtWork,
    /// Breach of policy
    BreachOfPolicy { policy: String },
    /// Criminal conduct
    CriminalConduct,
}

/// Bardal factors for reasonable notice
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BardalFactor {
    /// Character of employment (nature of position)
    CharacterOfEmployment,
    /// Length of service
    LengthOfService,
    /// Age of employee
    AgeOfEmployee,
    /// Availability of similar employment
    AvailabilityOfEmployment,
}

// ============================================================================
// Human Rights
// ============================================================================

/// Protected ground under Human Rights Code
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProtectedGround {
    /// Race
    Race,
    /// Ancestry/place of origin
    Ancestry,
    /// Colour
    Colour,
    /// Ethnic origin
    EthnicOrigin,
    /// Citizenship
    Citizenship,
    /// Creed/religion
    Creed,
    /// Sex (including pregnancy)
    Sex,
    /// Sexual orientation
    SexualOrientation,
    /// Gender identity/expression
    GenderIdentity,
    /// Age
    Age,
    /// Record of offences
    RecordOfOffences,
    /// Marital status
    MaritalStatus,
    /// Family status
    FamilyStatus,
    /// Disability
    Disability,
    /// Receipt of public assistance
    ReceiptOfPublicAssistance,
}

/// Type of discrimination
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscriminationType {
    /// Direct discrimination
    Direct,
    /// Adverse effect/indirect discrimination
    AdverseEffect,
    /// Systemic discrimination
    Systemic,
    /// Harassment
    Harassment,
    /// Poisoned work environment
    PoisonedEnvironment,
    /// Failure to accommodate
    FailureToAccommodate,
    /// Reprisal
    Reprisal,
}

/// Duty to accommodate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DutyToAccommodate {
    /// Protected ground requiring accommodation
    pub ground: ProtectedGround,
    /// Type of accommodation needed
    pub accommodation_type: AccommodationType,
    /// Whether undue hardship applies
    pub undue_hardship: bool,
    /// Undue hardship factors
    pub hardship_factors: Vec<HardshipFactor>,
}

/// Type of accommodation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccommodationType {
    /// Modified duties
    ModifiedDuties,
    /// Modified schedule
    ModifiedSchedule,
    /// Physical modification
    PhysicalModification,
    /// Leave of absence
    LeaveOfAbsence,
    /// Reassignment
    Reassignment,
    /// Assistive technology
    AssistiveTechnology,
    /// Religious accommodation
    ReligiousAccommodation,
    /// Other
    Other { description: String },
}

/// Undue hardship factors
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HardshipFactor {
    /// Cost
    Cost,
    /// Health and safety risk
    HealthAndSafety,
    /// Effect on other employees
    EffectOnOthers,
    /// Operational disruption
    OperationalDisruption,
    /// Size of organization
    OrganizationSize,
    /// Interchangeability of workforce
    WorkforceInterchangeability,
}

// ============================================================================
// Wrongful Dismissal
// ============================================================================

/// Wrongful dismissal damages
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WrongfulDismissalDamages {
    /// Pay in lieu of notice
    PayInLieuOfNotice,
    /// Benefits continuation
    BenefitsContinuation,
    /// Bonus/commission
    BonusCommission,
    /// Stock options
    StockOptions,
    /// Pension loss
    PensionLoss,
    /// Aggravated damages (Keays v Honda)
    AggravatedDamages,
    /// Punitive damages (exceptional)
    PunitiveDamages,
    /// Bad faith damages (Wallace)
    BadFaithDamages,
}

/// Mitigation requirement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitigationRequirement {
    /// Whether mitigation required
    pub required: bool,
    /// Mitigation efforts made
    pub efforts: Vec<String>,
    /// Whether efforts reasonable
    pub efforts_reasonable: bool,
    /// New employment income (cents)
    pub new_income_cents: Option<i64>,
}

// ============================================================================
// Key Cases
// ============================================================================

/// Key Canadian employment law case
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmploymentCase {
    /// Citation
    pub citation: CaseCitation,
    /// Legal principle
    pub principle: String,
    /// Area of employment law
    pub area: EmploymentArea,
}

/// Area of employment law
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmploymentArea {
    /// Employment status
    EmploymentStatus,
    /// Termination/notice
    Termination,
    /// Wrongful dismissal
    WrongfulDismissal,
    /// Just cause
    JustCause,
    /// Human rights
    HumanRights,
    /// Employment standards
    EmploymentStandards,
    /// Constructive dismissal
    ConstructiveDismissal,
}

impl EmploymentCase {
    /// Bardal v Globe & Mail \[1960\] - reasonable notice factors
    pub fn bardal() -> Self {
        Self {
            citation: CaseCitation {
                name: "Bardal v Globe & Mail Ltd".to_string(),
                year: 1960,
                neutral_citation: None,
                report_citation: Some("(1960) 24 DLR (2d) 140 (Ont HC)".to_string()),
                court: crate::common::Court::SuperiorCourt {
                    province: Province::Ontario,
                    name: "High Court of Justice".to_string(),
                },
                principle: "Factors for determining reasonable notice period".to_string(),
            },
            principle: "Reasonable notice depends on: (1) character of employment, \
                (2) length of service, (3) age of employee, (4) availability of similar employment"
                .to_string(),
            area: EmploymentArea::Termination,
        }
    }

    /// 671122 Ontario Ltd v Sagaz Industries \[2001\] SCC 59 - employee vs contractor
    pub fn sagaz() -> Self {
        Self {
            citation: CaseCitation::scc(
                "671122 Ontario Ltd v Sagaz Industries Canada Inc",
                2001,
                59,
                "Test for employee vs independent contractor",
            ),
            principle:
                "Central question: whose business is it? Consider control, ownership of tools, \
                chance of profit, risk of loss. No single conclusive test."
                    .to_string(),
            area: EmploymentArea::EmploymentStatus,
        }
    }

    /// McKinley v BC Tel \[2001\] SCC 38 - just cause and proportionality
    pub fn mckinley() -> Self {
        Self {
            citation: CaseCitation::scc(
                "McKinley v BC Tel",
                2001,
                38,
                "Contextual approach to just cause for dishonesty",
            ),
            principle: "Just cause requires contextual approach. Dishonesty must be assessed in \
                context - was it fundamentally incompatible with employment relationship?"
                .to_string(),
            area: EmploymentArea::JustCause,
        }
    }

    /// Wallace v United Grain Growers \[1997\] - bad faith damages
    pub fn wallace() -> Self {
        Self {
            citation: CaseCitation::scc(
                "Wallace v United Grain Growers Ltd",
                1997,
                46,
                "Bad faith damages in manner of dismissal",
            ),
            principle: "Employer owes duty of good faith and fair dealing in manner of dismissal. \
                Bad faith in manner of dismissal can extend notice period (Wallace bump)."
                .to_string(),
            area: EmploymentArea::WrongfulDismissal,
        }
    }

    /// Keays v Honda Canada \[2008\] SCC 39 - aggravated damages
    pub fn keays_v_honda() -> Self {
        Self {
            citation: CaseCitation::scc(
                "Keays v Honda Canada Inc",
                2008,
                39,
                "Replaces Wallace bump with compensatory damages",
            ),
            principle: "Wallace bump replaced by regular compensatory damages framework. \
                Aggravated damages for mental distress require proof of actual damages."
                .to_string(),
            area: EmploymentArea::WrongfulDismissal,
        }
    }

    /// Potter v New Brunswick Legal Aid \[2015\] - constructive dismissal
    pub fn potter() -> Self {
        Self {
            citation: CaseCitation::scc(
                "Potter v New Brunswick Legal Aid Services Commission",
                2015,
                10,
                "Two-branch test for constructive dismissal",
            ),
            principle: "Two branches: (1) Express/implied breach of essential term, OR \
                (2) Employer conduct demonstrates intention not to be bound by contract."
                .to_string(),
            area: EmploymentArea::ConstructiveDismissal,
        }
    }

    /// British Columbia v BCGSEU (Meiorin) \[1999\] - duty to accommodate
    pub fn meiorin() -> Self {
        Self {
            citation: CaseCitation::scc(
                "British Columbia (Public Service Employee Relations Commission) v BCGSEU",
                1999,
                48,
                "Unified approach to accommodation - BFOR test",
            ),
            principle:
                "Three-step BFOR test: (1) Standard adopted for purpose rationally connected \
                to job, (2) Standard adopted in good faith, (3) Standard reasonably necessary - \
                impossible to accommodate without undue hardship."
                    .to_string(),
            area: EmploymentArea::HumanRights,
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_employment_status() {
        let employee = EmploymentStatus::Employee;
        let contractor = EmploymentStatus::IndependentContractor;
        assert_ne!(employee, contractor);
    }

    #[test]
    fn test_federal_jurisdiction() {
        let federal = EmploymentJurisdiction::Federal;
        let provincial = EmploymentJurisdiction::Provincial {
            province: Province::Ontario,
        };
        assert_ne!(federal, provincial);
    }

    #[test]
    fn test_employment_standards_default() {
        let standards = EmploymentStandards::default();
        assert!(standards.minimum_wage_cents > 0);
        assert_eq!(standards.overtime_rate, 1.5);
    }

    #[test]
    fn test_bardal_factors() {
        let factor = BardalFactor::LengthOfService;
        assert_eq!(factor, BardalFactor::LengthOfService);
    }

    #[test]
    fn test_protected_grounds() {
        let ground = ProtectedGround::Disability;
        assert_eq!(ground, ProtectedGround::Disability);
    }

    #[test]
    fn test_bardal_case() {
        let case = EmploymentCase::bardal();
        assert_eq!(case.citation.year, 1960);
        assert_eq!(case.area, EmploymentArea::Termination);
    }

    #[test]
    fn test_sagaz_case() {
        let case = EmploymentCase::sagaz();
        assert_eq!(case.citation.year, 2001);
        assert!(case.principle.contains("whose business"));
    }

    #[test]
    fn test_meiorin_case() {
        let case = EmploymentCase::meiorin();
        assert_eq!(case.area, EmploymentArea::HumanRights);
        assert!(case.principle.contains("BFOR"));
    }
}
