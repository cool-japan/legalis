//! UK Criminal Defences - General Defences
//!
//! This module covers general defences available in English criminal law,
//! including self-defence, duress, necessity, consent, and intoxication.
//!
//! # Categories of Defence
//!
//! ## Complete Defences
//! - Self-defence / Defence of another (common law + s.76 CJIA 2008)
//! - Prevention of crime (s.3 CLA 1967)
//! - Duress by threats (R v Hasan [2005])
//! - Duress of circumstances (R v Conway [1989])
//! - Necessity (limited recognition)
//! - Consent (limited application)
//! - Automatism (non-insane)
//! - Mistake (honest mistake as to facts)
//!
//! ## Limited Availability
//! - Intoxication (only for specific intent crimes)
//! - Duress (not available for murder/attempted murder)

// Allow missing docs on enum variant struct fields - these are self-documenting in context
#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use crate::criminal::error::{CriminalResult, DefenceError};
use crate::criminal::types::{CaseCitation, DefenceEffect, DefenceResult, DefenceType};

// ============================================================================
// Self-Defence
// ============================================================================

/// Facts for self-defence analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelfDefenceFacts {
    /// Did D honestly believe force was necessary?
    pub honest_belief: HonestBeliefFacts,
    /// Was force reasonable in circumstances as D believed them?
    pub reasonableness: ReasonablenessFacts,
    /// Type of defence claim
    pub defence_type: SelfDefenceType,
    /// Special circumstances (householder, etc.)
    pub special_circumstances: Option<SpecialCircumstances>,
}

/// Type of self-defence claim
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelfDefenceType {
    /// Defence of self
    SelfDefence,
    /// Defence of another
    DefenceOfAnother {
        /// Who was being defended
        who: String,
    },
    /// Prevention of crime (s.3 CLA 1967)
    PreventionOfCrime,
    /// Defence of property
    DefenceOfProperty,
}

/// Honest belief facts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HonestBeliefFacts {
    /// Did D honestly believe force was necessary?
    pub believed_force_necessary: bool,
    /// What did D believe about the circumstances?
    pub believed_circumstances: String,
    /// Did D believe there was an imminent threat?
    pub believed_imminent_threat: bool,
    /// Evidence of belief
    pub evidence: Vec<String>,
}

/// Reasonableness facts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReasonablenessFacts {
    /// What force was used?
    pub force_used: String,
    /// What threat did D perceive?
    pub perceived_threat: String,
    /// Was force proportionate to perceived threat?
    pub proportionate: bool,
    /// Was there opportunity to retreat? (not required but relevant)
    pub opportunity_retreat: Option<bool>,
    /// Was D acting in heat of moment?
    pub heat_of_moment: bool,
}

/// Special circumstances for self-defence
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SpecialCircumstances {
    /// Householder case (s.76(5A) CJIA 2008)
    Householder {
        /// Was D in a dwelling?
        in_dwelling: bool,
        /// Was V an intruder?
        intruder: bool,
    },
    /// Psychiatric condition affecting perception
    PsychiatricCondition {
        /// Description of condition
        condition: String,
    },
    /// Pre-emptive strike
    PreEmptiveStrike {
        /// Was the pre-emptive strike reasonable?
        reasonable: bool,
    },
}

/// Result of self-defence analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelfDefenceResult {
    /// Defence available?
    pub available: bool,
    /// Honest belief finding
    pub honest_belief_finding: String,
    /// Reasonableness finding
    pub reasonableness_finding: String,
    /// Special circumstances finding
    pub special_circumstances_finding: Option<String>,
    /// Key case law
    pub case_law: Vec<CaseCitation>,
}

/// Analyzer for self-defence
pub struct SelfDefenceAnalyzer;

impl SelfDefenceAnalyzer {
    /// Analyze self-defence claim
    pub fn analyze(facts: &SelfDefenceFacts) -> CriminalResult<SelfDefenceResult> {
        let mut case_law = vec![
            CaseCitation::new(
                "R v Williams (Gladstone)",
                1987,
                "3 All ER 411",
                "D judged on circumstances as honestly believed them to be",
            ),
            CaseCitation::new(
                "Palmer v R",
                1971,
                "AC 814",
                "Force must be reasonable in circumstances - not weighed to nicety",
            ),
        ];

        // Honest belief analysis
        let honest_belief_met = facts.honest_belief.believed_force_necessary
            && facts.honest_belief.believed_imminent_threat;

        let honest_belief_finding = if honest_belief_met {
            format!(
                "D honestly believed force was necessary: {}",
                facts.honest_belief.believed_circumstances
            )
        } else if !facts.honest_belief.believed_force_necessary {
            "D did not honestly believe force was necessary".to_string()
        } else {
            "D did not believe there was an imminent threat".to_string()
        };

        // Reasonableness analysis
        let reasonableness_finding = if facts.reasonableness.proportionate {
            format!(
                "Force ({}) was proportionate to perceived threat ({})",
                facts.reasonableness.force_used, facts.reasonableness.perceived_threat
            )
        } else {
            format!(
                "Force ({}) was disproportionate to threat ({})",
                facts.reasonableness.force_used, facts.reasonableness.perceived_threat
            )
        };

        // Special circumstances
        let special_circumstances_finding = facts.special_circumstances.as_ref().map(|sc| {
            match sc {
                SpecialCircumstances::Householder { in_dwelling, intruder } => {
                    if *in_dwelling && *intruder {
                        case_law.push(CaseCitation::new(
                            "s.76(5A) CJIA 2008",
                            2013,
                            "Statute",
                            "Householder cases: force not grossly disproportionate suffices",
                        ));
                        "Householder provisions apply: force need only be not grossly disproportionate".to_string()
                    } else {
                        "Householder provisions do not apply".to_string()
                    }
                }
                SpecialCircumstances::PsychiatricCondition { condition } => {
                    format!("Psychiatric condition ({}) may affect perception of threat", condition)
                }
                SpecialCircumstances::PreEmptiveStrike { reasonable } => {
                    if *reasonable {
                        "Pre-emptive strike: D need not wait to be hit first".to_string()
                    } else {
                        "Pre-emptive strike but not reasonably perceived as necessary".to_string()
                    }
                }
            }
        });

        // Check for householder lowered standard
        let householder_applies = matches!(
            &facts.special_circumstances,
            Some(SpecialCircumstances::Householder {
                in_dwelling: true,
                intruder: true
            })
        );

        let available =
            honest_belief_met && (facts.reasonableness.proportionate || householder_applies);

        Ok(SelfDefenceResult {
            available,
            honest_belief_finding,
            reasonableness_finding,
            special_circumstances_finding,
            case_law,
        })
    }
}

