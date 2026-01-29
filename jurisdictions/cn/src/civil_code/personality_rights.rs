//! Book IV: Personality Rights (人格权编)
//!
//! Articles 989-1039 of the Civil Code
//!
//! Covers:
//! - Right to life, body, and health
//! - Right to name, image, reputation, and honor
//! - Right to privacy
//! - Personal information protection

use crate::i18n::BilingualText;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

// ============================================================================
// Types
// ============================================================================

/// Personality rights (人格权)
///
/// Articles 989-1039
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PersonalityRight {
    /// Right to life (生命权) - Article 1002
    Life,
    /// Right to body (身体权) - Article 1003
    Body,
    /// Right to health (健康权) - Article 1004
    Health,
    /// Right to name (姓名权) - Article 1012
    Name,
    /// Right to image (肖像权) - Article 1018
    Image,
    /// Right to reputation (名誉权) - Article 1024
    Reputation,
    /// Right to honor (荣誉权) - Article 1031
    Honor,
    /// Right to privacy (隐私权) - Article 1032
    Privacy,
    /// Personal information rights (个人信息权益) - Article 1034
    PersonalInformation,
}

impl PersonalityRight {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::Life => BilingualText::new("生命权", "Right to life"),
            Self::Body => BilingualText::new("身体权", "Right to body"),
            Self::Health => BilingualText::new("健康权", "Right to health"),
            Self::Name => BilingualText::new("姓名权", "Right to name"),
            Self::Image => BilingualText::new("肖像权", "Right to image"),
            Self::Reputation => BilingualText::new("名誉权", "Right to reputation"),
            Self::Honor => BilingualText::new("荣誉权", "Right to honor"),
            Self::Privacy => BilingualText::new("隐私权", "Right to privacy"),
            Self::PersonalInformation => {
                BilingualText::new("个人信息权益", "Personal information rights")
            }
        }
    }
}

/// Privacy scope (隐私范围)
///
/// Article 1032
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrivacyScope {
    /// Private life tranquility (私人生活安宁)
    PrivateLifeTranquility,
    /// Private space (私密空间)
    PrivateSpace,
    /// Private activities (私密活动)
    PrivateActivities,
    /// Private information (私密信息)
    PrivateInformation,
}

impl PrivacyScope {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::PrivateLifeTranquility => {
                BilingualText::new("私人生活安宁", "Private life tranquility")
            }
            Self::PrivateSpace => BilingualText::new("私密空间", "Private space"),
            Self::PrivateActivities => BilingualText::new("私密活动", "Private activities"),
            Self::PrivateInformation => BilingualText::new("私密信息", "Private information"),
        }
    }
}

/// Personal information processing principles (个人信息处理原则)
///
/// Article 1035
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PersonalInfoProcessingPrinciple {
    /// Legality, legitimacy, necessity (合法、正当、必要)
    LegalityLegitimacyNecessity,
    /// Good faith (诚信)
    GoodFaith,
    /// No excessive processing (不得过度处理)
    NoExcessiveProcessing,
}

impl PersonalInfoProcessingPrinciple {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::LegalityLegitimacyNecessity => {
                BilingualText::new("合法、正当、必要原则", "Legality, legitimacy, necessity")
            }
            Self::GoodFaith => BilingualText::new("诚信原则", "Good faith principle"),
            Self::NoExcessiveProcessing => {
                BilingualText::new("不得过度处理", "No excessive processing")
            }
        }
    }
}

/// Personal information rights (个人信息权益)
///
/// Articles 1034-1039
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalInformation {
    /// Information subject (信息主体)
    pub subject: String,
    /// Information type
    pub info_type: BilingualText,
    /// Processing purpose
    pub processing_purpose: BilingualText,
    /// Processor (处理者)
    pub processor: String,
    /// Consent obtained
    pub consent_obtained: bool,
    /// Consent date
    pub consent_date: Option<DateTime<Utc>>,
}

/// Image use (肖像使用)
///
/// Articles 1018-1023
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageUse {
    /// Person whose image is used
    pub person: String,
    /// User of the image
    pub user: String,
    /// Purpose of use
    pub purpose: BilingualText,
    /// Consent obtained
    pub consent_obtained: bool,
    /// Is for-profit use (营利使用)
    pub is_for_profit: bool,
    /// License fee (if applicable)
    pub license_fee: Option<f64>,
    /// Currency
    pub currency: Option<String>,
}

/// Reputation infringement (名誉侵权)
///
/// Articles 1024-1028
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationInfringement {
    /// Infringer
    pub infringer: String,
    /// Victim
    pub victim: String,
    /// Infringement description
    pub description: BilingualText,
    /// Date of infringement
    pub infringement_date: DateTime<Utc>,
    /// Infringement type
    pub infringement_type: ReputationInfringementType,
}

/// Type of reputation infringement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReputationInfringementType {
    /// Insult (侮辱)
    Insult,
    /// Defamation (诽谤)
    Defamation,
    /// False statements harming reputation
    FalseStatements,
}

