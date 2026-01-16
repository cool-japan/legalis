//! UK Tort Law - Core Types
//!
//! This module defines the core types for UK tort law under:
//! - Common law negligence (Donoghue v Stevenson, Caparo v Dickman)
//! - Occupiers' Liability Act 1957
//! - Occupiers' Liability Act 1984
//! - Defamation Act 2013
//! - Consumer Protection Act 1987
//! - Animals Act 1971
//! - Torts (Interference with Goods) Act 1977

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

// ============================================================================
// General Tort Types
// ============================================================================

/// Type of tort
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TortType {
    /// Negligence (duty, breach, causation, damage)
    Negligence,
    /// Occupiers' liability under OLA 1957/1984
    OccupiersLiability,
    /// Private nuisance (interference with land)
    PrivateNuisance,
    /// Public nuisance (criminal and tortious)
    PublicNuisance,
    /// Rylands v Fletcher strict liability
    RylandsVFletcher,
    /// Defamation (libel/slander)
    Defamation,
    /// Economic torts (inducing breach, conspiracy, etc.)
    EconomicTort,
    /// Product liability under CPA 1987
    ProductLiability,
    /// Breach of statutory duty
    BreachOfStatutoryDuty,
    /// Trespass to person
    TrespassToPerson,
    /// Trespass to land
    TrespassToLand,
    /// Trespass to goods
    TrespassToGoods,
}

/// Party in a tort claim
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TortParty {
    /// Name of the party
    pub name: String,
    /// Role in the tort
    pub role: PartyRole,
    /// Type of party
    pub party_type: PartyType,
    /// Is the party a vulnerable person?
    pub vulnerable: bool,
    /// Professional capacity (if relevant)
    pub professional_capacity: Option<ProfessionalCapacity>,
}

/// Role of a party in tort proceedings
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartyRole {
    /// Claimant (person who suffered damage)
    Claimant,
    /// Defendant (person alleged to have committed tort)
    Defendant,
    /// Third party (joined to proceedings)
    ThirdParty,
    /// Part 20 defendant
    Part20Defendant,
}

/// Type of party
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartyType {
    /// Natural person
    Individual,
    /// Company registered under Companies Act 2006
    Company,
    /// Limited liability partnership
    LLP,
    /// Partnership
    Partnership,
    /// Public authority (subject to HRA 1998)
    PublicAuthority,
    /// Crown body
    Crown,
    /// Charity
    Charity,
    /// Unincorporated association
    UnincorporatedAssociation,
}

/// Professional capacity of a tortfeasor
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProfessionalCapacity {
    /// Medical professional (doctor, nurse, etc.)
    Medical,
    /// Legal professional (solicitor, barrister)
    Legal,
    /// Accountant or auditor
    Accountant,
    /// Surveyor or valuer
    Surveyor,
    /// Architect
    Architect,
    /// Financial adviser
    FinancialAdviser,
    /// Construction professional
    Construction,
    /// Other professional
    Other(String),
}

// ============================================================================
// Negligence Types
// ============================================================================

/// Duty of care analysis under Caparo v Dickman [1990] 2 AC 605
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DutyOfCareAnalysis {
    /// Was harm reasonably foreseeable?
    pub foreseeability: Foreseeability,
    /// Proximity between claimant and defendant
    pub proximity: Proximity,
    /// Is it fair, just and reasonable to impose a duty?
    pub fair_just_reasonable: FairJustReasonable,
    /// Established duty category (if any)
    pub established_category: Option<EstablishedDutyCategory>,
    /// Novel duty claim?
    pub novel_claim: bool,
    /// Result of duty analysis
    pub duty_exists: bool,
    /// Reasoning for conclusion
    pub reasoning: String,
}

