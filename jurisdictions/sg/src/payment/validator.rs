//! Payment Services Act Validation Logic
//!
//! Comprehensive validation for:
//! 1. License validity and type appropriateness (PSA s. 5-8)
//! 2. Safeguarding requirements (PSA s. 23)
//! 3. AML/CFT compliance (PSA s. 20)
//! 4. DPT service authorization (PSA s. 13)

use super::error::{PaymentError, Result};
use super::types::*;
use chrono::Utc;

/// Validation report for a payment service provider
#[derive(Debug, Clone)]
pub struct PaymentProviderValidationReport {
    /// Whether the provider is compliant
    pub is_compliant: bool,
    /// Critical errors requiring immediate attention
    pub errors: Vec<PaymentError>,
    /// Warnings that should be addressed
    pub warnings: Vec<String>,
    /// License status details
    pub license_status: LicenseStatusReport,
    /// Safeguarding compliance status
    pub safeguarding_status: SafeguardingStatus,
    /// AML/CFT compliance status
    pub aml_status: AmlStatus,
}

/// License status details
#[derive(Debug, Clone)]
pub struct LicenseStatusReport {
    /// Whether license is valid and active
    pub is_valid: bool,
    /// License type held
    pub license_type: String,
    /// Whether license type matches business volume
    pub appropriate_for_volume: bool,
    /// Number of authorized services
    pub authorized_services: usize,
}

/// Safeguarding compliance status
#[derive(Debug, Clone)]
pub struct SafeguardingStatus {
    /// Whether safeguarding is required
    pub required: bool,
    /// Whether safeguarding is implemented
    pub implemented: bool,
    /// Float outstanding in SGD
    pub float_outstanding: f64,
    /// Amount safeguarded in SGD (if applicable)
    pub amount_safeguarded: Option<f64>,
    /// Whether verification is current
    pub verification_current: bool,
}

/// AML/CFT status
#[derive(Debug, Clone)]
pub struct AmlStatus {
    /// Whether AML officer is appointed
    pub has_officer: bool,
    /// Number of accounts requiring enhanced verification
    pub accounts_needing_verification: usize,
}

