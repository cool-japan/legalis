//! Canada Constitutional Law - Charter Analysis
//!
//! Analyzers for Charter of Rights and Freedoms claims and s.1 justification.

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use super::types::{
    CharterRight, ConstitutionalCase, MinimalImpairment, OakesTest, PressAndSubstantial,
    ProportionalityAnalysis, ProportionalityStrictoSensu, RationalConnection,
};

// ============================================================================
// Charter Claim Analysis
// ============================================================================

/// Facts for analyzing a Charter claim
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharterClaimFacts {
    /// The claimant (individual, corporation, etc.)
    pub claimant: String,
    /// Charter right allegedly infringed
    pub right_claimed: CharterRight,
    /// Government action challenged
    pub government_action: GovernmentAction,
    /// How the right was allegedly infringed
    pub alleged_infringement: String,
    /// Evidence of infringement
    pub evidence: Vec<String>,
    /// Whether government is asserting s.1 justification
    pub section_1_asserted: bool,
}

/// Type of government action challenged
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GovernmentAction {
    /// Legislation (federal or provincial)
    Legislation { name: String, section: String },
    /// Regulatory action
    Regulation { name: String },
    /// Administrative decision
    AdministrativeDecision { body: String, decision: String },
    /// Police action
    PoliceAction { description: String },
    /// Prosecution
    Prosecution { offence: String },
    /// Policy or guideline
    Policy { description: String },
}

/// Result of Charter claim analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharterClaimResult {
    /// Whether the right applies to the claimant
    pub right_applies: bool,
    /// Whether there is an infringement
    pub infringement_found: bool,
    /// Analysis of the infringement
    pub infringement_analysis: String,
    /// Whether justified under s.1 (if infringement found)
    pub section_1_justified: Option<bool>,
    /// Oakes test analysis (if s.1 asserted)
    pub oakes_analysis: Option<OakesTest>,
    /// Remedy if successful
    pub remedy: Option<CharterRemedy>,
    /// Key cases cited
    pub key_cases: Vec<ConstitutionalCase>,
    /// Overall reasoning
    pub reasoning: String,
}

/// Charter remedy
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CharterRemedy {
    /// Declaration of invalidity (s.52)
    DeclarationOfInvalidity,
    /// Suspended declaration of invalidity
    SuspendedDeclaration { months: u32 },
    /// Reading in
    ReadingIn { addition: String },
    /// Reading down
    ReadingDown { interpretation: String },
    /// Severance
    Severance { removed: String },
    /// Exclusion of evidence (s.24(2))
    EvidenceExclusion,
    /// Stay of proceedings
    StayOfProceedings,
    /// Damages (s.24(1))
    Damages,
    /// Other just and appropriate remedy (s.24(1))
    OtherRemedy { description: String },
}

// ============================================================================
// Charter Analyzer
// ============================================================================

/// Analyzer for Charter claims
pub struct CharterAnalyzer;

impl CharterAnalyzer {
    /// Analyze a Charter claim
    pub fn analyze(facts: &CharterClaimFacts) -> CharterClaimResult {
        let mut key_cases = Vec::new();

        // Check if right applies
        let right_applies = Self::check_right_applies(facts);

        // Analyze infringement
        let (infringement_found, infringement_analysis) = if right_applies {
            Self::analyze_infringement(facts)
        } else {
            (
                false,
                "Right does not apply to claimant or situation".to_string(),
            )
        };

        // Analyze s.1 justification if infringement found and s.1 asserted
        let (section_1_justified, oakes_analysis) =
            if infringement_found && facts.section_1_asserted {
                key_cases.push(ConstitutionalCase::oakes());
                let oakes = Self::analyze_section_1(facts);
                let justified = oakes.pressing_objective.is_pressing
                    && oakes.proportionality.rational_connection.connected
                    && oakes.proportionality.minimal_impairment.is_minimal
                    && oakes
                        .proportionality
                        .proportionality_stricto_sensu
                        .proportionate;
                (Some(justified), Some(oakes))
            } else {
                (None, None)
            };

        // Determine remedy
        let remedy = if infringement_found && section_1_justified != Some(true) {
            Some(Self::determine_remedy(facts))
        } else {
            None
        };

        // Build reasoning
        let reasoning = Self::build_reasoning(
            &facts.right_claimed,
            right_applies,
            infringement_found,
            section_1_justified,
        );

        CharterClaimResult {
            right_applies,
            infringement_found,
            infringement_analysis,
            section_1_justified,
            oakes_analysis,
            remedy,
            key_cases,
            reasoning,
        }
    }

    /// Check if the Charter right applies
    fn check_right_applies(facts: &CharterClaimFacts) -> bool {
        // Charter applies to government action (s.32)
        // Some rights only apply to citizens (mobility, voting)
        match &facts.right_claimed {
            CharterRight::RightToVote | CharterRight::MobilityOfCitizens => {
                // Would need to check citizenship - assume applies for analysis
                true
            }
            CharterRight::MinorityLanguageEducation => {
                // Specific requirements for s.23
                true
            }
            _ => {
                // Most Charter rights apply to everyone in Canada
                true
            }
        }
    }

