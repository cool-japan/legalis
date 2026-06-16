//! PDPA data breach notification regime (Part 6A, ss. 26A-26E).
//!
//! The 2020 amendments introduced a mandatory data breach notification regime:
//!
//! * **s. 26C** — duty to **assess**, in a reasonable and expeditious manner,
//!   whether a data breach is a *notifiable* data breach.
//! * **s. 26B** — defines a **notifiable** data breach. A breach is notifiable
//!   if it (a) results in, or is likely to result in, **significant harm** to an
//!   affected individual (s. 26B(1)(a), deemed by s. 26B(2) where prescribed
//!   data is involved); **or** (b) is, or is likely to be, of a **significant
//!   scale** (s. 26B(1)(b), deemed by s. 26B(3)(a) at **500** affected
//!   individuals — PDP (Notification of Data Breaches) Regulations 2021, reg. 4).
//! * **s. 26D(1)** — duty to notify the **PDPC** as soon as practicable, but in
//!   any case **no later than 3 calendar days** after the day the organisation
//!   *assesses* the breach to be notifiable.
//! * **s. 26D(2)** — duty to notify **affected individuals** (on or after
//!   notifying the PDPC) where the significant-harm limb applies, subject to the
//!   exceptions in s. 26D(5)-(7).
//!
//! A breach occurring only within an organisation is excluded (s. 26B(4)).

use super::consent::PersonalDataCategory;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

/// Prescribed significant-scale threshold: a data breach affecting at least this
/// number of individuals is of a *significant scale* and therefore notifiable.
///
/// PDPA s. 26B(3)(a) read with reg. 4 of the Personal Data Protection
/// (Notification of Data Breaches) Regulations 2021 (S 64/2021).
pub const SIGNIFICANT_SCALE_THRESHOLD: u32 = 500;

/// Maximum number of **calendar** days within which an organisation must notify
/// the PDPC of a notifiable data breach, running from the day the organisation
/// assesses the breach to be notifiable (PDPA s. 26D(1)).
pub const PDPC_NOTIFICATION_DEADLINE_DAYS: i64 = 3;

/// Kind of data breach event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreachType {
    /// Unauthorised access to personal data.
    UnauthorizedAccess,
    /// Unauthorised disclosure of personal data.
    UnauthorizedDisclosure,
    /// Unauthorised modification of personal data.
    UnauthorizedModification,
    /// Loss of storage media or device on which personal data is stored.
    DataLoss,
    /// Ransomware / encryption of data by an attacker.
    Ransomware,
    /// Theft of personal data.
    Theft,
    /// Accidental public exposure (e.g. misconfigured database).
    AccidentalExposure,
}

/// Whether the breach was contained entirely within a single organisation
/// (s. 26B(4) carve-out), which is *not* notifiable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreachScope {
    /// The breach was, or is likely to be, accessible outside the organisation.
    External,
    /// The breach occurred only within the organisation (e.g. an employee
    /// accessed data they were not authorised to, with no external disclosure).
    /// Excluded from notification by s. 26B(4).
    InternalOnly,
}

/// The reason (if any) a notifiable significant-harm breach is exempt from the
/// duty to notify affected individuals (s. 26D(5)).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IndividualNotificationExemption {
    /// Remedial action taken on or after assessing notifiability that renders
    /// significant harm to the individual unlikely (s. 26D(5)(a)).
    RemedialActionTaken,
    /// A technological protection measure (e.g. strong encryption) implemented
    /// **before** the breach that renders significant harm unlikely
    /// (s. 26D(5)(b)).
    TechnologicalProtection,
    /// A prescribed law-enforcement agency or the Commission instructed the
    /// organisation not to notify (s. 26D(6)).
    LawEnforcementInstruction,
    /// The Commission waived the requirement on the organisation's application
    /// (s. 26D(7)).
    CommissionWaiver,
}

