//! GDPR Article 37 - Data Protection Officer (DPO)
//!
//! This module implements the requirements for designating a Data Protection Officer
//! under GDPR Article 37.
//!
//! ## Key Requirements
//!
//! **Article 37(1) - Mandatory DPO Designation**
//!
//! A DPO must be designated when:
//! - (a) Processing is carried out by a public authority/body (except courts)
//! - (b) Core activities consist of processing operations requiring regular and systematic
//!   monitoring of data subjects on a large scale
//! - (c) Core activities consist of large-scale processing of special categories (Article 9)
//!   or criminal conviction data (Article 10)
//!
//! **Article 37(2) - Group DPO**
//!
//! A group of undertakings may appoint a single DPO, provided they are accessible from
//! each establishment.
//!
//! **Article 37(5) - DPO Qualifications**
//!
//! The DPO shall be designated based on:
//! - Professional qualities
//! - Expert knowledge of data protection law and practices
//! - Ability to fulfill tasks under Article 39
//!
//! **Article 37(6) - DPO Contact Details**
//!
//! Contact details must be published and communicated to the supervisory authority.
//!
//! **Article 38 - Position of the DPO**
//!
//! - Shall be involved in all issues related to data protection (Article 38(1))
//! - Shall not receive instructions regarding the exercise of tasks (Article 38(3))
//! - Shall report directly to highest management level (Article 38(3))
//! - Shall not be dismissed or penalized for performing tasks (Article 38(3))
//!
//! **Article 39 - Tasks of the DPO**
//!
//! The DPO shall have at least the following tasks:
//! - (a) Inform and advise controller/processor and employees
//! - (b) Monitor compliance with GDPR
//! - (c) Provide advice concerning DPIA
//! - (d) Cooperate with supervisory authority
//! - (e) Act as contact point for supervisory authority and data subjects
//!
//! ## EUR-Lex References
//!
//! - Article 37: CELEX:32016R0679 Art. 37
//! - Article 38: CELEX:32016R0679 Art. 38
//! - Article 39: CELEX:32016R0679 Art. 39
//! - WP29 Guidelines on DPOs (WP243 rev.01)
//!
//! ## Example Usage
//!
//! ```rust
//! use legalis_eu::gdpr::dpo::*;
//!
//! // Check if DPO required for public authority
//! let assessment = DpoDesignationAssessment::new()
//!     .with_entity_type(DpoEntityType::PublicAuthority)
//!     .with_organization_name("City Council");
//!
//! match assessment.validate() {
//!     Ok(result) => {
//!         if result.dpo_required {
//!             println!("✅ DPO designation mandatory: {}", result.reason);
//!         }
//!     }
//!     Err(e) => println!("Error: {}", e),
//! }
//!
//! // Create full DPO designation
//! let dpo = DpoDesignation::new()
//!     .with_name("Jane Doe")
//!     .with_email("dpo@organization.eu")
//!     .with_phone("+49 30 12345678")
//!     .with_qualifications("CIPP/E certified, 10 years data protection experience")
//!     .reports_to_highest_management(true)
//!     .is_independent(true);
//!
//! match dpo.validate() {
//!     Ok(validation) => {
//!         if validation.compliant {
//!             println!("✅ DPO designation compliant with Articles 37-39");
//!         }
//!     }
//!     Err(e) => println!("Error: {}", e),
//! }
//! ```

use crate::gdpr::error::GdprError;
use crate::gdpr::types::SpecialCategory;
use crate::shared::MemberState;
use chrono::{DateTime, Utc};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ========================================
// Article 37(1) - Designation Criteria
// ========================================

/// Entity type for Article 37(1)(a) assessment
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DpoEntityType {
    /// Public authority or body (Article 37(1)(a))
    /// Except courts acting in their judicial capacity
    PublicAuthority,

    /// Private controller or processor
    PrivateEntity,

    /// Court acting in judicial capacity (exempt from DPO requirement)
    JudicialCourt,
}

/// Scale of processing operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ProcessingScale {
    /// Small scale - few data subjects, limited scope
    Small,

    /// Medium scale - moderate number of data subjects
    Medium,

    /// Large scale - extensive processing affecting many data subjects
    /// WP29: Consider factors like number of data subjects, volume of data,
    /// duration, geographical extent
    LargeScale,
}

