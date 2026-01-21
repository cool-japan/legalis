//! DPDPA Error Types
//!
//! # Digital Personal Data Protection Act, 2023 - Errors

#![allow(missing_docs)]

use crate::citation::{Citation, cite};
use thiserror::Error;

use super::types::PenaltyTier;

/// DPDPA compliance errors
#[derive(Debug, Clone, Error)]
pub enum DpdpaError {
    /// Processing without lawful grounds (Section 4)
    #[error("DPDPA Section 4: Processing personal data without lawful grounds")]
    ProcessingWithoutLawfulGrounds,

    /// Invalid consent (Section 6)
    #[error("DPDPA Section 6: Consent not free, specific, informed, unconditional, or unambiguous")]
    InvalidConsent { reason: String },

    /// Consent not obtained for specified purpose
    #[error("DPDPA Section 5: Notice not given or processing not limited to specified purpose")]
    PurposeViolation,

    /// Processing beyond specified purpose
    #[error(
        "DPDPA Section 5: Personal data processed beyond the purpose for which consent was given"
    )]
    ProcessingBeyondPurpose,

    /// Consent withdrawal not honored (Section 6)
    #[error("DPDPA Section 6(6): Failed to cease processing after consent withdrawal")]
    ConsentWithdrawalNotHonored,

    /// Child data violation (Section 9)
    #[error(
        "DPDPA Section 9: Processing child's data without parental consent or with detrimental effect"
    )]
    ChildDataViolation { reason: String },

    /// SDF obligations not met (Section 10)
    #[error(
        "DPDPA Section 10: Significant Data Fiduciary failed to comply with additional obligations"
    )]
    SdfObligationViolation { obligation: String },

    /// DPO not appointed (Section 10)
    #[error("DPDPA Section 10: Data Protection Officer not appointed as required")]
    DpoNotAppointed,

    /// DPO not based in India
    #[error("DPDPA Section 10: Data Protection Officer must be based in India")]
    DpoNotInIndia,

    /// Access right violation (Section 11)
    #[error("DPDPA Section 11: Failed to provide access to personal data summary")]
    AccessRightViolation,

    /// Correction/Erasure violation (Section 12)
    #[error("DPDPA Section 12: Failed to correct or erase personal data as requested")]
    CorrectionErasureViolation,

    /// Grievance not redressed (Section 13)
    #[error("DPDPA Section 13: Failed to provide grievance redressal mechanism")]
    GrievanceViolation,

    /// Security safeguard failure (Section 8)
    #[error("DPDPA Section 8(5): Failed to implement reasonable security safeguards")]
    SecuritySafeguardFailure,

    /// Data breach notification failure (Section 8)
    #[error("DPDPA Section 8(6): Failed to notify data breach to Board and affected principals")]
    BreachNotificationFailure,

    /// Cross-border transfer violation (Section 16)
    #[error("DPDPA Section 16: Transfer to restricted country without authorization")]
    CrossBorderViolation { country: String },

    /// Data processor violation (Section 8)
    #[error("DPDPA Section 8: Data processor processed data beyond instructions")]
    ProcessorViolation,

    /// Consent manager not registered
    #[error("DPDPA Section 6: Consent manager not registered with Board")]
    ConsentManagerNotRegistered,

    /// Retention violation (Section 8)
    #[error("DPDPA Section 8(7): Personal data retained longer than necessary")]
    RetentionViolation,

    /// Accuracy violation (Section 8)
    #[error("DPDPA Section 8(3): Failed to ensure completeness, accuracy, and consistency of data")]
    AccuracyViolation,

    /// Data principal duty breach (Section 15)
    #[error("DPDPA Section 15: Data principal breached duty")]
    DataPrincipalDutyBreach { duty: String },

    /// General validation error
    #[error("DPDPA validation error: {message}")]
    ValidationError { message: String },
}

