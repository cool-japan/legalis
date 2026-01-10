//! State Law Comparison Engine
//!
//! This module provides functionality for comparing legal rules across US states,
//! identifying majority/minority approaches, and generating comparative reports.
//!
//! ## Core Functionality
//! - Compare specific legal topics across multiple states
//! - Identify majority and minority rules
//! - Generate similarity matrices between states
//! - Produce markdown comparison reports
//!
//! ## Integration
//! Uses `legalis-core::StatuteHarmonizer` for similarity scoring between state statutes.

use super::{
    california::CaliforniaLaw,
    florida::FloridaLaw,
    new_york::NewYorkLaw,
    registry::StateRegistry,
    texas::TexasLaw,
    types::{LegalTopic, StateId, StateLawVariation, StateRule},
};
use std::collections::HashMap;

/// State law comparison engine.
///
/// Compares legal rules across US states to identify variations, majority/minority
/// approaches, and similarities.
pub struct StateLawComparator {
    /// State registry for metadata
    #[allow(dead_code)] // Reserved for future use
    registry: StateRegistry,

    /// Cached state variations by topic
    variations_cache: HashMap<LegalTopic, Vec<StateLawVariation>>,
}

impl Default for StateLawComparator {
    fn default() -> Self {
        Self::new()
    }
}

impl StateLawComparator {
    /// Create a new state law comparator.
    #[must_use]
    pub fn new() -> Self {
        let mut comparator = Self {
            registry: StateRegistry::new(),
            variations_cache: HashMap::new(),
        };

        // Pre-populate cache with Phase 1 states
        comparator.load_phase_1_variations();

        comparator
    }

    /// Load variations from Phase 1 states into cache.
    fn load_phase_1_variations(&mut self) {
        // Load comparative negligence variations
        let mut comp_neg_variations = Vec::new();
        comp_neg_variations.extend(CaliforniaLaw::state_variations());
        comp_neg_variations.extend(NewYorkLaw::state_variations());
        comp_neg_variations.extend(TexasLaw::state_variations());
        comp_neg_variations.extend(FloridaLaw::state_variations());
        // Louisiana doesn't fit standard comparative negligence categories

        self.variations_cache
            .insert(LegalTopic::ComparativeNegligence, comp_neg_variations);

        // Load joint and several liability variations
        let mut joint_variations = Vec::new();
        joint_variations.extend(
            TexasLaw::state_variations()
                .into_iter()
                .filter(|v| v.topic == LegalTopic::JointAndSeveralLiability),
        );

        self.variations_cache
            .insert(LegalTopic::JointAndSeveralLiability, joint_variations);
    }

