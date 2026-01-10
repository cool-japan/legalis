//! Administrative Procedure Act + Electronic Signatures Example
//!
//! Demonstrates e-Gov electronic filing with administrative procedures
//! and electronic signatures under the Administrative Procedure Act
//! (行政手続法) and Electronic Signatures Act (電子署名法).
//!
//! Run with:
//! ```bash
//! cargo run --example administrative-procedure-filing
//! ```

use chrono::{NaiveDate, Utc};
use legalis_jp::administrative_procedure::{
    AdministrativeFilingService, Applicant, CertificateBuilder, Document, DocumentType,
    ProcedureBuilder, ProcedureType, SignatureAlgorithm, SignatureBuilder, validate_procedure,
};
use legalis_jp::egov::{GovernmentAgency, ValidationReport};

fn main() {
    println!("=== Administrative Procedure Act + e-Gov Electronic Filing ===\n");

    // Example 1: Basic application submission
    println!("📝 Example 1: Business License Application");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    example_business_license_application();
    println!();

    // Example 2: Notification (届出)
    println!("📬 Example 2: Change of Address Notification");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    example_address_change_notification();
    println!();

    // Example 3: Electronic signature validation
    println!("🔐 Example 3: Electronic Signature Validation");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    example_electronic_signature();
    println!();

    // Example 4: e-Gov XML/JSON export
    println!("💾 Example 4: e-Gov Filing Export (XML/JSON)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    example_egov_export();
    println!();

    // Example 5: Processing period validation
    println!("⏱️  Example 5: Processing Period Validation");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    example_processing_period();
}

fn example_business_license_application() {
    // Build applicant using builder pattern
    let applicant = Applicant::corporation(
        "Tokyo Innovation Inc.",
        "1-2-3 Shibuya, Shibuya-ku, Tokyo 150-0001",
    )
    .with_phone("03-1234-5678")
    .with_email("legal@tokyo-innovation.jp")
    .with_identification("corporate_number", "1234567890123");

    // Build procedure using builder pattern
    let procedure = ProcedureBuilder::new()
        .procedure_id("APP-20260109-001")
        .procedure_type(ProcedureType::Application)
        .agency(GovernmentAgency::MinistryOfEconomy)
        .applicant(applicant)
        .add_document(Document {
            id: "DOC-001".to_string(),
            title: "Business License Application Form".to_string(),
            document_type: DocumentType::ApplicationForm,
            includes_reason_statement: false,
            created_date: Utc::now().date_naive(),
        })
        .add_document(Document {
            id: "DOC-002".to_string(),
            title: "Corporate Registration Certificate".to_string(),
            document_type: DocumentType::SupportingDocument,
            includes_reason_statement: false,
            created_date: Utc::now().date_naive(),
        })
        .processing_period_days(30)
        .notes("Initial business license application for new corporation")
        .build()
        .unwrap();

    println!("Application Details:");
    println!("  ID: {}", procedure.procedure_id);
    println!(
        "  Type: {} ({})",
        procedure.procedure_type.name_en(),
        procedure.procedure_type.name_ja()
    );
    println!("  Agency: {:?}", procedure.agency);
    println!("  Applicant: {}", procedure.applicant.name);
    println!("  Documents: {} attached", procedure.documents.len());
    println!(
        "  Processing Period: {} days",
        procedure.processing_period_days.unwrap_or(0)
    );

    // Validate
    match validate_procedure(&procedure) {
        Ok(report) => {
            if report.is_valid() {
                println!("\n✅ Application is VALID");
                if !report.warnings.is_empty() {
                    println!("\n⚠️  Warnings:");
                    for warning in &report.warnings {
                        println!("  • {}", warning);
                    }
                }
            } else {
                println!("\n❌ Application is INVALID");
                for error in &report.errors {
                    println!("  • {}", error);
                }
            }
        }
        Err(e) => println!("❌ Validation error: {}", e),
    }
}

fn example_address_change_notification() {
    let applicant =
        Applicant::individual("Tanaka Taro", "5-6-7 Roppongi, Minato-ku, Tokyo 106-0032")
            .with_phone("090-1234-5678")
            .with_email("tanaka@example.jp")
            .with_identification("my_number", "123456789012");

    let procedure = ProcedureBuilder::new()
        .procedure_id("NOTIFY-20260109-001")
        .procedure_type(ProcedureType::Notification)
        .agency(GovernmentAgency::MinistryOfJustice)
        .applicant(applicant)
        .add_document(Document {
            id: "NOTIFY-DOC-001".to_string(),
            title: "Change of Address Notification".to_string(),
            document_type: DocumentType::NotificationForm,
            includes_reason_statement: false,
            created_date: Utc::now().date_naive(),
        })
        .notes("Change of residential address notification")
        .build()
        .unwrap();

    println!("Notification Details:");
    println!(
        "  Type: {} ({})",
        procedure.procedure_type.name_en(),
        procedure.procedure_type.name_ja()
    );
    println!("  Individual: {}", procedure.applicant.name);
    println!("  New Address: {}", procedure.applicant.contact.address);
    println!("  Agency: {:?}", procedure.agency);

    match validate_procedure(&procedure) {
        Ok(report) => {
            if report.is_valid() {
                println!("\n✅ Notification is VALID and ready for submission");
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }
}

fn example_electronic_signature() {
    // Create certificate using builder
    let certificate = CertificateBuilder::new()
        .issuer("Japan Certification Services")
        .subject("Yamada Ichiro")
        .valid_from(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
        .valid_until(NaiveDate::from_ymd_opt(2028, 1, 1).unwrap())
        .serial_number("CERT-2025-001")
        .public_key(vec![0x04; 256]) // Simplified public key
        .build()
        .unwrap();

    println!("Certificate Details:");
    println!("  Subject: {}", certificate.subject);
    println!("  Issuer: {}", certificate.issuer);
    println!("  Serial: {}", certificate.serial_number);
    println!("  Valid From: {}", certificate.valid_from);
    println!("  Valid Until: {}", certificate.valid_until);
    println!(
        "  Currently Valid: {}",
        if certificate.is_valid() { "Yes" } else { "No" }
    );

    // Create electronic signature
    let signature = SignatureBuilder::new()
        .algorithm(SignatureAlgorithm::Rsa2048)
        .certificate(certificate)
        .signature_value(vec![0xAB; 256]) // Simplified signature
        .signed_at(Utc::now())
        .build()
        .unwrap();

    println!("\nElectronic Signature Details:");
    println!(
        "  Algorithm: {} ({})",
        signature.signature_algorithm.name(),
        if signature.signature_algorithm.is_recommended() {
            "Recommended"
        } else {
            "Advanced"
        }
    );

    // Validate signature
    let mut report = ValidationReport::new();
    match legalis_jp::administrative_procedure::validate_electronic_signature(
        &signature,
        &mut report,
    ) {
        Ok(_) => {
            println!("\n✅ Electronic signature is VALID");
            println!("  Certificate is within validity period");
            println!("  Signature algorithm is supported");
        }
        Err(e) => {
            println!("\n❌ Electronic signature validation failed: {}", e);
        }
    }

    // Test expired certificate
    println!("\n--- Testing Expired Certificate ---");
    let expired_cert = CertificateBuilder::new()
        .issuer("Old CA")
        .subject("Test User")
        .valid_from(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
        .valid_until(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()) // Expired
        .serial_number("CERT-EXPIRED")
        .public_key(vec![0x04; 128])
        .build()
        .unwrap();

    let expired_signature = SignatureBuilder::new()
        .algorithm(SignatureAlgorithm::Rsa2048)
        .certificate(expired_cert)
        .signature_value(vec![0xCD; 256])
        .signed_at(Utc::now())
        .build()
        .unwrap();

    let mut expired_report = ValidationReport::new();
    match legalis_jp::administrative_procedure::validate_electronic_signature(
        &expired_signature,
        &mut expired_report,
    ) {
        Ok(_) => println!("✅ Signature valid"),
        Err(e) => println!("❌ Expected error: {}", e),
    }
}

fn example_egov_export() {
    // Create procedure
    let applicant =
        Applicant::corporation("Sample Corporation", "1-1-1 Chiyoda, Chiyoda-ku, Tokyo")
            .with_phone("03-9999-8888")
            .with_email("info@sample.jp")
            .with_identification("corporate_number", "9876543210123");

    let procedure = ProcedureBuilder::new()
        .procedure_id("DGOV-20260109-001")
        .procedure_type(ProcedureType::Application)
        .agency(GovernmentAgency::DigitalAgency)
        .applicant(applicant)
        .add_document(Document {
            id: "APP-DOC-001".to_string(),
            title: "Digital Service License Application".to_string(),
            document_type: DocumentType::ApplicationForm,
            includes_reason_statement: false,
            created_date: Utc::now().date_naive(),
        })
        .processing_period_days(45)
        .notes("Application for digital service provider license")
        .build()
        .unwrap();

    println!("Preparing e-Gov filing...");

    // Create filing service
    let filing_service = AdministrativeFilingService::new();

    // Export to XML (legacy format)
    match filing_service.export_xml(&procedure) {
        Ok(xml) => {
            println!("\n✅ XML Export successful");
            println!("XML length: {} bytes", xml.len());
            println!("Preview (first 200 chars):");
            let preview_len = xml.len().min(200);
            println!("{}", &xml[..preview_len]);
            if xml.len() > 200 {
                println!("...");
            }
        }
        Err(e) => println!("❌ XML export error: {}", e),
    }

    // Export to JSON (modern format)
    match filing_service.export_json(&procedure) {
        Ok(json) => {
            println!("\n✅ JSON Export successful");
            println!("JSON length: {} bytes", json.len());
            println!("Preview (first 300 chars):");
            let preview_len = json.len().min(300);
            println!("{}", &json[..preview_len]);
            if json.len() > 300 {
                println!("...");
            }
        }
        Err(e) => println!("❌ JSON export error: {}", e),
    }
}

fn example_processing_period() {
    println!("Standard Processing Period Validation (Article 7)");
    println!();

    let test_cases = vec![
        ("Quick Processing", 14),
        ("Standard Processing", 30),
        ("Extended Processing", 60),
        ("Very Long Processing", 120),
    ];

    for (name, days) in test_cases {
        let applicant = Applicant::individual("Test Applicant", "Tokyo")
            .with_phone("03-0000-0000")
            .with_email("test@example.jp")
            .with_identification("my_number", "000000000000");

        let procedure = ProcedureBuilder::new()
            .procedure_id(format!("TEST-{}", days))
            .procedure_type(ProcedureType::Application)
            .agency(GovernmentAgency::MinistryOfJustice)
            .applicant(applicant)
            .processing_period_days(days)
            .build()
            .unwrap();

        print!("{} ({} days): ", name, days);

        match validate_procedure(&procedure) {
            Ok(report) => {
                if report.is_valid() {
                    if report.warnings.is_empty() {
                        println!("✅ Valid");
                    } else {
                        println!("⚠️  Valid with warnings");
                        for warning in &report.warnings {
                            println!("    • {}", warning);
                        }
                    }
                }
            }
            Err(e) => println!("❌ Error: {}", e),
        }
    }

    println!();
    println!("📌 Note: Article 7 recommends processing periods ≤ 90 days");
    println!("   Longer periods trigger warnings but remain valid.");
}
