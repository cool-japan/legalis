//! AML/CTF Validators (AUSTRAC)

use super::error::{AmlCtfError, Result};
use super::types::*;
use chrono::Utc;

/// Validate customer identification
///
/// Checks:
/// - Identity verified
/// - Sufficient documentation
/// - Beneficial owners identified (for entities)
/// - PEP screening completed
/// - EDD applied if required
pub fn validate_customer_identification(cdd: &AuCustomerDueDiligence) -> Result<()> {
    // Check identity verified
    if !cdd.identity_verified {
        return Err(AmlCtfError::IdentityNotVerified {
            customer_name: cdd.customer_name.clone(),
        });
    }

    // Check sufficient documents
    validate_documentation(cdd)?;

    // Check beneficial owners for entities
    if matches!(
        cdd.customer_type,
        CustomerType::Company | CustomerType::Trust | CustomerType::Partnership
    ) && cdd.beneficial_owners.is_empty()
    {
        return Err(AmlCtfError::BeneficialOwnerNotIdentified {
            entity_name: cdd.customer_name.clone(),
        });
    }

    // Check sanctions screening
    if !cdd.sanctions_screening_passed {
        return Err(AmlCtfError::SanctionsMatch {
            customer_name: cdd.customer_name.clone(),
            list_name: "Sanctions list".to_string(),
        });
    }

    // Check EDD requirements
    if cdd.pep_status.requires_enhanced_dd() || cdd.risk_rating == RiskRating::High {
        validate_enhanced_dd(cdd)?;
    }

    // Check prohibited risk rating
    if cdd.risk_rating == RiskRating::Prohibited {
        return Err(AmlCtfError::ProhibitedCustomer {
            customer_name: cdd.customer_name.clone(),
            reason: "Risk assessment resulted in prohibited rating".to_string(),
        });
    }

    Ok(())
}

/// Validate documentation
fn validate_documentation(cdd: &AuCustomerDueDiligence) -> Result<()> {
    if cdd.documents.is_empty() {
        return Err(AmlCtfError::InsufficientDocuments {
            customer_name: cdd.customer_name.clone(),
            documents_provided: "None".to_string(),
            documents_required: "At least 1 primary document or 2 secondary documents".to_string(),
        });
    }

    // Check if documents are verified
    let verified_docs: Vec<_> = cdd.documents.iter().filter(|d| d.verified).collect();
    if verified_docs.is_empty() {
        return Err(AmlCtfError::IdentityNotVerified {
            customer_name: cdd.customer_name.clone(),
        });
    }

    // Check document categories (need 1 primary OR 2 secondary)
    let primary_count = verified_docs
        .iter()
        .filter(|d| d.document_type.is_primary())
        .count();
    let secondary_count = verified_docs
        .iter()
        .filter(|d| d.document_type.is_secondary())
        .count();

    if primary_count == 0 && secondary_count < 2 {
        return Err(AmlCtfError::InsufficientDocuments {
            customer_name: cdd.customer_name.clone(),
            documents_provided: format!("{} primary, {} secondary", primary_count, secondary_count),
            documents_required: "1 primary OR 2 secondary documents".to_string(),
        });
    }

    Ok(())
}

/// Validate enhanced due diligence
fn validate_enhanced_dd(cdd: &AuCustomerDueDiligence) -> Result<()> {
    // Check CDD level is Enhanced
    if cdd.cdd_level != CddLevel::Enhanced {
        let reason = if cdd.pep_status.is_pep() {
            "Customer is a PEP"
        } else {
            "High risk customer"
        };
        return Err(AmlCtfError::EddRequired {
            customer_name: cdd.customer_name.clone(),
            reason: reason.to_string(),
        });
    }

    // Check source of funds
    if cdd.source_of_funds.is_none() {
        return Err(AmlCtfError::EddIncomplete {
            customer_name: cdd.customer_name.clone(),
            missing: "Source of funds".to_string(),
        });
    }

    // Check source of wealth for PEPs
    if cdd.pep_status.is_pep() && cdd.source_of_wealth.is_none() {
        return Err(AmlCtfError::EddIncomplete {
            customer_name: cdd.customer_name.clone(),
            missing: "Source of wealth".to_string(),
        });
    }

    Ok(())
}

