//! Personal Information Protection Act Validation
//!
//! Validation logic for personal information handling, data subject requests,
//! and AI risk assessments.

use crate::egov::ValidationReport;

use super::error::{AppiError, Result};
use super::types::*;

/// Validate personal information handler
pub fn validate_personal_info_handling(
    handler: &PersonalInformationHandler,
) -> Result<ValidationReport> {
    let mut report = ValidationReport::new();

    // Check business name
    if handler.business_name.is_empty() {
        report.add_error("Missing business name".to_string());
    }

    // Article 15: Purpose specification at collection
    for purpose in &handler.purposes {
        if !purpose.specified_at_collection {
            report.add_error(format!(
                "Purpose '{}' must be specified at collection (個人情報保護法第15条)",
                purpose.purpose
            ));
        }
    }

    // Article 17-2: Consent for sensitive personal information
    if handler.has_sensitive_data() {
        let all_consented = handler.purposes.iter().all(|p| p.consent_obtained);

        if !all_consented {
            report.add_error(
                "Consent required for sensitive personal information (個人情報保護法第17条2項)"
                    .to_string(),
            );
        }
    }

    // Article 20: Security measures
    if !handler.has_required_security_measures() {
        report.add_warning(
            "Required security measures not fully implemented (個人情報保護法第20条)".to_string(),
        );
    }

    // Recommend encryption for large-scale handlers
    if matches!(handler.handling_volume, DataHandlingVolume::Over100000) {
        let has_encryption = handler
            .security_measures
            .iter()
            .any(|m| m.measure_type == SecurityMeasureType::Encryption && m.implemented);

        if !has_encryption {
            report.add_warning("Encryption recommended for large-scale data handling".to_string());
        }
    }

    // Article 23: Third-party provision
    if let Some(provision) = &handler.third_party_provision {
        if !provision.consent_obtained && !provision.opt_out_provided {
            report.add_error(
                "Consent or opt-out required for third-party provision (個人情報保護法第23条)"
                    .to_string(),
            );
        }

        // Article 25: Record keeping
        if !provision.record_keeping {
            report.add_error(
                "Records of third-party provision must be maintained (個人情報保護法第25条)"
                    .to_string(),
            );
        }
    }

    // Article 24: Cross-border transfer
    if let Some(transfer) = &handler.cross_border_transfer {
        if !transfer.adequacy_decision && !transfer.consent_obtained {
            report.add_error(
                "Consent required for cross-border transfer without adequacy decision (個人情報保護法第24条)"
                    .to_string(),
            );
        }

        if transfer.appropriate_measures.is_empty() && !transfer.adequacy_decision {
            report.add_warning(
                "Appropriate protection measures should be documented for cross-border transfer"
                    .to_string(),
            );
        }
    }

    // Large-scale handler obligations
    if handler.requires_annual_reporting() {
        report.add_warning(
            "Annual report to Personal Information Protection Commission required (個人情報保護委員会)"
                .to_string(),
        );
    }

    Ok(report)
}

/// Validate data subject request
pub fn validate_data_subject_request(request: &DataSubjectRequest) -> Result<ValidationReport> {
    let mut report = ValidationReport::new();

    // Check requester name
    if request.requester.name.is_empty() {
        report.add_error("Requester name required".to_string());
    }

    // Article 28: Identity verification required for disclosure
    if matches!(request.request_type, RequestType::Disclosure)
        && !request.requester.identification_verified
    {
        report.add_error(
            "Identity verification required for disclosure requests (個人情報保護法第28条)"
                .to_string(),
        );
    }

    // Check data concerned
    if request.data_concerned.is_empty() {
        report.add_error("Data concerned must be specified".to_string());
    }

    // Response deadline check (typically within 2 weeks)
    let response_time_days = (request.response_deadline - request.request_date).num_days();

    if response_time_days > 14 {
        report.add_warning(
            "Response should be provided without delay, typically within 2 weeks (個人情報保護法第29条)"
                .to_string(),
        );
    }

    if response_time_days < 0 {
        report.add_error("Response deadline cannot be before request date".to_string());
    }

    Ok(report)
}

/// Assess AI risk level
pub fn assess_ai_risk(assessment: &AiRiskAssessment) -> Result<RiskReport> {
    let mut risk_factors = Vec::new();
    let mut risk_score = 0u32;

    // Sensitive data processing
    if assessment.sensitive_data_included {
        risk_factors.push("Sensitive personal information included".to_string());
        risk_score += 30;
    }

    // Automated decision-making
    if assessment.automated_decision_making {
        risk_factors.push("Automated decision-making affecting individuals".to_string());
        risk_score += 25;
    }

    // Profiling
    if assessment.profiling {
        risk_factors.push("Profiling of individuals".to_string());
        risk_score += 20;
    }

    // Large data volume
    if assessment.data_volume > 100_000 {
        risk_factors.push("Large volume of personal data".to_string());
        risk_score += 15;
    }

    // Very large data volume
    if assessment.data_volume > 1_000_000 {
        risk_factors.push("Very large volume of personal data (>1M records)".to_string());
        risk_score += 10;
    }

    // Determine risk level
    let risk_level = if risk_score >= 70 {
        RiskLevel::Critical
    } else if risk_score >= 50 {
        RiskLevel::High
    } else if risk_score >= 30 {
        RiskLevel::Medium
    } else {
        RiskLevel::Low
    };

    // Generate recommendations
    let recommended_measures = generate_risk_mitigation_recommendations(risk_level);

    Ok(RiskReport {
        risk_level,
        risk_score,
        risk_factors,
        high_risk_determination: assessment.high_risk_determination,
        recommended_measures,
    })
}

