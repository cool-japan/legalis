//! UK Non-Fatal Offences Against the Person
//!
//! This module covers non-fatal offences including assault, battery, ABH, GBH,
//! and wounding under the Offences Against the Person Act 1861.
//!
//! # Statutory Framework
//!
//! - **Common assault** (s.39 CJA 1988) - Summary only, max 6 months
//! - **Battery** (common law) - Summary only, max 6 months
//! - **ABH** (s.47 OAPA 1861) - Either way, max 5 years
//! - **Wounding/GBH** (s.20 OAPA 1861) - Either way, max 5 years
//! - **Wounding/GBH with intent** (s.18 OAPA 1861) - Indictable only, max life
//!
//! # Key Cases
//!
//! - R v Ireland [1998] - Assault can be by words alone
//! - R v Savage; DPP v Parmenter [1992] - ABH definition
//! - R v Chan Fook [1994] - Psychiatric injury can be ABH
//! - R v Burstow [1998] - GBH includes serious psychiatric injury
//! - R v Dica [2004] - Transmission of disease as GBH

// Allow missing docs on enum variant struct fields - these are self-documenting in context
#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use crate::criminal::error::{CriminalError, CriminalResult};
use crate::criminal::types::{
    ActType, ActusReusElement, CaseCitation, DefenceResult, DefenceType, MaximumSentence,
    MensReaAnalysis, MensReaType, Offence, OffenceBuilder, OffenceCategory, OffenceClassification,
    OffenceSeverity,
};

// ============================================================================
// Offence Types
// ============================================================================

/// Non-fatal offence against the person
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NonFatalOffence {
    /// Common assault (s.39 CJA 1988)
    CommonAssault,
    /// Battery (common law)
    Battery,
    /// Assault occasioning ABH (s.47 OAPA 1861)
    AssaultOccasioningABH,
    /// Malicious wounding / inflicting GBH (s.20 OAPA 1861)
    Section20GBH,
    /// Wounding / causing GBH with intent (s.18 OAPA 1861)
    Section18GBH,
    /// Threats to kill (s.16 OAPA 1861)
    ThreatsToKill,
    /// Administering poison (s.23/24 OAPA 1861)
    AdministeringPoison { with_intent: bool },
}

// ============================================================================
// Assault and Battery
// ============================================================================

/// Facts for assault analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssaultFacts {
    /// Type of conduct
    pub conduct: AssaultConduct,
    /// Did victim apprehend immediate unlawful violence?
    pub apprehension: ApprehensionDetails,
    /// Mens rea evidence
    pub mens_rea: AssaultMensRea,
    /// Consent facts (if any)
    pub consent: Option<ConsentFacts>,
    /// Self-defence facts (if any)
    pub self_defence: Option<SelfDefenceFacts>,
}

/// Type of assault conduct
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AssaultConduct {
    /// Physical gesture
    Gesture { description: String },
    /// Words alone (R v Ireland)
    Words { words: String },
    /// Words and gesture combined
    WordsAndGesture { words: String, gesture: String },
    /// Silence (R v Ireland - silent phone calls)
    Silence { context: String },
}

/// Apprehension details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApprehensionDetails {
    /// Did V apprehend violence?
    pub apprehended_violence: bool,
    /// Was apprehension of immediate violence?
    pub immediate: bool,
    /// Evidence of apprehension
    pub evidence: Vec<String>,
}

/// Mens rea for assault
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssaultMensRea {
    /// Intentionally caused apprehension?
    pub intentional: bool,
    /// Reckless as to apprehension?
    pub reckless: bool,
    /// Evidence
    pub evidence: Vec<String>,
}

/// Facts for battery analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BatteryFacts {
    /// The touching/force applied
    pub force: ForceDetails,
    /// Mens rea evidence
    pub mens_rea: BatteryMensRea,
    /// Consent facts (if any)
    pub consent: Option<ConsentFacts>,
    /// Self-defence facts (if any)
    pub self_defence: Option<SelfDefenceFacts>,
}

/// Details of force applied
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForceDetails {
    /// Description of force
    pub description: String,
    /// Type of force
    pub force_type: ForceType,
    /// Was force unlawful?
    pub unlawful: bool,
}

/// Types of force for battery
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ForceType {
    /// Direct physical touching
    DirectTouch,
    /// Indirect force (throwing object)
    IndirectForce,
    /// Through another person/object
    ThroughMedium { medium: String },
    /// Continuing act (R v Fagan)
    ContinuingAct,
}

/// Mens rea for battery
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BatteryMensRea {
    /// Intentionally applied force?
    pub intentional: bool,
    /// Reckless as to application of force?
    pub reckless: bool,
    /// Evidence
    pub evidence: Vec<String>,
}

/// Consent facts (defence to assault/battery)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsentFacts {
    /// Was there consent?
    pub consent_given: bool,
    /// Type of activity
    pub activity_type: ConsentActivityType,
    /// Was consent valid?
    pub valid_consent: bool,
    /// Reasons consent may be invalid
    pub invalidity_reasons: Vec<String>,
}

