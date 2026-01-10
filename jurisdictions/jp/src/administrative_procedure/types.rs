//! Administrative Procedure Act Core Types
//!
//! Data structures for administrative procedures, electronic signatures,
//! and related concepts under Japanese law.

use chrono::{DateTime, NaiveDate, Utc};

use crate::egov::GovernmentAgency;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Type of administrative procedure (行政手続法第2条)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ProcedureType {
    /// Application (申請) - Article 2-3
    Application,
    /// Notification (届出) - Article 2-7
    Notification,
    /// Administrative guidance (行政指導) - Article 2-6
    AdministrativeGuidance,
    /// Administrative disposition (処分) - Article 2-2
    AdministrativeDisposition,
    /// Hearing (聴聞)
    Hearing,
}

impl ProcedureType {
    /// Get Japanese name
    pub fn name_ja(&self) -> &'static str {
        match self {
            Self::Application => "申請",
            Self::Notification => "届出",
            Self::AdministrativeGuidance => "行政指導",
            Self::AdministrativeDisposition => "処分",
            Self::Hearing => "聴聞",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Application => "Application",
            Self::Notification => "Notification",
            Self::AdministrativeGuidance => "Administrative Guidance",
            Self::AdministrativeDisposition => "Administrative Disposition",
            Self::Hearing => "Hearing",
        }
    }
}

/// Type of applicant
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ApplicantType {
    /// Individual (個人)
    Individual,
    /// Corporation (法人)
    Corporation,
    /// Government entity (行政機関)
    GovernmentEntity,
}

/// Contact information
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ContactInfo {
    /// Postal code
    pub postal_code: Option<String>,
    /// Address
    pub address: String,
    /// Phone number
    pub phone: Option<String>,
    /// Email address
    pub email: Option<String>,
}

/// Identification information
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Identification {
    /// ID type (e.g., "my_number", "corporate_number", "passport")
    pub id_type: String,
    /// ID number
    pub id_number: String,
}

/// Applicant information
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Applicant {
    /// Name
    pub name: String,
    /// Applicant type
    pub applicant_type: ApplicantType,
    /// Contact information
    pub contact: ContactInfo,
    /// Identification
    pub identification: Option<Identification>,
}

impl Applicant {
    /// Create new individual applicant
    pub fn individual(name: impl Into<String>, address: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            applicant_type: ApplicantType::Individual,
            contact: ContactInfo {
                postal_code: None,
                address: address.into(),
                phone: None,
                email: None,
            },
            identification: None,
        }
    }

    /// Create new corporate applicant
    pub fn corporation(name: impl Into<String>, address: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            applicant_type: ApplicantType::Corporation,
            contact: ContactInfo {
                postal_code: None,
                address: address.into(),
                phone: None,
                email: None,
            },
            identification: None,
        }
    }

    /// Add contact phone
    pub fn with_phone(mut self, phone: impl Into<String>) -> Self {
        self.contact.phone = Some(phone.into());
        self
    }

    /// Add contact email
    pub fn with_email(mut self, email: impl Into<String>) -> Self {
        self.contact.email = Some(email.into());
        self
    }

    /// Add identification
    pub fn with_identification(
        mut self,
        id_type: impl Into<String>,
        id_number: impl Into<String>,
    ) -> Self {
        self.identification = Some(Identification {
            id_type: id_type.into(),
            id_number: id_number.into(),
        });
        self
    }
}

/// Document attached to procedure
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Document {
    /// Document ID
    pub id: String,
    /// Document title
    pub title: String,
    /// Document type
    pub document_type: DocumentType,
    /// Whether reason statement is included (Article 5)
    pub includes_reason_statement: bool,
    /// Creation date
    pub created_date: NaiveDate,
}

/// Type of document
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DocumentType {
    /// Application form (申請書)
    ApplicationForm,
    /// Notification form (届出書)
    NotificationForm,
    /// Supporting document (添付書類)
    SupportingDocument,
    /// Reason statement (理由の提示)
    ReasonStatement,
    /// Disposition notice (処分通知書)
    DispositionNotice,
    /// Other
    Other(String),
}