// ============================================================================
// Duress
// ============================================================================

/// Facts for duress analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DuressFacts {
    /// Type of duress
    pub duress_type: DuressType,
    /// The threat
    pub threat: ThreatDetails,
    /// D's response
    pub response: DuressResponse,
    /// The offence charged
    pub offence_charged: String,
    /// Voluntary association with criminals?
    pub voluntary_association: Option<VoluntaryAssociation>,
}

/// Type of duress
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DuressType {
    /// Duress by threats (another person's threats)
    ByThreats,
    /// Duress of circumstances (external circumstances)
    OfCircumstances,
}

/// Threat details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThreatDetails {
    /// Nature of threat
    pub threat: String,
    /// Against whom?
    pub against: ThreatTarget,
    /// Was threat of death or serious injury?
    pub death_or_serious_injury: bool,
    /// Was threat imminent?
    pub imminent: bool,
    /// Was there a nominated offence? (duress by threats)
    pub nominated_offence: Option<String>,
}

/// Target of threat
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThreatTarget {
    /// Threat to defendant
    Defendant,
    /// Threat to family member
    Family {
        /// Relationship to D
        relationship: String,
    },
    /// Threat to other person
    Other {
        /// Who was threatened
        who: String,
    },
}

/// D's response to duress
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DuressResponse {
    /// What did D do?
    pub action: String,
    /// Could D have escaped/evaded?
    pub evasion_possible: bool,
    /// Did D take opportunity to escape?
    pub attempted_escape: bool,
    /// Did D seek police protection?
    pub sought_police: bool,
}

/// Voluntary association with criminals
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoluntaryAssociation {
    /// Did D voluntarily associate?
    pub associated: bool,
    /// With whom?
    pub with_whom: String,
    /// Should D have foreseen risk of compulsion?
    pub foresaw_compulsion_risk: bool,
}

/// Result of duress analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DuressResult {
    /// Defence available?
    pub available: bool,
    /// Reason if unavailable
    pub unavailable_reason: Option<String>,
    /// Threat analysis
    pub threat_analysis: String,
    /// Response analysis
    pub response_analysis: String,
    /// Graham/Hasan test findings
    pub test_findings: DuressTestFindings,
    /// Key case law
    pub case_law: Vec<CaseCitation>,
}

/// Graham/Hasan test findings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DuressTestFindings {
    /// Subjective: Was D compelled to act?
    pub subjectively_compelled: bool,
    /// Objective: Would reasonable person have acted similarly?
    pub objectively_reasonable: bool,
    /// Immediacy: Was threat sufficiently imminent?
    pub immediacy_satisfied: bool,
}

/// Analyzer for duress
pub struct DuressAnalyzer;