/// Types of activity for consent analysis
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsentActivityType {
    /// Sporting activity
    Sport { sport: String },
    /// Horseplay
    Horseplay,
    /// Sexual activity
    Sexual,
    /// Medical treatment
    Medical,
    /// Tattooing/piercing
    BodyModification,
    /// Other lawful activity
    OtherLawful { description: String },
    /// Unlawful activity (consent no defence)
    Unlawful { activity: String },
}

/// Self-defence facts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelfDefenceFacts {
    /// Honest belief in need to use force
    pub honest_belief: bool,
    /// Was force necessary?
    pub necessity: bool,
    /// Was force reasonable/proportionate?
    pub proportionate: bool,
    /// Circumstances as D believed them
    pub believed_circumstances: String,
    /// Force actually used
    pub force_used: String,
}

/// Result of assault/battery analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssaultBatteryResult {
    /// Offence established?
    pub established: bool,
    /// Which offence?
    pub offence: NonFatalOffence,
    /// Actus reus findings
    pub actus_reus_findings: Vec<String>,
    /// Mens rea analysis
    pub mens_rea: MensReaAnalysis,
    /// Defence analysis (if any)
    pub defences: Vec<DefenceResult>,
    /// Key case law
    pub case_law: Vec<CaseCitation>,
}

/// Analyzer for assault and battery
pub struct AssaultBatteryAnalyzer;

impl AssaultBatteryAnalyzer {
    /// Analyze assault
    pub fn analyze_assault(facts: &AssaultFacts) -> CriminalResult<AssaultBatteryResult> {
        let case_law = vec![
            CaseCitation::new(
                "R v Ireland",
                1998,
                "AC 147",
                "Assault can be committed by words alone; silent phone calls can constitute assault",
            ),
            CaseCitation::new(
                "Tuberville v Savage",
                1669,
                "1 Mod Rep 3",
                "Words may negate otherwise threatening conduct",
            ),
        ];

        let mut ar_findings = Vec::new();
        let mut established = true;

        // Check conduct
        match &facts.conduct {
            AssaultConduct::Words { words } => {
                ar_findings.push(format!(
                    "Words used: '{}' (R v Ireland allows assault by words)",
                    words
                ));
            }
            AssaultConduct::Gesture { description } => {
                ar_findings.push(format!("Gesture: {}", description));
            }
            AssaultConduct::WordsAndGesture { words, gesture } => {
                ar_findings.push(format!("Words: '{}' with gesture: {}", words, gesture));
            }
            AssaultConduct::Silence { context } => {
                ar_findings.push(format!("Silence in context: {} (R v Ireland)", context));
            }
        }

        // Check apprehension
        if !facts.apprehension.apprehended_violence {
            established = false;
            ar_findings.push("No apprehension of violence by victim".into());
        } else {
            ar_findings.push("Victim apprehended violence".into());
        }

        if !facts.apprehension.immediate {
            established = false;
            ar_findings.push("Apprehension was not of immediate violence".into());
        } else {
            ar_findings.push("Apprehension was of immediate violence".into());
        }

        // Mens rea
        let mens_rea_established = facts.mens_rea.intentional || facts.mens_rea.reckless;
        let mens_rea = MensReaAnalysis {
            mens_rea_type: if facts.mens_rea.intentional {
                MensReaType::DirectIntention
            } else {
                MensReaType::SubjectiveRecklessness
            },
            established: mens_rea_established,
            evidence: facts.mens_rea.evidence.clone(),
            reasoning: if mens_rea_established {
                "D intended or was reckless as to causing V to apprehend violence".into()
            } else {
                "No intention or recklessness established".into()
            },
            case_law: vec![],
        };

        if !mens_rea_established {
            established = false;
        }

        // Check defences
        let mut defences = Vec::new();

        // Consent
        if let Some(consent) = &facts.consent {
            let consent_result = Self::analyze_consent(consent);
            if consent_result.available {
                established = false;
            }
            defences.push(consent_result);
        }

        // Self-defence
        if let Some(sd) = &facts.self_defence {
            let sd_result = Self::analyze_self_defence(sd);
            if sd_result.available {
                established = false;
            }
            defences.push(sd_result);
        }

        Ok(AssaultBatteryResult {
            established,
            offence: NonFatalOffence::CommonAssault,
            actus_reus_findings: ar_findings,
            mens_rea,
            defences,
            case_law,
        })
    }