impl IndividualNotificationExemption {
    /// Returns the governing PDPA section reference for this exemption.
    pub fn statute_section(&self) -> &'static str {
        match self {
            IndividualNotificationExemption::RemedialActionTaken => "PDPA s. 26D(5)(a)",
            IndividualNotificationExemption::TechnologicalProtection => "PDPA s. 26D(5)(b)",
            IndividualNotificationExemption::LawEnforcementInstruction => "PDPA s. 26D(6)",
            IndividualNotificationExemption::CommissionWaiver => "PDPA s. 26D(7)",
        }
    }
}

/// The outcome of a notifiability assessment (s. 26B / s. 26C).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotifiabilityAssessment {
    /// Whether the significant-harm limb is met (s. 26B(1)(a)/(2)).
    pub significant_harm: bool,
    /// Whether the significant-scale limb is met (s. 26B(1)(b)/(3)(a)).
    pub significant_scale: bool,
    /// Whether the breach is excluded as internal-only (s. 26B(4)).
    pub internal_only: bool,
}

impl NotifiabilityAssessment {
    /// Returns whether the breach is **notifiable** to the PDPC: at least one
    /// limb is satisfied and the breach is not internal-only (s. 26B).
    pub fn is_notifiable(&self) -> bool {
        !self.internal_only && (self.significant_harm || self.significant_scale)
    }

    /// Returns whether the duty to notify **affected individuals** is engaged.
    ///
    /// Individual notification is required only where the *significant-harm*
    /// limb applies (s. 26D(2)); a pure significant-scale (500+) breach does not
    /// of itself require notifying individuals.
    pub fn requires_individual_notification(&self) -> bool {
        self.is_notifiable() && self.significant_harm
    }
}

/// A data breach and its notification lifecycle (Part 6A).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataBreachNotification {
    /// Stable identifier for this breach record.
    pub breach_id: String,
    /// The kind of breach.
    pub breach_type: BreachType,
    /// Whether the breach extends outside the organisation (s. 26B(4)).
    pub scope: BreachScope,
    /// When the breach was discovered by the organisation.
    pub discovery_date: DateTime<Utc>,
    /// When the breach is believed to have occurred (if known).
    pub occurrence_date: Option<DateTime<Utc>>,
    /// When the organisation completed its s. 26C assessment of notifiability.
    /// The s. 26D(1) 3-calendar-day clock runs from this date.
    pub assessment_date: Option<DateTime<Utc>>,
    /// Number of individuals affected (for the significant-scale limb).
    pub affected_individuals: u32,
    /// Categories of personal data involved (for the significant-harm limb).
    pub affected_data_categories: Vec<PersonalDataCategory>,
    /// Whether a name or identification number is among the affected data, which
    /// is a precondition for the reg. 3(1)(a) significant-harm deeming.
    pub includes_name_or_id: bool,
    /// When the PDPC was notified (s. 26D(1)).
    pub pdpc_notification_date: Option<DateTime<Utc>>,
    /// When affected individuals were notified (s. 26D(2)).
    pub individuals_notification_date: Option<DateTime<Utc>>,
    /// Any exemption relied upon to avoid notifying individuals (s. 26D(5)-(7)).
    pub individual_notification_exemption: Option<IndividualNotificationExemption>,
    /// Free-text description of the breach.
    pub description: String,
    /// Remedial actions taken.
    pub remedial_actions: Vec<String>,
}

impl DataBreachNotification {
    /// Creates a new external data breach record discovered now.
    pub fn new(
        breach_id: impl Into<String>,
        breach_type: BreachType,
        affected_individuals: u32,
        description: impl Into<String>,
    ) -> Self {
        Self {
            breach_id: breach_id.into(),
            breach_type,
            scope: BreachScope::External,
            discovery_date: Utc::now(),
            occurrence_date: None,
            assessment_date: None,
            affected_individuals,
            affected_data_categories: Vec::new(),
            includes_name_or_id: false,
            pdpc_notification_date: None,
            individuals_notification_date: None,
            individual_notification_exemption: None,
            description: description.into(),
            remedial_actions: Vec::new(),
        }
    }

