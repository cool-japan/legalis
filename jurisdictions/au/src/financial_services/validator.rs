//! Financial Services Validators (Corporations Act 2001 Chapter 7)

use super::error::{FinancialServicesError, Result};
use super::types::*;
use chrono::Utc;

/// Validate client classification
///
/// Checks:
/// - Classification basis meets requirements
/// - Required documentation present
/// - Classification not expired
pub fn validate_client_classification(classification: &ClientClassification) -> Result<()> {
    // Check if classification is expired
    if let Some(expiry) = classification.expiry_date {
        let today = Utc::now().date_naive();
        if expiry < today {
            return Err(FinancialServicesError::ClassificationExpired {
                client_name: classification.client_name.clone(),
                expiry_date: expiry.to_string(),
            });
        }
    }

    // Validate basis for wholesale classification
    if classification.classification == ClientClass::Wholesale {
        validate_wholesale_basis(classification)?;
    }

    Ok(())
}

/// Validate wholesale classification basis
fn validate_wholesale_basis(classification: &ClientClassification) -> Result<()> {
    match &classification.basis {
        ClassificationBasis::ProductValueTest { consideration_aud } => {
            if *consideration_aud < 500_000.0 {
                return Err(FinancialServicesError::InvalidWholesaleClassification {
                    client_name: classification.client_name.clone(),
                    basis: "Product value test".to_string(),
                    reason: format!(
                        "Consideration ${:.0} is below $500,000 threshold",
                        consideration_aud
                    ),
                });
            }
        }
        ClassificationBasis::AssetsTest {
            net_assets_aud,
            accountant_certificate,
        } => {
            if *net_assets_aud < 2_500_000.0 {
                return Err(FinancialServicesError::InvalidWholesaleClassification {
                    client_name: classification.client_name.clone(),
                    basis: "Assets test".to_string(),
                    reason: format!(
                        "Net assets ${:.0} is below $2,500,000 threshold",
                        net_assets_aud
                    ),
                });
            }
            if !accountant_certificate {
                return Err(FinancialServicesError::MissingAccountantCertificate {
                    client_name: classification.client_name.clone(),
                    test_type: "assets test".to_string(),
                });
            }
        }
        ClassificationBasis::IncomeTest {
            income_year_1_aud,
            income_year_2_aud,
            accountant_certificate,
        } => {
            if *income_year_1_aud < 250_000.0 || *income_year_2_aud < 250_000.0 {
                return Err(FinancialServicesError::InvalidWholesaleClassification {
                    client_name: classification.client_name.clone(),
                    basis: "Income test".to_string(),
                    reason: format!(
                        "Income must be $250,000+ for each of last 2 years. Year 1: ${:.0}, Year 2: ${:.0}",
                        income_year_1_aud, income_year_2_aud
                    ),
                });
            }
            if !accountant_certificate {
                return Err(FinancialServicesError::MissingAccountantCertificate {
                    client_name: classification.client_name.clone(),
                    test_type: "income test".to_string(),
                });
            }
        }
        ClassificationBasis::RegulatedSuperFund { net_assets_aud, .. } => {
            if *net_assets_aud < 10_000_000.0 {
                return Err(FinancialServicesError::InvalidWholesaleClassification {
                    client_name: classification.client_name.clone(),
                    basis: "Regulated superannuation fund".to_string(),
                    reason: format!(
                        "Fund net assets ${:.0} is below $10,000,000 threshold",
                        net_assets_aud
                    ),
                });
            }
        }
        ClassificationBasis::RetailDefault => {
            return Err(FinancialServicesError::InvalidWholesaleClassification {
                client_name: classification.client_name.clone(),
                basis: "Retail default".to_string(),
                reason: "Cannot classify as wholesale with retail default basis".to_string(),
            });
        }
        // Other bases are valid if documents support
        _ => {}
    }

    Ok(())
}