impl DuressAnalyzer {
    /// Analyze duress defence
    pub fn analyze(facts: &DuressFacts) -> CriminalResult<DuressResult> {
        let mut case_law = vec![
            CaseCitation::new(
                "R v Graham",
                1982,
                "1 WLR 294",
                "Two-part test: subjective compulsion + objective reasonableness",
            ),
            CaseCitation::new(
                "R v Hasan",
                2005,
                "UKHL 22",
                "Strict limits on duress: immediacy, no evasion, no voluntary association",
            ),
        ];

        // Check if offence allows duress
        let offence_lower = facts.offence_charged.to_lowercase();
        if offence_lower.contains("murder") || offence_lower.contains("attempted murder") {
            return Ok(DuressResult {
                available: false,
                unavailable_reason: Some(
                    "Duress not available for murder or attempted murder (R v Howe)".into(),
                ),
                threat_analysis: String::new(),
                response_analysis: String::new(),
                test_findings: DuressTestFindings {
                    subjectively_compelled: false,
                    objectively_reasonable: false,
                    immediacy_satisfied: false,
                },
                case_law: vec![CaseCitation::new(
                    "R v Howe",
                    1987,
                    "AC 417",
                    "Duress not available for murder",
                )],
            });
        }

        // Analyze threat
        let threat_analysis = Self::analyze_threat(&facts.threat);

        // Check threat requirements
        if !facts.threat.death_or_serious_injury {
            return Ok(DuressResult {
                available: false,
                unavailable_reason: Some("Threat must be of death or serious injury".into()),
                threat_analysis,
                response_analysis: String::new(),
                test_findings: DuressTestFindings {
                    subjectively_compelled: false,
                    objectively_reasonable: false,
                    immediacy_satisfied: false,
                },
                case_law,
            });
        }

        // Analyze response
        let response_analysis = Self::analyze_response(&facts.response);

        // Check evasion (R v Hasan)
        if facts.response.evasion_possible && !facts.response.attempted_escape {
            return Ok(DuressResult {
                available: false,
                unavailable_reason: Some(
                    "D could have evaded threat but did not (R v Hasan)".into(),
                ),
                threat_analysis,
                response_analysis,
                test_findings: DuressTestFindings {
                    subjectively_compelled: true,
                    objectively_reasonable: false,
                    immediacy_satisfied: facts.threat.imminent,
                },
                case_law,
            });
        }

        // Check voluntary association
        if let Some(va) = &facts.voluntary_association {
            if va.associated && va.foresaw_compulsion_risk {
                case_law.push(CaseCitation::new(
                    "R v Sharp",
                    1987,
                    "QB 853",
                    "Duress unavailable where D voluntarily joins criminal gang",
                ));
                return Ok(DuressResult {
                    available: false,
                    unavailable_reason: Some(
                        "D voluntarily associated with criminals and foresaw risk of compulsion"
                            .into(),
                    ),
                    threat_analysis,
                    response_analysis,
                    test_findings: DuressTestFindings {
                        subjectively_compelled: true,
                        objectively_reasonable: false,
                        immediacy_satisfied: facts.threat.imminent,
                    },
                    case_law,
                });
            }
        }

        // Graham/Hasan test
        let test_findings = DuressTestFindings {
            subjectively_compelled: true,
            objectively_reasonable: true, // Assume if all checks passed
            immediacy_satisfied: facts.threat.imminent,
        };

        let available = test_findings.subjectively_compelled
            && test_findings.objectively_reasonable
            && test_findings.immediacy_satisfied;

        Ok(DuressResult {
            available,
            unavailable_reason: if !available {
                Some("Duress test not fully satisfied".into())
            } else {
                None
            },
            threat_analysis,
            response_analysis,
            test_findings,
            case_law,
        })
    }

    fn analyze_threat(facts: &ThreatDetails) -> String {
        let target = match &facts.against {
            ThreatTarget::Defendant => "defendant".to_string(),
            ThreatTarget::Family { relationship } => format!("{} (family)", relationship),
            ThreatTarget::Other { who } => format!("{} (other)", who),
        };

        format!(
            "Threat: {} against {}; Death/serious injury: {}; Imminent: {}",
            facts.threat,
            target,
            if facts.death_or_serious_injury {
                "yes"
            } else {
                "no"
            },
            if facts.imminent { "yes" } else { "no" }
        )
    }

    fn analyze_response(facts: &DuressResponse) -> String {
        let mut parts = vec![format!("D's action: {}", facts.action)];

        if facts.evasion_possible {
            parts.push(format!(
                "Evasion was possible; D {} attempt to escape",
                if facts.attempted_escape {
                    "did"
                } else {
                    "did not"
                }
            ));
        }

        if facts.sought_police {
            parts.push("D sought police protection".into());
        }

        parts.join("; ")
    }
}

// ============================================================================
// Intoxication
// ============================================================================

/// Facts for intoxication defence analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntoxicationFacts {
    /// Type of intoxication
    pub intoxication_type: IntoxicationType,
    /// Substance involved
    pub substance: String,
    /// Effect on D
    pub effect: IntoxicationEffect,
    /// Offence charged
    pub offence_charged: String,
    /// Is offence one of specific or basic intent?
    pub offence_intent_type: OffenceIntentType,
}

/// Type of intoxication
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntoxicationType {
    /// Voluntary intoxication (self-induced)
    Voluntary,
    /// Involuntary intoxication (spiked drink etc.)
    Involuntary,
    /// Intoxication by non-dangerous drugs taken as prescribed
    NonDangerousDrugs,
}

/// Effect of intoxication
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntoxicationEffect {
    /// Did intoxication prevent D forming mens rea?
    pub prevented_mens_rea: bool,
    /// Level of intoxication
    pub level: IntoxicationLevel,
    /// Effect on judgment/perception
    pub effect_description: String,
}

/// Level of intoxication
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntoxicationLevel {
    /// Mild
    Mild,
    /// Moderate
    Moderate,
    /// Severe
    Severe,
    /// Complete (automatism-like)
    Complete,
}

/// Type of offence for intoxication analysis
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OffenceIntentType {
    /// Specific intent (intoxication may negate mens rea)
    SpecificIntent,
    /// Basic intent (voluntary intoxication no defence)
    BasicIntent,
}

/// Result of intoxication analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntoxicationResult {
    /// Defence available?
    pub available: bool,
    /// Type of intoxication finding
    pub intoxication_finding: String,
    /// Intent type finding
    pub intent_finding: String,
    /// Effect on mens rea
    pub mens_rea_effect: String,
    /// Key case law
    pub case_law: Vec<CaseCitation>,
}

/// Analyzer for intoxication defence
pub struct IntoxicationAnalyzer;

