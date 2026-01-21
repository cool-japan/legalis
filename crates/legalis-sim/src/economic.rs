//! Economic modeling for legal simulations.
//!
//! This module provides data structures for analyzing the economic impacts of legal statutes,
//! including tax revenue projections, compliance costs, and cost-benefit analysis.
//!
//! ## Features
//! - Tax revenue and compliance cost analysis
//! - Distributional impact assessment
//! - Macroeconomic indicators integration
//! - Labor market simulation
//! - Housing market effects modeling
//! - Inflation adjustment
//! - GDP impact estimation

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

// ============================================================================
// Macroeconomic Indicators (v0.1.3)
// ============================================================================

/// Macroeconomic indicators for context and analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroeconomicIndicators {
    /// GDP (Gross Domestic Product) in billions
    pub gdp: f64,
    /// GDP growth rate (annual percentage)
    pub gdp_growth_rate: f64,
    /// Unemployment rate (percentage)
    pub unemployment_rate: f64,
    /// Inflation rate (annual percentage)
    pub inflation_rate: f64,
    /// Interest rate (central bank rate, percentage)
    pub interest_rate: f64,
    /// Consumer Price Index
    pub cpi: f64,
    /// Labor force participation rate (percentage)
    pub labor_force_participation: f64,
    /// Trade balance (exports - imports, billions)
    pub trade_balance: f64,
    /// Government debt as percentage of GDP
    pub debt_to_gdp: f64,
    /// Year/period for these indicators
    pub year: u32,
}

impl MacroeconomicIndicators {
    /// Create new macroeconomic indicators
    pub fn new(year: u32, gdp: f64, gdp_growth_rate: f64, inflation_rate: f64) -> Self {
        Self {
            gdp,
            gdp_growth_rate,
            unemployment_rate: 0.0,
            inflation_rate,
            interest_rate: 0.0,
            cpi: 100.0,
            labor_force_participation: 0.0,
            trade_balance: 0.0,
            debt_to_gdp: 0.0,
            year,
        }
    }

    /// Check if economy is in recession (two consecutive quarters of negative growth)
    pub fn is_recession(&self) -> bool {
        self.gdp_growth_rate < 0.0
    }

    /// Check if economy is overheating (high growth + high inflation)
    pub fn is_overheating(&self) -> bool {
        self.gdp_growth_rate > 4.0 && self.inflation_rate > 3.0
    }

    /// Get output gap estimate (actual - potential GDP, as % of potential)
    /// Simplified using Okun's law approximation
    pub fn output_gap(&self, natural_unemployment_rate: f64) -> f64 {
        // Okun's coefficient (typically -2 to -3)
        let okun_coefficient = -2.5;
        okun_coefficient * (self.unemployment_rate - natural_unemployment_rate)
    }

    /// Apply inflation adjustment to nominal value
    pub fn real_value(&self, nominal_value: f64, base_cpi: f64) -> f64 {
        nominal_value * (base_cpi / self.cpi)
    }

    /// Project next period's indicators (simple extrapolation)
    pub fn project_next_period(&self) -> Self {
        Self {
            gdp: self.gdp * (1.0 + self.gdp_growth_rate / 100.0),
            gdp_growth_rate: self.gdp_growth_rate * 0.9, // Mean reversion
            unemployment_rate: self.unemployment_rate,
            inflation_rate: self.inflation_rate * 0.95, // Mean reversion
            interest_rate: self.interest_rate,
            cpi: self.cpi * (1.0 + self.inflation_rate / 100.0),
            labor_force_participation: self.labor_force_participation,
            trade_balance: self.trade_balance,
            debt_to_gdp: self.debt_to_gdp,
            year: self.year + 1,
        }
    }
}

// ============================================================================
// Labor Market Simulation (v0.1.3)
// ============================================================================

/// Labor market state and dynamics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaborMarket {
    /// Total labor force
    pub labor_force: u64,
    /// Number of employed individuals
    pub employed: u64,
    /// Number of unemployed individuals
    pub unemployed: u64,
    /// Job openings
    pub job_openings: u64,
    /// Average wage
    pub average_wage: f64,
    /// Wage growth rate (annual percentage)
    pub wage_growth_rate: f64,
    /// Labor force by sector
    pub employment_by_sector: HashMap<String, u64>,
    /// Wage by sector
    pub wage_by_sector: HashMap<String, f64>,
}

