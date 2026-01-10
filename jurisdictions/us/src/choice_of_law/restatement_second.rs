//! Restatement (Second) of Conflict of Laws (1971)
//!
//! Modern choice of law approach adopted by 44 states. Uses "most significant
//! relationship" test that considers multiple factors and policy considerations.
//!
//! ## Key Principles
//!
//! ### § 6 - General Principles
//! Courts should consider:
//! (a) Needs of interstate and international systems
//! (b) Relevant policies of the forum
//! (c) Relevant policies of other interested states
//! (d) Protection of justified expectations
//! (e) Basic policies underlying particular field of law
//! (f) Certainty, predictability, and uniformity
//! (g) Ease in determination and application
//!
//! ### § 145 - Torts
//! Apply law of state with most significant relationship considering:
//! (a) Place of injury
//! (b) Place where conduct occurred
//! (c) Domicile, residence, nationality, place of incorporation, place of business
//! (d) Center of parties' relationship
//!
//! ### § 188 - Contracts
//! Apply law of state with most significant relationship considering:
//! (a) Place of contracting
//! (b) Place of negotiation
//! (c) Place of performance
//! (d) Location of subject matter
//! (e) Domicile, residence, nationality, place of incorporation, place of business

use super::factors::{ContactingFactor, USChoiceOfLawFactors};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Restatement (Second) choice of law analyzer.
#[derive(Debug, Clone)]
pub struct RestatementSecond {
    /// Section 6 factor weights (customizable per forum)
    factor_weights: HashMap<Section6Factor, f64>,
}

impl Default for RestatementSecond {
    fn default() -> Self {
        Self::new()
    }
}

impl RestatementSecond {
    /// Create new Restatement (Second) analyzer with default weights.
    #[must_use]
    pub fn new() -> Self {
        let mut weights = HashMap::new();

        // Default equal weights (can be customized per forum)
        weights.insert(Section6Factor::InterstateSystemNeeds, 1.0);
        weights.insert(Section6Factor::ForumPolicies, 1.0);
        weights.insert(Section6Factor::OtherStatePolicies, 1.0);
        weights.insert(Section6Factor::ProtectionOfExpectations, 1.0);
        weights.insert(Section6Factor::BasicPolicies, 1.0);
        weights.insert(Section6Factor::CertaintyAndUniformity, 1.0);
        weights.insert(Section6Factor::EaseOfApplication, 1.0);

        Self {
            factor_weights: weights,
        }
    }

    /// Customize weight for a Section 6 factor.
    #[must_use]
    pub fn with_factor_weight(mut self, factor: Section6Factor, weight: f64) -> Self {
        self.factor_weights.insert(factor, weight);
        self
    }

