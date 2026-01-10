//! Restatement (First) of Conflict of Laws (1934)
//!
//! Traditional choice of law approach using mechanical rules based on territorial
//! principles. Each legal issue has a specific connecting factor that determines
//! the applicable law.
//!
//! ## Key Principles
//!
//! ### Torts: Lex Loci Delicti (Place of Wrong)
//! - Apply law of state where injury occurred
//! - Simple, mechanical rule
//! - No consideration of policy or parties' interests
//!
//! ### Contracts: Lex Loci Contractus (Place of Contract)
//! - Formation: Law of place where contract was made
//! - Performance: Law of place of performance
//!
//! ## Criticism
//! - Too mechanical and inflexible
//! - Ignores important policy considerations
//! - Can lead to arbitrary results
//! - Only 6 states still follow this approach
//!
//! ## Current Status
//! Minority approach - Most states have adopted Restatement (Second) or other modern approaches.

use super::factors::{ContactingFactor, USChoiceOfLawFactors};
use serde::{Deserialize, Serialize};

/// Restatement (First) choice of law analyzer.
#[derive(Debug, Clone)]
pub struct RestatementFirst {
    /// Whether to apply exception for public policy
    apply_public_policy_exception: bool,
}

impl Default for RestatementFirst {
    fn default() -> Self {
        Self::new()
    }
}

impl RestatementFirst {
    /// Create new Restatement (First) analyzer.
    #[must_use]
    pub fn new() -> Self {
        Self {
            apply_public_policy_exception: true,
        }
    }

    /// Disable public policy exception.
    #[must_use]
    pub fn without_public_policy_exception(mut self) -> Self {
        self.apply_public_policy_exception = false;
        self
    }

    /// Analyze choice of law for torts under Restatement (First).
    ///
    /// Rule: Apply law of state where injury occurred (lex loci delicti).
    #[must_use]
    pub fn analyze_tort(&self, factors: &USChoiceOfLawFactors) -> RestatementFirstResult {
        // Look for place of injury
        let place_of_injury = factors.factors().iter().find_map(|f| {
            if let ContactingFactor::PlaceOfInjury(state) = f {
                Some(state.clone())
            } else {
                None
            }
        });

        if let Some(state) = place_of_injury {
            RestatementFirstResult {
                applicable_law: state.clone(),
                rule: RestatementFirstRule::LexLociDelicti,
                explanation: format!(
                    "Under Restatement (First) § 377, the law of the place of wrong (lex loci \
                     delicti) applies to torts. The injury occurred in {state}, so {state} law governs."
                ),
                public_policy_exception_applies: false,
            }
        } else {
            // Fallback: Use place of conduct if injury location unknown
            let place_of_conduct = factors.factors().iter().find_map(|f| {
                if let ContactingFactor::PlaceOfConduct(state) = f {
                    Some(state.clone())
                } else {
                    None
                }
            });

            if let Some(state) = place_of_conduct {
                RestatementFirstResult {
                    applicable_law: state.clone(),
                    rule: RestatementFirstRule::LexLociDelicti,
                    explanation: format!(
                        "Place of injury unclear. Using place of conduct ({state}) as proxy \
                         for place of wrong under Restatement (First) § 377."
                    ),
                    public_policy_exception_applies: false,
                }
            } else {
                // No clear place of wrong - cannot determine
                RestatementFirstResult {
                    applicable_law: "UNKNOWN".to_string(),
                    rule: RestatementFirstRule::Indeterminate,
                    explanation: "Cannot determine place of wrong. Restatement (First) requires \
                                  clear identification of injury location."
                        .to_string(),
                    public_policy_exception_applies: false,
                }
            }
        }
    }

    /// Analyze choice of law for contracts under Restatement (First).
    ///
    /// Rule: Apply law of place where contract was made (lex loci contractus).
    #[must_use]
    pub fn analyze_contract(&self, factors: &USChoiceOfLawFactors) -> RestatementFirstResult {
        // Look for place of execution (where contract was made)
        let place_of_execution = factors.factors().iter().find_map(|f| {
            if let ContactingFactor::PlaceOfExecution(state) = f {
                Some(state.clone())
            } else {
                None
            }
        });

        if let Some(state) = place_of_execution {
            RestatementFirstResult {
                applicable_law: state.clone(),
                rule: RestatementFirstRule::LexLociContractus,
                explanation: format!(
                    "Under Restatement (First) § 332, the law of the place where contract was \
                     made (lex loci contractus) governs contract formation and validity. \
                     Contract executed in {state}."
                ),
                public_policy_exception_applies: false,
            }
        } else {
            // Fallback: Place of negotiation
            let place_of_negotiation = factors.factors().iter().find_map(|f| {
                if let ContactingFactor::PlaceOfNegotiation(state) = f {
                    Some(state.clone())
                } else {
                    None
                }
            });

            if let Some(state) = place_of_negotiation {
                RestatementFirstResult {
                    applicable_law: state.clone(),
                    rule: RestatementFirstRule::LexLociContractus,
                    explanation: format!(
                        "Place of execution unclear. Using place of negotiation ({state}) as \
                         proxy under Restatement (First) § 332."
                    ),
                    public_policy_exception_applies: false,
                }
            } else {
                RestatementFirstResult {
                    applicable_law: "UNKNOWN".to_string(),
                    rule: RestatementFirstRule::Indeterminate,
                    explanation: "Cannot determine where contract was made. Restatement (First) \
                                  requires clear place of contract formation."
                        .to_string(),
                    public_policy_exception_applies: false,
                }
            }
        }
    }
}