/// Foreseeability analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Foreseeability {
    /// Was the type of harm foreseeable?
    pub harm_foreseeable: bool,
    /// Was the claimant foreseeable (type of person)?
    pub claimant_foreseeable: bool,
    /// Was the manner of harm foreseeable?
    pub manner_foreseeable: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Proximity analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Proximity {
    /// Type of proximity
    pub proximity_type: ProximityType,
    /// Degree of proximity (1-10 scale)
    pub degree: u8,
    /// Physical proximity exists?
    pub physical_proximity: bool,
    /// Circumstantial proximity exists?
    pub circumstantial_proximity: bool,
    /// Causal proximity exists?
    pub causal_proximity: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Type of proximity relationship
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProximityType {
    /// Direct physical proximity
    Physical,
    /// Assumed responsibility (Hedley Byrne)
    AssumedResponsibility,
    /// Employer-employee
    Employment,
    /// Professional-client
    ProfessionalClient,
    /// Manufacturer-consumer
    ManufacturerConsumer,
    /// Road user relationship
    RoadUser,
    /// Rescuer relationship
    Rescuer,
    /// Neighbor relationship
    Neighbor,
    /// Special relationship
    Special,
    /// None established
    None,
}

/// Fair, just and reasonable policy analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FairJustReasonable {
    /// Is it fair to impose duty?
    pub fair: bool,
    /// Is it just to impose duty?
    pub just: bool,
    /// Is it reasonable to impose duty?
    pub reasonable: bool,
    /// Policy considerations
    pub policy_considerations: Vec<PolicyConsideration>,
    /// Overall assessment
    pub overall: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Policy considerations in duty analysis
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyConsideration {
    /// Floodgates concern (indeterminate liability)
    Floodgates,
    /// Defensive practices concern
    DefensivePractices,
    /// Alternative remedies available
    AlternativeRemedies,
    /// Statutory scheme exists
    StatutoryScheme,
    /// Insurance considerations
    Insurance,
    /// Public interest in activity
    PublicInterest,
    /// Deterrence value
    Deterrence,
    /// Loss distribution
    LossDistribution,
    /// Moral blameworthiness
    MoralBlame,
    /// Constitutional/separation of powers
    ConstitutionalConcerns,
}

/// Established duty of care categories
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EstablishedDutyCategory {
    /// Road users to other road users
    RoadUsers,
    /// Employer to employee
    EmployerEmployee,
    /// Manufacturer to consumer (Donoghue v Stevenson)
    ManufacturerConsumer,
    /// Occupier to visitor/trespasser
    OccupierVisitor,
    /// Doctor to patient
    DoctorPatient,
    /// Professional to client
    ProfessionalClient,
    /// School to pupil
    SchoolPupil,
    /// Prison authority to prisoner
    PrisonPrisoner,
    /// Parent to child
    ParentChild,
    /// Rescuer duty
    Rescuer,
}

/// Standard of care analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StandardOfCare {
    /// Type of standard applied
    pub standard_type: StandardType,
    /// Reasonable person test
    pub reasonable_person: ReasonablePersonTest,
    /// Special skill factors (Bolam)
    pub special_skill: Option<BolamTest>,
    /// Child defendant?
    pub child_defendant: Option<ChildStandard>,
    /// Resulting standard description
    pub standard_description: String,
}

/// Type of standard of care
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StandardType {
    /// Ordinary reasonable person
    ReasonablePerson,
    /// Reasonable professional (Bolam)
    ReasonableProfessional,
    /// Child standard (Mullin v Richards)
    Child,
    /// Learner/trainee standard
    Learner,
    /// Enhanced due to disability awareness
    EnhancedDisability,
}

/// Reasonable person test factors
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReasonablePersonTest {
    /// Magnitude of risk (Bolton v Stone)
    pub magnitude_of_risk: RiskLevel,
    /// Gravity of harm
    pub gravity_of_harm: HarmGravity,
    /// Cost of precautions
    pub cost_of_precautions: CostLevel,
    /// Social utility of activity
    pub social_utility: SocialUtility,
    /// Common practice followed?
    pub common_practice: Option<CommonPractice>,
    /// Factors considered
    pub factors: Vec<BreachFactor>,
}

/// Risk level assessment
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    /// Minimal/trivial risk
    Minimal,
    /// Low risk
    Low,
    /// Moderate risk
    Moderate,
    /// High risk
    High,
    /// Very high risk
    VeryHigh,
}

/// Gravity of potential harm
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HarmGravity {
    /// Minor harm
    Minor,
    /// Moderate harm
    Moderate,
    /// Serious harm
    Serious,
    /// Severe/catastrophic harm
    Severe,
    /// Fatal
    Fatal,
}

/// Cost of taking precautions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CostLevel {
    /// Negligible cost
    Negligible,
    /// Low cost
    Low,
    /// Moderate cost
    Moderate,
    /// High cost
    High,
    /// Prohibitive cost
    Prohibitive,
}

/// Social utility of defendant's activity
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SocialUtility {
    /// No social value
    None,
    /// Low social value
    Low,
    /// Moderate social value
    Moderate,
    /// High social value (emergency services, etc.)
    High,
    /// Essential service
    Essential,
}

/// Common practice analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommonPractice {
    /// Was common practice followed?
    pub followed: bool,
    /// Is common practice itself negligent? (Re Herald of Free Enterprise)
    pub practice_negligent: bool,
    /// Description of common practice
    pub description: String,
}

/// Factors in breach assessment
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreachFactor {
    /// Knowledge of risk at time
    KnowledgeAtTime,
    /// Available precautions
    AvailablePrecautions,
    /// Emergency situation (Watt v Hertfordshire)
    Emergency,
    /// Vulnerable claimant (Paris v Stepney)
    VulnerableClaimant,
    /// Experience of defendant
    Experience,
    /// Resources available
    Resources,
}

