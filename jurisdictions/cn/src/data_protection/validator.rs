//! PIPL Validation Functions
//!
//! # 个人信息保护法合规验证

#![allow(missing_docs)]

use super::error::{PiplError, PiplResult};
use super::types::*;
use crate::i18n::BilingualText;

/// Validation report for PIPL compliance
#[derive(Debug, Clone)]
pub struct PiplComplianceReport {
    /// Is compliant
    pub compliant: bool,
    /// Violations found
    pub violations: Vec<PiplError>,
    /// Recommendations
    pub recommendations: Vec<BilingualText>,
    /// Legal basis assessment
    pub legal_basis_valid: bool,
    /// Consent assessment
    pub consent_valid: bool,
    /// Security measures assessment
    pub security_adequate: bool,
    /// Cross-border compliance
    pub cross_border_compliant: bool,
}

impl Default for PiplComplianceReport {
    fn default() -> Self {
        Self {
            compliant: true,
            violations: Vec::new(),
            recommendations: Vec::new(),
            legal_basis_valid: true,
            consent_valid: true,
            security_adequate: true,
            cross_border_compliant: true,
        }
    }
}

/// Validate consent record
pub fn validate_consent(consent: &ConsentRecord) -> PiplResult<()> {
    // Check if consent covers sensitive PI
    if !consent.sensitive_categories.is_empty() {
        // Separate consent required (Article 29)
        if consent.consent_type != ConsentType::Separate {
            return Err(PiplError::MissingSeparateConsent {
                pi_type: consent
                    .sensitive_categories
                    .first()
                    .map(|s| s.name_zh().to_string())
                    .unwrap_or_default(),
            });
        }
    }

    // Check if purposes are specified
    if consent.purposes.is_empty() {
        return Err(PiplError::InvalidConsent {
            reason: "No processing purposes specified / 未指定处理目的".to_string(),
        });
    }

    // Check if consent has been withdrawn
    if consent
        .withdrawal
        .as_ref()
        .is_some_and(|w| w.processing_stopped)
    {
        return Err(PiplError::InvalidConsent {
            reason: "Consent has been withdrawn / 同意已撤回".to_string(),
        });
    }

    Ok(())
}

/// Validate processing activity record (Article 54)
pub fn validate_processing_record(record: &ProcessingActivityRecord) -> PiplResult<()> {
    let mut missing_fields = Vec::new();

    if record.handler_name.is_empty() {
        missing_fields.push("handler_name / 处理者名称".to_string());
    }

    if record.purposes.is_empty() {
        missing_fields.push("purposes / 处理目的".to_string());
    }

    if record.pi_categories.is_empty() {
        missing_fields.push("pi_categories / 个人信息类别".to_string());
    }

    if record.retention_period.is_empty() {
        missing_fields.push("retention_period / 保存期限".to_string());
    }

    if record.security_measures.is_empty() {
        missing_fields.push("security_measures / 安全措施".to_string());
    }

    if !missing_fields.is_empty() {
        return Err(PiplError::IncompleteProcessingRecord { missing_fields });
    }

    Ok(())
}

/// Validate handler compliance
pub fn validate_handler_compliance(handler: &PersonalInformationHandler) -> PiplComplianceReport {
    let mut report = PiplComplianceReport::default();

    // Check if DPO required (Article 52)
    let dpo_required = matches!(
        handler.category,
        HandlerCategory::ImportantInternetPlatform
            | HandlerCategory::CriticalInformationInfrastructure
            | HandlerCategory::SensitivePiHandler
    ) || handler.processing_volume.total_individuals >= 1_000_000;

    if dpo_required && handler.dpo.is_none() {
        report.violations.push(PiplError::MissingDpo);
        report.compliant = false;
    }

    // Check cross-border transfer requirements
    if handler.processing_volume.cross_border_individuals > 0
        && handler.processing_volume.requires_security_assessment()
    {
        report.recommendations.push(BilingualText::new(
            "需要向网信部门申报数据出境安全评估",
            "Security assessment filing required with CAC for cross-border transfer",
        ));

        // For CII operators, security assessment is mandatory
        if matches!(
            handler.category,
            HandlerCategory::CriticalInformationInfrastructure
        ) {
            report
                .violations
                .push(PiplError::SecurityAssessmentRequired {
                    individuals: handler.processing_volume.total_individuals,
                });
            report.cross_border_compliant = false;
            report.compliant = false;
        }
    }

    // Add recommendations based on handler category
    match handler.category {
        HandlerCategory::ImportantInternetPlatform => {
            report.recommendations.push(BilingualText::new(
                "需要建立个人信息保护合规制度体系",
                "Must establish PI protection compliance system",
            ));
            report.recommendations.push(BilingualText::new(
                "需要成立独立机构监督个人信息保护",
                "Must establish independent body for PI protection supervision",
            ));
            report.recommendations.push(BilingualText::new(
                "需要定期发布个人信息保护社会责任报告",
                "Must publish regular PI protection social responsibility reports",
            ));
        }
        HandlerCategory::CriticalInformationInfrastructure => {
            report.recommendations.push(BilingualText::new(
                "个人信息应当在境内存储",
                "PI must be stored domestically",
            ));
            report.recommendations.push(BilingualText::new(
                "确需向境外提供的，应当通过安全评估",
                "If cross-border transfer necessary, must pass security assessment",
            ));
        }
        _ => {}
    }

    report
}

