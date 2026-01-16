//! Torrens Title System
//!
//! Implementation of Torrens title registration system.

use serde::{Deserialize, Serialize};

use super::types::{IndefeasibilityException, OverridingInterest};

// ============================================================================
// Indefeasibility Analyzer
// ============================================================================

/// Analyzer for Torrens title indefeasibility
pub struct IndefeasibilityAnalyzer;

impl IndefeasibilityAnalyzer {
    /// Analyze indefeasibility of title
    pub fn analyze(facts: &IndefeasibilityFacts) -> IndefeasibilityResult {
        let exceptions = Self::check_exceptions(facts);
        let overriding = Self::check_overriding_interests(facts);

        let indefeasible = exceptions.is_empty();
        let reasoning = Self::build_reasoning(facts, &exceptions, &overriding);

        IndefeasibilityResult {
            registered: facts.duly_registered,
            indefeasible,
            exceptions_apply: exceptions,
            overriding_interests: overriding,
            reasoning,
        }
    }

    /// Check exceptions to indefeasibility
    fn check_exceptions(facts: &IndefeasibilityFacts) -> Vec<IndefeasibilityException> {
        let mut exceptions = Vec::new();

        // Fraud exception
        if facts.fraud_by_registered_proprietor {
            exceptions.push(IndefeasibilityException::Fraud);
        }

        // Prior certificate
        if facts.prior_certificate_exists {
            exceptions.push(IndefeasibilityException::PriorCertificate);
        }

        // Boundary error
        if facts.boundary_encroachment {
            exceptions.push(IndefeasibilityException::BoundaryError);
        }

        exceptions
    }

    /// Check overriding interests
    fn check_overriding_interests(facts: &IndefeasibilityFacts) -> Vec<OverridingInterest> {
        let mut interests = Vec::new();

        if facts.person_in_actual_occupation {
            interests.push(OverridingInterest::ActualOccupation);
        }

        if facts.short_term_lease_exists {
            interests.push(OverridingInterest::ShortLease);
        }

        if facts.prescriptive_easement_exists {
            interests.push(OverridingInterest::PrescriptiveEasement);
        }

        interests
    }

    /// Build reasoning
    fn build_reasoning(
        facts: &IndefeasibilityFacts,
        exceptions: &[IndefeasibilityException],
        overriding: &[OverridingInterest],
    ) -> String {
        let mut parts = Vec::new();

        parts.push("Torrens title indefeasibility analysis".to_string());

        if facts.duly_registered {
            parts.push("Registration complete - indefeasibility prima facie applies".to_string());
            parts.push("Per Breskvar v Wall (1971): immediate indefeasibility".to_string());
        }

        if exceptions.is_empty() {
            parts.push("No exceptions to indefeasibility apply".to_string());
        } else {
            for exception in exceptions {
                match exception {
                    IndefeasibilityException::Fraud => {
                        parts.push(
                            "Exception: Fraud by registered proprietor (or agent)".to_string(),
                        );
                    }
                    IndefeasibilityException::PriorCertificate => {
                        parts.push("Exception: Prior certificate of title exists".to_string());
                    }
                    IndefeasibilityException::BoundaryError => {
                        parts.push("Exception: Error in description of boundaries".to_string());
                    }
                    _ => {
                        parts.push(format!("Exception: {:?}", exception));
                    }
                }
            }
        }

        if !overriding.is_empty() {
            parts.push("Overriding interests may affect title:".to_string());
            for interest in overriding {
                parts.push(format!("- {:?}", interest));
            }
        }

        parts.join(". ")
    }
}

/// Facts for indefeasibility analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IndefeasibilityFacts {
    /// Duly registered
    pub duly_registered: bool,
    /// Fraud by registered proprietor
    pub fraud_by_registered_proprietor: bool,
    /// Prior certificate exists
    pub prior_certificate_exists: bool,
    /// Boundary encroachment
    pub boundary_encroachment: bool,
    /// Person in actual occupation
    pub person_in_actual_occupation: bool,
    /// Short-term lease exists
    pub short_term_lease_exists: bool,
    /// Prescriptive easement exists
    pub prescriptive_easement_exists: bool,
}

/// Result of indefeasibility analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndefeasibilityResult {
    /// Registered on title
    pub registered: bool,
    /// Title indefeasible
    pub indefeasible: bool,
    /// Exceptions that apply
    pub exceptions_apply: Vec<IndefeasibilityException>,
    /// Overriding interests
    pub overriding_interests: Vec<OverridingInterest>,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Priority Analyzer
