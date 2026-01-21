//! PIPL Types and Data Structures
//!
//! # 个人信息保护法数据类型

#![allow(missing_docs)]

use crate::i18n::BilingualText;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Personal information category
///
/// # 个人信息类别
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PersonalInformationCategory {
    /// 基本信息 / Basic Information
    BasicInfo,
    /// 身份信息 / Identity Information
    IdentityInfo,
    /// 联系信息 / Contact Information
    ContactInfo,
    /// 位置信息 / Location Information
    LocationInfo,
    /// 网络身份标识 / Online Identifiers
    OnlineIdentifiers,
    /// 设备信息 / Device Information
    DeviceInfo,
    /// 行为信息 / Behavioral Information
    BehavioralInfo,
    /// 交易信息 / Transaction Information
    TransactionInfo,
    /// 其他 / Other
    Other(String),
}

impl PersonalInformationCategory {
    /// Get Chinese name
    pub fn name_zh(&self) -> &str {
        match self {
            Self::BasicInfo => "基本信息",
            Self::IdentityInfo => "身份信息",
            Self::ContactInfo => "联系信息",
            Self::LocationInfo => "位置信息",
            Self::OnlineIdentifiers => "网络身份标识",
            Self::DeviceInfo => "设备信息",
            Self::BehavioralInfo => "行为信息",
            Self::TransactionInfo => "交易信息",
            Self::Other(_) => "其他",
        }
    }
}

/// Sensitive personal information category (Article 28)
///
/// # 敏感个人信息类别
///
/// Requires explicit consent and additional safeguards
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SensitivePersonalInformation {
    /// 生物识别信息 / Biometric Information
    Biometric,
    /// 宗教信仰 / Religious Beliefs
    ReligiousBeliefs,
    /// 特定身份 / Specific Identity
    SpecificIdentity,
    /// 医疗健康信息 / Medical/Health Information
    MedicalHealth,
    /// 金融账户信息 / Financial Account Information
    FinancialAccount,
    /// 行踪轨迹信息 / Location Tracking
    LocationTracking,
    /// 未成年人信息 / Information of Minors (under 14)
    MinorInformation,
    /// 其他可能导致歧视的信息 / Other Potentially Discriminatory Information
    OtherDiscriminatory(String),
}

