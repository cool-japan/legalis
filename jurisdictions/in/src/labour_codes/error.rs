//! Labour Codes 2020 Error Types
//!
//! Error types for India's consolidated Labour Codes

use crate::citation::{Citation, cite};
use thiserror::Error;

/// Labour Codes errors
#[derive(Debug, Clone, Error)]
pub enum LabourCodeError {
    // Code on Wages errors
    /// Below minimum wage
    #[error("Code on Wages Section 9: Wages below minimum wage floor")]
    BelowMinimumWage {
        actual: f64,
        minimum: f64,
        shortfall: f64,
    },

    /// Payment delayed
    #[error("Code on Wages Section 17: Wages not paid within prescribed time")]
    PaymentDelayed { days_late: u32 },

    /// Unauthorized deduction
    #[error("Code on Wages Section 18: Unauthorized deduction from wages")]
    UnauthorizedDeduction { deduction_type: String, amount: f64 },

    /// Excess deduction
    #[error("Code on Wages Section 18: Deductions exceed permissible limit")]
    ExcessDeduction {
        total_deduction: f64,
        max_allowed: f64,
    },

    /// Bonus not paid
    #[error("Code on Wages Section 26: Minimum bonus not paid to eligible employee")]
    BonusNotPaid { employee: String, amount_due: f64 },

    /// Equal pay violation
    #[error("Code on Wages Section 3: Discrimination in wages based on gender")]
    EqualPayViolation,

    // Code on Social Security errors
    /// EPF not deposited
    #[error("Code on Social Security Section 6: EPF contribution not deposited")]
    EpfNotDeposited { amount: f64, months: u32 },

    /// ESI not deposited
    #[error("Code on Social Security Section 28: ESI contribution not deposited")]
    EsiNotDeposited { amount: f64, months: u32 },

    /// Gratuity not paid
    #[error("Code on Social Security Section 53: Gratuity not paid within 30 days")]
    GratuityNotPaid { amount: f64, service_years: f64 },

    /// Maternity benefit denied
    #[error("Code on Social Security Section 60: Maternity benefit denied")]
    MaternityBenefitDenied { reason: String },

    /// Gig worker not registered
    #[error("Code on Social Security Section 114: Gig worker not registered for social security")]
    GigWorkerNotRegistered,

    /// Platform worker not registered
    #[error("Code on Social Security Section 114: Platform worker not registered")]
    PlatformWorkerNotRegistered,

    // Industrial Relations Code errors
    /// Unfair labour practice
    #[error("Industrial Relations Code Section 2(zc): Unfair labour practice")]
    UnfairLabourPractice { practice: String },

    /// Illegal strike
    #[error("Industrial Relations Code Section 62: Illegal strike in progress")]
    IllegalStrike { reason: String },

    /// Illegal lockout
    #[error("Industrial Relations Code Section 62: Illegal lockout declared")]
    IllegalLockout { reason: String },

    /// Strike notice not given
    #[error("Industrial Relations Code Section 62: Strike notice not given 14 days in advance")]
    StrikeNoticeNotGiven,

    /// Lockout notice not given
    #[error("Industrial Relations Code Section 62: Lockout notice not given 14 days in advance")]
    LockoutNoticeNotGiven,

    /// Retrenchment without permission
    #[error("Industrial Relations Code Section 79: Retrenchment without government permission")]
    RetrenchmentWithoutPermission { workers_affected: u32 },

    /// Layoff without permission
    #[error("Industrial Relations Code Section 78: Layoff without government permission")]
    LayoffWithoutPermission { workers_affected: u32 },

    /// Closure without notice
    #[error("Industrial Relations Code Section 81: Closure without 60 days notice")]
    ClosureWithoutNotice,

    /// Standing orders not certified
    #[error("Industrial Relations Code Section 30: Standing orders not certified")]
    StandingOrdersNotCertified,

    /// Trade union not registered
    #[error("Industrial Relations Code Section 6: Trade union not registered")]
    TradeUnionNotRegistered,

    /// Grievance not addressed
    #[error("Industrial Relations Code Section 4: Grievance not addressed within 30 days")]
    GrievanceNotAddressed { grievance_id: String },

    // OSH Code errors
    /// Working hours exceeded
    #[error("OSH Code Section 25: Working hours exceed maximum limit")]
    WorkingHoursExceeded { actual: f64, maximum: f64 },

    /// Weekly off not provided
    #[error("OSH Code Section 28: Weekly off not provided")]
    WeeklyOffNotProvided,

    /// Overtime not paid
    #[error("OSH Code Section 27: Overtime wages not paid at double rate")]
    OvertimeNotPaid { hours: f64, amount_due: f64 },

    /// Annual leave not granted
    #[error("OSH Code Section 32: Annual leave not granted")]
    AnnualLeaveNotGranted { days_due: u32 },

    /// Safety committee not constituted
    #[error("OSH Code Section 22: Safety committee not constituted")]
    SafetyCommitteeNotConstituted { worker_count: u32 },

