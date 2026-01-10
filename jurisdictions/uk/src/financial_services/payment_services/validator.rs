//! Payment Services Validators (Payment Services Regulations 2017 - PSD2)

use super::error::{PaymentServicesError, Result};
use super::types::*;
use chrono::NaiveDate;

/// Validate Strong Customer Authentication (PSR 2017 Reg 67-68)
///
/// Checks SCA compliance:
/// - Two or more independent authentication factors
/// - Factors from different categories (knowledge, possession, inherence)
/// - Authentication succeeded
pub fn validate_sca(sca: &StrongCustomerAuthentication) -> Result<()> {
    // Check authentication succeeded
    if !sca.authenticated {
        return Err(PaymentServicesError::ValidationError {
            message: format!(
                "Authentication failed for user '{}'. SCA requires successful authentication.",
                sca.user_id
            ),
        });
    }

    // Check two or more factors present (PSR 2017 Reg 68(1))
    let factors_present = sca.factor_count();

    if factors_present < 2 {
        return Err(PaymentServicesError::ScaNonCompliant {
            user_id: sca.user_id.clone(),
            factors_present,
        });
    }

    Ok(())
}

/// Validate Open Banking consent (PSR 2017, CMA Order 2017)
///
/// Checks consent compliance:
/// - Consent status is Authorized
/// - Consent not expired
/// - Permissions granted
pub fn validate_open_banking_consent(
    consent: &OpenBankingConsent,
    current_date: NaiveDate,
) -> Result<()> {
    // Check consent status
    match consent.status {
        ConsentStatus::Authorized => {}
        ConsentStatus::Revoked => {
            return Err(PaymentServicesError::ConsentRevoked {
                consent_id: consent.consent_id.clone(),
            });
        }
        ConsentStatus::Rejected => {
            return Err(PaymentServicesError::ConsentNotObtained {
                user_id: consent.user_id.clone(),
                provider_type: format!("{:?}", consent.provider_type),
            });
        }
        ConsentStatus::AwaitingAuthorization => {
            return Err(PaymentServicesError::ConsentNotObtained {
                user_id: consent.user_id.clone(),
                provider_type: format!("{:?}", consent.provider_type),
            });
        }
        ConsentStatus::Expired => {
            return Err(PaymentServicesError::ConsentExpired {
                consent_id: consent.consent_id.clone(),
                expiry_date: consent
                    .expiry_date
                    .map(|d| d.to_string())
                    .unwrap_or_else(|| "Unknown".to_string()),
            });
        }
    }

    // Check consent not expired (PSR 2017 Reg 67(3) - max 90 days for AIS)
    if let Some(expiry) = consent.expiry_date {
        if current_date > expiry {
            return Err(PaymentServicesError::ConsentExpired {
                consent_id: consent.consent_id.clone(),
                expiry_date: expiry.to_string(),
            });
        }
    }

    // Check permissions granted
    if consent.permissions.is_empty() {
        return Err(PaymentServicesError::ValidationError {
            message: format!(
                "Consent '{}' has no permissions granted",
                consent.consent_id
            ),
        });
    }

    // Check accounts authorized
    if consent.accounts_authorized.is_empty() {
        return Err(PaymentServicesError::ValidationError {
            message: format!(
                "Consent '{}' has no accounts authorized",
                consent.consent_id
            ),
        });
    }

    Ok(())
}

