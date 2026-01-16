//! Commonwealth Constitutional Analysis
//!
//! Analysis of Commonwealth legislative power, characterization,
//! and constitutional validity.

use serde::{Deserialize, Serialize};

use super::types::{
    CharacterizationResult, CommonwealthPower, ExpressRight, InconsistencyAnalysis,
    InconsistencyType, MelbourneCorporationAnalysis,
};
use crate::common::StateTerritory;

// ============================================================================
// Characterization Analysis
// ============================================================================

/// Analyzer for characterizing Commonwealth laws
pub struct CharacterizationAnalyzer;

impl CharacterizationAnalyzer {
    /// Analyze whether a law is within Commonwealth power
    pub fn analyze(
        law_id: &str,
        claimed_power: CommonwealthPower,
        facts: &CharacterizationFacts,
    ) -> CharacterizationResult {
        let sufficient_connection = Self::check_sufficient_connection(&claimed_power, facts);
        let within_power =
            sufficient_connection && Self::check_purpose_and_effect(&claimed_power, facts);

        let reasoning =
            Self::build_reasoning(&claimed_power, facts, within_power, sufficient_connection);

        CharacterizationResult {
            law_id: law_id.to_string(),
            claimed_power,
            within_power,
            sufficient_connection,
            reasoning,
        }
    }

    /// Check sufficient connection between law and power
    fn check_sufficient_connection(
        power: &CommonwealthPower,
        facts: &CharacterizationFacts,
    ) -> bool {
        match power {
            CommonwealthPower::TradeAndCommerce => {
                facts.involves_interstate_trade || facts.involves_international_trade
            }
            CommonwealthPower::Corporations => facts.involves_constitutional_corporation,
            CommonwealthPower::ExternalAffairs => {
                facts.implements_treaty || facts.matters_of_international_concern
            }
            CommonwealthPower::Defence => facts.purpose_defence_related,
            CommonwealthPower::Taxation => facts.imposes_tax || facts.tax_related,
            CommonwealthPower::Immigration => facts.involves_aliens || facts.involves_migration,
            CommonwealthPower::Marriage | CommonwealthPower::DivorceMatrimonial => {
                facts.involves_marriage_divorce
            }
            CommonwealthPower::BankruptcyInsolvency => facts.involves_bankruptcy_insolvency,
            CommonwealthPower::IndustrialDisputes => {
                facts.involves_industrial_dispute && facts.dispute_is_interstate
            }
            _ => {
                // Generic connection test for other powers
                facts.has_factual_connection
            }
        }
    }

    /// Check purpose and effect alignment with power
    fn check_purpose_and_effect(power: &CommonwealthPower, facts: &CharacterizationFacts) -> bool {
        // Purpose must align with the head of power
        // Effect must not be merely incidental with a different purpose

        if facts.purpose_is_punitive && !matches!(power, CommonwealthPower::Defence) {
            // Punitive measures need careful scrutiny
            // But Criminal Code equivalent doesn't exist at Cth level - uses other powers
            return facts.punitive_purpose_connected_to_power;
        }

        true
    }

    /// Build reasoning explanation
    fn build_reasoning(
        power: &CommonwealthPower,
        facts: &CharacterizationFacts,
        within_power: bool,
        sufficient_connection: bool,
    ) -> String {
        let mut parts = Vec::new();

        parts.push(format!(
            "Characterization analysis under {}",
            power.section()
        ));

        if !sufficient_connection {
            parts.push("Insufficient connection to constitutional head of power".to_string());
            parts.push(
                "Law does not sufficiently relate to the subject matter of the power".to_string(),
            );
        } else if !within_power {
            parts.push("Connection established but law exceeds power scope".to_string());
        } else {
            parts.push("Law properly characterized under the power".to_string());

            // Add specific reasoning based on power
            match power {
                CommonwealthPower::Corporations => {
                    parts.push(
                        "Constitutional corporation involved (trading/financial/foreign)"
                            .to_string(),
                    );
                    if facts.involves_constitutional_corporation {
                        parts.push(
                            "Per Work Choices (2006): corporations power broadly construed"
                                .to_string(),
                        );
                    }
                }
                CommonwealthPower::ExternalAffairs => {
                    if facts.implements_treaty {
                        parts.push(
                            "Law implements treaty obligations (Tasmanian Dam Case)".to_string(),
                        );
                    }
                    if facts.matters_of_international_concern {
                        parts.push("Matter of international concern".to_string());
                    }
                }
                CommonwealthPower::TradeAndCommerce => {
                    parts.push(
                        "Affects trade and commerce with other countries or among states"
                            .to_string(),
                    );
                }
                _ => {}
            }
        }

        parts.join(". ")
    }
}

