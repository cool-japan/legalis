//! Uniform Partnership Act (UPA) / Revised Uniform Partnership Act (RUPA) Tracker
//!
//! Tracks which states have adopted UPA (1914) vs RUPA (1997) for governing
//! partnerships.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Partnership act version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PartnershipActVersion {
    /// Original Uniform Partnership Act (1914)
    UPA1914,

    /// Revised Uniform Partnership Act (1997)
    RUPA1997,

    /// State-specific custom partnership law
    Custom,

    /// No uniform act adopted
    None,
}

impl PartnershipActVersion {
    /// Get human-readable name.
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Self::UPA1914 => "Uniform Partnership Act (1914)",
            Self::RUPA1997 => "Revised Uniform Partnership Act (1997)",
            Self::Custom => "State-Specific Partnership Law",
            Self::None => "No Uniform Act Adopted",
        }
    }

    /// Check if this is a modern version (RUPA).
    #[must_use]
    pub fn is_modern(&self) -> bool {
        matches!(self, Self::RUPA1997)
    }

    /// Get year.
    #[must_use]
    pub fn year(&self) -> Option<u16> {
        match self {
            Self::UPA1914 => Some(1914),
            Self::RUPA1997 => Some(1997),
            Self::Custom | Self::None => None,
        }
    }
}

/// State's adoption status for partnership act.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UPAAdoption {
    /// State code
    pub state: String,

    /// Version adopted
    pub version: PartnershipActVersion,

    /// State-specific variations
    pub variations: Vec<String>,

    /// Effective date
    pub effective_date: Option<chrono::NaiveDate>,

    /// Citation to state statute
    pub citation: Option<String>,

    /// Notes about state's partnership law
    pub notes: Option<String>,
}

impl UPAAdoption {
    /// Create new UPA adoption record.
    #[must_use]
    pub fn new(state: impl Into<String>, version: PartnershipActVersion) -> Self {
        Self {
            state: state.into(),
            version,
            variations: vec![],
            effective_date: None,
            citation: None,
            notes: None,
        }
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

    /// Set notes.
    #[must_use]
    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    /// Check if has state variations.
    #[must_use]
    pub fn has_variations(&self) -> bool {
        !self.variations.is_empty()
    }
}

/// UPA/RUPA adoption tracker for all 50 states.
#[derive(Debug, Clone, Default)]
pub struct UPATracker {
    /// Adoptions by state
    adoptions: HashMap<String, UPAAdoption>,
}

impl UPATracker {
    /// Create new UPA tracker with default adoptions.
    #[must_use]
    pub fn new() -> Self {
        let mut tracker = Self {
            adoptions: HashMap::new(),
        };

        tracker.initialize_adoptions();
        tracker
    }

