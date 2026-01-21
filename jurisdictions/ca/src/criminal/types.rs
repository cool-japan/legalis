//! Canada Criminal Law - Types
//!
//! Core types for Canadian criminal law (Criminal Code, RSC 1985, c C-46).

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use crate::common::CaseCitation;

// ============================================================================
// Offence Classification
// ============================================================================

/// Classification of criminal offence
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OffenceType {
    /// Summary conviction offence (less serious)
    Summary,
    /// Indictable offence (more serious)
    Indictable,
    /// Hybrid/dual procedure offence (Crown elects)
    Hybrid,
}

/// Category of criminal offence
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OffenceCategory {
    /// Offences against the person
    AgainstPerson,
    /// Property offences
    Property,
    /// Sexual offences
    Sexual,
    /// Drug offences (CDSA)
    DrugRelated,
    /// Firearms offences
    Firearms,
    /// Motor vehicle offences
    MotorVehicle,
    /// Fraud and financial crimes
    Fraud,
    /// Administration of justice
    AdministrationOfJustice,
    /// Public order
    PublicOrder,
    /// Terrorism
    Terrorism,
    /// Other
    Other { description: String },
}

// ============================================================================
// Mens Rea (Mental Element)
// ============================================================================

/// Mens rea (mental element) requirement
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MensRea {
    /// Intention (subjective)
    Intention,
    /// Knowledge
    Knowledge,
    /// Recklessness (subjective awareness of risk)
    Recklessness,
    /// Criminal negligence (marked departure)
    CriminalNegligence,
    /// Willful blindness
    WillfulBlindness,
    /// Strict liability (defence of due diligence)
    StrictLiability,
    /// Absolute liability (no mens rea required)
    AbsoluteLiability,
}

/// Types of intention for murder
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntentionType {
    /// Direct intent (s.229(a)(i))
    Direct,
    /// Oblique intent - know death likely (s.229(a)(ii), Woollin)
    Oblique,
    /// Transferred intent (s.229(b))
    Transferred,
    /// Felony murder (s.229(c)) - during unlawful act
    FelonyMurder,
}

/// Recklessness standard
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecklessnessType {
    /// Subjective recklessness (actual awareness)
    Subjective,
    /// Advertent recklessness
    Advertent,
}

// ============================================================================
// Actus Reus (Physical Element)
// ============================================================================

/// Actus reus (physical element) components
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActusReus {
    /// Voluntary act
    VoluntaryAct,
    /// Omission where duty to act
    Omission { duty_source: DutySource },
    /// State of being
    StateOfBeing,
    /// Consequence
    Consequence,
    /// Circumstance
    Circumstance,
}

/// Source of duty for omission liability
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DutySource {
    /// Statutory duty (s.215 - necessaries of life)
    Statutory { section: String },
    /// Contractual duty
    Contractual,
    /// Relationship (parent-child)
    Relationship,
    /// Voluntary assumption
    VoluntaryAssumption,
    /// Creation of dangerous situation
    DangerCreation,
}

// ============================================================================
// Homicide Offences
// ============================================================================

/// Homicide type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HomicideType {
    /// First degree murder (s.231)
    FirstDegreeMurder,
    /// Second degree murder (s.231)
    SecondDegreeMurder,
    /// Manslaughter (s.234)
    Manslaughter,
    /// Infanticide (s.233)
    Infanticide,
    /// Criminal negligence causing death (s.220)
    CriminalNegligenceDeath,
}

/// Factors making murder first degree
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FirstDegreeFactor {
    /// Planned and deliberate (s.231(2))
    PlannedDeliberate,
    /// Murder of peace officer (s.231(4))
    PeaceOfficer,
    /// Murder during hijacking/sexual assault/kidnapping (s.231(5))
    DuringDesignatedOffence,
    /// Murder by criminal organization (s.231(6.1))
    CriminalOrganization,
    /// Terrorism (s.231(6.01))
    Terrorism,
    /// Intimidation of justice system (s.231(6.2))
    IntimidationJustice,
}

/// Manslaughter type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManslaughterType {
    /// Unlawful act manslaughter
    UnlawfulAct,
    /// Criminal negligence
    CriminalNegligence,
    /// Provocation reducing murder to manslaughter (s.232)
    Provoked,
}

// ============================================================================
// Assault Offences
// ============================================================================