impl ReputationInfringementType {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::Insult => BilingualText::new("侮辱", "Insult"),
            Self::Defamation => BilingualText::new("诽谤", "Defamation"),
            Self::FalseStatements => {
                BilingualText::new("虚假陈述损害名誉", "False statements harming reputation")
            }
        }
    }
}

/// Privacy infringement (隐私侵权)
///
/// Article 1033
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyInfringement {
    /// Infringer
    pub infringer: String,
    /// Victim
    pub victim: String,
    /// Infringement description
    pub description: BilingualText,
    /// Date of infringement
    pub infringement_date: DateTime<Utc>,
    /// Privacy scope infringed
    pub privacy_scope: PrivacyScope,
    /// Infringement method
    pub method: PrivacyInfringementMethod,
}

/// Method of privacy infringement (隐私侵权方式)
///
/// Article 1033
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrivacyInfringementMethod {
    /// Intrusion into private space (闯入私密空间)
    IntrusionIntoPrivateSpace,
    /// Filming/Photographing private activities (拍摄、窥视、窃听私密活动)
    FilmingPrivateActivities,
    /// Disclosure of private information (泄露、公开私密信息)
    DisclosureOfPrivateInfo,
    /// Collecting private information (收集私密信息)
    CollectingPrivateInfo,
    /// Processing private information (处理私密信息)
    ProcessingPrivateInfo,
}

impl PrivacyInfringementMethod {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::IntrusionIntoPrivateSpace => {
                BilingualText::new("闯入私密空间", "Intrusion into private space")
            }
            Self::FilmingPrivateActivities => BilingualText::new(
                "拍摄、窥视、窃听私密活动",
                "Filming/spying on private activities",
            ),
            Self::DisclosureOfPrivateInfo => {
                BilingualText::new("泄露、公开私密信息", "Disclosure of private information")
            }
            Self::CollectingPrivateInfo => {
                BilingualText::new("收集私密信息", "Collecting private information")
            }
            Self::ProcessingPrivateInfo => {
                BilingualText::new("处理私密信息", "Processing private information")
            }
        }
    }
}

// ============================================================================
// Validators
// ============================================================================

/// Validate personal information processing
///
/// Article 1035: Processors must follow legality, legitimacy, necessity, and good faith principles
pub fn validate_personal_info_processing(
    info: &PersonalInformation,
) -> Result<(), PersonalityRightsError> {
    // Check consent (Article 1035, paragraph 1)
    if !info.consent_obtained {
        return Err(PersonalityRightsError::NoConsentForProcessing {
            subject: info.subject.clone(),
            processor: info.processor.clone(),
        });
    }

    // Check that consent date exists if consent is obtained
    if info.consent_date.is_none() {
        return Err(PersonalityRightsError::MissingConsentDate {
            subject: info.subject.clone(),
        });
    }

    Ok(())
}

/// Validate image use
///
/// Articles 1018-1023
pub fn validate_image_use(image_use: &ImageUse) -> Result<(), PersonalityRightsError> {
    // Article 1019: Image use requires consent unless exception applies
    if !image_use.consent_obtained {
        return Err(PersonalityRightsError::NoConsentForImageUse {
            person: image_use.person.clone(),
            user: image_use.user.clone(),
        });
    }

    // Article 1020: For-profit use requires express consent and may require compensation
    if image_use.is_for_profit && image_use.license_fee.is_none() {
        // Warning: for-profit use typically requires compensation
        // This is not strictly required by law, but recommended
    }

    Ok(())
}

/// Check if privacy infringement occurred
///
/// Articles 1032-1033
pub fn check_privacy_infringement(
    infringement: &PrivacyInfringement,
) -> Result<(), PersonalityRightsError> {
    // Article 1032: Natural persons enjoy right to privacy
    // Article 1033 lists prohibited acts

    // All privacy infringements listed in Article 1033 are prohibited
    Err(PersonalityRightsError::PrivacyInfringement(Box::new(
        PrivacyInfringementError {
            victim: infringement.victim.clone(),
            infringer: infringement.infringer.clone(),
            method: infringement.method.description(),
            privacy_scope: infringement.privacy_scope.description(),
        },
    )))
}

/// Check if reputation infringement occurred
///
/// Articles 1024-1028
pub fn check_reputation_infringement(
    infringement: &ReputationInfringement,
) -> Result<(), PersonalityRightsError> {
    // Article 1024: Natural persons enjoy right to reputation
    // Article 1024, para 2: Infringement by insult, defamation, etc.

    Err(PersonalityRightsError::ReputationInfringement(Box::new(
        ReputationInfringementError {
            victim: infringement.victim.clone(),
            infringer: infringement.infringer.clone(),
            infringement_type: infringement.infringement_type.description(),
        },
    )))
}

// ============================================================================
// Errors
// ============================================================================