    /// Initialize partnership act adoptions for all states.
    fn initialize_adoptions(&mut self) {
        // States that adopted RUPA (1997) - majority as of 2024
        let rupa_states = vec![
            "AL", "AK", "AZ", "AR", "CA", "CO", "CT", "DE", "FL", "HI", "ID", "IL", "IA", "KS",
            "KY", "ME", "MD", "MN", "MS", "MT", "NE", "NV", "NJ", "NM", "ND", "OH", "OK", "OR",
            "SD", "TN", "TX", "UT", "VT", "VA", "WA", "WV", "WY", "DC",
        ];

        for state in rupa_states {
            self.adoptions.insert(
                state.to_string(),
                UPAAdoption::new(state, PartnershipActVersion::RUPA1997),
            );
        }

        // States still using UPA (1914) or custom law
        self.adoptions.insert(
            "GA".to_string(),
            UPAAdoption::new("GA", PartnershipActVersion::UPA1914),
        );

        self.adoptions.insert(
            "LA".to_string(),
            UPAAdoption::new("LA", PartnershipActVersion::Custom)
                .with_notes("Louisiana Civil Code governs partnerships"),
        );

        self.adoptions.insert(
            "MA".to_string(),
            UPAAdoption::new("MA", PartnershipActVersion::UPA1914),
        );

        self.adoptions.insert(
            "MI".to_string(),
            UPAAdoption::new("MI", PartnershipActVersion::UPA1914),
        );

        self.adoptions.insert(
            "MO".to_string(),
            UPAAdoption::new("MO", PartnershipActVersion::UPA1914),
        );

        self.adoptions.insert(
            "NH".to_string(),
            UPAAdoption::new("NH", PartnershipActVersion::UPA1914),
        );

        self.adoptions.insert(
            "NY".to_string(),
            UPAAdoption::new("NY", PartnershipActVersion::UPA1914)
                .with_citation("N.Y. Partnership Law"),
        );

        self.adoptions.insert(
            "NC".to_string(),
            UPAAdoption::new("NC", PartnershipActVersion::UPA1914),
        );

        self.adoptions.insert(
            "PA".to_string(),
            UPAAdoption::new("PA", PartnershipActVersion::UPA1914),
        );

        self.adoptions.insert(
            "RI".to_string(),
            UPAAdoption::new("RI", PartnershipActVersion::UPA1914),
        );

        self.adoptions.insert(
            "SC".to_string(),
            UPAAdoption::new("SC", PartnershipActVersion::UPA1914),
        );

        self.adoptions.insert(
            "WI".to_string(),
            UPAAdoption::new("WI", PartnershipActVersion::UPA1914),
        );

        self.adoptions.insert(
            "IN".to_string(),
            UPAAdoption::new("IN", PartnershipActVersion::UPA1914),
        );
    }

    /// Get adoption status for a state.
    #[must_use]
    pub fn get_adoption(&self, state: &str) -> Option<&UPAAdoption> {
        self.adoptions.get(state)
    }

    /// Get version a state uses.
    #[must_use]
    pub fn state_version(&self, state: &str) -> Option<PartnershipActVersion> {
        self.get_adoption(state).map(|adoption| adoption.version)
    }

    /// Check if state has adopted RUPA (modern version).
    #[must_use]
    pub fn has_rupa(&self, state: &str) -> bool {
        self.state_version(state) == Some(PartnershipActVersion::RUPA1997)
    }

    /// Check if state still uses UPA (1914).
    #[must_use]
    pub fn has_upa(&self, state: &str) -> bool {
        self.state_version(state) == Some(PartnershipActVersion::UPA1914)
    }

