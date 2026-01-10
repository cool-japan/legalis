//! AML/CTF Validators (Money Laundering Regulations 2017, POCA 2002, Terrorism Act 2000)

use super::error::{AmlCtfError, Result};
use super::types::*;

/// Validate Customer Due Diligence (MLR 2017 Reg 27-28)
///
/// Checks compliance with CDD requirements:
/// - Identity verification (Reg 28(2))
/// - Beneficial ownership establishment for entities (Reg 28(3)(b))
/// - Purpose of business relationship (Reg 28(3)(c))
/// - Enhanced DD for PEPs (Reg 35)
/// - Sanctions screening
///
/// # Arguments
///
/// * `cdd` - Customer Due Diligence assessment to validate
///
/// # Returns
///
/// * `Ok(())` if CDD is compliant
/// * `Err(AmlCtfError)` if CDD is non-compliant
///
/// # Example
///
/// ```ignore
/// use legalis_uk::financial_services::aml_ctf::*;
/// use chrono::NaiveDate;
///
/// let cdd = CustomerDueDiligence {
///     customer_name: "John Smith".to_string(),
///     customer_type: CustomerType::Individual,
///     cdd_level: CddLevel::Standard,
///     assessment_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
///     identity_verified: true,
///     identity_documents: vec![/* ... */],
///     // ... other fields
/// };
///
/// validate_cdd(&cdd)?;
/// ```
pub fn validate_cdd(cdd: &CustomerDueDiligence) -> Result<()> {
    // Check identity verified (MLR 2017 Reg 28(2))
    if !cdd.identity_verified {
        return Err(AmlCtfError::IdentityNotVerified {
            customer_name: cdd.customer_name.clone(),
        });
    }

    // Check identity documents provided
    if cdd.identity_documents.is_empty() {
        return Err(AmlCtfError::IdentityNotVerified {
            customer_name: cdd.customer_name.clone(),
        });
    }

    // Check all identity documents are verified
    for doc in &cdd.identity_documents {
        if !doc.verified {
            return Err(AmlCtfError::IdentityNotVerified {
                customer_name: cdd.customer_name.clone(),
            });
        }
    }

    // Check beneficial ownership for entities (MLR 2017 Reg 28(3)(b))
    if matches!(cdd.customer_type, CustomerType::Entity { .. }) {
        // Entities must have beneficial ownership information
        if cdd.beneficial_owners.is_empty() {
            return Err(AmlCtfError::BeneficialOwnershipNotEstablished {
                entity_name: cdd.customer_name.clone(),
            });
        }

        // Ownership structure must be verified
        if !cdd.ownership_structure_verified {
            return Err(AmlCtfError::BeneficialOwnershipNotEstablished {
                entity_name: cdd.customer_name.clone(),
            });
        }

        // Check all beneficial owners have verified identities
        for bo in &cdd.beneficial_owners {
            if !bo.identity_verified {
                return Err(AmlCtfError::BeneficialOwnershipNotEstablished {
                    entity_name: cdd.customer_name.clone(),
                });
            }

            // Beneficial ownership threshold is >25% (MLR 2017 Reg 5)
            if bo.ownership_percentage <= 25.0 {
                return Err(AmlCtfError::ValidationError {
                    message: format!(
                        "Beneficial owner {} has ownership of {:.2}% which is below 25% threshold",
                        bo.name, bo.ownership_percentage
                    ),
                });
            }
        }
    }

    // Check purpose of business relationship established (MLR 2017 Reg 28(3)(c))
    if cdd.purpose_of_relationship.trim().is_empty() {
        return Err(AmlCtfError::PurposeNotEstablished {
            customer_name: cdd.customer_name.clone(),
        });
    }

    if cdd.nature_of_business.trim().is_empty() {
        return Err(AmlCtfError::PurposeNotEstablished {
            customer_name: cdd.customer_name.clone(),
        });
    }

    // Check Enhanced Due Diligence for PEPs (MLR 2017 Reg 35)
    if cdd.pep_status.requires_edd() {
        // PEPs must have Enhanced DD level
        if cdd.cdd_level != CddLevel::Enhanced {
            return Err(AmlCtfError::EddNotPerformedForPep {
                pep_name: cdd.customer_name.clone(),
                position: format!("{:?}", cdd.pep_status),
            });
        }

        // PEPs require source of wealth and funds (MLR 2017 Reg 35(4)(b))
        if cdd.source_of_wealth.is_none() {
            return Err(AmlCtfError::SourceOfWealthNotEstablished {
                pep_name: cdd.customer_name.clone(),
            });
        }

        if cdd.source_of_funds.is_none() {
            return Err(AmlCtfError::SourceOfWealthNotEstablished {
                pep_name: cdd.customer_name.clone(),
            });
        }

        // Check source of wealth/funds are not empty strings
        if let Some(ref sow) = cdd.source_of_wealth {
            if sow.trim().is_empty() {
                return Err(AmlCtfError::SourceOfWealthNotEstablished {
                    pep_name: cdd.customer_name.clone(),
                });
            }
        }

        if let Some(ref sof) = cdd.source_of_funds {
            if sof.trim().is_empty() {
                return Err(AmlCtfError::SourceOfWealthNotEstablished {
                    pep_name: cdd.customer_name.clone(),
                });
            }
        }
    }

    // Check sanctions screening performed and passed
    if !cdd.sanctions_screening_passed {
        return Err(AmlCtfError::SanctionsViolation {
            details: format!(
                "Sanctions screening failed or not passed for customer '{}'",
                cdd.customer_name
            ),
        });
    }

    Ok(())
}

