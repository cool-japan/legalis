//! Data Protection Officer (DPO) designation rules
//!
//! Implements the UK GDPR rules governing the designation, position and tasks of
//! a Data Protection Officer, together with the obligation to notify the
//! Information Commissioner's Office (ICO) of the DPO's contact details.
//!
//! # Legal basis
//!
//! - **UK GDPR Article 37** — designation of the DPO (when mandatory; voluntary
//!   designation; group DPO; service contract; publication and notification of
//!   contact details).
//! - **UK GDPR Article 38** — position of the DPO (involvement, resources,
//!   independence, no dismissal/penalty for performing tasks, reporting to the
//!   highest management level, conflict of interests).
//! - **UK GDPR Article 39** — tasks of the DPO (informing/advising, monitoring
//!   compliance, advising on DPIAs, cooperating with and acting as contact point
//!   for the ICO, having due regard to risk).
//! - **DPA 2018 s.69-71** — DPO provisions for *competent authorities* processing
//!   for the law-enforcement purposes (Part 3). The mandatory grounds there differ
//!   from Article 37 (a competent authority must designate a DPO regardless of the
//!   nature/scale of processing, save for courts acting judicially).
//!
//! # Mandatory designation (Article 37(1))
//!
//! A controller and processor **must** designate a DPO where:
//!
//! - **(a)** the processing is carried out by a **public authority or body**
//!   (except courts acting in their judicial capacity); or
//! - **(b)** the **core activities** of the controller/processor consist of
//!   processing operations which, by their nature, scope and/or purposes, require
//!   **regular and systematic monitoring of data subjects on a large scale**; or
//! - **(c)** the **core activities** consist of processing **on a large scale** of
//!   **special categories of data** (Article 9) or personal data relating to
//!   **criminal convictions and offences** (Article 10).
//!
//! "Core activities" are the key operations necessary to achieve the controller's
//! or processor's goals — not ancillary support functions (e.g. routine HR or IT
//! support). This implementation follows Article 29 Working Party guidance
//! WP243 rev.01, endorsed for UK purposes by the ICO.
//!
//! # ICO notification
//!
//! Article 37(7) requires the contact details of the DPO to be **published** and
//! **communicated to the ICO**. The ICO provides an online facility for this. A
//! controller that has designated a DPO but failed to notify the ICO is in breach
//! of Article 37(7).
//!
//! # Example
//!
//! ```rust
//! use legalis_uk::data_protection::dpo::{DpoAssessment, OrganisationType, MonitoringScale};
//!
//! let assessment = DpoAssessment {
//!     organisation_type: OrganisationType::PublicAuthority,
//!     court_acting_judicially: false,
//!     regular_systematic_monitoring_is_core: false,
//!     monitoring_scale: MonitoringScale::NotApplicable,
//!     special_category_processing_is_core: false,
//!     special_category_scale: MonitoringScale::NotApplicable,
//!     criminal_data_processing_is_core: false,
//!     criminal_data_scale: MonitoringScale::NotApplicable,
//!     competent_authority_law_enforcement: false,
//! };
//!
//! let outcome = assessment.assess();
//! assert!(outcome.is_mandatory());
//! ```

use serde::{Deserialize, Serialize};

/// Classification of the organisation for the purposes of Article 37(1)(a).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrganisationType {
    /// A public authority or body (e.g. central/local government, NHS bodies,
    /// regulators, state schools). Designation is mandatory under Art 37(1)(a),
    /// except for courts acting in their judicial capacity.
    PublicAuthority,
    /// A private-sector controller or processor. Designation depends on
    /// Art 37(1)(b)/(c).
    PrivateSector,
    /// A not-for-profit body that is not a public authority. Treated as
    /// private-sector for the Art 37(1)(a) test.
    NotForProfit,
}

impl OrganisationType {
    /// Whether the organisation is a public authority/body for Art 37(1)(a).
    pub fn is_public_authority(&self) -> bool {
        matches!(self, Self::PublicAuthority)
    }
}

