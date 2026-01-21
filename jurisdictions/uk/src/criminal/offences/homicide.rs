//! UK Homicide Offences
//!
//! This module provides comprehensive analysis of homicide offences under English law,
//! including murder, manslaughter (voluntary and involuntary), and corporate manslaughter.
//!
//! # Legal Framework
//!
//! ## Murder (Common Law)
//! - Definition: Unlawful killing with malice aforethought (intent to kill or cause GBH)
//! - Sentence: Mandatory life imprisonment
//! - Key cases: R v Woollin \[1999\] (oblique intention), R v Cunningham \[1982\] (GBH rule)
//!
//! ## Voluntary Manslaughter
//! - Murder reduced by partial defence:
//!   - Loss of control (Coroners and Justice Act 2009)
//!   - Diminished responsibility (s.2 Homicide Act 1957, as amended)
//!   - Suicide pact (s.4 Homicide Act 1957)
//!
//! ## Involuntary Manslaughter
//! - Unlawful Act Manslaughter (R v Church \[1966\], R v Newbury & Jones \[1977\])
//! - Gross Negligence Manslaughter (R v Adomako \[1995\])
//! - Reckless Manslaughter (rarely charged separately)
//!
//! ## Corporate Manslaughter
//! - Corporate Manslaughter and Corporate Homicide Act 2007

// Allow missing docs on enum variant struct fields - these are self-documenting in context
#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use crate::criminal::error::{CriminalError, CriminalResult};
use crate::criminal::types::{
    ActType, ActusReusElement, CaseCitation, CausationAnalysis, DefenceEffect, DefenceResult,
    DefenceType, FactualCausation, InterveningAct, InterveningActType, LegalCausation,
    MaximumSentence, MensReaAnalysis, MensReaType, Offence, OffenceBuilder, OffenceCategory,
    OffenceClassification, OffenceSeverity,
};

// ============================================================================
// Murder Analysis
// ============================================================================

/// Facts for murder analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MurderFacts {
    /// Was there a killing (death of victim)?
    pub death_occurred: bool,
    /// Victim details
    pub victim: VictimDetails,
    /// Defendant's conduct
    pub defendant_conduct: DefendantConduct,
    /// Intent evidence
    pub intent_evidence: IntentEvidence,
    /// Causation facts
    pub causation_facts: CausationFacts,
    /// Was the killing under Queen's Peace?
    pub under_queens_peace: bool,
    /// Potential partial defences
    pub partial_defences: Option<PartialDefenceFacts>,
}

/// Details about the victim
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VictimDetails {
    /// Was victim a "reasonable creature in being"?
    pub reasonable_creature: bool,
    /// Any pre-existing conditions (thin skull rule)
    pub pre_existing_conditions: Vec<String>,
    /// Victim's age (relevant for sentencing)
    pub age: Option<u32>,
    /// Victim's vulnerability
    pub vulnerability: Option<VictimVulnerability>,
}

/// Victim vulnerability for sentencing
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VictimVulnerability {
    /// Child victim
    Child,
    /// Elderly victim
    Elderly,
    /// Disability
    Disability,
    /// Relationship of trust
    TrustRelationship,
    /// Other vulnerability
    Other(String),
}

/// Defendant's conduct
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DefendantConduct {
    /// Type of conduct (act or omission)
    pub conduct_type: ConductType,
    /// Description of the conduct
    pub description: String,
    /// Weapon used (if any)
    pub weapon: Option<WeaponUsed>,
    /// Was there planning/premeditation?
    pub premeditation: Option<PremediationEvidence>,
}

/// Type of conduct
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConductType {
    /// Positive act
    Act,
    /// Omission (requires duty)
    Omission { duty_source: OmissionDutySource },
}

/// Source of omission duty
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OmissionDutySource {
    /// Statutory duty
    Statutory,
    /// Contractual duty
    Contract,
    /// Family relationship
    Relationship,
    /// Voluntary assumption of care
    Assumption,
    /// Creating dangerous situation
    DangerousSituation,
}

/// Weapon used
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeaponUsed {
    /// Type of weapon
    pub weapon_type: WeaponType,
    /// Was weapon brought to scene?
    pub brought_to_scene: bool,
}

/// Types of weapons
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeaponType {
    /// Firearm
    Firearm,
    /// Knife or bladed article
    Knife,
    /// Blunt instrument
    BluntInstrument,
    /// Vehicle
    Vehicle,
    /// Ligature
    Ligature,
    /// Hands/feet (no weapon)
    BodyOnly,
    /// Poison
    Poison,
    /// Fire/arson
    Fire,
    /// Other
    Other(String),
}

/// Evidence of premeditation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PremediationEvidence {
    /// Was there planning?
    pub planning: bool,
    /// Evidence of planning
    pub planning_evidence: Vec<String>,
    /// Motive evidence
    pub motive: Option<String>,
}

/// Evidence relating to intent
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntentEvidence {
    /// Evidence suggesting direct intention to kill
    pub direct_intent_kill: Vec<String>,
    /// Evidence suggesting direct intention to cause GBH
    pub direct_intent_gbh: Vec<String>,
    /// Evidence suggesting oblique intention (virtual certainty)
    pub oblique_intent_evidence: Option<ObliqueIntentEvidence>,
    /// Defendant's statements
    pub defendant_statements: Vec<String>,
}

/// Evidence for oblique intention (Woollin direction)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObliqueIntentEvidence {
    /// Was death/GBH virtually certain to result?
    pub virtually_certain: bool,
    /// Did defendant appreciate this?
    pub defendant_appreciated: bool,
    /// Supporting evidence
    pub evidence: Vec<String>,
}

/// Causation facts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CausationFacts {
    /// "But for" analysis
    pub but_for_satisfied: bool,
    /// Was D's act operating and substantial cause?
    pub operating_substantial: bool,
    /// Any intervening acts
    pub intervening_acts: Vec<InterveningActDetails>,
    /// Thin skull situation?
    pub thin_skull: Option<ThinSkullDetails>,
}

/// Details of intervening act
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterveningActDetails {
    /// Description of the act
    pub description: String,
    /// Type of intervening act
    pub act_type: InterveningActType,
    /// For medical treatment: was it palpably wrong?
    pub palpably_wrong: Option<bool>,
    /// For victim acts: was it reasonably foreseeable?
    pub foreseeable: Option<bool>,
}

/// Thin skull details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThinSkullDetails {
    /// The pre-existing condition
    pub condition: String,
    /// Did it contribute to death?
    pub contributed: bool,
}

/// Partial defence facts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartialDefenceFacts {
    /// Loss of control facts
    pub loss_of_control: Option<LossOfControlFacts>,
    /// Diminished responsibility facts
    pub diminished_responsibility: Option<DiminishedResponsibilityFacts>,
    /// Suicide pact facts
    pub suicide_pact: Option<SuicidePactFacts>,
}

