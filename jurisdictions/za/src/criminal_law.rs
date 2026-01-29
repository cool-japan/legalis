//! South African Criminal Law
//!
//! Based on Roman-Dutch common law and statutory law.
//!
//! ## Key Legislation
//!
//! - Criminal Law Amendment Act 105 of 1997 (minimum sentences)
//! - Criminal Procedure Act 51 of 1977
//! - Prevention and Combating of Corrupt Activities Act 12 of 2004
//! - Sexual Offences Act 32 of 2007
//! - Domestic Violence Act 116 of 1998
//!
//! ## Criminal Capacity
//!
//! - Under 10: No criminal capacity (irrebuttable presumption)
//! - 10-14: Rebuttable presumption of no capacity
//! - 14+: Full criminal capacity

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for criminal law operations
pub type CriminalResult<T> = Result<T, CriminalError>;

/// Criminal capacity by age
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CriminalCapacity {
    /// Under 10 years - no capacity (irrebuttable)
    NoCapacity,
    /// 10-14 years - rebuttable presumption
    RebuttablePresumption { age: u8 },
    /// 14+ years - full capacity
    FullCapacity,
}

impl CriminalCapacity {
    /// Determine capacity from age
    pub fn from_age(age: u8) -> Self {
        match age {
            0..=9 => Self::NoCapacity,
            10..=13 => Self::RebuttablePresumption { age },
            _ => Self::FullCapacity,
        }
    }

    /// Can be prosecuted
    pub fn can_be_prosecuted(&self) -> bool {
        !matches!(self, Self::NoCapacity)
    }
}

/// Elements of crime (common law)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CrimeElement {
    /// Conduct (actus reus)
    Conduct,
    /// Unlawfulness
    Unlawfulness,
    /// Fault (culpability - intention or negligence)
    Fault,
    /// Causation
    Causation,
}

/// Forms of fault (culpability)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Fault {
    /// Dolus (intention) - Direct, indirect, or dolus eventualis
    Intention(IntentionType),
    /// Culpa (negligence) - Failure to meet reasonable person standard
    Negligence,
}

/// Types of intention
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IntentionType {
    /// Dolus directus - direct intention to cause result
    Direct,
    /// Dolus indirectus - foresees as substantially certain
    Indirect,
    /// Dolus eventualis - foresees as possible and reconciles
    Eventualis,
}

/// Grounds of justification (unlawfulness excluded)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GroundOfJustification {
    /// Private defence (self-defense)
    PrivateDefence,
    /// Necessity
    Necessity,
    /// Consent
    Consent,
    /// Official capacity
    OfficialCapacity,
    /// Disciplinary chastisement (limited)
    DisciplinaryChastisement,
}

/// Grounds excluding fault
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GroundExcludingFault {
    /// Youth (under 14)
    Youth,
    /// Mental illness or defect
    MentalIllnessOrDefect,
    /// Intoxication (involuntary)
    Intoxication,
    /// Mistake of fact
    MistakeOfFact,
    /// Compulsion/Duress
    Compulsion,
}

/// Schedule offences (minimum sentences - Criminal Law Amendment Act)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScheduleOffence {
    /// Murder
    Murder,
    /// Rape
    Rape,
    /// Robbery with aggravating circumstances
    RobberyAggravating,
    /// Kidnapping
    Kidnapping,
    /// Child-stealing
    ChildStealing,
    /// Drug trafficking (over threshold)
    DrugTrafficking,
    /// Housebreaking with aggravating circumstances
    HousebreakingAggravating,
}

impl ScheduleOffence {
    /// Get schedule classification
    pub fn schedule(&self) -> u8 {
        // Part I (most serious), Part II, Part III
        match self {
            Self::Murder | Self::Rape => 1,
            Self::RobberyAggravating | Self::Kidnapping => 2,
            _ => 3,
        }
    }

    /// Get minimum sentence for first offender (years)
    pub fn minimum_sentence_first_offender(&self) -> u8 {
        match self {
            Self::Murder | Self::Rape => 10,
            Self::RobberyAggravating => 15,
            Self::Kidnapping => 10,
            Self::ChildStealing => 10,
            Self::DrugTrafficking => 10,
            Self::HousebreakingAggravating => 15,
        }
    }