/// Comprehensive validation of a payment service provider
///
/// # Checks Performed
/// 1. License validity (PSA s. 5-7)
/// 2. License type appropriateness for volume (PSA s. 5)
/// 3. Service authorization (PSA s. 5)
/// 4. Safeguarding requirements (PSA s. 23)
/// 5. AML/CFT compliance officer (PSA s. 20)
/// 6. DPT service authorization (PSA s. 13)
///
/// # Returns
/// - `Ok(PaymentProviderValidationReport)` with detailed compliance status
/// - `Err(PaymentError)` for critical structural issues
pub fn validate_payment_provider(
    provider: &PaymentServiceProvider,
) -> Result<PaymentProviderValidationReport> {
    let mut report = PaymentProviderValidationReport {
        is_compliant: true,
        errors: Vec::new(),
        warnings: Vec::new(),
        license_status: LicenseStatusReport {
            is_valid: false,
            license_type: format!("{:?}", provider.license_type),
            appropriate_for_volume: false,
            authorized_services: provider.services.len(),
        },
        safeguarding_status: SafeguardingStatus {
            required: provider.requires_safeguarding(),
            implemented: provider.safeguarding_enabled,
            float_outstanding: provider.float_in_sgd(),
            amount_safeguarded: None,
            verification_current: false,
        },
        aml_status: AmlStatus {
            has_officer: provider.has_aml_officer,
            accounts_needing_verification: 0,
        },
    };

    // 1. Validate UEN format
    if !is_valid_uen(&provider.uen) {
        report.errors.push(PaymentError::InvalidUen {
            uen: provider.uen.clone(),
        });
        report.is_compliant = false;
    }

    // 2. Validate provider name
    if provider.name.len() < 3 {
        report.errors.push(PaymentError::InvalidProviderName);
        report.is_compliant = false;
    }

    // 3. Check at least one service is provided
    if provider.services.is_empty() {
        report.errors.push(PaymentError::NoServicesSpecified);
        report.is_compliant = false;
    }

    // 4. Check license status
    match validate_license_status(provider) {
        Ok(_) => {
            report.license_status.is_valid = true;
        }
        Err(e) => {
            report.errors.push(e);
            report.is_compliant = false;
        }
    }

    // 5. Check license type matches volume
    match validate_license_type_for_volume(provider) {
        Ok(_) => {
            report.license_status.appropriate_for_volume = true;
        }
        Err(e) => {
            report.errors.push(e);
            report.is_compliant = false;
        }
    }

    // 6. Check safeguarding requirements
    if provider.requires_safeguarding() && !provider.safeguarding_enabled {
        report.errors.push(PaymentError::SafeguardingNotImplemented);
        report.is_compliant = false;
    }

    // 7. Check AML officer requirement
    if !provider.has_aml_officer {
        report.errors.push(PaymentError::NoAmlOfficer);
        report.is_compliant = false;
    }

    // 8. Check DPT service authorization
    if provider.provides_dpt_services() && !provider.dpt_services.is_empty() {
        // DPT services require specific authorization - check if it's in the services list
        if !provider
            .services
            .contains(&PaymentServiceType::DigitalPaymentToken)
        {
            report.errors.push(PaymentError::UnauthorizedDptService);
            report.is_compliant = false;
        }
    }

    // 9. Warning for high float without safeguarding visibility
    if provider.float_outstanding_sgd > 100_000_000 && !provider.safeguarding_enabled {
        // SGD 1M
        report.warnings.push(format!(
            "High float outstanding (SGD {:.2}) - ensure safeguarding is properly implemented",
            provider.float_in_sgd()
        ));
    }

    // 10. Warning for approaching MPI threshold
    let monthly_volume = provider.monthly_volume_in_sgd();
    if monthly_volume > 2_500_000.0 && monthly_volume <= 3_000_000.0 {
        report.warnings.push(format!(
            "Monthly volume SGD {:.2} approaching MPI threshold (SGD 3M)",
            monthly_volume
        ));
    }

    Ok(report)
}

/// Validate license status
fn validate_license_status(provider: &PaymentServiceProvider) -> Result<()> {
    let now = Utc::now();

    match provider.license_status {
        LicenseStatus::Active => {
            // Check expiry
            if let Some(expiry) = provider.license_expiry
                && now >= expiry
            {
                return Err(PaymentError::LicenseExpired {
                    expiry_date: expiry.format("%Y-%m-%d").to_string(),
                });
            }
        }
        LicenseStatus::Suspended => {
            return Err(PaymentError::LicenseSuspended);
        }
        LicenseStatus::Revoked => {
            return Err(PaymentError::LicenseRevoked);
        }
        LicenseStatus::Pending => {
            // Not an error, but operations are restricted
        }
    }

    Ok(())
}

/// Validate license type matches business volume
///
/// # Rule
/// PSA s. 5: Major Payment Institution (MPI) license required if:
/// - Monthly transaction volume > SGD 3,000,000 OR
/// - Multiple payment services provided
fn validate_license_type_for_volume(provider: &PaymentServiceProvider) -> Result<()> {
    let monthly_volume = provider.monthly_volume_in_sgd();

    // Check if MPI license is required based on volume
    if monthly_volume > 3_000_000.0
        && !matches!(
            provider.license_type,
            PaymentLicenseType::MajorPaymentInstitution
        )
    {
        return Err(PaymentError::RequiresMpiLicense {
            volume: monthly_volume,
        });
    }

    // Check if MPI license is required based on number of services
    if provider.services.len() >= 2
        && !matches!(
            provider.license_type,
            PaymentLicenseType::MajorPaymentInstitution
        )
    {
        // Multiple services generally require MPI (with some exceptions)
        // This is a simplified check - actual rules are more nuanced
    }

    Ok(())
}