    /// Analyze battery
    pub fn analyze_battery(facts: &BatteryFacts) -> CriminalResult<AssaultBatteryResult> {
        let case_law = vec![
            CaseCitation::new(
                "Collins v Wilcock",
                1984,
                "1 WLR 1172",
                "Battery is unlawful application of force; everyday touching excluded",
            ),
            CaseCitation::new(
                "R v Fagan",
                1969,
                "1 QB 439",
                "Continuing act doctrine - actus reus can be continuing",
            ),
        ];

        let mut ar_findings = Vec::new();
        let mut established = true;

        // Check force
        ar_findings.push(format!("Force applied: {}", facts.force.description));

        match &facts.force.force_type {
            ForceType::DirectTouch => {
                ar_findings.push("Direct physical touching".into());
            }
            ForceType::IndirectForce => {
                ar_findings.push("Indirect force applied".into());
            }
            ForceType::ThroughMedium { medium } => {
                ar_findings.push(format!("Force through medium: {}", medium));
            }
            ForceType::ContinuingAct => {
                ar_findings.push("Continuing act doctrine applies (R v Fagan)".into());
            }
        }

        if !facts.force.unlawful {
            established = false;
            ar_findings.push("Force was lawful (everyday touching/implied consent)".into());
        }

        // Mens rea
        let mens_rea_established = facts.mens_rea.intentional || facts.mens_rea.reckless;
        let mens_rea = MensReaAnalysis {
            mens_rea_type: if facts.mens_rea.intentional {
                MensReaType::DirectIntention
            } else {
                MensReaType::SubjectiveRecklessness
            },
            established: mens_rea_established,
            evidence: facts.mens_rea.evidence.clone(),
            reasoning: if mens_rea_established {
                "D intended or was reckless as to application of force".into()
            } else {
                "No intention or recklessness established".into()
            },
            case_law: vec![],
        };

        if !mens_rea_established {
            established = false;
        }

        // Check defences
        let mut defences = Vec::new();

        if let Some(consent) = &facts.consent {
            let consent_result = Self::analyze_consent(consent);
            if consent_result.available {
                established = false;
            }
            defences.push(consent_result);
        }

        if let Some(sd) = &facts.self_defence {
            let sd_result = Self::analyze_self_defence(sd);
            if sd_result.available {
                established = false;
            }
            defences.push(sd_result);
        }

        Ok(AssaultBatteryResult {
            established,
            offence: NonFatalOffence::Battery,
            actus_reus_findings: ar_findings,
            mens_rea,
            defences,
            case_law,
        })
    }

    fn analyze_consent(facts: &ConsentFacts) -> DefenceResult {
        let mut findings = Vec::new();
        let mut available = facts.consent_given && facts.valid_consent;

        if !facts.consent_given {
            findings.push("No consent given".into());
            available = false;
        } else {
            findings.push("Consent given".into());
        }

        // Check activity type
        match &facts.activity_type {
            ConsentActivityType::Sport { sport } => {
                findings.push(format!(
                    "Sporting activity ({}): consent valid for contact within rules",
                    sport
                ));
            }
            ConsentActivityType::Horseplay => {
                findings.push("Horseplay: consent valid (R v Jones)".into());
            }
            ConsentActivityType::Medical => {
                findings.push("Medical treatment: consent valid".into());
            }
            ConsentActivityType::Unlawful { activity } => {
                findings.push(format!(
                    "Unlawful activity ({}): consent no defence (R v Brown)",
                    activity
                ));
                available = false;
            }
            _ => {}
        }

        if !facts.valid_consent {
            findings.push("Consent invalid".into());
            findings.extend(facts.invalidity_reasons.clone());
            available = false;
        }

        DefenceResult {
            defence_type: DefenceType::Consent,
            available,
            effect: if available {
                Some(crate::criminal::types::DefenceEffect::Acquittal)
            } else {
                None
            },
            findings,
            case_law: vec![CaseCitation::new(
                "R v Brown",
                1994,
                "1 AC 212",
                "Consent no defence to intentional infliction of harm except recognized categories",
            )],
        }
    }

    fn analyze_self_defence(facts: &SelfDefenceFacts) -> DefenceResult {
        let mut findings = Vec::new();
        let available = facts.honest_belief && facts.proportionate;

        if facts.honest_belief {
            findings.push("D honestly believed force was necessary".into());
            findings.push(format!(
                "Believed circumstances: {}",
                facts.believed_circumstances
            ));
        } else {
            findings.push("No honest belief in need to use force".into());
        }

        if facts.proportionate {
            findings.push("Force used was reasonable/proportionate".into());
        } else {
            findings.push(format!(
                "Force used ({}) was disproportionate",
                facts.force_used
            ));
        }

        DefenceResult {
            defence_type: DefenceType::SelfDefence,
            available,
            effect: if available {
                Some(crate::criminal::types::DefenceEffect::Acquittal)
            } else {
                None
            },
            findings,
            case_law: vec![CaseCitation::new(
                "R v Williams (Gladstone)",
                1987,
                "3 All ER 411",
                "Judged on facts as D honestly believed them to be",
            )],
        }
    }
}

// ============================================================================
// ABH, GBH, Wounding
// ============================================================================

/// Facts for s.47 ABH analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ABHFacts {
    /// The assault or battery
    pub assault_battery: AssaultOrBattery,
    /// Harm caused
    pub harm: ABHHarm,
    /// Mens rea
    pub mens_rea: ABHMensRea,
}