    /// Get minimum sentence for second offender (years)
    pub fn minimum_sentence_second_offender(&self) -> u8 {
        match self {
            Self::Murder => 15,
            Self::Rape => 15,
            Self::RobberyAggravating => 20,
            Self::Kidnapping => 15,
            _ => 15,
        }
    }

    /// Can court deviate from minimum sentence
    pub fn can_deviate_with_substantial_and_compelling_circumstances(&self) -> bool {
        true // s51(3)(a) - court may deviate if substantial and compelling circumstances
    }
}

/// Sentence types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SentenceType {
    /// Imprisonment
    Imprisonment { years: u8 },
    /// Life imprisonment
    LifeImprisonment,
    /// Fine
    Fine { amount_zar: i64 },
    /// Suspended sentence
    SuspendedSentence { years: u8, conditions: String },
    /// Correctional supervision
    CorrectionalSupervision { months: u8 },
    /// Community service
    CommunityService { hours: u16 },
    /// Caution and discharge
    CautionAndDischarge,
}

/// Bail considerations (s60 CPA)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BailConsideration {
    /// Likelihood of fleeing
    pub flight_risk: bool,
    /// Danger to public
    pub danger_to_public: bool,
    /// Likelihood of interfering with investigation
    pub interference_risk: bool,
    /// Likelihood of committing further offences
    pub further_offence_risk: bool,
    /// Interests of justice
    pub interests_of_justice: bool,
}

impl BailConsideration {
    /// Should bail be granted
    pub fn should_grant_bail(&self) -> bool {
        !self.flight_risk
            && !self.danger_to_public
            && !self.interference_risk
            && !self.further_offence_risk
            && self.interests_of_justice
    }

    /// Schedule 5 or 6 offence (presumption against bail)
    pub fn schedule_5_or_6_offence(offence: &ScheduleOffence) -> bool {
        matches!(
            offence,
            ScheduleOffence::Murder | ScheduleOffence::Rape | ScheduleOffence::RobberyAggravating
        )
    }
}

/// Appeal rights
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppealCourt {
    /// High Court (from Magistrate)
    HighCourt,
    /// Supreme Court of Appeal
    SupremeCourtOfAppeal,
    /// Constitutional Court (constitutional matters)
    ConstitutionalCourt,
}

/// Criminal errors
#[derive(Debug, Error)]
pub enum CriminalError {
    /// No criminal capacity
    #[error("No criminal capacity (age {age} - under 10)")]
    NoCapacity { age: u8 },

    /// Element of crime missing
    #[error("Element of crime not proven: {element}")]
    ElementNotProven { element: String },

    /// Ground of justification applies
    #[error("Conduct justified: {ground}")]
    ConductJustified { ground: String },

    /// Minimum sentence violation
    #[error(
        "Sentence below minimum (offence: {offence}, minimum: {minimum} years, imposed: {imposed} years)"
    )]
    BelowMinimumSentence {
        offence: String,
        minimum: u8,
        imposed: u8,
    },

    /// Bail inappropriately granted
    #[error("Bail should not be granted: {reason}")]
    BailInappropriate { reason: String },

    /// Procedural error
    #[error("Procedural error (CPA): {description}")]
    ProceduralError { description: String },
}

/// Validate sentence against minimum sentence requirements
pub fn validate_sentence(
    offence: &ScheduleOffence,
    sentence_years: u8,
    is_first_offender: bool,
    substantial_and_compelling: bool,
) -> CriminalResult<()> {
    let minimum = if is_first_offender {
        offence.minimum_sentence_first_offender()
    } else {
        offence.minimum_sentence_second_offender()
    };

    if sentence_years < minimum && !substantial_and_compelling {
        return Err(CriminalError::BelowMinimumSentence {
            offence: format!("{:?}", offence),
            minimum,
            imposed: sentence_years,
        });
    }

    Ok(())
}

