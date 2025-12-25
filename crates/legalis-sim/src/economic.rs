//! Economic modeling for legal simulations.
//!
//! This module provides data structures for analyzing the economic impacts of legal statutes,
//! including tax revenue projections, compliance costs, and cost-benefit analysis.
//!
//! Note: Full implementations require adaptation to the current legalis-core API.
//! This module currently provides the data structures and interfaces.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Economic impact analysis result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicImpact {
    /// Total tax revenue projected.
    pub tax_revenue: f64,
    /// Total compliance costs.
    pub compliance_costs: f64,
    /// Administrative costs.
    pub administrative_costs: f64,
    /// Net economic benefit (revenue - costs).
    pub net_benefit: f64,
    /// Revenue by entity type.
    pub revenue_by_type: HashMap<String, f64>,
    /// Costs by entity type.
    pub costs_by_type: HashMap<String, f64>,
    /// Distributional impacts.
    pub distributional_impact: DistributionalImpact,
}

/// Distributional impact across income/wealth groups.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionalImpact {
    /// Impact by quintile (5 groups).
    pub quintile_impacts: Vec<QuintileImpact>,
    /// Gini coefficient change.
    pub gini_change: f64,
    /// Progressive/regressive indicator.
    pub progressivity: Progressivity,
}

/// Impact on a specific income quintile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuintileImpact {
    /// Quintile number (1-5, 1 is lowest income).
    pub quintile: u8,
    /// Average tax burden per entity.
    pub avg_tax_burden: f64,
    /// Average compliance cost per entity.
    pub avg_compliance_cost: f64,
    /// Number of entities in quintile.
    pub entity_count: usize,
    /// Total impact as percentage of quintile income.
    pub impact_pct_of_income: f64,
}

