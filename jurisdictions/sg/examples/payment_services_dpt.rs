//! Payment Services Act 2019 - Digital Payment Token (DPT) Example
//!
//! This example demonstrates comprehensive validation of payment service providers
//! under Singapore's Payment Services Act 2019, with focus on Digital Payment Token
//! (cryptocurrency) services.
//!
//! # Topics Covered
//!
//! 1. **7 Payment Service Types** (PSA s. 3)
//! 2. **License Tiers** - Money-changing, SPI, MPI (PSA s. 5-6)
//! 3. **Digital Payment Token (DPT) Services** (PSA s. 13)
//! 4. **Safeguarding Requirements** (PSA s. 23)
//! 5. **KYC and Enhanced Verification** (PSA Notice PSN02)
//!
//! # Regulatory Framework
//!
//! ## Payment Services Act 2019
//!
//! Singapore brought cryptocurrency under comprehensive regulation on 28 January 2020,
//! making it one of the first countries to regulate DPT services explicitly.
//!
//! ## License Tiers
//!
//! - **Money-Changing License**: Money-changing only
//! - **Standard Payment Institution (SPI)**: Monthly volume ≤ SGD 3M
//! - **Major Payment Institution (MPI)**: Volume > SGD 3M OR multiple services

use chrono::Utc;
use legalis_sg::payment::*;

fn main() {
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║   PAYMENT SERVICES ACT 2019 - DIGITAL PAYMENT TOKENS (DPT)    ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    // Example 1: Cryptocurrency Exchange (DPT Service)
    println!("━━━ 1. CRYPTOCURRENCY EXCHANGE (DPT SERVICE) ━━━\n");
    example_crypto_exchange();

    println!("\n{}\n", "═".repeat(66));

    // Example 2: E-Wallet with Safeguarding
    println!("━━━ 2. E-WALLET - SAFEGUARDING REQUIREMENTS ━━━\n");
    example_ewallet_safeguarding();

    println!("\n{}\n", "═".repeat(66));

    // Example 3: Remittance Service Provider
    println!("━━━ 3. CROSS-BORDER REMITTANCE SERVICE ━━━\n");
    example_remittance_service();

    println!("\n{}\n", "═".repeat(66));

    // Example 4: License Tier Assessment (SPI vs MPI)
    println!("━━━ 4. LICENSE TIER ASSESSMENT (SPI vs MPI) ━━━\n");
    example_license_tier_assessment();

    println!("\n{}\n", "═".repeat(66));

    // Example 5: Customer KYC and Enhanced Verification
    println!("━━━ 5. CUSTOMER KYC & ENHANCED VERIFICATION ━━━\n");
    example_customer_kyc();

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("✅ All payment services validation examples completed");
    println!("═══════════════════════════════════════════════════════════════\n");
}

