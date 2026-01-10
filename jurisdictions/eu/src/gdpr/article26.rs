//! GDPR Article 26 - Joint Controllers
//!
//! This module implements Article 26 of the GDPR, which governs situations where
//! two or more controllers jointly determine the purposes and means of processing
//! personal data.
//!
//! ## Article 26 Structure
//!
//! - **Article 26(1)**: Joint controllers must determine their respective responsibilities
//!   in a transparent manner by means of an arrangement
//!
//! - **Article 26(2)**: The arrangement must duly reflect the respective roles and
//!   relationships of the joint controllers vis-à-vis the data subjects
//!
//! - **Article 26(3)**: Data subjects can exercise their rights under the GDPR against
//!   each of the joint controllers
//!
//! ## Key Concepts
//!
//! **Joint Controllership** exists when:
//! 1. Two or more controllers exist
//! 2. They jointly determine the purposes of processing
//! 3. They jointly determine the means of processing
//! 4. The determination is made in a joint decision
//!
//! **CJEU Case Law:**
//! - **Fashion ID (C-40/17)**: Joint controllership can exist even if one party has
//!   more influence than another
//! - **Wirtschaftsakademie (C-210/16)**: Facebook page administrators are joint controllers
//!   with Facebook
//! - **Google Spain (C-131/12)**: Search engines can be joint controllers with website operators
//!
//! ## Article 26 vs Article 28
//!
//! | Article 26 (Joint Controllers) | Article 28 (Processor) |
//! |-------------------------------|------------------------|
//! | Jointly determine purposes | Controller determines purposes |
//! | Jointly determine means | Processor follows instructions |
//! | Equal responsibility | Limited liability |
//! | Arrangement required | Contract required |
//! | Data subjects can sue either | Data subjects sue controller |
//!
//! ## References
//!
//! - EUR-Lex: CELEX:32016R0679 - Article 26
//! - CJEU C-40/17 Fashion ID (joint controllership definition)
//! - CJEU C-210/16 Wirtschaftsakademie Schleswig-Holstein (Facebook page admins)
//! - WP29 Guidelines on Controllers and Processors (WP 169)
//! - EDPB Guidelines 07/2020 on controllers and processors
//!
//! ## Example
//!
//! ```rust
//! use legalis_eu::gdpr::*;
//! use chrono::Utc;
//!
//! // Example: Two universities running a joint research project
//! let arrangement = JointControllerArrangement::new()
//!     .add_controller(JointController {
//!         name: "University of Munich".to_string(),
//!         contact: Some("dpo@uni-munich.de".to_string()),
//!         responsibilities: vec![
//!             Responsibility::DataCollection,
//!             Responsibility::DataStorage,
//!         ],
//!         is_contact_point: true,
//!         notes: None,
//!     })
//!     .add_controller(JointController {
//!         name: "ETH Zurich".to_string(),
//!         contact: Some("privacy@ethz.ch".to_string()),
//!         responsibilities: vec![
//!             Responsibility::DataAnalysis,
//!             Responsibility::DataDeletion,
//!         ],
//!         is_contact_point: false,
//!         notes: None,
//!     })
//!     .with_processing_purpose("Joint COVID-19 research study")
//!     .with_data_categories(vec![
//!         PersonalDataCategory::Regular("name".to_string()),
//!         PersonalDataCategory::Special(SpecialCategory::HealthData),
//!     ])
//!     .with_arrangement_documented(true)
//!     .with_essence_available_to_data_subjects(true)
//!     .with_assessment_date(Utc::now());
//!
//! match arrangement.validate() {
//!     Ok(validation) => {
//!         if validation.compliant {
//!             println!("✅ Joint controller arrangement is Article 26 compliant");
//!         }
//!     }
//!     Err(e) => println!("❌ Error: {}", e),
//! }
//! ```

use crate::gdpr::error::GdprError;
use crate::gdpr::types::PersonalDataCategory;
use chrono::{DateTime, Utc};

