//! Personal Data Protection Law (PDPL) - نظام حماية البيانات الشخصية
//!
//! Royal Decree No. M/19 dated 9/2/1443H (2021)
//!
//! Saudi Arabia's data protection framework, enforced by SDAIA
//! (Saudi Data and AI Authority - الهيئة السعودية للبيانات والذكاء الاصطناعي)

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for data protection operations
pub type DataProtectionResult<T> = Result<T, DataProtectionError>;

/// Data protection errors
#[derive(Debug, Error)]
pub enum DataProtectionError {
    /// Invalid processing
    #[error("معالجة بيانات غير صالحة: {reason}")]
    InvalidProcessing { reason: String },

    /// Missing legal basis
    #[error("لا يوجد أساس قانوني: {description}")]
    MissingLegalBasis { description: String },

    /// Consent violation
    #[error("انتهاك الموافقة: {description}")]
    ConsentViolation { description: String },
}

/// Categories of personal data
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataCategory {
    /// General personal data
    General,
    /// Sensitive personal data
    Sensitive,
    /// Children's data (under 18)
    Children,
}

/// Legal basis for processing
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LegalBasis {
    /// Consent (الموافقة)
    Consent,
    /// Contract performance (تنفيذ العقد)
    ContractPerformance,
    /// Legal obligation (التزام قانوني)
    LegalObligation,
    /// Vital interests (مصلحة حيوية)
    VitalInterests,
    /// Public interest (مصلحة عامة)
    PublicInterest,
    /// Legitimate interests (مصلحة مشروعة)
    LegitimateInterests,
}

impl LegalBasis {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::Consent => "الموافقة",
            Self::ContractPerformance => "تنفيذ العقد",
            Self::LegalObligation => "التزام قانوني",
            Self::VitalInterests => "مصلحة حيوية",
            Self::PublicInterest => "مصلحة عامة",
            Self::LegitimateInterests => "مصلحة مشروعة",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Consent => "Consent",
            Self::ContractPerformance => "Contract Performance",
            Self::LegalObligation => "Legal Obligation",
            Self::VitalInterests => "Vital Interests",
            Self::PublicInterest => "Public Interest",
            Self::LegitimateInterests => "Legitimate Interests",
        }
    }
}

/// Data subject rights under PDPL
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataSubjectRight {
    /// Right to access (الحق في الوصول)
    Access,
    /// Right to rectification (الحق في التصحيح)
    Rectification,
    /// Right to erasure (الحق في المحو)
    Erasure,
    /// Right to restriction (الحق في تقييد المعالجة)
    Restriction,
    /// Right to data portability (الحق في نقل البيانات)
    DataPortability,
    /// Right to object (الحق في الاعتراض)
    Object,
    /// Right to withdraw consent (الحق في سحب الموافقة)
    WithdrawConsent,
}

impl DataSubjectRight {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::Access => "الحق في الوصول",
            Self::Rectification => "الحق في التصحيح",
            Self::Erasure => "الحق في المحو",
            Self::Restriction => "الحق في تقييد المعالجة",
            Self::DataPortability => "الحق في نقل البيانات",
            Self::Object => "الحق في الاعتراض",
            Self::WithdrawConsent => "الحق في سحب الموافقة",
        }
    }
}

/// Processing purposes
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProcessingPurpose {
    /// Marketing
    Marketing,
    /// Analytics
    Analytics,
    /// Service delivery
    ServiceDelivery,
    /// Legal compliance
    LegalCompliance,
    /// Research
    Research,
}

/// Validate data processing
pub fn validate_processing(
    category: &DataCategory,
    legal_basis: &LegalBasis,
    is_cross_border: bool,
) -> DataProtectionResult<()> {
    // Sensitive data requires explicit consent
    if *category == DataCategory::Sensitive
        && !matches!(
            legal_basis,
            LegalBasis::Consent | LegalBasis::LegalObligation
        )
    {
        return Err(DataProtectionError::MissingLegalBasis {
            description: "Sensitive data requires explicit consent or legal obligation".to_string(),
        });
    }

    // Children's data requires parental consent
    if *category == DataCategory::Children && *legal_basis != LegalBasis::Consent {
        return Err(DataProtectionError::ConsentViolation {
            description: "Children's data requires parental consent".to_string(),
        });
    }

    // Cross-border transfers require additional safeguards
    if is_cross_border {
        // Would need to check adequate protection mechanisms
        // This is simplified
    }

    Ok(())
}

/// Get PDPL compliance checklist
pub fn get_pdpl_checklist() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "تسجيل مراقب البيانات",
            "Register as data controller (if required)",
        ),
        ("سياسة الخصوصية", "Privacy policy in Arabic"),
        ("الحصول على الموافقة", "Obtain valid consent"),
        ("تقييم الأثر", "Data protection impact assessment (DPIA)"),
        ("أمن البيانات", "Implement data security measures"),
        (
            "حقوق أصحاب البيانات",
            "Implement data subject rights procedures",
        ),
        ("الاحتفاظ بالبيانات", "Data retention policy"),
        ("الإبلاغ عن الانتهاكات", "Breach notification procedures"),
        ("النقل عبر الحدود", "Cross-border transfer mechanisms"),
        ("التدريب والوعي", "Staff training and awareness"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legal_basis_names() {
        assert_eq!(LegalBasis::Consent.name_ar(), "الموافقة");
        assert_eq!(
            LegalBasis::ContractPerformance.name_en(),
            "Contract Performance"
        );
    }

    #[test]
    fn test_data_subject_rights() {
        assert_eq!(DataSubjectRight::Access.name_ar(), "الحق في الوصول");
        assert_eq!(DataSubjectRight::Erasure.name_ar(), "الحق في المحو");
    }

    #[test]
    fn test_valid_general_processing() {
        let result = validate_processing(&DataCategory::General, &LegalBasis::Consent, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sensitive_data_requires_consent() {
        let result = validate_processing(
            &DataCategory::Sensitive,
            &LegalBasis::LegitimateInterests,
            false,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_children_data_requires_consent() {
        let result = validate_processing(
            &DataCategory::Children,
            &LegalBasis::LegitimateInterests,
            false,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_checklist() {
        let checklist = get_pdpl_checklist();
        assert!(!checklist.is_empty());
        assert!(checklist.len() >= 10);
    }
}
