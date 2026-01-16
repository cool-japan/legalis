//! Australian Reasoning Engine Integration
//!
//! Integration with legalis-core for Australian law analysis.

use legalis_core::Statute;

use crate::common::StateTerritory;

// ============================================================================
// Reasoning Engine
// ============================================================================

/// Australian reasoning engine
pub struct AustralianReasoningEngine {
    /// Loaded statutes
    statutes: Vec<Statute>,
}

impl AustralianReasoningEngine {
    /// Create new Australian reasoning engine
    pub fn new() -> Self {
        Self {
            statutes: Vec::new(),
        }
    }

    /// Load major Australian statutes
    pub fn load_major_statutes(&mut self) {
        // Constitutional
        self.statutes
            .push(crate::constitution::create_constitution_statute());

        // Criminal
        self.statutes
            .push(crate::criminal::create_criminal_code_act());

        // Employment
        self.statutes
            .push(crate::employment::create_fair_work_act());

        // Contract/Consumer
        self.statutes.push(crate::contract::create_acl_statute());
        self.statutes.push(crate::contract::create_cca_statute());

        // Family
        self.statutes.push(crate::family::create_family_law_act());

        // Property
        self.statutes
            .push(crate::property::create_native_title_act());

        // Corporate
        self.statutes
            .push(crate::corporate::create_corporations_act());

        // State-specific
        for state in StateTerritory::all() {
            self.statutes
                .push(crate::tort::create_civil_liability_act(state));
            self.statutes
                .push(crate::tort::create_defamation_act(state));
        }
    }

    /// Get loaded statutes
    pub fn statutes(&self) -> &[Statute] {
        &self.statutes
    }

    /// Get statute by ID
    pub fn get_statute(&self, id: &str) -> Option<&Statute> {
        self.statutes.iter().find(|s| s.id == id)
    }

    /// Find statutes by jurisdiction
    pub fn find_by_jurisdiction(&self, jurisdiction: &str) -> Vec<&Statute> {
        self.statutes
            .iter()
            .filter(|s| {
                s.jurisdiction
                    .as_ref()
                    .map(|j| j.contains(jurisdiction))
                    .unwrap_or(false)
            })
            .collect()
    }

    /// Get all statutes for a state
    pub fn get_state_statutes(&self, state: &StateTerritory) -> Vec<&Statute> {
        let abbr = state.abbreviation();
        self.statutes
            .iter()
            .filter(|s| {
                s.id.contains(abbr)
                    || s.jurisdiction
                        .as_ref()
                        .map(|j| j == "AU" || j.contains(abbr))
                        .unwrap_or(false)
            })
            .collect()
    }
}

impl Default for AustralianReasoningEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Constitutional Verifier
// ============================================================================

/// Constitutional law verifier
pub struct ConstitutionalVerifier;

impl ConstitutionalVerifier {
    /// Verify Commonwealth power to make law
    pub fn verify_power(law_description: &str, power_claimed: &str) -> VerificationResult {
        let valid = Self::check_power(power_claimed);
        let reasoning = Self::build_reasoning(law_description, power_claimed, valid);

        VerificationResult {
            valid,
            power_claimed: power_claimed.to_string(),
            reasoning,
        }
    }

    /// Check if power is valid
    fn check_power(power: &str) -> bool {
        let valid_powers = [
            "s.51(i)",
            "s.51(ii)",
            "s.51(xx)",
            "s.51(xxix)",
            "s.51(xxxi)",
            "corporations",
            "external affairs",
            "taxation",
            "defence",
        ];
        valid_powers
            .iter()
            .any(|p| power.to_lowercase().contains(p))
    }

    /// Build reasoning
    fn build_reasoning(law: &str, power: &str, valid: bool) -> String {
        let mut parts = Vec::new();

        parts.push(format!("Constitutional verification for: {}", law));
        parts.push(format!("Power claimed: {}", power));

        if valid {
            parts.push("Power falls within s.51 Commonwealth heads of power".to_string());
        } else {
            parts.push("Power not clearly within Commonwealth competence".to_string());
            parts.push("May require characterization analysis".to_string());
        }

        parts.join(". ")
    }
}

/// Verification result
#[derive(Debug, Clone)]
pub struct VerificationResult {
    /// Valid under Constitution
    pub valid: bool,
    /// Power claimed
    pub power_claimed: String,
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
    fn test_create_engine() {
        let engine = AustralianReasoningEngine::new();
        assert!(engine.statutes().is_empty());
    }

    #[test]
    fn test_load_statutes() {
        let mut engine = AustralianReasoningEngine::new();
        engine.load_major_statutes();
        assert!(!engine.statutes().is_empty());
    }

    #[test]
    fn test_find_by_jurisdiction() {
        let mut engine = AustralianReasoningEngine::new();
        engine.load_major_statutes();
        let au_statutes = engine.find_by_jurisdiction("AU");
        assert!(!au_statutes.is_empty());
    }

    #[test]
    fn test_verify_corporations_power() {
        let result =
            ConstitutionalVerifier::verify_power("Fair Work Act", "s.51(xx) corporations power");
        assert!(result.valid);
    }

    #[test]
    fn test_verify_invalid_power() {
        let result = ConstitutionalVerifier::verify_power("Some law", "invalid power");
        assert!(!result.valid);
    }
}