impl LaborMarket {
    /// Create a new labor market
    pub fn new(labor_force: u64, employed: u64) -> Self {
        let unemployed = labor_force.saturating_sub(employed);
        Self {
            labor_force,
            employed,
            unemployed,
            job_openings: 0,
            average_wage: 50_000.0,
            wage_growth_rate: 2.0,
            employment_by_sector: HashMap::new(),
            wage_by_sector: HashMap::new(),
        }
    }

    /// Get unemployment rate
    pub fn unemployment_rate(&self) -> f64 {
        if self.labor_force == 0 {
            0.0
        } else {
            (self.unemployed as f64 / self.labor_force as f64) * 100.0
        }
    }

    /// Get employment rate
    pub fn employment_rate(&self) -> f64 {
        if self.labor_force == 0 {
            0.0
        } else {
            (self.employed as f64 / self.labor_force as f64) * 100.0
        }
    }

    /// Get job vacancy rate
    pub fn vacancy_rate(&self) -> f64 {
        let total_positions = self.employed + self.job_openings;
        if total_positions == 0 {
            0.0
        } else {
            (self.job_openings as f64 / total_positions as f64) * 100.0
        }
    }

    /// Simulate job creation
    pub fn create_jobs(&mut self, count: u64) {
        self.job_openings += count;
    }

    /// Simulate hiring (fill job openings)
    pub fn hire(&mut self, count: u64) {
        let hires = count.min(self.job_openings).min(self.unemployed);
        self.employed += hires;
        self.unemployed = self.unemployed.saturating_sub(hires);
        self.job_openings = self.job_openings.saturating_sub(hires);
    }

    /// Simulate job losses
    pub fn job_losses(&mut self, count: u64) {
        let losses = count.min(self.employed);
        self.employed = self.employed.saturating_sub(losses);
        self.unemployed += losses;
    }

    /// Apply wage growth
    pub fn apply_wage_growth(&mut self) {
        self.average_wage *= 1.0 + self.wage_growth_rate / 100.0;
        for wage in self.wage_by_sector.values_mut() {
            *wage *= 1.0 + self.wage_growth_rate / 100.0;
        }
    }

    /// Get total wage bill
    pub fn total_wage_bill(&self) -> f64 {
        self.employed as f64 * self.average_wage
    }
}

// ============================================================================
// Housing Market (v0.1.3)
// ============================================================================

/// Housing market state and dynamics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HousingMarket {
    /// Median home price
    pub median_price: f64,
    /// Home price appreciation rate (annual percentage)
    pub price_growth_rate: f64,
    /// Housing inventory (homes available for sale)
    pub inventory: u64,
    /// Monthly sales volume
    pub monthly_sales: u64,
    /// Mortgage interest rate (percentage)
    pub mortgage_rate: f64,
    /// Rental vacancy rate (percentage)
    pub rental_vacancy_rate: f64,
    /// Median rent
    pub median_rent: f64,
    /// Home ownership rate (percentage)
    pub ownership_rate: f64,
}

impl HousingMarket {
    /// Create a new housing market
    pub fn new(median_price: f64, mortgage_rate: f64) -> Self {
        Self {
            median_price,
            price_growth_rate: 3.0,
            inventory: 10_000,
            monthly_sales: 500,
            mortgage_rate,
            rental_vacancy_rate: 5.0,
            median_rent: 1_500.0,
            ownership_rate: 65.0,
        }
    }

    /// Get months of supply (inventory / monthly sales)
    pub fn months_of_supply(&self) -> f64 {
        if self.monthly_sales == 0 {
            0.0
        } else {
            self.inventory as f64 / self.monthly_sales as f64
        }
    }

    /// Check if market is seller's market (low inventory)
    pub fn is_sellers_market(&self) -> bool {
        self.months_of_supply() < 4.0
    }

    /// Check if market is buyer's market (high inventory)
    pub fn is_buyers_market(&self) -> bool {
        self.months_of_supply() > 6.0
    }

    /// Calculate monthly mortgage payment (30-year fixed)
    pub fn monthly_mortgage_payment(&self, down_payment_pct: f64) -> f64 {
        let loan_amount = self.median_price * (1.0 - down_payment_pct / 100.0);
        let monthly_rate = self.mortgage_rate / 100.0 / 12.0;
        let num_payments = 30.0 * 12.0;

        if monthly_rate == 0.0 {
            loan_amount / num_payments
        } else {
            let factor = (1.0 + monthly_rate).powf(num_payments);
            loan_amount * (monthly_rate * factor) / (factor - 1.0)
        }
    }

