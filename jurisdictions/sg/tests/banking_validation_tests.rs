//! Integration tests for Banking Act validation

use chrono::Utc;
use legalis_sg::banking::*;

#[test]
fn test_full_bank_compliant() {
    let capital = CapitalAdequacy {
        cet1_capital_sgd: 1_500_000_000_00,
        at1_capital_sgd: 300_000_000_00,
        tier2_capital_sgd: 500_000_000_00,
        risk_weighted_assets_sgd: 10_000_000_000_00,
        calculation_date: Utc::now(),
    };

    let aml_officer = ComplianceOfficer {
        name: "Test Officer".to_string(),
        identification: "S1234567A".to_string(),
        email: "test@bank.com".to_string(),
        phone: "+65 6123 4567".to_string(),
        appointed_date: Utc::now(),
        qualifications: vec!["ACAMS".to_string()],
    };

    let bank = BankBuilder::new()
        .uen("197700001E".to_string())
        .name("Test Bank Ltd".to_string())
        .license_type(BankLicenseType::FullBank)
        .license_date(Utc::now())
        .locally_incorporated(true)
        .country_of_incorporation("Singapore".to_string())
        .capital_adequacy(capital)
        .aml_officer(aml_officer)
        .total_assets_sgd(50_000_000_000_00)
        .total_deposits_sgd(40_000_000_000_00)
        .build()
        .unwrap();

    let report = validate_bank(&bank).unwrap();
    assert!(report.is_compliant);
    assert!(report.errors.is_empty());
    assert!(report.capital_status.meets_minimum);
    assert!(report.aml_compliance.has_aml_officer);
}

#[test]
fn test_insufficient_cet1_capital() {
    let capital = CapitalAdequacy {
        cet1_capital_sgd: 500_000_000_00, // 5% - INSUFFICIENT
        at1_capital_sgd: 200_000_000_00,
        tier2_capital_sgd: 300_000_000_00,
        risk_weighted_assets_sgd: 10_000_000_000_00,
        calculation_date: Utc::now(),
    };

    let bank = Bank::new(
        "197700001E".to_string(),
        "Test Bank Ltd".to_string(),
        BankLicenseType::FullBank,
        Utc::now(),
        true,
        "Singapore".to_string(),
        capital,
    );

    let report = validate_bank(&bank).unwrap();
    assert!(!report.is_compliant);
    assert!(!report.errors.is_empty());
    assert!(!report.capital_status.meets_minimum);
}

#[test]
fn test_wholesale_bank_deposit_validation() {
    // Valid deposit above threshold
    let result = validate_wholesale_deposit(&BankLicenseType::WholesaleBank, 500_000.0);
    assert!(result.is_ok());

    // Invalid deposit below threshold
    let result = validate_wholesale_deposit(&BankLicenseType::WholesaleBank, 100_000.0);
    assert!(result.is_err());
    match result {
        Err(BankingError::WholesaleBankMinimumDeposit { amount }) => {
            assert_eq!(amount, 100_000.0);
        }
        _ => panic!("Expected WholesaleBankMinimumDeposit error"),
    }

    // Edge case: exactly at threshold
    let result = validate_wholesale_deposit(&BankLicenseType::WholesaleBank, 250_000.0);
    assert!(result.is_ok());
}

#[test]
fn test_merchant_bank_retail_deposit_restriction() {
    // Merchant bank accepting retail deposits - INVALID
    let result = validate_merchant_bank_activities(&BankLicenseType::MerchantBank, true);
    assert!(result.is_err());
    match result {
        Err(BankingError::MerchantBankRetailDeposit) => {}
        _ => panic!("Expected MerchantBankRetailDeposit error"),
    }

    // Merchant bank not accepting retail deposits - VALID
    let result = validate_merchant_bank_activities(&BankLicenseType::MerchantBank, false);
    assert!(result.is_ok());

    // Full bank can accept retail deposits
    let result = validate_merchant_bank_activities(&BankLicenseType::FullBank, true);
    assert!(result.is_ok());
}