impl IntoxicationAnalyzer {
    /// Analyze intoxication defence
    pub fn analyze(facts: &IntoxicationFacts) -> CriminalResult<IntoxicationResult> {
        let mut case_law = vec![CaseCitation::new(
            "DPP v Majewski",
            1977,
            "AC 443",
            "Voluntary intoxication no defence to basic intent crimes",
        )];

        let intoxication_finding = match facts.intoxication_type {
            IntoxicationType::Voluntary => "Voluntary intoxication".to_string(),
            IntoxicationType::Involuntary => {
                case_law.push(CaseCitation::new(
                    "R v Kingston",
                    1995,
                    "2 AC 355",
                    "Involuntary intoxication: still no defence if D formed mens rea",
                ));
                "Involuntary intoxication".to_string()
            }
            IntoxicationType::NonDangerousDrugs => {
                "Non-dangerous drugs taken as prescribed".to_string()
            }
        };

        let intent_finding = match facts.offence_intent_type {
            OffenceIntentType::SpecificIntent => {
                "Specific intent offence (e.g., murder, s.18 GBH, theft)".to_string()
            }
            OffenceIntentType::BasicIntent => {
                "Basic intent offence (e.g., manslaughter, s.20/47, assault)".to_string()
            }
        };

        // Determine availability
        let available = match (&facts.intoxication_type, &facts.offence_intent_type) {
            // Voluntary + specific intent = defence may apply if prevented mens rea
            (IntoxicationType::Voluntary, OffenceIntentType::SpecificIntent) => {
                facts.effect.prevented_mens_rea
            }
            // Voluntary + basic intent = no defence (Majewski rule)
            (IntoxicationType::Voluntary, OffenceIntentType::BasicIntent) => false,
            // Involuntary = defence if prevented mens rea (for either)
            (IntoxicationType::Involuntary, _) => facts.effect.prevented_mens_rea,
            // Non-dangerous drugs = defence available
            (IntoxicationType::NonDangerousDrugs, _) => facts.effect.prevented_mens_rea,
        };

        let mens_rea_effect = if facts.effect.prevented_mens_rea {
            format!(
                "Intoxication prevented D forming mens rea: {}",
                facts.effect.effect_description
            )
        } else {
            "D formed mens rea despite intoxication".to_string()
        };

        Ok(IntoxicationResult {
            available,
            intoxication_finding,
            intent_finding,
            mens_rea_effect,
            case_law,
        })
    }
}

// ============================================================================
// Automatism
// ============================================================================

/// Facts for automatism defence analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutomatismFacts {
    /// Type of automatism
    pub automatism_type: AutomatismType,
    /// Cause of automatism
    pub cause: AutomatismCause,
    /// Was conduct totally involuntary?
    pub total_loss_of_control: bool,
    /// Was automatism self-induced?
    pub self_induced: bool,
}

/// Type of automatism
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AutomatismType {
    /// Non-insane automatism (external cause)
    NonInsane,
    /// Insane automatism (disease of mind - goes to insanity)
    Insane,
}

/// Cause of automatism
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AutomatismCause {
    /// External physical force
    ExternalForce {
        /// Description of the external force
        description: String,
    },
    /// Reflex action
    Reflex,
    /// Hypoglycaemia (external - insulin)
    Hypoglycaemia,
    /// Hyperglycaemia (internal - goes to insanity)
    Hyperglycaemia,
    /// Concussion from external blow
    Concussion,
    /// Sleepwalking
    Sleepwalking,
    /// Post-traumatic stress
    PostTraumaticStress,
    /// Other external
    OtherExternal {
        /// Description of the external cause
        cause: String,
    },
}

/// Result of automatism analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutomatismResult {
    /// Defence available?
    pub available: bool,
    /// Type finding
    pub type_finding: String,
    /// Cause finding
    pub cause_finding: String,
    /// Self-induced finding
    pub self_induced_finding: Option<String>,
    /// Recommendation (insanity vs automatism)
    pub recommendation: String,
    /// Key case law
    pub case_law: Vec<CaseCitation>,
}

/// Analyzer for automatism
pub struct AutomatismAnalyzer;

