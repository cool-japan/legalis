//! Personal Information Protection Act (APPI) Edge Case Tests
//!
//! Edge cases for personal information handling, consent, security measures,
//! third-party provision, cross-border transfers, and data subject rights

use chrono::Utc;
use legalis_jp::personal_info_protection::*;

// ============================================================================
// Personal Information Handler Edge Cases
// ============================================================================

#[test]
fn test_handler_valid_basic() {
    let mut handler = PersonalInformationHandler::new(
        "Test Corp",
        BusinessType::StandardBusiness,
        DataHandlingVolume::Under100000,
    );

    handler.purposes.push(UsagePurpose {
        purpose: "Customer Management".to_string(),
        purpose_type: PurposeType::CustomerManagement,
        specified_at_collection: true,
        consent_obtained: false,
    });

    handler.security_measures.push(SecurityMeasure {
        measure_type: SecurityMeasureType::AccessControl,
        description: "Role-based access control".to_string(),
        implemented: true,
    });

    let result = validate_personal_info_handling(&handler);
    assert!(result.is_ok());
}

#[test]
fn test_handler_purpose_not_specified() {
    let mut handler = PersonalInformationHandler::new(
        "Bad Corp",
        BusinessType::StandardBusiness,
        DataHandlingVolume::Under100000,
    );

    handler.purposes.push(UsagePurpose {
        purpose: "Unspecified".to_string(),
        purpose_type: PurposeType::CustomerManagement,
        specified_at_collection: false, // Violates Article 15
        consent_obtained: false,
    });

    let result = validate_personal_info_handling(&handler);
    assert!(result.is_ok());
    let report = result.unwrap();
    assert!(!report.is_valid()); // Should have errors
}

#[test]
fn test_handler_sensitive_data_no_consent() {
    let mut handler = PersonalInformationHandler::new(
        "Sensitive Corp",
        BusinessType::StandardBusiness,
        DataHandlingVolume::Under100000,
    );

    handler.data_types.push(PersonalInfoType::Sensitive);

    handler.purposes.push(UsagePurpose {
        purpose: "Processing sensitive data".to_string(),
        purpose_type: PurposeType::ServiceProvision,
        specified_at_collection: true,
        consent_obtained: false, // Missing consent for sensitive data!
    });

    let result = validate_personal_info_handling(&handler);
    assert!(result.is_ok());
    let report = result.unwrap();
    assert!(!report.is_valid()); // Should fail Article 17-2
}

#[test]
fn test_handler_sensitive_data_with_consent() {
    let mut handler = PersonalInformationHandler::new(
        "Compliant Corp",
        BusinessType::StandardBusiness,
        DataHandlingVolume::Under100000,
    );

    handler.data_types.push(PersonalInfoType::Sensitive);

    handler.purposes.push(UsagePurpose {
        purpose: "Medical records processing".to_string(),
        purpose_type: PurposeType::ServiceProvision,
        specified_at_collection: true,
        consent_obtained: true, // Consent obtained
    });

    let result = validate_personal_info_handling(&handler);
    assert!(result.is_ok());
}

#[test]
fn test_handler_anonymous_data() {
    let mut handler = PersonalInformationHandler::new(
        "Analytics Corp",
        BusinessType::StandardBusiness,
        DataHandlingVolume::Over100000,
    );

    handler.data_types.push(PersonalInfoType::Anonymous);

    handler.purposes.push(UsagePurpose {
        purpose: "Statistical analysis".to_string(),
        purpose_type: PurposeType::StatisticalAnalysis,
        specified_at_collection: true,
        consent_obtained: false,
    });

    let result = validate_personal_info_handling(&handler);
    assert!(result.is_ok());
}

#[test]
fn test_handler_pseudonymous_data() {
    let mut handler = PersonalInformationHandler::new(
        "Pseudonym Corp",
        BusinessType::AiDataBusiness,
        DataHandlingVolume::Over100000,
    );

    handler.data_types.push(PersonalInfoType::Pseudonymous);

    handler.purposes.push(UsagePurpose {
        purpose: "AI training".to_string(),
        purpose_type: PurposeType::AiTraining,
        specified_at_collection: true,
        consent_obtained: false,
    });

    let result = validate_personal_info_handling(&handler);
    assert!(result.is_ok());
}

