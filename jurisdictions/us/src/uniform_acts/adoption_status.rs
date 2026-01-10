//! Uniform Act Adoption Status and Comparison Utilities
//!
//! This module provides tools for comparing uniform act adoption across states
//! and analyzing patterns of adoption.

use super::{ucc::UCCArticle, upa::PartnershipActVersion};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Adoption status for a uniform act provision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AdoptionStatus {
    /// Fully adopted without significant variations
    FullyAdopted,

    /// Adopted with minor state-specific variations
    AdoptedWithVariations,

    /// Partially adopted (some provisions only)
    PartiallyAdopted,

    /// Not adopted
    NotAdopted,

    /// Custom state law instead of uniform act
    CustomLaw,
}

impl AdoptionStatus {
    /// Get human-readable description.
    #[must_use]
    pub fn description(&self) -> &'static str {
        match self {
            Self::FullyAdopted => "Fully Adopted",
            Self::AdoptedWithVariations => "Adopted with State Variations",
            Self::PartiallyAdopted => "Partially Adopted",
            Self::NotAdopted => "Not Adopted",
            Self::CustomLaw => "Custom State Law",
        }
    }

    /// Check if act is adopted in any form.
    #[must_use]
    pub fn is_adopted(&self) -> bool {
        matches!(
            self,
            Self::FullyAdopted | Self::AdoptedWithVariations | Self::PartiallyAdopted
        )
    }
}

/// Comparison result for uniform act adoption across states.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdoptionComparison {
    /// Act being compared
    pub act_name: String,

    /// Total number of jurisdictions analyzed
    pub total_jurisdictions: usize,

    /// Number of jurisdictions that adopted
    pub adopted_count: usize,

    /// Number of jurisdictions that did not adopt
    pub not_adopted_count: usize,

    /// Adoption percentage
    pub adoption_percentage: f64,

    /// States grouped by adoption status
    pub states_by_status: HashMap<AdoptionStatus, Vec<String>>,

    /// Notable variations summary
    pub notable_variations: Vec<String>,

    /// Majority version (if applicable)
    pub majority_version: Option<String>,
}

impl AdoptionComparison {
    /// Create new adoption comparison.
    #[must_use]
    pub fn new(act_name: impl Into<String>) -> Self {
        Self {
            act_name: act_name.into(),
            total_jurisdictions: 0,
            adopted_count: 0,
            not_adopted_count: 0,
            adoption_percentage: 0.0,
            states_by_status: HashMap::new(),
            notable_variations: vec![],
            majority_version: None,
        }
    }

    /// Add state to specific status category.
    pub fn add_state(&mut self, status: AdoptionStatus, state: impl Into<String>) {
        self.states_by_status
            .entry(status)
            .or_default()
            .push(state.into());

        self.total_jurisdictions += 1;

        if status.is_adopted() {
            self.adopted_count += 1;
        } else {
            self.not_adopted_count += 1;
        }

        self.recalculate_percentage();
    }

    /// Recalculate adoption percentage.
    fn recalculate_percentage(&mut self) {
        if self.total_jurisdictions > 0 {
            self.adoption_percentage =
                (self.adopted_count as f64 / self.total_jurisdictions as f64) * 100.0;
        }
    }

    /// Add notable variation.
    pub fn add_variation(&mut self, variation: impl Into<String>) {
        self.notable_variations.push(variation.into());
    }

    /// Set majority version.
    pub fn set_majority_version(&mut self, version: impl Into<String>) {
        self.majority_version = Some(version.into());
    }

    /// Get states with specific status.
    #[must_use]
    pub fn states_with_status(&self, status: AdoptionStatus) -> Vec<&str> {
        self.states_by_status
            .get(&status)
            .map(|states| states.iter().map(String::as_str).collect())
            .unwrap_or_default()
    }