/// Type of monitoring for Article 37(1)(b)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MonitoringType {
    /// Behavioral advertising and online tracking
    BehavioralAdvertising,

    /// Location tracking (e.g., mobile apps)
    LocationTracking,

    /// Health or fitness monitoring (wearables, apps)
    HealthMonitoring,

    /// Employee monitoring (email, internet usage, CCTV)
    EmployeeMonitoring,

    /// Video surveillance (CCTV in public spaces)
    VideoSurveillance,

    /// Telecommunications data retention
    TelecommunicationsMonitoring,

    /// Credit scoring or risk profiling
    CreditScoring,

    /// Custom monitoring type
    Other(String),
}

/// Core activity determination (Article 37(1)(b)(c))
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CoreActivity {
    /// Processing is a core activity (primary business function)
    /// WP29: Ancillary activities (e.g., payroll, IT support) are NOT core activities
    Core { description: String },

    /// Processing is ancillary (supporting activity)
    Ancillary { description: String },
}

impl CoreActivity {
    /// Returns true if this is a core activity
    pub fn is_core(&self) -> bool {
        matches!(self, CoreActivity::Core { .. })
    }
}

// ========================================
// Article 37 - DPO Designation Assessment
// ========================================

/// Assessment of whether DPO designation is mandatory (Article 37(1))
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DpoDesignationAssessment {
    /// Organization name
    pub organization_name: Option<String>,

    /// Entity type (public authority vs private)
    pub entity_type: Option<DpoEntityType>,

    /// Article 37(1)(b): Monitoring-based requirement
    pub monitoring_activities: Vec<MonitoringType>,
    pub monitoring_is_core_activity: Option<CoreActivity>,
    pub monitoring_scale: Option<ProcessingScale>,
    pub monitoring_regular: Option<bool>,
    pub monitoring_systematic: Option<bool>,

    /// Article 37(1)(c): Special categories/criminal data requirement
    pub processes_special_categories: Option<bool>,
    pub special_categories: Vec<SpecialCategory>,
    pub special_categories_is_core_activity: Option<CoreActivity>,
    pub special_categories_scale: Option<ProcessingScale>,

    /// Article 37(1)(c): Criminal conviction data
    pub processes_criminal_data: Option<bool>,
    pub criminal_data_is_core_activity: Option<CoreActivity>,
    pub criminal_data_scale: Option<ProcessingScale>,

    /// Member state law may require DPO in additional cases
    pub member_state_law_requirement: Option<String>,
}

impl Default for DpoDesignationAssessment {
    fn default() -> Self {
        Self::new()
    }
}

impl DpoDesignationAssessment {
    /// Create a new DPO designation assessment
    pub fn new() -> Self {
        Self {
            organization_name: None,
            entity_type: None,
            monitoring_activities: Vec::new(),
            monitoring_is_core_activity: None,
            monitoring_scale: None,
            monitoring_regular: None,
            monitoring_systematic: None,
            processes_special_categories: None,
            special_categories: Vec::new(),
            special_categories_is_core_activity: None,
            special_categories_scale: None,
            processes_criminal_data: None,
            criminal_data_is_core_activity: None,
            criminal_data_scale: None,
            member_state_law_requirement: None,
        }
    }

    /// Set organization name
    pub fn with_organization_name(mut self, name: impl Into<String>) -> Self {
        self.organization_name = Some(name.into());
        self
    }

    /// Set entity type (public authority or private)
    pub fn with_entity_type(mut self, entity_type: DpoEntityType) -> Self {
        self.entity_type = Some(entity_type);
        self
    }

    /// Add monitoring activity type
    pub fn add_monitoring_activity(mut self, activity: MonitoringType) -> Self {
        self.monitoring_activities.push(activity);
        self
    }

    /// Set whether monitoring is a core activity
    pub fn with_monitoring_core_activity(mut self, core_activity: CoreActivity) -> Self {
        self.monitoring_is_core_activity = Some(core_activity);
        self
    }

    /// Set monitoring scale
    pub fn with_monitoring_scale(mut self, scale: ProcessingScale) -> Self {
        self.monitoring_scale = Some(scale);
        self
    }

