//! DPDPA Validation
//!
//! # Digital Personal Data Protection Act, 2023 - Validation

#![allow(missing_docs)]

use super::error::{DpdpaError, DpdpaResult};
use super::types::*;

/// DPDPA compliance report
#[derive(Debug, Clone)]
pub struct DpdpaComplianceReport {
    pub compliant: bool,
    pub violations: Vec<DpdpaError>,
    pub warnings: Vec<String>,
    pub recommendations: Vec<String>,
}

impl Default for DpdpaComplianceReport {
    fn default() -> Self {
        Self {
            compliant: true,
            violations: Vec::new(),
            warnings: Vec::new(),
            recommendations: Vec::new(),
        }
    }
}

/// Validate data fiduciary compliance
pub fn validate_fiduciary_compliance(fiduciary: &DataFiduciary) -> DpdpaComplianceReport {
    let mut report = DpdpaComplianceReport::default();

    // Check SDF obligations
    if matches!(fiduciary.category, DataFiduciaryCategory::Significant) {
        // SDF must have DPO
        if fiduciary.dpo.is_none() {
            report.violations.push(DpdpaError::DpoNotAppointed);
            report.compliant = false;
        } else if let Some(dpo) = &fiduciary.dpo {
            // DPO must be based in India
            if !dpo.based_in_india {
                report.violations.push(DpdpaError::DpoNotInIndia);
                report.compliant = false;
            }
        }

        // SDF should conduct periodic DPIA
        report.recommendations.push(
            "Significant Data Fiduciary should conduct periodic Data Protection Impact Assessment"
                .to_string(),
        );

        // SDF should conduct periodic audits
        report.recommendations.push(
            "Significant Data Fiduciary should conduct periodic audits by independent auditor"
                .to_string(),
        );
    }

    // Check registration (when rules are notified)
    if fiduciary.registration_number.is_none() {
        report.warnings.push(
            "Data fiduciary registration may be required once rules are notified".to_string(),
        );
    }

    report
}

/// Validate consent
pub fn validate_consent(consent: &ConsentRecord) -> DpdpaResult<()> {
    // Check if consent is for specific purpose
    if !consent.specific_purpose {
        return Err(DpdpaError::InvalidConsent {
            reason: "Consent must be for a specific, clear, and lawful purpose".to_string(),
        });
    }

    // Check if consent is limited to necessary data
    if !consent.limited_to_necessary {
        return Err(DpdpaError::InvalidConsent {
            reason: "Consent must be limited to personal data necessary for the purpose"
                .to_string(),
        });
    }

    // Check if consent was withdrawn
    if consent.is_withdrawn() {
        return Err(DpdpaError::ConsentWithdrawalNotHonored);
    }

    Ok(())
}

/// Validate child data processing (Section 9)
pub fn validate_child_processing(processing: &ChildDataProcessing) -> DpdpaComplianceReport {
    let mut report = DpdpaComplianceReport::default();

    // Check parental consent
    if !processing.parental_consent {
        report.violations.push(DpdpaError::ChildDataViolation {
            reason: "Processing child's data without verifiable parental consent".to_string(),
        });
        report.compliant = false;
    }

    // Check age verification
    if !processing.age_verified {
        report.violations.push(DpdpaError::ChildDataViolation {
            reason: "Age of child not verified".to_string(),
        });
        report.compliant = false;
    }

    // Check detrimental effect assessment
    if !processing.detrimental_check {
        report.warnings.push(
            "Assessment for detrimental effect on child's well-being not documented".to_string(),
        );
    }

    // Check tracking (prohibited)
    if processing.tracking_enabled {
        report.violations.push(DpdpaError::ChildDataViolation {
            reason: "Tracking or behavioral monitoring of children is prohibited".to_string(),
        });
        report.compliant = false;
    }

    // Check targeted advertising (prohibited)
    if processing.targeted_advertising {
        report.violations.push(DpdpaError::ChildDataViolation {
            reason: "Targeted advertising directed at children is prohibited".to_string(),
        });
        report.compliant = false;
    }

    report
}