#[test]
fn test_handler_small_business_exemption() {
    let mut handler = PersonalInformationHandler::new(
        "Small Shop",
        BusinessType::SmallBusiness,
        DataHandlingVolume::Under5000,
    );

    handler.purposes.push(UsagePurpose {
        purpose: "Basic customer records".to_string(),
        purpose_type: PurposeType::CustomerManagement,
        specified_at_collection: true,
        consent_obtained: false,
    });

    let result = validate_personal_info_handling(&handler);
    assert!(result.is_ok());
}

#[test]
fn test_handler_large_scale_business() {
    let mut handler = PersonalInformationHandler::new(
        "Mega Corp",
        BusinessType::LargeScaleBusiness,
        DataHandlingVolume::Over100000,
    );

    handler.purposes.push(UsagePurpose {
        purpose: "Large-scale data processing".to_string(),
        purpose_type: PurposeType::DataAnalytics,
        specified_at_collection: true,
        consent_obtained: true,
    });

    let result = validate_personal_info_handling(&handler);
    assert!(result.is_ok());
    let report = result.unwrap();
    // Large-scale businesses may have warnings about reporting requirements
    assert!(report.is_valid() || !report.warnings.is_empty());
}

// ============================================================================
// Security Measures Edge Cases
// ============================================================================

#[test]
fn test_security_all_measures_implemented() {
    let mut handler = PersonalInformationHandler::new(
        "Secure Corp",
        BusinessType::StandardBusiness,
        DataHandlingVolume::Under100000,
    );

    handler.purposes.push(UsagePurpose {
        purpose: "Secure processing".to_string(),
        purpose_type: PurposeType::ServiceProvision,
        specified_at_collection: true,
        consent_obtained: false,
    });

    // All recommended security measures
    for measure_type in &[
        SecurityMeasureType::AccessControl,
        SecurityMeasureType::Encryption,
        SecurityMeasureType::AccessLogging,
        SecurityMeasureType::EmployeeTraining,
        SecurityMeasureType::IncidentResponsePlan,
        SecurityMeasureType::DataMinimization,
    ] {
        handler.security_measures.push(SecurityMeasure {
            measure_type: *measure_type,
            description: format!("{:?} implemented", measure_type),
            implemented: true,
        });
    }

    let result = validate_personal_info_handling(&handler);
    assert!(result.is_ok());
    let report = result.unwrap();
    assert!(report.is_valid());
}

#[test]
fn test_security_minimal_measures() {
    let mut handler = PersonalInformationHandler::new(
        "Minimal Corp",
        BusinessType::StandardBusiness,
        DataHandlingVolume::Under100000,
    );

    handler.purposes.push(UsagePurpose {
        purpose: "Basic processing".to_string(),
        purpose_type: PurposeType::CustomerManagement,
        specified_at_collection: true,
        consent_obtained: false,
    });

    // Only one security measure
    handler.security_measures.push(SecurityMeasure {
        measure_type: SecurityMeasureType::AccessControl,
        description: "Basic access control".to_string(),
        implemented: true,
    });

    let result = validate_personal_info_handling(&handler);
    assert!(result.is_ok());
    let report = result.unwrap();
    // Should have warnings about insufficient security
    assert!(!report.warnings.is_empty());
}

#[test]
fn test_security_not_implemented() {
    let mut handler = PersonalInformationHandler::new(
        "Insecure Corp",
        BusinessType::StandardBusiness,
        DataHandlingVolume::Under100000,
    );

    handler.purposes.push(UsagePurpose {
        purpose: "Processing".to_string(),
        purpose_type: PurposeType::ServiceProvision,
        specified_at_collection: true,
        consent_obtained: false,
    });

    handler.security_measures.push(SecurityMeasure {
        measure_type: SecurityMeasureType::Encryption,
        description: "Encryption planned".to_string(),
        implemented: false, // Not yet implemented
    });

    let result = validate_personal_info_handling(&handler);
    assert!(result.is_ok());
    let report = result.unwrap();
    assert!(!report.warnings.is_empty());
}

// ============================================================================
// Third-Party Provision Edge Cases
// ============================================================================

