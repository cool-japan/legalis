//! Banking Act - Basel III Capital Adequacy Example
//!
//! This example demonstrates comprehensive validation of banking institutions under
//! Singapore's Banking Act (Cap. 19) and MAS Notice 637 (Basel III framework).
//!
//! # Topics Covered
//!
//! 1. **Banking License Types** (s. 4, s. 28)
//! 2. **Basel III Capital Adequacy Ratios** (MAS Notice 637)
//! 3. **AML/CFT Compliance** (MAS Notice 626)
//! 4. **Customer Due Diligence (CDD)**
//! 5. **Suspicious Transaction Reporting (STR)**
//!
//! # Regulatory Framework
//!
//! ## Basel III Capital Requirements (MAS Notice 637)
//!
//! - **CET1 (Common Equity Tier 1)**: ≥ 6.5% (includes 2.5% conservation buffer)
//! - **Tier 1 Capital**: ≥ 8.0%
//! - **Total Capital**: ≥ 10.0%
//!
//! ## Banking License Types (Banking Act s. 4)
//!
//! - **Full Bank**: Can accept deposits of any size
//! - **Wholesale Bank**: Minimum deposit SGD 250,000
//! - **Merchant Bank**: Investment banking only, no retail deposits

use chrono::Utc;
use legalis_sg::banking::*;

fn main() {
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║   SINGAPORE BANKING ACT - BASEL III CAPITAL ADEQUACY          ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    // Example 1: Full Bank with Strong Capital Position
    println!("━━━ 1. FULL BANK - STRONG CAPITAL POSITION ━━━\n");
    example_full_bank_strong_capital();

    println!("\n{}\n", "═".repeat(66));

    // Example 2: Bank with Insufficient Capital
    println!("━━━ 2. BANK WITH INSUFFICIENT CAPITAL ━━━\n");
    example_insufficient_capital();

    println!("\n{}\n", "═".repeat(66));

    // Example 3: Wholesale Bank Deposit Validation
    println!("━━━ 3. WHOLESALE BANK - DEPOSIT VALIDATION ━━━\n");
    example_wholesale_bank();

    println!("\n{}\n", "═".repeat(66));

    // Example 4: AML/CFT Customer Due Diligence
    println!("━━━ 4. AML/CFT - CUSTOMER DUE DILIGENCE ━━━\n");
    example_aml_cdd();

    println!("\n{}\n", "═".repeat(66));

    // Example 5: Suspicious Transaction Reporting
    println!("━━━ 5. SUSPICIOUS TRANSACTION REPORTING (STR) ━━━\n");
    example_str_reporting();

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("✅ All banking validation examples completed");
    println!("═══════════════════════════════════════════════════════════════\n");
}

