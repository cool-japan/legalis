//! GDPR Article 28 - Processor Contracts
//!
//! This module implements the mandatory requirements for contracts or other legal acts
//! between controllers and processors under GDPR Article 28.
//!
//! ## Key Requirements
//!
//! **Article 28(1) - Written Contract Requirement**
//!
//! Processing by a processor shall be governed by a contract or other legal act that:
//! - Is in writing (including electronic format)
//! - Is binding on the processor
//! - Sets out the subject matter and duration of processing
//! - Sets out the nature and purpose of processing
//! - Sets out the type of personal data and categories of data subjects
//! - Sets out the obligations and rights of the controller
//!
//! **Article 28(3) - Mandatory Contract Clauses**
//!
//! The contract must stipulate that the processor will:
//! - (a) Process personal data only on documented instructions from controller
//! - (b) Ensure persons processing data are under confidentiality obligation
//! - (c) Take appropriate security measures (Article 32)
//! - (d) Respect sub-processor conditions (Article 28(2) and 28(4))
//! - (e) Assist controller with data subject rights (Chapter III)
//! - (f) Assist controller with security, breach notification, DPIA, prior consultation
//! - (g) Delete or return all personal data after end of processing services
//! - (h) Make available information to demonstrate compliance and allow audits
//!
//! **Article 28(2) - Sub-Processors**
//!
//! The processor shall not engage another processor (sub-processor) without:
//! - Prior specific written authorization from controller (specific authorization), OR
//! - Prior general written authorization with notification of changes (general authorization)
//!
//! **Article 28(4) - Sub-Processor Obligations**
//!
//! Where processor engages sub-processor:
//! - Same data protection obligations as in main contract must be imposed
//! - Processor remains fully liable to controller for sub-processor's performance
//!
//! ## EUR-Lex References
//!
//! - Article 28: CELEX:32016R0679 Art. 28
//! - Recital 81: Importance of written contracts
//! - WP29 Guidelines on Contracts (WP247)
//!
//! ## Example Usage
//!
//! ```rust
//! use legalis_eu::gdpr::processor_contract::*;
//! use chrono::Utc;
//!
//! let contract = ProcessorContract::new()
//!     .with_controller("Acme Corp", "controller@acme.com")
//!     .with_processor("CloudService GmbH", "processor@cloudservice.de")
//!     .with_subject_matter("Customer data processing and storage")
//!     .with_duration_months(24)
//!     .with_processing_nature("Cloud-based CRM system")
//!     .with_processing_purpose("Customer relationship management")
//!     .add_data_category("customer contact details")
//!     .add_data_subject_category("customers")
//!     .with_clause(Article28Clause::ProcessOnlyOnInstructions)
//!     .with_clause(Article28Clause::ConfidentialityObligation)
//!     .with_clause(Article28Clause::SecurityMeasures)
//!     .with_clause(Article28Clause::SubProcessorConditions)
//!     .with_clause(Article28Clause::AssistDataSubjectRights)
//!     .with_clause(Article28Clause::AssistSecurity)
//!     .with_clause(Article28Clause::DeletionOrReturn)
//!     .with_clause(Article28Clause::AuditsAndInspections);
//!
//! match contract.validate() {
//!     Ok(validation) => {
//!         if validation.compliant {
//!             println!("✅ Contract complies with Article 28");
//!         }
//!     }
//!     Err(e) => println!("❌ Error: {}", e),
//! }
//! ```

use crate::gdpr::error::GdprError;
use crate::gdpr::types::PersonalDataCategory;
use chrono::{DateTime, Utc};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ========================================
// Article 28(3) - Mandatory Contract Clauses
// ========================================