fn example_crypto_exchange() {
    println!("📋 Provider: Singapore Crypto Exchange Pte Ltd");
    println!("{}\\n", "─".repeat(66));

    let provider = PaymentServiceProviderBuilder::new()
        .uen("202098765B".to_string())
        .name("Singapore Crypto Exchange Pte Ltd".to_string())
        .license_type(PaymentLicenseType::MajorPaymentInstitution)
        .license_date(Utc::now())
        .add_service(PaymentServiceType::DigitalPaymentToken)
        .add_dpt_service(DptServiceType::Exchange)
        .add_dpt_service(DptServiceType::Custody)
        .monthly_volume_sgd(500_000_000) // SGD 5M/month
        .has_aml_officer(true)
        .build()
        .expect("Failed to build payment provider");

    println!("🏢 Provider Details:");
    println!("   Name: {}", provider.name);
    println!("   UEN: {}", provider.uen);
    println!("   License Type: {:?}", provider.license_type);
    println!(
        "   Monthly Volume: SGD {:.2}\n",
        provider.monthly_volume_in_sgd()
    );

    println!("💱 DPT Services Provided (PSA s. 13):");
    for service in &provider.dpt_services {
        match service {
            DptServiceType::Dealing => {
                println!("   • Dealing in DPTs (buying/selling cryptocurrencies)")
            }
            DptServiceType::Exchange => println!("   • Facilitating exchange of DPTs"),
            DptServiceType::Custody => {
                println!("   • DPT wallet custody and administration")
            }
        }
    }
    println!();

    println!("📋 Regulatory Requirements:");
    println!("   ✓ MPI License Required (volume > SGD 3M/month)");
    println!("   ✓ AML/CFT Compliance Officer Appointed");
    println!("   ✓ Enhanced Due Diligence for DPT customers");
    println!("   ✓ Technology Risk Management (PSA Notice PSN03)");
    println!("   ✓ Customer Disclosure of Risks\n");

    // Validate provider
    println!("✅ Provider Validation:");
    match validate_payment_provider(&provider) {
        Ok(report) => {
            if report.is_compliant {
                println!("   Status: FULLY COMPLIANT\n");

                println!("   License Status:");
                println!("      • Type: {}", report.license_status.license_type);
                println!(
                    "      • Appropriate for Volume: {}",
                    if report.license_status.appropriate_for_volume {
                        "Yes"
                    } else {
                        "No"
                    }
                );
                println!(
                    "      • Authorized Services: {}\n",
                    report.license_status.authorized_services
                );

                println!("   AML/CFT Status:");
                println!(
                    "      • Compliance Officer: {}",
                    if report.aml_status.has_officer {
                        "Appointed"
                    } else {
                        "Not Appointed"
                    }
                );
            } else {
                println!("   Status: NON-COMPLIANT\n");
                for error in &report.errors {
                    println!("   • Error: {}", error);
                }
            }

            if !report.warnings.is_empty() {
                println!("\n   ⚠️  Warnings:");
                for warning in &report.warnings {
                    println!("      • {}", warning);
                }
            }
        }
        Err(e) => {
            println!("   ✗ Validation Error: {}\n", e);
        }
    }

    // Validate DPT-specific requirements
    println!("🔐 DPT-Specific Validation (PSA Notice PSN02):");
    match validate_dpt_service(&provider) {
        Ok(warnings) => {
            println!("   ✓ DPT service authorization confirmed");
            println!("   ✓ AML/CFT measures in place");

            if !warnings.is_empty() {
                println!("\n   ⚠️  Advisory:");
                for warning in warnings {
                    println!("      • {}", warning);
                }
            }
        }
        Err(e) => {
            println!("   ✗ DPT Compliance Error: {}", e);
        }
    }

    println!("\n💡 DPT Provider Best Practices:");
    println!("   • Segregate customer crypto assets from company assets");
    println!("   • Implement cold storage for majority of crypto holdings");
    println!("   • Multi-signature wallet controls");
    println!("   • Regular security audits and penetration testing");
    println!("   • Cyber insurance coverage");
    println!("   • Clear disclosure of risks to customers");
}