/// Facts for characterization analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CharacterizationFacts {
    /// Involves interstate trade
    pub involves_interstate_trade: bool,
    /// Involves international trade
    pub involves_international_trade: bool,
    /// Involves constitutional corporation
    pub involves_constitutional_corporation: bool,
    /// Implements a treaty
    pub implements_treaty: bool,
    /// Matters of international concern
    pub matters_of_international_concern: bool,
    /// Purpose is defence related
    pub purpose_defence_related: bool,
    /// Imposes tax
    pub imposes_tax: bool,
    /// Tax related
    pub tax_related: bool,
    /// Involves aliens
    pub involves_aliens: bool,
    /// Involves migration
    pub involves_migration: bool,
    /// Involves marriage/divorce
    pub involves_marriage_divorce: bool,
    /// Involves bankruptcy/insolvency
    pub involves_bankruptcy_insolvency: bool,
    /// Involves industrial dispute
    pub involves_industrial_dispute: bool,
    /// Dispute is interstate
    pub dispute_is_interstate: bool,
    /// Has factual connection (generic)
    pub has_factual_connection: bool,
    /// Purpose is punitive
    pub purpose_is_punitive: bool,
    /// Punitive purpose connected to power
    pub punitive_purpose_connected_to_power: bool,
}

// ============================================================================
// Section 109 Inconsistency Analysis
// ============================================================================

/// Analyzer for s.109 inconsistency
pub struct InconsistencyAnalyzer;

impl InconsistencyAnalyzer {
    /// Analyze potential inconsistency between Commonwealth and state laws
    pub fn analyze(
        commonwealth_law: &str,
        state_law: &str,
        facts: &InconsistencyFacts,
    ) -> InconsistencyAnalysis {
        let inconsistency_type = Self::determine_inconsistency_type(facts);
        let state_law_inoperative = inconsistency_type.is_some();

        let reasoning = Self::build_reasoning(facts, &inconsistency_type);

        InconsistencyAnalysis {
            commonwealth_law: commonwealth_law.to_string(),
            state_law: state_law.to_string(),
            inconsistency_type,
            state_law_inoperative,
            reasoning,
        }
    }

    /// Determine the type of inconsistency
    fn determine_inconsistency_type(facts: &InconsistencyFacts) -> Option<InconsistencyType> {
        // Direct inconsistency - impossible to comply with both
        if facts.impossible_to_obey_both {
            return Some(InconsistencyType::Direct);
        }

        // Covering the field - Commonwealth law intended to be exhaustive
        if facts.commonwealth_covers_field && facts.state_law_in_same_field {
            return Some(InconsistencyType::CoveringTheField);
        }

        // No inconsistency
        None
    }

