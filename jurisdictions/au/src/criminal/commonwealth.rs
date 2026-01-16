//! Commonwealth Criminal Law
//!
//! Implementation of Criminal Code Act 1995 (Cth) and related legislation.

use serde::{Deserialize, Serialize};

use super::types::{Defence, FaultElement, PhysicalElement};

// ============================================================================
// Offence Analyzer
// ============================================================================

/// Analyzer for Commonwealth criminal offences
pub struct OffenceAnalyzer;

impl OffenceAnalyzer {
    /// Analyze whether offence elements are satisfied
    pub fn analyze_offence(facts: &OffenceFacts) -> OffenceResult {
        let physical_elements = Self::check_physical_elements(facts);
        let fault_elements = Self::check_fault_elements(facts);
        let defences = Self::check_defences(facts);

        let all_elements_satisfied =
            physical_elements.all_satisfied && fault_elements.all_satisfied;
        let defence_available = !defences.is_empty();

        let liability = if !all_elements_satisfied {
            LiabilityStatus::ElementsNotProved
        } else if defence_available {
            LiabilityStatus::DefenceAvailable
        } else {
            LiabilityStatus::Guilty
        };

        let reasoning =
            Self::build_reasoning(&physical_elements, &fault_elements, &defences, &liability);

        OffenceResult {
            offence_description: facts.offence_description.clone(),
            physical_elements_satisfied: physical_elements.all_satisfied,
            fault_elements_satisfied: fault_elements.all_satisfied,
            available_defences: defences,
            liability_status: liability,
            reasoning,
        }
    }

    /// Check physical elements
    fn check_physical_elements(facts: &OffenceFacts) -> PhysicalElementResult {
        let mut satisfied = Vec::new();
        let mut not_satisfied = Vec::new();

        // Conduct
        if facts.conduct_occurred {
            satisfied.push(PhysicalElement::Conduct);
        } else {
            not_satisfied.push(PhysicalElement::Conduct);
        }

        // Result (if required)
        if facts.result_required {
            if facts.result_occurred {
                satisfied.push(PhysicalElement::Result);
            } else {
                not_satisfied.push(PhysicalElement::Result);
            }
        }

        // Circumstance
        if facts.circumstance_required {
            if facts.circumstance_exists {
                satisfied.push(PhysicalElement::Circumstance);
            } else {
                not_satisfied.push(PhysicalElement::Circumstance);
            }
        }

        PhysicalElementResult {
            satisfied,
            not_satisfied: not_satisfied.clone(),
            all_satisfied: not_satisfied.is_empty(),
        }
    }

    /// Check fault elements
    fn check_fault_elements(facts: &OffenceFacts) -> FaultElementResult {
        // Handle strict/absolute liability
        if facts.strict_liability {
            return FaultElementResult {
                required: FaultElement::StrictLiability,
                proved: true,
                honest_reasonable_mistake: facts.honest_reasonable_mistake,
                all_satisfied: !facts.honest_reasonable_mistake,
            };
        }

        if facts.absolute_liability {
            return FaultElementResult {
                required: FaultElement::AbsoluteLiability,
                proved: true,
                honest_reasonable_mistake: false,
                all_satisfied: true,
            };
        }

        // Standard fault elements
        let required = facts.required_fault_element.clone();
        let proved = facts.fault_element_proved;

        FaultElementResult {
            required,
            proved,
            honest_reasonable_mistake: false,
            all_satisfied: proved,
        }
    }

    /// Check defences
    fn check_defences(facts: &OffenceFacts) -> Vec<DefenceResult> {
        let mut defences = Vec::new();

        // Self-defence (s.10.4)
        if facts.self_defence_claimed {
            let available = Self::check_self_defence(facts);
            if available {
                defences.push(DefenceResult {
                    defence: Defence::SelfDefence,
                    available: true,
                    reasoning: "Self-defence established under s.10.4 Criminal Code Act"
                        .to_string(),
                });
            }
        }

        // Duress (s.10.2)
        if facts.duress_claimed {
            let available = Self::check_duress(facts);
            if available {
                defences.push(DefenceResult {
                    defence: Defence::Duress,
                    available: true,
                    reasoning: "Duress established under s.10.2 Criminal Code Act".to_string(),
                });
            }
        }

        // Mental impairment (s.7.3)
        if facts.mental_impairment_claimed {
            let available = Self::check_mental_impairment(facts);
            if available {
                defences.push(DefenceResult {
                    defence: Defence::MentalImpairment,
                    available: true,
                    reasoning: "Mental impairment defence available under s.7.3".to_string(),
                });
            }
        }

        // Mistake of fact (s.9.1)
        if facts.mistake_of_fact_claimed {
            let available = Self::check_mistake_of_fact(facts);
            if available {
                defences.push(DefenceResult {
                    defence: Defence::MistakeOfFact,
                    available: true,
                    reasoning: "Mistake of fact negates fault element under s.9.1".to_string(),
                });
            }
        }

        defences
    }