/// Bolam test for professionals
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BolamTest {
    /// Profession being assessed
    pub profession: String,
    /// Did defendant act in accordance with a practice?
    pub followed_practice: bool,
    /// Is the practice accepted by responsible body?
    pub responsible_body_accepts: bool,
    /// Bolitho override - does practice withstand logical analysis?
    pub bolitho_logical: bool,
    /// Specialist or general practitioner?
    pub specialist: bool,
    /// Result of Bolam/Bolitho test
    pub meets_standard: bool,
}

/// Child standard of care
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChildStandard {
    /// Age of child defendant
    pub age: u8,
    /// Activity engaged in
    pub activity: String,
    /// Was activity an adult activity?
    pub adult_activity: bool,
    /// Standard to apply
    pub applicable_standard: String,
}

/// Breach of duty analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreachOfDuty {
    /// Standard of care owed
    pub standard: StandardOfCare,
    /// Conduct of defendant
    pub defendant_conduct: String,
    /// Did conduct fall below standard?
    pub fell_below_standard: bool,
    /// Evidence of breach
    pub evidence: Vec<BreachEvidence>,
    /// Res ipsa loquitur applies?
    pub res_ipsa_loquitur: Option<ResIpsaLoquitur>,
    /// Reasoning
    pub reasoning: String,
}

/// Evidence of breach
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreachEvidence {
    /// Type of evidence
    pub evidence_type: EvidenceType,
    /// Description
    pub description: String,
    /// Strength of evidence
    pub strength: EvidenceStrength,
}

/// Type of evidence in tort claims
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceType {
    /// Expert evidence
    Expert,
    /// Witness testimony
    Witness,
    /// Documentary evidence
    Documentary,
    /// Physical evidence
    Physical,
    /// Medical records
    MedicalRecords,
    /// Photographs/video
    Visual,
    /// Contemporaneous records
    Contemporaneous,
}

/// Strength of evidence
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceStrength {
    /// Weak evidence
    Weak,
    /// Moderate evidence
    Moderate,
    /// Strong evidence
    Strong,
    /// Compelling/overwhelming
    Compelling,
}

/// Res ipsa loquitur analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResIpsaLoquitur {
    /// Thing causing damage in defendant's control?
    pub defendant_control: bool,
    /// Accident would not normally happen without negligence?
    pub would_not_normally_happen: bool,
    /// Cause of accident unknown?
    pub cause_unknown: bool,
    /// Doctrine applies?
    pub applies: bool,
    /// Effect if applies
    pub effect: ResIpsaEffect,
}

/// Effect of res ipsa loquitur
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResIpsaEffect {
    /// Evidential - supports inference of negligence
    Evidential,
    /// Reverses burden of proof
    ReversesBurden,
    /// Not applicable
    NotApplicable,
}

// ============================================================================
// Causation Types
// ============================================================================

/// Causation analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CausationAnalysis {
    /// Factual causation (but-for test)
    pub factual_causation: FactualCausation,
    /// Legal causation (remoteness)
    pub legal_causation: LegalCausation,
    /// Intervening acts (novus actus)
    pub intervening_acts: Vec<InterveningAct>,
    /// Causation established?
    pub causation_established: bool,
}

