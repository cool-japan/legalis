//! Canada Tort Law - Negligence Analysis
//!
//! Analyzers for negligence claims using the Anns/Cooper test.

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use super::types::{
    BreachFactor, CausationTest, InterveningCause, NegligenceDefence, PolicyNegation,
    ProximityFactor, RecognizedDutyCategory, RemotenessTest, StandardOfCare, TortCase, TortDamages,
};

// ============================================================================
// Duty of Care Analysis
// ============================================================================

/// Facts for duty of care analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DutyOfCareFacts {
    /// Description of defendant's conduct
    pub defendant_conduct: String,
    /// Description of claimant's injury
    pub claimant_injury: String,
    /// Relationship between parties
    pub relationship: String,
    /// Whether harm was foreseeable
    pub harm_foreseeable: bool,
    /// Proximity factors present
    pub proximity_factors: Vec<ProximityFactor>,
    /// Whether recognized duty category
    pub recognized_category: Option<RecognizedDutyCategory>,
    /// Policy concerns raised
    pub policy_concerns: Vec<PolicyNegation>,
}

/// Result of duty of care analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DutyOfCareResult {
    /// Whether prima facie duty established (Stage 1)
    pub prima_facie_duty: bool,
    /// Foreseeability analysis
    pub foreseeability_satisfied: bool,
    /// Proximity analysis
    pub proximity_satisfied: bool,
    /// Whether policy negates duty (Stage 2)
    pub policy_negates: bool,
    /// Policy reasons that negate
    pub policy_negation_reasons: Vec<PolicyNegation>,
    /// Final duty owed
    pub duty_owed: bool,
    /// Key cases
    pub key_cases: Vec<TortCase>,
    /// Reasoning
    pub reasoning: String,
}

/// Analyzer for duty of care (Anns/Cooper test)
pub struct DutyOfCareAnalyzer;

impl DutyOfCareAnalyzer {
    /// Analyze duty of care using Anns/Cooper test
    pub fn analyze(facts: &DutyOfCareFacts) -> DutyOfCareResult {
        let mut key_cases = vec![TortCase::cooper_v_hobart()];

        // Stage 1: Prima facie duty
        let foreseeability_satisfied = facts.harm_foreseeable;

        let proximity_satisfied = Self::analyze_proximity(facts);

        let prima_facie_duty = foreseeability_satisfied && proximity_satisfied;

        // If recognized category, add Donoghue v Stevenson
        if facts.recognized_category.is_some() {
            key_cases.push(TortCase::donoghue_v_stevenson());
        }

        // Stage 2: Policy considerations
        let (policy_negates, policy_negation_reasons) = if prima_facie_duty {
            Self::analyze_policy(&facts.policy_concerns)
        } else {
            (false, vec![])
        };

        // Final determination
        let duty_owed = prima_facie_duty && !policy_negates;

        let reasoning = Self::build_reasoning(
            foreseeability_satisfied,
            proximity_satisfied,
            facts.recognized_category.as_ref(),
            policy_negates,
        );

        DutyOfCareResult {
            prima_facie_duty,
            foreseeability_satisfied,
            proximity_satisfied,
            policy_negates,
            policy_negation_reasons,
            duty_owed,
            key_cases,
            reasoning,
        }
    }

    /// Analyze proximity factors
    fn analyze_proximity(facts: &DutyOfCareFacts) -> bool {
        // If recognized category, proximity presumed
        if facts.recognized_category.is_some() {
            return true;
        }

        // Otherwise need sufficient proximity factors
        !facts.proximity_factors.is_empty()
    }

    /// Analyze policy considerations
    fn analyze_policy(concerns: &[PolicyNegation]) -> (bool, Vec<PolicyNegation>) {
        // Filter to valid policy negations
        let valid_negations: Vec<_> = concerns
            .iter()
            .filter(|c| Self::is_valid_policy_concern(c))
            .cloned()
            .collect();

        let negates = !valid_negations.is_empty();
        (negates, valid_negations)
    }

    /// Check if policy concern is valid
    fn is_valid_policy_concern(concern: &PolicyNegation) -> bool {
        match concern {
            PolicyNegation::IndeterminateLiability => true,
            PolicyNegation::ConflictWithOtherDuties => true,
            PolicyNegation::ChillingEffect => true,
            PolicyNegation::OtherRemedies => true,
            PolicyNegation::Constitutional => true,
            PolicyNegation::StatutoryExclusion => true,
            PolicyNegation::PublicPolicy { .. } => true,
        }
    }