    /// Check self-defence (s.10.4)
    fn check_self_defence(facts: &OffenceFacts) -> bool {
        // s.10.4(2): Believed on reasonable grounds conduct necessary
        facts.believed_conduct_necessary
            && facts.reasonable_grounds_for_belief
            && facts.response_proportionate
    }

    /// Check duress (s.10.2)
    fn check_duress(facts: &OffenceFacts) -> bool {
        // s.10.2(2): Threat of death/serious harm, no reasonable way to escape
        facts.threat_of_serious_harm && facts.no_reasonable_escape && !facts.offence_is_murder // Duress not available for murder
    }

    /// Check mental impairment (s.7.3)
    fn check_mental_impairment(facts: &OffenceFacts) -> bool {
        facts.mental_impairment_present
            && (facts.could_not_understand_nature || facts.could_not_know_wrongful)
    }

    /// Check mistake of fact (s.9.1)
    fn check_mistake_of_fact(facts: &OffenceFacts) -> bool {
        facts.honest_mistake && facts.mistake_reasonable
    }

    /// Build reasoning
    fn build_reasoning(
        physical: &PhysicalElementResult,
        fault: &FaultElementResult,
        defences: &[DefenceResult],
        liability: &LiabilityStatus,
    ) -> String {
        let mut parts = Vec::new();

        parts.push("Criminal offence analysis (Criminal Code Act 1995 (Cth))".to_string());

        // Physical elements
        if physical.all_satisfied {
            parts.push("Physical elements (s.4.1) established".to_string());
        } else {
            parts.push(format!(
                "Physical elements not proved: {:?}",
                physical.not_satisfied
            ));
        }

        // Fault elements
        parts.push(format!(
            "Fault element required: {:?} ({})",
            fault.required,
            fault.required.section()
        ));
        if fault.all_satisfied {
            parts.push("Fault element proved".to_string());
        } else if fault.honest_reasonable_mistake {
            parts.push("Honest and reasonable mistake defence available (s.9.2)".to_string());
        } else {
            parts.push("Fault element not proved".to_string());
        }

        // Defences
        for defence in defences {
            if defence.available {
                parts.push(format!("Defence available: {:?}", defence.defence));
            }
        }

        // Liability
        match liability {
            LiabilityStatus::Guilty => {
                parts.push("All elements proved, no defence - guilty".to_string())
            }
            LiabilityStatus::DefenceAvailable => {
                parts.push("Elements proved but defence available".to_string())
            }
            LiabilityStatus::ElementsNotProved => {
                parts.push("Prosecution failed to prove all elements".to_string())
            }
        }

        parts.join(". ")
    }
}

/// Facts for offence analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OffenceFacts {
    /// Offence description
    pub offence_description: String,

    // Physical elements
    /// Conduct occurred
    pub conduct_occurred: bool,
    /// Result required
    pub result_required: bool,
    /// Result occurred
    pub result_occurred: bool,
    /// Circumstance required
    pub circumstance_required: bool,
    /// Circumstance exists
    pub circumstance_exists: bool,

    // Fault elements
    /// Required fault element
    pub required_fault_element: FaultElement,
    /// Fault element proved
    pub fault_element_proved: bool,
    /// Strict liability offence
    pub strict_liability: bool,
    /// Absolute liability offence
    pub absolute_liability: bool,
    /// Honest and reasonable mistake
    pub honest_reasonable_mistake: bool,

    // Defences
    /// Self-defence claimed
    pub self_defence_claimed: bool,
    /// Believed conduct necessary
    pub believed_conduct_necessary: bool,
    /// Reasonable grounds for belief
    pub reasonable_grounds_for_belief: bool,
    /// Response proportionate
    pub response_proportionate: bool,

    /// Duress claimed
    pub duress_claimed: bool,
    /// Threat of serious harm
    pub threat_of_serious_harm: bool,
    /// No reasonable escape
    pub no_reasonable_escape: bool,
    /// Offence is murder
    pub offence_is_murder: bool,

    /// Mental impairment claimed
    pub mental_impairment_claimed: bool,
    /// Mental impairment present
    pub mental_impairment_present: bool,
    /// Could not understand nature of conduct
    pub could_not_understand_nature: bool,
    /// Could not know conduct wrongful
    pub could_not_know_wrongful: bool,

    /// Mistake of fact claimed
    pub mistake_of_fact_claimed: bool,
    /// Honest mistake
    pub honest_mistake: bool,
    /// Mistake reasonable
    pub mistake_reasonable: bool,
}

