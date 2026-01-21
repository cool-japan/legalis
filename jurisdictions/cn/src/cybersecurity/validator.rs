//! Cybersecurity Law Validation
//!
//! # 网络安全法合规验证

#![allow(missing_docs)]

use super::error::{CybersecurityError, CybersecurityResult};
use super::mlps::{FilingStatus, MlpsAssessment};
use super::types::*;
use crate::i18n::BilingualText;
use chrono::NaiveDate;

/// Cybersecurity compliance report
#[derive(Debug, Clone)]
pub struct CybersecurityComplianceReport {
    pub compliant: bool,
    pub violations: Vec<CybersecurityError>,
    pub recommendations: Vec<BilingualText>,
    pub mlps_compliant: bool,
    pub cii_compliant: bool,
    pub incident_response_ready: bool,
}

impl Default for CybersecurityComplianceReport {
    fn default() -> Self {
        Self {
            compliant: true,
            violations: Vec::new(),
            recommendations: Vec::new(),
            mlps_compliant: true,
            cii_compliant: true,
            incident_response_ready: true,
        }
    }
}

/// Validate network operator compliance
pub fn validate_operator_compliance(
    operator: &NetworkOperator,
    mlps_assessment: Option<&MlpsAssessment>,
    has_security_plan: bool,
    log_retention_days: u32,
    as_of: NaiveDate,
) -> CybersecurityComplianceReport {
    let mut report = CybersecurityComplianceReport::default();

    // Check MLPS level determination (Article 21)
    if operator.mlps_level.is_none() {
        report
            .violations
            .push(CybersecurityError::MlpsLevelNotDetermined {
                system_name: operator.name.zh.clone(),
            });
        report.mlps_compliant = false;
        report.compliant = false;
    }

    // Check MLPS filing for Level 2+
    if let Some(level) = operator.mlps_level
        && level >= MlpsLevel::Level2
    {
        match mlps_assessment {
            Some(assessment) => {
                // Check filing status
                if assessment.filing_status != FilingStatus::Filed {
                    report
                        .violations
                        .push(CybersecurityError::MlpsFilingRequired { level });
                    report.mlps_compliant = false;
                    report.compliant = false;
                }

                // Check assessment validity
                if !assessment.is_valid(as_of) {
                    report
                        .violations
                        .push(CybersecurityError::MlpsAssessmentExpired {
                            last_assessment: assessment.assessment_date.to_string(),
                        });
                    report.mlps_compliant = false;
                    report.compliant = false;
                }

                // Check third-party assessment for Level 3+
                if level >= MlpsLevel::Level3 && assessment.assessor.is_none() {
                    report
                        .violations
                        .push(CybersecurityError::ThirdPartyAssessmentRequired {
                            level: level.level_number(),
                        });
                    report.mlps_compliant = false;
                    report.compliant = false;
                }
            }
            None => {
                report
                    .violations
                    .push(CybersecurityError::MlpsFilingRequired { level });
                report.mlps_compliant = false;
                report.compliant = false;
            }
        }
    }

    // Check security contact (Article 21)
    if operator.security_contact.is_none() {
        report
            .violations
            .push(CybersecurityError::SecurityContactMissing);
        report.compliant = false;
    }

    // Check emergency response plan (Article 21)
    if !has_security_plan {
        report
            .violations
            .push(CybersecurityError::EmergencyPlanMissing);
        report.incident_response_ready = false;
        report.compliant = false;
    }

    // Check log retention (Article 21) - minimum 6 months
    if log_retention_days < 180 {
        report
            .violations
            .push(CybersecurityError::LogRetentionInsufficient {
                actual_days: log_retention_days,
            });
        report.compliant = false;
    }

    // CII-specific requirements
    if operator.category.is_cii() {
        // CII must have security assessment every year
        if let Some(assessment) = mlps_assessment {
            let days_since_assessment = (as_of - assessment.assessment_date).num_days();
            if days_since_assessment > 365 {
                report.recommendations.push(BilingualText::new(
                    "关键信息基础设施应每年进行安全检测评估",
                    "CII should conduct annual security assessment",
                ));
            }
        }

        // Add CII recommendations
        report.recommendations.push(BilingualText::new(
            "应设置专门安全管理机构",
            "Dedicated security management organization required",
        ));
        report.recommendations.push(BilingualText::new(
            "关键岗位人员应进行安全背景审查",
            "Security background checks required for key personnel",
        ));
    }

    report
}