impl AutomatismAnalyzer {
    /// Analyze automatism defence
    pub fn analyze(facts: &AutomatismFacts) -> CriminalResult<AutomatismResult> {
        let mut case_law = vec![
            CaseCitation::new(
                "Bratty v AG for NI",
                1963,
                "AC 386",
                "Automatism = total destruction of voluntary control",
            ),
            CaseCitation::new(
                "R v Quick",
                1973,
                "QB 910",
                "External cause (insulin) = automatism; internal cause = insanity",
            ),
        ];

        // Check total loss of control
        if !facts.total_loss_of_control {
            return Ok(AutomatismResult {
                available: false,
                type_finding: "Impaired control only".into(),
                cause_finding: String::new(),
                self_induced_finding: None,
                recommendation: "Automatism requires total loss of voluntary control".into(),
                case_law,
            });
        }

        // Determine internal vs external
        let (is_external, cause_finding) = match &facts.cause {
            AutomatismCause::ExternalForce { description } => {
                (true, format!("External force: {}", description))
            }
            AutomatismCause::Reflex => (true, "Reflex action".into()),
            AutomatismCause::Hypoglycaemia => {
                (true, "Hypoglycaemia (external cause - insulin)".into())
            }
            AutomatismCause::Hyperglycaemia => {
                (false, "Hyperglycaemia (internal cause - disease)".into())
            }
            AutomatismCause::Concussion => (true, "Concussion from external blow".into()),
            AutomatismCause::Sleepwalking => {
                case_law.push(CaseCitation::new(
                    "R v Burgess",
                    1991,
                    "2 QB 92",
                    "Sleepwalking = insane automatism (internal)",
                ));
                (
                    false,
                    "Sleepwalking (internal cause per R v Burgess)".into(),
                )
            }
            AutomatismCause::PostTraumaticStress => {
                (true, "Post-traumatic stress (external cause)".into())
            }
            AutomatismCause::OtherExternal { cause } => {
                (true, format!("External cause: {}", cause))
            }
        };

        // Type finding
        let type_finding = if is_external {
            "Non-insane automatism (external cause)".to_string()
        } else {
            "Insane automatism (internal cause - should be insanity defence)".to_string()
        };

        // Self-induced finding
        let self_induced_finding = if facts.self_induced {
            case_law.push(CaseCitation::new(
                "R v Bailey",
                1983,
                "1 WLR 760",
                "Self-induced automatism: no defence to basic intent if risk foreseeable",
            ));
            Some("Self-induced: may not be defence to basic intent crimes".into())
        } else {
            None
        };

        let available = is_external && !facts.self_induced;

        let recommendation = if !is_external {
            "Consider insanity defence (M'Naghten Rules)".into()
        } else if facts.self_induced {
            "Self-induced automatism: may still be defence to specific intent crimes".into()
        } else {
            "Non-insane automatism: complete defence if established".into()
        };

        Ok(AutomatismResult {
            available,
            type_finding,
            cause_finding,
            self_induced_finding,
            recommendation,
            case_law,
        })
    }
}

// ============================================================================
// Mistake
// ============================================================================

/// Facts for mistake defence analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MistakeFacts {
    /// What was D mistaken about?
    pub mistake: MistakeDetails,
    /// Was mistake honest?
    pub honest: bool,
    /// Was mistake reasonable?
    pub reasonable: bool,
    /// How did mistake affect D's conduct?
    pub effect: String,
}

/// Details of mistake
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MistakeDetails {
    /// Description of mistake
    pub description: String,
    /// Type of mistake
    pub mistake_type: MistakeType,
}

/// Type of mistake
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MistakeType {
    /// Mistake as to fact
    Fact,
    /// Mistake as to law (generally no defence)
    Law,
    /// Mistake as to defence (e.g., believed being attacked)
    Defence,
    /// Mistake as to consent
    Consent,
}

/// Result of mistake analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MistakeResult {
    /// Defence available?
    pub available: bool,
    /// Mistake finding
    pub mistake_finding: String,
    /// Honesty finding
    pub honesty_finding: String,
    /// Key case law
    pub case_law: Vec<CaseCitation>,
}

/// Analyzer for mistake defence
pub struct MistakeAnalyzer;

impl MistakeAnalyzer {
    /// Analyze mistake defence
    pub fn analyze(facts: &MistakeFacts) -> CriminalResult<MistakeResult> {
        let case_law = vec![
            CaseCitation::new(
                "DPP v Morgan",
                1976,
                "AC 182",
                "Honest mistake negates mens rea (even if unreasonable)",
            ),
            CaseCitation::new(
                "R v Williams (Gladstone)",
                1987,
                "3 All ER 411",
                "Mistake as to facts founding defence: judged on D's honest belief",
            ),
        ];

        let available = match facts.mistake.mistake_type {
            MistakeType::Fact => facts.honest,
            MistakeType::Law => false, // Ignorance of law no defence
            MistakeType::Defence => facts.honest,
            MistakeType::Consent => facts.honest,
        };

        let mistake_finding = match facts.mistake.mistake_type {
            MistakeType::Fact => {
                format!("Mistake of fact: {}", facts.mistake.description)
            }
            MistakeType::Law => "Mistake of law: generally no defence".to_string(),
            MistakeType::Defence => {
                format!(
                    "Mistake relating to defence: {} (R v Williams)",
                    facts.mistake.description
                )
            }
            MistakeType::Consent => {
                format!("Mistake as to consent: {}", facts.mistake.description)
            }
        };

        let honesty_finding = if facts.honest {
            format!(
                "Mistake was honest{}",
                if facts.reasonable {
                    " and reasonable"
                } else {
                    " (reasonableness not required per DPP v Morgan)"
                }
            )
        } else {
            "Mistake was not honestly held".to_string()
        };

        Ok(MistakeResult {
            available,
            mistake_finding,
            honesty_finding,
            case_law,
        })
    }
}

// ============================================================================
// Consent
// ============================================================================

/// Facts for consent defence analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsentDefenceFacts {
    /// Was consent given?
    pub consent_given: bool,
    /// Level of harm
    pub harm_level: ConsentHarmLevel,
    /// Type of activity
    pub activity: ConsentActivityType,
    /// Was consent valid?
    pub validity: ConsentValidity,
}

/// Level of harm for consent
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsentHarmLevel {
    /// No harm / minimal
    Minimal,
    /// Actual bodily harm
    ABH,
    /// Grievous bodily harm
    GBH,
    /// Death
    Death,
}