/// Factual causation analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FactualCausation {
    /// But-for test satisfied?
    pub but_for_satisfied: bool,
    /// Material contribution to harm (Bailey v MOD)?
    pub material_contribution: Option<MaterialContribution>,
    /// Material increase in risk (Fairchild)?
    pub material_increase_risk: Option<MaterialIncrease>,
    /// Loss of chance (Hotson)?
    pub loss_of_chance: Option<LossOfChance>,
    /// Multiple sufficient causes?
    pub multiple_sufficient_causes: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Material contribution analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaterialContribution {
    /// Did breach make material contribution to harm?
    pub contributes: bool,
    /// More than de minimis?
    pub more_than_de_minimis: bool,
    /// Scientific uncertainty?
    pub scientific_uncertainty: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Material increase in risk (Fairchild exception)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaterialIncrease {
    /// Multiple tortfeasors created same risk?
    pub multiple_tortfeasors: bool,
    /// Agent causing harm has single cause?
    pub single_agent: bool,
    /// Impossible to identify which caused harm?
    pub impossible_to_identify: bool,
    /// Each defendant increased risk?
    pub each_increased_risk: bool,
    /// Fairchild exception applies?
    pub fairchild_applies: bool,
    /// Apportionment approach
    pub apportionment: Option<Apportionment>,
}

/// Loss of chance analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LossOfChance {
    /// Type of case (personal injury vs other)
    pub case_type: LossOfChanceCase,
    /// Chance lost (percentage)
    pub chance_lost_percentage: f64,
    /// Was chance greater than 50%?
    pub greater_than_fifty: bool,
    /// Is loss of chance recoverable in this context?
    pub recoverable: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Type of loss of chance case
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LossOfChanceCase {
    /// Personal injury (Hotson - generally not recoverable)
    PersonalInjury,
    /// Medical negligence (Gregg v Scott)
    MedicalNegligence,
    /// Commercial/economic (Allied Maples - recoverable)
    Commercial,
    /// Solicitor's negligence (recoverable)
    SolicitorsNegligence,
}

/// Apportionment approach
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Apportionment {
    /// Joint and several liability
    JointAndSeveral,
    /// Several liability (Barker v Corus)
    Several,
    /// Contribution under Civil Liability (Contribution) Act 1978
    Contribution,
}

/// Legal causation (remoteness) analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LegalCausation {
    /// Type of harm
    pub harm_type: HarmType,
    /// Was type of harm reasonably foreseeable (Wagon Mound)?
    pub type_foreseeable: bool,
    /// Extent of harm irrelevant (Smith v Leech Brain)?
    pub extent_irrelevant: bool,
    /// Egg-shell skull rule applies?
    pub eggshell_skull: bool,
    /// Remoteness test satisfied?
    pub remoteness_satisfied: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Type of harm suffered
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HarmType {
    /// Physical injury to person
    PhysicalInjury,
    /// Psychiatric injury
    PsychiatricInjury,
    /// Property damage
    PropertyDamage,
    /// Pure economic loss
    PureEconomicLoss,
    /// Consequential economic loss
    ConsequentialEconomicLoss,
    /// Loss of amenity
    LossOfAmenity,
}

/// Intervening act analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterveningAct {
    /// Type of intervening act
    pub act_type: InterveningActType,
    /// Description of the act
    pub description: String,
    /// Was the act foreseeable?
    pub foreseeable: bool,
    /// Did it break the chain of causation?
    pub breaks_chain: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Type of intervening act
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterveningActType {
    /// Act of claimant
    ClaimantAct,
    /// Act of third party
    ThirdPartyAct,
    /// Natural event
    NaturalEvent,
    /// Medical treatment
    MedicalTreatment,
    /// Rescue attempt
    RescueAttempt,
}

// ============================================================================
// Damage Types
// ============================================================================

/// Damage suffered
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Damage {
    /// Type of damage
    pub damage_type: DamageType,
    /// Description
    pub description: String,
    /// Monetary value (if quantifiable)
    pub monetary_value: Option<f64>,
    /// Date damage occurred/discovered
    pub date: Option<NaiveDate>,
    /// Is damage continuing?
    pub continuing: bool,
}

/// Type of damage
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DamageType {
    /// Personal injury
    PersonalInjury(InjurySeverity),
    /// Psychiatric harm
    PsychiatricHarm(PsychiatricHarmType),
    /// Property damage
    PropertyDamage,
    /// Economic loss
    EconomicLoss(EconomicLossType),
    /// Loss of amenity
    LossOfAmenity,
    /// Pain and suffering
    PainAndSuffering,
    /// Bereavement
    Bereavement,
    /// Aggravated damages
    AggravatedDamages,
}

/// Severity of personal injury
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InjurySeverity {
    /// Minor injury
    Minor,
    /// Moderate injury
    Moderate,
    /// Serious injury
    Serious,
    /// Severe/permanent injury
    Severe,
    /// Catastrophic injury
    Catastrophic,
    /// Fatal injury
    Fatal,
}

/// Type of psychiatric harm
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PsychiatricHarmType {
    /// Primary victim (in zone of danger)
    PrimaryVictim,
    /// Secondary victim (witnessed harm)
    SecondaryVictim,
    /// Bystander
    Bystander,
    /// Rescuer psychiatric injury
    Rescuer,
    /// Stress at work
    OccupationalStress,
}

/// Type of economic loss
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EconomicLossType {
    /// Pure economic loss (no physical damage)
    Pure,
    /// Consequential on physical damage
    Consequential,
    /// Reliance loss
    Reliance,
    /// Expectation loss
    Expectation,
    /// Wasted expenditure
    WastedExpenditure,
}

// ============================================================================
// Defences Types
// ============================================================================

/// Defence to negligence claim
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NegligenceDefence {
    /// Type of defence
    pub defence_type: DefenceType,
    /// Does defence apply?
    pub applies: bool,
    /// Effect if successful
    pub effect: DefenceEffect,
    /// Evidence supporting defence
    pub evidence: String,
}

/// Type of defence
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DefenceType {
    /// Contributory negligence (reduces damages)
    ContributoryNegligence,
    /// Volenti non fit injuria (complete defence)
    Volenti,
    /// Ex turpi causa (illegality)
    ExTurpiCausa,
    /// Limitation (claim time-barred)
    Limitation,
    /// Exclusion of liability (contract/notice)
    ExclusionOfLiability,
    /// Necessity
    Necessity,
    /// Statutory authority
    StatutoryAuthority,
    /// Act of God
    ActOfGod,
    /// Inevitable accident
    InevitableAccident,
}

/// Effect of defence
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DefenceEffect {
    /// Complete defence - no liability
    CompleteDefence,
    /// Partial defence - reduces damages
    ReducesDamages(f64),
    /// Bars claim entirely
    BarsClaimEntirely,
    /// Defence fails
    NoEffect,
}

/// Contributory negligence analysis (Law Reform (Contributory Negligence) Act 1945)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContributoryNegligence {
    /// Claimant's fault
    pub claimant_fault: String,
    /// Did claimant fail to take reasonable care?
    pub failed_reasonable_care: bool,
    /// Did failure contribute to damage?
    pub contributed_to_damage: bool,
    /// Reduction percentage (0-100)
    pub reduction_percentage: u8,
    /// Factors in assessment
    pub assessment_factors: Vec<String>,
}

