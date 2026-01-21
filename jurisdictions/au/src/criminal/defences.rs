//! Criminal Defences
//!
//! Implementation of criminal defences under Australian law including:
//! - Self-defence (Zecevic v DPP (Vic) (1987))
//! - Duress and necessity
//! - Mental impairment and fitness to plead
//! - Intoxication
//! - Mistake of fact
//! - Automatism
//!
//! ## Key Legislation
//!
//! - Criminal Code Act 1995 (Cth) Part 2.3
//! - State criminal codes and Crimes Acts
//!
//! ## Key Cases
//!
//! - Zecevic v DPP (Vic) (1987) 162 CLR 645 - Self-defence test
//! - R v Falconer (1990) 171 CLR 30 - Mental impairment
//! - R v O'Connor (1980) 146 CLR 64 - Intoxication
//! - R v Loughnan (1981) VR 443 - Duress

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

// =============================================================================
// Self-Defence (Criminal Code Part 2.3 Div 10)
// =============================================================================

/// Self-defence elements under Zecevic v DPP test
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelfDefenceClaim {
    /// Accused's belief about the threat
    pub belief: ThreatBelief,
    /// Nature of the threat
    pub threat_nature: ThreatNature,
    /// Response taken
    pub response: DefensiveResponse,
    /// Proportionality assessment
    pub proportionality: ProportionalityAssessment,
}

/// Accused's belief about the threat
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThreatBelief {
    /// Believed unlawful violence was threatened
    pub believed_unlawful_violence: bool,
    /// Believed response was necessary
    pub believed_response_necessary: bool,
    /// Source of belief
    pub belief_source: BeliefSource,
    /// Whether belief was based on reasonable grounds
    pub reasonable_grounds: bool,
}

/// Source of belief about threat
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BeliefSource {
    /// Direct threat by victim
    DirectThreat,
    /// Victim's conduct
    VictimConduct,
    /// Third party information
    ThirdPartyInformation,
    /// Prior history with victim
    PriorHistory,
    /// Circumstances suggesting threat
    Circumstances,
}

/// Nature of the threat
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThreatNature {
    /// Type of threat
    pub threat_type: ThreatType,
    /// Immediacy
    pub immediacy: ThreatImminence,
    /// Severity
    pub severity: ThreatSeverity,
    /// Whether threat could be avoided
    pub avoidable: bool,
}

/// Type of threat
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThreatType {
    /// Threat to life
    ThreatToLife,
    /// Threat of serious harm
    ThreatOfSeriousHarm,
    /// Threat of unlawful imprisonment
    UnlawfulImprisonment,
    /// Sexual assault threat
    SexualAssault,
    /// Property threat (limited defence)
    PropertyThreat,
}

/// Threat imminence
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThreatImminence {
    /// Immediate threat
    Immediate,
    /// Imminent (about to happen)
    Imminent,
    /// Future threat
    Future,
    /// Ongoing threat (domestic violence context)
    Ongoing,
}

/// Threat severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThreatSeverity {
    /// Minor harm
    Minor,
    /// Serious bodily harm
    SeriousBodily,
    /// Grievous bodily harm
    Grievous,
    /// Death
    Death,
}

/// Defensive response
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DefensiveResponse {
    /// Type of response
    pub response_type: ResponseType,
    /// Force used
    pub force_level: ForceLevel,
    /// Weapon used
    pub weapon_used: Option<WeaponType>,
    /// Harm caused
    pub harm_caused: HarmCaused,
}

/// Type of defensive response
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResponseType {
    /// Physical force
    PhysicalForce,
    /// Use of object
    UseOfObject,
    /// Use of weapon
    UseOfWeapon,
    /// Defensive positioning
    DefensivePositioning,
}

/// Level of force
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ForceLevel {
    /// Minimal force
    Minimal,
    /// Moderate force
    Moderate,
    /// Significant force
    Significant,
    /// Lethal force
    Lethal,
}

