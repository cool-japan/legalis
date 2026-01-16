//! Canada Tort Law - Defamation
//!
//! Analyzers for defamation claims (libel and slander).

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use super::types::{DefamationDefence, DefamationType, ResponsibleCommunicationFactors, TortCase};

// ============================================================================
// Defamation Analysis
// ============================================================================

/// Facts for defamation analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefamationFacts {
    /// Type of defamation
    pub defamation_type: DefamationType,
    /// The defamatory statement
    pub statement: String,
    /// Medium of publication
    pub medium: PublicationMedium,
    /// Whether statement refers to claimant
    pub refers_to_claimant: bool,
    /// Whether statement published to third parties
    pub published: bool,
    /// Number of people who received
    pub reach: PublicationReach,
    /// Whether statement true
    pub statement_true: bool,
    /// Context of statement
    pub context: StatementContext,
    /// Defences claimed
    pub defences_claimed: Vec<DefamationDefenceClaim>,
}

/// Medium of publication
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublicationMedium {
    /// Newspaper/magazine
    Print,
    /// Television
    Television,
    /// Radio
    Radio,
    /// Internet (website, social media)
    Internet { platform: String },
    /// Email
    Email,
    /// Oral communication
    Oral,
    /// Letter/document
    Written,
}

/// Reach of publication
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublicationReach {
    /// Single person
    Single,
    /// Small group
    SmallGroup,
    /// Large audience
    LargeAudience,
    /// Mass media
    MassMedia,
    /// Internet (potentially unlimited)
    Internet,
}

/// Context of defamatory statement
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatementContext {
    /// News reporting
    NewsReporting,
    /// Political commentary
    Political,
    /// Opinion/editorial
    Opinion,
    /// Social media post
    SocialMedia,
    /// Business communication
    Business,
    /// Personal communication
    Personal,
    /// Court/legal proceedings
    Legal,
    /// Parliamentary/legislative
    Parliamentary,
}

/// Defamation defence claim with supporting facts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefamationDefenceClaim {
    /// Defence type
    pub defence: DefamationDefence,
    /// Supporting facts
    pub supporting_facts: Vec<String>,
    /// Responsible communication factors (if applicable)
    pub responsible_communication: Option<ResponsibleCommunicationFactors>,
}

/// Result of defamation analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefamationResult {
    /// Whether prima facie defamation established
    pub defamation_established: bool,
    /// Elements analysis
    pub elements: DefamationElements,
    /// Applicable defences
    pub applicable_defences: Vec<DefamationDefence>,
    /// Defence analysis
    pub defence_analysis: Vec<DefenceAnalysis>,
    /// Damages assessment
    pub damages: DamagesAssessment,
    /// Key cases
    pub key_cases: Vec<TortCase>,
    /// Reasoning
    pub reasoning: String,
}

/// Analysis of defamation elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefamationElements {
    /// Whether statement defamatory
    pub defamatory_meaning: bool,
    /// Whether refers to claimant
    pub reference_to_claimant: bool,
    /// Whether published
    pub publication: bool,
}

/// Analysis of specific defence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefenceAnalysis {
    /// Defence type
    pub defence: DefamationDefence,
    /// Whether defence succeeds
    pub succeeds: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Assessment of damages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamagesAssessment {
    /// Type of damages available
    pub damages_type: DamagesType,
    /// Whether aggravated damages available
    pub aggravated_available: bool,
    /// Whether punitive damages available
    pub punitive_available: bool,
    /// Factors affecting quantum
    pub quantum_factors: Vec<String>,
}

/// Type of defamation damages
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DamagesType {
    /// General damages (presumed in libel)
    General,
    /// Special damages (must prove specific loss)
    Special,
    /// Nominal damages
    Nominal,
}

// ============================================================================
// Defamation Analyzer
// ============================================================================

/// Analyzer for defamation claims
pub struct DefamationAnalyzer;

