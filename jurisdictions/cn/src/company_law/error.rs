//! Company Law Error Types
//!
//! # 公司法错误类型

#![allow(missing_docs)]

use crate::citation::{Citation, cite};
use crate::i18n::BilingualText;
use thiserror::Error;

/// Company Law compliance errors
#[derive(Debug, Clone, Error)]
pub enum CompanyLawError {
    /// Insufficient shareholders
    #[error(
        "Company Law Article 23: {company_type} requires at least {min} shareholder(s), got {actual}"
    )]
    InsufficientShareholders {
        company_type: String,
        min: u32,
        actual: u32,
    },

    /// Too many shareholders for LLC
    #[error("Company Law Article 24: LLC cannot have more than 50 shareholders, got {actual}")]
    TooManyShareholders { actual: u32 },

    /// Invalid board composition
    #[error("Company Law Article {article}: Board composition invalid - {reason}")]
    InvalidBoardComposition { article: u8, reason: String },

    /// Missing supervisory board
    #[error("Company Law Article 69: Supervisory board or supervisor required")]
    MissingSupervisoryBoard,

    /// Insufficient employee supervisors
    #[error(
        "Company Law Article 71: Employee supervisors must be at least 1/3 of supervisory board"
    )]
    InsufficientEmployeeSupervisors,

    /// Insufficient independent directors
    #[error("Listed company regulation: Independent directors must be at least 1/3 of board")]
    InsufficientIndependentDirectors,

    /// Capital contribution deadline exceeded
    #[error("Company Law Article 28: Capital contribution deadline exceeded")]
    CapitalContributionOverdue,

    /// Non-monetary contribution not valued
    #[error("Company Law Article 27: Non-monetary contribution must be properly valued")]
    ContributionNotValued,

    /// Equity transfer without consent
    #[error(
        "Company Law Article 71: External equity transfer requires majority shareholder consent"
    )]
    EquityTransferNoConsent,

    /// Preemptive rights not respected
    #[error("Company Law Article 71: Preemptive rights of existing shareholders not respected")]
    PreemptiveRightsViolation,

    /// Notice period not satisfied
    #[error("Company Law Article 71: 30-day notice period for equity transfer not satisfied")]
    NoticePeriodNotSatisfied,

    /// Invalid resolution
    #[error("Company Law Article 66: {matter} requires special resolution (2/3 majority)")]
    InvalidResolution { matter: String },

    /// Meeting quorum not met
    #[error("Company Law: Shareholder meeting quorum not met")]
    QuorumNotMet,

    /// Dividend distribution violation
    #[error(
        "Company Law Article 166: Cannot distribute dividends before statutory reserve contribution"
    )]
    DividendBeforeReserve,

    /// Illegal profit distribution
    #[error("Company Law Article 166: Profit distribution violates statutory requirements")]
    IllegalProfitDistribution,

    /// Self-dealing by director
    #[error("Company Law Article 148: Director self-dealing requires shareholder approval")]
    DirectorSelfDealing,

    /// Director competition
    #[error("Company Law Article 148: Director competing with company without authorization")]
    DirectorCompetition,

    /// Director disqualification
    #[error("Company Law Article 146: Person disqualified from serving as director")]
    DirectorDisqualified { reason: String },

    /// Missing legal representative
    #[error("Company Law Article 13: Company must have a legal representative")]
    MissingLegalRepresentative,

    /// Articles of association missing essential content
    #[error("Company Law Article 25: Articles of association missing essential content: {missing}")]
    ArticlesMissingContent { missing: String },

    /// Liquidation procedure violation
    #[error("Company Law Article 183: Liquidation procedure violation - {issue}")]
    LiquidationViolation { issue: String },

    /// Piercing corporate veil
    #[error("Company Law Article 20: Shareholder abuse of limited liability")]
    PiercingCorporateVeil,

    /// General validation error
    #[error("Company Law validation error: {message}")]
    ValidationError { message: String },
}

impl CompanyLawError {
    /// Get relevant citation
    pub fn citation(&self) -> Option<Citation> {
        match self {
            Self::InsufficientShareholders { .. } => Some(cite::company_law(23)),
            Self::TooManyShareholders { .. } => Some(cite::company_law(24)),
            Self::InvalidBoardComposition { article, .. } => {
                Some(cite::company_law(u32::from(*article)))
            }
            Self::MissingSupervisoryBoard => Some(cite::company_law(69)),
            Self::InsufficientEmployeeSupervisors => Some(cite::company_law(71)),
            Self::InsufficientIndependentDirectors => None, // Listed company rules
            Self::CapitalContributionOverdue => Some(cite::company_law(28)),
            Self::ContributionNotValued => Some(cite::company_law(27)),
            Self::EquityTransferNoConsent => Some(cite::company_law(71)),
            Self::PreemptiveRightsViolation => Some(cite::company_law(71)),
            Self::NoticePeriodNotSatisfied => Some(cite::company_law(71)),
            Self::InvalidResolution { .. } => Some(cite::company_law(66)),
            Self::QuorumNotMet => Some(cite::company_law(41)),
            Self::DividendBeforeReserve => Some(cite::company_law(166)),
            Self::IllegalProfitDistribution => Some(cite::company_law(166)),
            Self::DirectorSelfDealing => Some(cite::company_law(148)),
            Self::DirectorCompetition => Some(cite::company_law(148)),
            Self::DirectorDisqualified { .. } => Some(cite::company_law(146)),
            Self::MissingLegalRepresentative => Some(cite::company_law(13)),
            Self::ArticlesMissingContent { .. } => Some(cite::company_law(25)),
            Self::LiquidationViolation { .. } => Some(cite::company_law(183)),
            Self::PiercingCorporateVeil => Some(cite::company_law(20)),
            Self::ValidationError { .. } => None,
        }
    }