    /// Calculate price-to-rent ratio
    pub fn price_to_rent_ratio(&self) -> f64 {
        self.median_price / (self.median_rent * 12.0)
    }

    /// Apply price appreciation
    pub fn apply_price_growth(&mut self) {
        self.median_price *= 1.0 + self.price_growth_rate / 100.0;
        self.median_rent *= 1.0 + (self.price_growth_rate / 2.0) / 100.0;
    }

    /// Simulate sales activity
    pub fn simulate_sales(&mut self, sales: u64) {
        self.monthly_sales = sales;
        self.inventory = self.inventory.saturating_sub(sales);
    }

    /// Add new inventory
    pub fn add_inventory(&mut self, count: u64) {
        self.inventory += count;
    }
}

// ============================================================================
// Inflation Adjustment (v0.1.3)
// ============================================================================

/// Inflation adjuster for converting between nominal and real values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InflationAdjuster {
    /// Base year for CPI
    pub base_year: u32,
    /// Base CPI value (typically 100.0)
    pub base_cpi: f64,
    /// CPI values by year
    pub cpi_by_year: HashMap<u32, f64>,
    /// Inflation rate by year
    pub inflation_by_year: HashMap<u32, f64>,
}

impl InflationAdjuster {
    /// Create a new inflation adjuster
    pub fn new(base_year: u32) -> Self {
        let mut cpi_by_year = HashMap::new();
        cpi_by_year.insert(base_year, 100.0);

        Self {
            base_year,
            base_cpi: 100.0,
            cpi_by_year,
            inflation_by_year: HashMap::new(),
        }
    }

    /// Set CPI for a year
    pub fn set_cpi(&mut self, year: u32, cpi: f64) {
        self.cpi_by_year.insert(year, cpi);

        // Calculate inflation rate if previous year exists
        if year > self.base_year
            && let Some(prev_cpi) = self.cpi_by_year.get(&(year - 1))
        {
            let inflation = ((cpi - prev_cpi) / prev_cpi) * 100.0;
            self.inflation_by_year.insert(year, inflation);
        }
    }

    /// Convert nominal value to real value (base year dollars)
    pub fn to_real_value(&self, nominal_value: f64, year: u32) -> f64 {
        let cpi = self
            .cpi_by_year
            .get(&year)
            .copied()
            .unwrap_or(self.base_cpi);
        nominal_value * (self.base_cpi / cpi)
    }

    /// Convert real value to nominal value (current year dollars)
    pub fn to_nominal_value(&self, real_value: f64, year: u32) -> f64 {
        let cpi = self
            .cpi_by_year
            .get(&year)
            .copied()
            .unwrap_or(self.base_cpi);
        real_value * (cpi / self.base_cpi)
    }

    /// Get inflation rate for a year
    pub fn get_inflation_rate(&self, year: u32) -> Option<f64> {
        self.inflation_by_year.get(&year).copied()
    }

    /// Project CPI for future year using average inflation rate
    pub fn project_cpi(&mut self, target_year: u32, avg_inflation_rate: f64) {
        let years_ahead = target_year.saturating_sub(self.base_year) as f64;
        let projected_cpi = self.base_cpi * (1.0 + avg_inflation_rate / 100.0).powf(years_ahead);
        self.set_cpi(target_year, projected_cpi);
    }

    /// Calculate cumulative inflation between two years
    pub fn cumulative_inflation(&self, from_year: u32, to_year: u32) -> f64 {
        let from_cpi = self
            .cpi_by_year
            .get(&from_year)
            .copied()
            .unwrap_or(self.base_cpi);
        let to_cpi = self
            .cpi_by_year
            .get(&to_year)
            .copied()
            .unwrap_or(self.base_cpi);
        ((to_cpi - from_cpi) / from_cpi) * 100.0
    }
}

// ============================================================================
// GDP Impact Estimation (v0.1.3)
// ============================================================================

/// GDP impact analysis from policy changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GdpImpactAnalysis {
    /// Baseline GDP (before policy)
    pub baseline_gdp: f64,
    /// Projected GDP (after policy)
    pub projected_gdp: f64,
    /// GDP impact (change in GDP)
    pub gdp_impact: f64,
    /// GDP impact as percentage
    pub gdp_impact_pct: f64,
    /// Impact by component (consumption, investment, government, exports, imports)
    pub component_impacts: HashMap<GdpComponent, f64>,
    /// Multiplier effect
    pub fiscal_multiplier: f64,
    /// Time horizon (years)
    pub time_horizon: u32,
}