    /// Set whether monitoring is regular
    pub fn monitoring_is_regular(mut self, regular: bool) -> Self {
        self.monitoring_regular = Some(regular);
        self
    }

    /// Set whether monitoring is systematic
    pub fn monitoring_is_systematic(mut self, systematic: bool) -> Self {
        self.monitoring_systematic = Some(systematic);
        self
    }

    /// Enable special categories processing
    pub fn processes_special_categories(mut self, processes: bool) -> Self {
        self.processes_special_categories = Some(processes);
        self
    }

    /// Add special category type
    pub fn add_special_category(mut self, category: SpecialCategory) -> Self {
        self.special_categories.push(category);
        self
    }

    /// Set whether special categories processing is core activity
    pub fn with_special_categories_core_activity(mut self, core_activity: CoreActivity) -> Self {
        self.special_categories_is_core_activity = Some(core_activity);
        self
    }

    /// Set special categories processing scale
    pub fn with_special_categories_scale(mut self, scale: ProcessingScale) -> Self {
        self.special_categories_scale = Some(scale);
        self
    }

    /// Enable criminal conviction data processing
    pub fn processes_criminal_data(mut self, processes: bool) -> Self {
        self.processes_criminal_data = Some(processes);
        self
    }

    /// Set whether criminal data processing is core activity
    pub fn with_criminal_data_core_activity(mut self, core_activity: CoreActivity) -> Self {
        self.criminal_data_is_core_activity = Some(core_activity);
        self
    }

    /// Set criminal data processing scale
    pub fn with_criminal_data_scale(mut self, scale: ProcessingScale) -> Self {
        self.criminal_data_scale = Some(scale);
        self
    }

    /// Set member state law requirement
    pub fn with_member_state_law_requirement(mut self, requirement: impl Into<String>) -> Self {
        self.member_state_law_requirement = Some(requirement.into());
        self
    }

