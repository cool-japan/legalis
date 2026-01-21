//! UK Criminal Law - Core Types
//!
//! This module provides comprehensive type definitions for UK criminal law,
//! covering offence classification, mens rea, actus reus, sentencing, and defences.
//!
//! # Legal Framework
//!
//! UK criminal law combines common law principles with statutory offences.
//! Key statutes include:
//! - Criminal Justice Act 2003 (sentencing framework)
//! - Theft Act 1968
//! - Offences Against the Person Act 1861
//! - Sexual Offences Act 2003
//! - Fraud Act 2006
//! - Criminal Justice and Immigration Act 2008 (self-defence)
//!
//! # Key Concepts
//!
//! - **Actus Reus**: The physical element of a crime (conduct, circumstances, consequences)
//! - **Mens Rea**: The mental element (intention, recklessness, negligence)
//! - **Causation**: The link between conduct and result
//! - **Defences**: Factors that negate criminal liability

// Allow missing docs on enum variant struct fields - these are self-documenting in context
#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

// ============================================================================
// Offence Classification
// ============================================================================

/// Classification of criminal offences by mode of trial
///
/// # UK Classification System
///
/// Criminal offences in England and Wales are classified by how they may be tried:
/// - Summary offences: Magistrates' Court only
/// - Indictable-only offences: Crown Court only
/// - Either-way offences: Can be tried in either court
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OffenceClassification {
    /// Summary offences - tried in Magistrates' Court
    ///
    /// Maximum sentence typically 6 months (or 12 months for consecutive sentences)
    /// Examples: Common assault, minor theft, most driving offences
    Summary,

    /// Indictable-only offences - tried in Crown Court
    ///
    /// Most serious offences including murder, rape, robbery
    /// Trial by jury with judge
    IndictableOnly,

    /// Either-way offences - can be tried in either court
    ///
    /// Mode of trial determined by allocation hearing
    /// Examples: Theft, ABH, burglary (non-dwelling)
    EitherWay,
}

/// Category of criminal offence by nature
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OffenceCategory {
    /// Offences against the person (homicide, assault, sexual offences)
    AgainstPerson,
    /// Property offences (theft, burglary, criminal damage)
    Property,
    /// Fraud and dishonesty offences
    FraudDishonesty,
    /// Drug offences (Misuse of Drugs Act 1971)
    Drugs,
    /// Public order offences
    PublicOrder,
    /// Driving and road traffic offences
    RoadTraffic,
    /// Terrorism offences (Terrorism Act 2000/2006)
    Terrorism,
    /// Regulatory and corporate offences
    Regulatory,
    /// Sexual offences (Sexual Offences Act 2003)
    Sexual,
    /// Weapons offences
    Weapons,
    /// Computer/cyber crimes (Computer Misuse Act 1990)
    Cyber,
}

/// Severity level for sentencing purposes
///
/// Based on Sentencing Council guidelines
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub enum OffenceSeverity {
    /// Minor offences - typically fines or discharges
    Minor,
    /// Less serious offences - community orders or short custody
    LessSerious,
    /// Moderately serious - medium custody ranges
    #[default]
    Moderate,
    /// Serious offences - longer custody
    Serious,
    /// Very serious - substantial custody
    VerySerious,
    /// Most serious - life imprisonment possible
    MostSerious,
}

/// Representation of a criminal offence
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Offence {
    /// Name of the offence
    pub name: String,
    /// Statutory source (if statutory offence)
    pub statutory_source: Option<StatutorySource>,
    /// Common law offence (if applicable)
    pub common_law: bool,
    /// Classification by mode of trial
    pub classification: OffenceClassification,
    /// Category of offence
    pub category: OffenceCategory,
    /// Severity level
    pub severity: OffenceSeverity,
    /// Maximum sentence available
    pub maximum_sentence: MaximumSentence,
    /// Required mens rea elements
    pub mens_rea_requirements: Vec<MensReaType>,
    /// Required actus reus elements
    pub actus_reus_elements: Vec<ActusReusElement>,
}

/// Statutory source of an offence
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatutorySource {
    /// Act name (e.g., "Theft Act 1968")
    pub act: String,
    /// Section number
    pub section: String,
    /// Subsection if applicable
    pub subsection: Option<String>,
}