impl DefamationAnalyzer {
    /// Analyze defamation claim
    pub fn analyze(facts: &DefamationFacts) -> DefamationResult {
        let mut key_cases = Vec::new();

        // Analyze elements
        let elements = Self::analyze_elements(facts);

        // Prima facie case
        let defamation_established =
            elements.defamatory_meaning && elements.reference_to_claimant && elements.publication;

        // Analyze defences
        let (applicable_defences, defence_analysis) = Self::analyze_defences(facts, &mut key_cases);

        // Assess damages
        let damages = Self::assess_damages(facts, &applicable_defences);

        let reasoning = Self::build_reasoning(&elements, &applicable_defences);

        DefamationResult {
            defamation_established,
            elements,
            applicable_defences,
            defence_analysis,
            damages,
            key_cases,
            reasoning,
        }
    }

    /// Analyze defamation elements
    fn analyze_elements(facts: &DefamationFacts) -> DefamationElements {
        DefamationElements {
            defamatory_meaning: Self::is_defamatory(&facts.statement),
            reference_to_claimant: facts.refers_to_claimant,
            publication: facts.published,
        }
    }

    /// Check if statement is defamatory
    fn is_defamatory(statement: &str) -> bool {
        // Would lower reputation in eyes of right-thinking members of society
        !statement.is_empty()
    }

    /// Analyze defences
    fn analyze_defences(
        facts: &DefamationFacts,
        key_cases: &mut Vec<TortCase>,
    ) -> (Vec<DefamationDefence>, Vec<DefenceAnalysis>) {
        let mut applicable = Vec::new();
        let mut analyses = Vec::new();

        for claim in &facts.defences_claimed {
            let (succeeds, reasoning) = Self::analyze_defence(&claim.defence, facts, key_cases);

            analyses.push(DefenceAnalysis {
                defence: claim.defence.clone(),
                succeeds,
                reasoning,
            });

            if succeeds {
                applicable.push(claim.defence.clone());
            }
        }

        // Truth is always a defence if established
        if facts.statement_true && !applicable.contains(&DefamationDefence::Truth) {
            applicable.push(DefamationDefence::Truth);
            analyses.push(DefenceAnalysis {
                defence: DefamationDefence::Truth,
                succeeds: true,
                reasoning: "Truth (justification) is a complete defence.".to_string(),
            });
        }

        (applicable, analyses)
    }

    /// Analyze specific defence
    fn analyze_defence(
        defence: &DefamationDefence,
        facts: &DefamationFacts,
        key_cases: &mut Vec<TortCase>,
    ) -> (bool, String) {
        match defence {
            DefamationDefence::Truth => {
                let succeeds = facts.statement_true;
                (
                    succeeds,
                    if succeeds {
                        "Truth (justification) established - complete defence.".to_string()
                    } else {
                        "Truth not established.".to_string()
                    },
                )
            }
            DefamationDefence::AbsolutePrivilege => {
                let succeeds = matches!(
                    facts.context,
                    StatementContext::Legal | StatementContext::Parliamentary
                );
                (
                    succeeds,
                    if succeeds {
                        "Absolute privilege applies (judicial/parliamentary proceedings)."
                            .to_string()
                    } else {
                        "Absolute privilege does not apply to this context.".to_string()
                    },
                )
            }
            DefamationDefence::QualifiedPrivilege => {
                let succeeds = Self::check_qualified_privilege(facts);
                (
                    succeeds,
                    if succeeds {
                        "Qualified privilege applies - duty/interest to communicate.".to_string()
                    } else {
                        "Qualified privilege not established.".to_string()
                    },
                )
            }
            DefamationDefence::FairComment => {
                let succeeds = Self::check_fair_comment(facts);
                (
                    succeeds,
                    if succeeds {
                        "Fair comment defence established - honest opinion on public matter."
                            .to_string()
                    } else {
                        "Fair comment requirements not met.".to_string()
                    },
                )
            }
            DefamationDefence::ResponsibleCommunication => {
                key_cases.push(TortCase::grant_v_torstar());
                let succeeds = Self::check_responsible_communication(facts);
                (
                    succeeds,
                    if succeeds {
                        "Responsible communication defence applies (Grant v Torstar).".to_string()
                    } else {
                        "Responsible communication requirements not met.".to_string()
                    },
                )
            }
            DefamationDefence::InnocentDissemination => {
                let succeeds = Self::check_innocent_dissemination(facts);
                (
                    succeeds,
                    if succeeds {
                        "Innocent dissemination defence applies.".to_string()
                    } else {
                        "Innocent dissemination requirements not met.".to_string()
                    },
                )
            }
            DefamationDefence::Consent => {
                // Need specific evidence of consent
                (
                    false,
                    "Consent must be specifically established.".to_string(),
                )
            }
            DefamationDefence::LimitationPeriod => {
                // Would need specific dates
                (
                    false,
                    "Limitation period defence requires date analysis.".to_string(),
                )
            }
        }
    }

