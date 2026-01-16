//! Unfair Dismissal Analysis
//!
//! Implementation of Fair Work Act Part 3-2: Unfair dismissal provisions.

use serde::{Deserialize, Serialize};

use super::types::{
    EmploymentType, Section387Factor, UnfairDismissalElement, UnfairDismissalRemedy,
};

// ============================================================================
// Eligibility Analyzer
// ============================================================================

/// Analyzer for unfair dismissal eligibility
pub struct EligibilityAnalyzer;

impl EligibilityAnalyzer {
    /// Check if employee is eligible to make unfair dismissal claim
    pub fn check_eligibility(facts: &EligibilityFacts) -> EligibilityResult {
        let mut issues = Vec::new();

        // Check minimum employment period (s.383)
        let minimum_period = if facts.small_business_employer {
            12.0 // 12 months for small business
        } else {
            6.0 // 6 months for others
        };

        let service_satisfied = facts.months_employed >= minimum_period;
        if !service_satisfied {
            issues.push(format!(
                "Minimum employment period not met: {:.1} months (required: {:.1})",
                facts.months_employed, minimum_period
            ));
        }

        // Check high income threshold (s.382(b))
        let income_threshold = 175_000.0; // 2024 threshold
        let income_satisfied = if facts.covered_by_award_or_agreement {
            true // No threshold if covered
        } else {
            facts.annual_earnings <= income_threshold
        };
        if !income_satisfied {
            issues.push(format!(
                "Earnings ${:.0} exceed high income threshold ${:.0} (s.382(b))",
                facts.annual_earnings, income_threshold
            ));
        }

        // Check national system employer (s.382(a))
        let national_system = facts.national_system_employer;
        if !national_system {
            issues.push("Not a national system employer (s.382(a))".to_string());
        }

        // Check exclusions
        let not_excluded = !Self::check_exclusions(facts);
        if !not_excluded {
            issues.push("Employee excluded from unfair dismissal protections".to_string());
        }

        let eligible = service_satisfied && income_satisfied && national_system && not_excluded;
        let reasoning = Self::build_reasoning(
            service_satisfied,
            income_satisfied,
            national_system,
            not_excluded,
            &issues,
        );

        EligibilityResult {
            eligible,
            minimum_period_satisfied: service_satisfied,
            income_threshold_satisfied: income_satisfied,
            national_system_employee: national_system,
            excluded: !not_excluded,
            issues,
            reasoning,
        }
    }

    /// Check exclusions (s.382(c))
    fn check_exclusions(facts: &EligibilityFacts) -> bool {
        // Casual employee without regular pattern
        if matches!(facts.employment_type, EmploymentType::Casual) && !facts.regular_casual {
            return true;
        }

        // Short-term or seasonal employee
        if facts.genuine_short_term_contract || facts.genuine_seasonal {
            return true;
        }

        // Probationary period (if reasonable)
        if facts.in_probation && facts.probation_reasonable {
            return true;
        }

        false
    }

    /// Build reasoning
    fn build_reasoning(
        service: bool,
        income: bool,
        national: bool,
        not_excluded: bool,
        issues: &[String],
    ) -> String {
        let mut parts = Vec::new();

        parts.push("Unfair dismissal eligibility analysis (Fair Work Act s.382-384)".to_string());

        if service {
            parts.push("Minimum employment period satisfied".to_string());
        }

        if income {
            parts.push("Income threshold satisfied or covered by award/agreement".to_string());
        }

        if national {
            parts.push("National system employer confirmed".to_string());
        }

        if not_excluded {
            parts.push("No exclusions apply".to_string());
        }

        for issue in issues {
            parts.push(format!("Issue: {}", issue));
        }

        parts.join(". ")
    }
}