    /// Analyze choice of law for torts under Restatement (Second) § 145.
    #[must_use]
    pub fn analyze_tort(&self, factors: &USChoiceOfLawFactors) -> RestatementSecondResult {
        let mut state_scores: HashMap<String, f64> = HashMap::new();
        let mut explanations: Vec<String> = Vec::new();

        // § 145(2)(a) - Place of injury (usually most significant for personal injury)
        for factor in factors.factors() {
            if let ContactingFactor::PlaceOfInjury(state) = factor {
                *state_scores.entry(state.clone()).or_insert(0.0) += 3.0;
                explanations.push(format!(
                    "Place of injury ({state}): +3.0 (§ 145(2)(a) - typically most significant)"
                ));
            }
        }

        // § 145(2)(b) - Place of conduct
        for factor in factors.factors() {
            if let ContactingFactor::PlaceOfConduct(state) = factor {
                *state_scores.entry(state.clone()).or_insert(0.0) += 2.0;
                explanations.push(format!("Place of conduct ({state}): +2.0 (§ 145(2)(b))"));
            }
        }

        // § 145(2)(c) - Domicile and place of business
        for factor in factors.factors() {
            match factor {
                ContactingFactor::PlaintiffDomicile(state)
                | ContactingFactor::DefendantDomicile(state) => {
                    *state_scores.entry(state.clone()).or_insert(0.0) += 1.5;
                    explanations.push(format!("Party domicile ({state}): +1.5 (§ 145(2)(c))"));
                }
                ContactingFactor::PlaintiffBusinessLocation(state)
                | ContactingFactor::DefendantBusinessLocation(state) => {
                    *state_scores.entry(state.clone()).or_insert(0.0) += 1.5;
                    explanations.push(format!("Business location ({state}): +1.5 (§ 145(2)(c))"));
                }
                _ => {}
            }
        }

        // § 145(2)(d) - Center of relationship
        for factor in factors.factors() {
            if let ContactingFactor::CenterOfRelationship(state) = factor {
                *state_scores.entry(state.clone()).or_insert(0.0) += 2.5;
                explanations.push(format!(
                    "Center of relationship ({state}): +2.5 (§ 145(2)(d))"
                ));
            }
        }

        // Apply § 6 general principles (policy considerations)
        for (state, interests) in factors.all_state_interests() {
            if !interests.is_empty() {
                let policy_score = interests.len() as f64 * 1.0;
                *state_scores.entry(state.clone()).or_insert(0.0) += policy_score;
                explanations.push(format!(
                    "State interests ({state}): +{policy_score} (§ 6 - {} policy considerations)",
                    interests.len()
                ));
            }
        }

        // Determine state with most significant relationship
        let most_significant = state_scores
            .iter()
            .max_by(|(_, score_a), (_, score_b)| {
                score_a
                    .partial_cmp(score_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(state, score)| (state.clone(), *score));

        if let Some((state, score)) = most_significant {
            RestatementSecondResult {
                applicable_law: state.clone(),
                rule: RestatementSecondRule::MostSignificantRelationship,
                section: Section::Section145Torts,
                score,
                all_scores: state_scores,
                explanation: format!(
                    "Under Restatement (Second) § 145, apply law of state with most significant \
                     relationship. {state} has highest score ({score:.1}). Analysis:\n{}",
                    explanations.join("\n")
                ),
            }
        } else {
            RestatementSecondResult {
                applicable_law: "UNKNOWN".to_string(),
                rule: RestatementSecondRule::Indeterminate,
                section: Section::Section145Torts,
                score: 0.0,
                all_scores: HashMap::new(),
                explanation: "Insufficient contacts to determine most significant relationship."
                    .to_string(),
            }
        }
    }

    /// Analyze choice of law for contracts under Restatement (Second) § 188.
    #[must_use]
    pub fn analyze_contract(&self, factors: &USChoiceOfLawFactors) -> RestatementSecondResult {
        let mut state_scores: HashMap<String, f64> = HashMap::new();
        let mut explanations: Vec<String> = Vec::new();

        // § 188(2)(a) - Place of contracting
        for factor in factors.factors() {
            if let ContactingFactor::PlaceOfExecution(state) = factor {
                *state_scores.entry(state.clone()).or_insert(0.0) += 2.0;
                explanations.push(format!(
                    "Place of contracting ({state}): +2.0 (§ 188(2)(a))"
                ));
            }
        }

        // § 188(2)(b) - Place of negotiation
        for factor in factors.factors() {
            if let ContactingFactor::PlaceOfNegotiation(state) = factor {
                *state_scores.entry(state.clone()).or_insert(0.0) += 1.5;
                explanations.push(format!(
                    "Place of negotiation ({state}): +1.5 (§ 188(2)(b))"
                ));
            }
        }

        // § 188(2)(c) - Place of performance
        for factor in factors.factors() {
            if let ContactingFactor::PlaceOfPerformance(state) = factor {
                *state_scores.entry(state.clone()).or_insert(0.0) += 2.5;
                explanations.push(format!(
                    "Place of performance ({state}): +2.5 (§ 188(2)(c) - often most significant)"
                ));
            }
        }

        // § 188(2)(d) - Location of subject matter
        for factor in factors.factors() {
            if let ContactingFactor::LocationOfSubjectMatter(state) = factor {
                *state_scores.entry(state.clone()).or_insert(0.0) += 2.0;
                explanations.push(format!(
                    "Location of subject matter ({state}): +2.0 (§ 188(2)(d))"
                ));
            }
        }

        // § 188(2)(e) - Domicile and place of business
        for factor in factors.factors() {
            match factor {
                ContactingFactor::PlaintiffDomicile(state)
                | ContactingFactor::DefendantDomicile(state)
                | ContactingFactor::PlaintiffBusinessLocation(state)
                | ContactingFactor::DefendantBusinessLocation(state) => {
                    *state_scores.entry(state.clone()).or_insert(0.0) += 1.0;
                    explanations.push(format!(
                        "Party domicile/business ({state}): +1.0 (§ 188(2)(e))"
                    ));
                }
                _ => {}
            }
        }

        // § 6 general principles
        for (state, interests) in factors.all_state_interests() {
            if !interests.is_empty() {
                let policy_score = interests.len() as f64 * 1.0;
                *state_scores.entry(state.clone()).or_insert(0.0) += policy_score;
                explanations.push(format!(
                    "State interests ({state}): +{policy_score} (§ 6 considerations)"
                ));
            }
        }

        let most_significant = state_scores
            .iter()
            .max_by(|(_, score_a), (_, score_b)| {
                score_a
                    .partial_cmp(score_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(state, score)| (state.clone(), *score));

        if let Some((state, score)) = most_significant {
            RestatementSecondResult {
                applicable_law: state.clone(),
                rule: RestatementSecondRule::MostSignificantRelationship,
                section: Section::Section188Contracts,
                score,
                all_scores: state_scores,
                explanation: format!(
                    "Under Restatement (Second) § 188, apply law of state with most significant \
                     relationship. {state} has highest score ({score:.1}). Analysis:\n{}",
                    explanations.join("\n")
                ),
            }
        } else {
            RestatementSecondResult {
                applicable_law: "UNKNOWN".to_string(),
                rule: RestatementSecondRule::Indeterminate,
                section: Section::Section188Contracts,
                score: 0.0,
                all_scores: HashMap::new(),
                explanation: "Insufficient contacts to determine most significant relationship."
                    .to_string(),
            }
        }
    }
}

/// Section 6 general principles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Section6Factor {
    /// Needs of interstate and international systems
    InterstateSystemNeeds,

    /// Relevant policies of forum
    ForumPolicies,

    /// Relevant policies of other interested states
    OtherStatePolicies,

    /// Protection of justified expectations
    ProtectionOfExpectations,

    /// Basic policies underlying particular field of law
    BasicPolicies,

    /// Certainty, predictability, and uniformity
    CertaintyAndUniformity,

    /// Ease in determination and application
    EaseOfApplication,
}

/// Restatement (Second) sections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Section {
    /// § 145 - Torts
    Section145Torts,

    /// § 188 - Contracts
    Section188Contracts,
}

/// Restatement (Second) rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RestatementSecondRule {
    /// Most significant relationship test
    MostSignificantRelationship,

    /// Cannot determine
    Indeterminate,
}

/// Result of Restatement (Second) analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestatementSecondResult {
    /// State whose law applies
    pub applicable_law: String,