/// Type of assault/battery base offence
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AssaultOrBattery {
    /// Assault
    Assault(AssaultFacts),
    /// Battery
    Battery(BatteryFacts),
}

/// Harm for ABH
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ABHHarm {
    /// Description of harm
    pub description: String,
    /// Type of harm
    pub harm_type: ABHHarmType,
    /// Is it more than transient or trifling?
    pub more_than_transient: bool,
}

/// Types of ABH harm
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ABHHarmType {
    /// Physical injury
    Physical,
    /// Psychiatric injury (R v Chan Fook)
    Psychiatric,
    /// Both physical and psychiatric
    Both,
}

/// Mens rea for ABH
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ABHMensRea {
    /// Mens rea for assault/battery established
    pub assault_battery_mr: bool,
    /// Note: No need for MR as to ABH itself (Savage; Parmenter)
    pub evidence: Vec<String>,
}

/// Facts for s.20 GBH analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Section20Facts {
    /// The wounding or GBH
    pub harm: Section20Harm,
    /// Causation (inflicting)
    pub infliction: InflictionDetails,
    /// Mens rea (maliciously)
    pub mens_rea: Section20MensRea,
}

/// Harm for s.20
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Section20Harm {
    /// Description
    pub description: String,
    /// Type: wounding or GBH
    pub harm_type: Section20HarmType,
    /// Evidence
    pub evidence: Vec<String>,
}

/// Type of s.20 harm
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Section20HarmType {
    /// Wounding (break in continuity of skin - JCC v Eisenhower)
    Wounding,
    /// Grievous bodily harm (really serious harm - DPP v Smith)
    GBH,
    /// Both wounding and GBH
    Both,
}

/// Details of infliction
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InflictionDetails {
    /// How was harm inflicted?
    pub method: String,
    /// Direct or indirect?
    pub direct: bool,
    /// Causation established?
    pub causation: bool,
}

/// Mens rea for s.20 ("maliciously")
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Section20MensRea {
    /// Intention to cause some harm (need not be GBH)
    pub intention_to_harm: bool,
    /// Recklessness as to some harm
    pub reckless_to_harm: bool,
    /// Evidence
    pub evidence: Vec<String>,
}

/// Facts for s.18 GBH analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Section18Facts {
    /// The wounding or GBH
    pub harm: Section20Harm,
    /// Causation
    pub causation: CausationDetails,
    /// Intent (specific intent offence)
    pub intent: Section18Intent,
}

/// Causation details for s.18
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CausationDetails {
    /// How was harm caused?
    pub method: String,
    /// Causation established?
    pub established: bool,
}

/// Intent for s.18 (specific intent)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Section18Intent {
    /// Intention to cause GBH?
    pub intent_gbh: bool,
    /// Intention to resist/prevent arrest?
    pub intent_resist_arrest: bool,
    /// Evidence of intent
    pub evidence: Vec<String>,
}

/// Result of aggravated assault analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AggravatedAssaultResult {
    /// Offence established?
    pub established: bool,
    /// Which offence?
    pub offence: NonFatalOffence,
    /// Alternative offences that may be established
    pub alternatives: Vec<NonFatalOffence>,
    /// Harm analysis
    pub harm_analysis: String,
    /// Mens rea analysis
    pub mens_rea: MensReaAnalysis,
    /// Key case law
    pub case_law: Vec<CaseCitation>,
}

/// Analyzer for aggravated assaults
pub struct AggravatedAssaultAnalyzer;

impl AggravatedAssaultAnalyzer {
    /// Analyze s.47 ABH
    pub fn analyze_abh(facts: &ABHFacts) -> CriminalResult<AggravatedAssaultResult> {
        let case_law = vec![
            CaseCitation::new(
                "R v Savage; DPP v Parmenter",
                1992,
                "1 AC 699",
                "ABH = any hurt calculated to interfere with health/comfort; no need for MR as to ABH",
            ),
            CaseCitation::new(
                "R v Chan Fook",
                1994,
                "1 WLR 689",
                "Psychiatric injury can amount to ABH if evidence of identifiable clinical condition",
            ),
        ];

        let mut established = true;
        let mut alternatives = Vec::new();

        // Check base offence (assault or battery)
        let base_established = match &facts.assault_battery {
            AssaultOrBattery::Assault(a) => a.mens_rea.intentional || a.mens_rea.reckless,
            AssaultOrBattery::Battery(b) => b.mens_rea.intentional || b.mens_rea.reckless,
        };

        if !base_established {
            established = false;
        } else {
            alternatives.push(NonFatalOffence::CommonAssault);
            alternatives.push(NonFatalOffence::Battery);
        }

        // Check harm
        let harm_analysis = if facts.harm.more_than_transient {
            format!(
                "ABH established: {} ({:?} injury more than transient/trifling)",
                facts.harm.description, facts.harm.harm_type
            )
        } else {
            established = false;
            format!(
                "Harm '{}' is transient/trifling - not ABH",
                facts.harm.description
            )
        };

        // Mens rea (only need MR for assault/battery, not ABH)
        let mens_rea = MensReaAnalysis {
            mens_rea_type: MensReaType::DirectIntention,
            established: facts.mens_rea.assault_battery_mr,
            evidence: facts.mens_rea.evidence.clone(),
            reasoning: "For ABH, only need mens rea for assault/battery - no need to foresee ABH \
                       (R v Savage; Parmenter)"
                .into(),
            case_law: vec![],
        };

        Ok(AggravatedAssaultResult {
            established,
            offence: NonFatalOffence::AssaultOccasioningABH,
            alternatives,
            harm_analysis,
            mens_rea,
            case_law,
        })
    }

