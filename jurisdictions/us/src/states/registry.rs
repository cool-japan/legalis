//! State registry with metadata for all US states.
//!
//! This module provides a centralized registry of US state metadata including
//! population, legal characteristics, court structure, and notable features.

use super::types::{LegalTradition, StateId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// State metadata - comprehensive information about a US state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateMetadata {
    /// State identifier
    pub id: StateId,

    /// State capital city
    pub capital: String,

    /// Estimated population (2023)
    pub population: u64,

    /// Court system structure
    pub court_structure: CourtStructure,

    /// Notable legal features or distinctions
    pub notable_features: Vec<String>,

    /// Year admitted to Union (statehood)
    pub statehood_year: u32,

    /// Geographic region
    pub region: GeographicRegion,
}

impl StateMetadata {
    /// Create new state metadata.
    #[must_use]
    pub fn new(
        id: StateId,
        capital: impl Into<String>,
        population: u64,
        court_structure: CourtStructure,
        statehood_year: u32,
        region: GeographicRegion,
    ) -> Self {
        Self {
            id,
            capital: capital.into(),
            population,
            court_structure,
            notable_features: Vec::new(),
            statehood_year,
            region,
        }
    }

    /// Add a notable feature.
    #[must_use]
    pub fn with_feature(mut self, feature: impl Into<String>) -> Self {
        self.notable_features.push(feature.into());
        self
    }
}

/// Court system structure for a state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourtStructure {
    /// Highest court name (usually "Supreme Court")
    pub highest_court: String,

    /// Intermediate appellate court name (if exists)
    pub appellate_court: Option<String>,

    /// Trial court name (general jurisdiction)
    pub trial_court: String,

    /// Specialized courts (e.g., probate, family, tax)
    pub specialized_courts: Vec<String>,
}

impl CourtStructure {
    /// Create a standard court structure (Supreme Court → Appellate → Trial).
    #[must_use]
    pub fn standard(
        highest: impl Into<String>,
        appellate: impl Into<String>,
        trial: impl Into<String>,
    ) -> Self {
        Self {
            highest_court: highest.into(),
            appellate_court: Some(appellate.into()),
            trial_court: trial.into(),
            specialized_courts: Vec::new(),
        }
    }

    /// Create court structure without intermediate appellate court.
    #[must_use]
    pub fn two_tier(highest: impl Into<String>, trial: impl Into<String>) -> Self {
        Self {
            highest_court: highest.into(),
            appellate_court: None,
            trial_court: trial.into(),
            specialized_courts: Vec::new(),
        }
    }

    /// Add specialized court.
    #[must_use]
    pub fn with_specialized_court(mut self, court: impl Into<String>) -> Self {
        self.specialized_courts.push(court.into());
        self
    }
}

/// Geographic region classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeographicRegion {
    /// New England states
    NewEngland,

    /// Mid-Atlantic states
    MidAtlantic,

    /// Southern states
    South,

    /// Midwest states
    Midwest,

    /// Southwest states
    Southwest,

    /// Western states
    West,

    /// Pacific states
    Pacific,
}

/// State registry - centralized lookup for state metadata.
pub struct StateRegistry {
    /// Map from state code to metadata
    states: HashMap<String, StateMetadata>,
}

impl Default for StateRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl StateRegistry {
    /// Create a new state registry with Phase 1 priority states pre-loaded.
    #[must_use]
    pub fn new() -> Self {
        let mut registry = Self {
            states: HashMap::new(),
        };

        // Pre-load Phase 1 priority states
        registry.register(Self::california_metadata());
        registry.register(Self::new_york_metadata());
        registry.register(Self::texas_metadata());
        registry.register(Self::louisiana_metadata());
        registry.register(Self::florida_metadata());

        registry
    }

    /// Register a state in the registry.
    pub fn register(&mut self, metadata: StateMetadata) {
        self.states.insert(metadata.id.code.clone(), metadata);
    }

    /// Get state metadata by code.
    #[must_use]
    pub fn get(&self, code: &str) -> Option<&StateMetadata> {
        self.states.get(code)
    }

    /// Get all registered states.
    #[must_use]
    pub fn all_states(&self) -> Vec<&StateMetadata> {
        self.states.values().collect()
    }

    /// Get Phase 1 priority state codes.
    #[must_use]
    pub fn phase_1_states() -> Vec<String> {
        vec![
            "CA".to_string(),
            "NY".to_string(),
            "TX".to_string(),
            "LA".to_string(),
            "FL".to_string(),
        ]
    }