/// Activity type for consent
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsentActivityType {
    /// Lawful sporting activity
    Sport {
        /// Name of the sport
        sport: String,
    },
    /// Horseplay
    Horseplay,
    /// Reasonable surgical interference
    Medical,
    /// Sexual activity
    Sexual,
    /// Tattooing/piercing
    BodyModification,
    /// Fighting for entertainment
    Fighting,
    /// Sadomasochism
    Sadomasochism,
    /// Other
    Other {
        /// Description of the activity
        description: String,
    },
}

/// Consent validity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsentValidity {
    /// Was consent informed?
    pub informed: bool,
    /// Was consent freely given?
    pub freely_given: bool,
    /// Did V have capacity?
    pub capacity: bool,
    /// Was consent obtained by fraud?
    pub fraud: Option<ConsentFraud>,
}

/// Fraud affecting consent
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsentFraud {
    /// Fraud as to nature of act
    NatureOfAct,
    /// Fraud as to identity
    Identity,
    /// Other fraud (may not vitiate consent)
    Other {
        /// Description of the fraud
        description: String,
    },
}

/// Result of consent defence analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsentDefenceResult {
    /// Defence available?
    pub available: bool,
    /// Harm level finding
    pub harm_finding: String,
    /// Activity finding
    pub activity_finding: String,
    /// Validity finding
    pub validity_finding: String,
    /// Key case law
    pub case_law: Vec<CaseCitation>,
}

/// Analyzer for consent defence
pub struct ConsentDefenceAnalyzer;

impl ConsentDefenceAnalyzer {
    /// Analyze consent defence
    pub fn analyze(facts: &ConsentDefenceFacts) -> CriminalResult<ConsentDefenceResult> {
        let mut case_law = vec![CaseCitation::new(
            "R v Brown",
            1994,
            "1 AC 212",
            "Consent no defence to intentional infliction of harm except recognized categories",
        )];

        if !facts.consent_given {
            return Ok(ConsentDefenceResult {
                available: false,
                harm_finding: String::new(),
                activity_finding: String::new(),
                validity_finding: "No consent given".into(),
                case_law,
            });
        }

        // Check validity
        let validity_finding = Self::analyze_validity(&facts.validity)?;
        if !facts.validity.informed || !facts.validity.freely_given || !facts.validity.capacity {
            return Ok(ConsentDefenceResult {
                available: false,
                harm_finding: String::new(),
                activity_finding: String::new(),
                validity_finding,
                case_law,
            });
        }

        // Check if fraud vitiates consent
        if let Some(fraud) = &facts.validity.fraud {
            if matches!(fraud, ConsentFraud::NatureOfAct | ConsentFraud::Identity) {
                return Ok(ConsentDefenceResult {
                    available: false,
                    harm_finding: String::new(),
                    activity_finding: String::new(),
                    validity_finding: format!("Consent vitiated by fraud: {:?}", fraud),
                    case_law,
                });
            }
        }

        // Check recognized categories
        let activity_finding = Self::analyze_activity(&facts.activity, &mut case_law);

        // Check harm level vs activity
        let (available, harm_finding) =
            Self::check_harm_limits(&facts.harm_level, &facts.activity, &mut case_law);

        Ok(ConsentDefenceResult {
            available,
            harm_finding,
            activity_finding,
            validity_finding,
            case_law,
        })
    }

    fn analyze_validity(validity: &ConsentValidity) -> CriminalResult<String> {
        let mut parts = Vec::new();

        if validity.informed {
            parts.push("Consent was informed");
        } else {
            parts.push("Consent was NOT informed");
        }

        if validity.freely_given {
            parts.push("freely given");
        } else {
            parts.push("NOT freely given");
        }

        if validity.capacity {
            parts.push("V had capacity");
        } else {
            parts.push("V lacked capacity");
        }

        Ok(parts.join("; "))
    }

    fn analyze_activity(
        activity: &ConsentActivityType,
        case_law: &mut Vec<CaseCitation>,
    ) -> String {
        match activity {
            ConsentActivityType::Sport { sport } => {
                format!("Lawful sporting activity ({}) - recognized category", sport)
            }
            ConsentActivityType::Horseplay => {
                case_law.push(CaseCitation::new(
                    "R v Jones",
                    1986,
                    "83 Cr App R 375",
                    "Horseplay is recognized category for consent",
                ));
                "Horseplay - recognized category".into()
            }
            ConsentActivityType::Medical => {
                "Reasonable surgical interference - recognized category".into()
            }
            ConsentActivityType::BodyModification => {
                "Tattooing/piercing - recognized category".into()
            }
            ConsentActivityType::Sadomasochism => {
                "Sadomasochism - NOT recognized category (R v Brown)".into()
            }
            ConsentActivityType::Fighting => "Fighting - NOT generally recognized".into(),
            _ => {
                format!("Activity: {:?}", activity)
            }
        }
    }