/// Assault type (Criminal Code ss.265-269)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssaultType {
    /// Common assault (s.265/266)
    Common,
    /// Assault with weapon (s.267(a))
    WithWeapon,
    /// Assault causing bodily harm (s.267(b))
    CausingBodilyHarm,
    /// Aggravated assault (s.268)
    Aggravated,
    /// Sexual assault (s.271)
    Sexual,
    /// Sexual assault with weapon (s.272)
    SexualWithWeapon,
    /// Aggravated sexual assault (s.273)
    AggravatedSexual,
    /// Assault peace officer (s.270)
    PeaceOfficer,
}

/// Bodily harm level
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BodilyHarmLevel {
    /// Minor injury
    Minor,
    /// Bodily harm (transient or trifling - R v McCraw)
    BodilyHarm,
    /// Serious bodily harm
    Serious,
    /// Grievous bodily harm (wounds, maims, disfigures)
    Grievous,
}

// ============================================================================
// Property Offences
// ============================================================================

/// Theft type (Criminal Code ss.322-334)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TheftType {
    /// Theft over $5,000 (s.334(a))
    Over5000,
    /// Theft under $5,000 (s.334(b))
    Under5000,
    /// Identity theft (s.402.2)
    Identity,
    /// Motor vehicle theft (s.333.1)
    MotorVehicle,
}

/// Fraud type (Criminal Code ss.380-400)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FraudType {
    /// General fraud (s.380)
    General,
    /// Fraud over $5,000
    Over5000,
    /// Fraud under $5,000
    Under5000,
    /// Fraud affecting public market (s.380(2))
    PublicMarket,
    /// Securities fraud
    Securities,
}

/// Break and enter type (s.348)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreakEnterType {
    /// Breaking and entering dwelling
    Dwelling,
    /// Breaking and entering other place
    NonDwelling,
    /// Being unlawfully in dwelling
    UnlawfullyInDwelling,
}

// ============================================================================
// Defences
// ============================================================================

/// Criminal defence type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CriminalDefence {
    /// Self-defence (s.34)
    SelfDefence,
    /// Defence of another (s.34)
    DefenceOfAnother,
    /// Defence of property (s.35)
    DefenceOfProperty,
    /// Necessity (Perka v The Queen)
    Necessity,
    /// Duress (s.17 and common law)
    Duress,
    /// Provocation (s.232 - partial defence to murder)
    Provocation,
    /// Mental disorder (s.16 - NCR)
    MentalDisorder,
    /// Automatism (non-insane)
    Automatism,
    /// Intoxication (Daviault)
    Intoxication,
    /// Mistake of fact
    MistakeOfFact,
    /// Consent
    Consent,
    /// Entrapment
    Entrapment,
}

/// Self-defence elements (s.34)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfDefenceElements {
    /// Believed force or threat of force
    pub reasonable_belief_threat: bool,
    /// Act for purpose of defending
    pub defensive_purpose: bool,
    /// Act reasonable in circumstances
    pub reasonable_response: bool,
}

/// Necessity elements (Perka)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NecessityElements {
    /// Imminent peril
    pub imminent_peril: bool,
    /// No reasonable legal alternative
    pub no_legal_alternative: bool,
    /// Proportionality
    pub proportional: bool,
}

/// Duress elements (s.17 and common law)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuressElements {
    /// Threat of death or bodily harm
    pub threat_death_or_harm: bool,
    /// Present threat
    pub present_threat: bool,
    /// No safe avenue of escape
    pub no_escape: bool,
    /// Proportionality
    pub proportional_response: bool,
}

/// Mental disorder defence elements (s.16)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MentalDisorderElements {
    /// Mental disorder at time of act
    pub mental_disorder: bool,
    /// Rendered incapable of appreciating nature/quality
    pub incapable_appreciating: bool,
    /// Or knowing act was wrong
    pub incapable_knowing_wrong: bool,
}

/// Intoxication defence type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntoxicationDefence {
    /// Mild intoxication (no defence)
    Mild,
    /// Advanced intoxication (specific intent only - Daley)
    Advanced,
    /// Extreme intoxication (Daviault - limited)
    Extreme,
}

// ============================================================================
// Sentencing
// ============================================================================

/// Sentence type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SentenceType {
    /// Absolute discharge (s.730)
    AbsoluteDischarge,
    /// Conditional discharge (s.730)
    ConditionalDischarge,
    /// Suspended sentence with probation
    SuspendedSentence,
    /// Fine
    Fine { amount_cents: i64 },
    /// Conditional sentence (house arrest) (s.742.1)
    ConditionalSentence,
    /// Imprisonment
    Imprisonment { months: u32 },
    /// Intermittent sentence (s.732)
    Intermittent { days: u32 },
    /// Life imprisonment
    LifeImprisonment,
    /// Dangerous offender designation (s.753)
    DangerousOffender,
    /// Long-term offender (s.753.1)
    LongTermOffender,
}