/// Validate Suspicious Activity Report (POCA 2002 s.330-332, Terrorism Act 2000 s.21A)
///
/// Checks compliance with SAR requirements:
/// - Report submitted to NCA
/// - NCA reference obtained
/// - Required information present
///
/// # Arguments
///
/// * `sar` - Suspicious Activity Report to validate
///
/// # Returns
///
/// * `Ok(())` if SAR is compliant
/// * `Err(AmlCtfError)` if SAR is non-compliant
///
/// # Example
///
/// ```ignore
/// use legalis_uk::financial_services::aml_ctf::*;
/// use chrono::NaiveDate;
///
/// let sar = SuspiciousActivityReport {
///     report_id: "SAR-2024-001".to_string(),
///     report_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
///     subject_name: "Suspicious Customer".to_string(),
///     // ... other fields
///     reported_to_nca: true,
///     nca_reference: Some("NCA-REF-12345".to_string()),
/// };
///
/// validate_sar(&sar)?;
/// ```
pub fn validate_sar(sar: &SuspiciousActivityReport) -> Result<()> {
    // Check report has been submitted to NCA
    if !sar.reported_to_nca {
        return Err(AmlCtfError::SarNotFiled {
            subject_name: sar.subject_name.clone(),
            details: format!(
                "SAR for '{}' created on {} but not reported to National Crime Agency",
                sar.subject_name,
                sar.report_date.format("%Y-%m-%d")
            ),
        });
    }

    // Check NCA reference exists (assigned after submission)
    if sar.nca_reference.is_none() {
        return Err(AmlCtfError::SarIncomplete {
            missing_fields: "NCA reference number missing (should be assigned after submission)"
                .to_string(),
        });
    }

    // Validate NCA reference is not empty
    if let Some(ref nca_ref) = sar.nca_reference {
        if nca_ref.trim().is_empty() {
            return Err(AmlCtfError::SarIncomplete {
                missing_fields: "NCA reference number is empty".to_string(),
            });
        }
    }

    // Check subject name is not empty
    if sar.subject_name.trim().is_empty() {
        return Err(AmlCtfError::SarIncomplete {
            missing_fields: "Subject name".to_string(),
        });
    }

    // Check grounds for suspicion are provided
    if sar.grounds_for_suspicion.trim().is_empty() {
        return Err(AmlCtfError::SarIncomplete {
            missing_fields: "Grounds for suspicion - must provide detailed explanation".to_string(),
        });
    }

    // Validate grounds for suspicion has sufficient detail (at least 50 characters)
    if sar.grounds_for_suspicion.len() < 50 {
        return Err(AmlCtfError::SarIncomplete {
            missing_fields: "Grounds for suspicion - insufficient detail provided (minimum 50 characters required for meaningful SAR)".to_string(),
        });
    }

    Ok(())
}

