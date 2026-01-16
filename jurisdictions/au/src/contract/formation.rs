//! Contract Formation
//!
//! Analysis of contract formation under Australian law.

use serde::{Deserialize, Serialize};

use super::types::{
    AcceptanceMode, ConsiderationType, FormationElement, OfferType, TermClassification,
    VitiatingFactor,
};

// ============================================================================
// Formation Analyzer
// ============================================================================

/// Analyzer for contract formation
pub struct FormationAnalyzer;

impl FormationAnalyzer {
    /// Analyze contract formation
    pub fn analyze(facts: &FormationFacts) -> FormationResult {
        let mut satisfied_elements = Vec::new();
        let mut missing_elements = Vec::new();
        let mut issues = Vec::new();

        // Check agreement
        if Self::check_agreement(facts) {
            satisfied_elements.push(FormationElement::Agreement);
        } else {
            missing_elements.push(FormationElement::Agreement);
            issues.push("No valid agreement (offer and acceptance)".to_string());
        }

        // Check consideration
        if Self::check_consideration(facts) {
            satisfied_elements.push(FormationElement::Consideration);
        } else {
            missing_elements.push(FormationElement::Consideration);
            issues.push("No valid consideration (and not a deed)".to_string());
        }

        // Check intention
        if Self::check_intention(facts) {
            satisfied_elements.push(FormationElement::IntentionToCreateLegalRelations);
        } else {
            missing_elements.push(FormationElement::IntentionToCreateLegalRelations);
            issues.push("No intention to create legal relations".to_string());
        }

        // Check capacity
        if Self::check_capacity(facts) {
            satisfied_elements.push(FormationElement::Capacity);
        } else {
            missing_elements.push(FormationElement::Capacity);
            issues.push("Lack of capacity to contract".to_string());
        }

        // Check certainty
        if Self::check_certainty(facts) {
            satisfied_elements.push(FormationElement::Certainty);
        } else {
            missing_elements.push(FormationElement::Certainty);
            issues.push("Terms insufficiently certain".to_string());
        }

        // Check legality
        if Self::check_legality(facts) {
            satisfied_elements.push(FormationElement::Legality);
        } else {
            missing_elements.push(FormationElement::Legality);
            issues.push("Contract purpose illegal".to_string());
        }

        let contract_formed = missing_elements.is_empty();
        let reasoning = Self::build_reasoning(facts, &satisfied_elements, &issues);

        FormationResult {
            contract_formed,
            satisfied_elements,
            missing_elements,
            vitiating_factors: facts.vitiating_factors.clone(),
            issues,
            reasoning,
        }
    }

    /// Check agreement (offer and acceptance)
    fn check_agreement(facts: &FormationFacts) -> bool {
        facts.offer_made && facts.acceptance_communicated && !facts.offer_revoked
    }

    /// Check consideration
    fn check_consideration(facts: &FormationFacts) -> bool {
        facts.is_deed || (facts.consideration_present && !facts.past_consideration_only)
    }

    /// Check intention to create legal relations
    fn check_intention(facts: &FormationFacts) -> bool {
        if facts.commercial_context {
            // Commercial: presumed unless rebutted
            !facts.honour_clause && !facts.letters_of_comfort
        } else {
            // Social/domestic: no presumption
            facts.express_intention || facts.evidence_of_intention
        }
    }

    /// Check capacity
    fn check_capacity(facts: &FormationFacts) -> bool {
        !facts.minor_involved || facts.contract_for_necessaries || facts.minor_contract_beneficial
    }

    /// Check certainty
    fn check_certainty(facts: &FormationFacts) -> bool {
        facts.essential_terms_agreed && !facts.agreement_to_agree
    }

    /// Check legality
    fn check_legality(facts: &FormationFacts) -> bool {
        !facts.illegal_purpose && !facts.contrary_to_public_policy
    }

    /// Build reasoning
    fn build_reasoning(
        facts: &FormationFacts,
        satisfied: &[FormationElement],
        issues: &[String],
    ) -> String {
        let mut parts = Vec::new();

        parts.push("Contract formation analysis under Australian law".to_string());

        // Agreement analysis
        if satisfied.contains(&FormationElement::Agreement) {
            match facts.offer_type {
                Some(OfferType::ToTheWorld) => {
                    parts.push("Offer to world (per Carlill v Carbolic Smoke Ball)".to_string());
                }
                Some(OfferType::CounterOffer) => {
                    parts.push("Counter-offer accepted".to_string());
                }
                _ => {
                    parts.push("Valid offer and acceptance".to_string());
                }
            }

            match facts.acceptance_mode {
                Some(AcceptanceMode::Postal) => {
                    parts.push("Postal rule: acceptance effective on posting".to_string());
                }
                Some(AcceptanceMode::Conduct) => {
                    parts.push("Acceptance inferred from conduct".to_string());
                }
                _ => {}
            }
        }

        // Consideration analysis
        if satisfied.contains(&FormationElement::Consideration) {
            if facts.is_deed {
                parts.push("Contract under deed - no consideration required".to_string());
            } else {
                parts.push(
                    "Valid consideration: need not be adequate, but must be sufficient".to_string(),
                );
            }
        }

        // Intention analysis
        if satisfied.contains(&FormationElement::IntentionToCreateLegalRelations) {
            if facts.commercial_context {
                parts.push("Commercial context: intention presumed (per Ermogenous)".to_string());
            } else {
                parts.push("Intention established on evidence".to_string());
            }
        }

        // Issues
        for issue in issues {
            parts.push(format!("Issue: {}", issue));
        }

        parts.join(". ")
    }
}