/// Sentencing principle (s.718)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SentencingPrinciple {
    /// Denunciation (s.718(a))
    Denunciation,
    /// Deterrence - specific and general (s.718(b))
    Deterrence,
    /// Separation from society (s.718(c))
    Incapacitation,
    /// Rehabilitation (s.718(d))
    Rehabilitation,
    /// Reparations (s.718(e))
    Reparations,
    /// Responsibility and acknowledgment (s.718(f))
    Responsibility,
}

/// Aggravating factor (s.718.2(a))
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AggravatingFactor {
    /// Hate crime (s.718.2(a)(i))
    HateCrime,
    /// Domestic violence
    DomesticViolence,
    /// Abuse of trust
    AbuseOfTrust,
    /// Victim vulnerability
    VulnerableVictim,
    /// Breach of court order
    BreachCourtOrder,
    /// Gang involvement
    CriminalOrganization,
    /// Terrorism motive
    Terrorism,
    /// Prior criminal record
    PriorRecord,
}

/// Mitigating factor
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MitigatingFactor {
    /// First offender
    FirstOffender,
    /// Guilty plea
    GuiltyPlea,
    /// Remorse
    Remorse,
    /// Good character
    GoodCharacter,
    /// Mental health issues
    MentalHealth,
    /// Addiction
    Addiction,
    /// Gladue factors (s.718.2(e))
    GladueFactor,
    /// Youth
    Youth,
    /// Cooperation
    Cooperation,
}

/// Gladue factors for Indigenous offenders (s.718.2(e))
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GladueFactor {
    /// Residential school experience (self or family)
    ResidentialSchool,
    /// Sixties Scoop
    SixtiesScoop,
    /// Foster care/child welfare involvement
    ChildWelfare,
    /// Systemic racism
    SystemicRacism,
    /// Intergenerational trauma
    IntergenerationalTrauma,
    /// Poverty/lack of opportunities
    SocioeconomicDisadvantage,
    /// Loss of culture/language
    CulturalDislocation,
    /// Lack of education
    EducationalDisadvantage,
    /// Family violence
    FamilyViolence,
}

// ============================================================================
// Charter Rights in Criminal Process
// ============================================================================

/// Charter right in criminal context
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CriminalCharterRight {
    /// Right to be secure against unreasonable search (s.8)
    SearchAndSeizure,
    /// Right not to be arbitrarily detained (s.9)
    ArbitraryDetention,
    /// Right on arrest/detention (s.10)
    RightsOnArrest,
    /// Right to counsel (s.10(b))
    RightToCounsel,
    /// Right to trial within reasonable time (s.11(b))
    TrialWithinReasonableTime,
    /// Right to be presumed innocent (s.11(d))
    PresumptionOfInnocence,
    /// Right not to be compelled to testify (s.11(c))
    SelfIncrimination,
    /// Right against double jeopardy (s.11(h))
    DoubleJeopardy,
    /// Right to jury (s.11(f))
    JuryTrial,
    /// Right against cruel punishment (s.12)
    CruelPunishment,
}

/// Section 24 Charter remedy
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CharterRemedy {
    /// Exclusion of evidence (s.24(2))
    ExclusionOfEvidence,
    /// Stay of proceedings
    Stay,
    /// Sentence reduction
    SentenceReduction,
    /// Costs
    Costs,
}

/// Grant analysis for s.24(2) exclusion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantAnalysis {
    /// Seriousness of Charter-infringing conduct
    pub seriousness_of_breach: BreachSeriousness,
    /// Impact on Charter-protected interests
    pub impact_on_accused: ImpactLevel,
    /// Society's interest in adjudication on merits
    pub societal_interest: SocietalInterest,
}

/// Seriousness of Charter breach (Grant factor 1)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreachSeriousness {
    /// Technical/minor breach
    Technical,
    /// Negligent
    Negligent,
    /// Willful/flagrant disregard
    Willful,
    /// Systemic pattern
    Systemic,
}

/// Impact on accused's rights (Grant factor 2)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImpactLevel {
    /// Minimal impact
    Minimal,
    /// Moderate impact
    Moderate,
    /// Significant impact
    Significant,
    /// Severe impact
    Severe,
}