/// Validate general obligations compliance (s.912A)
///
/// Checks all general obligations are met
pub fn validate_general_obligations(compliance: &GeneralObligationsCompliance) -> Result<()> {
    let mut errors = Vec::new();

    if !compliance.efficient_honest_fair.compliant {
        errors.push(FinancialServicesError::BreachEfficientHonestFair {
            details: compliance
                .efficient_honest_fair
                .breach_details
                .clone()
                .unwrap_or_else(|| "No details provided".to_string()),
        });
    }

    if !compliance.conflicts_management.compliant {
        errors.push(FinancialServicesError::InadequateConflictsManagement {
            details: compliance
                .conflicts_management
                .breach_details
                .clone()
                .unwrap_or_else(|| "No details provided".to_string()),
        });
    }

    if !compliance.legal_compliance.compliant {
        errors.push(FinancialServicesError::BreachFinancialServicesLaws {
            details: compliance
                .legal_compliance
                .breach_details
                .clone()
                .unwrap_or_else(|| "No details provided".to_string()),
        });
    }

    if !compliance.risk_management.compliant {
        errors.push(FinancialServicesError::InadequateRiskManagement {
            details: compliance
                .risk_management
                .breach_details
                .clone()
                .unwrap_or_else(|| "No details provided".to_string()),
        });
    }

    if !compliance.competence.compliant {
        errors.push(FinancialServicesError::InadequateCompetence {
            details: compliance
                .competence
                .breach_details
                .clone()
                .unwrap_or_else(|| "No details provided".to_string()),
        });
    }

    if !compliance.representative_training.compliant {
        errors.push(FinancialServicesError::InadequateTraining {
            details: compliance
                .representative_training
                .breach_details
                .clone()
                .unwrap_or_else(|| "No details provided".to_string()),
        });
    }

    if !compliance.adequate_resources.compliant {
        errors.push(FinancialServicesError::InadequateResources {
            details: compliance
                .adequate_resources
                .breach_details
                .clone()
                .unwrap_or_else(|| "No details provided".to_string()),
        });
    }

    if !compliance.dispute_resolution.compliant {
        errors.push(FinancialServicesError::IdrNonCompliant {
            deficiency: compliance
                .dispute_resolution
                .breach_details
                .clone()
                .unwrap_or_else(|| "IDR system not compliant".to_string()),
        });
    }

    if !compliance.compensation_arrangements.compliant {
        errors.push(FinancialServicesError::InadequateCompensationArrangements {
            reason: compliance
                .compensation_arrangements
                .breach_details
                .clone()
                .unwrap_or_else(|| "No details provided".to_string()),
        });
    }

    if !errors.is_empty() {
        // Return first error (or could return MultipleErrors)
        return Err(errors.into_iter().next().expect("errors is non-empty"));
    }

    Ok(())
}

/// Validate resource requirements (s.912A(1)(d))
///
/// Checks:
/// - Financial resources meet NTA requirements
/// - Human resources adequate
/// - Technological resources adequate
pub fn validate_resources(resources: &ResourceRequirement) -> Result<()> {
    // Check financial resources
    if resources.financial.net_tangible_assets_aud < resources.financial.required_nta_aud {
        return Err(FinancialServicesError::InadequateResources {
            details: format!(
                "Net tangible assets ${:.0} is below required NTA ${:.0}",
                resources.financial.net_tangible_assets_aud, resources.financial.required_nta_aud
            ),
        });
    }

    if !resources.financial.meets_requirements {
        return Err(FinancialServicesError::InadequateResources {
            details: "Financial resources do not meet ASIC requirements".to_string(),
        });
    }

    // Check human resources
    if resources.human.responsible_managers == 0 {
        return Err(FinancialServicesError::InadequateResources {
            details: "No responsible managers nominated".to_string(),
        });
    }

    if !resources.human.competency_standards_met {
        return Err(FinancialServicesError::InadequateTraining {
            details: "Staff do not meet competency standards".to_string(),
        });
    }

    // Check technological resources
    if !resources.technological.record_keeping_systems {
        return Err(FinancialServicesError::InadequateResources {
            details: "No adequate record keeping systems".to_string(),
        });
    }

    if !resources.technological.security_controls {
        return Err(FinancialServicesError::InadequateResources {
            details: "Inadequate security controls".to_string(),
        });
    }

    Ok(())
}

/// Validate dispute resolution (s.912A(1)(g))
///
/// Checks:
/// - IDR system complies with RG 271
/// - EDR membership (AFCA) if providing retail services
pub fn validate_dispute_resolution(
    dispute_resolution: &DisputeResolution,
    provides_retail_services: bool,
) -> Result<()> {
    // Check IDR system
    if !dispute_resolution.idr_system.rg271_compliant {
        return Err(FinancialServicesError::IdrNonCompliant {
            deficiency: "IDR system does not comply with ASIC RG 271".to_string(),
        });
    }

    if !dispute_resolution.idr_system.written_procedures {
        return Err(FinancialServicesError::IdrNonCompliant {
            deficiency: "No written IDR procedures".to_string(),
        });
    }

    // Check EDR membership for retail services
    if provides_retail_services {
        match &dispute_resolution.edr_membership {
            None => {
                return Err(FinancialServicesError::NoEdrMembership);
            }
            Some(edr) if edr.status != EdrStatus::Active => {
                return Err(FinancialServicesError::NoEdrMembership);
            }
            _ => {}
        }
    }

    Ok(())
}

