//! Australian Negligence Law
//!
//! Analysis of negligence under Australian common law and
//! Civil Liability Act reforms.

use serde::{Deserialize, Serialize};

use super::types::{
    CLADefence, DutyCategory, NovusActus, ObviousRisk, RecognizedDuty, SalientFeature,
    StandardOfCareFactor,
};
use crate::common::StateTerritory;

// ============================================================================
// Duty of Care Analysis
// ============================================================================

/// Analyzer for duty of care
pub struct DutyOfCareAnalyzer;

impl DutyOfCareAnalyzer {
    /// Analyze whether duty of care exists
    pub fn analyze(facts: &DutyFacts) -> DutyResult {
        // First check recognized categories
        if let Some(recognized) = Self::check_recognized_category(facts) {
            return DutyResult {
                duty_exists: true,
                category: DutyCategory::Recognized(recognized.clone()),
                salient_features_analysis: None,
                reasoning: format!(
                    "Recognized duty category: {:?}. Duty established.",
                    recognized
                ),
            };
        }

        // Novel duty - salient features analysis (Sullivan v Moody)
        let salient_analysis = Self::salient_features_analysis(facts);
        let duty_exists = salient_analysis.supports_duty;

        DutyResult {
            duty_exists,
            category: DutyCategory::Novel,
            salient_features_analysis: Some(salient_analysis.clone()),
            reasoning: Self::build_novel_duty_reasoning(&salient_analysis),
        }
    }

    /// Check for recognized duty category
    fn check_recognized_category(facts: &DutyFacts) -> Option<RecognizedDuty> {
        if facts.manufacturer_consumer_relationship {
            return Some(RecognizedDuty::ManufacturerConsumer);
        }
        if facts.employer_employee_relationship {
            return Some(RecognizedDuty::EmployerEmployee);
        }
        if facts.professional_client_relationship {
            return Some(RecognizedDuty::Professional);
        }
        if facts.occupier_visitor_relationship {
            return Some(RecognizedDuty::OccupierVisitor);
        }
        if facts.road_users {
            return Some(RecognizedDuty::RoadUser);
        }
        if facts.teacher_student_relationship {
            return Some(RecognizedDuty::TeacherStudent);
        }
        None
    }

    /// Perform salient features analysis (Sullivan v Moody)
    fn salient_features_analysis(facts: &DutyFacts) -> SalientFeaturesAnalysis {
        let mut supporting = Vec::new();
        let mut opposing = Vec::new();

        // Foreseeability
        if facts.harm_foreseeable {
            supporting.push(SalientFeature::Foreseeability);
        } else {
            opposing.push(SalientFeature::Foreseeability);
        }

        // Vulnerability
        if facts.plaintiff_vulnerable {
            supporting.push(SalientFeature::Vulnerability);
        }

        // Control
        if facts.defendant_had_control {
            supporting.push(SalientFeature::ControlOverRisk);
        }

        // Assumption of responsibility
        if facts.assumption_of_responsibility {
            supporting.push(SalientFeature::AssumptionOfResponsibility);
        }

        // Indeterminacy concerns
        if facts.indeterminate_class {
            opposing.push(SalientFeature::IndeterminacyConcerns);
        }

        // Coherence with existing law
        if facts.would_create_incoherence {
            opposing.push(SalientFeature::CoherenceWithLaw);
        }

        // Conflicting duties
        if facts.would_create_conflicting_duties {
            opposing.push(SalientFeature::ConflictingDuties);
        }

        let supports_duty = supporting.len() > opposing.len()
            && !opposing.contains(&SalientFeature::Foreseeability)
            && !facts.would_create_incoherence;

        SalientFeaturesAnalysis {
            supporting_features: supporting,
            opposing_features: opposing,
            supports_duty,
        }
    }

    /// Build reasoning for novel duty
    fn build_novel_duty_reasoning(analysis: &SalientFeaturesAnalysis) -> String {
        let mut parts = Vec::new();

        parts.push("Novel duty analysis (Sullivan v Moody (2001))".to_string());

        if !analysis.supporting_features.is_empty() {
            parts.push(format!(
                "Supporting features: {:?}",
                analysis.supporting_features
            ));
        }

        if !analysis.opposing_features.is_empty() {
            parts.push(format!(
                "Opposing features: {:?}",
                analysis.opposing_features
            ));
        }

        if analysis.supports_duty {
            parts.push("Balance of salient features supports imposition of duty".to_string());
        } else {
            parts.push("Salient features analysis does not support duty".to_string());
        }

        parts.join(". ")
    }
}

