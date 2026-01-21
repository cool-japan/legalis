//! UK Negligence Law
//!
//! This module implements the law of negligence under common law, including:
//! - Donoghue v Stevenson \[1932\] AC 562 (neighbour principle)
//! - Caparo Industries v Dickman \[1990\] 2 AC 605 (three-stage test)
//! - Bolam v Friern Hospital \[1957\] 1 WLR 582 (professional standard)
//! - Bolitho v City and Hackney HA \[1998\] AC 232 (logical Bolam)
//! - Psychiatric injury (Alcock control mechanisms)
//! - Pure economic loss (Hedley Byrne, Murphy)

use serde::{Deserialize, Serialize};

use super::error::TortError;
use super::types::{
    AlcockControl, BolamTest, BreachEvidence, BreachFactor, BreachOfDuty, CausationAnalysis,
    CloseTie, CommonPractice, ContributoryNegligence, CostLevel, Damage, DefenceType,
    DutyOfCareAnalysis, EconomicLossClaimType, EstablishedDutyCategory, EvidenceStrength,
    EvidenceType, ExtendedHedleyByrne, FactualCausation, FairJustReasonable, Foreseeability,
    HarmGravity, HarmType, HedleyByrneAnalysis, LegalCausation, LimitationAnalysis,
    NegligenceDefence, PolicyConsideration, ProfessionalCapacity, Proximity, ProximityTimeSpace,
    ProximityType, PsychiatricInjuryAnalysis, PsychiatricVictimType, PureEconomicLossAnalysis,
    ReasonablePersonTest, Relationship, ResIpsaEffect, ResIpsaLoquitur, RiskLevel, SocialUtility,
    StandardOfCare, StandardType, TortParty, TortType, Volenti, VolentiExclusion,
};

// ============================================================================
// Negligence Claim Analysis
// ============================================================================

/// Full negligence claim analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NegligenceClaimAnalysis {
    /// Claimant details
    pub claimant: TortParty,
    /// Defendant details
    pub defendant: TortParty,
    /// Tort type
    pub tort_type: TortType,
    /// Duty of care analysis (Caparo)
    pub duty_of_care: DutyOfCareAnalysis,
    /// Standard and breach analysis
    pub breach: BreachOfDuty,
    /// Causation analysis
    pub causation: CausationAnalysis,
    /// Damages
    pub damages: Vec<Damage>,
    /// Defences raised
    pub defences: Vec<NegligenceDefence>,
    /// Limitation analysis
    pub limitation: Option<LimitationAnalysis>,
    /// Claim succeeds?
    pub claim_succeeds: bool,
    /// If claim succeeds, quantum of damages
    pub quantum: Option<f64>,
    /// Overall reasoning
    pub reasoning: String,
}

impl NegligenceClaimAnalysis {
    /// Create a new negligence claim analysis
    pub fn new(claimant: TortParty, defendant: TortParty) -> Self {
        Self {
            claimant,
            defendant,
            tort_type: TortType::Negligence,
            duty_of_care: create_default_duty_analysis(),
            breach: create_default_breach_analysis(),
            causation: create_default_causation_analysis(),
            damages: Vec::new(),
            defences: Vec::new(),
            limitation: None,
            claim_succeeds: false,
            quantum: None,
            reasoning: String::new(),
        }
    }

    /// Set duty of care analysis
    pub fn with_duty_analysis(mut self, duty: DutyOfCareAnalysis) -> Self {
        self.duty_of_care = duty;
        self
    }

    /// Set breach analysis
    pub fn with_breach_analysis(mut self, breach: BreachOfDuty) -> Self {
        self.breach = breach;
        self
    }

    /// Set causation analysis
    pub fn with_causation_analysis(mut self, causation: CausationAnalysis) -> Self {
        self.causation = causation;
        self
    }

    /// Add damage
    pub fn with_damage(mut self, damage: Damage) -> Self {
        self.damages.push(damage);
        self
    }

    /// Add defence
    pub fn with_defence(mut self, defence: NegligenceDefence) -> Self {
        self.defences.push(defence);
        self
    }

    /// Set limitation analysis
    pub fn with_limitation(mut self, limitation: LimitationAnalysis) -> Self {
        self.limitation = Some(limitation);
        self
    }

    /// Analyze the complete claim
    pub fn analyze(mut self) -> Result<Self, TortError> {
        // Check limitation first
        if let Some(ref limitation) = self.limitation
            && limitation.time_barred
            && !limitation.section_33_discretion
        {
            return Err(TortError::LimitationExpired {
                period: format!("{:?}", limitation.limitation_period),
                expired_date: limitation
                    .accrual_date
                    .map(|d| d.to_string())
                    .unwrap_or_else(|| "unknown".to_string()),
            });
        }

        // Check duty of care
        if !self.duty_of_care.duty_exists {
            return Err(TortError::NoDutyOfCare {
                reason: self.duty_of_care.reasoning.clone(),
                missing_element: identify_missing_caparo_element(&self.duty_of_care),
            });
        }

        // Check breach
        if !self.breach.fell_below_standard {
            return Err(TortError::NoBreachOfDuty {
                expected_standard: self.breach.standard.standard_description.clone(),
                actual_conduct: self.breach.defendant_conduct.clone(),
            });
        }

        // Check causation
        if !self.causation.causation_established {
            if !self.causation.factual_causation.but_for_satisfied {
                return Err(TortError::ButForNotSatisfied {
                    alternative_cause: self.causation.factual_causation.reasoning.clone(),
                });
            }
            if self.causation.chain_broken()
                && let Some(act) = self
                    .causation
                    .intervening_acts
                    .iter()
                    .find(|a| a.breaks_chain)
            {
                return Err(TortError::ChainOfCausationBroken {
                    intervening_act: format!("{:?}", act.act_type),
                    explanation: act.reasoning.clone(),
                });
            }
            if !self.causation.legal_causation.remoteness_satisfied {
                return Err(TortError::DamageTooRemote {
                    harm_type: format!("{:?}", self.causation.legal_causation.harm_type),
                    explanation: self.causation.legal_causation.reasoning.clone(),
                });
            }
        }

        // Check damages
        if self.damages.is_empty() {
            return Err(TortError::NoActionableDamage {
                claimed_damage: "none".to_string(),
                reason: "No damage claimed".to_string(),
            });
        }

        // Check defences
        for defence in &self.defences {
            if defence.applies {
                match defence.defence_type {
                    DefenceType::Volenti => {
                        return Err(TortError::VolentiApplies {
                            consent: defence.evidence.clone(),
                        });
                    }
                    DefenceType::ExTurpiCausa => {
                        return Err(TortError::ExTurpiApplies {
                            illegality: defence.evidence.clone(),
                        });
                    }
                    _ => {}
                }
            }
        }

        // Calculate quantum
        let mut total_damages: f64 = self.damages.iter().filter_map(|d| d.monetary_value).sum();

        // Apply contributory negligence reduction
        for defence in &self.defences {
            if let DefenceType::ContributoryNegligence = defence.defence_type
                && defence.applies
                && let super::types::DefenceEffect::ReducesDamages(percentage) = defence.effect
            {
                total_damages *= 1.0 - percentage;
            }
        }

        self.claim_succeeds = true;
        self.quantum = Some(total_damages);
        self.reasoning = build_success_reasoning(&self);

        Ok(self)
    }
}

