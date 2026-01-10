//! Personal Information Protection Act Types
//!
//! Core data structures for Act on the Protection of Personal Information
//! (個人情報の保護に関する法律).

use chrono::NaiveDate;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Personal information type (個人情報の種類)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PersonalInfoType {
    /// Basic personal information (個人情報 - Article 2-1)
    Basic,
    /// Sensitive personal information (要配慮個人情報 - Article 2-3)
    Sensitive,
    /// Anonymous processed information (匿名加工情報 - Article 2-5)
    Anonymous,
    /// Pseudonymous processed information (仮名加工情報 - Article 2-9, 2020 amendment)
    Pseudonymous,
}

impl PersonalInfoType {
    /// Get Japanese name
    pub fn name_ja(&self) -> &'static str {
        match self {
            Self::Basic => "個人情報",
            Self::Sensitive => "要配慮個人情報",
            Self::Anonymous => "匿名加工情報",
            Self::Pseudonymous => "仮名加工情報",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Basic => "Personal Information",
            Self::Sensitive => "Sensitive Personal Information",
            Self::Anonymous => "Anonymous Processed Information",
            Self::Pseudonymous => "Pseudonymous Processed Information",
        }
    }

    /// Check if consent required for acquisition
    pub fn requires_consent(&self) -> bool {
        matches!(self, Self::Sensitive)
    }
}

/// Business type for data handling volume classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BusinessType {
    /// Small business (under 5,000 records - some exemptions)
    SmallBusiness,
    /// Standard business (5,000-100,000 records)
    StandardBusiness,
    /// Large-scale business (over 100,000 records - additional obligations)
    LargeScaleBusiness,
    /// AI/data analytics business (special attention from regulators)
    AiDataBusiness,
}

impl BusinessType {
    /// Get Japanese name
    pub fn name_ja(&self) -> &'static str {
        match self {
            Self::SmallBusiness => "小規模事業者",
            Self::StandardBusiness => "一般事業者",
            Self::LargeScaleBusiness => "大規模事業者",
            Self::AiDataBusiness => "AI・データ活用事業者",
        }
    }

    /// Check if annual reporting required to Personal Information Protection Commission
    pub fn requires_annual_reporting(&self) -> bool {
        matches!(self, Self::LargeScaleBusiness | Self::AiDataBusiness)
    }
}

/// Data handling volume (取扱件数)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DataHandlingVolume {
    /// Under 5,000 records
    Under5000,
    /// 5,000-100,000 records
    Under100000,
    /// Over 100,000 records
    Over100000,
}

impl DataHandlingVolume {
    /// Create from record count
    pub fn from_count(count: u64) -> Self {
        if count < 5000 {
            Self::Under5000
        } else if count < 100_000 {
            Self::Under100000
        } else {
            Self::Over100000
        }
    }

    /// Get business type recommendation
    pub fn to_business_type(&self) -> BusinessType {
        match self {
            Self::Under5000 => BusinessType::SmallBusiness,
            Self::Under100000 => BusinessType::StandardBusiness,
            Self::Over100000 => BusinessType::LargeScaleBusiness,
        }
    }
}

/// Purpose type for data usage (利用目的の種類)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PurposeType {
    /// Customer management (顧客管理)
    CustomerManagement,
    /// Marketing and advertising (マーケティング・広告)
    MarketingAdvertising,
    /// Service provision (サービス提供)
    ServiceProvision,
    /// Statistical analysis (統計分析)
    StatisticalAnalysis,
    /// AI training (AI学習)
    AiTraining,
    /// Data analytics (データ分析)
    DataAnalytics,
    /// Contract fulfillment (契約履行)
    ContractFulfillment,
    /// Legal compliance (法令遵守)
    LegalCompliance,
}

impl PurposeType {
    /// Get Japanese name
    pub fn name_ja(&self) -> &'static str {
        match self {
            Self::CustomerManagement => "顧客管理",
            Self::MarketingAdvertising => "マーケティング・広告",
            Self::ServiceProvision => "サービス提供",
            Self::StatisticalAnalysis => "統計分析",
            Self::AiTraining => "AI学習",
            Self::DataAnalytics => "データ分析",
            Self::ContractFulfillment => "契約履行",
            Self::LegalCompliance => "法令遵守",
        }
    }
}

