//! US Labor Law Simulation Module.
//!
//! This module provides simulation capabilities for analyzing the economic
//! impacts of US labor law changes, particularly:
//! - FLSA minimum wage impact analysis
//! - Overtime regulation effects
//! - Labor market simulation
//! - Economic distributional analysis

use legalis_core::{BasicEntity, Condition, Effect, EffectType, LegalEntity, Statute};
use legalis_sim::{
    BenefitPreset, DistributionalImpact, EconomicImpact, GdpImpactAnalysis, LaborMarket,
    MacroeconomicIndicators, Progressivity, QuintileImpact, SimEngineBuilder, SimulationMetrics,
    TaxSystemPreset,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// FLSA Minimum Wage Simulation
// ============================================================================

/// FLSA minimum wage simulation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlsaMinimumWageConfig {
    /// Current federal minimum wage ($7.25 as of 2024)
    pub current_minimum_wage: f64,
    /// Proposed minimum wage to simulate
    pub proposed_minimum_wage: f64,
    /// Population size for simulation
    pub population_size: usize,
    /// Number of Monte Carlo runs
    pub monte_carlo_runs: usize,
    /// Time horizon in years
    pub time_horizon: u32,
}

impl Default for FlsaMinimumWageConfig {
    fn default() -> Self {
        Self {
            current_minimum_wage: 7.25,
            proposed_minimum_wage: 15.00,
            population_size: 10_000,
            monte_carlo_runs: 100,
            time_horizon: 5,
        }
    }
}

/// FLSA minimum wage simulation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlsaSimulationResult {
    /// Workers affected (earning below proposed minimum)
    pub workers_affected: usize,
    /// Workers affected percentage
    pub workers_affected_pct: f64,
    /// Average wage increase for affected workers
    pub avg_wage_increase: f64,
    /// Total additional labor cost (annual)
    pub total_labor_cost_increase: f64,
    /// Estimated employment reduction (based on elasticity)
    pub estimated_job_losses: usize,
    /// Economic distributional impact
    pub distributional_impact: DistributionalImpact,
    /// GDP impact analysis
    pub gdp_impact: GdpImpactAnalysis,
    /// Metrics from simulation
    pub simulation_metrics: SimulationMetrics,
}

/// FLSA minimum wage simulator
pub struct FlsaMinimumWageSimulator {
    config: FlsaMinimumWageConfig,
}

impl FlsaMinimumWageSimulator {
    /// Creates a new simulator with the given configuration
    pub fn new(config: FlsaMinimumWageConfig) -> Self {
        Self { config }
    }

    /// Creates a simulator with default configuration
    pub fn default_config() -> Self {
        Self::new(FlsaMinimumWageConfig::default())
    }

    /// Runs the minimum wage impact simulation
    pub fn simulate(&self) -> FlsaSimulationResult {
        let population = self.generate_population();

        // Count affected workers (those earning between current and proposed minimum)
        let mut workers_affected = 0;
        let mut total_wage_increase = 0.0;

        for worker in &population {
            let wage_opt: Option<f64> = worker
                .get_attribute("hourly_wage")
                .and_then(|s| s.parse().ok());
            if let Some(wage) = wage_opt
                && wage < self.config.proposed_minimum_wage
                && wage >= self.config.current_minimum_wage
            {
                workers_affected += 1;
                total_wage_increase += self.config.proposed_minimum_wage - wage;
            }
        }

        let workers_affected_pct = workers_affected as f64 / population.len() as f64 * 100.0;
        let avg_wage_increase = if workers_affected > 0 {
            total_wage_increase / workers_affected as f64
        } else {
            0.0
        };

        // Calculate annual labor cost increase
        let annual_hours = 2080.0; // 40 hours * 52 weeks
        let total_labor_cost_increase = total_wage_increase * annual_hours;

        // Estimate job losses using elasticity
        let labor_demand_elasticity = -0.1; // Conservative estimate
        let wage_increase_pct = (self.config.proposed_minimum_wage
            - self.config.current_minimum_wage)
            / self.config.current_minimum_wage;
        let employment_change_pct = labor_demand_elasticity * wage_increase_pct;
        let estimated_job_losses =
            (workers_affected as f64 * employment_change_pct.abs()).round() as usize;

        // Build simulation engine and run
        let metrics = self.run_engine_simulation(&population);

        // Calculate distributional impact
        let distributional_impact = self.calculate_distributional_impact(&population);

        // Calculate GDP impact
        let gdp_impact = self.calculate_gdp_impact(total_labor_cost_increase, workers_affected);

        FlsaSimulationResult {
            workers_affected,
            workers_affected_pct,
            avg_wage_increase,
            total_labor_cost_increase,
            estimated_job_losses,
            distributional_impact,
            gdp_impact,
            simulation_metrics: metrics,
        }
    }

