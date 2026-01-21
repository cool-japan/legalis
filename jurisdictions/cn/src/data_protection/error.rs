//! PIPL Error Types
//!
//! # 个人信息保护法错误类型

#![allow(missing_docs)]

use crate::citation::{Citation, cite};
use crate::i18n::BilingualText;
use thiserror::Error;

/// PIPL compliance errors
///
/// # 个人信息保护法合规错误
#[derive(Debug, Clone, Error)]
pub enum PiplError {
    /// Missing consent for processing
    #[error(
        "PIPL Article 13: No valid legal basis for processing. {description_zh} / {description_en}"
    )]
    NoLegalBasis {
        description_zh: String,
        description_en: String,
    },

    /// Invalid consent
    #[error("PIPL Article 14: Consent is invalid. {reason}")]
    InvalidConsent { reason: String },

    /// Missing separate consent for sensitive PI
    #[error(
        "PIPL Article 29: Separate consent required for sensitive personal information ({pi_type})"
    )]
    MissingSeparateConsent { pi_type: String },

    /// Missing written consent
    #[error("PIPL Article {article}: Written consent required. {reason}")]
    MissingWrittenConsent { article: u32, reason: String },

    /// Cross-border transfer violation
    #[error("PIPL Article 38-40: Cross-border transfer violation. {violation}")]
    CrossBorderViolation { violation: String },

    /// Security assessment required
    #[error(
        "PIPL Article 40: Security assessment required before cross-border transfer. Processing {individuals} individuals."
    )]
    SecurityAssessmentRequired { individuals: u64 },

    /// Missing security measures
    #[error("PIPL Article 51: Inadequate security measures. Missing: {missing_measures:?}")]
    InadequateSecurityMeasures { missing_measures: Vec<String> },

    /// Data breach notification failure
    #[error("PIPL Article 57: Data breach notification not completed within required timeframe")]
    BreachNotificationFailure,

    /// Individual rights violation
    #[error("PIPL Article {article}: Individual right ({right_zh}) violation. {details}")]
    IndividualRightsViolation {
        article: u32,
        right_zh: String,
        details: String,
    },

    /// Missing Data Protection Officer
    #[error("PIPL Article 52: Data Protection Officer required but not appointed")]
    MissingDpo,

    /// Inadequate processing record
    #[error("PIPL Article 54: Processing activity record incomplete. Missing: {missing_fields:?}")]
    IncompleteProcessingRecord { missing_fields: Vec<String> },

    /// Retention period exceeded
    #[error("PIPL Article 19: Retention period ({retention_days} days) exceeds minimum necessary")]
    ExcessiveRetention { retention_days: u32 },

    /// Minor information violation
    #[error("PIPL Article 31: Processing minor's (under 14) information without guardian consent")]
    MinorConsentViolation,

    /// Automated decision-making violation
    #[error("PIPL Article 24: Automated decision-making violation. {violation}")]
    AutomatedDecisionViolation { violation: String },

    /// Privacy policy violation
    #[error("PIPL Article 17: Privacy policy inadequate. Missing elements: {missing:?}")]
    InadequatePrivacyPolicy { missing: Vec<String> },

    /// Overseas recipient violation
    #[error("PIPL Article 39: Overseas recipient failed to meet protection standards")]
    OverseasRecipientViolation { recipient: String },

    /// CAC filing required
    #[error("CAC Measures: Filing with Cyberspace Administration required for {reason}")]
    CacFilingRequired { reason: String },

    /// General validation error
    #[error("PIPL validation error: {message}")]
    ValidationError { message: String },
}

impl PiplError {
    /// Get the relevant PIPL citation
    pub fn citation(&self) -> Option<Citation> {
        match self {
            Self::NoLegalBasis { .. } => Some(cite::pipl(13)),
            Self::InvalidConsent { .. } => Some(cite::pipl(14)),
            Self::MissingSeparateConsent { .. } => Some(cite::pipl(29)),
            Self::MissingWrittenConsent { article, .. } => Some(cite::pipl(*article)),
            Self::CrossBorderViolation { .. } => Some(cite::pipl(38)),
            Self::SecurityAssessmentRequired { .. } => Some(cite::pipl(40)),
            Self::InadequateSecurityMeasures { .. } => Some(cite::pipl(51)),
            Self::BreachNotificationFailure => Some(cite::pipl(57)),
            Self::IndividualRightsViolation { article, .. } => Some(cite::pipl(*article)),
            Self::MissingDpo => Some(cite::pipl(52)),
            Self::IncompleteProcessingRecord { .. } => Some(cite::pipl(54)),
            Self::ExcessiveRetention { .. } => Some(cite::pipl(19)),
            Self::MinorConsentViolation => Some(cite::pipl(31)),
            Self::AutomatedDecisionViolation { .. } => Some(cite::pipl(24)),
            Self::InadequatePrivacyPolicy { .. } => Some(cite::pipl(17)),
            Self::OverseasRecipientViolation { .. } => Some(cite::pipl(39)),
            Self::CacFilingRequired { .. } => None,
            Self::ValidationError { .. } => None,
        }
    }