/// Society's interest in adjudication (Grant factor 3)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SocietalInterest {
    /// Reliable evidence, serious offence
    High,
    /// Moderate
    Moderate,
    /// Less reliable evidence, less serious
    Low,
}

// ============================================================================
// Criminal Procedure
// ============================================================================

/// Mode of trial
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModeOfTrial {
    /// Summary conviction in provincial court
    SummaryProvincial,
    /// Indictable in provincial court
    IndictableProvincial,
    /// Superior Court judge alone
    SuperiorJudgeAlone,
    /// Superior Court judge and jury
    SuperiorJury,
}

/// Election for hybrid offences
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CrownElection {
    /// Proceed summarily
    Summary,
    /// Proceed by indictment
    Indictment,
}

/// Accused's election (s.536)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccusedElection {
    /// Provincial court judge without preliminary
    ProvincialCourt,
    /// Superior Court judge alone
    SuperiorJudgeAlone,
    /// Superior Court judge and jury
    SuperiorJury,
    /// Re-election
    ReElection,
}

/// Bail/judicial interim release type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BailType {
    /// Release on undertaking (s.498)
    Undertaking,
    /// Release with conditions (s.499)
    ReleasedWithConditions,
    /// Release with surety (s.515)
    Surety { amount_cents: i64 },
    /// Detention pending trial (s.515)
    Detained,
}

/// Grounds for detention (s.515(10))
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DetentionGround {
    /// Primary - to ensure attendance (s.515(10)(a))
    EnsureAttendance,
    /// Secondary - protection of public (s.515(10)(b))
    PublicProtection,
    /// Tertiary - maintain confidence (s.515(10)(c))
    JusticeConfidence,
}

// ============================================================================
// Key Cases
// ============================================================================

/// Criminal law case
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriminalCase {
    /// Citation
    pub citation: CaseCitation,
    /// Legal principle
    pub principle: String,
    /// Area of criminal law
    pub area: CriminalArea,
}

/// Area of criminal law
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CriminalArea {
    /// Mens rea
    MensRea,
    /// Actus reus
    ActusReus,
    /// Homicide
    Homicide,
    /// Sexual offences
    SexualOffences,
    /// Defences
    Defences,
    /// Sentencing
    Sentencing,
    /// Charter rights
    CharterRights,
    /// Evidence
    Evidence,
}

impl CriminalCase {
    /// R v Woollin \[1999\] - oblique intention
    pub fn woollin() -> Self {
        Self {
            citation: CaseCitation {
                name: "R v Woollin".to_string(),
                year: 1999,
                neutral_citation: Some("[1999] 1 AC 82".to_string()),
                report_citation: Some("[1998] UKHL 28".to_string()),
                court: crate::common::Court::Tribunal {
                    name: "House of Lords (UK - persuasive)".to_string(),
                },
                principle: "Oblique intention - virtual certainty test".to_string(),
            },
            principle: "For murder, oblique intention established where death or GBH \
                was a virtual certainty of defendant's act and defendant appreciated this. \
                Canadian courts follow this approach."
                .to_string(),
            area: CriminalArea::MensRea,
        }
    }

    /// R v Sault Ste. Marie \[1978\] - regulatory offences
    pub fn sault_ste_marie() -> Self {
        Self {
            citation: CaseCitation::scc(
                "R v Sault Ste. Marie (City)",
                1978,
                1299,
                "Three categories of offences",
            ),
            principle: "Three categories: (1) True crimes requiring mens rea, \
                (2) Strict liability with due diligence defence, \
                (3) Absolute liability with no defence."
                .to_string(),
            area: CriminalArea::MensRea,
        }
    }

    /// R v Martineau \[1990\] - murder mens rea
    pub fn martineau() -> Self {
        Self {
            citation: CaseCitation::scc(
                "R v Martineau",
                1990,
                633,
                "Constitutional minimum mens rea for murder",
            ),
            principle: "Murder requires subjective foresight of death (minimum constitutional \
                requirement for murder). Constructive murder provisions struck down."
                .to_string(),
            area: CriminalArea::Homicide,
        }
    }

    /// R v Creighton \[1993\] - manslaughter
    pub fn creighton() -> Self {
        Self {
            citation: CaseCitation::scc(
                "R v Creighton",
                1993,
                3,
                "Objective foreseeability for manslaughter",
            ),
            principle: "Unlawful act manslaughter requires: (1) Unlawful act, \
                (2) Objective foreseeability of bodily harm. No need to foresee death."
                .to_string(),
            area: CriminalArea::Homicide,
        }
    }

