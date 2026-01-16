//! UK Criminal Defences - Insanity and Related Defences
//!
//! This module covers the insanity defence under the M'Naghten Rules
//! and related mental condition defences.
//!
//! # Legal Framework
//!
//! ## M'Naghten Rules (1843)
//! - Every person presumed sane until contrary proved
//! - D must establish on balance of probabilities:
//!   1. Defect of reason
//!   2. From disease of the mind
//!   3. Such that D did not know nature/quality of act OR did not know it was wrong
//!
//! ## Key Cases
//! - M'Naghten's Case (1843) - Foundation of insanity defence
//! - R v Kemp [1957] - "Disease of mind" broadly interpreted
//! - R v Sullivan [1984] - Epilepsy can be disease of mind
//! - R v Burgess [1991] - Sleepwalking = disease of mind
//! - R v Hennessy [1989] - Hyperglycaemia = disease of mind
//!
//! ## Verdict and Consequences
//! - Special verdict: "Not guilty by reason of insanity"
//! - Criminal Procedure (Insanity) Act 1964 (as amended)
//! - Disposal options: hospital order, supervision order, absolute discharge

// Allow missing docs on enum variant struct fields - these are self-documenting in context
#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use crate::criminal::error::CriminalResult;
use crate::criminal::types::{CaseCitation, DefenceEffect, DefenceResult, DefenceType};

// ============================================================================
// Insanity (M'Naghten Rules)
// ============================================================================

/// Facts for insanity defence analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InsanityFacts {
    /// Defect of reason
    pub defect_of_reason: DefectOfReasonFacts,
    /// Disease of mind
    pub disease_of_mind: DiseaseOfMindFacts,
    /// M'Naghten limbs
    pub mcnaghten_limbs: McNaghtenLimbs,
    /// Expert psychiatric evidence
    pub psychiatric_evidence: Vec<PsychiatricEvidence>,
}

/// Defect of reason facts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DefectOfReasonFacts {
    /// Was there a defect of reason?
    pub defect_present: bool,
    /// Nature of defect
    pub defect_description: String,
    /// Was it total or partial?
    pub total_defect: bool,
    /// Momentary or persistent?
    pub momentary: bool,
}

/// Disease of mind facts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiseaseOfMindFacts {
    /// Was there a disease of mind?
    pub disease_present: bool,
    /// Medical diagnosis (if any)
    pub diagnosis: Option<String>,
    /// Internal or external cause?
    pub cause_type: MindDiseaseCause,
    /// Evidence
    pub evidence: Vec<String>,
}

/// Cause type for disease of mind
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MindDiseaseCause {
    /// Internal cause (disease of mind)
    Internal {
        /// Name of the medical condition
        condition: String,
    },
    /// External cause (automatism, not insanity)
    External {
        /// Description of the external cause
        cause: String,
    },
    /// Mixed or unclear
    Mixed {
        /// Description of the mixed causation
        description: String,
    },
}

/// M'Naghten limbs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct McNaghtenLimbs {
    /// First limb: Did not know nature and quality of act
    pub first_limb: FirstLimbFacts,
    /// Second limb: Did not know act was wrong
    pub second_limb: SecondLimbFacts,
}

/// First limb: nature and quality
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FirstLimbFacts {
    /// Did D know the physical nature of the act?
    pub knew_physical_nature: bool,
    /// Did D understand what they were doing?
    pub understood_act: bool,
    /// Details
    pub details: String,
}

/// Second limb: knowledge of wrong
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecondLimbFacts {
    /// Did D know act was legally wrong?
    pub knew_legally_wrong: bool,
    /// Did D know act was morally wrong? (R v Windle interpretation)
    pub knew_morally_wrong: bool,
    /// Details
    pub details: String,
}

/// Psychiatric evidence
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PsychiatricEvidence {
    /// Expert name/qualification
    pub expert: String,
    /// Diagnosis
    pub diagnosis: String,
    /// Opinion on M'Naghten
    pub mcnaghten_opinion: String,
    /// Supporting evidence
    pub evidence: Vec<String>,
}