#[test]
fn test_customer_cdd_high_risk() {
    let account = CustomerAccount {
        account_number: "ACC001".to_string(),
        customer_name: "Test Customer".to_string(),
        customer_id: "S1234567A".to_string(),
        risk_category: CustomerRiskCategory::High,
        account_opened: Utc::now(),
        last_cdd_review: Utc::now(),
        edd_performed: true,
        source_of_funds_verified: true,
        beneficial_owner_identified: true,
        balance_sgd: 100_000_00,
    };

    let result = validate_customer_account(&account);
    assert!(result.is_ok());

    // Test overdue CDD for high risk (requires annual review)
    let mut overdue_account = account.clone();
    overdue_account.last_cdd_review = Utc::now() - chrono::Duration::days(400); // Over 1 year

    let result = validate_customer_account(&overdue_account);
    assert!(result.is_err());
    match result {
        Err(BankingError::CddReviewOverdue { days_ago, .. }) => {
            assert!(days_ago > 365);
        }
        _ => panic!("Expected CddReviewOverdue error"),
    }
}

#[test]
fn test_edd_requirement() {
    let mut account = CustomerAccount {
        account_number: "ACC002".to_string(),
        customer_name: "High Risk Customer".to_string(),
        customer_id: "S9876543B".to_string(),
        risk_category: CustomerRiskCategory::High,
        account_opened: Utc::now(),
        last_cdd_review: Utc::now(),
        edd_performed: false, // EDD NOT performed
        source_of_funds_verified: true,
        beneficial_owner_identified: true,
        balance_sgd: 100_000_00,
    };

    assert!(account.requires_edd());

    let result = validate_customer_account(&account);
    assert!(result.is_err());
    match result {
        Err(BankingError::EddRequired { account_number }) => {
            assert_eq!(account_number, "ACC002");
        }
        _ => panic!("Expected EddRequired error"),
    }

    // Fix by performing EDD
    account.edd_performed = true;
    let result = validate_customer_account(&account);
    assert!(result.is_ok());
}

#[test]
fn test_str_filing_timeline() {
    let transaction_date = Utc::now();
    let filing_date = transaction_date + chrono::Duration::days(7); // 7 days later

    let str = SuspiciousTransactionReport {
        reference_number: "STR001".to_string(),
        account_number: "ACC003".to_string(),
        customer_name: "Test".to_string(),
        transaction_amount_sgd: 100_000_00,
        transaction_date,
        filing_date,
        suspicion_description: "Suspicious pattern".to_string(),
        transaction_proceeded: false,
    };

    assert!(str.filed_timely());

    // Test late filing
    let late_str = SuspiciousTransactionReport {
        filing_date: transaction_date + chrono::Duration::days(15), // 15 days - late
        ..str
    };

    assert!(!late_str.filed_timely());

    let result = validate_str_filing(&late_str);
    assert!(result.is_err());
    match result {
        Err(BankingError::StrFiledLate { days_late }) => {
            assert_eq!(days_late, 15);
        }
        _ => panic!("Expected StrFiledLate error"),
    }
}

#[test]
fn test_cash_transaction_reporting_threshold() {
    let ctr_above_threshold = CashTransactionReport {
        account_number: "ACC004".to_string(),
        customer_name: "Test Customer".to_string(),
        customer_id: "S1234567A".to_string(),
        amount_sgd: 2_500_000, // SGD 25,000
        transaction_type: CashTransactionType::Deposit,
        transaction_date: Utc::now(),
        purpose: "Business proceeds".to_string(),
    };

    assert!(ctr_above_threshold.meets_reporting_threshold());
    assert!(validate_cash_transaction(&ctr_above_threshold).is_ok());

    let ctr_below_threshold = CashTransactionReport {
        amount_sgd: 1_000_000, // SGD 10,000 - below threshold
        ..ctr_above_threshold
    };

    assert!(!ctr_below_threshold.meets_reporting_threshold());
    let result = validate_cash_transaction(&ctr_below_threshold);
    assert!(result.is_err());
}