    /// Get states by legal tradition.
    #[must_use]
    pub fn states_by_tradition(&self, tradition: LegalTradition) -> Vec<&StateMetadata> {
        self.states
            .values()
            .filter(|meta| meta.id.legal_tradition == tradition)
            .collect()
    }

    /// Get states by region.
    #[must_use]
    pub fn states_by_region(&self, region: GeographicRegion) -> Vec<&StateMetadata> {
        self.states
            .values()
            .filter(|meta| meta.region == region)
            .collect()
    }

    /// Check if UCC has been adopted (placeholder - will be refined in uniform_acts module).
    #[must_use]
    pub fn has_adopted_ucc(&self, state_code: &str, article: u8) -> bool {
        // All 50 states have adopted UCC Articles 1-9, with Louisiana having unique variations
        if let Some(metadata) = self.get(state_code) {
            if metadata.id.legal_tradition == LegalTradition::CivilLaw && article >= 2 {
                // Louisiana has modified versions of UCC
                return true;
            }
            // All other states have standard UCC adoption
            return (1..=9).contains(&article);
        }
        false
    }

    // ===== Metadata Constructors for Phase 1 States =====

    /// California metadata.
    #[must_use]
    fn california_metadata() -> StateMetadata {
        StateMetadata::new(
            StateId::california(),
            "Sacramento",
            39_000_000,
            CourtStructure::standard(
                "California Supreme Court",
                "California Court of Appeal (6 districts)",
                "Superior Court",
            )
            .with_specialized_court("Appellate Division of Superior Court"),
            1850,
            GeographicRegion::Pacific,
        )
        .with_feature("Pure comparative negligence (Li v. Yellow Cab, 1975)")
        .with_feature("Interest analysis for choice of law")
        .with_feature("CCPA privacy protection (2018)")
        .with_feature("Community property state")
        .with_feature("Largest state economy ($3.9 trillion GDP)")
    }

    /// New York metadata.
    #[must_use]
    fn new_york_metadata() -> StateMetadata {
        StateMetadata::new(
            StateId::new_york(),
            "Albany",
            19_000_000,
            CourtStructure::standard(
                "New York Court of Appeals",
                "Appellate Division of Supreme Court (4 departments)",
                "Supreme Court",
            )
            .with_specialized_court("Surrogate's Court")
            .with_specialized_court("Family Court")
            .with_specialized_court("Court of Claims"),
            1788,
            GeographicRegion::MidAtlantic,
        )
        .with_feature("Pure comparative negligence (CPLR § 1411)")
        .with_feature("Cardozo Court of Appeals legacy (Palsgraf, MacPherson)")
        .with_feature("Combined modern approach to choice of law")
        .with_feature("Financial capital of the world")
        .with_feature("Highest appellate court influence nationwide")
    }

    /// Texas metadata.
    #[must_use]
    fn texas_metadata() -> StateMetadata {
        StateMetadata::new(
            StateId::texas(),
            "Austin",
            30_000_000,
            CourtStructure::standard(
                "Texas Supreme Court",
                "Court of Appeals (14 districts)",
                "District Court",
            )
            .with_specialized_court("Texas Court of Criminal Appeals")
            .with_specialized_court("County Court")
            .with_specialized_court("Justice Court"),
            1845,
            GeographicRegion::Southwest,
        )
        .with_feature("Modified comparative negligence - 51% bar")
        .with_feature("Tort reform with medical malpractice caps ($250,000 non-economic)")
        .with_feature("Community property state")
        .with_feature("No state income tax")
        .with_feature("Second largest state by population and economy")
    }

    /// Louisiana metadata.
    #[must_use]
    fn louisiana_metadata() -> StateMetadata {
        StateMetadata::new(
            StateId::louisiana(),
            "Baton Rouge",
            4_600_000,
            CourtStructure::standard(
                "Louisiana Supreme Court",
                "Court of Appeal (5 circuits)",
                "District Court",
            )
            .with_specialized_court("City Court")
            .with_specialized_court("Justice of the Peace Court")
            .with_specialized_court("Parish Court"),
            1812,
            GeographicRegion::South,
        )
        .with_feature("ONLY Civil Law state in US (French/Spanish heritage)")
        .with_feature("Louisiana Civil Code (similar to Code Napoléon)")
        .with_feature("Forced heirship (limited)")
        .with_feature("Community property state")
        .with_feature("Unique legal terminology (obligor/obligee vs debtor/creditor)")
        .with_feature("No 'tort' concept - uses 'delict' from French law")
    }