    /// Build reasoning
    fn build_reasoning(
        facts: &InconsistencyFacts,
        inconsistency_type: &Option<InconsistencyType>,
    ) -> String {
        let mut parts = Vec::new();

        parts.push("Section 109 inconsistency analysis".to_string());

        match inconsistency_type {
            Some(InconsistencyType::Direct) => {
                parts.push("Direct inconsistency: impossible to obey both laws".to_string());
                parts.push("State law inoperative to extent of inconsistency".to_string());
                parts.push("Per Clyde Engineering v Cowburn (1926) 37 CLR 466".to_string());
            }
            Some(InconsistencyType::CoveringTheField) => {
                parts.push(
                    "Covering the field: Commonwealth law exhaustively deals with subject"
                        .to_string(),
                );
                parts.push("State law inoperative even if not directly contradictory".to_string());
                parts.push("Per Ex parte McLean (1930) 43 CLR 472".to_string());

                if facts.express_savings_clause {
                    parts
                        .push("However, express savings clause may preserve state law".to_string());
                }
            }
            None => {
                parts.push("No inconsistency found".to_string());
                if facts.commonwealth_permits_state_law {
                    parts
                        .push("Commonwealth law expressly permits state law operation".to_string());
                }
                if !facts.state_law_in_same_field {
                    parts.push("State law operates in different field".to_string());
                }
            }
        }

        parts.join(". ")
    }
}

/// Facts for inconsistency analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InconsistencyFacts {
    /// Impossible to comply with both laws
    pub impossible_to_obey_both: bool,
    /// Commonwealth law intended to cover the field
    pub commonwealth_covers_field: bool,
    /// State law in the same field
    pub state_law_in_same_field: bool,
    /// Commonwealth has express savings clause
    pub express_savings_clause: bool,
    /// Commonwealth permits state law operation
    pub commonwealth_permits_state_law: bool,
}

// ============================================================================
// Melbourne Corporation Doctrine
// ============================================================================

/// Analyzer for Melbourne Corporation doctrine
pub struct MelbourneCorporationAnalyzer;

impl MelbourneCorporationAnalyzer {
    /// Analyze whether Commonwealth law violates Melbourne Corporation doctrine
    pub fn analyze(
        commonwealth_law: &str,
        affected_state: StateTerritory,
        facts: &MelbourneCorporationFacts,
    ) -> MelbourneCorporationAnalysis {
        let discriminates = facts.discriminates_against_states;
        let impairs_functioning = facts.impairs_state_governmental_capacity;

        let invalid = discriminates || impairs_functioning;

        let reasoning = Self::build_reasoning(facts, discriminates, impairs_functioning);

        MelbourneCorporationAnalysis {
            commonwealth_law: commonwealth_law.to_string(),
            affected_state,
            discriminates,
            impairs_functioning,
            invalid,
            reasoning,
        }
    }

    /// Build reasoning
    fn build_reasoning(
        facts: &MelbourneCorporationFacts,
        discriminates: bool,
        impairs: bool,
    ) -> String {
        let mut parts = Vec::new();

        parts.push("Melbourne Corporation doctrine analysis".to_string());
        parts.push("Per Melbourne Corporation v Commonwealth (1947) 74 CLR 31".to_string());

        if discriminates {
            parts.push("First limb violated: law discriminates against states".to_string());
            parts.push("Commonwealth cannot single out states for adverse treatment".to_string());
        }

        if impairs {
            parts.push("Second limb violated: law impairs state governmental capacity".to_string());
            parts.push(
                "Per Austin v Commonwealth (2003): substantial impairment required".to_string(),
            );

            if facts.affects_core_governmental_function {
                parts.push("Affects core governmental function of state".to_string());
            }
        }

        if !discriminates && !impairs {
            parts.push("Neither limb violated".to_string());
            parts.push("Commonwealth law valid notwithstanding impact on states".to_string());
        }

        parts.join(". ")
    }
}

/// Facts for Melbourne Corporation analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MelbourneCorporationFacts {
    /// Law discriminates against states
    pub discriminates_against_states: bool,
    /// Law impairs state governmental capacity
    pub impairs_state_governmental_capacity: bool,
    /// Affects core governmental function
    pub affects_core_governmental_function: bool,
}

// ============================================================================
// Express Rights Analysis
// ============================================================================

/// Analyzer for express constitutional rights
pub struct ExpressRightsAnalyzer;