#[test]
fn test_capital_calculations() {
    let capital = CapitalAdequacy {
        cet1_capital_sgd: 1_000_000_000_00,          // SGD 10M
        at1_capital_sgd: 200_000_000_00,             // SGD 2M
        tier2_capital_sgd: 300_000_000_00,           // SGD 3M
        risk_weighted_assets_sgd: 10_000_000_000_00, // SGD 100M
        calculation_date: Utc::now(),
    };

    assert_eq!(capital.cet1_ratio(), 10.0);
    assert_eq!(capital.tier1_ratio(), 12.0);
    assert_eq!(capital.total_capital_ratio(), 15.0);
    assert!(capital.meets_regulatory_minimum());

    // Test capital shortfall calculation
    let required = calculate_required_capital(capital.risk_weighted_assets_sgd, 6.5);
    assert_eq!(required, 650_000_000_00); // 6.5% of 100M

    let shortfall = calculate_capital_shortfall(
        500_000_000_00, // Only SGD 5M
        capital.risk_weighted_assets_sgd,
        6.5,
    );
    assert_eq!(shortfall, 150_000_000_00); // Need additional 1.5M
}

#[test]
fn test_bank_builder_validation() {
    // Test successful build
    let capital = CapitalAdequacy {
        cet1_capital_sgd: 1_000_000_000_00,
        at1_capital_sgd: 200_000_000_00,
        tier2_capital_sgd: 300_000_000_00,
        risk_weighted_assets_sgd: 10_000_000_000_00,
        calculation_date: Utc::now(),
    };

    let result = BankBuilder::new()
        .uen("197700001E".to_string())
        .name("Test Bank Ltd".to_string())
        .license_type(BankLicenseType::FullBank)
        .license_date(Utc::now())
        .locally_incorporated(true)
        .country_of_incorporation("Singapore".to_string())
        .capital_adequacy(capital)
        .build();

    assert!(result.is_ok());

    // Test missing required fields
    let result = BankBuilder::new().name("Test Bank".to_string()).build();

    assert!(result.is_err());
}

#[test]
fn test_aml_compliance_assessment() {
    let accounts = vec![
        CustomerAccount {
            account_number: "ACC001".to_string(),
            customer_name: "Customer 1".to_string(),
            customer_id: "S1111111A".to_string(),
            risk_category: CustomerRiskCategory::High,
            account_opened: Utc::now(),
            last_cdd_review: Utc::now() - chrono::Duration::days(400), // Overdue
            edd_performed: true,
            source_of_funds_verified: true,
            beneficial_owner_identified: true,
            balance_sgd: 100_000_00,
        },
        CustomerAccount {
            account_number: "ACC002".to_string(),
            customer_name: "Customer 2".to_string(),
            customer_id: "S2222222B".to_string(),
            risk_category: CustomerRiskCategory::PoliticallyExposed,
            account_opened: Utc::now(),
            last_cdd_review: Utc::now(),
            edd_performed: true,
            source_of_funds_verified: true,
            beneficial_owner_identified: true,
            balance_sgd: 200_000_00,
        },
        CustomerAccount {
            account_number: "ACC003".to_string(),
            customer_name: "Customer 3".to_string(),
            customer_id: "S3333333C".to_string(),
            risk_category: CustomerRiskCategory::Low,
            account_opened: Utc::now(),
            last_cdd_review: Utc::now(),
            edd_performed: false,
            source_of_funds_verified: true,
            beneficial_owner_identified: true,
            balance_sgd: 50_000_00,
        },
    ];

    let status = assess_aml_compliance(&accounts);

    assert_eq!(status.high_risk_accounts, 2); // High + PEP
    assert_eq!(status.overdue_cdd_reviews, 1); // ACC001
}