/// Maximum sentence for an offence
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MaximumSentence {
    /// Life imprisonment
    Life,
    /// Whole life order (minimum term not set)
    WholeLife,
    /// Fixed maximum in years
    Years(u32),
    /// Fixed maximum in months
    Months(u32),
    /// Fine only (no custody)
    FineOnly { unlimited: bool },
    /// Summary maximum (6 months or level fine)
    SummaryMaximum,
}

// ============================================================================
// Mens Rea (Mental Element)
// ============================================================================

/// Types of mens rea (mental element) in criminal law
///
/// # Hierarchy
///
/// From highest to lowest culpability:
/// 1. Direct Intention - purpose to bring about result
/// 2. Oblique Intention - virtual certainty (R v Woollin \[1999\])
/// 3. Subjective Recklessness - conscious risk-taking (R v Cunningham \[1957\])
/// 4. Objective Recklessness - obvious risk (now largely abandoned)
/// 5. Negligence - failure to meet reasonable standard
/// 6. Strict Liability - no mental element required
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MensReaType {
    /// Direct intention - D's purpose or aim
    ///
    /// D acts with the purpose of bringing about the prohibited consequence
    /// Highest level of culpability
    DirectIntention,

    /// Oblique intention - virtual certainty test
    ///
    /// From R v Woollin \[1999\] AC 82:
    /// Jury may FIND intention where D foresaw consequence as virtually certain
    /// Note: "may find" not "must find" - remains jury question
    ObliqueIntention,

    /// Subjective recklessness - conscious risk-taking
    ///
    /// From R v Cunningham \[1957\] 2 QB 396:
    /// D is aware of an unjustifiable risk and takes it anyway
    /// Standard for most non-fatal offences
    SubjectiveRecklessness,

    /// Knowledge - awareness of circumstances
    ///
    /// D knows or believes relevant circumstances exist
    /// Often required for handling stolen goods, etc.
    Knowledge,

    /// Belief - subjective state
    ///
    /// D believes something to be true
    /// May be honest but unreasonable (DPP v Morgan \[1976\])
    Belief,

    /// Dishonesty (Ivey v Genting Casinos \[2017\])
    ///
    /// Two-stage test:
    /// 1. What was D's actual state of knowledge/belief?
    /// 2. Was that conduct dishonest by ordinary standards?
    Dishonesty,

    /// Negligence - failure to meet standard
    ///
    /// Objective standard - what reasonable person would do
    /// Required for gross negligence manslaughter (R v Adomako \[1995\])
    Negligence,

    /// Gross negligence - seriously below standard
    ///
    /// For gross negligence manslaughter - conduct so bad
    /// that it should be considered criminal
    GrossNegligence,

    /// Strict liability - no mental element
    ///
    /// Liability attaches regardless of D's mental state
    /// Usually regulatory offences (Gammon criteria)
    StrictLiability,

    /// Wilful blindness - deliberate ignorance
    ///
    /// D deliberately shuts eyes to obvious truth
    /// Treated as equivalent to knowledge
    WilfulBlindness,

    /// Transferred malice
    ///
    /// From R v Latimer (1886):
    /// Intent transfers to actual victim if of same type
    /// Murder of B when D intended to kill A
    TransferredMalice,
}

/// Analysis of mens rea
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MensReaAnalysis {
    /// Type of mens rea identified
    pub mens_rea_type: MensReaType,
    /// Whether mens rea is established
    pub established: bool,
    /// Evidence supporting mens rea
    pub evidence: Vec<String>,
    /// Reasoning for conclusion
    pub reasoning: String,
    /// Relevant case law
    pub case_law: Vec<CaseCitation>,
}

/// Direct intention analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectIntentionFacts {
    /// What was D's stated purpose?
    pub stated_purpose: Option<String>,
    /// Actions indicating purpose
    pub purposive_actions: Vec<String>,
    /// Preparation or planning evidence
    pub preparation_evidence: Vec<String>,
    /// Post-offence conduct indicating intent
    pub post_offence_conduct: Vec<String>,
}

/// Oblique intention facts (Woollin direction)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObliqueIntentionFacts {
    /// Was consequence virtually certain to result?
    pub virtually_certain: bool,
    /// Did D appreciate this virtual certainty?
    pub defendant_appreciation: bool,
    /// Evidence of foresight
    pub foresight_evidence: Vec<String>,
}

