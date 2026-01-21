//! GDPR Article 30 - Records of Processing Activities (ROPA)
//!
//! This module implements the record-keeping requirements under Article 30 GDPR.
//!
//! ## When ROPA is Required
//!
//! **Controllers** (Article 30(1)): All controllers must maintain records UNLESS:
//! - Enterprise with <250 employees AND
//! - Processing is occasional AND
//! - Processing is not likely to result in risk to rights AND
//! - Does not include special categories or criminal data
//!
//! **Processors** (Article 30(2)): All processors must maintain records (same exemptions)
//!
//! ## Required Information
//!
//! ### Controller Records (Article 30(1)) must contain:
//! - Name and contact details of controller (and DPO if applicable)
//! - Purposes of processing
//! - Categories of data subjects and personal data
//! - Categories of recipients (including third countries)
//! - Transfers to third countries (with documentation of safeguards)
//! - Retention periods
//! - Technical and organizational security measures (general description)
//!
//! ### Processor Records (Article 30(2)) must contain:
//! - Name and contact details of processor (and DPO if applicable)
//! - Categories of processing carried out on behalf of each controller
//! - Transfers to third countries
//! - Technical and organizational security measures
//!
//! ## Example
//!
//! ```rust
//! use legalis_eu::gdpr::ropa::*;
//! use legalis_eu::gdpr::types::{PersonalDataCategory, ProcessingOperation};
//!
//! let record = ProcessingRecord::new()
//!     .with_name("Customer Relationship Management")
//!     .with_controller("Acme Corp", "privacy@acme.com")
//!     .with_purpose("Customer service and support")
//!     .add_data_subject_category("customers")
//!     .add_data_category(PersonalDataCategory::Regular("name".to_string()))
//!     .add_data_category(PersonalDataCategory::Regular("email".to_string()))
//!     .add_recipient("Support team (internal)")
//!     .with_retention_period("7 years after last contact")
//!     .add_security_measure("Encryption at rest")
//!     .add_security_measure("Role-based access control");
//!
//! match record.validate() {
//!     Ok(validation) => println!("✅ Record complete"),
//!     Err(e) => println!("❌ Record incomplete: {}", e),
//! }
//! ```

use crate::gdpr::error::GdprError;
use crate::gdpr::types::{LawfulBasis, PersonalDataCategory, ProcessingOperation};
use chrono::{DateTime, Utc};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::gdpr::types::SpecialCategory;

/// Type of entity maintaining the record
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EntityType {
    /// Data controller (Article 30(1))
    Controller,
    /// Data processor (Article 30(2))
    Processor,
    /// Joint controller (Article 26)
    JointController,
}

/// Contact details for controller/processor
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ContactDetails {
    /// Name of organization
    pub name: String,
    /// Email address
    pub email: String,
    /// Physical address (optional)
    pub address: Option<String>,
    /// Phone number (optional)
    pub phone: Option<String>,
}

impl ContactDetails {
    pub fn new(name: impl Into<String>, email: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            email: email.into(),
            address: None,
            phone: None,
        }
    }

    pub fn with_address(mut self, address: impl Into<String>) -> Self {
        self.address = Some(address.into());
        self
    }

    pub fn with_phone(mut self, phone: impl Into<String>) -> Self {
        self.phone = Some(phone.into());
        self
    }
}

/// Third country transfer details
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ThirdCountryTransfer {
    /// Destination country
    pub country: String,
    /// Safeguard mechanism used
    pub safeguard: String,
    /// Documentation reference
    pub documentation: Option<String>,
}

/// Processing record under Article 30
#[derive(Debug, Clone)]
pub struct ProcessingRecord {
    /// Type of entity (controller or processor)
    pub entity_type: EntityType,

    /// Name of this processing activity
    pub name: Option<String>,

    /// Controller contact details
    pub controller: Option<ContactDetails>,

    /// Processor contact details (if processor record)
    pub processor: Option<ContactDetails>,

    /// Data Protection Officer contact (if applicable)
    pub dpo: Option<ContactDetails>,

    /// Representative in EU (if controller/processor outside EU)
    pub representative: Option<ContactDetails>,

    /// Purposes of processing (required for controllers)
    pub purposes: Vec<String>,