    /// Safety officer not appointed
    #[error("OSH Code Section 22: Safety officer not appointed")]
    SafetyOfficerNotAppointed,

    /// Hazardous work without precautions
    #[error("OSH Code Section 23: Hazardous work without adequate safety precautions")]
    HazardousWorkUnsafe { hazard: String },

    /// Contract labour in core activity
    #[error("OSH Code Section 67: Contract labour engaged in core activity")]
    ContractLabourInCore,

    /// Migrant worker facilities not provided
    #[error("OSH Code Section 74: Facilities not provided to inter-state migrant workers")]
    MigrantWorkerFacilitiesNotProvided,

    /// Establishment not registered
    #[error("OSH Code Section 3: Establishment not registered")]
    EstablishmentNotRegistered,

    /// License not obtained
    #[error("OSH Code: Required license not obtained")]
    LicenseNotObtained { license_type: String },

    /// Inspection compliance failure
    #[error("OSH Code Section 35: Non-compliance with inspection findings")]
    InspectionNonCompliance { findings: String },

    // General errors
    /// Records not maintained
    #[error("{code} Section {section}: Required records not maintained")]
    RecordsNotMaintained { code: String, section: u32 },

    /// Returns not filed
    #[error("{code}: Required returns not filed")]
    ReturnsNotFiled { code: String, return_type: String },

    /// Notice not displayed
    #[error("{code}: Required notice/abstract not displayed")]
    NoticeNotDisplayed { code: String },

    /// Validation error
    #[error("Labour code validation error: {message}")]
    ValidationError { message: String },
}

impl LabourCodeError {
    /// Get relevant citation
    pub fn citation(&self) -> Option<Citation> {
        match self {
            Self::BelowMinimumWage { .. } => Some(cite::wages_code(9)),
            Self::PaymentDelayed { .. } => Some(cite::wages_code(17)),
            Self::UnauthorizedDeduction { .. } | Self::ExcessDeduction { .. } => {
                Some(cite::wages_code(18))
            }
            Self::BonusNotPaid { .. } => Some(cite::wages_code(26)),
            Self::EqualPayViolation => Some(cite::wages_code(3)),
            Self::EpfNotDeposited { .. } => Some(cite::social_security(6)),
            Self::EsiNotDeposited { .. } => Some(cite::social_security(28)),
            Self::GratuityNotPaid { .. } => Some(cite::social_security(53)),
            Self::MaternityBenefitDenied { .. } => Some(cite::social_security(60)),
            Self::GigWorkerNotRegistered | Self::PlatformWorkerNotRegistered => {
                Some(cite::social_security(114))
            }
            Self::UnfairLabourPractice { .. } => Some(cite::ir_code(2)),
            Self::IllegalStrike { .. }
            | Self::IllegalLockout { .. }
            | Self::StrikeNoticeNotGiven
            | Self::LockoutNoticeNotGiven => Some(cite::ir_code(62)),
            Self::RetrenchmentWithoutPermission { .. } => Some(cite::ir_code(79)),
            Self::LayoffWithoutPermission { .. } => Some(cite::ir_code(78)),
            Self::ClosureWithoutNotice => Some(cite::ir_code(81)),
            Self::StandingOrdersNotCertified => Some(cite::ir_code(30)),
            Self::TradeUnionNotRegistered => Some(cite::ir_code(6)),
            Self::GrievanceNotAddressed { .. } => Some(cite::ir_code(4)),
            Self::WorkingHoursExceeded { .. } => Some(cite::osh_code(25)),
            Self::WeeklyOffNotProvided => Some(cite::osh_code(28)),
            Self::OvertimeNotPaid { .. } => Some(cite::osh_code(27)),
            Self::AnnualLeaveNotGranted { .. } => Some(cite::osh_code(32)),
            Self::SafetyCommitteeNotConstituted { .. } | Self::SafetyOfficerNotAppointed => {
                Some(cite::osh_code(22))
            }
            Self::HazardousWorkUnsafe { .. } => Some(cite::osh_code(23)),
            Self::ContractLabourInCore => Some(cite::osh_code(67)),
            Self::MigrantWorkerFacilitiesNotProvided => Some(cite::osh_code(74)),
            Self::EstablishmentNotRegistered => Some(cite::osh_code(3)),
            Self::InspectionNonCompliance { .. } => Some(cite::osh_code(35)),
            _ => None,
        }
    }