/// Validate sanctions screening (Sanctions and Anti-Money Laundering Act 2018)
///
/// Checks compliance with sanctions screening requirements:
/// - Screening performed
/// - Appropriate sanctions lists checked
/// - Matches resolved (true positive or false positive)
///
/// # Arguments
///
/// * `screening` - Sanctions screening result to validate
///
/// # Returns
///
/// * `Ok(())` if screening is compliant
/// * `Err(AmlCtfError)` if screening is non-compliant
pub fn validate_sanctions_screening(screening: &SanctionsScreening) -> Result<()> {
    // Check at least one sanctions list has been checked
    if !screening.ofsi_checked && !screening.un_checked && !screening.eu_checked {
        return Err(AmlCtfError::SanctionsScreeningNotPerformed {
            subject_name: screening.subject_name.clone(),
        });
    }

    // If match found, must be resolved
    if screening.match_found {
        // If not marked as false positive, this is a true positive = sanctions violation
        if !screening.false_positive {
            let match_details = screening
                .match_details
                .clone()
                .unwrap_or_else(|| "Match found on sanctions list".to_string());

            return Err(AmlCtfError::SanctionsViolation {
                details: format!(
                    "Sanctions match for '{}': {}",
                    screening.subject_name, match_details
                ),
            });
        }

        // If marked as false positive, must have reviewer
        if screening.false_positive && screening.reviewed_by.is_none() {
            return Err(AmlCtfError::SanctionsMatchNotResolved {
                subject_name: screening.subject_name.clone(),
                sanctions_list: "Unknown".to_string(),
            });
        }
    }

    Ok(())
}

/// Validate Travel Rule compliance (MLR 2017 reg 14A, FATF Recommendation 16)
///
/// Checks compliance with Travel Rule for cryptoasset transfers:
/// - Information transmission for transfers ≥£1,000
/// - Complete originator information
/// - Complete beneficiary information
///
/// # Arguments
///
/// * `transfer` - Travel Rule transfer to validate
///
/// # Returns
///
/// * `Ok(())` if transfer is compliant
/// * `Err(AmlCtfError)` if transfer is non-compliant
///
/// # Example
///
/// ```ignore
/// use legalis_uk::financial_services::aml_ctf::*;
/// use chrono::NaiveDate;
///
/// let transfer = TravelRuleTransfer {
///     transaction_id: "TX-001".to_string(),
///     transaction_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
///     amount_gbp: 5000.0,
///     originator_name: "Alice Smith".to_string(),
///     originator_wallet_address: "0x1234...".to_string(),
///     // ... other fields
///     information_transmitted: true,
/// };
///
/// validate_travel_rule(&transfer)?;
/// ```
pub fn validate_travel_rule(transfer: &TravelRuleTransfer) -> Result<()> {
    // Travel Rule only applies to transfers ≥£1000 (MLR 2017 reg 14A)
    if !transfer.travel_rule_applies() {
        return Ok(());
    }

    // Check information has been transmitted
    if !transfer.information_transmitted {
        return Err(AmlCtfError::TravelRuleViolation {
            amount_gbp: transfer.amount_gbp,
        });
    }

    // Validate originator information (MLR 2017 reg 14A(3))
    if transfer.originator_name.trim().is_empty() {
        return Err(AmlCtfError::IncompleteOriginatorInfo {
            amount_gbp: transfer.amount_gbp,
        });
    }

    if transfer.originator_wallet_address.trim().is_empty() {
        return Err(AmlCtfError::IncompleteOriginatorInfo {
            amount_gbp: transfer.amount_gbp,
        });
    }

    // Validate beneficiary information (MLR 2017 reg 14A(4))
    if transfer.beneficiary_name.trim().is_empty() {
        return Err(AmlCtfError::IncompleteBeneficiaryInfo {
            amount_gbp: transfer.amount_gbp,
        });
    }

    if transfer.beneficiary_wallet_address.trim().is_empty() {
        return Err(AmlCtfError::IncompleteBeneficiaryInfo {
            amount_gbp: transfer.amount_gbp,
        });
    }

    Ok(())
}

