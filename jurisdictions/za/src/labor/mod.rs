//! South African Labour Law
//!
//! ## Key Legislation
//!
//! - Labour Relations Act 66 of 1995 (LRA) - Collective bargaining, unfair dismissal
//! - Basic Conditions of Employment Act 75 of 1997 (BCEA) - Working hours, leave
//! - Employment Equity Act 55 of 1998 (EEA) - Affirmative action
//! - National Minimum Wage Act 9 of 2018 (NMWA)
//!
//! ## CCMA
//!
//! The Commission for Conciliation, Mediation and Arbitration handles:
//! - Unfair dismissal disputes (30 days to refer)
//! - Unfair labour practice disputes
//! - Collective bargaining disputes

use crate::common::Zar;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for labor law operations
pub type LaborResult<T> = Result<T, LaborError>;

/// Employment contract types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContractType {
    /// Permanent/Indefinite contract
    Permanent,
    /// Fixed-term contract
    FixedTerm {
        /// Duration in months
        duration_months: u32,
    },
    /// Part-time employment
    PartTime,
    /// Temporary employment (TES) - s198 LRA
    TemporaryEmploymentServices,
}

impl ContractType {
    /// Check if contract type provides full employment protection
    pub fn has_full_protection(&self) -> bool {
        match self {
            Self::Permanent => true,
            // Fixed-term > 3 months gets protection under s198B
            Self::FixedTerm { duration_months } => *duration_months > 3,
            Self::PartTime => true,
            Self::TemporaryEmploymentServices => true, // After 3 months, deemed permanent
        }
    }
}

/// Working hours under BCEA
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingHours {
    /// Ordinary hours per week (max 45)
    pub ordinary_hours_per_week: u32,
    /// Working days per week
    pub days_per_week: u32,
    /// Overtime hours per week
    pub overtime_per_week: u32,
}

impl WorkingHours {
    /// Standard working hours (45/week, 5 days)
    pub fn standard() -> Self {
        Self {
            ordinary_hours_per_week: 45,
            days_per_week: 5,
            overtime_per_week: 0,
        }
    }

    /// Total weekly hours
    pub fn total_weekly_hours(&self) -> u32 {
        self.ordinary_hours_per_week + self.overtime_per_week
    }

    /// Check if within BCEA limits
    pub fn is_within_limits(&self) -> bool {
        // Ordinary hours max 45/week
        // Overtime max 10/week or 3/day (s10 BCEA)
        self.ordinary_hours_per_week <= 45 && self.overtime_per_week <= 10
    }
}

/// Leave entitlements under BCEA (s20-27)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LeaveType {
    /// Annual leave (21 consecutive days or 15 working days per cycle)
    Annual,
    /// Sick leave (30 days per 3-year cycle, s22)
    Sick,
    /// Family responsibility leave (3 days per year, s27)
    FamilyResponsibility,
    /// Maternity leave (4 consecutive months, s25)
    Maternity,
    /// Parental leave (10 consecutive days, s25A - 2020 amendment)
    Parental,
    /// Adoption leave (10 weeks, s25B)
    Adoption,
    /// Commissioning parental leave (10 weeks, s25C)
    CommissioningParental,
}

impl LeaveType {
    /// Get statutory minimum days
    pub fn statutory_days(&self) -> u32 {
        match self {
            Self::Annual => 21,              // Or 15 working days
            Self::Sick => 30,                // Per 3-year cycle
            Self::FamilyResponsibility => 3, // Per annual leave cycle
            Self::Maternity => 120,          // 4 months
            Self::Parental => 10,            // New fathers/adoptive parents
            Self::Adoption => 70,            // 10 weeks
            Self::CommissioningParental => 70,
        }
    }

    /// Check if leave is paid
    pub fn is_paid(&self) -> bool {
        match self {
            Self::Annual => true,
            Self::Sick => true, // Full pay first 6 weeks, half pay next
            Self::FamilyResponsibility => true,
            Self::Maternity => false, // UIF provides benefits
            Self::Parental => false,  // UIF provides benefits
            Self::Adoption => false,
            Self::CommissioningParental => false,
        }
    }
}

