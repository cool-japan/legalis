//! Contract Breach and Remedies
//!
//! Analysis of breach and available remedies under Australian law.

use serde::{Deserialize, Serialize};

use super::types::{BreachType, ContractRemedy, DamagesType, TermClassification};

// ============================================================================
// Breach Analysis
// ============================================================================

/// Analyzer for breach of contract
pub struct BreachAnalyzer;

impl BreachAnalyzer {
    /// Analyze breach of contract
    pub fn analyze(facts: &BreachFacts) -> BreachResult {
        let breach_type = Self::classify_breach(facts);
        let repudiatory = Self::is_repudiatory(&breach_type, facts);
        let available_remedies = Self::determine_remedies(&breach_type, facts);

        let reasoning = Self::build_reasoning(facts, &breach_type, repudiatory);

        BreachResult {
            breach_occurred: facts.failure_to_perform
                || facts.defective_performance
                || facts.anticipatory_breach
                || facts.express_refusal_to_perform,
            breach_type,
            repudiatory,
            can_terminate: repudiatory && facts.innocent_party_ready_willing,
            available_remedies,
            reasoning,
        }
    }

    /// Classify the breach
    fn classify_breach(facts: &BreachFacts) -> Option<BreachType> {
        // Check for anticipatory breach first (doesn't require actual failure)
        if facts.anticipatory_breach {
            return Some(BreachType::Anticipatory);
        }

        // Check for renunciation (express refusal)
        if facts.express_refusal_to_perform {
            return Some(BreachType::Renunciation);
        }

        // For other breach types, need actual failure or defective performance
        if !facts.failure_to_perform && !facts.defective_performance {
            return None;
        }

        // Classify by term type
        match &facts.term_classification {
            Some(TermClassification::Condition) => Some(BreachType::Condition),
            Some(TermClassification::Warranty) => Some(BreachType::Warranty),
            Some(TermClassification::Intermediate) => {
                // Hong Kong Fir: look at consequences
                if facts.deprives_substantially_whole_benefit {
                    Some(BreachType::Fundamental)
                } else {
                    Some(BreachType::Intermediate)
                }
            }
            None => Some(BreachType::Warranty), // Default
        }
    }

    /// Determine if breach is repudiatory
    fn is_repudiatory(breach_type: &Option<BreachType>, facts: &BreachFacts) -> bool {
        match breach_type {
            Some(BreachType::Condition) => true,
            Some(BreachType::Fundamental) => true,
            Some(BreachType::Anticipatory) => true,
            Some(BreachType::Renunciation) => true,
            Some(BreachType::Intermediate) => {
                // Hong Kong Fir: depends on consequences
                facts.deprives_substantially_whole_benefit
            }
            Some(BreachType::Warranty) => false,
            None => false,
        }
    }

    /// Determine available remedies
    fn determine_remedies(
        breach_type: &Option<BreachType>,
        facts: &BreachFacts,
    ) -> Vec<ContractRemedy> {
        let mut remedies = Vec::new();

        if breach_type.is_none() {
            return remedies;
        }

        // Damages always available for breach
        remedies.push(ContractRemedy::Damages);

        let repudiatory = Self::is_repudiatory(breach_type, facts);

        if repudiatory && facts.innocent_party_ready_willing {
            remedies.push(ContractRemedy::Termination);
        }

        // Specific performance if appropriate
        if facts.damages_inadequate
            && facts.contract_certain_and_fair
            && !facts.personal_service_contract
        {
            remedies.push(ContractRemedy::SpecificPerformance);
        }

        // Injunction
        if facts.prohibitory_injunction_appropriate {
            remedies.push(ContractRemedy::Injunction);
        }

        // Quantum meruit for part performance
        if facts.partial_performance_accepted {
            remedies.push(ContractRemedy::QuantumMeruit);
        }

        remedies
    }

    /// Build reasoning
    fn build_reasoning(
        facts: &BreachFacts,
        breach_type: &Option<BreachType>,
        repudiatory: bool,
    ) -> String {
        let mut parts = Vec::new();

        parts.push("Breach of contract analysis".to_string());

        match breach_type {
            Some(BreachType::Condition) => {
                parts.push("Breach of condition - repudiatory breach".to_string());
                parts.push("Innocent party may terminate and claim damages".to_string());
            }
            Some(BreachType::Warranty) => {
                parts.push("Breach of warranty - damages only".to_string());
                parts.push("No right to terminate".to_string());
            }
            Some(BreachType::Intermediate) => {
                parts.push("Breach of intermediate term (Hong Kong Fir)".to_string());
                if repudiatory {
                    parts.push("Consequences sufficiently serious - repudiatory".to_string());
                } else {
                    parts.push("Consequences not serious enough for termination".to_string());
                }
            }
            Some(BreachType::Anticipatory) => {
                parts.push("Anticipatory breach - breach before performance due".to_string());
                parts.push("Innocent party can treat as repudiation or wait".to_string());
            }
            Some(BreachType::Renunciation) => {
                parts.push("Renunciation - express refusal to perform".to_string());
                parts.push("Repudiatory breach".to_string());
            }
            Some(BreachType::Fundamental) => {
                parts.push("Fundamental breach of intermediate term (Hong Kong Fir)".to_string());
                parts.push("Deprives of substantially whole benefit - repudiatory".to_string());
            }
            None => {
                parts.push("No breach established".to_string());
            }
        }

        if facts.affirmation_occurred {
            parts.push(
                "Warning: affirmation may have occurred - loss of right to terminate".to_string(),
            );
        }

        parts.join(". ")
    }
}