    fn check_harm_limits(
        harm: &ConsentHarmLevel,
        activity: &ConsentActivityType,
        _case_law: &mut Vec<CaseCitation>,
    ) -> (bool, String) {
        match (harm, activity) {
            // Minimal harm - consent generally valid
            (ConsentHarmLevel::Minimal, _) => (true, "Minimal harm - consent valid".into()),

            // ABH in recognized categories - may be valid
            (ConsentHarmLevel::ABH, ConsentActivityType::Sport { .. }) => (
                true,
                "ABH in sporting context - consent may be valid within rules".into(),
            ),
            (ConsentHarmLevel::ABH, ConsentActivityType::Medical) => {
                (true, "ABH in medical context - consent valid".into())
            }
            (ConsentHarmLevel::ABH, ConsentActivityType::BodyModification) => {
                (true, "ABH for body modification - consent valid".into())
            }

            // GBH - generally consent no defence
            (ConsentHarmLevel::GBH, ConsentActivityType::Medical) => (
                true,
                "GBH in medical context (surgery) - consent valid".into(),
            ),
            (ConsentHarmLevel::GBH, _) => (
                false,
                "GBH - consent generally no defence (R v Brown)".into(),
            ),

            // Death - consent never defence
            (ConsentHarmLevel::Death, _) => (false, "Death - consent never a defence".into()),

            // ABH in non-recognized categories
            (ConsentHarmLevel::ABH, _) => (
                false,
                "ABH outside recognized categories - consent no defence".into(),
            ),
        }
    }
}

// ============================================================================
// Unified Defence Analysis
// ============================================================================