/// Identify which Caparo element is missing
fn identify_missing_caparo_element(duty: &DutyOfCareAnalysis) -> String {
    if !duty.foreseeability.harm_foreseeable {
        "foreseeability".to_string()
    } else if duty.proximity.degree < 5 {
        "proximity".to_string()
    } else if !duty.fair_just_reasonable.overall {
        "fair just and reasonable".to_string()
    } else {
        "unknown".to_string()
    }
}

/// Build success reasoning
fn build_success_reasoning(analysis: &NegligenceClaimAnalysis) -> String {
    format!(
        "Claim succeeds: Duty of care established under Caparo test. \
         Breach: defendant fell below the standard of {}. \
         Causation: but-for test satisfied and damage not too remote. \
         Damages: £{:.2}",
        analysis.breach.standard.standard_description,
        analysis.quantum.unwrap_or(0.0)
    )
}

/// Create default duty analysis
fn create_default_duty_analysis() -> DutyOfCareAnalysis {
    DutyOfCareAnalysis {
        foreseeability: Foreseeability {
            harm_foreseeable: false,
            claimant_foreseeable: false,
            manner_foreseeable: false,
            reasoning: String::new(),
        },
        proximity: Proximity {
            proximity_type: ProximityType::None,
            degree: 0,
            physical_proximity: false,
            circumstantial_proximity: false,
            causal_proximity: false,
            reasoning: String::new(),
        },
        fair_just_reasonable: FairJustReasonable {
            fair: false,
            just: false,
            reasonable: false,
            policy_considerations: Vec::new(),
            overall: false,
            reasoning: String::new(),
        },
        established_category: None,
        novel_claim: true,
        duty_exists: false,
        reasoning: String::new(),
    }
}

/// Create default breach analysis
fn create_default_breach_analysis() -> BreachOfDuty {
    BreachOfDuty {
        standard: StandardOfCare {
            standard_type: StandardType::ReasonablePerson,
            reasonable_person: ReasonablePersonTest {
                magnitude_of_risk: RiskLevel::Low,
                gravity_of_harm: HarmGravity::Minor,
                cost_of_precautions: CostLevel::Low,
                social_utility: SocialUtility::Low,
                common_practice: None,
                factors: Vec::new(),
            },
            special_skill: None,
            child_defendant: None,
            standard_description: String::new(),
        },
        defendant_conduct: String::new(),
        fell_below_standard: false,
        evidence: Vec::new(),
        res_ipsa_loquitur: None,
        reasoning: String::new(),
    }
}

/// Create default causation analysis
fn create_default_causation_analysis() -> CausationAnalysis {
    CausationAnalysis {
        factual_causation: FactualCausation {
            but_for_satisfied: false,
            material_contribution: None,
            material_increase_risk: None,
            loss_of_chance: None,
            multiple_sufficient_causes: false,
            reasoning: String::new(),
        },
        legal_causation: LegalCausation {
            harm_type: HarmType::PhysicalInjury,
            type_foreseeable: false,
            extent_irrelevant: true,
            eggshell_skull: false,
            remoteness_satisfied: false,
            reasoning: String::new(),
        },
        intervening_acts: Vec::new(),
        causation_established: false,
    }
}

// ============================================================================
// Duty of Care Analyzer (Caparo Test)
// ============================================================================

/// Analyzer for duty of care using the Caparo three-stage test
#[derive(Debug, Clone)]
pub struct CaparoAnalyzer {
    /// Context of the case
    context: CaseContext,
    /// Established category if any
    established_category: Option<EstablishedDutyCategory>,
}

/// Case context for duty analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaseContext {
    /// Type of harm
    pub harm_type: HarmType,
    /// Is this a novel claim?
    pub novel_claim: bool,
    /// Relationship between parties
    pub relationship: String,
    /// Professional context?
    pub professional_context: Option<ProfessionalCapacity>,
    /// Description of facts
    pub facts: String,
}

impl CaparoAnalyzer {
    /// Create a new Caparo analyzer
    pub fn new(context: CaseContext) -> Self {
        let established_category = identify_established_category(&context);
        Self {
            context,
            established_category,
        }
    }

    /// Analyze foreseeability
    pub fn analyze_foreseeability(&self, facts: &ForeseeabilityFacts) -> Foreseeability {
        let harm_foreseeable = facts.known_risk || facts.risk_obvious || facts.previous_incidents;
        let claimant_foreseeable = facts.claimant_identifiable || facts.class_foreseeable;
        let manner_foreseeable = facts.manner_similar_to_known || !facts.highly_unusual_manner;

        Foreseeability {
            harm_foreseeable,
            claimant_foreseeable,
            manner_foreseeable,
            reasoning: build_foreseeability_reasoning(facts, harm_foreseeable),
        }
    }

    /// Analyze proximity
    pub fn analyze_proximity(&self, facts: &ProximityFacts) -> Proximity {
        let degree = calculate_proximity_degree(facts);
        let proximity_type = identify_proximity_type(&self.context, facts);

        Proximity {
            proximity_type,
            degree,
            physical_proximity: facts.physical_closeness,
            circumstantial_proximity: facts.pre_existing_relationship,
            causal_proximity: facts.direct_cause,
            reasoning: build_proximity_reasoning(facts, degree),
        }
    }

    /// Analyze fair, just and reasonable
    pub fn analyze_fair_just_reasonable(&self, facts: &PolicyFacts) -> FairJustReasonable {
        let policy_considerations = identify_policy_considerations(facts);
        let (fair, just, reasonable) = assess_fjr(&policy_considerations, facts);
        let overall = fair && just && reasonable;

        FairJustReasonable {
            fair,
            just,
            reasonable,
            policy_considerations,
            overall,
            reasoning: build_fjr_reasoning(fair, just, reasonable, facts),
        }
    }

    /// Complete Caparo analysis
    pub fn analyze(
        &self,
        foreseeability_facts: &ForeseeabilityFacts,
        proximity_facts: &ProximityFacts,
        policy_facts: &PolicyFacts,
    ) -> DutyOfCareAnalysis {
        // For established categories, duty follows automatically
        if let Some(ref category) = self.established_category {
            return DutyOfCareAnalysis {
                foreseeability: self.analyze_foreseeability(foreseeability_facts),
                proximity: self.analyze_proximity(proximity_facts),
                fair_just_reasonable: self.analyze_fair_just_reasonable(policy_facts),
                established_category: Some(category.clone()),
                novel_claim: false,
                duty_exists: true,
                reasoning: format!("Established duty category applies: {:?}", category),
            };
        }

        // For novel claims, apply full Caparo test
        let foreseeability = self.analyze_foreseeability(foreseeability_facts);
        let proximity = self.analyze_proximity(proximity_facts);
        let fjr = self.analyze_fair_just_reasonable(policy_facts);

        let duty_exists = foreseeability.harm_foreseeable && proximity.degree >= 5 && fjr.overall;

        DutyOfCareAnalysis {
            foreseeability,
            proximity,
            fair_just_reasonable: fjr,
            established_category: None,
            novel_claim: true,
            duty_exists,
            reasoning: if duty_exists {
                "Novel duty established under Caparo three-stage test".to_string()
            } else {
                "Novel duty not established - Caparo test not satisfied".to_string()
            },
        }
    }
}