    /// Build reasoning
    fn build_reasoning(
        foreseeability: bool,
        proximity: bool,
        recognized: Option<&RecognizedDutyCategory>,
        policy_negates: bool,
    ) -> String {
        let mut parts = Vec::new();

        // Stage 1
        if let Some(category) = recognized {
            parts.push(format!(
                "Stage 1: This falls within a recognized duty category ({:?}), \
                so proximity is established. Foreseeability: {}.",
                category,
                if foreseeability {
                    "satisfied"
                } else {
                    "not satisfied"
                }
            ));
        } else {
            parts.push(format!(
                "Stage 1: Novel category - must establish proximity. \
                Foreseeability: {}. Proximity: {}.",
                if foreseeability {
                    "satisfied"
                } else {
                    "not satisfied"
                },
                if proximity {
                    "established"
                } else {
                    "not established"
                }
            ));
        }

        // Stage 2
        if foreseeability && proximity {
            if policy_negates {
                parts.push("Stage 2: Policy considerations negate the duty of care.".to_string());
            } else {
                parts.push("Stage 2: No policy considerations negate the duty.".to_string());
            }
        }

        parts.join(" ")
    }
}

// ============================================================================
// Standard of Care Analysis
// ============================================================================

/// Facts for standard of care analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardOfCareFacts {
    /// Defendant's status (professional, ordinary person, etc.)
    pub defendant_status: String,
    /// Standard applicable
    pub standard: StandardOfCare,
    /// Defendant's actual conduct
    pub actual_conduct: String,
    /// Expected conduct under standard
    pub expected_conduct: String,
    /// Factors relevant to breach
    pub breach_factors: Vec<BreachFactor>,
}

/// Result of standard of care analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardOfCareResult {
    /// Applicable standard
    pub standard: StandardOfCare,
    /// Whether defendant met standard
    pub standard_met: bool,
    /// Breach factors considered
    pub factors_considered: Vec<BreachFactor>,
    /// Reasoning
    pub reasoning: String,
}

/// Analyzer for standard of care
pub struct StandardOfCareAnalyzer;

impl StandardOfCareAnalyzer {
    /// Analyze whether standard of care was met
    pub fn analyze(facts: &StandardOfCareFacts) -> StandardOfCareResult {
        let standard_met = Self::assess_conduct(&facts.standard, &facts.breach_factors);

        let reasoning = Self::build_reasoning(&facts.standard, standard_met, &facts.breach_factors);

        StandardOfCareResult {
            standard: facts.standard.clone(),
            standard_met,
            factors_considered: facts.breach_factors.clone(),
            reasoning,
        }
    }

    /// Assess conduct against standard
    fn assess_conduct(standard: &StandardOfCare, factors: &[BreachFactor]) -> bool {
        // Simple heuristic - more factors suggesting breach = standard not met
        let breach_indicators: usize = factors
            .iter()
            .filter(|f| {
                matches!(
                    f,
                    BreachFactor::LikelihoodOfHarm | BreachFactor::SeverityOfHarm
                )
            })
            .count();

        let compliance_indicators: usize = factors
            .iter()
            .filter(|f| {
                matches!(
                    f,
                    BreachFactor::StatutoryCompliance
                        | BreachFactor::IndustryPractice
                        | BreachFactor::CostOfPrecautions
                )
            })
            .count();

        // If professional standard, compliance indicators count more
        match standard {
            StandardOfCare::Professional { .. } => compliance_indicators >= breach_indicators,
            StandardOfCare::Emergency => true, // Lower standard
            _ => compliance_indicators > breach_indicators,
        }
    }

