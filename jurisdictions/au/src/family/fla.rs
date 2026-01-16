//! Family Law Act 1975 Analysis
//!
//! Implementation of Family Law Act provisions including:
//! - Divorce
//! - Parenting orders
//! - Property settlement

use serde::{Deserialize, Serialize};

use super::types::{
    AdditionalConsideration, ParentalResponsibility, PrimaryConsideration, TimeArrangement,
};

// ============================================================================
// Divorce Analyzer
// ============================================================================

/// Analyzer for divorce applications
pub struct DivorceAnalyzer;

impl DivorceAnalyzer {
    /// Analyze divorce application
    pub fn analyze(facts: &DivorceFacts) -> DivorceResult {
        let eligible = Self::check_eligibility(facts);
        let grounds = Self::check_grounds(facts);
        let children_provisions = Self::check_children_arrangements(facts);

        let can_proceed = eligible && grounds && children_provisions;
        let reasoning = Self::build_reasoning(facts, eligible, grounds, children_provisions);

        DivorceResult {
            eligible,
            grounds_established: grounds,
            children_arrangements_adequate: children_provisions,
            can_proceed,
            reasoning,
        }
    }

    /// Check eligibility (s.44)
    fn check_eligibility(facts: &DivorceFacts) -> bool {
        // One party Australian citizen, domiciled, or ordinarily resident
        facts.jurisdictional_connection
    }

    /// Check grounds - irretrievable breakdown (s.48)
    fn check_grounds(facts: &DivorceFacts) -> bool {
        // Separated for 12 months and no reasonable likelihood of cohabitation
        facts.separated_12_months && !facts.reasonable_likelihood_cohabitation
    }

    /// Check children arrangements (s.55A)
    fn check_children_arrangements(facts: &DivorceFacts) -> bool {
        // If children under 18, proper arrangements must be shown
        if !facts.children_under_18 {
            return true;
        }
        facts.proper_children_arrangements
    }

    /// Build reasoning
    fn build_reasoning(
        facts: &DivorceFacts,
        eligible: bool,
        grounds: bool,
        children: bool,
    ) -> String {
        let mut parts = Vec::new();

        parts.push("Divorce application analysis (Family Law Act 1975)".to_string());

        if eligible {
            parts.push("Jurisdictional requirements satisfied (s.44)".to_string());
        } else {
            parts.push("Jurisdictional requirements not met".to_string());
        }

        if grounds {
            parts.push(
                "Grounds established: 12 months separation, irretrievable breakdown (s.48)"
                    .to_string(),
            );
        } else if !facts.separated_12_months {
            parts.push("Not separated for 12 months".to_string());
        } else {
            parts.push("Reasonable likelihood of cohabitation exists".to_string());
        }

        if facts.children_under_18 {
            if children {
                parts.push("Proper arrangements for children demonstrated (s.55A)".to_string());
            } else {
                parts.push("Need to demonstrate proper children arrangements".to_string());
            }
        }

        parts.join(". ")
    }
}

/// Facts for divorce analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DivorceFacts {
    /// Jurisdictional connection
    pub jurisdictional_connection: bool,
    /// Separated for 12 months
    pub separated_12_months: bool,
    /// Reasonable likelihood of cohabitation
    pub reasonable_likelihood_cohabitation: bool,
    /// Children under 18
    pub children_under_18: bool,
    /// Proper children arrangements
    pub proper_children_arrangements: bool,
}

/// Result of divorce analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DivorceResult {
    /// Eligible to apply
    pub eligible: bool,
    /// Grounds established
    pub grounds_established: bool,
    /// Children arrangements adequate
    pub children_arrangements_adequate: bool,
    /// Can proceed
    pub can_proceed: bool,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Parenting Analyzer
// ============================================================================

/// Analyzer for parenting matters
pub struct ParentingAnalyzer;