/// Facts for foreseeability analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForeseeabilityFacts {
    /// Was the risk known to defendant?
    pub known_risk: bool,
    /// Was the risk obvious?
    pub risk_obvious: bool,
    /// Were there previous similar incidents?
    pub previous_incidents: bool,
    /// Was the claimant identifiable?
    pub claimant_identifiable: bool,
    /// Was the class of claimants foreseeable?
    pub class_foreseeable: bool,
    /// Was the manner of harm similar to known risks?
    pub manner_similar_to_known: bool,
    /// Was the manner highly unusual?
    pub highly_unusual_manner: bool,
}

/// Facts for proximity analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProximityFacts {
    /// Physical closeness between parties
    pub physical_closeness: bool,
    /// Pre-existing relationship
    pub pre_existing_relationship: bool,
    /// Direct causal relationship
    pub direct_cause: bool,
    /// Assumption of responsibility
    pub assumption_of_responsibility: bool,
    /// Reliance by claimant
    pub reliance: bool,
    /// Vulnerable claimant
    pub vulnerable_claimant: bool,
    /// Known reliance
    pub known_reliance: bool,
}

/// Facts for policy analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PolicyFacts {
    /// Would liability lead to indeterminate class?
    pub indeterminate_class: bool,
    /// Would liability cause defensive practices?
    pub defensive_practices_concern: bool,
    /// Are alternative remedies available?
    pub alternative_remedies: bool,
    /// Is there a statutory scheme?
    pub statutory_scheme: bool,
    /// Insurance available/usual?
    pub insurance_available: bool,
    /// Public interest in defendant's activity?
    pub public_interest_in_activity: bool,
    /// Deterrence value
    pub deterrence_value: bool,
    /// Moral blameworthiness of defendant
    pub morally_blameworthy: bool,
}

/// Identify if case falls within established category
fn identify_established_category(context: &CaseContext) -> Option<EstablishedDutyCategory> {
    let relationship_lower = context.relationship.to_lowercase();

    if relationship_lower.contains("employer") && relationship_lower.contains("employee") {
        return Some(EstablishedDutyCategory::EmployerEmployee);
    }
    if relationship_lower.contains("manufacturer") || relationship_lower.contains("product") {
        return Some(EstablishedDutyCategory::ManufacturerConsumer);
    }
    if relationship_lower.contains("road") || relationship_lower.contains("driver") {
        return Some(EstablishedDutyCategory::RoadUsers);
    }
    if relationship_lower.contains("doctor") || relationship_lower.contains("patient") {
        return Some(EstablishedDutyCategory::DoctorPatient);
    }
    if relationship_lower.contains("occupier") || relationship_lower.contains("visitor") {
        return Some(EstablishedDutyCategory::OccupierVisitor);
    }
    if relationship_lower.contains("school") || relationship_lower.contains("pupil") {
        return Some(EstablishedDutyCategory::SchoolPupil);
    }
    if let Some(ref prof) = context.professional_context {
        match prof {
            ProfessionalCapacity::Medical => {
                return Some(EstablishedDutyCategory::DoctorPatient);
            }
            ProfessionalCapacity::Legal
            | ProfessionalCapacity::Accountant
            | ProfessionalCapacity::Surveyor
            | ProfessionalCapacity::Architect
            | ProfessionalCapacity::FinancialAdviser => {
                return Some(EstablishedDutyCategory::ProfessionalClient);
            }
            _ => {}
        }
    }

    None
}

fn build_foreseeability_reasoning(facts: &ForeseeabilityFacts, foreseeable: bool) -> String {
    if foreseeable {
        let reasons: Vec<&str> = [
            facts.known_risk.then_some("defendant knew of risk"),
            facts.risk_obvious.then_some("risk was obvious"),
            facts
                .previous_incidents
                .then_some("previous incidents occurred"),
        ]
        .into_iter()
        .flatten()
        .collect();
        format!("Harm foreseeable: {}", reasons.join(", "))
    } else {
        "Harm not reasonably foreseeable in the circumstances".to_string()
    }
}

fn calculate_proximity_degree(facts: &ProximityFacts) -> u8 {
    let mut degree = 0u8;
    if facts.physical_closeness {
        degree += 2;
    }
    if facts.pre_existing_relationship {
        degree += 2;
    }
    if facts.direct_cause {
        degree += 2;
    }
    if facts.assumption_of_responsibility {
        degree += 2;
    }
    if facts.reliance && facts.known_reliance {
        degree += 2;
    }
    degree.min(10)
}

fn identify_proximity_type(context: &CaseContext, facts: &ProximityFacts) -> ProximityType {
    if facts.assumption_of_responsibility {
        return ProximityType::AssumedResponsibility;
    }
    if context.professional_context.is_some() {
        return ProximityType::ProfessionalClient;
    }
    if facts.physical_closeness {
        return ProximityType::Physical;
    }
    if facts.pre_existing_relationship {
        return ProximityType::Special;
    }
    ProximityType::None
}

fn build_proximity_reasoning(facts: &ProximityFacts, degree: u8) -> String {
    if degree >= 5 {
        format!(
            "Sufficient proximity (degree {}/10): {}",
            degree,
            if facts.assumption_of_responsibility {
                "defendant assumed responsibility"
            } else if facts.pre_existing_relationship {
                "pre-existing relationship"
            } else {
                "direct relationship established"
            }
        )
    } else {
        format!("Insufficient proximity (degree {}/10)", degree)
    }
}

fn identify_policy_considerations(facts: &PolicyFacts) -> Vec<PolicyConsideration> {
    let mut considerations = Vec::new();
    if facts.indeterminate_class {
        considerations.push(PolicyConsideration::Floodgates);
    }
    if facts.defensive_practices_concern {
        considerations.push(PolicyConsideration::DefensivePractices);
    }
    if facts.alternative_remedies {
        considerations.push(PolicyConsideration::AlternativeRemedies);
    }
    if facts.statutory_scheme {
        considerations.push(PolicyConsideration::StatutoryScheme);
    }
    if facts.insurance_available {
        considerations.push(PolicyConsideration::Insurance);
    }
    if facts.public_interest_in_activity {
        considerations.push(PolicyConsideration::PublicInterest);
    }
    if facts.deterrence_value {
        considerations.push(PolicyConsideration::Deterrence);
    }
    if facts.morally_blameworthy {
        considerations.push(PolicyConsideration::MoralBlame);
    }
    considerations
}

fn assess_fjr(_considerations: &[PolicyConsideration], facts: &PolicyFacts) -> (bool, bool, bool) {
    // Fair: moral blameworthiness matters
    let fair = facts.morally_blameworthy || facts.deterrence_value;

    // Just: alternative remedies weigh against
    let just = !facts.alternative_remedies || facts.morally_blameworthy;

    // Reasonable: policy concerns must not outweigh
    let reasonable = !facts.indeterminate_class && !facts.defensive_practices_concern;

    (fair, just, reasonable)
}

fn build_fjr_reasoning(fair: bool, just: bool, reasonable: bool, facts: &PolicyFacts) -> String {
    if fair && just && reasonable {
        "Fair, just and reasonable to impose duty".to_string()
    } else {
        let mut reasons = Vec::new();
        if !fair {
            reasons.push("not fair");
        }
        if !just && facts.alternative_remedies {
            reasons.push("alternative remedies available");
        }
        if !reasonable && facts.indeterminate_class {
            reasons.push("indeterminate liability concern");
        }
        format!("Not fair, just and reasonable: {}", reasons.join(", "))
    }
}