/// Volenti non fit injuria analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Volenti {
    /// Was there genuine knowledge of risk?
    pub knowledge_of_risk: bool,
    /// Was there genuine consent?
    pub genuine_consent: bool,
    /// Was consent freely given (not under duress)?
    pub freely_given: bool,
    /// Did claimant consent to particular risk that materialized?
    pub consent_to_particular_risk: bool,
    /// Is the context one where volenti cannot apply?
    pub excluded_context: Option<VolentiExclusion>,
    /// Defence succeeds?
    pub defence_succeeds: bool,
}

/// Contexts where volenti is excluded
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VolentiExclusion {
    /// Employment (Smith v Baker)
    Employment,
    /// Road traffic (s.149 Road Traffic Act 1988)
    RoadTraffic,
    /// Rescue cases
    Rescue,
    /// Unfair Contract Terms Act 1977
    UCTA,
}

/// Ex turpi causa analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExTurpiCausa {
    /// Was claimant engaged in illegal activity?
    pub illegal_activity: bool,
    /// Type of illegality
    pub illegality_type: Option<IllegalityType>,
    /// Sufficient connection between illegality and harm?
    pub sufficient_connection: bool,
    /// Would allowing claim damage integrity of legal system?
    pub damages_legal_integrity: bool,
    /// Proportionality (Patel v Mirza)
    pub proportionate: bool,
    /// Defence succeeds?
    pub defence_succeeds: bool,
}

/// Type of illegality for ex turpi
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IllegalityType {
    /// Criminal act
    Criminal,
    /// Civil wrong
    CivilWrong,
    /// Breach of statutory duty
    BreachStatutoryDuty,
    /// Fraud
    Fraud,
}

// ============================================================================
// Limitation Types
// ============================================================================

/// Limitation period analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LimitationAnalysis {
    /// Type of claim
    pub claim_type: LimitationClaimType,
    /// Applicable limitation period
    pub limitation_period: LimitationPeriod,
    /// Date of accrual
    pub accrual_date: Option<NaiveDate>,
    /// Date of knowledge (if applicable)
    pub date_of_knowledge: Option<NaiveDate>,
    /// Is claim time-barred?
    pub time_barred: bool,
    /// Court discretion under s.33 LA 1980?
    pub section_33_discretion: bool,
}

/// Type of claim for limitation purposes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LimitationClaimType {
    /// Personal injury
    PersonalInjury,
    /// Property damage/other tort
    OtherTort,
    /// Fatal accident claim
    FatalAccident,
    /// Defamation
    Defamation,
    /// Consumer Protection Act claim
    ConsumerProtection,
}

/// Limitation period
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LimitationPeriod {
    /// 1 year (defamation under s.4A LA 1980)
    OneYear,
    /// 3 years (personal injury under s.11 LA 1980)
    ThreeYears,
    /// 6 years (other torts under s.2 LA 1980)
    SixYears,
    /// 10 years (CPA 1987 longstop)
    TenYears,
    /// 15 years (under s.14B for latent damage)
    FifteenYears,
}

// ============================================================================
// Psychiatric Injury Types
// ============================================================================