/// Progressivity indicator for tax/regulatory burden.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Progressivity {
    /// Burden increases with income.
    Progressive,
    /// Burden is proportional to income.
    Proportional,
    /// Burden decreases with income.
    Regressive,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_structures() {
        let impact = QuintileImpact {
            quintile: 1,
            avg_tax_burden: 1000.0,
            avg_compliance_cost: 500.0,
            entity_count: 100,
            impact_pct_of_income: 5.0,
        };

        assert_eq!(impact.quintile, 1);
        assert_eq!(impact.entity_count, 100);
    }

    #[test]
    fn test_progressivity() {
        let prog = Progressivity::Progressive;
        assert_eq!(prog, Progressivity::Progressive);
    }

    #[test]
    fn test_economic_impact_creation() {
        let mut revenue_by_type = HashMap::new();
        revenue_by_type.insert("individual".to_string(), 100_000.0);
        revenue_by_type.insert("corporate".to_string(), 500_000.0);

        let mut costs_by_type = HashMap::new();
        costs_by_type.insert("individual".to_string(), 10_000.0);
        costs_by_type.insert("corporate".to_string(), 50_000.0);

        let quintile_impacts = vec![
            QuintileImpact {
                quintile: 1,
                avg_tax_burden: 500.0,
                avg_compliance_cost: 100.0,
                entity_count: 200,
                impact_pct_of_income: 3.0,
            },
            QuintileImpact {
                quintile: 5,
                avg_tax_burden: 5000.0,
                avg_compliance_cost: 1000.0,
                entity_count: 200,
                impact_pct_of_income: 2.0,
            },
        ];

        let distributional_impact = DistributionalImpact {
            quintile_impacts,
            gini_change: -0.01,
            progressivity: Progressivity::Progressive,
        };

        let impact = EconomicImpact {
            tax_revenue: 600_000.0,
            compliance_costs: 60_000.0,
            administrative_costs: 40_000.0,
            net_benefit: 500_000.0,
            revenue_by_type: revenue_by_type.clone(),
            costs_by_type: costs_by_type.clone(),
            distributional_impact: distributional_impact.clone(),
        };

        assert_eq!(impact.tax_revenue, 600_000.0);
        assert_eq!(impact.compliance_costs, 60_000.0);
        assert_eq!(impact.administrative_costs, 40_000.0);
        assert_eq!(impact.net_benefit, 500_000.0);
        assert_eq!(impact.revenue_by_type.len(), 2);
        assert_eq!(impact.costs_by_type.len(), 2);
        assert_eq!(impact.distributional_impact.quintile_impacts.len(), 2);
    }

    #[test]
    fn test_distributional_impact() {
        let quintile_impacts = vec![
            QuintileImpact {
                quintile: 1,
                avg_tax_burden: 500.0,
                avg_compliance_cost: 100.0,
                entity_count: 200,
                impact_pct_of_income: 6.0,
            },
            QuintileImpact {
                quintile: 2,
                avg_tax_burden: 1000.0,
                avg_compliance_cost: 150.0,
                entity_count: 200,
                impact_pct_of_income: 5.0,
            },
            QuintileImpact {
                quintile: 3,
                avg_tax_burden: 2000.0,
                avg_compliance_cost: 200.0,
                entity_count: 200,
                impact_pct_of_income: 4.0,
            },
            QuintileImpact {
                quintile: 4,
                avg_tax_burden: 3000.0,
                avg_compliance_cost: 300.0,
                entity_count: 200,
                impact_pct_of_income: 3.0,
            },
            QuintileImpact {
                quintile: 5,
                avg_tax_burden: 5000.0,
                avg_compliance_cost: 500.0,
                entity_count: 200,
                impact_pct_of_income: 2.0,
            },
        ];

        let impact = DistributionalImpact {
            quintile_impacts: quintile_impacts.clone(),
            gini_change: 0.02,
            progressivity: Progressivity::Regressive,
        };

        assert_eq!(impact.quintile_impacts.len(), 5);
        assert_eq!(impact.progressivity, Progressivity::Regressive);
        assert!(impact.gini_change > 0.0);

        // Verify quintiles are ordered correctly
        for (i, quintile_impact) in impact.quintile_impacts.iter().enumerate() {
            assert_eq!(quintile_impact.quintile, (i + 1) as u8);
        }

        // Regressive tax: impact_pct_of_income should decrease with quintile
        assert!(
            impact.quintile_impacts[0].impact_pct_of_income
                > impact.quintile_impacts[4].impact_pct_of_income
        );
    }

    #[test]
    fn test_progressive_tax() {
        let quintile_impacts = vec![
            QuintileImpact {
                quintile: 1,
                avg_tax_burden: 100.0,
                avg_compliance_cost: 50.0,
                entity_count: 200,
                impact_pct_of_income: 1.0,
            },
            QuintileImpact {
                quintile: 5,
                avg_tax_burden: 10_000.0,
                avg_compliance_cost: 500.0,
                entity_count: 200,
                impact_pct_of_income: 5.0,
            },
        ];

        let impact = DistributionalImpact {
            quintile_impacts: quintile_impacts.clone(),
            gini_change: -0.03,
            progressivity: Progressivity::Progressive,
        };

        assert_eq!(impact.progressivity, Progressivity::Progressive);
        assert!(impact.gini_change < 0.0);

        // Progressive tax: impact_pct_of_income should increase with quintile
        assert!(
            impact.quintile_impacts[0].impact_pct_of_income
                < impact.quintile_impacts[1].impact_pct_of_income
        );
    }

    #[test]
    fn test_proportional_tax() {
        let quintile_impacts = (1..=5)
            .map(|q| QuintileImpact {
                quintile: q,
                avg_tax_burden: 1000.0 * q as f64,
                avg_compliance_cost: 100.0 * q as f64,
                entity_count: 200,
                impact_pct_of_income: 5.0,
            })
            .collect();

        let impact = DistributionalImpact {
            quintile_impacts,
            gini_change: 0.0,
            progressivity: Progressivity::Proportional,
        };

        assert_eq!(impact.progressivity, Progressivity::Proportional);
        assert_eq!(impact.gini_change, 0.0);

        // Proportional tax: all quintiles have same impact_pct_of_income
        for quintile_impact in &impact.quintile_impacts {
            assert_eq!(quintile_impact.impact_pct_of_income, 5.0);
        }
    }

    #[test]
    fn test_net_benefit_calculation() {
        let impact = EconomicImpact {
            tax_revenue: 1_000_000.0,
            compliance_costs: 150_000.0,
            administrative_costs: 50_000.0,
            net_benefit: 1_000_000.0 - 150_000.0 - 50_000.0,
            revenue_by_type: HashMap::new(),
            costs_by_type: HashMap::new(),
            distributional_impact: DistributionalImpact {
                quintile_impacts: vec![],
                gini_change: 0.0,
                progressivity: Progressivity::Proportional,
            },
        };

        assert_eq!(impact.net_benefit, 800_000.0);
        assert_eq!(
            impact.net_benefit,
            impact.tax_revenue - impact.compliance_costs - impact.administrative_costs
        );
    }

    #[test]
    fn test_serialization() {
        let impact = QuintileImpact {
            quintile: 1,
            avg_tax_burden: 1000.0,
            avg_compliance_cost: 500.0,
            entity_count: 100,
            impact_pct_of_income: 5.0,
        };

        let json = serde_json::to_string(&impact).unwrap();
        let deserialized: QuintileImpact = serde_json::from_str(&json).unwrap();

        assert_eq!(impact.quintile, deserialized.quintile);
        assert_eq!(impact.avg_tax_burden, deserialized.avg_tax_burden);
        assert_eq!(impact.entity_count, deserialized.entity_count);
    }
}