/// Result of insanity defence analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InsanityResult {
    /// Defence available?
    pub available: bool,
    /// Defect of reason finding
    pub defect_finding: String,
    /// Disease of mind finding
    pub disease_finding: String,
    /// M'Naghten limbs finding
    pub limbs_finding: McNaghtenLimbsResult,
    /// Verdict
    pub verdict: InsanityVerdict,
    /// Disposal options
    pub disposal_options: Vec<InsanityDisposal>,
    /// Key case law
    pub case_law: Vec<CaseCitation>,
}

/// M'Naghten limbs result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct McNaghtenLimbsResult {
    /// First limb satisfied?
    pub first_limb_satisfied: bool,
    /// First limb finding
    pub first_limb_finding: String,
    /// Second limb satisfied?
    pub second_limb_satisfied: bool,
    /// Second limb finding
    pub second_limb_finding: String,
}

/// Insanity verdict
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InsanityVerdict {
    /// Special verdict: not guilty by reason of insanity
    NotGuiltyByReasonOfInsanity,
    /// Insanity not established
    InsanityNotEstablished,
    /// Consider automatism instead (external cause)
    ConsiderAutomatism,
}

/// Disposal options under CP(I)A 1964
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InsanityDisposal {
    /// Hospital order (with or without restrictions)
    HospitalOrder {
        /// Whether restrictions apply (s.41 MHA)
        with_restrictions: bool,
    },
    /// Supervision order
    SupervisionOrder,
    /// Absolute discharge
    AbsoluteDischarge,
}

/// Analyzer for insanity defence
pub struct InsanityAnalyzer;

impl InsanityAnalyzer {
    /// Analyze insanity defence
    pub fn analyze(facts: &InsanityFacts) -> CriminalResult<InsanityResult> {
        let mut case_law = vec![
            CaseCitation::new(
                "M'Naghten's Case",
                1843,
                "10 Cl & Fin 200",
                "Foundation of insanity defence - defect of reason from disease of mind",
            ),
            CaseCitation::new(
                "R v Kemp",
                1957,
                "1 QB 399",
                "Disease of mind includes any disease affecting mind, not just brain disease",
            ),
        ];

        // Analyze defect of reason
        let defect_finding = Self::analyze_defect(&facts.defect_of_reason);
        if !facts.defect_of_reason.defect_present {
            return Ok(InsanityResult {
                available: false,
                defect_finding,
                disease_finding: String::new(),
                limbs_finding: McNaghtenLimbsResult {
                    first_limb_satisfied: false,
                    first_limb_finding: String::new(),
                    second_limb_satisfied: false,
                    second_limb_finding: String::new(),
                },
                verdict: InsanityVerdict::InsanityNotEstablished,
                disposal_options: vec![],
                case_law,
            });
        }

        // Analyze disease of mind
        let (disease_finding, is_internal) =
            Self::analyze_disease(&facts.disease_of_mind, &mut case_law);

        if !is_internal {
            return Ok(InsanityResult {
                available: false,
                defect_finding,
                disease_finding,
                limbs_finding: McNaghtenLimbsResult {
                    first_limb_satisfied: false,
                    first_limb_finding: String::new(),
                    second_limb_satisfied: false,
                    second_limb_finding: String::new(),
                },
                verdict: InsanityVerdict::ConsiderAutomatism,
                disposal_options: vec![],
                case_law,
            });
        }

        // Analyze M'Naghten limbs
        let limbs_finding = Self::analyze_limbs(&facts.mcnaghten_limbs, &mut case_law);

        // Determine if either limb satisfied
        let either_limb_satisfied =
            limbs_finding.first_limb_satisfied || limbs_finding.second_limb_satisfied;

        let available = facts.defect_of_reason.defect_present
            && facts.disease_of_mind.disease_present
            && either_limb_satisfied;

        let verdict = if available {
            InsanityVerdict::NotGuiltyByReasonOfInsanity
        } else {
            InsanityVerdict::InsanityNotEstablished
        };

        let disposal_options = if available {
            vec![
                InsanityDisposal::HospitalOrder {
                    with_restrictions: false,
                },
                InsanityDisposal::SupervisionOrder,
                InsanityDisposal::AbsoluteDischarge,
            ]
        } else {
            vec![]
        };

        Ok(InsanityResult {
            available,
            defect_finding,
            disease_finding,
            limbs_finding,
            verdict,
            disposal_options,
            case_law,
        })
    }

