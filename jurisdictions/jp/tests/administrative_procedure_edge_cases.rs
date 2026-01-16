//! Administrative Procedure Act Edge Case Tests
//!
//! Comprehensive edge case testing for:
//! - Procedure validation with boundary conditions
//! - Electronic signature validation edge cases
//! - Document validation corner cases
//! - Agency-specific validation rules
//! - Date/time boundary validation
//! - UTF-8 and special character handling

use chrono::{Duration, NaiveDate, Utc};
use legalis_jp::administrative_procedure::*;
use legalis_jp::egov::{GovernmentAgency, ValidationReport};

// ============================================================================
// Applicant Edge Cases
// ============================================================================

#[test]
fn test_applicant_with_minimal_information() {
    let applicant = Applicant::individual("田中", "住所");

    assert_eq!(applicant.name, "田中");
    assert_eq!(applicant.contact.address, "住所");
    assert!(applicant.contact.phone.is_none());
    assert!(applicant.contact.email.is_none());
}

#[test]
fn test_applicant_with_unicode_characters() {
    let applicant = Applicant::individual("山田 太郎 🎌", "東京都渋谷区1-2-3 🏢")
        .with_phone("📞 03-1234-5678")
        .with_email("test@example.jp 📧");

    assert!(applicant.name.contains("🎌"));
    assert!(applicant.contact.address.contains("🏢"));
}

#[test]
fn test_applicant_with_very_long_name() {
    let long_name = "田".repeat(100);
    let applicant = Applicant::individual(&long_name, "住所");

    assert_eq!(applicant.name.len(), 300); // 100 characters × 3 bytes per 田
}

#[test]
fn test_applicant_with_empty_identification() {
    let applicant = Applicant::individual("名前", "住所").with_identification("", "");

    assert!(applicant.identification.is_some());
    let id = applicant.identification.unwrap();
    assert_eq!(id.id_type, "");
    assert_eq!(id.id_number, "");
}

#[test]
fn test_applicant_multiple_identification_calls() {
    let applicant = Applicant::individual("名前", "住所")
        .with_identification("my_number", "123456789012")
        .with_identification("passport", "AB1234567"); // Should overwrite

    let id = applicant.identification.unwrap();
    assert_eq!(id.id_type, "passport");
    assert_eq!(id.id_number, "AB1234567");
}

#[test]
fn test_corporate_applicant_with_corporate_number() {
    let applicant = Applicant::corporation("株式会社テスト", "東京都千代田区")
        .with_identification("corporate_number", "1234567890123");

    assert_eq!(applicant.applicant_type, ApplicantType::Corporation);
    assert!(applicant.identification.is_some());
}

// ============================================================================
// Document Edge Cases
// ============================================================================

#[test]
fn test_document_with_past_date() {
    let past_date = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();

    let doc = Document {
        id: "DOC-OLD".to_string(),
        title: "Old Document".to_string(),
        document_type: DocumentType::ApplicationForm,
        includes_reason_statement: false,
        created_date: past_date,
    };

    assert_eq!(doc.created_date, past_date);
}

#[test]
fn test_document_with_future_date() {
    let future_date = NaiveDate::from_ymd_opt(2099, 12, 31).unwrap();

    let doc = Document {
        id: "DOC-FUTURE".to_string(),
        title: "Future Document".to_string(),
        document_type: DocumentType::SupportingDocument,
        includes_reason_statement: false,
        created_date: future_date,
    };

    assert_eq!(doc.created_date, future_date);
}

#[test]
fn test_document_with_very_long_id() {
    let long_id = "X".repeat(1000);

    let doc = Document {
        id: long_id.clone(),
        title: "Test".to_string(),
        document_type: DocumentType::NotificationForm,
        includes_reason_statement: false,
        created_date: Utc::now().date_naive(),
    };

    assert_eq!(doc.id.len(), 1000);
}

#[test]
fn test_document_with_special_characters_in_title() {
    let doc = Document {
        id: "DOC-001".to_string(),
        title: "申請書<>&\"'\\n\\t\\r".to_string(),
        document_type: DocumentType::ApplicationForm,
        includes_reason_statement: false,
        created_date: Utc::now().date_naive(),
    };

    assert!(doc.title.contains("<>&\"'"));
}

// ============================================================================
// Procedure Builder Edge Cases
// ============================================================================

#[test]
fn test_procedure_builder_missing_required_fields() {
    let result = ProcedureBuilder::new().build();

    assert!(result.is_err());
}