impl DpdpaError {
    /// Get relevant citation
    pub fn citation(&self) -> Option<Citation> {
        match self {
            Self::ProcessingWithoutLawfulGrounds => Some(cite::dpdpa(4)),
            Self::InvalidConsent { .. } => Some(cite::dpdpa(6)),
            Self::PurposeViolation => Some(cite::dpdpa(5)),
            Self::ProcessingBeyondPurpose => Some(cite::dpdpa(5)),
            Self::ConsentWithdrawalNotHonored => Some(cite::dpdpa(6)),
            Self::ChildDataViolation { .. } => Some(cite::dpdpa(9)),
            Self::SdfObligationViolation { .. } => Some(cite::dpdpa(10)),
            Self::DpoNotAppointed => Some(cite::dpdpa(10)),
            Self::DpoNotInIndia => Some(cite::dpdpa(10)),
            Self::AccessRightViolation => Some(cite::dpdpa(11)),
            Self::CorrectionErasureViolation => Some(cite::dpdpa(12)),
            Self::GrievanceViolation => Some(cite::dpdpa(13)),
            Self::SecuritySafeguardFailure => Some(cite::dpdpa(8)),
            Self::BreachNotificationFailure => Some(cite::dpdpa(8)),
            Self::CrossBorderViolation { .. } => Some(cite::dpdpa(16)),
            Self::ProcessorViolation => Some(cite::dpdpa(8)),
            Self::ConsentManagerNotRegistered => Some(cite::dpdpa(6)),
            Self::RetentionViolation => Some(cite::dpdpa(8)),
            Self::AccuracyViolation => Some(cite::dpdpa(8)),
            Self::DataPrincipalDutyBreach { .. } => Some(cite::dpdpa(15)),
            Self::ValidationError { .. } => None,
        }
    }

    /// Get applicable penalty tier (Section 33)
    pub fn penalty_tier(&self) -> PenaltyTier {
        match self {
            Self::DataPrincipalDutyBreach { .. } => PenaltyTier::DataPrincipalBreach,
            Self::SecuritySafeguardFailure => PenaltyTier::Tier1,
            Self::BreachNotificationFailure | Self::ChildDataViolation { .. } => PenaltyTier::Tier2,
            Self::ProcessingWithoutLawfulGrounds
            | Self::InvalidConsent { .. }
            | Self::ConsentWithdrawalNotHonored
            | Self::SdfObligationViolation { .. }
            | Self::DpoNotAppointed
            | Self::DpoNotInIndia
            | Self::AccessRightViolation
            | Self::CorrectionErasureViolation
            | Self::CrossBorderViolation { .. } => PenaltyTier::Tier3,
            _ => PenaltyTier::Tier1,
        }
    }

    /// Get penalty description
    pub fn penalty_description(&self) -> String {
        let tier = self.penalty_tier();
        format!(
            "{} - Maximum penalty: Rs. {} crore",
            tier.description(),
            tier.max_amount_rupees() / 10_000_000
        )
    }
}

/// Result type for DPDPA operations
pub type DpdpaResult<T> = Result<T, DpdpaError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_citation() {
        let error = DpdpaError::ProcessingWithoutLawfulGrounds;
        let citation = error.citation().expect("Should have citation");
        assert_eq!(citation.number, 4);
    }

    #[test]
    fn test_penalty_tier() {
        let error = DpdpaError::ProcessingWithoutLawfulGrounds;
        assert_eq!(error.penalty_tier(), PenaltyTier::Tier3);

        let error = DpdpaError::SecuritySafeguardFailure;
        assert_eq!(error.penalty_tier(), PenaltyTier::Tier1);

        let error = DpdpaError::BreachNotificationFailure;
        assert_eq!(error.penalty_tier(), PenaltyTier::Tier2);
    }

    #[test]
    fn test_data_principal_penalty() {
        let error = DpdpaError::DataPrincipalDutyBreach {
            duty: "NoFalseParticulars".to_string(),
        };
        assert_eq!(error.penalty_tier(), PenaltyTier::DataPrincipalBreach);
        assert_eq!(error.penalty_tier().max_amount_rupees(), 10_000);
    }
}