    /// Lawful basis (recommended but not explicitly required by Article 30)
    pub lawful_basis: Option<LawfulBasis>,

    /// Categories of data subjects
    pub data_subject_categories: Vec<String>,

    /// Categories of personal data
    pub data_categories: Vec<PersonalDataCategory>,

    /// Categories of recipients (including third parties)
    pub recipients: Vec<String>,

    /// Third country transfers
    pub third_country_transfers: Vec<ThirdCountryTransfer>,

    /// Retention periods (or criteria for determining them)
    pub retention_period: Option<String>,

    /// Technical and organizational security measures (general description)
    pub security_measures: Vec<String>,

    /// Date record was created
    pub created_date: Option<DateTime<Utc>>,

    /// Date record was last updated
    pub last_updated: Option<DateTime<Utc>>,

    /// Processing operations (optional, helpful for documentation)
    pub operations: Vec<ProcessingOperation>,

    /// Notes or additional information
    pub notes: Option<String>,
}

impl ProcessingRecord {
    pub fn new() -> Self {
        Self {
            entity_type: EntityType::Controller,
            name: None,
            controller: None,
            processor: None,
            dpo: None,
            representative: None,
            purposes: Vec::new(),
            lawful_basis: None,
            data_subject_categories: Vec::new(),
            data_categories: Vec::new(),
            recipients: Vec::new(),
            third_country_transfers: Vec::new(),
            retention_period: None,
            security_measures: Vec::new(),
            created_date: None,
            last_updated: None,
            operations: Vec::new(),
            notes: None,
        }
    }