/// Weapon type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeaponType {
    /// Firearm
    Firearm,
    /// Knife or bladed weapon
    Knife,
    /// Blunt object
    BluntObject,
    /// Vehicle
    Vehicle,
    /// Other weapon
    Other,
}

/// Harm caused by defensive action
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HarmCaused {
    /// No harm
    None,
    /// Minor injuries
    Minor,
    /// Serious injuries
    Serious,
    /// Grievous bodily harm
    Grievous,
    /// Death
    Death,
}

/// Proportionality assessment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProportionalityAssessment {
    /// Response proportionate to threat
    pub proportionate: bool,
    /// Factors considered
    pub factors: Vec<ProportionalityFactor>,
    /// Whether excessive force was used
    pub excessive_force: bool,
}

/// Factor in proportionality assessment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProportionalityFactor {
    /// Size disparity
    SizeDisparity,
    /// Multiple attackers
    MultipleAttackers,
    /// Weapon involved
    WeaponInvolved,
    /// No means of retreat
    NoRetreat,
    /// Domestic violence history
    DomesticViolenceHistory,
    /// Physical vulnerability
    PhysicalVulnerability,
}

// =============================================================================
// Duress (Criminal Code Part 2.3 Div 12)
// =============================================================================

/// Duress defence claim
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DuressClaim {
    /// Threat made
    pub threat: DuressThrea,
    /// Reasonable belief about threat
    pub reasonable_belief: bool,
    /// No reasonable way to escape
    pub no_escape: bool,
    /// Response proportionate
    pub response_proportionate: bool,
    /// Offence not murder (murder excluded at common law)
    pub offence_type: DuressOffenceType,
}

/// Threat for duress
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DuressThrea {
    /// Threat of death
    pub threat_of_death: bool,
    /// Threat of serious harm
    pub threat_of_serious_harm: bool,
    /// Threat to self
    pub threat_to_self: bool,
    /// Threat to family/loved ones
    pub threat_to_others: bool,
    /// Threatener identified
    pub threatener: String,
    /// Immediacy of threat
    pub immediacy: ThreatImminence,
}

/// Offence type for duress availability
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DuressOffenceType {
    /// Murder (duress generally not available)
    Murder,
    /// Attempted murder
    AttemptedMurder,
    /// Other offence (duress may be available)
    OtherOffence,
}

// =============================================================================
// Necessity (Criminal Code Part 2.3 Div 13)
// =============================================================================

/// Necessity defence claim
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NecessityClaim {
    /// Emergency situation
    pub emergency: EmergencySituation,
    /// No reasonable legal alternative
    pub no_legal_alternative: bool,
    /// Response proportionate to harm avoided
    pub proportionate: bool,
    /// Danger not caused by accused
    pub danger_not_self_created: bool,
}

/// Emergency situation for necessity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmergencySituation {
    /// Type of emergency
    pub emergency_type: EmergencyType,
    /// Immediacy
    pub immediate: bool,
    /// Harm threatened
    pub threatened_harm: ThreatSeverity,
    /// Person at risk
    pub person_at_risk: PersonAtRisk,
}

/// Type of emergency
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EmergencyType {
    /// Medical emergency
    Medical,
    /// Natural disaster
    NaturalDisaster,
    /// Fire
    Fire,
    /// Immediate danger to life
    DangerToLife,
    /// Prevention of serious harm
    PreventionOfHarm,
}

/// Person at risk
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PersonAtRisk {
    /// Accused themselves
    Accused,
    /// Family member
    Family,
    /// Third party
    ThirdParty,
    /// Multiple people
    Multiple,
}

// =============================================================================
// Mental Impairment (Criminal Code Part 2.3 Div 8)
// =============================================================================

/// Mental impairment defence
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MentalImpairmentDefence {
    /// Mental condition
    pub condition: MentalCondition,
    /// Impairment at time of offence
    pub impairment_at_time: bool,
    /// Nature of impairment
    pub impairment_nature: ImpairmentNature,
    /// M'Naghten test elements
    pub m_naghten_elements: MNaghtenElements,
    /// Expert evidence
    pub expert_evidence: Option<ExpertEvidence>,
}