/// Facts for duty of care analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DutyFacts {
    // Recognized categories
    /// Manufacturer-consumer relationship
    pub manufacturer_consumer_relationship: bool,
    /// Employer-employee relationship
    pub employer_employee_relationship: bool,
    /// Professional-client relationship
    pub professional_client_relationship: bool,
    /// Occupier-visitor relationship
    pub occupier_visitor_relationship: bool,
    /// Road users
    pub road_users: bool,
    /// Teacher-student relationship
    pub teacher_student_relationship: bool,

    // Salient features
    /// Harm foreseeable
    pub harm_foreseeable: bool,
    /// Plaintiff vulnerable
    pub plaintiff_vulnerable: bool,
    /// Defendant had control
    pub defendant_had_control: bool,
    /// Assumption of responsibility
    pub assumption_of_responsibility: bool,
    /// Indeterminate class
    pub indeterminate_class: bool,
    /// Would create incoherence
    pub would_create_incoherence: bool,
    /// Would create conflicting duties
    pub would_create_conflicting_duties: bool,
}

/// Result of duty analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DutyResult {
    /// Whether duty exists
    pub duty_exists: bool,
    /// Duty category
    pub category: DutyCategory,
    /// Salient features analysis (for novel)
    pub salient_features_analysis: Option<SalientFeaturesAnalysis>,
    /// Reasoning
    pub reasoning: String,
}

/// Salient features analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalientFeaturesAnalysis {
    /// Supporting features
    pub supporting_features: Vec<SalientFeature>,
    /// Opposing features
    pub opposing_features: Vec<SalientFeature>,
    /// Whether supports duty
    pub supports_duty: bool,
}

// ============================================================================
// Breach of Duty Analysis
// ============================================================================

/// Analyzer for breach of duty
pub struct BreachAnalyzer;

impl BreachAnalyzer {
    /// Analyze breach of duty (CLA calculus)
    pub fn analyze(facts: &BreachFacts, state: StateTerritory) -> BreachResult {
        // CLA s.9 factors (named differently in each state)
        let breach = Self::apply_cla_calculus(facts);

        // Check CLA defences
        let defences = Self::check_defences(facts, state);

        let reasoning = Self::build_reasoning(facts, breach, &defences);

        BreachResult {
            breach_established: breach && defences.is_empty(),
            breach_without_defences: breach,
            applicable_defences: defences,
            standard_of_care_factors: Self::identify_factors(facts),
            reasoning,
        }
    }

    /// Apply CLA calculus for breach
    fn apply_cla_calculus(facts: &BreachFacts) -> bool {
        // CLA s.9: reasonable person would have taken precautions if:
        // (a) risk not insignificant
        // (b) reasonable person would have taken precautions

        if facts.risk_not_insignificant {
            // Weigh factors
            let risk_weight = if facts.high_probability_of_harm { 2 } else { 1 }
                + if facts.serious_harm_likely { 2 } else { 1 };

            let precaution_burden = if facts.precautions_burdensome { 2 } else { 1 }
                - if facts.high_social_utility { 1 } else { 0 };

            // Breach if risk outweighs burden
            risk_weight > precaution_burden && !facts.precautions_taken
        } else {
            false
        }
    }

    /// Check CLA defences
    fn check_defences(facts: &BreachFacts, _state: StateTerritory) -> Vec<CLADefence> {
        let mut defences = Vec::new();

        // Obvious risk defence
        if facts.obvious_risk.as_ref().is_some_and(|r| r.obvious) {
            defences.push(CLADefence::ObviousRiskNoWarning);
        }

        // Inherent risk
        if facts.obvious_risk.as_ref().is_some_and(|r| r.inherent) {
            defences.push(CLADefence::InherentRisk);
        }

        // Recreational activity
        if facts.recreational_activity && facts.plaintiff_aware_of_risk {
            defences.push(CLADefence::VoluntaryAssumptionRecreational);
        }

        // Dangerous recreational activity
        if facts.dangerous_recreational_activity {
            defences.push(CLADefence::DangerousRecreationalActivity);
        }

        // Good Samaritan
        if facts.good_samaritan_context && facts.acting_in_good_faith {
            defences.push(CLADefence::GoodSamaritan);
        }

        // Intoxication
        if facts.plaintiff_intoxicated && facts.intoxication_contributed {
            defences.push(CLADefence::PlaintiffIntoxication);
        }

        // Illegal activity
        if facts.plaintiff_engaged_in_illegal_activity {
            defences.push(CLADefence::PlaintiffIllegalActivity);
        }

        defences
    }