    /// Generate summary report.
    #[must_use]
    pub fn summary(&self) -> String {
        let mut report = String::new();

        report.push_str(&format!("# {} Adoption Summary\n\n", self.act_name));
        report.push_str(&format!(
            "- **Total Jurisdictions**: {}\n",
            self.total_jurisdictions
        ));
        report.push_str(&format!("- **Adopted**: {}\n", self.adopted_count));
        report.push_str(&format!("- **Not Adopted**: {}\n", self.not_adopted_count));
        report.push_str(&format!(
            "- **Adoption Rate**: {:.1}%\n\n",
            self.adoption_percentage
        ));

        if let Some(version) = &self.majority_version {
            report.push_str(&format!("**Majority Version**: {version}\n\n"));
        }

        if !self.notable_variations.is_empty() {
            report.push_str("## Notable Variations\n\n");
            for variation in &self.notable_variations {
                report.push_str(&format!("- {variation}\n"));
            }
            report.push('\n');
        }

        for (status, states) in &self.states_by_status {
            if !states.is_empty() {
                report.push_str(&format!(
                    "## {} ({})\n\n",
                    status.description(),
                    states.len()
                ));
                report.push_str(&format!("{}\n\n", states.join(", ")));
            }
        }

        report
    }
}

/// Comparator for analyzing uniform act adoptions across states.
#[derive(Debug, Clone, Default)]
pub struct UniformActComparator {
    /// UCC comparisons by article
    ucc_comparisons: HashMap<UCCArticle, AdoptionComparison>,

    /// Partnership act comparison
    partnership_comparison: Option<AdoptionComparison>,
}

impl UniformActComparator {
    /// Create new uniform act comparator.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add UCC article comparison.
    pub fn add_ucc_comparison(&mut self, article: UCCArticle, comparison: AdoptionComparison) {
        self.ucc_comparisons.insert(article, comparison);
    }

    /// Get UCC article comparison.
    #[must_use]
    pub fn ucc_comparison(&self, article: UCCArticle) -> Option<&AdoptionComparison> {
        self.ucc_comparisons.get(&article)
    }

    /// Set partnership act comparison.
    pub fn set_partnership_comparison(&mut self, comparison: AdoptionComparison) {
        self.partnership_comparison = Some(comparison);
    }

    /// Get partnership act comparison.
    #[must_use]
    pub fn partnership_comparison(&self) -> Option<&AdoptionComparison> {
        self.partnership_comparison.as_ref()
    }

    /// Generate comprehensive comparison report.
    #[must_use]
    pub fn generate_report(&self) -> String {
        let mut report = String::new();

        report.push_str("# US Uniform Acts Adoption Report\n\n");

        if !self.ucc_comparisons.is_empty() {
            report.push_str("## Uniform Commercial Code (UCC)\n\n");

            for article in UCCArticle::all() {
                if let Some(comparison) = self.ucc_comparisons.get(&article) {
                    report.push_str(&format!("### {}\n\n", article.name()));
                    report.push_str(&comparison.summary());
                    report.push_str("\n---\n\n");
                }
            }
        }

        if let Some(partnership) = &self.partnership_comparison {
            report.push_str("## Partnership Acts (UPA/RUPA)\n\n");
            report.push_str(&partnership.summary());
        }

        report
    }

    /// Find states with inconsistent adoptions (e.g., adopted some UCC articles but not others).
    #[must_use]
    pub fn find_inconsistent_adoptions(&self) -> Vec<String> {
        let mut state_adoption_counts: HashMap<String, usize> = HashMap::new();
        let total_articles = self.ucc_comparisons.len();

        if total_articles == 0 {
            return vec![];
        }

        for comparison in self.ucc_comparisons.values() {
            for (status, states) in &comparison.states_by_status {
                if status.is_adopted() {
                    for state in states {
                        *state_adoption_counts.entry(state.clone()).or_insert(0) += 1;
                    }
                }
            }
        }

        // Find states that didn't adopt all articles
        state_adoption_counts
            .into_iter()
            .filter_map(|(state, count)| {
                if count < total_articles {
                    Some(state)
                } else {
                    None
                }
            })
            .collect()
    }
}