/// Mental condition type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MentalCondition {
    /// Psychotic illness (schizophrenia, etc.)
    PsychoticIllness { diagnosis: String },
    /// Severe depression
    SevereDepression,
    /// Intellectual disability
    IntellectualDisability { severity: DisabilitySeverity },
    /// Organic brain disorder
    OrganicBrainDisorder { cause: String },
    /// Dissociative disorder
    DissociativeDisorder,
    /// Other mental illness
    Other { description: String },
}

/// Severity of intellectual disability
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DisabilitySeverity {
    /// Mild
    Mild,
    /// Moderate
    Moderate,
    /// Severe
    Severe,
    /// Profound
    Profound,
}

/// Nature of impairment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImpairmentNature {
    /// Did not know nature of conduct
    DidNotKnowNature,
    /// Did not know conduct was wrong
    DidNotKnowWrongfulness,
    /// Could not control conduct
    CouldNotControl,
    /// Delusional belief
    DelusionalBelief,
}

/// M'Naghten test elements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MNaghtenElements {
    /// Disease of the mind
    pub disease_of_mind: bool,
    /// Defect of reason from disease
    pub defect_of_reason: bool,
    /// Did not know nature and quality of act
    pub did_not_know_nature: bool,
    /// Did not know act was wrong
    pub did_not_know_wrong: bool,
}

/// Expert psychiatric evidence
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExpertEvidence {
    /// Expert name
    pub expert_name: String,
    /// Qualifications
    pub qualifications: String,
    /// Examination date
    pub examination_date: NaiveDate,
    /// Diagnosis
    pub diagnosis: String,
    /// Opinion on impairment
    pub impairment_opinion: ImpairmentOpinion,
}

/// Expert opinion on impairment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImpairmentOpinion {
    /// Significant impairment
    SignificantImpairment,
    /// Partial impairment
    PartialImpairment,
    /// Minimal impairment
    MinimalImpairment,
    /// No impairment from condition
    NoImpairment,
}

// =============================================================================
// Fitness to Plead
// =============================================================================

/// Fitness to plead assessment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FitnessToPlead {
    /// Pressley test criteria
    pub pressley_criteria: PressleyCriteria,
    /// Current mental state
    pub current_mental_state: CurrentMentalState,
    /// Can accused be made fit
    pub can_be_made_fit: bool,
    /// Recommended disposition
    pub recommended_disposition: UnfitnessDisposition,
}

/// Pressley test criteria for fitness
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PressleyCriteria {
    /// Understands nature of charge
    pub understands_charge: bool,
    /// Can plead to charge
    pub can_plead: bool,
    /// Understands nature of proceedings
    pub understands_proceedings: bool,
    /// Can follow course of proceedings
    pub can_follow_proceedings: bool,
    /// Can understand evidence
    pub understands_evidence: bool,
    /// Can give instructions to counsel
    pub can_instruct_counsel: bool,
}

/// Current mental state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CurrentMentalState {
    /// Fit to plead
    FitToPlead,
    /// Unfit to plead
    UnfitToPlead,
    /// Temporarily unfit
    TemporarilyUnfit,
    /// Fitness uncertain
    Uncertain,
}

/// Disposition for unfitness
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnfitnessDisposition {
    /// Adjourn for treatment
    AdjournForTreatment,
    /// Special hearing
    SpecialHearing,
    /// Dismissal/discharge
    Dismissal,
    /// Supervision order
    SupervisionOrder,
    /// Detention in hospital
    HospitalDetention,
}

// =============================================================================
// Intoxication (Criminal Code Part 2.3 Div 9)
// =============================================================================