    /// Rule applied
    pub rule: RestatementSecondRule,

    /// Section applied
    pub section: Section,

    /// Score for chosen state
    pub score: f64,

    /// All state scores
    pub all_scores: HashMap<String, f64>,

    /// Detailed explanation
    pub explanation: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restatement_second_creation() {
        let rst = RestatementSecond::new();
        assert_eq!(rst.factor_weights.len(), 7);
    }

    #[test]
    fn test_tort_analysis_place_of_injury_dominant() {
        let rst = RestatementSecond::new();

        let factors = USChoiceOfLawFactors::new()
            .with_factor(ContactingFactor::PlaceOfInjury("CA".to_string()))
            .with_factor(ContactingFactor::DefendantDomicile("NY".to_string()));

        let result = rst.analyze_tort(&factors);

        assert_eq!(result.applicable_law, "CA");
        assert_eq!(
            result.rule,
            RestatementSecondRule::MostSignificantRelationship
        );
        assert_eq!(result.section, Section::Section145Torts);
        assert!(result.score > 0.0);
        assert!(result.explanation.contains("§ 145"));
    }

    #[test]
    fn test_tort_analysis_multiple_factors() {
        let rst = RestatementSecond::new();

        let factors = USChoiceOfLawFactors::new()
            .with_factor(ContactingFactor::PlaceOfInjury("TX".to_string()))
            .with_factor(ContactingFactor::PlaceOfConduct("TX".to_string()))
            .with_factor(ContactingFactor::DefendantDomicile("TX".to_string()))
            .with_factor(ContactingFactor::PlaintiffDomicile("CA".to_string()));

        let result = rst.analyze_tort(&factors);

        // TX should win with 3 contacts vs CA's 1
        assert_eq!(result.applicable_law, "TX");
        assert!(result.all_scores.get("TX").unwrap() > result.all_scores.get("CA").unwrap());
    }

