//! UK Criminal Procedure
//!
//! This module covers criminal procedure under English law, including
//! PACE 1984 provisions and trial procedures.
//!
//! # Police and Criminal Evidence Act 1984 (PACE)
//!
//! PACE regulates police powers including:
//! - Stop and search (ss.1-7)
//! - Arrest (ss.24-33)
//! - Detention (ss.34-46)
//! - Questioning and treatment (ss.53-65)
//!
//! # Codes of Practice
//! - Code A: Stop and search
//! - Code B: Searching premises and seizure
//! - Code C: Detention, treatment and questioning
//! - Code D: Identification
//! - Code E: Audio recording
//! - Code F: Visual recording
//! - Code G: Arrest
//! - Code H: Terrorism

// Allow missing docs on enum variant struct fields - these are self-documenting in context
#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use crate::criminal::types::OffenceClassification;

// ============================================================================
// Arrest
// ============================================================================

/// Facts for arrest analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrestFacts {
    /// Who made the arrest?
    pub arresting_officer: ArrestingOfficer,
    /// Grounds for arrest
    pub grounds: ArrestGrounds,
    /// Was arrest necessary? (necessity criteria s.24(5))
    pub necessity: NecessityCriteria,
    /// Was caution given?
    pub caution_given: bool,
    /// Was reason for arrest given?
    pub reason_given: bool,
}

/// Who made the arrest
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArrestingOfficer {
    /// Police constable
    Constable,
    /// PCSO with limited powers
    PCSO,
    /// Private citizen (s.24A)
    Citizen,
}

/// Grounds for arrest under s.24 PACE
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrestGrounds {
    /// Is offence an indictable offence?
    pub indictable_offence: bool,
    /// Reasonable grounds to believe person is committing/has committed offence?
    pub reasonable_grounds: bool,
    /// Evidence supporting reasonable grounds
    pub evidence: Vec<String>,
}

/// Necessity criteria under s.24(5) PACE
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NecessityCriteria {
    /// Which necessity criterion applies?
    pub criterion: NecessityCriterion,
    /// Evidence supporting necessity
    pub evidence: String,
}

/// Necessity criteria for arrest
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NecessityCriterion {
    /// To ascertain person's name
    AscertainName,
    /// To ascertain person's address
    AscertainAddress,
    /// To prevent harm to person or another
    PreventHarm,
    /// To prevent damage to property
    PreventDamage,
    /// To prevent offence against public decency
    PreventDecency,
    /// To prevent obstruction of highway
    PreventObstruction,
    /// To protect child or vulnerable person
    ProtectVulnerable,
    /// To allow prompt and effective investigation
    PromptInvestigation,
    /// To prevent disappearance
    PreventDisappearance,
}

/// Result of arrest analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrestAnalysisResult {
    /// Was arrest lawful?
    pub lawful: bool,
    /// Grounds analysis
    pub grounds_analysis: String,
    /// Necessity analysis
    pub necessity_analysis: String,
    /// Formalities analysis
    pub formalities_analysis: String,
    /// Consequences if unlawful
    pub consequences: Option<UnlawfulArrestConsequences>,
}

/// Consequences of unlawful arrest
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnlawfulArrestConsequences {
    /// Civil liability
    pub civil_liability: String,
    /// Evidence exclusion risk
    pub evidence_exclusion: String,
    /// Potential charges against officer
    pub potential_charges: Vec<String>,
}

/// Arrest analyzer
pub struct ArrestAnalyzer;