/// Whether a category of processing reaches the "large scale" threshold.
///
/// "Large scale" is assessed against the number of data subjects, the volume
/// and range of data, the duration/permanence and the geographical extent of
/// the processing (WP243 rev.01). `Unclear` records that the controller should
/// document its reasoning and, in case of doubt, designate a DPO as best
/// practice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MonitoringScale {
    /// The relevant processing is carried out on a large scale.
    LargeScale,
    /// The relevant processing is not carried out on a large scale.
    SmallScale,
    /// The scale is genuinely borderline / not yet determined.
    Unclear,
    /// The relevant kind of processing is not carried out at all.
    NotApplicable,
}

impl MonitoringScale {
    /// Whether this scale satisfies the "large scale" element of the test.
    pub fn is_large_scale(&self) -> bool {
        matches!(self, Self::LargeScale)
    }
}

/// The statutory ground on which DPO designation is mandatory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DesignationGround {
    /// Article 37(1)(a) — processing by a public authority or body.
    PublicAuthority,
    /// Article 37(1)(b) — core activities require large-scale regular and
    /// systematic monitoring of data subjects.
    LargeScaleMonitoring,
    /// Article 37(1)(c) — core activities consist of large-scale processing of
    /// special-category data (Art 9).
    LargeScaleSpecialCategory,
    /// Article 37(1)(c) — core activities consist of large-scale processing of
    /// criminal-offence data (Art 10).
    LargeScaleCriminalData,
    /// DPA 2018 s.69 — a competent authority processing for the law-enforcement
    /// purposes must designate a DPO.
    CompetentAuthority,
}

impl DesignationGround {
    /// The statutory provision establishing this ground.
    pub fn statutory_provision(&self) -> &'static str {
        match self {
            Self::PublicAuthority => "UK GDPR Article 37(1)(a)",
            Self::LargeScaleMonitoring => "UK GDPR Article 37(1)(b)",
            Self::LargeScaleSpecialCategory => "UK GDPR Article 37(1)(c) (Article 9 data)",
            Self::LargeScaleCriminalData => "UK GDPR Article 37(1)(c) (Article 10 data)",
            Self::CompetentAuthority => "DPA 2018 s.69",
        }
    }

    /// A short human-readable explanation of the ground.
    pub fn explanation(&self) -> &'static str {
        match self {
            Self::PublicAuthority => {
                "Processing is carried out by a public authority or body \
                 (other than a court acting in its judicial capacity)."
            }
            Self::LargeScaleMonitoring => {
                "Core activities require regular and systematic monitoring of \
                 data subjects on a large scale."
            }
            Self::LargeScaleSpecialCategory => {
                "Core activities consist of large-scale processing of special \
                 categories of personal data (Article 9)."
            }
            Self::LargeScaleCriminalData => {
                "Core activities consist of large-scale processing of personal \
                 data relating to criminal convictions and offences (Article 10)."
            }
            Self::CompetentAuthority => {
                "A competent authority processing personal data for the \
                 law-enforcement purposes must designate a data protection officer."
            }
        }
    }
}

/// The structured input to a DPO designation assessment.
///
/// Each field captures one element that Article 37(1) (and DPA 2018 s.69) makes
/// relevant. The booleans named `*_is_core` record whether the processing forms
/// part of the controller's/processor's **core activities** (as opposed to an
/// ancillary support function), which is a precondition for grounds (b) and (c).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DpoAssessment {
    /// How the organisation is classified for Art 37(1)(a).
    pub organisation_type: OrganisationType,
    /// True if (and only if) the organisation is a court acting in its judicial
    /// capacity, which is excluded from the public-authority ground.
    pub court_acting_judicially: bool,
    /// Whether regular and systematic monitoring of data subjects is a *core
    /// activity* (Art 37(1)(b)).
    pub regular_systematic_monitoring_is_core: bool,
    /// The scale of that monitoring.
    pub monitoring_scale: MonitoringScale,
    /// Whether processing of special-category data (Art 9) is a *core activity*
    /// (Art 37(1)(c)).
    pub special_category_processing_is_core: bool,
    /// The scale of that special-category processing.
    pub special_category_scale: MonitoringScale,
    /// Whether processing of criminal-offence data (Art 10) is a *core activity*
    /// (Art 37(1)(c)).
    pub criminal_data_processing_is_core: bool,
    /// The scale of that criminal-offence processing.
    pub criminal_data_scale: MonitoringScale,
    /// Whether the organisation is a competent authority processing for the
    /// law-enforcement purposes (DPA 2018 Part 3, s.69).
    pub competent_authority_law_enforcement: bool,
}

