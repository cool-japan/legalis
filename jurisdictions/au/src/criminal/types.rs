//! Criminal Law Types
//!
//! Types for Australian criminal law analysis.

use serde::{Deserialize, Serialize};

use crate::common::StateTerritory;

// ============================================================================
// Offence Types
// ============================================================================

/// Jurisdiction for criminal offence
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CriminalJurisdiction {
    /// Commonwealth (federal) offence
    Commonwealth,
    /// State/Territory offence
    State(StateTerritory),
}

/// Category of offence
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum OffenceCategory {
    /// Summary offence (heard in Magistrates/Local Court)
    #[default]
    Summary,
    /// Indictable offence (heard in higher courts)
    Indictable,
    /// Indictable treated summarily (hybrid)
    IndictableSummarily,
}

/// Type of criminal offence
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OffenceType {
    /// Murder
    Murder,
    /// Manslaughter
    Manslaughter,
    /// Assault (various grades)
    Assault(AssaultGrade),
    /// Sexual offence
    SexualOffence(SexualOffenceType),
    /// Theft
    Theft,
    /// Robbery
    Robbery,
    /// Burglary/Breaking and entering
    Burglary,
    /// Fraud
    Fraud(FraudType),
    /// Drug offence
    DrugOffence(DrugOffenceType),
    /// Firearms offence
    FirearmsOffence,
    /// Terrorism offence
    TerrorismOffence,
    /// Commonwealth regulatory offence
    RegulatoryOffence,
    /// Other specified offence
    Other(String),
}

/// Grade of assault
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssaultGrade {
    /// Common assault
    Common,
    /// Assault occasioning actual bodily harm
    ActualBodilyHarm,
    /// Assault occasioning grievous bodily harm
    GrievousBodilyHarm,
    /// Wounding
    Wounding,
    /// Assault with intent
    WithIntent,
}

/// Type of sexual offence
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SexualOffenceType {
    /// Sexual assault
    SexualAssault,
    /// Aggravated sexual assault
    AggravatedSexualAssault,
    /// Indecent assault
    IndecentAssault,
    /// Act of indecency
    ActOfIndecency,
    /// Child sexual abuse
    ChildSexualAbuse,
}

/// Type of fraud
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FraudType {
    /// Obtaining by deception
    ObtainingByDeception,
    /// False accounting
    FalseAccounting,
    /// Identity fraud
    IdentityFraud,
    /// Tax fraud
    TaxFraud,
    /// Securities fraud
    SecuritiesFraud,
}

/// Type of drug offence
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DrugOffenceType {
    /// Possession
    Possession,
    /// Use
    Use,
    /// Supply (small quantity)
    Supply,
    /// Supply (commercial quantity)
    CommercialSupply,
    /// Manufacture
    Manufacture,
    /// Importation
    Importation,
    /// Trafficking
    Trafficking,
}

// ============================================================================
// Elements of Offence
// ============================================================================

/// Element type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ElementType {
    /// Physical element (actus reus)
    Physical(PhysicalElement),
    /// Fault element (mens rea)
    Fault(FaultElement),
}

/// Physical element (actus reus) types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhysicalElement {
    /// Conduct (act or omission)
    Conduct,
    /// Result of conduct
    Result,
    /// Circumstance
    Circumstance,
}

/// Fault element (mens rea) types - Criminal Code Act 1995 (Cth)
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum FaultElement {
    /// Intention (s.5.2)
    #[default]
    Intention,
    /// Knowledge (s.5.3)
    Knowledge,
    /// Recklessness (s.5.4)
    Recklessness,
    /// Negligence (s.5.5)
    Negligence,
    /// Strict liability (s.6.1)
    StrictLiability,
    /// Absolute liability (s.6.2)
    AbsoluteLiability,
}