/// Intoxication defence
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntoxicationDefence {
    /// Type of intoxication
    pub intoxication_type: IntoxicationType,
    /// Voluntariness
    pub voluntary: bool,
    /// Substance involved
    pub substance: Substance,
    /// Effect on mental state
    pub effect: IntoxicationEffect,
    /// Applicable offences
    pub applicable_to: IntoxicationApplicability,
}

/// Type of intoxication
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IntoxicationType {
    /// Self-induced (voluntary)
    SelfInduced,
    /// Involuntary (spiked drink, etc.)
    Involuntary,
    /// Prescribed medication
    PrescribedMedication,
    /// Pathological intoxication
    Pathological,
}

/// Substance causing intoxication
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Substance {
    /// Alcohol
    Alcohol,
    /// Cannabis
    Cannabis,
    /// Amphetamines
    Amphetamines,
    /// Opioids
    Opioids,
    /// Prescription drugs
    PrescriptionDrugs { medication: String },
    /// Other substance
    Other { name: String },
}

/// Effect of intoxication
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntoxicationEffect {
    /// Level of intoxication
    pub level: IntoxicationLevel,
    /// Affected intention
    pub affected_intention: bool,
    /// Caused automatism
    pub caused_automatism: bool,
    /// Relevant blood alcohol/drug level
    pub substance_level: Option<f64>,
}

/// Level of intoxication
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IntoxicationLevel {
    /// Mild
    Mild,
    /// Moderate
    Moderate,
    /// Severe
    Severe,
    /// Extreme
    Extreme,
}

/// Applicability of intoxication defence
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IntoxicationApplicability {
    /// Applies to specific intent offences only
    SpecificIntentOnly,
    /// Does not apply (basic intent)
    DoesNotApply,
    /// Full defence (involuntary intoxication)
    FullDefence,
}

// =============================================================================
// Mistake of Fact (Criminal Code Part 2.3 Div 9)
// =============================================================================

/// Mistake of fact defence
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MistakeOfFactDefence {
    /// Mistaken belief
    pub belief: MistakenBelief,
    /// Reasonableness of belief
    pub reasonable: bool,
    /// If true, would negate offence element
    pub negates_element: bool,
    /// Applicable fault element
    pub applicable_to: MistakeFaultElement,
}

/// Mistaken belief
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MistakenBelief {
    /// What accused believed
    pub believed_fact: String,
    /// Actual fact
    pub actual_fact: String,
    /// Basis for belief
    pub basis: BeliefBasis,
}

/// Basis for mistaken belief
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BeliefBasis {
    /// Information from another person
    Information,
    /// Accused's own observation
    Observation,
    /// Reasonable inference
    Inference,
    /// Prior experience
    PriorExperience,
}

/// Fault element mistake applies to
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MistakeFaultElement {
    /// Intention
    Intention,
    /// Knowledge
    Knowledge,
    /// Recklessness
    Recklessness,
    /// Negligence (doesn't apply)
    Negligence,
}

// =============================================================================
// Automatism
// =============================================================================

/// Automatism defence
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutomatismDefence {
    /// Type of automatism
    pub automatism_type: AutomatismType,
    /// Cause
    pub cause: AutomatismCause,
    /// Involuntary conduct
    pub involuntary: bool,
    /// Evidence supporting
    pub evidence: AutomatismEvidence,
}

/// Type of automatism
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AutomatismType {
    /// Sane automatism (complete defence)
    Sane,
    /// Insane automatism (leads to NCR verdict)
    Insane,
}

/// Cause of automatism
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AutomatismCause {
    /// Sleepwalking
    Sleepwalking,
    /// Hypoglycemia
    Hypoglycemia,
    /// Epileptic seizure (usually insane)
    Epilepsy,
    /// Concussion
    Concussion,
    /// Dissociative state
    DissociativeState,
    /// Extreme external shock
    ExternalShock,
    /// Reflex action
    ReflexAction,
    /// Other cause
    Other { description: String },
}

