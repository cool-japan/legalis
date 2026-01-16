//! Canada Family Law - Support Analysis
//!
//! Child support and spousal support calculations.

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use super::types::{
    Section7Expense, SpousalSupportBasis, SpousalSupportType, SsagFormula, SupportDuration,
    UndueHardshipFactor,
};
use crate::common::Province;

// ============================================================================
// Child Support Analysis
// ============================================================================

/// Facts for child support calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChildSupportFacts {
    /// Payor's annual income (cents)
    pub payor_income_cents: i64,
    /// Recipient's annual income (cents)
    pub recipient_income_cents: i64,
    /// Number of children
    pub num_children: u32,
    /// Ages of children
    pub children_ages: Vec<u32>,
    /// Province (for table lookup)
    pub province: Province,
    /// Parenting time percentage with payor
    pub payor_time_percentage: f64,
    /// Section 7 expenses
    pub section_7_expenses: Vec<Section7ExpenseItem>,
    /// Undue hardship claimed
    pub undue_hardship: Option<UndueHardshipClaim>,
}

/// Section 7 expense item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section7ExpenseItem {
    /// Type of expense
    pub expense_type: Section7Expense,
    /// Annual amount (cents)
    pub annual_amount_cents: i64,
    /// Description
    pub description: String,
}

/// Undue hardship claim
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndueHardshipClaim {
    /// Factors claimed
    pub factors: Vec<UndueHardshipFactor>,
    /// Claimed amounts (cents)
    pub amounts: Vec<i64>,
    /// Household standard comparison
    pub household_standards_compared: bool,
}

/// Result of child support calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChildSupportResult {
    /// Table amount (cents/month)
    pub table_amount_cents: i64,
    /// Section 7 contribution (cents/month)
    pub section_7_contribution_cents: i64,
    /// Total monthly support (cents)
    pub total_monthly_cents: i64,
    /// Calculation type used
    pub calculation_type: ChildSupportCalculationType,
    /// Set-off amount if shared custody (cents)
    pub set_off_amount_cents: Option<i64>,
    /// Reasoning
    pub reasoning: String,
}

/// Child support calculation type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChildSupportCalculationType {
    /// Basic table amount
    Basic,
    /// Shared custody (over 40% time)
    SharedCustody,
    /// Split custody
    SplitCustody,
    /// Undue hardship adjustment
    UndueHardship,
}

/// Child support analyzer
pub struct ChildSupportAnalyzer;

impl ChildSupportAnalyzer {
    /// Calculate child support
    pub fn calculate(facts: &ChildSupportFacts) -> ChildSupportResult {
        // Determine calculation type
        let calc_type = if facts.payor_time_percentage >= 40.0 {
            ChildSupportCalculationType::SharedCustody
        } else {
            ChildSupportCalculationType::Basic
        };

        // Get table amount
        let table_amount = Self::lookup_table_amount(
            facts.payor_income_cents,
            facts.num_children,
            &facts.province,
        );

        // Calculate Section 7 contributions
        let section_7_total: i64 = facts
            .section_7_expenses
            .iter()
            .map(|e| e.annual_amount_cents / 12)
            .sum();

        // Proportional share based on incomes
        let total_income = facts.payor_income_cents + facts.recipient_income_cents;
        let payor_share = if total_income > 0 {
            facts.payor_income_cents as f64 / total_income as f64
        } else {
            0.5
        };
        let section_7_contribution = (section_7_total as f64 * payor_share) as i64;

        // Handle shared custody
        let (final_table, set_off) = if calc_type == ChildSupportCalculationType::SharedCustody {
            let recipient_table = Self::lookup_table_amount(
                facts.recipient_income_cents,
                facts.num_children,
                &facts.province,
            );
            let set_off_amount = table_amount - recipient_table;
            // Adjust for shared custody - reduce by proportion of time
            let adjustment = (set_off_amount as f64 * 0.7) as i64; // Simplified adjustment
            (adjustment.max(0), Some(set_off_amount))
        } else {
            (table_amount, None)
        };

        let total_monthly = final_table + section_7_contribution;

        let reasoning = Self::build_reasoning(
            facts,
            &calc_type,
            final_table,
            section_7_contribution,
            set_off,
        );

        ChildSupportResult {
            table_amount_cents: final_table,
            section_7_contribution_cents: section_7_contribution,
            total_monthly_cents: total_monthly,
            calculation_type: calc_type,
            set_off_amount_cents: set_off,
            reasoning,
        }
    }

