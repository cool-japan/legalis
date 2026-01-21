//! AFS Licensing Validators

use super::error::{AfsLicensingError, Result};
use super::types::*;

/// Validate AFSL authorization for a service
///
/// Checks:
/// - License is current
/// - Service is authorized under license
pub fn validate_afsl_authorization(
    license: &AfslLicense,
    service: &AuthorizedService,
) -> Result<()> {
    // Check license status
    if !license.status.can_conduct_activities() {
        return Err(AfsLicensingError::LicenseNotCurrent {
            license_number: license.license_number.clone(),
            status: format!("{:?}", license.status),
        });
    }

    // Check service is authorized
    if !license.is_service_authorized(service) {
        let authorized = license
            .authorized_services
            .iter()
            .map(|s| format!("{:?}", s))
            .collect::<Vec<_>>()
            .join(", ");

        return Err(AfsLicensingError::ServiceNotAuthorized {
            service: format!("{:?}", service),
            license_number: license.license_number.clone(),
            authorized,
        });
    }

    Ok(())
}

/// Validate license conditions
///
/// Checks all conditions are complied with
pub fn validate_license_conditions(license: &AfslLicense) -> Result<()> {
    for condition in &license.conditions {
        if !condition.compliant {
            return Err(AfsLicensingError::ConditionBreach {
                condition_id: condition.condition_id.clone(),
                description: condition.description.clone(),
            });
        }
    }
    Ok(())
}

/// Validate responsible manager
///
/// Checks:
/// - License has at least one responsible manager
/// - Responsible managers meet fit and proper requirements
pub fn validate_responsible_manager(license: &AfslLicense) -> Result<()> {
    if license.responsible_managers.is_empty() {
        return Err(AfsLicensingError::NoResponsibleManager {
            license_number: license.license_number.clone(),
        });
    }

    for manager in &license.responsible_managers {
        if !manager.fit_and_proper {
            return Err(AfsLicensingError::ResponsibleManagerNotFitProper {
                name: manager.name.clone(),
                reason: "Does not meet fit and proper requirements".to_string(),
            });
        }

        // Check minimum experience (generally 3 years for RG 105)
        if manager.experience_years < 3 {
            return Err(AfsLicensingError::ResponsibleManagerNotFitProper {
                name: manager.name.clone(),
                reason: format!(
                    "Insufficient experience: {} years (minimum 3 required)",
                    manager.experience_years
                ),
            });
        }
    }

    Ok(())
}