#[test]
fn test_procedure_builder_missing_applicant() {
    let result = ProcedureBuilder::new()
        .procedure_id("TEST-001")
        .procedure_type(ProcedureType::Application)
        .agency(GovernmentAgency::MinistryOfJustice)
        .add_document(Document {
            id: "DOC".to_string(),
            title: "Title".to_string(),
            document_type: DocumentType::ApplicationForm,
            includes_reason_statement: false,
            created_date: Utc::now().date_naive(),
        })
        .build();

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("applicant"));
}

#[test]
fn test_procedure_builder_with_no_documents() {
    // Test that procedures can be built without documents
    // (validator may flag this as warning/error later)
    let applicant = Applicant::individual("名前", "住所");

    let result = ProcedureBuilder::new()
        .procedure_id("TEST-001")
        .procedure_type(ProcedureType::Application)
        .agency(GovernmentAgency::MinistryOfJustice)
        .applicant(applicant)
        .build();

    // Builder allows empty documents, but validator should flag it
    assert!(result.is_ok());
    let procedure = result.unwrap();
    assert_eq!(procedure.documents.len(), 0);

    // Validation should fail or warn
    let validation = validate_procedure(&procedure);
    if let Ok(report) = validation {
        assert!(!report.is_valid() || !report.warnings.is_empty());
    }
}

#[test]
fn test_procedure_builder_with_many_documents() {
    let applicant = Applicant::individual("名前", "住所");

    let mut builder = ProcedureBuilder::new()
        .procedure_id("TEST-MANY")
        .procedure_type(ProcedureType::Application)
        .agency(GovernmentAgency::MinistryOfEconomy)
        .applicant(applicant);

    // Add 100 documents
    for i in 0..100 {
        builder = builder.add_document(Document {
            id: format!("DOC-{:03}", i),
            title: format!("Document {}", i),
            document_type: DocumentType::SupportingDocument,
            includes_reason_statement: false,
            created_date: Utc::now().date_naive(),
        });
    }

    let procedure = builder.build().unwrap();
    assert_eq!(procedure.documents.len(), 100);
}

#[test]
fn test_procedure_with_zero_processing_period() {
    let applicant = Applicant::individual("名前", "住所");

    let procedure = ProcedureBuilder::new()
        .procedure_id("TEST-ZERO")
        .procedure_type(ProcedureType::Notification)
        .agency(GovernmentAgency::MinistryOfJustice)
        .applicant(applicant)
        .add_document(Document {
            id: "DOC".to_string(),
            title: "Title".to_string(),
            document_type: DocumentType::NotificationForm,
            includes_reason_statement: false,
            created_date: Utc::now().date_naive(),
        })
        .processing_period_days(0)
        .build()
        .unwrap();

    assert_eq!(procedure.processing_period_days, Some(0));
}

#[test]
fn test_procedure_with_very_long_processing_period() {
    let applicant = Applicant::individual("名前", "住所");

    let procedure = ProcedureBuilder::new()
        .procedure_id("TEST-LONG")
        .procedure_type(ProcedureType::Application)
        .agency(GovernmentAgency::MinistryOfLand)
        .applicant(applicant)
        .add_document(Document {
            id: "DOC".to_string(),
            title: "Title".to_string(),
            document_type: DocumentType::ApplicationForm,
            includes_reason_statement: false,
            created_date: Utc::now().date_naive(),
        })
        .processing_period_days(365) // 1 year
        .build()
        .unwrap();

    assert_eq!(procedure.processing_period_days, Some(365));
}

#[test]
fn test_procedure_with_empty_notes() {
    let applicant = Applicant::individual("名前", "住所");

    let procedure = ProcedureBuilder::new()
        .procedure_id("TEST-EMPTY-NOTES")
        .procedure_type(ProcedureType::Application)
        .agency(GovernmentAgency::DigitalAgency)
        .applicant(applicant)
        .add_document(Document {
            id: "DOC".to_string(),
            title: "Title".to_string(),
            document_type: DocumentType::ApplicationForm,
            includes_reason_statement: false,
            created_date: Utc::now().date_naive(),
        })
        .notes("")
        .build()
        .unwrap();

    assert_eq!(procedure.notes, Some("".to_string()));
}