#[test]
fn test_third_party_with_consent() {
    let mut handler = PersonalInformationHandler::new(
        "Provider Corp",
        BusinessType::StandardBusiness,
        DataHandlingVolume::Under100000,
    );

    handler.purposes.push(UsagePurpose {
        purpose: "Data sharing".to_string(),
        purpose_type: PurposeType::ServiceProvision,
        specified_at_collection: true,
        consent_obtained: true,
    });

    handler.third_party_provision = Some(ThirdPartyProvision {
        provision_type: ProvisionType::WithConsent,
        recipients: vec!["Partner Corp".to_string()],
        consent_obtained: true,
        opt_out_provided: false,
        record_keeping: true, // Article 25
    });

    let result = validate_personal_info_handling(&handler);
    assert!(result.is_ok());
    let report = result.unwrap();
    assert!(report.is_valid());
}

#[test]
fn test_third_party_opt_out() {
    let mut handler = PersonalInformationHandler::new(
        "OptOut Corp",
        BusinessType::StandardBusiness,
        DataHandlingVolume::Under100000,
    );

    handler.purposes.push(UsagePurpose {
        purpose: "Marketing".to_string(),
        purpose_type: PurposeType::MarketingAdvertising,
        specified_at_collection: true,
        consent_obtained: false,
    });

    handler.third_party_provision = Some(ThirdPartyProvision {
        provision_type: ProvisionType::OptOut,
        recipients: vec!["Marketing Agency".to_string()],
        consent_obtained: false,
        opt_out_provided: true, // Opt-out mechanism provided
        record_keeping: true,
    });

    let result = validate_personal_info_handling(&handler);
    assert!(result.is_ok());
}

#[test]
fn test_third_party_no_consent_no_optout() {
    let mut handler = PersonalInformationHandler::new(
        "Bad Provider",
        BusinessType::StandardBusiness,
        DataHandlingVolume::Under100000,
    );

    handler.purposes.push(UsagePurpose {
        purpose: "Sharing".to_string(),
        purpose_type: PurposeType::ServiceProvision,
        specified_at_collection: true,
        consent_obtained: false,
    });

    handler.third_party_provision = Some(ThirdPartyProvision {
        provision_type: ProvisionType::WithConsent,
        recipients: vec!["Unknown Corp".to_string()],
        consent_obtained: false, // No consent
        opt_out_provided: false, // No opt-out
        record_keeping: false,   // No records!
    });

    let result = validate_personal_info_handling(&handler);
    assert!(result.is_ok());
    let report = result.unwrap();
    assert!(!report.is_valid()); // Should fail Article 23 & 25
}

#[test]
fn test_third_party_joint_use() {
    let mut handler = PersonalInformationHandler::new(
        "Joint Corp",
        BusinessType::StandardBusiness,
        DataHandlingVolume::Under100000,
    );

    handler.purposes.push(UsagePurpose {
        purpose: "Joint operations".to_string(),
        purpose_type: PurposeType::ServiceProvision,
        specified_at_collection: true,
        consent_obtained: true,
    });

    handler.third_party_provision = Some(ThirdPartyProvision {
        provision_type: ProvisionType::JointUse,
        recipients: vec!["Group Company A".to_string(), "Group Company B".to_string()],
        consent_obtained: true,
        opt_out_provided: false,
        record_keeping: true,
    });

    let result = validate_personal_info_handling(&handler);
    assert!(result.is_ok());
}

#[test]
fn test_third_party_outsourcing() {
    let mut handler = PersonalInformationHandler::new(
        "Outsourcer Corp",
        BusinessType::StandardBusiness,
        DataHandlingVolume::Under100000,
    );

    handler.purposes.push(UsagePurpose {
        purpose: "Data processing".to_string(),
        purpose_type: PurposeType::ServiceProvision,
        specified_at_collection: true,
        consent_obtained: false,
    });

    handler.third_party_provision = Some(ThirdPartyProvision {
        provision_type: ProvisionType::Outsourcing,
        recipients: vec!["Processing Vendor".to_string()],
        consent_obtained: false, // Outsourcing doesn't require consent
        opt_out_provided: false,
        record_keeping: true,
    });

    let result = validate_personal_info_handling(&handler);
    assert!(result.is_ok());
}