impl FaultElement {
    /// Get Criminal Code Act section
    pub fn section(&self) -> &'static str {
        match self {
            Self::Intention => "s.5.2",
            Self::Knowledge => "s.5.3",
            Self::Recklessness => "s.5.4",
            Self::Negligence => "s.5.5",
            Self::StrictLiability => "s.6.1",
            Self::AbsoluteLiability => "s.6.2",
        }
    }

    /// Get definition
    pub fn definition(&self) -> &'static str {
        match self {
            Self::Intention => {
                "Means to bring about a result or is aware that it will occur in ordinary course"
            }
            Self::Knowledge => "Aware that circumstance exists or will exist",
            Self::Recklessness => "Aware of substantial risk and unjustifiable to take it",
            Self::Negligence => {
                "Such a great falling short of care that a reasonable person would exercise"
            }
            Self::StrictLiability => {
                "No fault required but defence of honest and reasonable mistake available"
            }
            Self::AbsoluteLiability => "No fault required and no defence of mistake",
        }
    }
}

// ============================================================================
// Defences
// ============================================================================

/// Defence type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Defence {
    /// Self-defence (s.10.4)
    SelfDefence,
    /// Defence of others
    DefenceOfOthers,
    /// Defence of property
    DefenceOfProperty,
    /// Duress (s.10.2)
    Duress,
    /// Sudden or extraordinary emergency (s.10.3)
    SuddenEmergency,
    /// Mental impairment (s.7.3)
    MentalImpairment,
    /// Intoxication (s.8)
    Intoxication,
    /// Mistake of fact (s.9.1)
    MistakeOfFact,
    /// Mistake of law (general rule: no defence)
    MistakeOfLaw,
    /// Claim of right
    ClaimOfRight,
    /// Consent
    Consent,
    /// Lawful authority
    LawfulAuthority,
    /// Provocation (partial - murder to manslaughter)
    Provocation,
    /// Diminished responsibility (partial)
    DiminishedResponsibility,
    /// Automatism
    Automatism,
    /// Infancy (under 10)
    Infancy,
}

impl Defence {
    /// Get Criminal Code Act section (Commonwealth)
    pub fn cth_section(&self) -> Option<&'static str> {
        match self {
            Self::SelfDefence => Some("s.10.4"),
            Self::Duress => Some("s.10.2"),
            Self::SuddenEmergency => Some("s.10.3"),
            Self::MentalImpairment => Some("s.7.3"),
            Self::Intoxication => Some("s.8"),
            Self::MistakeOfFact => Some("s.9.1"),
            _ => None,
        }
    }
}

// ============================================================================
// Sentencing
// ============================================================================

/// Type of sentence
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SentenceType {
    /// Imprisonment (full-time custody)
    Imprisonment,
    /// Intensive correction order
    IntensiveCorrectionOrder,
    /// Suspended sentence (where available)
    SuspendedSentence,
    /// Community correction order
    CommunityCorrectionOrder,
    /// Good behaviour bond/recognisance
    GoodBehaviourBond,
    /// Fine
    Fine,
    /// Conditional release order
    ConditionalReleaseOrder,
    /// Dismissal/discharge
    Dismissal,
    /// No conviction recorded
    NoConvictionRecorded,
}

/// Sentencing purpose (Crimes (Sentencing Procedure) Act 1999 (NSW) s.3A)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SentencingPurpose {
    /// Ensure offender adequately punished
    Punishment,
    /// Prevent crime by deterring offender
    SpecificDeterrence,
    /// Prevent crime by deterring others
    GeneralDeterrence,
    /// Protect community
    CommunityProtection,
    /// Promote rehabilitation
    Rehabilitation,
    /// Make offender accountable
    Accountability,
    /// Denounce conduct
    Denunciation,
    /// Recognise harm to victim and community
    RecognitionOfHarm,
}

