//! UAE Personal Data Protection Law - Federal Decree-Law No. 45/2021
//!
//! The UAE PDPL came into effect on 2 January 2022, with enforcement
//! beginning in phases. It establishes comprehensive data protection rules.
//!
//! ## Key Features
//!
//! - Consent-based processing model
//! - 9 data subject rights
//! - Cross-border transfer restrictions
//! - Data Protection Officer requirements for certain controllers
//! - Exemptions for free zones (DIFC and ADGM have separate laws)
//!
//! ## Exemptions
//!
//! - Personal/household activities
//! - Government data for national security
//! - Statistical/research data (anonymized)
//! - DIFC and ADGM (separate frameworks)

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for data protection operations
pub type DataProtectionResult<T> = Result<T, DataProtectionError>;

/// Personal data categories - Article 3
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataCategory {
    /// General personal data
    General,
    /// Sensitive personal data - Article 5
    Sensitive(SensitiveDataType),
}

/// Types of sensitive personal data - Article 5
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SensitiveDataType {
    /// Health data
    Health,
    /// Biometric data
    Biometric,
    /// Genetic data
    Genetic,
    /// Race or ethnic origin
    RacialOrEthnic,
    /// Religious beliefs
    Religious,
    /// Political opinions
    Political,
    /// Trade union membership
    TradeUnion,
    /// Sexual life/orientation
    SexualOrientation,
    /// Criminal convictions
    CriminalRecords,
    /// Children's data
    ChildData,
}

/// Legal bases for processing - Article 4
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LegalBasis {
    /// Data subject consent
    Consent,
    /// Performance of contract
    ContractPerformance,
    /// Legal obligation
    LegalObligation,
    /// Vital interests
    VitalInterests,
    /// Public interest / official authority
    PublicInterest,
    /// Legitimate interests (with balancing test)
    LegitimateInterests,
    /// Employment relationship
    Employment,
    /// Legal claims
    LegalClaims,
}

impl LegalBasis {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::Consent => "الموافقة",
            Self::ContractPerformance => "تنفيذ العقد",
            Self::LegalObligation => "الالتزام القانوني",
            Self::VitalInterests => "المصالح الحيوية",
            Self::PublicInterest => "المصلحة العامة",
            Self::LegitimateInterests => "المصالح المشروعة",
            Self::Employment => "علاقة العمل",
            Self::LegalClaims => "المطالبات القانونية",
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
            Self::Employment => "Employment Relationship",
            Self::LegalClaims => "Legal Claims",
        }
    }
}

/// Data subject rights - Article 13-18
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataSubjectRight {
    /// Right to be informed - Article 13
    Information,
    /// Right of access - Article 14
    Access,
    /// Right to rectification - Article 15
    Rectification,
    /// Right to erasure - Article 16
    Erasure,
    /// Right to restriction - Article 17
    Restriction,
    /// Right to data portability - Article 18
    Portability,
    /// Right to object - Article 19
    Objection,
    /// Right not to be subject to automated decisions - Article 20
    AutomatedDecision,
    /// Right to withdraw consent
    WithdrawConsent,
}

impl DataSubjectRight {
    /// Get the statutory reference (Article number)
    pub fn statutory_reference(&self) -> &'static str {
        match self {
            Self::Information => "Article 13",
            Self::Access => "Article 14",
            Self::Rectification => "Article 15",
            Self::Erasure => "Article 16",
            Self::Restriction => "Article 17",
            Self::Portability => "Article 18",
            Self::Objection => "Article 19",
            Self::AutomatedDecision => "Article 20",
            Self::WithdrawConsent => "Article 6",
        }
    }

    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::Information => "الحق في الإعلام",
            Self::Access => "حق الوصول",
            Self::Rectification => "حق التصحيح",
            Self::Erasure => "حق المحو",
            Self::Restriction => "حق تقييد المعالجة",
            Self::Portability => "حق نقل البيانات",
            Self::Objection => "حق الاعتراض",
            Self::AutomatedDecision => "حق عدم الخضوع لقرارات آلية",
            Self::WithdrawConsent => "حق سحب الموافقة",
        }
    }
}