/// Validate CII data localization
pub fn validate_cii_data_localization(
    is_cii: bool,
    data_stored_in_china: bool,
    cross_border_approved: bool,
) -> CybersecurityResult<()> {
    if is_cii && !data_stored_in_china && !cross_border_approved {
        return Err(CybersecurityError::CiiDataLocalizationViolation);
    }
    Ok(())
}

/// Validate incident reporting
pub fn validate_incident_reporting(
    incident: &SecurityIncident,
    reported_on_time: bool,
) -> CybersecurityResult<()> {
    let deadline_hours = incident.severity.reporting_deadline_hours();

    if !reported_on_time {
        return Err(CybersecurityError::IncidentNotReported {
            severity: incident.severity.name_zh().to_string(),
            deadline_hours,
        });
    }

    // Check if report was submitted
    if incident.reported_at.is_none() {
        return Err(CybersecurityError::IncidentNotReported {
            severity: incident.severity.name_zh().to_string(),
            deadline_hours,
        });
    }

    Ok(())
}

/// Check if cybersecurity review is required
pub fn check_review_required(
    is_cii: bool,
    is_platform_operator: bool,
    user_count: u64,
    overseas_listing_planned: bool,
    procurement_affects_national_security: bool,
) -> Option<ReviewTrigger> {
    // CII procurement review
    if is_cii && procurement_affects_national_security {
        return Some(ReviewTrigger::CiiProcurement);
    }

    // Overseas listing review for large platforms
    if overseas_listing_planned {
        if user_count >= 1_000_000 {
            return Some(ReviewTrigger::OverseasListingLargeUserBase);
        }
        if is_platform_operator {
            return Some(ReviewTrigger::OverseasListing);
        }
    }

    None
}

/// Determine required MLPS level for a system
pub fn determine_mlps_level(
    business_importance: u8,
    user_count: u64,
    handles_sensitive_data: bool,
    is_cii_related: bool,
) -> MlpsLevel {
    // CII-related systems are minimum Level 3
    if is_cii_related {
        return if business_importance >= 4 {
            MlpsLevel::Level4
        } else {
            MlpsLevel::Level3
        };
    }

    // Determine based on user count and data sensitivity
    let base_level = match user_count {
        0..=10_000 => 1,
        10_001..=1_000_000 => 2,
        1_000_001..=10_000_000 => 3,
        _ => 4,
    };

    // Adjust for sensitive data
    let adjusted = if handles_sensitive_data {
        base_level + 1
    } else {
        base_level
    };

    // Factor in business importance
    let final_level = adjusted.max(business_importance).min(5);

    match final_level {
        1 => MlpsLevel::Level1,
        2 => MlpsLevel::Level2,
        3 => MlpsLevel::Level3,
        4 => MlpsLevel::Level4,
        _ => MlpsLevel::Level5,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mlps_level_determination() {
        // Small system, no sensitive data
        let level = determine_mlps_level(1, 5_000, false, false);
        assert_eq!(level, MlpsLevel::Level1);

        // Large platform with sensitive data
        let level = determine_mlps_level(3, 5_000_000, true, false);
        assert_eq!(level, MlpsLevel::Level4);

        // CII-related system
        let level = determine_mlps_level(2, 100_000, false, true);
        assert_eq!(level, MlpsLevel::Level3);
    }

    #[test]
    fn test_review_required() {
        // Large platform overseas listing
        let trigger = check_review_required(false, true, 2_000_000, true, false);
        assert!(matches!(
            trigger,
            Some(ReviewTrigger::OverseasListingLargeUserBase)
        ));

        // CII procurement
        let trigger = check_review_required(true, false, 100_000, false, true);
        assert!(matches!(trigger, Some(ReviewTrigger::CiiProcurement)));

        // No review needed
        let trigger = check_review_required(false, false, 50_000, false, false);
        assert!(trigger.is_none());
    }

    #[test]
    fn test_cii_data_localization() {
        // CII with data in China - OK
        assert!(validate_cii_data_localization(true, true, false).is_ok());

        // CII with data abroad, approved - OK
        assert!(validate_cii_data_localization(true, false, true).is_ok());

        // CII with data abroad, not approved - Error
        assert!(validate_cii_data_localization(true, false, false).is_err());

        // Non-CII - always OK
        assert!(validate_cii_data_localization(false, false, false).is_ok());
    }
}