// ============================================================================
// Breach Analyzer (Bolam/Bolitho)
// ============================================================================

/// Analyzer for breach of duty
#[derive(Debug, Clone)]
pub struct BreachAnalyzer {
    /// Is defendant a professional?
    is_professional: bool,
    /// Profession (if applicable)
    profession: Option<String>,
}

impl BreachAnalyzer {
    /// Create analyzer for lay person
    pub fn for_reasonable_person() -> Self {
        Self {
            is_professional: false,
            profession: None,
        }
    }

    /// Create analyzer for professional
    pub fn for_professional(profession: String) -> Self {
        Self {
            is_professional: true,
            profession: Some(profession),
        }
    }

    /// Analyze the standard of care
    pub fn analyze_standard(&self, facts: &BreachFacts) -> StandardOfCare {
        let reasonable_person = ReasonablePersonTest {
            magnitude_of_risk: facts.risk_level.clone(),
            gravity_of_harm: facts.harm_gravity.clone(),
            cost_of_precautions: facts.precaution_cost.clone(),
            social_utility: facts.social_utility.clone(),
            common_practice: facts.common_practice.clone(),
            factors: facts.relevant_factors.clone(),
        };

        if self.is_professional {
            let profession = self
                .profession
                .clone()
                .unwrap_or_else(|| "professional".to_string());
            StandardOfCare {
                standard_type: StandardType::ReasonableProfessional,
                reasonable_person,
                special_skill: Some(BolamTest {
                    profession: profession.clone(),
                    followed_practice: facts.followed_professional_practice,
                    responsible_body_accepts: facts.responsible_body_accepts,
                    bolitho_logical: facts.practice_logical,
                    specialist: facts.is_specialist,
                    meets_standard: facts.followed_professional_practice
                        && facts.responsible_body_accepts
                        && facts.practice_logical,
                }),
                child_defendant: None,
                standard_description: format!(
                    "The standard of a reasonably competent {}",
                    profession
                ),
            }
        } else {
            StandardOfCare {
                standard_type: StandardType::ReasonablePerson,
                reasonable_person,
                special_skill: None,
                child_defendant: None,
                standard_description: "The standard of the ordinary reasonable person".to_string(),
            }
        }
    }

    /// Analyze whether breach occurred
    pub fn analyze_breach(&self, facts: &BreachFacts) -> BreachOfDuty {
        let standard = self.analyze_standard(facts);
        let fell_below_standard = self.determine_breach(&standard, facts);

        let mut evidence = Vec::new();
        for (i, ev) in facts.evidence.iter().enumerate() {
            evidence.push(BreachEvidence {
                evidence_type: facts
                    .evidence_types
                    .get(i)
                    .cloned()
                    .unwrap_or(EvidenceType::Witness),
                description: ev.clone(),
                strength: EvidenceStrength::Moderate,
            });
        }

        let res_ipsa = facts.res_ipsa_facts.as_ref().map(|rif| {
            let applies =
                rif.defendant_control && rif.would_not_normally_happen && rif.cause_unknown;
            ResIpsaLoquitur {
                defendant_control: rif.defendant_control,
                would_not_normally_happen: rif.would_not_normally_happen,
                cause_unknown: rif.cause_unknown,
                applies,
                effect: if applies {
                    ResIpsaEffect::Evidential
                } else {
                    ResIpsaEffect::NotApplicable
                },
            }
        });

        BreachOfDuty {
            standard,
            defendant_conduct: facts.defendant_conduct.clone(),
            fell_below_standard,
            evidence,
            res_ipsa_loquitur: res_ipsa,
            reasoning: self.build_breach_reasoning(fell_below_standard, facts),
        }
    }

    fn determine_breach(&self, standard: &StandardOfCare, facts: &BreachFacts) -> bool {
        // For professionals, apply Bolam/Bolitho
        if let Some(ref bolam) = standard.special_skill {
            // If followed accepted practice AND practice is logical, no breach
            if bolam.followed_practice && bolam.responsible_body_accepts && bolam.bolitho_logical {
                return false;
            }
            // If practice not logical despite being accepted, breach may occur
            if bolam.followed_practice && !bolam.bolitho_logical {
                return true;
            }
        }

        // For reasonable person standard, use Learned Hand formula analogy
        // B < P * L means breach (cost of precautions less than probability * gravity)
        let risk_score = match facts.risk_level {
            RiskLevel::Minimal => 1.0,
            RiskLevel::Low => 2.0,
            RiskLevel::Moderate => 4.0,
            RiskLevel::High => 7.0,
            RiskLevel::VeryHigh => 10.0,
        };

        let gravity_score = match facts.harm_gravity {
            HarmGravity::Minor => 1.0,
            HarmGravity::Moderate => 3.0,
            HarmGravity::Serious => 6.0,
            HarmGravity::Severe => 9.0,
            HarmGravity::Fatal => 10.0,
        };

        let cost_score = match facts.precaution_cost {
            CostLevel::Negligible => 1.0,
            CostLevel::Low => 3.0,
            CostLevel::Moderate => 5.0,
            CostLevel::High => 8.0,
            CostLevel::Prohibitive => 10.0,
        };

        let utility_adjustment = match facts.social_utility {
            SocialUtility::None => 0.0,
            SocialUtility::Low => 1.0,
            SocialUtility::Moderate => 2.0,
            SocialUtility::High => 4.0,
            SocialUtility::Essential => 6.0,
        };

        // Higher risk * gravity compared to cost = breach more likely
        let breach_threshold = (risk_score * gravity_score) - utility_adjustment;
        cost_score < breach_threshold && !facts.took_reasonable_precautions
    }

    fn build_breach_reasoning(&self, fell_below: bool, facts: &BreachFacts) -> String {
        if self.is_professional {
            if fell_below {
                format!(
                    "Professional fell below Bolam standard: {}",
                    facts.defendant_conduct
                )
            } else {
                "Professional met Bolam/Bolitho standard by following accepted practice".to_string()
            }
        } else if fell_below {
            format!(
                "Defendant fell below reasonable person standard: {}",
                facts.defendant_conduct
            )
        } else {
            "Defendant met the reasonable person standard".to_string()
        }
    }
}

/// Facts for breach analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreachFacts {
    /// Defendant's conduct
    pub defendant_conduct: String,
    /// Risk level
    pub risk_level: RiskLevel,
    /// Gravity of potential harm
    pub harm_gravity: HarmGravity,
    /// Cost of precautions
    pub precaution_cost: CostLevel,
    /// Social utility of activity
    pub social_utility: SocialUtility,
    /// Common practice
    pub common_practice: Option<CommonPractice>,
    /// Relevant factors
    pub relevant_factors: Vec<BreachFactor>,
    /// Evidence
    pub evidence: Vec<String>,
    /// Evidence types
    pub evidence_types: Vec<EvidenceType>,
    /// Did defendant take reasonable precautions?
    pub took_reasonable_precautions: bool,
    /// For professionals: did they follow accepted practice?
    pub followed_professional_practice: bool,
    /// For professionals: does responsible body accept practice?
    pub responsible_body_accepts: bool,
    /// Bolitho: is practice logical?
    pub practice_logical: bool,
    /// Is defendant a specialist?
    pub is_specialist: bool,
    /// Res ipsa loquitur facts
    pub res_ipsa_facts: Option<ResIpsaFacts>,
}