    /// Identify standard of care factors
    fn identify_factors(facts: &BreachFacts) -> Vec<StandardOfCareFactor> {
        let mut factors = Vec::new();

        if facts.high_probability_of_harm {
            factors.push(StandardOfCareFactor::ProbabilityOfHarm);
        }
        if facts.serious_harm_likely {
            factors.push(StandardOfCareFactor::SeriousnessOfHarm);
        }
        if facts.precautions_burdensome {
            factors.push(StandardOfCareFactor::BurdenOfPrecautions);
        }
        if facts.high_social_utility {
            factors.push(StandardOfCareFactor::SocialUtility);
        }

        factors
    }

    /// Build reasoning
    fn build_reasoning(facts: &BreachFacts, breach: bool, defences: &[CLADefence]) -> String {
        let mut parts = Vec::new();

        parts.push("Breach analysis under Civil Liability Act".to_string());

        if !facts.risk_not_insignificant {
            parts.push("Risk was insignificant - no breach".to_string());
            return parts.join(". ");
        }

        parts.push("Risk was not insignificant (CLA threshold met)".to_string());

        // Calculus factors
        parts.push("Applying CLA s.9 calculus:".to_string());
        if facts.high_probability_of_harm {
            parts.push("- High probability of harm".to_string());
        }
        if facts.serious_harm_likely {
            parts.push("- Serious harm likely".to_string());
        }
        if facts.precautions_burdensome {
            parts.push("- Precautions would be burdensome".to_string());
        }
        if facts.high_social_utility {
            parts.push("- Activity has high social utility".to_string());
        }

        if breach {
            if facts.precautions_taken {
                parts.push("Precautions taken - no breach".to_string());
            } else {
                parts.push("Reasonable person would have taken precautions - breach".to_string());
            }
        } else {
            parts.push("Burden of precautions outweighs risk - no breach".to_string());
        }

        // Defences
        if !defences.is_empty() {
            parts.push(format!("CLA defences available: {:?}", defences));
        }

        parts.join(". ")
    }
}

/// Facts for breach analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BreachFacts {
    // CLA calculus
    /// Risk not insignificant
    pub risk_not_insignificant: bool,
    /// High probability of harm
    pub high_probability_of_harm: bool,
    /// Serious harm likely
    pub serious_harm_likely: bool,
    /// Precautions burdensome
    pub precautions_burdensome: bool,
    /// High social utility
    pub high_social_utility: bool,
    /// Precautions taken
    pub precautions_taken: bool,

    // Obvious risk
    /// Obvious risk information
    pub obvious_risk: Option<ObviousRisk>,
    /// Plaintiff aware of risk
    pub plaintiff_aware_of_risk: bool,

    // Recreational activity
    /// Recreational activity
    pub recreational_activity: bool,
    /// Dangerous recreational activity
    pub dangerous_recreational_activity: bool,

    // Good Samaritan
    /// Good Samaritan context
    pub good_samaritan_context: bool,
    /// Acting in good faith
    pub acting_in_good_faith: bool,

    // Plaintiff conduct
    /// Plaintiff intoxicated
    pub plaintiff_intoxicated: bool,
    /// Intoxication contributed to harm
    pub intoxication_contributed: bool,
    /// Plaintiff engaged in illegal activity
    pub plaintiff_engaged_in_illegal_activity: bool,
}

/// Result of breach analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreachResult {
    /// Breach established (accounting for defences)
    pub breach_established: bool,
    /// Breach without considering defences
    pub breach_without_defences: bool,
    /// Applicable CLA defences
    pub applicable_defences: Vec<CLADefence>,
    /// Standard of care factors considered
    pub standard_of_care_factors: Vec<StandardOfCareFactor>,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Causation Analysis
// ============================================================================

/// Analyzer for causation
pub struct CausationAnalyzer;

impl CausationAnalyzer {
    /// Analyze causation (CLA s.5D)
    pub fn analyze(facts: &CausationFacts) -> CausationResult {
        // CLA s.5D: Two limbs
        // (1)(a) Factual causation - defendant's negligence was necessary condition
        // (1)(b) Scope of liability - appropriate to extend liability

        let factual = Self::check_factual_causation(facts);
        let scope = Self::check_scope_of_liability(facts);
        let novus_actus = Self::check_novus_actus(facts);

        let causation_established = factual && scope && novus_actus.is_none();

        let reasoning = Self::build_reasoning(facts, factual, scope, &novus_actus);

        CausationResult {
            causation_established,
            factual_causation: factual,
            scope_of_liability: scope,
            novus_actus_interveniens: novus_actus,
            reasoning,
        }
    }