impl ParentingAnalyzer {
    /// Analyze parenting case
    pub fn analyze(facts: &ParentingFacts) -> ParentingResult {
        let primary = Self::assess_primary_considerations(facts);
        let additional = Self::assess_additional_considerations(facts);
        let recommendations = Self::make_recommendations(facts, &primary);

        let reasoning = Self::build_reasoning(facts, &primary, &additional);

        ParentingResult {
            primary_considerations: primary,
            additional_considerations: additional,
            recommended_responsibility: recommendations.responsibility,
            recommended_time: recommendations.time,
            family_violence_finding: facts.family_violence_alleged && facts.family_violence_found,
            reasoning,
        }
    }

    /// Assess primary considerations (s.60CC(2))
    fn assess_primary_considerations(facts: &ParentingFacts) -> Vec<PrimaryConsideration> {
        let mut considerations = Vec::new();

        // Always consider both primary considerations
        if !facts.family_violence_alleged || !facts.family_violence_found {
            considerations.push(PrimaryConsideration::MeaningfulRelationship);
        }

        if facts.family_violence_alleged || facts.child_abuse_alleged {
            considerations.push(PrimaryConsideration::ProtectionFromHarm);
        }

        considerations
    }

    /// Assess additional considerations (s.60CC(3))
    fn assess_additional_considerations(facts: &ParentingFacts) -> Vec<AdditionalConsideration> {
        let mut considerations = Vec::new();

        if facts.child_views_expressed {
            considerations.push(AdditionalConsideration::ChildViews);
        }

        if facts.indigenous_heritage {
            considerations.push(AdditionalConsideration::IndigenousHeritage);
        }

        if facts.family_violence_alleged {
            considerations.push(AdditionalConsideration::FamilyViolence);
        }

        considerations.push(AdditionalConsideration::RelationshipWithParents);
        considerations.push(AdditionalConsideration::ParentalCapacity);

        considerations
    }

    /// Make recommendations
    fn make_recommendations(
        facts: &ParentingFacts,
        primary: &[PrimaryConsideration],
    ) -> ParentingRecommendations {
        let responsibility = if primary.contains(&PrimaryConsideration::ProtectionFromHarm) {
            ParentalResponsibility::Sole
        } else if facts.high_conflict {
            ParentalResponsibility::SharedSpecific
        } else {
            ParentalResponsibility::EqualShared
        };

        let time = if facts.family_violence_found {
            TimeArrangement::Supervised
        } else if facts.equal_care_practical {
            TimeArrangement::EqualTime
        } else if facts.substantial_time_practical {
            TimeArrangement::SubstantialSignificant
        } else {
            TimeArrangement::WeekendHalfHolidays
        };

        ParentingRecommendations {
            responsibility,
            time,
        }
    }

    /// Build reasoning
    fn build_reasoning(
        facts: &ParentingFacts,
        primary: &[PrimaryConsideration],
        _additional: &[AdditionalConsideration],
    ) -> String {
        let mut parts = Vec::new();

        parts.push("Parenting analysis (Family Law Act 1975 Part VII)".to_string());
        parts.push("Best interests of child paramount (s.60CA)".to_string());

        // Primary considerations
        for consideration in primary {
            match consideration {
                PrimaryConsideration::MeaningfulRelationship => {
                    parts.push("Primary: benefit of meaningful relationship with both parents (s.60CC(2)(a))".to_string());
                }
                PrimaryConsideration::ProtectionFromHarm => {
                    parts.push(
                        "Primary: protection from harm from family violence/abuse (s.60CC(2)(b))"
                            .to_string(),
                    );
                }
            }
        }

        if facts.family_violence_found {
            parts.push("Family violence finding - affects arrangements".to_string());
        }

        if facts.child_views_expressed {
            parts.push(format!(
                "Child's views considered (age {} - weight given accordingly)",
                facts.child_age
            ));
        }

        parts.join(". ")
    }
}

/// Facts for parenting analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ParentingFacts {
    /// Child's age
    pub child_age: u32,
    /// Child views expressed
    pub child_views_expressed: bool,
    /// Family violence alleged
    pub family_violence_alleged: bool,
    /// Family violence found
    pub family_violence_found: bool,
    /// Child abuse alleged
    pub child_abuse_alleged: bool,
    /// Indigenous heritage
    pub indigenous_heritage: bool,
    /// High conflict
    pub high_conflict: bool,
    /// Equal care practical
    pub equal_care_practical: bool,
    /// Substantial time practical
    pub substantial_time_practical: bool,
}