    pub fn with_entity_type(mut self, entity_type: EntityType) -> Self {
        self.entity_type = entity_type;
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_controller(mut self, name: impl Into<String>, email: impl Into<String>) -> Self {
        self.controller = Some(ContactDetails::new(name, email));
        self
    }

    pub fn with_controller_details(mut self, details: ContactDetails) -> Self {
        self.controller = Some(details);
        self
    }

    pub fn with_processor(mut self, name: impl Into<String>, email: impl Into<String>) -> Self {
        self.processor = Some(ContactDetails::new(name, email));
        self
    }

    pub fn with_dpo(mut self, name: impl Into<String>, email: impl Into<String>) -> Self {
        self.dpo = Some(ContactDetails::new(name, email));
        self
    }

    pub fn with_representative(
        mut self,
        name: impl Into<String>,
        email: impl Into<String>,
    ) -> Self {
        self.representative = Some(ContactDetails::new(name, email));
        self
    }

    pub fn with_purpose(mut self, purpose: impl Into<String>) -> Self {
        self.purposes.push(purpose.into());
        self
    }

    pub fn with_lawful_basis(mut self, basis: LawfulBasis) -> Self {
        self.lawful_basis = Some(basis);
        self
    }

    pub fn add_data_subject_category(mut self, category: impl Into<String>) -> Self {
        self.data_subject_categories.push(category.into());
        self
    }

    pub fn add_data_category(mut self, category: PersonalDataCategory) -> Self {
        self.data_categories.push(category);
        self
    }

    pub fn add_recipient(mut self, recipient: impl Into<String>) -> Self {
        self.recipients.push(recipient.into());
        self
    }

    pub fn add_third_country_transfer(mut self, transfer: ThirdCountryTransfer) -> Self {
        self.third_country_transfers.push(transfer);
        self
    }

    pub fn with_retention_period(mut self, period: impl Into<String>) -> Self {
        self.retention_period = Some(period.into());
        self
    }

    pub fn add_security_measure(mut self, measure: impl Into<String>) -> Self {
        self.security_measures.push(measure.into());
        self
    }

    pub fn with_created_date(mut self, date: DateTime<Utc>) -> Self {
        self.created_date = Some(date);
        self
    }

    pub fn with_last_updated(mut self, date: DateTime<Utc>) -> Self {
        self.last_updated = Some(date);
        self
    }

    pub fn add_operation(mut self, operation: ProcessingOperation) -> Self {
        self.operations.push(operation);
        self
    }

    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    /// Validate that record meets Article 30 requirements
    pub fn validate(&self) -> Result<RecordValidation, GdprError> {
        let mut missing_fields = Vec::new();

        // Common requirements for both controllers and processors
        match self.entity_type {
            EntityType::Controller | EntityType::JointController => {
                // Article 30(1) - Controller requirements
                if self.controller.is_none() {
                    missing_fields.push("controller contact details (Article 30(1)(a))");
                }

                if self.purposes.is_empty() {
                    missing_fields.push("purposes of processing (Article 30(1)(b))");
                }

                if self.data_subject_categories.is_empty() {
                    missing_fields.push("categories of data subjects (Article 30(1)(c))");
                }

                if self.data_categories.is_empty() {
                    missing_fields.push("categories of personal data (Article 30(1)(c))");
                }

                // Recipients can be empty if no sharing
                // Third country transfers required only if applicable

                if self.retention_period.is_none() {
                    missing_fields.push("retention periods or criteria (Article 30(1)(f))");
                }

                if self.security_measures.is_empty() {
                    missing_fields.push(
                        "description of technical and organizational security measures (Article 30(1)(g))",
                    );
                }
            }

            EntityType::Processor => {
                // Article 30(2) - Processor requirements
                if self.processor.is_none() {
                    missing_fields.push("processor contact details (Article 30(2)(a))");
                }

                // Processors need categories of processing, but we can use purposes + operations
                if self.purposes.is_empty() && self.operations.is_empty() {
                    missing_fields.push("categories of processing (Article 30(2)(b))");
                }

                if self.security_measures.is_empty() {
                    missing_fields.push("description of security measures (Article 30(2)(d))");
                }
            }
        }

        if !missing_fields.is_empty() {
            return Err(GdprError::InvalidValue {
                field: "processing_record".to_string(),
                reason: format!("Missing required fields: {}", missing_fields.join(", ")),
            });
        }

        // Check for special categories - if present, should be documented
        let contains_special_categories = self
            .data_categories
            .iter()
            .any(|cat| matches!(cat, PersonalDataCategory::Special(_)));

        Ok(RecordValidation {
            complete: true,
            contains_special_categories,
            has_third_country_transfers: !self.third_country_transfers.is_empty(),
            has_dpo: self.dpo.is_some(),
            warnings: self.generate_warnings(),
        })
    }

    /// Generate warnings for incomplete or missing optional information
    fn generate_warnings(&self) -> Vec<String> {
        let mut warnings = Vec::new();

        // Check if processing special categories without explicit documentation
        let has_special = self
            .data_categories
            .iter()
            .any(|cat| matches!(cat, PersonalDataCategory::Special(_)));

        if has_special && self.lawful_basis.is_none() {
            warnings.push(
                "Processing special categories but no lawful basis documented \
                 (consider documenting Article 9 exception)"
                    .to_string(),
            );
        }

        // Check if third country transfers lack documentation
        for transfer in &self.third_country_transfers {
            if transfer.documentation.is_none() {
                warnings.push(format!(
                    "Third country transfer to {} lacks documentation reference \
                     (Article 30(1)(e) requires documentation of safeguards)",
                    transfer.country
                ));
            }
        }

        // Check if retention period is vague
        if let Some(ref period) = self.retention_period
            && (period.contains("indefinite") || period.contains("permanent"))
        {
            warnings.push(
                "Indefinite/permanent retention may violate storage limitation principle \
                     (Article 5(1)(e)) - consider specific retention criteria"
                    .to_string(),
            );
        }

        // Check if security measures are minimal
        if self.security_measures.len() < 3 {
            warnings.push(
                "Limited security measures documented - consider documenting more details \
                 (encryption, access controls, monitoring, backups, etc.)"
                    .to_string(),
            );
        }

        // Check if no DPO when potentially required
        if self.dpo.is_none() && has_special {
            warnings.push(
                "Processing special categories without DPO documented - \
                 check if DPO designation required (Article 37)"
                    .to_string(),
            );
        }

        warnings
    }
}

impl Default for ProcessingRecord {
    fn default() -> Self {
        Self::new()
    }
}

/// Validation result for processing record
#[derive(Debug, Clone)]
pub struct RecordValidation {
    /// Whether record is complete per Article 30
    pub complete: bool,

    /// Whether processing includes special categories
    pub contains_special_categories: bool,

    /// Whether processing includes third country transfers
    pub has_third_country_transfers: bool,

    /// Whether DPO is documented
    pub has_dpo: bool,