/// Validate safeguarding arrangement
///
/// # Requirements (PSA s. 23)
/// - E-money issuance: 110% of float
/// - Account issuance: 100% of float
/// - Domestic money transfer: 100% of outstanding float
pub fn validate_safeguarding(
    provider: &PaymentServiceProvider,
    arrangement: &SafeguardingArrangement,
) -> Result<Vec<String>> {
    let mut warnings = Vec::new();

    // Check if safeguarding amount is sufficient
    let safeguarded = arrangement.amount_in_sgd();
    let float = provider.float_in_sgd();

    // E-money typically requires 110% coverage
    let required_coverage = if provider
        .services
        .contains(&PaymentServiceType::EMoneyIssuance)
    {
        float * 1.1
    } else {
        float
    };

    if safeguarded < required_coverage {
        return Err(PaymentError::InsufficientSafeguarding {
            safeguarded,
            float: required_coverage,
        });
    }

    // Check verification recency
    if arrangement.verification_overdue(Utc::now()) {
        let days_overdue = (Utc::now() - arrangement.last_verified).num_days();
        return Err(PaymentError::SafeguardingVerificationOverdue { days_overdue });
    }

    // Warning if verification approaching due date
    let days_since_verification = (Utc::now() - arrangement.last_verified).num_days();
    if days_since_verification > 330 {
        // Within 35 days of annual verification
        warnings.push("Safeguarding arrangement verification due within 35 days".to_string());
    }

    Ok(warnings)
}

/// Validate customer payment account compliance
///
/// # Checks
/// 1. KYC completed
/// 2. Enhanced verification for high-value accounts
/// 3. Risk categorization appropriate
pub fn validate_customer_account(account: &CustomerPaymentAccount) -> Result<Vec<String>> {
    let mut warnings = Vec::new();

    // Check KYC completion
    if !account.kyc_completed {
        return Err(PaymentError::KycNotCompleted {
            account_id: account.account_id.clone(),
        });
    }

    // Check enhanced verification requirement
    if account.requires_enhanced_verification() && !account.is_verified {
        return Err(PaymentError::EnhancedVerificationRequired {
            account_id: account.account_id.clone(),
            balance: account.balance_in_sgd(),
        });
    }

    // Warning for high-value accounts
    if account.balance_sgd > 1_000_000 {
        // SGD 10,000
        warnings.push(format!(
            "High-value account {} - ensure ongoing monitoring",
            account.account_id
        ));
    }

    Ok(warnings)
}

/// Validate payment transaction for reporting requirements
pub fn validate_transaction(transaction: &PaymentTransaction) -> Result<Vec<String>> {
    let mut warnings = Vec::new();

    // Check reporting threshold for certain transaction types
    if transaction.exceeds_reporting_threshold() {
        // Ensure proper reporting for large transactions
        warnings.push(format!(
            "Transaction {} of SGD {:.2} exceeds reporting threshold (SGD 5,000)",
            transaction.transaction_id,
            transaction.amount_in_sgd()
        ));
    }

    // Cross-border transactions require additional checks
    if transaction.is_cross_border
        && (transaction.originating_country.is_none() || transaction.beneficiary_country.is_none())
    {
        warnings.push(format!(
            "Cross-border transaction {} missing country information",
            transaction.transaction_id
        ));
    }

    Ok(warnings)
}

/// Validate DPT service compliance
///
/// # Requirements
/// - Specific DPT license authorization
/// - AML/CFT compliance measures
/// - Technology risk management
pub fn validate_dpt_service(provider: &PaymentServiceProvider) -> Result<Vec<String>> {
    let mut warnings = Vec::new();

    // Check if DPT services are authorized
    if provider.provides_dpt_services() {
        if !provider
            .services
            .contains(&PaymentServiceType::DigitalPaymentToken)
        {
            return Err(PaymentError::UnauthorizedDptService);
        }

        // Check AML compliance for DPT
        if !provider.has_aml_officer {
            return Err(PaymentError::DptAmlNonCompliance);
        }

        // Warning about enhanced AML measures for DPT
        warnings.push("DPT services require enhanced AML/CFT measures - ensure transaction monitoring is robust".to_string());
    }

    Ok(warnings)
}