/// Loss of control facts (Coroners and Justice Act 2009)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LossOfControlFacts {
    /// Did D lose self-control?
    pub lost_control: bool,
    /// Evidence of loss of control
    pub loss_evidence: Vec<String>,
    /// Qualifying trigger (s.55)
    pub qualifying_trigger: Option<QualifyingTrigger>,
    /// Would person of D's age/sex with normal tolerance have reacted same way?
    pub normal_tolerance_test: Option<NormalToleranceAssessment>,
}

/// Qualifying triggers for loss of control
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QualifyingTrigger {
    /// Fear of violence trigger (s.55(3))
    FearOfViolence {
        fear_from: String,
        evidence: Vec<String>,
    },
    /// Anger trigger - circumstances of extremely grave character (s.55(4))
    Anger {
        circumstances: String,
        justifiable_sense_of_being_wronged: bool,
        evidence: Vec<String>,
    },
    /// Combination of both
    Combined {
        fear_element: String,
        anger_element: String,
        evidence: Vec<String>,
    },
}

/// Normal tolerance test assessment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NormalToleranceAssessment {
    /// Defendant's age
    pub defendant_age: u32,
    /// Defendant's sex
    pub defendant_sex: String,
    /// Relevant circumstances to consider
    pub relevant_circumstances: Vec<String>,
    /// Would person of normal tolerance have reacted similarly?
    pub normal_person_would_react: bool,
}

/// Diminished responsibility facts (s.2 Homicide Act 1957)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiminishedResponsibilityFacts {
    /// Abnormality of mental functioning?
    pub abnormality: bool,
    /// Arose from recognized medical condition?
    pub recognized_condition: Option<String>,
    /// Did it substantially impair ability to:
    pub impairments: DiminishedImpairments,
    /// Was it significant factor in D's acts?
    pub significant_factor: bool,
    /// Expert psychiatric evidence
    pub psychiatric_evidence: Vec<String>,
}

/// Impairments for diminished responsibility
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiminishedImpairments {
    /// Impaired ability to understand nature of conduct
    pub understand_conduct: bool,
    /// Impaired ability to form rational judgment
    pub form_rational_judgment: bool,
    /// Impaired ability to exercise self-control
    pub exercise_self_control: bool,
}

/// Suicide pact facts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuicidePactFacts {
    /// Was there a common agreement to die?
    pub common_agreement: bool,
    /// Did D have settled intention of dying?
    pub settled_intention: bool,
    /// Evidence of pact
    pub evidence: Vec<String>,
}

/// Result of murder analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MurderAnalysisResult {
    /// Is murder established?
    pub murder_established: bool,
    /// Actus reus analysis
    pub actus_reus: ActusReusAnalysisResult,
    /// Mens rea analysis
    pub mens_rea: MensReaAnalysis,
    /// Causation analysis
    pub causation: CausationAnalysis,
    /// Partial defence results (if any)
    pub partial_defences: Vec<DefenceResult>,
    /// Final verdict
    pub verdict: HomicideVerdict,
    /// Sentencing implications
    pub sentencing: MurderSentencing,
    /// Key case law applied
    pub case_law: Vec<CaseCitation>,
}

/// Actus reus analysis result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActusReusAnalysisResult {
    /// Actus reus established?
    pub established: bool,
    /// Unlawful act identified
    pub unlawful_act: bool,
    /// Reasonable creature in being
    pub reasonable_creature: bool,
    /// Under Queen's Peace
    pub under_queens_peace: bool,
    /// Findings
    pub findings: Vec<String>,
}

/// Homicide verdict
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HomicideVerdict {
    /// Murder
    Murder,
    /// Voluntary manslaughter (partial defence succeeded)
    VoluntaryManslaughter { defence: String },
    /// Involuntary manslaughter
    InvoluntaryManslaughter { basis: InvoluntaryManslaughterType },
    /// Not guilty of homicide
    NotGuilty { reason: String },
}

/// Type of involuntary manslaughter
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvoluntaryManslaughterType {
    /// Unlawful act manslaughter
    UnlawfulAct,
    /// Gross negligence manslaughter
    GrossNegligence,
    /// Reckless manslaughter
    Reckless,
}

/// Murder sentencing (mandatory life with minimum term)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MurderSentencing {
    /// Starting point under Schedule 21
    pub starting_point: Schedule21StartingPoint,
    /// Aggravating factors identified
    pub aggravating: Vec<String>,
    /// Mitigating factors identified
    pub mitigating: Vec<String>,
    /// Recommended minimum term (years)
    pub minimum_term_years: Option<u32>,
}

/// Schedule 21 starting points for murder
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Schedule21StartingPoint {
    /// Whole life order - exceptionally serious
    WholeLife,
    /// 30 years - particularly serious
    ThirtyYears,
    /// 25 years - knife/weapon taken to scene
    TwentyFiveYears,
    /// 15 years - other murder
    FifteenYears,
    /// Lower starting point for diminished responsibility etc.
    Reduced { reason: String },
}

/// Murder analyzer
pub struct MurderAnalyzer;

impl MurderAnalyzer {
    /// Analyze murder case
    pub fn analyze(facts: &MurderFacts) -> CriminalResult<MurderAnalysisResult> {
        // Analyze actus reus
        let actus_reus = Self::analyze_actus_reus(facts)?;

        // Analyze mens rea
        let mens_rea = Self::analyze_mens_rea(facts)?;

        // Analyze causation
        let causation = Self::analyze_causation(facts)?;

        // Analyze partial defences
        let partial_defences = Self::analyze_partial_defences(facts)?;

        // Determine verdict
        let verdict =
            Self::determine_verdict(&actus_reus, &mens_rea, &causation, &partial_defences);

        // Determine sentencing
        let sentencing = Self::determine_sentencing(facts, &verdict);

        let case_law = Self::collect_case_law(&mens_rea, &causation, &partial_defences);

        let murder_established = matches!(verdict, HomicideVerdict::Murder);

        Ok(MurderAnalysisResult {
            murder_established,
            actus_reus,
            mens_rea,
            causation,
            partial_defences,
            verdict,
            sentencing,
            case_law,
        })
    }

