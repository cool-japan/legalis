//! Implied Constitutional Rights
//!
//! Analysis of implied constitutional rights, particularly the
//! implied freedom of political communication (Lange v ABC).

use serde::{Deserialize, Serialize};

use super::types::ImpliedRight;

// ============================================================================
// Implied Freedom of Political Communication
// ============================================================================

/// Analyzer for implied freedom of political communication
///
/// Two-stage Lange test:
/// 1. Does the law effectively burden political communication?
/// 2. If so, is it reasonably appropriate and adapted to serve a legitimate end
///    compatible with representative government?
pub struct PoliticalCommunicationAnalyzer;

impl PoliticalCommunicationAnalyzer {
    /// Analyze a law for potential violation of implied freedom
    pub fn analyze(
        law_id: &str,
        facts: &PoliticalCommunicationFacts,
    ) -> PoliticalCommunicationResult {
        // Stage 1: Does law burden political communication?
        let burdens_communication = Self::assess_burden(facts);

        // Stage 2: Is burden justified? (if there is one)
        let justified = if burdens_communication {
            Self::assess_justification(facts)
        } else {
            true // No burden, no need for justification
        };

        let invalid = burdens_communication && !justified;

        let reasoning = Self::build_reasoning(facts, burdens_communication, justified);

        PoliticalCommunicationResult {
            law_id: law_id.to_string(),
            burdens_communication,
            legitimate_purpose: facts.serves_legitimate_end,
            reasonably_appropriate: facts.reasonably_appropriate_and_adapted,
            compatible_with_representative_gov: facts.compatible_with_representative_government,
            justified,
            invalid,
            reasoning,
        }
    }

    /// Stage 1: Assess whether law burdens political communication
    fn assess_burden(facts: &PoliticalCommunicationFacts) -> bool {
        // Law burdens freedom if it limits discussion of governmental matters
        facts.restricts_political_discussion
            || facts.restricts_criticism_of_government
            || facts.restricts_electoral_communication
            || facts.has_chilling_effect
    }

    /// Stage 2: Assess whether burden is justified
    fn assess_justification(facts: &PoliticalCommunicationFacts) -> bool {
        // McCloy v NSW (2015) refined test:
        // 1. Legitimate purpose compatible with representative government
        // 2. Suitable means (rational connection to purpose)
        // 3. Necessary (no obvious, less restrictive alternative)
        // 4. Adequate in balance (not disproportionate)

        if !facts.serves_legitimate_end {
            return false;
        }

        if !facts.compatible_with_representative_government {
            return false;
        }

        if !facts.suitable_means {
            return false;
        }

        if !facts.necessary_means {
            return false;
        }

        facts.adequate_in_balance && facts.reasonably_appropriate_and_adapted
    }

    /// Build reasoning
    fn build_reasoning(
        facts: &PoliticalCommunicationFacts,
        burdens: bool,
        justified: bool,
    ) -> String {
        let mut parts = Vec::new();

        parts.push("Implied freedom of political communication analysis".to_string());
        parts
            .push("Per Lange v Australian Broadcasting Corporation (1997) 189 CLR 520".to_string());
        parts.push("And McCloy v NSW (2015) 257 CLR 178".to_string());

        // Stage 1
        parts.push(format!(
            "Stage 1 - Burden: {}",
            if burdens { "Yes" } else { "No" }
        ));

        if burdens {
            if facts.restricts_political_discussion {
                parts
                    .push("Law restricts discussion of governmental/political matters".to_string());
            }
            if facts.restricts_criticism_of_government {
                parts.push("Law restricts criticism of government".to_string());
            }
            if facts.restricts_electoral_communication {
                parts.push("Law restricts electoral communication".to_string());
            }
            if facts.has_chilling_effect {
                parts.push("Law has chilling effect on political speech".to_string());
            }

            // Stage 2
            parts.push(format!(
                "Stage 2 - Justified: {}",
                if justified { "Yes" } else { "No" }
            ));

            if !facts.serves_legitimate_end {
                parts.push("No legitimate purpose identified".to_string());
            } else {
                parts.push(format!(
                    "Legitimate purpose: {}",
                    facts.stated_purpose.as_deref().unwrap_or("stated")
                ));
            }

            if !facts.compatible_with_representative_government {
                parts.push("Purpose not compatible with representative government".to_string());
            }

            if !facts.suitable_means {
                parts.push("Means not suitable - no rational connection to purpose".to_string());
            }

            if !facts.necessary_means {
                parts.push("Less restrictive alternatives available".to_string());
            }

            if !facts.adequate_in_balance {
                parts.push("Disproportionate burden on political communication".to_string());
            }
        } else {
            parts.push("No burden on political communication - freedom not engaged".to_string());
        }

        parts.join(". ")
    }
}