struct ParentingRecommendations {
    responsibility: ParentalResponsibility,
    time: TimeArrangement,
}

/// Result of parenting analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParentingResult {
    /// Primary considerations
    pub primary_considerations: Vec<PrimaryConsideration>,
    /// Additional considerations
    pub additional_considerations: Vec<AdditionalConsideration>,
    /// Recommended responsibility allocation
    pub recommended_responsibility: ParentalResponsibility,
    /// Recommended time arrangement
    pub recommended_time: TimeArrangement,
    /// Family violence finding
    pub family_violence_finding: bool,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Property Analyzer
// ============================================================================

/// Analyzer for property settlement
pub struct PropertyAnalyzer;

impl PropertyAnalyzer {
    /// Analyze property settlement (four-step approach)
    pub fn analyze(facts: &PropertyFacts) -> PropertyResult {
        // Step 1: Identify and value property pool
        let pool = Self::calculate_pool(facts);

        // Step 2: Assess contributions
        let contribution_split = Self::assess_contributions(facts);

        // Step 3: Assess future needs
        let future_adjustment = Self::assess_future_needs(facts);

        // Step 4: Just and equitable
        let final_split = Self::apply_just_equitable(contribution_split, future_adjustment, facts);

        let reasoning = Self::build_reasoning(facts, pool, contribution_split, future_adjustment);

        PropertyResult {
            total_pool: pool,
            contribution_based_split: contribution_split,
            future_needs_adjustment: future_adjustment,
            final_percentage_party1: final_split,
            final_percentage_party2: 100.0 - final_split,
            just_and_equitable: true,
            reasoning,
        }
    }

    /// Calculate property pool
    fn calculate_pool(facts: &PropertyFacts) -> f64 {
        facts.total_assets - facts.total_liabilities
    }

    /// Assess contributions (s.79(4))
    fn assess_contributions(facts: &PropertyFacts) -> f64 {
        let mut party1_percentage = 50.0;

        // Financial contributions
        if facts.party1_financial_contribution > facts.party2_financial_contribution {
            let diff = facts.party1_financial_contribution - facts.party2_financial_contribution;
            let total = facts.party1_financial_contribution + facts.party2_financial_contribution;
            if total > 0.0 {
                party1_percentage += (diff / total) * 15.0;
            }
        }

        // Homemaker contributions (per Mallet v Mallet)
        if facts.party2_primary_homemaker {
            party1_percentage -= 10.0;
        }

        party1_percentage.clamp(30.0, 70.0)
    }

    /// Assess future needs (s.75(2))
    fn assess_future_needs(facts: &PropertyFacts) -> f64 {
        let mut adjustment = 0.0;

        // Care of children
        if facts.party1_primary_carer {
            adjustment -= 5.0;
        } else if facts.party2_primary_carer {
            adjustment += 5.0;
        }

        // Income disparity
        if facts.party1_income > facts.party2_income * 2.0 {
            adjustment += 5.0;
        } else if facts.party2_income > facts.party1_income * 2.0 {
            adjustment -= 5.0;
        }

        // Health
        if facts.party1_health_issues {
            adjustment -= 3.0;
        }
        if facts.party2_health_issues {
            adjustment += 3.0;
        }

        adjustment
    }

    /// Apply just and equitable (Stanford)
    fn apply_just_equitable(contribution: f64, future_adj: f64, _facts: &PropertyFacts) -> f64 {
        let result = contribution + future_adj;
        result.clamp(20.0, 80.0)
    }

    /// Build reasoning
    fn build_reasoning(
        _facts: &PropertyFacts,
        pool: f64,
        contribution: f64,
        future_adj: f64,
    ) -> String {
        let mut parts = Vec::new();

        parts.push("Property settlement analysis (s.79 Family Law Act)".to_string());
        parts.push("Four-step approach per Stanford v Stanford [2012] HCA 52".to_string());

        parts.push(format!("Step 1 - Net property pool: ${:.2}", pool));
        parts.push(format!(
            "Step 2 - Contribution-based split: {:.1}%/{:.1}%",
            contribution,
            100.0 - contribution
        ));
        parts.push(format!(
            "Step 3 - Future needs adjustment: {:.1}%",
            future_adj
        ));
        parts.push(format!(
            "Step 4 - Just and equitable: {:.1}%/{:.1}%",
            contribution + future_adj,
            100.0 - (contribution + future_adj)
        ));

        parts.join(". ")
    }
}

