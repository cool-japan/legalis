//! Australian Defamation Law
//!
//! Analysis under the Uniform Defamation Laws adopted by all
//! states and territories.

use serde::{Deserialize, Serialize};

use super::types::{DefamationDefence, DefamationElement, ImputationType};

// ============================================================================
// Defamation Claim Analysis
// ============================================================================

/// Analyzer for defamation claims
pub struct DefamationAnalyzer;

impl DefamationAnalyzer {
    /// Analyze defamation claim
    pub fn analyze(facts: &DefamationFacts) -> DefamationResult {
        // Check preliminary matters
        let preliminary = Self::check_preliminary_matters(facts);
        if !preliminary.can_proceed {
            let reasoning = preliminary.reasoning.clone();
            let serious_harm_met = preliminary.serious_harm_met;
            return DefamationResult {
                defamation_established: false,
                elements_satisfied: Vec::new(),
                defences_available: Vec::new(),
                defence_succeeds: false,
                serious_harm_threshold: serious_harm_met,
                preliminary_issues: Some(preliminary),
                reasoning,
            };
        }

        // Check elements
        let elements = Self::check_elements(facts);
        let all_elements = elements.len() == 3; // Publication, Identification, Defamatory

        // Check defences
        let defences = Self::check_defences(facts);
        let defence_succeeds = !defences.is_empty() && facts.defence_proven;

        let defamation_established = all_elements && !defence_succeeds;

        let reasoning = Self::build_reasoning(facts, &elements, &defences, defamation_established);

        DefamationResult {
            defamation_established,
            elements_satisfied: elements,
            defences_available: defences,
            defence_succeeds,
            serious_harm_threshold: preliminary.serious_harm_met,
            preliminary_issues: Some(preliminary),
            reasoning,
        }
    }

    /// Check preliminary matters
    fn check_preliminary_matters(facts: &DefamationFacts) -> PreliminaryMatters {
        let mut issues = Vec::new();
        let mut can_proceed = true;

        // Corporations cannot sue (s.9)
        if facts.plaintiff_is_corporation && !facts.corporation_can_sue {
            issues.push("Corporations generally cannot sue in defamation (s.9)".to_string());
            can_proceed = false;
        }

        // Government cannot sue
        if facts.plaintiff_is_government_body {
            issues.push("Government bodies cannot sue (Derbyshire principle)".to_string());
            can_proceed = false;
        }

        // Dead persons cannot sue
        if facts.plaintiff_deceased {
            issues.push("No action for defamation of deceased person".to_string());
            can_proceed = false;
        }

        // Serious harm threshold (2021 amendments)
        let serious_harm_met = facts.serious_harm_caused || facts.serious_harm_likely;
        if !serious_harm_met {
            issues.push("Serious harm threshold not met (s.10A)".to_string());
            can_proceed = false;
        }

        // Single publication rule
        if facts.republication_same_material {
            issues.push("Single publication rule may apply".to_string());
        }

        // Limitation period (1 year, extendable to 3)
        if facts.limitation_period_expired {
            issues.push("Limitation period expired".to_string());
            can_proceed = false;
        }

        let reasoning = if can_proceed {
            "Preliminary matters satisfied".to_string()
        } else {
            format!("Claim cannot proceed: {}", issues.join("; "))
        };

        PreliminaryMatters {
            can_proceed,
            serious_harm_met,
            issues,
            reasoning,
        }
    }

    /// Check elements of defamation
    fn check_elements(facts: &DefamationFacts) -> Vec<DefamationElement> {
        let mut elements = Vec::new();

        // Publication
        if facts.matter_published && facts.publication_to_third_party {
            elements.push(DefamationElement::Publication);
        }

        // Identification
        if facts.plaintiff_identified || facts.plaintiff_identifiable {
            elements.push(DefamationElement::Identification);
        }

        // Defamatory meaning
        if facts.matter_defamatory {
            elements.push(DefamationElement::DefamatoryMeaning);
        }

        elements
    }

