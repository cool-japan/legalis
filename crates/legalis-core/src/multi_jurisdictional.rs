//! Multi-Jurisdictional Legal Reasoning
//!
//! This module provides tools for handling legal issues across multiple jurisdictions:
//! - Jurisdiction conflict resolution rules
//! - Choice-of-law heuristics
//! - Treaty and international law integration
//! - Federal/state/local hierarchy modeling
//! - Cross-border statute harmonization
//!
//! # Examples
//!
//! ```
//! use legalis_core::multi_jurisdictional::{JurisdictionConflictResolver, ConflictRule};
//!
//! let resolver = JurisdictionConflictResolver::new();
//! // Resolve conflicts between different jurisdictions
//! ```

use std::collections::HashMap;
use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::Statute;

/// Jurisdiction hierarchy level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum JurisdictionLevel {
    /// International law (treaties, conventions)
    International,
    /// Federal/national law
    Federal,
    /// State/provincial law
    State,
    /// Local/municipal law
    Local,
}

impl fmt::Display for JurisdictionLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JurisdictionLevel::International => write!(f, "International"),
            JurisdictionLevel::Federal => write!(f, "Federal"),
            JurisdictionLevel::State => write!(f, "State"),
            JurisdictionLevel::Local => write!(f, "Local"),
        }
    }
}

impl JurisdictionLevel {
    /// Get the precedence value (higher = more authoritative)
    pub fn precedence(&self) -> u8 {
        match self {
            JurisdictionLevel::International => 10,
            JurisdictionLevel::Federal => 30,
            JurisdictionLevel::State => 20,
            JurisdictionLevel::Local => 10,
        }
    }

    /// Check if this level supersedes another
    pub fn supersedes(&self, other: &JurisdictionLevel) -> bool {
        self.precedence() > other.precedence()
    }
}

/// Jurisdiction conflict resolution rule
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ConflictRule {
    /// Lex superior - higher authority prevails
    LexSuperior,
    /// Lex specialis - more specific law prevails
    LexSpecialis,
    /// Lex posterior - later law prevails
    LexPosterior,
    /// Forum law - law of the forum prevails
    LexFori,
    /// Place of wrong - law where harm occurred
    LexLoci,
    /// Most significant relationship
    MostSignificantRelationship,
}

impl fmt::Display for ConflictRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConflictRule::LexSuperior => write!(f, "Lex Superior (Higher Authority)"),
            ConflictRule::LexSpecialis => write!(f, "Lex Specialis (More Specific)"),
            ConflictRule::LexPosterior => write!(f, "Lex Posterior (Later Law)"),
            ConflictRule::LexFori => write!(f, "Lex Fori (Forum Law)"),
            ConflictRule::LexLoci => write!(f, "Lex Loci (Place of Wrong)"),
            ConflictRule::MostSignificantRelationship => {
                write!(f, "Most Significant Relationship")
            }
        }
    }
}

impl ConflictRule {
    /// Get a description of when this rule applies
    pub fn description(&self) -> &str {
        match self {
            ConflictRule::LexSuperior => {
                "When laws from different hierarchical levels conflict, the higher authority prevails"
            }
            ConflictRule::LexSpecialis => {
                "When general and specific laws conflict, the more specific law prevails"
            }
            ConflictRule::LexPosterior => {
                "When laws from the same authority conflict, the later law prevails"
            }
            ConflictRule::LexFori => "The law of the forum (court's jurisdiction) applies",
            ConflictRule::LexLoci => "The law of the place where the wrongful act occurred applies",
            ConflictRule::MostSignificantRelationship => {
                "Apply the law of the jurisdiction with the most significant relationship to the issue"
            }
        }
    }
}

/// Result of jurisdiction conflict resolution
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConflictResolution {
    /// The winning statute
    pub winner: String,
    /// The rule that was applied
    pub rule: ConflictRule,
    /// Explanation of why this resolution was chosen
    pub explanation: String,
    /// Alternative statutes that could apply
    pub alternatives: Vec<String>,
}