    /// Generates a simulated workforce population
    fn generate_population(&self) -> Vec<BasicEntity> {
        let mut population = Vec::with_capacity(self.config.population_size);

        for i in 0..self.config.population_size {
            let mut entity = BasicEntity::new();

            // Age distribution (18-65)
            let age = 18 + (i * 47 / self.config.population_size);
            entity.set_attribute("age", age.to_string());

            // Income distribution (skewed toward lower wages for realistic simulation)
            let wage = self.generate_wage_for_entity(i);
            entity.set_attribute("hourly_wage", format!("{:.2}", wage));
            entity.set_attribute("annual_income", format!("{:.2}", wage * 2080.0));

            // Employment status
            let employment_status = if i % 20 == 0 {
                "unemployed"
            } else {
                "employed"
            };
            entity.set_attribute("employment_status", employment_status.to_string());

            // Exempt status
            let exempt_status = if wage > 35.0 { "exempt" } else { "non_exempt" };
            entity.set_attribute("exempt_status", exempt_status.to_string());

            // Income quintile (1-5)
            let quintile = 1 + (i * 5 / self.config.population_size);
            entity.set_attribute("income_quintile", quintile.to_string());

            population.push(entity);
        }

        population
    }

    /// Generates a wage for an entity based on realistic distribution
    fn generate_wage_for_entity(&self, index: usize) -> f64 {
        // Create a right-skewed distribution typical of wage data
        let base = self.config.current_minimum_wage;
        let percentile = index as f64 / self.config.population_size as f64;

        if percentile < 0.15 {
            // Minimum wage workers (15%)
            base + percentile * 2.0
        } else if percentile < 0.30 {
            // Near-minimum wage workers (15%)
            base + 1.0 + (percentile - 0.15) * 40.0
        } else if percentile < 0.60 {
            // Middle-income workers (30%)
            12.0 + (percentile - 0.30) * 50.0
        } else if percentile < 0.85 {
            // Upper-middle income (25%)
            27.0 + (percentile - 0.60) * 80.0
        } else {
            // High earners (15%)
            47.0 + (percentile - 0.85) * 150.0
        }
    }

    /// Runs the simulation engine
    fn run_engine_simulation(&self, population: &[BasicEntity]) -> SimulationMetrics {
        let mut builder = SimEngineBuilder::new().validate(false);

        for entity in population {
            builder = builder.add_entity(Box::new(entity.clone()));
        }

        // Add FLSA minimum wage statute
        let flsa_statute = self.create_flsa_statute();
        builder = builder.add_statute(flsa_statute);

        match builder.build() {
            Ok(engine) => match tokio::runtime::Runtime::new() {
                Ok(rt) => rt.block_on(async { engine.run_simulation().await }),
                Err(_) => SimulationMetrics::new(),
            },
            Err(_) => SimulationMetrics::new(),
        }
    }