impl DpoAssessment {
    /// Run the designation assessment, returning a structured outcome.
    ///
    /// The outcome lists **every** mandatory ground that applies, plus any
    /// "borderline" grounds where the scale is `Unclear` (for which designation
    /// is recommended as best practice even though it is not strictly mandatory).
    pub fn assess(&self) -> DpoAssessmentOutcome {
        let mut mandatory_grounds: Vec<DesignationGround> = Vec::new();
        let mut borderline_grounds: Vec<DesignationGround> = Vec::new();

        // Article 37(1)(a): public authority/body, excluding courts acting
        // judicially.
        if self.organisation_type.is_public_authority() && !self.court_acting_judicially {
            mandatory_grounds.push(DesignationGround::PublicAuthority);
        }

        // Article 37(1)(b): large-scale regular and systematic monitoring as a
        // core activity.
        if self.regular_systematic_monitoring_is_core {
            match self.monitoring_scale {
                MonitoringScale::LargeScale => {
                    mandatory_grounds.push(DesignationGround::LargeScaleMonitoring);
                }
                MonitoringScale::Unclear => {
                    borderline_grounds.push(DesignationGround::LargeScaleMonitoring);
                }
                MonitoringScale::SmallScale | MonitoringScale::NotApplicable => {}
            }
        }

        // Article 37(1)(c): large-scale special-category processing as a core
        // activity.
        if self.special_category_processing_is_core {
            match self.special_category_scale {
                MonitoringScale::LargeScale => {
                    mandatory_grounds.push(DesignationGround::LargeScaleSpecialCategory);
                }
                MonitoringScale::Unclear => {
                    borderline_grounds.push(DesignationGround::LargeScaleSpecialCategory);
                }
                MonitoringScale::SmallScale | MonitoringScale::NotApplicable => {}
            }
        }

        // Article 37(1)(c): large-scale criminal-offence processing as a core
        // activity.
        if self.criminal_data_processing_is_core {
            match self.criminal_data_scale {
                MonitoringScale::LargeScale => {
                    mandatory_grounds.push(DesignationGround::LargeScaleCriminalData);
                }
                MonitoringScale::Unclear => {
                    borderline_grounds.push(DesignationGround::LargeScaleCriminalData);
                }
                MonitoringScale::SmallScale | MonitoringScale::NotApplicable => {}
            }
        }

        // DPA 2018 s.69: competent authority processing for law-enforcement
        // purposes. A court acting judicially is again excluded.
        if self.competent_authority_law_enforcement && !self.court_acting_judicially {
            mandatory_grounds.push(DesignationGround::CompetentAuthority);
        }

        DpoAssessmentOutcome {
            mandatory_grounds,
            borderline_grounds,
        }
    }
}

/// The result of a [`DpoAssessment`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DpoAssessmentOutcome {
    /// All statutory grounds on which designation is mandatory.
    pub mandatory_grounds: Vec<DesignationGround>,
    /// Grounds that are borderline (scale `Unclear`): designation is recommended
    /// as best practice but is not strictly mandatory.
    pub borderline_grounds: Vec<DesignationGround>,
}

impl DpoAssessmentOutcome {
    /// Whether designation of a DPO is mandatory.
    pub fn is_mandatory(&self) -> bool {
        !self.mandatory_grounds.is_empty()
    }

    /// Whether designation, although not mandatory, is recommended because at
    /// least one ground is borderline.
    pub fn is_recommended(&self) -> bool {
        !self.is_mandatory() && !self.borderline_grounds.is_empty()
    }

    /// Whether, on the information provided, designation is neither mandatory nor
    /// specifically recommended. (A voluntary DPO may still be appointed, and the
    /// Article 38/39 requirements then apply in full.)
    pub fn is_voluntary_only(&self) -> bool {
        !self.is_mandatory() && !self.is_recommended()
    }
}