#[cfg(test)]
use crate::gdpr::types::SpecialCategory;

/// Responsibilities that can be allocated between joint controllers (Article 26(1))
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Responsibility {
    /// Data collection and initial processing
    DataCollection,

    /// Storage and retention of personal data
    DataStorage,

    /// Data analysis and processing operations
    DataAnalysis,

    /// Security measures (Article 32)
    SecurityMeasures,

    /// Handling data subject rights requests (Articles 15-22)
    DataSubjectRights,

    /// Breach notification (Articles 33-34)
    BreachNotification,

    /// Data protection impact assessment (Article 35)
    DataProtectionImpactAssessment,

    /// Records of processing activities (Article 30)
    RecordsOfProcessing,

    /// Third-party disclosures
    ThirdPartyDisclosure,

    /// International data transfers (Articles 44-49)
    InternationalTransfers,

    /// Data deletion and retention
    DataDeletion,

    /// Consent management (if consent is lawful basis)
    ConsentManagement,

    /// Contact point for data subjects (Article 26(1) second sentence)
    ContactPoint,

    /// Other custom responsibility
    Other { description: String },
}

impl Responsibility {
    /// Returns true if this is a mandatory responsibility that must be allocated
    pub fn is_mandatory(&self) -> bool {
        matches!(
            self,
            Responsibility::DataSubjectRights
                | Responsibility::BreachNotification
                | Responsibility::SecurityMeasures
                | Responsibility::ContactPoint
        )
    }

    /// Returns the GDPR article(s) this responsibility relates to
    pub fn article_reference(&self) -> &'static str {
        match self {
            Responsibility::DataCollection => "Articles 5-6",
            Responsibility::DataStorage => "Article 5(1)(e)",
            Responsibility::DataAnalysis => "Article 6",
            Responsibility::SecurityMeasures => "Article 32",
            Responsibility::DataSubjectRights => "Articles 15-22",
            Responsibility::BreachNotification => "Articles 33-34",
            Responsibility::DataProtectionImpactAssessment => "Article 35",
            Responsibility::RecordsOfProcessing => "Article 30",
            Responsibility::ThirdPartyDisclosure => "Article 13-14",
            Responsibility::InternationalTransfers => "Articles 44-49",
            Responsibility::DataDeletion => "Article 17",
            Responsibility::ConsentManagement => "Article 7",
            Responsibility::ContactPoint => "Article 26(1)",
            Responsibility::Other { .. } => "Article 26",
        }
    }
}

/// A joint controller in a joint controller arrangement
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct JointController {
    /// Name of the controller
    pub name: String,

    /// Contact information (email or URL)
    pub contact: Option<String>,

    /// Responsibilities allocated to this controller
    pub responsibilities: Vec<Responsibility>,

    /// Whether this controller is the contact point for data subjects (Article 26(1))
    pub is_contact_point: bool,

    /// Additional notes
    pub notes: Option<String>,
}

impl JointController {
    /// Create a new joint controller
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            contact: None,
            responsibilities: Vec::new(),
            is_contact_point: false,
            notes: None,
        }
    }

    /// Set contact information
    pub fn with_contact(mut self, contact: impl Into<String>) -> Self {
        self.contact = Some(contact.into());
        self
    }

    /// Add a responsibility
    pub fn add_responsibility(mut self, responsibility: Responsibility) -> Self {
        self.responsibilities.push(responsibility);
        self
    }

    /// Set as contact point for data subjects
    pub fn as_contact_point(mut self) -> Self {
        self.is_contact_point = true;
        self
    }

    /// Set notes
    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }
}

/// Legal basis for joint controllership determination
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum JointControllershipBasis {
    /// Joint decision on purposes and means
    JointDecision {
        purposes_jointly_determined: bool,
        means_jointly_determined: bool,
    },

    /// Common purpose with converging interests (Fashion ID pattern)
    CommonPurpose {
        purpose: String,
        interests_converge: bool,
    },

    /// Platform and user relationship (Wirtschaftsakademie pattern)
    PlatformUser {
        platform: String,
        user: String,
        joint_benefit: bool,
    },

    /// Contractual joint venture
    ContractualJointVenture { contract_type: String },

    /// Statutory joint controllership
    StatutoryRequirement { legal_basis: String },
}