    /// Build reasoning
    fn build_reasoning(standard: &StandardOfCare, met: bool, factors: &[BreachFactor]) -> String {
        let standard_desc = match standard {
            StandardOfCare::ReasonablePerson => "reasonable person standard",
            StandardOfCare::Professional { profession } => {
                return format!(
                    "Professional standard applies ({} profession). Standard {}. \
                    Factors considered: {:?}.",
                    profession,
                    if met { "met" } else { "breached" },
                    factors
                );
            }
            StandardOfCare::Emergency => "modified (emergency) standard",
            StandardOfCare::Heightened { reason } => {
                return format!(
                    "Heightened standard applies ({}). Standard {}.",
                    reason,
                    if met { "met" } else { "breached" }
                );
            }
            StandardOfCare::Statutory { statute, section } => {
                return format!(
                    "Statutory standard applies ({} s.{}). Standard {}.",
                    statute,
                    section,
                    if met { "met" } else { "breached" }
                );
            }
        };

        format!(
            "Applying {}: standard {}. Factors: {:?}.",
            standard_desc,
            if met { "met" } else { "breached" },
            factors
        )
    }
}

// ============================================================================
// Causation Analysis
// ============================================================================

/// Facts for causation analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausationFacts {
    /// Defendant's breach
    pub breach_description: String,
    /// Claimant's injury
    pub injury_description: String,
    /// Whether but-for causation can be established
    pub but_for_possible: bool,
    /// Whether scientific impossibility applies
    pub scientific_uncertainty: bool,
    /// Multiple potential causes
    pub multiple_causes: bool,
    /// Intervening causes
    pub intervening_causes: Vec<InterveningCause>,
}

/// Result of causation analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausationResult {
    /// Test applied
    pub test_applied: CausationTest,
    /// Whether causation established
    pub causation_established: bool,
    /// Whether chain broken by intervening cause
    pub chain_broken: bool,
    /// Intervening cause that broke chain (if any)
    pub breaking_cause: Option<InterveningCause>,
    /// Key cases
    pub key_cases: Vec<TortCase>,
    /// Reasoning
    pub reasoning: String,
}

/// Analyzer for causation
pub struct CausationAnalyzer;

impl CausationAnalyzer {
    /// Analyze causation
    pub fn analyze(facts: &CausationFacts) -> CausationResult {
        let mut key_cases = Vec::new();

        // Determine which test to apply
        let test_applied = Self::determine_test(facts);

        if matches!(test_applied, CausationTest::MaterialContribution) {
            key_cases.push(TortCase::clements_v_clements());
        }

        // Apply the test
        let causation_on_test = match &test_applied {
            CausationTest::ButFor => facts.but_for_possible,
            CausationTest::MaterialContribution => {
                facts.scientific_uncertainty && facts.multiple_causes
            }
            CausationTest::MaterialIncreaseInRisk => facts.scientific_uncertainty,
        };

        // Check intervening causes
        let (chain_broken, breaking_cause) = Self::check_intervening(&facts.intervening_causes);

        let causation_established = causation_on_test && !chain_broken;

        let reasoning = Self::build_reasoning(&test_applied, causation_on_test, chain_broken);

        CausationResult {
            test_applied,
            causation_established,
            chain_broken,
            breaking_cause,
            key_cases,
            reasoning,
        }
    }

    /// Determine which causation test to apply
    fn determine_test(facts: &CausationFacts) -> CausationTest {
        if facts.but_for_possible {
            CausationTest::ButFor
        } else if facts.scientific_uncertainty && facts.multiple_causes {
            // Clements material contribution
            CausationTest::MaterialContribution
        } else {
            CausationTest::ButFor // Default
        }
    }

    /// Check if intervening cause breaks chain
    fn check_intervening(causes: &[InterveningCause]) -> (bool, Option<InterveningCause>) {
        for cause in causes {
            match cause {
                InterveningCause::MedicalTreatment => {
                    // Generally doesn't break chain unless grossly negligent
                }
                InterveningCause::ThirdPartyAct => {
                    // May break chain if not foreseeable
                    return (true, Some(cause.clone()));
                }
                InterveningCause::ClaimantAct => {
                    // May break chain if unreasonable
                }
                InterveningCause::NaturalEvent => {
                    // May break chain if unforeseeable
                }
            }
        }
        (false, None)
    }

