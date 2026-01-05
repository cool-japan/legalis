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

impl ImpactReport {
    /// Creates a new impact report with default values.
    pub fn new(total_population: usize) -> Self {
        Self {
            executive_summary: ExecutiveSummary {
                total_population,
                directly_affected: 0,
                indirectly_affected: 0,
                compliance_rate: 0.0,
                net_economic_impact: 0.0,
            },
            affected_populations: AffectedPopulations {
                by_type: HashMap::new(),
                by_income: HashMap::new(),
                by_region: HashMap::new(),
            },
            equity_analysis: EquityAnalysis {
                equity_score: 1.0,
                vulnerable_impact: VulnerableImpact {
                    low_income_impact: 0.0,
                    elderly_impact: 0.0,
                    small_business_impact: 0.0,
                    mitigation_recommendations: Vec::new(),
                },
                disparate_impact: DisparateImpact {
                    detected: false,
                    affected_groups: Vec::new(),
                    severity: 0.0,
                },
                fairness_metrics: FairnessMetrics {
                    demographic_parity: 0.0,
                    equal_opportunity: 0.0,
                    treatment_equality: 0.0,
                },
            },
            compliance_burden: ComplianceBurden {
                avg_time_burden_hours: 0.0,
                avg_financial_burden: 0.0,
                total_compliance_cost: 0.0,
                burden_distribution: Vec::new(),
            },
            economic_impact: EconomicImpactSummary {
                projected_revenue: 0.0,
                total_costs: 0.0,
                net_benefit: 0.0,
                benefit_cost_ratio: 0.0,
                gdp_impact: 0.0,
            },
            administrative_costs: AdministrativeCosts {
                setup_costs: 0.0,
                annual_operating_costs: 0.0,
                enforcement_costs: 0.0,
                total_administrative_burden: 0.0,
            },
            unintended_consequences: Vec::new(),
        }
    }

    /// Updates the compliance burden from a list of individual burdens.
    pub fn calculate_compliance_burden(&mut self, burdens: &[f64]) {
        if burdens.is_empty() {
            return;
        }

        let total: f64 = burdens.iter().sum();
        let avg = total / burdens.len() as f64;

        self.compliance_burden.avg_financial_burden = avg;
        self.compliance_burden.total_compliance_cost = total;

        // Calculate burden distribution (percentiles)
        let mut sorted = burdens.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let percentiles = [10, 25, 50, 75, 90, 95, 99];
        self.compliance_burden.burden_distribution = percentiles
            .iter()
            .map(|&p| {
                let idx = (p as f64 / 100.0 * sorted.len() as f64) as usize;
                let idx = idx.min(sorted.len() - 1);
                (p, sorted[idx])
            })
            .collect();
    }

    /// Calculates fairness metrics from compliance rates by group.
    pub fn calculate_fairness_metrics(
        &mut self,
        group_a_rate: f64,
        group_b_rate: f64,
        group_a_benefit: f64,
        group_b_benefit: f64,
    ) {
        // Demographic parity: difference in compliance rates
        self.equity_analysis.fairness_metrics.demographic_parity =
            (group_a_rate - group_b_rate).abs();

        // Equal opportunity: difference in benefits received
        self.equity_analysis.fairness_metrics.equal_opportunity =
            (group_a_benefit - group_b_benefit).abs();

        // Treatment equality: variance in outcomes
        let mean = (group_a_benefit + group_b_benefit) / 2.0;
        self.equity_analysis.fairness_metrics.treatment_equality =
            ((group_a_benefit - mean).powi(2) + (group_b_benefit - mean).powi(2)) / 2.0;
    }

    /// Adds a population segment to the affected populations.
    pub fn add_population_segment(
        &mut self,
        category: &str,
        segment_name: impl Into<String>,
        count: usize,
        avg_burden: f64,
        compliance_rate: f64,
    ) {
        let segment = PopulationSegment {
            count,
            percentage: if self.executive_summary.total_population > 0 {
                count as f64 / self.executive_summary.total_population as f64 * 100.0
            } else {
                0.0
            },
            avg_burden,
            compliance_rate,
        };

        match category {
            "type" => {
                self.affected_populations
                    .by_type
                    .insert(segment_name.into(), segment);
            }
            "income" => {
                self.affected_populations
                    .by_income
                    .insert(segment_name.into(), segment);
            }
            "region" => {
                self.affected_populations
                    .by_region
                    .insert(segment_name.into(), segment);
            }
            _ => {}
        }
    }

    /// Adds an unintended consequence.
    pub fn add_unintended_consequence(
        &mut self,
        description: impl Into<String>,
        severity: f64,
        affected_count: usize,
        mitigation: Vec<String>,
    ) {
        self.unintended_consequences.push(UnintendedConsequence {
            description: description.into(),
            severity: severity.clamp(0.0, 1.0),
            affected_count,
            mitigation,
        });
    }