/// Restatement (First) rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RestatementFirstRule {
    /// Lex loci delicti - Law of place of wrong (torts)
    LexLociDelicti,

    /// Lex loci contractus - Law of place where contract made
    LexLociContractus,

    /// Cannot determine applicable law
    Indeterminate,
}

/// Result of Restatement (First) analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestatementFirstResult {
    /// State whose law applies
    pub applicable_law: String,

    /// Rule applied
    pub rule: RestatementFirstRule,

    /// Explanation of analysis
    pub explanation: String,

    /// Whether public policy exception applies (forum refuses to apply foreign law)
    pub public_policy_exception_applies: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restatement_first_creation() {
        let rst = RestatementFirst::new();
        assert!(rst.apply_public_policy_exception);

        let rst = RestatementFirst::new().without_public_policy_exception();
        assert!(!rst.apply_public_policy_exception);
    }

    #[test]
    fn test_tort_place_of_injury() {
        let rst = RestatementFirst::new();

        let factors = USChoiceOfLawFactors::new()
            .with_factor(ContactingFactor::PlaceOfInjury("CA".to_string()))
            .with_factor(ContactingFactor::DefendantDomicile("NY".to_string()));

        let result = rst.analyze_tort(&factors);

        assert_eq!(result.applicable_law, "CA");
        assert_eq!(result.rule, RestatementFirstRule::LexLociDelicti);
        assert!(result.explanation.contains("place of wrong"));
        assert!(result.explanation.contains("CA"));
    }

    #[test]
    fn test_tort_fallback_to_conduct() {
        let rst = RestatementFirst::new();

        let factors = USChoiceOfLawFactors::new()
            .with_factor(ContactingFactor::PlaceOfConduct("TX".to_string()))
            .with_factor(ContactingFactor::DefendantDomicile("NY".to_string()));

        let result = rst.analyze_tort(&factors);

        assert_eq!(result.applicable_law, "TX");
        assert_eq!(result.rule, RestatementFirstRule::LexLociDelicti);
        assert!(result.explanation.contains("place of conduct"));
    }

    #[test]
    fn test_tort_indeterminate() {
        let rst = RestatementFirst::new();

        let factors = USChoiceOfLawFactors::new()
            .with_factor(ContactingFactor::DefendantDomicile("NY".to_string()));

        let result = rst.analyze_tort(&factors);

        assert_eq!(result.applicable_law, "UNKNOWN");
        assert_eq!(result.rule, RestatementFirstRule::Indeterminate);
    }

    #[test]
    fn test_contract_place_of_execution() {
        let rst = RestatementFirst::new();

        let factors = USChoiceOfLawFactors::new()
            .with_factor(ContactingFactor::PlaceOfExecution("NY".to_string()))
            .with_factor(ContactingFactor::PlaceOfPerformance("CA".to_string()));

        let result = rst.analyze_contract(&factors);

        assert_eq!(result.applicable_law, "NY");
        assert_eq!(result.rule, RestatementFirstRule::LexLociContractus);
        assert!(result.explanation.contains("place where contract was made"));
    }

    #[test]
    fn test_contract_fallback_to_negotiation() {
        let rst = RestatementFirst::new();

        let factors = USChoiceOfLawFactors::new()
            .with_factor(ContactingFactor::PlaceOfNegotiation("FL".to_string()));

        let result = rst.analyze_contract(&factors);

        assert_eq!(result.applicable_law, "FL");
        assert_eq!(result.rule, RestatementFirstRule::LexLociContractus);
    }

    #[test]
    fn test_contract_indeterminate() {
        let rst = RestatementFirst::new();

        let factors = USChoiceOfLawFactors::new()
            .with_factor(ContactingFactor::PlaintiffDomicile("CA".to_string()));

        let result = rst.analyze_contract(&factors);

        assert_eq!(result.applicable_law, "UNKNOWN");
        assert_eq!(result.rule, RestatementFirstRule::Indeterminate);
    }
}