/// Signature algorithm (電子署名法)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SignatureAlgorithm {
    /// RSA 2048-bit
    Rsa2048,
    /// RSA 4096-bit
    Rsa4096,
    /// ECDSA P-256
    EcdsaP256,
    /// ECDSA P-384
    EcdsaP384,
}

impl SignatureAlgorithm {
    /// Get algorithm name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Rsa2048 => "RSA-2048",
            Self::Rsa4096 => "RSA-4096",
            Self::EcdsaP256 => "ECDSA-P256",
            Self::EcdsaP384 => "ECDSA-P384",
        }
    }

    /// Check if algorithm is recommended
    pub fn is_recommended(&self) -> bool {
        matches!(self, Self::Rsa2048 | Self::EcdsaP256)
    }
}

/// Digital certificate (電子証明書)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Certificate {
    /// Issuer name
    pub issuer: String,
    /// Subject name
    pub subject: String,
    /// Valid from date
    pub valid_from: NaiveDate,
    /// Valid until date
    pub valid_until: NaiveDate,
    /// Public key (DER format)
    #[cfg_attr(feature = "serde", serde(skip))]
    pub public_key: Vec<u8>,
    /// Serial number
    pub serial_number: String,
}

impl Certificate {
    /// Check if certificate is currently valid
    pub fn is_valid(&self) -> bool {
        let now = Utc::now().date_naive();
        now >= self.valid_from && now <= self.valid_until
    }

    /// Check if certificate is expired
    pub fn is_expired(&self) -> bool {
        let now = Utc::now().date_naive();
        now > self.valid_until
    }

    /// Check if certificate is not yet valid
    pub fn is_not_yet_valid(&self) -> bool {
        let now = Utc::now().date_naive();
        now < self.valid_from
    }

    /// Get days until expiration
    pub fn days_until_expiration(&self) -> i64 {
        let now = Utc::now().date_naive();
        (self.valid_until - now).num_days()
    }
}

/// Electronic signature (電子署名)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ElectronicSignature {
    /// Signature algorithm
    pub signature_algorithm: SignatureAlgorithm,
    /// Certificate
    pub certificate: Certificate,
    /// Signature value (binary)
    #[cfg_attr(feature = "serde", serde(skip))]
    pub signature_value: Vec<u8>,
    /// Timestamp when signed
    pub signed_at: DateTime<Utc>,
}

impl ElectronicSignature {
    /// Create new electronic signature
    pub fn new(
        algorithm: SignatureAlgorithm,
        certificate: Certificate,
        signature_value: Vec<u8>,
    ) -> Self {
        Self {
            signature_algorithm: algorithm,
            certificate,
            signature_value,
            signed_at: Utc::now(),
        }
    }

    /// Check if signature certificate is valid
    pub fn is_certificate_valid(&self) -> bool {
        self.certificate.is_valid()
    }
}

/// Administrative procedure
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AdministrativeProcedure {
    /// Procedure ID
    pub procedure_id: String,
    /// Procedure type
    pub procedure_type: ProcedureType,
    /// Government agency handling this
    pub agency: GovernmentAgency,
    /// Applicant information
    pub applicant: Applicant,
    /// Submission date
    pub submission_date: NaiveDate,
    /// Documents
    pub documents: Vec<Document>,
    /// Electronic signature (optional)
    pub electronic_signature: Option<ElectronicSignature>,
    /// Standard processing period in days (Article 7)
    pub processing_period_days: Option<u32>,
    /// Notes
    pub notes: Option<String>,
}

impl AdministrativeProcedure {
    /// Create new procedure
    pub fn new(
        procedure_id: impl Into<String>,
        procedure_type: ProcedureType,
        agency: GovernmentAgency,
        applicant: Applicant,
    ) -> Self {
        Self {
            procedure_id: procedure_id.into(),
            procedure_type,
            agency,
            applicant,
            submission_date: Utc::now().date_naive(),
            documents: Vec::new(),
            electronic_signature: None,
            processing_period_days: None,
            notes: None,
        }
    }