    /// Creates FLSA minimum wage statute for simulation
    fn create_flsa_statute(&self) -> Statute {
        Statute::new(
            "FLSA_206_sim",
            "FLSA Minimum Wage (Simulation)",
            Effect::new(
                EffectType::Obligation,
                "Employer must pay at least federal minimum wage",
            )
            .with_parameter(
                "federal_minimum_wage",
                format!("{:.2}", self.config.proposed_minimum_wage),
            )
            .with_parameter(
                "current_minimum",
                format!("{:.2}", self.config.current_minimum_wage),
            ),
        )
        .with_precondition(Condition::AttributeEquals {
            key: "exempt_status".to_string(),
            value: "non_exempt".to_string(),
        })
        .with_jurisdiction("US")
    }

    /// Calculates distributional impact across income quintiles
    fn calculate_distributional_impact(&self, population: &[BasicEntity]) -> DistributionalImpact {
        let mut quintile_data: HashMap<u8, Vec<f64>> = HashMap::new();

        for entity in population {
            let quintile_opt: Option<u8> = entity
                .get_attribute("income_quintile")
                .and_then(|s| s.parse().ok());
            let wage_opt: Option<f64> = entity
                .get_attribute("hourly_wage")
                .and_then(|s| s.parse().ok());

            if let (Some(quintile), Some(wage)) = (quintile_opt, wage_opt) {
                quintile_data.entry(quintile).or_default().push(wage);
            }
        }

        let mut quintile_impacts = Vec::new();
        let mut first_quintile_impact = 0.0;
        let mut fifth_quintile_impact = 0.0;

        for quintile in 1..=5 {
            if let Some(wages) = quintile_data.get(&quintile) {
                let avg_wage: f64 = wages.iter().sum::<f64>() / wages.len() as f64;
                let affected: usize = wages
                    .iter()
                    .filter(|&&w| {
                        w < self.config.proposed_minimum_wage
                            && w >= self.config.current_minimum_wage
                    })
                    .count();

                let avg_increase = if affected > 0 {
                    let total_increase: f64 = wages
                        .iter()
                        .filter(|&&w| {
                            w < self.config.proposed_minimum_wage
                                && w >= self.config.current_minimum_wage
                        })
                        .map(|&w| self.config.proposed_minimum_wage - w)
                        .sum();
                    total_increase / affected as f64
                } else {
                    0.0
                };

                let impact_pct = if avg_wage > 0.0 {
                    avg_increase / avg_wage * 100.0
                } else {
                    0.0
                };

                if quintile == 1 {
                    first_quintile_impact = impact_pct;
                }
                if quintile == 5 {
                    fifth_quintile_impact = impact_pct;
                }

                quintile_impacts.push(QuintileImpact {
                    quintile,
                    avg_tax_burden: 0.0,
                    avg_compliance_cost: avg_increase * 2080.0,
                    entity_count: wages.len(),
                    impact_pct_of_income: impact_pct,
                });
            }
        }

        let progressivity = if first_quintile_impact > fifth_quintile_impact {
            Progressivity::Progressive
        } else if (first_quintile_impact - fifth_quintile_impact).abs() < 0.5 {
            Progressivity::Proportional
        } else {
            Progressivity::Regressive
        };

        // Estimate Gini change
        let gini_change = if first_quintile_impact > fifth_quintile_impact {
            -0.01 * (first_quintile_impact - fifth_quintile_impact).min(5.0)
        } else {
            0.01 * (fifth_quintile_impact - first_quintile_impact).min(5.0)
        };

        DistributionalImpact {
            quintile_impacts,
            gini_change,
            progressivity,
        }
    }

    /// Calculates GDP impact from wage changes
    fn calculate_gdp_impact(
        &self,
        total_labor_cost_increase: f64,
        _workers_affected: usize,
    ) -> GdpImpactAnalysis {
        // US GDP baseline (in billions)
        let baseline_gdp = 27_000.0;
        let fiscal_multiplier = 1.2;

        let mut analysis = GdpImpactAnalysis::new(baseline_gdp, fiscal_multiplier, 5);

        // Add consumption impact from increased wages
        let marginal_propensity_to_consume = 0.9; // Low-income workers spend more
        let consumption_boost = total_labor_cost_increase * marginal_propensity_to_consume;

        analysis.add_tax_impact(-consumption_boost, marginal_propensity_to_consume);

        // Investment impact (businesses may reduce investment)
        let investment_reduction = total_labor_cost_increase * 0.1;
        analysis.add_investment_impact(-investment_reduction);

        analysis
    }
}