/// Validate cross-border transfer (Section 16)
pub fn validate_cross_border_transfer(transfer: &CrossBorderTransfer) -> DpdpaResult<()> {
    if transfer.country_restricted && !transfer.transfer_allowed {
        return Err(DpdpaError::CrossBorderViolation {
            country: transfer.destination_country.clone(),
        });
    }

    Ok(())
}

/// Validate processing record
pub fn validate_processing_record(record: &ProcessingRecord) -> DpdpaComplianceReport {
    let mut report = DpdpaComplianceReport::default();

    // Check lawful basis
    if matches!(record.lawful_basis, LawfulPurpose::Consent) {
        report.warnings.push(
            "Ensure consent was obtained in accordance with Section 6 requirements".to_string(),
        );
    }

    // Check retention period
    if record.retention_period_days.is_none() {
        report.warnings.push(
            "Retention period not specified - data should be erased when no longer necessary"
                .to_string(),
        );
    }

    // Check cross-border transfer documentation
    if record.cross_border {
        report.warnings.push(
            "Cross-border transfer - ensure destination country is not restricted under Section 16"
                .to_string(),
        );
    }

    report
}

/// Check if entity is Significant Data Fiduciary
pub fn check_sdf_status(criteria: &SdfCriteria) -> bool {
    criteria.qualifies_as_sdf()
}

/// Get required obligations for data fiduciary category
pub fn get_obligations(category: DataFiduciaryCategory) -> Vec<String> {
    let mut obligations = vec![
        "Implement reasonable security safeguards (Section 8(5))".to_string(),
        "Notify Board and data principals of breach (Section 8(6))".to_string(),
        "Erase data no longer needed (Section 8(7))".to_string(),
        "Publish contact details for grievances".to_string(),
        "Respond to data principal requests".to_string(),
    ];

    if matches!(category, DataFiduciaryCategory::Significant) {
        obligations.extend([
            "Appoint Data Protection Officer based in India (Section 10(2)(a))".to_string(),
            "Appoint independent data auditor (Section 10(2)(b))".to_string(),
            "Conduct periodic Data Protection Impact Assessment (Section 10(2)(c))".to_string(),
            "Undertake periodic audits".to_string(),
            "Comply with additional obligations as notified".to_string(),
        ]);
    }

    obligations
}

/// Validate data retention compliance
pub fn validate_retention(
    data_collected_date: chrono::NaiveDate,
    purpose_fulfilled: bool,
    retention_period_days: Option<u32>,
    current_date: chrono::NaiveDate,
) -> DpdpaResult<()> {
    if purpose_fulfilled {
        return Err(DpdpaError::RetentionViolation);
    }

    if let Some(days) = retention_period_days {
        let max_date = data_collected_date + chrono::Days::new(u64::from(days));
        if current_date > max_date {
            return Err(DpdpaError::RetentionViolation);
        }
    }

    Ok(())
}

/// Get data principal rights summary
pub fn get_principal_rights() -> Vec<(DataPrincipalRight, &'static str)> {
    vec![
        (
            DataPrincipalRight::Access,
            "Request summary of personal data and processing activities",
        ),
        (
            DataPrincipalRight::Correction,
            "Request correction of inaccurate or misleading personal data",
        ),
        (
            DataPrincipalRight::Erasure,
            "Request erasure of personal data no longer necessary",
        ),
        (
            DataPrincipalRight::GrievanceRedressal,
            "Have grievances redressed by Data Fiduciary",
        ),
        (
            DataPrincipalRight::Nomination,
            "Nominate person to exercise rights in case of death or incapacity",
        ),
    ]
}

