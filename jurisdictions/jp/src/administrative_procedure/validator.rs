//! Administrative Procedure Validator
//!
//! Validation logic for administrative procedures under Japanese law,
//! including Article 5 (reason statement), Article 7 (processing period),
//! and electronic signature validation.

use crate::administrative_procedure::{
    error::{AdministrativeError, Result},
    types::{AdministrativeProcedure, ElectronicSignature, ProcedureType, SignatureAlgorithm},
};
use crate::egov::ValidationReport;

/// Validate an administrative procedure
pub fn validate_procedure(procedure: &AdministrativeProcedure) -> Result<ValidationReport> {
    let mut report = ValidationReport::new();

    // Basic field validation
    validate_basic_fields(procedure, &mut report);

    // Article 5: Reason statement requirement
    validate_reason_statement(procedure, &mut report);

    // Article 7: Standard processing period
    validate_processing_period(procedure, &mut report);

    // Electronic signature validation
    if let Some(signature) = &procedure.electronic_signature {
        validate_electronic_signature(signature, &mut report)?;
    }

    // Document validation
    validate_documents(procedure, &mut report);

    Ok(report)
}

/// Validate basic required fields
fn validate_basic_fields(procedure: &AdministrativeProcedure, report: &mut ValidationReport) {
    if procedure.procedure_id.is_empty() {
        report.add_error("Procedure ID is required (手続IDが必要です)");
    }

    if procedure.applicant.name.is_empty() {
        report.add_error("Applicant name is required (申請者名が必要です)");
    }

    if procedure.applicant.contact.address.is_empty() {
        report.add_error("Address is required (住所が必要です)");
    }
}

/// Validate reason statement requirement (Article 5)
///
/// Article 5 of the Administrative Procedure Act requires that when an administrative
/// agency makes a disposition that adversely affects the rights or interests of a party,
/// it must simultaneously present the reasons for the disposition.
fn validate_reason_statement(procedure: &AdministrativeProcedure, report: &mut ValidationReport) {
    if matches!(
        procedure.procedure_type,
        ProcedureType::AdministrativeDisposition
    ) && !procedure.has_reason_statement()
    {
        report.add_error(
            "Reason statement required for disposition (処分には理由の提示が必要です - 第5条)",
        );
    }
}

/// Validate standard processing period (Article 7)
///
/// Article 7 requires administrative agencies to set a standard processing period
/// for applications. Typical period is 30-90 days depending on complexity.
fn validate_processing_period(procedure: &AdministrativeProcedure, report: &mut ValidationReport) {
    const RECOMMENDED_MAX_PERIOD: u32 = 90;
    const TYPICAL_PERIOD: u32 = 30;

    if matches!(procedure.procedure_type, ProcedureType::Application) {
        if let Some(period) = procedure.processing_period_days {
            if period > RECOMMENDED_MAX_PERIOD {
                report.add_warning(format!(
                    "Processing period {} days exceeds typical maximum of {} days (標準処理期間 - 第7条)",
                    period, RECOMMENDED_MAX_PERIOD
                ));
            } else if period > TYPICAL_PERIOD {
                report.add_warning(format!(
                    "Processing period {} days is longer than typical {} days (標準処理期間)",
                    period, TYPICAL_PERIOD
                ));
            }
        } else {
            report.add_warning(
                "Standard processing period not set (標準処理期間が設定されていません - 第7条推奨)",
            );
        }
    }
}