impl SensitivePersonalInformation {
    /// Get Chinese name
    pub fn name_zh(&self) -> &str {
        match self {
            Self::Biometric => "生物识别信息",
            Self::ReligiousBeliefs => "宗教信仰",
            Self::SpecificIdentity => "特定身份",
            Self::MedicalHealth => "医疗健康信息",
            Self::FinancialAccount => "金融账户信息",
            Self::LocationTracking => "行踪轨迹信息",
            Self::MinorInformation => "未成年人信息",
            Self::OtherDiscriminatory(_) => "其他可能导致歧视的信息",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &str {
        match self {
            Self::Biometric => "Biometric Information",
            Self::ReligiousBeliefs => "Religious Beliefs",
            Self::SpecificIdentity => "Specific Identity",
            Self::MedicalHealth => "Medical/Health Information",
            Self::FinancialAccount => "Financial Account Information",
            Self::LocationTracking => "Location Tracking",
            Self::MinorInformation => "Information of Minors",
            Self::OtherDiscriminatory(_) => "Other Potentially Discriminatory Information",
        }
    }
}

/// Legal basis for processing personal information (Article 13)
///
/// # 处理个人信息的法律依据
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProcessingBasis {
    /// 个人同意 / Individual Consent
    Consent,
    /// 订立、履行合同所必需 / Contract Necessity
    ContractNecessity { contract_type: String },
    /// 履行法定职责或义务 / Legal Obligation
    LegalObligation { legal_basis: String },
    /// 应对突发公共卫生事件 / Public Health Emergency
    PublicHealthEmergency,
    /// 公共利益 / Public Interest
    PublicInterest { purpose: String },
    /// 合理范围内处理自行公开的信息 / Publicly Available Information
    PubliclyAvailable,
    /// 法律、行政法规规定的其他情形 / Other Legal Circumstances
    OtherLegal { legal_basis: String },
}

impl ProcessingBasis {
    /// Get Chinese description
    pub fn description_zh(&self) -> String {
        match self {
            Self::Consent => "取得个人同意".to_string(),
            Self::ContractNecessity { contract_type } => {
                format!("为订立、履行{}合同所必需", contract_type)
            }
            Self::LegalObligation { legal_basis } => {
                format!("为履行法定职责或者法定义务所必需（{}）", legal_basis)
            }
            Self::PublicHealthEmergency => {
                "为应对突发公共卫生事件，或者紧急情况下为保护自然人的生命健康".to_string()
            }
            Self::PublicInterest { purpose } => {
                format!("为公共利益实施的{}行为", purpose)
            }
            Self::PubliclyAvailable => {
                "在合理的范围内处理个人自行公开或者其他已经合法公开的个人信息".to_string()
            }
            Self::OtherLegal { legal_basis } => {
                format!("法律、行政法规规定的其他情形（{}）", legal_basis)
            }
        }
    }

    /// Check if consent is required
    pub fn requires_consent(&self) -> bool {
        matches!(self, Self::Consent)
    }
}

/// Personal information handler (处理者)
///
/// Equivalent to GDPR's data controller
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonalInformationHandler {
    /// Handler name
    pub name: BilingualText,
    /// Unified Social Credit Code (统一社会信用代码)
    pub uscc: Option<String>,
    /// Contact information
    pub contact: ContactInfo,
    /// Data Protection Officer (if applicable)
    pub dpo: Option<DataProtectionOfficer>,
    /// Handler category
    pub category: HandlerCategory,
    /// Processing volume
    pub processing_volume: ProcessingVolume,
}

/// Handler category
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HandlerCategory {
    /// 一般处理者 / General Handler
    General,
    /// 提供重要互联网平台服务 / Important Internet Platform
    ImportantInternetPlatform,
    /// 关键信息基础设施运营者 / CII Operator
    CriticalInformationInfrastructure,
    /// 处理敏感个人信息 / Sensitive PI Handler
    SensitivePiHandler,
    /// 国家机关 / Government Agency
    GovernmentAgency,
}

/// Processing volume thresholds
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcessingVolume {
    /// Number of individuals whose PI is processed
    pub total_individuals: u64,
    /// Number of individuals whose sensitive PI is processed
    pub sensitive_pi_individuals: u64,
    /// Cross-border transfer volume
    pub cross_border_individuals: u64,
}

impl ProcessingVolume {
    /// Check if security assessment required for cross-border transfer (Article 40)
    pub fn requires_security_assessment(&self) -> bool {
        // CII operators or processing PI of 1M+ individuals
        self.total_individuals >= 1_000_000
            // or cumulative cross-border transfer of 100k+ individuals
            || self.cross_border_individuals >= 100_000
            // or sensitive PI of 10k+ individuals
            || self.sensitive_pi_individuals >= 10_000
    }

    /// Check if standard contract allowed
    pub fn standard_contract_allowed(&self) -> bool {
        // Not CII operator AND
        // PI of less than 1M individuals AND
        // Cross-border less than 100k AND
        // Sensitive PI cross-border less than 10k
        self.total_individuals < 1_000_000
            && self.cross_border_individuals < 100_000
            && self.sensitive_pi_individuals < 10_000
    }
}

/// Contact information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ContactInfo {
    /// Address
    pub address: Option<String>,
    /// Phone
    pub phone: Option<String>,
    /// Email
    pub email: Option<String>,
}

/// Data Protection Officer (Article 52)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DataProtectionOfficer {
    /// Name
    pub name: String,
    /// Contact information
    pub contact: ContactInfo,
    /// Appointment date
    pub appointment_date: NaiveDate,
}

/// Consent record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsentRecord {
    /// Individual identifier
    pub individual_id: String,
    /// Consent timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Purposes consented to
    pub purposes: Vec<String>,
    /// Categories of PI
    pub pi_categories: Vec<PersonalInformationCategory>,
    /// Sensitive PI categories (if any)
    pub sensitive_categories: Vec<SensitivePersonalInformation>,
    /// Consent type
    pub consent_type: ConsentType,
    /// Consent method
    pub method: ConsentMethod,
    /// Retention period (days)
    pub retention_days: Option<u32>,
    /// Withdrawal record
    pub withdrawal: Option<WithdrawalRecord>,
}

/// Consent type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsentType {
    /// 一般同意 / General Consent
    General,
    /// 单独同意 / Separate Consent (required for sensitive PI)
    Separate,
    /// 书面同意 / Written Consent (required for certain processing)
    Written,
}

