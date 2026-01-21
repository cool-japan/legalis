//! Companies Act 2013 Error Types
//!
//! Error types for Indian corporate law compliance

use crate::citation::{Citation, cite};
use thiserror::Error;

/// Companies Act 2013 compliance errors
#[derive(Debug, Clone, Error)]
pub enum CompaniesActError {
    // Director-related errors
    /// Insufficient directors (Section 149)
    #[error(
        "Companies Act Section 149: Minimum {required} directors required, only {actual} appointed"
    )]
    InsufficientDirectors { required: u32, actual: u32 },

    /// No resident director (Section 149(3))
    #[error(
        "Companies Act Section 149(3): At least one director must stay in India for at least 182 days"
    )]
    NoResidentDirector,

    /// Insufficient independent directors (Section 149(4))
    #[error(
        "Companies Act Section 149(4): Listed company requires minimum 1/3 independent directors"
    )]
    InsufficientIndependentDirectors { required: u32, actual: u32 },

    /// No woman director (Section 149(1))
    #[error(
        "Companies Act Section 149(1): Woman director required for prescribed class of companies"
    )]
    NoWomanDirector,

    /// Director disqualified (Section 164)
    #[error("Companies Act Section 164: Director {din} is disqualified under Section 164")]
    DirectorDisqualified { din: String },

    /// DIN not approved
    #[error("Companies Act Section 153: Director {din} does not have approved DIN status")]
    DinNotApproved { din: String },

    /// Directorship limit exceeded (Section 165)
    #[error("Companies Act Section 165: Director {din} exceeds maximum number of directorships")]
    DirectorshipLimitExceeded { din: String },

    /// Independent director term exceeded (Section 149(10))
    #[error(
        "Companies Act Section 149(10): Independent director cannot hold office for more than two consecutive terms of five years each"
    )]
    IndependentDirectorTermExceeded { din: String },

    // KMP-related errors
    /// No Company Secretary (Section 203)
    #[error(
        "Companies Act Section 203: Company Secretary required for prescribed class of companies"
    )]
    NoCompanySecretary,

    /// No CFO (Section 203)
    #[error(
        "Companies Act Section 203: Chief Financial Officer required for prescribed class of companies"
    )]
    NoCfo,

    /// Duplicate KMP position
    #[error("Companies Act Section 203: Multiple persons appointed to same KMP position")]
    DuplicateKmpPosition,

    // Committee-related errors
    /// No Audit Committee (Section 177)
    #[error(
        "Companies Act Section 177: Audit Committee required for prescribed class of companies"
    )]
    NoAuditCommittee,

    /// Audit Committee composition invalid (Section 177(2))
    #[error(
        "Companies Act Section 177(2): Audit Committee must have minimum 3 directors with majority being independent"
    )]
    InvalidAuditCommitteeComposition,

    /// No Nomination and Remuneration Committee (Section 178)
    #[error("Companies Act Section 178: Nomination and Remuneration Committee required")]
    NoNominationRemunerationCommittee,

    /// No Stakeholders Relationship Committee (Section 178)
    #[error(
        "Companies Act Section 178: Stakeholders Relationship Committee required for companies with >1000 shareholders"
    )]
    NoStakeholdersCommittee,

    /// No CSR Committee (Section 135)
    #[error(
        "Companies Act Section 135: CSR Committee required for companies meeting threshold criteria"
    )]
    NoCsrCommittee,

    // Meeting-related errors
    /// AGM not held (Section 96)
    #[error("Companies Act Section 96: Annual General Meeting must be held within stipulated time")]
    AgmNotHeld,

    /// Board meeting gap exceeded (Section 173)
    #[error("Companies Act Section 173: Gap between two board meetings cannot exceed 120 days")]
    BoardMeetingGapExceeded { days: u32 },

    /// Insufficient board meetings (Section 173)
    #[error("Companies Act Section 173: Minimum 4 board meetings required per year")]
    InsufficientBoardMeetings { count: u32 },

    /// Quorum not present (Section 174)
    #[error("Companies Act Section 174: Quorum not present for meeting")]
    QuorumNotPresent,

    // Resolution-related errors
    /// Special resolution required (Section 114)
    #[error("Companies Act Section 114: Special resolution required for this matter")]
    SpecialResolutionRequired,

    /// Resolution not passed
    #[error("Companies Act: Resolution not passed, required {required}% votes, received {actual}%")]
    ResolutionNotPassed { required: f64, actual: f64 },

    // Filing-related errors
    /// Annual return not filed (Section 92)
    #[error("Companies Act Section 92: Annual return (MGT-7) not filed within stipulated time")]
    AnnualReturnNotFiled,

    /// Financial statements not filed (Section 137)
    #[error(
        "Companies Act Section 137: Financial statements (AOC-4) not filed within stipulated time"
    )]
    FinancialStatementsNotFiled,

    /// Form filing delayed
    #[error("Companies Act: Form {form} filing delayed by {days} days")]
    FilingDelayed { form: String, days: u32 },

    // Share capital errors
    /// Authorized capital exceeded
    #[error("Companies Act Section 61: Issued capital cannot exceed authorized capital")]
    AuthorizedCapitalExceeded,

    /// Share allotment violation
    #[error(
        "Companies Act Section 62: Share allotment must comply with preferential allotment rules"
    )]
    ShareAllotmentViolation { reason: String },

    /// Buy-back limit exceeded (Section 68)
    #[error(
        "Companies Act Section 68: Buy-back cannot exceed 25% of paid-up capital and free reserves"
    )]
    BuybackLimitExceeded,

    // CSR errors
    /// CSR spending shortfall (Section 135)
    #[error(
        "Companies Act Section 135: CSR spending of Rs. {spent} is less than required Rs. {required}"
    )]
    CsrSpendingShortfall { required: i64, spent: i64 },

    /// CSR activity not in Schedule VII
    #[error("Companies Act Schedule VII: CSR activity not covered under permissible activities")]
    InvalidCsrActivity,

    // Related party transaction errors
    /// RPT not approved (Section 188)
    #[error(
        "Companies Act Section 188: Related party transaction requires Board/Shareholder approval"
    )]
    RptNotApproved,

    /// RPT arm's length violation (Section 188)
    #[error("Companies Act Section 188: Related party transaction not at arm's length price")]
    RptArmsLengthViolation,

    // Audit errors
    /// Auditor not appointed (Section 139)
    #[error("Companies Act Section 139: Auditor must be appointed within 30 days of incorporation")]
    AuditorNotAppointed,

    /// Auditor tenure exceeded (Section 139(2))
    #[error(
        "Companies Act Section 139(2): Same auditor cannot be appointed for more than one term of 5 consecutive years"
    )]
    AuditorTenureExceeded,

    /// Auditor rotation not done (Section 139(2))
    #[error(
        "Companies Act Section 139(2): Mandatory auditor rotation for prescribed class of companies"
    )]
    AuditorRotationRequired,

    // Charge-related errors
    /// Charge not registered (Section 77)
    #[error("Companies Act Section 77: Charge must be registered within 30 days of creation")]
    ChargeNotRegistered,

    /// Charge registration delayed
    #[error("Companies Act Section 77: Charge registration delayed, additional fee applicable")]
    ChargeRegistrationDelayed { days: u32 },

    // General errors
    /// Invalid CIN format
    #[error("Companies Act: Invalid Corporate Identity Number (CIN) format")]
    InvalidCinFormat,

    /// Company not active
    #[error("Companies Act: Company is not in Active status")]
    CompanyNotActive,

    /// Validation error
    #[error("Companies Act validation error: {message}")]
    ValidationError { message: String },
}