fn example_ewallet_safeguarding() {
    println!("📋 Provider: PaySG E-Wallet Pte Ltd");
    println!("{}\\n", "─".repeat(66));

    let provider = PaymentServiceProviderBuilder::new()
        .uen("202012345A".to_string())
        .name("PaySG E-Wallet Pte Ltd".to_string())
        .license_type(PaymentLicenseType::StandardPaymentInstitution)
        .license_date(Utc::now())
        .add_service(PaymentServiceType::EMoneyIssuance)
        .add_service(PaymentServiceType::AccountIssuance)
        .monthly_volume_sgd(200_000_000) // SGD 2M/month
        .float_outstanding_sgd(50_000_000) // SGD 500k outstanding
        .safeguarding_enabled(true)
        .has_aml_officer(true)
        .build()
        .expect("Failed to build payment provider");

    println!("🏢 Provider Details:");
    println!("   Name: {}", provider.name);
    println!("   License Type: {:?}", provider.license_type);
    println!(
        "   Monthly Volume: SGD {:.2}",
        provider.monthly_volume_in_sgd()
    );
    println!("   Float Outstanding: SGD {:.2}\n", provider.float_in_sgd());

    println!("💰 Safeguarding Requirements (PSA s. 23):");

    // Calculate required safeguarding for e-money (110%)
    let required_emoney = calculate_required_safeguarding(
        provider.float_outstanding_sgd,
        &PaymentServiceType::EMoneyIssuance,
    );

    println!("   Service: E-Money Issuance");
    println!(
        "   Required: SGD {:.2} (110% of float)",
        required_emoney as f64 / 100.0
    );
    println!("   Float: SGD {:.2}\n", provider.float_in_sgd());

    // Create safeguarding arrangement
    let arrangement = SafeguardingArrangement {
        arrangement_type: SafeguardingType::TrustAccount,
        institution_name: "DBS Bank Ltd".to_string(),
        reference: "TRUST-ACC-202400123".to_string(),
        amount_safeguarded_sgd: 55_000_000, // SGD 550k (110%)
        established_date: Utc::now(),
        last_verified: Utc::now(),
    };

    println!("🏦 Safeguarding Arrangement:");
    println!("   Type: {:?}", arrangement.arrangement_type);
    println!("   Institution: {}", arrangement.institution_name);
    println!("   Reference: {}", arrangement.reference);
    println!(
        "   Amount Safeguarded: SGD {:.2}",
        arrangement.amount_in_sgd()
    );
    println!(
        "   Last Verified: {}\n",
        arrangement.last_verified.format("%Y-%m-%d")
    );

    // Validate safeguarding
    println!("✅ Safeguarding Validation:");
    match validate_safeguarding(&provider, &arrangement) {
        Ok(warnings) => {
            println!("   ✓ Safeguarding amount sufficient");
            println!("   ✓ Annual verification current");

            if !warnings.is_empty() {
                println!("\n   ⚠️  Warnings:");
                for warning in warnings {
                    println!("      • {}", warning);
                }
            }
        }
        Err(e) => {
            println!("   ✗ Safeguarding Error: {}", e);
        }
    }

    println!("\n📋 Safeguarding Methods (PSA s. 23):");
    println!("   1. Trust Account - Funds held in trust with licensed bank");
    println!("   2. Statutory Deposit - Segregated deposit with MAS");
    println!("   3. Insurance/Guarantee - With MAS approval");
    println!("\n   Note: E-money requires 110%, other services 100% of float");
}