/// Validate bail decision
pub fn validate_bail(consideration: &BailConsideration) -> CriminalResult<()> {
    if consideration.danger_to_public {
        return Err(CriminalError::BailInappropriate {
            reason: "Danger to public".to_string(),
        });
    }

    if consideration.flight_risk {
        return Err(CriminalError::BailInappropriate {
            reason: "Flight risk".to_string(),
        });
    }

    Ok(())
}

/// Get criminal procedure checklist
pub fn get_criminal_procedure_checklist() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Arrest lawful", "s39-41 CPA"),
        ("Rights explained (s35 Constitution)", "s35 Constitution"),
        ("Detained person brought before court within 48h", "s50 CPA"),
        ("Bail hearing", "s60 CPA"),
        ("Charge specified", "s144 CPA"),
        ("Plea recorded", "s106 CPA"),
        (
            "Prosecution proves guilt beyond reasonable doubt",
            "Common law",
        ),
        ("All elements of crime proven", "Common law"),
        ("Mitigating/aggravating factors considered", "s274 CPA"),
        ("Sentence within legal limits", "s276-297 CPA"),
        ("Minimum sentence (if schedule offence)", "Act 105/1997"),
        ("Appeal rights explained", "s309 CPA"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_criminal_capacity_age() {
        assert_eq!(CriminalCapacity::from_age(8), CriminalCapacity::NoCapacity);
        assert!(matches!(
            CriminalCapacity::from_age(12),
            CriminalCapacity::RebuttablePresumption { age: 12 }
        ));
        assert_eq!(
            CriminalCapacity::from_age(16),
            CriminalCapacity::FullCapacity
        );
    }

    #[test]
    fn test_can_be_prosecuted() {
        assert!(!CriminalCapacity::NoCapacity.can_be_prosecuted());
        assert!(CriminalCapacity::RebuttablePresumption { age: 12 }.can_be_prosecuted());
        assert!(CriminalCapacity::FullCapacity.can_be_prosecuted());
    }

    #[test]
    fn test_schedule_offence_minimum_sentences() {
        assert_eq!(
            ScheduleOffence::Murder.minimum_sentence_first_offender(),
            10
        );
        assert_eq!(
            ScheduleOffence::Murder.minimum_sentence_second_offender(),
            15
        );
        assert_eq!(ScheduleOffence::Rape.minimum_sentence_first_offender(), 10);
    }

    #[test]
    fn test_schedule_classification() {
        assert_eq!(ScheduleOffence::Murder.schedule(), 1);
        assert_eq!(ScheduleOffence::RobberyAggravating.schedule(), 2);
    }

    #[test]
    fn test_validate_sentence_within_minimum() {
        let result = validate_sentence(&ScheduleOffence::Murder, 15, true, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_sentence_below_minimum() {
        let result = validate_sentence(&ScheduleOffence::Murder, 5, true, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_sentence_below_with_circumstances() {
        let result = validate_sentence(&ScheduleOffence::Murder, 5, true, true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_bail_consideration_grant() {
        let consideration = BailConsideration {
            flight_risk: false,
            danger_to_public: false,
            interference_risk: false,
            further_offence_risk: false,
            interests_of_justice: true,
        };
        assert!(consideration.should_grant_bail());
        assert!(validate_bail(&consideration).is_ok());
    }

    #[test]
    fn test_bail_consideration_deny() {
        let consideration = BailConsideration {
            flight_risk: true,
            danger_to_public: false,
            interference_risk: false,
            further_offence_risk: false,
            interests_of_justice: true,
        };
        assert!(!consideration.should_grant_bail());
        assert!(validate_bail(&consideration).is_err());
    }

    #[test]
    fn test_schedule_5_6_offence() {
        assert!(BailConsideration::schedule_5_or_6_offence(
            &ScheduleOffence::Murder
        ));
        assert!(BailConsideration::schedule_5_or_6_offence(
            &ScheduleOffence::Rape
        ));
    }

    #[test]
    fn test_criminal_procedure_checklist() {
        let checklist = get_criminal_procedure_checklist();
        assert!(!checklist.is_empty());
        assert!(checklist.len() >= 10);
    }
}