/// Facts for formation analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormationFacts {
    // Agreement
    /// Offer made
    pub offer_made: bool,
    /// Type of offer
    pub offer_type: Option<OfferType>,
    /// Acceptance communicated
    pub acceptance_communicated: bool,
    /// Mode of acceptance
    pub acceptance_mode: Option<AcceptanceMode>,
    /// Offer revoked before acceptance
    pub offer_revoked: bool,

    // Consideration
    /// Consideration present
    pub consideration_present: bool,
    /// Consideration type
    pub consideration_type: Option<ConsiderationType>,
    /// Only past consideration
    pub past_consideration_only: bool,
    /// Contract is a deed
    pub is_deed: bool,

    // Intention
    /// Commercial context
    pub commercial_context: bool,
    /// Honour clause present
    pub honour_clause: bool,
    /// Letters of comfort
    pub letters_of_comfort: bool,
    /// Express intention stated
    pub express_intention: bool,
    /// Evidence of intention (social/domestic)
    pub evidence_of_intention: bool,

    // Capacity
    /// Minor involved
    pub minor_involved: bool,
    /// Contract for necessaries
    pub contract_for_necessaries: bool,
    /// Minor contract beneficial
    pub minor_contract_beneficial: bool,

    // Certainty
    /// Essential terms agreed
    pub essential_terms_agreed: bool,
    /// Agreement to agree
    pub agreement_to_agree: bool,

    // Legality
    /// Illegal purpose
    pub illegal_purpose: bool,
    /// Contrary to public policy
    pub contrary_to_public_policy: bool,

    // Vitiating factors
    /// Vitiating factors present
    pub vitiating_factors: Vec<VitiatingFactor>,
}

/// Result of formation analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormationResult {
    /// Whether contract formed
    pub contract_formed: bool,
    /// Satisfied formation elements
    pub satisfied_elements: Vec<FormationElement>,
    /// Missing formation elements
    pub missing_elements: Vec<FormationElement>,
    /// Vitiating factors identified
    pub vitiating_factors: Vec<VitiatingFactor>,
    /// Issues identified
    pub issues: Vec<String>,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Terms Analysis
// ============================================================================

/// Analyzer for contractual terms
pub struct TermsAnalyzer;

impl TermsAnalyzer {
    /// Analyze whether term is incorporated
    pub fn check_incorporation(facts: &IncorporationFacts) -> IncorporationResult {
        let incorporated = Self::is_incorporated(facts);
        let reasoning = Self::build_incorporation_reasoning(facts, incorporated);

        IncorporationResult {
            term_description: facts.term_description.clone(),
            incorporated,
            incorporation_method: facts.incorporation_method.clone(),
            reasoning,
        }
    }

    /// Check if term incorporated
    fn is_incorporated(facts: &IncorporationFacts) -> bool {
        match &facts.incorporation_method {
            Some(IncorporationMethod::Signature) => {
                // L'Estrange v Graucob: signature binds
                facts.document_signed && !facts.misrepresentation_of_term
            }
            Some(IncorporationMethod::ReasonableNotice) => {
                // Parker v South Eastern Railway
                facts.notice_given
                    && facts.notice_before_contract
                    && (facts.standard_term || facts.reasonable_notice_of_onerous)
            }
            Some(IncorporationMethod::CourseOfDealing) => {
                facts.consistent_previous_course && facts.dealings_sufficient_number
            }
            Some(IncorporationMethod::TradeCustom) => {
                facts.custom_notorious && facts.custom_reasonable
            }
            None => false,
        }
    }

    /// Build incorporation reasoning
    fn build_incorporation_reasoning(facts: &IncorporationFacts, incorporated: bool) -> String {
        let mut parts = Vec::new();

        parts.push("Term incorporation analysis".to_string());

        if incorporated {
            match &facts.incorporation_method {
                Some(IncorporationMethod::Signature) => {
                    parts.push("Incorporated by signature (L'Estrange v Graucob)".to_string());
                }
                Some(IncorporationMethod::ReasonableNotice) => {
                    parts.push(
                        "Incorporated by reasonable notice (Parker v South Eastern Rly)"
                            .to_string(),
                    );
                    if facts.reasonable_notice_of_onerous {
                        parts.push(
                            "Onerous term: extra notice required (Thornton v Shoe Lane)"
                                .to_string(),
                        );
                    }
                }
                Some(IncorporationMethod::CourseOfDealing) => {
                    parts.push("Incorporated through course of dealing".to_string());
                }
                Some(IncorporationMethod::TradeCustom) => {
                    parts.push("Incorporated by trade custom".to_string());
                }
                None => {}
            }
        } else {
            parts.push("Term not incorporated".to_string());
            if facts.notice_after_contract {
                parts.push("Notice given after contract formed - too late".to_string());
            }
            if !facts.reasonable_notice_of_onerous && facts.onerous_term {
                parts.push("Insufficient notice of onerous term (Interfoto)".to_string());
            }
        }

        parts.join(". ")
    }