    /// Validate and determine if DPO is required
    pub fn validate(&self) -> Result<DpoRequirementResult, GdprError> {
        // Article 37(1)(a): Public authority or body (except courts)
        if let Some(entity_type) = &self.entity_type {
            if *entity_type == DpoEntityType::PublicAuthority {
                return Ok(DpoRequirementResult {
                    dpo_required: true,
                    reason: "Article 37(1)(a): Processing carried out by public authority or body"
                        .to_string(),
                    article: "Article 37(1)(a)".to_string(),
                    recommendations: vec![
                        "Designate DPO with professional qualities and expert knowledge"
                            .to_string(),
                        "Publish DPO contact details".to_string(),
                        "Communicate DPO details to supervisory authority".to_string(),
                    ],
                });
            } else if *entity_type == DpoEntityType::JudicialCourt {
                return Ok(DpoRequirementResult {
                    dpo_required: false,
                    reason:
                        "Article 37(1)(a) exemption: Courts acting in judicial capacity are exempt"
                            .to_string(),
                    article: "Article 37(1)(a)".to_string(),
                    recommendations: Vec::new(),
                });
            }
        }

        // Article 37(1)(b): Regular and systematic monitoring on large scale
        if !self.monitoring_activities.is_empty() {
            let is_core = self
                .monitoring_is_core_activity
                .as_ref()
                .map(|a| a.is_core())
                .unwrap_or(false);
            let is_large_scale = matches!(self.monitoring_scale, Some(ProcessingScale::LargeScale));
            let is_regular = self.monitoring_regular.unwrap_or(false);
            let is_systematic = self.monitoring_systematic.unwrap_or(false);

            if is_core && is_large_scale && is_regular && is_systematic {
                return Ok(DpoRequirementResult {
                    dpo_required: true,
                    reason: format!(
                        "Article 37(1)(b): Core activities consist of regular and systematic \
                         monitoring of data subjects on a large scale ({} monitoring activities)",
                        self.monitoring_activities.len()
                    ),
                    article: "Article 37(1)(b)".to_string(),
                    recommendations: vec![
                        "Designate DPO with expertise in monitoring and profiling".to_string(),
                        "Ensure DPO is involved in all data protection decisions".to_string(),
                        "Consider DPIA for high-risk monitoring activities".to_string(),
                    ],
                });
            }
        }

        // Article 37(1)(c): Large-scale processing of special categories or criminal data
        let special_cat_processing = self.processes_special_categories.unwrap_or(false)
            && !self.special_categories.is_empty();
        let criminal_processing = self.processes_criminal_data.unwrap_or(false);

        if special_cat_processing || criminal_processing {
            let is_core_special = self
                .special_categories_is_core_activity
                .as_ref()
                .map(|a| a.is_core())
                .unwrap_or(false);
            let is_large_scale_special = matches!(
                self.special_categories_scale,
                Some(ProcessingScale::LargeScale)
            );

            let is_core_criminal = self
                .criminal_data_is_core_activity
                .as_ref()
                .map(|a| a.is_core())
                .unwrap_or(false);
            let is_large_scale_criminal =
                matches!(self.criminal_data_scale, Some(ProcessingScale::LargeScale));

            if (special_cat_processing && is_core_special && is_large_scale_special)
                || (criminal_processing && is_core_criminal && is_large_scale_criminal)
            {
                let data_type = if special_cat_processing {
                    format!("special categories ({})", self.special_categories.len())
                } else {
                    "criminal conviction data".to_string()
                };

                return Ok(DpoRequirementResult {
                    dpo_required: true,
                    reason: format!(
                        "Article 37(1)(c): Core activities consist of large-scale processing of {}",
                        data_type
                    ),
                    article: "Article 37(1)(c)".to_string(),
                    recommendations: vec![
                        "Designate DPO with expertise in sensitive data processing".to_string(),
                        "Conduct DPIA for special category processing".to_string(),
                        "Implement enhanced security measures for sensitive data".to_string(),
                    ],
                });
            }
        }

        // Member state law requirement
        if let Some(requirement) = &self.member_state_law_requirement {
            return Ok(DpoRequirementResult {
                dpo_required: true,
                reason: format!("Member state law requires DPO: {}", requirement),
                article: "Article 37(4) - Member state law".to_string(),
                recommendations: vec![
                    "Comply with national data protection law requirements".to_string(),
                    "Check local supervisory authority guidance".to_string(),
                ],
            });
        }

        // No requirement found
        Ok(DpoRequirementResult {
            dpo_required: false,
            reason: "No Article 37(1) criteria met - DPO designation not mandatory".to_string(),
            article: "Article 37(1)".to_string(),
            recommendations: vec![
                "Consider voluntary DPO designation for good governance".to_string(),
                "Monitor processing activities for future DPO requirement triggers".to_string(),
            ],
        })
    }
}

/// Result of DPO requirement assessment
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DpoRequirementResult {
    /// Is DPO designation mandatory?
    pub dpo_required: bool,

    /// Reason for requirement/exemption
    pub reason: String,

    /// Article reference
    pub article: String,

    /// Recommendations
    pub recommendations: Vec<String>,
}

// ========================================
// Article 37(5) - DPO Qualifications
// ========================================

/// Professional qualities and expertise of DPO (Article 37(5))
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DpoQualification {
    /// Legal qualification (law degree)
    LegalQualification { degree: String, year_obtained: u16 },

    /// Professional certification (CIPP, CIPM, CIPT)
    Certification {
        certification_name: String,
        issuing_body: String,
        certification_date: DateTime<Utc>,
    },

    /// Work experience in data protection
    WorkExperience { years: u8, description: String },

    /// Technical expertise
    TechnicalExpertise { domain: String, description: String },

    /// Training courses completed
    Training {
        course_name: String,
        provider: String,
        completion_date: DateTime<Utc>,
    },
}

// ========================================
// Article 37-39 - Full DPO Designation
// ========================================

/// Contact details for DPO (Article 37(6))
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DpoContactDetails {
    /// DPO name
    pub name: String,

    /// Email address (must be published)
    pub email: String,

    /// Phone number (optional but recommended)
    pub phone: Option<String>,

    /// Postal address (optional)
    pub address: Option<String>,

    /// Website or online form (optional)
    pub website: Option<String>,
}

