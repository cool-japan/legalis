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
}