    /// Analyze whether there is an infringement
    fn analyze_infringement(facts: &CharterClaimFacts) -> (bool, String) {
        match &facts.right_claimed {
            CharterRight::LifeLibertySecurityOfPerson => {
                // s.7 analysis: Is there a deprivation? Is it in accordance with principles of fundamental justice?
                let analysis = format!(
                    "Section 7 analysis: The government action ({}) must be assessed for \
                     deprivation of life, liberty, or security of the person, and if so, \
                     whether it accords with principles of fundamental justice.",
                    facts.alleged_infringement
                );
                (!facts.evidence.is_empty(), analysis)
            }
            CharterRight::FreedomOfExpression => {
                // s.2(b) is given broad scope - purpose or effect that restricts expression
                let analysis = format!(
                    "Section 2(b) analysis: Expression is broadly defined. The question is \
                     whether the government action has the purpose or effect of restricting \
                     expressive activity. Alleged infringement: {}",
                    facts.alleged_infringement
                );
                (!facts.evidence.is_empty(), analysis)
            }
            CharterRight::EqualityRights => {
                // s.15 analysis using substantive equality approach
                let analysis = format!(
                    "Section 15 analysis: (1) Does the law create a distinction based on \
                     enumerated or analogous grounds? (2) Does the distinction create a \
                     disadvantage by perpetuating prejudice or stereotyping? \
                     Alleged infringement: {}",
                    facts.alleged_infringement
                );
                (!facts.evidence.is_empty(), analysis)
            }
            CharterRight::SearchAndSeizure => {
                // s.8 - was there a reasonable expectation of privacy? Was search unreasonable?
                let analysis = format!(
                    "Section 8 analysis: (1) Was there a reasonable expectation of privacy? \
                     (2) If so, was the search or seizure reasonable (authorized by law, law \
                     itself reasonable, manner of execution reasonable)? Alleged: {}",
                    facts.alleged_infringement
                );
                (!facts.evidence.is_empty(), analysis)
            }
            CharterRight::ArbitraryDetention => {
                // s.9 - was detention arbitrary?
                let analysis = format!(
                    "Section 9 analysis: Detention is arbitrary if it is not authorized by \
                     law or if the law itself is arbitrary. Alleged: {}",
                    facts.alleged_infringement
                );
                (!facts.evidence.is_empty(), analysis)
            }
            _ => {
                let analysis = format!(
                    "Analysis of {} ({:?}): Government action must be assessed against the \
                     protected right. Alleged infringement: {}",
                    facts.right_claimed.section(),
                    facts.right_claimed,
                    facts.alleged_infringement
                );
                (!facts.evidence.is_empty(), analysis)
            }
        }
    }

    /// Analyze s.1 justification using Oakes test
    fn analyze_section_1(facts: &CharterClaimFacts) -> OakesTest {
        // This is a framework - actual application requires detailed evidence
        OakesTest {
            pressing_objective: PressAndSubstantial {
                objective: format!(
                    "Objective of {}",
                    match &facts.government_action {
                        GovernmentAction::Legislation { name, .. } => name.clone(),
                        GovernmentAction::Regulation { name } => name.clone(),
                        GovernmentAction::Policy { description } => description.clone(),
                        _ => "government action".to_string(),
                    }
                ),
                is_pressing: true, // Courts usually find objectives pressing
                reasoning: "The objective must relate to concerns that are pressing and \
                    substantial in a free and democratic society. Courts give deference \
                    to legislative objectives."
                    .to_string(),
            },
            proportionality: ProportionalityAnalysis {
                rational_connection: RationalConnection {
                    connected: true,
                    means: "Means adopted by government".to_string(),
                    reasoning: "There must be a rational connection between the objective \
                        and the means chosen. This is not a high threshold."
                        .to_string(),
                },
                minimal_impairment: MinimalImpairment {
                    is_minimal: false, // This is often where laws fail
                    alternatives: vec![
                        "Less restrictive alternative 1".to_string(),
                        "Less restrictive alternative 2".to_string(),
                    ],
                    why_not_alternatives: "Analysis of whether alternatives would achieve \
                        objective as effectively"
                        .to_string(),
                },
                proportionality_stricto_sensu: ProportionalityStrictoSensu {
                    proportionate: true,
                    benefits: vec!["Benefits of the measure".to_string()],
                    deleterious_effects: vec!["Negative effects on Charter right".to_string()],
                    balance: "Overall balance between benefits and deleterious effects".to_string(),
                },
            },
        }
    }

    /// Determine appropriate remedy
    fn determine_remedy(facts: &CharterClaimFacts) -> CharterRemedy {
        match &facts.government_action {
            GovernmentAction::Legislation { .. } | GovernmentAction::Regulation { .. } => {
                // Legislation usually gets suspended declaration
                CharterRemedy::SuspendedDeclaration { months: 12 }
            }
            GovernmentAction::PoliceAction { .. } => {
                // Police action often leads to evidence exclusion
                CharterRemedy::EvidenceExclusion
            }
            GovernmentAction::Prosecution { .. } => {
                // Serious violations may lead to stay
                CharterRemedy::StayOfProceedings
            }
            _ => CharterRemedy::OtherRemedy {
                description: "Just and appropriate remedy".to_string(),
            },
        }
    }

