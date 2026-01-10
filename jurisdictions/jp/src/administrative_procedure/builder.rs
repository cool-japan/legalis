//! Administrative Procedure Builder
//!
//! Fluent API for building administrative procedures with validation.

use crate::administrative_procedure::{
    error::{AdministrativeError, Result},
    types::{
        AdministrativeProcedure, Applicant, Certificate, Document, ElectronicSignature,
        ProcedureType, SignatureAlgorithm,
    },
};
use crate::egov::GovernmentAgency;
use chrono::{DateTime, NaiveDate, Utc};

/// Builder for AdministrativeProcedure
#[derive(Debug)]
pub struct ProcedureBuilder {
    procedure_id: Option<String>,
    procedure_type: Option<ProcedureType>,
    agency: Option<GovernmentAgency>,
    applicant: Option<Applicant>,
    submission_date: Option<NaiveDate>,
    documents: Vec<Document>,
    electronic_signature: Option<ElectronicSignature>,
    processing_period_days: Option<u32>,
    notes: Option<String>,
}

impl ProcedureBuilder {
    /// Create new builder
    pub fn new() -> Self {
        Self {
            procedure_id: None,
            procedure_type: None,
            agency: None,
            applicant: None,
            submission_date: None,
            documents: Vec::new(),
            electronic_signature: None,
            processing_period_days: None,
            notes: None,
        }
    }

    /// Set procedure ID
    pub fn procedure_id(mut self, id: impl Into<String>) -> Self {
        self.procedure_id = Some(id.into());
        self
    }

    /// Set procedure type
    pub fn procedure_type(mut self, procedure_type: ProcedureType) -> Self {
        self.procedure_type = Some(procedure_type);
        self
    }

    /// Set government agency
    pub fn agency(mut self, agency: GovernmentAgency) -> Self {
        self.agency = Some(agency);
        self
    }

    /// Set applicant
    pub fn applicant(mut self, applicant: Applicant) -> Self {
        self.applicant = Some(applicant);
        self
    }

    /// Set submission date
    pub fn submission_date(mut self, date: NaiveDate) -> Self {
        self.submission_date = Some(date);
        self
    }

    /// Add document
    pub fn add_document(mut self, document: Document) -> Self {
        self.documents.push(document);
        self
    }

    /// Set electronic signature
    pub fn with_signature(mut self, signature: ElectronicSignature) -> Self {
        self.electronic_signature = Some(signature);
        self
    }

    /// Set processing period
    pub fn processing_period_days(mut self, days: u32) -> Self {
        self.processing_period_days = Some(days);
        self
    }

    /// Set notes
    pub fn notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    /// Build the procedure
    pub fn build(self) -> Result<AdministrativeProcedure> {
        let procedure_id =
            self.procedure_id
                .ok_or_else(|| AdministrativeError::MissingRequiredField {
                    field: "procedure_id".to_string(),
                    article: "N/A".to_string(),
                })?;

        let procedure_type =
            self.procedure_type
                .ok_or_else(|| AdministrativeError::MissingRequiredField {
                    field: "procedure_type".to_string(),
                    article: "2".to_string(),
                })?;

        let agency = self
            .agency
            .ok_or_else(|| AdministrativeError::MissingRequiredField {
                field: "agency".to_string(),
                article: "N/A".to_string(),
            })?;

        let applicant =
            self.applicant
                .ok_or_else(|| AdministrativeError::MissingRequiredField {
                    field: "applicant".to_string(),
                    article: "N/A".to_string(),
                })?;

        let submission_date = self
            .submission_date
            .unwrap_or_else(|| Utc::now().date_naive());

        Ok(AdministrativeProcedure {
            procedure_id,
            procedure_type,
            agency,
            applicant,
            submission_date,
            documents: self.documents,
            electronic_signature: self.electronic_signature,
            processing_period_days: self.processing_period_days,
            notes: self.notes,
        })
    }
}

impl Default for ProcedureBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for Certificate
#[derive(Debug)]
pub struct CertificateBuilder {
    issuer: Option<String>,
    subject: Option<String>,
    valid_from: Option<NaiveDate>,
    valid_until: Option<NaiveDate>,
    public_key: Vec<u8>,
    serial_number: Option<String>,
}

impl CertificateBuilder {
    /// Create new certificate builder
    pub fn new() -> Self {
        Self {
            issuer: None,
            subject: None,
            valid_from: None,
            valid_until: None,
            public_key: Vec::new(),
            serial_number: None,
        }
    }

    /// Set issuer
    pub fn issuer(mut self, issuer: impl Into<String>) -> Self {
        self.issuer = Some(issuer.into());
        self
    }

    /// Set subject
    pub fn subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    /// Set validity period
    pub fn valid_from(mut self, from: NaiveDate) -> Self {
        self.valid_from = Some(from);
        self
    }

    /// Set validity end date
    pub fn valid_until(mut self, until: NaiveDate) -> Self {
        self.valid_until = Some(until);
        self
    }

    /// Set validity period in days from now
    pub fn valid_for_days(mut self, days: i64) -> Self {
        let now = Utc::now().date_naive();
        self.valid_from = Some(now);
        self.valid_until = Some(now + chrono::Duration::days(days));
        self
    }

    /// Set public key
    pub fn public_key(mut self, key: Vec<u8>) -> Self {
        self.public_key = key;
        self
    }

    /// Set serial number
    pub fn serial_number(mut self, serial: impl Into<String>) -> Self {
        self.serial_number = Some(serial.into());
        self
    }