/// Validate AUSTRAC compliance
///
/// Checks:
/// - AML/CTF program in place
/// - MLRO appointed
/// - Registered with AUSTRAC
/// - Employee training
pub fn validate_austrac_compliance(compliance: &AustracCompliance) -> Result<()> {
    // Check program exists
    if !compliance.has_program {
        return Err(AmlCtfError::NoAmlCtfProgram);
    }

    // Check MLRO appointed
    if !compliance.has_mlro {
        return Err(AmlCtfError::NoMlro);
    }

    // Check MLRO details
    if let Some(ref mlro) = compliance.mlro
        && !mlro.training_completed
    {
        return Err(AmlCtfError::ProgramDeficient {
            deficiency: "MLRO has not completed required training".to_string(),
        });
    }

    // Check AUSTRAC registration
    if !compliance.austrac_registered {
        return Err(AmlCtfError::NotRegistered);
    }

    // Check employee training
    if !compliance.employee_training {
        return Err(AmlCtfError::ProgramDeficient {
            deficiency: "Employee AML/CTF training not completed".to_string(),
        });
    }

    // Check training completion rate (should be >90%)
    if let Some(rate) = compliance.training_completion_rate
        && rate < 90.0
    {
        return Err(AmlCtfError::ProgramDeficient {
            deficiency: format!(
                "Training completion rate {:.1}% is below 90% threshold",
                rate
            ),
        });
    }

    Ok(())
}

/// Validate suspicious matter report
///
/// Checks:
/// - Report has required information
/// - Report submitted within timeframe
pub fn validate_smr(smr: &SuspiciousMatterReport) -> Result<()> {
    // Check report has grounds
    if smr.grounds.is_empty() {
        return Err(AmlCtfError::ValidationError {
            message: "SMR must include grounds for suspicion".to_string(),
        });
    }

    // Check submission status
    if !smr.submitted_to_austrac {
        let now = Utc::now();
        if now > smr.submission_deadline {
            return Err(AmlCtfError::SmrLate);
        }

        // Check days remaining
        let days = match smr.suspicion_type {
            SuspicionType::TerrorismFinancing => 1,
            _ => 3,
        };

        return Err(AmlCtfError::SmrNotSubmitted {
            subject: smr.subject_name.clone(),
            days,
        });
    }

    Ok(())
}

/// Validate threshold transaction report
///
/// Checks:
/// - Transaction meets threshold ($10,000+)
/// - Report submitted
pub fn validate_ttr(ttr: &ThresholdTransaction) -> Result<()> {
    // Check threshold met
    if ttr.amount_aud < 10_000.0 {
        return Ok(()); // Below threshold, no reporting required
    }

    // Check submission
    if !ttr.submitted_to_austrac {
        return Err(AmlCtfError::TtrNotSubmitted {
            amount: ttr.amount_aud,
        });
    }

    Ok(())
}

