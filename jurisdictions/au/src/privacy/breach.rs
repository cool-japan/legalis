//! Notifiable Data Breaches (NDB) Scheme
//!
//! This module implements the Notifiable Data Breaches scheme under Part IIIC
//! of the Privacy Act 1988, in effect since February 2018.
//!
//! ## Eligible Data Breach
//!
//! An eligible data breach occurs when there is:
//! 1. Unauthorized access to, disclosure of, or loss of personal information
//! 2. Likely serious harm to individuals
//! 3. Remedial action has not been taken to prevent harm
//!
//! ## Notification Requirements
//!
//! Entities must:
//! - Complete assessment within 30 days (or as soon as practicable)
//! - Notify OAIC and affected individuals if breach is eligible
//! - Include specified information in notification

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::types::PersonalInformationType;

/// Data breach record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataBreach {
    /// Unique breach identifier
    pub breach_id: String,
    /// Date breach occurred (if known)
    pub occurrence_date: Option<DateTime<Utc>>,
    /// Date breach discovered
    pub discovery_date: DateTime<Utc>,
    /// Type of breach
    pub breach_type: BreachType,
    /// Categories of information affected
    pub affected_information: Vec<PersonalInformationType>,
    /// Number of individuals affected (if known)
    pub affected_individuals: Option<u32>,
    /// Description of breach
    pub description: String,
    /// Cause of breach
    pub cause: BreachCause,
    /// Remedial actions taken
    pub remedial_actions: Vec<String>,
    /// Assessment status
    pub assessment_status: AssessmentStatus,
    /// Assessment completion date
    pub assessment_completed: Option<DateTime<Utc>>,
}

impl DataBreach {
    /// Create new data breach record
    pub fn new(
        breach_id: impl Into<String>,
        breach_type: BreachType,
        description: impl Into<String>,
    ) -> Self {
        Self {
            breach_id: breach_id.into(),
            occurrence_date: None,
            discovery_date: Utc::now(),
            breach_type,
            affected_information: Vec::new(),
            affected_individuals: None,
            description: description.into(),
            cause: BreachCause::Unknown,
            remedial_actions: Vec::new(),
            assessment_status: AssessmentStatus::NotStarted,
            assessment_completed: None,
        }
    }

    /// Set occurrence date
    pub fn with_occurrence_date(mut self, date: DateTime<Utc>) -> Self {
        self.occurrence_date = Some(date);
        self
    }

    /// Add affected information type
    pub fn add_affected_information(&mut self, info_type: PersonalInformationType) {
        if !self.affected_information.contains(&info_type) {
            self.affected_information.push(info_type);
        }
    }

    /// Set affected individuals count
    pub fn with_affected_count(mut self, count: u32) -> Self {
        self.affected_individuals = Some(count);
        self
    }

    /// Add remedial action
    pub fn add_remedial_action(&mut self, action: impl Into<String>) {
        self.remedial_actions.push(action.into());
    }

    /// Calculate days since discovery
    pub fn days_since_discovery(&self) -> i64 {
        let now = Utc::now();
        now.signed_duration_since(self.discovery_date).num_days()
    }

    /// Check if assessment deadline approaching (> 25 days)
    pub fn assessment_deadline_approaching(&self) -> bool {
        self.days_since_discovery() > 25 && self.assessment_completed.is_none()
    }

    /// Check if assessment overdue (> 30 days)
    pub fn assessment_overdue(&self) -> bool {
        self.days_since_discovery() > 30 && self.assessment_completed.is_none()
    }
}

/// Type of data breach
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreachType {
    /// Unauthorized access
    UnauthorizedAccess,
    /// Unauthorized disclosure
    UnauthorizedDisclosure,
    /// Loss of personal information
    Loss,
}

/// Cause of data breach
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreachCause {
    /// Malicious external attack
    MaliciousExternal,
    /// Malicious internal (insider threat)
    MaliciousInternal,
    /// Human error
    HumanError,
    /// System fault
    SystemFault,
    /// Lost/stolen device
    LostStolenDevice,
    /// Social engineering
    SocialEngineering,
    /// Unknown
    Unknown,
}

/// Assessment status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssessmentStatus {
    /// Assessment not started
    NotStarted,
    /// Assessment in progress
    InProgress,
    /// Assessment completed - eligible breach
    CompletedEligible,
    /// Assessment completed - not eligible
    CompletedNotEligible,
}