/// Validate authorised representative
///
/// Checks:
/// - AR is active
/// - AR meets training requirements
/// - AR is authorized for the service
pub fn validate_authorized_representative(
    ar: &AuthorizedRepresentative,
    service: &AuthorizedService,
) -> Result<()> {
    // Check AR status
    if ar.status != ArStatus::Active {
        return Err(AfsLicensingError::ArNotAuthorized {
            ar_name: ar.name.clone(),
            ar_number: ar.ar_number.clone(),
        });
    }

    // Check training compliance
    if !ar.training_completed {
        return Err(AfsLicensingError::ArNotCompliant {
            ar_name: ar.name.clone(),
            reason: "Training not completed".to_string(),
        });
    }

    if !ar.rg146_compliant {
        return Err(AfsLicensingError::ArNotCompliant {
            ar_name: ar.name.clone(),
            reason: "Does not meet RG 146 training requirements".to_string(),
        });
    }

    // Check service authorization
    if !ar.can_provide_service(service) {
        return Err(AfsLicensingError::ArNotAuthorized {
            ar_name: ar.name.clone(),
            ar_number: ar.ar_number.clone(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn create_test_license() -> AfslLicense {
        AfslLicense {
            license_number: "123456".to_string(),
            licensee_name: "Test Pty Ltd".to_string(),
            abn: "12345678901".to_string(),
            acn: None,
            status: LicenseStatus::Current,
            issue_date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            variation_date: None,
            authorized_services: vec![
                AuthorizedService::ProvideFinancialProductAdvice {
                    product_type: ProductType::Securities,
                    client_type: ClientType::Retail,
                },
                AuthorizedService::DealInFinancialProduct {
                    product_type: ProductType::Securities,
                    deal_type: DealType::OnBehalfOfAnother,
                    client_type: ClientType::Retail,
                },
            ],
            conditions: vec![AfslCondition {
                condition_id: "COND-001".to_string(),
                condition_type: ConditionType::Standard,
                description: "Must maintain PI insurance".to_string(),
                imposed_date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
                compliant: true,
                last_check_date: None,
            }],
            responsible_managers: vec![ResponsibleManager {
                name: "John Smith".to_string(),
                position: "Director".to_string(),
                start_date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
                qualifications: vec!["CFA".to_string()],
                experience_years: 10,
                responsibility_areas: vec!["Advice".to_string()],
                fit_and_proper: true,
            }],
            authorised_rep_count: Some(5),
        }
    }

    #[test]
    fn test_validate_afsl_authorization_success() {
        let license = create_test_license();
        let service = AuthorizedService::ProvideFinancialProductAdvice {
            product_type: ProductType::Securities,
            client_type: ClientType::Retail,
        };

        assert!(validate_afsl_authorization(&license, &service).is_ok());
    }

    #[test]
    fn test_validate_afsl_authorization_not_authorized() {
        let license = create_test_license();
        let service = AuthorizedService::OperateRegisteredScheme;

        let result = validate_afsl_authorization(&license, &service);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(AfsLicensingError::ServiceNotAuthorized { .. })
        ));
    }

    #[test]
    fn test_validate_afsl_authorization_suspended() {
        let mut license = create_test_license();
        license.status = LicenseStatus::Suspended;

        let service = AuthorizedService::ProvideFinancialProductAdvice {
            product_type: ProductType::Securities,
            client_type: ClientType::Retail,
        };

        let result = validate_afsl_authorization(&license, &service);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(AfsLicensingError::LicenseNotCurrent { .. })
        ));
    }

    #[test]
    fn test_validate_responsible_manager() {
        let license = create_test_license();
        assert!(validate_responsible_manager(&license).is_ok());
    }

    #[test]
    fn test_validate_responsible_manager_no_manager() {
        let mut license = create_test_license();
        license.responsible_managers.clear();

        let result = validate_responsible_manager(&license);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(AfsLicensingError::NoResponsibleManager { .. })
        ));
    }

    #[test]
    fn test_validate_authorized_representative() {
        let ar = AuthorizedRepresentative {
            ar_number: "AR-001".to_string(),
            name: "Jane Doe".to_string(),
            is_corporate: false,
            abn: None,
            principal_afsl: "123456".to_string(),
            authorized_services: vec![AuthorizedService::ProvideFinancialProductAdvice {
                product_type: ProductType::Securities,
                client_type: ClientType::Retail,
            }],
            authorization_date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            status: ArStatus::Active,
            training_completed: true,
            rg146_compliant: true,
        };

        let service = AuthorizedService::ProvideFinancialProductAdvice {
            product_type: ProductType::Securities,
            client_type: ClientType::Retail,
        };

        assert!(validate_authorized_representative(&ar, &service).is_ok());
    }

    #[test]
    fn test_validate_authorized_representative_not_trained() {
        let ar = AuthorizedRepresentative {
            ar_number: "AR-001".to_string(),
            name: "Jane Doe".to_string(),
            is_corporate: false,
            abn: None,
            principal_afsl: "123456".to_string(),
            authorized_services: vec![AuthorizedService::ProvideFinancialProductAdvice {
                product_type: ProductType::Securities,
                client_type: ClientType::Retail,
            }],
            authorization_date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            status: ArStatus::Active,
            training_completed: false, // Not trained
            rg146_compliant: false,
        };

        let service = AuthorizedService::ProvideFinancialProductAdvice {
            product_type: ProductType::Securities,
            client_type: ClientType::Retail,
        };

        let result = validate_authorized_representative(&ar, &service);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(AfsLicensingError::ArNotCompliant { .. })
        ));
    }
}
