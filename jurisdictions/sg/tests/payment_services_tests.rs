//! Integration tests for Payment Services Act validation

use chrono::Utc;
use legalis_sg::payment::*;

#[test]
fn test_payment_provider_compliant() {
    let provider = PaymentServiceProviderBuilder::new()
        .uen("202012345A".to_string())
        .name("Test Payment Ltd".to_string())
        .license_type(PaymentLicenseType::StandardPaymentInstitution)
        .license_date(Utc::now())
        .add_service(PaymentServiceType::EMoneyIssuance)
        .monthly_volume_sgd(200_000_000) // SGD 2M/month
        .safeguarding_enabled(true) // Required for e-money
        .has_aml_officer(true)
        .build()
        .unwrap();

    let report = validate_payment_provider(&provider).unwrap();
    assert!(report.is_compliant);
    assert!(report.errors.is_empty());
    assert!(report.license_status.is_valid);
    assert!(report.aml_status.has_officer);
}

#[test]
fn test_mpi_volume_threshold() {
    // Provider exceeding SPI threshold
    let provider = PaymentServiceProviderBuilder::new()
        .uen("202012345A".to_string())
        .name("Growing Payment Ltd".to_string())
        .license_type(PaymentLicenseType::StandardPaymentInstitution) // Wrong license type
        .license_date(Utc::now())
        .add_service(PaymentServiceType::EMoneyIssuance)
        .monthly_volume_sgd(400_000_000) // SGD 4M/month - exceeds SGD 3M threshold
        .has_aml_officer(true)
        .build()
        .unwrap();

    assert!(provider.requires_mpi_license());

    let report = validate_payment_provider(&provider).unwrap();
    assert!(!report.is_compliant);

    // Check for specific error
    let has_mpi_error = report
        .errors
        .iter()
        .any(|e| matches!(e, PaymentError::RequiresMpiLicense { .. }));
    assert!(has_mpi_error);
}

#[test]
fn test_safeguarding_requirement() {
    // Provider requiring safeguarding
    let provider = PaymentServiceProviderBuilder::new()
        .uen("202012345A".to_string())
        .name("E-Wallet Ltd".to_string())
        .license_type(PaymentLicenseType::StandardPaymentInstitution)
        .license_date(Utc::now())
        .add_service(PaymentServiceType::EMoneyIssuance)
        .monthly_volume_sgd(100_000_000)
        .float_outstanding_sgd(50_000_000) // SGD 500k
        .safeguarding_enabled(false) // NOT enabled - violation
        .has_aml_officer(true)
        .build()
        .unwrap();

    assert!(provider.requires_safeguarding());

    let report = validate_payment_provider(&provider).unwrap();
    assert!(!report.is_compliant);

    let has_safeguarding_error = report
        .errors
        .iter()
        .any(|e| matches!(e, PaymentError::SafeguardingNotImplemented));
    assert!(has_safeguarding_error);
}

#[test]
fn test_safeguarding_validation() {
    let provider = PaymentServiceProviderBuilder::new()
        .uen("202012345A".to_string())
        .name("E-Wallet Ltd".to_string())
        .license_type(PaymentLicenseType::StandardPaymentInstitution)
        .license_date(Utc::now())
        .add_service(PaymentServiceType::EMoneyIssuance)
        .monthly_volume_sgd(100_000_000)
        .float_outstanding_sgd(50_000_000) // SGD 500k
        .safeguarding_enabled(true)
        .has_aml_officer(true)
        .build()
        .unwrap();

    // Sufficient safeguarding (110% for e-money)
    let arrangement = SafeguardingArrangement {
        arrangement_type: SafeguardingType::TrustAccount,
        institution_name: "DBS Bank Ltd".to_string(),
        reference: "TRUST-ACC-123".to_string(),
        amount_safeguarded_sgd: 55_000_000, // SGD 550k (110%)
        established_date: Utc::now(),
        last_verified: Utc::now(),
    };

    let result = validate_safeguarding(&provider, &arrangement);
    assert!(result.is_ok());

    // Insufficient safeguarding
    let insufficient_arrangement = SafeguardingArrangement {
        amount_safeguarded_sgd: 40_000_000, // SGD 400k - insufficient
        ..arrangement
    };

    let result = validate_safeguarding(&provider, &insufficient_arrangement);
    assert!(result.is_err());
    match result {
        Err(PaymentError::InsufficientSafeguarding { .. }) => {}
        _ => panic!("Expected InsufficientSafeguarding error"),
    }
}

#[test]
fn test_safeguarding_calculation() {
    // E-money requires 110%
    let required = calculate_required_safeguarding(
        100_000_000, // SGD 1M
        &PaymentServiceType::EMoneyIssuance,
    );
    assert_eq!(required, 110_000_000); // SGD 1.1M

    // Account issuance requires 100%
    let required =
        calculate_required_safeguarding(100_000_000, &PaymentServiceType::AccountIssuance);
    assert_eq!(required, 100_000_000); // SGD 1M

    // Merchant acquisition doesn't require safeguarding
    let required =
        calculate_required_safeguarding(100_000_000, &PaymentServiceType::MerchantAcquisition);
    assert_eq!(required, 0);
}