/// Joint controller arrangement (Article 26)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct JointControllerArrangement {
    /// Controllers in the joint arrangement
    pub controllers: Vec<JointController>,

    /// Purpose(s) of the joint processing
    pub processing_purpose: Option<String>,

    /// Categories of personal data processed jointly
    pub data_categories: Vec<PersonalDataCategory>,

    /// Basis for joint controllership
    pub joint_controllership_basis: Option<JointControllershipBasis>,

    /// Whether the arrangement is documented in writing (Article 26(1))
    pub arrangement_documented: Option<bool>,

    /// Whether the essence is available to data subjects (Article 26(2))
    pub essence_available_to_data_subjects: Option<bool>,

    /// How the essence is made available (URL, document, etc.)
    pub essence_availability_method: Option<String>,

    /// Whether data subjects can exercise rights against each controller (Article 26(3))
    pub rights_exercisable_against_each: Option<bool>,

    /// Date of arrangement
    pub arrangement_date: Option<DateTime<Utc>>,

    /// Date of assessment
    pub assessment_date: Option<DateTime<Utc>>,

    /// Additional notes
    pub notes: Option<String>,
}

impl JointControllerArrangement {
    /// Create a new joint controller arrangement
    pub fn new() -> Self {
        Self {
            controllers: Vec::new(),
            processing_purpose: None,
            data_categories: Vec::new(),
            joint_controllership_basis: None,
            arrangement_documented: None,
            essence_available_to_data_subjects: None,
            essence_availability_method: None,
            rights_exercisable_against_each: None,
            arrangement_date: None,
            assessment_date: None,
            notes: None,
        }
    }

    /// Add a joint controller
    pub fn add_controller(mut self, controller: JointController) -> Self {
        self.controllers.push(controller);
        self
    }

    /// Set processing purpose
    pub fn with_processing_purpose(mut self, purpose: impl Into<String>) -> Self {
        self.processing_purpose = Some(purpose.into());
        self
    }

    /// Add a data category
    pub fn add_data_category(mut self, category: PersonalDataCategory) -> Self {
        self.data_categories.push(category);
        self
    }

    /// Set data categories
    pub fn with_data_categories(mut self, categories: Vec<PersonalDataCategory>) -> Self {
        self.data_categories = categories;
        self
    }

    /// Set joint controllership basis
    pub fn with_joint_controllership_basis(mut self, basis: JointControllershipBasis) -> Self {
        self.joint_controllership_basis = Some(basis);
        self
    }

    /// Set arrangement documented status
    pub fn with_arrangement_documented(mut self, documented: bool) -> Self {
        self.arrangement_documented = Some(documented);
        self
    }

    /// Set essence availability to data subjects
    pub fn with_essence_available_to_data_subjects(mut self, available: bool) -> Self {
        self.essence_available_to_data_subjects = Some(available);
        self
    }

    /// Set how essence is made available
    pub fn with_essence_availability_method(mut self, method: impl Into<String>) -> Self {
        self.essence_availability_method = Some(method.into());
        self
    }

    /// Set whether rights are exercisable against each controller
    pub fn with_rights_exercisable_against_each(mut self, exercisable: bool) -> Self {
        self.rights_exercisable_against_each = Some(exercisable);
        self
    }

    /// Set arrangement date
    pub fn with_arrangement_date(mut self, date: DateTime<Utc>) -> Self {
        self.arrangement_date = Some(date);
        self
    }

    /// Set assessment date
    pub fn with_assessment_date(mut self, date: DateTime<Utc>) -> Self {
        self.assessment_date = Some(date);
        self
    }