impl ConflictResolution {
    /// Create a new conflict resolution
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::multi_jurisdictional::{ConflictResolution, ConflictRule};
    ///
    /// let resolution = ConflictResolution::new(
    ///     "federal-statute-1",
    ///     ConflictRule::LexSuperior,
    ///     "Federal law supersedes state law",
    ///     vec!["state-statute-1".to_string()]
    /// );
    ///
    /// assert_eq!(resolution.winner, "federal-statute-1");
    /// ```
    pub fn new(
        winner: impl Into<String>,
        rule: ConflictRule,
        explanation: impl Into<String>,
        alternatives: Vec<String>,
    ) -> Self {
        Self {
            winner: winner.into(),
            rule,
            explanation: explanation.into(),
            alternatives,
        }
    }
}

/// Jurisdiction conflict resolver
///
/// Resolves conflicts between statutes from different jurisdictions
/// using established choice-of-law principles.
///
/// # Examples
///
/// ```
/// use legalis_core::multi_jurisdictional::{JurisdictionConflictResolver, JurisdictionLevel};
/// use legalis_core::{Statute, Effect, EffectType};
///
/// let mut resolver = JurisdictionConflictResolver::new();
///
/// let federal_statute = Statute::new("fed-1", "Federal Rule", Effect::new(EffectType::Grant, "Right"));
/// let state_statute = Statute::new("state-1", "State Rule", Effect::new(EffectType::Grant, "Right"));
///
/// resolver.add_statute_with_level(federal_statute, JurisdictionLevel::Federal);
/// resolver.add_statute_with_level(state_statute, JurisdictionLevel::State);
///
/// // Federal law should win due to lex superior
/// let resolution = resolver.resolve_conflict(&["fed-1", "state-1"]);
/// assert_eq!(resolution.winner, "fed-1");
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct JurisdictionConflictResolver {
    /// Statutes indexed by ID
    statutes: HashMap<String, Statute>,
    /// Jurisdiction levels for each statute
    levels: HashMap<String, JurisdictionLevel>,
}

impl JurisdictionConflictResolver {
    /// Create a new jurisdiction conflict resolver
    pub fn new() -> Self {
        Self {
            statutes: HashMap::new(),
            levels: HashMap::new(),
        }
    }

    /// Add a statute with its jurisdiction level
    pub fn add_statute_with_level(&mut self, statute: Statute, level: JurisdictionLevel) {
        let id = statute.id.clone();
        self.statutes.insert(id.clone(), statute);
        self.levels.insert(id, level);
    }

    /// Resolve conflict between multiple statutes
    pub fn resolve_conflict(&self, statute_ids: &[&str]) -> ConflictResolution {
        if statute_ids.is_empty() {
            return ConflictResolution::new(
                "",
                ConflictRule::LexSuperior,
                "No statutes to resolve",
                vec![],
            );
        }

        if statute_ids.len() == 1 {
            return ConflictResolution::new(
                statute_ids[0],
                ConflictRule::LexSuperior,
                "Only one statute - no conflict",
                vec![],
            );
        }

        // Try lex superior first (hierarchy)
        if let Some(resolution) = self.apply_lex_superior(statute_ids) {
            return resolution;
        }

        // Then try lex posterior (temporal precedence)
        if let Some(resolution) = self.apply_lex_posterior(statute_ids) {
            return resolution;
        }

        // Default to first statute
        let alternatives: Vec<String> = statute_ids[1..].iter().map(|s| s.to_string()).collect();

        ConflictResolution::new(
            statute_ids[0],
            ConflictRule::MostSignificantRelationship,
            "Unable to determine clear winner - using first statute",
            alternatives,
        )
    }