impl ExpressRightsAnalyzer {
    /// Analyze potential violation of express right
    pub fn analyze(
        law_id: &str,
        right: ExpressRight,
        facts: &ExpressRightsFacts,
    ) -> ExpressRightsResult {
        let violated = match &right {
            ExpressRight::TrialByJury => {
                facts.is_commonwealth_indictable_offence && !facts.provides_jury_trial
            }
            ExpressRight::FreeTrade => {
                facts.discriminatory_trade_measure && facts.protectionist_purpose
            }
            ExpressRight::FreedomOfReligion => {
                facts.establishes_religion
                    || facts.imposes_religious_observance
                    || facts.prohibits_free_exercise
            }
            ExpressRight::StateResidentProtection => {
                facts.discriminates_against_other_state_residents
            }
            ExpressRight::JustTerms => facts.acquires_property && !facts.on_just_terms,
        };

        let reasoning = Self::build_reasoning(&right, facts, violated);

        ExpressRightsResult {
            law_id: law_id.to_string(),
            right,
            violated,
            reasoning,
        }
    }

    /// Build reasoning
    fn build_reasoning(right: &ExpressRight, facts: &ExpressRightsFacts, violated: bool) -> String {
        let mut parts = Vec::new();

        parts.push(format!("Express rights analysis: {}", right.section()));

        match right {
            ExpressRight::TrialByJury => {
                if violated {
                    parts.push(
                        "Section 80 violated: no jury trial for indictable Commonwealth offence"
                            .to_string(),
                    );
                    parts.push("Per R v Archdall (1928): s.80 applies to offences punishable on indictment".to_string());
                } else if !facts.is_commonwealth_indictable_offence {
                    parts.push(
                        "Section 80 not engaged: not a Commonwealth indictable offence".to_string(),
                    );
                } else {
                    parts.push("Section 80 satisfied: jury trial provided".to_string());
                }
            }
            ExpressRight::FreeTrade => {
                if violated {
                    parts.push(
                        "Section 92 violated: discriminatory/protectionist trade measure"
                            .to_string(),
                    );
                    parts.push(
                        "Per Cole v Whitfield (1988): s.92 prohibits protectionism".to_string(),
                    );
                } else {
                    parts.push(
                        "Section 92 not violated: no protectionist discrimination".to_string(),
                    );
                }
            }
            ExpressRight::FreedomOfReligion => {
                if violated {
                    parts.push("Section 116 violated (Commonwealth law only)".to_string());
                    if facts.establishes_religion {
                        parts.push("Establishes a religion".to_string());
                    }
                    if facts.prohibits_free_exercise {
                        parts.push("Prohibits free exercise of religion".to_string());
                    }
                } else {
                    parts.push("Section 116 not violated".to_string());
                }
            }
            ExpressRight::StateResidentProtection => {
                if violated {
                    parts.push(
                        "Section 117 violated: discriminates against other state residents"
                            .to_string(),
                    );
                    parts.push("Per Street v Queensland Bar Association (1989)".to_string());
                } else {
                    parts.push("Section 117 not violated".to_string());
                }
            }
            ExpressRight::JustTerms => {
                if violated {
                    parts.push(
                        "Section 51(xxxi) violated: acquisition not on just terms".to_string(),
                    );
                    parts.push("Per Minister for the Army v Dalziel (1944)".to_string());
                } else if !facts.acquires_property {
                    parts.push(
                        "Section 51(xxxi) not engaged: no acquisition of property".to_string(),
                    );
                } else {
                    parts.push("Section 51(xxxi) satisfied: just terms provided".to_string());
                }
            }
        }

        parts.join(". ")
    }
}