/// Recklessness analysis (Cunningham test)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecklessnessAnalysis {
    /// Risk that D was aware of
    pub risk_identified: String,
    /// Was D aware of the risk?
    pub awareness_established: bool,
    /// Was risk-taking unjustifiable?
    pub unjustifiable: bool,
    /// Evidence of awareness
    pub awareness_evidence: Vec<String>,
}

/// Dishonesty analysis (Ivey v Genting Casinos \[2017\])
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DishonestyAnalysis {
    /// D's actual state of knowledge/belief
    pub defendants_knowledge: String,
    /// Was conduct dishonest by ordinary standards?
    pub dishonest_by_ordinary_standards: bool,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Actus Reus (Physical Element)
// ============================================================================

/// Elements of actus reus (physical element)
///
/// # Components
///
/// Actus reus may consist of:
/// - Conduct (an act or omission)
/// - Circumstances (relevant surrounding facts)
/// - Consequences (results of conduct)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActusReusElement {
    /// Voluntary positive act
    Act(ActType),
    /// Failure to act where duty exists
    Omission(OmissionDuty),
    /// Relevant circumstances
    Circumstances(String),
    /// Prohibited consequence/result
    Consequence(String),
    /// State of affairs (rare)
    StateOfAffairs,
}

/// Types of acts
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActType {
    /// Voluntary bodily movement
    VoluntaryAct,
    /// Reflex/involuntary (not actus reus)
    InvoluntaryAct,
    /// Under external compulsion
    CompelledAct,
    /// Verbal conduct (words as acts)
    VerbalConduct,
    /// Possession (continuing act)
    Possession,
}

/// Source of duty for omission liability
///
/// # Duty Sources
///
/// Criminal liability for omission requires a duty to act from:
/// - Statute
/// - Contract
/// - Relationship
/// - Voluntary assumption of care
/// - Creation of dangerous situation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OmissionDuty {
    /// Statutory duty (e.g., Road Traffic Act)
    Statutory { act: String, section: String },
    /// Contractual duty (R v Pittwood)
    Contract { relationship: String },
    /// Family/special relationship (R v Gibbins & Proctor)
    Relationship { relationship_type: String },
    /// Voluntary assumption of care (R v Stone & Dobinson)
    VoluntaryAssumption { description: String },
    /// Creation of dangerous situation (R v Miller)
    DangerousSituation { situation: String },
    /// Public office duty
    PublicOffice { office: String },
}

/// Voluntariness analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoluntarinessAnalysis {
    /// Was the act voluntary?
    pub voluntary: bool,
    /// If not voluntary, what was the cause?
    pub involuntary_cause: Option<InvoluntaryCause>,
    /// Evidence of voluntariness
    pub evidence: Vec<String>,
}

/// Causes of involuntary conduct
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvoluntaryCause {
    /// Reflex action
    Reflex,
    /// Muscle spasm
    MuscleSpasm,
    /// Physical compulsion by another
    PhysicalCompulsion,
    /// Automatism (see defences)
    Automatism,
    /// Sleepwalking
    Sleepwalking,
    /// Hypoglycaemia
    Hypoglycaemia,
    /// Concussion
    Concussion,
}

// ============================================================================
// Causation
// ============================================================================

/// Causation analysis for result crimes
///
/// # Two-Stage Test
///
/// For result crimes (e.g., murder, manslaughter):
/// 1. Factual causation - "but for" test
/// 2. Legal causation - D's act must be "more than minimal" cause
///
/// # Breaking the Chain
///
/// Intervening acts may break the causal chain (novus actus interveniens)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CausationAnalysis {
    /// Factual causation established?
    pub factual_causation: FactualCausation,
    /// Legal causation established?
    pub legal_causation: LegalCausation,
    /// Any intervening acts?
    pub intervening_acts: Vec<InterveningAct>,
    /// Overall causation established?
    pub causation_established: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Factual causation ("but for" test)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FactualCausation {
    /// Was D's conduct a factual cause?
    pub established: bool,
    /// "But for" analysis
    pub but_for_analysis: String,
}

/// Legal causation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LegalCausation {
    /// Was D's act a legal cause (more than minimal)?
    pub established: bool,
    /// Was D's act an "operating and substantial" cause?
    pub operating_and_substantial: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Types of intervening acts (novus actus interveniens)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterveningAct {
    /// Description of the intervening act
    pub description: String,
    /// Type of intervening act
    pub act_type: InterveningActType,
    /// Does it break the chain of causation?
    pub breaks_chain: bool,
    /// Analysis
    pub analysis: String,
}