    /// Warnings about incomplete or problematic information
    pub warnings: Vec<String>,
}

/// Records of Processing Activities (ROPA) - Collection of records
#[derive(Debug, Clone)]
pub struct RecordsOfProcessingActivities {
    /// Organization name
    pub organization: String,

    /// All processing records
    pub records: Vec<ProcessingRecord>,

    /// Date ROPA was last reviewed
    pub last_reviewed: Option<DateTime<Utc>>,
}

impl RecordsOfProcessingActivities {
    pub fn new(organization: impl Into<String>) -> Self {
        Self {
            organization: organization.into(),
            records: Vec::new(),
            last_reviewed: None,
        }
    }

    pub fn add_record(mut self, record: ProcessingRecord) -> Self {
        self.records.push(record);
        self
    }

    pub fn with_last_reviewed(mut self, date: DateTime<Utc>) -> Self {
        self.last_reviewed = Some(date);
        self
    }

    /// Check if organization is exempt from ROPA requirement
    ///
    /// Exemption applies if ALL of the following are true:
    /// - Enterprise with <250 employees
    /// - Processing is occasional
    /// - Processing not likely to result in risk
    /// - Does not include special categories or criminal data
    pub fn is_exempt(&self, employee_count: u32) -> RopaExemption {
        // Article 30(5) exemption
        if employee_count >= 250 {
            return RopaExemption::NotExempt {
                reason: "Organization has 250+ employees (Article 30(5))".to_string(),
            };
        }

        // Check for special categories or high risk
        for record in &self.records {
            let has_special = record
                .data_categories
                .iter()
                .any(|cat| matches!(cat, PersonalDataCategory::Special(_)));

            if has_special {
                return RopaExemption::NotExempt {
                    reason: "Processing includes special categories of data (Article 30(5))"
                        .to_string(),
                };
            }

            // If processing is systematic or large-scale, not occasional
            if record.data_subject_categories.iter().any(|cat| {
                cat.contains("customer") || cat.contains("employee") || cat.contains("patient")
            }) {
                return RopaExemption::NotExempt {
                    reason: "Processing is not occasional (systematic processing of customers/employees/patients)".to_string(),
                };
            }
        }

        RopaExemption::Exempt
    }

    /// Validate entire ROPA
    pub fn validate(&self) -> Result<RopaValidation, GdprError> {
        if self.records.is_empty() {
            return Err(GdprError::InvalidValue {
                field: "records".to_string(),
                reason: "ROPA contains no processing records".to_string(),
            });
        }

        let mut validations = Vec::new();
        for record in &self.records {
            validations.push(record.validate()?);
        }

        let total_records = validations.len();
        let complete_records = validations.iter().filter(|v| v.complete).count();
        let records_with_special_categories = validations
            .iter()
            .filter(|v| v.contains_special_categories)
            .count();
        let records_with_transfers = validations
            .iter()
            .filter(|v| v.has_third_country_transfers)
            .count();

        Ok(RopaValidation {
            total_records,
            complete_records,
            records_with_special_categories,
            records_with_transfers,
            all_records_complete: total_records == complete_records,
            record_validations: validations,
        })
    }
}

/// ROPA exemption status
#[derive(Debug, Clone, PartialEq)]
pub enum RopaExemption {
    /// Organization is exempt from ROPA (Article 30(5))
    Exempt,
    /// Organization is not exempt
    NotExempt { reason: String },
}

/// ROPA validation result
#[derive(Debug, Clone)]
pub struct RopaValidation {
    /// Total number of processing records
    pub total_records: usize,

    /// Number of complete records
    pub complete_records: usize,

    /// Number of records processing special categories
    pub records_with_special_categories: usize,

    /// Number of records with third country transfers
    pub records_with_transfers: usize,

    /// Whether all records are complete
    pub all_records_complete: bool,

    /// Individual record validations
    pub record_validations: Vec<RecordValidation>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete_controller_record() {
        let record = ProcessingRecord::new()
            .with_entity_type(EntityType::Controller)
            .with_name("CRM System")
            .with_controller("Acme Corp", "privacy@acme.com")
            .with_purpose("Customer relationship management")
            .add_data_subject_category("customers")
            .add_data_category(PersonalDataCategory::Regular("name".to_string()))
            .add_data_category(PersonalDataCategory::Regular("email".to_string()))
            .add_recipient("Sales team (internal)")
            .with_retention_period("7 years after last contact")
            .add_security_measure("Encryption at rest")
            .add_security_measure("Access controls")
            .add_security_measure("Audit logging");

        let result = record.validate().unwrap();
        assert!(result.complete);
        assert!(!result.contains_special_categories);
    }

