//! Native Title
//!
//! Implementation of Native Title Act 1993 (Cth) following Mabo v Queensland (No 2).

use serde::{Deserialize, Serialize};

use super::types::NativeTitleRight;

// ============================================================================
// Native Title Analyzer
// ============================================================================

/// Analyzer for native title claims
pub struct NativeTitleAnalyzer;

impl NativeTitleAnalyzer {
    /// Analyze native title claim
    pub fn analyze(facts: &NativeTitleFacts) -> NativeTitleResult {
        let connection_established = Self::check_connection(facts);
        let rights = Self::determine_rights(facts);
        let extinguishment = Self::check_extinguishment(facts);

        let native_title_exists = connection_established && !extinguishment.complete_extinguishment;
        let reasoning = Self::build_reasoning(facts, connection_established, &extinguishment);

        NativeTitleResult {
            native_title_exists,
            connection_established,
            rights_and_interests: rights,
            extinguishment_analysis: extinguishment,
            reasoning,
        }
    }

    /// Check traditional connection (Mabo requirements)
    fn check_connection(facts: &NativeTitleFacts) -> bool {
        // Connection to land/waters under traditional laws and customs
        facts.traditional_laws_acknowledged
            && facts.traditional_customs_observed
            && facts.continuous_connection
            && facts.identifiable_group
    }

    /// Determine rights and interests
    fn determine_rights(facts: &NativeTitleFacts) -> Vec<NativeTitleRight> {
        let mut rights = Vec::new();

        if facts.exclusive_occupation_possible {
            rights.push(NativeTitleRight::ExclusivePossession);
        } else {
            rights.push(NativeTitleRight::NonExclusiveAccess);
        }

        if facts.hunting_rights_claimed {
            rights.push(NativeTitleRight::Hunting);
        }

        if facts.fishing_rights_claimed {
            rights.push(NativeTitleRight::Fishing);
        }

        if facts.gathering_rights_claimed {
            rights.push(NativeTitleRight::Gathering);
        }

        if facts.ceremonial_rights_claimed {
            rights.push(NativeTitleRight::Ceremonial);
        }

        rights
    }

    /// Check extinguishment
    fn check_extinguishment(facts: &NativeTitleFacts) -> ExtinguishmentAnalysis {
        let mut complete = false;
        let mut partial = false;
        let mut reasons = Vec::new();

        // Freehold extinguishes completely
        if facts.freehold_grant {
            complete = true;
            reasons.push("Freehold grant - complete extinguishment".to_string());
        }

        // Pastoral lease - per Wik, may not extinguish
        if facts.pastoral_lease && !complete {
            partial = true;
            reasons.push("Pastoral lease - partial extinguishment per Wik (1996)".to_string());
        }

        // Exclusive possession lease - complete extinguishment
        if facts.exclusive_possession_lease {
            complete = true;
            reasons.push("Exclusive possession lease - complete extinguishment".to_string());
        }

        // Public works - complete extinguishment
        if facts.public_works_constructed {
            partial = true;
            reasons.push("Public works - extinguishment to extent of works".to_string());
        }

        ExtinguishmentAnalysis {
            complete_extinguishment: complete,
            partial_extinguishment: partial,
            extinguishment_reasons: reasons,
        }
    }

    /// Build reasoning
    fn build_reasoning(
        facts: &NativeTitleFacts,
        connection: bool,
        extinguishment: &ExtinguishmentAnalysis,
    ) -> String {
        let mut parts = Vec::new();

        parts.push("Native title analysis (Native Title Act 1993)".to_string());
        parts.push("Per Mabo v Queensland (No 2) (1992) 175 CLR 1".to_string());

        // Connection
        if connection {
            parts.push("Traditional connection to land established".to_string());
            parts.push("Claimants hold rights under traditional laws and customs".to_string());
        } else {
            parts.push("Traditional connection requirements not satisfied".to_string());
            if !facts.continuous_connection {
                parts.push("Connection to land not continuous".to_string());
            }
        }

        // Extinguishment
        if extinguishment.complete_extinguishment {
            parts.push("Native title completely extinguished".to_string());
            for reason in &extinguishment.extinguishment_reasons {
                parts.push(format!("- {}", reason));
            }
        } else if extinguishment.partial_extinguishment {
            parts.push("Native title partially extinguished".to_string());
            parts.push("Some rights may coexist with other interests".to_string());
        } else {
            parts.push("No extinguishment - native title subsists".to_string());
        }

        parts.join(". ")
    }
}

/// Facts for native title analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NativeTitleFacts {
    /// Traditional laws acknowledged
    pub traditional_laws_acknowledged: bool,
    /// Traditional customs observed
    pub traditional_customs_observed: bool,
    /// Continuous connection
    pub continuous_connection: bool,
    /// Identifiable group of claimants
    pub identifiable_group: bool,
    /// Exclusive occupation possible
    pub exclusive_occupation_possible: bool,
    /// Hunting rights claimed
    pub hunting_rights_claimed: bool,
    /// Fishing rights claimed
    pub fishing_rights_claimed: bool,
    /// Gathering rights claimed
    pub gathering_rights_claimed: bool,
    /// Ceremonial rights claimed
    pub ceremonial_rights_claimed: bool,
    /// Freehold grant over area
    pub freehold_grant: bool,
    /// Pastoral lease over area
    pub pastoral_lease: bool,
    /// Exclusive possession lease
    pub exclusive_possession_lease: bool,
    /// Public works constructed
    pub public_works_constructed: bool,
}

