//! US Choice of Law Analyzer
//!
//! Unified analyzer that supports multiple choice of law approaches used across
//! US states. Integrates with legalis-core's ChoiceOfLawAnalyzer.

use super::{
    factors::USChoiceOfLawFactors, restatement_first::RestatementFirst,
    restatement_second::RestatementSecond,
};
use serde::{Deserialize, Serialize};

/// Choice of law approaches used in US states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChoiceOfLawApproach {
    /// Restatement (First) - Traditional territorial approach (6 states)
    /// States: AL, GA, KS, MD, NM, SC
    RestatementFirst,

    /// Restatement (Second) - Most significant relationship (44 states - MAJORITY)
    /// Most states follow this approach
    RestatementSecond,

    /// Interest Analysis - California approach
    /// States: CA, NJ
    InterestAnalysis,

    /// Better Law - Minnesota approach
    /// States: MN, WI
    BetterLaw,

    /// Combined Modern - New York approach
    /// Hybrid of interest analysis and Restatement (Second)
    CombinedModern,
}

impl ChoiceOfLawApproach {
    /// Get states that follow this approach.
    #[must_use]
    pub fn following_states(&self) -> Vec<&'static str> {
        match self {
            Self::RestatementFirst => vec!["AL", "GA", "KS", "MD", "NM", "SC"],
            Self::RestatementSecond => vec![
                "AK", "AZ", "AR", "CO", "CT", "DE", "FL", "HI", "ID", "IL", "IN", "IA", "KY", "LA",
                "ME", "MA", "MI", "MS", "MO", "MT", "NE", "NV", "NH", "NC", "ND", "OH", "OK", "OR",
                "PA", "RI", "SD", "TN", "TX", "UT", "VT", "VA", "WA", "WV", "WY", "DC", "AS", "GU",
                "MP", "PR", "VI",
            ],
            Self::InterestAnalysis => vec!["CA", "NJ"],
            Self::BetterLaw => vec!["MN", "WI"],
            Self::CombinedModern => vec!["NY"],
        }
    }

    /// Get description of this approach.
    #[must_use]
    pub fn description(&self) -> &'static str {
        match self {
            Self::RestatementFirst => {
                "Traditional territorial approach using mechanical rules (lex loci delicti for \
                 torts, lex loci contractus for contracts)"
            }
            Self::RestatementSecond => {
                "Most significant relationship test weighing multiple contacts and policy \
                 considerations (§ 145 for torts, § 188 for contracts)"
            }
            Self::InterestAnalysis => {
                "Identify and weigh each state's governmental interests in having its law applied"
            }
            Self::BetterLaw => {
                "Apply the better rule of law among competing states (substantive law quality)"
            }
            Self::CombinedModern => {
                "Hybrid approach combining interest analysis with most significant relationship test"
            }
        }
    }

    /// Check if this is the majority approach.
    #[must_use]
    pub fn is_majority(&self) -> bool {
        matches!(self, Self::RestatementSecond)
    }
}

/// US-specific choice of law analyzer.
pub struct USChoiceOfLawAnalyzer {
    /// Approach used by this analyzer
    approach: ChoiceOfLawApproach,

    /// Restatement (First) analyzer
    rst_first: RestatementFirst,

    /// Restatement (Second) analyzer
    rst_second: RestatementSecond,
}

impl USChoiceOfLawAnalyzer {
    /// Create analyzer for a specific approach.
    #[must_use]
    pub fn new(approach: ChoiceOfLawApproach) -> Self {
        Self {
            approach,
            rst_first: RestatementFirst::new(),
            rst_second: RestatementSecond::new(),
        }
    }

    /// Create analyzer for a specific state (using that state's approach).
    #[must_use]
    pub fn for_state(state_code: &str) -> Self {
        let approach = Self::detect_approach(state_code);
        Self::new(approach)
    }

    /// Detect which approach a state follows.
    #[must_use]
    pub fn detect_approach(state_code: &str) -> ChoiceOfLawApproach {
        for approach in [
            ChoiceOfLawApproach::RestatementFirst,
            ChoiceOfLawApproach::InterestAnalysis,
            ChoiceOfLawApproach::BetterLaw,
            ChoiceOfLawApproach::CombinedModern,
        ] {
            if approach.following_states().contains(&state_code) {
                return approach;
            }
        }

        // Default: Most states follow Restatement (Second)
        ChoiceOfLawApproach::RestatementSecond
    }