    fn analyze_actus_reus(facts: &MurderFacts) -> CriminalResult<ActusReusAnalysisResult> {
        let mut findings = Vec::new();

        // Check death occurred
        if !facts.death_occurred {
            return Ok(ActusReusAnalysisResult {
                established: false,
                unlawful_act: false,
                reasonable_creature: facts.victim.reasonable_creature,
                under_queens_peace: facts.under_queens_peace,
                findings: vec!["No death occurred - no completed homicide".into()],
            });
        }
        findings.push("Death of victim established".into());

        // Check reasonable creature in being
        if !facts.victim.reasonable_creature {
            return Ok(ActusReusAnalysisResult {
                established: false,
                unlawful_act: true,
                reasonable_creature: false,
                under_queens_peace: facts.under_queens_peace,
                findings: vec!["Victim was not a 'reasonable creature in being'".into()],
            });
        }
        findings.push("Victim was reasonable creature in being".into());

        // Check Queen's Peace
        if !facts.under_queens_peace {
            findings.push("Warning: killing may not be under Queen's Peace".into());
        }

        // Analyze conduct
        match &facts.defendant_conduct.conduct_type {
            ConductType::Act => {
                findings.push("Positive act by defendant identified".into());
            }
            ConductType::Omission { duty_source } => {
                findings.push(format!("Omission with duty from {:?} source", duty_source));
            }
        }

        Ok(ActusReusAnalysisResult {
            established: true,
            unlawful_act: true,
            reasonable_creature: true,
            under_queens_peace: facts.under_queens_peace,
            findings,
        })
    }

    fn analyze_mens_rea(facts: &MurderFacts) -> CriminalResult<MensReaAnalysis> {
        let mut evidence = Vec::new();
        let mut case_law = Vec::new();

        // Check for direct intention to kill
        if !facts.intent_evidence.direct_intent_kill.is_empty() {
            evidence.extend(facts.intent_evidence.direct_intent_kill.clone());
            return Ok(MensReaAnalysis {
                mens_rea_type: MensReaType::DirectIntention,
                established: true,
                evidence,
                reasoning: "Direct intention to kill established from evidence".into(),
                case_law,
            });
        }

        // Check for direct intention to cause GBH (R v Cunningham [1982])
        if !facts.intent_evidence.direct_intent_gbh.is_empty() {
            evidence.extend(facts.intent_evidence.direct_intent_gbh.clone());
            case_law.push(CaseCitation::new(
                "R v Cunningham",
                1982,
                "AC 566",
                "Intention to cause GBH suffices for murder",
            ));
            return Ok(MensReaAnalysis {
                mens_rea_type: MensReaType::DirectIntention,
                established: true,
                evidence,
                reasoning: "Direct intention to cause GBH established - sufficient for murder \
                           per R v Cunningham"
                    .into(),
                case_law,
            });
        }

        // Check for oblique intention (Woollin)
        if let Some(oblique) = &facts.intent_evidence.oblique_intent_evidence {
            case_law.push(CaseCitation::new(
                "R v Woollin",
                1999,
                "AC 82",
                "Jury may find intention where death/GBH virtually certain and D appreciated this",
            ));

            if oblique.virtually_certain && oblique.defendant_appreciated {
                evidence.extend(oblique.evidence.clone());
                return Ok(MensReaAnalysis {
                    mens_rea_type: MensReaType::ObliqueIntention,
                    established: true,
                    evidence,
                    reasoning: "Oblique intention established: death/GBH virtually certain \
                               and D appreciated this (Woollin direction)"
                        .into(),
                    case_law,
                });
            }
        }

        // No malice aforethought
        evidence.extend(facts.intent_evidence.defendant_statements.clone());

        Ok(MensReaAnalysis {
            mens_rea_type: MensReaType::DirectIntention,
            established: false,
            evidence,
            reasoning: "Malice aforethought not established - neither intention to kill \
                       nor intention to cause GBH proven"
                .into(),
            case_law,
        })
    }

    fn analyze_causation(facts: &MurderFacts) -> CriminalResult<CausationAnalysis> {
        let cf = &facts.causation_facts;

        // Factual causation
        let factual = FactualCausation {
            established: cf.but_for_satisfied,
            but_for_analysis: if cf.but_for_satisfied {
                "But for D's conduct, V would not have died".into()
            } else {
                "V would have died regardless of D's conduct".into()
            },
        };

        // Legal causation
        let legal = LegalCausation {
            established: cf.operating_substantial,
            operating_and_substantial: cf.operating_substantial,
            reasoning: if cf.operating_substantial {
                "D's act was an operating and substantial cause of death".into()
            } else {
                "D's act was not sufficiently connected to death".into()
            },
        };

        // Analyze intervening acts
        let intervening_acts: Vec<InterveningAct> = cf
            .intervening_acts
            .iter()
            .map(|ia| {
                let breaks_chain = Self::assess_chain_break(ia);
                InterveningAct {
                    description: ia.description.clone(),
                    act_type: ia.act_type.clone(),
                    breaks_chain,
                    analysis: Self::analyze_intervening_act(ia),
                }
            })
            .collect();

        let chain_broken = intervening_acts.iter().any(|ia| ia.breaks_chain);

        let causation_established = factual.established && legal.established && !chain_broken;

        let mut reasoning = String::new();
        if !factual.established {
            reasoning.push_str("Factual causation not established. ");
        }
        if !legal.established {
            reasoning.push_str("Legal causation not established. ");
        }
        if chain_broken {
            reasoning.push_str("Causal chain broken by intervening act. ");
        }
        if causation_established {
            reasoning = "Both factual and legal causation established with no chain break".into();
        }

        Ok(CausationAnalysis {
            factual_causation: factual,
            legal_causation: legal,
            intervening_acts,
            causation_established,
            reasoning,
        })
    }

    fn assess_chain_break(ia: &InterveningActDetails) -> bool {
        match &ia.act_type {
            InterveningActType::MedicalTreatment => {
                // Only palpably wrong treatment breaks chain (R v Cheshire)
                ia.palpably_wrong.unwrap_or(false)
            }
            InterveningActType::VictimAct => {
                // Only daft/unexpected acts break chain (R v Roberts)
                !ia.foreseeable.unwrap_or(true)
            }
            InterveningActType::ThirdPartyAct => {
                // Free, deliberate, informed acts may break chain
                !ia.foreseeable.unwrap_or(true)
            }
            InterveningActType::NaturalEvent => {
                // Natural events rarely break chain
                false
            }
            InterveningActType::PreExistingCondition => {
                // Never breaks chain (thin skull rule - R v Blaue)
                false
            }
        }
    }

    fn analyze_intervening_act(ia: &InterveningActDetails) -> String {
        match &ia.act_type {
            InterveningActType::MedicalTreatment => {
                if ia.palpably_wrong == Some(true) {
                    "Treatment was palpably wrong - may break chain (cf. R v Jordan)".into()
                } else {
                    "Medical treatment does not break chain unless palpably wrong (R v Cheshire)"
                        .into()
                }
            }
            InterveningActType::VictimAct => {
                if ia.foreseeable == Some(true) {
                    "Victim's act was reasonably foreseeable - does not break chain (R v Roberts)"
                        .into()
                } else {
                    "Victim's act may have been 'daft' - potentially breaks chain".into()
                }
            }
            InterveningActType::PreExistingCondition => {
                "Thin skull rule applies - D takes victim as found (R v Blaue)".into()
            }
            _ => "Intervening act analyzed under general principles".into(),
        }
    }

