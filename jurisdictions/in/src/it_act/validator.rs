//! Information Technology Act 2000 Validation
//!
//! Validation logic for cyber law compliance

use super::error::{ItActError, ItActResult, ItComplianceReport};
use super::types::*;
use chrono::NaiveDate;

/// Validate digital certificate
pub fn validate_certificate(cert: &DigitalCertificate, check_date: NaiveDate) -> ItActResult<()> {
    if cert.revoked {
        return Err(ItActError::RevokedCertificate);
    }

    if check_date > cert.expiry_date {
        return Err(ItActError::ExpiredCertificate);
    }

    if check_date < cert.issue_date {
        return Err(ItActError::ValidationError {
            message: "Certificate not yet valid".to_string(),
        });
    }

    Ok(())
}

/// Validate intermediary safe harbor eligibility (Section 79)
pub fn validate_safe_harbor(conditions: &SafeHarborConditions) -> ItActResult<()> {
    if !conditions.limited_function {
        return Err(ItActError::NoSafeHarbor {
            reason: "Function not limited to transmission/storage".to_string(),
        });
    }

    if !conditions.does_not_initiate {
        return Err(ItActError::NoSafeHarbor {
            reason: "Intermediary initiates transmission".to_string(),
        });
    }

    if !conditions.does_not_select_receiver {
        return Err(ItActError::NoSafeHarbor {
            reason: "Intermediary selects receiver".to_string(),
        });
    }

    if !conditions.does_not_modify {
        return Err(ItActError::NoSafeHarbor {
            reason: "Intermediary modifies information".to_string(),
        });
    }

    if !conditions.due_diligence {
        return Err(ItActError::DueDiligenceFailure);
    }

    if !conditions.complies_with_govt_directions {
        return Err(ItActError::NoSafeHarbor {
            reason: "Non-compliance with government directions".to_string(),
        });
    }

    if !conditions.does_not_aid {
        return Err(ItActError::NoSafeHarbor {
            reason: "Conspires, abets, aids or induces commission of unlawful act".to_string(),
        });
    }

    Ok(())
}

/// Validate intermediary compliance
pub fn validate_intermediary_compliance(check: &IntermediaryComplianceCheck) -> ItComplianceReport {
    let mut report = ItComplianceReport {
        compliant: true,
        ..Default::default()
    };

    // Basic due diligence requirements (Rule 3)
    if !check.has_privacy_policy {
        report.violations.push(ItActError::DueDiligenceFailure);
        report.compliant = false;
        report
            .recommendations
            .push("Publish privacy policy as required under Rule 3(1)(a)".to_string());
    }

    if !check.has_user_agreement {
        report
            .warnings
            .push("User agreement should be published as per Rule 3(1)(b)".to_string());
    }

    if !check.has_grievance_officer {
        report.violations.push(ItActError::DueDiligenceFailure);
        report.compliant = false;
        report
            .recommendations
            .push("Appoint Grievance Officer as required under Rule 3(2)".to_string());
    }

    // SSMI requirements (50 lakh+ registered users in India)
    let ssmi_threshold: u64 = 5_000_000;
    if check.intermediary_type.is_ssmi_threshold_applicable() && check.user_count >= ssmi_threshold
    {
        if !check.has_compliance_officer {
            report.violations.push(ItActError::SsmiComplianceFailure {
                requirement: "Chief Compliance Officer (Rule 4(1)(a))".to_string(),
            });
            report.compliant = false;
        }

        if !check.has_nodal_person {
            report.violations.push(ItActError::SsmiComplianceFailure {
                requirement: "Nodal Contact Person (Rule 4(1)(b))".to_string(),
            });
            report.compliant = false;
        }

        if !check.has_grievance_officer {
            report.violations.push(ItActError::SsmiComplianceFailure {
                requirement: "Resident Grievance Officer (Rule 4(1)(c))".to_string(),
            });
            report.compliant = false;
        }

        if !check.monthly_report_filed {
            report
                .warnings
                .push("Monthly compliance report required under Rule 4(1)(d)".to_string());
        }
    }

    report
}

/// Validate takedown compliance
pub fn validate_takedown_compliance(
    takedown_received: bool,
    takedown_completed: bool,
    response_time_hours: Option<u32>,
    content_type: &str,
) -> ItActResult<()> {
    if !takedown_received {
        return Ok(());
    }

    if !takedown_completed {
        return Err(ItActError::TakedownNonCompliance);
    }

    // Check response time
    let max_hours = match content_type.to_lowercase().as_str() {
        "intimate images" | "sexually explicit" => 24, // 24 hours for intimate content
        _ => 36,                                       // 36 hours for other unlawful content
    };

    if let Some(hours) = response_time_hours
        && hours > max_hours
    {
        return Err(ItActError::TakedownNonCompliance);
    }

    Ok(())
}