/// Types of intervening acts
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterveningActType {
    /// Act of the victim
    ///
    /// Generally doesn't break chain unless "daft" or unexpected
    /// R v Roberts (1971), R v Williams (1992)
    VictimAct,

    /// Act of third party
    ///
    /// May break chain if free, deliberate, informed
    /// R v Pagett (1983)
    ThirdPartyAct,

    /// Medical treatment
    ///
    /// Only breaks chain if "palpably wrong" treatment
    /// R v Cheshire \[1991\], R v Jordan (1956)
    MedicalTreatment,

    /// Natural event
    NaturalEvent,

    /// Pre-existing condition of victim ("thin skull" rule)
    ///
    /// D takes victim as found - R v Blaue \[1975\]
    PreExistingCondition,
}

/// Thin skull rule (egg-shell skull)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThinSkullAnalysis {
    /// Pre-existing condition
    pub condition: String,
    /// Did condition contribute to harm?
    pub contributed_to_harm: bool,
    /// D still liable? (Usually yes per R v Blaue)
    pub defendant_liable: bool,
    /// Analysis
    pub analysis: String,
}

// ============================================================================
// Parties to Crime
// ============================================================================

/// Role of party in criminal offence
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartyRole {
    /// Principal offender - commits actus reus
    Principal,
    /// Joint principal - commits actus reus together
    JointPrincipal,
    /// Secondary party - aids, abets, counsels, or procures
    SecondaryParty(SecondaryParticipation),
    /// Innocent agent - used by another
    InnocentAgent,
}

/// Types of secondary participation
///
/// From s.8 Accessories and Abettors Act 1861
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecondaryParticipation {
    /// Aiding - helping or assisting
    Aiding,
    /// Abetting - encouraging at the scene
    Abetting,
    /// Counselling - encouraging before offence
    Counselling,
    /// Procuring - bringing about the offence
    Procuring,
}

/// Joint enterprise analysis
///
/// # Post-Jogee Analysis
///
/// R v Jogee \[2016\] UKSC 8 abolished "parasitic accessory liability"
/// Secondary party now requires intention to assist/encourage AND
/// intention that P commit offence (or conditional intent)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JointEnterpriseAnalysis {
    /// Common purpose established?
    pub common_purpose: bool,
    /// Description of common purpose
    pub purpose_description: String,
    /// Did secondary party intend to assist/encourage?
    pub intended_assistance: bool,
    /// Did secondary party intend P to commit offence?
    pub intended_offence: bool,
    /// Departure from common purpose?
    pub departure: Option<DepartureAnalysis>,
    /// Withdrawal analysis
    pub withdrawal: Option<WithdrawalAnalysis>,
}

/// Departure from common purpose
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DepartureAnalysis {
    /// How did P depart from plan?
    pub departure_description: String,
    /// Was departure fundamental?
    pub fundamental: bool,
    /// Is secondary party liable for departure?
    pub secondary_liable: bool,
}

/// Withdrawal from joint enterprise
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WithdrawalAnalysis {
    /// Did D attempt to withdraw?
    pub withdrawal_attempted: bool,
    /// Was withdrawal communicated?
    pub communicated: bool,
    /// Were reasonable steps taken to prevent?
    pub reasonable_steps: bool,
    /// Was withdrawal effective?
    pub effective: bool,
}

// ============================================================================
// Defences - Overview Types
// ============================================================================

/// Categories of criminal defence
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DefenceCategory {
    /// Complete defences - full acquittal
    Complete,
    /// Partial defences - reduce liability (e.g., murder to manslaughter)
    Partial,
    /// Procedural defences (e.g., abuse of process)
    Procedural,
}

/// Types of criminal defences
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DefenceType {
    // General defences (complete)
    /// Self-defence / defence of another
    SelfDefence,
    /// Prevention of crime (s.3 Criminal Law Act 1967)
    PreventionOfCrime,
    /// Duress by threats
    DuressByThreats,
    /// Duress of circumstances
    DuressCircumstances,
    /// Necessity
    Necessity,
    /// Consent
    Consent,
    /// Intoxication (in limited circumstances)
    Intoxication,
    /// Mistake
    Mistake,
    /// Automatism
    Automatism,
    /// Insanity (M'Naghten Rules)
    Insanity,
    /// Lawful excuse / authority
    LawfulAuthority,

    // Partial defences to murder
    /// Loss of control (Coroners and Justice Act 2009)
    LossOfControl,
    /// Diminished responsibility (s.2 Homicide Act 1957)
    DiminishedResponsibility,
    /// Suicide pact (s.4 Homicide Act 1957)
    SuicidePact,

    // Specific defences
    /// Marital coercion (historical, now limited)
    MaritalCoercion,
    /// Claim of right (for theft offences)
    ClaimOfRight,
}