/// Validate compensation arrangements (s.912B)
///
/// Checks:
/// - Adequate compensation arrangements in place
/// - PI insurance coverage adequate and current
pub fn validate_compensation_arrangements(
    arrangement: &CompensationArrangement,
    required_coverage_aud: f64,
) -> Result<()> {
    if !arrangement.adequate {
        return Err(FinancialServicesError::InadequateCompensationArrangements {
            reason: "Compensation arrangements not adequate".to_string(),
        });
    }

    if let Some(ref pi) = arrangement.pi_insurance {
        // Check coverage amount
        if pi.coverage_aud < required_coverage_aud {
            return Err(FinancialServicesError::InsufficientPiInsurance {
                coverage_aud: pi.coverage_aud,
                required_aud: required_coverage_aud,
            });
        }

        // Check expiry
        let today = Utc::now().date_naive();
        if pi.end_date < today {
            return Err(FinancialServicesError::PiInsuranceExpired {
                expiry_date: pi.end_date.to_string(),
            });
        }

        // Check covers all services
        if !pi.covers_all_services {
            return Err(FinancialServicesError::InadequateCompensationArrangements {
                reason: "PI insurance does not cover all authorized services".to_string(),
            });
        }
    } else if arrangement.alternative_arrangements.is_none() {
        return Err(FinancialServicesError::InadequateCompensationArrangements {
            reason: "No PI insurance and no alternative arrangements".to_string(),
        });
    }

    Ok(())
}

/// Validate breach notification compliance (s.912D)
///
/// Checks:
/// - Significant breaches reported within 30 days
/// - Appropriate remediation
pub fn validate_breach_notification(breach: &BreachNotification) -> Result<()> {
    if breach.significance == BreachSignificance::Significant {
        // Check if reported to ASIC
        if !breach.reported_to_asic {
            let days_since = (Utc::now() - breach.identification_date)
                .num_days()
                .try_into()
                .unwrap_or(0);
            if days_since > 30 {
                return Err(FinancialServicesError::SignificantBreachNotReported {
                    days_ago: days_since,
                });
            }
        } else if let Some(notification_date) = breach.asic_notification_date {
            let days_to_notify = (notification_date - breach.identification_date).num_days();
            if days_to_notify > 30 {
                return Err(FinancialServicesError::LateBreachNotification {
                    identification_date: breach.identification_date.to_string(),
                    notification_date: notification_date.to_string(),
                });
            }
        }
    }

    Ok(())
}