/// Usage purpose (利用目的 - Article 15)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UsagePurpose {
    /// Purpose description
    pub purpose: String,
    /// Purpose type
    pub purpose_type: PurposeType,
    /// Whether specified at collection (Article 15-1)
    pub specified_at_collection: bool,
    /// Whether consent obtained
    pub consent_obtained: bool,
}

/// Security measure type (安全管理措置 - Article 20)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SecurityMeasureType {
    /// Access control (アクセス制御)
    AccessControl,
    /// Encryption (暗号化)
    Encryption,
    /// Access logging (アクセスログ)
    AccessLogging,
    /// Employee training (従業員教育)
    EmployeeTraining,
    /// Incident response plan (漏洩対応計画)
    IncidentResponsePlan,
    /// Data minimization (データ最小化)
    DataMinimization,
    /// Pseudonymization/Anonymization (仮名化・匿名化)
    PseudonymizationAnonymization,
}

impl SecurityMeasureType {
    /// Get Japanese name
    pub fn name_ja(&self) -> &'static str {
        match self {
            Self::AccessControl => "アクセス制御",
            Self::Encryption => "暗号化",
            Self::AccessLogging => "アクセスログ",
            Self::EmployeeTraining => "従業員教育",
            Self::IncidentResponsePlan => "漏洩対応計画",
            Self::DataMinimization => "データ最小化",
            Self::PseudonymizationAnonymization => "仮名化・匿名化",
        }
    }

    /// Check if this is a required measure
    pub fn is_required(&self) -> bool {
        matches!(
            self,
            Self::AccessControl | Self::EmployeeTraining | Self::IncidentResponsePlan
        )
    }
}

/// Security measure (安全管理措置)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SecurityMeasure {
    /// Measure type
    pub measure_type: SecurityMeasureType,
    /// Description
    pub description: String,
    /// Whether implemented
    pub implemented: bool,
}

/// Third-party provision type (第三者提供の種類 - Article 23)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ProvisionType {
    /// With consent (同意あり - Article 23-1)
    WithConsent,
    /// Opt-out (オプトアウト - Article 23-2)
    OptOut,
    /// Joint use (共同利用 - Article 23-5-3)
    JointUse,
    /// Outsourcing (委託 - Article 23-5-1)
    Outsourcing,
}

impl ProvisionType {
    /// Get Japanese name
    pub fn name_ja(&self) -> &'static str {
        match self {
            Self::WithConsent => "同意あり",
            Self::OptOut => "オプトアウト",
            Self::JointUse => "共同利用",
            Self::Outsourcing => "委託",
        }
    }
}

/// Third-party provision information (第三者提供)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ThirdPartyProvision {
    /// Provision type
    pub provision_type: ProvisionType,
    /// Recipients
    pub recipients: Vec<String>,
    /// Whether consent obtained (Article 23-1)
    pub consent_obtained: bool,
    /// Whether opt-out mechanism provided (Article 23-2)
    pub opt_out_provided: bool,
    /// Whether records maintained (Article 25)
    pub record_keeping: bool,
}

/// Cross-border transfer (越境移転 - Article 24)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CrossBorderTransfer {
    /// Destination countries
    pub destination_countries: Vec<String>,
    /// Whether adequacy decision exists (十分性認定)
    pub adequacy_decision: bool,
    /// Whether consent obtained
    pub consent_obtained: bool,
    /// Appropriate protection measures
    pub appropriate_measures: Vec<String>,
}

/// Data subject request type (本人からの請求 - Articles 28-30)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RequestType {
    /// Disclosure request (開示請求 - Article 28)
    Disclosure,
    /// Correction request (訂正請求 - Article 29)
    Correction,
    /// Stop usage request (利用停止請求 - Article 30)
    StopUsage,
    /// Deletion request (削除請求 - Article 30)
    Deletion,
    /// Stop third-party provision (第三者提供停止請求 - Article 30)
    StopThirdParty,
}