    fn apply_lex_superior(&self, statute_ids: &[&str]) -> Option<ConflictResolution> {
        let mut max_level: Option<JurisdictionLevel> = None;
        let mut winner = None;

        for &id in statute_ids {
            if let Some(&level) = self.levels.get(id) {
                if max_level.is_none() || level.precedence() > max_level.unwrap().precedence() {
                    max_level = Some(level);
                    winner = Some(id);
                }
            }
        }

        winner.map(|w| {
            let alternatives: Vec<String> = statute_ids
                .iter()
                .filter(|&&id| id != w)
                .map(|s| s.to_string())
                .collect();

            ConflictResolution::new(
                w,
                ConflictRule::LexSuperior,
                format!(
                    "{} law supersedes lower jurisdiction laws",
                    max_level.unwrap()
                ),
                alternatives,
            )
        })
    }

    fn apply_lex_posterior(&self, statute_ids: &[&str]) -> Option<ConflictResolution> {
        let mut latest_version = 0;
        let mut winner = None;

        for &id in statute_ids {
            if let Some(statute) = self.statutes.get(id) {
                if statute.version > latest_version {
                    latest_version = statute.version;
                    winner = Some(id);
                }
            }
        }

        if latest_version > 0 {
            winner.map(|w| {
                let alternatives: Vec<String> = statute_ids
                    .iter()
                    .filter(|&&id| id != w)
                    .map(|s| s.to_string())
                    .collect();

                ConflictResolution::new(
                    w,
                    ConflictRule::LexPosterior,
                    format!(
                        "Later version (v{}) supersedes earlier versions",
                        latest_version
                    ),
                    alternatives,
                )
            })
        } else {
            None
        }
    }

    /// Get statute by ID
    pub fn get_statute(&self, id: &str) -> Option<&Statute> {
        self.statutes.get(id)
    }

    /// Get jurisdiction level for a statute
    pub fn get_level(&self, id: &str) -> Option<JurisdictionLevel> {
        self.levels.get(id).copied()
    }

    /// Get all statutes at a specific level
    pub fn statutes_at_level(&self, level: JurisdictionLevel) -> Vec<&Statute> {
        self.levels
            .iter()
            .filter(|(_, l)| **l == level)
            .filter_map(|(id, _)| self.statutes.get(id))
            .collect()
    }

    /// Count statutes by level
    pub fn count_by_level(&self, level: JurisdictionLevel) -> usize {
        self.levels.iter().filter(|(_, l)| **l == level).count()
    }
}

impl Default for JurisdictionConflictResolver {
    fn default() -> Self {
        Self::new()
    }
}

/// Choice-of-law analyzer
///
/// Determines which jurisdiction's law should apply to a legal issue
/// based on connecting factors and choice-of-law rules.
///
/// # Examples
///
/// ```
/// use legalis_core::multi_jurisdictional::{ChoiceOfLawAnalyzer, ConnectingFactor};
///
/// let mut analyzer = ChoiceOfLawAnalyzer::new();
/// analyzer.add_factor(ConnectingFactor::PlaceOfPerformance, "California");
/// analyzer.add_factor(ConnectingFactor::DomicileOfParties, "New York");
///
/// let recommendation = analyzer.recommend_jurisdiction();
/// assert!(recommendation.is_some());
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChoiceOfLawAnalyzer {
    /// Connecting factors and their jurisdictions
    factors: HashMap<ConnectingFactor, String>,
}

/// Connecting factors for choice of law
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ConnectingFactor {
    /// Where the contract was formed
    PlaceOfContract,
    /// Where the contract is performed
    PlaceOfPerformance,
    /// Where the wrongful act occurred
    PlaceOfWrong,
    /// Where the injury occurred
    PlaceOfInjury,
    /// Domicile of the parties
    DomicileOfParties,
    /// Habitual residence
    HabitualResidence,
    /// Location of property
    SitusOfProperty,
    /// Place of business
    PlaceOfBusiness,
    /// Forum selection clause
    ForumSelection,
}