    /// Get bilingual error message
    pub fn bilingual_message(&self) -> BilingualText {
        match self {
            Self::NoLegalBasis {
                description_zh,
                description_en,
            } => BilingualText::new(
                format!("无有效的法律依据：{}", description_zh),
                format!("No valid legal basis: {}", description_en),
            ),
            Self::MissingSeparateConsent { pi_type } => BilingualText::new(
                format!("处理敏感个人信息需要单独同意：{}", pi_type),
                format!("Separate consent required for sensitive PI: {}", pi_type),
            ),
            Self::SecurityAssessmentRequired { individuals } => BilingualText::new(
                format!("数据出境需要安全评估（处理{}人个人信息）", individuals),
                format!(
                    "Security assessment required for cross-border transfer ({} individuals)",
                    individuals
                ),
            ),
            _ => BilingualText::new("个人信息保护法合规错误".to_string(), self.to_string()),
        }
    }

    /// Get potential penalty range (Article 66)
    pub fn penalty_range(&self) -> PenaltyRange {
        match self {
            Self::NoLegalBasis { .. }
            | Self::CrossBorderViolation { .. }
            | Self::SecurityAssessmentRequired { .. } => PenaltyRange::Serious,
            Self::InadequateSecurityMeasures { .. }
            | Self::BreachNotificationFailure
            | Self::MissingDpo => PenaltyRange::Moderate,
            _ => PenaltyRange::Minor,
        }
    }
}

/// Penalty range under PIPL Article 66
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PenaltyRange {
    /// 轻微 / Minor: Warning, order to rectify
    Minor,
    /// 一般 / Moderate: Fine up to 1M RMB
    Moderate,
    /// 严重 / Serious: Fine up to 50M RMB or 5% of previous year's revenue
    Serious,
}

impl PenaltyRange {
    /// Get maximum fine in RMB
    pub fn max_fine_rmb(&self) -> Option<f64> {
        match self {
            Self::Minor => None, // Warning only
            Self::Moderate => Some(1_000_000.0),
            Self::Serious => Some(50_000_000.0), // Or 5% of revenue
        }
    }

    /// Get description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::Minor => BilingualText::new(
                "轻微违规：责令改正、警告",
                "Minor: Warning, order to rectify",
            ),
            Self::Moderate => BilingualText::new(
                "一般违规：罚款不超过100万元",
                "Moderate: Fine up to 1 million RMB",
            ),
            Self::Serious => BilingualText::new(
                "严重违规：罚款不超过5000万元或上一年度营业额5%",
                "Serious: Fine up to 50 million RMB or 5% of previous year's revenue",
            ),
        }
    }
}

/// Result type for PIPL operations
pub type PiplResult<T> = Result<T, PiplError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_citation() {
        let error = PiplError::MissingSeparateConsent {
            pi_type: "生物识别信息".to_string(),
        };
        let citation = error.citation().unwrap();
        assert_eq!(citation.article, 29);
    }

    #[test]
    fn test_penalty_range() {
        let serious = PiplError::CrossBorderViolation {
            violation: "未经安全评估".to_string(),
        };
        assert_eq!(serious.penalty_range(), PenaltyRange::Serious);
        assert_eq!(serious.penalty_range().max_fine_rmb(), Some(50_000_000.0));
    }

    #[test]
    fn test_bilingual_message() {
        let error = PiplError::SecurityAssessmentRequired {
            individuals: 1_500_000,
        };
        let msg = error.bilingual_message();
        assert!(msg.zh.contains("安全评估"));
        assert!(msg.en.contains("Security assessment"));
    }
}