/// Breach assessment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreachAssessment {
    /// Breach being assessed
    pub breach_id: String,
    /// Assessment date
    pub assessment_date: DateTime<Utc>,
    /// Assessed by
    pub assessed_by: String,
    /// Whether breach is eligible
    pub is_eligible: bool,
    /// Serious harm assessment
    pub harm_assessment: SeriousHarmAssessment,
    /// Whether remedial action prevented harm
    pub remedial_prevented_harm: bool,
    /// Recommendation
    pub recommendation: AssessmentRecommendation,
    /// Reasoning
    pub reasoning: String,
}

impl BreachAssessment {
    /// Create new assessment
    pub fn new(breach_id: impl Into<String>, assessed_by: impl Into<String>) -> Self {
        Self {
            breach_id: breach_id.into(),
            assessment_date: Utc::now(),
            assessed_by: assessed_by.into(),
            is_eligible: false,
            harm_assessment: SeriousHarmAssessment::default(),
            remedial_prevented_harm: false,
            recommendation: AssessmentRecommendation::NoNotificationRequired,
            reasoning: String::new(),
        }
    }

    /// Assess eligibility
    pub fn assess_eligibility(&mut self) {
        // Eligible if serious harm likely AND remedial action hasn't prevented harm
        self.is_eligible =
            self.harm_assessment.likely_serious_harm && !self.remedial_prevented_harm;

        self.recommendation = if self.is_eligible {
            AssessmentRecommendation::NotifyOaicAndIndividuals
        } else {
            AssessmentRecommendation::NoNotificationRequired
        };
    }
}

/// Serious harm assessment
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SeriousHarmAssessment {
    /// Type of information involved
    pub information_types: Vec<PersonalInformationType>,
    /// Whether harm is likely
    pub likely_serious_harm: bool,
    /// Types of harm that could result
    pub potential_harm_types: Vec<HarmType>,
    /// Factors increasing likelihood of harm
    pub increasing_factors: Vec<String>,
    /// Factors decreasing likelihood of harm
    pub decreasing_factors: Vec<String>,
    /// Risk level
    pub risk_level: HarmRiskLevel,
}

impl SeriousHarmAssessment {
    /// Assess harm likelihood
    pub fn assess(&mut self) {
        // Sensitive information increases harm likelihood
        let has_sensitive = self.information_types.iter().any(|t| t.is_sensitive());

        // Financial/identity information increases harm likelihood
        let has_financial = self
            .information_types
            .contains(&PersonalInformationType::Financial);
        let has_identity = self
            .information_types
            .iter()
            .any(|t| matches!(t, PersonalInformationType::DateOfBirth));

        let increasing = self.increasing_factors.len();
        let decreasing = self.decreasing_factors.len();

        // Determine risk level
        self.risk_level = if has_sensitive || (has_financial && has_identity) {
            if increasing > decreasing {
                HarmRiskLevel::High
            } else {
                HarmRiskLevel::Medium
            }
        } else if increasing > decreasing + 1 {
            HarmRiskLevel::Medium
        } else {
            HarmRiskLevel::Low
        };

        // Determine likelihood
        self.likely_serious_harm =
            matches!(self.risk_level, HarmRiskLevel::High | HarmRiskLevel::Medium);
    }
}

/// Type of harm
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HarmType {
    /// Financial harm
    Financial,
    /// Identity theft
    IdentityTheft,
    /// Reputation damage
    ReputationDamage,
    /// Physical harm
    Physical,
    /// Psychological harm
    Psychological,
    /// Discrimination
    Discrimination,
    /// Legal consequences
    Legal,
}

/// Harm risk level
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum HarmRiskLevel {
    #[default]
    /// Low risk
    Low,
    /// Medium risk
    Medium,
    /// High risk
    High,
}

/// Assessment recommendation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssessmentRecommendation {
    /// No notification required
    NoNotificationRequired,
    /// Notify OAIC only
    NotifyOaicOnly,
    /// Notify OAIC and affected individuals
    NotifyOaicAndIndividuals,
    /// Further assessment required
    FurtherAssessmentRequired,
}

/// Eligible data breach (requires notification)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EligibleBreach {
    /// Breach details
    pub breach: DataBreach,
    /// Assessment
    pub assessment: BreachAssessment,
    /// Notification requirement
    pub notification_requirement: NotificationRequirement,
}

/// Notification requirement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationRequirement {
    /// Notify OAIC
    pub notify_oaic: bool,
    /// Notify individuals
    pub notify_individuals: bool,
    /// OAIC notification date
    pub oaic_notification_date: Option<DateTime<Utc>>,
    /// Individual notification date
    pub individual_notification_date: Option<DateTime<Utc>>,
    /// Notification content requirements
    pub content_requirements: NotificationContent,
}