/// Mandatory clauses that must be in processor contract (Article 28(3))
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Article28Clause {
    /// (a) Process data only on documented instructions from controller
    /// Including transfers to third countries
    ProcessOnlyOnInstructions,

    /// (b) Ensure persons authorized to process data are under confidentiality obligation
    ConfidentialityObligation,

    /// (c) Take all measures required pursuant to Article 32 (security)
    SecurityMeasures,

    /// (d) Respect conditions for engaging sub-processors (Article 28(2) and (4))
    SubProcessorConditions,

    /// (e) Assist controller in responding to data subject rights requests (Chapter III)
    AssistDataSubjectRights,

    /// (f) Assist controller with security, breach notification, DPIA, prior consultation
    AssistSecurity,

    /// (g) Delete or return all personal data after end of services
    /// (unless required to store by EU/Member State law)
    DeletionOrReturn,

    /// (h) Make available all information necessary to demonstrate compliance
    /// Allow and contribute to audits and inspections
    AuditsAndInspections,
}

impl Article28Clause {
    /// Get all 8 mandatory clauses
    pub fn all_mandatory() -> Vec<Article28Clause> {
        vec![
            Article28Clause::ProcessOnlyOnInstructions,
            Article28Clause::ConfidentialityObligation,
            Article28Clause::SecurityMeasures,
            Article28Clause::SubProcessorConditions,
            Article28Clause::AssistDataSubjectRights,
            Article28Clause::AssistSecurity,
            Article28Clause::DeletionOrReturn,
            Article28Clause::AuditsAndInspections,
        ]
    }

    /// Get article reference for this clause
    pub fn article_reference(&self) -> &'static str {
        match self {
            Article28Clause::ProcessOnlyOnInstructions => "Article 28(3)(a)",
            Article28Clause::ConfidentialityObligation => "Article 28(3)(b)",
            Article28Clause::SecurityMeasures => "Article 28(3)(c)",
            Article28Clause::SubProcessorConditions => "Article 28(3)(d)",
            Article28Clause::AssistDataSubjectRights => "Article 28(3)(e)",
            Article28Clause::AssistSecurity => "Article 28(3)(f)",
            Article28Clause::DeletionOrReturn => "Article 28(3)(g)",
            Article28Clause::AuditsAndInspections => "Article 28(3)(h)",
        }
    }

    /// Get description of this clause requirement
    pub fn description(&self) -> &'static str {
        match self {
            Article28Clause::ProcessOnlyOnInstructions => {
                "Processor must process personal data only on documented instructions from \
                 controller, including with regard to transfers to third countries"
            }
            Article28Clause::ConfidentialityObligation => {
                "Processor must ensure that persons authorized to process personal data have \
                 committed themselves to confidentiality or are under appropriate statutory \
                 obligation of confidentiality"
            }
            Article28Clause::SecurityMeasures => {
                "Processor must take all measures required pursuant to Article 32 (security of \
                 processing)"
            }
            Article28Clause::SubProcessorConditions => {
                "Processor must respect conditions for engaging another processor (sub-processor) \
                 under Article 28(2) and (4)"
            }
            Article28Clause::AssistDataSubjectRights => {
                "Processor must assist controller in responding to requests for exercising data \
                 subject rights under Chapter III"
            }
            Article28Clause::AssistSecurity => {
                "Processor must assist controller in ensuring compliance with obligations under \
                 Articles 32-36 (security, breach notification, DPIA, prior consultation)"
            }
            Article28Clause::DeletionOrReturn => {
                "Processor must delete or return all personal data to controller after end of \
                 provision of services, unless EU or Member State law requires storage"
            }
            Article28Clause::AuditsAndInspections => {
                "Processor must make available to controller all information necessary to \
                 demonstrate compliance and allow for and contribute to audits and inspections"
            }
        }
    }
}

// ========================================
// Article 28(2) - Sub-Processor Authorization
// ========================================