fn example_remittance_service() {
    println!("📋 Provider: GlobalRemit Singapore Pte Ltd");
    println!("{}\\n", "─".repeat(66));

    let provider = PaymentServiceProviderBuilder::new()
        .uen("202156789C".to_string())
        .name("GlobalRemit Singapore Pte Ltd".to_string())
        .license_type(PaymentLicenseType::StandardPaymentInstitution)
        .license_date(Utc::now())
        .add_service(PaymentServiceType::CrossBorderMoneyTransfer)
        .monthly_volume_sgd(250_000_000) // SGD 2.5M/month
        .float_outstanding_sgd(30_000_000) // SGD 300k
        .safeguarding_enabled(false) // Cross-border may not require safeguarding
        .has_aml_officer(true)
        .build()
        .expect("Failed to build payment provider");

    println!("🏢 Provider Details:");
    println!("   Name: {}", provider.name);
    println!("   Service: Cross-Border Money Transfer (Remittance)");
    println!(
        "   Monthly Volume: SGD {:.2}\n",
        provider.monthly_volume_in_sgd()
    );

    // Create a sample cross-border transaction
    let transaction = PaymentTransaction {
        transaction_id: "TXN-2024-001234".to_string(),
        service_type: PaymentServiceType::CrossBorderMoneyTransfer,
        sender: "John Tan (S1234567A)".to_string(),
        recipient: "Family Member (Philippines)".to_string(),
        amount_sgd: 250_000, // SGD 2,500
        currency: Some("PHP".to_string()),
        timestamp: Utc::now(),
        is_cross_border: true,
        originating_country: Some("Singapore".to_string()),
        beneficiary_country: Some("Philippines".to_string()),
    };

    println!("💸 Sample Transaction:");
    println!("   Transaction ID: {}", transaction.transaction_id);
    println!("   Sender: {}", transaction.sender);
    println!("   Recipient: {}", transaction.recipient);
    println!("   Amount: SGD {:.2}", transaction.amount_in_sgd());
    println!("   Currency: {}", transaction.currency.as_ref().unwrap());
    println!(
        "   Origin: {}",
        transaction.originating_country.as_ref().unwrap()
    );
    println!(
        "   Destination: {}\n",
        transaction.beneficiary_country.as_ref().unwrap()
    );

    // Validate transaction
    println!("✅ Transaction Validation:");
    match validate_transaction(&transaction) {
        Ok(warnings) => {
            println!("   ✓ Transaction compliant");

            if transaction.exceeds_reporting_threshold() {
                println!(
                    "   ⚠️  Exceeds reporting threshold (≥ SGD 5,000) - ensure proper records"
                );
            }

            if !warnings.is_empty() {
                for warning in warnings {
                    println!("   ⚠️  {}", warning);
                }
            }
        }
        Err(e) => {
            println!("   ✗ Transaction Error: {}", e);
        }
    }

    println!("\n📋 Cross-Border Remittance Compliance:");
    println!("   • Source of funds verification");
    println!("   • Purpose of remittance documentation");
    println!("   • Beneficiary identification");
    println!("   • FATF Travel Rule compliance (sender/recipient info)");
    println!("   • Sanctions screening (OFAC, UN, MAS lists)");
}

fn example_license_tier_assessment() {
    println!("📋 Scenario: Growing E-Wallet Provider");
    println!("{}\\n", "─".repeat(66));

    // Scenario 1: SPI-eligible provider
    println!("📊 Scenario 1: Small Provider (SPI-eligible)\n");

    let small_provider = PaymentServiceProviderBuilder::new()
        .uen("202234567D".to_string())
        .name("StartupPay Pte Ltd".to_string())
        .license_type(PaymentLicenseType::StandardPaymentInstitution)
        .license_date(Utc::now())
        .add_service(PaymentServiceType::EMoneyIssuance)
        .monthly_volume_sgd(150_000_000) // SGD 1.5M/month
        .has_aml_officer(true)
        .build()
        .unwrap();

    println!("   Provider: {}", small_provider.name);
    println!(
        "   Monthly Volume: SGD {:.2}",
        small_provider.monthly_volume_in_sgd()
    );
    println!("   Services: {} type", small_provider.services.len());
    println!("   Current License: {:?}\n", small_provider.license_type);

    if small_provider.requires_mpi_license() {
        println!("   ⚠️  MPI License Required");
    } else {
        println!("   ✓ SPI License Appropriate");
    }

    // Scenario 2: Provider exceeding SPI threshold
    println!("\n📊 Scenario 2: Growing Provider (MPI required)\n");

    let growing_provider = PaymentServiceProviderBuilder::new()
        .uen("202234567D".to_string())
        .name("StartupPay Pte Ltd".to_string())
        .license_type(PaymentLicenseType::StandardPaymentInstitution) // Still SPI
        .license_date(Utc::now())
        .add_service(PaymentServiceType::EMoneyIssuance)
        .monthly_volume_sgd(350_000_000) // SGD 3.5M/month - EXCEEDS threshold!
        .has_aml_officer(true)
        .build()
        .unwrap();

    println!("   Provider: {}", growing_provider.name);
    println!(
        "   Monthly Volume: SGD {:.2}",
        growing_provider.monthly_volume_in_sgd()
    );
    println!("   MPI Threshold: SGD 3,000,000/month");
    println!("   Current License: {:?}\n", growing_provider.license_type);

    match validate_payment_provider(&growing_provider) {
        Ok(report) => {
            if !report.is_compliant {
                println!("   ❌ License Non-Compliant:");
                for error in &report.errors {
                    println!("      • {}", error);
                }
                println!("\n   📌 Action Required:");
                println!("      • Apply for MPI license upgrade with MAS");
                println!("      • Enhanced compliance requirements");
                println!("      • Additional capital requirements");
                println!("      • More frequent reporting to MAS");
            }
        }
        Err(e) => {
            println!("   ✗ Validation Error: {}", e);
        }
    }

    println!("\n📋 License Tier Comparison (PSA s. 5):");
    println!("\n   Standard Payment Institution (SPI):");
    println!("      • Monthly volume ≤ SGD 3,000,000");
    println!("      • Single payment service type");
    println!("      • Standard compliance requirements");
    println!("\n   Major Payment Institution (MPI):");
    println!("      • Monthly volume > SGD 3,000,000 OR");
    println!("      • Multiple payment service types");
    println!("      • Enhanced compliance requirements");
    println!("      • Technology risk management (PSA Notice PSN03)");
    println!("      • More frequent MAS reporting");
}