impl Default for NotificationRequirement {
    fn default() -> Self {
        Self {
            notify_oaic: true,
            notify_individuals: true,
            oaic_notification_date: None,
            individual_notification_date: None,
            content_requirements: NotificationContent::default(),
        }
    }
}

/// Notification content requirements
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct NotificationContent {
    /// Entity identity and contact
    pub entity_identity: bool,
    /// Description of breach
    pub breach_description: bool,
    /// Types of information involved
    pub information_types: bool,
    /// Recommendations to individuals
    pub recommendations: bool,
    /// Steps being taken
    pub steps_taken: bool,
    /// Contact information for OAIC
    pub oaic_contact: bool,
}

impl NotificationContent {
    /// Check if all required content is addressed
    pub fn is_complete(&self) -> bool {
        self.entity_identity
            && self.breach_description
            && self.information_types
            && self.recommendations
    }
}

/// Breach notification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreachNotification {
    /// Notification ID
    pub notification_id: String,
    /// Breach ID
    pub breach_id: String,
    /// Notification type
    pub notification_type: NotificationType,
    /// Notification date
    pub notification_date: DateTime<Utc>,
    /// Entity name
    pub entity_name: String,
    /// Entity contact
    pub entity_contact: String,
    /// Breach description
    pub breach_description: String,
    /// Information types affected
    pub information_types_affected: Vec<String>,
    /// Recommendations to individuals
    pub recommendations: Vec<String>,
    /// Steps entity is taking
    pub steps_being_taken: Vec<String>,
}

impl BreachNotification {
    /// Create OAIC notification
    pub fn to_oaic(
        notification_id: impl Into<String>,
        breach: &DataBreach,
        entity_name: impl Into<String>,
        entity_contact: impl Into<String>,
    ) -> Self {
        Self {
            notification_id: notification_id.into(),
            breach_id: breach.breach_id.clone(),
            notification_type: NotificationType::Oaic,
            notification_date: Utc::now(),
            entity_name: entity_name.into(),
            entity_contact: entity_contact.into(),
            breach_description: breach.description.clone(),
            information_types_affected: breach
                .affected_information
                .iter()
                .map(|t| format!("{:?}", t))
                .collect(),
            recommendations: Vec::new(),
            steps_being_taken: breach.remedial_actions.clone(),
        }
    }

    /// Create individual notification
    pub fn to_individuals(
        notification_id: impl Into<String>,
        breach: &DataBreach,
        entity_name: impl Into<String>,
        entity_contact: impl Into<String>,
    ) -> Self {
        Self {
            notification_id: notification_id.into(),
            breach_id: breach.breach_id.clone(),
            notification_type: NotificationType::Individuals,
            notification_date: Utc::now(),
            entity_name: entity_name.into(),
            entity_contact: entity_contact.into(),
            breach_description: breach.description.clone(),
            information_types_affected: breach
                .affected_information
                .iter()
                .map(|t| format!("{:?}", t))
                .collect(),
            recommendations: Vec::new(),
            steps_being_taken: breach.remedial_actions.clone(),
        }
    }

    /// Add recommendation
    pub fn add_recommendation(&mut self, rec: impl Into<String>) {
        self.recommendations.push(rec.into());
    }
}

/// Notification type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationType {
    /// Notification to OAIC
    Oaic,
    /// Notification to individuals
    Individuals,
    /// Public notification (if individuals not identifiable)
    PublicNotification,
}

/// Data breach analyzer
pub struct DataBreachAnalyzer;

impl DataBreachAnalyzer {
    /// Assess whether breach is eligible for notification
    pub fn assess(breach: &DataBreach) -> BreachAssessment {
        let mut assessment = BreachAssessment::new(&breach.breach_id, "System");

        // Assess harm
        assessment.harm_assessment.information_types = breach.affected_information.clone();

        // Add increasing factors based on breach type
        match breach.breach_type {
            BreachType::UnauthorizedAccess => {
                assessment
                    .harm_assessment
                    .increasing_factors
                    .push("Unauthorized access may enable misuse".into());
            }
            BreachType::UnauthorizedDisclosure => {
                assessment
                    .harm_assessment
                    .increasing_factors
                    .push("Information disclosed to unauthorized parties".into());
            }
            BreachType::Loss => {
                assessment
                    .harm_assessment
                    .increasing_factors
                    .push("Information may be accessed by unknown parties".into());
            }
        }

        // Check for remedial actions
        if !breach.remedial_actions.is_empty() {
            assessment.harm_assessment.decreasing_factors.push(format!(
                "{} remedial actions taken",
                breach.remedial_actions.len()
            ));
            assessment.remedial_prevented_harm = breach.remedial_actions.len() >= 2;
        }

        // Assess harm likelihood
        assessment.harm_assessment.assess();

        // Determine eligibility
        assessment.assess_eligibility();

        assessment
    }