/// Facts for res ipsa loquitur
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResIpsaFacts {
    /// Was thing in defendant's control?
    pub defendant_control: bool,
    /// Would accident not normally happen without negligence?
    pub would_not_normally_happen: bool,
    /// Is cause unknown?
    pub cause_unknown: bool,
}

// ============================================================================
// Psychiatric Injury Analyzer
// ============================================================================

/// Analyzer for psychiatric injury claims
#[derive(Debug, Clone)]
pub struct PsychiatricInjuryAnalyzer {
    /// Victim type
    victim_type: PsychiatricVictimType,
}

impl PsychiatricInjuryAnalyzer {
    /// Create analyzer for primary victim
    pub fn for_primary_victim() -> Self {
        Self {
            victim_type: PsychiatricVictimType::Primary,
        }
    }

    /// Create analyzer for secondary victim
    pub fn for_secondary_victim() -> Self {
        Self {
            victim_type: PsychiatricVictimType::Secondary,
        }
    }

    /// Analyze psychiatric injury claim
    pub fn analyze(
        &self,
        facts: &PsychiatricInjuryFacts,
    ) -> Result<PsychiatricInjuryAnalysis, TortError> {
        // Must have recognized psychiatric illness
        if !facts.recognized_illness {
            return Err(TortError::NoRecognizedIllness {
                claimed_condition: facts.illness_name.clone(),
            });
        }

        match self.victim_type {
            PsychiatricVictimType::Primary => self.analyze_primary(facts),
            PsychiatricVictimType::Secondary => self.analyze_secondary(facts),
            PsychiatricVictimType::Rescuer => self.analyze_rescuer(facts),
            PsychiatricVictimType::Employee => self.analyze_employee(facts),
            PsychiatricVictimType::Communicator => Err(TortError::PsychiatricInjuryClaimFails {
                missing_requirement: "Communicators of bad news not generally owed duty"
                    .to_string(),
            }),
        }
    }

    fn analyze_primary(
        &self,
        facts: &PsychiatricInjuryFacts,
    ) -> Result<PsychiatricInjuryAnalysis, TortError> {
        if !facts.in_zone_of_danger {
            return Err(TortError::PsychiatricInjuryClaimFails {
                missing_requirement: "Primary victim must be in zone of physical danger"
                    .to_string(),
            });
        }

        Ok(PsychiatricInjuryAnalysis::primary_victim(
            facts.illness_name.clone(),
            true,
        ))
    }

    fn analyze_secondary(
        &self,
        facts: &PsychiatricInjuryFacts,
    ) -> Result<PsychiatricInjuryAnalysis, TortError> {
        // Apply Alcock control mechanisms
        let close_tie = self.assess_close_tie(facts)?;
        let proximity = self.assess_proximity_time_space(facts)?;

        if !facts.own_unaided_senses {
            return Err(TortError::SecondaryVictimRequirementsNotMet {
                failed_control: "Must perceive through own unaided senses".to_string(),
            });
        }

        let alcock = AlcockControl {
            close_tie,
            proximity_time_space: proximity,
            own_unaided_senses: facts.own_unaided_senses,
            all_satisfied: true,
        };

        Ok(PsychiatricInjuryAnalysis::secondary_victim(
            facts.illness_name.clone(),
            alcock,
        ))
    }

    fn analyze_rescuer(
        &self,
        facts: &PsychiatricInjuryFacts,
    ) -> Result<PsychiatricInjuryAnalysis, TortError> {
        // White v Chief Constable - must be in physical danger
        if !facts.in_zone_of_danger {
            return Err(TortError::PsychiatricInjuryClaimFails {
                missing_requirement: "Rescuer must have been objectively in danger (White v CC)"
                    .to_string(),
            });
        }

        Ok(PsychiatricInjuryAnalysis {
            victim_type: PsychiatricVictimType::Rescuer,
            recognized_illness: true,
            illness: facts.illness_name.clone(),
            alcock_control: None,
            claim_succeeds: true,
            reasoning: "Rescuer in danger can recover (Chadwick v BRB)".to_string(),
        })
    }

    fn analyze_employee(
        &self,
        facts: &PsychiatricInjuryFacts,
    ) -> Result<PsychiatricInjuryAnalysis, TortError> {
        // Hatton v Sutherland principles for occupational stress
        if !facts.work_related_stress {
            return Err(TortError::PsychiatricInjuryClaimFails {
                missing_requirement: "Psychiatric injury must be work-related".to_string(),
            });
        }

        Ok(PsychiatricInjuryAnalysis {
            victim_type: PsychiatricVictimType::Employee,
            recognized_illness: true,
            illness: facts.illness_name.clone(),
            alcock_control: None,
            claim_succeeds: facts.employer_knew_of_vulnerability && facts.employer_could_prevent,
            reasoning: if facts.employer_knew_of_vulnerability && facts.employer_could_prevent {
                "Employer breached duty per Hatton v Sutherland".to_string()
            } else {
                "Employer not liable per Hatton v Sutherland principles".to_string()
            },
        })
    }

    fn assess_close_tie(&self, facts: &PsychiatricInjuryFacts) -> Result<CloseTie, TortError> {
        let presumed = matches!(
            facts.relationship,
            Relationship::ParentChild | Relationship::Spouse | Relationship::Engaged
        );

        if !presumed && !facts.evidence_of_close_tie {
            return Err(TortError::SecondaryVictimRequirementsNotMet {
                failed_control: "No close tie of love and affection proven".to_string(),
            });
        }

        Ok(CloseTie {
            relationship: facts.relationship.clone(),
            presumed,
            evidence: facts.close_tie_evidence.clone(),
            satisfied: true,
        })
    }

    fn assess_proximity_time_space(
        &self,
        facts: &PsychiatricInjuryFacts,
    ) -> Result<ProximityTimeSpace, TortError> {
        if !facts.witnessed_accident && !facts.witnessed_immediate_aftermath {
            return Err(TortError::SecondaryVictimRequirementsNotMet {
                failed_control: "Did not witness accident or immediate aftermath".to_string(),
            });
        }

        Ok(ProximityTimeSpace {
            witnessed_accident: facts.witnessed_accident,
            witnessed_immediate_aftermath: facts.witnessed_immediate_aftermath,
            time_delay: facts.time_delay.clone(),
            satisfied: true,
            reasoning: if facts.witnessed_accident {
                "Witnessed the accident itself".to_string()
            } else {
                "Witnessed immediate aftermath".to_string()
            },
        })
    }
}

/// Facts for psychiatric injury analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PsychiatricInjuryFacts {
    /// Is the illness a recognized psychiatric illness?
    pub recognized_illness: bool,
    /// Name of illness
    pub illness_name: String,
    /// Was claimant in zone of physical danger?
    pub in_zone_of_danger: bool,
    /// Relationship to primary victim
    pub relationship: Relationship,
    /// Evidence of close tie (for non-presumed relationships)
    pub evidence_of_close_tie: bool,
    /// Close tie evidence details
    pub close_tie_evidence: Option<String>,
    /// Did claimant witness the accident?
    pub witnessed_accident: bool,
    /// Did claimant witness immediate aftermath?
    pub witnessed_immediate_aftermath: bool,
    /// Time delay before witnessing
    pub time_delay: Option<String>,
    /// Did claimant perceive through own unaided senses?
    pub own_unaided_senses: bool,
    /// Work-related stress (for employee claims)
    pub work_related_stress: bool,
    /// Did employer know of vulnerability?
    pub employer_knew_of_vulnerability: bool,
    /// Could employer have prevented harm?
    pub employer_could_prevent: bool,
}