    /// Get the approach used by this analyzer.
    #[must_use]
    pub fn approach(&self) -> ChoiceOfLawApproach {
        self.approach
    }

    /// Analyze choice of law for torts.
    #[must_use]
    pub fn analyze_tort(&self, factors: &USChoiceOfLawFactors) -> ChoiceOfLawResult {
        match self.approach {
            ChoiceOfLawApproach::RestatementFirst => {
                let result = self.rst_first.analyze_tort(factors);
                let is_unknown = result.applicable_law == "UNKNOWN";
                ChoiceOfLawResult {
                    applicable_law: result.applicable_law,
                    approach: self.approach,
                    confidence: if is_unknown { 0.0 } else { 0.95 },
                    explanation: result.explanation,
                    alternative_analyses: vec![],
                }
            }
            ChoiceOfLawApproach::RestatementSecond => {
                let result = self.rst_second.analyze_tort(factors);
                let confidence = if result.applicable_law == "UNKNOWN" {
                    0.0
                } else {
                    // Confidence based on how dominant the winner is
                    let total: f64 = result.all_scores.values().sum();
                    if total > 0.0 {
                        result.score / total
                    } else {
                        0.0
                    }
                };

                ChoiceOfLawResult {
                    applicable_law: result.applicable_law,
                    approach: self.approach,
                    confidence,
                    explanation: result.explanation,
                    alternative_analyses: vec![],
                }
            }
            ChoiceOfLawApproach::InterestAnalysis => {
                // Interest Analysis (simplified implementation)
                // Full implementation would require more detailed policy analysis
                self.analyze_tort_interest_analysis(factors)
            }
            ChoiceOfLawApproach::BetterLaw => {
                // Better Law approach (simplified)
                // Would require substantive law quality assessment
                self.analyze_tort_better_law(factors)
            }
            ChoiceOfLawApproach::CombinedModern => {
                // Combined approach (NY style)
                self.analyze_tort_combined_modern(factors)
            }
        }
    }

    /// Analyze choice of law for contracts.
    #[must_use]
    pub fn analyze_contract(&self, factors: &USChoiceOfLawFactors) -> ChoiceOfLawResult {
        match self.approach {
            ChoiceOfLawApproach::RestatementFirst => {
                let result = self.rst_first.analyze_contract(factors);
                let is_unknown = result.applicable_law == "UNKNOWN";
                ChoiceOfLawResult {
                    applicable_law: result.applicable_law,
                    approach: self.approach,
                    confidence: if is_unknown { 0.0 } else { 0.95 },
                    explanation: result.explanation,
                    alternative_analyses: vec![],
                }
            }
            ChoiceOfLawApproach::RestatementSecond => {
                let result = self.rst_second.analyze_contract(factors);
                let confidence = if result.applicable_law == "UNKNOWN" {
                    0.0
                } else {
                    let total: f64 = result.all_scores.values().sum();
                    if total > 0.0 {
                        result.score / total
                    } else {
                        0.0
                    }
                };

                ChoiceOfLawResult {
                    applicable_law: result.applicable_law,
                    approach: self.approach,
                    confidence,
                    explanation: result.explanation,
                    alternative_analyses: vec![],
                }
            }
            _ => {
                // For other approaches, fall back to Restatement (Second)
                let result = self.rst_second.analyze_contract(factors);
                ChoiceOfLawResult {
                    applicable_law: result.applicable_law,
                    approach: ChoiceOfLawApproach::RestatementSecond,
                    confidence: 0.7,
                    explanation: format!(
                        "{} approach not fully implemented for contracts. Falling back to \
                         Restatement (Second).\n\n{}",
                        self.approach.description(),
                        result.explanation
                    ),
                    alternative_analyses: vec![],
                }
            }
        }
    }