    /// Generate notification requirements
    pub fn notification_requirements(
        breach: &DataBreach,
        assessment: &BreachAssessment,
    ) -> NotificationRequirement {
        if !assessment.is_eligible {
            return NotificationRequirement {
                notify_oaic: false,
                notify_individuals: false,
                oaic_notification_date: None,
                individual_notification_date: None,
                content_requirements: NotificationContent::default(),
            };
        }

        NotificationRequirement {
            notify_oaic: true,
            notify_individuals: breach.affected_individuals.map(|n| n > 0).unwrap_or(true),
            oaic_notification_date: None,
            individual_notification_date: None,
            content_requirements: NotificationContent {
                entity_identity: true,
                breach_description: true,
                information_types: true,
                recommendations: true,
                oaic_contact: true,
                ..Default::default()
            },
        }
    }

    /// Check if assessment is timely
    pub fn is_timely_assessment(breach: &DataBreach) -> bool {
        breach.days_since_discovery() <= 30
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breach_creation() {
        let breach = DataBreach::new(
            "breach-001",
            BreachType::UnauthorizedAccess,
            "Database accessed by unauthorized party",
        );

        assert_eq!(breach.breach_id, "breach-001");
        assert_eq!(breach.assessment_status, AssessmentStatus::NotStarted);
    }

    #[test]
    fn test_breach_with_sensitive_info() {
        let mut breach = DataBreach::new(
            "breach-002",
            BreachType::UnauthorizedDisclosure,
            "Health records disclosed",
        );

        breach.add_affected_information(PersonalInformationType::Health);
        breach.add_affected_information(PersonalInformationType::Name);

        let assessment = DataBreachAnalyzer::assess(&breach);

        assert!(assessment.is_eligible);
        assert!(assessment.harm_assessment.likely_serious_harm);
    }

    #[test]
    fn test_breach_with_remedial_action() {
        let mut breach = DataBreach::new("breach-003", BreachType::Loss, "USB drive lost");

        breach.add_affected_information(PersonalInformationType::Name);
        breach.add_remedial_action("Remote wiped device");
        breach.add_remedial_action("Changed all passwords");
        breach.add_remedial_action("Notified affected users");

        let assessment = DataBreachAnalyzer::assess(&breach);

        // Remedial action may prevent eligibility
        assert!(assessment.remedial_prevented_harm);
    }

    #[test]
    fn test_harm_assessment() {
        let mut harm = SeriousHarmAssessment {
            information_types: vec![
                PersonalInformationType::Financial,
                PersonalInformationType::DateOfBirth,
            ],
            ..Default::default()
        };
        harm.increasing_factors
            .push("Data publicly accessible".into());

        harm.assess();

        assert!(harm.likely_serious_harm);
        assert_eq!(harm.risk_level, HarmRiskLevel::High);
    }

    #[test]
    fn test_notification_requirements() {
        let mut breach = DataBreach::new(
            "breach-004",
            BreachType::UnauthorizedAccess,
            "System compromised",
        )
        .with_affected_count(1000);

        breach.add_affected_information(PersonalInformationType::Financial);

        let assessment = DataBreachAnalyzer::assess(&breach);
        let requirements = DataBreachAnalyzer::notification_requirements(&breach, &assessment);

        if assessment.is_eligible {
            assert!(requirements.notify_oaic);
            assert!(requirements.notify_individuals);
        }
    }

    #[test]
    fn test_notification_content() {
        let content = NotificationContent {
            entity_identity: true,
            breach_description: true,
            information_types: true,
            recommendations: true,
            steps_taken: false,
            oaic_contact: false,
        };

        assert!(content.is_complete());
    }

    #[test]
    fn test_breach_notification_creation() {
        let breach = DataBreach::new(
            "breach-005",
            BreachType::UnauthorizedDisclosure,
            "Data leak",
        );

        let notification =
            BreachNotification::to_oaic("notif-001", &breach, "Acme Corp", "privacy@acme.com");

        assert_eq!(notification.notification_type, NotificationType::Oaic);
        assert_eq!(notification.entity_name, "Acme Corp");
    }
}