    /// Classify a term
    pub fn classify_term(facts: &ClassificationFacts) -> TermClassification {
        // Hong Kong Fir approach for intermediate terms
        if facts.express_classification.is_some() {
            return facts.express_classification.clone().expect("checked above");
        }

        if facts.parties_treated_as_essential {
            return TermClassification::Condition;
        }

        if facts.statute_classifies_as_condition {
            return TermClassification::Condition;
        }

        // Default to intermediate term (Hong Kong Fir)
        TermClassification::Intermediate
    }
}

/// Method of incorporation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IncorporationMethod {
    /// Signature
    Signature,
    /// Reasonable notice
    ReasonableNotice,
    /// Course of dealing
    CourseOfDealing,
    /// Trade custom
    TradeCustom,
}

/// Facts for incorporation analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IncorporationFacts {
    /// Term description
    pub term_description: String,
    /// Incorporation method
    pub incorporation_method: Option<IncorporationMethod>,
    /// Document signed
    pub document_signed: bool,
    /// Misrepresentation of term
    pub misrepresentation_of_term: bool,
    /// Notice given
    pub notice_given: bool,
    /// Notice before contract
    pub notice_before_contract: bool,
    /// Notice after contract
    pub notice_after_contract: bool,
    /// Standard term
    pub standard_term: bool,
    /// Onerous term
    pub onerous_term: bool,
    /// Reasonable notice of onerous term
    pub reasonable_notice_of_onerous: bool,
    /// Consistent previous course
    pub consistent_previous_course: bool,
    /// Sufficient dealings
    pub dealings_sufficient_number: bool,
    /// Custom notorious
    pub custom_notorious: bool,
    /// Custom reasonable
    pub custom_reasonable: bool,
}

/// Result of incorporation analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncorporationResult {
    /// Term description
    pub term_description: String,
    /// Whether incorporated
    pub incorporated: bool,
    /// Method of incorporation
    pub incorporation_method: Option<IncorporationMethod>,
    /// Reasoning
    pub reasoning: String,
}

/// Facts for term classification
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClassificationFacts {
    /// Express classification in contract
    pub express_classification: Option<TermClassification>,
    /// Parties treated as essential
    pub parties_treated_as_essential: bool,
    /// Statute classifies as condition
    pub statute_classifies_as_condition: bool,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formation_commercial() {
        let facts = FormationFacts {
            offer_made: true,
            acceptance_communicated: true,
            consideration_present: true,
            commercial_context: true,
            essential_terms_agreed: true,
            ..Default::default()
        };

        let result = FormationAnalyzer::analyze(&facts);
        assert!(result.contract_formed);
    }

    #[test]
    fn test_formation_no_consideration() {
        let facts = FormationFacts {
            offer_made: true,
            acceptance_communicated: true,
            consideration_present: false,
            commercial_context: true,
            essential_terms_agreed: true,
            ..Default::default()
        };

        let result = FormationAnalyzer::analyze(&facts);
        assert!(!result.contract_formed);
        assert!(
            result
                .missing_elements
                .contains(&FormationElement::Consideration)
        );
    }

    #[test]
    fn test_formation_deed() {
        let facts = FormationFacts {
            offer_made: true,
            acceptance_communicated: true,
            is_deed: true,
            commercial_context: true,
            essential_terms_agreed: true,
            ..Default::default()
        };

        let result = FormationAnalyzer::analyze(&facts);
        assert!(result.contract_formed);
        assert!(result.reasoning.contains("deed"));
    }

    #[test]
    fn test_incorporation_signature() {
        let facts = IncorporationFacts {
            term_description: "Exclusion clause".to_string(),
            incorporation_method: Some(IncorporationMethod::Signature),
            document_signed: true,
            ..Default::default()
        };

        let result = TermsAnalyzer::check_incorporation(&facts);
        assert!(result.incorporated);
        assert!(result.reasoning.contains("L'Estrange"));
    }

    #[test]
    fn test_incorporation_notice_too_late() {
        let facts = IncorporationFacts {
            term_description: "Limitation clause".to_string(),
            incorporation_method: Some(IncorporationMethod::ReasonableNotice),
            notice_given: true,
            notice_after_contract: true,
            ..Default::default()
        };

        let result = TermsAnalyzer::check_incorporation(&facts);
        assert!(!result.incorporated);
    }
}