    /// Set notes
    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    /// Validate the joint controller arrangement (Article 26)
    pub fn validate(&self) -> Result<Article26Validation, GdprError> {
        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();

        // 1. Check that at least two controllers exist
        if self.controllers.len() < 2 {
            return Err(GdprError::invalid_value(
                "controllers",
                "Joint controllership requires at least 2 controllers",
            ));
        }

        // 2. Check arrangement is documented (Article 26(1) - MANDATORY)
        if !self.arrangement_documented.unwrap_or(false) {
            warnings.push(
                "Article 26(1): Joint controllers must determine their respective responsibilities in a transparent manner by means of an arrangement".to_string(),
            );
        }

        // 3. Check essence is available to data subjects (Article 26(2) - MANDATORY)
        if !self.essence_available_to_data_subjects.unwrap_or(false) {
            warnings.push(
                "Article 26(2): The essence of the arrangement must be made available to the data subjects".to_string(),
            );
        }

        if self.essence_available_to_data_subjects.unwrap_or(false)
            && self.essence_availability_method.is_none()
        {
            recommendations.push(
                "Specify how the essence of the arrangement is made available (e.g., privacy notice, website, etc.)".to_string(),
            );
        }

        // 4. Check contact point designation (Article 26(1) second sentence)
        let contact_point_count = self
            .controllers
            .iter()
            .filter(|c| c.is_contact_point)
            .count();

        if contact_point_count == 0 {
            warnings.push(
                "Article 26(1): At least one joint controller must be designated as the contact point for data subjects".to_string(),
            );
        } else if contact_point_count > 1 {
            recommendations.push(format!(
                "{} controllers designated as contact points - consider designating only one primary contact",
                contact_point_count
            ));
        }

        // 5. Check rights exercisable against each controller (Article 26(3))
        if !self.rights_exercisable_against_each.unwrap_or(true) {
            warnings.push(
                "Article 26(3): Data subjects must be able to exercise their rights against each of the joint controllers".to_string(),
            );
        }

        // 6. Check mandatory responsibilities are allocated
        let all_responsibilities: Vec<&Responsibility> = self
            .controllers
            .iter()
            .flat_map(|c| c.responsibilities.iter())
            .collect();

        let has_data_subject_rights = all_responsibilities
            .iter()
            .any(|r| matches!(r, Responsibility::DataSubjectRights));
        let has_breach_notification = all_responsibilities
            .iter()
            .any(|r| matches!(r, Responsibility::BreachNotification));
        let has_security_measures = all_responsibilities
            .iter()
            .any(|r| matches!(r, Responsibility::SecurityMeasures));

        if !has_data_subject_rights {
            warnings.push(
                "Responsibility for data subject rights (Articles 15-22) must be allocated"
                    .to_string(),
            );
        }

        if !has_breach_notification {
            warnings.push(
                "Responsibility for breach notification (Articles 33-34) must be allocated"
                    .to_string(),
            );
        }

        if !has_security_measures {
            warnings.push(
                "Responsibility for security measures (Article 32) must be allocated".to_string(),
            );
        }

        // 7. Check for special categories (Article 9)
        let has_special_categories = self
            .data_categories
            .iter()
            .any(|cat| matches!(cat, PersonalDataCategory::Special(_)));

        if has_special_categories {
            recommendations.push(
                "Processing special categories of data (Article 9) - ensure Article 9 exception is documented in arrangement".to_string(),
            );
        }

        // 8. Check joint controllership basis is documented
        if self.joint_controllership_basis.is_none() {
            recommendations.push(
                "Document the basis for joint controllership (joint decision on purposes and means)".to_string(),
            );
        }

        // 9. Validate controller details
        for (idx, controller) in self.controllers.iter().enumerate() {
            if controller.contact.is_none() {
                recommendations.push(format!(
                    "Controller '{}' (#{}): Contact information not provided",
                    controller.name,
                    idx + 1
                ));
            }

            if controller.responsibilities.is_empty() {
                warnings.push(format!(
                    "Controller '{}' (#{}): No responsibilities allocated",
                    controller.name,
                    idx + 1
                ));
            }
        }

        // 10. Check for responsibility overlap/gaps
        let total_responsibilities = all_responsibilities.len();
        let unique_responsibilities: std::collections::HashSet<_> =
            all_responsibilities.into_iter().collect();

        if total_responsibilities > unique_responsibilities.len() {
            let overlap_count = total_responsibilities - unique_responsibilities.len();
            recommendations.push(format!(
                "{} responsibility/responsibilities allocated to multiple controllers - ensure coordination",
                overlap_count
            ));
        }

        // 11. Final compliance determination
        let compliant = warnings.is_empty();

        Ok(Article26Validation {
            compliant,
            warnings,
            recommendations,
            controllers_count: self.controllers.len(),
            contact_point_designated: contact_point_count > 0,
            arrangement_documented: self.arrangement_documented.unwrap_or(false),
            essence_available: self.essence_available_to_data_subjects.unwrap_or(false),
            rights_exercisable: self.rights_exercisable_against_each.unwrap_or(true),
            mandatory_responsibilities_allocated: has_data_subject_rights
                && has_breach_notification
                && has_security_measures,
            has_special_categories,
        })
    }
}