/// Validate Enhanced Due Diligence requirements (MLR 2017 Reg 33-35)
///
/// Checks if Enhanced DD has been appropriately applied for high-risk customers.
///
/// # Arguments
///
/// * `cdd` - Customer Due Diligence assessment to validate
/// * `senior_management_approval` - Whether senior management approval obtained (for PEPs)
///
/// # Returns
///
/// * `Ok(())` if Enhanced DD is compliant
/// * `Err(AmlCtfError)` if Enhanced DD is non-compliant
pub fn validate_enhanced_dd(
    cdd: &CustomerDueDiligence,
    senior_management_approval: bool,
) -> Result<()> {
    // If PEP, check senior management approval (MLR 2017 Reg 35(4)(a))
    if cdd.pep_status.requires_edd() && !senior_management_approval {
        return Err(AmlCtfError::SeniorManagementApprovalNotObtained {
            pep_name: cdd.customer_name.clone(),
        });
    }

    // Check Enhanced DD level applied
    if cdd.pep_status.requires_edd() && cdd.cdd_level != CddLevel::Enhanced {
        return Err(AmlCtfError::EddNotPerformedForPep {
            pep_name: cdd.customer_name.clone(),
            position: format!("{:?}", cdd.pep_status),
        });
    }

    // High-risk customers should have Enhanced DD
    if cdd.risk_rating >= RiskRating::High && cdd.cdd_level != CddLevel::Enhanced {
        return Err(AmlCtfError::EddNotPerformed {
            customer_name: cdd.customer_name.clone(),
            reason: format!("High-risk customer (risk rating: {:?})", cdd.risk_rating),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_validate_cdd_success() {
        let cdd = CustomerDueDiligence {
            customer_name: "John Smith".to_string(),
            customer_type: CustomerType::Individual,
            cdd_level: CddLevel::Standard,
            assessment_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            identity_verified: true,
            identity_documents: vec![IdentityDocument {
                document_type: "Passport".to_string(),
                document_number: "AB123456".to_string(),
                issuing_country: "GBR".to_string(),
                expiry_date: Some(NaiveDate::from_ymd_opt(2030, 1, 1).unwrap()),
                verified: true,
            }],
            beneficial_owners: vec![],
            ownership_structure_verified: true,
            purpose_of_relationship: "Investment account".to_string(),
            nature_of_business: "Salaried employee".to_string(),
            source_of_funds: None,
            source_of_wealth: None,
            risk_rating: RiskRating::Low,
            pep_status: PepStatus::NonPep,
            sanctions_screening_passed: true,
            ongoing_monitoring_frequency: MonitoringFrequency::Annual,
            last_review_date: None,
        };

        assert!(validate_cdd(&cdd).is_ok());
    }

    #[test]
    fn test_validate_cdd_identity_not_verified() {
        let cdd = CustomerDueDiligence {
            customer_name: "John Smith".to_string(),
            customer_type: CustomerType::Individual,
            cdd_level: CddLevel::Standard,
            assessment_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            identity_verified: false, // NOT VERIFIED
            identity_documents: vec![],
            beneficial_owners: vec![],
            ownership_structure_verified: true,
            purpose_of_relationship: "Investment".to_string(),
            nature_of_business: "Employee".to_string(),
            source_of_funds: None,
            source_of_wealth: None,
            risk_rating: RiskRating::Low,
            pep_status: PepStatus::NonPep,
            sanctions_screening_passed: true,
            ongoing_monitoring_frequency: MonitoringFrequency::Annual,
            last_review_date: None,
        };

        let result = validate_cdd(&cdd);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(AmlCtfError::IdentityNotVerified { .. })
        ));
    }

    #[test]
    fn test_validate_cdd_pep_requires_edd() {
        let cdd = CustomerDueDiligence {
            customer_name: "Foreign Minister".to_string(),
            customer_type: CustomerType::Individual,
            cdd_level: CddLevel::Standard, // Should be Enhanced for PEP
            assessment_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            identity_verified: true,
            identity_documents: vec![IdentityDocument {
                document_type: "Passport".to_string(),
                document_number: "XY789012".to_string(),
                issuing_country: "FRA".to_string(),
                expiry_date: Some(NaiveDate::from_ymd_opt(2030, 1, 1).unwrap()),
                verified: true,
            }],
            beneficial_owners: vec![],
            ownership_structure_verified: true,
            purpose_of_relationship: "Investment".to_string(),
            nature_of_business: "Government official".to_string(),
            source_of_funds: Some("Salary".to_string()),
            source_of_wealth: Some("Government salary and savings".to_string()),
            risk_rating: RiskRating::High,
            pep_status: PepStatus::ForeignPep {
                country: "France".to_string(),
                position: "Minister of Finance".to_string(),
            },
            sanctions_screening_passed: true,
            ongoing_monitoring_frequency: MonitoringFrequency::Quarterly,
            last_review_date: None,
        };

        let result = validate_cdd(&cdd);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(AmlCtfError::EddNotPerformedForPep { .. })
        ));
    }

    #[test]
    fn test_validate_sar_success() {
        let sar = SuspiciousActivityReport {
            report_id: "SAR-2024-001".to_string(),
            report_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            subject_name: "Suspicious Person".to_string(),
            subject_id: Some("CUST-123".to_string()),
            suspicion_type: SuspicionType::MoneyLaundering,
            grounds_for_suspicion: "Customer deposited large amounts of cash in small denominations over several days, inconsistent with stated business activity and income level. Pattern suggests structuring to avoid reporting thresholds.".to_string(),
            transaction_amount_gbp: Some(50_000.0),
            transaction_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
            transaction_description: Some("Cash deposits".to_string()),
            reported_to_nca: true,
            nca_reference: Some("NCA-2024-12345".to_string()),
            nca_consent_obtained: None,
        };

        assert!(validate_sar(&sar).is_ok());
    }

    #[test]
    fn test_validate_sar_not_reported() {
        let sar = SuspiciousActivityReport {
            report_id: "SAR-2024-002".to_string(),
            report_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            subject_name: "Suspicious Person".to_string(),
            subject_id: None,
            suspicion_type: SuspicionType::MoneyLaundering,
            grounds_for_suspicion:
                "Suspicious transaction pattern detected with multiple high-value transfers"
                    .to_string(),
            transaction_amount_gbp: None,
            transaction_date: None,
            transaction_description: None,
            reported_to_nca: false, // NOT REPORTED
            nca_reference: None,
            nca_consent_obtained: None,
        };

        let result = validate_sar(&sar);
        assert!(result.is_err());
        assert!(matches!(result, Err(AmlCtfError::SarNotFiled { .. })));
    }

    #[test]
    fn test_validate_travel_rule_below_threshold() {
        let transfer = TravelRuleTransfer {
            transaction_id: "TX-001".to_string(),
            transaction_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            amount_gbp: 999.99, // Below £1000 threshold
            originator_name: String::new(),
            originator_wallet_address: String::new(),
            originator_account_number: None,
            beneficiary_name: String::new(),
            beneficiary_wallet_address: String::new(),
            beneficiary_account_number: None,
            information_transmitted: false,
            transmission_method: None,
        };

        // Should be OK because below threshold
        assert!(validate_travel_rule(&transfer).is_ok());
    }

    #[test]
    fn test_validate_travel_rule_above_threshold_compliant() {
        let transfer = TravelRuleTransfer {
            transaction_id: "TX-002".to_string(),
            transaction_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            amount_gbp: 5000.0, // Above threshold
            originator_name: "Alice Smith".to_string(),
            originator_wallet_address: "0x1234abcd...".to_string(),
            originator_account_number: Some("ACC-001".to_string()),
            beneficiary_name: "Bob Jones".to_string(),
            beneficiary_wallet_address: "0x5678efgh...".to_string(),
            beneficiary_account_number: Some("ACC-002".to_string()),
            information_transmitted: true,
            transmission_method: Some("SWIFT MT103".to_string()),
        };

        assert!(validate_travel_rule(&transfer).is_ok());
    }

    #[test]
    fn test_validate_travel_rule_above_threshold_non_compliant() {
        let transfer = TravelRuleTransfer {
            transaction_id: "TX-003".to_string(),
            transaction_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            amount_gbp: 5000.0,             // Above threshold
            originator_name: String::new(), // MISSING
            originator_wallet_address: "0x1234...".to_string(),
            originator_account_number: None,
            beneficiary_name: "Bob Jones".to_string(),
            beneficiary_wallet_address: "0x5678...".to_string(),
            beneficiary_account_number: None,
            information_transmitted: true,
            transmission_method: None,
        };

        let result = validate_travel_rule(&transfer);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(AmlCtfError::IncompleteOriginatorInfo { .. })
        ));
    }
}