    fn analyze_partial_defences(facts: &MurderFacts) -> CriminalResult<Vec<DefenceResult>> {
        let mut results = Vec::new();

        if let Some(pd) = &facts.partial_defences {
            // Loss of control
            if let Some(loc) = &pd.loss_of_control {
                results.push(Self::analyze_loss_of_control(loc));
            }

            // Diminished responsibility
            if let Some(dr) = &pd.diminished_responsibility {
                results.push(Self::analyze_diminished_responsibility(dr));
            }

            // Suicide pact
            if let Some(sp) = &pd.suicide_pact {
                results.push(Self::analyze_suicide_pact(sp));
            }
        }

        Ok(results)
    }

    fn analyze_loss_of_control(facts: &LossOfControlFacts) -> DefenceResult {
        let mut available = true;
        let mut findings = Vec::new();
        let case_law = vec![
            CaseCitation::new(
                "R v Clinton",
                2012,
                "EWCA Crim 2",
                "Sexual infidelity alone cannot be qualifying trigger",
            ),
            CaseCitation::new(
                "R v Dawes",
                2013,
                "EWCA Crim 322",
                "Loss of control guidance post-CJA 2009",
            ),
        ];

        // Check loss of self-control
        if !facts.lost_control {
            available = false;
            findings.push("D did not lose self-control".into());
        } else {
            findings.push("D lost self-control".into());
        }

        // Check qualifying trigger
        let has_trigger = match &facts.qualifying_trigger {
            Some(QualifyingTrigger::FearOfViolence { .. }) => {
                findings.push("Fear of violence trigger identified (s.55(3))".into());
                true
            }
            Some(QualifyingTrigger::Anger {
                justifiable_sense_of_being_wronged,
                ..
            }) => {
                if *justifiable_sense_of_being_wronged {
                    findings.push(
                        "Anger trigger with justifiable sense of being wronged (s.55(4))".into(),
                    );
                    true
                } else {
                    findings.push("Anger trigger but no justifiable sense of being wronged".into());
                    false
                }
            }
            Some(QualifyingTrigger::Combined { .. }) => {
                findings.push("Combined fear and anger triggers".into());
                true
            }
            None => {
                findings.push("No qualifying trigger identified".into());
                false
            }
        };

        if !has_trigger {
            available = false;
        }

        // Check normal tolerance test
        if let Some(ntt) = &facts.normal_tolerance_test {
            if !ntt.normal_person_would_react {
                available = false;
                findings.push(
                    "Person of D's age/sex with normal tolerance would not have reacted similarly"
                        .into(),
                );
            } else {
                findings.push("Normal tolerance test satisfied".into());
            }
        }

        DefenceResult {
            defence_type: DefenceType::LossOfControl,
            available,
            effect: if available {
                Some(DefenceEffect::LesserOffence {
                    offence: "Voluntary Manslaughter".into(),
                })
            } else {
                None
            },
            findings,
            case_law,
        }
    }

    fn analyze_diminished_responsibility(facts: &DiminishedResponsibilityFacts) -> DefenceResult {
        let mut available = true;
        let mut findings = Vec::new();

        // Check abnormality of mental functioning
        if !facts.abnormality {
            available = false;
            findings.push("No abnormality of mental functioning".into());
        } else {
            findings.push("Abnormality of mental functioning identified".into());
        }

        // Check recognized medical condition
        match &facts.recognized_condition {
            Some(condition) => {
                findings.push(format!("Recognized medical condition: {condition}"));
            }
            None => {
                available = false;
                findings.push("No recognized medical condition identified".into());
            }
        }

        // Check substantial impairment (one of three abilities)
        let has_impairment = facts.impairments.understand_conduct
            || facts.impairments.form_rational_judgment
            || facts.impairments.exercise_self_control;

        if !has_impairment {
            available = false;
            findings.push("No substantial impairment of required abilities".into());
        } else {
            if facts.impairments.understand_conduct {
                findings
                    .push("Substantially impaired ability to understand nature of conduct".into());
            }
            if facts.impairments.form_rational_judgment {
                findings.push("Substantially impaired ability to form rational judgment".into());
            }
            if facts.impairments.exercise_self_control {
                findings.push("Substantially impaired ability to exercise self-control".into());
            }
        }

        // Check significant contributory factor
        if !facts.significant_factor {
            available = false;
            findings.push("Abnormality was not significant contributory factor in killing".into());
        }

        let case_law = vec![
            CaseCitation::new(
                "R v Golds",
                2016,
                "UKSC 61",
                "Meaning of 'substantial' in diminished responsibility",
            ),
            CaseCitation::new(
                "R v Brennan",
                2014,
                "EWCA Crim 2387",
                "Voluntary intoxication and diminished responsibility",
            ),
        ];

        DefenceResult {
            defence_type: DefenceType::DiminishedResponsibility,
            available,
            effect: if available {
                Some(DefenceEffect::LesserOffence {
                    offence: "Voluntary Manslaughter".into(),
                })
            } else {
                None
            },
            findings,
            case_law,
        }
    }

    fn analyze_suicide_pact(facts: &SuicidePactFacts) -> DefenceResult {
        let available = facts.common_agreement && facts.settled_intention;

        let mut findings = Vec::new();
        if facts.common_agreement {
            findings.push("Common agreement to die together established".into());
        } else {
            findings.push("No common agreement to die together".into());
        }

        if facts.settled_intention {
            findings.push("D had settled intention of dying".into());
        } else {
            findings.push("D did not have settled intention of dying".into());
        }

        DefenceResult {
            defence_type: DefenceType::SuicidePact,
            available,
            effect: if available {
                Some(DefenceEffect::LesserOffence {
                    offence: "Voluntary Manslaughter".into(),
                })
            } else {
                None
            },
            findings,
            case_law: vec![],
        }
    }