fn example_full_bank_strong_capital() {
    println!("📋 Bank: Singapore Commercial Bank Ltd");
    println!("{}\\n", "─".repeat(66));

    // Create capital adequacy data meeting all requirements
    let capital = CapitalAdequacy {
        cet1_capital_sgd: 1_500_000_000_00,          // SGD 15M
        at1_capital_sgd: 300_000_000_00,             // SGD 3M
        tier2_capital_sgd: 500_000_000_00,           // SGD 5M
        risk_weighted_assets_sgd: 10_000_000_000_00, // SGD 100M
        calculation_date: Utc::now(),
    };

    println!("💰 Capital Structure:");
    println!(
        "   CET1 Capital:  SGD {:>12.2}",
        capital.cet1_capital_sgd_amount()
    );
    println!(
        "   AT1 Capital:   SGD {:>12.2}",
        capital.at1_capital_sgd_amount()
    );
    println!(
        "   Tier 2 Capital: SGD {:>12.2}",
        capital.tier2_capital_sgd_amount()
    );
    println!("   ───────────────────────────────");
    println!(
        "   Total Capital:  SGD {:>12.2}",
        capital.cet1_capital_sgd_amount()
            + capital.at1_capital_sgd_amount()
            + capital.tier2_capital_sgd_amount()
    );
    println!("   RWA:           SGD {:>12.2}\n", capital.rwa_sgd_amount());

    println!("📊 Basel III Ratios (MAS Notice 637):");
    println!(
        "   CET1 Ratio:    {:>6.2}% (minimum: 6.5%)",
        capital.cet1_ratio()
    );
    println!(
        "   Tier 1 Ratio:  {:>6.2}% (minimum: 8.0%)",
        capital.tier1_ratio()
    );
    println!(
        "   Total CAR:     {:>6.2}% (minimum: 10.0%)\n",
        capital.total_capital_ratio()
    );

    if capital.meets_regulatory_minimum() {
        println!("✅ Capital Adequacy: COMPLIANT");
        println!(
            "   CET1 buffer: {:.2} percentage points above minimum",
            capital.cet1_ratio() - 6.5
        );
        println!(
            "   Tier 1 buffer: {:.2} percentage points above minimum",
            capital.tier1_ratio() - 8.0
        );
        println!(
            "   Total buffer: {:.2} percentage points above minimum",
            capital.total_capital_ratio() - 10.0
        );
    } else {
        println!("❌ Capital Adequacy: NON-COMPLIANT");
    }

    // Create full bank
    let aml_officer = ComplianceOfficer {
        name: "Jane Tan".to_string(),
        identification: "S9876543A".to_string(),
        email: "jane.tan@sgbank.com.sg".to_string(),
        phone: "+65 6123 4567".to_string(),
        appointed_date: Utc::now(),
        qualifications: vec!["ACAMS CAMS".to_string(), "ICA Diploma".to_string()],
    };

    let bank = BankBuilder::new()
        .uen("197700001E".to_string())
        .name("Singapore Commercial Bank Ltd".to_string())
        .license_type(BankLicenseType::FullBank)
        .license_date(Utc::now())
        .locally_incorporated(true)
        .country_of_incorporation("Singapore".to_string())
        .capital_adequacy(capital)
        .aml_officer(aml_officer)
        .total_assets_sgd(50_000_000_000_00) // SGD 500M
        .total_deposits_sgd(40_000_000_000_00) // SGD 400M
        .build()
        .expect("Failed to build bank");

    println!("\n🏦 Bank Details:");
    println!("   Name: {}", bank.name);
    println!("   UEN: {}", bank.uen);
    println!("   License: {:?}", bank.license_type);
    println!("   Incorporated: {}", bank.country_of_incorporation);
    println!("   Total Assets: SGD {:.2}", bank.total_assets_in_sgd());
    println!("   Total Deposits: SGD {:.2}", bank.total_deposits_in_sgd());
    println!(
        "   Deposit-Asset Ratio: {:.2}%\n",
        bank.deposit_asset_ratio()
    );

    // Validate bank
    match validate_bank(&bank) {
        Ok(report) => {
            if report.is_compliant {
                println!("✅ Bank Validation: FULLY COMPLIANT\n");
            } else {
                println!("⚠️  Bank Validation: ISSUES DETECTED\n");
            }

            if !report.errors.is_empty() {
                println!("🚨 Errors:");
                for error in &report.errors {
                    println!("   • {}", error);
                }
                println!();
            }

            if !report.warnings.is_empty() {
                println!("⚠️  Warnings:");
                for warning in &report.warnings {
                    println!("   • {}", warning);
                }
            }
        }
        Err(e) => {
            println!("❌ Validation Error: {}\n", e);
        }
    }
}

fn example_insufficient_capital() {
    println!("📋 Bank: Undercapitalized Regional Bank");
    println!("{}\\n", "─".repeat(66));

    // Create capital adequacy data BELOW requirements
    let capital = CapitalAdequacy {
        cet1_capital_sgd: 500_000_000_00,  // SGD 5M - INSUFFICIENT (5%)
        at1_capital_sgd: 150_000_000_00,   // SGD 1.5M
        tier2_capital_sgd: 200_000_000_00, // SGD 2M
        risk_weighted_assets_sgd: 10_000_000_000_00, // SGD 100M
        calculation_date: Utc::now(),
    };

    println!("💰 Capital Structure:");
    println!(
        "   CET1 Capital:  SGD {:>12.2}",
        capital.cet1_capital_sgd_amount()
    );
    println!(
        "   AT1 Capital:   SGD {:>12.2}",
        capital.at1_capital_sgd_amount()
    );
    println!(
        "   Tier 2 Capital: SGD {:>12.2}",
        capital.tier2_capital_sgd_amount()
    );
    println!("   RWA:           SGD {:>12.2}\n", capital.rwa_sgd_amount());

    println!("📊 Basel III Ratios (MAS Notice 637):");
    println!(
        "   CET1 Ratio:    {:>6.2}% ❌ (minimum: 6.5%)",
        capital.cet1_ratio()
    );
    println!(
        "   Tier 1 Ratio:  {:>6.2}% ❌ (minimum: 8.0%)",
        capital.tier1_ratio()
    );
    println!(
        "   Total CAR:     {:>6.2}% ❌ (minimum: 10.0%)\n",
        capital.total_capital_ratio()
    );

    // Calculate shortfall
    let cet1_required = calculate_required_capital(capital.risk_weighted_assets_sgd, 6.5);
    let cet1_shortfall = calculate_capital_shortfall(
        capital.cet1_capital_sgd,
        capital.risk_weighted_assets_sgd,
        6.5,
    );

    println!("💡 Capital Shortfall Analysis:");
    println!("   CET1 Required: SGD {:.2}", cet1_required as f64 / 100.0);
    println!(
        "   CET1 Shortfall: SGD {:.2}\n",
        cet1_shortfall as f64 / 100.0
    );

    let bank = Bank::new(
        "198800002F".to_string(),
        "Regional Bank Pte Ltd".to_string(),
        BankLicenseType::FullBank,
        Utc::now(),
        true,
        "Singapore".to_string(),
        capital,
    );

    // Validate bank (will fail due to insufficient capital)
    match validate_bank(&bank) {
        Ok(report) => {
            if !report.is_compliant {
                println!("❌ Bank Validation: NON-COMPLIANT\n");
                println!("🚨 Critical Errors:");
                for error in &report.errors {
                    println!("   • {}", error);
                    println!("     Statutory Reference: {}", error.statutory_reference());
                    println!("     Severity: {}\n", error.severity());
                }
            }
        }
        Err(e) => {
            println!("❌ Validation Error: {}", e);
        }
    }

    println!("📌 Remedial Actions Required:");
    println!("   1. Raise additional CET1 capital (e.g., rights issue, retained earnings)");
    println!("   2. Reduce RWA through deleveraging or asset reallocation");
    println!("   3. Submit capital restoration plan to MAS");
    println!("   4. Restrict dividend payments until capital restored");
}

