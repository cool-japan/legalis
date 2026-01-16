//! Corporations Act 2001 Analysis
//!
//! Implementation of key Corporations Act provisions.

use serde::{Deserialize, Serialize};

use super::types::{DirectorsDuty, InsolventTradingDefence};

// ============================================================================
// Directors Duties Analyzer
// ============================================================================

/// Analyzer for directors duties
pub struct DirectorsDutiesAnalyzer;

impl DirectorsDutiesAnalyzer {
    /// Analyze directors duty breach
    pub fn analyze(facts: &DutyFacts) -> DutyResult {
        let breached_duties = Self::check_duties(facts);
        let bjr_available = Self::check_business_judgment_rule(facts);
        let defences = Self::check_defences(facts);

        let breach = !breached_duties.is_empty() && !bjr_available;
        let reasoning = Self::build_reasoning(facts, &breached_duties, bjr_available);

        DutyResult {
            breach_found: breach,
            breached_duties,
            business_judgment_rule_applies: bjr_available,
            defences_available: defences,
            civil_penalty_risk: breach,
            reasoning,
        }
    }

    /// Check duties
    fn check_duties(facts: &DutyFacts) -> Vec<DirectorsDuty> {
        let mut breaches = Vec::new();

        // s.180 - Care and diligence
        if !facts.exercised_care_diligence {
            breaches.push(DirectorsDuty::CareAndDiligence);
        }

        // s.181 - Good faith and proper purpose
        if !facts.acted_good_faith || !facts.proper_purpose {
            breaches.push(DirectorsDuty::GoodFaithProperPurpose);
        }

        // s.182 - Improper use of position
        if facts.used_position_improperly {
            breaches.push(DirectorsDuty::NotImproperlyUsePosition);
        }

        // s.183 - Improper use of information
        if facts.used_information_improperly {
            breaches.push(DirectorsDuty::NotImproperlyUseInformation);
        }

        breaches
    }

    /// Check business judgment rule (s.180(2))
    fn check_business_judgment_rule(facts: &DutyFacts) -> bool {
        // All elements must be satisfied
        facts.bjr_good_faith
            && facts.bjr_no_personal_interest
            && facts.bjr_informed_appropriately
            && facts.bjr_rational_belief
    }

    /// Check defences
    fn check_defences(facts: &DutyFacts) -> Vec<String> {
        let mut defences = Vec::new();

        if facts.reasonable_reliance_on_others {
            defences.push("Reasonable reliance on professional advice (s.189)".to_string());
        }

        if facts.delegation_reasonable {
            defences.push("Reasonable delegation (s.190)".to_string());
        }

        defences
    }

    /// Build reasoning
    fn build_reasoning(facts: &DutyFacts, breached: &[DirectorsDuty], bjr: bool) -> String {
        let mut parts = Vec::new();

        parts.push("Directors duties analysis (Corporations Act 2001 Part 2D.1)".to_string());

        if breached.is_empty() {
            parts.push("No duty breaches identified".to_string());
        } else {
            for duty in breached {
                parts.push(format!(
                    "Potential breach of {} ({:?})",
                    duty.section(),
                    duty
                ));
            }
        }

        if bjr {
            parts.push("Business judgment rule applies (s.180(2))".to_string());
            parts.push("Director made informed decision in good faith".to_string());
        }

        if !facts.exercised_care_diligence {
            parts.push(
                "Standard: would reasonable person exercise care? (ASIC v Healey)".to_string(),
            );
        }

        parts.join(". ")
    }
}