/// Generate risk mitigation recommendations based on risk level
fn generate_risk_mitigation_recommendations(risk_level: RiskLevel) -> Vec<String> {
    match risk_level {
        RiskLevel::Critical | RiskLevel::High => vec![
            "Conduct Privacy Impact Assessment (PIA)".to_string(),
            "Implement strong encryption for data at rest and in transit".to_string(),
            "Implement pseudonymization or anonymization where possible".to_string(),
            "Establish regular security audits and penetration testing".to_string(),
            "Provide detailed transparency to data subjects".to_string(),
            "Consider appointing Data Protection Officer".to_string(),
            "Implement automated monitoring and breach detection".to_string(),
            "Establish incident response procedures with regular drills".to_string(),
        ],
        RiskLevel::Medium => vec![
            "Implement access controls and activity logging".to_string(),
            "Conduct regular employee training on data protection".to_string(),
            "Establish incident response procedures".to_string(),
            "Review and update security measures quarterly".to_string(),
            "Consider encryption for sensitive data".to_string(),
        ],
        RiskLevel::Low => vec![
            "Maintain basic security measures (access control, training)".to_string(),
            "Document processing activities".to_string(),
            "Review security measures annually".to_string(),
        ],
    }
}

/// Quick validation helper for personal information handler
pub fn quick_validate_handler(handler: &PersonalInformationHandler) -> Result<()> {
    let report = validate_personal_info_handling(handler)?;
    if !report.is_valid() {
        Err(AppiError::Validation(format!(
            "{} validation errors",
            report.errors.len()
        )))
    } else {
        Ok(())
    }
}