fn example_wholesale_bank() {
    println!("📋 Bank: International Wholesale Bank Singapore");
    println!("{}\\n", "─".repeat(66));

    let capital = CapitalAdequacy {
        cet1_capital_sgd: 800_000_000_00,
        at1_capital_sgd: 200_000_000_00,
        tier2_capital_sgd: 300_000_000_00,
        risk_weighted_assets_sgd: 10_000_000_000_00,
        calculation_date: Utc::now(),
    };

    let bank = Bank::new(
        "199900003G".to_string(),
        "International Wholesale Bank Singapore".to_string(),
        BankLicenseType::WholesaleBank,
        Utc::now(),
        false, // Not locally incorporated
        "United States".to_string(),
        capital,
    );

    println!("🏦 Bank Details:");
    println!("   Name: {}", bank.name);
    println!("   License Type: {:?}", bank.license_type);
    println!("   Parent Country: {}", bank.country_of_incorporation);
    println!(
        "   Locally Incorporated: {}\n",
        bank.is_locally_incorporated
    );

    println!("📋 Wholesale Bank Restrictions (Banking Act s. 4):");
    println!("   • Minimum deposit: SGD 250,000");
    println!("   • Cannot accept retail deposits below threshold");
    println!("   • Serves corporate and institutional clients only\n");

    // Test deposit validations
    println!("💵 Deposit Validation Tests:");

    // Valid deposit (above threshold)
    let valid_deposit = 500_000.0;
    match validate_wholesale_deposit(&bank.license_type, valid_deposit) {
        Ok(()) => {
            println!(
                "   ✓ Deposit SGD {:.2} - ACCEPTED (≥ SGD 250,000)",
                valid_deposit
            );
        }
        Err(e) => {
            println!("   ✗ {}", e);
        }
    }

    // Invalid deposit (below threshold)
    let invalid_deposit = 100_000.0;
    match validate_wholesale_deposit(&bank.license_type, invalid_deposit) {
        Ok(()) => {
            println!("   ✓ Deposit SGD {:.2} - ACCEPTED", invalid_deposit);
        }
        Err(e) => {
            println!("   ✗ Deposit SGD {:.2} - REJECTED: {}", invalid_deposit, e);
        }
    }

    // Edge case (exactly at threshold)
    let threshold_deposit = 250_000.0;
    match validate_wholesale_deposit(&bank.license_type, threshold_deposit) {
        Ok(()) => {
            println!(
                "   ✓ Deposit SGD {:.2} - ACCEPTED (at threshold)",
                threshold_deposit
            );
        }
        Err(e) => {
            println!("   ✗ {}", e);
        }
    }
}