/// Type of sub-processor authorization (Article 28(2))
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SubProcessorAuthorization {
    /// Specific written authorization for each sub-processor
    /// Controller must approve each sub-processor individually
    Specific {
        /// List of specifically authorized sub-processors
        authorized_processors: Vec<String>,
    },

    /// General written authorization with notification of changes
    /// Controller has right to object to changes
    General {
        /// Objection period in days (reasonable period for controller to object)
        objection_period_days: u32,
        /// Current sub-processors under general authorization
        current_processors: Vec<String>,
    },
}

/// Sub-processor entry
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SubProcessor {
    /// Name of sub-processor
    pub name: String,
    /// Email or contact details
    pub contact: String,
    /// Processing activities performed by sub-processor
    pub activities: Vec<String>,
    /// Authorization date
    pub authorized_date: Option<DateTime<Utc>>,
}

// ========================================
// Article 28(1) - Contract Elements
// ========================================

/// Party to a processor contract
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ContractParty {
    /// Organization name
    pub name: String,
    /// Contact email
    pub email: String,
    /// Address (optional)
    pub address: Option<String>,
    /// Legal representative (optional)
    pub representative: Option<String>,
}

impl ContractParty {
    pub fn new(name: impl Into<String>, email: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            email: email.into(),
            address: None,
            representative: None,
        }
    }
}

/// Duration specification for contract
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ContractDuration {
    /// Fixed duration (start + end dates)
    Fixed {
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    },

    /// Fixed duration in months from signing
    Months(u32),

    /// Indefinite duration with termination notice period
    Indefinite { notice_period_days: u32 },
}

// ========================================
// Article 28 - Complete Processor Contract
// ========================================

/// Complete processor contract compliant with Article 28 GDPR
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProcessorContract {
    /// Controller (data controller)
    pub controller: Option<ContractParty>,

    /// Processor (data processor)
    pub processor: Option<ContractParty>,

    /// Subject matter of processing (Article 28(1))
    pub subject_matter: Option<String>,

    /// Duration of processing (Article 28(1))
    pub duration: Option<ContractDuration>,

    /// Nature of processing (Article 28(1))
    pub processing_nature: Option<String>,

    /// Purpose of processing (Article 28(1))
    pub processing_purpose: Option<String>,

    /// Type of personal data (Article 28(1))
    pub data_categories: Vec<PersonalDataCategory>,

    /// Categories of data subjects (Article 28(1))
    pub data_subject_categories: Vec<String>,

    /// Mandatory clauses included (Article 28(3))
    pub clauses: Vec<Article28Clause>,

    /// Sub-processor authorization model (Article 28(2))
    pub sub_processor_authorization: Option<SubProcessorAuthorization>,

    /// List of sub-processors (Article 28(4))
    pub sub_processors: Vec<SubProcessor>,

    /// Contract signing date
    pub signed_date: Option<DateTime<Utc>>,

    /// Is contract in writing (including electronic)? (Article 28(1))
    pub in_writing: Option<bool>,

    /// Additional notes
    pub notes: Option<String>,
}

impl Default for ProcessorContract {
    fn default() -> Self {
        Self::new()
    }
}

impl ProcessorContract {
    /// Create new processor contract
    pub fn new() -> Self {
        Self {
            controller: None,
            processor: None,
            subject_matter: None,
            duration: None,
            processing_nature: None,
            processing_purpose: None,
            data_categories: Vec::new(),
            data_subject_categories: Vec::new(),
            clauses: Vec::new(),
            sub_processor_authorization: None,
            sub_processors: Vec::new(),
            signed_date: None,
            in_writing: None,
            notes: None,
        }
    }

    /// Set controller party
    pub fn with_controller(mut self, name: impl Into<String>, email: impl Into<String>) -> Self {
        self.controller = Some(ContractParty::new(name, email));
        self
    }

    /// Set controller with full details
    pub fn with_controller_details(mut self, controller: ContractParty) -> Self {
        self.controller = Some(controller);
        self
    }

    /// Set processor party
    pub fn with_processor(mut self, name: impl Into<String>, email: impl Into<String>) -> Self {
        self.processor = Some(ContractParty::new(name, email));
        self
    }