/// Evidence of automatism
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutomatismEvidence {
    /// Medical evidence
    pub medical_evidence: bool,
    /// Witness observations
    pub witness_evidence: bool,
    /// History of condition
    pub history_of_condition: bool,
    /// Expert opinion
    pub expert_opinion: Option<String>,
}

// =============================================================================
// Defence Assessment
// =============================================================================

/// Result of defence assessment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DefenceAssessment {
    /// Defence type
    pub defence_type: DefenceType,
    /// Is defence available
    pub available: bool,
    /// Likelihood of success
    pub likelihood: DefenceLikelihood,
    /// Issues with defence
    pub issues: Vec<String>,
    /// Legal references
    pub legal_references: Vec<String>,
    /// Recommendation
    pub recommendation: DefenceRecommendation,
}

/// Type of defence
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DefenceType {
    /// Self-defence
    SelfDefence,
    /// Duress
    Duress,
    /// Necessity
    Necessity,
    /// Mental impairment
    MentalImpairment,
    /// Intoxication
    Intoxication,
    /// Mistake of fact
    MistakeOfFact,
    /// Automatism
    Automatism,
    /// Provocation (partial defence)
    Provocation,
    /// Diminished responsibility (partial defence)
    DiminishedResponsibility,
}

/// Likelihood of defence success
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DefenceLikelihood {
    /// Very likely to succeed
    VeryLikely,
    /// Likely to succeed
    Likely,
    /// Uncertain
    Uncertain,
    /// Unlikely to succeed
    Unlikely,
    /// Very unlikely to succeed
    VeryUnlikely,
}

/// Defence recommendation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DefenceRecommendation {
    /// Run defence at trial
    RunDefence,
    /// Consider as alternative
    ConsiderAsAlternative,
    /// Obtain expert evidence
    ObtainExpertEvidence,
    /// Do not recommend
    DoNotRecommend,
}

// =============================================================================
// Assessment Functions
// =============================================================================

/// Assess self-defence claim
pub fn assess_self_defence(claim: &SelfDefenceClaim) -> DefenceAssessment {
    let mut issues = Vec::new();
    let mut legal_references = vec![
        "Criminal Code Act 1995 (Cth) s.10.4".to_string(),
        "Zecevic v DPP (Vic) (1987) 162 CLR 645".to_string(),
    ];

    // Check belief elements
    if !claim.belief.believed_unlawful_violence {
        issues.push("No belief in unlawful violence".to_string());
    }
    if !claim.belief.believed_response_necessary {
        issues.push("No belief response was necessary".to_string());
    }

    // Check threat
    if !matches!(
        claim.threat_nature.immediacy,
        ThreatImminence::Immediate | ThreatImminence::Imminent | ThreatImminence::Ongoing
    ) {
        issues.push("Threat was not immediate or imminent".to_string());
        legal_references.push("Future threats generally insufficient".to_string());
    }

    // Check proportionality
    if !claim.proportionality.proportionate {
        issues.push("Response was not proportionate".to_string());
    }
    if claim.proportionality.excessive_force {
        issues.push("Excessive force may vitiate defence".to_string());
    }

    // Special consideration for domestic violence
    if claim
        .proportionality
        .factors
        .contains(&ProportionalityFactor::DomesticViolenceHistory)
    {
        legal_references.push("Social context/battered woman syndrome may be relevant".to_string());
    }

    let available = issues.len() <= 1;
    let likelihood = if issues.is_empty() {
        DefenceLikelihood::VeryLikely
    } else if issues.len() == 1 {
        DefenceLikelihood::Likely
    } else if issues.len() == 2 {
        DefenceLikelihood::Uncertain
    } else {
        DefenceLikelihood::Unlikely
    };

    DefenceAssessment {
        defence_type: DefenceType::SelfDefence,
        available,
        likelihood,
        issues,
        legal_references,
        recommendation: if available {
            DefenceRecommendation::RunDefence
        } else {
            DefenceRecommendation::ConsiderAsAlternative
        },
    }
}

