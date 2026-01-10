//! e-Gov Electronic Filing Core Types
//!
//! Provides core data structures for e-Gov electronic filing system,
//! including application metadata, government agencies, and form data.

use chrono::NaiveDate;
use std::collections::HashMap;

pub use crate::egov::error::ApplicationStatus;

/// Government agency handling the application
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GovernmentAgency {
    /// Digital Agency (デジタル庁)
    DigitalAgency,
    /// Ministry of Justice (法務省)
    MinistryOfJustice,
    /// Ministry of Land, Infrastructure, Transport and Tourism (国土交通省)
    MinistryOfLand,
    /// Ministry of the Environment (環境省)
    MinistryOfEnvironment,
    /// Ministry of Economy, Trade and Industry (経済産業省)
    MinistryOfEconomy,
    /// Personal Information Protection Commission (個人情報保護委員会)
    PersonalInfoCommission,
    /// Ministry of Health, Labour and Welfare (厚生労働省)
    MinistryOfHealthLabour,
    /// Consumer Affairs Agency (消費者庁)
    ConsumerAffairsAgency,
}

impl GovernmentAgency {
    /// Get the Japanese name of the agency
    pub fn name_ja(&self) -> &'static str {
        match self {
            Self::DigitalAgency => "デジタル庁",
            Self::MinistryOfJustice => "法務省",
            Self::MinistryOfLand => "国土交通省",
            Self::MinistryOfEnvironment => "環境省",
            Self::MinistryOfEconomy => "経済産業省",
            Self::PersonalInfoCommission => "個人情報保護委員会",
            Self::MinistryOfHealthLabour => "厚生労働省",
            Self::ConsumerAffairsAgency => "消費者庁",
        }
    }

    /// Get the English name of the agency
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::DigitalAgency => "Digital Agency",
            Self::MinistryOfJustice => "Ministry of Justice",
            Self::MinistryOfLand => "Ministry of Land, Infrastructure, Transport and Tourism",
            Self::MinistryOfEnvironment => "Ministry of the Environment",
            Self::MinistryOfEconomy => "Ministry of Economy, Trade and Industry",
            Self::PersonalInfoCommission => "Personal Information Protection Commission",
            Self::MinistryOfHealthLabour => "Ministry of Health, Labour and Welfare",
            Self::ConsumerAffairsAgency => "Consumer Affairs Agency",
        }
    }

    /// Get the agency code used in e-Gov system
    pub fn agency_code(&self) -> &'static str {
        match self {
            Self::DigitalAgency => "00100",
            Self::MinistryOfJustice => "00200",
            Self::MinistryOfLand => "00800",
            Self::MinistryOfEnvironment => "01300",
            Self::MinistryOfEconomy => "00900",
            Self::PersonalInfoCommission => "01800",
            Self::MinistryOfHealthLabour => "00500",
            Self::ConsumerAffairsAgency => "01700",
        }
    }
}

/// Application metadata for e-Gov filing
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ApplicationMetadata {
    /// Unique application ID
    pub application_id: String,
    /// Type of application (e.g., "construction_license", "factory_notification")
    pub application_type: String,
    /// Name of the applicant
    pub applicant_name: String,
    /// Date when application was submitted (None if not yet submitted)
    pub submission_date: Option<NaiveDate>,
    /// Current status of the application
    pub status: ApplicationStatus,
    /// Government agency handling this application
    pub agency: GovernmentAgency,
    /// Date when application was created
    pub created_date: NaiveDate,
    /// Date when application was last updated
    pub updated_date: NaiveDate,
    /// Additional notes or comments
    pub notes: Option<String>,
}

impl ApplicationMetadata {
    /// Create new application metadata
    pub fn new(
        application_id: impl Into<String>,
        application_type: impl Into<String>,
        applicant_name: impl Into<String>,
        agency: GovernmentAgency,
    ) -> Self {
        let today = chrono::Utc::now().date_naive();
        Self {
            application_id: application_id.into(),
            application_type: application_type.into(),
            applicant_name: applicant_name.into(),
            submission_date: None,
            status: ApplicationStatus::Draft,
            agency,
            created_date: today,
            updated_date: today,
            notes: None,
        }
    }

    /// Check if application is submitted
    pub fn is_submitted(&self) -> bool {
        self.submission_date.is_some() && !matches!(self.status, ApplicationStatus::Draft)
    }

    /// Check if application is in terminal state
    pub fn is_terminal(&self) -> bool {
        matches!(
            self.status,
            ApplicationStatus::Approved
                | ApplicationStatus::Rejected
                | ApplicationStatus::Withdrawn
        )
    }

    /// Check if application can be edited
    pub fn can_edit(&self) -> bool {
        matches!(
            self.status,
            ApplicationStatus::Draft | ApplicationStatus::RequiresRevision
        )
    }
}