impl CompaniesActError {
    /// Get relevant citation
    pub fn citation(&self) -> Option<Citation> {
        match self {
            Self::InsufficientDirectors { .. }
            | Self::NoResidentDirector
            | Self::InsufficientIndependentDirectors { .. }
            | Self::NoWomanDirector => Some(cite::companies_act(149)),
            Self::DirectorDisqualified { .. } => Some(cite::companies_act(164)),
            Self::DirectorshipLimitExceeded { .. } => Some(cite::companies_act(165)),
            Self::IndependentDirectorTermExceeded { .. } => Some(cite::companies_act(149)),
            Self::DinNotApproved { .. } => Some(cite::companies_act(153)),
            Self::NoCompanySecretary | Self::NoCfo | Self::DuplicateKmpPosition => {
                Some(cite::companies_act(203))
            }
            Self::NoAuditCommittee | Self::InvalidAuditCommitteeComposition => {
                Some(cite::companies_act(177))
            }
            Self::NoNominationRemunerationCommittee | Self::NoStakeholdersCommittee => {
                Some(cite::companies_act(178))
            }
            Self::NoCsrCommittee | Self::CsrSpendingShortfall { .. } | Self::InvalidCsrActivity => {
                Some(cite::companies_act(135))
            }
            Self::AgmNotHeld => Some(cite::companies_act(96)),
            Self::BoardMeetingGapExceeded { .. }
            | Self::InsufficientBoardMeetings { .. }
            | Self::QuorumNotPresent => Some(cite::companies_act(173)),
            Self::SpecialResolutionRequired | Self::ResolutionNotPassed { .. } => {
                Some(cite::companies_act(114))
            }
            Self::AnnualReturnNotFiled => Some(cite::companies_act(92)),
            Self::FinancialStatementsNotFiled => Some(cite::companies_act(137)),
            Self::AuthorizedCapitalExceeded => Some(cite::companies_act(61)),
            Self::ShareAllotmentViolation { .. } => Some(cite::companies_act(62)),
            Self::BuybackLimitExceeded => Some(cite::companies_act(68)),
            Self::RptNotApproved | Self::RptArmsLengthViolation => Some(cite::companies_act(188)),
            Self::AuditorNotAppointed
            | Self::AuditorTenureExceeded
            | Self::AuditorRotationRequired => Some(cite::companies_act(139)),
            Self::ChargeNotRegistered | Self::ChargeRegistrationDelayed { .. } => {
                Some(cite::companies_act(77))
            }
            Self::FilingDelayed { .. }
            | Self::InvalidCinFormat
            | Self::CompanyNotActive
            | Self::ValidationError { .. } => None,
        }
    }