/// Facts for political communication analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PoliticalCommunicationFacts {
    // Stage 1 - Burden
    /// Restricts political discussion
    pub restricts_political_discussion: bool,
    /// Restricts criticism of government
    pub restricts_criticism_of_government: bool,
    /// Restricts electoral communication
    pub restricts_electoral_communication: bool,
    /// Has chilling effect on political speech
    pub has_chilling_effect: bool,

    // Stage 2 - Justification
    /// Serves a legitimate end
    pub serves_legitimate_end: bool,
    /// Stated purpose
    pub stated_purpose: Option<String>,
    /// Compatible with representative government
    pub compatible_with_representative_government: bool,
    /// Suitable means (rational connection)
    pub suitable_means: bool,
    /// Necessary means (no less restrictive alternative)
    pub necessary_means: bool,
    /// Adequate in balance (proportionate)
    pub adequate_in_balance: bool,
    /// Reasonably appropriate and adapted
    pub reasonably_appropriate_and_adapted: bool,
}

/// Result of political communication analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoliticalCommunicationResult {
    /// Law analyzed
    pub law_id: String,
    /// Whether law burdens political communication
    pub burdens_communication: bool,
    /// Whether serves legitimate purpose
    pub legitimate_purpose: bool,
    /// Whether reasonably appropriate and adapted
    pub reasonably_appropriate: bool,
    /// Whether compatible with representative government
    pub compatible_with_representative_gov: bool,
    /// Whether burden is justified
    pub justified: bool,
    /// Whether law is invalid
    pub invalid: bool,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Implied Right to Vote
// ============================================================================

/// Analyzer for implied right to vote
///
/// Per Roach v Electoral Commissioner (2007): Constitution requires
/// direct choice by the people, implying protection of right to vote.
pub struct RightToVoteAnalyzer;

impl RightToVoteAnalyzer {
    /// Analyze whether law impermissibly restricts voting rights
    pub fn analyze(law_id: &str, facts: &VotingRightsFacts) -> VotingRightsResult {
        let restricts_voting = Self::assess_restriction(facts);
        let justified = if restricts_voting {
            Self::assess_justification(facts)
        } else {
            true
        };

        let invalid = restricts_voting && !justified;

        let reasoning = Self::build_reasoning(facts, restricts_voting, justified);

        VotingRightsResult {
            law_id: law_id.to_string(),
            restricts_voting,
            justified,
            invalid,
            reasoning,
        }
    }

    /// Assess whether law restricts voting
    fn assess_restriction(facts: &VotingRightsFacts) -> bool {
        facts.disenfranchises_citizens || facts.restricts_prisoner_voting_beyond_constitutional
    }

    /// Assess whether restriction is justified
    fn assess_justification(facts: &VotingRightsFacts) -> bool {
        // Per Roach: substantial reasons required
        // Blanket disenfranchisement of all prisoners unconstitutional
        // But some restrictions on prisoners serving lengthy sentences may be valid

        if facts.disenfranchises_citizens && !facts.for_substantial_reason {
            return false;
        }

        if facts.blanket_prisoner_ban {
            return false; // Per Roach
        }

        facts.proportionate_restriction
    }

