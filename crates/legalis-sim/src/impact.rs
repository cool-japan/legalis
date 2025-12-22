//! Impact assessment tools for regulatory analysis.
//!
//! This module provides data structures for analyzing the impacts of legal statutes,
//! including equity analysis, compliance burden, and regulatory impact reports.
//!
//! Note: Full implementations require adaptation to the current legalis-core API.
//! This module currently provides the data structures and interfaces.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Regulatory impact assessment report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactReport {
    /// Executive summary.
    pub executive_summary: ExecutiveSummary,
    /// Affected populations breakdown.
    pub affected_populations: AffectedPopulations,
    /// Equity analysis.
    pub equity_analysis: EquityAnalysis,
    /// Compliance burden metrics.
    pub compliance_burden: ComplianceBurden,
    /// Economic impacts.
    pub economic_impact: EconomicImpactSummary,
    /// Administrative costs.
    pub administrative_costs: AdministrativeCosts,
    /// Unintended consequences.
    pub unintended_consequences: Vec<UnintendedConsequence>,
}

/// Executive summary of impact assessment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutiveSummary {
    /// Total population size.
    pub total_population: usize,
    /// Number of entities directly affected.
    pub directly_affected: usize,
    /// Number of entities indirectly affected.
    pub indirectly_affected: usize,
    /// Overall compliance rate.
    pub compliance_rate: f64,
    /// Net economic impact.
    pub net_economic_impact: f64,
}

/// Breakdown of affected populations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffectedPopulations {
    /// Breakdown by entity type.
    pub by_type: HashMap<String, PopulationSegment>,
    /// Breakdown by income level.
    pub by_income: HashMap<String, PopulationSegment>,
    /// Breakdown by geographic region.
    pub by_region: HashMap<String, PopulationSegment>,
}

/// Population segment analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationSegment {
    /// Number of entities in segment.
    pub count: usize,
    /// Percentage of total population.
    pub percentage: f64,
    /// Average compliance burden.
    pub avg_burden: f64,
    /// Compliance rate.
    pub compliance_rate: f64,
}

/// Equity analysis results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquityAnalysis {
    /// Distributional equity score (0-1, higher is more equitable).
    pub equity_score: f64,
    /// Impact on vulnerable populations.
    pub vulnerable_impact: VulnerableImpact,
    /// Disparate impact indicators.
    pub disparate_impact: DisparateImpact,
    /// Fairness metrics.
    pub fairness_metrics: FairnessMetrics,
}

/// Impact on vulnerable populations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnerableImpact {
    /// Impact on low-income population.
    pub low_income_impact: f64,
    /// Impact on elderly population.
    pub elderly_impact: f64,
    /// Impact on small businesses.
    pub small_business_impact: f64,
    /// Recommendations for mitigation.
    pub mitigation_recommendations: Vec<String>,
}

/// Disparate impact analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisparateImpact {
    /// Whether disparate impact detected.
    pub detected: bool,
    /// Affected groups.
    pub affected_groups: Vec<String>,
    /// Severity score (0-1).
    pub severity: f64,
}

/// Fairness metrics for policy evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FairnessMetrics {
    /// Demographic parity difference.
    pub demographic_parity: f64,
    /// Equal opportunity difference.
    pub equal_opportunity: f64,
    /// Treatment equality (variance in outcomes).
    pub treatment_equality: f64,
}

/// Compliance burden analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceBurden {
    /// Average time burden (hours per entity).
    pub avg_time_burden_hours: f64,
    /// Average financial burden per entity.
    pub avg_financial_burden: f64,
    /// Total compliance cost estimate.
    pub total_compliance_cost: f64,
    /// Burden distribution (by percentile).
    pub burden_distribution: Vec<(u8, f64)>,
}

/// Economic impact summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicImpactSummary {
    /// Projected revenue (if applicable).
    pub projected_revenue: f64,
    /// Total compliance costs.
    pub total_costs: f64,
    /// Net benefit.
    pub net_benefit: f64,
    /// Benefit-cost ratio.
    pub benefit_cost_ratio: f64,
    /// GDP impact estimate.
    pub gdp_impact: f64,
}

/// Administrative costs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdministrativeCosts {
    /// Initial setup costs.
    pub setup_costs: f64,
    /// Annual operating costs.
    pub annual_operating_costs: f64,
    /// Enforcement costs.
    pub enforcement_costs: f64,
    /// Total administrative burden.
    pub total_administrative_burden: f64,
}

/// Unintended consequence identified.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnintendedConsequence {
    /// Description of the consequence.
    pub description: String,
    /// Severity (0-1).
    pub severity: f64,
    /// Affected population count.
    pub affected_count: usize,
    /// Mitigation suggestions.
    pub mitigation: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_structures() {
        let summary = ExecutiveSummary {
            total_population: 1000,
            directly_affected: 800,
            indirectly_affected: 200,
            compliance_rate: 0.9,
            net_economic_impact: 1000000.0,
        };

        assert_eq!(summary.total_population, 1000);
        assert_eq!(summary.directly_affected, 800);
    }
}