    /// Lookup table amount (simplified - actual tables are complex)
    fn lookup_table_amount(income_cents: i64, num_children: u32, _province: &Province) -> i64 {
        // Simplified calculation - actual Federal Child Support Guidelines tables
        // are based on income ranges and vary by province
        let annual_income = income_cents / 100;

        let base_rate = match num_children {
            1 => 0.10,                                                  // ~10% of income for 1 child
            2 => 0.16,                                                  // ~16% for 2 children
            3 => 0.20,                                                  // ~20% for 3 children
            _ => 0.22 + (num_children.saturating_sub(4) as f64 * 0.02), // 22%+ for 4+
        };

        let annual_support = (annual_income as f64 * base_rate) as i64;
        let monthly = annual_support * 100 / 12; // Convert back to cents

        monthly.max(0)
    }

    /// Build reasoning
    fn build_reasoning(
        facts: &ChildSupportFacts,
        calc_type: &ChildSupportCalculationType,
        table: i64,
        section_7: i64,
        set_off: Option<i64>,
    ) -> String {
        let mut parts = Vec::new();

        parts.push(format!(
            "Federal Child Support Guidelines - {} province",
            facts.province.abbreviation()
        ));
        parts.push(format!(
            "Payor income: ${:.2}/year, {} children",
            facts.payor_income_cents as f64 / 100.0,
            facts.num_children
        ));

        match calc_type {
            ChildSupportCalculationType::SharedCustody => {
                parts.push(format!(
                    "Shared custody calculation (payor has {:.0}% parenting time)",
                    facts.payor_time_percentage
                ));
                if let Some(offset) = set_off {
                    parts.push(format!("Set-off amount: ${:.2}", offset as f64 / 100.0));
                }
            }
            ChildSupportCalculationType::Basic => {
                parts.push("Basic table amount calculation".to_string());
            }
            _ => {}
        }

        parts.push(format!("Table amount: ${:.2}/month", table as f64 / 100.0));

        if section_7 > 0 {
            parts.push(format!(
                "Section 7 expenses contribution: ${:.2}/month (proportional to income)",
                section_7 as f64 / 100.0
            ));
        }

        parts.join(". ")
    }
}

// ============================================================================
// Spousal Support Analysis
// ============================================================================

/// Facts for spousal support calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpousalSupportFacts {
    /// Higher earner's income (cents)
    pub higher_income_cents: i64,
    /// Lower earner's income (cents)
    pub lower_income_cents: i64,
    /// Length of relationship (years)
    pub relationship_years: u32,
    /// Age of recipient at separation
    pub recipient_age: u32,
    /// Whether there are dependent children
    pub has_children: bool,
    /// Child support amount being paid (cents/month)
    pub child_support_cents: Option<i64>,
    /// Basis for support claim
    pub basis: Vec<SpousalSupportBasis>,
    /// Province
    pub province: Province,
    /// Whether recipient sacrificed career
    pub career_sacrifice: bool,
    /// Health issues affecting work ability
    pub health_limitations: bool,
}

/// Result of spousal support calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpousalSupportResult {
    /// Support type
    pub support_type: SpousalSupportType,
    /// Formula used
    pub formula: SsagFormula,
    /// Monthly range (low, mid, high in cents)
    pub monthly_range: SpousalSupportRange,
    /// Duration recommendation
    pub duration: DurationRange,
    /// Reasoning
    pub reasoning: String,
}

/// Spousal support range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpousalSupportRange {
    /// Low end (cents/month)
    pub low_cents: i64,
    /// Mid (cents/month)
    pub mid_cents: i64,
    /// High end (cents/month)
    pub high_cents: i64,
}

/// Duration range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DurationRange {
    /// Minimum duration
    pub minimum: SupportDuration,
    /// Maximum duration
    pub maximum: SupportDuration,
    /// Recommended
    pub recommended: SupportDuration,
}