    fn analyze_defect(facts: &DefectOfReasonFacts) -> String {
        if !facts.defect_present {
            return "No defect of reason present".into();
        }

        let mut parts = vec![format!("Defect of reason: {}", facts.defect_description)];

        if facts.total_defect {
            parts.push("Total defect of reason".into());
        } else {
            parts.push("Partial defect only - may not suffice".into());
        }

        if facts.momentary {
            parts.push("Momentary lapse - still may qualify if from disease".into());
        }

        parts.join("; ")
    }

    fn analyze_disease(
        facts: &DiseaseOfMindFacts,
        case_law: &mut Vec<CaseCitation>,
    ) -> (String, bool) {
        if !facts.disease_present {
            return ("No disease of mind present".into(), false);
        }

        let (description, is_internal) = match &facts.cause_type {
            MindDiseaseCause::Internal { condition } => {
                let desc = if let Some(diag) = &facts.diagnosis {
                    format!(
                        "Disease of mind established: {} (internal condition: {})",
                        diag, condition
                    )
                } else {
                    format!("Disease of mind from internal condition: {}", condition)
                };
                (desc, true)
            }
            MindDiseaseCause::External { cause } => {
                case_law.push(CaseCitation::new(
                    "R v Quick",
                    1973,
                    "QB 910",
                    "External cause = automatism not insanity",
                ));
                (
                    format!(
                        "External cause ({}) - not disease of mind, consider automatism",
                        cause
                    ),
                    false,
                )
            }
            MindDiseaseCause::Mixed { description } => (
                format!(
                    "Mixed/unclear cause: {} - further analysis needed",
                    description
                ),
                false,
            ),
        };

        (description, is_internal)
    }

    fn analyze_limbs(
        facts: &McNaghtenLimbs,
        case_law: &mut Vec<CaseCitation>,
    ) -> McNaghtenLimbsResult {
        // First limb: nature and quality
        let first_limb_satisfied =
            !facts.first_limb.knew_physical_nature || !facts.first_limb.understood_act;

        let first_limb_finding = if first_limb_satisfied {
            format!(
                "First limb satisfied: D did not know nature/quality - {}",
                facts.first_limb.details
            )
        } else {
            "First limb NOT satisfied: D knew nature and quality of act".into()
        };

        // Second limb: knowledge of wrong
        // R v Windle: "wrong" means legally wrong
        case_law.push(CaseCitation::new(
            "R v Windle",
            1952,
            "2 QB 826",
            "'Wrong' means contrary to law, not morally wrong",
        ));

        let second_limb_satisfied = !facts.second_limb.knew_legally_wrong;

        let second_limb_finding = if second_limb_satisfied {
            format!(
                "Second limb satisfied: D did not know act was legally wrong - {}",
                facts.second_limb.details
            )
        } else {
            "Second limb NOT satisfied: D knew act was legally wrong".into()
        };

        McNaghtenLimbsResult {
            first_limb_satisfied,
            first_limb_finding,
            second_limb_satisfied,
            second_limb_finding,
        }
    }
}

// ============================================================================
// Unfitness to Plead
// ============================================================================

/// Facts for unfitness to plead analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnfitnessToPlead {
    /// Pritchard criteria
    pub pritchard_criteria: PritchardCriteria,
    /// Medical evidence
    pub medical_evidence: Vec<PsychiatricEvidence>,
    /// Current mental state
    pub current_state: CurrentMentalState,
}

/// Pritchard criteria for fitness to plead
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PritchardCriteria {
    /// Can D understand the charge?
    pub understand_charge: bool,
    /// Can D plead to the indictment?
    pub can_plead: bool,
    /// Can D challenge jurors?
    pub can_challenge_jurors: bool,
    /// Can D instruct counsel?
    pub can_instruct_counsel: bool,
    /// Can D understand evidence?
    pub can_understand_evidence: bool,
    /// Can D give evidence?
    pub can_give_evidence: bool,
}

/// Current mental state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentMentalState {
    /// Diagnosis
    pub diagnosis: Option<String>,
    /// Severity
    pub severity: MentalStateSeverity,
    /// Treatment status
    pub treatment: TreatmentStatus,
}

/// Severity of mental state
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MentalStateSeverity {
    /// Mild impairment
    Mild,
    /// Moderate impairment
    Moderate,
    /// Severe impairment
    Severe,
    /// Profound impairment
    Profound,
}

