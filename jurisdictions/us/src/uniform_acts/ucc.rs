//! Uniform Commercial Code (UCC) Adoption Tracker
//!
//! Tracks which UCC articles and versions each state has adopted,
//! including state-specific variations and amendments.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// UCC Article identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UCCArticle {
    /// Article 1: General Provisions
    Article1,
    /// Article 2: Sales of Goods
    Article2,
    /// Article 2A: Leases
    Article2A,
    /// Article 3: Negotiable Instruments
    Article3,
    /// Article 4: Bank Deposits and Collections
    Article4,
    /// Article 4A: Funds Transfers
    Article4A,
    /// Article 5: Letters of Credit
    Article5,
    /// Article 6: Bulk Transfers (mostly repealed)
    Article6,
    /// Article 7: Documents of Title
    Article7,
    /// Article 8: Investment Securities
    Article8,
    /// Article 9: Secured Transactions
    Article9,
}

impl UCCArticle {
    /// Get human-readable name.
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Self::Article1 => "Article 1: General Provisions",
            Self::Article2 => "Article 2: Sales of Goods",
            Self::Article2A => "Article 2A: Leases",
            Self::Article3 => "Article 3: Negotiable Instruments",
            Self::Article4 => "Article 4: Bank Deposits and Collections",
            Self::Article4A => "Article 4A: Funds Transfers",
            Self::Article5 => "Article 5: Letters of Credit",
            Self::Article6 => "Article 6: Bulk Transfers (mostly repealed)",
            Self::Article7 => "Article 7: Documents of Title",
            Self::Article8 => "Article 8: Investment Securities",
            Self::Article9 => "Article 9: Secured Transactions",
        }
    }

    /// Get article number.
    #[must_use]
    pub fn number(&self) -> u8 {
        match self {
            Self::Article1 => 1,
            Self::Article2 => 2,
            Self::Article2A => 20, // 2A represented as 20
            Self::Article3 => 3,
            Self::Article4 => 4,
            Self::Article4A => 40, // 4A represented as 40
            Self::Article5 => 5,
            Self::Article6 => 6,
            Self::Article7 => 7,
            Self::Article8 => 8,
            Self::Article9 => 9,
        }
    }

    /// Get all articles.
    #[must_use]
    pub fn all() -> Vec<Self> {
        vec![
            Self::Article1,
            Self::Article2,
            Self::Article2A,
            Self::Article3,
            Self::Article4,
            Self::Article4A,
            Self::Article5,
            Self::Article6,
            Self::Article7,
            Self::Article8,
            Self::Article9,
        ]
    }
}

/// UCC version identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UCCVersion {
    /// Original 1952 version
    Original1952,
    /// 1962 official text
    Official1962,
    /// 1972 amendments
    Amended1972,
    /// 1990 revision (Article 3 & 4)
    Revised1990,
    /// 2001 revision (Article 1)
    Revised2001,
    /// 2003 amendments
    Amended2003,
    /// 2010 revision (Article 9)
    Revised2010,
    /// State-specific custom version
    Custom { year: u16 },
}

impl UCCVersion {
    /// Get version year.
    #[must_use]
    pub fn year(&self) -> u16 {
        match self {
            Self::Original1952 => 1952,
            Self::Official1962 => 1962,
            Self::Amended1972 => 1972,
            Self::Revised1990 => 1990,
            Self::Revised2001 => 2001,
            Self::Amended2003 => 2003,
            Self::Revised2010 => 2010,
            Self::Custom { year } => *year,
        }
    }

    /// Check if this is a modern version (2001+).
    #[must_use]
    pub fn is_modern(&self) -> bool {
        self.year() >= 2001
    }
}

/// State's adoption status for a specific UCC article.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UCCAdoption {
    /// State code
    pub state: String,

    /// Article adopted
    pub article: UCCArticle,

    /// Version adopted
    pub version: UCCVersion,

    /// Whether state has adopted this article
    pub adopted: bool,

    /// State-specific variations/amendments
    pub variations: Vec<String>,

    /// Effective date
    pub effective_date: Option<chrono::NaiveDate>,

    /// Citation to state statute
    pub citation: Option<String>,
}

impl UCCAdoption {
    /// Create new adoption record.
    #[must_use]
    pub fn new(state: impl Into<String>, article: UCCArticle) -> Self {
        Self {
            state: state.into(),
            article,
            version: UCCVersion::Revised2001, // Default to 2001 revision
            adopted: true,
            variations: vec![],
            effective_date: None,
            citation: None,
        }
    }

    /// Set version.
    #[must_use]
    pub fn with_version(mut self, version: UCCVersion) -> Self {
        self.version = version;
        self
    }

    /// Set adoption status.
    #[must_use]
    pub fn with_adopted(mut self, adopted: bool) -> Self {
        self.adopted = adopted;
        self
    }