/// Facts for property analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PropertyFacts {
    /// Total assets
    pub total_assets: f64,
    /// Total liabilities
    pub total_liabilities: f64,
    /// Party 1 financial contribution
    pub party1_financial_contribution: f64,
    /// Party 2 financial contribution
    pub party2_financial_contribution: f64,
    /// Party 2 primary homemaker
    pub party2_primary_homemaker: bool,
    /// Party 1 primary carer
    pub party1_primary_carer: bool,
    /// Party 2 primary carer
    pub party2_primary_carer: bool,
    /// Party 1 income
    pub party1_income: f64,
    /// Party 2 income
    pub party2_income: f64,
    /// Party 1 health issues
    pub party1_health_issues: bool,
    /// Party 2 health issues
    pub party2_health_issues: bool,
    /// Marriage duration years
    pub marriage_duration_years: u32,
}

/// Result of property analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyResult {
    /// Total property pool
    pub total_pool: f64,
    /// Contribution-based split
    pub contribution_based_split: f64,
    /// Future needs adjustment
    pub future_needs_adjustment: f64,
    /// Final percentage for party 1
    pub final_percentage_party1: f64,
    /// Final percentage for party 2
    pub final_percentage_party2: f64,
    /// Just and equitable
    pub just_and_equitable: bool,
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
    fn test_divorce_grounds() {
        let facts = DivorceFacts {
            jurisdictional_connection: true,
            separated_12_months: true,
            reasonable_likelihood_cohabitation: false,
            children_under_18: false,
            ..Default::default()
        };

        let result = DivorceAnalyzer::analyze(&facts);
        assert!(result.can_proceed);
    }

    #[test]
    fn test_divorce_with_children() {
        let facts = DivorceFacts {
            jurisdictional_connection: true,
            separated_12_months: true,
            reasonable_likelihood_cohabitation: false,
            children_under_18: true,
            proper_children_arrangements: false,
        };

        let result = DivorceAnalyzer::analyze(&facts);
        assert!(!result.can_proceed);
    }

    #[test]
    fn test_parenting_family_violence() {
        let facts = ParentingFacts {
            family_violence_alleged: true,
            family_violence_found: true,
            ..Default::default()
        };

        let result = ParentingAnalyzer::analyze(&facts);
        assert!(result.family_violence_finding);
        assert_eq!(result.recommended_time, TimeArrangement::Supervised);
    }

    #[test]
    fn test_parenting_equal_time() {
        let facts = ParentingFacts {
            equal_care_practical: true,
            ..Default::default()
        };

        let result = ParentingAnalyzer::analyze(&facts);
        assert_eq!(result.recommended_time, TimeArrangement::EqualTime);
        assert_eq!(
            result.recommended_responsibility,
            ParentalResponsibility::EqualShared
        );
    }

    #[test]
    fn test_property_equal_contributions() {
        let facts = PropertyFacts {
            total_assets: 1_000_000.0,
            total_liabilities: 200_000.0,
            party1_financial_contribution: 400_000.0,
            party2_financial_contribution: 400_000.0,
            ..Default::default()
        };

        let result = PropertyAnalyzer::analyze(&facts);
        assert_eq!(result.total_pool, 800_000.0);
        // Should be approximately 50/50
        assert!((result.final_percentage_party1 - 50.0).abs() < 15.0);
    }

    #[test]
    fn test_property_homemaker_contribution() {
        let facts = PropertyFacts {
            total_assets: 800_000.0,
            party2_primary_homemaker: true,
            ..Default::default()
        };

        let result = PropertyAnalyzer::analyze(&facts);
        // Homemaker contribution should reduce party 1's share
        assert!(result.final_percentage_party1 < 50.0);
    }
}