/// Psychiatric injury analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PsychiatricInjuryAnalysis {
    /// Type of victim
    pub victim_type: PsychiatricVictimType,
    /// Recognized psychiatric illness?
    pub recognized_illness: bool,
    /// Illness name
    pub illness: String,
    /// Alcock control mechanisms (for secondary victims)
    pub alcock_control: Option<AlcockControl>,
    /// Claim succeeds?
    pub claim_succeeds: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Type of psychiatric injury victim
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PsychiatricVictimType {
    /// Primary victim (in zone of physical danger)
    Primary,
    /// Secondary victim (witnessed aftermath)
    Secondary,
    /// Rescuer
    Rescuer,
    /// Communicator of bad news (not generally liable)
    Communicator,
    /// Employee (occupational stress)
    Employee,
}

/// Alcock control mechanisms for secondary victims
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlcockControl {
    /// Close tie of love and affection?
    pub close_tie: CloseTie,
    /// Proximity in time and space?
    pub proximity_time_space: ProximityTimeSpace,
    /// Perception through own unaided senses?
    pub own_unaided_senses: bool,
    /// All control mechanisms satisfied?
    pub all_satisfied: bool,
}

/// Close tie of love and affection
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloseTie {
    /// Relationship to primary victim
    pub relationship: Relationship,
    /// Is close tie presumed or must be proved?
    pub presumed: bool,
    /// Evidence of close tie
    pub evidence: Option<String>,
    /// Requirement satisfied?
    pub satisfied: bool,
}

/// Relationship for psychiatric injury
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Relationship {
    /// Parent-child (presumed)
    ParentChild,
    /// Spouse/civil partner (presumed)
    Spouse,
    /// Engaged couple (presumed)
    Engaged,
    /// Sibling (must be proved)
    Sibling,
    /// Grandparent (must be proved)
    Grandparent,
    /// Close friend (must be proved)
    CloseFriend,
    /// Other (must be proved)
    Other(String),
}

/// Proximity in time and space
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProximityTimeSpace {
    /// Witnessed accident itself?
    pub witnessed_accident: bool,
    /// Witnessed immediate aftermath?
    pub witnessed_immediate_aftermath: bool,
    /// Time between accident and witnessing
    pub time_delay: Option<String>,
    /// Requirement satisfied?
    pub satisfied: bool,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Pure Economic Loss Types
// ============================================================================

/// Pure economic loss analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PureEconomicLossAnalysis {
    /// Type of economic loss claim
    pub claim_type: EconomicLossClaimType,
    /// Is this a recognized exception?
    pub recognized_exception: bool,
    /// Hedley Byrne analysis (if applicable)
    pub hedley_byrne: Option<HedleyByrneAnalysis>,
    /// Extended Hedley Byrne (if applicable)
    pub extended_hedley_byrne: Option<ExtendedHedleyByrne>,
    /// Claim likely to succeed?
    pub claim_succeeds: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Type of pure economic loss claim
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EconomicLossClaimType {
    /// Negligent misstatement (Hedley Byrne)
    NegligentMisstatement,
    /// Negligent service (Henderson v Merrett)
    NegligentService,
    /// Defective product (Murphy v Brentwood)
    DefectiveProduct,
    /// Damage to third party property
    ThirdPartyPropertyDamage,
    /// Relational economic loss
    RelationalLoss,
    /// Wasted expenditure
    WastedExpenditure,
}

/// Hedley Byrne analysis for negligent misstatement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HedleyByrneAnalysis {
    /// Special relationship exists?
    pub special_relationship: bool,
    /// Defendant assumed responsibility?
    pub assumption_of_responsibility: bool,
    /// Claimant relied on statement?
    pub reliance: bool,
    /// Reliance was reasonable?
    pub reasonable_reliance: bool,
    /// Defendant knew/should have known claimant would rely?
    pub defendant_knew_of_reliance: bool,
    /// Disclaimer effective?
    pub effective_disclaimer: bool,
    /// All requirements met?
    pub requirements_met: bool,
}

/// Extended Hedley Byrne (Henderson v Merrett)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtendedHedleyByrne {
    /// Defendant assumed responsibility for task?
    pub assumed_responsibility: bool,
    /// Was there an undertaking to exercise skill/care?
    pub undertaking: bool,
    /// Did claimant rely on that undertaking?
    pub claimant_relied: bool,
    /// Would concurrent liability in contract exclude?
    pub contract_excludes: bool,
    /// Requirements met?
    pub requirements_met: bool,
}

impl DutyOfCareAnalysis {
    /// Create a new duty of care analysis
    pub fn new(
        foreseeability: Foreseeability,
        proximity: Proximity,
        fair_just_reasonable: FairJustReasonable,
    ) -> Self {
        let duty_exists = foreseeability.harm_foreseeable
            && proximity.degree >= 5
            && fair_just_reasonable.overall;

        Self {
            foreseeability,
            proximity,
            fair_just_reasonable,
            established_category: None,
            novel_claim: true,
            duty_exists,
            reasoning: String::new(),
        }
    }