/// Facts for eligibility analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EligibilityFacts {
    /// Months employed
    pub months_employed: f64,
    /// Small business employer (< 15 employees)
    pub small_business_employer: bool,
    /// Annual earnings
    pub annual_earnings: f64,
    /// Covered by award or enterprise agreement
    pub covered_by_award_or_agreement: bool,
    /// National system employer
    pub national_system_employer: bool,
    /// Employment type
    pub employment_type: EmploymentType,
    /// Regular casual engagement
    pub regular_casual: bool,
    /// Genuine short-term contract
    pub genuine_short_term_contract: bool,
    /// Genuine seasonal employee
    pub genuine_seasonal: bool,
    /// In probationary period
    pub in_probation: bool,
    /// Probationary period reasonable
    pub probation_reasonable: bool,
}

/// Result of eligibility analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EligibilityResult {
    /// Whether eligible
    pub eligible: bool,
    /// Minimum period satisfied
    pub minimum_period_satisfied: bool,
    /// Income threshold satisfied
    pub income_threshold_satisfied: bool,
    /// National system employee
    pub national_system_employee: bool,
    /// Excluded from protections
    pub excluded: bool,
    /// Issues
    pub issues: Vec<String>,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Unfair Dismissal Analyzer
// ============================================================================

/// Analyzer for unfair dismissal claims
pub struct UnfairDismissalAnalyzer;

impl UnfairDismissalAnalyzer {
    /// Analyze unfair dismissal claim
    pub fn analyze(facts: &UnfairDismissalFacts) -> UnfairDismissalResult {
        let mut satisfied_elements = Vec::new();
        let mut unsatisfied_elements = Vec::new();

        // s.385(a) - person dismissed
        if facts.dismissal_occurred {
            satisfied_elements.push(UnfairDismissalElement::PersonDismissed);
        } else {
            unsatisfied_elements.push(UnfairDismissalElement::PersonDismissed);
        }

        // s.385(b) - harsh, unjust or unreasonable
        let hju_result = Self::assess_section_387(facts);
        if hju_result.harsh_unjust_unreasonable {
            satisfied_elements.push(UnfairDismissalElement::HarshUnjustUnreasonable);
        } else {
            unsatisfied_elements.push(UnfairDismissalElement::HarshUnjustUnreasonable);
        }

        // s.385(c) - Small Business Fair Dismissal Code
        if !facts.small_business_employer || !facts.complied_with_sbfdc {
            if facts.small_business_employer && facts.complied_with_sbfdc {
                unsatisfied_elements.push(UnfairDismissalElement::NotConsistentWithCode);
            }
        } else {
            satisfied_elements.push(UnfairDismissalElement::NotConsistentWithCode);
        }

        // s.385(d) - genuine redundancy
        if facts.redundancy_claimed {
            let genuine = Self::assess_genuine_redundancy(facts);
            if !genuine {
                satisfied_elements.push(UnfairDismissalElement::NotGenuineRedundancy);
            } else {
                unsatisfied_elements.push(UnfairDismissalElement::NotGenuineRedundancy);
            }
        }

        let unfair = !unsatisfied_elements.contains(&UnfairDismissalElement::PersonDismissed)
            && satisfied_elements.contains(&UnfairDismissalElement::HarshUnjustUnreasonable);

        let remedies = if unfair {
            Self::determine_remedies(facts)
        } else {
            Vec::new()
        };

        let reasoning = Self::build_reasoning(
            &satisfied_elements,
            &unsatisfied_elements,
            &hju_result,
            facts,
        );

        UnfairDismissalResult {
            dismissal_unfair: unfair,
            satisfied_elements,
            unsatisfied_elements,
            section_387_analysis: hju_result,
            recommended_remedy: Self::recommend_remedy(&remedies, facts),
            available_remedies: remedies,
            reasoning,
        }
    }