    /// Set processor with full details
    pub fn with_processor_details(mut self, processor: ContractParty) -> Self {
        self.processor = Some(processor);
        self
    }

    /// Set subject matter of processing
    pub fn with_subject_matter(mut self, subject_matter: impl Into<String>) -> Self {
        self.subject_matter = Some(subject_matter.into());
        self
    }

    /// Set duration in months
    pub fn with_duration_months(mut self, months: u32) -> Self {
        self.duration = Some(ContractDuration::Months(months));
        self
    }

    /// Set fixed duration with dates
    pub fn with_duration_fixed(mut self, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        self.duration = Some(ContractDuration::Fixed {
            start_date: start,
            end_date: end,
        });
        self
    }

    /// Set indefinite duration with notice period
    pub fn with_duration_indefinite(mut self, notice_period_days: u32) -> Self {
        self.duration = Some(ContractDuration::Indefinite { notice_period_days });
        self
    }

    /// Set nature of processing
    pub fn with_processing_nature(mut self, nature: impl Into<String>) -> Self {
        self.processing_nature = Some(nature.into());
        self
    }

    /// Set purpose of processing
    pub fn with_processing_purpose(mut self, purpose: impl Into<String>) -> Self {
        self.processing_purpose = Some(purpose.into());
        self
    }

    /// Add data category
    pub fn add_data_category(mut self, category: impl Into<String>) -> Self {
        self.data_categories
            .push(PersonalDataCategory::Regular(category.into()));
        self
    }

    /// Add data subject category
    pub fn add_data_subject_category(mut self, category: impl Into<String>) -> Self {
        self.data_subject_categories.push(category.into());
        self
    }

    /// Add mandatory clause
    pub fn with_clause(mut self, clause: Article28Clause) -> Self {
        if !self.clauses.contains(&clause) {
            self.clauses.push(clause);
        }
        self
    }

    /// Add all mandatory clauses
    pub fn with_all_mandatory_clauses(mut self) -> Self {
        self.clauses = Article28Clause::all_mandatory();
        self
    }

    /// Set sub-processor authorization to specific
    pub fn with_specific_sub_processor_auth(mut self, processors: Vec<String>) -> Self {
        self.sub_processor_authorization = Some(SubProcessorAuthorization::Specific {
            authorized_processors: processors,
        });
        self
    }

    /// Set sub-processor authorization to general
    pub fn with_general_sub_processor_auth(
        mut self,
        objection_period_days: u32,
        current_processors: Vec<String>,
    ) -> Self {
        self.sub_processor_authorization = Some(SubProcessorAuthorization::General {
            objection_period_days,
            current_processors,
        });
        self
    }

    /// Add sub-processor
    pub fn add_sub_processor(mut self, sub_processor: SubProcessor) -> Self {
        self.sub_processors.push(sub_processor);
        self
    }

    /// Set signed date
    pub fn with_signed_date(mut self, date: DateTime<Utc>) -> Self {
        self.signed_date = Some(date);
        self
    }

    /// Mark contract as in writing
    pub fn in_writing(mut self, in_writing: bool) -> Self {
        self.in_writing = Some(in_writing);
        self
    }