/// Get data principal duties summary
pub fn get_principal_duties() -> Vec<(DataPrincipalDuty, &'static str)> {
    vec![
        (
            DataPrincipalDuty::ComplyWithLaw,
            "Comply with applicable law when exercising rights",
        ),
        (
            DataPrincipalDuty::NoFalseParticulars,
            "Not register false or frivolous grievance",
        ),
        (
            DataPrincipalDuty::NoFrivolousComplaint,
            "Not file false or frivolous complaint with Board",
        ),
        (
            DataPrincipalDuty::NoImpersonation,
            "Not impersonate another person while providing personal data",
        ),
        (
            DataPrincipalDuty::NoSuppression,
            "Not suppress any material information when exercising right",
        ),
        (
            DataPrincipalDuty::AuthenticInformation,
            "Provide verifiably authentic information when required",
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn create_test_fiduciary(is_sdf: bool, has_dpo: bool, dpo_in_india: bool) -> DataFiduciary {
        DataFiduciary {
            registration_number: Some("REG001".to_string()),
            name: "Test Corp".to_string(),
            category: if is_sdf {
                DataFiduciaryCategory::Significant
            } else {
                DataFiduciaryCategory::General
            },
            principal_place: "Mumbai".to_string(),
            contact_email: "privacy@test.com".to_string(),
            dpo: if has_dpo {
                Some(DataProtectionOfficer {
                    name: "DPO Name".to_string(),
                    designation: "Chief Privacy Officer".to_string(),
                    contact_email: "dpo@test.com".to_string(),
                    phone: "+91-9999999999".to_string(),
                    based_in_india: dpo_in_india,
                    appointment_date: NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date"),
                })
            } else {
                None
            },
            consent_manager: None,
            registration_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date")),
        }
    }

    #[test]
    fn test_sdf_without_dpo() {
        let fiduciary = create_test_fiduciary(true, false, false);
        let report = validate_fiduciary_compliance(&fiduciary);
        assert!(!report.compliant);
        assert!(
            report
                .violations
                .iter()
                .any(|e| matches!(e, DpdpaError::DpoNotAppointed))
        );
    }

    #[test]
    fn test_sdf_with_dpo_not_in_india() {
        let fiduciary = create_test_fiduciary(true, true, false);
        let report = validate_fiduciary_compliance(&fiduciary);
        assert!(!report.compliant);
        assert!(
            report
                .violations
                .iter()
                .any(|e| matches!(e, DpdpaError::DpoNotInIndia))
        );
    }

    #[test]
    fn test_sdf_compliant() {
        let fiduciary = create_test_fiduciary(true, true, true);
        let report = validate_fiduciary_compliance(&fiduciary);
        assert!(report.compliant);
    }

    #[test]
    fn test_general_fiduciary() {
        let fiduciary = create_test_fiduciary(false, false, false);
        let report = validate_fiduciary_compliance(&fiduciary);
        assert!(report.compliant); // No DPO required for general
    }

    #[test]
    fn test_consent_validation() {
        let valid_consent = ConsentRecord {
            principal_id: "DP001".to_string(),
            consent_date: NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date"),
            purpose: "Marketing".to_string(),
            data_items: vec!["email".to_string()],
            via_consent_manager: false,
            consent_manager_id: None,
            specific_purpose: true,
            limited_to_necessary: true,
            language: "English".to_string(),
            withdrawn_at: None,
        };
        assert!(validate_consent(&valid_consent).is_ok());

        let invalid_consent = ConsentRecord {
            specific_purpose: false,
            ..valid_consent
        };
        assert!(matches!(
            validate_consent(&invalid_consent),
            Err(DpdpaError::InvalidConsent { .. })
        ));
    }

    #[test]
    fn test_child_processing_validation() {
        let compliant = ChildDataProcessing {
            child_id: "CH001".to_string(),
            parental_consent: true,
            age_verified: true,
            verification_method: "DigiLocker".to_string(),
            purpose: "Education".to_string(),
            detrimental_check: true,
            tracking_enabled: false,
            targeted_advertising: false,
        };
        let report = validate_child_processing(&compliant);
        assert!(report.compliant);

        let non_compliant = ChildDataProcessing {
            tracking_enabled: true,
            ..compliant.clone()
        };
        let report = validate_child_processing(&non_compliant);
        assert!(!report.compliant);
    }

    #[test]
    fn test_sdf_obligations() {
        let general_obligations = get_obligations(DataFiduciaryCategory::General);
        let sdf_obligations = get_obligations(DataFiduciaryCategory::Significant);

        assert!(sdf_obligations.len() > general_obligations.len());
        assert!(
            sdf_obligations
                .iter()
                .any(|o| o.contains("Data Protection Officer"))
        );
    }

    #[test]
    fn test_principal_rights() {
        let rights = get_principal_rights();
        assert_eq!(rights.len(), 5);
    }

    #[test]
    fn test_principal_duties() {
        let duties = get_principal_duties();
        assert_eq!(duties.len(), 6);
    }
}