/// Assess duress defence
pub fn assess_duress(claim: &DuressClaim) -> DefenceAssessment {
    let mut issues = Vec::new();
    let legal_references = vec![
        "Criminal Code Act 1995 (Cth) s.10.2".to_string(),
        "R v Loughnan (1981) VR 443".to_string(),
    ];

    // Check threat
    if !claim.threat.threat_of_death && !claim.threat.threat_of_serious_harm {
        issues.push("Threat must be of death or serious harm".to_string());
    }

    if !claim.reasonable_belief {
        issues.push("Belief in threat must be reasonable".to_string());
    }

    if !claim.no_escape {
        issues.push("Must have no reasonable way to escape threat".to_string());
    }

    if !claim.response_proportionate {
        issues.push("Response must be proportionate to threat".to_string());
    }

    // Murder exception
    if matches!(
        claim.offence_type,
        DuressOffenceType::Murder | DuressOffenceType::AttemptedMurder
    ) {
        issues.push("Duress is not available for murder at common law".to_string());
    }

    let available = issues.is_empty();
    let likelihood = if available {
        DefenceLikelihood::Likely
    } else if issues.len() == 1 {
        DefenceLikelihood::Uncertain
    } else {
        DefenceLikelihood::Unlikely
    };

    DefenceAssessment {
        defence_type: DefenceType::Duress,
        available,
        likelihood,
        issues,
        legal_references,
        recommendation: if available {
            DefenceRecommendation::RunDefence
        } else {
            DefenceRecommendation::DoNotRecommend
        },
    }
}

/// Assess mental impairment defence
pub fn assess_mental_impairment(defence: &MentalImpairmentDefence) -> DefenceAssessment {
    let mut issues = Vec::new();
    let mut legal_references = vec![
        "Criminal Code Act 1995 (Cth) s.7.3".to_string(),
        "R v Falconer (1990) 171 CLR 30".to_string(),
    ];

    // Check M'Naghten elements
    if !defence.m_naghten_elements.disease_of_mind {
        issues.push("Disease of the mind not established".to_string());
    }
    if !defence.m_naghten_elements.defect_of_reason {
        issues.push("Defect of reason not established".to_string());
    }
    if !defence.m_naghten_elements.did_not_know_nature
        && !defence.m_naghten_elements.did_not_know_wrong
    {
        issues.push("Must show did not know nature of act OR that it was wrong".to_string());
    }

    // Check impairment at time
    if !defence.impairment_at_time {
        issues.push("Impairment must exist at time of offence".to_string());
    }

    // Expert evidence
    if defence.expert_evidence.is_none() {
        issues.push("Expert psychiatric evidence recommended".to_string());
        legal_references.push("Expert evidence typically required".to_string());
    } else if let Some(ref expert) = defence.expert_evidence
        && matches!(
            expert.impairment_opinion,
            ImpairmentOpinion::MinimalImpairment | ImpairmentOpinion::NoImpairment
        )
    {
        issues.push("Expert opinion does not support significant impairment".to_string());
    }

    let available = defence.m_naghten_elements.disease_of_mind
        && defence.m_naghten_elements.defect_of_reason
        && (defence.m_naghten_elements.did_not_know_nature
            || defence.m_naghten_elements.did_not_know_wrong);

    let likelihood = if available && defence.expert_evidence.is_some() {
        DefenceLikelihood::Likely
    } else if available {
        DefenceLikelihood::Uncertain
    } else {
        DefenceLikelihood::Unlikely
    };

    DefenceAssessment {
        defence_type: DefenceType::MentalImpairment,
        available,
        likelihood,
        issues,
        legal_references,
        recommendation: if defence.expert_evidence.is_none() {
            DefenceRecommendation::ObtainExpertEvidence
        } else if available {
            DefenceRecommendation::RunDefence
        } else {
            DefenceRecommendation::DoNotRecommend
        },
    }
}