impl fmt::Display for ConnectingFactor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectingFactor::PlaceOfContract => write!(f, "Place of Contract"),
            ConnectingFactor::PlaceOfPerformance => write!(f, "Place of Performance"),
            ConnectingFactor::PlaceOfWrong => write!(f, "Place of Wrong"),
            ConnectingFactor::PlaceOfInjury => write!(f, "Place of Injury"),
            ConnectingFactor::DomicileOfParties => write!(f, "Domicile of Parties"),
            ConnectingFactor::HabitualResidence => write!(f, "Habitual Residence"),
            ConnectingFactor::SitusOfProperty => write!(f, "Situs of Property"),
            ConnectingFactor::PlaceOfBusiness => write!(f, "Place of Business"),
            ConnectingFactor::ForumSelection => write!(f, "Forum Selection"),
        }
    }
}

impl ConnectingFactor {
    /// Get the weight/importance of this factor (higher = more important)
    pub fn weight(&self) -> u8 {
        match self {
            ConnectingFactor::ForumSelection => 90,
            ConnectingFactor::PlaceOfWrong => 80,
            ConnectingFactor::PlaceOfInjury => 75,
            ConnectingFactor::PlaceOfPerformance => 70,
            ConnectingFactor::DomicileOfParties => 60,
            ConnectingFactor::HabitualResidence => 55,
            ConnectingFactor::PlaceOfContract => 50,
            ConnectingFactor::PlaceOfBusiness => 45,
            ConnectingFactor::SitusOfProperty => 40,
        }
    }
}

impl ChoiceOfLawAnalyzer {
    /// Create a new choice-of-law analyzer
    pub fn new() -> Self {
        Self {
            factors: HashMap::new(),
        }
    }

    /// Add a connecting factor
    pub fn add_factor(&mut self, factor: ConnectingFactor, jurisdiction: impl Into<String>) {
        self.factors.insert(factor, jurisdiction.into());
    }

    /// Recommend which jurisdiction's law should apply
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::multi_jurisdictional::{ChoiceOfLawAnalyzer, ConnectingFactor};
    ///
    /// let mut analyzer = ChoiceOfLawAnalyzer::new();
    /// analyzer.add_factor(ConnectingFactor::ForumSelection, "Delaware");
    /// analyzer.add_factor(ConnectingFactor::PlaceOfPerformance, "California");
    ///
    /// let recommendation = analyzer.recommend_jurisdiction();
    /// assert_eq!(recommendation.as_deref(), Some("Delaware"));
    /// ```
    pub fn recommend_jurisdiction(&self) -> Option<String> {
        if self.factors.is_empty() {
            return None;
        }

        // Find the factor with the highest weight
        let mut max_weight = 0;
        let mut recommended = None;

        for (factor, jurisdiction) in &self.factors {
            let weight = factor.weight();
            if weight > max_weight {
                max_weight = weight;
                recommended = Some(jurisdiction.clone());
            }
        }

        recommended
    }

    /// Get all jurisdictions mentioned with their weights
    pub fn jurisdiction_weights(&self) -> HashMap<String, u8> {
        let mut weights: HashMap<String, u8> = HashMap::new();

        for (factor, jurisdiction) in &self.factors {
            let weight = factor.weight();
            *weights.entry(jurisdiction.clone()).or_insert(0) += weight;
        }

        weights
    }

    /// Get the most significant jurisdiction based on cumulative weights
    pub fn most_significant_jurisdiction(&self) -> Option<String> {
        let weights = self.jurisdiction_weights();
        weights.into_iter().max_by_key(|(_, w)| *w).map(|(j, _)| j)
    }

    /// Get all connecting factors
    pub fn factors(&self) -> &HashMap<ConnectingFactor, String> {
        &self.factors
    }

    /// Clear all factors
    pub fn clear(&mut self) {
        self.factors.clear();
    }
}