#[test]
fn test_dpt_service_authorization() {
    // DPT provider with proper authorization
    let provider = PaymentServiceProviderBuilder::new()
        .uen("202098765B".to_string())
        .name("Crypto Exchange Ltd".to_string())
        .license_type(PaymentLicenseType::MajorPaymentInstitution)
        .license_date(Utc::now())
        .add_service(PaymentServiceType::DigitalPaymentToken)
        .add_dpt_service(DptServiceType::Exchange)
        .has_aml_officer(true)
        .build()
        .unwrap();

    assert!(provider.provides_dpt_services());

    let result = validate_dpt_service(&provider);
    assert!(result.is_ok());

    // DPT service without AML officer - violation
    let dpt_without_aml = PaymentServiceProviderBuilder::new()
        .uen("202098765C".to_string())
        .name("Non-compliant Crypto Ltd".to_string())
        .license_type(PaymentLicenseType::MajorPaymentInstitution)
        .license_date(Utc::now())
        .add_service(PaymentServiceType::DigitalPaymentToken)
        .add_dpt_service(DptServiceType::Exchange)
        .has_aml_officer(false) // Missing AML officer
        .build()
        .unwrap();

    let result = validate_dpt_service(&dpt_without_aml);
    assert!(result.is_err());
    match result {
        Err(PaymentError::DptAmlNonCompliance) => {}
        _ => panic!("Expected DptAmlNonCompliance error"),
    }
}

#[test]
fn test_customer_kyc_validation() {
    let account = CustomerPaymentAccount {
        account_id: "EWALLET-001".to_string(),
        customer_name: "Test User".to_string(),
        customer_id: "S1234567A".to_string(),
        account_type: PaymentAccountType::EWallet,
        balance_sgd: 300_000, // SGD 3,000 - below threshold
        opened_date: Utc::now(),
        kyc_completed: true,
        risk_category: RiskCategory::Low,
        is_verified: false, // Not required below threshold
    };

    assert!(!account.requires_enhanced_verification());
    let result = validate_customer_account(&account);
    assert!(result.is_ok());

    // High balance account requiring enhanced verification
    let mut high_balance_account = account.clone();
    high_balance_account.balance_sgd = 800_000; // SGD 8,000 - exceeds SGD 5,000 threshold
    high_balance_account.is_verified = false; // NOT verified - violation

    assert!(high_balance_account.requires_enhanced_verification());

    let result = validate_customer_account(&high_balance_account);
    assert!(result.is_err());
    match result {
        Err(PaymentError::EnhancedVerificationRequired { balance, .. }) => {
            assert_eq!(balance, 8000.0);
        }
        _ => panic!("Expected EnhancedVerificationRequired error"),
    }

    // Fix by completing verification
    high_balance_account.is_verified = true;
    let result = validate_customer_account(&high_balance_account);
    assert!(result.is_ok());
}

#[test]
fn test_transaction_validation() {
    let transaction = PaymentTransaction {
        transaction_id: "TXN001".to_string(),
        service_type: PaymentServiceType::CrossBorderMoneyTransfer,
        sender: "Alice".to_string(),
        recipient: "Bob".to_string(),
        amount_sgd: 600_000, // SGD 6,000
        currency: Some("USD".to_string()),
        timestamp: Utc::now(),
        is_cross_border: true,
        originating_country: Some("Singapore".to_string()),
        beneficiary_country: Some("United States".to_string()),
    };

    assert!(transaction.exceeds_reporting_threshold());

    let result = validate_transaction(&transaction);
    assert!(result.is_ok());

    // Cross-border without country information - warning
    let incomplete_transaction = PaymentTransaction {
        originating_country: None,
        beneficiary_country: None,
        ..transaction
    };

    let result = validate_transaction(&incomplete_transaction);
    assert!(result.is_ok()); // Not an error, but has warnings
    if let Ok(warnings) = result {
        assert!(!warnings.is_empty());
    }
}

#[test]
fn test_payment_provider_builder_validation() {
    // Successful build
    let result = PaymentServiceProviderBuilder::new()
        .uen("202012345A".to_string())
        .name("Test Payment Ltd".to_string())
        .license_type(PaymentLicenseType::StandardPaymentInstitution)
        .license_date(Utc::now())
        .add_service(PaymentServiceType::EMoneyIssuance)
        .build();

    assert!(result.is_ok());

    // Missing required fields
    let result = PaymentServiceProviderBuilder::new()
        .name("Test Payment Ltd".to_string())
        .build();

    assert!(result.is_err());

    // No services specified
    let result = PaymentServiceProviderBuilder::new()
        .uen("202012345A".to_string())
        .name("Test Payment Ltd".to_string())
        .license_type(PaymentLicenseType::StandardPaymentInstitution)
        .license_date(Utc::now())
        .build();

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "At least one payment service is required"
    );
}