/// Facts for breach analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BreachFacts {
    /// Failure to perform
    pub failure_to_perform: bool,
    /// Defective performance
    pub defective_performance: bool,
    /// Term classification
    pub term_classification: Option<TermClassification>,
    /// Anticipatory breach
    pub anticipatory_breach: bool,
    /// Express refusal to perform
    pub express_refusal_to_perform: bool,
    /// Deprives of substantially whole benefit
    pub deprives_substantially_whole_benefit: bool,
    /// Innocent party ready and willing
    pub innocent_party_ready_willing: bool,
    /// Affirmation occurred
    pub affirmation_occurred: bool,
    /// Damages inadequate
    pub damages_inadequate: bool,
    /// Contract certain and fair
    pub contract_certain_and_fair: bool,
    /// Personal service contract
    pub personal_service_contract: bool,
    /// Prohibitory injunction appropriate
    pub prohibitory_injunction_appropriate: bool,
    /// Partial performance accepted
    pub partial_performance_accepted: bool,
}

/// Result of breach analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreachResult {
    /// Breach occurred
    pub breach_occurred: bool,
    /// Type of breach
    pub breach_type: Option<BreachType>,
    /// Whether repudiatory
    pub repudiatory: bool,
    /// Can terminate
    pub can_terminate: bool,
    /// Available remedies
    pub available_remedies: Vec<ContractRemedy>,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Damages Analysis
// ============================================================================

/// Analyzer for contract damages
pub struct DamagesAnalyzer;

impl DamagesAnalyzer {
    /// Analyze damages claim
    pub fn analyze(facts: &DamagesFacts) -> DamagesResult {
        let causation = Self::check_causation(facts);
        let remoteness = Self::check_remoteness(facts);
        let mitigation = Self::check_mitigation(facts);

        let recoverable = causation && remoteness;
        let damages_type = Self::determine_damages_type(facts);
        let quantum = Self::calculate_quantum(facts, mitigation);

        let reasoning = Self::build_reasoning(facts, causation, remoteness, mitigation);

        DamagesResult {
            recoverable,
            damages_type,
            quantum,
            reduction_for_mitigation: facts.failed_to_mitigate,
            reasoning,
        }
    }

    /// Check causation (but-for test)
    fn check_causation(facts: &DamagesFacts) -> bool {
        facts.loss_caused_by_breach
    }

    /// Check remoteness (Hadley v Baxendale)
    fn check_remoteness(facts: &DamagesFacts) -> bool {
        // Two limbs of Hadley v Baxendale:
        // 1. Arising naturally from breach
        // 2. In contemplation of parties at contract time

        facts.loss_arising_naturally || facts.loss_in_contemplation
    }

    /// Check mitigation
    fn check_mitigation(facts: &DamagesFacts) -> bool {
        facts.reasonable_steps_to_mitigate || !facts.failed_to_mitigate
    }

    /// Determine damages type
    fn determine_damages_type(facts: &DamagesFacts) -> DamagesType {
        if facts.claiming_expectation_loss {
            DamagesType::Expectation
        } else if facts.claiming_reliance_loss {
            DamagesType::Reliance
        } else if facts.claiming_restitution {
            DamagesType::Restitution
        } else {
            DamagesType::Nominal
        }
    }

    /// Calculate quantum
    fn calculate_quantum(facts: &DamagesFacts, mitigated: bool) -> Option<f64> {
        let base = facts.claimed_amount?;

        let adjusted = if !mitigated && facts.failed_to_mitigate {
            // Reduce by avoidable loss
            base - facts.avoidable_loss.unwrap_or(0.0)
        } else {
            base
        };

        Some(adjusted.max(0.0))
    }

    /// Build reasoning
    fn build_reasoning(
        facts: &DamagesFacts,
        causation: bool,
        remoteness: bool,
        mitigation: bool,
    ) -> String {
        let mut parts = Vec::new();

        parts.push("Damages analysis".to_string());

        // Causation
        if causation {
            parts.push("Causation established: loss caused by breach".to_string());
        } else {
            parts.push("Causation not established".to_string());
            return parts.join(". ");
        }

        // Remoteness
        if remoteness {
            parts.push("Remoteness satisfied (Hadley v Baxendale)".to_string());
            if facts.loss_arising_naturally {
                parts.push("Loss arises naturally from breach (first limb)".to_string());
            }
            if facts.loss_in_contemplation {
                parts.push("Loss in contemplation of parties (second limb)".to_string());
            }
        } else {
            parts.push("Loss too remote - not recoverable".to_string());
            return parts.join(". ");
        }

        // Mitigation
        if !mitigation {
            parts.push("Failed to mitigate: damages reduced".to_string());
            parts.push("Duty to take reasonable steps (British Westinghouse)".to_string());
        }

        // Type of damages
        if facts.claiming_expectation_loss {
            parts.push("Expectation damages: put in position as if contract performed".to_string());
        } else if facts.claiming_reliance_loss {
            parts.push("Reliance damages: put in pre-contract position".to_string());
        }

        parts.join(". ")
    }
}