/// Assess intoxication defence
pub fn assess_intoxication(defence: &IntoxicationDefence) -> DefenceAssessment {
    let mut issues = Vec::new();
    let mut legal_references = vec![
        "Criminal Code Act 1995 (Cth) s.8.1-8.5".to_string(),
        "R v O'Connor (1980) 146 CLR 64".to_string(),
    ];

    // Voluntary intoxication has limited application
    if defence.voluntary {
        if !matches!(
            defence.applicable_to,
            IntoxicationApplicability::SpecificIntentOnly
        ) {
            issues.push(
                "Voluntary intoxication only available for specific intent offences".to_string(),
            );
        }
        legal_references
            .push("Voluntary intoxication is not defence to basic intent offences".to_string());
    }

    // Involuntary intoxication is full defence
    if !defence.voluntary && !defence.effect.affected_intention && !defence.effect.caused_automatism
    {
        issues
            .push("Involuntary intoxication must affect intention or cause automatism".to_string());
    }

    // Check level
    if matches!(defence.effect.level, IntoxicationLevel::Mild) {
        issues.push("Mild intoxication unlikely to support defence".to_string());
    }

    let available = if defence.voluntary {
        matches!(
            defence.applicable_to,
            IntoxicationApplicability::SpecificIntentOnly
        ) && matches!(
            defence.effect.level,
            IntoxicationLevel::Severe | IntoxicationLevel::Extreme
        )
    } else {
        defence.effect.affected_intention || defence.effect.caused_automatism
    };

    let likelihood = if !defence.voluntary && available {
        DefenceLikelihood::Likely
    } else if available {
        DefenceLikelihood::Uncertain
    } else {
        DefenceLikelihood::VeryUnlikely
    };

    DefenceAssessment {
        defence_type: DefenceType::Intoxication,
        available,
        likelihood,
        issues,
        legal_references,
        recommendation: if available && !defence.voluntary {
            DefenceRecommendation::RunDefence
        } else if available {
            DefenceRecommendation::ConsiderAsAlternative
        } else {
            DefenceRecommendation::DoNotRecommend
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self_defence_valid() {
        let claim = SelfDefenceClaim {
            belief: ThreatBelief {
                believed_unlawful_violence: true,
                believed_response_necessary: true,
                belief_source: BeliefSource::DirectThreat,
                reasonable_grounds: true,
            },
            threat_nature: ThreatNature {
                threat_type: ThreatType::ThreatOfSeriousHarm,
                immediacy: ThreatImminence::Immediate,
                severity: ThreatSeverity::SeriousBodily,
                avoidable: false,
            },
            response: DefensiveResponse {
                response_type: ResponseType::PhysicalForce,
                force_level: ForceLevel::Moderate,
                weapon_used: None,
                harm_caused: HarmCaused::Minor,
            },
            proportionality: ProportionalityAssessment {
                proportionate: true,
                factors: vec![ProportionalityFactor::NoRetreat],
                excessive_force: false,
            },
        };

        let result = assess_self_defence(&claim);
        assert!(result.available);
        assert!(matches!(
            result.likelihood,
            DefenceLikelihood::VeryLikely | DefenceLikelihood::Likely
        ));
    }

    #[test]
    fn test_self_defence_excessive_force() {
        let claim = SelfDefenceClaim {
            belief: ThreatBelief {
                believed_unlawful_violence: true,
                believed_response_necessary: true,
                belief_source: BeliefSource::DirectThreat,
                reasonable_grounds: true,
            },
            threat_nature: ThreatNature {
                threat_type: ThreatType::ThreatOfSeriousHarm,
                immediacy: ThreatImminence::Immediate,
                severity: ThreatSeverity::Minor,
                avoidable: false,
            },
            response: DefensiveResponse {
                response_type: ResponseType::UseOfWeapon,
                force_level: ForceLevel::Lethal,
                weapon_used: Some(WeaponType::Firearm),
                harm_caused: HarmCaused::Death,
            },
            proportionality: ProportionalityAssessment {
                proportionate: false,
                factors: vec![],
                excessive_force: true,
            },
        };

        let result = assess_self_defence(&claim);
        assert!(!result.issues.is_empty());
        assert!(result.issues.iter().any(|i| i.contains("proportionate")));
    }

    #[test]
    fn test_duress_murder_excluded() {
        let claim = DuressClaim {
            threat: DuressThrea {
                threat_of_death: true,
                threat_of_serious_harm: true,
                threat_to_self: true,
                threat_to_others: false,
                threatener: "Criminal gang".to_string(),
                immediacy: ThreatImminence::Immediate,
            },
            reasonable_belief: true,
            no_escape: true,
            response_proportionate: true,
            offence_type: DuressOffenceType::Murder,
        };

        let result = assess_duress(&claim);
        assert!(!result.available);
        assert!(result.issues.iter().any(|i| i.contains("murder")));
    }

    #[test]
    fn test_mental_impairment_valid() {
        let defence = MentalImpairmentDefence {
            condition: MentalCondition::PsychoticIllness {
                diagnosis: "Schizophrenia".to_string(),
            },
            impairment_at_time: true,
            impairment_nature: ImpairmentNature::DidNotKnowWrongfulness,
            m_naghten_elements: MNaghtenElements {
                disease_of_mind: true,
                defect_of_reason: true,
                did_not_know_nature: false,
                did_not_know_wrong: true,
            },
            expert_evidence: Some(ExpertEvidence {
                expert_name: "Dr Smith".to_string(),
                qualifications: "Forensic Psychiatrist".to_string(),
                examination_date: NaiveDate::from_ymd_opt(2026, 1, 1).expect("valid date"),
                diagnosis: "Paranoid Schizophrenia".to_string(),
                impairment_opinion: ImpairmentOpinion::SignificantImpairment,
            }),
        };

        let result = assess_mental_impairment(&defence);
        assert!(result.available);
        assert!(matches!(result.likelihood, DefenceLikelihood::Likely));
    }

    #[test]
    fn test_intoxication_voluntary_basic_intent() {
        let defence = IntoxicationDefence {
            intoxication_type: IntoxicationType::SelfInduced,
            voluntary: true,
            substance: Substance::Alcohol,
            effect: IntoxicationEffect {
                level: IntoxicationLevel::Severe,
                affected_intention: true,
                caused_automatism: false,
                substance_level: Some(0.25),
            },
            applicable_to: IntoxicationApplicability::DoesNotApply,
        };

        let result = assess_intoxication(&defence);
        assert!(!result.available);
        assert!(result.issues.iter().any(|i| i.contains("specific intent")));
    }

    #[test]
    fn test_intoxication_involuntary() {
        let defence = IntoxicationDefence {
            intoxication_type: IntoxicationType::Involuntary,
            voluntary: false,
            substance: Substance::Other {
                name: "Unknown drug in drink".to_string(),
            },
            effect: IntoxicationEffect {
                level: IntoxicationLevel::Extreme,
                affected_intention: true,
                caused_automatism: true,
                substance_level: None,
            },
            applicable_to: IntoxicationApplicability::FullDefence,
        };

        let result = assess_intoxication(&defence);
        assert!(result.available);
        assert!(matches!(result.likelihood, DefenceLikelihood::Likely));
    }

    #[test]
    fn test_fitness_to_plead_criteria() {
        let fitness = FitnessToPlead {
            pressley_criteria: PressleyCriteria {
                understands_charge: true,
                can_plead: true,
                understands_proceedings: true,
                can_follow_proceedings: true,
                understands_evidence: true,
                can_instruct_counsel: true,
            },
            current_mental_state: CurrentMentalState::FitToPlead,
            can_be_made_fit: true,
            recommended_disposition: UnfitnessDisposition::AdjournForTreatment,
        };

        assert!(matches!(
            fitness.current_mental_state,
            CurrentMentalState::FitToPlead
        ));
        assert!(fitness.pressley_criteria.can_instruct_counsel);
    }
}