// ============================================================================
// Pure Economic Loss Analyzer
// ============================================================================

/// Analyzer for pure economic loss claims
#[derive(Debug, Clone)]
pub struct PureEconomicLossAnalyzer {
    /// Type of claim
    claim_type: EconomicLossClaimType,
}

impl PureEconomicLossAnalyzer {
    /// Create analyzer for negligent misstatement (Hedley Byrne)
    pub fn for_negligent_misstatement() -> Self {
        Self {
            claim_type: EconomicLossClaimType::NegligentMisstatement,
        }
    }

    /// Create analyzer for negligent service (Henderson v Merrett)
    pub fn for_negligent_service() -> Self {
        Self {
            claim_type: EconomicLossClaimType::NegligentService,
        }
    }

    /// Create analyzer for defective product (Murphy v Brentwood)
    pub fn for_defective_product() -> Self {
        Self {
            claim_type: EconomicLossClaimType::DefectiveProduct,
        }
    }

    /// Analyze pure economic loss claim
    pub fn analyze(
        &self,
        facts: &EconomicLossFacts,
    ) -> Result<PureEconomicLossAnalysis, TortError> {
        match self.claim_type {
            EconomicLossClaimType::NegligentMisstatement => self.analyze_hedley_byrne(facts),
            EconomicLossClaimType::NegligentService => self.analyze_henderson(facts),
            EconomicLossClaimType::DefectiveProduct => {
                // Murphy v Brentwood - generally not recoverable
                Err(TortError::PureEconomicLossNotRecoverable {
                    loss_type: "defective product".to_string(),
                    reason: "Pure economic loss from defective products not recoverable per Murphy v Brentwood".to_string(),
                })
            }
            EconomicLossClaimType::RelationalLoss => {
                // Spartan Steel - relational economic loss not recoverable
                Err(TortError::PureEconomicLossNotRecoverable {
                    loss_type: "relational economic loss".to_string(),
                    reason: "Relational economic loss not recoverable per Spartan Steel"
                        .to_string(),
                })
            }
            EconomicLossClaimType::ThirdPartyPropertyDamage => {
                Err(TortError::PureEconomicLossNotRecoverable {
                    loss_type: "third party property damage".to_string(),
                    reason: "Loss from damage to third party property not recoverable".to_string(),
                })
            }
            EconomicLossClaimType::WastedExpenditure => {
                // May be recoverable in specific circumstances
                if facts.special_relationship {
                    Ok(PureEconomicLossAnalysis {
                        claim_type: self.claim_type.clone(),
                        recognized_exception: true,
                        hedley_byrne: None,
                        extended_hedley_byrne: None,
                        claim_succeeds: true,
                        reasoning: "Wasted expenditure recoverable due to special relationship"
                            .to_string(),
                    })
                } else {
                    Err(TortError::PureEconomicLossNotRecoverable {
                        loss_type: "wasted expenditure".to_string(),
                        reason: "No special relationship to support recovery".to_string(),
                    })
                }
            }
        }
    }

    fn analyze_hedley_byrne(
        &self,
        facts: &EconomicLossFacts,
    ) -> Result<PureEconomicLossAnalysis, TortError> {
        // Check all Hedley Byrne requirements
        if !facts.special_relationship {
            return Err(TortError::HedleyByrneNotMet {
                missing_requirement: "No special relationship".to_string(),
            });
        }
        if !facts.assumption_of_responsibility {
            return Err(TortError::NoAssumptionOfResponsibility {
                context: "statement/advice context".to_string(),
            });
        }
        if !facts.reliance {
            return Err(TortError::HedleyByrneNotMet {
                missing_requirement: "No reliance on statement".to_string(),
            });
        }
        if !facts.reasonable_reliance {
            return Err(TortError::HedleyByrneNotMet {
                missing_requirement: "Reliance was not reasonable".to_string(),
            });
        }
        if facts.effective_disclaimer {
            return Err(TortError::HedleyByrneNotMet {
                missing_requirement: "Effective disclaimer excludes liability".to_string(),
            });
        }

        Ok(PureEconomicLossAnalysis {
            claim_type: self.claim_type.clone(),
            recognized_exception: true,
            hedley_byrne: Some(HedleyByrneAnalysis {
                special_relationship: true,
                assumption_of_responsibility: true,
                reliance: true,
                reasonable_reliance: true,
                defendant_knew_of_reliance: facts.defendant_knew_of_reliance,
                effective_disclaimer: false,
                requirements_met: true,
            }),
            extended_hedley_byrne: None,
            claim_succeeds: true,
            reasoning: "Hedley Byrne requirements satisfied".to_string(),
        })
    }

    fn analyze_henderson(
        &self,
        facts: &EconomicLossFacts,
    ) -> Result<PureEconomicLossAnalysis, TortError> {
        // Henderson v Merrett extended Hedley Byrne to services
        if !facts.assumption_of_responsibility {
            return Err(TortError::NoAssumptionOfResponsibility {
                context: "service provision".to_string(),
            });
        }
        if !facts.undertaking_skill_care {
            return Err(TortError::HedleyByrneNotMet {
                missing_requirement: "No undertaking to exercise skill and care".to_string(),
            });
        }
        if !facts.reliance {
            return Err(TortError::HedleyByrneNotMet {
                missing_requirement: "No reliance on undertaking".to_string(),
            });
        }
        // Contract may exclude if comprehensive
        if facts.contract_comprehensive {
            return Err(TortError::HedleyByrneNotMet {
                missing_requirement: "Contractual provisions comprehensive - tort claim excluded"
                    .to_string(),
            });
        }

        Ok(PureEconomicLossAnalysis {
            claim_type: self.claim_type.clone(),
            recognized_exception: true,
            hedley_byrne: None,
            extended_hedley_byrne: Some(ExtendedHedleyByrne {
                assumed_responsibility: true,
                undertaking: true,
                claimant_relied: true,
                contract_excludes: false,
                requirements_met: true,
            }),
            claim_succeeds: true,
            reasoning: "Extended Hedley Byrne (Henderson v Merrett) requirements satisfied"
                .to_string(),
        })
    }
}

/// Facts for pure economic loss analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EconomicLossFacts {
    /// Special relationship exists?
    pub special_relationship: bool,
    /// Assumption of responsibility?
    pub assumption_of_responsibility: bool,
    /// Reliance by claimant?
    pub reliance: bool,
    /// Was reliance reasonable?
    pub reasonable_reliance: bool,
    /// Did defendant know of reliance?
    pub defendant_knew_of_reliance: bool,
    /// Effective disclaimer?
    pub effective_disclaimer: bool,
    /// Undertaking to exercise skill and care?
    pub undertaking_skill_care: bool,
    /// Is contract comprehensive (excluding tort)?
    pub contract_comprehensive: bool,
}

// ============================================================================
// Defence Analyzer
// ============================================================================

/// Analyzer for negligence defences
#[derive(Debug, Clone)]
pub struct DefenceAnalyzer;