    /// Adds an affected personal-data category (idempotent), and records whether
    /// it is a name or identification number.
    pub fn add_affected_category(&mut self, category: PersonalDataCategory) -> &mut Self {
        if matches!(
            category,
            PersonalDataCategory::Name | PersonalDataCategory::IdentificationNumber
        ) {
            self.includes_name_or_id = true;
        }
        if !self.affected_data_categories.contains(&category) {
            self.affected_data_categories.push(category);
        }
        self
    }

    /// Returns `true` if the *significant-harm* limb is satisfied (s. 26B(1)(a),
    /// deemed by s. 26B(2) and reg. 3 of S 64/2021).
    ///
    /// Two deeming routes are modelled:
    /// * reg. 3(1)(a): a name or identification number **plus** at least one
    ///   prescribed category (financial or health data); or
    /// * reg. 3(1)(b): account credentials (a password/security/access code or
    ///   biometric tied to an account).
    pub fn meets_significant_harm(&self) -> bool {
        let has_credentials = self
            .affected_data_categories
            .contains(&PersonalDataCategory::AccountCredentials);
        if has_credentials {
            return true; // reg. 3(1)(b)
        }
        let has_prescribed = self
            .affected_data_categories
            .iter()
            .any(PersonalDataCategory::is_significant_harm_category);
        self.includes_name_or_id && has_prescribed // reg. 3(1)(a)
    }

    /// Returns `true` if the *significant-scale* limb is satisfied: at least
    /// [`SIGNIFICANT_SCALE_THRESHOLD`] (500) individuals are affected
    /// (s. 26B(1)(b)/(3)(a), reg. 4).
    pub fn meets_significant_scale(&self) -> bool {
        self.affected_individuals >= SIGNIFICANT_SCALE_THRESHOLD
    }

    /// Performs the s. 26B / s. 26C notifiability assessment.
    pub fn assess_notifiability(&self) -> NotifiabilityAssessment {
        NotifiabilityAssessment {
            significant_harm: self.meets_significant_harm(),
            significant_scale: self.meets_significant_scale(),
            internal_only: self.scope == BreachScope::InternalOnly,
        }
    }

    /// Returns whether the breach is notifiable to the PDPC (s. 26B).
    pub fn is_notifiable(&self) -> bool {
        self.assess_notifiability().is_notifiable()
    }

    /// Records completion of the s. 26C assessment, starting the s. 26D(1) clock.
    pub fn record_assessment(&mut self, when: DateTime<Utc>) -> &mut Self {
        self.assessment_date = Some(when);
        self
    }

    /// Records notification to the PDPC at the given time (s. 26D(1)).
    pub fn notify_pdpc(&mut self, when: DateTime<Utc>) -> &mut Self {
        self.pdpc_notification_date = Some(when);
        self
    }

    /// Records notification to affected individuals at the given time
    /// (s. 26D(2)).
    pub fn notify_individuals(&mut self, when: DateTime<Utc>) -> &mut Self {
        self.individuals_notification_date = Some(when);
        self
    }

    /// Records an exemption from the duty to notify individuals (s. 26D(5)-(7)).
    pub fn set_individual_notification_exemption(
        &mut self,
        exemption: IndividualNotificationExemption,
    ) -> &mut Self {
        self.individual_notification_exemption = Some(exemption);
        self
    }

    /// Adds a remedial action.
    pub fn add_remedial_action(&mut self, action: impl Into<String>) -> &mut Self {
        self.remedial_actions.push(action.into());
        self
    }

    /// Returns the **deadline** by which the PDPC must be notified: 3 calendar
    /// days after the assessment date (s. 26D(1)). Returns `None` if no
    /// assessment has been recorded yet.
    pub fn pdpc_notification_deadline(&self) -> Option<DateTime<Utc>> {
        self.assessment_date
            .map(|assessed| assessed + Duration::days(PDPC_NOTIFICATION_DEADLINE_DAYS))
    }