/// Result of defence analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DefenceResult {
    /// Defence type considered
    pub defence_type: DefenceType,
    /// Is defence available?
    pub available: bool,
    /// If available, what is the effect?
    pub effect: Option<DefenceEffect>,
    /// Key findings
    pub findings: Vec<String>,
    /// Relevant case law
    pub case_law: Vec<CaseCitation>,
}

/// Effect of successful defence
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DefenceEffect {
    /// Complete acquittal
    Acquittal,
    /// Conviction for lesser offence
    LesserOffence { offence: String },
    /// Special verdict (insanity)
    SpecialVerdict,
    /// Mitigation only
    Mitigation,
}

// ============================================================================
// Sentencing
// ============================================================================

/// Types of sentence available
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SentenceType {
    /// Immediate custody
    Custody(CustodialSentence),
    /// Suspended sentence order
    Suspended(SuspendedSentence),
    /// Community order
    CommunityOrder(CommunityOrder),
    /// Fine
    Fine(FineDetails),
    /// Discharge
    Discharge(DischargeType),
    /// Compensation order
    Compensation { amount: f64 },
    /// Hospital order (Mental Health Act 1983)
    HospitalOrder { with_restrictions: bool },
    /// Life sentence
    LifeSentence(LifeSentenceType),
}

/// Custodial sentence details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustodialSentence {
    /// Length in months
    pub length_months: u32,
    /// Category of custody
    pub custody_type: CustodyType,
    /// Time spent on remand to count
    pub remand_time_months: Option<u32>,
    /// Extended sentence?
    pub extended: Option<ExtendedSentence>,
}

/// Types of custody
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CustodyType {
    /// Standard determinate sentence
    Determinate,
    /// Young offender institution
    YoungOffenderInstitution,
    /// Detention and training order (under 18)
    DetentionTrainingOrder,
}

/// Extended sentence for dangerous offenders
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtendedSentence {
    /// Custodial term in months
    pub custodial_term_months: u32,
    /// Extended licence period in months
    pub extension_period_months: u32,
}

/// Suspended sentence details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuspendedSentence {
    /// Operational period in months
    pub operational_period_months: u32,
    /// Supervision period in months
    pub supervision_period_months: u32,
    /// Requirements attached
    pub requirements: Vec<CommunityRequirement>,
}

/// Community order details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommunityOrder {
    /// Length in months
    pub length_months: u32,
    /// Requirements
    pub requirements: Vec<CommunityRequirement>,
}

/// Community order requirements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CommunityRequirement {
    /// Unpaid work (40-300 hours)
    UnpaidWork { hours: u32 },
    /// Curfew requirement
    Curfew {
        hours_per_day: u32,
        period_months: u32,
    },
    /// Programme requirement
    Programme { programme_name: String },
    /// Supervision requirement
    Supervision { period_months: u32 },
    /// Drug rehabilitation
    DrugRehabilitation { period_months: u32 },
    /// Alcohol treatment
    AlcoholTreatment { period_months: u32 },
    /// Mental health treatment
    MentalHealthTreatment { period_months: u32 },
    /// Residence requirement
    Residence { location: String },
    /// Activity requirement
    Activity { days: u32 },
    /// Exclusion requirement
    Exclusion { area: String, period_months: u32 },
    /// Electronic monitoring
    ElectronicMonitoring,
    /// Attendance centre (under 25)
    AttendanceCentre { hours: u32 },
}

/// Fine details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FineDetails {
    /// Fine amount
    pub amount: f64,
    /// Time to pay in days
    pub time_to_pay_days: Option<u32>,
    /// Compensation order attached?
    pub with_compensation: Option<f64>,
}

/// Types of discharge
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DischargeType {
    /// Absolute discharge - no penalty
    Absolute,
    /// Conditional discharge
    Conditional { period_months: u32 },
}

