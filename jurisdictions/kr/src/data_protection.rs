//! Personal Information Protection Act (개인정보 보호법)
//!
//! # 개인정보 보호법 / Personal Information Protection Act (PIPA)
//!
//! Enacted: 2011
//! Effective: 2011-09-30
//! Last amendment: 2023
//!
//! Korea's main data protection law

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Data protection errors
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum DataProtectionError {
    /// Invalid consent
    #[error("Invalid consent: {0}")]
    InvalidConsent(String),

    /// Processing violation
    #[error("Processing violation: {0}")]
    ProcessingViolation(String),

    /// Security violation
    #[error("Security violation: {0}")]
    SecurityViolation(String),
}

/// Result type for data protection operations (PIPA compliance)
pub type DataProtectionResult<T> = Result<T, DataProtectionError>;

/// Personal information category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PersonalInfoCategory {
    /// General personal information (일반 개인정보)
    General,
    /// Sensitive personal information (민감정보)
    Sensitive,
    /// Unique identification information (고유식별정보)
    UniqueIdentification,
}

/// Legal basis for processing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProcessingBasis {
    /// Consent (동의)
    Consent,
    /// Contract performance (계약 이행)
    ContractPerformance,
    /// Legal obligation (법적 의무)
    LegalObligation,
    /// Vital interests (중대한 이익)
    VitalInterests,
    /// Public interest (공익)
    PublicInterest,
    /// Legitimate interest (정당한 이익)
    LegitimateInterest,
}

/// Consent record
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsentRecord {
    /// Data subject
    pub data_subject: String,
    /// Consent date
    pub consent_date: NaiveDate,
    /// Purpose
    pub purpose: String,
    /// Categories consented
    pub categories: Vec<PersonalInfoCategory>,
    /// Retention period
    pub retention_period: String,
    /// Is withdrawal available
    pub withdrawal_available: bool,
}

impl ConsentRecord {
    /// Create new consent record
    pub fn new(
        data_subject: impl Into<String>,
        consent_date: NaiveDate,
        purpose: impl Into<String>,
        retention_period: impl Into<String>,
    ) -> Self {
        Self {
            data_subject: data_subject.into(),
            consent_date,
            purpose: purpose.into(),
            categories: Vec::new(),
            retention_period: retention_period.into(),
            withdrawal_available: true,
        }
    }

    /// Add category
    pub fn add_category(mut self, category: PersonalInfoCategory) -> Self {
        self.categories.push(category);
        self
    }
}

/// Validate consent for sensitive information
/// Article 23: Separate consent required for sensitive information
pub fn validate_sensitive_consent(consent: &ConsentRecord) -> DataProtectionResult<()> {
    if consent
        .categories
        .contains(&PersonalInfoCategory::Sensitive)
        && consent.purpose.is_empty()
    {
        return Err(DataProtectionError::InvalidConsent(
            "Purpose must be specified for sensitive information".to_string(),
        ));
    }

    Ok(())
}

/// Data breach notification requirement
/// Article 34: Must notify within 24 hours of discovery
pub const BREACH_NOTIFICATION_HOURS: u32 = 24;

/// Validate data breach notification timing
pub fn validate_breach_notification_timing(
    discovery_time: NaiveDate,
    notification_time: NaiveDate,
) -> DataProtectionResult<()> {
    let hours_elapsed = (notification_time - discovery_time).num_hours();

    if hours_elapsed > BREACH_NOTIFICATION_HOURS as i64 {
        return Err(DataProtectionError::SecurityViolation(format!(
            "Breach notification exceeded {} hours",
            BREACH_NOTIFICATION_HOURS
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consent_record() {
        if let Some(date) = NaiveDate::from_ymd_opt(2024, 1, 1) {
            let consent = ConsentRecord::new("김철수", date, "서비스 제공", "3년")
                .add_category(PersonalInfoCategory::General);

            assert_eq!(consent.data_subject, "김철수");
            assert_eq!(consent.categories.len(), 1);
        }
    }

    #[test]
    fn test_validate_sensitive_consent() {
        if let Some(date) = NaiveDate::from_ymd_opt(2024, 1, 1) {
            let consent = ConsentRecord::new("김철수", date, "의료 서비스", "5년")
                .add_category(PersonalInfoCategory::Sensitive);

            let result = validate_sensitive_consent(&consent);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_validate_breach_notification() {
        if let (Some(discovery), Some(notification)) = (
            NaiveDate::from_ymd_opt(2024, 1, 1),
            NaiveDate::from_ymd_opt(2024, 1, 1),
        ) {
            let result = validate_breach_notification_timing(discovery, notification);
            assert!(result.is_ok());
        }
    }
}