/// Field value types for e-Gov forms
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EgovFieldValue {
    /// Text field
    Text(String),
    /// Number field
    Number(i64),
    /// Decimal number field
    Decimal(f64),
    /// Date field
    Date(NaiveDate),
    /// Boolean field
    Boolean(bool),
    /// List of values
    List(Vec<EgovFieldValue>),
    /// Nested object
    Object(HashMap<String, EgovFieldValue>),
}

impl EgovFieldValue {
    /// Try to get as text
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Text(s) => Some(s),
            _ => None,
        }
    }

    /// Try to get as number
    pub fn as_number(&self) -> Option<i64> {
        match self {
            Self::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Try to get as decimal
    pub fn as_decimal(&self) -> Option<f64> {
        match self {
            Self::Decimal(d) => Some(*d),
            _ => None,
        }
    }

    /// Try to get as date
    pub fn as_date(&self) -> Option<NaiveDate> {
        match self {
            Self::Date(d) => Some(*d),
            _ => None,
        }
    }

    /// Try to get as boolean
    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Self::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// Try to get as list
    pub fn as_list(&self) -> Option<&[EgovFieldValue]> {
        match self {
            Self::List(l) => Some(l),
            _ => None,
        }
    }

    /// Try to get as object
    pub fn as_object(&self) -> Option<&HashMap<String, EgovFieldValue>> {
        match self {
            Self::Object(o) => Some(o),
            _ => None,
        }
    }
}

/// Attachment metadata for e-Gov applications
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Attachment {
    /// Attachment ID
    pub id: String,
    /// File name
    pub filename: String,
    /// MIME type
    pub mime_type: String,
    /// File size in bytes
    pub size_bytes: usize,
    /// File content (base64 encoded in serialized form)
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub content: Option<Vec<u8>>,
    /// Description of the attachment
    pub description: Option<String>,
    /// Whether this attachment is required
    pub required: bool,
}

impl Attachment {
    /// Create new attachment
    pub fn new(
        id: impl Into<String>,
        filename: impl Into<String>,
        mime_type: impl Into<String>,
        size_bytes: usize,
    ) -> Self {
        Self {
            id: id.into(),
            filename: filename.into(),
            mime_type: mime_type.into(),
            size_bytes,
            content: None,
            description: None,
            required: false,
        }
    }

    /// Set attachment content
    pub fn with_content(mut self, content: Vec<u8>) -> Self {
        self.content = Some(content);
        self
    }

    /// Set description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Mark as required
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// Check if file size is within limit (default 10MB)
    pub fn is_size_valid(&self, max_bytes: usize) -> bool {
        self.size_bytes <= max_bytes
    }

    /// Check if file type is allowed
    pub fn is_type_allowed(&self, allowed_types: &[&str]) -> bool {
        allowed_types.iter().any(|t| self.mime_type.starts_with(t))
    }
}

/// e-Gov application structure
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EgovApplication {
    /// Application metadata
    pub metadata: ApplicationMetadata,
    /// Form field data
    pub form_data: HashMap<String, EgovFieldValue>,
    /// Attached files
    pub attachments: Vec<Attachment>,
}

impl EgovApplication {
    /// Create new e-Gov application
    pub fn new(
        application_id: impl Into<String>,
        application_type: impl Into<String>,
        applicant_name: impl Into<String>,
        agency: GovernmentAgency,
    ) -> Self {
        Self {
            metadata: ApplicationMetadata::new(
                application_id,
                application_type,
                applicant_name,
                agency,
            ),
            form_data: HashMap::new(),
            attachments: Vec::new(),
        }
    }

    /// Add form field
    pub fn add_field(&mut self, key: impl Into<String>, value: EgovFieldValue) {
        self.form_data.insert(key.into(), value);
    }

    /// Get form field
    pub fn get_field(&self, key: &str) -> Option<&EgovFieldValue> {
        self.form_data.get(key)
    }

    /// Add attachment
    pub fn add_attachment(&mut self, attachment: Attachment) {
        self.attachments.push(attachment);
    }

    /// Get attachment by ID
    pub fn get_attachment(&self, id: &str) -> Option<&Attachment> {
        self.attachments.iter().find(|a| a.id == id)
    }

    /// Check if all required fields are present
    pub fn has_required_fields(&self, required_fields: &[&str]) -> bool {
        required_fields
            .iter()
            .all(|field| self.form_data.contains_key(*field))
    }

    /// Check if all required attachments are present
    pub fn has_required_attachments(&self) -> bool {
        self.attachments
            .iter()
            .filter(|a| a.required)
            .all(|a| a.content.is_some())
    }
}

/// Validation report for e-Gov applications
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValidationReport {
    /// Validation errors
    pub errors: Vec<String>,
    /// Validation warnings
    pub warnings: Vec<String>,
}

impl ValidationReport {
    /// Create new empty validation report
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Check if validation passed (no errors)
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    /// Add error
    pub fn add_error(&mut self, error: impl Into<String>) {
        self.errors.push(error.into());
    }

    /// Add warning
    pub fn add_warning(&mut self, warning: impl Into<String>) {
        self.warnings.push(warning.into());
    }

    /// Get total issue count
    pub fn issue_count(&self) -> usize {
        self.errors.len() + self.warnings.len()
    }
}

impl Default for ValidationReport {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_government_agency_names() {
        assert_eq!(GovernmentAgency::DigitalAgency.name_ja(), "デジタル庁");
        assert_eq!(GovernmentAgency::DigitalAgency.name_en(), "Digital Agency");
        assert_eq!(GovernmentAgency::MinistryOfJustice.agency_code(), "00200");
    }

    #[test]
    fn test_application_metadata_creation() {
        let metadata = ApplicationMetadata::new(
            "APP-001",
            "test_application",
            "Test Applicant",
            GovernmentAgency::DigitalAgency,
        );

        assert_eq!(metadata.application_id, "APP-001");
        assert_eq!(metadata.status, ApplicationStatus::Draft);
        assert!(!metadata.is_submitted());
        assert!(metadata.can_edit());
        assert!(!metadata.is_terminal());
    }

    #[test]
    fn test_application_states() {
        let mut metadata = ApplicationMetadata::new(
            "APP-002",
            "test",
            "Applicant",
            GovernmentAgency::MinistryOfJustice,
        );

        // Draft state
        assert!(metadata.can_edit());
        assert!(!metadata.is_submitted());

        // Submitted state
        metadata.status = ApplicationStatus::Submitted;
        metadata.submission_date = Some(chrono::Utc::now().date_naive());
        assert!(metadata.is_submitted());
        assert!(!metadata.can_edit());

        // Terminal state
        metadata.status = ApplicationStatus::Approved;
        assert!(metadata.is_terminal());
        assert!(!metadata.can_edit());
    }

    #[test]
    fn test_field_value_conversions() {
        let text_val = EgovFieldValue::Text("test".to_string());
        assert_eq!(text_val.as_text(), Some("test"));
        assert_eq!(text_val.as_number(), None);

        let num_val = EgovFieldValue::Number(42);
        assert_eq!(num_val.as_number(), Some(42));
        assert_eq!(num_val.as_text(), None);

        let bool_val = EgovFieldValue::Boolean(true);
        assert_eq!(bool_val.as_boolean(), Some(true));
    }

    #[test]
    fn test_attachment_validation() {
        let attachment = Attachment::new("ATT-001", "document.pdf", "application/pdf", 1_000_000)
            .with_description("Test document")
            .required();

        assert!(attachment.is_size_valid(10_000_000)); // 10MB limit
        assert!(!attachment.is_size_valid(500_000)); // 500KB limit
        assert!(attachment.is_type_allowed(&["application/pdf", "image/"]));
        assert!(!attachment.is_type_allowed(&["image/"]));
    }

    #[test]
    fn test_egov_application_fields() {
        let mut app = EgovApplication::new(
            "APP-003",
            "test",
            "Test User",
            GovernmentAgency::MinistryOfEnvironment,
        );

        app.add_field("field1", EgovFieldValue::Text("value1".to_string()));
        app.add_field("field2", EgovFieldValue::Number(100));

        assert!(app.get_field("field1").is_some());
        assert_eq!(app.get_field("field1").unwrap().as_text(), Some("value1"));
        assert!(app.has_required_fields(&["field1", "field2"]));
        assert!(!app.has_required_fields(&["field1", "field2", "field3"]));
    }

    #[test]
    fn test_egov_application_attachments() {
        let mut app = EgovApplication::new(
            "APP-004",
            "test",
            "User",
            GovernmentAgency::MinistryOfEconomy,
        );

        let attachment = Attachment::new("ATT-001", "file.pdf", "application/pdf", 1000)
            .with_content(vec![1, 2, 3])
            .required();

        app.add_attachment(attachment);

        assert!(app.get_attachment("ATT-001").is_some());
        assert!(app.has_required_attachments());
    }

    #[test]
    fn test_validation_report() {
        let mut report = ValidationReport::new();
        assert!(report.is_valid());

        report.add_error("Error 1");
        report.add_warning("Warning 1");

        assert!(!report.is_valid());
        assert_eq!(report.errors.len(), 1);
        assert_eq!(report.warnings.len(), 1);
        assert_eq!(report.issue_count(), 2);
    }
}