    /// Build reasoning
    fn build_reasoning(test: &CausationTest, established: bool, broken: bool) -> String {
        let test_name = match test {
            CausationTest::ButFor => "but-for test",
            CausationTest::MaterialContribution => "material contribution to risk (Clements)",
            CausationTest::MaterialIncreaseInRisk => "material increase in risk",
        };

        if broken {
            format!(
                "Applying {}: causation potentially established but chain of \
                causation broken by intervening cause.",
                test_name
            )
        } else if established {
            format!(
                "Applying {}: causation established. Breach was a cause of injury.",
                test_name
            )
        } else {
            format!("Applying {}: causation not established.", test_name)
        }
    }
}

// ============================================================================
// Remoteness Analysis
// ============================================================================

/// Facts for remoteness analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemotenessFacts {
    /// Type of harm suffered
    pub harm_type: String,
    /// Whether type of harm foreseeable
    pub harm_type_foreseeable: bool,
    /// Whether extent of harm foreseeable
    pub extent_foreseeable: bool,
    /// Whether claimant has thin skull
    pub thin_skull: bool,
    /// Psychological injury claimed
    pub psychological_injury: bool,
}

/// Result of remoteness analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemotenessResult {
    /// Test applied
    pub test: RemotenessTest,
    /// Whether harm too remote
    pub too_remote: bool,
    /// Thin skull rule applies
    pub thin_skull_applies: bool,
    /// Key cases
    pub key_cases: Vec<TortCase>,
    /// Reasoning
    pub reasoning: String,
}

/// Analyzer for remoteness
pub struct RemotenessAnalyzer;

impl RemotenessAnalyzer {
    /// Analyze remoteness
    pub fn analyze(facts: &RemotenessFacts) -> RemotenessResult {
        let mut key_cases = Vec::new();

        // For psychological injury, need person of ordinary fortitude test
        if facts.psychological_injury {
            key_cases.push(TortCase::mustapha_v_culligan());
        }

        // Type of harm must be foreseeable (not extent)
        let too_remote = !facts.harm_type_foreseeable;

        // Thin skull rule
        let thin_skull_applies = facts.thin_skull && facts.harm_type_foreseeable;

        let reasoning = Self::build_reasoning(
            facts.harm_type_foreseeable,
            thin_skull_applies,
            facts.psychological_injury,
        );

        RemotenessResult {
            test: RemotenessTest::ReasonableForeseeability,
            too_remote,
            thin_skull_applies,
            key_cases,
            reasoning,
        }
    }

    /// Build reasoning
    fn build_reasoning(type_foreseeable: bool, thin_skull: bool, psychological: bool) -> String {
        let mut parts = Vec::new();

        if psychological {
            parts.push(
                "For psychological injury, harm must be foreseeable in person of \
                ordinary fortitude (Mustapha)."
                    .to_string(),
            );
        }

        if type_foreseeable {
            parts.push("Type of harm was reasonably foreseeable.".to_string());
            if thin_skull {
                parts.push(
                    "Thin skull rule applies - defendant takes claimant as found.".to_string(),
                );
            }
        } else {
            parts.push("Type of harm was not reasonably foreseeable - too remote.".to_string());
        }

        parts.join(" ")
    }
}

// ============================================================================
// Damages Analysis
// ============================================================================

/// Facts for damages analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamagesFacts {
    /// Types of damage claimed
    pub damages_claimed: Vec<TortDamages>,
    /// Whether catastrophic injury
    pub catastrophic: bool,
    /// Pecuniary losses (cents)
    pub pecuniary_losses_cents: Option<i64>,
    /// Whether punitive damages claimed
    pub punitive_claimed: bool,
    /// Whether defendant's conduct was egregious
    pub egregious_conduct: bool,
    /// Contributory negligence percentage
    pub contributory_negligence_percent: Option<u8>,
}

/// Result of damages analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamagesResult {
    /// Damages available
    pub damages_available: Vec<TortDamages>,
    /// Non-pecuniary cap applies
    pub cap_applies: bool,
    /// Cap amount (if applies)
    pub cap_amount_cents: Option<i64>,
    /// Punitive damages available
    pub punitive_available: bool,
    /// Reduction for contributory negligence
    pub contributory_reduction_percent: Option<u8>,
    /// Key cases
    pub key_cases: Vec<TortCase>,
    /// Reasoning
    pub reasoning: String,
}

/// Analyzer for tort damages
pub struct TortDamagesAnalyzer;