    /// Compare a legal topic across multiple states.
    ///
    /// # Arguments
    /// * `topic` - Legal topic to compare
    /// * `states` - State codes to include in comparison (e.g., &["CA", "NY", "TX"])
    ///
    /// # Returns
    /// `StateComparison` with majority/minority rules and state-by-state breakdown
    #[must_use]
    pub fn compare_states(&self, topic: LegalTopic, states: &[&str]) -> StateComparison {
        let variations = self.get_variations_for_states(topic, states);

        // Count rule frequencies
        let mut rule_counts: HashMap<String, usize> = HashMap::new();
        for variation in &variations {
            let rule_key = format!("{:?}", variation.rule);
            *rule_counts.entry(rule_key).or_insert(0) += 1;
        }

        // Determine majority rule (most common)
        let majority_rule =
            rule_counts
                .iter()
                .max_by_key(|(_, count)| *count)
                .and_then(|(rule_str, _)| {
                    variations
                        .iter()
                        .find(|v| format!("{:?}", v.rule) == *rule_str)
                        .map(|v| v.rule.clone())
                });

        // Determine minority rules (less common)
        let mut minority_rules: Vec<StateRule> = variations
            .iter()
            .map(|v| v.rule.clone())
            .filter(|rule| Some(rule) != majority_rule.as_ref())
            .collect();

        // Deduplicate minority rules manually (since StateRule may not be Hash due to Custom variant)
        minority_rules.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));
        minority_rules.dedup();

        // Build state-by-state map
        let by_state: HashMap<StateId, StateLawVariation> = variations
            .into_iter()
            .map(|v| (v.state.clone(), v))
            .collect();

        // Build similarity matrix (simplified for Phase 1)
        let similarity_matrix = self.build_similarity_matrix(states, &by_state);

        StateComparison {
            topic,
            majority_rule,
            minority_rules,
            by_state,
            similarity_matrix,
            total_states: states.len(),
        }
    }

    /// Get variations for specific states on a topic.
    fn get_variations_for_states(
        &self,
        topic: LegalTopic,
        states: &[&str],
    ) -> Vec<StateLawVariation> {
        self.variations_cache
            .get(&topic)
            .map(|variations| {
                variations
                    .iter()
                    .filter(|v| states.contains(&v.state.code.as_str()))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Build similarity matrix between states.
    ///
    /// Similarity is 1.0 if states have identical rules, 0.0 if completely different.
    fn build_similarity_matrix(
        &self,
        states: &[&str],
        by_state: &HashMap<StateId, StateLawVariation>,
    ) -> Vec<Vec<f64>> {
        let n = states.len();
        let mut matrix = vec![vec![0.0; n]; n];

        // Build list of state variations in same order as states array
        let state_variations: Vec<Option<&StateLawVariation>> = states
            .iter()
            .map(|code| by_state.values().find(|v| v.state.code == *code))
            .collect();

        for i in 0..n {
            for j in 0..n {
                if i == j {
                    matrix[i][j] = 1.0; // State compared to itself = 100% similar
                } else {
                    matrix[i][j] = match (&state_variations[i], &state_variations[j]) {
                        (Some(vi), Some(vj)) if vi.rule == vj.rule => 1.0,
                        (Some(_), Some(_)) => 0.5, // Different rules but both have rules
                        _ => 0.0,                  // One or both missing
                    };
                }
            }
        }

        matrix
    }

    /// Find states that follow a specific rule.
    #[must_use]
    pub fn states_with_rule(&self, topic: LegalTopic, rule: &StateRule) -> Vec<StateId> {
        self.variations_cache
            .get(&topic)
            .map(|variations| {
                variations
                    .iter()
                    .filter(|v| &v.rule == rule)
                    .map(|v| v.state.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Generate a markdown comparison report.
    #[must_use]
    pub fn generate_report(&self, comparison: &StateComparison) -> String {
        let mut report = String::new();

        report.push_str(&format!("# State Comparison: {}\n\n", comparison.topic));

        // Majority rule section
        if let Some(ref majority) = comparison.majority_rule {
            let majority_count = comparison
                .by_state
                .values()
                .filter(|v| &v.rule == majority)
                .count();

            report.push_str(&format!(
                "## Majority Rule ({}/{})\n\n",
                majority_count, comparison.total_states
            ));
            report.push_str(&format!("**{}**\n\n", majority));

            report.push_str("### States Following Majority:\n");
            for (state, var) in &comparison.by_state {
                if &var.rule == majority {
                    report.push_str(&format!("- {} ({})\n", state.name, state.code));
                }
            }
            report.push('\n');
        }

        // Minority rules section
        if !comparison.minority_rules.is_empty() {
            report.push_str("## Minority Rules\n\n");

            for rule in &comparison.minority_rules {
                let count = comparison
                    .by_state
                    .values()
                    .filter(|v| &v.rule == rule)
                    .count();

                report.push_str(&format!("### {} ({} states)\n\n", rule, count));

                report.push_str("States:\n");
                for (state, var) in &comparison.by_state {
                    if &var.rule == rule {
                        report.push_str(&format!("- {} ({})\n", state.name, state.code));
                    }
                }
                report.push('\n');
            }
        }

        // Detailed breakdown
        report.push_str("## State-by-State Details\n\n");
        for (state, variation) in &comparison.by_state {
            report.push_str(&format!("### {} ({})\n\n", state.name, state.code));
            report.push_str(&format!("**Rule**: {}\n\n", variation.rule));

            if let Some(ref statute) = variation.statutory_basis {
                report.push_str(&format!("**Statutory Basis**: {}\n\n", statute.citation));
            }

            if !variation.case_basis.is_empty() {
                report.push_str("**Case Law**:\n");
                for case in &variation.case_basis {
                    report.push_str(&format!("- {}\n", case));
                }
                report.push('\n');
            }

            if let Some(ref date) = variation.adoption_date {
                report.push_str(&format!("**Adopted**: {}\n\n", date));
            }
        }

        report
    }
}

/// Result of comparing a legal topic across states.
#[derive(Debug, Clone)]
pub struct StateComparison {
    /// Topic being compared
    pub topic: LegalTopic,

    /// Most common rule (if clear majority exists)
    pub majority_rule: Option<StateRule>,

    /// Less common rules
    pub minority_rules: Vec<StateRule>,

    /// Variations by state
    pub by_state: HashMap<StateId, StateLawVariation>,

    /// Similarity matrix (similarity scores between all state pairs, 0.0-1.0)
    ///
    /// Matrix\[i\]\[j\] represents similarity between state i and state j.
    pub similarity_matrix: Vec<Vec<f64>>,

    /// Total number of states in comparison
    pub total_states: usize,
}

impl StateComparison {
    /// Get states following the majority rule.
    #[must_use]
    pub fn majority_states(&self) -> Vec<&StateId> {
        if let Some(ref majority) = self.majority_rule {
            self.by_state
                .iter()
                .filter(|(_, v)| &v.rule == majority)
                .map(|(state, _)| state)
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get states following minority rules.
    #[must_use]
    pub fn minority_states(&self) -> Vec<&StateId> {
        self.by_state
            .iter()
            .filter(|(_, v)| Some(&v.rule) != self.majority_rule.as_ref())
            .map(|(state, _)| state)
            .collect()
    }

    /// Check if a specific state was included in the comparison.
    #[must_use]
    pub fn includes_state(&self, state_code: &str) -> bool {
        self.by_state.keys().any(|s| s.code == state_code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comparator_creation() {
        let comparator = StateLawComparator::new();

        // Should have Phase 1 states in registry
        assert!(comparator.registry.get("CA").is_some());
        assert!(comparator.registry.get("NY").is_some());
        assert!(comparator.registry.get("TX").is_some());
        assert!(comparator.registry.get("LA").is_some());
        assert!(comparator.registry.get("FL").is_some());
    }

    #[test]
    fn test_comparative_negligence_comparison() {
        let comparator = StateLawComparator::new();

        let comparison =
            comparator.compare_states(LegalTopic::ComparativeNegligence, &["CA", "NY", "TX", "FL"]);

        assert_eq!(comparison.topic, LegalTopic::ComparativeNegligence);
        assert!(comparison.majority_rule.is_some());

        // Should have 4 states in comparison
        assert_eq!(comparison.by_state.len(), 4);
        assert!(comparison.includes_state("CA"));
        assert!(comparison.includes_state("NY"));
        assert!(comparison.includes_state("TX"));
        assert!(comparison.includes_state("FL"));
    }

    #[test]
    fn test_majority_pure_comparative() {
        let comparator = StateLawComparator::new();

        let comparison =
            comparator.compare_states(LegalTopic::ComparativeNegligence, &["CA", "NY", "FL"]);

        // CA, NY, FL all have pure comparative negligence
        assert_eq!(
            comparison.majority_rule,
            Some(StateRule::PureComparativeNegligence)
        );
        assert_eq!(comparison.majority_states().len(), 3);
    }

    #[test]
    fn test_texas_as_minority() {
        let comparator = StateLawComparator::new();

        let comparison =
            comparator.compare_states(LegalTopic::ComparativeNegligence, &["CA", "NY", "TX", "FL"]);

        // TX has modified comparative (minority)
        let minority_states = comparison.minority_states();
        assert_eq!(minority_states.len(), 1);
        assert_eq!(minority_states[0].code, "TX");
    }

    #[test]
    fn test_states_with_rule() {
        let comparator = StateLawComparator::new();

        let pure_states = comparator.states_with_rule(
            LegalTopic::ComparativeNegligence,
            &StateRule::PureComparativeNegligence,
        );

        assert!(pure_states.len() >= 3); // CA, NY, FL
        assert!(pure_states.iter().any(|s| s.code == "CA"));
        assert!(pure_states.iter().any(|s| s.code == "NY"));
        assert!(pure_states.iter().any(|s| s.code == "FL"));

        let modified_states = comparator.states_with_rule(
            LegalTopic::ComparativeNegligence,
            &StateRule::ModifiedComparative51,
        );

        assert!(modified_states.iter().any(|s| s.code == "TX"));
    }

    #[test]
    fn test_similarity_matrix() {
        let comparator = StateLawComparator::new();

        let comparison =
            comparator.compare_states(LegalTopic::ComparativeNegligence, &["CA", "NY"]);

        // Similarity matrix should be 2x2
        assert_eq!(comparison.similarity_matrix.len(), 2);
        assert_eq!(comparison.similarity_matrix[0].len(), 2);

        // Diagonal should be 1.0 (state compared to itself)
        assert_eq!(comparison.similarity_matrix[0][0], 1.0);
        assert_eq!(comparison.similarity_matrix[1][1], 1.0);

        // CA and NY both have pure comparative, so should be 1.0 similarity
        assert_eq!(comparison.similarity_matrix[0][1], 1.0);
        assert_eq!(comparison.similarity_matrix[1][0], 1.0);
    }

    #[test]
    fn test_generate_report() {
        let comparator = StateLawComparator::new();

        let comparison =
            comparator.compare_states(LegalTopic::ComparativeNegligence, &["CA", "NY", "TX"]);

        let report = comparator.generate_report(&comparison);

        // Report should contain key sections
        assert!(report.contains("State Comparison"));
        assert!(report.contains("Majority Rule"));
        assert!(report.contains("State-by-State Details"));
        assert!(report.contains("California"));
        assert!(report.contains("New York"));
        assert!(report.contains("Texas"));
    }

    #[test]
    fn test_comparison_includes_state() {
        let comparator = StateLawComparator::new();

        let comparison =
            comparator.compare_states(LegalTopic::ComparativeNegligence, &["CA", "TX"]);

        assert!(comparison.includes_state("CA"));
        assert!(comparison.includes_state("TX"));
        assert!(!comparison.includes_state("NY"));
        assert!(!comparison.includes_state("FL"));
    }
}