    /// Get penalty information
    pub fn penalty_info(&self) -> PenaltyInfo {
        match self {
            Self::InsufficientDirectors { .. }
            | Self::NoResidentDirector
            | Self::NoWomanDirector => PenaltyInfo {
                company_penalty: Some((50_000, 500_000)),
                officer_penalty: Some((50_000, 500_000)),
                continuing_penalty: Some(1_000),
                imprisonment: None,
            },
            Self::DirectorDisqualified { .. } => PenaltyInfo {
                company_penalty: None,
                officer_penalty: Some((100_000, 500_000)),
                continuing_penalty: None,
                imprisonment: Some("Up to 1 year"),
            },
            Self::NoAuditCommittee | Self::InvalidAuditCommitteeComposition => PenaltyInfo {
                company_penalty: Some((100_000, 500_000)),
                officer_penalty: Some((25_000, 100_000)),
                continuing_penalty: None,
                imprisonment: None,
            },
            Self::CsrSpendingShortfall { .. } => PenaltyInfo {
                company_penalty: Some((50_000, 2_500_000)),
                officer_penalty: Some((50_000, 500_000)),
                continuing_penalty: None,
                imprisonment: Some("Up to 3 years"),
            },
            Self::AgmNotHeld => PenaltyInfo {
                company_penalty: Some((100_000, 500_000)),
                officer_penalty: Some((50_000, 100_000)),
                continuing_penalty: None,
                imprisonment: None,
            },
            Self::AnnualReturnNotFiled | Self::FinancialStatementsNotFiled => PenaltyInfo {
                company_penalty: Some((50_000, 500_000)),
                officer_penalty: Some((50_000, 500_000)),
                continuing_penalty: Some(100),
                imprisonment: None,
            },
            Self::RptNotApproved | Self::RptArmsLengthViolation => PenaltyInfo {
                company_penalty: Some((2_500_000, 2_500_000)),
                officer_penalty: Some((25_000, 500_000)),
                continuing_penalty: None,
                imprisonment: Some("Up to 1 year"),
            },
            _ => PenaltyInfo {
                company_penalty: Some((10_000, 100_000)),
                officer_penalty: Some((10_000, 100_000)),
                continuing_penalty: None,
                imprisonment: None,
            },
        }
    }
}

/// Penalty information structure
#[derive(Debug, Clone)]
pub struct PenaltyInfo {
    /// Penalty on company (min, max) in rupees
    pub company_penalty: Option<(u64, u64)>,
    /// Penalty on officers in default (min, max) in rupees
    pub officer_penalty: Option<(u64, u64)>,
    /// Continuing penalty per day
    pub continuing_penalty: Option<u64>,
    /// Imprisonment provision
    pub imprisonment: Option<&'static str>,
}

impl PenaltyInfo {
    /// Format penalty description
    pub fn format(&self) -> String {
        let mut parts = Vec::new();

        if let Some((min, max)) = self.company_penalty {
            parts.push(format!(
                "Company: Rs. {} to Rs. {}",
                format_amount(min),
                format_amount(max)
            ));
        }

        if let Some((min, max)) = self.officer_penalty {
            parts.push(format!(
                "Officers: Rs. {} to Rs. {}",
                format_amount(min),
                format_amount(max)
            ));
        }

        if let Some(daily) = self.continuing_penalty {
            parts.push(format!("Continuing: Rs. {} per day", format_amount(daily)));
        }

        if let Some(imprisonment) = self.imprisonment {
            parts.push(format!("Imprisonment: {}", imprisonment));
        }

        parts.join("; ")
    }
}

fn format_amount(amount: u64) -> String {
    if amount >= 10_000_000 {
        format!("{} crore", amount / 10_000_000)
    } else if amount >= 100_000 {
        format!("{} lakh", amount / 100_000)
    } else {
        amount.to_string()
    }
}

/// Result type for Companies Act operations
pub type CompaniesActResult<T> = Result<T, CompaniesActError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_citation() {
        let error = CompaniesActError::NoResidentDirector;
        let citation = error.citation().expect("Should have citation");
        assert_eq!(citation.number, 149);
    }

    #[test]
    fn test_penalty_info() {
        let error = CompaniesActError::AgmNotHeld;
        let penalty = error.penalty_info();
        assert!(penalty.company_penalty.is_some());
        assert!(penalty.officer_penalty.is_some());
    }

    #[test]
    fn test_csr_penalty() {
        let error = CompaniesActError::CsrSpendingShortfall {
            required: 10_000_000,
            spent: 5_000_000,
        };
        let penalty = error.penalty_info();
        assert!(penalty.imprisonment.is_some());
    }

    #[test]
    fn test_rpt_citation() {
        let error = CompaniesActError::RptNotApproved;
        let citation = error.citation().expect("Should have citation");
        assert_eq!(citation.number, 188);
    }

    #[test]
    fn test_format_amount() {
        assert_eq!(format_amount(50_000_000), "5 crore");
        assert_eq!(format_amount(500_000), "5 lakh");
        assert_eq!(format_amount(50_000), "50000");
    }
}