    /// Returns the number of **calendar** days between the assessment date and
    /// the PDPC notification date. `None` if either date is missing.
    pub fn days_to_pdpc_notification(&self) -> Option<i64> {
        match (self.assessment_date, self.pdpc_notification_date) {
            (Some(assessed), Some(notified)) => Some(calendar_days_between(assessed, notified)),
            _ => None,
        }
    }

    /// Returns `true` if the PDPC was notified within the 3-calendar-day deadline
    /// (s. 26D(1)). Returns `false` if not notified, or notified late, or if no
    /// assessment date has been recorded.
    pub fn is_pdpc_notification_timely(&self) -> bool {
        match self.days_to_pdpc_notification() {
            Some(days) => (0..=PDPC_NOTIFICATION_DEADLINE_DAYS).contains(&days),
            None => false,
        }
    }

    /// Returns whether the duty to notify affected individuals is engaged and
    /// has *not* been discharged or exempted (s. 26D(2)/(5)-(7)).
    pub fn individual_notification_outstanding(&self) -> bool {
        self.assess_notifiability()
            .requires_individual_notification()
            && self.individuals_notification_date.is_none()
            && self.individual_notification_exemption.is_none()
    }
}

/// Returns the number of whole **calendar** days from `start` to `end`,
/// computed on the calendar date (not on elapsed 24-hour periods), matching the
/// PDPA's "calendar days" wording in s. 26D(1).
///
/// A notification made later on the same calendar date as the assessment counts
/// as `0` days; the next calendar date counts as `1`, and so on.
pub fn calendar_days_between(start: DateTime<Utc>, end: DateTime<Utc>) -> i64 {
    (end.date_naive() - start.date_naive()).num_days()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn significant_scale_is_500_inclusive() {
        let mut b = DataBreachNotification::new("b", BreachType::DataLoss, 499, "x");
        assert!(!b.meets_significant_scale());
        b.affected_individuals = 500;
        assert!(b.meets_significant_scale());
        b.affected_individuals = 501;
        assert!(b.meets_significant_scale());
    }

    #[test]
    fn significant_harm_name_plus_financial() {
        let mut b = DataBreachNotification::new("b", BreachType::UnauthorizedAccess, 10, "x");
        b.add_affected_category(PersonalDataCategory::Name)
            .add_affected_category(PersonalDataCategory::Financial);
        assert!(b.meets_significant_harm());
    }

    #[test]
    fn significant_harm_credentials_alone() {
        let mut b = DataBreachNotification::new("b", BreachType::Theft, 1, "x");
        b.add_affected_category(PersonalDataCategory::AccountCredentials);
        assert!(b.meets_significant_harm());
    }

    #[test]
    fn name_alone_is_not_significant_harm() {
        let mut b = DataBreachNotification::new("b", BreachType::AccidentalExposure, 10, "x");
        b.add_affected_category(PersonalDataCategory::Name)
            .add_affected_category(PersonalDataCategory::Email);
        assert!(!b.meets_significant_harm());
    }

    #[test]
    fn internal_only_breach_not_notifiable() {
        let mut b = DataBreachNotification::new("b", BreachType::UnauthorizedAccess, 1000, "x");
        b.scope = BreachScope::InternalOnly;
        assert!(
            !b.is_notifiable(),
            "s. 26B(4) excludes internal-only breaches"
        );
    }

    #[test]
    fn calendar_days_same_day_is_zero() {
        let start = DateTime::parse_from_rfc3339("2026-01-10T09:00:00Z")
            .expect("valid")
            .with_timezone(&Utc);
        let end = DateTime::parse_from_rfc3339("2026-01-10T23:00:00Z")
            .expect("valid")
            .with_timezone(&Utc);
        assert_eq!(calendar_days_between(start, end), 0);
    }
}