/// Spousal support analyzer
pub struct SpousalSupportAnalyzer;

impl SpousalSupportAnalyzer {
    /// Calculate spousal support using SSAG
    pub fn calculate(facts: &SpousalSupportFacts) -> SpousalSupportResult {
        // Determine support type
        let support_type = Self::determine_support_type(facts);

        // Determine formula
        let formula = if facts.has_children {
            SsagFormula::WithChildSupport
        } else {
            SsagFormula::WithoutChildSupport
        };

        // Calculate range
        let range = Self::calculate_range(facts, &formula);

        // Calculate duration
        let duration = Self::calculate_duration(facts);

        let reasoning = Self::build_reasoning(facts, &support_type, &formula, &range, &duration);

        SpousalSupportResult {
            support_type,
            formula,
            monthly_range: range,
            duration,
            reasoning,
        }
    }

    /// Determine support type
    fn determine_support_type(facts: &SpousalSupportFacts) -> SpousalSupportType {
        if facts.career_sacrifice
            || facts
                .basis
                .contains(&SpousalSupportBasis::EconomicDisadvantage)
        {
            SpousalSupportType::Compensatory
        } else if facts.health_limitations
            || facts.basis.contains(&SpousalSupportBasis::EconomicHardship)
        {
            SpousalSupportType::NonCompensatory
        } else {
            SpousalSupportType::Compensatory
        }
    }

    /// Calculate SSAG range
    fn calculate_range(facts: &SpousalSupportFacts, formula: &SsagFormula) -> SpousalSupportRange {
        let income_diff = facts.higher_income_cents - facts.lower_income_cents;

        let (low_pct, high_pct) = match formula {
            SsagFormula::WithoutChildSupport => {
                // 1.5% - 2% of income diff per year of marriage
                let years = facts.relationship_years.min(25) as f64;
                (0.015 * years, 0.02 * years)
            }
            SsagFormula::WithChildSupport => {
                // More complex - simplified here
                // Range is 40-46% of payor's individual NDI minus recipient's NDI
                (0.30, 0.40)
            }
            SsagFormula::CustodialPayor => (0.25, 0.35),
        };

        let monthly_low = ((income_diff as f64 * low_pct) / 12.0) as i64;
        let monthly_high = ((income_diff as f64 * high_pct) / 12.0) as i64;
        let monthly_mid = (monthly_low + monthly_high) / 2;

        SpousalSupportRange {
            low_cents: monthly_low.max(0),
            mid_cents: monthly_mid.max(0),
            high_cents: monthly_high.max(0),
        }
    }

    /// Calculate duration
    fn calculate_duration(facts: &SpousalSupportFacts) -> DurationRange {
        let years = facts.relationship_years;

        // SSAG duration: 0.5 - 1 year per year of marriage
        let min_months = (years as f64 * 6.0) as u32;
        let max_months = years * 12;

        // Rule of 65 - if age + years >= 65, indefinite
        let rule_of_65 = facts.recipient_age + years >= 65;
        // 20 year rule - if 20+ years, indefinite
        let twenty_year_rule = years >= 20;

        let minimum = SupportDuration::TimeLimited { months: min_months };
        let maximum = if rule_of_65 || twenty_year_rule {
            SupportDuration::Indefinite
        } else {
            SupportDuration::TimeLimited { months: max_months }
        };

        let recommended = if rule_of_65 || twenty_year_rule {
            SupportDuration::Indefinite
        } else {
            SupportDuration::TimeLimited {
                months: (min_months + max_months) / 2,
            }
        };

        DurationRange {
            minimum,
            maximum,
            recommended,
        }
    }