/// Validate electronic signature (電子署名法)
pub fn validate_electronic_signature(
    signature: &ElectronicSignature,
    report: &mut ValidationReport,
) -> Result<()> {
    // Check certificate validity period
    if !signature.is_certificate_valid() {
        let cert = &signature.certificate;
        if cert.is_expired() {
            return Err(AdministrativeError::CertificateValidityError {
                reason: format!(
                    "Certificate expired on {} (証明書の有効期限切れ)",
                    cert.valid_until
                ),
            });
        } else if cert.is_not_yet_valid() {
            return Err(AdministrativeError::CertificateValidityError {
                reason: format!(
                    "Certificate not yet valid until {} (証明書の有効期間前)",
                    cert.valid_from
                ),
            });
        }
    }

    // Warn if certificate expires soon (within 30 days)
    let days_until_expiration = signature.certificate.days_until_expiration();
    if days_until_expiration > 0 && days_until_expiration < 30 {
        report.add_warning(format!(
            "Certificate expires in {} days (証明書の有効期限が近い)",
            days_until_expiration
        ));
    }

    // Check signature algorithm
    if !signature.signature_algorithm.is_recommended() {
        report.add_warning(format!(
            "Signature algorithm {} is not recommended (推奨されない署名アルゴリズム)",
            signature.signature_algorithm.name()
        ));
    }

    // Check signature value is not empty
    if signature.signature_value.is_empty() {
        report.add_error("Signature value is empty (署名値が空)");
    }

    Ok(())
}

/// Validate documents
fn validate_documents(procedure: &AdministrativeProcedure, report: &mut ValidationReport) {
    if procedure.documents.is_empty() {
        report.add_warning("No documents attached (添付書類なし)");
    }

    for document in &procedure.documents {
        if document.title.is_empty() {
            report.add_error(format!(
                "Document {} has no title (書類タイトルが空)",
                document.id
            ));
        }
    }
}

/// Quick validation check (returns true if valid)
pub fn quick_validate(procedure: &AdministrativeProcedure) -> bool {
    validate_procedure(procedure)
        .map(|report| report.is_valid())
        .unwrap_or(false)
}