/// The tasks of the DPO under UK GDPR Article 39(1)(a)-(e).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DpoTask {
    /// Art 39(1)(a) — inform and advise the controller/processor and employees
    /// of their obligations under data protection law.
    InformAndAdvise,
    /// Art 39(1)(b) — monitor compliance with data protection law and with the
    /// controller's/processor's policies, including assignment of
    /// responsibilities, awareness-raising, training and audits.
    MonitorCompliance,
    /// Art 39(1)(c) — provide advice, where requested, on the data protection
    /// impact assessment and monitor its performance (Article 35).
    AdviseOnDpia,
    /// Art 39(1)(d) — cooperate with the ICO.
    CooperateWithIco,
    /// Art 39(1)(e) — act as the contact point for the ICO and consult on any
    /// other matter, having due regard to the risk of processing.
    ActAsContactPoint,
}

impl DpoTask {
    /// All statutory tasks of the DPO under Article 39(1).
    pub fn all() -> [DpoTask; 5] {
        [
            Self::InformAndAdvise,
            Self::MonitorCompliance,
            Self::AdviseOnDpia,
            Self::CooperateWithIco,
            Self::ActAsContactPoint,
        ]
    }

    /// The statutory provision establishing this task.
    pub fn statutory_provision(&self) -> &'static str {
        match self {
            Self::InformAndAdvise => "UK GDPR Article 39(1)(a)",
            Self::MonitorCompliance => "UK GDPR Article 39(1)(b)",
            Self::AdviseOnDpia => "UK GDPR Article 39(1)(c)",
            Self::CooperateWithIco => "UK GDPR Article 39(1)(d)",
            Self::ActAsContactPoint => "UK GDPR Article 39(1)(e)",
        }
    }

    /// A short description of the task.
    pub fn description(&self) -> &'static str {
        match self {
            Self::InformAndAdvise => {
                "Inform and advise the controller, processor and employees of their \
                 data protection obligations."
            }
            Self::MonitorCompliance => {
                "Monitor compliance with data protection law and internal policies, \
                 including awareness-raising, training and audits."
            }
            Self::AdviseOnDpia => {
                "Provide advice, where requested, on data protection impact \
                 assessments and monitor their performance (Article 35)."
            }
            Self::CooperateWithIco => "Cooperate with the Information Commissioner's Office.",
            Self::ActAsContactPoint => {
                "Act as the contact point for the ICO and data subjects, having due \
                 regard to the risk associated with processing."
            }
        }
    }
}

/// A check on the position and independence of the DPO under Article 38.
///
/// Article 38 guarantees the independence and proper resourcing of the DPO. This
/// structure records the key Article 38 conditions so they can be validated.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DpoPosition {
    /// Art 38(1) — the DPO is involved, properly and in a timely manner, in all
    /// issues relating to the protection of personal data.
    pub involved_in_all_issues: bool,
    /// Art 38(2) — the DPO is provided with the resources necessary to carry out
    /// the tasks and to maintain expert knowledge.
    pub provided_with_resources: bool,
    /// Art 38(3) — the DPO does not receive instructions regarding the exercise
    /// of the tasks (operational independence).
    pub operationally_independent: bool,
    /// Art 38(3) — the DPO is not dismissed or penalised for performing the tasks.
    pub protected_from_dismissal: bool,
    /// Art 38(3) — the DPO reports directly to the highest management level.
    pub reports_to_highest_management: bool,
    /// Art 38(6) — any other tasks/duties of the DPO do not result in a conflict
    /// of interests (e.g. the DPO does not also determine the purposes and means
    /// of processing, as a senior operational manager might).
    pub free_of_conflict_of_interest: bool,
}

impl DpoPosition {
    /// Whether the DPO's position complies with Article 38.
    pub fn is_compliant(&self) -> bool {
        self.compliance_failures().is_empty()
    }