    /// Build reasoning
    fn build_reasoning(facts: &VotingRightsFacts, restricts: bool, justified: bool) -> String {
        let mut parts = Vec::new();

        parts.push("Implied right to vote analysis".to_string());
        parts.push("Per Roach v Electoral Commissioner (2007) 233 CLR 162".to_string());
        parts.push("Constitution requires 'direct choice by the people' (ss.7, 24)".to_string());

        if restricts {
            parts.push("Law restricts voting rights".to_string());

            if facts.blanket_prisoner_ban {
                parts.push("Blanket ban on prisoner voting - invalid per Roach".to_string());
            }

            if !facts.for_substantial_reason {
                parts.push("No substantial reason for restriction".to_string());
            }

            if !justified {
                parts.push("Restriction not proportionate to legitimate purpose".to_string());
            }
        } else {
            parts.push("No restriction on implied right to vote".to_string());
        }

        parts.join(". ")
    }
}

/// Facts for voting rights analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VotingRightsFacts {
    /// Disenfranchises citizens
    pub disenfranchises_citizens: bool,
    /// Restricts prisoner voting beyond constitutional limits
    pub restricts_prisoner_voting_beyond_constitutional: bool,
    /// Blanket ban on all prisoner voting
    pub blanket_prisoner_ban: bool,
    /// For substantial reason
    pub for_substantial_reason: bool,
    /// Proportionate restriction
    pub proportionate_restriction: bool,
}

/// Result of voting rights analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotingRightsResult {
    /// Law analyzed
    pub law_id: String,
    /// Whether restricts voting
    pub restricts_voting: bool,
    /// Whether justified
    pub justified: bool,
    /// Whether invalid
    pub invalid: bool,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Kable Doctrine
// ============================================================================

/// Analyzer for Kable doctrine (state courts cannot exercise non-judicial functions
/// incompatible with federal jurisdiction)
pub struct KableAnalyzer;

impl KableAnalyzer {
    /// Analyze whether state law violates Kable doctrine
    pub fn analyze(law_id: &str, facts: &KableFacts) -> KableResult {
        // Kable: state law cannot confer functions on state courts that are
        // incompatible with their role as repositories of federal jurisdiction

        let violates_kable = Self::assess_violation(facts);

        let reasoning = Self::build_reasoning(facts, violates_kable);

        KableResult {
            law_id: law_id.to_string(),
            requires_court_function: facts.requires_court_to_exercise_function,
            function_is_non_judicial: facts.function_is_non_judicial,
            incompatible_with_chapter_iii: facts.incompatible_with_chapter_iii,
            invalid: violates_kable,
            reasoning,
        }
    }

    /// Assess whether Kable doctrine violated
    fn assess_violation(facts: &KableFacts) -> bool {
        facts.requires_court_to_exercise_function
            && facts.function_is_non_judicial
            && facts.incompatible_with_chapter_iii
    }

    /// Build reasoning
    fn build_reasoning(facts: &KableFacts, violated: bool) -> String {
        let mut parts = Vec::new();

        parts.push("Kable doctrine analysis".to_string());
        parts.push(
            "Per Kable v Director of Public Prosecutions (NSW) (1996) 189 CLR 51".to_string(),
        );
        parts
            .push("State courts form integrated national judicial system under Ch III".to_string());

        if violated {
            parts.push("Kable doctrine violated".to_string());

            if facts.function_is_non_judicial {
                parts.push("Law confers non-judicial function on state court".to_string());
            }

            if facts.incompatible_with_chapter_iii {
                parts.push("Function incompatible with court's role under Chapter III".to_string());
            }

            if facts.involves_executive_function {
                parts.push("Court required to exercise essentially executive function".to_string());
            }

            if facts.predetermined_outcome {
                parts.push("Outcome effectively predetermined by legislature".to_string());
            }
        } else {
            if !facts.requires_court_to_exercise_function {
                parts.push("No function conferred on court - Kable not engaged".to_string());
            } else if !facts.function_is_non_judicial {
                parts.push("Function is judicial in nature - Kable not violated".to_string());
            } else {
                parts.push("Function compatible with Chapter III - valid".to_string());
            }
        }

        parts.join(". ")
    }
}