/// Full DPO designation with Articles 37-39 compliance
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DpoDesignation {
    /// DPO contact details (Article 37(6))
    pub contact: Option<DpoContactDetails>,

    /// DPO qualifications (Article 37(5))
    pub qualifications: Vec<DpoQualification>,

    /// Qualification summary text
    pub qualification_summary: Option<String>,

    /// Article 38(3): Reports directly to highest management level
    pub reports_to_highest_management: Option<bool>,

    /// Article 38(3): Does not receive instructions on task performance
    pub is_independent: Option<bool>,

    /// Article 38(1): Involved properly and timely in all data protection matters
    pub involved_in_all_matters: Option<bool>,

    /// Article 38(2): Provided with necessary resources
    pub has_necessary_resources: Option<bool>,

    /// Article 39: Tasks assigned to DPO
    pub tasks_assigned: Vec<DpoTask>,

    /// Date of designation
    pub designation_date: Option<DateTime<Utc>>,

    /// Supervisory authority notified
    pub supervisory_authority_notified: Option<MemberState>,
    pub notification_date: Option<DateTime<Utc>>,

    /// Contact details published
    pub contact_details_published: Option<bool>,
    pub publication_url: Option<String>,

    /// Article 37(2): Group DPO accessible from all establishments
    pub is_group_dpo: Option<bool>,
    pub group_establishments: Vec<String>,

    /// Additional notes
    pub notes: Option<String>,
}

impl Default for DpoDesignation {
    fn default() -> Self {
        Self::new()
    }
}

impl DpoDesignation {
    /// Create new DPO designation
    pub fn new() -> Self {
        Self {
            contact: None,
            qualifications: Vec::new(),
            qualification_summary: None,
            reports_to_highest_management: None,
            is_independent: None,
            involved_in_all_matters: None,
            has_necessary_resources: None,
            tasks_assigned: Vec::new(),
            designation_date: None,
            supervisory_authority_notified: None,
            notification_date: None,
            contact_details_published: None,
            publication_url: None,
            is_group_dpo: None,
            group_establishments: Vec::new(),
            notes: None,
        }
    }

    /// Set DPO contact details
    pub fn with_contact_details(mut self, contact: DpoContactDetails) -> Self {
        self.contact = Some(contact);
        self
    }