impl RequestType {
    /// Get Japanese name
    pub fn name_ja(&self) -> &'static str {
        match self {
            Self::Disclosure => "開示請求",
            Self::Correction => "訂正請求",
            Self::StopUsage => "利用停止請求",
            Self::Deletion => "削除請求",
            Self::StopThirdParty => "第三者提供停止請求",
        }
    }

    /// Get article number
    pub fn article(&self) -> &'static str {
        match self {
            Self::Disclosure => "28",
            Self::Correction => "29",
            Self::StopUsage | Self::Deletion | Self::StopThirdParty => "30",
        }
    }
}

/// Data subject (本人)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DataSubject {
    /// Name
    pub name: String,
    /// Whether identity verified
    pub identification_verified: bool,
}

/// Data subject request (本人からの請求)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DataSubjectRequest {
    /// Request type
    pub request_type: RequestType,
    /// Requester information
    pub requester: DataSubject,
    /// Request date
    pub request_date: NaiveDate,
    /// Data concerned
    pub data_concerned: String,
    /// Response deadline
    pub response_deadline: NaiveDate,
}

/// Personal information handler (個人情報取扱事業者)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PersonalInformationHandler {
    /// Business name
    pub business_name: String,
    /// Business type
    pub business_type: BusinessType,
    /// Data handling volume
    pub handling_volume: DataHandlingVolume,
    /// Types of personal information handled
    pub data_types: Vec<PersonalInfoType>,
    /// Usage purposes
    pub purposes: Vec<UsagePurpose>,
    /// Security measures
    pub security_measures: Vec<SecurityMeasure>,
    /// Third-party provision
    pub third_party_provision: Option<ThirdPartyProvision>,
    /// Cross-border transfer
    pub cross_border_transfer: Option<CrossBorderTransfer>,
}

impl PersonalInformationHandler {
    /// Create new handler
    pub fn new(
        business_name: impl Into<String>,
        business_type: BusinessType,
        handling_volume: DataHandlingVolume,
    ) -> Self {
        Self {
            business_name: business_name.into(),
            business_type,
            handling_volume,
            data_types: Vec::new(),
            purposes: Vec::new(),
            security_measures: Vec::new(),
            third_party_provision: None,
            cross_border_transfer: None,
        }
    }

    /// Check if handler has sensitive data
    pub fn has_sensitive_data(&self) -> bool {
        self.data_types
            .iter()
            .any(|t| matches!(t, PersonalInfoType::Sensitive))
    }

    /// Check if all required security measures are implemented
    pub fn has_required_security_measures(&self) -> bool {
        let required_types = [
            SecurityMeasureType::AccessControl,
            SecurityMeasureType::EmployeeTraining,
            SecurityMeasureType::IncidentResponsePlan,
        ];

        required_types.iter().all(|required_type| {
            self.security_measures
                .iter()
                .any(|m| m.measure_type == *required_type && m.implemented)
        })
    }

    /// Check if annual reporting required
    pub fn requires_annual_reporting(&self) -> bool {
        self.business_type.requires_annual_reporting()
    }
}

/// Risk level for AI systems
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RiskLevel {
    /// Low risk
    Low,
    /// Medium risk
    Medium,
    /// High risk
    High,
    /// Critical risk
    Critical,
}

impl RiskLevel {
    /// Get Japanese name
    pub fn name_ja(&self) -> &'static str {
        match self {
            Self::Low => "低リスク",
            Self::Medium => "中リスク",
            Self::High => "高リスク",
            Self::Critical => "最高リスク",
        }
    }
}

/// AI risk assessment (AIリスク評価 - Digital Agency 2025 focus)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AiRiskAssessment {
    /// AI system name
    pub ai_system_name: String,
    /// Data volume processed
    pub data_volume: u64,
    /// Whether sensitive data included
    pub sensitive_data_included: bool,
    /// Automated decision-making
    pub automated_decision_making: bool,
    /// Profiling of individuals
    pub profiling: bool,
    /// High-risk determination
    pub high_risk_determination: bool,
    /// Risk mitigation measures
    pub risk_mitigation_measures: Vec<String>,
}