    /// Assess s.387 factors
    fn assess_section_387(facts: &UnfairDismissalFacts) -> Section387Result {
        let mut factors_for: Vec<Section387Factor> = Vec::new();
        let mut factors_against: Vec<Section387Factor> = Vec::new();

        // s.387(a) - valid reason
        if facts.valid_reason {
            factors_against.push(Section387Factor::ValidReason);
        } else {
            factors_for.push(Section387Factor::ValidReason);
        }

        // s.387(b) - notified of reason
        if facts.notified_of_reason {
            factors_against.push(Section387Factor::NotifiedOfReason);
        } else {
            factors_for.push(Section387Factor::NotifiedOfReason);
        }

        // s.387(c) - opportunity to respond
        if facts.opportunity_to_respond {
            factors_against.push(Section387Factor::OpportunityToRespond);
        } else {
            factors_for.push(Section387Factor::OpportunityToRespond);
        }

        // s.387(d) - support person
        if facts.support_person_allowed {
            factors_against.push(Section387Factor::SupportPerson);
        } else {
            factors_for.push(Section387Factor::SupportPerson);
        }

        // s.387(e) - warnings
        if facts.performance_issue && !facts.warnings_given {
            factors_for.push(Section387Factor::WarningsGiven);
        } else if facts.warnings_given {
            factors_against.push(Section387Factor::WarningsGiven);
        }

        // s.387(f) - enterprise size
        if facts.small_business_employer {
            factors_against.push(Section387Factor::EnterpriseSize);
        }

        // s.387(g) - HR specialists
        if facts.hr_specialists_available {
            factors_against.push(Section387Factor::HRSpecialists);
        } else {
            factors_for.push(Section387Factor::HRSpecialists);
        }

        // Balance factors
        let harsh_unjust_unreasonable = factors_for.len() > factors_against.len()
            || (!facts.valid_reason && !facts.opportunity_to_respond);

        Section387Result {
            valid_reason: facts.valid_reason,
            procedural_fairness: facts.notified_of_reason
                && facts.opportunity_to_respond
                && facts.support_person_allowed,
            factors_for_unfairness: factors_for,
            factors_against_unfairness: factors_against,
            harsh_unjust_unreasonable,
        }
    }

    /// Assess genuine redundancy (s.389)
    fn assess_genuine_redundancy(facts: &UnfairDismissalFacts) -> bool {
        // s.389(1)(a) - no longer required to be performed
        if !facts.job_no_longer_required {
            return false;
        }

        // s.389(1)(b) - consultation requirements
        if facts.consultation_required && !facts.consultation_occurred {
            return false;
        }

        // s.389(2) - redeployment
        if facts.redeployment_reasonable && !facts.redeployment_offered {
            return false;
        }

        true
    }

    /// Determine available remedies
    fn determine_remedies(facts: &UnfairDismissalFacts) -> Vec<UnfairDismissalRemedy> {
        let mut remedies = Vec::new();

        // Reinstatement is primary remedy
        if !facts.reinstatement_inappropriate {
            remedies.push(UnfairDismissalRemedy::Reinstatement);
        }

        // Reemployment alternative
        if !facts.reinstatement_inappropriate {
            remedies.push(UnfairDismissalRemedy::Reemployment);
        }

        // Compensation if reinstatement inappropriate
        remedies.push(UnfairDismissalRemedy::Compensation);

        remedies
    }

    /// Recommend remedy
    fn recommend_remedy(
        remedies: &[UnfairDismissalRemedy],
        facts: &UnfairDismissalFacts,
    ) -> Option<UnfairDismissalRemedy> {
        if remedies.is_empty() {
            return None;
        }

        // Reinstatement is primary remedy (s.390)
        if remedies.contains(&UnfairDismissalRemedy::Reinstatement)
            && !facts.reinstatement_inappropriate
            && !facts.relationship_destroyed
        {
            return Some(UnfairDismissalRemedy::Reinstatement);
        }

        // Otherwise compensation
        if remedies.contains(&UnfairDismissalRemedy::Compensation) {
            return Some(UnfairDismissalRemedy::Compensation);
        }

        None
    }