    /// R v Daviault \[1994\] - extreme intoxication
    pub fn daviault() -> Self {
        Self {
            citation: CaseCitation::scc("R v Daviault", 1994, 63, "Extreme intoxication defence"),
            principle: "Extreme intoxication akin to automatism may negate mens rea even \
                for general intent offences. Parliament responded with s.33.1."
                .to_string(),
            area: CriminalArea::Defences,
        }
    }

    /// R v Grant \[2009\] - s.24(2) exclusion
    pub fn grant() -> Self {
        Self {
            citation: CaseCitation::scc(
                "R v Grant",
                2009,
                32,
                "New framework for s.24(2) exclusion",
            ),
            principle: "Three factors for s.24(2): (1) Seriousness of Charter-infringing conduct, \
                (2) Impact on accused's Charter-protected interests, \
                (3) Society's interest in adjudication on merits."
                .to_string(),
            area: CriminalArea::CharterRights,
        }
    }

    /// R v Jordan \[2016\] - trial delay
    pub fn jordan() -> Self {
        Self {
            citation: CaseCitation::scc(
                "R v Jordan",
                2016,
                27,
                "Presumptive ceiling for trial delay",
            ),
            principle: "Presumptive ceiling: 18 months (provincial court), 30 months (superior court). \
                Delay exceeding ceiling presumptively unreasonable unless exceptional circumstances."
                .to_string(),
            area: CriminalArea::CharterRights,
        }
    }

    /// R v Gladue \[1999\] - Indigenous sentencing
    pub fn gladue() -> Self {
        Self {
            citation: CaseCitation::scc(
                "R v Gladue",
                1999,
                688,
                "Section 718.2(e) - Indigenous offenders",
            ),
            principle: "Section 718.2(e) requires sentencing judges to consider: \
                (1) Unique systemic background factors affecting Indigenous peoples, \
                (2) Types of sanctions appropriate in the circumstances."
                .to_string(),
            area: CriminalArea::Sentencing,
        }
    }

    /// R v Ipeelee \[2012\] - Gladue factors
    pub fn ipeelee() -> Self {
        Self {
            citation: CaseCitation::scc("R v Ipeelee", 2012, 13, "Gladue applies to all offences"),
            principle: "Gladue applies regardless of seriousness of offence. Background factors \
                may mitigate moral culpability. Courts must have Gladue report."
                .to_string(),
            area: CriminalArea::Sentencing,
        }
    }

    /// R v Perka \[1984\] - necessity
    pub fn perka() -> Self {
        Self {
            citation: CaseCitation::scc(
                "Perka v The Queen",
                1984,
                232,
                "Necessity defence elements",
            ),
            principle: "Necessity requires: (1) Imminent peril or danger, \
                (2) No reasonable legal alternative, (3) Proportionality between harm avoided \
                and harm inflicted."
                .to_string(),
            area: CriminalArea::Defences,
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
    fn test_offence_type() {
        let hybrid = OffenceType::Hybrid;
        assert_eq!(hybrid, OffenceType::Hybrid);
    }

    #[test]
    fn test_mens_rea() {
        let intention = MensRea::Intention;
        let recklessness = MensRea::Recklessness;
        assert_ne!(intention, recklessness);
    }

    #[test]
    fn test_self_defence_elements() {
        let elements = SelfDefenceElements {
            reasonable_belief_threat: true,
            defensive_purpose: true,
            reasonable_response: true,
        };
        assert!(elements.reasonable_belief_threat);
    }

    #[test]
    fn test_sentence_type() {
        let imprisonment = SentenceType::Imprisonment { months: 24 };
        match imprisonment {
            SentenceType::Imprisonment { months } => assert_eq!(months, 24),
            _ => panic!("Expected imprisonment"),
        }
    }

    #[test]
    fn test_grant_case() {
        let case = CriminalCase::grant();
        assert_eq!(case.citation.year, 2009);
        assert_eq!(case.area, CriminalArea::CharterRights);
    }

    #[test]
    fn test_gladue_case() {
        let case = CriminalCase::gladue();
        assert!(case.principle.contains("718.2(e)"));
    }

    #[test]
    fn test_martineau_case() {
        let case = CriminalCase::martineau();
        assert!(case.principle.contains("subjective foresight"));
    }

    #[test]
    fn test_jordan_case() {
        let case = CriminalCase::jordan();
        assert!(case.principle.contains("18 months"));
        assert!(case.principle.contains("30 months"));
    }
}