    /// Florida metadata.
    #[must_use]
    fn florida_metadata() -> StateMetadata {
        StateMetadata::new(
            StateId::florida(),
            "Tallahassee",
            22_000_000,
            CourtStructure::standard(
                "Florida Supreme Court",
                "District Court of Appeal (5 districts)",
                "Circuit Court",
            )
            .with_specialized_court("County Court")
            .with_specialized_court("Small Claims Court"),
            1845,
            GeographicRegion::South,
        )
        .with_feature("Pure comparative negligence (Fla. Stat. § 768.81)")
        .with_feature("Stand Your Ground law (Fla. Stat. § 776.032)")
        .with_feature("No state income tax")
        .with_feature("Third largest state by population")
        .with_feature("Significant retiree population affecting legal landscape")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let registry = StateRegistry::new();
        assert_eq!(registry.states.len(), 5); // Phase 1 states only
    }

    #[test]
    fn test_get_state_by_code() {
        let registry = StateRegistry::new();

        let ca = registry.get("CA").unwrap();
        assert_eq!(ca.id.name, "California");
        assert_eq!(ca.capital, "Sacramento");
        assert_eq!(ca.population, 39_000_000);

        let ny = registry.get("NY").unwrap();
        assert_eq!(ny.id.name, "New York");
        assert_eq!(
            ny.court_structure.highest_court,
            "New York Court of Appeals"
        );
    }

    #[test]
    fn test_louisiana_civil_law() {
        let registry = StateRegistry::new();
        let la = registry.get("LA").unwrap();

        assert_eq!(la.id.legal_tradition, LegalTradition::CivilLaw);
        assert!(la.notable_features.iter().any(|f| f.contains("Civil Law")));
        assert!(la.notable_features.iter().any(|f| f.contains("Civil Code")));
    }

    #[test]
    fn test_states_by_tradition() {
        let registry = StateRegistry::new();

        let common_law_states = registry.states_by_tradition(LegalTradition::CommonLaw);
        assert_eq!(common_law_states.len(), 4); // CA, NY, TX, FL

        let civil_law_states = registry.states_by_tradition(LegalTradition::CivilLaw);
        assert_eq!(civil_law_states.len(), 1); // LA only
    }

    #[test]
    fn test_states_by_region() {
        let registry = StateRegistry::new();

        let south_states = registry.states_by_region(GeographicRegion::South);
        assert!(south_states.len() >= 2); // LA, FL

        let pacific_states = registry.states_by_region(GeographicRegion::Pacific);
        assert_eq!(pacific_states.len(), 1); // CA
    }

    #[test]
    fn test_phase_1_states() {
        let phase_1 = StateRegistry::phase_1_states();
        assert_eq!(phase_1.len(), 5);
        assert!(phase_1.contains(&"CA".to_string()));
        assert!(phase_1.contains(&"NY".to_string()));
        assert!(phase_1.contains(&"TX".to_string()));
        assert!(phase_1.contains(&"LA".to_string()));
        assert!(phase_1.contains(&"FL".to_string()));
    }

    #[test]
    fn test_ucc_adoption() {
        let registry = StateRegistry::new();

        // All states should have adopted UCC Articles 1-9
        assert!(registry.has_adopted_ucc("CA", 2));
        assert!(registry.has_adopted_ucc("NY", 2));
        assert!(registry.has_adopted_ucc("LA", 2)); // Louisiana too, with modifications

        // Invalid state code
        assert!(!registry.has_adopted_ucc("XX", 2));
    }

    #[test]
    fn test_court_structure() {
        let ca_courts = CourtStructure::standard(
            "California Supreme Court",
            "Court of Appeal",
            "Superior Court",
        )
        .with_specialized_court("Appellate Division");

        assert_eq!(ca_courts.highest_court, "California Supreme Court");
        assert!(ca_courts.appellate_court.is_some());
        assert_eq!(ca_courts.specialized_courts.len(), 1);
    }

    #[test]
    fn test_metadata_builder() {
        let metadata = StateMetadata::new(
            StateId::california(),
            "Sacramento",
            39_000_000,
            CourtStructure::standard("Supreme Court", "Court of Appeal", "Superior Court"),
            1850,
            GeographicRegion::Pacific,
        )
        .with_feature("Feature 1")
        .with_feature("Feature 2");

        assert_eq!(metadata.notable_features.len(), 2);
        assert_eq!(metadata.statehood_year, 1850);
    }
}