    /// Add notes
    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    /// Validate processor contract compliance with Article 28
    pub fn validate(&self) -> Result<ProcessorContractValidation, GdprError> {
        let mut warnings = Vec::new();
        let mut missing_clauses = Vec::new();

        // Article 28(1): Written requirement
        if self.in_writing != Some(true) {
            warnings.push(
                "Contract must be in writing (including electronic format) - Article 28(1)"
                    .to_string(),
            );
        }

        // Article 28(1): Parties
        if self.controller.is_none() {
            warnings.push("Controller details are missing - Article 28(1)".to_string());
        }
        if self.processor.is_none() {
            warnings.push("Processor details are missing - Article 28(1)".to_string());
        }

        // Article 28(1): Subject matter
        if self.subject_matter.is_none() {
            warnings.push("Subject matter of processing is missing - Article 28(1)".to_string());
        }

        // Article 28(1): Duration
        if self.duration.is_none() {
            warnings.push("Duration of processing is missing - Article 28(1)".to_string());
        }

        // Article 28(1): Nature
        if self.processing_nature.is_none() {
            warnings.push("Nature of processing is missing - Article 28(1)".to_string());
        }

        // Article 28(1): Purpose
        if self.processing_purpose.is_none() {
            warnings.push("Purpose of processing is missing - Article 28(1)".to_string());
        }

        // Article 28(1): Data types
        if self.data_categories.is_empty() {
            warnings.push("Type of personal data is missing - Article 28(1)".to_string());
        }

        // Article 28(1): Data subjects
        if self.data_subject_categories.is_empty() {
            warnings.push("Categories of data subjects are missing - Article 28(1)".to_string());
        }

        // Article 28(3): Check all 8 mandatory clauses
        for mandatory_clause in Article28Clause::all_mandatory() {
            if !self.clauses.contains(&mandatory_clause) {
                missing_clauses.push(mandatory_clause);
                warnings.push(format!(
                    "Missing mandatory clause: {} - {}",
                    mandatory_clause.article_reference(),
                    mandatory_clause.description()
                ));
            }
        }

        // Article 28(2): Sub-processor authorization
        if !self.sub_processors.is_empty() && self.sub_processor_authorization.is_none() {
            warnings.push(
                "Sub-processors are listed but authorization model is not specified - Article 28(2)"
                    .to_string(),
            );
        }

        // Determine compliance
        let compliant = warnings.is_empty();
        let has_all_mandatory_clauses = missing_clauses.is_empty();

        Ok(ProcessorContractValidation {
            compliant,
            warnings,
            missing_clauses,
            has_all_mandatory_clauses,
            has_controller: self.controller.is_some(),
            has_processor: self.processor.is_some(),
            has_subject_matter: self.subject_matter.is_some(),
            has_duration: self.duration.is_some(),
            has_data_categories: !self.data_categories.is_empty(),
            in_writing: self.in_writing.unwrap_or(false),
            sub_processor_count: self.sub_processors.len(),
        })
    }
}