    /// Build reasoning
    fn build_reasoning(
        satisfied: &[UnfairDismissalElement],
        unsatisfied: &[UnfairDismissalElement],
        hju: &Section387Result,
        facts: &UnfairDismissalFacts,
    ) -> String {
        let mut parts = Vec::new();

        parts.push("Unfair dismissal analysis (Fair Work Act Part 3-2)".to_string());

        // s.385 elements
        if satisfied.contains(&UnfairDismissalElement::PersonDismissed) {
            parts.push("Dismissal established (s.385(a))".to_string());
        }

        // s.387 analysis
        parts.push(format!(
            "s.387 factors: {} for unfairness, {} against",
            hju.factors_for_unfairness.len(),
            hju.factors_against_unfairness.len()
        ));

        if !hju.valid_reason {
            parts.push("No valid reason for dismissal (s.387(a))".to_string());
        }

        if !hju.procedural_fairness {
            parts.push("Procedural fairness not afforded".to_string());
            if !facts.notified_of_reason {
                parts.push("Not notified of reason (s.387(b))".to_string());
            }
            if !facts.opportunity_to_respond {
                parts.push("No opportunity to respond (s.387(c))".to_string());
            }
        }

        if hju.harsh_unjust_unreasonable {
            parts.push("Dismissal harsh, unjust or unreasonable (s.385(b))".to_string());
        }

        // Genuine redundancy
        if facts.redundancy_claimed {
            if unsatisfied.contains(&UnfairDismissalElement::NotGenuineRedundancy) {
                parts.push("Genuine redundancy established - claim fails (s.389)".to_string());
            } else {
                parts.push("Not a genuine redundancy (s.389)".to_string());
            }
        }

        parts.join(". ")
    }
}

/// Facts for unfair dismissal analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnfairDismissalFacts {
    /// Dismissal occurred
    pub dismissal_occurred: bool,
    /// Small business employer
    pub small_business_employer: bool,
    /// Complied with Small Business Fair Dismissal Code
    pub complied_with_sbfdc: bool,
    /// Valid reason for dismissal
    pub valid_reason: bool,
    /// Reason related to conduct
    pub conduct_reason: bool,
    /// Reason related to capacity
    pub capacity_reason: bool,
    /// Performance issue
    pub performance_issue: bool,
    /// Warnings given
    pub warnings_given: bool,
    /// Notified of reason
    pub notified_of_reason: bool,
    /// Opportunity to respond
    pub opportunity_to_respond: bool,
    /// Support person allowed
    pub support_person_allowed: bool,
    /// HR specialists available
    pub hr_specialists_available: bool,
    /// Redundancy claimed
    pub redundancy_claimed: bool,
    /// Job no longer required
    pub job_no_longer_required: bool,
    /// Consultation required under award/agreement
    pub consultation_required: bool,
    /// Consultation occurred
    pub consultation_occurred: bool,
    /// Redeployment reasonable
    pub redeployment_reasonable: bool,
    /// Redeployment offered
    pub redeployment_offered: bool,
    /// Reinstatement inappropriate
    pub reinstatement_inappropriate: bool,
    /// Relationship destroyed
    pub relationship_destroyed: bool,
}

/// Result of s.387 analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section387Result {
    /// Valid reason established
    pub valid_reason: bool,
    /// Procedural fairness afforded
    pub procedural_fairness: bool,
    /// Factors supporting unfairness finding
    pub factors_for_unfairness: Vec<Section387Factor>,
    /// Factors against unfairness finding
    pub factors_against_unfairness: Vec<Section387Factor>,
    /// Overall: harsh, unjust or unreasonable
    pub harsh_unjust_unreasonable: bool,
}