    /// Build the certificate
    pub fn build(self) -> Result<Certificate> {
        let issuer = self
            .issuer
            .ok_or_else(|| AdministrativeError::InvalidCertificate {
                reason: "Missing issuer".to_string(),
            })?;

        let subject = self
            .subject
            .ok_or_else(|| AdministrativeError::InvalidCertificate {
                reason: "Missing subject".to_string(),
            })?;

        let valid_from =
            self.valid_from
                .ok_or_else(|| AdministrativeError::InvalidCertificate {
                    reason: "Missing valid_from date".to_string(),
                })?;

        let valid_until =
            self.valid_until
                .ok_or_else(|| AdministrativeError::InvalidCertificate {
                    reason: "Missing valid_until date".to_string(),
                })?;

        let serial_number = self
            .serial_number
            .unwrap_or_else(|| format!("{}", chrono::Utc::now().timestamp()));

        if valid_from >= valid_until {
            return Err(AdministrativeError::InvalidCertificate {
                reason: "valid_from must be before valid_until".to_string(),
            });
        }

        Ok(Certificate {
            issuer,
            subject,
            valid_from,
            valid_until,
            public_key: self.public_key,
            serial_number,
        })
    }
}

impl Default for CertificateBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for ElectronicSignature
#[derive(Debug)]
pub struct SignatureBuilder {
    algorithm: Option<SignatureAlgorithm>,
    certificate: Option<Certificate>,
    signature_value: Vec<u8>,
    signed_at: Option<DateTime<Utc>>,
}

impl SignatureBuilder {
    /// Create new signature builder
    pub fn new() -> Self {
        Self {
            algorithm: None,
            certificate: None,
            signature_value: Vec::new(),
            signed_at: None,
        }
    }

    /// Set signature algorithm
    pub fn algorithm(mut self, algorithm: SignatureAlgorithm) -> Self {
        self.algorithm = Some(algorithm);
        self
    }

    /// Set certificate
    pub fn certificate(mut self, certificate: Certificate) -> Self {
        self.certificate = Some(certificate);
        self
    }

    /// Set signature value
    pub fn signature_value(mut self, value: Vec<u8>) -> Self {
        self.signature_value = value;
        self
    }

    /// Set timestamp
    pub fn signed_at(mut self, timestamp: DateTime<Utc>) -> Self {
        self.signed_at = Some(timestamp);
        self
    }

    /// Build the signature
    pub fn build(self) -> Result<ElectronicSignature> {
        let algorithm = self
            .algorithm
            .ok_or_else(|| AdministrativeError::InvalidCertificate {
                reason: "Missing signature algorithm".to_string(),
            })?;

        let certificate =
            self.certificate
                .ok_or_else(|| AdministrativeError::InvalidCertificate {
                    reason: "Missing certificate".to_string(),
                })?;

        if self.signature_value.is_empty() {
            return Err(AdministrativeError::InvalidCertificate {
                reason: "Signature value is empty".to_string(),
            });
        }

        let signed_at = self.signed_at.unwrap_or_else(Utc::now);

        Ok(ElectronicSignature {
            signature_algorithm: algorithm,
            certificate,
            signature_value: self.signature_value,
            signed_at,
        })
    }
}

impl Default for SignatureBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::administrative_procedure::types::Applicant;

    #[test]
    fn test_procedure_builder() {
        let applicant = Applicant::individual("Test User", "Tokyo");

        let procedure = ProcedureBuilder::new()
            .procedure_id("PROC-001")
            .procedure_type(ProcedureType::Application)
            .agency(GovernmentAgency::DigitalAgency)
            .applicant(applicant)
            .processing_period_days(30)
            .notes("Test procedure")
            .build();

        assert!(procedure.is_ok());
        let proc = procedure.unwrap();
        assert_eq!(proc.procedure_id, "PROC-001");
        assert_eq!(proc.processing_period_days, Some(30));
    }

    #[test]
    fn test_procedure_builder_missing_field() {
        let result = ProcedureBuilder::new()
            .procedure_id("PROC-002")
            .procedure_type(ProcedureType::Application)
            .build();

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AdministrativeError::MissingRequiredField { .. }
        ));
    }

    #[test]
    fn test_certificate_builder() {
        let cert = CertificateBuilder::new()
            .issuer("Test CA")
            .subject("Test User")
            .valid_for_days(365)
            .public_key(vec![1, 2, 3])
            .serial_number("123456")
            .build();

        assert!(cert.is_ok());
        let certificate = cert.unwrap();
        assert_eq!(certificate.issuer, "Test CA");
        assert!(certificate.is_valid());
    }

    #[test]
    fn test_certificate_builder_invalid_dates() {
        let now = Utc::now().date_naive();

        let result = CertificateBuilder::new()
            .issuer("Test CA")
            .subject("Test User")
            .valid_from(now)
            .valid_until(now - chrono::Duration::days(1)) // Invalid: end before start
            .build();

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AdministrativeError::InvalidCertificate { .. }
        ));
    }

    #[test]
    fn test_signature_builder() {
        let cert = CertificateBuilder::new()
            .issuer("Test CA")
            .subject("Test User")
            .valid_for_days(365)
            .build()
            .unwrap();

        let signature = SignatureBuilder::new()
            .algorithm(SignatureAlgorithm::Rsa2048)
            .certificate(cert)
            .signature_value(vec![1, 2, 3, 4, 5])
            .build();

        assert!(signature.is_ok());
        let sig = signature.unwrap();
        assert_eq!(sig.signature_algorithm, SignatureAlgorithm::Rsa2048);
    }

    #[test]
    fn test_signature_builder_empty_value() {
        let cert = CertificateBuilder::new()
            .issuer("Test CA")
            .subject("Test User")
            .valid_for_days(365)
            .build()
            .unwrap();

        let result = SignatureBuilder::new()
            .algorithm(SignatureAlgorithm::EcdsaP256)
            .certificate(cert)
            .build(); // No signature value

        assert!(result.is_err());
    }
}