/// Extinguishment analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtinguishmentAnalysis {
    /// Complete extinguishment
    pub complete_extinguishment: bool,
    /// Partial extinguishment
    pub partial_extinguishment: bool,
    /// Reasons for extinguishment
    pub extinguishment_reasons: Vec<String>,
}

/// Result of native title analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeTitleResult {
    /// Native title exists
    pub native_title_exists: bool,
    /// Traditional connection established
    pub connection_established: bool,
    /// Rights and interests
    pub rights_and_interests: Vec<NativeTitleRight>,
    /// Extinguishment analysis
    pub extinguishment_analysis: ExtinguishmentAnalysis,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Future Acts Analyzer
// ============================================================================

/// Analyzer for future acts affecting native title
pub struct FutureActsAnalyzer;

impl FutureActsAnalyzer {
    /// Analyze future act validity
    pub fn analyze(facts: &FutureActFacts) -> FutureActResult {
        let valid = Self::check_validity(facts);
        let procedural = Self::check_procedural_rights(facts);
        let reasoning = Self::build_reasoning(facts, valid, &procedural);

        FutureActResult {
            valid_future_act: valid,
            procedural_rights: procedural,
            compensation_payable: valid && facts.affects_native_title,
            reasoning,
        }
    }

    /// Check validity of future act
    fn check_validity(facts: &FutureActFacts) -> bool {
        // Future act valid if:
        // 1. Right to negotiate complied with, or
        // 2. Falls within exception, or
        // 3. Indigenous Land Use Agreement (ILUA)
        facts.right_to_negotiate_complied || facts.ilua_exists || facts.exception_applies
    }

    /// Check procedural rights
    fn check_procedural_rights(facts: &FutureActFacts) -> Vec<ProceduralRight> {
        let mut rights = Vec::new();

        if facts.mining_act {
            rights.push(ProceduralRight::RightToNegotiate);
            rights.push(ProceduralRight::RightToBeConsulted);
        }

        if facts.affects_native_title {
            rights.push(ProceduralRight::RightToCompensation);
        }

        if facts.notification_required {
            rights.push(ProceduralRight::RightToNotification);
        }

        rights
    }

    /// Build reasoning
    fn build_reasoning(
        facts: &FutureActFacts,
        valid: bool,
        procedural: &[ProceduralRight],
    ) -> String {
        let mut parts = Vec::new();

        parts.push("Future act analysis (Part 2 Division 3 Native Title Act)".to_string());

        if valid {
            parts.push("Future act is valid".to_string());
            if facts.ilua_exists {
                parts.push("Covered by Indigenous Land Use Agreement".to_string());
            } else if facts.right_to_negotiate_complied {
                parts.push("Right to negotiate process completed".to_string());
            }
        } else {
            parts.push("Future act validity uncertain".to_string());
            parts.push("Right to negotiate may apply (s.26)".to_string());
        }

        if !procedural.is_empty() {
            parts.push("Procedural rights apply:".to_string());
            for right in procedural {
                parts.push(format!("- {:?}", right));
            }
        }

        parts.join(". ")
    }
}

/// Facts for future act analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FutureActFacts {
    /// Affects native title
    pub affects_native_title: bool,
    /// Mining act
    pub mining_act: bool,
    /// Right to negotiate complied with
    pub right_to_negotiate_complied: bool,
    /// ILUA exists
    pub ilua_exists: bool,
    /// Exception applies
    pub exception_applies: bool,
    /// Notification required
    pub notification_required: bool,
}

/// Procedural right
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProceduralRight {
    /// Right to negotiate
    RightToNegotiate,
    /// Right to be consulted
    RightToBeConsulted,
    /// Right to compensation
    RightToCompensation,
    /// Right to notification
    RightToNotification,
}

/// Result of future act analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FutureActResult {
    /// Valid future act
    pub valid_future_act: bool,
    /// Procedural rights
    pub procedural_rights: Vec<ProceduralRight>,
    /// Compensation payable
    pub compensation_payable: bool,
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
    fn test_native_title_connection() {
        let facts = NativeTitleFacts {
            traditional_laws_acknowledged: true,
            traditional_customs_observed: true,
            continuous_connection: true,
            identifiable_group: true,
            hunting_rights_claimed: true,
            fishing_rights_claimed: true,
            ..Default::default()
        };

        let result = NativeTitleAnalyzer::analyze(&facts);
        assert!(result.native_title_exists);
        assert!(result.connection_established);
    }

    #[test]
    fn test_native_title_freehold_extinguishment() {
        let facts = NativeTitleFacts {
            traditional_laws_acknowledged: true,
            traditional_customs_observed: true,
            continuous_connection: true,
            identifiable_group: true,
            freehold_grant: true,
            ..Default::default()
        };

        let result = NativeTitleAnalyzer::analyze(&facts);
        assert!(!result.native_title_exists);
        assert!(result.extinguishment_analysis.complete_extinguishment);
    }

    #[test]
    fn test_wik_pastoral_lease() {
        let facts = NativeTitleFacts {
            traditional_laws_acknowledged: true,
            traditional_customs_observed: true,
            continuous_connection: true,
            identifiable_group: true,
            pastoral_lease: true,
            ..Default::default()
        };

        let result = NativeTitleAnalyzer::analyze(&facts);
        // Per Wik, pastoral lease doesn't completely extinguish
        assert!(result.native_title_exists);
        assert!(result.extinguishment_analysis.partial_extinguishment);
    }

    #[test]
    fn test_future_act_ilua() {
        let facts = FutureActFacts {
            affects_native_title: true,
            ilua_exists: true,
            ..Default::default()
        };

        let result = FutureActsAnalyzer::analyze(&facts);
        assert!(result.valid_future_act);
    }
}