/// Analyze any general defence
pub fn analyze_defence(
    defence_type: DefenceType,
    facts: &serde_json::Value,
) -> CriminalResult<DefenceResult> {
    match defence_type {
        DefenceType::SelfDefence | DefenceType::PreventionOfCrime => {
            let facts: SelfDefenceFacts = serde_json::from_value(facts.clone())
                .map_err(|e| crate::criminal::error::CriminalError::InvalidInput(e.to_string()))?;
            let result = SelfDefenceAnalyzer::analyze(&facts)?;
            Ok(DefenceResult {
                defence_type,
                available: result.available,
                effect: if result.available {
                    Some(DefenceEffect::Acquittal)
                } else {
                    None
                },
                findings: vec![result.honest_belief_finding, result.reasonableness_finding],
                case_law: result.case_law,
            })
        }
        DefenceType::DuressByThreats | DefenceType::DuressCircumstances => {
            let facts: DuressFacts = serde_json::from_value(facts.clone())
                .map_err(|e| crate::criminal::error::CriminalError::InvalidInput(e.to_string()))?;
            let result = DuressAnalyzer::analyze(&facts)?;
            Ok(DefenceResult {
                defence_type,
                available: result.available,
                effect: if result.available {
                    Some(DefenceEffect::Acquittal)
                } else {
                    None
                },
                findings: vec![result.threat_analysis, result.response_analysis],
                case_law: result.case_law,
            })
        }
        DefenceType::Intoxication => {
            let facts: IntoxicationFacts = serde_json::from_value(facts.clone())
                .map_err(|e| crate::criminal::error::CriminalError::InvalidInput(e.to_string()))?;
            let result = IntoxicationAnalyzer::analyze(&facts)?;
            Ok(DefenceResult {
                defence_type,
                available: result.available,
                effect: if result.available {
                    Some(DefenceEffect::Acquittal)
                } else {
                    None
                },
                findings: vec![
                    result.intoxication_finding,
                    result.intent_finding,
                    result.mens_rea_effect,
                ],
                case_law: result.case_law,
            })
        }
        DefenceType::Automatism => {
            let facts: AutomatismFacts = serde_json::from_value(facts.clone())
                .map_err(|e| crate::criminal::error::CriminalError::InvalidInput(e.to_string()))?;
            let result = AutomatismAnalyzer::analyze(&facts)?;
            Ok(DefenceResult {
                defence_type,
                available: result.available,
                effect: if result.available {
                    Some(DefenceEffect::Acquittal)
                } else {
                    None
                },
                findings: vec![result.type_finding, result.cause_finding],
                case_law: result.case_law,
            })
        }
        DefenceType::Mistake => {
            let facts: MistakeFacts = serde_json::from_value(facts.clone())
                .map_err(|e| crate::criminal::error::CriminalError::InvalidInput(e.to_string()))?;
            let result = MistakeAnalyzer::analyze(&facts)?;
            Ok(DefenceResult {
                defence_type,
                available: result.available,
                effect: if result.available {
                    Some(DefenceEffect::Acquittal)
                } else {
                    None
                },
                findings: vec![result.mistake_finding, result.honesty_finding],
                case_law: result.case_law,
            })
        }
        DefenceType::Consent => {
            let facts: ConsentDefenceFacts = serde_json::from_value(facts.clone())
                .map_err(|e| crate::criminal::error::CriminalError::InvalidInput(e.to_string()))?;
            let result = ConsentDefenceAnalyzer::analyze(&facts)?;
            Ok(DefenceResult {
                defence_type,
                available: result.available,
                effect: if result.available {
                    Some(DefenceEffect::Acquittal)
                } else {
                    None
                },
                findings: vec![
                    result.harm_finding,
                    result.activity_finding,
                    result.validity_finding,
                ],
                case_law: result.case_law,
            })
        }
        _ => Err(crate::criminal::error::CriminalError::Defence(
            DefenceError::NotAvailable {
                defence: format!("{:?}", defence_type),
                offence: "unknown".into(),
                reason: "Defence analysis not implemented".into(),
            },
        )),
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self_defence_basic() {
        let facts = SelfDefenceFacts {
            honest_belief: HonestBeliefFacts {
                believed_force_necessary: true,
                believed_circumstances: "Attacker raised fist to punch".into(),
                believed_imminent_threat: true,
                evidence: vec!["Witness testimony".into()],
            },
            reasonableness: ReasonablenessFacts {
                force_used: "Single push".into(),
                perceived_threat: "Punch to face".into(),
                proportionate: true,
                opportunity_retreat: Some(false),
                heat_of_moment: true,
            },
            defence_type: SelfDefenceType::SelfDefence,
            special_circumstances: None,
        };

        let result = SelfDefenceAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.available);
    }

    #[test]
    fn test_householder_defence() {
        let facts = SelfDefenceFacts {
            honest_belief: HonestBeliefFacts {
                believed_force_necessary: true,
                believed_circumstances: "Intruder in home at night".into(),
                believed_imminent_threat: true,
                evidence: vec![],
            },
            reasonableness: ReasonablenessFacts {
                force_used: "Hit intruder with bat".into(),
                perceived_threat: "Home invasion".into(),
                proportionate: false, // Would normally fail
                opportunity_retreat: None,
                heat_of_moment: true,
            },
            defence_type: SelfDefenceType::SelfDefence,
            special_circumstances: Some(SpecialCircumstances::Householder {
                in_dwelling: true,
                intruder: true,
            }),
        };

        let result = SelfDefenceAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        // Householder provisions: need only not be grossly disproportionate
        assert!(analysis.available);
    }

    #[test]
    fn test_duress_unavailable_for_murder() {
        let facts = DuressFacts {
            duress_type: DuressType::ByThreats,
            threat: ThreatDetails {
                threat: "Will kill your family".into(),
                against: ThreatTarget::Family {
                    relationship: "children".into(),
                },
                death_or_serious_injury: true,
                imminent: true,
                nominated_offence: Some("Kill the victim".into()),
            },
            response: DuressResponse {
                action: "Killed victim".into(),
                evasion_possible: false,
                attempted_escape: false,
                sought_police: false,
            },
            offence_charged: "Murder".into(),
            voluntary_association: None,
        };

        let result = DuressAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(!analysis.available);
        assert!(
            analysis
                .unavailable_reason
                .as_ref()
                .map(|r| r.contains("murder"))
                .unwrap_or(false)
        );
    }

    #[test]
    fn test_voluntary_intoxication_specific_intent() {
        let facts = IntoxicationFacts {
            intoxication_type: IntoxicationType::Voluntary,
            substance: "Alcohol".into(),
            effect: IntoxicationEffect {
                prevented_mens_rea: true,
                level: IntoxicationLevel::Severe,
                effect_description: "Unable to form intent to kill".into(),
            },
            offence_charged: "Murder".into(),
            offence_intent_type: OffenceIntentType::SpecificIntent,
        };

        let result = IntoxicationAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        // Voluntary intoxication can be defence to specific intent
        assert!(analysis.available);
    }

    #[test]
    fn test_voluntary_intoxication_basic_intent() {
        let facts = IntoxicationFacts {
            intoxication_type: IntoxicationType::Voluntary,
            substance: "Alcohol".into(),
            effect: IntoxicationEffect {
                prevented_mens_rea: true,
                level: IntoxicationLevel::Severe,
                effect_description: "Unable to control actions".into(),
            },
            offence_charged: "s.20 GBH".into(),
            offence_intent_type: OffenceIntentType::BasicIntent,
        };

        let result = IntoxicationAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        // Voluntary intoxication NO defence to basic intent (Majewski)
        assert!(!analysis.available);
    }

    #[test]
    fn test_automatism_external_cause() {
        let facts = AutomatismFacts {
            automatism_type: AutomatismType::NonInsane,
            cause: AutomatismCause::Hypoglycaemia,
            total_loss_of_control: true,
            self_induced: false,
        };

        let result = AutomatismAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.available);
    }

    #[test]
    fn test_automatism_internal_cause() {
        let facts = AutomatismFacts {
            automatism_type: AutomatismType::Insane,
            cause: AutomatismCause::Hyperglycaemia,
            total_loss_of_control: true,
            self_induced: false,
        };

        let result = AutomatismAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        // Internal cause = insanity, not automatism
        assert!(!analysis.available);
        assert!(analysis.recommendation.contains("insanity"));
    }

    #[test]
    fn test_consent_r_v_brown() {
        let facts = ConsentDefenceFacts {
            consent_given: true,
            harm_level: ConsentHarmLevel::GBH,
            activity: ConsentActivityType::Sadomasochism,
            validity: ConsentValidity {
                informed: true,
                freely_given: true,
                capacity: true,
                fraud: None,
            },
        };

        let result = ConsentDefenceAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        // R v Brown: consent no defence to sadomasochism causing GBH
        assert!(!analysis.available);
    }

    #[test]
    fn test_consent_sport() {
        let facts = ConsentDefenceFacts {
            consent_given: true,
            harm_level: ConsentHarmLevel::ABH,
            activity: ConsentActivityType::Sport {
                sport: "Rugby".into(),
            },
            validity: ConsentValidity {
                informed: true,
                freely_given: true,
                capacity: true,
                fraud: None,
            },
        };

        let result = ConsentDefenceAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        // ABH in sporting context - consent may be valid
        assert!(analysis.available);
    }

    #[test]
    fn test_mistake_honest() {
        let facts = MistakeFacts {
            mistake: MistakeDetails {
                description: "Believed victim was attacker".into(),
                mistake_type: MistakeType::Defence,
            },
            honest: true,
            reasonable: false, // Reasonableness not required
            effect: "Used force in mistaken self-defence".into(),
        };

        let result = MistakeAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        // Honest mistake sufficient (DPP v Morgan)
        assert!(analysis.available);
    }
}