impl TortDamagesAnalyzer {
    /// Analyze available damages
    pub fn analyze(facts: &DamagesFacts) -> DamagesResult {
        let mut key_cases = Vec::new();

        // Check if non-pecuniary cap applies
        let (cap_applies, cap_amount) =
            if facts.catastrophic && facts.damages_claimed.contains(&TortDamages::NonPecuniary) {
                key_cases.push(TortCase::andrews_v_grand_toy());
                // 2024 indexed amount approximately $420,000
                (true, Some(42_000_000)) // ~$420,000 in cents
            } else {
                (false, None)
            };

        // Punitive damages
        let punitive_available = facts.punitive_claimed && facts.egregious_conduct;

        let reasoning = Self::build_reasoning(
            cap_applies,
            punitive_available,
            facts.contributory_negligence_percent,
        );

        DamagesResult {
            damages_available: facts.damages_claimed.clone(),
            cap_applies,
            cap_amount_cents: cap_amount,
            punitive_available,
            contributory_reduction_percent: facts.contributory_negligence_percent,
            key_cases,
            reasoning,
        }
    }

    /// Build reasoning
    fn build_reasoning(cap: bool, punitive: bool, contrib: Option<u8>) -> String {
        let mut parts = Vec::new();

        if cap {
            parts.push(
                "Non-pecuniary damages subject to cap (Andrews v Grand & Toy trilogy).".to_string(),
            );
        }

        if punitive {
            parts.push("Punitive damages may be available given egregious conduct.".to_string());
        }

        if let Some(percent) = contrib {
            parts.push(format!(
                "Damages reduced by {}% for contributory negligence.",
                percent
            ));
        }

        if parts.is_empty() {
            "Compensatory damages available.".to_string()
        } else {
            parts.join(" ")
        }
    }
}

// ============================================================================
// Full Negligence Claim Analysis
// ============================================================================

/// Facts for complete negligence claim
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NegligenceFacts {
    /// Duty of care facts
    pub duty_facts: DutyOfCareFacts,
    /// Standard of care facts
    pub standard_facts: StandardOfCareFacts,
    /// Causation facts
    pub causation_facts: CausationFacts,
    /// Remoteness facts
    pub remoteness_facts: RemotenessFacts,
    /// Damages facts
    pub damages_facts: DamagesFacts,
    /// Defences raised
    pub defences: Vec<NegligenceDefence>,
}

/// Result of complete negligence analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NegligenceResult {
    /// Duty of care result
    pub duty: DutyOfCareResult,
    /// Standard of care result
    pub standard: StandardOfCareResult,
    /// Causation result
    pub causation: CausationResult,
    /// Remoteness result
    pub remoteness: RemotenessResult,
    /// Damages result
    pub damages: DamagesResult,
    /// Defences that apply
    pub applicable_defences: Vec<NegligenceDefence>,
    /// Overall success
    pub claim_succeeds: bool,
    /// Summary reasoning
    pub summary: String,
}

/// Analyzer for complete negligence claim
pub struct NegligenceAnalyzer;

impl NegligenceAnalyzer {
    /// Analyze complete negligence claim
    pub fn analyze(facts: &NegligenceFacts) -> NegligenceResult {
        let duty = DutyOfCareAnalyzer::analyze(&facts.duty_facts);
        let standard = StandardOfCareAnalyzer::analyze(&facts.standard_facts);
        let causation = CausationAnalyzer::analyze(&facts.causation_facts);
        let remoteness = RemotenessAnalyzer::analyze(&facts.remoteness_facts);
        let damages = TortDamagesAnalyzer::analyze(&facts.damages_facts);

        // Check defences
        let applicable_defences: Vec<_> = facts
            .defences
            .iter()
            .filter(|d| Self::defence_applies(d))
            .cloned()
            .collect();

        // Claim succeeds if all elements met and no complete defence
        let claim_succeeds = duty.duty_owed
            && !standard.standard_met
            && causation.causation_established
            && !remoteness.too_remote
            && !Self::has_complete_defence(&applicable_defences);

        let summary = Self::build_summary(
            duty.duty_owed,
            !standard.standard_met,
            causation.causation_established,
            !remoteness.too_remote,
            &applicable_defences,
        );

        NegligenceResult {
            duty,
            standard,
            causation,
            remoteness,
            damages,
            applicable_defences,
            claim_succeeds,
            summary,
        }
    }