impl ArrestAnalyzer {
    /// Analyze arrest lawfulness
    pub fn analyze(facts: &ArrestFacts) -> ArrestAnalysisResult {
        let mut lawful = true;

        // Check grounds
        let grounds_analysis = if !facts.grounds.indictable_offence {
            // For summary offences, check specific power
            "Summary offence - specific power required".to_string()
        } else if facts.grounds.reasonable_grounds {
            "Reasonable grounds established for indictable offence".to_string()
        } else {
            lawful = false;
            "No reasonable grounds for arrest".to_string()
        };

        // Check necessity
        let necessity_analysis = format!(
            "Necessity criterion {:?}: {}",
            facts.necessity.criterion, facts.necessity.evidence
        );

        // Check formalities
        let formalities_analysis = if facts.caution_given && facts.reason_given {
            "Caution given and reason for arrest stated".to_string()
        } else if !facts.caution_given {
            lawful = false;
            "Caution not given at time of arrest".to_string()
        } else {
            lawful = false;
            "Reason for arrest not stated".to_string()
        };

        let consequences = if !lawful {
            Some(UnlawfulArrestConsequences {
                civil_liability: "Potential claim for false imprisonment".into(),
                evidence_exclusion: "Evidence obtained may be excluded under s.78 PACE".into(),
                potential_charges: vec!["Assault".into(), "False imprisonment".into()],
            })
        } else {
            None
        };

        ArrestAnalysisResult {
            lawful,
            grounds_analysis,
            necessity_analysis,
            formalities_analysis,
            consequences,
        }
    }
}

// ============================================================================
// Detention
// ============================================================================

/// Facts for detention analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetentionFacts {
    /// Duration of detention
    pub duration: DetentionDuration,
    /// Type of offence
    pub offence_type: OffenceClassification,
    /// Reviews conducted
    pub reviews: Vec<DetentionReview>,
    /// Extensions granted
    pub extensions: Vec<DetentionExtension>,
    /// Rights provided
    pub rights: DetentionRights,
}

/// Detention duration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetentionDuration {
    /// Hours detained
    pub hours: u32,
    /// Still in custody?
    pub ongoing: bool,
}

/// Detention review
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetentionReview {
    /// Hours into detention
    pub at_hours: u32,
    /// Who conducted review
    pub reviewer: String,
    /// Outcome
    pub outcome: ReviewOutcome,
}

/// Review outcome
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewOutcome {
    /// Continued detention authorized
    Authorized,
    /// Detention not authorized - release
    NotAuthorized,
    /// Charge
    Charge,
}

/// Detention extension
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetentionExtension {
    /// Who authorized extension
    pub authorized_by: ExtensionAuthority,
    /// Extension period (hours)
    pub hours: u32,
    /// Grounds
    pub grounds: String,
}

/// Extension authority
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExtensionAuthority {
    /// Superintendent (up to 36 hours)
    Superintendent,
    /// Magistrates' Court (up to 96 hours)
    MagistratesCourt,
}

/// Detention rights
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetentionRights {
    /// Right to have someone informed (s.56)
    pub someone_informed: RightStatus,
    /// Right to legal advice (s.58)
    pub legal_advice: RightStatus,
    /// Right to consult Codes of Practice
    pub codes_consultation: bool,
    /// Appropriate adult (if juvenile/vulnerable)
    pub appropriate_adult: Option<bool>,
}

/// Status of detention right
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RightStatus {
    /// Right provided
    Provided,
    /// Right delayed (with authorization)
    Delayed { authorization: String },
    /// Right denied without proper authorization
    DeniedImproperly,
}

/// Result of detention analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetentionAnalysisResult {
    /// Is detention lawful?
    pub lawful: bool,
    /// Time limit analysis
    pub time_limit_analysis: String,
    /// Reviews analysis
    pub reviews_analysis: String,
    /// Rights analysis
    pub rights_analysis: String,
    /// Breaches identified
    pub breaches: Vec<String>,
}

/// Detention analyzer
pub struct DetentionAnalyzer;