/// Helper function to create UCC Article 2 comparison from UCC tracker.
#[must_use]
pub fn create_ucc_article_comparison(
    article: UCCArticle,
    tracker: &super::ucc::UCCTracker,
) -> AdoptionComparison {
    let mut comparison = AdoptionComparison::new(article.name());

    let states_with = tracker.states_with_article(article);
    let states_without = tracker.states_without_article(article);

    for state in states_with {
        if let Some(adoption) = tracker.get_adoption(&state, article) {
            let status = if adoption.has_variations() {
                AdoptionStatus::AdoptedWithVariations
            } else {
                AdoptionStatus::FullyAdopted
            };
            comparison.add_state(status, state);
        }
    }

    for state in states_without {
        comparison.add_state(AdoptionStatus::NotAdopted, state);
    }

    comparison
}

/// Helper function to create partnership act comparison from UPA tracker.
#[must_use]
pub fn create_partnership_comparison(tracker: &super::upa::UPATracker) -> AdoptionComparison {
    let mut comparison = AdoptionComparison::new("Partnership Acts (UPA/RUPA)");

    let rupa_states = tracker.rupa_states();
    let upa_states = tracker.upa_states();

    for state in rupa_states {
        comparison.add_state(AdoptionStatus::FullyAdopted, state);
    }

    for state in upa_states {
        comparison.add_state(AdoptionStatus::AdoptedWithVariations, state);
    }

    // Handle Louisiana (custom)
    if let Some(la_adoption) = tracker.get_adoption("LA") {
        if la_adoption.version == PartnershipActVersion::Custom {
            comparison.add_state(AdoptionStatus::CustomLaw, "LA");
            comparison.add_variation("Louisiana uses Civil Code for partnership law");
        }
    }

    comparison.set_majority_version("RUPA (1997)");

    comparison
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adoption_status_description() {
        assert_eq!(AdoptionStatus::FullyAdopted.description(), "Fully Adopted");
        assert_eq!(
            AdoptionStatus::AdoptedWithVariations.description(),
            "Adopted with State Variations"
        );
        assert_eq!(AdoptionStatus::NotAdopted.description(), "Not Adopted");
    }

    #[test]
    fn test_adoption_status_is_adopted() {
        assert!(AdoptionStatus::FullyAdopted.is_adopted());
        assert!(AdoptionStatus::AdoptedWithVariations.is_adopted());
        assert!(AdoptionStatus::PartiallyAdopted.is_adopted());
        assert!(!AdoptionStatus::NotAdopted.is_adopted());
        assert!(!AdoptionStatus::CustomLaw.is_adopted());
    }

    #[test]
    fn test_adoption_comparison_creation() {
        let mut comparison = AdoptionComparison::new("UCC Article 2");

        comparison.add_state(AdoptionStatus::FullyAdopted, "CA");
        comparison.add_state(AdoptionStatus::FullyAdopted, "NY");
        comparison.add_state(AdoptionStatus::NotAdopted, "LA");

        assert_eq!(comparison.total_jurisdictions, 3);
        assert_eq!(comparison.adopted_count, 2);
        assert_eq!(comparison.not_adopted_count, 1);
        assert!((comparison.adoption_percentage - 66.67).abs() < 0.1);
    }

    #[test]
    fn test_adoption_comparison_states_with_status() {
        let mut comparison = AdoptionComparison::new("Test Act");

        comparison.add_state(AdoptionStatus::FullyAdopted, "CA");
        comparison.add_state(AdoptionStatus::FullyAdopted, "TX");
        comparison.add_state(AdoptionStatus::NotAdopted, "LA");

        let fully_adopted = comparison.states_with_status(AdoptionStatus::FullyAdopted);
        assert_eq!(fully_adopted.len(), 2);
        assert!(fully_adopted.contains(&"CA"));
        assert!(fully_adopted.contains(&"TX"));

        let not_adopted = comparison.states_with_status(AdoptionStatus::NotAdopted);
        assert_eq!(not_adopted.len(), 1);
        assert!(not_adopted.contains(&"LA"));
    }

    #[test]
    fn test_adoption_comparison_summary() {
        let mut comparison = AdoptionComparison::new("UCC Article 9");

        comparison.add_state(AdoptionStatus::FullyAdopted, "CA");
        comparison.add_state(AdoptionStatus::FullyAdopted, "NY");
        comparison.set_majority_version("Revised 2010");
        comparison.add_variation("California added special filing requirements");

        let summary = comparison.summary();

        assert!(summary.contains("UCC Article 9"));
        assert!(summary.contains("**Total Jurisdictions**: 2"));
        assert!(summary.contains("Revised 2010"));
        assert!(summary.contains("California added special filing requirements"));
    }

    #[test]
    fn test_uniform_act_comparator() {
        let mut comparator = UniformActComparator::new();

        let mut article2_comparison = AdoptionComparison::new("UCC Article 2");
        article2_comparison.add_state(AdoptionStatus::FullyAdopted, "CA");
        article2_comparison.add_state(AdoptionStatus::NotAdopted, "LA");

        comparator.add_ucc_comparison(UCCArticle::Article2, article2_comparison);

        let comparison = comparator.ucc_comparison(UCCArticle::Article2);
        assert!(comparison.is_some());

        let comparison = comparison.unwrap();
        assert_eq!(comparison.total_jurisdictions, 2);
    }

    #[test]
    fn test_uniform_act_comparator_report() {
        let mut comparator = UniformActComparator::new();

        let mut article2_comparison = AdoptionComparison::new("UCC Article 2");
        article2_comparison.add_state(AdoptionStatus::FullyAdopted, "CA");

        comparator.add_ucc_comparison(UCCArticle::Article2, article2_comparison);

        let report = comparator.generate_report();

        assert!(report.contains("US Uniform Acts Adoption Report"));
        assert!(report.contains("Uniform Commercial Code"));
        assert!(report.contains("UCC Article 2"));
    }

    #[test]
    fn test_create_ucc_article_comparison() {
        use super::super::ucc::UCCTracker;

        let tracker = UCCTracker::new();
        let comparison = create_ucc_article_comparison(UCCArticle::Article2, &tracker);

        // Article 2 should be adopted by 50 states (all except Louisiana)
        assert_eq!(comparison.adopted_count, 50);
        assert_eq!(comparison.not_adopted_count, 1);

        let not_adopted = comparison.states_with_status(AdoptionStatus::NotAdopted);
        assert_eq!(not_adopted.len(), 1);
        assert!(not_adopted.contains(&"LA"));
    }

    #[test]
    fn test_create_partnership_comparison() {
        use super::super::upa::UPATracker;

        let tracker = UPATracker::new();
        let comparison = create_partnership_comparison(&tracker);

        // Should have majority adoption
        assert!(comparison.adoption_percentage > 50.0);

        // Louisiana should be custom law
        let custom_law = comparison.states_with_status(AdoptionStatus::CustomLaw);
        assert!(custom_law.contains(&"LA"));

        // Should have RUPA as majority version
        assert_eq!(comparison.majority_version, Some("RUPA (1997)".to_string()));
    }

    #[test]
    fn test_find_inconsistent_adoptions() {
        let mut comparator = UniformActComparator::new();

        let mut article1_comparison = AdoptionComparison::new("Article 1");
        article1_comparison.add_state(AdoptionStatus::FullyAdopted, "CA");
        article1_comparison.add_state(AdoptionStatus::FullyAdopted, "LA");

        let mut article2_comparison = AdoptionComparison::new("Article 2");
        article2_comparison.add_state(AdoptionStatus::FullyAdopted, "CA");
        article2_comparison.add_state(AdoptionStatus::NotAdopted, "LA");

        comparator.add_ucc_comparison(UCCArticle::Article1, article1_comparison);
        comparator.add_ucc_comparison(UCCArticle::Article2, article2_comparison);

        let inconsistent = comparator.find_inconsistent_adoptions();

        // Louisiana should be flagged as inconsistent (Article 1 but not Article 2)
        assert!(inconsistent.contains(&"LA".to_string()));

        // California should NOT be flagged (adopted both)
        assert!(!inconsistent.contains(&"CA".to_string()));
    }
}