    /// Set DPO name (creates contact if not exists)
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        let name_str = name.into();
        if let Some(contact) = &mut self.contact {
            contact.name = name_str;
        } else {
            self.contact = Some(DpoContactDetails {
                name: name_str,
                email: String::new(),
                phone: None,
                address: None,
                website: None,
            });
        }
        self
    }

    /// Set DPO email
    pub fn with_email(mut self, email: impl Into<String>) -> Self {
        let email_str = email.into();
        if let Some(contact) = &mut self.contact {
            contact.email = email_str;
        } else {
            self.contact = Some(DpoContactDetails {
                name: String::new(),
                email: email_str,
                phone: None,
                address: None,
                website: None,
            });
        }
        self
    }

    /// Set DPO phone
    pub fn with_phone(mut self, phone: impl Into<String>) -> Self {
        if let Some(contact) = &mut self.contact {
            contact.phone = Some(phone.into());
        }
        self
    }

    /// Add qualification
    pub fn add_qualification(mut self, qualification: DpoQualification) -> Self {
        self.qualifications.push(qualification);
        self
    }

    /// Set qualifications summary
    pub fn with_qualifications(mut self, summary: impl Into<String>) -> Self {
        self.qualification_summary = Some(summary.into());
        self
    }

    /// Set reporting to highest management
    pub fn reports_to_highest_management(mut self, reports: bool) -> Self {
        self.reports_to_highest_management = Some(reports);
        self
    }

    /// Set independence
    pub fn is_independent(mut self, independent: bool) -> Self {
        self.is_independent = Some(independent);
        self
    }

    /// Set involvement in all matters
    pub fn involved_in_all_matters(mut self, involved: bool) -> Self {
        self.involved_in_all_matters = Some(involved);
        self
    }

    /// Set resources provided
    pub fn has_necessary_resources(mut self, has_resources: bool) -> Self {
        self.has_necessary_resources = Some(has_resources);
        self
    }

    /// Add DPO task
    pub fn add_task(mut self, task: DpoTask) -> Self {
        self.tasks_assigned.push(task);
        self
    }

    /// Set designation date
    pub fn with_designation_date(mut self, date: DateTime<Utc>) -> Self {
        self.designation_date = Some(date);
        self
    }

    /// Set supervisory authority notification
    pub fn notified_to_authority(mut self, authority: MemberState, date: DateTime<Utc>) -> Self {
        self.supervisory_authority_notified = Some(authority);
        self.notification_date = Some(date);
        self
    }

    /// Set contact details published
    pub fn with_publication(mut self, published: bool, url: Option<String>) -> Self {
        self.contact_details_published = Some(published);
        self.publication_url = url;
        self
    }

    /// Set as group DPO
    pub fn as_group_dpo(mut self, establishments: Vec<String>) -> Self {
        self.is_group_dpo = Some(true);
        self.group_establishments = establishments;
        self
    }

    /// Add notes
    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    /// Validate DPO designation compliance
    pub fn validate(&self) -> Result<DpoValidation, GdprError> {
        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();

        // Article 37(6): Contact details
        let has_contact = self.contact.is_some();
        if !has_contact {
            warnings.push("Missing DPO contact details (Article 37(6) requirement)".to_string());
        } else if let Some(contact) = &self.contact {
            if contact.name.is_empty() {
                warnings.push("DPO name is empty".to_string());
            }
            if contact.email.is_empty() {
                warnings.push("DPO email is required (Article 37(6))".to_string());
            }
            if contact.phone.is_none() {
                recommendations.push("Consider adding phone number for accessibility".to_string());
            }
        }

        // Article 37(5): Qualifications
        let has_qualifications =
            !self.qualifications.is_empty() || self.qualification_summary.is_some();
        if !has_qualifications {
            warnings.push(
                "No DPO qualifications documented (Article 37(5) requires professional qualities \
                 and expert knowledge)"
                    .to_string(),
            );
        }

        // Article 38(3): Independence and reporting
        if self.reports_to_highest_management != Some(true) {
            warnings.push(
                "DPO should report directly to highest management level (Article 38(3))"
                    .to_string(),
            );
        }

        if self.is_independent != Some(true) {
            warnings.push(
                "DPO should not receive instructions on task performance (Article 38(3))"
                    .to_string(),
            );
        }

        // Article 38(1): Involvement
        if self.involved_in_all_matters != Some(true) {
            recommendations.push(
                "Ensure DPO is properly and timely involved in all data protection matters \
                 (Article 38(1))"
                    .to_string(),
            );
        }

        // Article 38(2): Resources
        if self.has_necessary_resources != Some(true) {
            warnings.push(
                "DPO should be provided with necessary resources (Article 38(2))".to_string(),
            );
        }

        // Article 39: Tasks
        if self.tasks_assigned.is_empty() {
            recommendations.push(
                "Document DPO tasks as per Article 39 (inform/advise, monitor compliance, DPIA \
                 advice, cooperate with SA)"
                    .to_string(),
            );
        } else {
            // Check for mandatory tasks
            let has_inform_advise = self
                .tasks_assigned
                .iter()
                .any(|t| matches!(t, DpoTask::InformAndAdvise));
            let has_monitor = self
                .tasks_assigned
                .iter()
                .any(|t| matches!(t, DpoTask::MonitorCompliance));
            let has_dpia = self
                .tasks_assigned
                .iter()
                .any(|t| matches!(t, DpoTask::ProvideDpiaAdvice));
            let has_cooperate = self.tasks_assigned.iter().any(|t| {
                matches!(
                    t,
                    DpoTask::CooperateWithSupervisoryAuthority | DpoTask::ActAsContactPoint
                )
            });

            if !has_inform_advise {
                recommendations.push("Add 'Inform and Advise' task (Article 39(1)(a))".to_string());
            }
            if !has_monitor {
                recommendations
                    .push("Add 'Monitor Compliance' task (Article 39(1)(b))".to_string());
            }
            if !has_dpia {
                recommendations
                    .push("Add 'Provide DPIA Advice' task (Article 39(1)(c))".to_string());
            }
            if !has_cooperate {
                recommendations.push(
                    "Add 'Cooperate with SA / Act as Contact Point' task (Article 39(1)(d)(e))"
                        .to_string(),
                );
            }
        }

        // Article 37(6): Publication and notification
        if self.contact_details_published != Some(true) {
            warnings.push("DPO contact details must be published (Article 37(6))".to_string());
        }

        if self.supervisory_authority_notified.is_none() {
            warnings.push(
                "DPO contact details must be communicated to supervisory authority (Article 37(6))"
                    .to_string(),
            );
        }

        // Article 37(2): Group DPO accessibility
        if self.is_group_dpo == Some(true) && self.group_establishments.is_empty() {
            warnings.push(
                "Group DPO must be accessible from all establishments (Article 37(2))".to_string(),
            );
        }

        // Determine compliance
        let compliant = warnings.is_empty();

        Ok(DpoValidation {
            compliant,
            warnings,
            recommendations,
            has_contact_details: has_contact,
            has_qualifications,
            is_independent: self.is_independent.unwrap_or(false),
            reports_to_management: self.reports_to_highest_management.unwrap_or(false),
            has_resources: self.has_necessary_resources.unwrap_or(false),
            contact_published: self.contact_details_published.unwrap_or(false),
            authority_notified: self.supervisory_authority_notified.is_some(),
        })
    }
}

