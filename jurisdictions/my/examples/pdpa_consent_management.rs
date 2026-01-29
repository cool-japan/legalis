//! Example: PDPA consent management.

use legalis_my::data_protection::*;

fn main() {
    println!("=== PDPA Consent Management Example ===\n");

    // Create consent record
    let consent = ConsentRecord::builder()
        .data_subject_id("customer@example.com")
        .purpose(PurposeOfCollection::Marketing)
        .consent_method(ConsentMethod::Written)
        .add_data_category(PersonalDataCategory::Name)
        .add_data_category(PersonalDataCategory::ContactInfo)
        .add_data_category(PersonalDataCategory::IcNumber)
        .notice_given(true)
        .build()
        .expect("Valid consent");

    println!("Consent Record ID: {}", consent.id);
    println!("Data Subject: {}", consent.data_subject_id);
    println!("Purpose: {:?}", consent.purpose);
    println!("Method: {:?}", consent.consent_method);
    println!("Notice Given: {}", consent.notice_given);
    println!(
        "Data Categories: {} categories",
        consent.data_categories.len()
    );

    // Validate consent
    match consent.validate() {
        Ok(()) => println!("\n✅ Consent is valid under PDPA 2010"),
        Err(e) => println!("\n❌ Consent validation failed: {}", e),
    }

    // Organisation compliance check
    println!("\n=== Organisation Compliance Check ===");
    let org = PdpaOrganisation::new("Tech Sdn Bhd", "201601012345")
        .with_dpo(true)
        .with_policy(true);

    let report = validate_organisation_compliance(&org);
    println!("Organisation: {}", org.name);
    println!(
        "Compliance Status: {}",
        if report.compliant {
            "✅ Compliant"
        } else {
            "❌ Non-compliant"
        }
    );

    if !report.issues.is_empty() {
        println!("\nIssues:");
        for issue in report.issues {
            println!("  - {}", issue);
        }
    }

    if !report.recommendations.is_empty() {
        println!("\nRecommendations:");
        for rec in report.recommendations {
            println!("  - {}", rec);
        }
    }
}