// ============================================================================
// FLSA Overtime Simulation
// ============================================================================

/// FLSA overtime regulation simulation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlsaOvertimeConfig {
    /// Current overtime threshold (40 hours/week)
    pub current_threshold_hours: u32,
    /// Proposed overtime threshold
    pub proposed_threshold_hours: u32,
    /// Current salary exemption threshold
    pub current_salary_threshold: f64,
    /// Proposed salary exemption threshold
    pub proposed_salary_threshold: f64,
    /// Population size
    pub population_size: usize,
}

impl Default for FlsaOvertimeConfig {
    fn default() -> Self {
        Self {
            current_threshold_hours: 40,
            proposed_threshold_hours: 40,
            current_salary_threshold: 35_568.0,
            proposed_salary_threshold: 55_000.0,
            population_size: 10_000,
        }
    }
}

/// FLSA overtime simulation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlsaOvertimeResult {
    /// Workers newly eligible for overtime
    pub newly_eligible_workers: usize,
    /// Total additional overtime cost (annual estimate)
    pub estimated_overtime_cost_increase: f64,
    /// Average hours worked over threshold
    pub avg_overtime_hours: f64,
}

/// FLSA overtime simulator
pub struct FlsaOvertimeSimulator {
    config: FlsaOvertimeConfig,
}

impl FlsaOvertimeSimulator {
    /// Creates a new overtime simulator
    pub fn new(config: FlsaOvertimeConfig) -> Self {
        Self { config }
    }

    /// Runs the overtime impact simulation
    pub fn simulate(&self) -> FlsaOvertimeResult {
        let mut newly_eligible = 0;
        let mut total_overtime_hours = 0.0;
        let mut total_overtime_cost = 0.0;

        for i in 0..self.config.population_size {
            // Simulate salary (distributed between thresholds)
            let salary = self.config.current_salary_threshold
                + (i as f64 / self.config.population_size as f64)
                    * (100_000.0 - self.config.current_salary_threshold);

            // Check if newly eligible (between current and proposed threshold)
            if salary >= self.config.current_salary_threshold
                && salary < self.config.proposed_salary_threshold
            {
                newly_eligible += 1;

                // Simulate weekly overtime hours (typically 5-15 hours for salaried workers)
                let overtime_hours = 5.0 + (i % 10) as f64;
                total_overtime_hours += overtime_hours;

                // Calculate overtime cost
                let hourly_rate = salary / 2080.0;
                let weekly_overtime_cost = overtime_hours * hourly_rate * 1.5;
                let annual_overtime_cost = weekly_overtime_cost * 52.0;
                total_overtime_cost += annual_overtime_cost;
            }
        }

        let avg_overtime_hours = if newly_eligible > 0 {
            total_overtime_hours / newly_eligible as f64
        } else {
            0.0
        };

        FlsaOvertimeResult {
            newly_eligible_workers: newly_eligible,
            estimated_overtime_cost_increase: total_overtime_cost,
            avg_overtime_hours,
        }
    }
}

// ============================================================================
// Unemployment Insurance Simulation
// ============================================================================

/// Creates US unemployment insurance benefit preset for simulation
pub fn us_unemployment_insurance_preset() -> BenefitPreset {
    BenefitPreset::us_unemployment_insurance()
}