    /// Add document
    pub fn add_document(&mut self, document: Document) {
        self.documents.push(document);
    }

    /// Set electronic signature
    pub fn set_signature(&mut self, signature: ElectronicSignature) {
        self.electronic_signature = Some(signature);
    }

    /// Set processing period
    pub fn set_processing_period(&mut self, days: u32) {
        self.processing_period_days = Some(days);
    }

    /// Check if procedure has reason statement
    pub fn has_reason_statement(&self) -> bool {
        self.documents.iter().any(|d| d.includes_reason_statement)
    }

    /// Check if procedure is electronically signed
    pub fn is_electronically_signed(&self) -> bool {
        self.electronic_signature.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_procedure_type_names() {
        assert_eq!(ProcedureType::Application.name_ja(), "申請");
        assert_eq!(ProcedureType::Application.name_en(), "Application");
    }

    #[test]
    fn test_applicant_creation() {
        let applicant = Applicant::individual("田中太郎", "東京都渋谷区")
            .with_phone("03-1234-5678")
            .with_email("tanaka@example.com");

        assert_eq!(applicant.name, "田中太郎");
        assert_eq!(applicant.applicant_type, ApplicantType::Individual);
        assert_eq!(applicant.contact.phone, Some("03-1234-5678".to_string()));
    }

    #[test]
    fn test_signature_algorithm() {
        assert!(SignatureAlgorithm::Rsa2048.is_recommended());
        assert!(SignatureAlgorithm::EcdsaP256.is_recommended());
        assert!(!SignatureAlgorithm::Rsa4096.is_recommended());
    }

    #[test]
    fn test_certificate_validity() {
        let valid_cert = Certificate {
            issuer: "Test CA".to_string(),
            subject: "Test User".to_string(),
            valid_from: Utc::now().date_naive() - chrono::Duration::days(30),
            valid_until: Utc::now().date_naive() + chrono::Duration::days(365),
            public_key: vec![],
            serial_number: "123456".to_string(),
        };

        assert!(valid_cert.is_valid());
        assert!(!valid_cert.is_expired());
        assert!(!valid_cert.is_not_yet_valid());
    }

    #[test]
    fn test_expired_certificate() {
        let expired_cert = Certificate {
            issuer: "Test CA".to_string(),
            subject: "Test User".to_string(),
            valid_from: Utc::now().date_naive() - chrono::Duration::days(400),
            valid_until: Utc::now().date_naive() - chrono::Duration::days(30),
            public_key: vec![],
            serial_number: "123456".to_string(),
        };

        assert!(!expired_cert.is_valid());
        assert!(expired_cert.is_expired());
    }

    #[test]
    fn test_procedure_creation() {
        let applicant = Applicant::individual("Test User", "Tokyo");
        let procedure = AdministrativeProcedure::new(
            "PROC-001",
            ProcedureType::Application,
            GovernmentAgency::DigitalAgency,
            applicant,
        );

        assert_eq!(procedure.procedure_id, "PROC-001");
        assert_eq!(procedure.procedure_type, ProcedureType::Application);
        assert!(!procedure.has_reason_statement());
    }

    #[test]
    fn test_procedure_with_documents() {
        let applicant = Applicant::individual("Test User", "Tokyo");
        let mut procedure = AdministrativeProcedure::new(
            "PROC-002",
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
        assert!(procedure.has_reason_statement());
    }

    #[test]
    fn test_electronic_signature() {
        let cert = Certificate {
            issuer: "Test CA".to_string(),
            subject: "Test User".to_string(),
            valid_from: Utc::now().date_naive(),
            valid_until: Utc::now().date_naive() + chrono::Duration::days(365),
            public_key: vec![1, 2, 3],
            serial_number: "123".to_string(),
        };

        let signature = ElectronicSignature::new(SignatureAlgorithm::Rsa2048, cert, vec![4, 5, 6]);

        assert!(signature.is_certificate_valid());
        assert_eq!(signature.signature_algorithm.name(), "RSA-2048");
    }
}