    /// Analyze s.20 GBH/wounding
    pub fn analyze_section20(facts: &Section20Facts) -> CriminalResult<AggravatedAssaultResult> {
        let case_law = vec![
            CaseCitation::new(
                "R v Cunningham",
                1957,
                "2 QB 396",
                "'Maliciously' means intention or subjective recklessness as to some harm",
            ),
            CaseCitation::new(
                "DPP v Smith",
                1961,
                "AC 290",
                "GBH means really serious harm",
            ),
            CaseCitation::new(
                "JCC v Eisenhower",
                1984,
                "QB 331",
                "Wound requires break in continuity of whole skin",
            ),
            CaseCitation::new(
                "R v Burstow",
                1998,
                "AC 147",
                "GBH can include serious psychiatric injury; 'inflict' doesn't require assault",
            ),
        ];

        let mut established = true;
        let mut alternatives = vec![];

        // Check harm
        let harm_analysis = match facts.harm.harm_type {
            Section20HarmType::Wounding => {
                "Wounding: break in continuity of whole skin (JCC v Eisenhower)".into()
            }
            Section20HarmType::GBH => {
                format!(
                    "GBH: {} - really serious harm (DPP v Smith)",
                    facts.harm.description
                )
            }
            Section20HarmType::Both => "Both wounding and GBH established".into(),
        };

        // Check infliction
        if !facts.infliction.causation {
            established = false;
        }

        // Check mens rea ("maliciously" = intention/recklessness to SOME harm)
        let mens_rea_established =
            facts.mens_rea.intention_to_harm || facts.mens_rea.reckless_to_harm;
        let mens_rea = MensReaAnalysis {
            mens_rea_type: if facts.mens_rea.intention_to_harm {
                MensReaType::DirectIntention
            } else {
                MensReaType::SubjectiveRecklessness
            },
            established: mens_rea_established,
            evidence: facts.mens_rea.evidence.clone(),
            reasoning: if mens_rea_established {
                "D intended or was reckless as to causing SOME harm (need not foresee GBH - \
                 Cunningham/Mowatt)"
                    .into()
            } else {
                "No mens rea for causing harm established".into()
            },
            case_law: vec![],
        };

        if !mens_rea_established {
            established = false;
        }

        // ABH as alternative
        alternatives.push(NonFatalOffence::AssaultOccasioningABH);

        Ok(AggravatedAssaultResult {
            established,
            offence: NonFatalOffence::Section20GBH,
            alternatives,
            harm_analysis,
            mens_rea,
            case_law,
        })
    }

    /// Analyze s.18 GBH with intent
    pub fn analyze_section18(facts: &Section18Facts) -> CriminalResult<AggravatedAssaultResult> {
        let case_law = vec![CaseCitation::new(
            "R v Belfon",
            1976,
            "1 WLR 741",
            "S.18 requires specific intent to cause GBH (or intent to resist arrest)",
        )];

        let mut established = true;
        let mut alternatives = vec![];

        // Check harm (same as s.20)
        let harm_analysis = match facts.harm.harm_type {
            Section20HarmType::Wounding => "Wounding established".into(),
            Section20HarmType::GBH => {
                format!("GBH: {} - really serious harm", facts.harm.description)
            }
            Section20HarmType::Both => "Both wounding and GBH established".into(),
        };

        // Check causation
        if !facts.causation.established {
            established = false;
        }

        // Check specific intent (this is what distinguishes s.18 from s.20)
        let specific_intent = facts.intent.intent_gbh || facts.intent.intent_resist_arrest;
        let mens_rea = MensReaAnalysis {
            mens_rea_type: MensReaType::DirectIntention,
            established: specific_intent,
            evidence: facts.intent.evidence.clone(),
            reasoning: if specific_intent {
                if facts.intent.intent_gbh {
                    "D intended to cause GBH (specific intent offence)".into()
                } else {
                    "D intended to resist/prevent lawful arrest".into()
                }
            } else {
                "No specific intent to cause GBH or resist arrest - consider s.20 as alternative"
                    .into()
            },
            case_law: vec![],
        };

        if !specific_intent {
            established = false;
            // S.20 is the main alternative
            alternatives.push(NonFatalOffence::Section20GBH);
        }
        alternatives.push(NonFatalOffence::AssaultOccasioningABH);

        Ok(AggravatedAssaultResult {
            established,
            offence: NonFatalOffence::Section18GBH,
            alternatives,
            harm_analysis,
            mens_rea,
            case_law,
        })
    }
}