    /// Check qualified privilege
    fn check_qualified_privilege(facts: &DefamationFacts) -> bool {
        matches!(
            facts.context,
            StatementContext::Business | StatementContext::Legal
        )
    }

    /// Check fair comment
    fn check_fair_comment(facts: &DefamationFacts) -> bool {
        matches!(
            facts.context,
            StatementContext::Opinion | StatementContext::Political
        )
    }

    /// Check responsible communication (Grant v Torstar)
    fn check_responsible_communication(facts: &DefamationFacts) -> bool {
        // Need to check Grant v Torstar factors
        for claim in &facts.defences_claimed {
            if matches!(claim.defence, DefamationDefence::ResponsibleCommunication) {
                if let Some(factors) = &claim.responsible_communication {
                    return factors.public_interest && factors.responsible_journalism;
                }
            }
        }
        false
    }

    /// Check innocent dissemination
    fn check_innocent_dissemination(facts: &DefamationFacts) -> bool {
        // Applies to subordinate distributors who didn't know
        matches!(
            facts.medium,
            PublicationMedium::Internet { .. } | PublicationMedium::Print
        )
    }

    /// Assess damages
    fn assess_damages(
        facts: &DefamationFacts,
        defences: &[DefamationDefence],
    ) -> DamagesAssessment {
        // If complete defence, no damages
        if defences.contains(&DefamationDefence::Truth)
            || defences.contains(&DefamationDefence::AbsolutePrivilege)
        {
            return DamagesAssessment {
                damages_type: DamagesType::Nominal,
                aggravated_available: false,
                punitive_available: false,
                quantum_factors: vec!["Complete defence applies".to_string()],
            };
        }

        let damages_type = match facts.defamation_type {
            DefamationType::Libel | DefamationType::OnlineDefamation => DamagesType::General,
            DefamationType::Slander => DamagesType::Special,
        };

        let mut quantum_factors = Vec::new();

        // Factors affecting quantum
        match facts.reach {
            PublicationReach::MassMedia | PublicationReach::Internet => {
                quantum_factors.push("Wide publication increases damages".to_string());
            }
            PublicationReach::Single => {
                quantum_factors.push("Limited publication reduces damages".to_string());
            }
            _ => {}
        }

        DamagesAssessment {
            damages_type,
            aggravated_available: matches!(
                facts.context,
                StatementContext::Personal | StatementContext::SocialMedia
            ),
            punitive_available: matches!(facts.medium, PublicationMedium::Internet { .. }),
            quantum_factors,
        }
    }