    fn determine_verdict(
        actus_reus: &ActusReusAnalysisResult,
        mens_rea: &MensReaAnalysis,
        causation: &CausationAnalysis,
        partial_defences: &[DefenceResult],
    ) -> HomicideVerdict {
        // Check basic elements
        if !actus_reus.established {
            return HomicideVerdict::NotGuilty {
                reason: "Actus reus not established".into(),
            };
        }

        if !causation.causation_established {
            return HomicideVerdict::NotGuilty {
                reason: "Causation not established".into(),
            };
        }

        if !mens_rea.established {
            // May still be involuntary manslaughter
            return HomicideVerdict::InvoluntaryManslaughter {
                basis: InvoluntaryManslaughterType::UnlawfulAct,
            };
        }

        // Check partial defences
        for defence in partial_defences {
            if defence.available
                && let Some(DefenceEffect::LesserOffence { offence: _ }) = &defence.effect
            {
                return HomicideVerdict::VoluntaryManslaughter {
                    defence: format!("{:?}", defence.defence_type),
                };
            }
        }

        // All elements satisfied, no partial defence
        HomicideVerdict::Murder
    }

    fn determine_sentencing(facts: &MurderFacts, verdict: &HomicideVerdict) -> MurderSentencing {
        match verdict {
            HomicideVerdict::Murder => {
                let starting_point = Self::determine_starting_point(facts);
                let aggravating = Self::identify_aggravating(facts);
                let mitigating = Self::identify_mitigating(facts);

                let minimum_term_years = match &starting_point {
                    Schedule21StartingPoint::WholeLife => None,
                    Schedule21StartingPoint::ThirtyYears => Some(30),
                    Schedule21StartingPoint::TwentyFiveYears => Some(25),
                    Schedule21StartingPoint::FifteenYears => Some(15),
                    Schedule21StartingPoint::Reduced { .. } => Some(10),
                };

                MurderSentencing {
                    starting_point,
                    aggravating,
                    mitigating,
                    minimum_term_years,
                }
            }
            _ => MurderSentencing {
                starting_point: Schedule21StartingPoint::Reduced {
                    reason: "Not murder".into(),
                },
                aggravating: vec![],
                mitigating: vec![],
                minimum_term_years: None,
            },
        }
    }

    fn determine_starting_point(facts: &MurderFacts) -> Schedule21StartingPoint {
        // Check for whole life factors
        if let Some(premediation) = &facts.defendant_conduct.premeditation
            && premediation.planning
        {
            // Multiple killings + planning could indicate whole life
        }

        // Check if weapon brought to scene
        if let Some(weapon) = &facts.defendant_conduct.weapon
            && weapon.brought_to_scene
        {
            match weapon.weapon_type {
                WeaponType::Knife | WeaponType::Firearm => {
                    return Schedule21StartingPoint::TwentyFiveYears;
                }
                _ => {}
            }
        }

        // Default
        Schedule21StartingPoint::FifteenYears
    }

    fn identify_aggravating(facts: &MurderFacts) -> Vec<String> {
        let mut agg = Vec::new();

        // Vulnerable victim
        if facts.victim.vulnerability.is_some() {
            agg.push("Vulnerable victim".into());
        }

        // Planning
        if let Some(premediation) = &facts.defendant_conduct.premeditation
            && premediation.planning
        {
            agg.push("Premeditation/planning".into());
        }

        // Weapon brought to scene
        if let Some(weapon) = &facts.defendant_conduct.weapon
            && weapon.brought_to_scene
        {
            agg.push("Weapon brought to scene".into());
        }

        agg
    }

    fn identify_mitigating(_facts: &MurderFacts) -> Vec<String> {
        // Would be populated based on specific facts
        vec![]
    }

    fn collect_case_law(
        mens_rea: &MensReaAnalysis,
        _causation: &CausationAnalysis,
        partial_defences: &[DefenceResult],
    ) -> Vec<CaseCitation> {
        let mut cases = mens_rea.case_law.clone();
        for pd in partial_defences {
            cases.extend(pd.case_law.clone());
        }
        cases
    }
}

// ============================================================================
// Involuntary Manslaughter
// ============================================================================

/// Facts for unlawful act manslaughter analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnlawfulActManslaughterFacts {
    /// The unlawful act committed
    pub unlawful_act: UnlawfulActDetails,
    /// Was the act dangerous (Church test)?
    pub dangerousness: DangerousnessAssessment,
    /// Causation facts
    pub causation: CausationFacts,
    /// Basic mens rea for the unlawful act
    pub mens_rea_for_act: bool,
}

/// Details of unlawful act
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnlawfulActDetails {
    /// The act description
    pub act: String,
    /// Is it a criminal offence?
    pub criminal_offence: bool,
    /// Is it inherently unlawful (not just tortious)?
    pub inherently_unlawful: bool,
    /// Evidence
    pub evidence: Vec<String>,
}

/// Dangerousness assessment (R v Church \[1966\])
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DangerousnessAssessment {
    /// Would sober and reasonable person recognize risk of some harm?
    pub reasonable_person_recognize_harm: bool,
    /// What harm would they recognize?
    pub recognizable_harm: String,
    /// Evidence
    pub evidence: Vec<String>,
}

/// Result of UAM analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnlawfulActManslaughterResult {
    /// Is UAM established?
    pub established: bool,
    /// Unlawful act finding
    pub unlawful_act: UnlawfulActFinding,
    /// Dangerousness finding
    pub dangerousness: DangerousnessFinding,
    /// Causation established?
    pub causation_established: bool,
    /// Mens rea established?
    pub mens_rea_established: bool,
    /// Key case law
    pub case_law: Vec<CaseCitation>,
}