// ============================================================================
// Disease Transmission (R v Dica)
// ============================================================================

/// Facts for disease transmission as GBH
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiseaseTransmissionFacts {
    /// Disease transmitted
    pub disease: String,
    /// Severity of disease
    pub severity: DiseaseSeverity,
    /// Knowledge of infection
    pub knowledge: DiseaseKnowledge,
    /// Consent to risk
    pub consent_to_risk: Option<ConsentToRiskFacts>,
}

/// Severity of disease
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiseaseSeverity {
    /// Serious disease (e.g., HIV) - GBH
    Serious,
    /// Moderate disease - may be ABH
    Moderate,
    /// Minor disease
    Minor,
}

/// D's knowledge of infection
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiseaseKnowledge {
    /// Did D know of infection?
    pub knew_infected: bool,
    /// Was D reckless as to infection?
    pub reckless: bool,
    /// Evidence
    pub evidence: Vec<String>,
}

/// Consent to risk facts (R v Konzani)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsentToRiskFacts {
    /// Was V informed of risk?
    pub informed: bool,
    /// Did V consent to specific risk?
    pub consented_to_risk: bool,
    /// Evidence
    pub evidence: Vec<String>,
}

/// Result of disease transmission analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiseaseTransmissionResult {
    /// Offence established?
    pub established: bool,
    /// Which offence?
    pub offence: NonFatalOffence,
    /// Consent defence available?
    pub consent_defence: Option<DefenceResult>,
    /// Key findings
    pub findings: Vec<String>,
    /// Key case law
    pub case_law: Vec<CaseCitation>,
}

/// Analyzer for disease transmission
pub struct DiseaseTransmissionAnalyzer;

impl DiseaseTransmissionAnalyzer {
    /// Analyze disease transmission as assault
    pub fn analyze(facts: &DiseaseTransmissionFacts) -> CriminalResult<DiseaseTransmissionResult> {
        let case_law = vec![
            CaseCitation::new(
                "R v Dica",
                2004,
                "EWCA Crim 1103",
                "Reckless transmission of HIV can amount to s.20 GBH",
            ),
            CaseCitation::new(
                "R v Konzani",
                2005,
                "EWCA Crim 706",
                "Informed consent to risk of infection is valid defence; uninformed consent is not",
            ),
        ];

        let mut findings = Vec::new();
        let mut established = true;

        // Determine offence level based on severity
        let offence = match facts.severity {
            DiseaseSeverity::Serious => {
                findings.push(format!(
                    "Serious disease ({}) amounts to GBH",
                    facts.disease
                ));
                NonFatalOffence::Section20GBH
            }
            DiseaseSeverity::Moderate => {
                findings.push(format!(
                    "Moderate disease ({}) may amount to ABH",
                    facts.disease
                ));
                NonFatalOffence::AssaultOccasioningABH
            }
            DiseaseSeverity::Minor => {
                findings.push("Minor disease unlikely to meet threshold".into());
                established = false;
                NonFatalOffence::Battery
            }
        };

        // Check mens rea
        if !facts.knowledge.knew_infected && !facts.knowledge.reckless {
            established = false;
            findings.push("D neither knew of infection nor was reckless".into());
        } else if facts.knowledge.knew_infected {
            findings.push("D knew of infection".into());
        } else {
            findings.push("D was reckless as to infection status".into());
        }

        // Check consent defence
        let consent_defence = if let Some(consent) = &facts.consent_to_risk {
            let available = consent.informed && consent.consented_to_risk;
            if available {
                established = false;
                findings.push("V gave informed consent to risk (R v Konzani defence)".into());
            } else if !consent.informed {
                findings.push("V's consent was uninformed - no valid defence (R v Konzani)".into());
            }

            Some(DefenceResult {
                defence_type: DefenceType::Consent,
                available,
                effect: if available {
                    Some(crate::criminal::types::DefenceEffect::Acquittal)
                } else {
                    None
                },
                findings: vec![if available {
                    "Informed consent to risk of infection".into()
                } else {
                    "No informed consent - concealment negates consent".into()
                }],
                case_law: vec![CaseCitation::new(
                    "R v Konzani",
                    2005,
                    "EWCA Crim 706",
                    "Informed consent required for defence to reckless transmission",
                )],
            })
        } else {
            None
        };

        Ok(DiseaseTransmissionResult {
            established,
            offence,
            consent_defence,
            findings,
            case_law,
        })
    }
}

// ============================================================================
// Offence Definitions
// ============================================================================