/// Result of physical element analysis
#[derive(Debug, Clone)]
struct PhysicalElementResult {
    #[allow(dead_code)]
    satisfied: Vec<PhysicalElement>,
    not_satisfied: Vec<PhysicalElement>,
    all_satisfied: bool,
}

/// Result of fault element analysis
#[derive(Debug, Clone)]
struct FaultElementResult {
    required: FaultElement,
    #[allow(dead_code)]
    proved: bool,
    honest_reasonable_mistake: bool,
    all_satisfied: bool,
}

/// Defence result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefenceResult {
    /// Defence type
    pub defence: Defence,
    /// Whether available
    pub available: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Liability status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LiabilityStatus {
    /// Guilty - all elements proved, no defence
    Guilty,
    /// Defence available
    DefenceAvailable,
    /// Prosecution failed to prove elements
    ElementsNotProved,
}

/// Result of offence analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OffenceResult {
    /// Offence description
    pub offence_description: String,
    /// Physical elements satisfied
    pub physical_elements_satisfied: bool,
    /// Fault elements satisfied
    pub fault_elements_satisfied: bool,
    /// Available defences
    pub available_defences: Vec<DefenceResult>,
    /// Liability status
    pub liability_status: LiabilityStatus,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offence_all_elements_proved() {
        let facts = OffenceFacts {
            offence_description: "Fraud".to_string(),
            conduct_occurred: true,
            result_required: true,
            result_occurred: true,
            required_fault_element: FaultElement::Intention,
            fault_element_proved: true,
            ..Default::default()
        };

        let result = OffenceAnalyzer::analyze_offence(&facts);
        assert_eq!(result.liability_status, LiabilityStatus::Guilty);
    }

    #[test]
    fn test_offence_elements_not_proved() {
        let facts = OffenceFacts {
            offence_description: "Assault".to_string(),
            conduct_occurred: true,
            required_fault_element: FaultElement::Intention,
            fault_element_proved: false, // Not proved
            ..Default::default()
        };

        let result = OffenceAnalyzer::analyze_offence(&facts);
        assert_eq!(result.liability_status, LiabilityStatus::ElementsNotProved);
    }

    #[test]
    fn test_self_defence_available() {
        let facts = OffenceFacts {
            offence_description: "Assault".to_string(),
            conduct_occurred: true,
            required_fault_element: FaultElement::Intention,
            fault_element_proved: true,
            self_defence_claimed: true,
            believed_conduct_necessary: true,
            reasonable_grounds_for_belief: true,
            response_proportionate: true,
            ..Default::default()
        };

        let result = OffenceAnalyzer::analyze_offence(&facts);
        assert_eq!(result.liability_status, LiabilityStatus::DefenceAvailable);
        assert!(
            result
                .available_defences
                .iter()
                .any(|d| d.defence == Defence::SelfDefence)
        );
    }

    #[test]
    fn test_duress_not_available_for_murder() {
        let facts = OffenceFacts {
            offence_description: "Murder".to_string(),
            conduct_occurred: true,
            result_required: true,
            result_occurred: true,
            required_fault_element: FaultElement::Intention,
            fault_element_proved: true,
            duress_claimed: true,
            threat_of_serious_harm: true,
            no_reasonable_escape: true,
            offence_is_murder: true, // Duress not available for murder
            ..Default::default()
        };

        let result = OffenceAnalyzer::analyze_offence(&facts);
        assert_eq!(result.liability_status, LiabilityStatus::Guilty);
        assert!(result.available_defences.is_empty());
    }

    #[test]
    fn test_strict_liability_with_mistake() {
        let facts = OffenceFacts {
            offence_description: "Regulatory offence".to_string(),
            conduct_occurred: true,
            strict_liability: true,
            honest_reasonable_mistake: true,
            ..Default::default()
        };

        let result = OffenceAnalyzer::analyze_offence(&facts);
        // Honest and reasonable mistake is a defence to strict liability
        assert!(!result.fault_elements_satisfied);
        assert!(result.reasoning.contains("mistake"));
    }
}