/// Treatment status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TreatmentStatus {
    /// Not receiving treatment
    None,
    /// Outpatient treatment
    Outpatient,
    /// Inpatient treatment
    Inpatient,
    /// Secure hospital
    SecureHospital,
}

/// Result of unfitness to plead analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnfitnessResult {
    /// Is D unfit to plead?
    pub unfit: bool,
    /// Criteria analysis
    pub criteria_analysis: Vec<String>,
    /// Recommendation
    pub recommendation: UnfitnessRecommendation,
    /// Key case law
    pub case_law: Vec<CaseCitation>,
}

/// Unfitness recommendation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnfitnessRecommendation {
    /// Fit to plead - proceed with trial
    FitToProceed,
    /// Unfit - hold finding of fact hearing
    FindingOfFactHearing,
    /// Postpone determination
    Postpone { reason: String },
}

/// Analyzer for unfitness to plead
pub struct UnfitnessAnalyzer;

impl UnfitnessAnalyzer {
    /// Analyze unfitness to plead
    pub fn analyze(facts: &UnfitnessToPlead) -> CriminalResult<UnfitnessResult> {
        let case_law = vec![
            CaseCitation::new(
                "R v Pritchard",
                1836,
                "7 C & P 303",
                "Six criteria for fitness to plead",
            ),
            CaseCitation::new(
                "R v Dyson",
                1831,
                "7 C & P 305",
                "Fitness determined at time of trial",
            ),
        ];

        let pc = &facts.pritchard_criteria;

        let mut criteria_analysis = Vec::new();
        let mut failures = 0;

        // Check each Pritchard criterion
        if !pc.understand_charge {
            criteria_analysis.push("Cannot understand the charge".into());
            failures += 1;
        }
        if !pc.can_plead {
            criteria_analysis.push("Cannot plead to indictment".into());
            failures += 1;
        }
        if !pc.can_challenge_jurors {
            criteria_analysis.push("Cannot challenge jurors".into());
            failures += 1;
        }
        if !pc.can_instruct_counsel {
            criteria_analysis.push("Cannot instruct counsel".into());
            failures += 1;
        }
        if !pc.can_understand_evidence {
            criteria_analysis.push("Cannot understand evidence".into());
            failures += 1;
        }
        if !pc.can_give_evidence {
            criteria_analysis.push("Cannot give evidence".into());
            failures += 1;
        }

        let unfit = failures > 0;

        let recommendation = if unfit {
            UnfitnessRecommendation::FindingOfFactHearing
        } else {
            UnfitnessRecommendation::FitToProceed
        };

        if criteria_analysis.is_empty() {
            criteria_analysis.push("All Pritchard criteria satisfied - fit to plead".into());
        }

        Ok(UnfitnessResult {
            unfit,
            criteria_analysis,
            recommendation,
            case_law,
        })
    }
}

// ============================================================================
// Infanticide
// ============================================================================

/// Facts for infanticide analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InfanticideFacts {
    /// Was victim under 12 months?
    pub child_under_12_months: bool,
    /// Was mother the killer?
    pub mother_killed: bool,
    /// Balance of mind disturbed?
    pub balance_disturbed: BalanceDisturbedFacts,
}

/// Balance of mind disturbed facts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BalanceDisturbedFacts {
    /// Was balance of mind disturbed?
    pub disturbed: bool,
    /// Reason for disturbance
    pub reason: BalanceDisturbanceReason,
    /// Medical evidence
    pub medical_evidence: Vec<String>,
}

/// Reason for balance disturbance
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BalanceDisturbanceReason {
    /// Effect of giving birth
    GivingBirth,
    /// Effect of lactation
    Lactation,
    /// Disorder consequent on birth
    ConsequentDisorder { disorder: String },
}

/// Result of infanticide analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InfanticideResult {
    /// Infanticide available?
    pub available: bool,
    /// Child age finding
    pub child_finding: String,
    /// Mother finding
    pub mother_finding: String,
    /// Balance finding
    pub balance_finding: String,
    /// Maximum sentence
    pub maximum_sentence: String,
    /// Key case law
    pub case_law: Vec<CaseCitation>,
}

/// Analyzer for infanticide
pub struct InfanticideAnalyzer;