/// Quick validation helper for data subject request
pub fn quick_validate_request(request: &DataSubjectRequest) -> Result<()> {
    let report = validate_data_subject_request(request)?;
    if !report.is_valid() {
        Err(AppiError::Validation(format!(
            "{} validation errors",
            report.errors.len()
        )))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_validate_basic_handler() {
        let mut handler = PersonalInformationHandler::new(
            "Test Corp",
            BusinessType::StandardBusiness,
            DataHandlingVolume::Under100000,
        );

        handler.purposes.push(UsagePurpose {
            purpose: "Customer management".to_string(),
            purpose_type: PurposeType::CustomerManagement,
            specified_at_collection: true,
            consent_obtained: false,
        });

        let report = validate_personal_info_handling(&handler).unwrap();
        assert!(report.is_valid());
    }

    #[test]
    fn test_validate_purpose_not_specified() {
        let mut handler = PersonalInformationHandler::new(
            "Test Corp",
            BusinessType::StandardBusiness,
            DataHandlingVolume::Under100000,
        );

        handler.purposes.push(UsagePurpose {
            purpose: "Marketing".to_string(),
            purpose_type: PurposeType::MarketingAdvertising,
            specified_at_collection: false,
            consent_obtained: false,
        });

        let report = validate_personal_info_handling(&handler).unwrap();
        assert!(!report.is_valid());
        assert!(report.errors.len() > 0);
    }

    #[test]
    fn test_validate_sensitive_data_without_consent() {
        let mut handler = PersonalInformationHandler::new(
            "Test Corp",
            BusinessType::StandardBusiness,
            DataHandlingVolume::Under100000,
        );

        handler.data_types.push(PersonalInfoType::Sensitive);
        handler.purposes.push(UsagePurpose {
            purpose: "Service provision".to_string(),
            purpose_type: PurposeType::ServiceProvision,
            specified_at_collection: true,
            consent_obtained: false,
        });

        let report = validate_personal_info_handling(&handler).unwrap();
        assert!(!report.is_valid());
    }

    #[test]
    fn test_validate_sensitive_data_with_consent() {
        let mut handler = PersonalInformationHandler::new(
            "Test Corp",
            BusinessType::StandardBusiness,
            DataHandlingVolume::Under100000,
        );

        handler.data_types.push(PersonalInfoType::Sensitive);
        handler.purposes.push(UsagePurpose {
            purpose: "Service provision".to_string(),
            purpose_type: PurposeType::ServiceProvision,
            specified_at_collection: true,
            consent_obtained: true,
        });

        // Add required security measures
        handler.security_measures.push(SecurityMeasure {
            measure_type: SecurityMeasureType::AccessControl,
            description: "Access control".to_string(),
            implemented: true,
        });
        handler.security_measures.push(SecurityMeasure {
            measure_type: SecurityMeasureType::EmployeeTraining,
            description: "Training".to_string(),
            implemented: true,
        });
        handler.security_measures.push(SecurityMeasure {
            measure_type: SecurityMeasureType::IncidentResponsePlan,
            description: "Response plan".to_string(),
            implemented: true,
        });

        let report = validate_personal_info_handling(&handler).unwrap();
        assert!(report.is_valid());
    }

    #[test]
    fn test_validate_third_party_provision_without_consent() {
        let mut handler = PersonalInformationHandler::new(
            "Test Corp",
            BusinessType::StandardBusiness,
            DataHandlingVolume::Under100000,
        );

        handler.third_party_provision = Some(ThirdPartyProvision {
            provision_type: ProvisionType::WithConsent,
            recipients: vec!["Third Party Inc.".to_string()],
            consent_obtained: false,
            opt_out_provided: false,
            record_keeping: false,
        });

        let report = validate_personal_info_handling(&handler).unwrap();
        assert!(!report.is_valid());
        assert!(report.errors.len() >= 2); // consent + record keeping
    }

    #[test]
    fn test_validate_cross_border_transfer() {
        let mut handler = PersonalInformationHandler::new(
            "Test Corp",
            BusinessType::StandardBusiness,
            DataHandlingVolume::Under100000,
        );

        handler.cross_border_transfer = Some(CrossBorderTransfer {
            destination_countries: vec!["USA".to_string()],
            adequacy_decision: false,
            consent_obtained: false,
            appropriate_measures: vec![],
        });

        let report = validate_personal_info_handling(&handler).unwrap();
        assert!(!report.is_valid());
    }

    #[test]
    fn test_validate_data_subject_request() {
        let request = DataSubjectRequest {
            request_type: RequestType::Disclosure,
            requester: DataSubject {
                name: "John Doe".to_string(),
                identification_verified: true,
            },
            request_date: Utc::now().date_naive(),
            data_concerned: "Personal information".to_string(),
            response_deadline: Utc::now().date_naive() + chrono::Duration::days(7),
        };

        let report = validate_data_subject_request(&request).unwrap();
        assert!(report.is_valid());
    }

    #[test]
    fn test_validate_request_without_verification() {
        let request = DataSubjectRequest {
            request_type: RequestType::Disclosure,
            requester: DataSubject {
                name: "John Doe".to_string(),
                identification_verified: false,
            },
            request_date: Utc::now().date_naive(),
            data_concerned: "Personal information".to_string(),
            response_deadline: Utc::now().date_naive() + chrono::Duration::days(7),
        };

        let report = validate_data_subject_request(&request).unwrap();
        assert!(!report.is_valid());
    }

    #[test]
    fn test_ai_risk_assessment_low() {
        let assessment = AiRiskAssessment {
            ai_system_name: "Simple Analytics".to_string(),
            data_volume: 1000,
            sensitive_data_included: false,
            automated_decision_making: false,
            profiling: false,
            high_risk_determination: false,
            risk_mitigation_measures: vec![],
        };

        let report = assess_ai_risk(&assessment).unwrap();
        assert_eq!(report.risk_level, RiskLevel::Low);
        assert!(report.risk_score < 30);
    }

    #[test]
    fn test_ai_risk_assessment_high() {
        let assessment = AiRiskAssessment {
            ai_system_name: "Credit Scoring AI".to_string(),
            data_volume: 500_000,
            sensitive_data_included: true,
            automated_decision_making: true,
            profiling: true,
            high_risk_determination: true,
            risk_mitigation_measures: vec![],
        };

        let report = assess_ai_risk(&assessment).unwrap();
        assert!(report.risk_level >= RiskLevel::High);
        assert!(report.risk_score >= 50);
        assert!(!report.risk_factors.is_empty());
    }

    #[test]
    fn test_ai_risk_assessment_critical() {
        let assessment = AiRiskAssessment {
            ai_system_name: "Large Scale Profiling".to_string(),
            data_volume: 2_000_000,
            sensitive_data_included: true,
            automated_decision_making: true,
            profiling: true,
            high_risk_determination: true,
            risk_mitigation_measures: vec![],
        };

        let report = assess_ai_risk(&assessment).unwrap();
        assert_eq!(report.risk_level, RiskLevel::Critical);
        assert!(report.risk_score >= 70);
    }

    #[test]
    fn test_quick_validate() {
        let mut handler = PersonalInformationHandler::new(
            "Test Corp",
            BusinessType::StandardBusiness,
            DataHandlingVolume::Under100000,
        );

        handler.purposes.push(UsagePurpose {
            purpose: "Service".to_string(),
            purpose_type: PurposeType::ServiceProvision,
            specified_at_collection: true,
            consent_obtained: false,
        });

        assert!(quick_validate_handler(&handler).is_ok());
    }
}