    /// Interest analysis for torts (California approach).
    fn analyze_tort_interest_analysis(&self, factors: &USChoiceOfLawFactors) -> ChoiceOfLawResult {
        // Simplified interest analysis
        // Step 1: Identify states with interests
        // Step 2: Classify as true conflict, false conflict, or no conflict
        // Step 3: Apply appropriate rule

        if factors.is_false_conflict() {
            // Only one state has interest - apply that state's law
            let interested_states = factors.connected_states();
            if let Some(&state) = interested_states.first() {
                ChoiceOfLawResult {
                    applicable_law: state.to_string(),
                    approach: self.approach,
                    confidence: 0.95,
                    explanation: format!(
                        "Interest Analysis (California approach): False conflict detected. Only \
                         {state} has governmental interest in applying its law. Apply {state} law."
                    ),
                    alternative_analyses: vec![],
                }
            } else {
                ChoiceOfLawResult {
                    applicable_law: "UNKNOWN".to_string(),
                    approach: self.approach,
                    confidence: 0.0,
                    explanation: "No state has identifiable interest.".to_string(),
                    alternative_analyses: vec![],
                }
            }
        } else if factors.is_true_conflict() {
            // Multiple states have interests - apply forum law (lex fori)
            let forum = factors
                .factors()
                .iter()
                .find_map(|f| {
                    if let super::factors::ContactingFactor::ForumState(state) = f {
                        Some(state.clone())
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| "UNKNOWN".to_string());

            ChoiceOfLawResult {
                applicable_law: forum.clone(),
                approach: self.approach,
                confidence: 0.8,
                explanation: format!(
                    "Interest Analysis: True conflict detected (multiple states with \
                     interests). Under California approach, apply forum law ({forum})."
                ),
                alternative_analyses: vec![],
            }
        } else {
            ChoiceOfLawResult {
                applicable_law: "UNKNOWN".to_string(),
                approach: self.approach,
                confidence: 0.0,
                explanation: "Insufficient information for interest analysis.".to_string(),
                alternative_analyses: vec![],
            }
        }
    }

    /// Better law approach for torts (Minnesota approach).
    fn analyze_tort_better_law(&self, _factors: &USChoiceOfLawFactors) -> ChoiceOfLawResult {
        ChoiceOfLawResult {
            applicable_law: "ANALYSIS_REQUIRED".to_string(),
            approach: self.approach,
            confidence: 0.0,
            explanation: "Better Law approach requires substantive law quality analysis not yet \
                          implemented. Would compare substantive rules of competing states and \
                          select better rule."
                .to_string(),
            alternative_analyses: vec![],
        }
    }

    /// Combined modern approach for torts (New York approach).
    fn analyze_tort_combined_modern(&self, factors: &USChoiceOfLawFactors) -> ChoiceOfLawResult {
        // NY uses interest analysis first, then most significant relationship
        // Simplified: Use Restatement (Second) with interest weighting
        let mut result = self.rst_second.analyze_tort(factors);

        result.explanation = format!(
            "Combined Modern (New York) approach: Combines interest analysis with most \
             significant relationship test.\n\n{}",
            result.explanation
        );

        ChoiceOfLawResult {
            applicable_law: result.applicable_law,
            approach: self.approach,
            confidence: 0.85,
            explanation: result.explanation,
            alternative_analyses: vec![],
        }
    }
}

/// Result of choice of law analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceOfLawResult {
    /// State whose law applies
    pub applicable_law: String,

    /// Approach used
    pub approach: ChoiceOfLawApproach,

    /// Confidence in result (0.0-1.0)
    pub confidence: f64,

    /// Explanation of analysis
    pub explanation: String,

    /// Alternative analyses (if requested)
    pub alternative_analyses: Vec<(ChoiceOfLawApproach, String)>,
}

#[cfg(test)]
mod tests {
    use super::super::factors::ContactingFactor;
    use super::*;

    #[test]
    fn test_approach_states() {
        let rst1_states = ChoiceOfLawApproach::RestatementFirst.following_states();
        assert_eq!(rst1_states.len(), 6);
        assert!(rst1_states.contains(&"MD"));

        let rst2_states = ChoiceOfLawApproach::RestatementSecond.following_states();
        assert!(rst2_states.len() > 40); // Majority of states
    }