/// Termination types under LRA
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TerminationType {
    /// Resignation by employee
    Resignation,
    /// Mutual agreement
    MutualAgreement,
    /// Dismissal for misconduct (s188)
    DismissalMisconduct,
    /// Dismissal for incapacity (s188)
    DismissalIncapacity,
    /// Dismissal for operational requirements/retrenchment (s189)
    Retrenchment,
    /// Constructive dismissal
    ConstructiveDismissal,
    /// End of fixed-term contract
    ContractExpiry,
    /// Retirement
    Retirement,
    /// Death
    Death,
}

impl TerminationType {
    /// Minimum notice period under BCEA s37
    pub fn notice_period_weeks(&self) -> u32 {
        match self {
            Self::Resignation | Self::DismissalMisconduct | Self::DismissalIncapacity => {
                1 // 1 week if employed < 6 months (increases with tenure)
            }
            Self::Retrenchment => 4,    // Minimum 4 weeks for retrenchment
            Self::MutualAgreement => 0, // As agreed
            Self::ContractExpiry => 0,  // As per contract
            Self::Retirement | Self::Death => 0,
            Self::ConstructiveDismissal => 0,
        }
    }

    /// Check if severance pay required (s41)
    pub fn requires_severance(&self) -> bool {
        matches!(self, Self::Retrenchment)
    }
}

/// Severance calculation (s41 BCEA - retrenchment only)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeverancePay {
    /// Years of service
    pub years_of_service: u32,
    /// Weekly remuneration
    pub weekly_remuneration: Zar,
    /// Total severance (1 week per year)
    pub severance_amount: Zar,
}

impl SeverancePay {
    /// Calculate severance pay
    /// Minimum 1 week's remuneration per completed year of service
    pub fn calculate(years_of_service: u32, weekly_remuneration: Zar) -> Self {
        let severance_amount =
            Zar::from_cents(weekly_remuneration.cents() * years_of_service as i64);

        Self {
            years_of_service,
            weekly_remuneration,
            severance_amount,
        }
    }
}

/// Unfair dismissal under LRA s188-191
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnfairDismissalType {
    /// No fair reason (substantive unfairness)
    NoFairReason,
    /// Unfair procedure (procedural unfairness)
    UnfairProcedure,
    /// Automatically unfair (s187 - protected grounds)
    AutomaticallyUnfair(AutomaticallyUnfairGround),
}

/// Automatically unfair dismissal grounds (s187 LRA)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AutomaticallyUnfairGround {
    /// Participation in protected strike
    ProtectedStrike,
    /// Trade union membership/activities
    TradeUnionActivity,
    /// Pregnancy, intended pregnancy, or related
    Pregnancy,
    /// Arbitrary discrimination (race, gender, etc.)
    Discrimination,
    /// Exercise of rights under LRA
    ExercisingRights,
    /// Refusal to do work of striking employee
    RefusingStrikeWork,
    /// Transfer of business (TUPE)
    TransferOfBusiness,
    /// Whistleblowing (Protected Disclosures Act)
    ProtectedDisclosure,
}

/// CCMA referral timeframes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CcmaTimeframes {
    /// Days to refer unfair dismissal dispute
    pub unfair_dismissal_days: u32,
    /// Days to refer unfair labour practice
    pub unfair_labour_practice_days: u32,
    /// Days for conciliation
    pub conciliation_days: u32,
}

impl Default for CcmaTimeframes {
    fn default() -> Self {
        Self {
            unfair_dismissal_days: 30,
            unfair_labour_practice_days: 90,
            conciliation_days: 30,
        }
    }
}