/// GDP components
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GdpComponent {
    /// Consumer spending
    Consumption,
    /// Business investment
    Investment,
    /// Government spending
    Government,
    /// Exports
    Exports,
    /// Imports
    Imports,
}

impl GdpImpactAnalysis {
    /// Create a new GDP impact analysis
    pub fn new(baseline_gdp: f64, fiscal_multiplier: f64, time_horizon: u32) -> Self {
        Self {
            baseline_gdp,
            projected_gdp: baseline_gdp,
            gdp_impact: 0.0,
            gdp_impact_pct: 0.0,
            component_impacts: HashMap::new(),
            fiscal_multiplier,
            time_horizon,
        }
    }

    /// Add direct government spending impact
    pub fn add_government_spending(&mut self, spending: f64) {
        let impact = spending * self.fiscal_multiplier;
        self.component_impacts
            .insert(GdpComponent::Government, impact);
        self.recalculate_total_impact();
    }

    /// Add tax policy impact on consumption
    pub fn add_tax_impact(&mut self, tax_change: f64, marginal_propensity_to_consume: f64) {
        // Tax decrease increases disposable income and consumption
        let consumption_impact =
            -tax_change * marginal_propensity_to_consume * self.fiscal_multiplier;
        self.component_impacts
            .insert(GdpComponent::Consumption, consumption_impact);
        self.recalculate_total_impact();
    }

    /// Add investment impact (from business tax changes, regulations, etc.)
    pub fn add_investment_impact(&mut self, investment_change: f64) {
        let impact = investment_change * self.fiscal_multiplier;
        self.component_impacts
            .insert(GdpComponent::Investment, impact);
        self.recalculate_total_impact();
    }

    /// Add trade impact
    pub fn add_trade_impact(&mut self, export_change: f64, import_change: f64) {
        self.component_impacts
            .insert(GdpComponent::Exports, export_change);
        self.component_impacts
            .insert(GdpComponent::Imports, import_change);
        self.recalculate_total_impact();
    }

    /// Recalculate total GDP impact
    fn recalculate_total_impact(&mut self) {
        self.gdp_impact = self.component_impacts.values().sum();
        self.projected_gdp = self.baseline_gdp + self.gdp_impact;
        self.gdp_impact_pct = (self.gdp_impact / self.baseline_gdp) * 100.0;
    }

    /// Get employment impact estimate (Okun's law)
    pub fn estimate_employment_impact(&self, okun_coefficient: f64) -> f64 {
        // Okun's law: 1% GDP change ≈ 0.5% employment change (typical coefficient -2)
        self.gdp_impact_pct / okun_coefficient.abs()
    }

    /// Project impact over multiple years with decay
    pub fn project_over_time(&self, decay_rate: f64) -> Vec<f64> {
        let mut projections = Vec::new();
        for year in 0..self.time_horizon {
            let factor = (1.0 - decay_rate / 100.0).powi(year as i32);
            projections.push(self.gdp_impact * factor);
        }
        projections
    }
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

    // Tests for Economic Extensions (v0.1.3)

    #[test]
    fn test_macroeconomic_indicators_creation() {
        let indicators = MacroeconomicIndicators::new(2024, 25_000.0, 2.5, 2.0);
        assert_eq!(indicators.year, 2024);
        assert_eq!(indicators.gdp, 25_000.0);
        assert_eq!(indicators.gdp_growth_rate, 2.5);
        assert_eq!(indicators.inflation_rate, 2.0);
    }

    #[test]
    fn test_macroeconomic_recession_detection() {
        let indicators = MacroeconomicIndicators::new(2024, 25_000.0, -1.5, 2.0);
        assert!(indicators.is_recession());

        let healthy = MacroeconomicIndicators::new(2024, 25_000.0, 2.5, 2.0);
        assert!(!healthy.is_recession());
    }

    #[test]
    fn test_macroeconomic_overheating() {
        let mut indicators = MacroeconomicIndicators::new(2024, 25_000.0, 5.0, 4.0);
        assert!(indicators.is_overheating());

        indicators.gdp_growth_rate = 2.0;
        assert!(!indicators.is_overheating());
    }

    #[test]
    fn test_macroeconomic_output_gap() {
        let mut indicators = MacroeconomicIndicators::new(2024, 25_000.0, 2.5, 2.0);
        indicators.unemployment_rate = 6.0;
        let gap = indicators.output_gap(4.0);
        assert!(gap < 0.0); // Negative gap when unemployment is above natural rate
    }