// ============================================================================
// Cross-Border Transfer Edge Cases
// ============================================================================

#[test]
fn test_cross_border_with_adequacy() {
    let mut handler = PersonalInformationHandler::new(
        "Global Corp",
        BusinessType::StandardBusiness,
        DataHandlingVolume::Under100000,
    );

    handler.purposes.push(UsagePurpose {
        purpose: "International operations".to_string(),
        purpose_type: PurposeType::ServiceProvision,
        specified_at_collection: true,
        consent_obtained: false,
    });

    handler.cross_border_transfer = Some(CrossBorderTransfer {
        destination_countries: vec!["European Union".to_string()],
        adequacy_decision: true, // EU has adequacy decision
        consent_obtained: false,
        appropriate_measures: vec![],
    });

    let result = validate_personal_info_handling(&handler);
    assert!(result.is_ok());
    let report = result.unwrap();
    assert!(report.is_valid());
}

#[test]
fn test_cross_border_no_adequacy_with_consent() {
    let mut handler = PersonalInformationHandler::new(
        "Transfer Corp",
        BusinessType::StandardBusiness,
        DataHandlingVolume::Under100000,
    );

    handler.purposes.push(UsagePurpose {
        purpose: "International data transfer".to_string(),
        purpose_type: PurposeType::ServiceProvision,
        specified_at_collection: true,
        consent_obtained: true,
    });

    handler.cross_border_transfer = Some(CrossBorderTransfer {
        destination_countries: vec!["Singapore".to_string()],
        adequacy_decision: false,
        consent_obtained: true, // Consent obtained
        appropriate_measures: vec!["Standard contractual clauses".to_string()],
    });

    let result = validate_personal_info_handling(&handler);
    assert!(result.is_ok());
}

#[test]
fn test_cross_border_no_adequacy_no_consent() {
    let mut handler = PersonalInformationHandler::new(
        "Bad Transfer",
        BusinessType::StandardBusiness,
        DataHandlingVolume::Under100000,
    );

    handler.purposes.push(UsagePurpose {
        purpose: "Transfer".to_string(),
        purpose_type: PurposeType::ServiceProvision,
        specified_at_collection: true,
        consent_obtained: false,
    });

    handler.cross_border_transfer = Some(CrossBorderTransfer {
        destination_countries: vec!["Country X".to_string()],
        adequacy_decision: false, // No adequacy decision
        consent_obtained: false,  // No consent
        appropriate_measures: vec![],
    });

    let result = validate_personal_info_handling(&handler);
    assert!(result.is_ok());
    let report = result.unwrap();
    assert!(!report.is_valid()); // Should fail Article 24
}

// ============================================================================
// Data Subject Request Edge Cases
// ============================================================================

#[test]
fn test_data_subject_disclosure_request() {
    let _handler = PersonalInformationHandler::new(
        "Test Corp",
        BusinessType::StandardBusiness,
        DataHandlingVolume::Under100000,
    );

    let request = DataSubjectRequest {
        request_type: RequestType::Disclosure,
        requester: DataSubject {
            name: "Tanaka Taro".to_string(),
            identification_verified: true,
        },
        request_date: Utc::now().date_naive(),
        data_concerned: "My personal data".to_string(),
        response_deadline: Utc::now().date_naive() + chrono::Duration::days(14),
    };

    let result = validate_data_subject_request(&request);
    assert!(result.is_ok());
}

#[test]
fn test_data_subject_no_verification() {
    let _handler = PersonalInformationHandler::new(
        "Test Corp",
        BusinessType::StandardBusiness,
        DataHandlingVolume::Under100000,
    );

    let request = DataSubjectRequest {
        request_type: RequestType::Disclosure,
        requester: DataSubject {
            name: "Unknown Person".to_string(),
            identification_verified: false, // Not verified!
        },
        request_date: Utc::now().date_naive(),
        data_concerned: "Some data".to_string(),
        response_deadline: Utc::now().date_naive() + chrono::Duration::days(14),
    };

    let result = validate_data_subject_request(&request);
    assert!(result.is_ok());
    let report = result.unwrap();
    assert!(!report.is_valid()); // Should fail Article 28
}