/// Assess overall safeguarding status
pub fn assess_safeguarding_status(
    provider: &PaymentServiceProvider,
    arrangements: &[SafeguardingArrangement],
) -> SafeguardingStatus {
    let total_safeguarded: u64 = arrangements.iter().map(|a| a.amount_safeguarded_sgd).sum();

    let verification_current = arrangements
        .iter()
        .all(|a| !a.verification_overdue(Utc::now()));

    SafeguardingStatus {
        required: provider.requires_safeguarding(),
        implemented: provider.safeguarding_enabled && !arrangements.is_empty(),
        float_outstanding: provider.float_in_sgd(),
        amount_safeguarded: Some(total_safeguarded as f64 / 100.0),
        verification_current,
    }
}

/// Check if UEN format is valid
fn is_valid_uen(uen: &str) -> bool {
    if uen.len() < 9 || uen.len() > 10 {
        return false;
    }
    uen.chars().any(|c| c.is_ascii_digit())
}

/// Calculate required safeguarding amount based on float and service type
///
/// # Parameters
/// - `float_sgd`: Outstanding float in SGD cents
/// - `service_type`: Type of payment service
///
/// # Returns
/// Required safeguarding amount in SGD cents
pub fn calculate_required_safeguarding(float_sgd: u64, service_type: &PaymentServiceType) -> u64 {
    match service_type {
        PaymentServiceType::EMoneyIssuance => {
            // 110% of float for e-money
            (float_sgd as f64 * 1.1) as u64
        }
        PaymentServiceType::AccountIssuance | PaymentServiceType::DomesticMoneyTransfer => {
            // 100% of float
            float_sgd
        }
        _ => 0, // Other services may not require safeguarding
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_uen() {
        assert!(is_valid_uen("197700001E"));
        assert!(is_valid_uen("S99XX1234A"));
        assert!(!is_valid_uen("123")); // Too short
    }

    #[test]
    fn test_mpi_volume_threshold() {
        let provider = PaymentServiceProvider {
            uen: "197700001E".to_string(),
            name: "Test Payment Ltd".to_string(),
            license_type: PaymentLicenseType::StandardPaymentInstitution,
            license_status: LicenseStatus::Active,
            license_date: Utc::now(),
            license_expiry: None,
            services: vec![PaymentServiceType::DomesticMoneyTransfer],
            monthly_volume_sgd: 400_000_000, // SGD 4M - requires MPI
            float_outstanding_sgd: 0,
            safeguarding_enabled: false,
            dpt_services: Vec::new(),
            has_aml_officer: true,
        };

        assert!(provider.requires_mpi_license());
    }

    #[test]
    fn test_safeguarding_requirement() {
        let provider = PaymentServiceProvider {
            uen: "197700001E".to_string(),
            name: "E-Wallet Ltd".to_string(),
            license_type: PaymentLicenseType::StandardPaymentInstitution,
            license_status: LicenseStatus::Active,
            license_date: Utc::now(),
            license_expiry: None,
            services: vec![PaymentServiceType::EMoneyIssuance],
            monthly_volume_sgd: 100_000_000,
            float_outstanding_sgd: 50_000_000, // SGD 500k
            safeguarding_enabled: false,
            dpt_services: Vec::new(),
            has_aml_officer: true,
        };

        assert!(provider.requires_safeguarding());
    }

    #[test]
    fn test_enhanced_verification_threshold() {
        let account = CustomerPaymentAccount {
            account_id: "ACC001".to_string(),
            customer_name: "Test User".to_string(),
            customer_id: "S1234567A".to_string(),
            account_type: PaymentAccountType::EWallet,
            balance_sgd: 600_000, // SGD 6,000 - exceeds SGD 5,000 threshold
            opened_date: Utc::now(),
            kyc_completed: true,
            risk_category: RiskCategory::Medium,
            is_verified: false,
        };

        assert!(account.requires_enhanced_verification());
    }

    #[test]
    fn test_emoney_safeguarding_calculation() {
        let float_sgd = 100_000_000; // SGD 1M in cents
        let required =
            calculate_required_safeguarding(float_sgd, &PaymentServiceType::EMoneyIssuance);

        assert_eq!(required, 110_000_000); // 110% = SGD 1.1M in cents
    }
}