/// Validate signature algorithm is acceptable
pub fn validate_signature_algorithm(algorithm: SignatureAlgorithm) -> Result<()> {
    // All currently defined algorithms are acceptable
    // In the future, we might deprecate older algorithms
    match algorithm {
        SignatureAlgorithm::Rsa2048
        | SignatureAlgorithm::Rsa4096
        | SignatureAlgorithm::EcdsaP256
        | SignatureAlgorithm::EcdsaP384 => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::administrative_procedure::types::{
        Applicant, Certificate, Document, DocumentType, ElectronicSignature,
    };
    use crate::egov::GovernmentAgency;
    use chrono::{Duration, Utc};

    fn create_test_procedure() -> AdministrativeProcedure {
        let applicant = Applicant::individual("Test User", "Tokyo, Japan");
        AdministrativeProcedure::new(
            "PROC-TEST-001",
            ProcedureType::Application,
            GovernmentAgency::DigitalAgency,
            applicant,
        )
    }

    #[test]
    fn test_validate_valid_procedure() {
        let procedure = create_test_procedure();
        let report = validate_procedure(&procedure).unwrap();

        // May have warnings but should have no errors
        assert!(report.is_valid());
    }

    #[test]
    fn test_validate_missing_reason_statement() {
        let applicant = Applicant::individual("Test User", "Tokyo");
        let procedure = AdministrativeProcedure::new(
            "PROC-002",
            ProcedureType::AdministrativeDisposition,
            GovernmentAgency::MinistryOfJustice,
            applicant,
        );

        let report = validate_procedure(&procedure).unwrap();
        assert!(!report.is_valid());
        assert!(report.errors.iter().any(|e| e.contains("第5条")));
    }

    #[test]
    fn test_validate_with_reason_statement() {
        let applicant = Applicant::individual("Test User", "Tokyo");
        let mut procedure = AdministrativeProcedure::new(
            "PROC-003",
            ProcedureType::AdministrativeDisposition,
            GovernmentAgency::MinistryOfJustice,
            applicant,
        );

        let document = Document {
            id: "DOC-001".to_string(),
            title: "Reason Statement".to_string(),
            document_type: DocumentType::ReasonStatement,
            includes_reason_statement: true,
            created_date: Utc::now().date_naive(),
        };

        procedure.add_document(document);

        let report = validate_procedure(&procedure).unwrap();
        assert!(report.is_valid());
    }

    #[test]
    fn test_validate_processing_period() {
        let mut procedure = create_test_procedure();

        // No processing period set - should get warning
        let report1 = validate_procedure(&procedure).unwrap();
        assert!(report1.warnings.iter().any(|w| w.contains("第7条")));

        // Set reasonable processing period
        procedure.set_processing_period(30);
        let report2 = validate_procedure(&procedure).unwrap();
        assert!(report2.is_valid());

        // Set excessive processing period
        procedure.set_processing_period(120);
        let report3 = validate_procedure(&procedure).unwrap();
        assert!(report3.warnings.iter().any(|w| w.contains("90 days")));
    }

    #[test]
    fn test_validate_expired_certificate() {
        let expired_cert = Certificate {
            issuer: "Test CA".to_string(),
            subject: "Test User".to_string(),
            valid_from: Utc::now().date_naive() - Duration::days(400),
            valid_until: Utc::now().date_naive() - Duration::days(1),
            public_key: vec![],
            serial_number: "123".to_string(),
        };

        let signature =
            ElectronicSignature::new(SignatureAlgorithm::Rsa2048, expired_cert, vec![1, 2, 3]);

        let mut report = ValidationReport::new();
        let result = validate_electronic_signature(&signature, &mut report);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AdministrativeError::CertificateValidityError { .. }
        ));
    }

    #[test]
    fn test_validate_valid_certificate() {
        let valid_cert = Certificate {
            issuer: "Test CA".to_string(),
            subject: "Test User".to_string(),
            valid_from: Utc::now().date_naive(),
            valid_until: Utc::now().date_naive() + Duration::days(365),
            public_key: vec![1, 2, 3],
            serial_number: "123".to_string(),
        };

        let signature =
            ElectronicSignature::new(SignatureAlgorithm::Rsa2048, valid_cert, vec![1, 2, 3]);

        let mut report = ValidationReport::new();
        let result = validate_electronic_signature(&signature, &mut report);

        assert!(result.is_ok());
        assert!(report.is_valid());
    }

    #[test]
    fn test_validate_expiring_soon_certificate() {
        let expiring_cert = Certificate {
            issuer: "Test CA".to_string(),
            subject: "Test User".to_string(),
            valid_from: Utc::now().date_naive() - Duration::days(335),
            valid_until: Utc::now().date_naive() + Duration::days(15),
            public_key: vec![],
            serial_number: "123".to_string(),
        };

        let signature =
            ElectronicSignature::new(SignatureAlgorithm::EcdsaP256, expiring_cert, vec![1, 2, 3]);

        let mut report = ValidationReport::new();
        let result = validate_electronic_signature(&signature, &mut report);

        assert!(result.is_ok());
        assert!(report.warnings.iter().any(|w| w.contains("expires in")));
    }

    #[test]
    fn test_quick_validate() {
        let procedure = create_test_procedure();
        assert!(quick_validate(&procedure));

        let applicant = Applicant::individual("", ""); // Invalid
        let invalid_procedure = AdministrativeProcedure::new(
            "",
            ProcedureType::Application,
            GovernmentAgency::DigitalAgency,
            applicant,
        );
        assert!(!quick_validate(&invalid_procedure));
    }

    #[test]
    fn test_validate_signature_algorithm() {
        assert!(validate_signature_algorithm(SignatureAlgorithm::Rsa2048).is_ok());
        assert!(validate_signature_algorithm(SignatureAlgorithm::EcdsaP256).is_ok());
        assert!(validate_signature_algorithm(SignatureAlgorithm::Rsa4096).is_ok());
    }

    #[test]
    fn test_validate_empty_signature_value() {
        let cert = Certificate {
            issuer: "Test CA".to_string(),
            subject: "Test User".to_string(),
            valid_from: Utc::now().date_naive(),
            valid_until: Utc::now().date_naive() + Duration::days(365),
            public_key: vec![],
            serial_number: "123".to_string(),
        };

        let signature = ElectronicSignature::new(
            SignatureAlgorithm::Rsa2048,
            cert,
            vec![], // Empty signature value
        );

        let mut report = ValidationReport::new();
        let result = validate_electronic_signature(&signature, &mut report);

        assert!(result.is_ok());
        assert!(!report.is_valid());
        assert!(report.errors.iter().any(|e| e.contains("empty")));
    }
}