    /// Build reasoning
    fn build_reasoning(
        facts: &SpousalSupportFacts,
        support_type: &SpousalSupportType,
        formula: &SsagFormula,
        range: &SpousalSupportRange,
        duration: &DurationRange,
    ) -> String {
        let mut parts = Vec::new();

        parts.push(format!(
            "Spousal Support Advisory Guidelines (SSAG) - {:?} formula",
            formula
        ));
        parts.push(format!("Support type: {:?}", support_type));
        parts.push(format!(
            "Income difference: ${:.2}/year",
            (facts.higher_income_cents - facts.lower_income_cents) as f64 / 100.0
        ));
        parts.push(format!(
            "Relationship length: {} years",
            facts.relationship_years
        ));

        parts.push(format!(
            "Monthly range: ${:.2} - ${:.2}",
            range.low_cents as f64 / 100.0,
            range.high_cents as f64 / 100.0
        ));

        match &duration.recommended {
            SupportDuration::Indefinite => {
                parts.push("Duration: Indefinite (Rule of 65 or 20+ year marriage)".to_string());
            }
            SupportDuration::TimeLimited { months } => {
                parts.push(format!("Recommended duration: {} months", months));
            }
            _ => {}
        }

        parts.join(". ")
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_child_support() {
        let facts = ChildSupportFacts {
            payor_income_cents: 10_000_000,    // $100,000
            recipient_income_cents: 5_000_000, // $50,000
            num_children: 2,
            children_ages: vec![8, 10],
            province: Province::Ontario,
            payor_time_percentage: 20.0,
            section_7_expenses: vec![],
            undue_hardship: None,
        };

        let result = ChildSupportAnalyzer::calculate(&facts);

        assert_eq!(result.calculation_type, ChildSupportCalculationType::Basic);
        assert!(result.table_amount_cents > 0);
    }

    #[test]
    fn test_shared_custody_calculation() {
        let facts = ChildSupportFacts {
            payor_income_cents: 12_000_000,    // $120,000
            recipient_income_cents: 6_000_000, // $60,000
            num_children: 1,
            children_ages: vec![12],
            province: Province::BritishColumbia,
            payor_time_percentage: 45.0, // Shared custody
            section_7_expenses: vec![],
            undue_hardship: None,
        };

        let result = ChildSupportAnalyzer::calculate(&facts);

        assert_eq!(
            result.calculation_type,
            ChildSupportCalculationType::SharedCustody
        );
        assert!(result.set_off_amount_cents.is_some());
    }

    #[test]
    fn test_section_7_expenses() {
        let facts = ChildSupportFacts {
            payor_income_cents: 8_000_000,
            recipient_income_cents: 4_000_000,
            num_children: 1,
            children_ages: vec![6],
            province: Province::Ontario,
            payor_time_percentage: 15.0,
            section_7_expenses: vec![Section7ExpenseItem {
                expense_type: Section7Expense::ChildCare,
                annual_amount_cents: 1_200_000, // $12,000/year
                description: "Daycare".to_string(),
            }],
            undue_hardship: None,
        };

        let result = ChildSupportAnalyzer::calculate(&facts);

        assert!(result.section_7_contribution_cents > 0);
    }

    #[test]
    fn test_spousal_support_no_children() {
        let facts = SpousalSupportFacts {
            higher_income_cents: 15_000_000, // $150,000
            lower_income_cents: 4_000_000,   // $40,000
            relationship_years: 15,
            recipient_age: 50,
            has_children: false,
            child_support_cents: None,
            basis: vec![SpousalSupportBasis::EconomicDisadvantage],
            province: Province::Ontario,
            career_sacrifice: true,
            health_limitations: false,
        };

        let result = SpousalSupportAnalyzer::calculate(&facts);

        assert_eq!(result.formula, SsagFormula::WithoutChildSupport);
        assert!(result.monthly_range.mid_cents > 0);
    }

    #[test]
    fn test_rule_of_65_indefinite() {
        let facts = SpousalSupportFacts {
            higher_income_cents: 10_000_000,
            lower_income_cents: 3_000_000,
            relationship_years: 20,
            recipient_age: 55, // 55 + 20 = 75 > 65
            has_children: false,
            child_support_cents: None,
            basis: vec![SpousalSupportBasis::EconomicDisadvantage],
            province: Province::Alberta,
            career_sacrifice: true,
            health_limitations: false,
        };

        let result = SpousalSupportAnalyzer::calculate(&facts);

        assert!(matches!(
            result.duration.recommended,
            SupportDuration::Indefinite
        ));
    }
}