/// Result of unfair dismissal analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnfairDismissalResult {
    /// Dismissal was unfair
    pub dismissal_unfair: bool,
    /// Satisfied elements
    pub satisfied_elements: Vec<UnfairDismissalElement>,
    /// Unsatisfied elements
    pub unsatisfied_elements: Vec<UnfairDismissalElement>,
    /// s.387 analysis
    pub section_387_analysis: Section387Result,
    /// Recommended remedy
    pub recommended_remedy: Option<UnfairDismissalRemedy>,
    /// Available remedies
    pub available_remedies: Vec<UnfairDismissalRemedy>,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Compensation Calculator
// ============================================================================

/// Calculator for unfair dismissal compensation (s.392)
pub struct CompensationCalculator;

impl CompensationCalculator {
    /// Calculate compensation
    pub fn calculate(facts: &CompensationFacts) -> CompensationResult {
        // Maximum is lesser of 26 weeks pay or half high income threshold
        let max_weeks = 26.0;
        let high_income_half = 87_500.0; // Half of threshold

        let weekly_pay = facts.annual_earnings / 52.0;
        let max_by_weeks = weekly_pay * max_weeks;
        let cap = max_by_weeks.min(high_income_half);

        // Start with remuneration lost
        let base = facts.remuneration_lost;

        // Apply s.392(2) factors
        let mut deductions = 0.0;

        // s.392(2)(c) - efforts to mitigate
        if !facts.reasonable_mitigation_efforts {
            deductions += base * 0.25;
        }

        // s.392(2)(d) - other remuneration earned
        deductions += facts.other_remuneration_earned;

        // s.392(2)(e) - employee conduct
        if facts.employee_contributed_to_dismissal {
            deductions += base * (facts.contribution_percentage / 100.0);
        }

        let calculated = (base - deductions).max(0.0);
        let final_amount = calculated.min(cap);

        let reasoning = Self::build_reasoning(base, deductions, cap, final_amount, facts);

        CompensationResult {
            base_amount: base,
            deductions,
            cap,
            final_amount,
            weeks_equivalent: final_amount / weekly_pay,
            reasoning,
        }
    }

    /// Build reasoning
    fn build_reasoning(
        base: f64,
        deductions: f64,
        cap: f64,
        final_amount: f64,
        facts: &CompensationFacts,
    ) -> String {
        let mut parts = Vec::new();

        parts.push("Compensation calculation (s.392)".to_string());
        parts.push(format!("Base remuneration lost: ${:.2}", base));

        if !facts.reasonable_mitigation_efforts {
            parts.push("Reduction for failure to mitigate (s.392(2)(c))".to_string());
        }

        if facts.other_remuneration_earned > 0.0 {
            parts.push(format!(
                "Other remuneration earned: ${:.2} (s.392(2)(d))",
                facts.other_remuneration_earned
            ));
        }

        if facts.employee_contributed_to_dismissal {
            parts.push(format!(
                "Employee contribution: {}% reduction (s.392(2)(e))",
                facts.contribution_percentage
            ));
        }

        parts.push(format!("Total deductions: ${:.2}", deductions));
        parts.push(format!("Cap (26 weeks or half threshold): ${:.2}", cap));
        parts.push(format!("Final compensation: ${:.2}", final_amount));

        parts.join(". ")
    }
}

/// Facts for compensation calculation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompensationFacts {
    /// Annual earnings
    pub annual_earnings: f64,
    /// Remuneration lost
    pub remuneration_lost: f64,
    /// Reasonable mitigation efforts
    pub reasonable_mitigation_efforts: bool,
    /// Other remuneration earned
    pub other_remuneration_earned: f64,
    /// Employee contributed to dismissal
    pub employee_contributed_to_dismissal: bool,
    /// Contribution percentage
    pub contribution_percentage: f64,
}