    /// Get all states with RUPA.
    #[must_use]
    pub fn rupa_states(&self) -> Vec<String> {
        self.adoptions
            .iter()
            .filter_map(|(state, adoption)| {
                if adoption.version == PartnershipActVersion::RUPA1997 {
                    Some(state.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get all states with UPA (1914).
    #[must_use]
    pub fn upa_states(&self) -> Vec<String> {
        self.adoptions
            .iter()
            .filter_map(|(state, adoption)| {
                if adoption.version == PartnershipActVersion::UPA1914 {
                    Some(state.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get adoption percentage for RUPA.
    #[must_use]
    pub fn rupa_adoption_percentage(&self) -> f64 {
        let total = self.adoptions.len() as f64;
        if total == 0.0 {
            return 0.0;
        }
        let rupa_count = self.rupa_states().len() as f64;
        (rupa_count / total) * 100.0
    }

    /// Add or update adoption record.
    pub fn add_adoption(&mut self, adoption: UPAAdoption) {
        let state = adoption.state.clone();
        self.adoptions.insert(state, adoption);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partnership_act_version_name() {
        assert_eq!(
            PartnershipActVersion::UPA1914.name(),
            "Uniform Partnership Act (1914)"
        );
        assert_eq!(
            PartnershipActVersion::RUPA1997.name(),
            "Revised Uniform Partnership Act (1997)"
        );
    }

    #[test]
    fn test_partnership_act_version_is_modern() {
        assert!(!PartnershipActVersion::UPA1914.is_modern());
        assert!(PartnershipActVersion::RUPA1997.is_modern());
        assert!(!PartnershipActVersion::Custom.is_modern());
    }

    #[test]
    fn test_partnership_act_version_year() {
        assert_eq!(PartnershipActVersion::UPA1914.year(), Some(1914));
        assert_eq!(PartnershipActVersion::RUPA1997.year(), Some(1997));
        assert_eq!(PartnershipActVersion::Custom.year(), None);
    }

    #[test]
    fn test_upa_tracker_initialization() {
        let tracker = UPATracker::new();

        // Should have all 51 jurisdictions
        assert_eq!(tracker.adoptions.len(), 51);
    }

    #[test]
    fn test_rupa_adoption() {
        let tracker = UPATracker::new();

        // California should have RUPA
        assert!(tracker.has_rupa("CA"));
        assert!(!tracker.has_upa("CA"));

        // Texas should have RUPA
        assert!(tracker.has_rupa("TX"));
    }

    #[test]
    fn test_upa_retention() {
        let tracker = UPATracker::new();

        // New York should still use UPA (1914)
        assert!(tracker.has_upa("NY"));
        assert!(!tracker.has_rupa("NY"));

        // Georgia should still use UPA (1914)
        assert!(tracker.has_upa("GA"));
    }

    #[test]
    fn test_louisiana_custom() {
        let tracker = UPATracker::new();

        let la_version = tracker.state_version("LA");
        assert_eq!(la_version, Some(PartnershipActVersion::Custom));

        let la_adoption = tracker.get_adoption("LA").unwrap();
        assert!(la_adoption.notes.is_some());
        assert!(la_adoption.notes.as_ref().unwrap().contains("Civil Code"));
    }

    #[test]
    fn test_rupa_states_list() {
        let tracker = UPATracker::new();

        let rupa_states = tracker.rupa_states();

        // Should include California
        assert!(rupa_states.contains(&"CA".to_string()));

        // Should include Texas
        assert!(rupa_states.contains(&"TX".to_string()));

        // Should NOT include New York (UPA)
        assert!(!rupa_states.contains(&"NY".to_string()));

        // Should NOT include Louisiana (Custom)
        assert!(!rupa_states.contains(&"LA".to_string()));
    }

    #[test]
    fn test_upa_states_list() {
        let tracker = UPATracker::new();

        let upa_states = tracker.upa_states();

        // Should include New York
        assert!(upa_states.contains(&"NY".to_string()));

        // Should include Georgia
        assert!(upa_states.contains(&"GA".to_string()));

        // Should NOT include California (RUPA)
        assert!(!upa_states.contains(&"CA".to_string()));
    }

    #[test]
    fn test_rupa_adoption_percentage() {
        let tracker = UPATracker::new();

        let percentage = tracker.rupa_adoption_percentage();

        // RUPA should be majority (>50%)
        assert!(percentage > 50.0);

        // Should be less than 100% (some states still use UPA)
        assert!(percentage < 100.0);
    }

    #[test]
    fn test_upa_adoption_builder() {
        let adoption = UPAAdoption::new("FL", PartnershipActVersion::RUPA1997)
            .with_citation("Fla. Stat. § 620.81 et seq.")
            .with_variation("Florida modified § 404 regarding partnership property")
            .with_notes("Adopted 1995, effective 1996");

        assert_eq!(adoption.state, "FL");
        assert_eq!(adoption.version, PartnershipActVersion::RUPA1997);
        assert!(adoption.has_variations());
        assert!(adoption.citation.is_some());
        assert!(adoption.notes.is_some());
    }

    #[test]
    fn test_add_custom_adoption() {
        let mut tracker = UPATracker::new();

        let custom_adoption = UPAAdoption::new("AZ", PartnershipActVersion::RUPA1997)
            .with_variation("Arizona added special provisions for LLPs");

        tracker.add_adoption(custom_adoption);

        let az_adoption = tracker.get_adoption("AZ").unwrap();
        assert!(az_adoption.has_variations());
    }
}