    /// Build overall reasoning
    fn build_reasoning(
        right: &CharterRight,
        applies: bool,
        infringed: bool,
        justified: Option<bool>,
    ) -> String {
        if !applies {
            format!(
                "The Charter right under s.{} does not apply in this situation.",
                right.section()
            )
        } else if !infringed {
            format!(
                "While s.{} applies, no infringement has been established on the evidence.",
                right.section()
            )
        } else {
            match justified {
                Some(true) => format!(
                    "There is an infringement of s.{}, but it is justified under s.1 \
                     as a reasonable limit in a free and democratic society.",
                    right.section()
                ),
                Some(false) => format!(
                    "There is an infringement of s.{} that cannot be justified under s.1. \
                     The law/action is of no force or effect to the extent of the inconsistency.",
                    right.section()
                ),
                None => format!(
                    "There is an infringement of s.{}. Section 1 justification was not asserted \
                     or analyzed.",
                    right.section()
                ),
            }
        }
    }
}

// ============================================================================
// Section 1 Analyzer
// ============================================================================

/// Dedicated analyzer for s.1 Oakes test
pub struct OakesAnalyzer;

impl OakesAnalyzer {
    /// Analyze pressing and substantial objective
    pub fn analyze_objective(objective: &str, evidence: &[String]) -> PressAndSubstantial {
        let is_pressing = !objective.is_empty() && !evidence.is_empty();

        PressAndSubstantial {
            objective: objective.to_string(),
            is_pressing,
            reasoning: if is_pressing {
                format!(
                    "The objective '{}' relates to pressing and substantial concerns. \
                     Evidence supports the importance of this objective.",
                    objective
                )
            } else {
                "Insufficient evidence of pressing and substantial objective.".to_string()
            },
        }
    }

    /// Analyze rational connection
    pub fn analyze_rational_connection(
        objective: &str,
        means: &str,
        connection_evidence: &[String],
    ) -> RationalConnection {
        let connected = !connection_evidence.is_empty();

        RationalConnection {
            connected,
            means: means.to_string(),
            reasoning: if connected {
                format!(
                    "There is a rational connection between the means ('{}') and the \
                     objective ('{}'). The measure is rationally designed to achieve \
                     the objective.",
                    means, objective
                )
            } else {
                "No rational connection established between means and objective.".to_string()
            },
        }
    }

    /// Analyze minimal impairment
    pub fn analyze_minimal_impairment(
        alternatives: &[String],
        why_chosen_means: &str,
    ) -> MinimalImpairment {
        let is_minimal = !alternatives.is_empty() && !why_chosen_means.is_empty();

        MinimalImpairment {
            is_minimal,
            alternatives: alternatives.to_vec(),
            why_not_alternatives: why_chosen_means.to_string(),
        }
    }

    /// Analyze proportionality stricto sensu
    pub fn analyze_proportionality(
        benefits: &[String],
        deleterious_effects: &[String],
    ) -> ProportionalityStrictoSensu {
        // Simple heuristic: proportionate if benefits outweigh effects
        let proportionate = benefits.len() >= deleterious_effects.len();

        ProportionalityStrictoSensu {
            proportionate,
            benefits: benefits.to_vec(),
            deleterious_effects: deleterious_effects.to_vec(),
            balance: if proportionate {
                "The salutary effects of the measure outweigh its deleterious effects.".to_string()
            } else {
                "The deleterious effects on Charter rights outweigh the benefits.".to_string()
            },
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_charter_claim_analysis() {
        let facts = CharterClaimFacts {
            claimant: "John Doe".to_string(),
            right_claimed: CharterRight::FreedomOfExpression,
            government_action: GovernmentAction::Legislation {
                name: "Advertising Restrictions Act".to_string(),
                section: "s.5".to_string(),
            },
            alleged_infringement: "Prohibition on commercial speech".to_string(),
            evidence: vec!["Advertising banned by s.5".to_string()],
            section_1_asserted: true,
        };

        let result = CharterAnalyzer::analyze(&facts);
        assert!(result.right_applies);
        assert!(result.infringement_found);
        assert!(result.oakes_analysis.is_some());
    }

    #[test]
    fn test_oakes_objective() {
        let objective = OakesAnalyzer::analyze_objective(
            "Protecting public health",
            &["Studies showing health risks".to_string()],
        );
        assert!(objective.is_pressing);
    }

    #[test]
    fn test_charter_remedy() {
        let facts = CharterClaimFacts {
            claimant: "Jane Doe".to_string(),
            right_claimed: CharterRight::SearchAndSeizure,
            government_action: GovernmentAction::PoliceAction {
                description: "Warrantless search".to_string(),
            },
            alleged_infringement: "Searched without warrant".to_string(),
            evidence: vec!["No warrant obtained".to_string()],
            section_1_asserted: false,
        };

        let result = CharterAnalyzer::analyze(&facts);
        assert!(matches!(
            result.remedy,
            Some(CharterRemedy::EvidenceExclusion)
        ));
    }
}