/// Unlawful act finding
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnlawfulActFinding {
    /// Unlawful act established?
    pub established: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Dangerousness finding
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DangerousnessFinding {
    /// Dangerousness established?
    pub established: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Analyzer for unlawful act manslaughter
pub struct UnlawfulActManslaughterAnalyzer;

impl UnlawfulActManslaughterAnalyzer {
    /// Analyze unlawful act manslaughter
    pub fn analyze(
        facts: &UnlawfulActManslaughterFacts,
    ) -> CriminalResult<UnlawfulActManslaughterResult> {
        let case_law = vec![
            CaseCitation::new(
                "R v Church",
                1966,
                "1 QB 59",
                "Unlawful act must be dangerous - sober/reasonable person would recognize harm risk",
            ),
            CaseCitation::new(
                "R v Newbury and Jones",
                1977,
                "AC 500",
                "D need not know act is dangerous or unlawful",
            ),
            CaseCitation::new(
                "R v Lamb",
                1967,
                "2 QB 981",
                "Act must be objectively unlawful - not just negligent",
            ),
        ];

        // Unlawful act analysis
        let unlawful = UnlawfulActFinding {
            established: facts.unlawful_act.criminal_offence
                && facts.unlawful_act.inherently_unlawful,
            reasoning: if facts.unlawful_act.criminal_offence
                && facts.unlawful_act.inherently_unlawful
            {
                "Act constitutes criminal offence and is inherently unlawful".into()
            } else if !facts.unlawful_act.criminal_offence {
                "Act is not a criminal offence (mere tort insufficient - R v Franklin)".into()
            } else {
                "Act is not inherently unlawful".into()
            },
        };

        // Dangerousness analysis (Church test)
        let dangerous = DangerousnessFinding {
            established: facts.dangerousness.reasonable_person_recognize_harm,
            reasoning: if facts.dangerousness.reasonable_person_recognize_harm {
                format!(
                    "Sober and reasonable person would recognize risk of {} (Church test satisfied)",
                    facts.dangerousness.recognizable_harm
                )
            } else {
                "Sober and reasonable person would not recognize risk of harm".into()
            },
        };

        // Causation
        let causation_established =
            facts.causation.but_for_satisfied && facts.causation.operating_substantial;

        // Mens rea (for the unlawful act, not for death)
        let mens_rea_established = facts.mens_rea_for_act;

        let established = unlawful.established
            && dangerous.established
            && causation_established
            && mens_rea_established;

        Ok(UnlawfulActManslaughterResult {
            established,
            unlawful_act: unlawful,
            dangerousness: dangerous,
            causation_established,
            mens_rea_established,
            case_law,
        })
    }
}

/// Facts for gross negligence manslaughter
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GrossNegligenceManslaughterFacts {
    /// Did D owe a duty of care?
    pub duty_of_care: DutyOfCareFacts,
    /// Was duty breached?
    pub breach: BreachOfDutyFacts,
    /// Did breach cause death?
    pub causation: CausationFacts,
    /// Was negligence gross?
    pub grossness: GrossNegligenceAssessment,
}

/// Duty of care facts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DutyOfCareFacts {
    /// Duty established?
    pub duty_exists: bool,
    /// Source of duty
    pub duty_source: String,
    /// Reasoning
    pub reasoning: String,
}

/// Breach of duty facts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreachOfDutyFacts {
    /// Duty breached?
    pub breached: bool,
    /// How duty was breached
    pub manner_of_breach: String,
    /// Risk of death involved
    pub risk_of_death: bool,
}

/// Gross negligence assessment (Adomako test)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GrossNegligenceAssessment {
    /// Was conduct so bad it should be criminal?
    pub criminal_level: bool,
    /// Evidence
    pub evidence: Vec<String>,
    /// Comparison to ordinary competent person
    pub comparison: String,
}

/// Result of gross negligence manslaughter analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GrossNegligenceManslaughterResult {
    /// Is GNM established?
    pub established: bool,
    /// Duty of care finding
    pub duty_finding: String,
    /// Breach finding
    pub breach_finding: String,
    /// Risk of death finding
    pub risk_of_death_finding: String,
    /// Grossness finding
    pub grossness_finding: String,
    /// Causation finding
    pub causation_finding: String,
    /// Key case law
    pub case_law: Vec<CaseCitation>,
}

/// Analyzer for gross negligence manslaughter
pub struct GrossNegligenceManslaughterAnalyzer;

impl GrossNegligenceManslaughterAnalyzer {
    /// Analyze gross negligence manslaughter (Adomako test)
    pub fn analyze(
        facts: &GrossNegligenceManslaughterFacts,
    ) -> CriminalResult<GrossNegligenceManslaughterResult> {
        let case_law = vec![
            CaseCitation::new(
                "R v Adomako",
                1995,
                "1 AC 171",
                "Four-stage test for gross negligence manslaughter",
            ),
            CaseCitation::new(
                "R v Rose",
                2017,
                "EWCA Crim 1168",
                "Duty must exist at time of alleged breach",
            ),
            CaseCitation::new(
                "R v Broughton",
                2020,
                "EWCA Crim 1093",
                "Grossness threshold guidance",
            ),
        ];

        // Check each Adomako element
        let duty_finding = if facts.duty_of_care.duty_exists {
            format!(
                "Duty of care established: {}",
                facts.duty_of_care.duty_source
            )
        } else {
            "No duty of care established".into()
        };

        let breach_finding = if facts.breach.breached {
            format!("Duty breached: {}", facts.breach.manner_of_breach)
        } else {
            "Duty not breached".into()
        };

        let risk_of_death_finding = if facts.breach.risk_of_death {
            "Breach involved obvious risk of death".into()
        } else {
            "Breach did not involve risk of death".into()
        };

        let grossness_finding = if facts.grossness.criminal_level {
            "Negligence was so gross as to be criminal".into()
        } else {
            "Negligence not gross enough to be criminal".into()
        };

        let causation_established =
            facts.causation.but_for_satisfied && facts.causation.operating_substantial;
        let causation_finding = if causation_established {
            "Breach caused death".into()
        } else {
            "Causation not established".into()
        };

        let established = facts.duty_of_care.duty_exists
            && facts.breach.breached
            && facts.breach.risk_of_death
            && facts.grossness.criminal_level
            && causation_established;

        Ok(GrossNegligenceManslaughterResult {
            established,
            duty_finding,
            breach_finding,
            risk_of_death_finding,
            grossness_finding,
            causation_finding,
            case_law,
        })
    }
}

// ============================================================================
// Corporate Manslaughter
// ============================================================================

/// Facts for corporate manslaughter analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CorporateManslaughterFacts {
    /// Organization details
    pub organization: OrganizationDetails,
    /// Management failure
    pub management_failure: ManagementFailureDetails,
    /// Death details
    pub death: DeathDetails,
    /// Senior management involvement
    pub senior_management: SeniorManagementInvolvement,
}

/// Organization details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationDetails {
    /// Type of organization
    pub organization_type: OrganizationType,
    /// Name
    pub name: String,
}

/// Types of organization under CMCHA 2007
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrganizationType {
    /// Corporation
    Corporation,
    /// Partnership
    Partnership,
    /// Government department
    GovernmentDepartment,
    /// Police force
    PoliceForce,
    /// Other statutory body
    StatutoryBody,
}

/// Management failure details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManagementFailureDetails {
    /// Activity being managed
    pub activity: String,
    /// How was it managed?
    pub management: String,
    /// Failure description
    pub failure: String,
    /// Gross breach of duty of care?
    pub gross_breach: bool,
}

/// Death details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeathDetails {
    /// Who died?
    pub victim: String,
    /// Cause of death
    pub cause: String,
    /// Link to management failure
    pub link_to_failure: String,
}

/// Senior management involvement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeniorManagementInvolvement {
    /// Senior management element substantial?
    pub substantial_element: bool,
    /// Details
    pub details: String,
}

/// Result of corporate manslaughter analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CorporateManslaughterResult {
    /// Offence established?
    pub established: bool,
    /// Organization qualifies?
    pub organization_qualifies: bool,
    /// Relevant duty owed?
    pub relevant_duty: bool,
    /// Gross breach found?
    pub gross_breach: bool,
    /// Senior management element?
    pub senior_management_element: bool,
    /// Sentencing
    pub sentencing: CorporateManslaughterSentencing,
    /// Findings
    pub findings: Vec<String>,
}