/// Validation result for processor contract
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProcessorContractValidation {
    /// Overall compliance with Article 28
    pub compliant: bool,

    /// List of warnings (non-compliance issues)
    pub warnings: Vec<String>,

    /// Missing mandatory clauses
    pub missing_clauses: Vec<Article28Clause>,

    /// Has all 8 mandatory clauses (Article 28(3))
    pub has_all_mandatory_clauses: bool,

    /// Has controller details (Article 28(1))
    pub has_controller: bool,

    /// Has processor details (Article 28(1))
    pub has_processor: bool,

    /// Has subject matter (Article 28(1))
    pub has_subject_matter: bool,

    /// Has duration (Article 28(1))
    pub has_duration: bool,

    /// Has data categories (Article 28(1))
    pub has_data_categories: bool,

    /// Is in writing (Article 28(1))
    pub in_writing: bool,

    /// Number of sub-processors
    pub sub_processor_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_mandatory_clauses() {
        let clauses = Article28Clause::all_mandatory();
        assert_eq!(clauses.len(), 8);
    }

    #[test]
    fn test_complete_contract_validation() {
        let contract = ProcessorContract::new()
            .with_controller("Acme Corp", "controller@acme.com")
            .with_processor("CloudService GmbH", "processor@cloudservice.de")
            .with_subject_matter("Customer data processing")
            .with_duration_months(24)
            .with_processing_nature("Cloud storage")
            .with_processing_purpose("Data backup")
            .add_data_category("customer names")
            .add_data_subject_category("customers")
            .with_all_mandatory_clauses()
            .in_writing(true);

        let validation = contract.validate().unwrap();

        assert!(validation.compliant);
        assert!(validation.has_all_mandatory_clauses);
        assert!(validation.warnings.is_empty());
    }

    #[test]
    fn test_missing_mandatory_clauses() {
        let contract = ProcessorContract::new()
            .with_controller("Acme Corp", "controller@acme.com")
            .with_processor("CloudService GmbH", "processor@cloudservice.de")
            .with_subject_matter("Data processing")
            .with_duration_months(12)
            .with_processing_nature("Processing")
            .with_processing_purpose("Storage")
            .add_data_category("data")
            .add_data_subject_category("users")
            .with_clause(Article28Clause::ProcessOnlyOnInstructions)
            .in_writing(true);

        let validation = contract.validate().unwrap();

        assert!(!validation.compliant);
        assert!(!validation.has_all_mandatory_clauses);
        assert_eq!(validation.missing_clauses.len(), 7);
    }

    #[test]
    fn test_missing_article_28_1_elements() {
        let contract = ProcessorContract::new()
            .with_all_mandatory_clauses()
            .in_writing(true);

        let validation = contract.validate().unwrap();

        assert!(!validation.compliant);
        assert!(!validation.warnings.is_empty());
        assert!(
            validation
                .warnings
                .iter()
                .any(|w| w.contains("Controller details"))
        );
        assert!(
            validation
                .warnings
                .iter()
                .any(|w| w.contains("Processor details"))
        );
    }

    #[test]
    fn test_sub_processor_authorization_warning() {
        let contract = ProcessorContract::new()
            .with_controller("Acme", "a@a.com")
            .with_processor("Processor", "p@p.com")
            .with_subject_matter("Processing")
            .with_duration_months(12)
            .with_processing_nature("Nature")
            .with_processing_purpose("Purpose")
            .add_data_category("data")
            .add_data_subject_category("users")
            .with_all_mandatory_clauses()
            .in_writing(true)
            .add_sub_processor(SubProcessor {
                name: "SubProc".to_string(),
                contact: "sub@sub.com".to_string(),
                activities: vec!["Storage".to_string()],
                authorized_date: Some(Utc::now()),
            });

        let validation = contract.validate().unwrap();

        assert!(!validation.compliant);
        assert!(
            validation
                .warnings
                .iter()
                .any(|w| w.contains("Sub-processors are listed"))
        );
    }

    #[test]
    fn test_specific_sub_processor_authorization() {
        let contract = ProcessorContract::new()
            .with_controller("Acme", "a@a.com")
            .with_processor("Processor", "p@p.com")
            .with_subject_matter("Processing")
            .with_duration_months(12)
            .with_processing_nature("Nature")
            .with_processing_purpose("Purpose")
            .add_data_category("data")
            .add_data_subject_category("users")
            .with_all_mandatory_clauses()
            .in_writing(true)
            .with_specific_sub_processor_auth(vec!["SubProc1".to_string()])
            .add_sub_processor(SubProcessor {
                name: "SubProc1".to_string(),
                contact: "sub@sub.com".to_string(),
                activities: vec!["Storage".to_string()],
                authorized_date: Some(Utc::now()),
            });

        let validation = contract.validate().unwrap();

        assert!(validation.compliant);
        assert_eq!(validation.sub_processor_count, 1);
    }

    #[test]
    fn test_general_sub_processor_authorization() {
        let contract = ProcessorContract::new()
            .with_controller("Acme", "a@a.com")
            .with_processor("Processor", "p@p.com")
            .with_subject_matter("Processing")
            .with_duration_months(12)
            .with_processing_nature("Nature")
            .with_processing_purpose("Purpose")
            .add_data_category("data")
            .add_data_subject_category("users")
            .with_all_mandatory_clauses()
            .in_writing(true)
            .with_general_sub_processor_auth(30, vec!["SubProc1".to_string()]);

        let validation = contract.validate().unwrap();

        assert!(validation.compliant);
    }
}