/// Validate data protection compliance (Section 43A / IT Rules 2011)
pub fn validate_data_protection_compliance(
    has_security_practices: bool,
    is_iso_27001_certified: bool,
    has_data_protection_policy: bool,
    sensitive_data_collected: Vec<SensitivePersonalData>,
    written_consent_obtained: bool,
) -> ItComplianceReport {
    let mut report = ItComplianceReport {
        compliant: true,
        ..Default::default()
    };

    // Must implement reasonable security practices
    if !has_security_practices && !is_iso_27001_certified {
        report.violations.push(ItActError::FailureToProtectData {
            details: "No reasonable security practices implemented".to_string(),
        });
        report.compliant = false;
    }

    // Must have documented information security policy
    if !has_data_protection_policy {
        report.warnings.push(
            "Documented information security policy and procedures required under Rule 8"
                .to_string(),
        );
    }

    // Sensitive personal data requires written consent
    if !sensitive_data_collected.is_empty() && !written_consent_obtained {
        report.violations.push(ItActError::FailureToProtectData {
            details: "Written consent required for collecting sensitive personal data".to_string(),
        });
        report.compliant = false;
    }

    // Recommendations
    if !is_iso_27001_certified {
        report.recommendations.push(
            "Consider ISO 27001 certification for deemed compliance with security requirements"
                .to_string(),
        );
    }

    report
}

/// Validate e-commerce compliance
pub fn validate_ecommerce_compliance(
    model: EcommerceModel,
    displays_seller_details: bool,
    has_return_policy: bool,
    has_grievance_mechanism: bool,
    displays_origin_country: bool,
    displays_mrp: bool,
) -> ItComplianceReport {
    let mut report = ItComplianceReport {
        compliant: true,
        ..Default::default()
    };

    // Common requirements
    if !has_grievance_mechanism {
        report.violations.push(ItActError::EcommerceViolation {
            violation: "No grievance redressal mechanism".to_string(),
        });
        report.compliant = false;
    }

    if !has_return_policy {
        report.violations.push(ItActError::EcommerceViolation {
            violation: "Return/refund/exchange policy not displayed".to_string(),
        });
        report.compliant = false;
    }

    // Model-specific requirements
    match model {
        EcommerceModel::Marketplace => {
            if !displays_seller_details {
                report.violations.push(ItActError::EcommerceViolation {
                    violation: "Seller details not displayed on marketplace".to_string(),
                });
                report.compliant = false;
            }
        }
        EcommerceModel::Inventory | EcommerceModel::Hybrid => {
            if !displays_origin_country {
                report
                    .warnings
                    .push("Country of origin should be displayed for products".to_string());
            }

            if !displays_mrp {
                report
                    .warnings
                    .push("MRP and delivery charges should be clearly disclosed".to_string());
            }
        }
    }

    report
}

/// Check cyber crime classification
pub fn classify_cyber_crime(
    unauthorized_access: bool,
    data_theft: bool,
    identity_theft: bool,
    malware_involved: bool,
    targets_critical_infrastructure: bool,
    involves_minors: bool,
    sexually_explicit: bool,
) -> Vec<CyberCrimeCategory> {
    let mut crimes = Vec::new();

    if unauthorized_access {
        crimes.push(CyberCrimeCategory::Hacking);
    }

    if data_theft {
        crimes.push(CyberCrimeCategory::ReceivingStolenResource);
    }

    if identity_theft {
        crimes.push(CyberCrimeCategory::IdentityTheft);
    }

    if malware_involved {
        // Could be Section 43(c) or Section 66
        crimes.push(CyberCrimeCategory::Hacking);
    }

    if targets_critical_infrastructure {
        crimes.push(CyberCrimeCategory::CyberTerrorism);
    }

    if involves_minors && sexually_explicit {
        crimes.push(CyberCrimeCategory::ChildPornography);
    } else if sexually_explicit {
        crimes.push(CyberCrimeCategory::SexuallyExplicitMaterial);
    }

    crimes
}

/// Calculate civil compensation under Section 43
pub fn calculate_section43_compensation(
    offences: &[ComputerOffence],
    actual_loss: f64,
    profits_from_offence: f64,
) -> f64 {
    // Section 43 provides for compensation to affected person
    // No upper limit after 2008 amendment
    // Adjudicating officer determines quantum based on loss suffered

    let base_compensation = actual_loss.max(profits_from_offence);

    // Multiple offences may increase compensation
    let multiplier = match offences.len() {
        1 => 1.0,
        2 => 1.25,
        3 => 1.5,
        _ => 2.0,
    };

    base_compensation * multiplier
}

/// Get limitation period for cyber crimes
pub fn get_limitation_period(crime: CyberCrimeCategory) -> u32 {
    // Based on punishment and Code of Criminal Procedure
    match crime.punishment().imprisonment_max_years {
        Some(years) if years <= 3 => 3, // 3 years limitation
        Some(_) => 0,                   // No limitation for serious crimes
        None => 0,                      // No limitation for life imprisonment crimes
    }
}