    /// Calculates the overall equity score based on fairness metrics.
    pub fn calculate_equity_score(&mut self) {
        let metrics = &self.equity_analysis.fairness_metrics;

        // Lower values are better (more equitable)
        let avg_disparity =
            (metrics.demographic_parity + metrics.equal_opportunity + metrics.treatment_equality)
                / 3.0;

        // Convert to 0-1 score where 1 is perfect equity
        self.equity_analysis.equity_score = (1.0 - avg_disparity.min(1.0)).max(0.0);
    }

    /// Calculates economic impact summary.
    pub fn calculate_economic_impact(
        &mut self,
        revenue: f64,
        costs: f64,
        admin_costs: &AdministrativeCosts,
    ) {
        let total_costs = costs + admin_costs.total_administrative_burden;

        self.economic_impact.projected_revenue = revenue;
        self.economic_impact.total_costs = total_costs;
        self.economic_impact.net_benefit = revenue - total_costs;
        self.economic_impact.benefit_cost_ratio = if total_costs > 0.0 {
            revenue / total_costs
        } else {
            0.0
        };
    }
}

impl AdministrativeCosts {
    /// Creates a new administrative costs structure.
    pub fn new(setup_costs: f64, annual_operating_costs: f64, enforcement_costs: f64) -> Self {
        let total = setup_costs + annual_operating_costs + enforcement_costs;
        Self {
            setup_costs,
            annual_operating_costs,
            enforcement_costs,
            total_administrative_burden: total,
        }
    }

    /// Updates the total administrative burden.
    pub fn update_total(&mut self) {
        self.total_administrative_burden =
            self.setup_costs + self.annual_operating_costs + self.enforcement_costs;
    }
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

    #[test]
    fn test_impact_report_creation() {
        let report = ImpactReport::new(1000);

        assert_eq!(report.executive_summary.total_population, 1000);
        assert_eq!(report.executive_summary.directly_affected, 0);
        assert_eq!(report.unintended_consequences.len(), 0);
    }

    #[test]
    fn test_calculate_compliance_burden() {
        let mut report = ImpactReport::new(100);
        let burdens = vec![100.0, 200.0, 300.0, 400.0, 500.0];

        report.calculate_compliance_burden(&burdens);

        assert_eq!(report.compliance_burden.avg_financial_burden, 300.0);
        assert_eq!(report.compliance_burden.total_compliance_cost, 1500.0);
        assert!(!report.compliance_burden.burden_distribution.is_empty());
    }

    #[test]
    fn test_calculate_compliance_burden_empty() {
        let mut report = ImpactReport::new(100);
        let burdens: Vec<f64> = vec![];

        report.calculate_compliance_burden(&burdens);

        assert_eq!(report.compliance_burden.avg_financial_burden, 0.0);
        assert_eq!(report.compliance_burden.total_compliance_cost, 0.0);
    }

    #[test]
    fn test_burden_distribution_percentiles() {
        let mut report = ImpactReport::new(100);
        let burdens: Vec<f64> = (1..=100).map(|x| x as f64).collect();

        report.calculate_compliance_burden(&burdens);

        // Check that percentiles are in ascending order
        let mut prev = 0.0;
        for (percentile, value) in &report.compliance_burden.burden_distribution {
            assert!(value >= &prev);
            assert!(*percentile <= 100);
            prev = *value;
        }
    }

    #[test]
    fn test_calculate_fairness_metrics() {
        let mut report = ImpactReport::new(100);

        // Group A: 80% compliance, 1000 benefit
        // Group B: 60% compliance, 800 benefit
        report.calculate_fairness_metrics(0.8, 0.6, 1000.0, 800.0);

        assert!((report.equity_analysis.fairness_metrics.demographic_parity - 0.2).abs() < 1e-6);
        assert!((report.equity_analysis.fairness_metrics.equal_opportunity - 200.0).abs() < 1e-6);
        assert!(report.equity_analysis.fairness_metrics.treatment_equality > 0.0);
    }

    #[test]
    fn test_calculate_equity_score() {
        let mut report = ImpactReport::new(100);

        // Perfect equity
        report.calculate_fairness_metrics(0.8, 0.8, 1000.0, 1000.0);
        report.calculate_equity_score();

        // With no disparity, equity score should be 1.0
        assert!((report.equity_analysis.equity_score - 1.0).abs() < 1e-6);

        // High disparity
        report.calculate_fairness_metrics(0.9, 0.1, 10000.0, 100.0);
        report.calculate_equity_score();

        // With high disparity, equity score should be lower
        assert!(report.equity_analysis.equity_score < 0.5);
    }

    #[test]
    fn test_add_population_segment() {
        let mut report = ImpactReport::new(1000);

        report.add_population_segment("type", "Individual", 600, 150.0, 0.85);
        report.add_population_segment("type", "Business", 400, 500.0, 0.75);

        assert_eq!(report.affected_populations.by_type.len(), 2);
        assert!(
            report
                .affected_populations
                .by_type
                .contains_key("Individual")
        );
        assert!(report.affected_populations.by_type.contains_key("Business"));

        let individual = &report.affected_populations.by_type["Individual"];
        assert_eq!(individual.count, 600);
        assert_eq!(individual.percentage, 60.0);
        assert_eq!(individual.avg_burden, 150.0);
        assert_eq!(individual.compliance_rate, 0.85);
    }