    /// A list of Article 38 failures, each as a statutory reference + reason.
    pub fn compliance_failures(&self) -> Vec<DpoPositionFailure> {
        let mut failures = Vec::new();
        if !self.involved_in_all_issues {
            failures.push(DpoPositionFailure {
                provision: "UK GDPR Article 38(1)",
                reason: "DPO is not involved, properly and in a timely manner, in all data \
                         protection issues.",
            });
        }
        if !self.provided_with_resources {
            failures.push(DpoPositionFailure {
                provision: "UK GDPR Article 38(2)",
                reason: "DPO is not provided with the resources necessary to carry out the \
                         tasks and maintain expertise.",
            });
        }
        if !self.operationally_independent {
            failures.push(DpoPositionFailure {
                provision: "UK GDPR Article 38(3)",
                reason: "DPO receives instructions on the exercise of the tasks (no operational \
                         independence).",
            });
        }
        if !self.protected_from_dismissal {
            failures.push(DpoPositionFailure {
                provision: "UK GDPR Article 38(3)",
                reason: "DPO is not protected from dismissal or penalty for performing the tasks.",
            });
        }
        if !self.reports_to_highest_management {
            failures.push(DpoPositionFailure {
                provision: "UK GDPR Article 38(3)",
                reason: "DPO does not report directly to the highest management level.",
            });
        }
        if !self.free_of_conflict_of_interest {
            failures.push(DpoPositionFailure {
                provision: "UK GDPR Article 38(6)",
                reason: "Other duties of the DPO give rise to a conflict of interests.",
            });
        }
        failures
    }
}

/// A single Article 38 position/independence failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DpoPositionFailure {
    /// The statutory provision breached.
    pub provision: &'static str,
    /// The reason for the failure.
    pub reason: &'static str,
}

/// The DPO's contact details, for publication (Art 37(7)) and notification to
/// the ICO.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DpoContactDetails {
    /// A name or a functional title (e.g. "Data Protection Officer"). A name is
    /// not strictly required to be published, but a contact point must be.
    pub name_or_title: String,
    /// A postal address at which the DPO can be reached.
    pub postal_address: String,
    /// A dedicated contact email address.
    pub email: String,
    /// A telephone number (optional).
    pub telephone: Option<String>,
    /// Whether the contact details have been published (e.g. in the privacy
    /// notice / on the website), as required by Art 37(7).
    pub published: bool,
    /// Whether the contact details have been communicated to the ICO, as required
    /// by Art 37(7).
    pub notified_to_ico: bool,
}

impl DpoContactDetails {
    /// Validate that the Article 37(7) publication and ICO-notification duties are
    /// met and that a usable contact point has been provided.
    ///
    /// Returns the list of failures (empty if compliant).
    pub fn validate_notification(&self) -> Vec<DpoNotificationFailure> {
        let mut failures = Vec::new();
        if self.postal_address.trim().is_empty() && self.email.trim().is_empty() {
            failures.push(DpoNotificationFailure::NoContactPoint);
        }
        if !self.published {
            failures.push(DpoNotificationFailure::NotPublished);
        }
        if !self.notified_to_ico {
            failures.push(DpoNotificationFailure::NotNotifiedToIco);
        }
        failures
    }

    /// Whether the contact details satisfy Article 37(7).
    pub fn is_compliant(&self) -> bool {
        self.validate_notification().is_empty()
    }
}

/// A failure of the Article 37(7) publication/notification duty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DpoNotificationFailure {
    /// No usable contact point (neither postal address nor email) was provided.
    NoContactPoint,
    /// The DPO's contact details have not been published.
    NotPublished,
    /// The DPO's contact details have not been communicated to the ICO.
    NotNotifiedToIco,
}

impl DpoNotificationFailure {
    /// The statutory provision breached.
    pub fn statutory_provision(&self) -> &'static str {
        // All three failures arise under Article 37(7).
        "UK GDPR Article 37(7)"
    }

    /// A human-readable description of the failure.
    pub fn message(&self) -> &'static str {
        match self {
            Self::NoContactPoint => {
                "The DPO's contact details must include a usable contact point \
                 (at least a postal address or email)."
            }
            Self::NotPublished => {
                "The DPO's contact details must be published (e.g. in the privacy notice)."
            }
            Self::NotNotifiedToIco => {
                "The DPO's contact details must be communicated to the Information \
                 Commissioner's Office."
            }
        }
    }
}

/// The ICO online registration facility for DPO contact details.
pub const ICO_DPO_NOTIFICATION_URL: &str =
    "https://ico.org.uk/for-organisations/data-protection-officers/";

#[cfg(test)]
mod tests {
    use super::*;