/// Facts for Kable analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KableFacts {
    /// Requires court to exercise function
    pub requires_court_to_exercise_function: bool,
    /// Function is non-judicial
    pub function_is_non_judicial: bool,
    /// Incompatible with Chapter III
    pub incompatible_with_chapter_iii: bool,
    /// Involves essentially executive function
    pub involves_executive_function: bool,
    /// Outcome predetermined
    pub predetermined_outcome: bool,
}

/// Result of Kable analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KableResult {
    /// Law analyzed
    pub law_id: String,
    /// Requires court function
    pub requires_court_function: bool,
    /// Function is non-judicial
    pub function_is_non_judicial: bool,
    /// Incompatible with Chapter III
    pub incompatible_with_chapter_iii: bool,
    /// Whether invalid
    pub invalid: bool,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Generic Implied Rights Analysis
// ============================================================================

/// Unified analyzer for implied rights
pub struct ImpliedRightsAnalyzer;

impl ImpliedRightsAnalyzer {
    /// Get the leading case for an implied right
    pub fn leading_case(right: &ImpliedRight) -> &'static str {
        right.leading_case()
    }

    /// Get the constitutional basis for an implied right
    pub fn constitutional_basis(right: &ImpliedRight) -> &'static str {
        right.constitutional_basis()
    }

    /// Check if an implied right is well-established
    pub fn is_established(right: &ImpliedRight) -> bool {
        matches!(
            right,
            ImpliedRight::PoliticalCommunication | ImpliedRight::RightToVote
        )
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_political_communication_no_burden() {
        let facts = PoliticalCommunicationFacts::default();

        let result = PoliticalCommunicationAnalyzer::analyze("Commercial Speech Act", &facts);

        assert!(!result.burdens_communication);
        assert!(!result.invalid);
    }

    #[test]
    fn test_political_communication_burden_justified() {
        let facts = PoliticalCommunicationFacts {
            restricts_political_discussion: true,
            serves_legitimate_end: true,
            stated_purpose: Some("Prevent electoral corruption".to_string()),
            compatible_with_representative_government: true,
            suitable_means: true,
            necessary_means: true,
            adequate_in_balance: true,
            reasonably_appropriate_and_adapted: true,
            ..Default::default()
        };

        let result = PoliticalCommunicationAnalyzer::analyze("Donation Caps Act", &facts);

        assert!(result.burdens_communication);
        assert!(result.justified);
        assert!(!result.invalid);
    }

    #[test]
    fn test_political_communication_burden_not_justified() {
        let facts = PoliticalCommunicationFacts {
            restricts_political_discussion: true,
            restricts_criticism_of_government: true,
            serves_legitimate_end: false,
            ..Default::default()
        };

        let result = PoliticalCommunicationAnalyzer::analyze("Sedition Act", &facts);

        assert!(result.burdens_communication);
        assert!(!result.justified);
        assert!(result.invalid);
    }

    #[test]
    fn test_voting_rights_blanket_ban() {
        let facts = VotingRightsFacts {
            disenfranchises_citizens: true,
            blanket_prisoner_ban: true,
            ..Default::default()
        };

        let result = RightToVoteAnalyzer::analyze("All Prisoners Excluded Act", &facts);

        assert!(result.restricts_voting);
        assert!(!result.justified);
        assert!(result.invalid);
    }

    #[test]
    fn test_kable_violation() {
        let facts = KableFacts {
            requires_court_to_exercise_function: true,
            function_is_non_judicial: true,
            incompatible_with_chapter_iii: true,
            involves_executive_function: true,
            predetermined_outcome: true,
        };

        let result = KableAnalyzer::analyze("Preventive Detention Act", &facts);

        assert!(result.invalid);
        assert!(result.reasoning.contains("Kable doctrine violated"));
    }

    #[test]
    fn test_kable_no_violation() {
        let facts = KableFacts {
            requires_court_to_exercise_function: true,
            function_is_non_judicial: false, // Judicial function
            ..Default::default()
        };

        let result = KableAnalyzer::analyze("Sentencing Act", &facts);

        assert!(!result.invalid);
    }
}