    #[test]
    fn test_macroeconomic_real_value() {
        let indicators = MacroeconomicIndicators::new(2024, 25_000.0, 2.5, 2.0);
        let real = indicators.real_value(100_000.0, 100.0);
        assert_eq!(real, 100_000.0); // Same CPI
    }

    #[test]
    fn test_macroeconomic_projection() {
        let indicators = MacroeconomicIndicators::new(2024, 25_000.0, 2.5, 2.0);
        let next = indicators.project_next_period();
        assert_eq!(next.year, 2025);
        assert!(next.gdp > indicators.gdp);
    }

    #[test]
    fn test_labor_market_creation() {
        let market = LaborMarket::new(100_000, 95_000);
        assert_eq!(market.labor_force, 100_000);
        assert_eq!(market.employed, 95_000);
        assert_eq!(market.unemployed, 5_000);
    }

    #[test]
    fn test_labor_market_unemployment_rate() {
        let market = LaborMarket::new(100_000, 95_000);
        assert_eq!(market.unemployment_rate(), 5.0);
    }

    #[test]
    fn test_labor_market_employment_rate() {
        let market = LaborMarket::new(100_000, 95_000);
        assert_eq!(market.employment_rate(), 95.0);
    }

    #[test]
    fn test_labor_market_job_creation() {
        let mut market = LaborMarket::new(100_000, 95_000);
        market.create_jobs(1_000);
        assert_eq!(market.job_openings, 1_000);
    }

    #[test]
    fn test_labor_market_hiring() {
        let mut market = LaborMarket::new(100_000, 95_000);
        market.create_jobs(1_000);
        market.hire(500);
        assert_eq!(market.employed, 95_500);
        assert_eq!(market.unemployed, 4_500);
        assert_eq!(market.job_openings, 500);
    }

    #[test]
    fn test_labor_market_job_losses() {
        let mut market = LaborMarket::new(100_000, 95_000);
        market.job_losses(1_000);
        assert_eq!(market.employed, 94_000);
        assert_eq!(market.unemployed, 6_000);
    }

    #[test]
    fn test_labor_market_wage_growth() {
        let mut market = LaborMarket::new(100_000, 95_000);
        let initial_wage = market.average_wage;
        market.apply_wage_growth();
        assert!(market.average_wage > initial_wage);
    }

    #[test]
    fn test_labor_market_total_wage_bill() {
        let market = LaborMarket::new(100_000, 95_000);
        let expected = 95_000.0 * market.average_wage;
        assert_eq!(market.total_wage_bill(), expected);
    }

    #[test]
    fn test_housing_market_creation() {
        let market = HousingMarket::new(400_000.0, 6.5);
        assert_eq!(market.median_price, 400_000.0);
        assert_eq!(market.mortgage_rate, 6.5);
    }

    #[test]
    fn test_housing_market_months_of_supply() {
        let market = HousingMarket::new(400_000.0, 6.5);
        let months = market.months_of_supply();
        assert_eq!(months, 10_000.0 / 500.0);
    }

    #[test]
    fn test_housing_market_sellers_market() {
        let mut market = HousingMarket::new(400_000.0, 6.5);
        market.inventory = 1_500;
        assert!(market.is_sellers_market());
    }

    #[test]
    fn test_housing_market_buyers_market() {
        let mut market = HousingMarket::new(400_000.0, 6.5);
        market.inventory = 4_000;
        assert!(market.is_buyers_market());
    }

    #[test]
    fn test_housing_market_mortgage_payment() {
        let market = HousingMarket::new(400_000.0, 6.5);
        let payment = market.monthly_mortgage_payment(20.0);
        assert!(payment > 0.0);
        assert!(payment < 5_000.0); // Reasonable range
    }

    #[test]
    fn test_housing_market_price_to_rent() {
        let market = HousingMarket::new(400_000.0, 6.5);
        let ratio = market.price_to_rent_ratio();
        assert!(ratio > 10.0);
    }

    #[test]
    fn test_housing_market_price_growth() {
        let mut market = HousingMarket::new(400_000.0, 6.5);
        let initial_price = market.median_price;
        market.apply_price_growth();
        assert!(market.median_price > initial_price);
    }

    #[test]
    fn test_housing_market_sales_simulation() {
        let mut market = HousingMarket::new(400_000.0, 6.5);
        let initial_inventory = market.inventory;
        market.simulate_sales(100);
        assert_eq!(market.inventory, initial_inventory - 100);
    }