impl DetentionAnalyzer {
    /// Analyze detention lawfulness
    pub fn analyze(facts: &DetentionFacts) -> DetentionAnalysisResult {
        let mut breaches = Vec::new();

        // Check time limits
        let basic_limit = 24; // hours
        let superintendent_limit = 36;
        let magistrates_limit = 96;

        let time_limit_analysis = if facts.duration.hours <= basic_limit {
            "Within basic 24-hour limit".to_string()
        } else if facts.duration.hours <= superintendent_limit {
            // Check superintendent authorization
            let has_auth = facts
                .extensions
                .iter()
                .any(|e| matches!(e.authorized_by, ExtensionAuthority::Superintendent));
            if has_auth {
                "Extended to 36 hours with superintendent authorization".to_string()
            } else {
                breaches.push("Exceeded 24 hours without authorization".into());
                "No valid authorization for detention beyond 24 hours".to_string()
            }
        } else if facts.duration.hours <= magistrates_limit {
            let has_court_auth = facts
                .extensions
                .iter()
                .any(|e| matches!(e.authorized_by, ExtensionAuthority::MagistratesCourt));
            if has_court_auth {
                "Extended with magistrates' warrant of further detention".to_string()
            } else {
                breaches.push("Exceeded 36 hours without court warrant".into());
                "No valid warrant for detention beyond 36 hours".to_string()
            }
        } else {
            breaches.push("Exceeded maximum 96-hour detention limit".into());
            "Exceeded maximum detention period".to_string()
        };

        // Check reviews (first at 6 hours, then every 9 hours)
        let reviews_analysis = if facts.reviews.is_empty() && facts.duration.hours >= 6 {
            breaches.push("No detention review conducted".into());
            "No reviews conducted despite detention over 6 hours".to_string()
        } else {
            format!("{} detention reviews conducted", facts.reviews.len())
        };

        // Check rights
        let rights_analysis = Self::analyze_rights(&facts.rights, &mut breaches);

        let lawful = breaches.is_empty();

        DetentionAnalysisResult {
            lawful,
            time_limit_analysis,
            reviews_analysis,
            rights_analysis,
            breaches,
        }
    }

    fn analyze_rights(rights: &DetentionRights, breaches: &mut Vec<String>) -> String {
        let mut parts: Vec<String> = Vec::new();

        match &rights.someone_informed {
            RightStatus::Provided => parts.push("Right to have someone informed: provided".into()),
            RightStatus::Delayed { authorization } => {
                parts.push(format!(
                    "Right to have someone informed: delayed ({})",
                    authorization
                ));
            }
            RightStatus::DeniedImproperly => {
                breaches.push("s.56 right to have someone informed denied improperly".into());
                parts.push("Right to have someone informed: BREACH".into());
            }
        }

        match &rights.legal_advice {
            RightStatus::Provided => parts.push("Right to legal advice: provided".into()),
            RightStatus::Delayed { authorization } => {
                parts.push(format!(
                    "Right to legal advice: delayed ({})",
                    authorization
                ));
            }
            RightStatus::DeniedImproperly => {
                breaches.push("s.58 right to legal advice denied improperly".into());
                parts.push("Right to legal advice: BREACH".into());
            }
        }

        if let Some(aa) = rights.appropriate_adult {
            if !aa {
                breaches.push("Appropriate adult not provided for vulnerable person".into());
                parts.push("Appropriate adult: NOT PROVIDED (BREACH)".into());
            } else {
                parts.push("Appropriate adult: provided".into());
            }
        }

        parts.join("; ")
    }
}

// ============================================================================
// Interview
// ============================================================================

/// Facts for interview analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterviewFacts {
    /// Was interview recorded?
    pub recorded: RecordingStatus,
    /// Caution given?
    pub caution: CautionStatus,
    /// Legal representation
    pub legal_rep: LegalRepresentation,
    /// Appropriate adult (if required)
    pub appropriate_adult: Option<AppropriateAdult>,
    /// Interview conduct
    pub conduct: InterviewConduct,
}

/// Recording status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecordingStatus {
    /// Audio recorded (Code E)
    AudioRecorded,
    /// Video recorded (Code F)
    VideoRecorded,
    /// Both audio and video
    Both,
    /// Not recorded (exceptional circumstances only)
    NotRecorded { reason: String },
}

/// Caution status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CautionStatus {
    /// Was full caution given?
    pub full_caution: bool,
    /// Was special warning given (if applicable)?
    pub special_warning: Option<SpecialWarning>,
}

/// Special warnings under CJPOA 1994
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpecialWarning {
    /// s.36 - Objects, substances, marks
    Section36,
    /// s.37 - Presence at scene
    Section37,
}

/// Legal representation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegalRepresentation {
    /// Solicitor present
    SolicitorPresent,
    /// Solicitor requested but delayed
    Delayed { reason: String },
    /// Solicitor waived
    Waived,
    /// Solicitor denied
    Denied { reason: String },
}

