//! Canada Reasoning Engine
//!
//! Integration with legalis-core for Canadian legal reasoning.

#![allow(missing_docs)]

use legalis_core::{Effect, EffectType, Statute};
use serde::{Deserialize, Serialize};

use crate::common::Province;

// ============================================================================
// Canadian Legal Reasoning Engine
// ============================================================================

/// Canadian legal reasoning engine
pub struct CanadianReasoningEngine {
    /// Loaded statutes
    statutes: Vec<Statute>,
    /// Jurisdiction
    jurisdiction: ReasoningJurisdiction,
}

/// Reasoning jurisdiction
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReasoningJurisdiction {
    /// Federal only
    Federal,
    /// Provincial only
    Provincial(Province),
    /// Both federal and provincial
    Combined(Province),
}

impl CanadianReasoningEngine {
    /// Create new reasoning engine
    pub fn new(jurisdiction: ReasoningJurisdiction) -> Self {
        Self {
            statutes: Vec::new(),
            jurisdiction,
        }
    }

    /// Load federal statutes
    pub fn load_federal_statutes(&mut self) {
        self.statutes
            .push(crate::constitution::create_charter_statute());
        self.statutes
            .push(crate::constitution::create_constitution_1867_statute());
        self.statutes
            .push(crate::constitution::create_constitution_1982_statute());
        self.statutes.push(crate::criminal::create_criminal_code());
        self.statutes.push(crate::family::create_divorce_act());
        self.statutes
            .push(crate::family::create_child_support_guidelines());
        self.statutes.push(crate::corporate::create_cbca());
        self.statutes.push(crate::property::create_indian_act());
    }

    /// Load provincial statutes
    pub fn load_provincial_statutes(&mut self, province: &Province) {
        self.statutes
            .extend(crate::contract::create_contract_statutes(province));
        self.statutes
            .extend(crate::employment::create_employment_statutes(province));
        self.statutes
            .extend(crate::family::create_family_statutes(province));
        self.statutes
            .extend(crate::property::create_property_statutes(province));
        self.statutes
            .extend(crate::corporate::create_corporate_statutes(province));

        if let Some(ola) = crate::tort::create_ola_statute(province) {
            self.statutes.push(ola);
        }
    }

    /// Load all statutes for jurisdiction
    pub fn load_all(&mut self) {
        let jurisdiction = self.jurisdiction.clone();
        match jurisdiction {
            ReasoningJurisdiction::Federal => {
                self.load_federal_statutes();
            }
            ReasoningJurisdiction::Provincial(province) => {
                self.load_provincial_statutes(&province);
            }
            ReasoningJurisdiction::Combined(province) => {
                self.load_federal_statutes();
                self.load_provincial_statutes(&province);
            }
        }
    }

    /// Get all loaded statutes
    pub fn statutes(&self) -> &[Statute] {
        &self.statutes
    }

    /// Find statutes by effect type
    pub fn find_by_effect_type(&self, effect_type: EffectType) -> Vec<&Statute> {
        self.statutes
            .iter()
            .filter(|s| s.effect.effect_type == effect_type)
            .collect()
    }

    /// Find statutes by keyword
    pub fn find_by_keyword(&self, keyword: &str) -> Vec<&Statute> {
        let keyword_lower = keyword.to_lowercase();
        self.statutes
            .iter()
            .filter(|s| {
                s.title.to_lowercase().contains(&keyword_lower)
                    || s.effect.description.to_lowercase().contains(&keyword_lower)
            })
            .collect()
    }

    /// Apply reasoning to facts
    pub fn reason(&self, query: &ReasoningQuery) -> ReasoningResult {
        let applicable_statutes = self.find_applicable_statutes(query);
        let effects = self.determine_effects(&applicable_statutes, query);
        let conflicts = self.identify_conflicts(&applicable_statutes);

        ReasoningResult {
            query: query.clone(),
            applicable_statutes: applicable_statutes.into_iter().cloned().collect(),
            effects,
            conflicts,
            jurisdiction: self.jurisdiction.clone(),
        }
    }

    /// Find applicable statutes for query
    fn find_applicable_statutes(&self, query: &ReasoningQuery) -> Vec<&Statute> {
        self.statutes
            .iter()
            .filter(|s| self.is_statute_applicable(s, query))
            .collect()
    }

    /// Check if statute is applicable
    fn is_statute_applicable(&self, statute: &Statute, query: &ReasoningQuery) -> bool {
        // Check jurisdiction match
        if let Some(jurisdiction) = &statute.jurisdiction {
            match &query.jurisdiction {
                Some(ReasoningJurisdiction::Federal) => {
                    if !jurisdiction.contains("FED") {
                        return false;
                    }
                }
                Some(ReasoningJurisdiction::Provincial(province)) => {
                    if !jurisdiction.contains(province.abbreviation()) {
                        return false;
                    }
                }
                Some(ReasoningJurisdiction::Combined(province)) => {
                    if !jurisdiction.contains("FED")
                        && !jurisdiction.contains(province.abbreviation())
                    {
                        return false;
                    }
                }
                None => {}
            }
        }

        // Check legal area match
        if let Some(area) = &query.legal_area {
            let area_lower = area.to_lowercase();
            if !statute.title.to_lowercase().contains(&area_lower)
                && !statute
                    .effect
                    .description
                    .to_lowercase()
                    .contains(&area_lower)
            {
                return false;
            }
        }

        true
    }