impl Default for JointControllerArrangement {
    fn default() -> Self {
        Self::new()
    }
}

/// Validation result for Article 26 joint controller arrangement
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Article26Validation {
    /// Whether the arrangement is Article 26 compliant
    pub compliant: bool,

    /// List of non-compliance warnings
    pub warnings: Vec<String>,

    /// List of recommendations for improvement
    pub recommendations: Vec<String>,

    /// Number of joint controllers
    pub controllers_count: usize,

    /// Whether at least one contact point is designated
    pub contact_point_designated: bool,

    /// Whether the arrangement is documented (Article 26(1))
    pub arrangement_documented: bool,

    /// Whether the essence is available to data subjects (Article 26(2))
    pub essence_available: bool,

    /// Whether rights are exercisable against each controller (Article 26(3))
    pub rights_exercisable: bool,

    /// Whether mandatory responsibilities are allocated
    pub mandatory_responsibilities_allocated: bool,

    /// Whether special categories are processed
    pub has_special_categories: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete_joint_controller_arrangement() {
        let arrangement = JointControllerArrangement::new()
            .add_controller(
                JointController::new("University A")
                    .with_contact("dpo@uni-a.edu")
                    .add_responsibility(Responsibility::DataCollection)
                    .add_responsibility(Responsibility::DataStorage)
                    .add_responsibility(Responsibility::SecurityMeasures)
                    .as_contact_point(),
            )
            .add_controller(
                JointController::new("University B")
                    .with_contact("privacy@uni-b.edu")
                    .add_responsibility(Responsibility::DataAnalysis)
                    .add_responsibility(Responsibility::DataSubjectRights)
                    .add_responsibility(Responsibility::BreachNotification),
            )
            .with_processing_purpose("Joint research project")
            .with_data_categories(vec![PersonalDataCategory::Regular("name".to_string())])
            .with_arrangement_documented(true)
            .with_essence_available_to_data_subjects(true)
            .with_essence_availability_method("Privacy notice on website")
            .with_rights_exercisable_against_each(true);

        let validation = arrangement.validate().unwrap();

        assert!(validation.compliant);
        assert_eq!(validation.warnings.len(), 0);
        assert_eq!(validation.controllers_count, 2);
        assert!(validation.contact_point_designated);
        assert!(validation.arrangement_documented);
        assert!(validation.essence_available);
        assert!(validation.rights_exercisable);
        assert!(validation.mandatory_responsibilities_allocated);
    }

    #[test]
    fn test_missing_arrangement_documentation() {
        let arrangement = JointControllerArrangement::new()
            .add_controller(JointController::new("Controller A").as_contact_point())
            .add_controller(JointController::new("Controller B"))
            .with_arrangement_documented(false);

        let validation = arrangement.validate().unwrap();

        assert!(!validation.compliant);
        assert!(
            validation
                .warnings
                .iter()
                .any(|w| w.contains("Article 26(1)"))
        );
        assert!(!validation.arrangement_documented);
    }

    #[test]
    fn test_missing_essence_availability() {
        let arrangement = JointControllerArrangement::new()
            .add_controller(JointController::new("Controller A").as_contact_point())
            .add_controller(JointController::new("Controller B"))
            .with_arrangement_documented(true)
            .with_essence_available_to_data_subjects(false);

        let validation = arrangement.validate().unwrap();

        assert!(!validation.compliant);
        assert!(
            validation
                .warnings
                .iter()
                .any(|w| w.contains("Article 26(2)"))
        );
        assert!(!validation.essence_available);
    }

    #[test]
    fn test_missing_contact_point() {
        let arrangement = JointControllerArrangement::new()
            .add_controller(JointController::new("Controller A"))
            .add_controller(JointController::new("Controller B"))
            .with_arrangement_documented(true)
            .with_essence_available_to_data_subjects(true);

        let validation = arrangement.validate().unwrap();

        assert!(!validation.compliant);
        assert!(
            validation
                .warnings
                .iter()
                .any(|w| w.contains("contact point"))
        );
        assert!(!validation.contact_point_designated);
    }

    #[test]
    fn test_missing_mandatory_responsibilities() {
        let arrangement = JointControllerArrangement::new()
            .add_controller(
                JointController::new("Controller A")
                    .add_responsibility(Responsibility::DataCollection)
                    .as_contact_point(),
            )
            .add_controller(
                JointController::new("Controller B")
                    .add_responsibility(Responsibility::DataStorage),
            )
            .with_arrangement_documented(true)
            .with_essence_available_to_data_subjects(true);

        let validation = arrangement.validate().unwrap();

        assert!(!validation.compliant);
        assert!(
            validation
                .warnings
                .iter()
                .any(|w| w.contains("data subject rights"))
        );
        assert!(
            validation
                .warnings
                .iter()
                .any(|w| w.contains("breach notification"))
        );
        assert!(
            validation
                .warnings
                .iter()
                .any(|w| w.contains("security measures"))
        );
        assert!(!validation.mandatory_responsibilities_allocated);
    }

    #[test]
    fn test_single_controller_fails() {
        let arrangement = JointControllerArrangement::new()
            .add_controller(JointController::new("Single Controller"));

        let result = arrangement.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_special_categories_recommendation() {
        let arrangement = JointControllerArrangement::new()
            .add_controller(
                JointController::new("Hospital A")
                    .add_responsibility(Responsibility::DataSubjectRights)
                    .add_responsibility(Responsibility::BreachNotification)
                    .add_responsibility(Responsibility::SecurityMeasures)
                    .as_contact_point(),
            )
            .add_controller(
                JointController::new("Hospital B")
                    .add_responsibility(Responsibility::DataCollection)
                    .add_responsibility(Responsibility::DataAnalysis),
            )
            .with_data_categories(vec![PersonalDataCategory::Special(
                SpecialCategory::HealthData,
            )])
            .with_arrangement_documented(true)
            .with_essence_available_to_data_subjects(true);

        let validation = arrangement.validate().unwrap();

        assert!(validation.compliant);
        assert!(validation.has_special_categories);
        assert!(
            validation
                .recommendations
                .iter()
                .any(|r| r.contains("Article 9"))
        );
    }

    #[test]
    fn test_responsibility_article_references() {
        assert_eq!(
            Responsibility::DataSubjectRights.article_reference(),
            "Articles 15-22"
        );
        assert_eq!(
            Responsibility::SecurityMeasures.article_reference(),
            "Article 32"
        );
        assert_eq!(
            Responsibility::BreachNotification.article_reference(),
            "Articles 33-34"
        );
    }
}