/// Simulates unemployment insurance impact
pub fn simulate_unemployment_insurance(population_size: usize) -> EconomicImpact {
    let benefit = us_unemployment_insurance_preset();

    let mut total_benefits = 0.0;
    let mut revenue_by_type = HashMap::new();
    let mut costs_by_type = HashMap::new();

    for i in 0..population_size {
        // Simulate income and assets
        let income = 20_000.0 + (i as f64 / population_size as f64) * 80_000.0;
        let assets = 1_000.0 + (i as f64 / population_size as f64) * 50_000.0;

        let result = benefit.check_eligibility(income, assets);
        if result.eligible {
            total_benefits += result.benefit_amount;
        }
    }

    revenue_by_type.insert("unemployment_tax".to_string(), total_benefits * 0.3);
    costs_by_type.insert("benefit_payments".to_string(), total_benefits);

    EconomicImpact {
        tax_revenue: total_benefits * 0.3,
        compliance_costs: total_benefits * 0.02,
        administrative_costs: total_benefits * 0.05,
        net_benefit: total_benefits * 0.23,
        revenue_by_type,
        costs_by_type,
        distributional_impact: DistributionalImpact {
            quintile_impacts: vec![],
            gini_change: -0.01,
            progressivity: Progressivity::Progressive,
        },
    }
}

// ============================================================================
// Tax System Simulation
// ============================================================================

/// Creates US federal income tax preset for simulation
pub fn us_federal_income_tax_preset() -> TaxSystemPreset {
    TaxSystemPreset::us_federal_income_tax_2024()
}

/// Simulates federal income tax impact
pub fn simulate_federal_income_tax(population_size: usize) -> EconomicImpact {
    let tax_system = us_federal_income_tax_preset();

    let mut total_tax_revenue = 0.0;
    let mut revenue_by_type = HashMap::new();
    let mut quintile_impacts = Vec::new();

    // Simulate by quintile
    for quintile in 1..=5_u8 {
        let quintile_size = population_size / 5;
        let base_income = match quintile {
            1 => 15_000.0,
            2 => 35_000.0,
            3 => 60_000.0,
            4 => 100_000.0,
            _ => 200_000.0,
        };

        let mut quintile_tax = 0.0;
        for i in 0..quintile_size {
            let income = base_income + (i as f64 / quintile_size as f64) * base_income * 0.5;
            let calc = tax_system.calculate_tax(income, 0);
            quintile_tax += calc.tax_owed;
        }

        total_tax_revenue += quintile_tax;
        let avg_tax = quintile_tax / quintile_size as f64;

        quintile_impacts.push(QuintileImpact {
            quintile,
            avg_tax_burden: avg_tax,
            avg_compliance_cost: 500.0,
            entity_count: quintile_size,
            impact_pct_of_income: (avg_tax / base_income) * 100.0,
        });
    }

    revenue_by_type.insert("federal_income".to_string(), total_tax_revenue);

    EconomicImpact {
        tax_revenue: total_tax_revenue,
        compliance_costs: population_size as f64 * 200.0,
        administrative_costs: total_tax_revenue * 0.01,
        net_benefit: total_tax_revenue * 0.98,
        revenue_by_type,
        costs_by_type: HashMap::new(),
        distributional_impact: DistributionalImpact {
            quintile_impacts,
            gini_change: -0.02,
            progressivity: Progressivity::Progressive,
        },
    }
}

// ============================================================================
// Labor Market Simulation
// ============================================================================

/// Creates a US labor market model
pub fn us_labor_market(total_labor_force: u64) -> LaborMarket {
    let employed = (total_labor_force as f64 * 0.96) as u64;
    let mut market = LaborMarket::new(total_labor_force, employed);

    // Set sector employment
    market
        .employment_by_sector
        .insert("manufacturing".to_string(), employed / 10);
    market
        .employment_by_sector
        .insert("services".to_string(), employed * 7 / 10);
    market
        .employment_by_sector
        .insert("government".to_string(), employed / 10);
    market
        .employment_by_sector
        .insert("other".to_string(), employed / 10);

    // Set sector wages
    market
        .wage_by_sector
        .insert("manufacturing".to_string(), 55_000.0);
    market
        .wage_by_sector
        .insert("services".to_string(), 45_000.0);
    market
        .wage_by_sector
        .insert("government".to_string(), 60_000.0);
    market.wage_by_sector.insert("other".to_string(), 50_000.0);

    market.average_wage = 52_000.0;
    market.wage_growth_rate = 3.5;
    market.job_openings = employed / 50;

    market
}