#[test]
fn test_procedure_with_very_long_notes() {
    let long_notes = "備考\n".repeat(1000);
    let applicant = Applicant::individual("名前", "住所");

    let procedure = ProcedureBuilder::new()
        .procedure_id("TEST-LONG-NOTES")
        .procedure_type(ProcedureType::Application)
        .agency(GovernmentAgency::MinistryOfEnvironment)
        .applicant(applicant)
        .add_document(Document {
            id: "DOC".to_string(),
            title: "Title".to_string(),
            document_type: DocumentType::ApplicationForm,
            includes_reason_statement: false,
            created_date: Utc::now().date_naive(),
        })
        .notes(&long_notes)
        .build()
        .unwrap();

    assert!(procedure.notes.unwrap().len() >= 5000);
}

// ============================================================================
// Validation Edge Cases
// ============================================================================

#[test]
fn test_validate_notification_without_processing_period() {
    let applicant = Applicant::individual("名前", "住所");

    let procedure = ProcedureBuilder::new()
        .procedure_id("NOTIFY-001")
        .procedure_type(ProcedureType::Notification)
        .agency(GovernmentAgency::MinistryOfJustice)
        .applicant(applicant)
        .add_document(Document {
            id: "DOC".to_string(),
            title: "Title".to_string(),
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
fn test_validate_procedure_with_91_day_processing() {
    let applicant = Applicant::individual("名前", "住所");

    let procedure = ProcedureBuilder::new()
        .procedure_id("APP-LONG")
        .procedure_type(ProcedureType::Application)
        .agency(GovernmentAgency::MinistryOfEconomy)
        .applicant(applicant)
        .add_document(Document {
            id: "DOC".to_string(),
            title: "Title".to_string(),
            document_type: DocumentType::ApplicationForm,
            includes_reason_statement: false,
            created_date: Utc::now().date_naive(),
        })
        .processing_period_days(91) // Just over 90-day threshold
        .build()
        .unwrap();

    let report = validate_procedure(&procedure).unwrap();
    assert!(report.is_valid()); // Valid but should have warning
    assert!(!report.warnings.is_empty());
}

#[test]
fn test_validate_disposition_without_reason_statement() {
    let applicant = Applicant::individual("名前", "住所");

    let procedure = ProcedureBuilder::new()
        .procedure_id("DISP-001")
        .procedure_type(ProcedureType::AdministrativeDisposition)
        .agency(GovernmentAgency::MinistryOfLand)
        .applicant(applicant)
        .add_document(Document {
            id: "DOC".to_string(),
            title: "Title".to_string(),
            document_type: DocumentType::DispositionNotice,
            includes_reason_statement: false, // Missing reason statement
            created_date: Utc::now().date_naive(),
        })
        .build()
        .unwrap();

    let report = validate_procedure(&procedure).unwrap();
    assert!(!report.is_valid()); // Should fail Article 5 validation
    assert!(!report.errors.is_empty());
}

#[test]
fn test_validate_disposition_with_reason_statement() {
    let applicant = Applicant::individual("名前", "住所");

    let procedure = ProcedureBuilder::new()
        .procedure_id("DISP-002")
        .procedure_type(ProcedureType::AdministrativeDisposition)
        .agency(GovernmentAgency::MinistryOfEnvironment)
        .applicant(applicant)
        .add_document(Document {
            id: "DOC".to_string(),
            title: "Title".to_string(),
            document_type: DocumentType::DispositionNotice,
            includes_reason_statement: true, // Has reason statement
            created_date: Utc::now().date_naive(),
        })
        .build()
        .unwrap();

    let report = validate_procedure(&procedure).unwrap();
    assert!(report.is_valid());
}

// ============================================================================
// Electronic Signature Edge Cases
// ============================================================================

#[test]
fn test_certificate_expired() {
    let cert = CertificateBuilder::new()
        .issuer("Test CA")
        .subject("Test Subject")
        .valid_from(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
        .valid_until(NaiveDate::from_ymd_opt(2021, 1, 1).unwrap()) // Expired
        .serial_number("EXPIRED-CERT")
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

    assert!(result.is_err() || !report.is_valid());
}

#[test]
fn test_certificate_not_yet_valid() {
    let cert = CertificateBuilder::new()
        .issuer("Test CA")
        .subject("Test Subject")
        .valid_from(NaiveDate::from_ymd_opt(2099, 1, 1).unwrap()) // Future
        .valid_until(NaiveDate::from_ymd_opt(2100, 1, 1).unwrap())
        .serial_number("FUTURE-CERT")
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

    assert!(result.is_err() || !report.is_valid());
}

#[test]
fn test_certificate_valid_today() {
    let today = Utc::now().date_naive();
    let cert = CertificateBuilder::new()
        .issuer("Test CA")
        .subject("Test Subject")
        .valid_from(today)
        .valid_until(today + Duration::days(365))
        .serial_number("VALID-TODAY")
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
}

#[test]
fn test_certificate_expires_today() {
    let today = Utc::now().date_naive();
    let cert = CertificateBuilder::new()
        .issuer("Test CA")
        .subject("Test Subject")
        .valid_from(today - Duration::days(365))
        .valid_until(today) // Expires today
        .serial_number("EXPIRES-TODAY")
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
    validate_electronic_signature(&signature, &mut report).ok();

    // Should be valid on expiration day
    assert!(report.is_valid());
}

#[test]
fn test_all_signature_algorithms() {
    let algorithms = vec![
        SignatureAlgorithm::Rsa2048,
        SignatureAlgorithm::Rsa4096,
        SignatureAlgorithm::EcdsaP256,
        SignatureAlgorithm::EcdsaP384,
    ];

    for algo in algorithms {
        let serial = format!("CERT-{:?}", algo);
        let cert = CertificateBuilder::new()
            .issuer("Test CA")
            .subject("Test Subject")
            .valid_from(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
            .valid_until(NaiveDate::from_ymd_opt(2028, 1, 1).unwrap())
            .serial_number(&serial)
            .public_key(vec![0x04; 256])
            .build()
            .unwrap();

        let signature = SignatureBuilder::new()
            .algorithm(algo)
            .certificate(cert)
            .signature_value(vec![0xAB; 256])
            .signed_at(Utc::now())
            .build()
            .unwrap();

        let mut report = ValidationReport::new();
        let result = validate_electronic_signature(&signature, &mut report);

        assert!(result.is_ok(), "Algorithm {:?} should be valid", algo);
    }
}

#[test]
fn test_signature_with_empty_signature_value() {
    let cert = CertificateBuilder::new()
        .issuer("Test CA")
        .subject("Test Subject")
        .valid_from(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
        .valid_until(NaiveDate::from_ymd_opt(2028, 1, 1).unwrap())
        .serial_number("CERT-EMPTY-SIG")
        .public_key(vec![0x04; 256])
        .build()
        .unwrap();

    let signature_result = SignatureBuilder::new()
        .algorithm(SignatureAlgorithm::Rsa2048)
        .certificate(cert)
        .signature_value(vec![]) // Empty signature
        .signed_at(Utc::now())
        .build();

    // Implementation rejects empty signature values
    assert!(signature_result.is_err());
    if let Err(e) = signature_result {
        let msg = e.to_string();
        assert!(msg.contains("empty") || msg.contains("Signature"));
    }
}

#[test]
fn test_signature_with_very_large_signature_value() {
    let cert = CertificateBuilder::new()
        .issuer("Test CA")
        .subject("Test Subject")
        .valid_from(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
        .valid_until(NaiveDate::from_ymd_opt(2028, 1, 1).unwrap())
        .serial_number("CERT-LARGE-SIG")
        .public_key(vec![0x04; 256])
        .build()
        .unwrap();

    let signature = SignatureBuilder::new()
        .algorithm(SignatureAlgorithm::Rsa4096)
        .certificate(cert)
        .signature_value(vec![0xFF; 10000]) // Very large signature
        .signed_at(Utc::now())
        .build()
        .unwrap();

    let mut report = ValidationReport::new();
    validate_electronic_signature(&signature, &mut report).ok();

    // Just verify it doesn't panic with large data
}

// ============================================================================
// e-Gov Filing Edge Cases
// ============================================================================

#[test]
fn test_filing_service_export_xml() {
    let applicant = Applicant::individual("名前", "住所");

    let procedure = ProcedureBuilder::new()
        .procedure_id("EGOV-XML-001")
        .procedure_type(ProcedureType::Application)
        .agency(GovernmentAgency::DigitalAgency)
        .applicant(applicant)
        .add_document(Document {
            id: "DOC".to_string(),
            title: "Title".to_string(),
            document_type: DocumentType::ApplicationForm,
            includes_reason_statement: false,
            created_date: Utc::now().date_naive(),
        })
        .build()
        .unwrap();

    let filing_service = AdministrativeFilingService::new();
    let xml_result = filing_service.export_xml(&procedure);

    assert!(xml_result.is_ok());
    let xml = xml_result.unwrap();
    assert!(!xml.is_empty());
    assert!(xml.contains("<?xml"));
}

#[test]
fn test_filing_service_export_json() {
    let applicant = Applicant::individual("名前", "住所");

    let procedure = ProcedureBuilder::new()
        .procedure_id("EGOV-JSON-001")
        .procedure_type(ProcedureType::Application)
        .agency(GovernmentAgency::MinistryOfJustice)
        .applicant(applicant)
        .add_document(Document {
            id: "DOC".to_string(),
            title: "Title".to_string(),
            document_type: DocumentType::ApplicationForm,
            includes_reason_statement: false,
            created_date: Utc::now().date_naive(),
        })
        .build()
        .unwrap();

    let filing_service = AdministrativeFilingService::new();
    let json_result = filing_service.export_json(&procedure);

    assert!(json_result.is_ok());
    let json = json_result.unwrap();
    assert!(!json.is_empty());
    assert!(json.starts_with("{") || json.starts_with("["));
}

#[test]
fn test_filing_service_with_special_characters() {
    let applicant = Applicant::individual("名前<>&\"'", "住所\n\t\r");

    let procedure = ProcedureBuilder::new()
        .procedure_id("EGOV-SPECIAL")
        .procedure_type(ProcedureType::Application)
        .agency(GovernmentAgency::MinistryOfEconomy)
        .applicant(applicant)
        .add_document(Document {
            id: "DOC<>".to_string(),
            title: "Title&\"'".to_string(),
            document_type: DocumentType::ApplicationForm,
            includes_reason_statement: false,
            created_date: Utc::now().date_naive(),
        })
        .build()
        .unwrap();

    let filing_service = AdministrativeFilingService::new();
    let xml_result = filing_service.export_xml(&procedure);

    // Should handle special characters properly (escape them)
    assert!(xml_result.is_ok());
}

// ============================================================================
// Procedure Type Specific Tests
// ============================================================================

#[test]
fn test_all_procedure_types() {
    let types = vec![
        ProcedureType::Application,
        ProcedureType::Notification,
        ProcedureType::AdministrativeGuidance,
        ProcedureType::AdministrativeDisposition,
        ProcedureType::Hearing,
    ];

    for proc_type in types {
        let applicant = Applicant::individual("名前", "住所");
        let proc_id = format!("TEST-{:?}", proc_type);

        let procedure = ProcedureBuilder::new()
            .procedure_id(&proc_id)
            .procedure_type(proc_type.clone())
            .agency(GovernmentAgency::DigitalAgency)
            .applicant(applicant)
            .add_document(Document {
                id: "DOC".to_string(),
                title: "Title".to_string(),
                document_type: DocumentType::ApplicationForm,
                includes_reason_statement: true, // Include for disposition
                created_date: Utc::now().date_naive(),
            })
            .build()
            .unwrap();

        assert_eq!(procedure.procedure_type, proc_type);
    }
}

#[test]
fn test_all_government_agencies() {
    let agencies = vec![
        GovernmentAgency::DigitalAgency,
        GovernmentAgency::MinistryOfJustice,
        GovernmentAgency::MinistryOfLand,
        GovernmentAgency::MinistryOfEnvironment,
        GovernmentAgency::MinistryOfEconomy,
        GovernmentAgency::PersonalInfoCommission,
    ];

    for agency in agencies {
        let applicant = Applicant::individual("名前", "住所");
        let proc_id = format!("TEST-{:?}", agency);

        let procedure = ProcedureBuilder::new()
            .procedure_id(&proc_id)
            .procedure_type(ProcedureType::Application)
            .agency(agency)
            .applicant(applicant)
            .add_document(Document {
                id: "DOC".to_string(),
                title: "Title".to_string(),
                document_type: DocumentType::ApplicationForm,
                includes_reason_statement: false,
                created_date: Utc::now().date_naive(),
            })
            .build()
            .unwrap();

        assert_eq!(procedure.agency, agency);
    }
}

#[test]
fn test_all_document_types() {
    let doc_types = vec![
        DocumentType::ApplicationForm,
        DocumentType::NotificationForm,
        DocumentType::SupportingDocument,
        DocumentType::DispositionNotice,
        DocumentType::ReasonStatement,
        DocumentType::Other("Custom".to_string()),
    ];

    for doc_type in doc_types {
        let doc = Document {
            id: format!("DOC-{:?}", doc_type),
            title: format!("Title {:?}", doc_type),
            document_type: doc_type.clone(),
            includes_reason_statement: false,
            created_date: Utc::now().date_naive(),
        };

        assert_eq!(doc.document_type, doc_type);
    }
}