/// Labor law errors
#[derive(Debug, Error)]
pub enum LaborError {
    /// Working hours exceeded - BCEA s9-10
    #[error(
        "Working hours exceed BCEA limits (s9-10): {hours} hours/week (max 45 ordinary + 10 overtime)"
    )]
    WorkingHoursExceeded { hours: u32 },

    /// Minimum wage violation - NMWA
    #[error("Below national minimum wage: R{actual} (minimum R{minimum})")]
    MinimumWageViolation { actual: i64, minimum: i64 },

    /// Unfair dismissal - LRA s188
    #[error("Unfair dismissal (LRA s188): {reason}")]
    UnfairDismissal { reason: String },

    /// Insufficient notice - BCEA s37
    #[error("Insufficient notice period: {provided} weeks (minimum {required})")]
    InsufficientNotice { provided: u32, required: u32 },

    /// CCMA time bar
    #[error("CCMA referral time-barred: {days} days elapsed (limit {limit})")]
    CcmaTimeBarred { days: u32, limit: u32 },

    /// Leave violation - BCEA s20-27
    #[error("Leave entitlement violation ({leave_type}): {description}")]
    LeaveViolation {
        leave_type: String,
        description: String,
    },
}

/// Validate working hours
pub fn validate_working_hours(hours: &WorkingHours) -> LaborResult<()> {
    if !hours.is_within_limits() {
        return Err(LaborError::WorkingHoursExceeded {
            hours: hours.total_weekly_hours(),
        });
    }
    Ok(())
}

/// Calculate severance pay
pub fn calculate_severance(years_of_service: u32, weekly_remuneration: Zar) -> SeverancePay {
    SeverancePay::calculate(years_of_service, weekly_remuneration)
}

/// Get labor law compliance checklist
pub fn get_labor_checklist() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Written employment contract", "BCEA s29"),
        ("Maximum 45 ordinary hours/week", "BCEA s9"),
        ("Maximum 10 overtime hours/week", "BCEA s10"),
        ("Overtime rate 1.5x normal", "BCEA s10"),
        ("21 days annual leave", "BCEA s20"),
        ("Sick leave cycle 30 days/3 years", "BCEA s22"),
        ("4 months maternity leave", "BCEA s25"),
        ("10 days parental leave", "BCEA s25A"),
        ("UIF registration", "UIF Act"),
        ("National minimum wage", "NMWA"),
        ("Fair dismissal procedure", "LRA s188"),
        ("1 week severance per year (retrenchment)", "BCEA s41"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_working_hours_standard() {
        let hours = WorkingHours::standard();
        assert!(hours.is_within_limits());
        assert_eq!(hours.total_weekly_hours(), 45);
    }

    #[test]
    fn test_working_hours_exceeded() {
        let hours = WorkingHours {
            ordinary_hours_per_week: 50, // Exceeds 45
            days_per_week: 5,
            overtime_per_week: 0,
        };
        assert!(!hours.is_within_limits());
    }

    #[test]
    fn test_leave_entitlements() {
        assert_eq!(LeaveType::Annual.statutory_days(), 21);
        assert_eq!(LeaveType::Maternity.statutory_days(), 120);
        assert_eq!(LeaveType::Parental.statutory_days(), 10);
    }

    #[test]
    fn test_severance_calculation() {
        let weekly = Zar::from_rands(5000);
        let severance = SeverancePay::calculate(5, weekly);
        assert_eq!(severance.severance_amount.rands(), 25000); // 5 weeks
    }

    #[test]
    fn test_termination_notice() {
        assert_eq!(TerminationType::Retrenchment.notice_period_weeks(), 4);
        assert!(TerminationType::Retrenchment.requires_severance());
        assert!(!TerminationType::Resignation.requires_severance());
    }

    #[test]
    fn test_ccma_timeframes() {
        let timeframes = CcmaTimeframes::default();
        assert_eq!(timeframes.unfair_dismissal_days, 30);
    }

    #[test]
    fn test_contract_types() {
        let permanent = ContractType::Permanent;
        assert!(permanent.has_full_protection());

        let short_term = ContractType::FixedTerm { duration_months: 2 };
        assert!(!short_term.has_full_protection());
    }

    #[test]
    fn test_labor_checklist() {
        let checklist = get_labor_checklist();
        assert!(!checklist.is_empty());
    }

    #[test]
    fn test_validate_working_hours() {
        let valid = WorkingHours::standard();
        assert!(validate_working_hours(&valid).is_ok());

        let invalid = WorkingHours {
            ordinary_hours_per_week: 50,
            days_per_week: 5,
            overtime_per_week: 15,
        };
        assert!(validate_working_hours(&invalid).is_err());
    }
}