    /// Apply Caparo three-stage test
    pub fn apply_caparo_test(&self) -> bool {
        self.foreseeability.harm_foreseeable
            && self.proximity.degree >= 5
            && self.fair_just_reasonable.overall
    }
}

impl StandardOfCare {
    /// Create standard for reasonable person
    pub fn reasonable_person(reasonable_person_test: ReasonablePersonTest) -> Self {
        Self {
            standard_type: StandardType::ReasonablePerson,
            reasonable_person: reasonable_person_test,
            special_skill: None,
            child_defendant: None,
            standard_description: "The standard of the ordinary reasonable person".to_string(),
        }
    }

    /// Create standard for professional (Bolam)
    pub fn professional(
        bolam_test: BolamTest,
        reasonable_person_test: ReasonablePersonTest,
    ) -> Self {
        let profession = bolam_test.profession.clone();
        Self {
            standard_type: StandardType::ReasonableProfessional,
            reasonable_person: reasonable_person_test,
            special_skill: Some(bolam_test),
            child_defendant: None,
            standard_description: format!("The standard of a reasonably competent {}", profession),
        }
    }
}

impl BreachOfDuty {
    /// Create a breach analysis
    pub fn analyze(
        standard: StandardOfCare,
        defendant_conduct: String,
        fell_below_standard: bool,
    ) -> Self {
        Self {
            standard,
            defendant_conduct,
            fell_below_standard,
            evidence: Vec::new(),
            res_ipsa_loquitur: None,
            reasoning: String::new(),
        }
    }

    /// Add evidence of breach
    pub fn with_evidence(mut self, evidence: BreachEvidence) -> Self {
        self.evidence.push(evidence);
        self
    }

    /// Add res ipsa loquitur analysis
    pub fn with_res_ipsa(mut self, res_ipsa: ResIpsaLoquitur) -> Self {
        self.res_ipsa_loquitur = Some(res_ipsa);
        self
    }
}

impl CausationAnalysis {
    /// Create a causation analysis
    pub fn new(factual: FactualCausation, legal: LegalCausation) -> Self {
        let causation_established = (factual.but_for_satisfied
            || factual
                .material_contribution
                .as_ref()
                .is_some_and(|m| m.contributes))
            && legal.remoteness_satisfied;

        Self {
            factual_causation: factual,
            legal_causation: legal,
            intervening_acts: Vec::new(),
            causation_established,
        }
    }

    /// Add intervening act
    pub fn with_intervening_act(mut self, act: InterveningAct) -> Self {
        if act.breaks_chain {
            self.causation_established = false;
        }
        self.intervening_acts.push(act);
        self
    }

    /// Check if any intervening act breaks the chain
    pub fn chain_broken(&self) -> bool {
        self.intervening_acts.iter().any(|a| a.breaks_chain)
    }
}

impl ContributoryNegligence {
    /// Create contributory negligence analysis
    pub fn analyze(
        claimant_fault: String,
        failed_reasonable_care: bool,
        contributed_to_damage: bool,
        reduction_percentage: u8,
    ) -> Self {
        Self {
            claimant_fault,
            failed_reasonable_care,
            contributed_to_damage,
            reduction_percentage: reduction_percentage.min(100),
            assessment_factors: Vec::new(),
        }
    }

    /// Calculate reduced damages
    pub fn calculate_reduced_damages(&self, full_damages: f64) -> f64 {
        if !self.failed_reasonable_care || !self.contributed_to_damage {
            return full_damages;
        }
        full_damages * (1.0 - (f64::from(self.reduction_percentage) / 100.0))
    }
}

impl LimitationAnalysis {
    /// Create limitation analysis
    pub fn for_personal_injury(accrual_date: NaiveDate, claim_date: NaiveDate) -> Self {
        let years_elapsed = (claim_date - accrual_date).num_days() / 365;
        let time_barred = years_elapsed >= 3;

        Self {
            claim_type: LimitationClaimType::PersonalInjury,
            limitation_period: LimitationPeriod::ThreeYears,
            accrual_date: Some(accrual_date),
            date_of_knowledge: None,
            time_barred,
            section_33_discretion: time_barred,
        }
    }

    /// Create limitation analysis for other torts
    pub fn for_other_tort(accrual_date: NaiveDate, claim_date: NaiveDate) -> Self {
        let years_elapsed = (claim_date - accrual_date).num_days() / 365;
        let time_barred = years_elapsed >= 6;

        Self {
            claim_type: LimitationClaimType::OtherTort,
            limitation_period: LimitationPeriod::SixYears,
            accrual_date: Some(accrual_date),
            date_of_knowledge: None,
            time_barred,
            section_33_discretion: false,
        }
    }
}