    #[test]
    fn test_inflation_adjuster_creation() {
        let adjuster = InflationAdjuster::new(2020);
        assert_eq!(adjuster.base_year, 2020);
        assert_eq!(adjuster.base_cpi, 100.0);
    }

    #[test]
    fn test_inflation_adjuster_set_cpi() {
        let mut adjuster = InflationAdjuster::new(2020);
        adjuster.set_cpi(2021, 102.0);
        assert_eq!(adjuster.cpi_by_year.get(&2021), Some(&102.0));
    }

    #[test]
    fn test_inflation_adjuster_calculate_inflation() {
        let mut adjuster = InflationAdjuster::new(2020);
        adjuster.set_cpi(2021, 102.0);
        let inflation = adjuster.get_inflation_rate(2021);
        assert!(inflation.is_some());
        assert!((inflation.unwrap() - 2.0).abs() < 0.1);
    }

    #[test]
    fn test_inflation_adjuster_to_real_value() {
        let mut adjuster = InflationAdjuster::new(2020);
        adjuster.set_cpi(2021, 110.0);
        let real = adjuster.to_real_value(110.0, 2021);
        assert!((real - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_inflation_adjuster_to_nominal_value() {
        let mut adjuster = InflationAdjuster::new(2020);
        adjuster.set_cpi(2021, 110.0);
        let nominal = adjuster.to_nominal_value(100.0, 2021);
        assert!((nominal - 110.0).abs() < 0.1);
    }

    #[test]
    fn test_inflation_adjuster_project_cpi() {
        let mut adjuster = InflationAdjuster::new(2020);
        adjuster.project_cpi(2030, 2.0);
        assert!(adjuster.cpi_by_year.contains_key(&2030));
    }

    #[test]
    fn test_inflation_adjuster_cumulative_inflation() {
        let mut adjuster = InflationAdjuster::new(2020);
        adjuster.set_cpi(2021, 102.0);
        adjuster.set_cpi(2022, 105.0);
        let cumulative = adjuster.cumulative_inflation(2020, 2022);
        assert!((cumulative - 5.0).abs() < 0.1);
    }

    #[test]
    fn test_gdp_impact_creation() {
        let analysis = GdpImpactAnalysis::new(20_000.0, 1.5, 5);
        assert_eq!(analysis.baseline_gdp, 20_000.0);
        assert_eq!(analysis.fiscal_multiplier, 1.5);
        assert_eq!(analysis.time_horizon, 5);
    }

    #[test]
    fn test_gdp_impact_government_spending() {
        let mut analysis = GdpImpactAnalysis::new(20_000.0, 1.5, 5);
        analysis.add_government_spending(1_000.0);
        assert!(analysis.gdp_impact > 0.0);
        assert_eq!(analysis.gdp_impact, 1_000.0 * 1.5);
    }

    #[test]
    fn test_gdp_impact_tax_impact() {
        let mut analysis = GdpImpactAnalysis::new(20_000.0, 1.5, 5);
        analysis.add_tax_impact(-500.0, 0.8); // Tax cut
        assert!(analysis.gdp_impact > 0.0);
    }

    #[test]
    fn test_gdp_impact_investment() {
        let mut analysis = GdpImpactAnalysis::new(20_000.0, 1.5, 5);
        analysis.add_investment_impact(1_000.0);
        assert!(analysis.gdp_impact > 0.0);
    }

    #[test]
    fn test_gdp_impact_trade() {
        let mut analysis = GdpImpactAnalysis::new(20_000.0, 1.5, 5);
        analysis.add_trade_impact(500.0, -300.0);
        assert!(analysis.gdp_impact > 0.0);
    }

    #[test]
    fn test_gdp_impact_employment_estimate() {
        let mut analysis = GdpImpactAnalysis::new(20_000.0, 1.5, 5);
        analysis.add_government_spending(1_000.0);
        let employment_impact = analysis.estimate_employment_impact(2.0);
        assert!(employment_impact > 0.0);
    }

    #[test]
    fn test_gdp_impact_projection_over_time() {
        let mut analysis = GdpImpactAnalysis::new(20_000.0, 1.5, 5);
        analysis.add_government_spending(1_000.0);
        let projections = analysis.project_over_time(10.0);
        assert_eq!(projections.len(), 5);
        // Impact should decay over time
        assert!(projections[4] < projections[0]);
    }
}