    /// Check factual causation (but-for test)
    fn check_factual_causation(facts: &CausationFacts) -> bool {
        // s.5D(1)(a): negligence was necessary condition of harm
        facts.but_for_negligence_no_harm
    }

    /// Check scope of liability
    fn check_scope_of_liability(facts: &CausationFacts) -> bool {
        // s.5D(1)(b): appropriate to extend liability to harm
        // Considers remoteness, nature of harm
        facts.harm_within_scope && !facts.harm_too_remote
    }

    /// Check for novus actus interveniens
    fn check_novus_actus(facts: &CausationFacts) -> Option<NovusActus> {
        if facts.intervening_third_party_act && facts.third_party_act_unforeseeable {
            return Some(NovusActus::ThirdPartyAct);
        }

        if facts.plaintiff_unreasonable_act && facts.plaintiff_act_unforeseeable {
            return Some(NovusActus::ClaimantAct);
        }

        if facts.intervening_natural_event && facts.natural_event_unforeseeable {
            return Some(NovusActus::NaturalEvent);
        }

        None
    }

    /// Build reasoning
    fn build_reasoning(
        facts: &CausationFacts,
        factual: bool,
        scope: bool,
        novus: &Option<NovusActus>,
    ) -> String {
        let mut parts = Vec::new();

        parts.push("Causation analysis (CLA s.5D)".to_string());

        // Factual causation
        if factual {
            parts.push("s.5D(1)(a) Factual causation: but-for test satisfied".to_string());
            parts.push("Defendant's negligence was necessary condition of harm".to_string());
        } else {
            parts.push("s.5D(1)(a) Factual causation not established".to_string());
            return parts.join(". ");
        }

        // Scope of liability
        if scope {
            parts
                .push("s.5D(1)(b) Scope of liability: appropriate to extend liability".to_string());
        } else {
            parts.push("s.5D(1)(b) Scope of liability: harm outside scope".to_string());
            if facts.harm_too_remote {
                parts.push("Harm too remote".to_string());
            }
            return parts.join(". ");
        }

        // Novus actus
        if let Some(n) = novus {
            parts.push(format!("Chain broken by novus actus interveniens: {:?}", n));
        }

        parts.join(". ")
    }
}

/// Facts for causation analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CausationFacts {
    // Factual causation
    /// But for negligence, no harm
    pub but_for_negligence_no_harm: bool,

    // Scope of liability
    /// Harm within scope
    pub harm_within_scope: bool,
    /// Harm too remote
    pub harm_too_remote: bool,

    // Novus actus
    /// Intervening third party act
    pub intervening_third_party_act: bool,
    /// Third party act unforeseeable
    pub third_party_act_unforeseeable: bool,
    /// Plaintiff unreasonable act
    pub plaintiff_unreasonable_act: bool,
    /// Plaintiff act unforeseeable
    pub plaintiff_act_unforeseeable: bool,
    /// Intervening natural event
    pub intervening_natural_event: bool,
    /// Natural event unforeseeable
    pub natural_event_unforeseeable: bool,
}

/// Result of causation analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausationResult {
    /// Causation established
    pub causation_established: bool,
    /// Factual causation
    pub factual_causation: bool,
    /// Scope of liability
    pub scope_of_liability: bool,
    /// Novus actus
    pub novus_actus_interveniens: Option<NovusActus>,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Complete Negligence Claim
// ============================================================================

/// Complete negligence claim analyzer
pub struct NegligenceAnalyzer;

impl NegligenceAnalyzer {
    /// Analyze complete negligence claim
    pub fn analyze(
        duty_facts: &DutyFacts,
        breach_facts: &BreachFacts,
        causation_facts: &CausationFacts,
        state: StateTerritory,
    ) -> NegligenceResult {
        let duty_result = DutyOfCareAnalyzer::analyze(duty_facts);
        let breach_result = BreachAnalyzer::analyze(breach_facts, state);
        let causation_result = CausationAnalyzer::analyze(causation_facts);

        let liable = duty_result.duty_exists
            && breach_result.breach_established
            && causation_result.causation_established;

        let reasoning =
            Self::build_reasoning(&duty_result, &breach_result, &causation_result, liable);

        NegligenceResult {
            liable,
            duty: duty_result,
            breach: breach_result,
            causation: causation_result,
            state,
            reasoning,
        }
    }