impl InfanticideAnalyzer {
    /// Analyze infanticide
    pub fn analyze(facts: &InfanticideFacts) -> CriminalResult<InfanticideResult> {
        let case_law = vec![CaseCitation::new(
            "Infanticide Act 1938",
            1938,
            "s.1",
            "Infanticide where mother kills child under 12 months while balance disturbed",
        )];

        let child_finding = if facts.child_under_12_months {
            "Child was under 12 months".into()
        } else {
            "Child was 12 months or over - infanticide not available".into()
        };

        let mother_finding = if facts.mother_killed {
            "Mother was the killer".into()
        } else {
            "Killer was not the mother - infanticide not available".into()
        };

        let balance_finding = if facts.balance_disturbed.disturbed {
            let reason = match &facts.balance_disturbed.reason {
                BalanceDisturbanceReason::GivingBirth => "effect of giving birth".to_string(),
                BalanceDisturbanceReason::Lactation => "effect of lactation".to_string(),
                BalanceDisturbanceReason::ConsequentDisorder { disorder } => {
                    format!("disorder consequent on birth: {}", disorder)
                }
            };
            format!("Balance of mind disturbed by reason of {}", reason)
        } else {
            "Balance of mind not disturbed - infanticide not available".into()
        };

        let available =
            facts.child_under_12_months && facts.mother_killed && facts.balance_disturbed.disturbed;

        Ok(InfanticideResult {
            available,
            child_finding,
            mother_finding,
            balance_finding,
            maximum_sentence: "Life imprisonment (but typically non-custodial)".into(),
            case_law,
        })
    }
}

// ============================================================================
// Helper function to convert to DefenceResult
// ============================================================================