impl DefenceAnalyzer {
    /// Analyze contributory negligence
    pub fn analyze_contributory_negligence(
        facts: &ContributoryNegligenceFacts,
    ) -> ContributoryNegligence {
        let failed_reasonable_care = facts.claimant_unreasonable;
        let contributed_to_damage = facts.contributed_to_harm;
        let reduction = calculate_reduction_percentage(facts);

        ContributoryNegligence::analyze(
            facts.claimant_conduct.clone(),
            failed_reasonable_care,
            contributed_to_damage,
            reduction,
        )
    }

    /// Analyze volenti defence
    pub fn analyze_volenti(facts: &VolentiFacts) -> Volenti {
        // Check excluded contexts first
        let excluded = if facts.employment_context {
            Some(VolentiExclusion::Employment)
        } else if facts.road_traffic_context {
            Some(VolentiExclusion::RoadTraffic)
        } else if facts.rescue_context {
            Some(VolentiExclusion::Rescue)
        } else if facts.ucta_applies {
            Some(VolentiExclusion::UCTA)
        } else {
            None
        };

        let defence_succeeds = excluded.is_none()
            && facts.knowledge_of_risk
            && facts.genuine_consent
            && facts.freely_given
            && facts.consent_to_specific_risk;

        Volenti {
            knowledge_of_risk: facts.knowledge_of_risk,
            genuine_consent: facts.genuine_consent,
            freely_given: facts.freely_given,
            consent_to_particular_risk: facts.consent_to_specific_risk,
            excluded_context: excluded,
            defence_succeeds,
        }
    }
}

/// Facts for contributory negligence
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContributoryNegligenceFacts {
    /// Claimant's conduct
    pub claimant_conduct: String,
    /// Was claimant unreasonable?
    pub claimant_unreasonable: bool,
    /// Did it contribute to harm?
    pub contributed_to_harm: bool,
    /// Degree of fault (1-10)
    pub fault_degree: u8,
    /// Causative potency of claimant's act
    pub causative_potency: u8,
}

/// Facts for volenti defence
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VolentiFacts {
    /// Did claimant have knowledge of risk?
    pub knowledge_of_risk: bool,
    /// Was there genuine consent?
    pub genuine_consent: bool,
    /// Was consent freely given?
    pub freely_given: bool,
    /// Consent to specific risk that materialized?
    pub consent_to_specific_risk: bool,
    /// Employment context?
    pub employment_context: bool,
    /// Road traffic context?
    pub road_traffic_context: bool,
    /// Rescue context?
    pub rescue_context: bool,
    /// Does UCTA apply?
    pub ucta_applies: bool,
}

fn calculate_reduction_percentage(facts: &ContributoryNegligenceFacts) -> u8 {
    // Combine fault and causative potency
    let combined = (facts.fault_degree as u16 + facts.causative_potency as u16) / 2;
    (combined * 10).min(100) as u8
}

#[cfg(test)]
mod tests {
    use super::super::types::{DamageType, InjurySeverity, PartyRole, PartyType};
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_caparo_established_category() {
        let context = CaseContext {
            harm_type: HarmType::PhysicalInjury,
            novel_claim: false,
            relationship: "employer-employee".to_string(),
            professional_context: None,
            facts: "Worker injured on job".to_string(),
        };

        let analyzer = CaparoAnalyzer::new(context);
        assert!(analyzer.established_category.is_some());
        assert!(matches!(
            analyzer.established_category,
            Some(EstablishedDutyCategory::EmployerEmployee)
        ));
    }

    #[test]
    fn test_caparo_novel_duty() {
        let context = CaseContext {
            harm_type: HarmType::PureEconomicLoss,
            novel_claim: true,
            relationship: "strangers".to_string(),
            professional_context: None,
            facts: "Novel situation".to_string(),
        };

        let analyzer = CaparoAnalyzer::new(context);

        let foreseeability = ForeseeabilityFacts {
            known_risk: true,
            risk_obvious: true,
            previous_incidents: false,
            claimant_identifiable: true,
            class_foreseeable: true,
            manner_similar_to_known: true,
            highly_unusual_manner: false,
        };

        let proximity = ProximityFacts {
            physical_closeness: false,
            pre_existing_relationship: false,
            direct_cause: true,
            assumption_of_responsibility: false,
            reliance: false,
            vulnerable_claimant: false,
            known_reliance: false,
        };

        let policy = PolicyFacts {
            indeterminate_class: true,
            defensive_practices_concern: true,
            alternative_remedies: false,
            statutory_scheme: false,
            insurance_available: true,
            public_interest_in_activity: false,
            deterrence_value: true,
            morally_blameworthy: true,
        };

        let result = analyzer.analyze(&foreseeability, &proximity, &policy);
        // Should fail due to policy concerns
        assert!(!result.duty_exists);
    }

    #[test]
    fn test_breach_professional_bolam() {
        let analyzer = BreachAnalyzer::for_professional("surgeon".to_string());

        let facts = BreachFacts {
            defendant_conduct: "Performed surgery using standard technique".to_string(),
            risk_level: RiskLevel::Moderate,
            harm_gravity: HarmGravity::Serious,
            precaution_cost: CostLevel::Moderate,
            social_utility: SocialUtility::High,
            common_practice: None,
            relevant_factors: vec![],
            evidence: vec![],
            evidence_types: vec![],
            took_reasonable_precautions: true,
            followed_professional_practice: true,
            responsible_body_accepts: true,
            practice_logical: true,
            is_specialist: true,
            res_ipsa_facts: None,
        };

        let breach = analyzer.analyze_breach(&facts);
        assert!(!breach.fell_below_standard);
    }

    #[test]
    fn test_breach_bolitho_override() {
        let analyzer = BreachAnalyzer::for_professional("doctor".to_string());

        let facts = BreachFacts {
            defendant_conduct: "Followed practice but illogical".to_string(),
            risk_level: RiskLevel::High,
            harm_gravity: HarmGravity::Severe,
            precaution_cost: CostLevel::Low,
            social_utility: SocialUtility::Moderate,
            common_practice: None,
            relevant_factors: vec![],
            evidence: vec![],
            evidence_types: vec![],
            took_reasonable_precautions: false,
            followed_professional_practice: true,
            responsible_body_accepts: true,
            practice_logical: false, // Bolitho override
            is_specialist: false,
            res_ipsa_facts: None,
        };

        let breach = analyzer.analyze_breach(&facts);
        assert!(breach.fell_below_standard);
    }

    #[test]
    fn test_psychiatric_injury_primary_victim() {
        let analyzer = PsychiatricInjuryAnalyzer::for_primary_victim();

        let facts = PsychiatricInjuryFacts {
            recognized_illness: true,
            illness_name: "PTSD".to_string(),
            in_zone_of_danger: true,
            relationship: Relationship::Other("bystander".to_string()),
            evidence_of_close_tie: false,
            close_tie_evidence: None,
            witnessed_accident: true,
            witnessed_immediate_aftermath: false,
            time_delay: None,
            own_unaided_senses: true,
            work_related_stress: false,
            employer_knew_of_vulnerability: false,
            employer_could_prevent: false,
        };

        let result = analyzer.analyze(&facts);
        assert!(result.is_ok());
        assert!(result.expect("should succeed").claim_succeeds);
    }