/// Get common assault offence definition
pub fn common_assault_offence() -> CriminalResult<Offence> {
    OffenceBuilder::new()
        .name("Common Assault")
        .statutory_source("Criminal Justice Act 1988", "s.39")
        .classification(OffenceClassification::Summary)
        .category(OffenceCategory::AgainstPerson)
        .severity(OffenceSeverity::Minor)
        .maximum_sentence(MaximumSentence::Months(6))
        .mens_rea(MensReaType::DirectIntention)
        .actus_reus(ActusReusElement::Act(ActType::VoluntaryAct))
        .build()
        .map_err(CriminalError::InvalidInput)
}

/// Get battery offence definition
pub fn battery_offence() -> CriminalResult<Offence> {
    OffenceBuilder::new()
        .name("Battery")
        .common_law()
        .classification(OffenceClassification::Summary)
        .category(OffenceCategory::AgainstPerson)
        .severity(OffenceSeverity::Minor)
        .maximum_sentence(MaximumSentence::Months(6))
        .mens_rea(MensReaType::DirectIntention)
        .actus_reus(ActusReusElement::Act(ActType::VoluntaryAct))
        .build()
        .map_err(CriminalError::InvalidInput)
}

/// Get s.47 ABH offence definition
pub fn abh_offence() -> CriminalResult<Offence> {
    OffenceBuilder::new()
        .name("Assault Occasioning Actual Bodily Harm")
        .statutory_source("Offences Against the Person Act 1861", "s.47")
        .classification(OffenceClassification::EitherWay)
        .category(OffenceCategory::AgainstPerson)
        .severity(OffenceSeverity::Moderate)
        .maximum_sentence(MaximumSentence::Years(5))
        .mens_rea(MensReaType::DirectIntention)
        .actus_reus(ActusReusElement::Act(ActType::VoluntaryAct))
        .actus_reus(ActusReusElement::Consequence("Actual bodily harm".into()))
        .build()
        .map_err(CriminalError::InvalidInput)
}

/// Get s.20 GBH offence definition
pub fn section20_offence() -> CriminalResult<Offence> {
    OffenceBuilder::new()
        .name("Malicious Wounding / Inflicting GBH")
        .statutory_source("Offences Against the Person Act 1861", "s.20")
        .classification(OffenceClassification::EitherWay)
        .category(OffenceCategory::AgainstPerson)
        .severity(OffenceSeverity::Serious)
        .maximum_sentence(MaximumSentence::Years(5))
        .mens_rea(MensReaType::SubjectiveRecklessness)
        .actus_reus(ActusReusElement::Act(ActType::VoluntaryAct))
        .actus_reus(ActusReusElement::Consequence("Wounding or GBH".into()))
        .build()
        .map_err(CriminalError::InvalidInput)
}