/// Appropriate adult status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppropriateAdult {
    /// Was AA present?
    pub present: bool,
    /// Who was the AA?
    pub identity: String,
}

/// Interview conduct
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterviewConduct {
    /// Duration (hours)
    pub duration_hours: f32,
    /// Were breaks provided?
    pub breaks_provided: bool,
    /// Any oppression or inducements?
    pub oppression: Option<String>,
}

/// Result of interview analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterviewAnalysisResult {
    /// Was interview conducted properly?
    pub proper: bool,
    /// Recording analysis
    pub recording_analysis: String,
    /// Rights analysis
    pub rights_analysis: String,
    /// Conduct analysis
    pub conduct_analysis: String,
    /// Admissibility risk
    pub admissibility_risk: AdmissibilityRisk,
}

/// Admissibility risk assessment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdmissibilityRisk {
    /// Risk level
    pub level: RiskLevel,
    /// Potential grounds for exclusion
    pub exclusion_grounds: Vec<ExclusionGround>,
}

/// Risk level
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    /// Low risk - likely admissible
    Low,
    /// Medium risk - may be challenged
    Medium,
    /// High risk - likely excluded
    High,
}

/// Grounds for exclusion
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExclusionGround {
    /// s.76(2)(a) PACE - Oppression
    Oppression,
    /// s.76(2)(b) PACE - Unreliability
    Unreliability,
    /// s.78 PACE - Unfairness
    Unfairness,
    /// Breach of Codes of Practice
    CodeBreach { code: String },
}

/// Interview analyzer
pub struct InterviewAnalyzer;

impl InterviewAnalyzer {
    /// Analyze interview procedure
    pub fn analyze(facts: &InterviewFacts) -> InterviewAnalysisResult {
        let mut exclusion_grounds = Vec::new();

        // Recording analysis
        let recording_analysis = match &facts.recorded {
            RecordingStatus::AudioRecorded
            | RecordingStatus::VideoRecorded
            | RecordingStatus::Both => "Interview properly recorded".to_string(),
            RecordingStatus::NotRecorded { reason } => {
                exclusion_grounds.push(ExclusionGround::CodeBreach { code: "E".into() });
                format!("Not recorded: {} - Code E breach", reason)
            }
        };

        // Rights analysis
        let rights_analysis = Self::analyze_rights(facts, &mut exclusion_grounds);

        // Conduct analysis
        let conduct_analysis = Self::analyze_conduct(&facts.conduct, &mut exclusion_grounds);

        let level = match exclusion_grounds.len() {
            0 => RiskLevel::Low,
            1..=2 => RiskLevel::Medium,
            _ => RiskLevel::High,
        };

        let proper = exclusion_grounds.is_empty();

        InterviewAnalysisResult {
            proper,
            recording_analysis,
            rights_analysis,
            conduct_analysis,
            admissibility_risk: AdmissibilityRisk {
                level,
                exclusion_grounds,
            },
        }
    }

    fn analyze_rights(facts: &InterviewFacts, grounds: &mut Vec<ExclusionGround>) -> String {
        let mut parts: Vec<String> = Vec::new();

        if !facts.caution.full_caution {
            grounds.push(ExclusionGround::Unfairness);
            parts.push("Caution NOT given".into());
        } else {
            parts.push("Caution given".into());
        }

        match &facts.legal_rep {
            LegalRepresentation::SolicitorPresent => parts.push("Solicitor present".into()),
            LegalRepresentation::Waived => parts.push("Legal advice waived".into()),
            LegalRepresentation::Delayed { reason } => {
                parts.push(format!("Solicitor delayed: {}", reason));
            }
            LegalRepresentation::Denied { reason } => {
                grounds.push(ExclusionGround::Unfairness);
                parts.push(format!("Solicitor denied: {}", reason));
            }
        }

        if let Some(aa) = &facts.appropriate_adult {
            if !aa.present {
                grounds.push(ExclusionGround::CodeBreach { code: "C".into() });
                parts.push("Appropriate adult NOT present (BREACH)".into());
            }
        }

        parts.join("; ")
    }