/// Cross-border transfer conditions - Article 21-22
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransferMechanism {
    /// Adequacy decision by UAE Data Office
    AdequacyDecision,
    /// Standard contractual clauses
    StandardContractualClauses,
    /// Binding corporate rules
    BindingCorporateRules,
    /// Certification mechanism
    Certification,
    /// Explicit consent
    ExplicitConsent,
    /// Contract performance
    ContractNecessity,
    /// Legal claims
    LegalClaims,
    /// Vital interests
    VitalInterests,
}

/// Data protection assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataProtectionAssessment {
    /// Processing activity name
    pub activity_name: String,
    /// Data categories processed
    pub data_categories: Vec<DataCategory>,
    /// Legal basis for processing
    pub legal_basis: LegalBasis,
    /// Cross-border transfers
    pub cross_border_transfers: Vec<String>,
    /// Transfer mechanism (if applicable)
    pub transfer_mechanism: Option<TransferMechanism>,
    /// Risk level
    pub risk_level: RiskLevel,
    /// Requires DPO
    pub requires_dpo: bool,
    /// Requires DPIA
    pub requires_dpia: bool,
}

/// Risk levels for processing
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RiskLevel {
    /// Low risk processing
    Low,
    /// Medium risk processing
    Medium,
    /// High risk processing (requires DPIA)
    High,
}

impl DataProtectionAssessment {
    /// Determine if processing requires a DPIA
    pub fn requires_dpia(&self) -> bool {
        // Required for sensitive data or high risk
        self.risk_level == RiskLevel::High
            || self
                .data_categories
                .iter()
                .any(|c| matches!(c, DataCategory::Sensitive(_)))
    }

    /// Determine if DPO is required
    pub fn requires_dpo(&self) -> bool {
        // Required for large-scale sensitive data processing
        self.risk_level == RiskLevel::High
    }
}

/// Data protection errors
#[derive(Debug, Error)]
pub enum DataProtectionError {
    /// No valid legal basis - Article 4
    #[error("لا يوجد أساس قانوني صالح لمعالجة البيانات (المادة 4)")]
    NoLegalBasis,

    /// Invalid consent - Article 6
    #[error("الموافقة غير صالحة (المادة 6): {reason}")]
    InvalidConsent { reason: String },

    /// Sensitive data processing without explicit consent - Article 5
    #[error("معالجة بيانات حساسة بدون موافقة صريحة (المادة 5)")]
    SensitiveDataWithoutConsent,

    /// Unlawful cross-border transfer - Article 21
    #[error("نقل البيانات عبر الحدود غير قانوني (المادة 21): {destination}")]
    UnlawfulTransfer { destination: String },

    /// Data subject rights violation
    #[error("انتهاك حقوق صاحب البيانات ({right}): {description}")]
    RightsViolation { right: String, description: String },

    /// Security breach - Article 11
    #[error("خرق أمني (المادة 11): {description}")]
    SecurityBreach { description: String },

    /// DPIA required but not conducted - Article 10
    #[error("تقييم الأثر على حماية البيانات مطلوب (المادة 10)")]
    DpiaRequired,
}

/// Validate data processing activity
pub fn validate_processing(
    data_category: &DataCategory,
    legal_basis: &LegalBasis,
    has_consent: bool,
) -> DataProtectionResult<()> {
    // Sensitive data requires explicit consent or specific legal basis
    if let DataCategory::Sensitive(_) = data_category {
        if !matches!(
            legal_basis,
            LegalBasis::Consent
                | LegalBasis::LegalObligation
                | LegalBasis::VitalInterests
                | LegalBasis::LegalClaims
        ) {
            return Err(DataProtectionError::SensitiveDataWithoutConsent);
        }

        if *legal_basis == LegalBasis::Consent && !has_consent {
            return Err(DataProtectionError::InvalidConsent {
                reason: "Sensitive data requires explicit consent".to_string(),
            });
        }
    }

    Ok(())
}

/// Validate cross-border transfer
pub fn validate_transfer(
    destination: &str,
    mechanism: Option<&TransferMechanism>,
) -> DataProtectionResult<()> {
    // Transfers require a valid mechanism
    if mechanism.is_none() {
        return Err(DataProtectionError::UnlawfulTransfer {
            destination: destination.to_string(),
        });
    }

    Ok(())
}

