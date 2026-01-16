//! Canada Contract Law - Breach and Remedies
//!
//! Analyzers for breach of contract and available remedies.

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use super::types::{
    BreachType, ContractCase, ContractRemedy, ContractTerm, DamagesCalculation, ExclusionClause,
    TermClassification,
};

// ============================================================================
// Breach Analysis
// ============================================================================

/// Facts for analyzing breach of contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreachFacts {
    /// Term(s) allegedly breached
    pub breached_terms: Vec<ContractTerm>,
    /// Description of the breach
    pub breach_description: String,
    /// Consequences of the breach
    pub consequences: Vec<String>,
    /// Whether breach was anticipatory
    pub anticipatory: bool,
    /// Whether other party has accepted repudiation
    pub repudiation_accepted: bool,
    /// Exclusion clauses that may apply
    pub exclusion_clauses: Vec<ExclusionClause>,
    /// Whether innocent party has mitigated
    pub mitigation_efforts: Vec<String>,
}

/// Result of breach analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreachResult {
    /// Whether there is a breach
    pub breach_established: bool,
    /// Type of breach
    pub breach_type: Option<BreachType>,
    /// Whether innocent party can terminate
    pub can_terminate: bool,
    /// Available remedies
    pub available_remedies: Vec<ContractRemedy>,
    /// Whether exclusion clause applies
    pub exclusion_clause_applies: bool,
    /// Key cases
    pub key_cases: Vec<ContractCase>,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Breach Analyzer
// ============================================================================

/// Analyzer for breach of contract
pub struct BreachAnalyzer;

impl BreachAnalyzer {
    /// Analyze breach of contract
    pub fn analyze(facts: &BreachFacts) -> BreachResult {
        let mut key_cases = Vec::new();

        // Determine if breach established
        let breach_established = !facts.breached_terms.is_empty();

        if !breach_established {
            return BreachResult {
                breach_established: false,
                breach_type: None,
                can_terminate: false,
                available_remedies: vec![],
                exclusion_clause_applies: false,
                key_cases: vec![],
                reasoning: "No breach of contract established".to_string(),
            };
        }

        // Determine type of breach
        let breach_type = Self::classify_breach(facts);

        // Determine if innocent party can terminate
        let can_terminate = Self::can_terminate(&breach_type);

        // Determine available remedies
        let available_remedies = Self::determine_remedies(&breach_type, facts);

        // Check exclusion clauses
        let exclusion_clause_applies = Self::check_exclusion_clauses(&facts.exclusion_clauses);

        // Add Tercon case for exclusion clauses
        if !facts.exclusion_clauses.is_empty() {
            key_cases.push(ContractCase::tercon());
            key_cases.push(ContractCase::hunter_engineering());
        }

        // Add Hadley v Baxendale for damages
        if available_remedies.contains(&ContractRemedy::ExpectationDamages) {
            key_cases.push(ContractCase::hadley_v_baxendale());
        }

        let reasoning =
            Self::build_reasoning(&breach_type, can_terminate, exclusion_clause_applies);

        BreachResult {
            breach_established,
            breach_type: Some(breach_type),
            can_terminate,
            available_remedies,
            exclusion_clause_applies,
            key_cases,
            reasoning,
        }
    }

    /// Classify the type of breach
    fn classify_breach(facts: &BreachFacts) -> BreachType {
        if facts.anticipatory {
            return BreachType::AnticipatoryBreach;
        }

        // Find the most serious breach type across all breached terms
        // Priority: Condition > Serious Innominate > Minor Innominate/Warranty
        let mut has_condition = false;
        let mut has_innominate = false;

        for term in &facts.breached_terms {
            match term.classification {
                TermClassification::Condition => {
                    has_condition = true;
                }
                TermClassification::Innominate => {
                    has_innominate = true;
                }
                TermClassification::Warranty => {
                    // Continue checking for more serious breaches
                }
            }
        }

        // Return based on most serious breach found
        if has_condition {
            return BreachType::BreachOfCondition;
        }

        if has_innominate {
            // Depends on consequences - Hong Kong Fir approach
            if Self::consequences_serious(&facts.consequences) {
                return BreachType::SeriousBreachOfInnominate;
            } else {
                return BreachType::MinorBreachOfInnominate;
            }
        }

        BreachType::BreachOfWarranty
    }

    /// Check if consequences are serious (for innominate terms)
    fn consequences_serious(consequences: &[String]) -> bool {
        // Serious if substantially deprived of whole benefit
        consequences.len() >= 3
            || consequences.iter().any(|c| {
                c.to_lowercase().contains("substantial")
                    || c.to_lowercase().contains("fundamental")
                    || c.to_lowercase().contains("entire")
            })
    }