    /// Check if defence applies
    fn defence_applies(defence: &NegligenceDefence) -> bool {
        match defence {
            NegligenceDefence::ContributoryNegligence { percentage } => *percentage > 0,
            NegligenceDefence::VolentiNonFitInjuria => true,
            NegligenceDefence::ExTurpiCausa => true,
            NegligenceDefence::InevitableAccident => true,
            NegligenceDefence::StatutoryAuthority => true,
            NegligenceDefence::LimitationPeriod => true,
            NegligenceDefence::WaiverExclusion => true,
        }
    }

    /// Check if any complete defence
    fn has_complete_defence(defences: &[NegligenceDefence]) -> bool {
        defences.iter().any(|d| {
            matches!(
                d,
                NegligenceDefence::VolentiNonFitInjuria
                    | NegligenceDefence::ExTurpiCausa
                    | NegligenceDefence::LimitationPeriod
            )
        })
    }

    /// Build summary
    fn build_summary(
        duty: bool,
        breach: bool,
        causation: bool,
        not_remote: bool,
        defences: &[NegligenceDefence],
    ) -> String {
        let elements = [
            ("Duty of care", duty),
            ("Breach of standard", breach),
            ("Causation", causation),
            ("Remoteness", not_remote),
        ];

        let failed: Vec<_> = elements.iter().filter(|(_, met)| !met).collect();

        if failed.is_empty() {
            if defences.is_empty() {
                "All elements of negligence established. Claim succeeds.".to_string()
            } else {
                format!(
                    "All elements established but defences apply: {:?}",
                    defences
                )
            }
        } else {
            format!(
                "Claim fails. Elements not established: {}",
                failed
                    .iter()
                    .map(|(name, _)| *name)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
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
    fn test_duty_of_care_recognized_category() {
        let facts = DutyOfCareFacts {
            defendant_conduct: "Manufactured defective product".to_string(),
            claimant_injury: "Illness from product".to_string(),
            relationship: "Manufacturer-consumer".to_string(),
            harm_foreseeable: true,
            proximity_factors: vec![],
            recognized_category: Some(RecognizedDutyCategory::ManufacturerConsumer),
            policy_concerns: vec![],
        };

        let result = DutyOfCareAnalyzer::analyze(&facts);
        assert!(result.duty_owed);
        assert!(result.prima_facie_duty);
    }

    #[test]
    fn test_duty_of_care_no_foreseeability() {
        let facts = DutyOfCareFacts {
            defendant_conduct: "Some conduct".to_string(),
            claimant_injury: "Unforeseeable harm".to_string(),
            relationship: "Unknown".to_string(),
            harm_foreseeable: false,
            proximity_factors: vec![],
            recognized_category: None,
            policy_concerns: vec![],
        };

        let result = DutyOfCareAnalyzer::analyze(&facts);
        assert!(!result.duty_owed);
    }

    #[test]
    fn test_causation_but_for() {
        let facts = CausationFacts {
            breach_description: "Drove negligently".to_string(),
            injury_description: "Collision injury".to_string(),
            but_for_possible: true,
            scientific_uncertainty: false,
            multiple_causes: false,
            intervening_causes: vec![],
        };

        let result = CausationAnalyzer::analyze(&facts);
        assert!(result.causation_established);
        assert!(matches!(result.test_applied, CausationTest::ButFor));
    }

    #[test]
    fn test_remoteness_psychological() {
        let facts = RemotenessFacts {
            harm_type: "Psychological harm".to_string(),
            harm_type_foreseeable: false,
            extent_foreseeable: false,
            thin_skull: false,
            psychological_injury: true,
        };

        let result = RemotenessAnalyzer::analyze(&facts);
        assert!(result.too_remote);
        assert!(!result.key_cases.is_empty());
    }

    #[test]
    fn test_damages_cap() {
        let facts = DamagesFacts {
            damages_claimed: vec![TortDamages::NonPecuniary],
            catastrophic: true,
            pecuniary_losses_cents: None,
            punitive_claimed: false,
            egregious_conduct: false,
            contributory_negligence_percent: None,
        };

        let result = TortDamagesAnalyzer::analyze(&facts);
        assert!(result.cap_applies);
        assert!(result.cap_amount_cents.is_some());
    }
}