    /// Build overall reasoning
    fn build_reasoning(
        duty: &DutyResult,
        breach: &BreachResult,
        causation: &CausationResult,
        liable: bool,
    ) -> String {
        let mut parts = Vec::new();

        parts.push("Negligence claim analysis".to_string());

        // Duty
        if duty.duty_exists {
            parts.push("1. Duty of care: ESTABLISHED".to_string());
        } else {
            parts.push("1. Duty of care: NOT ESTABLISHED - claim fails".to_string());
            return parts.join(". ");
        }

        // Breach
        if breach.breach_established {
            parts.push("2. Breach of duty: ESTABLISHED".to_string());
        } else {
            if breach.breach_without_defences {
                parts.push(format!(
                    "2. Breach of duty: NEGATED BY DEFENCES ({:?})",
                    breach.applicable_defences
                ));
            } else {
                parts.push("2. Breach of duty: NOT ESTABLISHED - claim fails".to_string());
            }
            return parts.join(". ");
        }

        // Causation
        if causation.causation_established {
            parts.push("3. Causation: ESTABLISHED".to_string());
        } else {
            parts.push("3. Causation: NOT ESTABLISHED - claim fails".to_string());
            return parts.join(". ");
        }

        if liable {
            parts.push("All elements satisfied - defendant liable in negligence".to_string());
        }

        parts.join(". ")
    }
}

/// Complete negligence result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NegligenceResult {
    /// Whether liable
    pub liable: bool,
    /// Duty analysis
    pub duty: DutyResult,
    /// Breach analysis
    pub breach: BreachResult,
    /// Causation analysis
    pub causation: CausationResult,
    /// State
    pub state: StateTerritory,
    /// Overall reasoning
    pub reasoning: String,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duty_recognized_category() {
        let facts = DutyFacts {
            manufacturer_consumer_relationship: true,
            harm_foreseeable: true,
            ..Default::default()
        };

        let result = DutyOfCareAnalyzer::analyze(&facts);
        assert!(result.duty_exists);
        assert!(matches!(result.category, DutyCategory::Recognized(_)));
    }

    #[test]
    fn test_duty_novel_supported() {
        let facts = DutyFacts {
            harm_foreseeable: true,
            plaintiff_vulnerable: true,
            defendant_had_control: true,
            assumption_of_responsibility: true,
            ..Default::default()
        };

        let result = DutyOfCareAnalyzer::analyze(&facts);
        assert!(result.duty_exists);
        assert!(matches!(result.category, DutyCategory::Novel));
    }

    #[test]
    fn test_breach_cla_calculus() {
        let facts = BreachFacts {
            risk_not_insignificant: true,
            high_probability_of_harm: true,
            serious_harm_likely: true,
            precautions_taken: false,
            ..Default::default()
        };

        let result = BreachAnalyzer::analyze(&facts, StateTerritory::NewSouthWales);
        assert!(result.breach_established);
    }

    #[test]
    fn test_breach_obvious_risk_defence() {
        let facts = BreachFacts {
            risk_not_insignificant: true,
            high_probability_of_harm: true,
            precautions_taken: false,
            obvious_risk: Some(ObviousRisk {
                obvious: true,
                inherent: false,
                recreational_activity: false,
                description: "Obvious danger".to_string(),
            }),
            ..Default::default()
        };

        let result = BreachAnalyzer::analyze(&facts, StateTerritory::NewSouthWales);
        assert!(!result.breach_established); // Defence applies
        assert!(
            result
                .applicable_defences
                .contains(&CLADefence::ObviousRiskNoWarning)
        );
    }

    #[test]
    fn test_causation_established() {
        let facts = CausationFacts {
            but_for_negligence_no_harm: true,
            harm_within_scope: true,
            ..Default::default()
        };

        let result = CausationAnalyzer::analyze(&facts);
        assert!(result.causation_established);
    }

    #[test]
    fn test_causation_broken_by_novus() {
        let facts = CausationFacts {
            but_for_negligence_no_harm: true,
            harm_within_scope: true,
            intervening_third_party_act: true,
            third_party_act_unforeseeable: true,
            ..Default::default()
        };

        let result = CausationAnalyzer::analyze(&facts);
        assert!(!result.causation_established);
        assert!(result.novus_actus_interveniens.is_some());
    }

    #[test]
    fn test_complete_negligence_claim() {
        let duty_facts = DutyFacts {
            road_users: true,
            harm_foreseeable: true,
            ..Default::default()
        };

        let breach_facts = BreachFacts {
            risk_not_insignificant: true,
            high_probability_of_harm: true,
            ..Default::default()
        };

        let causation_facts = CausationFacts {
            but_for_negligence_no_harm: true,
            harm_within_scope: true,
            ..Default::default()
        };

        let result = NegligenceAnalyzer::analyze(
            &duty_facts,
            &breach_facts,
            &causation_facts,
            StateTerritory::NewSouthWales,
        );

        assert!(result.liable);
    }
}