/// Life sentence types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LifeSentenceType {
    /// Mandatory life (murder)
    Mandatory { minimum_term_years: u32 },
    /// Discretionary life
    Discretionary { minimum_term_years: u32 },
    /// Whole life order
    WholeLife,
    /// Life for second listed offence (s.122 LASPO 2012)
    TwoStrike { minimum_term_years: u32 },
}

// ============================================================================
// Sentencing Guidelines
// ============================================================================

/// Sentencing Council guidelines structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SentencingGuideline {
    /// Offence this guideline applies to
    pub offence: String,
    /// Culpability factors
    pub culpability: CulpabilityAssessment,
    /// Harm factors
    pub harm: HarmAssessment,
    /// Starting point and range for category
    pub starting_point: SentenceRange,
    /// Aggravating factors
    pub aggravating_factors: Vec<AggravatingFactor>,
    /// Mitigating factors
    pub mitigating_factors: Vec<MitigatingFactor>,
}

/// Culpability assessment
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CulpabilityAssessment {
    /// Category (A = highest, C/D = lowest)
    pub category: CulpabilityCategory,
    /// Factors present
    pub factors: Vec<String>,
}

/// Culpability category
///
/// Note: Ord is implemented so that A > B > C > D (higher culpability ranks higher)
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CulpabilityCategory {
    /// Highest culpability
    A,
    /// High culpability
    B,
    /// Medium culpability
    #[default]
    C,
    /// Lower culpability
    D,
}

impl PartialOrd for CulpabilityCategory {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CulpabilityCategory {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse ordering: A > B > C > D (higher culpability = greater)
        let self_rank = match self {
            Self::A => 3,
            Self::B => 2,
            Self::C => 1,
            Self::D => 0,
        };
        let other_rank = match other {
            Self::A => 3,
            Self::B => 2,
            Self::C => 1,
            Self::D => 0,
        };
        self_rank.cmp(&other_rank)
    }
}

/// Harm assessment
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HarmAssessment {
    /// Category (1 = highest harm, 3/4 = lowest)
    pub category: HarmCategory,
    /// Factors present
    pub factors: Vec<String>,
}

/// Harm category
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub enum HarmCategory {
    /// Highest harm
    Category1,
    /// Significant harm
    #[default]
    Category2,
    /// Lesser harm
    Category3,
    /// Minimal harm (where applicable)
    Category4,
}

/// Sentence range
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SentenceRange {
    /// Starting point
    pub starting_point: String,
    /// Range minimum
    pub range_min: String,
    /// Range maximum
    pub range_max: String,
}

/// Aggravating factor in sentencing
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AggravatingFactor {
    /// Previous convictions
    PreviousConvictions,
    /// Offence committed on bail
    OnBail,
    /// Vulnerable victim
    VulnerableVictim,
    /// Abuse of position of trust
    AbuseOfTrust,
    /// Offence motivated by hostility (hate crime)
    HateCrime { protected_characteristic: String },
    /// Planning/premeditation
    Premeditation,
    /// Group offending
    GroupOffending,
    /// Use of weapon
    UseOfWeapon,
    /// Location of offence (e.g., victim's home)
    Location { description: String },
    /// Timing (e.g., at night)
    Timing { description: String },
    /// Commission while under influence
    UnderInfluence,
    /// Attempts to conceal evidence
    ConcealEvidence,
    /// Failure to respond to warnings
    FailureToRespond,
    /// High value/significant harm
    SignificantHarm,
    /// Other statutory aggravating factor
    StatutoryAggravating { description: String },
    /// Other aggravating factor
    Other { description: String },
}

/// Mitigating factor in sentencing
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MitigatingFactor {
    /// No previous convictions or no relevant/recent
    NoPreviousConvictions,
    /// Good character
    GoodCharacter,
    /// Remorse
    Remorse,
    /// Self-reported to authorities
    SelfReported,
    /// Co-operation with investigation
    Cooperation,
    /// Age and/or lack of maturity
    AgeOrMaturity,
    /// Mental disorder or learning disability
    MentalDisorder,
    /// Physical disability or illness
    PhysicalDisability,
    /// Determination to address addiction
    AddressingAddiction,
    /// Sole or primary carer
    CarerResponsibilities,
    /// Serious medical condition
    SeriousMedicalCondition,
    /// Isolated incident
    IsolatedIncident,
    /// Delay in proceedings not attributable to D
    Delay,
    /// Provocation (where not defence)
    Provocation,
    /// Activity lawful in other jurisdiction
    LawfulElsewhere,
    /// Other mitigating factor
    Other { description: String },
}