    /// Determine if innocent party can terminate
    fn can_terminate(breach_type: &BreachType) -> bool {
        matches!(
            breach_type,
            BreachType::BreachOfCondition
                | BreachType::SeriousBreachOfInnominate
                | BreachType::AnticipatoryBreach
                | BreachType::Repudiation
                | BreachType::FundamentalBreach
        )
    }

    /// Determine available remedies
    fn determine_remedies(breach_type: &BreachType, facts: &BreachFacts) -> Vec<ContractRemedy> {
        let mut remedies = Vec::new();

        // Damages always available
        remedies.push(ContractRemedy::ExpectationDamages);

        // Can claim reliance as alternative
        remedies.push(ContractRemedy::RelianceDamages);

        // Termination remedies if can terminate
        if Self::can_terminate(breach_type) {
            remedies.push(ContractRemedy::Rescission);
        }

        // Specific performance (exceptional)
        if Self::specific_performance_available(facts) {
            remedies.push(ContractRemedy::SpecificPerformance);
        }

        // Restitution
        remedies.push(ContractRemedy::Restitution);

        remedies
    }

    /// Check if specific performance available
    fn specific_performance_available(facts: &BreachFacts) -> bool {
        // Available where damages inadequate (unique goods, land, etc.)
        facts.breached_terms.iter().any(|t| {
            t.description.to_lowercase().contains("land")
                || t.description.to_lowercase().contains("unique")
                || t.description.to_lowercase().contains("real property")
        })
    }

    /// Check if exclusion clause applies (Tercon framework)
    fn check_exclusion_clauses(clauses: &[ExclusionClause]) -> bool {
        for clause in clauses {
            // Step 1: Does clause apply?
            if !clause.incorporated || !clause.covers_breach {
                continue;
            }

            // Step 2: Is clause unconscionable?
            if clause.unconscionable {
                continue;
            }

            // Step 3: Public policy
            if clause.consumer_protection.is_some() {
                // Consumer protection may override
                continue;
            }

            return true;
        }
        false
    }

    /// Build reasoning
    fn build_reasoning(
        breach_type: &BreachType,
        can_terminate: bool,
        exclusion_applies: bool,
    ) -> String {
        let breach_desc = match breach_type {
            BreachType::BreachOfCondition => "breach of condition (essential term)",
            BreachType::BreachOfWarranty => "breach of warranty (minor term)",
            BreachType::SeriousBreachOfInnominate => {
                "serious breach of innominate term (substantial deprivation)"
            }
            BreachType::MinorBreachOfInnominate => "minor breach of innominate term",
            BreachType::AnticipatoryBreach => "anticipatory breach",
            BreachType::Repudiation => "repudiation",
            BreachType::FundamentalBreach => "fundamental breach",
        };

        let termination = if can_terminate {
            "Innocent party may elect to terminate the contract."
        } else {
            "Innocent party must continue to perform; damages only."
        };

        let exclusion = if exclusion_applies {
            " Exclusion clause applies to limit liability (Tercon)."
        } else {
            ""
        };

        format!(
            "This constitutes a {}. {} Damages available subject to remoteness \
             (Hadley v Baxendale) and mitigation.{}",
            breach_desc, termination, exclusion
        )
    }
}

// ============================================================================
// Damages Analyzer
// ============================================================================

/// Analyzer for contract damages
pub struct DamagesAnalyzer;

impl DamagesAnalyzer {
    /// Analyze damages claim
    pub fn analyze(facts: &DamagesFacts) -> DamagesResult {
        let mut key_cases = vec![ContractCase::hadley_v_baxendale()];

        // Check remoteness
        let remoteness_satisfied = Self::check_remoteness(facts);

        // Check mitigation
        let mitigation_satisfied = Self::check_mitigation(facts);

        // Calculate quantum
        let calculation =
            Self::calculate_quantum(facts, remoteness_satisfied, mitigation_satisfied);

        // Mental distress damages
        let mental_distress_available = if facts.peace_of_mind_contract {
            key_cases.push(ContractCase::fidler());
            true
        } else {
            false
        };

        DamagesResult {
            remoteness_satisfied,
            mitigation_satisfied,
            calculation,
            mental_distress_available,
            key_cases,
            reasoning: Self::build_reasoning(remoteness_satisfied, mitigation_satisfied),
        }
    }

    /// Check remoteness (Hadley v Baxendale)
    fn check_remoteness(facts: &DamagesFacts) -> bool {
        // First limb: arising naturally from breach
        if facts.arises_naturally {
            return true;
        }

        // Second limb: in contemplation of parties
        if facts.in_contemplation {
            return true;
        }

        false
    }

    /// Check mitigation
    fn check_mitigation(facts: &DamagesFacts) -> bool {
        // Must take reasonable steps to mitigate
        !facts.mitigation_steps.is_empty() || facts.no_mitigation_possible
    }

