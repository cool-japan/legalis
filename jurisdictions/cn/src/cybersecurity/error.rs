//! Cybersecurity Law Error Types
//!
//! # 网络安全法错误类型

#![allow(missing_docs)]

use crate::citation::{Citation, cite};
use crate::i18n::BilingualText;
use thiserror::Error;

use super::types::MlpsLevel;

/// Cybersecurity Law compliance errors
#[derive(Debug, Clone, Error)]
pub enum CybersecurityError {
    /// MLPS level not determined
    #[error("Cybersecurity Law Article 21: MLPS level not determined for system '{system_name}'")]
    MlpsLevelNotDetermined { system_name: String },

    /// MLPS filing missing
    #[error("Cybersecurity Law Article 21: MLPS filing required for system at level {level:?}")]
    MlpsFilingRequired { level: MlpsLevel },

    /// MLPS assessment expired
    #[error("Cybersecurity Law: MLPS assessment expired. Last assessment: {last_assessment}")]
    MlpsAssessmentExpired { last_assessment: String },

    /// Third-party assessment required
    #[error("Cybersecurity Law: Third-party assessment required for MLPS Level {level}")]
    ThirdPartyAssessmentRequired { level: u8 },

    /// Security controls below threshold
    #[error(
        "Cybersecurity Law: Security controls score ({score:.1}%) below threshold ({threshold:.1}%) for Level {level}"
    )]
    ControlsBelowThreshold {
        score: f64,
        threshold: f64,
        level: u8,
    },

    /// CII designation not completed
    #[error("Cybersecurity Law Article 31: CII designation and notification not completed")]
    CiiDesignationIncomplete,

    /// CII data localization violation
    #[error(
        "Cybersecurity Law Article 37: Personal information and important data must be stored within China for CII operators"
    )]
    CiiDataLocalizationViolation,

    /// Cybersecurity review required
    #[error("Cybersecurity Law Article 35: Cybersecurity review required for {trigger}")]
    CybersecurityReviewRequired { trigger: String },

    /// Incident not reported
    #[error(
        "Cybersecurity Law Article 25: Security incident (severity: {severity}) not reported within {deadline_hours} hours"
    )]
    IncidentNotReported {
        severity: String,
        deadline_hours: u32,
    },

    /// Real-name registration missing
    #[error(
        "Cybersecurity Law Article 24: Real-name registration required for network service users"
    )]
    RealNameRegistrationMissing,

    /// Security contact not designated
    #[error("Cybersecurity Law Article 21: Security management personnel not designated")]
    SecurityContactMissing,

    /// Emergency response plan missing
    #[error("Cybersecurity Law Article 21: Emergency response plan not established")]
    EmergencyPlanMissing,

    /// Log retention insufficient
    #[error("Cybersecurity Law Article 21: Network logs must be retained for at least 6 months")]
    LogRetentionInsufficient { actual_days: u32 },

    /// Vulnerability not remediated
    #[error(
        "Cybersecurity Law Article 22: Known vulnerability not remediated within required timeframe"
    )]
    VulnerabilityNotRemediated { vulnerability_id: String },

    /// General validation error
    #[error("Cybersecurity Law validation error: {message}")]
    ValidationError { message: String },
}

impl CybersecurityError {
    /// Get relevant citation
    pub fn citation(&self) -> Option<Citation> {
        match self {
            Self::MlpsLevelNotDetermined { .. } => Some(cite::cybersecurity(21)),
            Self::MlpsFilingRequired { .. } => Some(cite::cybersecurity(21)),
            Self::MlpsAssessmentExpired { .. } => Some(cite::cybersecurity(21)),
            Self::ThirdPartyAssessmentRequired { .. } => Some(cite::cybersecurity(21)),
            Self::ControlsBelowThreshold { .. } => Some(cite::cybersecurity(21)),
            Self::CiiDesignationIncomplete => Some(cite::cybersecurity(31)),
            Self::CiiDataLocalizationViolation => Some(cite::cybersecurity(37)),
            Self::CybersecurityReviewRequired { .. } => Some(cite::cybersecurity(35)),
            Self::IncidentNotReported { .. } => Some(cite::cybersecurity(25)),
            Self::RealNameRegistrationMissing => Some(cite::cybersecurity(24)),
            Self::SecurityContactMissing => Some(cite::cybersecurity(21)),
            Self::EmergencyPlanMissing => Some(cite::cybersecurity(21)),
            Self::LogRetentionInsufficient { .. } => Some(cite::cybersecurity(21)),
            Self::VulnerabilityNotRemediated { .. } => Some(cite::cybersecurity(22)),
            Self::ValidationError { .. } => None,
        }
    }

    /// Get bilingual error message
    pub fn bilingual_message(&self) -> BilingualText {
        match self {
            Self::MlpsLevelNotDetermined { system_name } => BilingualText::new(
                format!("系统'{}'未确定等级保护级别", system_name),
                format!("MLPS level not determined for system '{}'", system_name),
            ),
            Self::CiiDataLocalizationViolation => BilingualText::new(
                "关键信息基础设施运营者应当在境内存储个人信息和重要数据",
                "CII operators must store personal information and important data within China",
            ),
            Self::IncidentNotReported {
                severity,
                deadline_hours,
            } => BilingualText::new(
                format!("{}级网络安全事件未在{}小时内报告", severity, deadline_hours),
                format!(
                    "{} severity incident not reported within {} hours",
                    severity, deadline_hours
                ),
            ),
            _ => BilingualText::new("网络安全法合规错误".to_string(), self.to_string()),
        }
    }

    /// Get penalty range
    pub fn penalty_range(&self) -> CybersecurityPenalty {
        match self {
            Self::CiiDataLocalizationViolation | Self::CybersecurityReviewRequired { .. } => {
                CybersecurityPenalty::Severe
            }
            Self::MlpsFilingRequired { .. }
            | Self::IncidentNotReported { .. }
            | Self::SecurityContactMissing => CybersecurityPenalty::Moderate,
            _ => CybersecurityPenalty::Minor,
        }
    }
}

/// Penalty severity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CybersecurityPenalty {
    /// 轻微 / Minor: Warning, order to correct
    Minor,
    /// 一般 / Moderate: Fine 10k-100k RMB
    Moderate,
    /// 严重 / Severe: Fine 100k-1M RMB, may suspend business
    Severe,
}

impl CybersecurityPenalty {
    pub fn description(&self) -> BilingualText {
        match self {
            Self::Minor => BilingualText::new(
                "轻微：责令改正、给予警告",
                "Minor: Order to correct, warning",
            ),
            Self::Moderate => BilingualText::new(
                "一般：罚款1万元至10万元",
                "Moderate: Fine 10,000-100,000 RMB",
            ),
            Self::Severe => BilingualText::new(
                "严重：罚款10万元至100万元，可责令停业整顿",
                "Severe: Fine 100,000-1,000,000 RMB, may suspend business",
            ),
        }
    }
}

/// Result type for cybersecurity operations
pub type CybersecurityResult<T> = Result<T, CybersecurityError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_citation() {
        let error = CybersecurityError::CiiDataLocalizationViolation;
        let citation = error.citation().unwrap();
        assert_eq!(citation.article, 37);
    }

    #[test]
    fn test_penalty_range() {
        let error = CybersecurityError::CiiDataLocalizationViolation;
        assert_eq!(error.penalty_range(), CybersecurityPenalty::Severe);
    }
}