    #[test]
    fn test_contract_analysis_place_of_performance() {
        let rst = RestatementSecond::new();

        let factors = USChoiceOfLawFactors::new()
            .with_factor(ContactingFactor::PlaceOfPerformance("NY".to_string()))
            .with_factor(ContactingFactor::PlaceOfExecution("FL".to_string()));

        let result = rst.analyze_contract(&factors);

        // NY should win (performance weighted higher: 2.5 vs 2.0)
        assert_eq!(result.applicable_law, "NY");
        assert_eq!(result.section, Section::Section188Contracts);
    }

    #[test]
    fn test_contract_analysis_multiple_contacts() {
        let rst = RestatementSecond::new();

        let factors = USChoiceOfLawFactors::new()
            .with_factor(ContactingFactor::PlaceOfNegotiation("CA".to_string()))
            .with_factor(ContactingFactor::PlaceOfExecution("CA".to_string()))
            .with_factor(ContactingFactor::PlaceOfPerformance("CA".to_string()))
            .with_factor(ContactingFactor::PlaintiffBusinessLocation(
                "NY".to_string(),
            ));

        let result = rst.analyze_contract(&factors);

        assert_eq!(result.applicable_law, "CA");
        assert!(result.all_scores.get("CA").unwrap() > &5.0);
    }

    #[test]
    fn test_policy_considerations() {
        let rst = RestatementSecond::new();

        let factors = USChoiceOfLawFactors::new()
            .with_factor(ContactingFactor::PlaceOfInjury("CA".to_string()))
            .with_state_interest("CA", "Protecting CA residents")
            .with_state_interest("CA", "Regulating conduct in CA");

        let result = rst.analyze_tort(&factors);

        assert_eq!(result.applicable_law, "CA");
        // Score should include both contact (3.0) and policy considerations (2.0)
        assert!(result.score >= 5.0);
    }

    #[test]
    fn test_indeterminate_result() {
        let rst = RestatementSecond::new();

        let factors = USChoiceOfLawFactors::new(); // No factors

        let result = rst.analyze_tort(&factors);

        assert_eq!(result.applicable_law, "UNKNOWN");
        assert_eq!(result.rule, RestatementSecondRule::Indeterminate);
    }

    #[test]
    fn test_custom_weights() {
        let rst = RestatementSecond::new().with_factor_weight(Section6Factor::ForumPolicies, 2.0);

        assert_eq!(
            *rst.factor_weights
                .get(&Section6Factor::ForumPolicies)
                .unwrap(),
            2.0
        );
    }
}