/// Creates US macroeconomic indicators (2024 baseline)
pub fn us_macroeconomic_indicators_2024() -> MacroeconomicIndicators {
    let mut indicators = MacroeconomicIndicators::new(2024, 27_000.0, 2.5, 3.0);

    indicators.unemployment_rate = 4.0;
    indicators.interest_rate = 5.25;
    indicators.cpi = 310.0;
    indicators.labor_force_participation = 62.5;
    indicators.trade_balance = -800.0;
    indicators.debt_to_gdp = 120.0;

    indicators
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flsa_minimum_wage_simulator_default() {
        let simulator = FlsaMinimumWageSimulator::default_config();
        let result = simulator.simulate();

        assert!(result.workers_affected > 0);
        assert!(result.workers_affected_pct > 0.0);
        assert!(result.avg_wage_increase >= 0.0);
    }

    #[test]
    fn test_flsa_minimum_wage_custom_config() {
        let config = FlsaMinimumWageConfig {
            current_minimum_wage: 7.25,
            proposed_minimum_wage: 10.00,
            population_size: 1000,
            monte_carlo_runs: 10,
            time_horizon: 3,
        };

        let simulator = FlsaMinimumWageSimulator::new(config);
        let result = simulator.simulate();

        assert!(result.workers_affected > 0);
        assert!(result.total_labor_cost_increase >= 0.0);
    }

    #[test]
    fn test_flsa_overtime_simulator() {
        let config = FlsaOvertimeConfig::default();
        let simulator = FlsaOvertimeSimulator::new(config);
        let result = simulator.simulate();

        assert!(result.newly_eligible_workers > 0);
        assert!(result.estimated_overtime_cost_increase >= 0.0);
    }

    #[test]
    fn test_unemployment_insurance_simulation() {
        let result = simulate_unemployment_insurance(1000);

        assert!(result.tax_revenue >= 0.0);
        assert!(result.compliance_costs >= 0.0);
    }

    #[test]
    fn test_federal_income_tax_simulation() {
        let result = simulate_federal_income_tax(1000);

        assert!(result.tax_revenue > 0.0);
        assert_eq!(result.distributional_impact.quintile_impacts.len(), 5);
        assert_eq!(
            result.distributional_impact.progressivity,
            Progressivity::Progressive
        );
    }

    #[test]
    fn test_us_labor_market() {
        let market = us_labor_market(150_000_000);

        assert!(market.unemployment_rate() < 5.0);
        assert!(market.employment_rate() > 95.0);
        assert_eq!(market.employment_by_sector.len(), 4);
    }

    #[test]
    fn test_us_macroeconomic_indicators() {
        let indicators = us_macroeconomic_indicators_2024();

        assert_eq!(indicators.year, 2024);
        assert!(!indicators.is_recession());
        assert!(!indicators.is_overheating());
    }

    #[test]
    fn test_distributional_impact_calculation() {
        let config = FlsaMinimumWageConfig {
            population_size: 500,
            ..Default::default()
        };
        let simulator = FlsaMinimumWageSimulator::new(config);
        let result = simulator.simulate();

        assert_eq!(result.distributional_impact.quintile_impacts.len(), 5);
        assert!(result.distributional_impact.gini_change <= 0.0);
    }

    #[test]
    fn test_gdp_impact_calculation() {
        let config = FlsaMinimumWageConfig::default();
        let simulator = FlsaMinimumWageSimulator::new(config);
        let result = simulator.simulate();

        assert!(result.gdp_impact.baseline_gdp > 0.0);
        assert_eq!(result.gdp_impact.time_horizon, 5);
    }
}