impl Default for ChoiceOfLawAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// International treaty or convention
///
/// Represents an international legal instrument that may override
/// or harmonize with domestic law.
///
/// # Examples
///
/// ```
/// use legalis_core::multi_jurisdictional::{Treaty, TreatyType};
///
/// let mut treaty = Treaty::new(
///     "CISG",
///     "UN Convention on Contracts for International Sale of Goods",
///     TreatyType::Convention
/// );
///
/// assert_eq!(treaty.id, "CISG");
/// treaty.add_signatory("US");
/// assert!(treaty.is_binding());
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Treaty {
    /// Treaty identifier
    pub id: String,
    /// Full name of the treaty
    pub name: String,
    /// Type of treaty
    pub treaty_type: TreatyType,
    /// Signatory jurisdictions
    pub signatories: Vec<String>,
    /// Whether the treaty is self-executing
    pub self_executing: bool,
}

/// Type of international legal instrument
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TreatyType {
    /// Bilateral treaty
    Bilateral,
    /// Multilateral treaty
    Multilateral,
    /// International convention
    Convention,
    /// Protocol (amendment to existing treaty)
    Protocol,
    /// Agreement
    Agreement,
}

impl fmt::Display for TreatyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TreatyType::Bilateral => write!(f, "Bilateral Treaty"),
            TreatyType::Multilateral => write!(f, "Multilateral Treaty"),
            TreatyType::Convention => write!(f, "Convention"),
            TreatyType::Protocol => write!(f, "Protocol"),
            TreatyType::Agreement => write!(f, "Agreement"),
        }
    }
}

impl Treaty {
    /// Create a new treaty
    pub fn new(id: impl Into<String>, name: impl Into<String>, treaty_type: TreatyType) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            treaty_type,
            signatories: Vec::new(),
            self_executing: false,
        }
    }

    /// Add a signatory jurisdiction
    pub fn add_signatory(&mut self, jurisdiction: impl Into<String>) {
        self.signatories.push(jurisdiction.into());
    }

    /// Check if a jurisdiction is a signatory
    pub fn is_signatory(&self, jurisdiction: &str) -> bool {
        self.signatories.iter().any(|s| s == jurisdiction)
    }

    /// Check if the treaty is binding
    pub fn is_binding(&self) -> bool {
        !self.signatories.is_empty()
    }

    /// Set whether the treaty is self-executing
    pub fn set_self_executing(&mut self, self_executing: bool) {
        self.self_executing = self_executing;
    }

    /// Check if treaty applies between two jurisdictions
    pub fn applies_between(&self, jurisdiction_a: &str, jurisdiction_b: &str) -> bool {
        self.is_signatory(jurisdiction_a) && self.is_signatory(jurisdiction_b)
    }
}

/// Federal-state-local hierarchy manager
///
/// Models the hierarchy of legal authority from federal down to local levels,
/// with proper preemption and supremacy rules.
///
/// # Examples
///
/// ```
/// use legalis_core::multi_jurisdictional::{HierarchyManager, JurisdictionLevel};
/// use legalis_core::{Statute, Effect, EffectType};
///
/// let mut manager = HierarchyManager::new();
///
/// let federal = Statute::new("fed-1", "Federal Rule", Effect::new(EffectType::Grant, "R"));
/// manager.add_statute(federal, JurisdictionLevel::Federal, None);
///
/// assert!(manager.has_federal_preemption("fed-1"));
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HierarchyManager {
    /// Statutes organized by level
    statutes: HashMap<String, (Statute, JurisdictionLevel)>,
    /// Parent jurisdiction relationships
    parent_jurisdiction: HashMap<String, String>,
}

impl HierarchyManager {
    /// Create a new hierarchy manager
    pub fn new() -> Self {
        Self {
            statutes: HashMap::new(),
            parent_jurisdiction: HashMap::new(),
        }
    }

    /// Add a statute at a specific level with optional parent jurisdiction
    pub fn add_statute(
        &mut self,
        statute: Statute,
        level: JurisdictionLevel,
        parent: Option<String>,
    ) {
        let id = statute.id.clone();
        self.statutes.insert(id.clone(), (statute, level));
        if let Some(parent_id) = parent {
            self.parent_jurisdiction.insert(id, parent_id);
        }
    }