    #[test]
    fn test_missing_controller_details() {
        let record = ProcessingRecord::new()
            .with_entity_type(EntityType::Controller)
            .with_purpose("Test");

        let result = record.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_processor_record() {
        let record = ProcessingRecord::new()
            .with_entity_type(EntityType::Processor)
            .with_processor("Cloud Provider", "dpo@cloud.com")
            .with_purpose("Data storage and backup")
            .add_security_measure("Encryption")
            .add_security_measure("Access controls")
            .add_security_measure("Regular backups");

        let result = record.validate().unwrap();
        assert!(result.complete);
    }

    #[test]
    fn test_special_categories_warning() {
        let record = ProcessingRecord::new()
            .with_controller("Hospital", "privacy@hospital.com")
            .with_purpose("Patient care")
            .add_data_subject_category("patients")
            .add_data_category(PersonalDataCategory::Special(SpecialCategory::HealthData))
            .with_retention_period("10 years")
            .add_security_measure("Encryption")
            .add_security_measure("MFA")
            .add_security_measure("Audit logs");

        let result = record.validate().unwrap();
        assert!(result.contains_special_categories);
        assert!(!result.warnings.is_empty());
    }

    #[test]
    fn test_ropa_exemption_large_organization() {
        let ropa = RecordsOfProcessingActivities::new("Big Corp")
            .add_record(ProcessingRecord::new().with_controller("Big Corp", "privacy@big.com"));

        let exemption = ropa.is_exempt(500); // 500 employees
        assert!(matches!(exemption, RopaExemption::NotExempt { .. }));
    }

    #[test]
    fn test_ropa_exemption_special_categories() {
        let ropa = RecordsOfProcessingActivities::new("Small Clinic").add_record(
            ProcessingRecord::new()
                .with_controller("Small Clinic", "privacy@clinic.com")
                .add_data_category(PersonalDataCategory::Special(SpecialCategory::HealthData)),
        );

        let exemption = ropa.is_exempt(50); // <250 employees
        assert!(matches!(exemption, RopaExemption::NotExempt { .. }));
    }

    #[test]
    fn test_ropa_validation() {
        let ropa = RecordsOfProcessingActivities::new("Acme Corp")
            .add_record(
                ProcessingRecord::new()
                    .with_controller("Acme Corp", "privacy@acme.com")
                    .with_purpose("Marketing")
                    .add_data_subject_category("subscribers")
                    .add_data_category(PersonalDataCategory::Regular("email".to_string()))
                    .with_retention_period("Until unsubscribe")
                    .add_security_measure("Encryption"),
            )
            .add_record(
                ProcessingRecord::new()
                    .with_controller("Acme Corp", "privacy@acme.com")
                    .with_purpose("HR")
                    .add_data_subject_category("employees")
                    .add_data_category(PersonalDataCategory::Regular("name".to_string()))
                    .with_retention_period("7 years after termination")
                    .add_security_measure("Access controls"),
            );

        let result = ropa.validate().unwrap();
        assert_eq!(result.total_records, 2);
        assert_eq!(result.complete_records, 2);
        assert!(result.all_records_complete);
    }

    #[test]
    fn test_third_country_transfer_warning() {
        let record = ProcessingRecord::new()
            .with_controller("Corp", "privacy@corp.com")
            .with_purpose("Cloud storage")
            .add_data_subject_category("customers")
            .add_data_category(PersonalDataCategory::Regular("name".to_string()))
            .with_retention_period("5 years")
            .add_security_measure("Encryption")
            .add_third_country_transfer(ThirdCountryTransfer {
                country: "US".to_string(),
                safeguard: "Standard Contractual Clauses".to_string(),
                documentation: None, // Missing documentation
            });

        let result = record.validate().unwrap();
        assert!(result.has_third_country_transfers);
        assert!(result.warnings.iter().any(|w| w.contains("documentation")));
    }
}