    fn baseline() -> DpoAssessment {
        DpoAssessment {
            organisation_type: OrganisationType::PrivateSector,
            court_acting_judicially: false,
            regular_systematic_monitoring_is_core: false,
            monitoring_scale: MonitoringScale::NotApplicable,
            special_category_processing_is_core: false,
            special_category_scale: MonitoringScale::NotApplicable,
            criminal_data_processing_is_core: false,
            criminal_data_scale: MonitoringScale::NotApplicable,
            competent_authority_law_enforcement: false,
        }
    }

    #[test]
    fn public_authority_is_mandatory_37_1_a() {
        let mut a = baseline();
        a.organisation_type = OrganisationType::PublicAuthority;
        let outcome = a.assess();
        assert!(outcome.is_mandatory());
        assert!(
            outcome
                .mandatory_grounds
                .contains(&DesignationGround::PublicAuthority)
        );
    }

    #[test]
    fn court_acting_judicially_is_exempt() {
        let mut a = baseline();
        a.organisation_type = OrganisationType::PublicAuthority;
        a.court_acting_judicially = true;
        let outcome = a.assess();
        assert!(!outcome.is_mandatory());
        assert!(outcome.is_voluntary_only());
    }

    #[test]
    fn large_scale_monitoring_is_mandatory_37_1_b() {
        let mut a = baseline();
        a.regular_systematic_monitoring_is_core = true;
        a.monitoring_scale = MonitoringScale::LargeScale;
        let outcome = a.assess();
        assert!(outcome.is_mandatory());
        assert!(
            outcome
                .mandatory_grounds
                .contains(&DesignationGround::LargeScaleMonitoring)
        );
    }

    #[test]
    fn monitoring_not_core_is_not_mandatory() {
        let mut a = baseline();
        a.regular_systematic_monitoring_is_core = false;
        a.monitoring_scale = MonitoringScale::LargeScale;
        let outcome = a.assess();
        assert!(!outcome.is_mandatory());
    }

    #[test]
    fn small_scale_monitoring_is_not_mandatory() {
        let mut a = baseline();
        a.regular_systematic_monitoring_is_core = true;
        a.monitoring_scale = MonitoringScale::SmallScale;
        let outcome = a.assess();
        assert!(!outcome.is_mandatory());
    }

    #[test]
    fn large_scale_special_category_is_mandatory_37_1_c() {
        let mut a = baseline();
        a.special_category_processing_is_core = true;
        a.special_category_scale = MonitoringScale::LargeScale;
        let outcome = a.assess();
        assert!(outcome.is_mandatory());
        assert!(
            outcome
                .mandatory_grounds
                .contains(&DesignationGround::LargeScaleSpecialCategory)
        );
    }

    #[test]
    fn large_scale_criminal_data_is_mandatory_37_1_c() {
        let mut a = baseline();
        a.criminal_data_processing_is_core = true;
        a.criminal_data_scale = MonitoringScale::LargeScale;
        let outcome = a.assess();
        assert!(outcome.is_mandatory());
        assert!(
            outcome
                .mandatory_grounds
                .contains(&DesignationGround::LargeScaleCriminalData)
        );
    }

    #[test]
    fn competent_authority_is_mandatory_dpa_s69() {
        let mut a = baseline();
        a.competent_authority_law_enforcement = true;
        let outcome = a.assess();
        assert!(outcome.is_mandatory());
        assert!(
            outcome
                .mandatory_grounds
                .contains(&DesignationGround::CompetentAuthority)
        );
    }

    #[test]
    fn unclear_scale_is_recommended_not_mandatory() {
        let mut a = baseline();
        a.special_category_processing_is_core = true;
        a.special_category_scale = MonitoringScale::Unclear;
        let outcome = a.assess();
        assert!(!outcome.is_mandatory());
        assert!(outcome.is_recommended());
        assert!(
            outcome
                .borderline_grounds
                .contains(&DesignationGround::LargeScaleSpecialCategory)
        );
    }

    #[test]
    fn multiple_grounds_can_apply() {
        let mut a = baseline();
        a.organisation_type = OrganisationType::PublicAuthority;
        a.special_category_processing_is_core = true;
        a.special_category_scale = MonitoringScale::LargeScale;
        let outcome = a.assess();
        assert_eq!(outcome.mandatory_grounds.len(), 2);
    }