/// Guilty plea reduction (Sentencing Council guideline)
///
/// Maximum reduction: 1/3 for plea at first reasonable opportunity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GuiltyPleaReduction {
    /// Stage of proceedings when plea entered
    pub plea_stage: PleaStage,
    /// Reduction fraction (e.g., 1/3, 1/4, 1/10)
    pub reduction_fraction: String,
    /// Calculated reduction
    pub reduction_description: String,
}

/// Stage of proceedings for guilty plea
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PleaStage {
    /// First reasonable opportunity
    #[default]
    FirstOpportunity,
    /// After first opportunity but before trial
    BeforeTrial,
    /// Day of trial or during trial
    DuringTrial,
}

// ============================================================================
// Case Law Citations
// ============================================================================

/// Case citation for reference
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaseCitation {
    /// Case name (e.g., "R v Woollin")
    pub name: String,
    /// Year
    pub year: u32,
    /// Report citation (e.g., "AC 82")
    pub citation: String,
    /// Principle established
    pub principle: String,
}

impl CaseCitation {
    /// Create a new case citation
    pub fn new(
        name: impl Into<String>,
        year: u32,
        citation: impl Into<String>,
        principle: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            year,
            citation: citation.into(),
            principle: principle.into(),
        }
    }
}

// ============================================================================
// Criminal Procedure Types
// ============================================================================

/// Stages of criminal proceedings
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CriminalStage {
    /// Investigation stage
    Investigation,
    /// Arrest
    Arrest,
    /// Detention at police station
    PoliceDetention,
    /// Charge
    Charge,
    /// First appearance at court
    FirstAppearance,
    /// Allocation/sending (for either-way/indictable)
    Allocation,
    /// Pre-trial hearings
    PreTrial,
    /// Trial
    Trial,
    /// Verdict
    Verdict,
    /// Sentencing
    Sentencing,
    /// Appeal
    Appeal,
}

/// Verdict options
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Verdict {
    /// Guilty as charged
    Guilty,
    /// Not guilty
    NotGuilty,
    /// Guilty of alternative/lesser offence
    GuiltyOfLesserOffence { offence: String },
    /// Special verdict - not guilty by reason of insanity
    SpecialVerdict,
    /// Jury unable to agree (hung jury)
    HungJury,
}

/// Appeal routes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppealRoute {
    /// Magistrates' to Crown (rehearing)
    MagistratesToCrown,
    /// Magistrates' to High Court (case stated)
    MagistratesToHighCourt,
    /// Crown to Court of Appeal (Criminal Division)
    CrownToCourtOfAppeal,
    /// Court of Appeal to Supreme Court
    CourtOfAppealToSupremeCourt,
}

// ============================================================================
// Builder Pattern for Complex Types
// ============================================================================

/// Builder for Offence
#[derive(Debug, Clone, Default)]
pub struct OffenceBuilder {
    name: Option<String>,
    statutory_source: Option<StatutorySource>,
    common_law: bool,
    classification: Option<OffenceClassification>,
    category: Option<OffenceCategory>,
    severity: OffenceSeverity,
    maximum_sentence: Option<MaximumSentence>,
    mens_rea_requirements: Vec<MensReaType>,
    actus_reus_elements: Vec<ActusReusElement>,
}

impl OffenceBuilder {
    /// Create new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set offence name
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set statutory source
    pub fn statutory_source(mut self, act: impl Into<String>, section: impl Into<String>) -> Self {
        self.statutory_source = Some(StatutorySource {
            act: act.into(),
            section: section.into(),
            subsection: None,
        });
        self
    }

    /// Set as common law offence
    pub fn common_law(mut self) -> Self {
        self.common_law = true;
        self
    }

    /// Set classification
    pub fn classification(mut self, classification: OffenceClassification) -> Self {
        self.classification = Some(classification);
        self
    }

    /// Set category
    pub fn category(mut self, category: OffenceCategory) -> Self {
        self.category = Some(category);
        self
    }

    /// Set severity
    pub fn severity(mut self, severity: OffenceSeverity) -> Self {
        self.severity = severity;
        self
    }

    /// Set maximum sentence
    pub fn maximum_sentence(mut self, max: MaximumSentence) -> Self {
        self.maximum_sentence = Some(max);
        self
    }

    /// Add mens rea requirement
    pub fn mens_rea(mut self, mens_rea: MensReaType) -> Self {
        self.mens_rea_requirements.push(mens_rea);
        self
    }

