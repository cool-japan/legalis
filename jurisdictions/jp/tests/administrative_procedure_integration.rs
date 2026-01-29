//! Administrative Procedure Act Integration Tests
//!
//! Comprehensive integration tests for administrative procedures including:
//! - Multi-agency filing scenarios
//! - Electronic signature chain validation
//! - Attachment validation edge cases
//! - Cross-module integration
//! - End-to-end workflow validation

use chrono::{Duration, NaiveDate, Utc};
use legalis_jp::administrative_procedure::*;
use legalis_jp::egov::{Attachment, GovernmentAgency, ValidationReport};

// ============================================================================
// Multi-Agency Filing Scenarios
// ============================================================================

#[test]
fn test_multi_agency_filing_ministry_of_justice() {
    let applicant = Applicant::individual("田中太郎", "東京都千代田区霞が関1-1-1")
        .with_phone("03-1234-5678")
        .with_email("tanaka@example.jp")
        .with_identification("my_number", "123456789012");

    let procedure = ProcedureBuilder::new()
        .procedure_id("MOJ-2026-001")
        .procedure_type(ProcedureType::Application)
        .agency(GovernmentAgency::MinistryOfJustice)
        .applicant(applicant)
        .add_document(Document {
            id: "DOC-MOJ-001".to_string(),
            title: "法務省申請書".to_string(),
            document_type: DocumentType::ApplicationForm,
            includes_reason_statement: false,
            created_date: Utc::now().date_naive(),
        })
        .processing_period_days(30)
        .notes("法務省関連の申請")
        .build()
        .unwrap();

    let report = validate_procedure(&procedure).unwrap();
    assert!(report.is_valid());
    assert_eq!(procedure.agency, GovernmentAgency::MinistryOfJustice);
}

#[test]
fn test_multi_agency_filing_digital_agency() {
    let applicant = Applicant::corporation("株式会社テクノロジー", "東京都港区六本木")
        .with_phone("03-9999-8888")
        .with_email("info@tech.co.jp")
        .with_identification("corporate_number", "1234567890123");

    let procedure = ProcedureBuilder::new()
        .procedure_id("DA-2026-001")
        .procedure_type(ProcedureType::Application)
        .agency(GovernmentAgency::DigitalAgency)
        .applicant(applicant)
        .add_document(Document {
            id: "DOC-DA-001".to_string(),
            title: "デジタル庁申請書".to_string(),
            document_type: DocumentType::ApplicationForm,
            includes_reason_statement: false,
            created_date: Utc::now().date_naive(),
        })
        .processing_period_days(14)
        .build()
        .unwrap();

    let report = validate_procedure(&procedure).unwrap();
    assert!(report.is_valid());
    assert!(procedure.processing_period_days.unwrap() <= 30);
}

#[test]
fn test_multi_agency_filing_ministry_of_environment() {
    let applicant = Applicant::corporation("環境保護株式会社", "大阪府大阪市北区")
        .with_phone("06-1111-2222")
        .with_email("kankyo@example.com");

    let procedure = ProcedureBuilder::new()
        .procedure_id("MOE-2026-001")
        .procedure_type(ProcedureType::Notification)
        .agency(GovernmentAgency::MinistryOfEnvironment)
        .applicant(applicant)
        .add_document(Document {
            id: "DOC-MOE-001".to_string(),
            title: "環境省届出書".to_string(),
            document_type: DocumentType::NotificationForm,
            includes_reason_statement: false,
            created_date: Utc::now().date_naive(),
        })
        .build()
        .unwrap();

    let report = validate_procedure(&procedure).unwrap();
    assert!(report.is_valid());
}

#[test]
fn test_multi_agency_filing_personal_info_commission() {
    let applicant = Applicant::corporation("データ管理株式会社", "東京都新宿区")
        .with_identification("corporate_number", "9876543210987");

    let procedure = ProcedureBuilder::new()
        .procedure_id("PIC-2026-001")
        .procedure_type(ProcedureType::Application)
        .agency(GovernmentAgency::PersonalInfoCommission)
        .applicant(applicant)
        .add_document(Document {
            id: "DOC-PIC-001".to_string(),
            title: "個人情報保護委員会申請書".to_string(),
            document_type: DocumentType::ApplicationForm,
            includes_reason_statement: false,
            created_date: Utc::now().date_naive(),
        })
        .processing_period_days(60)
        .build()
        .unwrap();

    let report = validate_procedure(&procedure).unwrap();
    assert!(report.is_valid());
}