/// Validate international funds transfer instruction
///
/// Checks:
/// - Report submitted
pub fn validate_ifti(ifti: &InternationalFundsTransfer) -> Result<()> {
    if !ifti.submitted_to_austrac {
        return Err(AmlCtfError::IftiNotSubmitted);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn create_valid_cdd() -> AuCustomerDueDiligence {
        AuCustomerDueDiligence {
            customer_name: "John Smith".to_string(),
            customer_type: CustomerType::Individual,
            cdd_level: CddLevel::Standard,
            assessment_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            identity_verified: true,
            documents: vec![IdentityDocument {
                document_type: DocumentType::Passport,
                document_number: "PA1234567".to_string(),
                issuing_country: "AUS".to_string(),
                expiry_date: Some(NaiveDate::from_ymd_opt(2030, 1, 1).unwrap()),
                verified: true,
            }],
            beneficial_owners: vec![],
            pep_status: PepStatus::NonPep,
            risk_rating: RiskRating::Low,
            ongoing_monitoring: true,
            monitoring_frequency: Some(MonitoringFrequency::Annual),
            source_of_funds: None,
            source_of_wealth: None,
            purpose_of_relationship: Some("Investment".to_string()),
            sanctions_screening_passed: true,
        }
    }

    #[test]
    fn test_validate_customer_identification_success() {
        let cdd = create_valid_cdd();
        assert!(validate_customer_identification(&cdd).is_ok());
    }

    #[test]
    fn test_validate_customer_identification_not_verified() {
        let mut cdd = create_valid_cdd();
        cdd.identity_verified = false;

        let result = validate_customer_identification(&cdd);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(AmlCtfError::IdentityNotVerified { .. })
        ));
    }

    #[test]
    fn test_validate_customer_identification_no_documents() {
        let mut cdd = create_valid_cdd();
        cdd.documents.clear();

        let result = validate_customer_identification(&cdd);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(AmlCtfError::InsufficientDocuments { .. })
        ));
    }

    #[test]
    fn test_validate_customer_identification_pep_requires_edd() {
        let mut cdd = create_valid_cdd();
        cdd.pep_status = PepStatus::ForeignPep {
            country: "US".to_string(),
            position: "Senator".to_string(),
        };
        // CDD level not enhanced

        let result = validate_customer_identification(&cdd);
        assert!(result.is_err());
        assert!(matches!(result, Err(AmlCtfError::EddRequired { .. })));
    }

    #[test]
    fn test_validate_customer_identification_pep_with_edd() {
        let mut cdd = create_valid_cdd();
        cdd.pep_status = PepStatus::ForeignPep {
            country: "US".to_string(),
            position: "Senator".to_string(),
        };
        cdd.cdd_level = CddLevel::Enhanced;
        cdd.source_of_funds = Some("Salary".to_string());
        cdd.source_of_wealth = Some("Salary and investments".to_string());

        assert!(validate_customer_identification(&cdd).is_ok());
    }

    #[test]
    fn test_validate_austrac_compliance_success() {
        let compliance = AustracCompliance {
            has_program: true,
            program_review_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
            has_mlro: true,
            mlro: Some(MlroDetails {
                name: "Jane Smith".to_string(),
                position: "Compliance Manager".to_string(),
                appointment_date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
                training_completed: true,
                contact_on_file: true,
            }),
            employee_training: true,
            training_completion_rate: Some(95.0),
            austrac_registered: true,
            registration_number: Some("REG-001".to_string()),
            independent_review: true,
            last_review_date: Some(NaiveDate::from_ymd_opt(2023, 6, 1).unwrap()),
        };

        assert!(validate_austrac_compliance(&compliance).is_ok());
    }

    #[test]
    fn test_validate_austrac_compliance_no_program() {
        let compliance = AustracCompliance {
            has_program: false,
            program_review_date: None,
            has_mlro: false,
            mlro: None,
            employee_training: false,
            training_completion_rate: None,
            austrac_registered: false,
            registration_number: None,
            independent_review: false,
            last_review_date: None,
        };

        let result = validate_austrac_compliance(&compliance);
        assert!(result.is_err());
        assert!(matches!(result, Err(AmlCtfError::NoAmlCtfProgram)));
    }

    #[test]
    fn test_validate_ttr() {
        let ttr = ThresholdTransaction {
            transaction_id: "TTR-001".to_string(),
            transaction_date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            transaction_type: ThresholdTransactionType::CashDeposit,
            amount_aud: 15_000.0,
            customer_name: "John Smith".to_string(),
            customer_identifier: "CUST-001".to_string(),
            submitted_to_austrac: true,
            submission_deadline: NaiveDate::from_ymd_opt(2024, 1, 25).unwrap(),
        };

        assert!(validate_ttr(&ttr).is_ok());
    }

    #[test]
    fn test_validate_ttr_not_submitted() {
        let ttr = ThresholdTransaction {
            transaction_id: "TTR-002".to_string(),
            transaction_date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            transaction_type: ThresholdTransactionType::CashDeposit,
            amount_aud: 12_000.0,
            customer_name: "Jane Doe".to_string(),
            customer_identifier: "CUST-002".to_string(),
            submitted_to_austrac: false, // Not submitted
            submission_deadline: NaiveDate::from_ymd_opt(2024, 1, 25).unwrap(),
        };

        let result = validate_ttr(&ttr);
        assert!(result.is_err());
        assert!(matches!(result, Err(AmlCtfError::TtrNotSubmitted { .. })));
    }

    #[test]
    fn test_validate_beneficial_owner_required() {
        let mut cdd = create_valid_cdd();
        cdd.customer_type = CustomerType::Company;
        cdd.beneficial_owners.clear();

        let result = validate_customer_identification(&cdd);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(AmlCtfError::BeneficialOwnerNotIdentified { .. })
        ));
    }
}