    /// Add actus reus element
    pub fn actus_reus(mut self, element: ActusReusElement) -> Self {
        self.actus_reus_elements.push(element);
        self
    }

    /// Build the offence
    ///
    /// # Errors
    ///
    /// Returns error if required fields are missing
    pub fn build(self) -> Result<Offence, String> {
        Ok(Offence {
            name: self.name.ok_or("Offence name is required")?,
            statutory_source: self.statutory_source,
            common_law: self.common_law,
            classification: self.classification.ok_or("Classification is required")?,
            category: self.category.ok_or("Category is required")?,
            severity: self.severity,
            maximum_sentence: self
                .maximum_sentence
                .ok_or("Maximum sentence is required")?,
            mens_rea_requirements: self.mens_rea_requirements,
            actus_reus_elements: self.actus_reus_elements,
        })
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offence_builder() {
        let theft = OffenceBuilder::new()
            .name("Theft")
            .statutory_source("Theft Act 1968", "s.1")
            .classification(OffenceClassification::EitherWay)
            .category(OffenceCategory::Property)
            .severity(OffenceSeverity::Moderate)
            .maximum_sentence(MaximumSentence::Years(7))
            .mens_rea(MensReaType::Dishonesty)
            .mens_rea(MensReaType::DirectIntention)
            .actus_reus(ActusReusElement::Act(ActType::VoluntaryAct))
            .build();

        assert!(theft.is_ok());
        let theft = theft.expect("should build");
        assert_eq!(theft.name, "Theft");
        assert_eq!(theft.mens_rea_requirements.len(), 2);
    }

    #[test]
    fn test_case_citation() {
        let woollin = CaseCitation::new(
            "R v Woollin",
            1999,
            "AC 82",
            "Virtual certainty test for oblique intention",
        );

        assert_eq!(woollin.year, 1999);
        assert!(woollin.principle.contains("oblique intention"));
    }

    #[test]
    fn test_mens_rea_types() {
        let types = vec![
            MensReaType::DirectIntention,
            MensReaType::ObliqueIntention,
            MensReaType::SubjectiveRecklessness,
            MensReaType::GrossNegligence,
        ];

        // All should serialize correctly
        for t in types {
            let json = serde_json::to_string(&t);
            assert!(json.is_ok());
        }
    }

    #[test]
    fn test_culpability_ordering() {
        assert!(CulpabilityCategory::A > CulpabilityCategory::B);
        assert!(CulpabilityCategory::B > CulpabilityCategory::C);
        assert!(CulpabilityCategory::C > CulpabilityCategory::D);
    }

    #[test]
    fn test_sentence_types() {
        let custodial = SentenceType::Custody(CustodialSentence {
            length_months: 24,
            custody_type: CustodyType::Determinate,
            remand_time_months: Some(3),
            extended: None,
        });

        let community = SentenceType::CommunityOrder(CommunityOrder {
            length_months: 12,
            requirements: vec![
                CommunityRequirement::UnpaidWork { hours: 200 },
                CommunityRequirement::Supervision { period_months: 12 },
            ],
        });

        // Both should serialize
        assert!(serde_json::to_string(&custodial).is_ok());
        assert!(serde_json::to_string(&community).is_ok());
    }

    #[test]
    fn test_intervening_act_types() {
        let medical = InterveningAct {
            description: "Negligent medical treatment".into(),
            act_type: InterveningActType::MedicalTreatment,
            breaks_chain: false,
            analysis: "Per R v Cheshire, only palpably wrong treatment breaks chain".into(),
        };

        assert!(!medical.breaks_chain);
    }

    #[test]
    fn test_guilty_plea_reduction() {
        let first_opportunity = GuiltyPleaReduction {
            plea_stage: PleaStage::FirstOpportunity,
            reduction_fraction: "1/3".into(),
            reduction_description: "Maximum reduction for early plea".into(),
        };

        assert_eq!(first_opportunity.reduction_fraction, "1/3");
    }

    #[test]
    fn test_defence_effects() {
        let acquittal = DefenceEffect::Acquittal;
        let lesser = DefenceEffect::LesserOffence {
            offence: "Manslaughter".into(),
        };

        assert!(matches!(acquittal, DefenceEffect::Acquittal));
        assert!(matches!(lesser, DefenceEffect::LesserOffence { .. }));
    }
}