    #[test]
    fn test_add_population_segment_categories() {
        let mut report = ImpactReport::new(1000);

        report.add_population_segment("type", "TypeA", 100, 100.0, 0.8);
        report.add_population_segment("income", "Low", 200, 150.0, 0.7);
        report.add_population_segment("region", "North", 300, 120.0, 0.85);

        assert_eq!(report.affected_populations.by_type.len(), 1);
        assert_eq!(report.affected_populations.by_income.len(), 1);
        assert_eq!(report.affected_populations.by_region.len(), 1);
    }

    #[test]
    fn test_add_unintended_consequence() {
        let mut report = ImpactReport::new(1000);

        report.add_unintended_consequence(
            "Small businesses may close",
            0.7,
            50,
            vec!["Provide tax credits".to_string()],
        );

        assert_eq!(report.unintended_consequences.len(), 1);

        let consequence = &report.unintended_consequences[0];
        assert_eq!(consequence.description, "Small businesses may close");
        assert_eq!(consequence.severity, 0.7);
        assert_eq!(consequence.affected_count, 50);
        assert_eq!(consequence.mitigation.len(), 1);
    }

    #[test]
    fn test_unintended_consequence_severity_clamp() {
        let mut report = ImpactReport::new(1000);

        // Test clamping to [0.0, 1.0]
        report.add_unintended_consequence("Test", 1.5, 10, vec![]);
        assert_eq!(report.unintended_consequences[0].severity, 1.0);

        report.add_unintended_consequence("Test2", -0.5, 10, vec![]);
        assert_eq!(report.unintended_consequences[1].severity, 0.0);
    }

    #[test]
    fn test_calculate_economic_impact() {
        let mut report = ImpactReport::new(1000);
        let admin_costs = AdministrativeCosts::new(10000.0, 5000.0, 3000.0);

        report.calculate_economic_impact(100000.0, 50000.0, &admin_costs);

        assert_eq!(report.economic_impact.projected_revenue, 100000.0);
        assert_eq!(report.economic_impact.total_costs, 68000.0); // 50000 + 18000
        assert_eq!(report.economic_impact.net_benefit, 32000.0); // 100000 - 68000
        assert!((report.economic_impact.benefit_cost_ratio - 1.47).abs() < 0.01);
    }

    #[test]
    fn test_administrative_costs_new() {
        let costs = AdministrativeCosts::new(10000.0, 5000.0, 3000.0);

        assert_eq!(costs.setup_costs, 10000.0);
        assert_eq!(costs.annual_operating_costs, 5000.0);
        assert_eq!(costs.enforcement_costs, 3000.0);
        assert_eq!(costs.total_administrative_burden, 18000.0);
    }

    #[test]
    fn test_administrative_costs_update_total() {
        let mut costs = AdministrativeCosts::new(10000.0, 5000.0, 3000.0);

        costs.setup_costs = 15000.0;
        costs.update_total();

        assert_eq!(costs.total_administrative_burden, 23000.0);
    }

    #[test]
    fn test_economic_impact_zero_costs() {
        let mut report = ImpactReport::new(1000);
        let admin_costs = AdministrativeCosts::new(0.0, 0.0, 0.0);

        report.calculate_economic_impact(100000.0, 0.0, &admin_costs);

        assert_eq!(report.economic_impact.benefit_cost_ratio, 0.0);
    }

    #[test]
    fn test_full_impact_report_workflow() {
        let mut report = ImpactReport::new(10000);

        // Set up populations
        report.add_population_segment("type", "Individual", 6000, 100.0, 0.85);
        report.add_population_segment("type", "Business", 4000, 500.0, 0.75);
        report.add_population_segment("income", "Low", 3000, 150.0, 0.70);
        report.add_population_segment("income", "High", 7000, 250.0, 0.88);

        // Calculate burden
        let burdens: Vec<f64> = (1..=10000).map(|x| x as f64 * 0.1).collect();
        report.calculate_compliance_burden(&burdens);

        // Calculate fairness with more reasonable values
        report.calculate_fairness_metrics(0.85, 0.75, 1200.0, 1100.0);
        report.calculate_equity_score();

        // Calculate economic impact
        let admin_costs = AdministrativeCosts::new(50000.0, 25000.0, 15000.0);
        report.calculate_economic_impact(5000000.0, 3000000.0, &admin_costs);

        // Add consequences
        report.add_unintended_consequence(
            "Job losses in affected sectors",
            0.6,
            500,
            vec!["Job retraining programs".to_string()],
        );

        // Verify report structure
        assert!(report.compliance_burden.total_compliance_cost > 0.0);
        assert!(report.equity_analysis.equity_score >= 0.0);
        assert!(report.equity_analysis.equity_score <= 1.0);
        assert!(report.economic_impact.net_benefit != 0.0);
        assert_eq!(report.unintended_consequences.len(), 1);
        assert_eq!(report.affected_populations.by_type.len(), 2);
        assert_eq!(report.affected_populations.by_income.len(), 2);
    }
}