    /// Check if a federal statute preempts state/local law
    pub fn has_federal_preemption(&self, federal_statute_id: &str) -> bool {
        if let Some((_, level)) = self.statutes.get(federal_statute_id) {
            *level == JurisdictionLevel::Federal
        } else {
            false
        }
    }

    /// Get the hierarchy path for a jurisdiction (local -> state -> federal)
    pub fn get_hierarchy_path(&self, jurisdiction_id: &str) -> Vec<String> {
        let mut path = vec![jurisdiction_id.to_string()];
        let mut current = jurisdiction_id;

        while let Some(parent) = self.parent_jurisdiction.get(current) {
            path.push(parent.clone());
            current = parent;
        }

        path
    }

    /// Find the highest authority statute that applies
    pub fn highest_authority(&self, statute_ids: &[&str]) -> Option<String> {
        let mut highest: Option<(String, JurisdictionLevel)> = None;

        for &id in statute_ids {
            if let Some((_, level)) = self.statutes.get(id) {
                if highest.is_none()
                    || level.precedence() > highest.as_ref().unwrap().1.precedence()
                {
                    highest = Some((id.to_string(), *level));
                }
            }
        }

        highest.map(|(id, _)| id)
    }

    /// Get statute by ID
    pub fn get_statute(&self, id: &str) -> Option<&Statute> {
        self.statutes.get(id).map(|(s, _)| s)
    }

    /// Get all statutes at a level
    pub fn statutes_at_level(&self, level: JurisdictionLevel) -> Vec<&Statute> {
        self.statutes
            .values()
            .filter(|(_, l)| *l == level)
            .map(|(s, _)| s)
            .collect()
    }
}

impl Default for HierarchyManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Cross-border statute harmonizer
///
/// Identifies similarities and conflicts between statutes from
/// different jurisdictions to facilitate harmonization.
///
/// # Examples
///
/// ```
/// use legalis_core::multi_jurisdictional::StatuteHarmonizer;
/// use legalis_core::{Statute, Effect, EffectType};
///
/// let mut harmonizer = StatuteHarmonizer::new();
///
/// let us_statute = Statute::new("us-1", "US Contract Law", Effect::new(EffectType::Grant, "R"));
/// let uk_statute = Statute::new("uk-1", "UK Contract Law", Effect::new(EffectType::Grant, "R"));
///
/// harmonizer.add_statute("US", us_statute);
/// harmonizer.add_statute("UK", uk_statute);
///
/// let similarities = harmonizer.find_similarities("US", "UK");
/// assert!(similarities >= 0.0 && similarities <= 1.0);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StatuteHarmonizer {
    /// Statutes by jurisdiction
    statutes_by_jurisdiction: HashMap<String, Vec<Statute>>,
}

impl StatuteHarmonizer {
    /// Create a new statute harmonizer
    pub fn new() -> Self {
        Self {
            statutes_by_jurisdiction: HashMap::new(),
        }
    }

    /// Add a statute to a jurisdiction
    pub fn add_statute(&mut self, jurisdiction: impl Into<String>, statute: Statute) {
        self.statutes_by_jurisdiction
            .entry(jurisdiction.into())
            .or_default()
            .push(statute);
    }

    /// Find similarity score between two jurisdictions' statutes (0.0 to 1.0)
    pub fn find_similarities(&self, jurisdiction_a: &str, jurisdiction_b: &str) -> f64 {
        let statutes_a = self.statutes_by_jurisdiction.get(jurisdiction_a);
        let statutes_b = self.statutes_by_jurisdiction.get(jurisdiction_b);

        match (statutes_a, statutes_b) {
            (Some(a), Some(b)) => {
                if a.is_empty() || b.is_empty() {
                    return 0.0;
                }

                // Simple similarity: count matching effect types
                let mut matches = 0;
                for statute_a in a {
                    for statute_b in b {
                        if statute_a.effect.effect_type == statute_b.effect.effect_type {
                            matches += 1;
                        }
                    }
                }

                let total = a.len() * b.len();
                if total == 0 {
                    0.0
                } else {
                    matches as f64 / total as f64
                }
            }
            _ => 0.0,
        }
    }