    /// Check available defences
    fn check_defences(facts: &DefamationFacts) -> Vec<DefamationDefence> {
        let mut defences = Vec::new();

        // Justification (truth)
        if facts.imputations_substantially_true {
            defences.push(DefamationDefence::Justification);
        }

        // Contextual truth
        if facts.contextual_imputations_true && facts.true_imputations_more_serious {
            defences.push(DefamationDefence::ContextualTruth);
        }

        // Absolute privilege
        if facts.parliamentary_proceedings
            || facts.judicial_proceedings
            || facts.executive_proceedings
        {
            defences.push(DefamationDefence::AbsolutePrivilege);
        }

        // Qualified privilege (statutory)
        if facts.matter_of_public_interest && facts.conduct_reasonable {
            defences.push(DefamationDefence::QualifiedPrivilegeStatutory);
        }

        // Qualified privilege (common law)
        if facts.reciprocal_duty_interest && !facts.malice_present {
            defences.push(DefamationDefence::QualifiedPrivilegeCommonLaw);
        }

        // Honest opinion
        if facts.opinion_not_fact
            && facts.relates_to_public_interest
            && facts.factual_basis_indicated
        {
            defences.push(DefamationDefence::HonestOpinion);
        }

        // Public interest
        if facts.matter_of_public_interest
            && facts.belief_publication_in_public_interest
            && facts.conduct_reasonable
        {
            defences.push(DefamationDefence::PublicInterest);
        }

        // Innocent dissemination
        if facts.subordinate_distributor && !facts.knew_defamatory && !facts.ought_to_have_known {
            defences.push(DefamationDefence::InnocentDissemination);
        }

        // Triviality
        if facts.circumstances_trivial && !facts.claimant_likely_to_suffer_harm {
            defences.push(DefamationDefence::Triviality);
        }

        // Scientific/academic peer review
        if facts.scientific_peer_review {
            defences.push(DefamationDefence::ScientificPeerReview);
        }

        defences
    }

    /// Build reasoning
    fn build_reasoning(
        facts: &DefamationFacts,
        elements: &[DefamationElement],
        defences: &[DefamationDefence],
        established: bool,
    ) -> String {
        let mut parts = Vec::new();

        parts.push("Defamation analysis under Uniform Defamation Laws".to_string());

        // Elements
        parts.push("Elements:".to_string());
        if elements.contains(&DefamationElement::Publication) {
            parts.push("- Publication: established".to_string());
        } else {
            parts.push("- Publication: not established".to_string());
        }

        if elements.contains(&DefamationElement::Identification) {
            parts.push("- Identification: established".to_string());
        } else {
            parts.push("- Identification: not established".to_string());
        }

        if elements.contains(&DefamationElement::DefamatoryMeaning) {
            parts.push("- Defamatory meaning: established".to_string());
            if !facts.imputations.is_empty() {
                parts.push(format!("  Imputations: {:?}", facts.imputations));
            }
        } else {
            parts.push("- Defamatory meaning: not established".to_string());
        }

        // Defences
        if !defences.is_empty() {
            parts.push(format!("Available defences: {:?}", defences));

            if defences.contains(&DefamationDefence::Justification) {
                parts.push("Justification (s.25): imputations substantially true".to_string());
            }

            if defences.contains(&DefamationDefence::HonestOpinion) {
                parts.push(
                    "Honest opinion (s.31): opinion on matter of public interest".to_string(),
                );
            }

            if defences.contains(&DefamationDefence::PublicInterest) {
                parts.push("Public interest (s.29A): 2021 amendment defence".to_string());
            }
        }

        // Conclusion
        if established {
            parts.push("Defamation established - claimant entitled to damages".to_string());
        } else if elements.len() == 3 {
            parts.push("Elements established but defence succeeds".to_string());
        } else {
            parts.push("Claim fails - not all elements established".to_string());
        }

        parts.join(". ")
    }
}