/// Privacy infringement error details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyInfringementError {
    /// Victim
    pub victim: String,
    /// Infringer
    pub infringer: String,
    /// Method
    pub method: BilingualText,
    /// Privacy scope
    pub privacy_scope: BilingualText,
}

impl std::fmt::Display for PrivacyInfringementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Privacy infringement against {} by {}: {} ({})",
            self.victim, self.infringer, self.method, self.privacy_scope
        )
    }
}

/// Reputation infringement error details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationInfringementError {
    /// Victim
    pub victim: String,
    /// Infringer
    pub infringer: String,
    /// Infringement type
    pub infringement_type: BilingualText,
}

impl std::fmt::Display for ReputationInfringementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Reputation infringement against {} by {}: {}",
            self.victim, self.infringer, self.infringement_type
        )
    }
}

/// Errors for Personality Rights
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum PersonalityRightsError {
    /// No consent for personal information processing
    #[error("No consent obtained from {subject} for processing by {processor}")]
    NoConsentForProcessing {
        /// Information subject
        subject: String,
        /// Processor
        processor: String,
    },

    /// Missing consent date
    #[error("Missing consent date for {subject}")]
    MissingConsentDate {
        /// Information subject
        subject: String,
    },

    /// No consent for image use
    #[error("No consent obtained from {person} for image use by {user}")]
    NoConsentForImageUse {
        /// Person
        person: String,
        /// User
        user: String,
    },

    /// Privacy infringement
    #[error("Privacy infringement: {0}")]
    PrivacyInfringement(Box<PrivacyInfringementError>),

    /// Reputation infringement
    #[error("Reputation infringement: {0}")]
    ReputationInfringement(Box<ReputationInfringementError>),

    /// Personality right violation
    #[error("Violation of personality right: {right}")]
    PersonalityRightViolation {
        /// Right violated
        right: BilingualText,
    },
}

/// Result type for Personality Rights operations
pub type PersonalityRightsResult<T> = Result<T, PersonalityRightsError>;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_personal_info_processing_with_consent() {
        let info = PersonalInformation {
            subject: "张三".to_string(),
            info_type: BilingualText::new("联系方式", "Contact information"),
            processing_purpose: BilingualText::new("服务提供", "Service provision"),
            processor: "某公司".to_string(),
            consent_obtained: true,
            consent_date: Some(Utc::now()),
        };

        assert!(validate_personal_info_processing(&info).is_ok());
    }

    #[test]
    fn test_personal_info_processing_without_consent() {
        let info = PersonalInformation {
            subject: "李四".to_string(),
            info_type: BilingualText::new("联系方式", "Contact information"),
            processing_purpose: BilingualText::new("营销", "Marketing"),
            processor: "某公司".to_string(),
            consent_obtained: false,
            consent_date: None,
        };

        assert!(validate_personal_info_processing(&info).is_err());
    }

    #[test]
    fn test_image_use_with_consent() {
        let image_use = ImageUse {
            person: "王五".to_string(),
            user: "广告公司".to_string(),
            purpose: BilingualText::new("商业广告", "Commercial advertisement"),
            consent_obtained: true,
            is_for_profit: true,
            license_fee: Some(10000.0),
            currency: Some("CNY".to_string()),
        };

        assert!(validate_image_use(&image_use).is_ok());
    }

    #[test]
    fn test_image_use_without_consent() {
        let image_use = ImageUse {
            person: "赵六".to_string(),
            user: "广告公司".to_string(),
            purpose: BilingualText::new("商业广告", "Commercial advertisement"),
            consent_obtained: false,
            is_for_profit: true,
            license_fee: None,
            currency: None,
        };

        assert!(validate_image_use(&image_use).is_err());
    }

    #[test]
    fn test_privacy_infringement() {
        let infringement = PrivacyInfringement {
            infringer: "侵权人".to_string(),
            victim: "受害人".to_string(),
            description: BilingualText::new(
                "非法收集个人信息",
                "Illegal collection of personal information",
            ),
            infringement_date: Utc::now(),
            privacy_scope: PrivacyScope::PrivateInformation,
            method: PrivacyInfringementMethod::CollectingPrivateInfo,
        };

        assert!(check_privacy_infringement(&infringement).is_err());
    }

    #[test]
    fn test_reputation_infringement() {
        let infringement = ReputationInfringement {
            infringer: "诽谤者".to_string(),
            victim: "受害人".to_string(),
            description: BilingualText::new("散布虚假信息", "Spread false information"),
            infringement_date: Utc::now(),
            infringement_type: ReputationInfringementType::Defamation,
        };

        assert!(check_reputation_infringement(&infringement).is_err());
    }

    #[test]
    fn test_personality_right_descriptions() {
        assert_eq!(PersonalityRight::Life.description().zh, "生命权");
        assert_eq!(PersonalityRight::Privacy.description().zh, "隐私权");
        assert_eq!(
            PersonalityRight::PersonalInformation.description().zh,
            "个人信息权益"
        );
    }
}