    #[test]
    fn test_psychiatric_injury_secondary_victim_alcock() {
        let analyzer = PsychiatricInjuryAnalyzer::for_secondary_victim();

        let facts = PsychiatricInjuryFacts {
            recognized_illness: true,
            illness_name: "Clinical depression".to_string(),
            in_zone_of_danger: false,
            relationship: Relationship::ParentChild,
            evidence_of_close_tie: true,
            close_tie_evidence: Some("Parent of victim".to_string()),
            witnessed_accident: true,
            witnessed_immediate_aftermath: false,
            time_delay: None,
            own_unaided_senses: true,
            work_related_stress: false,
            employer_knew_of_vulnerability: false,
            employer_could_prevent: false,
        };

        let result = analyzer.analyze(&facts);
        assert!(result.is_ok());
    }

    #[test]
    fn test_psychiatric_injury_alcock_fails_no_proximity() {
        let analyzer = PsychiatricInjuryAnalyzer::for_secondary_victim();

        let facts = PsychiatricInjuryFacts {
            recognized_illness: true,
            illness_name: "PTSD".to_string(),
            in_zone_of_danger: false,
            relationship: Relationship::ParentChild,
            evidence_of_close_tie: true,
            close_tie_evidence: None,
            witnessed_accident: false,
            witnessed_immediate_aftermath: false, // Fails proximity
            time_delay: Some("8 hours later".to_string()),
            own_unaided_senses: true,
            work_related_stress: false,
            employer_knew_of_vulnerability: false,
            employer_could_prevent: false,
        };

        let result = analyzer.analyze(&facts);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(TortError::SecondaryVictimRequirementsNotMet { .. })
        ));
    }

    #[test]
    fn test_hedley_byrne_satisfied() {
        let analyzer = PureEconomicLossAnalyzer::for_negligent_misstatement();

        let facts = EconomicLossFacts {
            special_relationship: true,
            assumption_of_responsibility: true,
            reliance: true,
            reasonable_reliance: true,
            defendant_knew_of_reliance: true,
            effective_disclaimer: false,
            undertaking_skill_care: true,
            contract_comprehensive: false,
        };

        let result = analyzer.analyze(&facts);
        assert!(result.is_ok());
        assert!(result.expect("should succeed").claim_succeeds);
    }

    #[test]
    fn test_hedley_byrne_disclaimer() {
        let analyzer = PureEconomicLossAnalyzer::for_negligent_misstatement();

        let facts = EconomicLossFacts {
            special_relationship: true,
            assumption_of_responsibility: true,
            reliance: true,
            reasonable_reliance: true,
            defendant_knew_of_reliance: true,
            effective_disclaimer: true, // Effective disclaimer
            undertaking_skill_care: false,
            contract_comprehensive: false,
        };

        let result = analyzer.analyze(&facts);
        assert!(result.is_err());
    }

    #[test]
    fn test_murphy_v_brentwood_pure_economic() {
        let analyzer = PureEconomicLossAnalyzer::for_defective_product();

        let facts = EconomicLossFacts {
            special_relationship: false,
            assumption_of_responsibility: false,
            reliance: true,
            reasonable_reliance: true,
            defendant_knew_of_reliance: false,
            effective_disclaimer: false,
            undertaking_skill_care: false,
            contract_comprehensive: false,
        };

        let result = analyzer.analyze(&facts);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(TortError::PureEconomicLossNotRecoverable { .. })
        ));
    }

    #[test]
    fn test_contributory_negligence() {
        let facts = ContributoryNegligenceFacts {
            claimant_conduct: "Failed to wear seatbelt".to_string(),
            claimant_unreasonable: true,
            contributed_to_harm: true,
            fault_degree: 3,
            causative_potency: 5,
        };

        let cn = DefenceAnalyzer::analyze_contributory_negligence(&facts);
        assert!(cn.failed_reasonable_care);
        assert!(cn.contributed_to_damage);
        assert!(cn.reduction_percentage > 0);
    }

    #[test]
    fn test_volenti_employment_excluded() {
        let facts = VolentiFacts {
            knowledge_of_risk: true,
            genuine_consent: true,
            freely_given: true,
            consent_to_specific_risk: true,
            employment_context: true, // Excluded
            road_traffic_context: false,
            rescue_context: false,
            ucta_applies: false,
        };

        let volenti = DefenceAnalyzer::analyze_volenti(&facts);
        assert!(!volenti.defence_succeeds);
        assert!(matches!(
            volenti.excluded_context,
            Some(VolentiExclusion::Employment)
        ));
    }

    #[test]
    fn test_full_negligence_claim_succeeds() {
        let claimant = TortParty {
            name: "John Smith".to_string(),
            role: PartyRole::Claimant,
            party_type: PartyType::Individual,
            vulnerable: false,
            professional_capacity: None,
        };

        let defendant = TortParty {
            name: "ABC Ltd".to_string(),
            role: PartyRole::Defendant,
            party_type: PartyType::Company,
            vulnerable: false,
            professional_capacity: None,
        };

        let duty = DutyOfCareAnalysis {
            foreseeability: Foreseeability {
                harm_foreseeable: true,
                claimant_foreseeable: true,
                manner_foreseeable: true,
                reasoning: "Clear foreseeable harm".to_string(),
            },
            proximity: Proximity {
                proximity_type: ProximityType::ManufacturerConsumer,
                degree: 8,
                physical_proximity: true,
                circumstantial_proximity: true,
                causal_proximity: true,
                reasoning: "Direct relationship".to_string(),
            },
            fair_just_reasonable: FairJustReasonable {
                fair: true,
                just: true,
                reasonable: true,
                policy_considerations: vec![],
                overall: true,
                reasoning: "Fair to impose".to_string(),
            },
            established_category: Some(EstablishedDutyCategory::ManufacturerConsumer),
            novel_claim: false,
            duty_exists: true,
            reasoning: "Established category".to_string(),
        };

        let breach = BreachOfDuty {
            standard: StandardOfCare::reasonable_person(ReasonablePersonTest {
                magnitude_of_risk: RiskLevel::Moderate,
                gravity_of_harm: HarmGravity::Serious,
                cost_of_precautions: CostLevel::Low,
                social_utility: SocialUtility::Low,
                common_practice: None,
                factors: vec![],
            }),
            defendant_conduct: "Failed to inspect products".to_string(),
            fell_below_standard: true,
            evidence: vec![],
            res_ipsa_loquitur: None,
            reasoning: "Fell below standard".to_string(),
        };

        let causation = CausationAnalysis {
            factual_causation: FactualCausation {
                but_for_satisfied: true,
                material_contribution: None,
                material_increase_risk: None,
                loss_of_chance: None,
                multiple_sufficient_causes: false,
                reasoning: "But-for satisfied".to_string(),
            },
            legal_causation: LegalCausation {
                harm_type: HarmType::PhysicalInjury,
                type_foreseeable: true,
                extent_irrelevant: true,
                eggshell_skull: false,
                remoteness_satisfied: true,
                reasoning: "Not too remote".to_string(),
            },
            intervening_acts: vec![],
            causation_established: true,
        };

        let damage = Damage {
            damage_type: DamageType::PersonalInjury(InjurySeverity::Moderate),
            description: "Broken arm".to_string(),
            monetary_value: Some(25000.0),
            date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date")),
            continuing: false,
        };

        let claim = NegligenceClaimAnalysis::new(claimant, defendant)
            .with_duty_analysis(duty)
            .with_breach_analysis(breach)
            .with_causation_analysis(causation)
            .with_damage(damage);

        let result = claim.analyze();
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.claim_succeeds);
        assert!(analysis.quantum.is_some());
        assert!((analysis.quantum.expect("has quantum") - 25000.0).abs() < 0.01);
    }
}