/// Consent method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsentMethod {
    /// 电子方式 / Electronic
    Electronic,
    /// 书面签字 / Written Signature
    WrittenSignature,
    /// 勾选框 / Checkbox
    Checkbox,
    /// 口头 / Oral
    Oral,
}

/// Withdrawal record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WithdrawalRecord {
    /// Withdrawal timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Reason (optional)
    pub reason: Option<String>,
    /// Processing stopped
    pub processing_stopped: bool,
}

/// Individual rights under PIPL (Articles 44-49)
///
/// # 个人权利
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IndividualRight {
    /// 知情权 / Right to Know
    RightToKnow,
    /// 决定权 / Right to Decide
    RightToDecide,
    /// 查阅权 / Right to Access
    RightToAccess,
    /// 复制权 / Right to Copy
    RightToCopy,
    /// 更正权 / Right to Rectification
    RightToRectification,
    /// 删除权 / Right to Deletion
    RightToDeletion,
    /// 撤回同意权 / Right to Withdraw Consent
    RightToWithdrawConsent,
    /// 可携带权 / Right to Portability
    RightToPortability,
    /// 拒绝自动化决策权 / Right Against Automated Decision-Making
    RightAgainstAutomatedDecision,
    /// 解释权 / Right to Explanation
    RightToExplanation,
}

impl IndividualRight {
    /// Get Chinese name
    pub fn name_zh(&self) -> &str {
        match self {
            Self::RightToKnow => "知情权",
            Self::RightToDecide => "决定权",
            Self::RightToAccess => "查阅权",
            Self::RightToCopy => "复制权",
            Self::RightToRectification => "更正权",
            Self::RightToDeletion => "删除权",
            Self::RightToWithdrawConsent => "撤回同意权",
            Self::RightToPortability => "可携带权",
            Self::RightAgainstAutomatedDecision => "拒绝自动化决策权",
            Self::RightToExplanation => "解释权",
        }
    }

    /// Get PIPL article reference
    pub fn article(&self) -> u32 {
        match self {
            Self::RightToKnow => 44,
            Self::RightToDecide => 44,
            Self::RightToAccess => 45,
            Self::RightToCopy => 45,
            Self::RightToRectification => 46,
            Self::RightToDeletion => 47,
            Self::RightToWithdrawConsent => 15,
            Self::RightToPortability => 45,
            Self::RightAgainstAutomatedDecision => 24,
            Self::RightToExplanation => 48,
        }
    }
}

/// PI processing activity record (Article 54)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessingActivityRecord {
    /// Activity ID
    pub id: String,
    /// Handler information
    pub handler_name: String,
    /// Processing purposes
    pub purposes: Vec<String>,
    /// PI categories processed
    pub pi_categories: Vec<PersonalInformationCategory>,
    /// Sensitive PI categories
    pub sensitive_categories: Vec<SensitivePersonalInformation>,
    /// Legal basis
    pub legal_basis: ProcessingBasis,
    /// Retention period
    pub retention_period: String,
    /// Security measures
    pub security_measures: Vec<String>,
    /// Cross-border transfer
    pub cross_border: bool,
    /// Record date
    pub record_date: NaiveDate,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processing_basis() {
        let basis = ProcessingBasis::Consent;
        assert!(basis.requires_consent());

        let contract = ProcessingBasis::ContractNecessity {
            contract_type: "劳动".to_string(),
        };
        assert!(!contract.requires_consent());
    }

    #[test]
    fn test_processing_volume_assessment() {
        let volume = ProcessingVolume {
            total_individuals: 500_000,
            sensitive_pi_individuals: 5_000,
            cross_border_individuals: 50_000,
        };
        assert!(!volume.requires_security_assessment());
        assert!(volume.standard_contract_allowed());

        let large_volume = ProcessingVolume {
            total_individuals: 2_000_000,
            sensitive_pi_individuals: 100_000,
            cross_border_individuals: 500_000,
        };
        assert!(large_volume.requires_security_assessment());
        assert!(!large_volume.standard_contract_allowed());
    }

    #[test]
    fn test_sensitive_pi_categories() {
        let biometric = SensitivePersonalInformation::Biometric;
        assert_eq!(biometric.name_zh(), "生物识别信息");
        assert_eq!(biometric.name_en(), "Biometric Information");
    }

    #[test]
    fn test_individual_rights() {
        let right = IndividualRight::RightToDeletion;
        assert_eq!(right.name_zh(), "删除权");
        assert_eq!(right.article(), 47);
    }
}