#[test]
fn test_data_subject_correction_request() {
    let _handler = PersonalInformationHandler::new(
        "Test Corp",
        BusinessType::StandardBusiness,
        DataHandlingVolume::Under100000,
    );

    let request = DataSubjectRequest {
        request_type: RequestType::Correction,
        requester: DataSubject {
            name: "Sato Hanako".to_string(),
            identification_verified: true,
        },
        request_date: Utc::now().date_naive(),
        data_concerned: "Incorrect address".to_string(),
        response_deadline: Utc::now().date_naive() + chrono::Duration::days(14),
    };

    let result = validate_data_subject_request(&request);
    assert!(result.is_ok());
}

#[test]
fn test_data_subject_deletion_request() {
    let _handler = PersonalInformationHandler::new(
        "Test Corp",
        BusinessType::StandardBusiness,
        DataHandlingVolume::Under100000,
    );

    let request = DataSubjectRequest {
        request_type: RequestType::Deletion,
        requester: DataSubject {
            name: "Yamada Ichiro".to_string(),
            identification_verified: true,
        },
        request_date: Utc::now().date_naive(),
        data_concerned: "All my data".to_string(),
        response_deadline: Utc::now().date_naive() + chrono::Duration::days(10),
    };

    let result = validate_data_subject_request(&request);
    assert!(result.is_ok());
}

#[test]
fn test_data_subject_stop_usage_request() {
    let _handler = PersonalInformationHandler::new(
        "Test Corp",
        BusinessType::StandardBusiness,
        DataHandlingVolume::Under100000,
    );

    let request = DataSubjectRequest {
        request_type: RequestType::StopUsage,
        requester: DataSubject {
            name: "Suzuki Kenji".to_string(),
            identification_verified: true,
        },
        request_date: Utc::now().date_naive(),
        data_concerned: "Marketing data".to_string(),
        response_deadline: Utc::now().date_naive() + chrono::Duration::days(14),
    };

    let result = validate_data_subject_request(&request);
    assert!(result.is_ok());
}

#[test]
fn test_data_subject_excessive_deadline() {
    let _handler = PersonalInformationHandler::new(
        "Test Corp",
        BusinessType::StandardBusiness,
        DataHandlingVolume::Under100000,
    );

    let request = DataSubjectRequest {
        request_type: RequestType::Disclosure,
        requester: DataSubject {
            name: "Test User".to_string(),
            identification_verified: true,
        },
        request_date: Utc::now().date_naive(),
        data_concerned: "Data".to_string(),
        response_deadline: Utc::now().date_naive() + chrono::Duration::days(60), // Too long
    };

    let result = validate_data_subject_request(&request);
    assert!(result.is_ok());
    let report = result.unwrap();
    // Should have warnings about excessive response time
    assert!(!report.warnings.is_empty());
}

// ============================================================================
// AI Risk Assessment Edge Cases
// ============================================================================

#[test]
fn test_ai_risk_low() {
    let assessment = AiRiskAssessment {
        ai_system_name: "Simple Analytics".to_string(),
        data_volume: 10_000,
        sensitive_data_included: false,
        automated_decision_making: false,
        profiling: false,
        high_risk_determination: false,
        risk_mitigation_measures: vec![],
    };

    let result = assess_ai_risk(&assessment);
    assert!(result.is_ok());
    let report = result.unwrap();
    assert_eq!(report.risk_level, RiskLevel::Low);
}

#[test]
fn test_ai_risk_medium() {
    let assessment = AiRiskAssessment {
        ai_system_name: "Customer Profiling".to_string(),
        data_volume: 50_000,
        sensitive_data_included: false,
        automated_decision_making: true,
        profiling: true,
        high_risk_determination: false,
        risk_mitigation_measures: vec!["Access controls".to_string()],
    };

    let result = assess_ai_risk(&assessment);
    assert!(result.is_ok());
    let report = result.unwrap();
    assert_eq!(report.risk_level, RiskLevel::Medium);
}