/// Validate client funds safeguarding (PSR 2017 Reg 20-22)
///
/// Checks safeguarding compliance:
/// - Safeguarding method appropriate
/// - Client funds segregated (if using segregation method)
/// - Daily reconciliation performed
pub fn validate_safeguarding(safeguarding: &ClientFundsSafeguarding) -> Result<()> {
    // If no client funds, safeguarding not required
    if safeguarding.client_funds_gbp == 0.0 {
        return Ok(());
    }

    // Check safeguarding method compliance
    match safeguarding.safeguarding_method {
        SafeguardingMethod::Segregation => {
            // Check funds are segregated (PSR 2017 Reg 20(1)(a))
            if !safeguarding.client_funds_segregated {
                return Err(PaymentServicesError::ClientFundsNotSegregated {
                    amount_gbp: safeguarding.client_funds_gbp,
                });
            }

            // Check safeguarding account designated
            if safeguarding.safeguarding_account.is_none() {
                return Err(PaymentServicesError::SafeguardingAccountNotDesignated {
                    institution_name: safeguarding.institution_name.clone(),
                });
            }
        }
        SafeguardingMethod::InsuranceOrGuarantee => {
            // For insurance/guarantee method, just check it's noted
            // Actual insurance/guarantee validation would require policy details
        }
    }

    // Check daily reconciliation performed (PSR 2017 Reg 21)
    if !safeguarding.daily_reconciliation_performed {
        return Err(PaymentServicesError::DailyReconciliationNotPerformed {
            institution_name: safeguarding.institution_name.clone(),
            last_reconciliation_date: safeguarding
                .last_reconciliation_date
                .map(|d| d.to_string())
                .unwrap_or_else(|| "Never".to_string()),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_sca_compliant() {
        let sca = StrongCustomerAuthentication {
            authentication_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            user_id: "USER001".to_string(),
            knowledge_factor: Some(KnowledgeFactor::Password),
            possession_factor: Some(PossessionFactor::MobileDevice {
                device_id: "DEV123".to_string(),
            }),
            inherence_factor: None,
            authenticated: true,
            authentication_method: "Password + SMS OTP".to_string(),
        };

        assert!(validate_sca(&sca).is_ok());
    }

    #[test]
    fn test_validate_sca_insufficient_factors() {
        let sca = StrongCustomerAuthentication {
            authentication_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            user_id: "USER001".to_string(),
            knowledge_factor: Some(KnowledgeFactor::Password),
            possession_factor: None,
            inherence_factor: None,
            authenticated: true,
            authentication_method: "Password only".to_string(),
        };

        let result = validate_sca(&sca);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(PaymentServicesError::ScaNonCompliant { .. })
        ));
    }

    #[test]
    fn test_validate_consent_valid() {
        let consent = OpenBankingConsent {
            consent_id: "CONSENT001".to_string(),
            user_id: "USER001".to_string(),
            provider_type: OpenBankingProviderType::AISP,
            consent_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            expiry_date: Some(NaiveDate::from_ymd_opt(2024, 4, 1).unwrap()),
            status: ConsentStatus::Authorized,
            permissions: vec![Permission::ReadAccountsDetail],
            accounts_authorized: vec!["ACC001".to_string()],
        };

        let current = NaiveDate::from_ymd_opt(2024, 2, 1).unwrap();
        assert!(validate_open_banking_consent(&consent, current).is_ok());
    }

    #[test]
    fn test_validate_consent_expired() {
        let consent = OpenBankingConsent {
            consent_id: "CONSENT002".to_string(),
            user_id: "USER001".to_string(),
            provider_type: OpenBankingProviderType::AISP,
            consent_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            expiry_date: Some(NaiveDate::from_ymd_opt(2024, 4, 1).unwrap()),
            status: ConsentStatus::Authorized,
            permissions: vec![Permission::ReadBalances],
            accounts_authorized: vec!["ACC001".to_string()],
        };

        let current = NaiveDate::from_ymd_opt(2024, 5, 1).unwrap(); // After expiry
        let result = validate_open_banking_consent(&consent, current);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(PaymentServicesError::ConsentExpired { .. })
        ));
    }

    #[test]
    fn test_validate_safeguarding_compliant() {
        let safeguarding = ClientFundsSafeguarding {
            institution_name: "Test PI".to_string(),
            safeguarding_method: SafeguardingMethod::Segregation,
            client_funds_gbp: 100_000.0,
            client_funds_segregated: true,
            safeguarding_account: Some(SafeguardingAccount {
                account_name: "Client Funds".to_string(),
                account_number: "12345678".to_string(),
                sort_code: "12-34-56".to_string(),
                bank_name: "Test Bank".to_string(),
            }),
            daily_reconciliation_performed: true,
            last_reconciliation_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
        };

        assert!(validate_safeguarding(&safeguarding).is_ok());
    }

    #[test]
    fn test_validate_safeguarding_not_segregated() {
        let safeguarding = ClientFundsSafeguarding {
            institution_name: "Test PI".to_string(),
            safeguarding_method: SafeguardingMethod::Segregation,
            client_funds_gbp: 100_000.0,
            client_funds_segregated: false, // NOT SEGREGATED
            safeguarding_account: None,
            daily_reconciliation_performed: true,
            last_reconciliation_date: None,
        };

        let result = validate_safeguarding(&safeguarding);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(PaymentServicesError::ClientFundsNotSegregated { .. })
        ));
    }
}