    /// Calculate quantum
    fn calculate_quantum(
        facts: &DamagesFacts,
        remoteness: bool,
        mitigation: bool,
    ) -> DamagesCalculation {
        let recoverable = remoteness && mitigation;

        DamagesCalculation {
            damages_type: ContractRemedy::ExpectationDamages,
            quantum_cents: if recoverable {
                facts.claimed_amount_cents
            } else {
                None
            },
            remoteness_satisfied: remoteness,
            mitigation_satisfied: mitigation,
            calculation: if recoverable {
                "Damages recoverable: remoteness and mitigation satisfied".to_string()
            } else {
                format!(
                    "Damages reduced or barred: remoteness={}, mitigation={}",
                    remoteness, mitigation
                )
            },
        }
    }

    /// Build reasoning
    fn build_reasoning(remoteness: bool, mitigation: bool) -> String {
        let remote_str = if remoteness {
            "Remoteness satisfied (Hadley v Baxendale)"
        } else {
            "Damages too remote under Hadley v Baxendale"
        };

        let mitigate_str = if mitigation {
            "Mitigation requirement satisfied"
        } else {
            "Failure to mitigate may reduce damages"
        };

        format!("{}. {}.", remote_str, mitigate_str)
    }
}

/// Facts for damages analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamagesFacts {
    /// Amount claimed (cents)
    pub claimed_amount_cents: Option<i64>,
    /// Whether loss arises naturally from breach
    pub arises_naturally: bool,
    /// Whether loss was in contemplation of parties
    pub in_contemplation: bool,
    /// Mitigation steps taken
    pub mitigation_steps: Vec<String>,
    /// Whether mitigation was possible
    pub no_mitigation_possible: bool,
    /// Whether contract was for peace of mind
    pub peace_of_mind_contract: bool,
}

/// Result of damages analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamagesResult {
    /// Whether remoteness satisfied
    pub remoteness_satisfied: bool,
    /// Whether mitigation satisfied
    pub mitigation_satisfied: bool,
    /// Damages calculation
    pub calculation: DamagesCalculation,
    /// Whether mental distress damages available
    pub mental_distress_available: bool,
    /// Key cases
    pub key_cases: Vec<ContractCase>,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract::types::TermType;

    #[test]
    fn test_breach_of_condition() {
        let facts = BreachFacts {
            breached_terms: vec![ContractTerm {
                description: "Delivery date".to_string(),
                classification: TermClassification::Condition,
                term_type: TermType::Express,
                is_exclusion_clause: false,
            }],
            breach_description: "Late delivery".to_string(),
            consequences: vec!["Lost business".to_string()],
            anticipatory: false,
            repudiation_accepted: false,
            exclusion_clauses: vec![],
            mitigation_efforts: vec![],
        };

        let result = BreachAnalyzer::analyze(&facts);
        assert!(result.breach_established);
        assert!(matches!(
            result.breach_type,
            Some(BreachType::BreachOfCondition)
        ));
        assert!(result.can_terminate);
    }

    #[test]
    fn test_breach_of_warranty() {
        let facts = BreachFacts {
            breached_terms: vec![ContractTerm {
                description: "Packaging color".to_string(),
                classification: TermClassification::Warranty,
                term_type: TermType::Express,
                is_exclusion_clause: false,
            }],
            breach_description: "Wrong color".to_string(),
            consequences: vec!["Minor inconvenience".to_string()],
            anticipatory: false,
            repudiation_accepted: false,
            exclusion_clauses: vec![],
            mitigation_efforts: vec![],
        };

        let result = BreachAnalyzer::analyze(&facts);
        assert!(result.breach_established);
        assert!(matches!(
            result.breach_type,
            Some(BreachType::BreachOfWarranty)
        ));
        assert!(!result.can_terminate);
    }

    #[test]
    fn test_damages_remoteness() {
        let facts = DamagesFacts {
            claimed_amount_cents: Some(100000),
            arises_naturally: true,
            in_contemplation: false,
            mitigation_steps: vec!["Sought alternative supplier".to_string()],
            no_mitigation_possible: false,
            peace_of_mind_contract: false,
        };

        let result = DamagesAnalyzer::analyze(&facts);
        assert!(result.remoteness_satisfied);
        assert!(result.mitigation_satisfied);
    }

    #[test]
    fn test_exclusion_clause_applies() {
        let facts = BreachFacts {
            breached_terms: vec![ContractTerm {
                description: "Service level".to_string(),
                classification: TermClassification::Warranty,
                term_type: TermType::Express,
                is_exclusion_clause: false,
            }],
            breach_description: "Service delayed".to_string(),
            consequences: vec![],
            anticipatory: false,
            repudiation_accepted: false,
            exclusion_clauses: vec![ExclusionClause {
                clause: "Liability limited to refund".to_string(),
                incorporated: true,
                covers_breach: true,
                unconscionable: false,
                consumer_protection: None,
            }],
            mitigation_efforts: vec![],
        };

        let result = BreachAnalyzer::analyze(&facts);
        assert!(result.exclusion_clause_applies);
    }
}