/// Result of compensation calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompensationResult {
    /// Base amount
    pub base_amount: f64,
    /// Total deductions
    pub deductions: f64,
    /// Compensation cap
    pub cap: f64,
    /// Final amount
    pub final_amount: f64,
    /// Weeks equivalent
    pub weeks_equivalent: f64,
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
    fn test_eligibility_satisfied() {
        let facts = EligibilityFacts {
            months_employed: 8.0,
            small_business_employer: false,
            annual_earnings: 80_000.0,
            covered_by_award_or_agreement: true,
            national_system_employer: true,
            employment_type: EmploymentType::FullTime,
            ..Default::default()
        };

        let result = EligibilityAnalyzer::check_eligibility(&facts);
        assert!(result.eligible);
    }

    #[test]
    fn test_eligibility_small_business_period() {
        let facts = EligibilityFacts {
            months_employed: 8.0,
            small_business_employer: true, // Need 12 months
            national_system_employer: true,
            ..Default::default()
        };

        let result = EligibilityAnalyzer::check_eligibility(&facts);
        assert!(!result.eligible);
        assert!(!result.minimum_period_satisfied);
    }

    #[test]
    fn test_eligibility_high_income() {
        let facts = EligibilityFacts {
            months_employed: 12.0,
            annual_earnings: 200_000.0, // Above threshold
            covered_by_award_or_agreement: false,
            national_system_employer: true,
            ..Default::default()
        };

        let result = EligibilityAnalyzer::check_eligibility(&facts);
        assert!(!result.eligible);
        assert!(!result.income_threshold_satisfied);
    }

    #[test]
    fn test_unfair_dismissal_no_reason() {
        let facts = UnfairDismissalFacts {
            dismissal_occurred: true,
            valid_reason: false,
            notified_of_reason: false,
            opportunity_to_respond: false,
            support_person_allowed: false,
            ..Default::default()
        };

        let result = UnfairDismissalAnalyzer::analyze(&facts);
        assert!(result.dismissal_unfair);
        assert!(result.section_387_analysis.harsh_unjust_unreasonable);
    }

    #[test]
    fn test_unfair_dismissal_fair_process() {
        let facts = UnfairDismissalFacts {
            dismissal_occurred: true,
            valid_reason: true,
            notified_of_reason: true,
            opportunity_to_respond: true,
            support_person_allowed: true,
            warnings_given: true,
            hr_specialists_available: true,
            ..Default::default()
        };

        let result = UnfairDismissalAnalyzer::analyze(&facts);
        assert!(!result.dismissal_unfair);
        assert!(result.section_387_analysis.procedural_fairness);
    }

    #[test]
    fn test_genuine_redundancy() {
        let facts = UnfairDismissalFacts {
            dismissal_occurred: true,
            redundancy_claimed: true,
            job_no_longer_required: true,
            consultation_required: true,
            consultation_occurred: true,
            redeployment_reasonable: false,
            ..Default::default()
        };

        let result = UnfairDismissalAnalyzer::analyze(&facts);
        assert!(
            result
                .unsatisfied_elements
                .contains(&UnfairDismissalElement::NotGenuineRedundancy)
        );
    }

    #[test]
    fn test_compensation_calculation() {
        let facts = CompensationFacts {
            annual_earnings: 78_000.0,
            remuneration_lost: 30_000.0,
            reasonable_mitigation_efforts: true,
            other_remuneration_earned: 5_000.0,
            employee_contributed_to_dismissal: true,
            contribution_percentage: 20.0,
        };

        let result = CompensationCalculator::calculate(&facts);
        // 30000 - 5000 - (30000 * 0.2) = 30000 - 5000 - 6000 = 19000
        assert!(result.final_amount > 0.0);
        assert!(result.final_amount <= result.cap);
    }

    #[test]
    fn test_compensation_cap() {
        let facts = CompensationFacts {
            annual_earnings: 200_000.0, // High earner
            remuneration_lost: 150_000.0,
            reasonable_mitigation_efforts: true,
            ..Default::default()
        };

        let result = CompensationCalculator::calculate(&facts);
        // Should be capped
        assert!(result.final_amount <= 87_500.0);
    }
}