/// Facts for damages analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DamagesFacts {
    /// Loss caused by breach
    pub loss_caused_by_breach: bool,
    /// Loss arising naturally
    pub loss_arising_naturally: bool,
    /// Loss in contemplation of parties
    pub loss_in_contemplation: bool,
    /// Reasonable steps to mitigate
    pub reasonable_steps_to_mitigate: bool,
    /// Failed to mitigate
    pub failed_to_mitigate: bool,
    /// Avoidable loss amount
    pub avoidable_loss: Option<f64>,
    /// Claiming expectation loss
    pub claiming_expectation_loss: bool,
    /// Claiming reliance loss
    pub claiming_reliance_loss: bool,
    /// Claiming restitution
    pub claiming_restitution: bool,
    /// Claimed amount
    pub claimed_amount: Option<f64>,
}

/// Result of damages analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamagesResult {
    /// Whether damages recoverable
    pub recoverable: bool,
    /// Type of damages
    pub damages_type: DamagesType,
    /// Quantum
    pub quantum: Option<f64>,
    /// Whether reduced for failure to mitigate
    pub reduction_for_mitigation: bool,
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
    fn test_breach_of_condition() {
        let facts = BreachFacts {
            failure_to_perform: true,
            term_classification: Some(TermClassification::Condition),
            innocent_party_ready_willing: true,
            ..Default::default()
        };

        let result = BreachAnalyzer::analyze(&facts);
        assert!(result.breach_occurred);
        assert!(result.repudiatory);
        assert!(result.can_terminate);
    }

    #[test]
    fn test_breach_of_warranty() {
        let facts = BreachFacts {
            defective_performance: true,
            term_classification: Some(TermClassification::Warranty),
            innocent_party_ready_willing: true,
            ..Default::default()
        };

        let result = BreachAnalyzer::analyze(&facts);
        assert!(result.breach_occurred);
        assert!(!result.repudiatory);
        assert!(!result.can_terminate);
        assert!(result.available_remedies.contains(&ContractRemedy::Damages));
    }

    #[test]
    fn test_intermediate_term_serious() {
        let facts = BreachFacts {
            failure_to_perform: true,
            term_classification: Some(TermClassification::Intermediate),
            deprives_substantially_whole_benefit: true,
            innocent_party_ready_willing: true,
            ..Default::default()
        };

        let result = BreachAnalyzer::analyze(&facts);
        assert!(result.repudiatory);
        assert!(result.reasoning.contains("Hong Kong Fir"));
    }

    #[test]
    fn test_anticipatory_breach() {
        let facts = BreachFacts {
            anticipatory_breach: true,
            innocent_party_ready_willing: true,
            ..Default::default()
        };

        let result = BreachAnalyzer::analyze(&facts);
        assert_eq!(result.breach_type, Some(BreachType::Anticipatory));
        assert!(result.repudiatory);
    }

    #[test]
    fn test_damages_recoverable() {
        let facts = DamagesFacts {
            loss_caused_by_breach: true,
            loss_arising_naturally: true,
            reasonable_steps_to_mitigate: true,
            claiming_expectation_loss: true,
            claimed_amount: Some(10000.0),
            ..Default::default()
        };

        let result = DamagesAnalyzer::analyze(&facts);
        assert!(result.recoverable);
        assert_eq!(result.quantum, Some(10000.0));
    }

    #[test]
    fn test_damages_too_remote() {
        let facts = DamagesFacts {
            loss_caused_by_breach: true,
            loss_arising_naturally: false,
            loss_in_contemplation: false,
            ..Default::default()
        };

        let result = DamagesAnalyzer::analyze(&facts);
        assert!(!result.recoverable);
        assert!(result.reasoning.contains("remote"));
    }

    #[test]
    fn test_damages_failure_to_mitigate() {
        let facts = DamagesFacts {
            loss_caused_by_breach: true,
            loss_arising_naturally: true,
            failed_to_mitigate: true,
            avoidable_loss: Some(2000.0),
            claiming_expectation_loss: true,
            claimed_amount: Some(10000.0),
            ..Default::default()
        };

        let result = DamagesAnalyzer::analyze(&facts);
        assert!(result.recoverable);
        assert_eq!(result.quantum, Some(8000.0)); // Reduced
        assert!(result.reduction_for_mitigation);
    }
}