fn example_customer_kyc() {
    println!("📋 Customer: High-Value E-Wallet Account");
    println!("{}\\n", "─".repeat(66));

    let customer = CustomerPaymentAccount {
        account_id: "EWALLET-2024-123456".to_string(),
        customer_name: "Alice Wong".to_string(),
        customer_id: "S9876543B".to_string(),
        account_type: PaymentAccountType::EWallet,
        balance_sgd: 800_000, // SGD 8,000 - exceeds SGD 5,000 threshold
        opened_date: Utc::now(),
        kyc_completed: true,
        risk_category: RiskCategory::Medium,
        is_verified: true,
    };

    println!("👤 Customer Details:");
    println!("   Account ID: {}", customer.account_id);
    println!("   Name: {}", customer.customer_name);
    println!("   NRIC: {}", customer.customer_id);
    println!("   Account Type: {:?}", customer.account_type);
    println!("   Balance: SGD {:.2}\n", customer.balance_in_sgd());

    println!("🔐 KYC/Verification Status (PSA Notice PSN02):");
    println!(
        "   Basic KYC: {}",
        if customer.kyc_completed {
            "✓ Completed"
        } else {
            "✗ Not Completed"
        }
    );
    println!("   Risk Category: {:?}", customer.risk_category);
    println!(
        "   Enhanced Verification: {}",
        if customer.is_verified {
            "✓ Completed"
        } else {
            "✗ Required"
        }
    );

    println!("\n📋 Verification Requirements:");
    if customer.requires_enhanced_verification() {
        println!("   ⚠️  Enhanced verification REQUIRED:");
        println!("      • Balance exceeds SGD 5,000 threshold");
        println!("      • Additional identity verification needed");
        println!("      • Source of funds documentation");
        println!("      • Ongoing transaction monitoring");
    } else {
        println!("   ✓ Standard KYC sufficient");
    }

    // Validate customer account
    println!("\n✅ Account Validation:");
    match validate_customer_account(&customer) {
        Ok(warnings) => {
            println!("   Status: COMPLIANT");

            if !warnings.is_empty() {
                println!("   ⚠️  Advisories:");
                for warning in warnings {
                    println!("      • {}", warning);
                }
            } else {
                println!("   All verification requirements met");
            }
        }
        Err(e) => {
            println!("   Status: NON-COMPLIANT");
            println!("   Error: {}", e);
        }
    }

    println!("\n💡 Enhanced Verification Process (SGD 5,000+ threshold):");
    println!("   1. Government-issued ID verification (NRIC/Passport)");
    println!("   2. Proof of residential address (< 3 months old)");
    println!("   3. Source of funds documentation");
    println!("   4. Face verification (liveness check)");
    println!("   5. Ongoing transaction monitoring");
    println!("   6. Annual review of customer information");
}