    fn analyze_conduct(conduct: &InterviewConduct, grounds: &mut Vec<ExclusionGround>) -> String {
        let mut parts: Vec<String> = Vec::new();

        if conduct.duration_hours > 2.0 && !conduct.breaks_provided {
            grounds.push(ExclusionGround::CodeBreach { code: "C".into() });
            parts.push("Long interview without breaks".into());
        }

        if let Some(oppression) = &conduct.oppression {
            grounds.push(ExclusionGround::Oppression);
            parts.push(format!("Oppression: {}", oppression));
        }

        if parts.is_empty() {
            "Interview conducted properly".to_string()
        } else {
            parts.join("; ")
        }
    }
}

// ============================================================================
// Mode of Trial
// ============================================================================

/// Facts for mode of trial determination
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModeOfTrialFacts {
    /// Offence classification
    pub classification: OffenceClassification,
    /// Prosecution's view
    pub prosecution_view: Option<ModePreference>,
    /// Defence election (for either-way)
    pub defence_election: Option<ModePreference>,
    /// Allocation guideline factors
    pub factors: Vec<AllocationFactor>,
}

/// Mode preference
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModePreference {
    /// Magistrates' Court
    Magistrates,
    /// Crown Court
    CrownCourt,
}

/// Allocation factors
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllocationFactor {
    /// Factor description
    pub factor: String,
    /// Points towards which court
    pub indicates: ModePreference,
}

/// Result of mode of trial analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModeOfTrialResult {
    /// Where will case be tried?
    pub trial_venue: TrialVenue,
    /// Reasoning
    pub reasoning: String,
}

/// Trial venue
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrialVenue {
    /// Magistrates' Court
    MagistratesCourt,
    /// Crown Court
    CrownCourt,
    /// Either (allocation decision pending)
    AllocationPending,
}

/// Mode of trial analyzer
pub struct ModeOfTrialAnalyzer;