/// Corporate manslaughter sentencing
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CorporateManslaughterSentencing {
    /// Unlimited fine
    pub fine: bool,
    /// Publicity order possible
    pub publicity_order: bool,
    /// Remedial order possible
    pub remedial_order: bool,
}

/// Analyzer for corporate manslaughter
pub struct CorporateManslaughterAnalyzer;

impl CorporateManslaughterAnalyzer {
    /// Analyze corporate manslaughter (CMCHA 2007)
    pub fn analyze(
        facts: &CorporateManslaughterFacts,
    ) -> CriminalResult<CorporateManslaughterResult> {
        let mut findings = Vec::new();

        // Check organization qualifies
        let organization_qualifies = true; // All organization types can be liable
        findings.push(format!(
            "Organization ({:?}) qualifies under CMCHA 2007",
            facts.organization.organization_type
        ));

        // Check relevant duty
        let relevant_duty = true; // Assume duty exists for activities
        findings.push(format!(
            "Relevant duty owed for activity: {}",
            facts.management_failure.activity
        ));

        // Check gross breach
        let gross_breach = facts.management_failure.gross_breach;
        if gross_breach {
            findings.push("Gross breach of duty of care established".into());
        } else {
            findings.push("No gross breach found".into());
        }

        // Check senior management element
        let senior_management_element = facts.senior_management.substantial_element;
        if senior_management_element {
            findings.push("Senior management element was substantial part of failure".into());
        } else {
            findings.push("Senior management element not substantial".into());
        }

        let established =
            organization_qualifies && relevant_duty && gross_breach && senior_management_element;

        let sentencing = CorporateManslaughterSentencing {
            fine: true,
            publicity_order: established,
            remedial_order: established,
        };

        Ok(CorporateManslaughterResult {
            established,
            organization_qualifies,
            relevant_duty,
            gross_breach,
            senior_management_element,
            sentencing,
            findings,
        })
    }
}

// ============================================================================
// Standard Homicide Offence Definitions
// ============================================================================

/// Get standard murder offence definition
pub fn murder_offence() -> CriminalResult<Offence> {
    OffenceBuilder::new()
        .name("Murder")
        .common_law()
        .classification(OffenceClassification::IndictableOnly)
        .category(OffenceCategory::AgainstPerson)
        .severity(OffenceSeverity::MostSerious)
        .maximum_sentence(MaximumSentence::Life)
        .mens_rea(MensReaType::DirectIntention)
        .actus_reus(ActusReusElement::Act(ActType::VoluntaryAct))
        .actus_reus(ActusReusElement::Consequence("Death of victim".into()))
        .build()
        .map_err(CriminalError::InvalidInput)
}

/// Get voluntary manslaughter offence definition
pub fn voluntary_manslaughter_offence() -> CriminalResult<Offence> {
    OffenceBuilder::new()
        .name("Voluntary Manslaughter")
        .common_law()
        .classification(OffenceClassification::IndictableOnly)
        .category(OffenceCategory::AgainstPerson)
        .severity(OffenceSeverity::VerySerious)
        .maximum_sentence(MaximumSentence::Life)
        .mens_rea(MensReaType::DirectIntention)
        .actus_reus(ActusReusElement::Act(ActType::VoluntaryAct))
        .actus_reus(ActusReusElement::Consequence("Death of victim".into()))
        .build()
        .map_err(CriminalError::InvalidInput)
}

/// Get involuntary manslaughter (unlawful act) offence definition
pub fn unlawful_act_manslaughter_offence() -> CriminalResult<Offence> {
    OffenceBuilder::new()
        .name("Unlawful Act Manslaughter")
        .common_law()
        .classification(OffenceClassification::IndictableOnly)
        .category(OffenceCategory::AgainstPerson)
        .severity(OffenceSeverity::VerySerious)
        .maximum_sentence(MaximumSentence::Life)
        .mens_rea(MensReaType::DirectIntention) // For the unlawful act
        .actus_reus(ActusReusElement::Act(ActType::VoluntaryAct))
        .actus_reus(ActusReusElement::Consequence("Death of victim".into()))
        .build()
        .map_err(CriminalError::InvalidInput)
}

/// Get gross negligence manslaughter offence definition
pub fn gross_negligence_manslaughter_offence() -> CriminalResult<Offence> {
    OffenceBuilder::new()
        .name("Gross Negligence Manslaughter")
        .common_law()
        .classification(OffenceClassification::IndictableOnly)
        .category(OffenceCategory::AgainstPerson)
        .severity(OffenceSeverity::VerySerious)
        .maximum_sentence(MaximumSentence::Life)
        .mens_rea(MensReaType::GrossNegligence)
        .actus_reus(ActusReusElement::Act(ActType::VoluntaryAct))
        .actus_reus(ActusReusElement::Consequence("Death of victim".into()))
        .build()
        .map_err(CriminalError::InvalidInput)
}