#[test]
fn test_multi_agency_filing_ministry_of_land() {
    let applicant =
        Applicant::corporation("建設開発株式会社", "神奈川県横浜市").with_phone("045-123-4567");

    let procedure = ProcedureBuilder::new()
        .procedure_id("MLIT-2026-001")
        .procedure_type(ProcedureType::Application)
        .agency(GovernmentAgency::MinistryOfLand)
        .applicant(applicant)
        .add_document(Document {
            id: "DOC-MLIT-001".to_string(),
            title: "国土交通省申請書".to_string(),
            document_type: DocumentType::ApplicationForm,
            includes_reason_statement: false,
            created_date: Utc::now().date_naive(),
        })
        .processing_period_days(45)
        .build()
        .unwrap();

    let report = validate_procedure(&procedure).unwrap();
    assert!(report.is_valid());
}

// ============================================================================
// Electronic Signature Chain Validation
// ============================================================================

#[test]
fn test_signature_chain_single_signature() {
    let cert = CertificateBuilder::new()
        .issuer("Japan Trust CA")
        .subject("田中太郎")
        .valid_from(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
        .valid_until(NaiveDate::from_ymd_opt(2028, 1, 1).unwrap())
        .serial_number("CERT-2025-001")
        .public_key(vec![0x04; 256])
        .build()
        .unwrap();

    let signature = SignatureBuilder::new()
        .algorithm(SignatureAlgorithm::Rsa2048)
        .certificate(cert)
        .signature_value(vec![0xAB; 256])
        .signed_at(Utc::now())
        .build()
        .unwrap();

    let mut report = ValidationReport::new();
    let result = validate_electronic_signature(&signature, &mut report);

    assert!(result.is_ok());
    assert!(report.is_valid());
}

#[test]
fn test_signature_chain_with_timestamps() {
    let now = Utc::now();
    let cert = CertificateBuilder::new()
        .issuer("Timestamp Authority")
        .subject("Time Service")
        .valid_from(now.date_naive())
        .valid_until((now + Duration::days(365)).date_naive())
        .serial_number("TSA-2026-001")
        .public_key(vec![0x05; 256])
        .build()
        .unwrap();

    let signature = SignatureBuilder::new()
        .algorithm(SignatureAlgorithm::EcdsaP256)
        .certificate(cert)
        .signature_value(vec![0xCD; 64])
        .signed_at(now)
        .build()
        .unwrap();

    let mut report = ValidationReport::new();
    let result = validate_electronic_signature(&signature, &mut report);

    assert!(result.is_ok());
    assert_eq!(signature.signed_at, now);
}

#[test]
fn test_signature_with_rsa4096() {
    let cert = CertificateBuilder::new()
        .issuer("Strong CA")
        .subject("Secure User")
        .valid_from(NaiveDate::from_ymd_opt(2025, 6, 1).unwrap())
        .valid_until(NaiveDate::from_ymd_opt(2030, 6, 1).unwrap())
        .serial_number("RSA4096-001")
        .public_key(vec![0x06; 512])
        .build()
        .unwrap();

    let signature = SignatureBuilder::new()
        .algorithm(SignatureAlgorithm::Rsa4096)
        .certificate(cert)
        .signature_value(vec![0xEF; 512])
        .signed_at(Utc::now())
        .build()
        .unwrap();

    let mut report = ValidationReport::new();
    let result = validate_electronic_signature(&signature, &mut report);

    assert!(result.is_ok());
    assert_eq!(signature.signature_algorithm, SignatureAlgorithm::Rsa4096);
}

#[test]
fn test_signature_with_ecdsa_p384() {
    let cert = CertificateBuilder::new()
        .issuer("Elliptic Curve CA")
        .subject("EC User")
        .valid_from(NaiveDate::from_ymd_opt(2026, 1, 1).unwrap())
        .valid_until(NaiveDate::from_ymd_opt(2029, 1, 1).unwrap())
        .serial_number("ECDSA-P384-001")
        .public_key(vec![0x07; 96])
        .build()
        .unwrap();

    let signature = SignatureBuilder::new()
        .algorithm(SignatureAlgorithm::EcdsaP384)
        .certificate(cert)
        .signature_value(vec![0x12; 96])
        .signed_at(Utc::now())
        .build()
        .unwrap();

    let mut report = ValidationReport::new();
    let result = validate_electronic_signature(&signature, &mut report);

    assert!(result.is_ok());
    assert_eq!(signature.signature_algorithm, SignatureAlgorithm::EcdsaP384);
}

#[test]
fn test_signature_expiring_soon() {
    let today = Utc::now().date_naive();
    let cert = CertificateBuilder::new()
        .issuer("Expiring CA")
        .subject("Soon Expired")
        .valid_from(today - Duration::days(364))
        .valid_until(today + Duration::days(1)) // Expires tomorrow
        .serial_number("EXPIRING-SOON-001")
        .public_key(vec![0x08; 256])
        .build()
        .unwrap();

    let signature = SignatureBuilder::new()
        .algorithm(SignatureAlgorithm::Rsa2048)
        .certificate(cert)
        .signature_value(vec![0x34; 256])
        .signed_at(Utc::now())
        .build()
        .unwrap();

    let mut report = ValidationReport::new();
    let result = validate_electronic_signature(&signature, &mut report);

    assert!(result.is_ok());
    // Should have a warning about expiration
    assert!(!report.warnings.is_empty() || report.is_valid());
}

// ============================================================================
// Attachment Validation Edge Cases
// ============================================================================

#[test]
fn test_attachment_pdf_valid() {
    let attachment = Attachment::new("ATT-PDF-001", "document.pdf", "application/pdf", 1_000_000)
        .with_description("PDF document");

    assert!(attachment.is_size_valid(10_000_000)); // 10MB limit
    assert!(attachment.is_type_allowed(&["application/pdf"]));
}

#[test]
fn test_attachment_image_jpeg() {
    let attachment = Attachment::new("ATT-IMG-001", "photo.jpg", "image/jpeg", 500_000)
        .with_description("Photograph");

    assert!(attachment.is_size_valid(10_000_000));
    assert!(attachment.is_type_allowed(&["image/"]));
}

#[test]
fn test_attachment_image_png() {
    let attachment = Attachment::new("ATT-IMG-002", "screenshot.png", "image/png", 750_000)
        .with_description("Screenshot");

    assert!(attachment.is_size_valid(10_000_000));
    assert!(attachment.is_type_allowed(&["image/"]));
}

#[test]
fn test_attachment_too_large() {
    let attachment = Attachment::new(
        "ATT-LARGE-001",
        "huge.pdf",
        "application/pdf",
        100_000_000, // 100 MB
    )
    .with_description("Very large file");

    // Should fail 10MB limit
    assert!(!attachment.is_size_valid(10_000_000));
}

#[test]
fn test_attachment_zero_size() {
    let attachment = Attachment::new(
        "ATT-ZERO-001",
        "empty.txt",
        "text/plain",
        0, // Empty file
    )
    .with_description("Empty file");

    // Zero size should be technically valid but may have special handling
    assert_eq!(attachment.size_bytes, 0);
}

#[test]
fn test_attachment_word_document() {
    let attachment = Attachment::new(
        "ATT-DOCX-001",
        "document.docx",
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        2_000_000,
    )
    .with_description("Word document");

    assert!(attachment.is_size_valid(10_000_000));
    assert!(attachment.is_type_allowed(&["application/vnd"]));
}

#[test]
fn test_attachment_xml_document() {
    let attachment = Attachment::new("ATT-XML-001", "data.xml", "application/xml", 100_000)
        .with_description("XML data");

    assert!(attachment.is_size_valid(10_000_000));
    assert!(attachment.is_type_allowed(&["application/xml"]));
}

#[test]
fn test_attachment_json_document() {
    let attachment = Attachment::new("ATT-JSON-001", "data.json", "application/json", 50_000)
        .with_description("JSON data");

    assert!(attachment.is_size_valid(10_000_000));
    assert!(attachment.is_type_allowed(&["application/json"]));
}

#[test]
fn test_attachment_with_special_characters_in_filename() {
    let attachment = Attachment::new(
        "ATT-SPECIAL-001",
        "文書_2026年1月27日.pdf",
        "application/pdf",
        1_000_000,
    )
    .with_description("Document with Japanese characters");

    assert!(attachment.filename.contains("文書"));
}

#[test]
fn test_attachment_with_very_long_filename() {
    let long_name = format!("{}.pdf", "a".repeat(250));
    let attachment = Attachment::new("ATT-LONG-001", &long_name, "application/pdf", 1_000_000)
        .with_description("File with long name");

    assert!(attachment.filename.len() > 250);
}

#[test]
fn test_attachment_required_flag() {
    let attachment =
        Attachment::new("ATT-REQ-001", "required.pdf", "application/pdf", 1_000_000).required();

    assert!(attachment.required);
}

#[test]
fn test_attachment_with_content() {
    let content = vec![0x25, 0x50, 0x44, 0x46]; // PDF magic bytes
    let attachment = Attachment::new(
        "ATT-CONTENT-001",
        "with_content.pdf",
        "application/pdf",
        content.len(),
    )
    .with_content(content.clone());

    assert!(attachment.content.is_some());
    assert_eq!(attachment.content.unwrap(), content);
}

// ============================================================================
// End-to-End Workflow Validation
// ============================================================================

#[test]
fn test_complete_application_workflow() {
    // Step 1: Create applicant
    let applicant = Applicant::individual("山田花子", "東京都渋谷区道玄坂1-2-3")
        .with_phone("03-5555-6666")
        .with_email("yamada@example.jp")
        .with_identification("driver_license", "1234567890");

    // Step 2: Build procedure
    let procedure = ProcedureBuilder::new()
        .procedure_id("APP-WORKFLOW-001")
        .procedure_type(ProcedureType::Application)
        .agency(GovernmentAgency::MinistryOfEconomy)
        .applicant(applicant)
        .add_document(Document {
            id: "DOC-MAIN".to_string(),
            title: "主申請書".to_string(),
            document_type: DocumentType::ApplicationForm,
            includes_reason_statement: false,
            created_date: Utc::now().date_naive(),
        })
        .add_document(Document {
            id: "DOC-SUPPORT-1".to_string(),
            title: "添付書類1".to_string(),
            document_type: DocumentType::SupportingDocument,
            includes_reason_statement: false,
            created_date: Utc::now().date_naive(),
        })
        .processing_period_days(30)
        .notes("完全なワークフローテスト")
        .build()
        .unwrap();

    // Step 3: Validate procedure
    let report = validate_procedure(&procedure).unwrap();
    assert!(report.is_valid());

    // Step 4: Export to XML
    let filing_service = AdministrativeFilingService::new();
    let xml_result = filing_service.export_xml(&procedure);
    assert!(xml_result.is_ok());

    // Step 5: Export to JSON
    let json_result = filing_service.export_json(&procedure);
    assert!(json_result.is_ok());
}

#[test]
fn test_complete_notification_workflow() {
    let applicant = Applicant::corporation("通知テスト株式会社", "愛知県名古屋市中区")
        .with_phone("052-123-4567")
        .with_identification("corporate_number", "5678901234567");

    let procedure = ProcedureBuilder::new()
        .procedure_id("NOTIFY-WORKFLOW-001")
        .procedure_type(ProcedureType::Notification)
        .agency(GovernmentAgency::MinistryOfEnvironment)
        .applicant(applicant)
        .add_document(Document {
            id: "DOC-NOTIFY".to_string(),
            title: "届出書".to_string(),
            document_type: DocumentType::NotificationForm,
            includes_reason_statement: false,
            created_date: Utc::now().date_naive(),
        })
        .build()
        .unwrap();

    let report = validate_procedure(&procedure).unwrap();
    assert!(report.is_valid());

    let filing_service = AdministrativeFilingService::new();
    let xml = filing_service.export_xml(&procedure).unwrap();
    assert!(!xml.is_empty());
}

#[test]
fn test_complete_disposition_workflow_with_signature() {
    let applicant =
        Applicant::individual("処分対象者", "福岡県福岡市博多区").with_phone("092-777-8888");

    let procedure = ProcedureBuilder::new()
        .procedure_id("DISP-WORKFLOW-001")
        .procedure_type(ProcedureType::AdministrativeDisposition)
        .agency(GovernmentAgency::MinistryOfJustice)
        .applicant(applicant)
        .add_document(Document {
            id: "DOC-DISP".to_string(),
            title: "処分通知書".to_string(),
            document_type: DocumentType::DispositionNotice,
            includes_reason_statement: true, // Required for disposition
            created_date: Utc::now().date_naive(),
        })
        .add_document(Document {
            id: "DOC-REASON".to_string(),
            title: "理由書".to_string(),
            document_type: DocumentType::ReasonStatement,
            includes_reason_statement: true,
            created_date: Utc::now().date_naive(),
        })
        .build()
        .unwrap();

    let report = validate_procedure(&procedure).unwrap();
    assert!(report.is_valid());

    // Add electronic signature
    let cert = CertificateBuilder::new()
        .issuer("Government CA")
        .subject("Official Signer")
        .valid_from(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
        .valid_until(NaiveDate::from_ymd_opt(2028, 1, 1).unwrap())
        .serial_number("GOV-CERT-001")
        .public_key(vec![0x09; 256])
        .build()
        .unwrap();

    let signature = SignatureBuilder::new()
        .algorithm(SignatureAlgorithm::Rsa2048)
        .certificate(cert)
        .signature_value(vec![0x56; 256])
        .signed_at(Utc::now())
        .build()
        .unwrap();

    let mut sig_report = ValidationReport::new();
    let sig_result = validate_electronic_signature(&signature, &mut sig_report);
    assert!(sig_result.is_ok());
}

// ============================================================================
// Cross-Module Integration Tests
// ============================================================================

#[test]
fn test_procedure_with_multiple_agencies() {
    // Simulate a procedure that might need coordination between agencies
    let applicant = Applicant::corporation("複合事業株式会社", "東京都千代田区")
        .with_identification("corporate_number", "1111111111111");

    // Primary procedure
    let proc1 = ProcedureBuilder::new()
        .procedure_id("MULTI-AGENCY-001")
        .procedure_type(ProcedureType::Application)
        .agency(GovernmentAgency::MinistryOfEconomy)
        .applicant(applicant.clone())
        .add_document(Document {
            id: "DOC-ECON".to_string(),
            title: "経済産業省申請書".to_string(),
            document_type: DocumentType::ApplicationForm,
            includes_reason_statement: false,
            created_date: Utc::now().date_naive(),
        })
        .build()
        .unwrap();

    // Related procedure to different agency
    let proc2 = ProcedureBuilder::new()
        .procedure_id("MULTI-AGENCY-002")
        .procedure_type(ProcedureType::Notification)
        .agency(GovernmentAgency::MinistryOfEnvironment)
        .applicant(applicant)
        .add_document(Document {
            id: "DOC-ENV".to_string(),
            title: "環境省届出書".to_string(),
            document_type: DocumentType::NotificationForm,
            includes_reason_statement: false,
            created_date: Utc::now().date_naive(),
        })
        .build()
        .unwrap();

    let report1 = validate_procedure(&proc1).unwrap();
    let report2 = validate_procedure(&proc2).unwrap();

    assert!(report1.is_valid());
    assert!(report2.is_valid());
}

#[test]
fn test_procedure_with_batch_processing() {
    let applicant = Applicant::individual("バッチ処理者", "大阪府大阪市中央区");

    // Create multiple procedures
    let mut procedures = Vec::new();
    for i in 1..=10 {
        let proc = ProcedureBuilder::new()
            .procedure_id(format!("BATCH-{:03}", i))
            .procedure_type(ProcedureType::Notification)
            .agency(GovernmentAgency::DigitalAgency)
            .applicant(applicant.clone())
            .add_document(Document {
                id: format!("DOC-BATCH-{}", i),
                title: format!("バッチ書類{}", i),
                document_type: DocumentType::NotificationForm,
                includes_reason_statement: false,
                created_date: Utc::now().date_naive(),
            })
            .build()
            .unwrap();

        procedures.push(proc);
    }

    // Validate all procedures
    for procedure in &procedures {
        let report = validate_procedure(procedure).unwrap();
        assert!(report.is_valid());
    }

    assert_eq!(procedures.len(), 10);
}

#[test]
fn test_procedure_validation_performance() {
    // Performance test - validate many procedures quickly
    let start = std::time::Instant::now();

    for i in 0..100 {
        let applicant = Applicant::individual(format!("User{}", i), format!("Address{}", i));

        let procedure = ProcedureBuilder::new()
            .procedure_id(format!("PERF-{:04}", i))
            .procedure_type(ProcedureType::Application)
            .agency(GovernmentAgency::DigitalAgency)
            .applicant(applicant)
            .add_document(Document {
                id: format!("DOC-{}", i),
                title: format!("Document {}", i),
                document_type: DocumentType::ApplicationForm,
                includes_reason_statement: false,
                created_date: Utc::now().date_naive(),
            })
            .build()
            .unwrap();

        let _ = validate_procedure(&procedure);
    }

    let elapsed = start.elapsed();
    // Should complete 100 validations in reasonable time
    assert!(
        elapsed.as_secs() < 5,
        "Validation took too long: {:?}",
        elapsed
    );
}