/// Validate market integrity compliance
pub fn validate_market_integrity(compliance: &MarketIntegrityCompliance) -> Result<()> {
    if !compliance.mir_compliant {
        return Err(FinancialServicesError::BreachFinancialServicesLaws {
            details: "Does not comply with ASIC Market Integrity Rules".to_string(),
        });
    }

    if !compliance.best_execution_policy {
        return Err(FinancialServicesError::ValidationError {
            message: "No best execution policy".to_string(),
        });
    }

    if !compliance.manipulation_prevention {
        return Err(FinancialServicesError::ValidationError {
            message: "Inadequate market manipulation prevention measures".to_string(),
        });
    }

    if !compliance.insider_trading_prevention {
        return Err(FinancialServicesError::ValidationError {
            message: "Inadequate insider trading prevention measures".to_string(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_validate_wholesale_classification_assets_test() {
        let valid = ClientClassification {
            client_name: "Wealthy Client".to_string(),
            classification: ClientClass::Wholesale,
            basis: ClassificationBasis::AssetsTest {
                net_assets_aud: 3_000_000.0,
                accountant_certificate: true,
            },
            classification_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            expiry_date: Some(NaiveDate::from_ymd_opt(2027, 1, 1).unwrap()),
            documentation: vec!["Accountant certificate".to_string()],
        };

        assert!(validate_client_classification(&valid).is_ok());
    }

    #[test]
    fn test_validate_wholesale_classification_assets_insufficient() {
        let invalid = ClientClassification {
            client_name: "Not Wealthy Enough".to_string(),
            classification: ClientClass::Wholesale,
            basis: ClassificationBasis::AssetsTest {
                net_assets_aud: 2_000_000.0, // Below $2.5M
                accountant_certificate: true,
            },
            classification_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            expiry_date: None,
            documentation: vec![],
        };

        let result = validate_client_classification(&invalid);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(FinancialServicesError::InvalidWholesaleClassification { .. })
        ));
    }

    #[test]
    fn test_validate_wholesale_classification_missing_certificate() {
        let invalid = ClientClassification {
            client_name: "No Certificate".to_string(),
            classification: ClientClass::Wholesale,
            basis: ClassificationBasis::IncomeTest {
                income_year_1_aud: 300_000.0,
                income_year_2_aud: 300_000.0,
                accountant_certificate: false, // Missing certificate
            },
            classification_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            expiry_date: None,
            documentation: vec![],
        };

        let result = validate_client_classification(&invalid);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(FinancialServicesError::MissingAccountantCertificate { .. })
        ));
    }

    #[test]
    fn test_validate_resources_insufficient_nta() {
        let resources = ResourceRequirement {
            financial: FinancialResources {
                net_tangible_assets_aud: 40_000.0,
                required_nta_aud: 50_000.0, // Required is higher
                cash_reserves_aud: 10_000.0,
                pi_insurance_aud: Some(1_000_000.0),
                surplus_assets_aud: 0.0,
                meets_requirements: false,
            },
            human: HumanResources {
                responsible_managers: 1,
                authorised_representatives: 5,
                compliance_staff: 1,
                training_program: true,
                competency_standards_met: true,
            },
            technological: TechnologicalResources {
                record_keeping_systems: true,
                transaction_monitoring: true,
                security_controls: true,
                disaster_recovery: true,
                privacy_compliance: true,
            },
        };

        let result = validate_resources(&resources);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(FinancialServicesError::InadequateResources { .. })
        ));
    }

    #[test]
    fn test_validate_dispute_resolution_no_edr() {
        let dispute_resolution = DisputeResolution {
            idr_system: InternalDisputeResolution {
                rg271_compliant: true,
                response_time_days: 5,
                resolution_time_days: 30,
                complaints_officer: true,
                written_procedures: true,
            },
            edr_membership: None, // No AFCA membership
        };

        // Should fail for retail services
        let result = validate_dispute_resolution(&dispute_resolution, true);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(FinancialServicesError::NoEdrMembership)
        ));

        // Should pass for wholesale-only services
        let result = validate_dispute_resolution(&dispute_resolution, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_compensation_arrangements() {
        let arrangement = CompensationArrangement {
            adequate: true,
            pi_insurance: Some(ProfessionalIndemnityInsurance {
                insurer: "Test Insurer".to_string(),
                policy_number: "PI-001".to_string(),
                coverage_aud: 2_000_000.0,
                excess_aud: 10_000.0,
                start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                end_date: NaiveDate::from_ymd_opt(2027, 1, 1).unwrap(),
                covers_all_services: true,
                run_off_cover: true,
            }),
            alternative_arrangements: None,
            last_review_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        };

        let result = validate_compensation_arrangements(&arrangement, 1_000_000.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_general_obligations_breach() {
        let compliance = GeneralObligationsCompliance {
            efficient_honest_fair: ObligationCompliance {
                compliant: false,
                evidence: String::new(),
                breach_details: Some("Misleading conduct".to_string()),
                last_review_date: None,
            },
            conflicts_management: ObligationCompliance {
                compliant: true,
                evidence: "Documented policy".to_string(),
                breach_details: None,
                last_review_date: None,
            },
            legal_compliance: ObligationCompliance {
                compliant: true,
                evidence: "Compliance program".to_string(),
                breach_details: None,
                last_review_date: None,
            },
            representative_compliance: ObligationCompliance {
                compliant: true,
                evidence: "Supervision framework".to_string(),
                breach_details: None,
                last_review_date: None,
            },
            risk_management: ObligationCompliance {
                compliant: true,
                evidence: "Risk framework".to_string(),
                breach_details: None,
                last_review_date: None,
            },
            competence: ObligationCompliance {
                compliant: true,
                evidence: "Qualifications".to_string(),
                breach_details: None,
                last_review_date: None,
            },
            representative_training: ObligationCompliance {
                compliant: true,
                evidence: "Training records".to_string(),
                breach_details: None,
                last_review_date: None,
            },
            adequate_resources: ObligationCompliance {
                compliant: true,
                evidence: "Resource assessment".to_string(),
                breach_details: None,
                last_review_date: None,
            },
            dispute_resolution: ObligationCompliance {
                compliant: true,
                evidence: "IDR/EDR membership".to_string(),
                breach_details: None,
                last_review_date: None,
            },
            compensation_arrangements: ObligationCompliance {
                compliant: true,
                evidence: "PI insurance".to_string(),
                breach_details: None,
                last_review_date: None,
            },
        };

        let result = validate_general_obligations(&compliance);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(FinancialServicesError::BreachEfficientHonestFair { .. })
        ));
    }
}