/// Article 39 - DPO Tasks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DpoTask {
    /// Article 39(1)(a): Inform and advise controller/processor and employees
    InformAndAdvise,

    /// Article 39(1)(b): Monitor compliance with GDPR and national law
    MonitorCompliance,

    /// Article 39(1)(c): Provide advice concerning DPIA
    ProvideDpiaAdvice,

    /// Article 39(1)(d): Cooperate with supervisory authority
    CooperateWithSupervisoryAuthority,

    /// Article 39(1)(e): Act as contact point for supervisory authority and data subjects
    ActAsContactPoint,
}

/// Result of DPO designation validation
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DpoValidation {
    /// Overall compliance with Articles 37-39
    pub compliant: bool,

    /// Critical issues preventing compliance
    pub warnings: Vec<String>,

    /// Recommendations for improvement
    pub recommendations: Vec<String>,

    /// Has contact details (Article 37(6))
    pub has_contact_details: bool,

    /// Has documented qualifications (Article 37(5))
    pub has_qualifications: bool,

    /// Is independent (Article 38(3))
    pub is_independent: bool,

    /// Reports to highest management (Article 38(3))
    pub reports_to_management: bool,

    /// Has necessary resources (Article 38(2))
    pub has_resources: bool,

    /// Contact details published (Article 37(6))
    pub contact_published: bool,

    /// Supervisory authority notified (Article 37(6))
    pub authority_notified: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_authority_requires_dpo() {
        let assessment = DpoDesignationAssessment::new()
            .with_entity_type(DpoEntityType::PublicAuthority)
            .with_organization_name("City Council");

        let result = assessment.validate().unwrap();

        assert!(result.dpo_required);
        assert!(result.reason.contains("Article 37(1)(a)"));
        assert!(result.reason.contains("public authority"));
    }

    #[test]
    fn test_judicial_court_exempt_from_dpo() {
        let assessment = DpoDesignationAssessment::new()
            .with_entity_type(DpoEntityType::JudicialCourt)
            .with_organization_name("Supreme Court");

        let result = assessment.validate().unwrap();

        assert!(!result.dpo_required);
        assert!(result.reason.contains("exempt"));
        assert!(result.reason.contains("judicial capacity"));
    }

    #[test]
    fn test_large_scale_monitoring_requires_dpo() {
        let assessment = DpoDesignationAssessment::new()
            .with_entity_type(DpoEntityType::PrivateEntity)
            .add_monitoring_activity(MonitoringType::BehavioralAdvertising)
            .add_monitoring_activity(MonitoringType::LocationTracking)
            .with_monitoring_core_activity(CoreActivity::Core {
                description: "Online advertising platform".to_string(),
            })
            .with_monitoring_scale(ProcessingScale::LargeScale)
            .monitoring_is_regular(true)
            .monitoring_is_systematic(true);

        let result = assessment.validate().unwrap();

        assert!(result.dpo_required);
        assert!(result.reason.contains("Article 37(1)(b)"));
        assert!(result.reason.contains("regular and systematic monitoring"));
    }

    #[test]
    fn test_ancillary_monitoring_no_dpo() {
        let assessment = DpoDesignationAssessment::new()
            .with_entity_type(DpoEntityType::PrivateEntity)
            .add_monitoring_activity(MonitoringType::VideoSurveillance)
            .with_monitoring_core_activity(CoreActivity::Ancillary {
                description: "Security cameras in office".to_string(),
            })
            .with_monitoring_scale(ProcessingScale::Small)
            .monitoring_is_regular(true)
            .monitoring_is_systematic(false);

        let result = assessment.validate().unwrap();

        assert!(!result.dpo_required);
        assert!(result.reason.contains("No Article 37(1) criteria met"));
    }

    #[test]
    fn test_large_scale_special_categories_requires_dpo() {
        let assessment = DpoDesignationAssessment::new()
            .with_entity_type(DpoEntityType::PrivateEntity)
            .processes_special_categories(true)
            .add_special_category(SpecialCategory::HealthData)
            .with_special_categories_core_activity(CoreActivity::Core {
                description: "Hospital patient records system".to_string(),
            })
            .with_special_categories_scale(ProcessingScale::LargeScale);

        let result = assessment.validate().unwrap();

        assert!(result.dpo_required);
        assert!(result.reason.contains("Article 37(1)(c)"));
        assert!(result.reason.contains("special categories"));
    }

    #[test]
    fn test_small_scale_health_data_no_dpo() {
        let assessment = DpoDesignationAssessment::new()
            .with_entity_type(DpoEntityType::PrivateEntity)
            .processes_special_categories(true)
            .add_special_category(SpecialCategory::HealthData)
            .with_special_categories_core_activity(CoreActivity::Core {
                description: "Small clinic".to_string(),
            })
            .with_special_categories_scale(ProcessingScale::Small);

        let result = assessment.validate().unwrap();

        assert!(!result.dpo_required);
    }

    #[test]
    fn test_member_state_law_requirement() {
        let assessment = DpoDesignationAssessment::new()
            .with_entity_type(DpoEntityType::PrivateEntity)
            .with_member_state_law_requirement("German BDSG §38 requires DPO for 20+ employees");

        let result = assessment.validate().unwrap();

        assert!(result.dpo_required);
        assert!(result.reason.contains("Member state law"));
        assert!(result.reason.contains("BDSG"));
    }

    #[test]
    fn test_dpo_designation_validation_complete() {
        let dpo = DpoDesignation::new()
            .with_name("Jane Doe")
            .with_email("dpo@organization.eu")
            .with_phone("+49 30 12345678")
            .with_qualifications("CIPP/E certified, 10 years experience")
            .add_qualification(DpoQualification::Certification {
                certification_name: "CIPP/E".to_string(),
                issuing_body: "IAPP".to_string(),
                certification_date: Utc::now(),
            })
            .reports_to_highest_management(true)
            .is_independent(true)
            .involved_in_all_matters(true)
            .has_necessary_resources(true)
            .add_task(DpoTask::InformAndAdvise)
            .add_task(DpoTask::MonitorCompliance)
            .add_task(DpoTask::ProvideDpiaAdvice)
            .add_task(DpoTask::CooperateWithSupervisoryAuthority)
            .add_task(DpoTask::ActAsContactPoint)
            .with_publication(true, Some("https://org.eu/dpo".to_string()))
            .notified_to_authority(MemberState::Germany, Utc::now());

        let validation = dpo.validate().unwrap();

        assert!(validation.compliant);
        assert!(validation.warnings.is_empty());
        assert!(validation.has_contact_details);
        assert!(validation.has_qualifications);
        assert!(validation.is_independent);
        assert!(validation.reports_to_management);
        assert!(validation.contact_published);
        assert!(validation.authority_notified);
    }

    #[test]
    fn test_dpo_designation_validation_incomplete() {
        let dpo = DpoDesignation::new().with_name("John Doe");

        let validation = dpo.validate().unwrap();

        assert!(!validation.compliant);
        assert!(!validation.warnings.is_empty());
        assert!(
            validation
                .warnings
                .iter()
                .any(|w| w.contains("email is required"))
        );
        assert!(
            validation
                .warnings
                .iter()
                .any(|w| w.contains("qualifications"))
        );
        assert!(
            validation
                .warnings
                .iter()
                .any(|w| w.contains("highest management"))
        );
    }
}