impl ModeOfTrialAnalyzer {
    /// Determine mode of trial
    pub fn analyze(facts: &ModeOfTrialFacts) -> ModeOfTrialResult {
        let (trial_venue, reasoning) = match facts.classification {
            OffenceClassification::Summary => (
                TrialVenue::MagistratesCourt,
                "Summary offence - Magistrates' Court only".to_string(),
            ),
            OffenceClassification::IndictableOnly => (
                TrialVenue::CrownCourt,
                "Indictable-only offence - Crown Court only".to_string(),
            ),
            OffenceClassification::EitherWay => {
                // Check defence election first
                if facts.defence_election == Some(ModePreference::CrownCourt) {
                    (
                        TrialVenue::CrownCourt,
                        "Defendant elected Crown Court trial".to_string(),
                    )
                } else {
                    // Apply allocation guideline
                    let crown_factors = facts
                        .factors
                        .iter()
                        .filter(|f| f.indicates == ModePreference::CrownCourt)
                        .count();

                    if crown_factors > facts.factors.len() / 2 {
                        (
                            TrialVenue::CrownCourt,
                            "Factors indicate Crown Court appropriate".to_string(),
                        )
                    } else {
                        (
                            TrialVenue::MagistratesCourt,
                            "Magistrates' Court suitable for trial".to_string(),
                        )
                    }
                }
            }
        };

        ModeOfTrialResult {
            trial_venue,
            reasoning,
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lawful_arrest() {
        let facts = ArrestFacts {
            arresting_officer: ArrestingOfficer::Constable,
            grounds: ArrestGrounds {
                indictable_offence: true,
                reasonable_grounds: true,
                evidence: vec!["Caught in act".into()],
            },
            necessity: NecessityCriteria {
                criterion: NecessityCriterion::PromptInvestigation,
                evidence: "Need to preserve evidence".into(),
            },
            caution_given: true,
            reason_given: true,
        };

        let result = ArrestAnalyzer::analyze(&facts);
        assert!(result.lawful);
    }

    #[test]
    fn test_unlawful_arrest_no_caution() {
        let facts = ArrestFacts {
            arresting_officer: ArrestingOfficer::Constable,
            grounds: ArrestGrounds {
                indictable_offence: true,
                reasonable_grounds: true,
                evidence: vec![],
            },
            necessity: NecessityCriteria {
                criterion: NecessityCriterion::AscertainName,
                evidence: "Refused to give name".into(),
            },
            caution_given: false,
            reason_given: true,
        };

        let result = ArrestAnalyzer::analyze(&facts);
        assert!(!result.lawful);
        assert!(result.consequences.is_some());
    }

    #[test]
    fn test_lawful_detention() {
        let facts = DetentionFacts {
            duration: DetentionDuration {
                hours: 20,
                ongoing: false,
            },
            offence_type: OffenceClassification::EitherWay,
            reviews: vec![DetentionReview {
                at_hours: 6,
                reviewer: "Inspector".into(),
                outcome: ReviewOutcome::Authorized,
            }],
            extensions: vec![],
            rights: DetentionRights {
                someone_informed: RightStatus::Provided,
                legal_advice: RightStatus::Provided,
                codes_consultation: true,
                appropriate_adult: None,
            },
        };

        let result = DetentionAnalyzer::analyze(&facts);
        assert!(result.lawful);
    }

    #[test]
    fn test_detention_breach_time_limit() {
        let facts = DetentionFacts {
            duration: DetentionDuration {
                hours: 30,
                ongoing: true,
            },
            offence_type: OffenceClassification::EitherWay,
            reviews: vec![],
            extensions: vec![], // No authorization for extension
            rights: DetentionRights {
                someone_informed: RightStatus::Provided,
                legal_advice: RightStatus::Provided,
                codes_consultation: true,
                appropriate_adult: None,
            },
        };

        let result = DetentionAnalyzer::analyze(&facts);
        assert!(!result.lawful);
        assert!(result.breaches.iter().any(|b| b.contains("24 hours")));
    }

    #[test]
    fn test_proper_interview() {
        let facts = InterviewFacts {
            recorded: RecordingStatus::AudioRecorded,
            caution: CautionStatus {
                full_caution: true,
                special_warning: None,
            },
            legal_rep: LegalRepresentation::SolicitorPresent,
            appropriate_adult: None,
            conduct: InterviewConduct {
                duration_hours: 1.5,
                breaks_provided: true,
                oppression: None,
            },
        };

        let result = InterviewAnalyzer::analyze(&facts);
        assert!(result.proper);
        assert!(matches!(result.admissibility_risk.level, RiskLevel::Low));
    }

    #[test]
    fn test_interview_exclusion_risk() {
        let facts = InterviewFacts {
            recorded: RecordingStatus::NotRecorded {
                reason: "Equipment failure".into(),
            },
            caution: CautionStatus {
                full_caution: false,
                special_warning: None,
            },
            legal_rep: LegalRepresentation::Denied {
                reason: "Unknown".into(),
            },
            appropriate_adult: None,
            conduct: InterviewConduct {
                duration_hours: 4.0,
                breaks_provided: false,
                oppression: Some("Raised voice".into()),
            },
        };

        let result = InterviewAnalyzer::analyze(&facts);
        assert!(!result.proper);
        assert!(matches!(result.admissibility_risk.level, RiskLevel::High));
    }

    #[test]
    fn test_mode_of_trial_summary() {
        let facts = ModeOfTrialFacts {
            classification: OffenceClassification::Summary,
            prosecution_view: None,
            defence_election: None,
            factors: vec![],
        };

        let result = ModeOfTrialAnalyzer::analyze(&facts);
        assert!(matches!(result.trial_venue, TrialVenue::MagistratesCourt));
    }

    #[test]
    fn test_mode_of_trial_defence_election() {
        let facts = ModeOfTrialFacts {
            classification: OffenceClassification::EitherWay,
            prosecution_view: Some(ModePreference::Magistrates),
            defence_election: Some(ModePreference::CrownCourt),
            factors: vec![],
        };

        let result = ModeOfTrialAnalyzer::analyze(&facts);
        assert!(matches!(result.trial_venue, TrialVenue::CrownCourt));
    }
}