    /// Add a state variation.
    #[must_use]
    pub fn with_variation(mut self, variation: impl Into<String>) -> Self {
        self.variations.push(variation.into());
        self
    }

    /// Set effective date.
    #[must_use]
    pub fn with_effective_date(mut self, date: chrono::NaiveDate) -> Self {
        self.effective_date = Some(date);
        self
    }

    /// Set citation.
    #[must_use]
    pub fn with_citation(mut self, citation: impl Into<String>) -> Self {
        self.citation = Some(citation.into());
        self
    }

    /// Check if has state variations.
    #[must_use]
    pub fn has_variations(&self) -> bool {
        !self.variations.is_empty()
    }
}

/// UCC adoption tracker for all 50 states.
#[derive(Debug, Clone, Default)]
pub struct UCCTracker {
    /// Adoptions by state and article
    adoptions: HashMap<String, HashMap<UCCArticle, UCCAdoption>>,
}

impl UCCTracker {
    /// Create new UCC tracker with default adoptions.
    #[must_use]
    pub fn new() -> Self {
        let mut tracker = Self {
            adoptions: HashMap::new(),
        };

        // Initialize all 50 states with standard UCC adoption
        tracker.initialize_standard_adoptions();

        // Add Louisiana exception (no Article 2)
        tracker.add_louisiana_exception();

        tracker
    }

    /// Initialize standard UCC adoptions for all states.
    fn initialize_standard_adoptions(&mut self) {
        let states = vec![
            "AL", "AK", "AZ", "AR", "CA", "CO", "CT", "DE", "FL", "GA", "HI", "ID", "IL", "IN",
            "IA", "KS", "KY", "LA", "ME", "MD", "MA", "MI", "MN", "MS", "MO", "MT", "NE", "NV",
            "NH", "NJ", "NM", "NY", "NC", "ND", "OH", "OK", "OR", "PA", "RI", "SC", "SD", "TN",
            "TX", "UT", "VT", "VA", "WA", "WV", "WI", "WY", "DC",
        ];

        for state in states {
            let mut state_adoptions = HashMap::new();

            for article in UCCArticle::all() {
                let adoption = UCCAdoption::new(state, article);
                state_adoptions.insert(article, adoption);
            }

            self.adoptions.insert(state.to_string(), state_adoptions);
        }
    }

    /// Add Louisiana's special exception (Civil Law state - no Article 2).
    fn add_louisiana_exception(&mut self) {
        if let Some(la_adoptions) = self.adoptions.get_mut("LA") {
            // Louisiana did NOT adopt Article 2 (conflicts with Louisiana Civil Code)
            if let Some(article2) = la_adoptions.get_mut(&UCCArticle::Article2) {
                article2.adopted = false;
                article2.variations.push(
                    "Louisiana Civil Code governs sale of goods instead of UCC Article 2"
                        .to_string(),
                );
                article2.citation = Some("La. Civ. Code art. 2439 et seq.".to_string());
            }
        }
    }

    /// Check if a state has adopted a specific article.
    #[must_use]
    pub fn has_adopted(&self, state: &str, article: UCCArticle) -> bool {
        self.adoptions
            .get(state)
            .and_then(|state_adoptions| state_adoptions.get(&article))
            .is_some_and(|adoption| adoption.adopted)
    }

    /// Get adoption details for a state and article.
    #[must_use]
    pub fn get_adoption(&self, state: &str, article: UCCArticle) -> Option<&UCCAdoption> {
        self.adoptions
            .get(state)
            .and_then(|state_adoptions| state_adoptions.get(&article))
    }

    /// Get all adoptions for a state.
    #[must_use]
    pub fn state_adoptions(&self, state: &str) -> Option<&HashMap<UCCArticle, UCCAdoption>> {
        self.adoptions.get(state)
    }

    /// Get state variations for a specific article.
    #[must_use]
    pub fn state_variations(&self, state: &str, article: UCCArticle) -> Vec<String> {
        self.get_adoption(state, article)
            .map(|adoption| adoption.variations.clone())
            .unwrap_or_default()
    }

    /// Compare which states have adopted a specific article.
    #[must_use]
    pub fn states_with_article(&self, article: UCCArticle) -> Vec<String> {
        self.adoptions
            .iter()
            .filter_map(|(state, state_adoptions)| {
                state_adoptions
                    .get(&article)
                    .filter(|adoption| adoption.adopted)
                    .map(|_| state.clone())
            })
            .collect()
    }

    /// Get states that have NOT adopted a specific article.
    #[must_use]
    pub fn states_without_article(&self, article: UCCArticle) -> Vec<String> {
        self.adoptions
            .iter()
            .filter_map(|(state, state_adoptions)| {
                state_adoptions
                    .get(&article)
                    .filter(|adoption| !adoption.adopted)
                    .map(|_| state.clone())
            })
            .collect()
    }