/// Facts for defamation analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DefamationFacts {
    // Preliminary matters
    /// Plaintiff is corporation
    pub plaintiff_is_corporation: bool,
    /// Corporation can sue (exceptions)
    pub corporation_can_sue: bool,
    /// Plaintiff is government body
    pub plaintiff_is_government_body: bool,
    /// Plaintiff deceased
    pub plaintiff_deceased: bool,
    /// Serious harm caused
    pub serious_harm_caused: bool,
    /// Serious harm likely
    pub serious_harm_likely: bool,
    /// Republication same material
    pub republication_same_material: bool,
    /// Limitation period expired
    pub limitation_period_expired: bool,

    // Elements
    /// Matter published
    pub matter_published: bool,
    /// Publication to third party
    pub publication_to_third_party: bool,
    /// Plaintiff identified
    pub plaintiff_identified: bool,
    /// Plaintiff identifiable
    pub plaintiff_identifiable: bool,
    /// Matter defamatory
    pub matter_defamatory: bool,
    /// Imputations
    pub imputations: Vec<ImputationType>,

    // Defences
    /// Imputations substantially true
    pub imputations_substantially_true: bool,
    /// Contextual imputations true
    pub contextual_imputations_true: bool,
    /// True imputations more serious
    pub true_imputations_more_serious: bool,
    /// Parliamentary proceedings
    pub parliamentary_proceedings: bool,
    /// Judicial proceedings
    pub judicial_proceedings: bool,
    /// Executive proceedings
    pub executive_proceedings: bool,
    /// Matter of public interest
    pub matter_of_public_interest: bool,
    /// Conduct reasonable
    pub conduct_reasonable: bool,
    /// Reciprocal duty/interest
    pub reciprocal_duty_interest: bool,
    /// Malice present
    pub malice_present: bool,
    /// Opinion not fact
    pub opinion_not_fact: bool,
    /// Relates to public interest
    pub relates_to_public_interest: bool,
    /// Factual basis indicated
    pub factual_basis_indicated: bool,
    /// Belief publication in public interest
    pub belief_publication_in_public_interest: bool,
    /// Subordinate distributor
    pub subordinate_distributor: bool,
    /// Knew defamatory
    pub knew_defamatory: bool,
    /// Ought to have known
    pub ought_to_have_known: bool,
    /// Circumstances trivial
    pub circumstances_trivial: bool,
    /// Claimant likely to suffer harm
    pub claimant_likely_to_suffer_harm: bool,
    /// Scientific peer review
    pub scientific_peer_review: bool,
    /// Defence proven
    pub defence_proven: bool,
}

/// Preliminary matters result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreliminaryMatters {
    /// Can proceed
    pub can_proceed: bool,
    /// Serious harm threshold met
    pub serious_harm_met: bool,
    /// Issues identified
    pub issues: Vec<String>,
    /// Reasoning
    pub reasoning: String,
}

/// Result of defamation analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefamationResult {
    /// Defamation established
    pub defamation_established: bool,
    /// Elements satisfied
    pub elements_satisfied: Vec<DefamationElement>,
    /// Available defences
    pub defences_available: Vec<DefamationDefence>,
    /// Defence succeeds
    pub defence_succeeds: bool,
    /// Serious harm threshold met
    pub serious_harm_threshold: bool,
    /// Preliminary issues
    pub preliminary_issues: Option<PreliminaryMatters>,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Damages
// ============================================================================

/// Defamation damages calculator
pub struct DefamationDamages;

impl DefamationDamages {
    /// Calculate damages range
    pub fn calculate(facts: &DamagesFacts) -> DamagesResult {
        let mut components = Vec::new();
        let mut total_min: f64 = 0.0;
        let mut total_max: f64 = 0.0;

        // Compensatory damages
        if facts.injury_to_reputation {
            let (min, max) = Self::reputation_damages(facts);
            components.push(("Reputation".to_string(), min, max));
            total_min += min;
            total_max += max;
        }

        // Hurt feelings
        if facts.hurt_feelings {
            let (min, max) = Self::hurt_feelings_damages(facts);
            components.push(("Hurt feelings".to_string(), min, max));
            total_min += min;
            total_max += max;
        }

        // Economic loss
        if let Some(loss) = facts.economic_loss {
            components.push(("Economic loss".to_string(), loss, loss));
            total_min += loss;
            total_max += loss;
        }

        // Aggravated damages
        let aggravated = if facts.defendant_conduct_aggravating {
            let (min, max) = Self::aggravated_damages(facts);
            components.push(("Aggravated".to_string(), min, max));
            total_min += min;
            total_max += max;
            true
        } else {
            false
        };

        // Cap (s.35): capped at maximum for non-economic loss in PI claims
        let capped_max = total_max.min(facts.damages_cap.unwrap_or(500_000.0));

        let reasoning = Self::build_reasoning(&components, total_min, capped_max);

        DamagesResult {
            components,
            total_range: (total_min, capped_max),
            aggravated_awarded: aggravated,
            capped: total_max > capped_max,
            reasoning,
        }
    }

    /// Calculate reputation damages
    fn reputation_damages(facts: &DamagesFacts) -> (f64, f64) {
        let base = if facts.widespread_publication {
            100_000.0
        } else {
            20_000.0
        };

        let multiplier = if facts.plaintiff_public_figure {
            1.5
        } else {
            1.0
        };

        (base * 0.5 * multiplier, base * multiplier)
    }

    /// Calculate hurt feelings damages
    fn hurt_feelings_damages(_facts: &DamagesFacts) -> (f64, f64) {
        (10_000.0, 50_000.0)
    }