fn example_aml_cdd() {
    println!("📋 Customer: High-Risk Corporate Account");
    println!("{}\\n", "─".repeat(66));

    let account = CustomerAccount {
        account_number: "ACC-2024-001234".to_string(),
        customer_name: "Global Trading Corporation Pte Ltd".to_string(),
        customer_id: "202012345A".to_string(),
        risk_category: CustomerRiskCategory::High,
        account_opened: Utc::now(),
        last_cdd_review: Utc::now(),
        edd_performed: true,
        source_of_funds_verified: true,
        beneficial_owner_identified: true,
        balance_sgd: 5_000_000_00, // SGD 50,000
    };

    println!("👤 Customer Details:");
    println!("   Account: {}", account.account_number);
    println!("   Name: {}", account.customer_name);
    println!("   Risk Category: {:?}", account.risk_category);
    println!(
        "   Balance: SGD {:.2}\n",
        account.balance_sgd as f64 / 100.0
    );

    println!("📋 AML/CFT Compliance Status (MAS Notice 626):");
    println!(
        "   Source of Funds Verified: {}",
        if account.source_of_funds_verified {
            "✓"
        } else {
            "✗"
        }
    );
    println!(
        "   Beneficial Owner Identified: {}",
        if account.beneficial_owner_identified {
            "✓"
        } else {
            "✗"
        }
    );
    println!(
        "   EDD Performed: {}",
        if account.edd_performed { "✓" } else { "✗" }
    );
    println!(
        "   CDD Review Current: {}\n",
        if !account.is_cdd_overdue(Utc::now()) {
            "✓"
        } else {
            "✗"
        }
    );

    println!("📅 CDD Review Schedule:");
    match account.risk_category {
        CustomerRiskCategory::High | CustomerRiskCategory::PoliticallyExposed => {
            println!("   Frequency: Annual (365 days)");
            println!("   Next Review: 1 year from last review");
        }
        CustomerRiskCategory::Medium => {
            println!("   Frequency: Biennial (730 days)");
            println!("   Next Review: 2 years from last review");
        }
        CustomerRiskCategory::Low => {
            println!("   Frequency: Triennial (1095 days)");
            println!("   Next Review: 3 years from last review");
        }
    }

    println!("\n✅ Validation Result:");
    match validate_customer_account(&account) {
        Ok(warnings) => {
            println!("   Status: COMPLIANT");
            if !warnings.is_empty() {
                println!("   Warnings:");
                for warning in warnings {
                    println!("      • {}", warning);
                }
            } else {
                println!("   No warnings - full compliance achieved");
            }
        }
        Err(e) => {
            println!("   Status: NON-COMPLIANT");
            println!("   Error: {}", e);
        }
    }
}

fn example_str_reporting() {
    println!("📋 STR: Structured Cash Deposits");
    println!("{}\\n", "─".repeat(66));

    let str_report = SuspiciousTransactionReport {
        reference_number: "STR-2024-001234".to_string(),
        account_number: "ACC-2024-005678".to_string(),
        customer_name: "ABC Trading Pte Ltd".to_string(),
        transaction_amount_sgd: 1_500_000_00, // SGD 15,000
        transaction_date: Utc::now(),
        filing_date: Utc::now(),
        suspicion_description:
            "Multiple cash deposits just below SGD 20,000 threshold over 5 days - possible structuring"
                .to_string(),
        transaction_proceeded: false,
    };

    println!("🚨 Suspicious Activity Details:");
    println!("   STR Reference: {}", str_report.reference_number);
    println!("   Account: {}", str_report.account_number);
    println!("   Customer: {}", str_report.customer_name);
    println!(
        "   Amount: SGD {:.2}\n",
        str_report.transaction_amount_sgd as f64 / 100.0
    );

    println!("📝 Nature of Suspicion:");
    println!("   {}\n", str_report.suspicion_description);

    println!("⏱️  Reporting Timeline:");
    let days_to_file = (str_report.filing_date - str_report.transaction_date).num_days();
    println!(
        "   Transaction Date: {}",
        str_report.transaction_date.format("%Y-%m-%d")
    );
    println!(
        "   Filing Date: {}",
        str_report.filing_date.format("%Y-%m-%d")
    );
    println!("   Days to File: {} days", days_to_file);
    println!("   Recommended Timeline: 5-10 business days");
    println!(
        "   Transaction Allowed: {}\n",
        if str_report.transaction_proceeded {
            "Yes"
        } else {
            "No (blocked)"
        }
    );

    println!("✅ STR Validation:");
    match validate_str_filing(&str_report) {
        Ok(warnings) => {
            if str_report.filed_timely() {
                println!("   ✓ Filed within recommended timeframe");
            } else {
                println!("   ✗ Filed late");
            }

            if !warnings.is_empty() {
                println!("   Warnings:");
                for warning in warnings {
                    println!("      • {}", warning);
                }
            }
        }
        Err(e) => {
            println!("   ✗ STR Filing Error: {}", e);
        }
    }

    println!("\n📌 Best Practices (MAS Notice 626):");
    println!("   • File STR as soon as suspicion arises (ideally within 5 days)");
    println!("   • Do not inform customer that STR has been filed (tipping off)");
    println!("   • Document all due diligence steps taken");
    println!("   • Senior management approval for blocking transactions");
    println!("   • Maintain STR records for at least 5 years");
}