    /// Find potential conflicts between jurisdictions
    pub fn find_conflicts(
        &self,
        jurisdiction_a: &str,
        jurisdiction_b: &str,
    ) -> Vec<(String, String)> {
        let statutes_a = self.statutes_by_jurisdiction.get(jurisdiction_a);
        let statutes_b = self.statutes_by_jurisdiction.get(jurisdiction_b);

        let mut conflicts = Vec::new();

        if let (Some(a), Some(b)) = (statutes_a, statutes_b) {
            for statute_a in a {
                for statute_b in b {
                    // Simple conflict detection: same title but different effect types
                    if statute_a.title.to_lowercase().contains(
                        statute_b
                            .title
                            .to_lowercase()
                            .split_whitespace()
                            .next()
                            .unwrap_or(""),
                    ) && statute_a.effect.effect_type != statute_b.effect.effect_type
                    {
                        conflicts.push((statute_a.id.clone(), statute_b.id.clone()));
                    }
                }
            }
        }

        conflicts
    }

    /// Get all jurisdictions
    pub fn jurisdictions(&self) -> Vec<String> {
        self.statutes_by_jurisdiction.keys().cloned().collect()
    }

    /// Get statutes for a jurisdiction
    pub fn get_statutes(&self, jurisdiction: &str) -> Option<&Vec<Statute>> {
        self.statutes_by_jurisdiction.get(jurisdiction)
    }

    /// Count total statutes
    pub fn total_statutes(&self) -> usize {
        self.statutes_by_jurisdiction
            .values()
            .map(|v| v.len())
            .sum()
    }
}