    #[test]
    fn typical_small_business_is_voluntary_only() {
        let outcome = baseline().assess();
        assert!(!outcome.is_mandatory());
        assert!(!outcome.is_recommended());
        assert!(outcome.is_voluntary_only());
    }

    #[test]
    fn designation_ground_provisions() {
        assert_eq!(
            DesignationGround::PublicAuthority.statutory_provision(),
            "UK GDPR Article 37(1)(a)"
        );
        assert_eq!(
            DesignationGround::LargeScaleMonitoring.statutory_provision(),
            "UK GDPR Article 37(1)(b)"
        );
        assert_eq!(
            DesignationGround::CompetentAuthority.statutory_provision(),
            "DPA 2018 s.69"
        );
    }

    #[test]
    fn all_five_dpo_tasks_present() {
        let tasks = DpoTask::all();
        assert_eq!(tasks.len(), 5);
        assert_eq!(
            DpoTask::AdviseOnDpia.statutory_provision(),
            "UK GDPR Article 39(1)(c)"
        );
        assert!(!DpoTask::MonitorCompliance.description().is_empty());
    }

    #[test]
    fn position_compliant_when_all_conditions_met() {
        let position = DpoPosition {
            involved_in_all_issues: true,
            provided_with_resources: true,
            operationally_independent: true,
            protected_from_dismissal: true,
            reports_to_highest_management: true,
            free_of_conflict_of_interest: true,
        };
        assert!(position.is_compliant());
        assert!(position.compliance_failures().is_empty());
    }

    #[test]
    fn position_conflict_of_interest_is_a_failure() {
        let position = DpoPosition {
            involved_in_all_issues: true,
            provided_with_resources: true,
            operationally_independent: true,
            protected_from_dismissal: true,
            reports_to_highest_management: true,
            free_of_conflict_of_interest: false,
        };
        assert!(!position.is_compliant());
        let failures = position.compliance_failures();
        assert_eq!(failures.len(), 1);
        assert_eq!(failures[0].provision, "UK GDPR Article 38(6)");
    }

    #[test]
    fn position_reports_multiple_failures() {
        let position = DpoPosition {
            involved_in_all_issues: false,
            provided_with_resources: false,
            operationally_independent: true,
            protected_from_dismissal: true,
            reports_to_highest_management: true,
            free_of_conflict_of_interest: true,
        };
        assert_eq!(position.compliance_failures().len(), 2);
    }

    #[test]
    fn contact_details_compliant() {
        let contact = DpoContactDetails {
            name_or_title: "Data Protection Officer".to_string(),
            postal_address: "1 Privacy Way, London".to_string(),
            email: "dpo@example.com".to_string(),
            telephone: Some("+44 20 7000 0000".to_string()),
            published: true,
            notified_to_ico: true,
        };
        assert!(contact.is_compliant());
        assert!(contact.validate_notification().is_empty());
    }

    #[test]
    fn contact_details_not_notified_to_ico_is_a_failure() {
        let contact = DpoContactDetails {
            name_or_title: "Data Protection Officer".to_string(),
            postal_address: "1 Privacy Way, London".to_string(),
            email: "dpo@example.com".to_string(),
            telephone: None,
            published: true,
            notified_to_ico: false,
        };
        assert!(!contact.is_compliant());
        let failures = contact.validate_notification();
        assert!(failures.contains(&DpoNotificationFailure::NotNotifiedToIco));
        assert_eq!(
            DpoNotificationFailure::NotNotifiedToIco.statutory_provision(),
            "UK GDPR Article 37(7)"
        );
    }

    #[test]
    fn contact_details_no_contact_point_is_a_failure() {
        let contact = DpoContactDetails {
            name_or_title: "DPO".to_string(),
            postal_address: "   ".to_string(),
            email: String::new(),
            telephone: None,
            published: false,
            notified_to_ico: false,
        };
        let failures = contact.validate_notification();
        assert!(failures.contains(&DpoNotificationFailure::NoContactPoint));
        assert!(failures.contains(&DpoNotificationFailure::NotPublished));
        assert!(failures.contains(&DpoNotificationFailure::NotNotifiedToIco));
        assert_eq!(failures.len(), 3);
    }
}
