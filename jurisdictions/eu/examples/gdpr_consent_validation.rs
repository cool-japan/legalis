//! GDPR Consent Validation Example
//!
//! This example demonstrates how to validate GDPR consent-based data processing.

use legalis_eu::gdpr::*;

fn main() {
    println!("=== GDPR Consent Validation Example ===\n");

    // Example 1: Valid consent-based processing
    println!("Example 1: Valid Consent");
    let processing = DataProcessing::new()
        .with_controller("Acme Corporation")
        .with_purpose("Email marketing for new product announcements")
        .add_data_category(PersonalDataCategory::Regular("email address".to_string()))
        .add_data_category(PersonalDataCategory::Regular("name".to_string()))
        .with_operation(ProcessingOperation::Collection)
        .with_operation(ProcessingOperation::Storage)
        .with_operation(ProcessingOperation::Use)
        .with_lawful_basis(LawfulBasis::Consent {
            freely_given: true,
            specific: true,
            informed: true,
            unambiguous: true,
        });

    match processing.validate() {
        Ok(validation) => {
            if validation.is_compliant() {
                println!("✅ Processing is GDPR compliant");
                println!("   Lawful basis: Consent (Article 6(1)(a))");
            }
        }
        Err(e) => println!("❌ Compliance error: {}", e),
    }

    println!("\n---\n");

    // Example 2: Invalid consent (not freely given)
    println!("Example 2: Invalid Consent (Coerced)");
    let invalid_processing = DataProcessing::new()
        .with_controller("Acme Corporation")
        .with_purpose("Marketing")
        .add_data_category(PersonalDataCategory::Regular("email".to_string()))
        .with_lawful_basis(LawfulBasis::Consent {
            freely_given: false, // Consent was coerced or bundled
            specific: true,
            informed: true,
            unambiguous: true,
        });

    match invalid_processing.validate() {
        Ok(_) => println!("Processing validated"),
        Err(e) => println!("❌ Validation failed: {}", e),
    }

    println!("\n---\n");

    // Example 3: Special category data
    println!("Example 3: Special Category Data (Health)");
    let health_processing = DataProcessing::new()
        .with_controller("Hospital Management System")
        .with_purpose("Patient medical records management")
        .add_data_category(PersonalDataCategory::Regular("patient name".to_string()))
        .add_data_category(PersonalDataCategory::Special(SpecialCategory::HealthData))
        .with_lawful_basis(LawfulBasis::Consent {
            freely_given: true,
            specific: true,
            informed: true,
            unambiguous: true,
        });

    match health_processing.validate() {
        Ok(validation) => {
            if validation.requires_article9_exception {
                println!("⚠️  Special category data detected");
                println!("   Requires Article 9 exception (in addition to Article 6)");
                println!("   Possible exceptions:");
                println!("   - Article 9(2)(a): Explicit consent");
                println!("   - Article 9(2)(h): Healthcare/medical diagnosis");
                println!("   - Article 9(2)(i): Public health");
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }

    println!("\n---\n");

    // Example 4: Contract-based processing
    println!("Example 4: Contract Performance");
    let contract_processing = DataProcessing::new()
        .with_controller("Online Retailer")
        .with_purpose("Order fulfillment and delivery")
        .add_data_category(PersonalDataCategory::Regular(
            "shipping address".to_string(),
        ))
        .add_data_category(PersonalDataCategory::Regular("phone number".to_string()))
        .with_lawful_basis(LawfulBasis::Contract {
            necessary_for_performance: true,
        });

    match contract_processing.validate() {
        Ok(validation) => {
            if validation.is_compliant() {
                println!("✅ Processing is GDPR compliant");
                println!("   Lawful basis: Contract performance (Article 6(1)(b))");
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }

    println!("\n---\n");

    // Example 5: Legitimate interests (requires balancing test)
    println!("Example 5: Legitimate Interests (Balancing Test Required)");
    let li_processing = DataProcessing::new()
        .with_controller("Security Company")
        .with_purpose("Fraud detection and prevention")
        .add_data_category(PersonalDataCategory::Regular("IP address".to_string()))
        .add_data_category(PersonalDataCategory::Regular(
            "transaction history".to_string(),
        ))
        .with_lawful_basis(LawfulBasis::LegitimateInterests {
            controller_interest: "Preventing fraudulent transactions to protect customers"
                .to_string(),
            balancing_test_passed: false, // Not yet assessed
        });

    match li_processing.validate() {
        Ok(validation) => {
            use legalis_core::LegalResult;
            match &validation.lawful_basis_valid {
                LegalResult::JudicialDiscretion {
                    issue,
                    narrative_hint,
                    ..
                } => {
                    println!("⚖️  Requires human judgment");
                    println!("   Issue: {}", issue);
                    if let Some(hint) = narrative_hint {
                        println!("   Guidance: {}", hint);
                    }
                }
                _ => println!("Validation result: {:?}", validation),
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }

    println!("\n=== End of Example ===");
}