    /// Calculate aggravated damages
    fn aggravated_damages(facts: &DamagesFacts) -> (f64, f64) {
        let base = if facts.malice_in_publication {
            50_000.0
        } else if facts.failure_to_apologize {
            20_000.0
        } else {
            10_000.0
        };

        (base * 0.5, base)
    }

    /// Build reasoning
    fn build_reasoning(components: &[(String, f64, f64)], min: f64, max: f64) -> String {
        let mut parts = Vec::new();

        parts.push("Defamation damages assessment".to_string());

        for (name, c_min, c_max) in components {
            parts.push(format!("{}: ${:.0} - ${:.0}", name, c_min, c_max));
        }

        parts.push(format!("Total range: ${:.0} - ${:.0}", min, max));

        parts.join(". ")
    }
}

/// Facts for damages calculation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DamagesFacts {
    /// Injury to reputation
    pub injury_to_reputation: bool,
    /// Hurt feelings
    pub hurt_feelings: bool,
    /// Economic loss
    pub economic_loss: Option<f64>,
    /// Widespread publication
    pub widespread_publication: bool,
    /// Plaintiff public figure
    pub plaintiff_public_figure: bool,
    /// Defendant conduct aggravating
    pub defendant_conduct_aggravating: bool,
    /// Malice in publication
    pub malice_in_publication: bool,
    /// Failure to apologize
    pub failure_to_apologize: bool,
    /// Damages cap
    pub damages_cap: Option<f64>,
}

/// Damages result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamagesResult {
    /// Damage components
    pub components: Vec<(String, f64, f64)>,
    /// Total range
    pub total_range: (f64, f64),
    /// Aggravated awarded
    pub aggravated_awarded: bool,
    /// Whether capped
    pub capped: bool,
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
    fn test_defamation_elements() {
        let facts = DefamationFacts {
            matter_published: true,
            publication_to_third_party: true,
            plaintiff_identified: true,
            matter_defamatory: true,
            serious_harm_caused: true,
            ..Default::default()
        };

        let result = DefamationAnalyzer::analyze(&facts);
        assert!(result.defamation_established);
        assert_eq!(result.elements_satisfied.len(), 3);
    }

    #[test]
    fn test_corporation_cannot_sue() {
        let facts = DefamationFacts {
            plaintiff_is_corporation: true,
            corporation_can_sue: false,
            ..Default::default()
        };

        let result = DefamationAnalyzer::analyze(&facts);
        assert!(!result.defamation_established);
        assert!(result.preliminary_issues.is_some_and(|p| !p.can_proceed));
    }

    #[test]
    fn test_justification_defence() {
        let facts = DefamationFacts {
            matter_published: true,
            publication_to_third_party: true,
            plaintiff_identified: true,
            matter_defamatory: true,
            serious_harm_caused: true,
            imputations_substantially_true: true,
            defence_proven: true,
            ..Default::default()
        };

        let result = DefamationAnalyzer::analyze(&facts);
        assert!(!result.defamation_established);
        assert!(
            result
                .defences_available
                .contains(&DefamationDefence::Justification)
        );
        assert!(result.defence_succeeds);
    }

    #[test]
    fn test_honest_opinion_defence() {
        let facts = DefamationFacts {
            matter_published: true,
            publication_to_third_party: true,
            plaintiff_identified: true,
            matter_defamatory: true,
            serious_harm_caused: true,
            opinion_not_fact: true,
            relates_to_public_interest: true,
            factual_basis_indicated: true,
            defence_proven: true,
            ..Default::default()
        };

        let result = DefamationAnalyzer::analyze(&facts);
        assert!(
            result
                .defences_available
                .contains(&DefamationDefence::HonestOpinion)
        );
    }

    #[test]
    fn test_serious_harm_threshold() {
        let facts = DefamationFacts {
            matter_published: true,
            publication_to_third_party: true,
            plaintiff_identified: true,
            matter_defamatory: true,
            serious_harm_caused: false,
            serious_harm_likely: false,
            ..Default::default()
        };

        let result = DefamationAnalyzer::analyze(&facts);
        assert!(!result.defamation_established);
        assert!(!result.serious_harm_threshold);
    }

    #[test]
    fn test_damages_calculation() {
        let facts = DamagesFacts {
            injury_to_reputation: true,
            hurt_feelings: true,
            widespread_publication: true,
            ..Default::default()
        };

        let result = DefamationDamages::calculate(&facts);
        assert!(result.total_range.0 > 0.0);
        assert!(result.total_range.1 > result.total_range.0);
    }
}