/// Risk report for AI assessment
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RiskReport {
    /// Overall risk level
    pub risk_level: RiskLevel,
    /// Risk score (0-100)
    pub risk_score: u32,
    /// Risk factors identified
    pub risk_factors: Vec<String>,
    /// High-risk determination
    pub high_risk_determination: bool,
    /// Recommended measures
    pub recommended_measures: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_personal_info_type_names() {
        assert_eq!(PersonalInfoType::Basic.name_ja(), "個人情報");
        assert_eq!(PersonalInfoType::Sensitive.name_ja(), "要配慮個人情報");
    }

    #[test]
    fn test_personal_info_type_consent() {
        assert!(PersonalInfoType::Sensitive.requires_consent());
        assert!(!PersonalInfoType::Basic.requires_consent());
    }

    #[test]
    fn test_business_type_reporting() {
        assert!(BusinessType::LargeScaleBusiness.requires_annual_reporting());
        assert!(!BusinessType::SmallBusiness.requires_annual_reporting());
    }

    #[test]
    fn test_data_handling_volume() {
        assert_eq!(
            DataHandlingVolume::from_count(3000),
            DataHandlingVolume::Under5000
        );
        assert_eq!(
            DataHandlingVolume::from_count(50000),
            DataHandlingVolume::Under100000
        );
        assert_eq!(
            DataHandlingVolume::from_count(150000),
            DataHandlingVolume::Over100000
        );
    }

    #[test]
    fn test_volume_to_business_type() {
        assert_eq!(
            DataHandlingVolume::Under5000.to_business_type(),
            BusinessType::SmallBusiness
        );
        assert_eq!(
            DataHandlingVolume::Over100000.to_business_type(),
            BusinessType::LargeScaleBusiness
        );
    }

    #[test]
    fn test_security_measure_required() {
        assert!(SecurityMeasureType::AccessControl.is_required());
        assert!(SecurityMeasureType::EmployeeTraining.is_required());
        assert!(!SecurityMeasureType::Encryption.is_required());
    }

    #[test]
    fn test_request_type_article() {
        assert_eq!(RequestType::Disclosure.article(), "28");
        assert_eq!(RequestType::Correction.article(), "29");
        assert_eq!(RequestType::Deletion.article(), "30");
    }

    #[test]
    fn test_handler_creation() {
        let handler = PersonalInformationHandler::new(
            "Test Corp",
            BusinessType::StandardBusiness,
            DataHandlingVolume::Under100000,
        );

        assert_eq!(handler.business_name, "Test Corp");
        assert!(!handler.has_sensitive_data());
        assert!(!handler.has_required_security_measures());
    }

    #[test]
    fn test_handler_sensitive_data() {
        let mut handler = PersonalInformationHandler::new(
            "Test Corp",
            BusinessType::StandardBusiness,
            DataHandlingVolume::Under100000,
        );

        handler.data_types.push(PersonalInfoType::Sensitive);
        assert!(handler.has_sensitive_data());
    }

    #[test]
    fn test_handler_security_measures() {
        let mut handler = PersonalInformationHandler::new(
            "Test Corp",
            BusinessType::StandardBusiness,
            DataHandlingVolume::Under100000,
        );

        handler.security_measures.push(SecurityMeasure {
            measure_type: SecurityMeasureType::AccessControl,
            description: "Access control implemented".to_string(),
            implemented: true,
        });
        handler.security_measures.push(SecurityMeasure {
            measure_type: SecurityMeasureType::EmployeeTraining,
            description: "Training program".to_string(),
            implemented: true,
        });
        handler.security_measures.push(SecurityMeasure {
            measure_type: SecurityMeasureType::IncidentResponsePlan,
            description: "Response plan".to_string(),
            implemented: true,
        });

        assert!(handler.has_required_security_measures());
    }

    #[test]
    fn test_risk_level_ordering() {
        assert!(RiskLevel::Low < RiskLevel::Medium);
        assert!(RiskLevel::Medium < RiskLevel::High);
        assert!(RiskLevel::High < RiskLevel::Critical);
    }
}