/// Get corporate manslaughter offence definition
pub fn corporate_manslaughter_offence() -> CriminalResult<Offence> {
    OffenceBuilder::new()
        .name("Corporate Manslaughter")
        .statutory_source(
            "Corporate Manslaughter and Corporate Homicide Act 2007",
            "s.1",
        )
        .classification(OffenceClassification::IndictableOnly)
        .category(OffenceCategory::Regulatory)
        .severity(OffenceSeverity::VerySerious)
        .maximum_sentence(MaximumSentence::FineOnly { unlimited: true })
        .mens_rea(MensReaType::GrossNegligence)
        .actus_reus(ActusReusElement::Consequence(
            "Death caused by gross breach".into(),
        ))
        .build()
        .map_err(CriminalError::InvalidInput)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn create_basic_murder_facts() -> MurderFacts {
        MurderFacts {
            death_occurred: true,
            victim: VictimDetails {
                reasonable_creature: true,
                pre_existing_conditions: vec![],
                age: Some(30),
                vulnerability: None,
            },
            defendant_conduct: DefendantConduct {
                conduct_type: ConductType::Act,
                description: "Stabbed victim multiple times".into(),
                weapon: Some(WeaponUsed {
                    weapon_type: WeaponType::Knife,
                    brought_to_scene: true,
                }),
                premeditation: Some(PremediationEvidence {
                    planning: true,
                    planning_evidence: vec!["Purchased knife day before".into()],
                    motive: Some("Revenge".into()),
                }),
            },
            intent_evidence: IntentEvidence {
                direct_intent_kill: vec!["Told friend 'I'm going to kill him'".into()],
                direct_intent_gbh: vec![],
                oblique_intent_evidence: None,
                defendant_statements: vec![],
            },
            causation_facts: CausationFacts {
                but_for_satisfied: true,
                operating_substantial: true,
                intervening_acts: vec![],
                thin_skull: None,
            },
            under_queens_peace: true,
            partial_defences: None,
        }
    }

    #[test]
    fn test_murder_analysis_basic() {
        let facts = create_basic_murder_facts();
        let result = MurderAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.murder_established);
        assert!(matches!(analysis.verdict, HomicideVerdict::Murder));
    }

    #[test]
    fn test_murder_with_diminished_responsibility() {
        let mut facts = create_basic_murder_facts();
        facts.partial_defences = Some(PartialDefenceFacts {
            loss_of_control: None,
            diminished_responsibility: Some(DiminishedResponsibilityFacts {
                abnormality: true,
                recognized_condition: Some("Paranoid schizophrenia".into()),
                impairments: DiminishedImpairments {
                    understand_conduct: false,
                    form_rational_judgment: true,
                    exercise_self_control: true,
                },
                significant_factor: true,
                psychiatric_evidence: vec!["Expert report from Dr. Smith".into()],
            }),
            suicide_pact: None,
        });

        let result = MurderAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(!analysis.murder_established);
        assert!(matches!(
            analysis.verdict,
            HomicideVerdict::VoluntaryManslaughter { .. }
        ));
    }

    #[test]
    fn test_oblique_intention() {
        let mut facts = create_basic_murder_facts();
        facts.intent_evidence.direct_intent_kill = vec![];
        facts.intent_evidence.oblique_intent_evidence = Some(ObliqueIntentEvidence {
            virtually_certain: true,
            defendant_appreciated: true,
            evidence: vec!["Locked victims in burning building".into()],
        });

        let result = MurderAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.mens_rea.established);
        assert!(matches!(
            analysis.mens_rea.mens_rea_type,
            MensReaType::ObliqueIntention
        ));
    }

    #[test]
    fn test_causation_chain_break() {
        let mut facts = create_basic_murder_facts();
        facts.causation_facts.intervening_acts = vec![InterveningActDetails {
            description: "Grossly negligent medical treatment".into(),
            act_type: InterveningActType::MedicalTreatment,
            palpably_wrong: Some(true),
            foreseeable: None,
        }];

        let result = MurderAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(!analysis.causation.causation_established);
    }

    #[test]
    fn test_thin_skull_rule() {
        let mut facts = create_basic_murder_facts();
        facts.victim.pre_existing_conditions = vec!["Haemophilia".into()];
        facts.causation_facts.thin_skull = Some(ThinSkullDetails {
            condition: "Haemophilia".into(),
            contributed: true,
        });

        let result = MurderAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        // Thin skull doesn't break causation
        assert!(analysis.causation.causation_established);
    }

    #[test]
    fn test_unlawful_act_manslaughter() {
        let facts = UnlawfulActManslaughterFacts {
            unlawful_act: UnlawfulActDetails {
                act: "Common assault".into(),
                criminal_offence: true,
                inherently_unlawful: true,
                evidence: vec!["Pushed victim".into()],
            },
            dangerousness: DangerousnessAssessment {
                reasonable_person_recognize_harm: true,
                recognizable_harm: "Some physical harm".into(),
                evidence: vec!["Pushing near stairs".into()],
            },
            causation: CausationFacts {
                but_for_satisfied: true,
                operating_substantial: true,
                intervening_acts: vec![],
                thin_skull: None,
            },
            mens_rea_for_act: true,
        };

        let result = UnlawfulActManslaughterAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.established);
    }

    #[test]
    fn test_gross_negligence_manslaughter() {
        let facts = GrossNegligenceManslaughterFacts {
            duty_of_care: DutyOfCareFacts {
                duty_exists: true,
                duty_source: "Doctor-patient relationship".into(),
                reasoning: "Professional duty to patients".into(),
            },
            breach: BreachOfDutyFacts {
                breached: true,
                manner_of_breach: "Failed to diagnose obvious symptoms".into(),
                risk_of_death: true,
            },
            causation: CausationFacts {
                but_for_satisfied: true,
                operating_substantial: true,
                intervening_acts: vec![],
                thin_skull: None,
            },
            grossness: GrossNegligenceAssessment {
                criminal_level: true,
                evidence: vec!["Multiple failures over several days".into()],
                comparison: "Far below competent doctor standard".into(),
            },
        };

        let result = GrossNegligenceManslaughterAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.established);
    }

    #[test]
    fn test_corporate_manslaughter() {
        let facts = CorporateManslaughterFacts {
            organization: OrganizationDetails {
                organization_type: OrganizationType::Corporation,
                name: "Example Ltd".into(),
            },
            management_failure: ManagementFailureDetails {
                activity: "Construction site safety".into(),
                management: "Health and safety procedures".into(),
                failure: "No training, inadequate equipment".into(),
                gross_breach: true,
            },
            death: DeathDetails {
                victim: "Construction worker".into(),
                cause: "Fall from height".into(),
                link_to_failure: "No safety harness provided".into(),
            },
            senior_management: SeniorManagementInvolvement {
                substantial_element: true,
                details: "Directors knew of safety issues".into(),
            },
        };

        let result = CorporateManslaughterAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.established);
        assert!(analysis.sentencing.publicity_order);
    }

    #[test]
    fn test_loss_of_control_defence() {
        let facts = LossOfControlFacts {
            lost_control: true,
            loss_evidence: vec!["Reacted immediately after provocation".into()],
            qualifying_trigger: Some(QualifyingTrigger::FearOfViolence {
                fear_from: "Victim's threats and history of violence".into(),
                evidence: vec!["Prior assaults by victim".into()],
            }),
            normal_tolerance_test: Some(NormalToleranceAssessment {
                defendant_age: 35,
                defendant_sex: "Male".into(),
                relevant_circumstances: vec!["History of abuse by victim".into()],
                normal_person_would_react: true,
            }),
        };

        let result = MurderAnalyzer::analyze_loss_of_control(&facts);
        assert!(result.available);
    }

    #[test]
    fn test_schedule_21_starting_point() {
        let facts = create_basic_murder_facts();
        let starting_point = MurderAnalyzer::determine_starting_point(&facts);
        // Knife brought to scene = 25 years
        assert!(matches!(
            starting_point,
            Schedule21StartingPoint::TwentyFiveYears
        ));
    }

    #[test]
    fn test_offence_definitions() {
        assert!(murder_offence().is_ok());
        assert!(voluntary_manslaughter_offence().is_ok());
        assert!(unlawful_act_manslaughter_offence().is_ok());
        assert!(gross_negligence_manslaughter_offence().is_ok());
        assert!(corporate_manslaughter_offence().is_ok());
    }
}