    /// Get bilingual error message
    pub fn bilingual_message(&self) -> BilingualText {
        match self {
            Self::InsufficientShareholders {
                company_type,
                min,
                actual,
            } => BilingualText::new(
                format!("{}需要至少{}名股东，实际{}名", company_type, min, actual),
                format!(
                    "{} requires at least {} shareholder(s), got {}",
                    company_type, min, actual
                ),
            ),
            Self::TooManyShareholders { actual } => BilingualText::new(
                format!("有限责任公司股东不得超过50人，实际{}人", actual),
                format!("LLC cannot have more than 50 shareholders, got {}", actual),
            ),
            Self::EquityTransferNoConsent => BilingualText::new(
                "向股东以外的人转让股权应当经其他股东过半数同意",
                "External equity transfer requires consent from majority of other shareholders",
            ),
            Self::PreemptiveRightsViolation => BilingualText::new(
                "其他股东享有优先购买权",
                "Other shareholders have preemptive purchase rights",
            ),
            Self::DirectorSelfDealing => BilingualText::new(
                "董事与公司进行交易应当经股东会或者股东大会同意",
                "Director self-dealing requires shareholder approval",
            ),
            Self::PiercingCorporateVeil => BilingualText::new(
                "股东滥用公司法人独立地位和股东有限责任的，应当对公司债务承担连带责任",
                "Shareholders abusing limited liability shall bear joint liability for company debts",
            ),
            _ => BilingualText::new("公司法合规错误".to_string(), self.to_string()),
        }
    }

    /// Get potential liability
    pub fn liability_type(&self) -> LiabilityType {
        match self {
            Self::PiercingCorporateVeil => LiabilityType::JointLiability,
            Self::DirectorSelfDealing | Self::DirectorCompetition => {
                LiabilityType::DamagesLiability
            }
            Self::LiquidationViolation { .. } => LiabilityType::PersonalLiability,
            Self::CapitalContributionOverdue => LiabilityType::ContributionLiability,
            _ => LiabilityType::Administrative,
        }
    }
}

/// Liability type for company law violations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiabilityType {
    /// 连带责任 / Joint and several liability
    JointLiability,
    /// 赔偿责任 / Damages liability
    DamagesLiability,
    /// 个人责任 / Personal liability
    PersonalLiability,
    /// 出资责任 / Capital contribution liability
    ContributionLiability,
    /// 行政责任 / Administrative liability
    Administrative,
}

impl LiabilityType {
    pub fn description(&self) -> BilingualText {
        match self {
            Self::JointLiability => BilingualText::new(
                "对公司债务承担连带责任",
                "Joint and several liability for company debts",
            ),
            Self::DamagesLiability => BilingualText::new(
                "应当依法承担赔偿责任",
                "Liable for damages according to law",
            ),
            Self::PersonalLiability => BilingualText::new("承担个人责任", "Personal liability"),
            Self::ContributionLiability => {
                BilingualText::new("应当足额缴纳出资", "Liable for full capital contribution")
            }
            Self::Administrative => {
                BilingualText::new("可能受到行政处罚", "Subject to administrative penalties")
            }
        }
    }
}

/// Result type for company law operations
pub type CompanyLawResult<T> = Result<T, CompanyLawError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_citation() {
        let error = CompanyLawError::TooManyShareholders { actual: 60 };
        let citation = error.citation().expect("Should have citation");
        assert_eq!(citation.article, 24);
    }

    #[test]
    fn test_liability_type() {
        let error = CompanyLawError::PiercingCorporateVeil;
        assert_eq!(error.liability_type(), LiabilityType::JointLiability);

        let error = CompanyLawError::DirectorSelfDealing;
        assert_eq!(error.liability_type(), LiabilityType::DamagesLiability);
    }

    #[test]
    fn test_bilingual_message() {
        let error = CompanyLawError::EquityTransferNoConsent;
        let msg = error.bilingual_message();
        assert!(msg.zh.contains("过半数"));
        assert!(msg.en.contains("majority"));
    }
}