/// Get jurisdictional authority
pub fn get_jurisdiction(
    offence_location: &str,
    victim_location: &str,
    server_location: &str,
) -> String {
    // Section 75: Offences committed outside India
    // Any act constituting offence involving computer/network in India

    if offence_location.to_lowercase().contains("india") {
        return format!(
            "Indian jurisdiction - offence committed in {}",
            offence_location
        );
    }

    if victim_location.to_lowercase().contains("india") {
        return "Indian jurisdiction - victim in India".to_string();
    }

    if server_location.to_lowercase().contains("india") {
        return "Indian jurisdiction - computer/network located in India".to_string();
    }

    "May not have Indian jurisdiction - verify extraterritorial provisions".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_certificate_validation() {
        let cert = DigitalCertificate {
            serial_number: "123".to_string(),
            subject_name: "Test".to_string(),
            issuer: "CA".to_string(),
            cert_type: DigitalSignatureType::Class2,
            issue_date: NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date"),
            expiry_date: NaiveDate::from_ymd_opt(2026, 1, 1).expect("valid date"),
            revoked: false,
            revocation_date: None,
        };

        let valid_date = NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date");
        assert!(validate_certificate(&cert, valid_date).is_ok());

        let expired_date = NaiveDate::from_ymd_opt(2027, 1, 1).expect("valid date");
        assert!(matches!(
            validate_certificate(&cert, expired_date),
            Err(ItActError::ExpiredCertificate)
        ));
    }

    #[test]
    fn test_safe_harbor_validation() {
        let valid = SafeHarborConditions {
            limited_function: true,
            does_not_initiate: true,
            does_not_select_receiver: true,
            does_not_modify: true,
            due_diligence: true,
            complies_with_govt_directions: true,
            does_not_aid: true,
        };
        assert!(validate_safe_harbor(&valid).is_ok());

        let invalid = SafeHarborConditions {
            due_diligence: false,
            ..valid
        };
        assert!(matches!(
            validate_safe_harbor(&invalid),
            Err(ItActError::DueDiligenceFailure)
        ));
    }

    #[test]
    fn test_intermediary_compliance() {
        let check = IntermediaryComplianceCheck {
            intermediary_type: IntermediaryType::SocialMedia,
            user_count: 10_000_000, // SSMI threshold exceeded
            has_grievance_officer: true,
            has_privacy_policy: true,
            has_user_agreement: true,
            has_compliance_officer: true,
            has_nodal_person: true,
            monthly_report_filed: true,
        };
        let report = validate_intermediary_compliance(&check);
        assert!(report.compliant);

        let check_fail = IntermediaryComplianceCheck {
            intermediary_type: IntermediaryType::SocialMedia,
            user_count: 10_000_000,
            has_grievance_officer: true,
            has_privacy_policy: true,
            has_user_agreement: true,
            has_compliance_officer: false, // No CCO
            has_nodal_person: false,       // No nodal person
            monthly_report_filed: false,
        };
        let report_ssmi_fail = validate_intermediary_compliance(&check_fail);
        assert!(!report_ssmi_fail.compliant);
    }

    #[test]
    fn test_takedown_compliance() {
        assert!(validate_takedown_compliance(true, true, Some(24), "general").is_ok());
        assert!(validate_takedown_compliance(true, false, None, "general").is_err());
    }

    #[test]
    fn test_cyber_crime_classification() {
        let crimes = classify_cyber_crime(
            true,  // unauthorized access
            true,  // data theft
            false, // identity theft
            true,  // malware
            false, // critical infra
            false, // minors
            false, // sexually explicit
        );
        assert!(crimes.contains(&CyberCrimeCategory::Hacking));
        assert!(crimes.contains(&CyberCrimeCategory::ReceivingStolenResource));
    }

    #[test]
    fn test_cyber_terrorism_classification() {
        let crimes = classify_cyber_crime(
            true,  // unauthorized access
            false, // data theft
            false, // identity theft
            false, // malware
            true,  // targets critical infrastructure
            false, // minors
            false, // sexually explicit
        );
        assert!(crimes.contains(&CyberCrimeCategory::CyberTerrorism));
    }

    #[test]
    fn test_section43_compensation() {
        let offences = vec![ComputerOffence::UnauthorizedAccess];
        let compensation = calculate_section43_compensation(&offences, 100000.0, 50000.0);
        assert_eq!(compensation, 100000.0); // Max of loss and profits

        let multiple_offences = vec![
            ComputerOffence::UnauthorizedAccess,
            ComputerOffence::IntroducingMalware,
            ComputerOffence::DamagingSystem,
        ];
        let compensation = calculate_section43_compensation(&multiple_offences, 100000.0, 50000.0);
        assert_eq!(compensation, 150000.0); // 1.5x multiplier
    }

    #[test]
    fn test_jurisdiction() {
        let jurisdiction = get_jurisdiction("Mumbai, India", "USA", "Singapore");
        assert!(jurisdiction.contains("Indian jurisdiction"));

        let jurisdiction = get_jurisdiction("USA", "Delhi, India", "Singapore");
        assert!(jurisdiction.contains("Indian jurisdiction"));
    }
}