/// Get data protection law compliance checklist
pub fn get_pdpl_checklist() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "تحديد الأساس القانوني للمعالجة",
            "Identify legal basis for processing",
            "Article 4",
        ),
        (
            "الحصول على موافقة صالحة (إن لزم)",
            "Obtain valid consent (if required)",
            "Article 6",
        ),
        ("إعلام أصحاب البيانات", "Inform data subjects", "Article 13"),
        (
            "تنفيذ حقوق أصحاب البيانات",
            "Implement data subject rights",
            "Articles 14-20",
        ),
        (
            "إجراء تقييم الأثر (DPIA)",
            "Conduct DPIA (if required)",
            "Article 10",
        ),
        (
            "تعيين مسؤول حماية البيانات (DPO)",
            "Appoint DPO (if required)",
            "Article 12",
        ),
        (
            "تأمين البيانات الشخصية",
            "Secure personal data",
            "Article 11",
        ),
        (
            "التحقق من نقل البيانات عبر الحدود",
            "Verify cross-border transfers",
            "Articles 21-22",
        ),
        ("الاحتفاظ بالسجلات", "Maintain records", "Article 9"),
        ("الإبلاغ عن الخروقات", "Report breaches", "Article 24"),
    ]
}

/// Free zone data protection frameworks
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FreeZoneFramework {
    /// DIFC Data Protection Law (DIFC Law No. 5 of 2020)
    Difc,
    /// ADGM Data Protection Regulations 2021
    Adgm,
    /// Federal UAE PDPL
    Federal,
}

impl FreeZoneFramework {
    /// Get the applicable data protection law name
    pub fn law_name(&self) -> &'static str {
        match self {
            Self::Difc => "DIFC Data Protection Law 2020",
            Self::Adgm => "ADGM Data Protection Regulations 2021",
            Self::Federal => "Federal Decree-Law No. 45/2021",
        }
    }

    /// Check if framework is GDPR-aligned
    pub fn is_gdpr_aligned(&self) -> bool {
        // All UAE frameworks are broadly GDPR-aligned
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legal_bases() {
        let consent = LegalBasis::Consent;
        assert_eq!(consent.name_en(), "Consent");
        assert!(!consent.name_ar().is_empty());
    }

    #[test]
    fn test_data_subject_rights() {
        let access = DataSubjectRight::Access;
        assert_eq!(access.statutory_reference(), "Article 14");
    }

    #[test]
    fn test_validate_general_data() {
        let result = validate_processing(
            &DataCategory::General,
            &LegalBasis::ContractPerformance,
            false,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_sensitive_without_consent() {
        let result = validate_processing(
            &DataCategory::Sensitive(SensitiveDataType::Health),
            &LegalBasis::LegitimateInterests,
            false,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_sensitive_with_consent() {
        let result = validate_processing(
            &DataCategory::Sensitive(SensitiveDataType::Health),
            &LegalBasis::Consent,
            true,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_transfer_validation() {
        let result = validate_transfer("USA", None);
        assert!(result.is_err());

        let result = validate_transfer("USA", Some(&TransferMechanism::StandardContractualClauses));
        assert!(result.is_ok());
    }

    #[test]
    fn test_dpia_requirement() {
        let assessment = DataProtectionAssessment {
            activity_name: "Health data processing".to_string(),
            data_categories: vec![DataCategory::Sensitive(SensitiveDataType::Health)],
            legal_basis: LegalBasis::Consent,
            cross_border_transfers: vec![],
            transfer_mechanism: None,
            risk_level: RiskLevel::High,
            requires_dpo: false,
            requires_dpia: false,
        };

        assert!(assessment.requires_dpia());
    }

    #[test]
    fn test_free_zone_frameworks() {
        let difc = FreeZoneFramework::Difc;
        assert!(difc.is_gdpr_aligned());
        assert!(difc.law_name().contains("DIFC"));
    }

    #[test]
    fn test_pdpl_checklist() {
        let checklist = get_pdpl_checklist();
        assert!(!checklist.is_empty());
    }
}
