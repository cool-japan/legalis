//! Personal Information Protection Act (APPI) Compliance Checker
//!
//! Demonstrates compliance checking for personal data handling under the
//! Act on the Protection of Personal Information (個人情報保護法).
//!
//! Run with:
//! ```bash
//! cargo run --example personal-info-compliance-checker
//! ```

use chrono::Utc;
use legalis_jp::personal_info_protection::{
    BusinessType, DataHandlingVolume, DataSubject, DataSubjectRequest, PersonalInfoType,
    PersonalInformationHandler, ProvisionType, PurposeType, RequestType, SecurityMeasure,
    SecurityMeasureType, ThirdPartyProvision, UsagePurpose, validate_data_subject_request,
    validate_personal_info_handling,
};

fn main() {
    println!("=== Personal Information Protection Act (APPI) Compliance Checker ===\n");

    // Example 1: Basic personal info handling validation
    println!("📋 Example 1: Basic Personal Information Handling");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    example_basic_handling();
    println!();

    // Example 2: Large-scale business handling
    println!("🏢 Example 2: Large-Scale Business Handling");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    example_large_scale_business();
    println!();

    // Example 3: Sensitive data handling
    println!("🔒 Example 3: Sensitive Personal Information");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    example_sensitive_data();
    println!();

    // Example 4: Third-party provision
    println!("🤝 Example 4: Third-Party Provision Validation");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    example_third_party_provision();
    println!();

    // Example 5: Data subject requests
    println!("👤 Example 5: Data Subject Rights (Articles 28-30)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    example_data_subject_requests();
}

fn example_basic_handling() {
    let handler = PersonalInformationHandler {
        business_name: "Tokyo E-Commerce Inc.".to_string(),
        business_type: BusinessType::StandardBusiness,
        handling_volume: DataHandlingVolume::Under100000,
        data_types: vec![PersonalInfoType::Basic],
        purposes: vec![
            UsagePurpose {
                purpose: "Customer order processing and fulfillment".to_string(),
                purpose_type: PurposeType::ServiceProvision,
                specified_at_collection: true, // Article 15: Purpose specification
                consent_obtained: true,
            },
            UsagePurpose {
                purpose: "Marketing communications and promotions".to_string(),
                purpose_type: PurposeType::MarketingAdvertising,
                specified_at_collection: true,
                consent_obtained: true,
            },
        ],
        security_measures: vec![
            SecurityMeasure {
                measure_type: SecurityMeasureType::AccessControl,
                description: "Role-based access control with authentication".to_string(),
                implemented: true,
            },
            SecurityMeasure {
                measure_type: SecurityMeasureType::Encryption,
                description: "TLS encryption for data in transit, AES-256 at rest".to_string(),
                implemented: true,
            },
            SecurityMeasure {
                measure_type: SecurityMeasureType::EmployeeTraining,
                description: "Annual privacy training for all employees".to_string(),
                implemented: true,
            },
        ],
        third_party_provision: None,
        cross_border_transfer: None,
    };

    println!("Business: {}", handler.business_name);
    println!("Type: {:?}", handler.business_type);
    println!("Data Volume: {:?}", handler.handling_volume);
    println!("Purposes: {} defined", handler.purposes.len());
    println!(
        "Security Measures: {} implemented",
        handler.security_measures.len()
    );

    match validate_personal_info_handling(&handler) {
        Ok(report) => {
            if report.is_valid() {
                println!("\n✅ Personal information handling is COMPLIANT");
                if !report.warnings.is_empty() {
                    println!("\n⚠️  Recommendations:");
                    for warning in &report.warnings {
                        println!("  • {}", warning);
                    }
                }
            } else {
                println!("\n❌ Compliance issues found:");
                for error in &report.errors {
                    println!("  • {}", error);
                }
            }
        }
        Err(e) => println!("❌ Validation error: {}", e),
    }
}

fn example_large_scale_business() {
    let handler = PersonalInformationHandler {
        business_name: "Mega Data Analytics Corp.".to_string(),
        business_type: BusinessType::LargeScaleBusiness,
        handling_volume: DataHandlingVolume::Over100000, // Over 100,000 records
        data_types: vec![PersonalInfoType::Basic],
        purposes: vec![
            UsagePurpose {
                purpose: "Data analytics and business intelligence".to_string(),
                purpose_type: PurposeType::DataAnalytics,
                specified_at_collection: true,
                consent_obtained: true,
            },
            UsagePurpose {
                purpose: "AI model training and improvement".to_string(),
                purpose_type: PurposeType::AiTraining,
                specified_at_collection: true,
                consent_obtained: true,
            },
        ],
        security_measures: vec![
            SecurityMeasure {
                measure_type: SecurityMeasureType::AccessControl,
                description: "Multi-factor authentication and audit logging".to_string(),
                implemented: true,
            },
            SecurityMeasure {
                measure_type: SecurityMeasureType::Encryption,
                description: "End-to-end encryption with key management".to_string(),
                implemented: true,
            },
            SecurityMeasure {
                measure_type: SecurityMeasureType::DataMinimization,
                description: "Automated data retention and deletion policies".to_string(),
                implemented: true,
            },
            SecurityMeasure {
                measure_type: SecurityMeasureType::PseudonymizationAnonymization,
                description: "Data pseudonymization for analytics (Article 35-2)".to_string(),
                implemented: true,
            },
            SecurityMeasure {
                measure_type: SecurityMeasureType::IncidentResponsePlan,
                description: "24/7 security monitoring and incident response".to_string(),
                implemented: true,
            },
        ],
        third_party_provision: None,
        cross_border_transfer: None,
    };

    println!("Business: {}", handler.business_name);
    println!("Type: {:?} (Large-Scale Handler)", handler.business_type);
    println!(
        "Data Volume: {:?} (>100,000 records)",
        handler.handling_volume
    );
    println!("AI Training Purpose: Yes (requires transparency)");

    match validate_personal_info_handling(&handler) {
        Ok(report) => {
            if report.is_valid() {
                println!("\n✅ Compliance verified for large-scale handler");
                println!("\n📌 Additional obligations:");
                println!("  • Annual report to Personal Information Protection Commission");
                println!("  • Enhanced security measures required");
                println!("  • Transparency in AI processing (Article 35-2)");

                if !report.warnings.is_empty() {
                    println!("\n⚠️  Recommendations:");
                    for warning in &report.warnings {
                        println!("  • {}", warning);
                    }
                }
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }
}

fn example_sensitive_data() {
    // Example with missing consent for sensitive data
    let non_compliant = PersonalInformationHandler {
        business_name: "Healthcare Data Services".to_string(),
        business_type: BusinessType::StandardBusiness,
        handling_volume: DataHandlingVolume::Under100000,
        data_types: vec![PersonalInfoType::Sensitive], // 要配慮個人情報
        purposes: vec![UsagePurpose {
            purpose: "Medical research and analysis".to_string(),
            purpose_type: PurposeType::StatisticalAnalysis,
            specified_at_collection: true,
            consent_obtained: false, // ERROR: Consent required!
        }],
        security_measures: vec![SecurityMeasure {
            measure_type: SecurityMeasureType::Encryption,
            description: "Basic encryption".to_string(),
            implemented: true,
        }],
        third_party_provision: None,
        cross_border_transfer: None,
    };

    println!("Business: {}", non_compliant.business_name);
    println!("Data Type: Sensitive Personal Information (要配慮個人情報)");
    println!("Consent Obtained: No ❌");

    match validate_personal_info_handling(&non_compliant) {
        Ok(report) => {
            if !report.is_valid() {
                println!("\n❌ COMPLIANCE VIOLATION:");
                for error in &report.errors {
                    println!("  • {}", error);
                }
                println!(
                    "\n📌 Article 17-2: Sensitive personal information requires explicit consent"
                );
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }

    // Now with proper consent
    println!("\n--- With Proper Consent ---");
    let compliant = PersonalInformationHandler {
        business_name: "Healthcare Data Services".to_string(),
        business_type: BusinessType::StandardBusiness,
        handling_volume: DataHandlingVolume::Under100000,
        data_types: vec![PersonalInfoType::Sensitive],
        purposes: vec![UsagePurpose {
            purpose: "Medical research and analysis".to_string(),
            purpose_type: PurposeType::StatisticalAnalysis,
            specified_at_collection: true,
            consent_obtained: true, // ✅ Consent obtained
        }],
        security_measures: vec![
            SecurityMeasure {
                measure_type: SecurityMeasureType::Encryption,
                description: "Strong encryption for sensitive data".to_string(),
                implemented: true,
            },
            SecurityMeasure {
                measure_type: SecurityMeasureType::AccessControl,
                description: "Strict access controls for medical data".to_string(),
                implemented: true,
            },
        ],
        third_party_provision: None,
        cross_border_transfer: None,
    };

    match validate_personal_info_handling(&compliant) {
        Ok(report) => {
            if report.is_valid() {
                println!("✅ Now COMPLIANT with Article 17-2");
                println!("  Explicit consent obtained for sensitive data");
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }
}

fn example_third_party_provision() {
    let handler = PersonalInformationHandler {
        business_name: "Marketing Analytics Platform".to_string(),
        business_type: BusinessType::StandardBusiness,
        handling_volume: DataHandlingVolume::Under100000,
        data_types: vec![PersonalInfoType::Basic],
        purposes: vec![UsagePurpose {
            purpose: "Customer analytics and marketing".to_string(),
            purpose_type: PurposeType::MarketingAdvertising,
            specified_at_collection: true,
            consent_obtained: true,
        }],
        security_measures: vec![SecurityMeasure {
            measure_type: SecurityMeasureType::AccessControl,
            description: "Access control implementation".to_string(),
            implemented: true,
        }],
        third_party_provision: Some(ThirdPartyProvision {
            provision_type: ProvisionType::WithConsent,
            recipients: vec![
                "Advertising Partner A".to_string(),
                "Analytics Service B".to_string(),
            ],
            consent_obtained: true, // Article 23: Consent required
            opt_out_provided: false,
            record_keeping: true, // Article 25: Record keeping
        }),
        cross_border_transfer: None,
    };

    println!("Business: {}", handler.business_name);
    println!(
        "Third-Party Recipients: {}",
        handler
            .third_party_provision
            .as_ref()
            .unwrap()
            .recipients
            .len()
    );
    println!("  • Advertising Partner A");
    println!("  • Analytics Service B");

    match validate_personal_info_handling(&handler) {
        Ok(report) => {
            if report.is_valid() {
                println!("\n✅ Third-party provision is COMPLIANT");
                println!("  ✓ Consent obtained (Article 23)");
                println!("  ✓ Records maintained (Article 25)");
            } else {
                println!("\n❌ Compliance issues:");
                for error in &report.errors {
                    println!("  • {}", error);
                }
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }
}

fn example_data_subject_requests() {
    println!("Article 28-30: Data Subject Rights\n");

    // Example 1: Disclosure request (開示請求)
    let disclosure_request = DataSubjectRequest {
        request_type: RequestType::Disclosure,
        requester: DataSubject {
            name: "Tanaka Taro".to_string(),
            identification_verified: true, // Required for disclosure
        },
        request_date: Utc::now().date_naive(),
        data_concerned: "All personal data held by the organization".to_string(),
        response_deadline: Utc::now().date_naive() + chrono::Duration::days(10),
    };

    println!("Request 1: Disclosure (開示請求 - Article 28)");
    println!("  Requester: {}", disclosure_request.requester.name);
    println!(
        "  ID Verified: {}",
        if disclosure_request.requester.identification_verified {
            "Yes"
        } else {
            "No"
        }
    );
    println!(
        "  Response Due: {} days",
        (disclosure_request.response_deadline - disclosure_request.request_date).num_days()
    );

    match validate_data_subject_request(&disclosure_request) {
        Ok(report) => {
            if report.is_valid() {
                println!("  ✅ Request is valid and should be processed");
            } else {
                for error in &report.errors {
                    println!("  ❌ {}", error);
                }
            }
        }
        Err(e) => println!("  ❌ Error: {}", e),
    }

    // Example 2: Correction request (訂正請求)
    println!("\nRequest 2: Correction (訂正請求 - Article 29)");
    let correction_request = DataSubjectRequest {
        request_type: RequestType::Correction,
        requester: DataSubject {
            name: "Sato Hanako".to_string(),
            identification_verified: true,
        },
        request_date: Utc::now().date_naive(),
        data_concerned: "Incorrect address on file".to_string(),
        response_deadline: Utc::now().date_naive() + chrono::Duration::days(7),
    };

    println!("  Requester: {}", correction_request.requester.name);
    println!("  Data: {}", correction_request.data_concerned);

    match validate_data_subject_request(&correction_request) {
        Ok(report) => {
            if report.is_valid() {
                println!("  ✅ Correction request is valid");
                println!("  📌 Must respond without delay (Article 29)");
            }
        }
        Err(e) => println!("  ❌ Error: {}", e),
    }

    // Example 3: Deletion request (削除請求)
    println!("\nRequest 3: Deletion (削除請求 - Article 30)");
    let deletion_request = DataSubjectRequest {
        request_type: RequestType::Deletion,
        requester: DataSubject {
            name: "Suzuki Yuki".to_string(),
            identification_verified: true,
        },
        request_date: Utc::now().date_naive(),
        data_concerned: "All personal data".to_string(),
        response_deadline: Utc::now().date_naive() + chrono::Duration::days(14),
    };

    println!("  Requester: {}", deletion_request.requester.name);
    println!("  Scope: {}", deletion_request.data_concerned);

    match validate_data_subject_request(&deletion_request) {
        Ok(report) => {
            if report.is_valid() {
                println!("  ✅ Deletion request is valid");
                println!("  📌 Right to erasure under Article 30");
            }
        }
        Err(e) => println!("  ❌ Error: {}", e),
    }

    println!("\n📌 Summary of Data Subject Rights:");
    println!("  • Article 28: Right to disclosure (開示)");
    println!("  • Article 29: Right to correction (訂正)");
    println!("  • Article 30: Right to deletion/stop usage (削除・利用停止)");
    println!("  • Response required: Without undue delay (typically within 2 weeks)");
}