/// Convert insanity result to standard defence result
pub fn insanity_to_defence_result(result: InsanityResult) -> DefenceResult {
    DefenceResult {
        defence_type: DefenceType::Insanity,
        available: result.available,
        effect: if result.available {
            Some(DefenceEffect::SpecialVerdict)
        } else {
            None
        },
        findings: vec![
            result.defect_finding,
            result.disease_finding,
            result.limbs_finding.first_limb_finding,
            result.limbs_finding.second_limb_finding,
        ],
        case_law: result.case_law,
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insanity_both_limbs() {
        let facts = InsanityFacts {
            defect_of_reason: DefectOfReasonFacts {
                defect_present: true,
                defect_description: "Could not reason at all".into(),
                total_defect: true,
                momentary: false,
            },
            disease_of_mind: DiseaseOfMindFacts {
                disease_present: true,
                diagnosis: Some("Paranoid schizophrenia".into()),
                cause_type: MindDiseaseCause::Internal {
                    condition: "Schizophrenia".into(),
                },
                evidence: vec!["Psychiatric report".into()],
            },
            mcnaghten_limbs: McNaghtenLimbs {
                first_limb: FirstLimbFacts {
                    knew_physical_nature: false,
                    understood_act: false,
                    details: "Believed killing demons not humans".into(),
                },
                second_limb: SecondLimbFacts {
                    knew_legally_wrong: false,
                    knew_morally_wrong: false,
                    details: "Believed acting on divine command".into(),
                },
            },
            psychiatric_evidence: vec![],
        };

        let result = InsanityAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.available);
        assert!(matches!(
            analysis.verdict,
            InsanityVerdict::NotGuiltyByReasonOfInsanity
        ));
    }

    #[test]
    fn test_insanity_first_limb_only() {
        let facts = InsanityFacts {
            defect_of_reason: DefectOfReasonFacts {
                defect_present: true,
                defect_description: "Severe delusions".into(),
                total_defect: true,
                momentary: false,
            },
            disease_of_mind: DiseaseOfMindFacts {
                disease_present: true,
                diagnosis: Some("Psychosis".into()),
                cause_type: MindDiseaseCause::Internal {
                    condition: "Psychotic disorder".into(),
                },
                evidence: vec![],
            },
            mcnaghten_limbs: McNaghtenLimbs {
                first_limb: FirstLimbFacts {
                    knew_physical_nature: false,
                    understood_act: false,
                    details: "Did not understand nature of act".into(),
                },
                second_limb: SecondLimbFacts {
                    knew_legally_wrong: true, // Knew it was wrong
                    knew_morally_wrong: true,
                    details: String::new(),
                },
            },
            psychiatric_evidence: vec![],
        };

        let result = InsanityAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        // First limb satisfied = defence available
        assert!(analysis.available);
    }

    #[test]
    fn test_external_cause_not_insanity() {
        let facts = InsanityFacts {
            defect_of_reason: DefectOfReasonFacts {
                defect_present: true,
                defect_description: "Hypoglycaemia from insulin".into(),
                total_defect: true,
                momentary: true,
            },
            disease_of_mind: DiseaseOfMindFacts {
                disease_present: true,
                diagnosis: Some("Hypoglycaemia".into()),
                cause_type: MindDiseaseCause::External {
                    cause: "Insulin injection".into(),
                },
                evidence: vec![],
            },
            mcnaghten_limbs: McNaghtenLimbs {
                first_limb: FirstLimbFacts {
                    knew_physical_nature: false,
                    understood_act: false,
                    details: String::new(),
                },
                second_limb: SecondLimbFacts {
                    knew_legally_wrong: false,
                    knew_morally_wrong: false,
                    details: String::new(),
                },
            },
            psychiatric_evidence: vec![],
        };

        let result = InsanityAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        // External cause = not insanity
        assert!(!analysis.available);
        assert!(matches!(
            analysis.verdict,
            InsanityVerdict::ConsiderAutomatism
        ));
    }

    #[test]
    fn test_unfitness_to_plead() {
        let facts = UnfitnessToPlead {
            pritchard_criteria: PritchardCriteria {
                understand_charge: false,
                can_plead: false,
                can_challenge_jurors: false,
                can_instruct_counsel: false,
                can_understand_evidence: false,
                can_give_evidence: false,
            },
            medical_evidence: vec![],
            current_state: CurrentMentalState {
                diagnosis: Some("Severe learning disability".into()),
                severity: MentalStateSeverity::Profound,
                treatment: TreatmentStatus::Inpatient,
            },
        };

        let result = UnfitnessAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.unfit);
        assert!(matches!(
            analysis.recommendation,
            UnfitnessRecommendation::FindingOfFactHearing
        ));
    }

    #[test]
    fn test_fit_to_plead() {
        let facts = UnfitnessToPlead {
            pritchard_criteria: PritchardCriteria {
                understand_charge: true,
                can_plead: true,
                can_challenge_jurors: true,
                can_instruct_counsel: true,
                can_understand_evidence: true,
                can_give_evidence: true,
            },
            medical_evidence: vec![],
            current_state: CurrentMentalState {
                diagnosis: None,
                severity: MentalStateSeverity::Mild,
                treatment: TreatmentStatus::None,
            },
        };

        let result = UnfitnessAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(!analysis.unfit);
        assert!(matches!(
            analysis.recommendation,
            UnfitnessRecommendation::FitToProceed
        ));
    }

    #[test]
    fn test_infanticide() {
        let facts = InfanticideFacts {
            child_under_12_months: true,
            mother_killed: true,
            balance_disturbed: BalanceDisturbedFacts {
                disturbed: true,
                reason: BalanceDisturbanceReason::ConsequentDisorder {
                    disorder: "Postnatal depression".into(),
                },
                medical_evidence: vec!["Psychiatric assessment".into()],
            },
        };

        let result = InfanticideAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.available);
    }

    #[test]
    fn test_infanticide_child_too_old() {
        let facts = InfanticideFacts {
            child_under_12_months: false, // Over 12 months
            mother_killed: true,
            balance_disturbed: BalanceDisturbedFacts {
                disturbed: true,
                reason: BalanceDisturbanceReason::GivingBirth,
                medical_evidence: vec![],
            },
        };

        let result = InfanticideAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(!analysis.available);
    }

    #[test]
    fn test_defence_result_conversion() {
        let insanity_result = InsanityResult {
            available: true,
            defect_finding: "Defect present".into(),
            disease_finding: "Disease present".into(),
            limbs_finding: McNaghtenLimbsResult {
                first_limb_satisfied: true,
                first_limb_finding: "First limb satisfied".into(),
                second_limb_satisfied: false,
                second_limb_finding: "Second limb not satisfied".into(),
            },
            verdict: InsanityVerdict::NotGuiltyByReasonOfInsanity,
            disposal_options: vec![],
            case_law: vec![],
        };

        let defence_result = insanity_to_defence_result(insanity_result);
        assert!(defence_result.available);
        assert!(matches!(
            defence_result.effect,
            Some(DefenceEffect::SpecialVerdict)
        ));
    }
}