impl PsychiatricInjuryAnalysis {
    /// Analyze secondary victim claim under Alcock
    pub fn secondary_victim(illness: String, alcock: AlcockControl) -> Self {
        let claim_succeeds = alcock.all_satisfied;

        Self {
            victim_type: PsychiatricVictimType::Secondary,
            recognized_illness: true,
            illness,
            alcock_control: Some(alcock),
            claim_succeeds,
            reasoning: String::new(),
        }
    }

    /// Analyze primary victim claim
    pub fn primary_victim(illness: String, in_zone_of_danger: bool) -> Self {
        Self {
            victim_type: PsychiatricVictimType::Primary,
            recognized_illness: true,
            illness,
            alcock_control: None,
            claim_succeeds: in_zone_of_danger,
            reasoning: if in_zone_of_danger {
                "Primary victim within zone of physical danger".to_string()
            } else {
                "Claimant was not within zone of physical danger".to_string()
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duty_of_care_caparo() {
        let foreseeability = Foreseeability {
            harm_foreseeable: true,
            claimant_foreseeable: true,
            manner_foreseeable: true,
            reasoning: "Harm was clearly foreseeable".to_string(),
        };

        let proximity = Proximity {
            proximity_type: ProximityType::ManufacturerConsumer,
            degree: 8,
            physical_proximity: true,
            circumstantial_proximity: true,
            causal_proximity: true,
            reasoning: "Direct manufacturer-consumer relationship".to_string(),
        };

        let fjr = FairJustReasonable {
            fair: true,
            just: true,
            reasonable: true,
            policy_considerations: vec![PolicyConsideration::Deterrence],
            overall: true,
            reasoning: "Established category applies".to_string(),
        };

        let analysis = DutyOfCareAnalysis::new(foreseeability, proximity, fjr);
        assert!(analysis.duty_exists);
        assert!(analysis.apply_caparo_test());
    }

    #[test]
    fn test_contributory_negligence_reduction() {
        let cn = ContributoryNegligence::analyze(
            "Claimant failed to wear seatbelt".to_string(),
            true,
            true,
            25,
        );

        let reduced = cn.calculate_reduced_damages(100_000.0);
        assert!((reduced - 75_000.0).abs() < 0.01);
    }

    #[test]
    fn test_limitation_personal_injury() {
        let accrual = NaiveDate::from_ymd_opt(2020, 1, 1).expect("valid date");
        let claim = NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date");

        let analysis = LimitationAnalysis::for_personal_injury(accrual, claim);
        assert!(analysis.time_barred);
        assert!(analysis.section_33_discretion);
    }

    #[test]
    fn test_limitation_within_time() {
        let accrual = NaiveDate::from_ymd_opt(2023, 1, 1).expect("valid date");
        let claim = NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date");

        let analysis = LimitationAnalysis::for_personal_injury(accrual, claim);
        assert!(!analysis.time_barred);
    }

    #[test]
    fn test_causation_but_for() {
        let factual = FactualCausation {
            but_for_satisfied: true,
            material_contribution: None,
            material_increase_risk: None,
            loss_of_chance: None,
            multiple_sufficient_causes: false,
            reasoning: "But-for test satisfied".to_string(),
        };

        let legal = LegalCausation {
            harm_type: HarmType::PhysicalInjury,
            type_foreseeable: true,
            extent_irrelevant: true,
            eggshell_skull: false,
            remoteness_satisfied: true,
            reasoning: "Type of harm foreseeable".to_string(),
        };

        let analysis = CausationAnalysis::new(factual, legal);
        assert!(analysis.causation_established);
    }

    #[test]
    fn test_intervening_act_breaks_chain() {
        let factual = FactualCausation {
            but_for_satisfied: true,
            material_contribution: None,
            material_increase_risk: None,
            loss_of_chance: None,
            multiple_sufficient_causes: false,
            reasoning: String::new(),
        };

        let legal = LegalCausation {
            harm_type: HarmType::PhysicalInjury,
            type_foreseeable: true,
            extent_irrelevant: true,
            eggshell_skull: false,
            remoteness_satisfied: true,
            reasoning: String::new(),
        };

        let act = InterveningAct {
            act_type: InterveningActType::ThirdPartyAct,
            description: "Unforeseeable third party intervention".to_string(),
            foreseeable: false,
            breaks_chain: true,
            reasoning: String::new(),
        };

        let analysis = CausationAnalysis::new(factual, legal).with_intervening_act(act);
        assert!(!analysis.causation_established);
        assert!(analysis.chain_broken());
    }
}