/// Get s.18 GBH offence definition
pub fn section18_offence() -> CriminalResult<Offence> {
    OffenceBuilder::new()
        .name("Wounding / Causing GBH with Intent")
        .statutory_source("Offences Against the Person Act 1861", "s.18")
        .classification(OffenceClassification::IndictableOnly)
        .category(OffenceCategory::AgainstPerson)
        .severity(OffenceSeverity::VerySerious)
        .maximum_sentence(MaximumSentence::Life)
        .mens_rea(MensReaType::DirectIntention)
        .actus_reus(ActusReusElement::Act(ActType::VoluntaryAct))
        .actus_reus(ActusReusElement::Consequence("Wounding or GBH".into()))
        .build()
        .map_err(CriminalError::InvalidInput)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assault_by_words() {
        let facts = AssaultFacts {
            conduct: AssaultConduct::Words {
                words: "I'm going to kill you".into(),
            },
            apprehension: ApprehensionDetails {
                apprehended_violence: true,
                immediate: true,
                evidence: vec!["Victim feared immediate attack".into()],
            },
            mens_rea: AssaultMensRea {
                intentional: true,
                reckless: false,
                evidence: vec!["D intended to cause fear".into()],
            },
            consent: None,
            self_defence: None,
        };

        let result = AssaultBatteryAnalyzer::analyze_assault(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.established);
    }

    #[test]
    fn test_battery_direct_force() {
        let facts = BatteryFacts {
            force: ForceDetails {
                description: "Punched victim in face".into(),
                force_type: ForceType::DirectTouch,
                unlawful: true,
            },
            mens_rea: BatteryMensRea {
                intentional: true,
                reckless: false,
                evidence: vec!["Deliberate punch".into()],
            },
            consent: None,
            self_defence: None,
        };

        let result = AssaultBatteryAnalyzer::analyze_battery(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.established);
    }

    #[test]
    fn test_consent_defence_sport() {
        let facts = BatteryFacts {
            force: ForceDetails {
                description: "Tackle during rugby match".into(),
                force_type: ForceType::DirectTouch,
                unlawful: true,
            },
            mens_rea: BatteryMensRea {
                intentional: true,
                reckless: false,
                evidence: vec![],
            },
            consent: Some(ConsentFacts {
                consent_given: true,
                activity_type: ConsentActivityType::Sport {
                    sport: "Rugby".into(),
                },
                valid_consent: true,
                invalidity_reasons: vec![],
            }),
            self_defence: None,
        };

        let result = AssaultBatteryAnalyzer::analyze_battery(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        // Consent should negate battery
        assert!(!analysis.established);
    }

    #[test]
    fn test_abh_analysis() {
        let facts = ABHFacts {
            assault_battery: AssaultOrBattery::Battery(BatteryFacts {
                force: ForceDetails {
                    description: "Pushed victim causing bruising".into(),
                    force_type: ForceType::DirectTouch,
                    unlawful: true,
                },
                mens_rea: BatteryMensRea {
                    intentional: true,
                    reckless: false,
                    evidence: vec![],
                },
                consent: None,
                self_defence: None,
            }),
            harm: ABHHarm {
                description: "Multiple bruises and minor cuts".into(),
                harm_type: ABHHarmType::Physical,
                more_than_transient: true,
            },
            mens_rea: ABHMensRea {
                assault_battery_mr: true,
                evidence: vec!["Intentional push".into()],
            },
        };

        let result = AggravatedAssaultAnalyzer::analyze_abh(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.established);
    }

    #[test]
    fn test_section20_analysis() {
        let facts = Section20Facts {
            harm: Section20Harm {
                description: "Deep laceration requiring stitches".into(),
                harm_type: Section20HarmType::Wounding,
                evidence: vec!["Medical report".into()],
            },
            infliction: InflictionDetails {
                method: "Slashed with knife".into(),
                direct: true,
                causation: true,
            },
            mens_rea: Section20MensRea {
                intention_to_harm: false,
                reckless_to_harm: true,
                evidence: vec!["D swung knife recklessly".into()],
            },
        };

        let result = AggravatedAssaultAnalyzer::analyze_section20(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.established);
    }

    #[test]
    fn test_section18_requires_specific_intent() {
        let facts = Section18Facts {
            harm: Section20Harm {
                description: "Fractured skull".into(),
                harm_type: Section20HarmType::GBH,
                evidence: vec!["Medical report".into()],
            },
            causation: CausationDetails {
                method: "Hit with baseball bat".into(),
                established: true,
            },
            intent: Section18Intent {
                intent_gbh: false,
                intent_resist_arrest: false,
                evidence: vec!["No evidence of specific intent".into()],
            },
        };

        let result = AggravatedAssaultAnalyzer::analyze_section18(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        // Without specific intent, s.18 not established
        assert!(!analysis.established);
        // But s.20 should be alternative
        assert!(
            analysis
                .alternatives
                .contains(&NonFatalOffence::Section20GBH)
        );
    }

    #[test]
    fn test_disease_transmission() {
        let facts = DiseaseTransmissionFacts {
            disease: "HIV".into(),
            severity: DiseaseSeverity::Serious,
            knowledge: DiseaseKnowledge {
                knew_infected: true,
                reckless: false,
                evidence: vec!["D had been diagnosed".into()],
            },
            consent_to_risk: Some(ConsentToRiskFacts {
                informed: false,
                consented_to_risk: false,
                evidence: vec!["D concealed status".into()],
            }),
        };

        let result = DiseaseTransmissionAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.established);
        assert!(matches!(analysis.offence, NonFatalOffence::Section20GBH));
    }

    #[test]
    fn test_informed_consent_to_disease_risk() {
        let facts = DiseaseTransmissionFacts {
            disease: "HIV".into(),
            severity: DiseaseSeverity::Serious,
            knowledge: DiseaseKnowledge {
                knew_infected: true,
                reckless: false,
                evidence: vec![],
            },
            consent_to_risk: Some(ConsentToRiskFacts {
                informed: true,
                consented_to_risk: true,
                evidence: vec!["V knew of D's status and consented".into()],
            }),
        };

        let result = DiseaseTransmissionAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        // Informed consent = defence
        assert!(!analysis.established);
    }

    #[test]
    fn test_offence_definitions() {
        assert!(common_assault_offence().is_ok());
        assert!(battery_offence().is_ok());
        assert!(abh_offence().is_ok());
        assert!(section20_offence().is_ok());
        assert!(section18_offence().is_ok());
    }

    #[test]
    fn test_self_defence() {
        let facts = BatteryFacts {
            force: ForceDetails {
                description: "Pushed attacker away".into(),
                force_type: ForceType::DirectTouch,
                unlawful: true,
            },
            mens_rea: BatteryMensRea {
                intentional: true,
                reckless: false,
                evidence: vec![],
            },
            consent: None,
            self_defence: Some(SelfDefenceFacts {
                honest_belief: true,
                necessity: true,
                proportionate: true,
                believed_circumstances: "Attacker was about to punch me".into(),
                force_used: "Single push".into(),
            }),
        };

        let result = AssaultBatteryAnalyzer::analyze_battery(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        // Self-defence should negate battery
        assert!(!analysis.established);
    }
}