    /// Get penalty information
    pub fn penalty_info(&self) -> LabourPenaltyInfo {
        match self {
            Self::BelowMinimumWage { .. } => LabourPenaltyInfo {
                fine_min: Some(50_000),
                fine_max: Some(500_000),
                imprisonment_months: Some(3),
                repeat_multiplier: 2.0,
                compoundable: true,
            },
            Self::EpfNotDeposited { .. } | Self::EsiNotDeposited { .. } => LabourPenaltyInfo {
                fine_min: Some(100_000),
                fine_max: Some(500_000),
                imprisonment_months: Some(24),
                repeat_multiplier: 2.0,
                compoundable: false,
            },
            Self::IllegalStrike { .. } | Self::IllegalLockout { .. } => LabourPenaltyInfo {
                fine_min: Some(50_000),
                fine_max: Some(200_000),
                imprisonment_months: Some(1),
                repeat_multiplier: 2.0,
                compoundable: true,
            },
            Self::WorkingHoursExceeded { .. } | Self::OvertimeNotPaid { .. } => LabourPenaltyInfo {
                fine_min: Some(50_000),
                fine_max: Some(200_000),
                imprisonment_months: None,
                repeat_multiplier: 2.0,
                compoundable: true,
            },
            Self::SafetyCommitteeNotConstituted { .. } | Self::SafetyOfficerNotAppointed => {
                LabourPenaltyInfo {
                    fine_min: Some(200_000),
                    fine_max: Some(500_000),
                    imprisonment_months: Some(6),
                    repeat_multiplier: 2.0,
                    compoundable: false,
                }
            }
            Self::HazardousWorkUnsafe { .. } => LabourPenaltyInfo {
                fine_min: Some(200_000),
                fine_max: Some(500_000),
                imprisonment_months: Some(12),
                repeat_multiplier: 3.0,
                compoundable: false,
            },
            _ => LabourPenaltyInfo {
                fine_min: Some(50_000),
                fine_max: Some(200_000),
                imprisonment_months: None,
                repeat_multiplier: 2.0,
                compoundable: true,
            },
        }
    }

    /// Get remedial action
    pub fn remedial_action(&self) -> &'static str {
        match self {
            Self::BelowMinimumWage { .. } => "Immediately pay arrears with interest at 10% p.a.",
            Self::PaymentDelayed { .. } => "Pay wages immediately with compensation",
            Self::EpfNotDeposited { .. } => {
                "Deposit dues with interest and damages through EPFO portal"
            }
            Self::EsiNotDeposited { .. } => "Deposit dues through ESIC portal",
            Self::GratuityNotPaid { .. } => "Pay gratuity within 30 days with simple interest",
            Self::IllegalStrike { .. } => {
                "Workers to resume work; negotiate through legal channels"
            }
            Self::WorkingHoursExceeded { .. } => {
                "Reduce working hours to legal limits; pay overtime"
            }
            Self::SafetyCommitteeNotConstituted { .. } => {
                "Constitute safety committee with worker representatives"
            }
            Self::EstablishmentNotRegistered => {
                "Apply for registration within 60 days of operations"
            }
            _ => "Take immediate corrective action and maintain compliance records",
        }
    }
}

/// Labour penalty information
#[derive(Debug, Clone)]
pub struct LabourPenaltyInfo {
    /// Minimum fine (Rs.)
    pub fine_min: Option<u64>,
    /// Maximum fine (Rs.)
    pub fine_max: Option<u64>,
    /// Maximum imprisonment (months)
    pub imprisonment_months: Option<u32>,
    /// Multiplier for repeat offence
    pub repeat_multiplier: f64,
    /// Can be compounded
    pub compoundable: bool,
}

/// Labour compliance report
#[derive(Debug, Clone, Default)]
pub struct LabourComplianceReport {
    /// Overall compliance
    pub compliant: bool,
    /// Code on Wages compliance
    pub wages_compliant: bool,
    /// Social Security compliance
    pub social_security_compliant: bool,
    /// IR Code compliance
    pub ir_compliant: bool,
    /// OSH Code compliance
    pub osh_compliant: bool,
    /// Violations
    pub violations: Vec<LabourCodeError>,
    /// Warnings
    pub warnings: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Penalty exposure
    pub penalty_exposure: f64,
}

/// Result type for labour code operations
pub type LabourCodeResult<T> = Result<T, LabourCodeError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_wage_error() {
        let error = LabourCodeError::BelowMinimumWage {
            actual: 8000.0,
            minimum: 10000.0,
            shortfall: 2000.0,
        };
        let citation = error.citation().expect("Should have citation");
        assert_eq!(citation.number, 9);
    }

    #[test]
    fn test_epf_penalty() {
        let error = LabourCodeError::EpfNotDeposited {
            amount: 50000.0,
            months: 3,
        };
        let penalty = error.penalty_info();
        assert_eq!(penalty.imprisonment_months, Some(24));
    }

    #[test]
    fn test_safety_violation_penalty() {
        let error = LabourCodeError::HazardousWorkUnsafe {
            hazard: "Chemical exposure".to_string(),
        };
        let penalty = error.penalty_info();
        assert_eq!(penalty.repeat_multiplier, 3.0);
    }

    #[test]
    fn test_compoundable_offence() {
        let error = LabourCodeError::WorkingHoursExceeded {
            actual: 52.0,
            maximum: 48.0,
        };
        assert!(error.penalty_info().compoundable);

        let error = LabourCodeError::EpfNotDeposited {
            amount: 50000.0,
            months: 3,
        };
        assert!(!error.penalty_info().compoundable);
    }
}