    /// Build reasoning
    fn build_reasoning(elements: &DefamationElements, defences: &[DefamationDefence]) -> String {
        let mut parts = Vec::new();

        if elements.defamatory_meaning && elements.reference_to_claimant && elements.publication {
            parts.push("Prima facie defamation established.".to_string());
        } else {
            let mut missing = Vec::new();
            if !elements.defamatory_meaning {
                missing.push("defamatory meaning");
            }
            if !elements.reference_to_claimant {
                missing.push("reference to claimant");
            }
            if !elements.publication {
                missing.push("publication");
            }
            parts.push(format!(
                "Prima facie defamation not established. Missing: {}.",
                missing.join(", ")
            ));
        }

        if !defences.is_empty() {
            parts.push(format!("Available defences: {:?}.", defences));
        }

        parts.join(" ")
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prima_facie_defamation() {
        let facts = DefamationFacts {
            defamation_type: DefamationType::Libel,
            statement: "X committed fraud".to_string(),
            medium: PublicationMedium::Print,
            refers_to_claimant: true,
            published: true,
            reach: PublicationReach::LargeAudience,
            statement_true: false,
            context: StatementContext::NewsReporting,
            defences_claimed: vec![],
        };

        let result = DefamationAnalyzer::analyze(&facts);
        assert!(result.defamation_established);
        assert!(result.elements.defamatory_meaning);
    }

    #[test]
    fn test_truth_defence() {
        let facts = DefamationFacts {
            defamation_type: DefamationType::Libel,
            statement: "X was convicted of fraud".to_string(),
            medium: PublicationMedium::Print,
            refers_to_claimant: true,
            published: true,
            reach: PublicationReach::LargeAudience,
            statement_true: true,
            context: StatementContext::NewsReporting,
            defences_claimed: vec![DefamationDefenceClaim {
                defence: DefamationDefence::Truth,
                supporting_facts: vec!["Court records".to_string()],
                responsible_communication: None,
            }],
        };

        let result = DefamationAnalyzer::analyze(&facts);
        assert!(
            result
                .applicable_defences
                .contains(&DefamationDefence::Truth)
        );
    }

    #[test]
    fn test_responsible_communication() {
        let facts = DefamationFacts {
            defamation_type: DefamationType::Libel,
            statement: "Politician allegedly involved in scandal".to_string(),
            medium: PublicationMedium::Print,
            refers_to_claimant: true,
            published: true,
            reach: PublicationReach::MassMedia,
            statement_true: false,
            context: StatementContext::NewsReporting,
            defences_claimed: vec![DefamationDefenceClaim {
                defence: DefamationDefence::ResponsibleCommunication,
                supporting_facts: vec!["Multiple sources".to_string()],
                responsible_communication: Some(ResponsibleCommunicationFactors {
                    public_interest: true,
                    seriousness: "High".to_string(),
                    urgency: true,
                    source_reliability: "Credible".to_string(),
                    claimant_side_sought: true,
                    inclusion_justifiable: true,
                    responsible_journalism: true,
                }),
            }],
        };

        let result = DefamationAnalyzer::analyze(&facts);
        assert!(
            result
                .applicable_defences
                .contains(&DefamationDefence::ResponsibleCommunication)
        );
    }

    #[test]
    fn test_absolute_privilege() {
        let facts = DefamationFacts {
            defamation_type: DefamationType::Slander,
            statement: "Witness testimony".to_string(),
            medium: PublicationMedium::Oral,
            refers_to_claimant: true,
            published: true,
            reach: PublicationReach::SmallGroup,
            statement_true: false,
            context: StatementContext::Legal,
            defences_claimed: vec![DefamationDefenceClaim {
                defence: DefamationDefence::AbsolutePrivilege,
                supporting_facts: vec!["Made in court proceedings".to_string()],
                responsible_communication: None,
            }],
        };

        let result = DefamationAnalyzer::analyze(&facts);
        assert!(
            result
                .applicable_defences
                .contains(&DefamationDefence::AbsolutePrivilege)
        );
    }

    #[test]
    fn test_online_defamation() {
        let facts = DefamationFacts {
            defamation_type: DefamationType::OnlineDefamation,
            statement: "Defamatory post".to_string(),
            medium: PublicationMedium::Internet {
                platform: "Twitter".to_string(),
            },
            refers_to_claimant: true,
            published: true,
            reach: PublicationReach::Internet,
            statement_true: false,
            context: StatementContext::SocialMedia,
            defences_claimed: vec![],
        };

        let result = DefamationAnalyzer::analyze(&facts);
        assert!(result.defamation_established);
        assert!(result.damages.punitive_available);
    }
}