/// Validate security measures (Article 51)
pub fn validate_security_measures(measures: &[String]) -> PiplResult<()> {
    let required_measures = [
        "数据分类分级", // Data classification
        "访问控制",     // Access control
        "加密处理",     // Encryption
        "去标识化处理", // De-identification
        "安全培训",     // Security training
        "应急预案",     // Emergency response plan
    ];

    let mut missing = Vec::new();
    for required in &required_measures {
        if !measures.iter().any(|m| m.contains(required)) {
            missing.push(required.to_string());
        }
    }

    if !missing.is_empty() {
        return Err(PiplError::InadequateSecurityMeasures {
            missing_measures: missing,
        });
    }

    Ok(())
}

/// Validate privacy policy (Article 17)
pub fn validate_privacy_policy(policy: &PrivacyPolicy) -> PiplResult<()> {
    let mut missing = Vec::new();

    if policy.handler_info.is_empty() {
        missing.push("处理者信息 / Handler information".to_string());
    }
    if policy.purposes.is_empty() {
        missing.push("处理目的 / Processing purposes".to_string());
    }
    if policy.pi_categories.is_empty() {
        missing.push("个人信息类别 / PI categories".to_string());
    }
    if policy.retention_period.is_empty() {
        missing.push("保存期限 / Retention period".to_string());
    }
    if policy.individual_rights.is_empty() {
        missing.push("个人权利 / Individual rights".to_string());
    }
    if policy.contact_info.is_empty() {
        missing.push("联系方式 / Contact information".to_string());
    }

    if !missing.is_empty() {
        return Err(PiplError::InadequatePrivacyPolicy { missing });
    }

    Ok(())
}

/// Privacy policy structure
#[derive(Debug, Clone, Default)]
pub struct PrivacyPolicy {
    /// Handler information
    pub handler_info: String,
    /// Processing purposes
    pub purposes: Vec<String>,
    /// PI categories
    pub pi_categories: Vec<String>,
    /// Retention period
    pub retention_period: String,
    /// Individual rights description
    pub individual_rights: String,
    /// Contact information
    pub contact_info: String,
    /// Cross-border transfer disclosure
    pub cross_border_disclosure: Option<String>,
    /// Third-party sharing disclosure
    pub third_party_disclosure: Option<String>,
}

/// Check if processing involves minors (under 14)
pub fn validate_minor_processing(involves_minors: bool, guardian_consent: bool) -> PiplResult<()> {
    if involves_minors && !guardian_consent {
        return Err(PiplError::MinorConsentViolation);
    }
    Ok(())
}

/// Validate automated decision-making (Article 24)
pub fn validate_automated_decision(
    uses_automated_decision: bool,
    transparency_provided: bool,
    opt_out_available: bool,
    affects_major_interests: bool,
) -> PiplResult<()> {
    if uses_automated_decision {
        if !transparency_provided {
            return Err(PiplError::AutomatedDecisionViolation {
                violation: "未提供透明度说明 / Transparency not provided".to_string(),
            });
        }
        if affects_major_interests && !opt_out_available {
            return Err(PiplError::AutomatedDecisionViolation {
                violation: "未提供拒绝权 / Opt-out not available for major decisions".to_string(),
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_validate_consent_basic() {
        let consent = ConsentRecord {
            individual_id: "user123".to_string(),
            timestamp: Utc::now(),
            purposes: vec!["服务提供".to_string()],
            pi_categories: vec![PersonalInformationCategory::BasicInfo],
            sensitive_categories: vec![],
            consent_type: ConsentType::General,
            method: ConsentMethod::Checkbox,
            retention_days: Some(365),
            withdrawal: None,
        };

        assert!(validate_consent(&consent).is_ok());
    }

    #[test]
    fn test_validate_consent_sensitive_pi() {
        let consent = ConsentRecord {
            individual_id: "user123".to_string(),
            timestamp: Utc::now(),
            purposes: vec!["身份验证".to_string()],
            pi_categories: vec![PersonalInformationCategory::IdentityInfo],
            sensitive_categories: vec![SensitivePersonalInformation::Biometric],
            consent_type: ConsentType::General, // Should be Separate
            method: ConsentMethod::Checkbox,
            retention_days: Some(365),
            withdrawal: None,
        };

        let result = validate_consent(&consent);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            PiplError::MissingSeparateConsent { .. }
        ));
    }

    #[test]
    fn test_validate_handler_compliance() {
        let handler = PersonalInformationHandler {
            name: BilingualText::new("测试公司", "Test Company"),
            uscc: Some("91110000000000000X".to_string()),
            contact: ContactInfo::default(),
            dpo: None,
            category: HandlerCategory::ImportantInternetPlatform,
            processing_volume: ProcessingVolume {
                total_individuals: 10_000_000,
                sensitive_pi_individuals: 100_000,
                cross_border_individuals: 0,
            },
        };

        let report = validate_handler_compliance(&handler);
        assert!(!report.compliant); // Missing DPO
    }

    #[test]
    fn test_validate_privacy_policy() {
        let policy = PrivacyPolicy {
            handler_info: "Test Company".to_string(),
            purposes: vec!["Service provision".to_string()],
            pi_categories: vec!["Basic info".to_string()],
            retention_period: "1 year".to_string(),
            individual_rights: "Access, deletion, portability".to_string(),
            contact_info: "privacy@test.com".to_string(),
            ..Default::default()
        };

        assert!(validate_privacy_policy(&policy).is_ok());
    }

    #[test]
    fn test_validate_minor_processing() {
        assert!(validate_minor_processing(true, true).is_ok());
        assert!(validate_minor_processing(true, false).is_err());
        assert!(validate_minor_processing(false, false).is_ok());
    }
}