/// Facts for duty analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DutyFacts {
    /// Exercised care and diligence
    pub exercised_care_diligence: bool,
    /// Acted in good faith
    pub acted_good_faith: bool,
    /// Proper purpose
    pub proper_purpose: bool,
    /// Used position improperly
    pub used_position_improperly: bool,
    /// Used information improperly
    pub used_information_improperly: bool,
    /// BJR: Good faith for proper purpose
    pub bjr_good_faith: bool,
    /// BJR: No material personal interest
    pub bjr_no_personal_interest: bool,
    /// BJR: Informed appropriately
    pub bjr_informed_appropriately: bool,
    /// BJR: Rational belief in company's interests
    pub bjr_rational_belief: bool,
    /// Reasonable reliance on others
    pub reasonable_reliance_on_others: bool,
    /// Delegation reasonable
    pub delegation_reasonable: bool,
}

/// Result of duty analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DutyResult {
    /// Breach found
    pub breach_found: bool,
    /// Breached duties
    pub breached_duties: Vec<DirectorsDuty>,
    /// Business judgment rule applies
    pub business_judgment_rule_applies: bool,
    /// Defences available
    pub defences_available: Vec<String>,
    /// Civil penalty risk
    pub civil_penalty_risk: bool,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Insolvent Trading Analyzer
// ============================================================================

/// Analyzer for insolvent trading liability (s.588G)
pub struct InsolventTradingAnalyzer;

impl InsolventTradingAnalyzer {
    /// Analyze insolvent trading
    pub fn analyze(facts: &InsolventTradingFacts) -> InsolventTradingResult {
        let elements = Self::check_elements(facts);
        let defences = Self::check_defences(facts);

        let liable = elements.all_satisfied && defences.is_empty();
        let reasoning = Self::build_reasoning(facts, &elements, &defences);

        InsolventTradingResult {
            elements_satisfied: elements.all_satisfied,
            reasonable_grounds_to_suspect: facts.reasonable_grounds_to_suspect,
            debt_incurred_while_insolvent: facts.debt_incurred,
            defences_available: defences,
            liable,
            reasoning,
        }
    }

    /// Check elements
    fn check_elements(facts: &InsolventTradingFacts) -> ElementsResult {
        let company_insolvent = facts.company_insolvent;
        let debt_incurred = facts.debt_incurred;
        let reasonable_grounds = facts.reasonable_grounds_to_suspect;
        let was_director = facts.was_director_at_time;

        ElementsResult {
            company_insolvent,
            debt_incurred,
            reasonable_grounds_to_suspect: reasonable_grounds,
            was_director,
            all_satisfied: company_insolvent && debt_incurred && reasonable_grounds && was_director,
        }
    }

    /// Check defences
    fn check_defences(facts: &InsolventTradingFacts) -> Vec<InsolventTradingDefence> {
        let mut defences = Vec::new();

        // s.588H(2) - Reasonable expectation of solvency
        if facts.reasonable_expectation_solvency {
            defences.push(InsolventTradingDefence::ReasonableExpectation);
        }

        // s.588H(3) - No reasonable grounds to suspect
        if !facts.reasonable_grounds_to_suspect {
            defences.push(InsolventTradingDefence::NoSuspicion);
        }

        // s.588H(4) - Took reasonable steps
        if facts.took_reasonable_steps {
            defences.push(InsolventTradingDefence::ReasonableSteps);
        }

        // s.588GA - Safe harbour
        if facts.safe_harbour_applies {
            defences.push(InsolventTradingDefence::SafeHarbour);
        }

        defences
    }

    /// Build reasoning
    fn build_reasoning(
        _facts: &InsolventTradingFacts,
        elements: &ElementsResult,
        defences: &[InsolventTradingDefence],
    ) -> String {
        let mut parts = Vec::new();

        parts.push("Insolvent trading analysis (s.588G Corporations Act)".to_string());

        // Elements
        if elements.company_insolvent {
            parts.push(
                "Company was insolvent (s.95A - unable to pay debts as they fall due)".to_string(),
            );
        }

        if elements.debt_incurred {
            parts.push("Debt incurred during insolvency".to_string());
        }

        if elements.reasonable_grounds_to_suspect {
            parts.push("Reasonable grounds to suspect insolvency existed".to_string());
        }

        // Defences
        if !defences.is_empty() {
            parts.push("Defences available:".to_string());
            for defence in defences {
                match defence {
                    InsolventTradingDefence::SafeHarbour => {
                        parts.push(
                            "- Safe harbour: developing course of action (s.588GA)".to_string(),
                        );
                    }
                    InsolventTradingDefence::ReasonableExpectation => {
                        parts.push("- Reasonable expectation of solvency (s.588H(2))".to_string());
                    }
                    _ => {
                        parts.push(format!("- {:?}", defence));
                    }
                }
            }
        }

        parts.join(". ")
    }
}