#[test]
fn test_safeguarding_verification_overdue() {
    let provider = PaymentServiceProviderBuilder::new()
        .uen("202012345A".to_string())
        .name("E-Wallet Ltd".to_string())
        .license_type(PaymentLicenseType::StandardPaymentInstitution)
        .license_date(Utc::now())
        .add_service(PaymentServiceType::EMoneyIssuance)
        .monthly_volume_sgd(100_000_000)
        .float_outstanding_sgd(50_000_000)
        .safeguarding_enabled(true)
        .has_aml_officer(true)
        .build()
        .unwrap();

    // Verification overdue
    let arrangement = SafeguardingArrangement {
        arrangement_type: SafeguardingType::TrustAccount,
        institution_name: "DBS Bank Ltd".to_string(),
        reference: "TRUST-ACC-123".to_string(),
        amount_safeguarded_sgd: 55_000_000,
        established_date: Utc::now() - chrono::Duration::days(400),
        last_verified: Utc::now() - chrono::Duration::days(400), // Over 1 year
    };

    assert!(arrangement.verification_overdue(Utc::now()));

    let result = validate_safeguarding(&provider, &arrangement);
    assert!(result.is_err());
    match result {
        Err(PaymentError::SafeguardingVerificationOverdue { days_overdue }) => {
            assert!(days_overdue > 365);
        }
        _ => panic!("Expected SafeguardingVerificationOverdue error"),
    }
}

#[test]
fn test_assess_safeguarding_status() {
    let provider = PaymentServiceProviderBuilder::new()
        .uen("202012345A".to_string())
        .name("E-Wallet Ltd".to_string())
        .license_type(PaymentLicenseType::StandardPaymentInstitution)
        .license_date(Utc::now())
        .add_service(PaymentServiceType::EMoneyIssuance)
        .monthly_volume_sgd(100_000_000)
        .float_outstanding_sgd(50_000_000)
        .safeguarding_enabled(true)
        .has_aml_officer(true)
        .build()
        .unwrap();

    let arrangements = vec![SafeguardingArrangement {
        arrangement_type: SafeguardingType::TrustAccount,
        institution_name: "DBS Bank Ltd".to_string(),
        reference: "TRUST-ACC-123".to_string(),
        amount_safeguarded_sgd: 55_000_000,
        established_date: Utc::now(),
        last_verified: Utc::now(),
    }];

    let status = assess_safeguarding_status(&provider, &arrangements);

    assert!(status.required);
    assert!(status.implemented);
    assert_eq!(status.float_outstanding, 500_000.0);
    assert_eq!(status.amount_safeguarded, Some(550_000.0));
    assert!(status.verification_current);
}

#[test]
fn test_no_aml_officer_error() {
    let provider = PaymentServiceProviderBuilder::new()
        .uen("202012345A".to_string())
        .name("Test Payment Ltd".to_string())
        .license_type(PaymentLicenseType::StandardPaymentInstitution)
        .license_date(Utc::now())
        .add_service(PaymentServiceType::EMoneyIssuance)
        .has_aml_officer(false) // No AML officer
        .build()
        .unwrap();

    let report = validate_payment_provider(&provider).unwrap();
    assert!(!report.is_compliant);

    let has_aml_error = report
        .errors
        .iter()
        .any(|e| matches!(e, PaymentError::NoAmlOfficer));
    assert!(has_aml_error);
}

#[test]
fn test_digital_payment_token_types() {
    let token = DigitalPaymentToken {
        symbol: "BTC".to_string(),
        name: "Bitcoin".to_string(),
        is_supported: true,
        daily_volume_sgd: 1_000_000_000, // SGD 10M
        price_sgd: 6_000_000,            // SGD 60,000
    };

    assert_eq!(token.daily_volume_in_sgd(), 10_000_000.0);
    assert_eq!(token.price_in_sgd(), 60_000.0);
}

#[test]
fn test_payment_account_types() {
    let ewallet = CustomerPaymentAccount {
        account_id: "EWALLET-001".to_string(),
        customer_name: "Test User".to_string(),
        customer_id: "S1234567A".to_string(),
        account_type: PaymentAccountType::EWallet,
        balance_sgd: 100_000,
        opened_date: Utc::now(),
        kyc_completed: true,
        risk_category: RiskCategory::Low,
        is_verified: false,
    };

    assert_eq!(ewallet.balance_in_sgd(), 1000.0);

    let crypto_wallet = CustomerPaymentAccount {
        account_type: PaymentAccountType::CryptoWallet,
        risk_category: RiskCategory::High, // Crypto usually higher risk
        ..ewallet.clone()
    };

    assert_eq!(crypto_wallet.account_type, PaymentAccountType::CryptoWallet);
}