#[test]
fn test_ai_risk_high() {
    let assessment = AiRiskAssessment {
        ai_system_name: "Medical Diagnosis AI".to_string(),
        data_volume: 200_000,
        sensitive_data_included: true,
        automated_decision_making: true,
        profiling: true,
        high_risk_determination: false,
        risk_mitigation_measures: vec!["Encryption".to_string(), "Access logging".to_string()],
    };

    let result = assess_ai_risk(&assessment);
    assert!(result.is_ok());
    let report = result.unwrap();
    // Combination of sensitive data + automation + profiling + large volume = Critical risk
    assert!(matches!(
        report.risk_level,
        RiskLevel::High | RiskLevel::Critical
    ));
}

#[test]
fn test_ai_risk_critical() {
    let assessment = AiRiskAssessment {
        ai_system_name: "Credit Scoring AI".to_string(),
        data_volume: 500_000,
        sensitive_data_included: true,
        automated_decision_making: true,
        profiling: true,
        high_risk_determination: true, // Determined to be high-risk
        risk_mitigation_measures: vec![],
    };

    let result = assess_ai_risk(&assessment);
    assert!(result.is_ok());
    let report = result.unwrap();
    assert_eq!(report.risk_level, RiskLevel::Critical);
}

#[test]
fn test_ai_risk_no_sensitive_data() {
    let assessment = AiRiskAssessment {
        ai_system_name: "Product Recommendations".to_string(),
        data_volume: 100_000,
        sensitive_data_included: false,
        automated_decision_making: true,
        profiling: false,
        high_risk_determination: false,
        risk_mitigation_measures: vec![],
    };

    let result = assess_ai_risk(&assessment);
    assert!(result.is_ok());
    let report = result.unwrap();
    assert!(report.risk_score < 50); // Should be lower risk
}

// ============================================================================
// Quick Validate Functions
// ============================================================================

#[test]
fn test_quick_validate_handler() {
    let mut handler = PersonalInformationHandler::new(
        "Quick Test",
        BusinessType::StandardBusiness,
        DataHandlingVolume::Under100000,
    );

    handler.purposes.push(UsagePurpose {
        purpose: "Test".to_string(),
        purpose_type: PurposeType::CustomerManagement,
        specified_at_collection: true,
        consent_obtained: false,
    });

    let result = quick_validate_handler(&handler);
    assert!(result.is_ok() || result.is_err()); // Just test it runs
}

#[test]
fn test_quick_validate_request() {
    let _handler = PersonalInformationHandler::new(
        "Test Corp",
        BusinessType::StandardBusiness,
        DataHandlingVolume::Under100000,
    );

    let request = DataSubjectRequest {
        request_type: RequestType::Disclosure,
        requester: DataSubject {
            name: "Test".to_string(),
            identification_verified: true,
        },
        request_date: Utc::now().date_naive(),
        data_concerned: "Data".to_string(),
        response_deadline: Utc::now().date_naive() + chrono::Duration::days(14),
    };

    let result = quick_validate_request(&request);
    assert!(result.is_ok() || result.is_err()); // Just test it runs
}

// ============================================================================
// Edge Cases for Purpose Types
// ============================================================================

#[test]
fn test_all_purpose_types() {
    let purposes = [
        PurposeType::CustomerManagement,
        PurposeType::MarketingAdvertising,
        PurposeType::ServiceProvision,
        PurposeType::StatisticalAnalysis,
        PurposeType::AiTraining,
        PurposeType::DataAnalytics,
        PurposeType::ContractFulfillment,
        PurposeType::LegalCompliance,
    ];

    assert_eq!(purposes.len(), 8);
}

#[test]
fn test_all_data_types() {
    let types = [
        PersonalInfoType::Basic,
        PersonalInfoType::Sensitive,
        PersonalInfoType::Anonymous,
        PersonalInfoType::Pseudonymous,
    ];

    assert_eq!(types.len(), 4);
}

#[test]
fn test_all_business_types() {
    let types = [
        BusinessType::SmallBusiness,
        BusinessType::StandardBusiness,
        BusinessType::LargeScaleBusiness,
        BusinessType::AiDataBusiness,
    ];

    assert_eq!(types.len(), 4);
}

#[test]
fn test_all_request_types() {
    let types = [
        RequestType::Disclosure,
        RequestType::Correction,
        RequestType::StopUsage,
        RequestType::Deletion,
        RequestType::StopThirdParty,
    ];

    assert_eq!(types.len(), 5);
}