    /// Determine effects from applicable statutes
    fn determine_effects(&self, statutes: &[&Statute], _query: &ReasoningQuery) -> Vec<Effect> {
        statutes.iter().map(|s| s.effect.clone()).collect()
    }

    /// Identify conflicts between statutes
    fn identify_conflicts(&self, statutes: &[&Statute]) -> Vec<StatuteConflict> {
        let mut conflicts = Vec::new();

        for (i, s1) in statutes.iter().enumerate() {
            for s2 in statutes.iter().skip(i + 1) {
                if let Some(conflict) = self.check_conflict(s1, s2) {
                    conflicts.push(conflict);
                }
            }
        }

        conflicts
    }

    /// Check for conflict between two statutes
    fn check_conflict(&self, s1: &Statute, s2: &Statute) -> Option<StatuteConflict> {
        // Check for federal-provincial conflict
        let j1 = s1.jurisdiction.as_deref().unwrap_or("");
        let j2 = s2.jurisdiction.as_deref().unwrap_or("");

        if j1.contains("FED")
            && !j2.contains("FED")
            && self.effects_conflict(&s1.effect, &s2.effect)
        {
            return Some(StatuteConflict {
                statute1: s1.id.clone(),
                statute2: s2.id.clone(),
                conflict_type: ConflictType::FederalProvincial,
                resolution: "Federal paramountcy doctrine may apply".to_string(),
            });
        }

        // Check for prohibition vs grant conflict
        if s1.effect.effect_type == EffectType::Prohibition
            && s2.effect.effect_type == EffectType::Grant
        {
            return Some(StatuteConflict {
                statute1: s1.id.clone(),
                statute2: s2.id.clone(),
                conflict_type: ConflictType::ProhibitionGrant,
                resolution: "Prohibition generally prevails over grant".to_string(),
            });
        }

        None
    }

    /// Check if effects conflict
    fn effects_conflict(&self, e1: &Effect, e2: &Effect) -> bool {
        matches!(
            (&e1.effect_type, &e2.effect_type),
            (EffectType::Grant, EffectType::Prohibition)
                | (EffectType::Prohibition, EffectType::Grant)
        )
    }
}

/// Reasoning query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningQuery {
    /// Description of legal question
    pub question: String,
    /// Specific legal area (optional)
    pub legal_area: Option<String>,
    /// Jurisdiction constraint (optional)
    pub jurisdiction: Option<ReasoningJurisdiction>,
    /// Relevant facts
    pub facts: Vec<String>,
}

/// Reasoning result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningResult {
    /// Original query
    pub query: ReasoningQuery,
    /// Applicable statutes
    pub applicable_statutes: Vec<Statute>,
    /// Effects determined
    pub effects: Vec<Effect>,
    /// Conflicts identified
    pub conflicts: Vec<StatuteConflict>,
    /// Jurisdiction used
    pub jurisdiction: ReasoningJurisdiction,
}

/// Conflict between statutes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteConflict {
    /// First statute
    pub statute1: String,
    /// Second statute
    pub statute2: String,
    /// Type of conflict
    pub conflict_type: ConflictType,
    /// Resolution guidance
    pub resolution: String,
}

/// Type of conflict
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictType {
    /// Federal-provincial conflict
    FederalProvincial,
    /// Prohibition vs grant
    ProhibitionGrant,
    /// Temporal (newer vs older)
    Temporal,
    /// Specific vs general
    SpecificGeneral,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_engine() {
        let engine = CanadianReasoningEngine::new(ReasoningJurisdiction::Federal);
        assert!(engine.statutes().is_empty());
    }

    #[test]
    fn test_load_federal_statutes() {
        let mut engine = CanadianReasoningEngine::new(ReasoningJurisdiction::Federal);
        engine.load_federal_statutes();
        assert!(!engine.statutes().is_empty());
    }

    #[test]
    fn test_load_provincial_statutes() {
        let mut engine =
            CanadianReasoningEngine::new(ReasoningJurisdiction::Provincial(Province::Ontario));
        engine.load_provincial_statutes(&Province::Ontario);
        assert!(!engine.statutes().is_empty());
    }

    #[test]
    fn test_load_combined() {
        let mut engine = CanadianReasoningEngine::new(ReasoningJurisdiction::Combined(
            Province::BritishColumbia,
        ));
        engine.load_all();
        assert!(engine.statutes().len() > 10);
    }

    #[test]
    fn test_find_by_keyword() {
        let mut engine = CanadianReasoningEngine::new(ReasoningJurisdiction::Federal);
        engine.load_federal_statutes();
        let results = engine.find_by_keyword("Charter");
        assert!(!results.is_empty());
    }

    #[test]
    fn test_reasoning_query() {
        let mut engine =
            CanadianReasoningEngine::new(ReasoningJurisdiction::Combined(Province::Ontario));
        engine.load_all();

        let query = ReasoningQuery {
            question: "Is the contract enforceable?".to_string(),
            legal_area: Some("contract".to_string()),
            jurisdiction: None,
            facts: vec!["Parties exchanged consideration".to_string()],
        };

        let result = engine.reason(&query);
        assert!(!result.applicable_statutes.is_empty());
    }
}