/// Facts for insolvent trading analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InsolventTradingFacts {
    /// Company was insolvent
    pub company_insolvent: bool,
    /// Debt was incurred
    pub debt_incurred: bool,
    /// Reasonable grounds to suspect insolvency
    pub reasonable_grounds_to_suspect: bool,
    /// Was director at time
    pub was_director_at_time: bool,
    /// Reasonable expectation of solvency
    pub reasonable_expectation_solvency: bool,
    /// Took reasonable steps
    pub took_reasonable_steps: bool,
    /// Safe harbour applies
    pub safe_harbour_applies: bool,
}

struct ElementsResult {
    company_insolvent: bool,
    debt_incurred: bool,
    reasonable_grounds_to_suspect: bool,
    #[allow(dead_code)]
    was_director: bool,
    all_satisfied: bool,
}

/// Result of insolvent trading analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsolventTradingResult {
    /// Elements satisfied
    pub elements_satisfied: bool,
    /// Reasonable grounds to suspect
    pub reasonable_grounds_to_suspect: bool,
    /// Debt incurred while insolvent
    pub debt_incurred_while_insolvent: bool,
    /// Defences available
    pub defences_available: Vec<InsolventTradingDefence>,
    /// Liable
    pub liable: bool,
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
    fn test_no_duty_breach() {
        let facts = DutyFacts {
            exercised_care_diligence: true,
            acted_good_faith: true,
            proper_purpose: true,
            ..Default::default()
        };

        let result = DirectorsDutiesAnalyzer::analyze(&facts);
        assert!(!result.breach_found);
    }

    #[test]
    fn test_duty_breach_care() {
        let facts = DutyFacts {
            exercised_care_diligence: false,
            ..Default::default()
        };

        let result = DirectorsDutiesAnalyzer::analyze(&facts);
        assert!(
            result
                .breached_duties
                .contains(&DirectorsDuty::CareAndDiligence)
        );
    }

    #[test]
    fn test_bjr_protects() {
        let facts = DutyFacts {
            exercised_care_diligence: false, // Would breach
            bjr_good_faith: true,
            bjr_no_personal_interest: true,
            bjr_informed_appropriately: true,
            bjr_rational_belief: true,
            ..Default::default()
        };

        let result = DirectorsDutiesAnalyzer::analyze(&facts);
        assert!(result.business_judgment_rule_applies);
        assert!(!result.breach_found);
    }

    #[test]
    fn test_insolvent_trading_liable() {
        let facts = InsolventTradingFacts {
            company_insolvent: true,
            debt_incurred: true,
            reasonable_grounds_to_suspect: true,
            was_director_at_time: true,
            ..Default::default()
        };

        let result = InsolventTradingAnalyzer::analyze(&facts);
        assert!(result.liable);
    }

    #[test]
    fn test_insolvent_trading_safe_harbour() {
        let facts = InsolventTradingFacts {
            company_insolvent: true,
            debt_incurred: true,
            reasonable_grounds_to_suspect: true,
            was_director_at_time: true,
            safe_harbour_applies: true,
            ..Default::default()
        };

        let result = InsolventTradingAnalyzer::analyze(&facts);
        assert!(!result.liable);
        assert!(
            result
                .defences_available
                .contains(&InsolventTradingDefence::SafeHarbour)
        );
    }
}