/// Aggravating factor
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AggravatingFactor {
    /// Offence committed while on bail/parole
    WhileOnBail,
    /// Use of weapon
    UseOfWeapon,
    /// Victim was vulnerable
    VulnerableVictim,
    /// Offence committed in company
    InCompany,
    /// Breach of trust
    BreachOfTrust,
    /// Premeditation/planning
    Premeditation,
    /// Substantial harm to victim
    SubstantialHarm,
    /// Gratuitous cruelty
    GratuitousCruelty,
    /// Motivated by hatred
    HateMotivation,
    /// High degree of planning
    HighDegreeOfPlanning,
}

/// Mitigating factor
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MitigatingFactor {
    /// Early guilty plea
    EarlyGuiltyPlea,
    /// Cooperation with authorities
    Cooperation,
    /// Previous good character
    GoodCharacter,
    /// Remorse
    Remorse,
    /// Provocation
    Provocation,
    /// Mental health condition
    MentalHealth,
    /// Youth
    Youth,
    /// Hardship to dependants
    HardshipToDependants,
    /// Unlikely to reoffend
    LowReoffendingRisk,
    /// Made reparations
    Reparations,
}

// ============================================================================
// Key Cases
// ============================================================================

/// Key Australian criminal law case
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriminalCase {
    /// Case name
    pub name: String,
    /// Citation
    pub citation: String,
    /// Court
    pub court: String,
    /// Key principle
    pub principle: String,
}

impl CriminalCase {
    /// He Kaw Teh v R (1985) - Importation mens rea
    pub fn he_kaw_teh() -> Self {
        Self {
            name: "He Kaw Teh v The Queen".to_string(),
            citation: "(1985) 157 CLR 523".to_string(),
            court: "High Court".to_string(),
            principle: "Presumption of mens rea in serious drug offences".to_string(),
        }
    }

    /// Brennan v R (1936) - Intoxication
    pub fn brennan() -> Self {
        Self {
            name: "Brennan v The King".to_string(),
            citation: "(1936) 55 CLR 253".to_string(),
            court: "High Court".to_string(),
            principle: "Voluntary intoxication cannot negate basic intent".to_string(),
        }
    }

    /// Zecevic v DPP (Vic) (1987) - Self-defence
    pub fn zecevic() -> Self {
        Self {
            name: "Zecevic v DPP (Vic)".to_string(),
            citation: "(1987) 162 CLR 645".to_string(),
            court: "High Court".to_string(),
            principle: "Self-defence test: believed on reasonable grounds conduct necessary"
                .to_string(),
        }
    }

    /// R v Falconer (1990) - Automatism
    pub fn falconer() -> Self {
        Self {
            name: "R v Falconer".to_string(),
            citation: "(1990) 171 CLR 30".to_string(),
            court: "High Court".to_string(),
            principle: "Sane automatism as complete defence negating voluntariness".to_string(),
        }
    }

    /// Veen v R (No 2) (1988) - Sentencing proportionality
    pub fn veen_no2() -> Self {
        Self {
            name: "Veen v The Queen (No 2)".to_string(),
            citation: "(1988) 164 CLR 465".to_string(),
            court: "High Court".to_string(),
            principle: "Sentence must be proportionate to gravity of offence".to_string(),
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
    fn test_fault_element_sections() {
        assert_eq!(FaultElement::Intention.section(), "s.5.2");
        assert_eq!(FaultElement::Recklessness.section(), "s.5.4");
        assert_eq!(FaultElement::StrictLiability.section(), "s.6.1");
    }

    #[test]
    fn test_defence_sections() {
        assert_eq!(Defence::SelfDefence.cth_section(), Some("s.10.4"));
        assert_eq!(Defence::Duress.cth_section(), Some("s.10.2"));
        assert_eq!(Defence::Provocation.cth_section(), None);
    }

    #[test]
    fn test_he_kaw_teh_case() {
        let case = CriminalCase::he_kaw_teh();
        assert!(case.citation.contains("157 CLR"));
    }

    #[test]
    fn test_zecevic_self_defence() {
        let case = CriminalCase::zecevic();
        assert!(case.principle.contains("Self-defence"));
    }
}