/// Facts for express rights analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExpressRightsFacts {
    // s.80 - Trial by jury
    /// Is Commonwealth indictable offence
    pub is_commonwealth_indictable_offence: bool,
    /// Provides jury trial
    pub provides_jury_trial: bool,

    // s.92 - Free trade
    /// Discriminatory trade measure
    pub discriminatory_trade_measure: bool,
    /// Protectionist purpose
    pub protectionist_purpose: bool,

    // s.116 - Freedom of religion
    /// Establishes a religion
    pub establishes_religion: bool,
    /// Imposes religious observance
    pub imposes_religious_observance: bool,
    /// Prohibits free exercise
    pub prohibits_free_exercise: bool,

    // s.117 - State residents
    /// Discriminates against other state residents
    pub discriminates_against_other_state_residents: bool,

    // s.51(xxxi) - Just terms
    /// Acquires property
    pub acquires_property: bool,
    /// Acquisition on just terms
    pub on_just_terms: bool,
}

/// Result of express rights analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressRightsResult {
    /// Law analyzed
    pub law_id: String,
    /// Right analyzed
    pub right: ExpressRight,
    /// Whether right violated
    pub violated: bool,
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
    fn test_characterization_corporations_power() {
        let facts = CharacterizationFacts {
            involves_constitutional_corporation: true,
            has_factual_connection: true,
            ..Default::default()
        };

        let result = CharacterizationAnalyzer::analyze(
            "Fair Work Act 2009",
            CommonwealthPower::Corporations,
            &facts,
        );

        assert!(result.within_power);
        assert!(result.sufficient_connection);
    }

    #[test]
    fn test_characterization_external_affairs_treaty() {
        let facts = CharacterizationFacts {
            implements_treaty: true,
            has_factual_connection: true,
            ..Default::default()
        };

        let result = CharacterizationAnalyzer::analyze(
            "Environment Protection Act",
            CommonwealthPower::ExternalAffairs,
            &facts,
        );

        assert!(result.within_power);
        assert!(result.reasoning.contains("treaty"));
    }

    #[test]
    fn test_inconsistency_direct() {
        let facts = InconsistencyFacts {
            impossible_to_obey_both: true,
            ..Default::default()
        };

        let result = InconsistencyAnalyzer::analyze("Commonwealth Act", "State Act", &facts);

        assert!(result.state_law_inoperative);
        assert_eq!(result.inconsistency_type, Some(InconsistencyType::Direct));
    }

    #[test]
    fn test_inconsistency_covering_field() {
        let facts = InconsistencyFacts {
            commonwealth_covers_field: true,
            state_law_in_same_field: true,
            ..Default::default()
        };

        let result = InconsistencyAnalyzer::analyze(
            "Comprehensive Commonwealth Act",
            "State Supplementary Act",
            &facts,
        );

        assert!(result.state_law_inoperative);
        assert_eq!(
            result.inconsistency_type,
            Some(InconsistencyType::CoveringTheField)
        );
    }

    #[test]
    fn test_melbourne_corporation_discrimination() {
        let facts = MelbourneCorporationFacts {
            discriminates_against_states: true,
            ..Default::default()
        };

        let result = MelbourneCorporationAnalyzer::analyze(
            "Discriminatory Commonwealth Act",
            StateTerritory::NewSouthWales,
            &facts,
        );

        assert!(result.invalid);
        assert!(result.discriminates);
    }

    #[test]
    fn test_express_right_trial_by_jury() {
        let facts = ExpressRightsFacts {
            is_commonwealth_indictable_offence: true,
            provides_jury_trial: false,
            ..Default::default()
        };

        let result = ExpressRightsAnalyzer::analyze(
            "No Jury Offence Act",
            ExpressRight::TrialByJury,
            &facts,
        );

        assert!(result.violated);
    }

    #[test]
    fn test_express_right_just_terms() {
        let facts = ExpressRightsFacts {
            acquires_property: true,
            on_just_terms: false,
            ..Default::default()
        };

        let result =
            ExpressRightsAnalyzer::analyze("Property Seizure Act", ExpressRight::JustTerms, &facts);

        assert!(result.violated);
    }
}