impl Default for StatuteHarmonizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Effect, EffectType};

    #[test]
    fn test_jurisdiction_level_precedence() {
        assert!(JurisdictionLevel::Federal.precedence() > JurisdictionLevel::State.precedence());
        assert!(JurisdictionLevel::Federal.supersedes(&JurisdictionLevel::State));
    }

    #[test]
    fn test_conflict_rule_display() {
        assert_eq!(
            ConflictRule::LexSuperior.to_string(),
            "Lex Superior (Higher Authority)"
        );
    }

    #[test]
    fn test_jurisdiction_conflict_resolver() {
        let mut resolver = JurisdictionConflictResolver::new();

        let fed = Statute::new(
            "fed-1",
            "Federal Law",
            Effect::new(EffectType::Grant, "Right"),
        );
        let state = Statute::new(
            "state-1",
            "State Law",
            Effect::new(EffectType::Grant, "Right"),
        );

        resolver.add_statute_with_level(fed, JurisdictionLevel::Federal);
        resolver.add_statute_with_level(state, JurisdictionLevel::State);

        let resolution = resolver.resolve_conflict(&["fed-1", "state-1"]);
        assert_eq!(resolution.winner, "fed-1");
        assert_eq!(resolution.rule, ConflictRule::LexSuperior);
    }

    #[test]
    fn test_choice_of_law_analyzer() {
        let mut analyzer = ChoiceOfLawAnalyzer::new();
        analyzer.add_factor(ConnectingFactor::ForumSelection, "Delaware");
        analyzer.add_factor(ConnectingFactor::PlaceOfPerformance, "California");

        let recommendation = analyzer.recommend_jurisdiction();
        assert_eq!(recommendation.as_deref(), Some("Delaware"));
    }

    #[test]
    fn test_connecting_factor_weights() {
        assert!(
            ConnectingFactor::ForumSelection.weight() > ConnectingFactor::PlaceOfContract.weight()
        );
    }

    #[test]
    fn test_most_significant_jurisdiction() {
        let mut analyzer = ChoiceOfLawAnalyzer::new();
        analyzer.add_factor(ConnectingFactor::PlaceOfContract, "NY");
        analyzer.add_factor(ConnectingFactor::PlaceOfPerformance, "NY");
        analyzer.add_factor(ConnectingFactor::DomicileOfParties, "CA");

        let most_sig = analyzer.most_significant_jurisdiction();
        assert_eq!(most_sig.as_deref(), Some("NY"));
    }

    #[test]
    fn test_statutes_at_level() {
        let mut resolver = JurisdictionConflictResolver::new();

        let fed1 = Statute::new("fed-1", "Federal 1", Effect::new(EffectType::Grant, "R1"));
        let fed2 = Statute::new("fed-2", "Federal 2", Effect::new(EffectType::Grant, "R2"));
        let state1 = Statute::new("state-1", "State 1", Effect::new(EffectType::Grant, "R3"));

        resolver.add_statute_with_level(fed1, JurisdictionLevel::Federal);
        resolver.add_statute_with_level(fed2, JurisdictionLevel::Federal);
        resolver.add_statute_with_level(state1, JurisdictionLevel::State);

        let federal_statutes = resolver.statutes_at_level(JurisdictionLevel::Federal);
        assert_eq!(federal_statutes.len(), 2);
        assert_eq!(resolver.count_by_level(JurisdictionLevel::Federal), 2);
    }

    #[test]
    fn test_treaty_creation() {
        let mut treaty = Treaty::new(
            "CISG",
            "UN Convention on International Sale of Goods",
            TreatyType::Convention,
        );
        assert_eq!(treaty.id, "CISG");
        assert!(!treaty.is_binding());

        treaty.add_signatory("US");
        treaty.add_signatory("Germany");

        assert!(treaty.is_binding());
        assert!(treaty.is_signatory("US"));
        assert!(treaty.applies_between("US", "Germany"));
        assert!(!treaty.applies_between("US", "UK"));
    }

    #[test]
    fn test_hierarchy_manager() {
        let mut manager = HierarchyManager::new();

        let federal = Statute::new("fed-1", "Federal Law", Effect::new(EffectType::Grant, "R"));
        let state = Statute::new("state-1", "State Law", Effect::new(EffectType::Grant, "R"));

        manager.add_statute(federal, JurisdictionLevel::Federal, None);
        manager.add_statute(state, JurisdictionLevel::State, Some("fed-1".to_string()));

        assert!(manager.has_federal_preemption("fed-1"));
        assert!(!manager.has_federal_preemption("state-1"));

        let path = manager.get_hierarchy_path("state-1");
        assert_eq!(path.len(), 2);
        assert_eq!(path[0], "state-1");
        assert_eq!(path[1], "fed-1");
    }

    #[test]
    fn test_statute_harmonizer() {
        let mut harmonizer = StatuteHarmonizer::new();

        let us1 = Statute::new("us-1", "Contract Law", Effect::new(EffectType::Grant, "R1"));
        let us2 = Statute::new(
            "us-2",
            "Property Law",
            Effect::new(EffectType::Obligation, "O1"),
        );
        let uk1 = Statute::new("uk-1", "Contract Law", Effect::new(EffectType::Grant, "R1"));

        harmonizer.add_statute("US", us1);
        harmonizer.add_statute("US", us2);
        harmonizer.add_statute("UK", uk1);

        let similarity = harmonizer.find_similarities("US", "UK");
        assert!(similarity > 0.0);
        assert_eq!(harmonizer.total_statutes(), 3);
    }

    #[test]
    fn test_hierarchy_highest_authority() {
        let mut manager = HierarchyManager::new();

        let fed = Statute::new("fed-1", "Federal", Effect::new(EffectType::Grant, "R"));
        let state = Statute::new("state-1", "State", Effect::new(EffectType::Grant, "R"));
        let local = Statute::new("local-1", "Local", Effect::new(EffectType::Grant, "R"));

        manager.add_statute(fed, JurisdictionLevel::Federal, None);
        manager.add_statute(state, JurisdictionLevel::State, None);
        manager.add_statute(local, JurisdictionLevel::Local, None);

        let highest = manager.highest_authority(&["fed-1", "state-1", "local-1"]);
        assert_eq!(highest.as_deref(), Some("fed-1"));
    }
}