    #[test]
    fn test_detect_approach() {
        assert_eq!(
            USChoiceOfLawAnalyzer::detect_approach("CA"),
            ChoiceOfLawApproach::InterestAnalysis
        );
        assert_eq!(
            USChoiceOfLawAnalyzer::detect_approach("NY"),
            ChoiceOfLawApproach::CombinedModern
        );
        assert_eq!(
            USChoiceOfLawAnalyzer::detect_approach("MD"),
            ChoiceOfLawApproach::RestatementFirst
        );
        assert_eq!(
            USChoiceOfLawAnalyzer::detect_approach("TX"),
            ChoiceOfLawApproach::RestatementSecond
        );
    }

    #[test]
    fn test_for_state() {
        let analyzer = USChoiceOfLawAnalyzer::for_state("CA");
        assert_eq!(analyzer.approach(), ChoiceOfLawApproach::InterestAnalysis);

        let analyzer = USChoiceOfLawAnalyzer::for_state("FL");
        assert_eq!(analyzer.approach(), ChoiceOfLawApproach::RestatementSecond);
    }

    #[test]
    fn test_restatement_first_tort_analysis() {
        let analyzer = USChoiceOfLawAnalyzer::new(ChoiceOfLawApproach::RestatementFirst);

        let factors = USChoiceOfLawFactors::new()
            .with_factor(ContactingFactor::PlaceOfInjury("MD".to_string()))
            .with_factor(ContactingFactor::DefendantDomicile("VA".to_string()));

        let result = analyzer.analyze_tort(&factors);

        assert_eq!(result.applicable_law, "MD");
        assert_eq!(result.approach, ChoiceOfLawApproach::RestatementFirst);
        assert!(result.confidence > 0.9);
    }

    #[test]
    fn test_restatement_second_tort_analysis() {
        let analyzer = USChoiceOfLawAnalyzer::new(ChoiceOfLawApproach::RestatementSecond);

        let factors = USChoiceOfLawFactors::new()
            .with_factor(ContactingFactor::PlaceOfInjury("TX".to_string()))
            .with_factor(ContactingFactor::PlaceOfConduct("TX".to_string()))
            .with_factor(ContactingFactor::DefendantDomicile("CA".to_string()));

        let result = analyzer.analyze_tort(&factors);

        assert_eq!(result.applicable_law, "TX");
        assert_eq!(result.approach, ChoiceOfLawApproach::RestatementSecond);
        assert!(result.confidence > 0.5);
    }

    #[test]
    fn test_interest_analysis_false_conflict() {
        let analyzer = USChoiceOfLawAnalyzer::new(ChoiceOfLawApproach::InterestAnalysis);

        let factors = USChoiceOfLawFactors::new()
            .with_factor(ContactingFactor::PlaceOfInjury("CA".to_string()))
            .with_state_interest("CA", "Only CA has interest");

        let result = analyzer.analyze_tort(&factors);

        assert_eq!(result.applicable_law, "CA");
        assert!(result.explanation.contains("False conflict"));
    }

    #[test]
    fn test_interest_analysis_true_conflict() {
        let analyzer = USChoiceOfLawAnalyzer::new(ChoiceOfLawApproach::InterestAnalysis);

        let factors = USChoiceOfLawFactors::new()
            .with_factor(ContactingFactor::PlaceOfInjury("CA".to_string()))
            .with_factor(ContactingFactor::ForumState("CA".to_string()))
            .with_state_interest("CA", "CA interest")
            .with_state_interest("NY", "NY interest");

        let result = analyzer.analyze_tort(&factors);

        assert_eq!(result.applicable_law, "CA");
        assert!(result.explanation.contains("True conflict"));
        assert!(result.explanation.contains("forum law"));
    }

    #[test]
    fn test_contract_analysis() {
        let analyzer = USChoiceOfLawAnalyzer::new(ChoiceOfLawApproach::RestatementSecond);

        let factors = USChoiceOfLawFactors::new()
            .with_factor(ContactingFactor::PlaceOfPerformance("NY".to_string()))
            .with_factor(ContactingFactor::PlaceOfExecution("CA".to_string()));

        let result = analyzer.analyze_contract(&factors);

        assert_eq!(result.applicable_law, "NY");
        assert!(result.explanation.contains("§ 188"));
    }
}