    /// Get version a state uses for an article.
    #[must_use]
    pub fn state_version(&self, state: &str, article: UCCArticle) -> Option<UCCVersion> {
        self.get_adoption(state, article)
            .map(|adoption| adoption.version)
    }

    /// Add or update adoption record.
    pub fn add_adoption(&mut self, adoption: UCCAdoption) {
        let state = adoption.state.clone();
        let article = adoption.article;

        self.adoptions
            .entry(state)
            .or_default()
            .insert(article, adoption);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ucc_article_names() {
        assert_eq!(UCCArticle::Article1.name(), "Article 1: General Provisions");
        assert_eq!(UCCArticle::Article2.name(), "Article 2: Sales of Goods");
        assert_eq!(
            UCCArticle::Article9.name(),
            "Article 9: Secured Transactions"
        );
    }

    #[test]
    fn test_ucc_article_numbers() {
        assert_eq!(UCCArticle::Article1.number(), 1);
        assert_eq!(UCCArticle::Article2A.number(), 20);
        assert_eq!(UCCArticle::Article4A.number(), 40);
        assert_eq!(UCCArticle::Article9.number(), 9);
    }

    #[test]
    fn test_ucc_version_year() {
        assert_eq!(UCCVersion::Original1952.year(), 1952);
        assert_eq!(UCCVersion::Revised2001.year(), 2001);
        assert_eq!(UCCVersion::Custom { year: 2015 }.year(), 2015);
    }

    #[test]
    fn test_ucc_version_is_modern() {
        assert!(!UCCVersion::Amended1972.is_modern());
        assert!(UCCVersion::Revised2001.is_modern());
        assert!(UCCVersion::Revised2010.is_modern());
    }

    #[test]
    fn test_ucc_tracker_initialization() {
        let tracker = UCCTracker::new();

        // All 50 states + DC should be initialized
        assert_eq!(tracker.adoptions.len(), 51);

        // California should have all articles
        assert!(tracker.has_adopted("CA", UCCArticle::Article1));
        assert!(tracker.has_adopted("CA", UCCArticle::Article2));
        assert!(tracker.has_adopted("CA", UCCArticle::Article9));
    }

    #[test]
    fn test_louisiana_article_2_exception() {
        let tracker = UCCTracker::new();

        // Louisiana should have Article 1
        assert!(tracker.has_adopted("LA", UCCArticle::Article1));

        // Louisiana should NOT have Article 2
        assert!(!tracker.has_adopted("LA", UCCArticle::Article2));

        // Louisiana should have Article 9
        assert!(tracker.has_adopted("LA", UCCArticle::Article9));

        // Check variation explanation
        let variations = tracker.state_variations("LA", UCCArticle::Article2);
        assert!(!variations.is_empty());
        assert!(variations[0].contains("Louisiana Civil Code"));
    }

    #[test]
    fn test_states_with_article() {
        let tracker = UCCTracker::new();

        let article1_states = tracker.states_with_article(UCCArticle::Article1);
        assert_eq!(article1_states.len(), 51); // All states

        let article2_states = tracker.states_with_article(UCCArticle::Article2);
        assert_eq!(article2_states.len(), 50); // All except Louisiana
        assert!(!article2_states.contains(&"LA".to_string()));
    }

    #[test]
    fn test_states_without_article() {
        let tracker = UCCTracker::new();

        let without_article1 = tracker.states_without_article(UCCArticle::Article1);
        assert_eq!(without_article1.len(), 0);

        let without_article2 = tracker.states_without_article(UCCArticle::Article2);
        assert_eq!(without_article2.len(), 1);
        assert_eq!(without_article2[0], "LA");
    }

    #[test]
    fn test_ucc_adoption_builder() {
        let adoption = UCCAdoption::new("TX", UCCArticle::Article2)
            .with_version(UCCVersion::Revised2001)
            .with_variation("Texas amended § 2-201 (Statute of Frauds)")
            .with_citation("Tex. Bus. & Com. Code § 2.101 et seq.");

        assert_eq!(adoption.state, "TX");
        assert_eq!(adoption.article, UCCArticle::Article2);
        assert!(adoption.adopted);
        assert!(adoption.has_variations());
        assert!(adoption.citation.is_some());
    }

    #[test]
    fn test_add_custom_adoption() {
        let mut tracker = UCCTracker::new();

        let custom_adoption = UCCAdoption::new("NY", UCCArticle::Article9)
            .with_version(UCCVersion::Revised2010)
            .with_variation("NY added additional filing requirements");

        tracker.add_adoption(custom_adoption);

        let ny_article9 = tracker.get_adoption("NY", UCCArticle::Article9).unwrap();
        assert_eq!(ny_article9.version, UCCVersion::Revised2010);
        assert!(ny_article9.has_variations());
    }
}