// ============================================================================

/// Analyzer for priority of interests
pub struct PriorityAnalyzer;

impl PriorityAnalyzer {
    /// Analyze priority between competing interests
    pub fn analyze(facts: &PriorityFacts) -> PriorityResult {
        let winner = Self::determine_priority(facts);
        let reasoning = Self::build_reasoning(facts, &winner);

        PriorityResult {
            first_interest_prevails: winner == PriorityWinner::First,
            second_interest_prevails: winner == PriorityWinner::Second,
            winner,
            reasoning,
        }
    }

    /// Determine priority
    fn determine_priority(facts: &PriorityFacts) -> PriorityWinner {
        // Registered vs unregistered - registered wins
        if facts.first_registered && !facts.second_registered {
            return PriorityWinner::First;
        }
        if facts.second_registered && !facts.first_registered {
            return PriorityWinner::Second;
        }

        // Both registered - first in time
        if facts.first_registered && facts.second_registered {
            if facts.first_registration_earlier {
                return PriorityWinner::First;
            } else {
                return PriorityWinner::Second;
            }
        }

        // Neither registered - equities race
        // Bona fide purchaser without notice may prevail
        if facts.second_bona_fide_without_notice {
            return PriorityWinner::Second;
        }

        // Otherwise, first in time (creation)
        if facts.first_created_earlier {
            PriorityWinner::First
        } else {
            PriorityWinner::Second
        }
    }

    /// Build reasoning
    fn build_reasoning(facts: &PriorityFacts, winner: &PriorityWinner) -> String {
        let mut parts = Vec::new();

        parts.push("Priority analysis between competing interests".to_string());

        if facts.first_registered && facts.second_registered {
            parts.push("Both interests registered - priority by order of registration".to_string());
        } else if facts.first_registered || facts.second_registered {
            parts.push("Registered interest prevails over unregistered".to_string());
        } else {
            parts.push("Neither registered - equity principles apply".to_string());
            if facts.second_bona_fide_without_notice {
                parts.push("Bona fide purchaser without notice may prevail".to_string());
            }
        }

        parts.push(format!("Winner: {:?}", winner));

        parts.join(". ")
    }
}

/// Facts for priority analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriorityFacts {
    /// First interest registered
    pub first_registered: bool,
    /// Second interest registered
    pub second_registered: bool,
    /// First registration earlier
    pub first_registration_earlier: bool,
    /// First created earlier
    pub first_created_earlier: bool,
    /// Second is bona fide purchaser without notice
    pub second_bona_fide_without_notice: bool,
}

/// Priority winner
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PriorityWinner {
    /// First interest prevails
    First,
    /// Second interest prevails
    Second,
}

/// Result of priority analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityResult {
    /// First interest prevails
    pub first_interest_prevails: bool,
    /// Second interest prevails
    pub second_interest_prevails: bool,
    /// Winner
    pub winner: PriorityWinner,
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
    fn test_indefeasibility_clean_title() {
        let facts = IndefeasibilityFacts {
            duly_registered: true,
            ..Default::default()
        };

        let result = IndefeasibilityAnalyzer::analyze(&facts);
        assert!(result.indefeasible);
        assert!(result.exceptions_apply.is_empty());
    }

    #[test]
    fn test_indefeasibility_fraud_exception() {
        let facts = IndefeasibilityFacts {
            duly_registered: true,
            fraud_by_registered_proprietor: true,
            ..Default::default()
        };

        let result = IndefeasibilityAnalyzer::analyze(&facts);
        assert!(!result.indefeasible);
        assert!(
            result
                .exceptions_apply
                .contains(&IndefeasibilityException::Fraud)
        );
    }

    #[test]
    fn test_priority_registered_wins() {
        let facts = PriorityFacts {
            first_registered: true,
            second_registered: false,
            ..Default::default()
        };

        let result = PriorityAnalyzer::analyze(&facts);
        assert!(result.first_interest_prevails);
    }

    #[test]
    fn test_priority_both_registered() {
        let facts = PriorityFacts {
            first_registered: true,
            second_registered: true,
            first_registration_earlier: true,
            ..Default::default()
        };

        let result = PriorityAnalyzer::analyze(&facts);
        assert_eq!(result.winner, PriorityWinner::First);
    }
}
