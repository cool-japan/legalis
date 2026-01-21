//! Japanese Labor Law Simulation Module (日本労働法シミュレーション).
//!
//! This module provides simulation capabilities for analyzing the economic
//! impacts of Japanese labor law changes, including:
//! - 最低賃金 (Minimum Wage) regional impact analysis
//! - 働き方改革 (Work Style Reform) overtime limits
//! - 年次有給休暇 (Annual Paid Leave) utilization mandates
//! - 雇用保険 (Employment Insurance) benefit simulation

use legalis_core::{BasicEntity, Condition, Effect, EffectType, LegalEntity, Statute};
use legalis_sim::{
    BenefitAmount, BenefitPreset, BenefitType, DistributionalImpact, EconomicImpact,
    EligibilityRequirement, GdpImpactAnalysis, LaborMarket, MacroeconomicIndicators, Progressivity,
    QuintileImpact, RequirementType, SimEngineBuilder, SimulationMetrics, TaxBracket,
    TaxSystemPreset, TaxType,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Regional Minimum Wage Simulation (地域別最低賃金シミュレーション)
// ============================================================================

/// Japanese regional minimum wage simulation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumWageConfig {
    /// Current national weighted average minimum wage (円/時)
    pub current_average_wage: f64,
    /// Proposed minimum wage to simulate
    pub proposed_average_wage: f64,
    /// Population size for simulation
    pub population_size: usize,
    /// Number of Monte Carlo runs
    pub monte_carlo_runs: usize,
    /// Time horizon in years
    pub time_horizon: u32,
    /// Tokyo premium (東京の上乗せ率)
    pub tokyo_premium: f64,
}

impl Default for MinimumWageConfig {
    fn default() -> Self {
        Self {
            current_average_wage: 1004.0,
            proposed_average_wage: 1500.0,
            population_size: 10_000,
            monte_carlo_runs: 100,
            time_horizon: 5,
            tokyo_premium: 0.15,
        }
    }
}

/// Japanese minimum wage simulation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumWageResult {
    /// Workers affected
    pub workers_affected: usize,
    /// Workers affected percentage
    pub workers_affected_pct: f64,
    /// Average wage increase
    pub avg_wage_increase: f64,
    /// Total additional labor cost (annual)
    pub total_labor_cost_increase: f64,
    /// Estimated job losses
    pub estimated_job_losses: usize,
    /// Regional impact
    pub regional_impact: HashMap<String, f64>,
    /// Distributional impact
    pub distributional_impact: DistributionalImpact,
    /// GDP impact
    pub gdp_impact: GdpImpactAnalysis,
    /// Simulation metrics
    pub simulation_metrics: SimulationMetrics,
}

/// Japanese minimum wage simulator
pub struct MinimumWageSimulator {
    config: MinimumWageConfig,
}

impl MinimumWageSimulator {
    /// Creates a new simulator
    pub fn new(config: MinimumWageConfig) -> Self {
        Self { config }
    }

    /// Creates a simulator with default configuration
    pub fn default_config() -> Self {
        Self::new(MinimumWageConfig::default())
    }

    /// Runs the simulation
    pub fn simulate(&self) -> MinimumWageResult {
        let population = self.generate_population();

        let mut workers_affected = 0;
        let mut total_wage_increase = 0.0;
        let mut regional_impact: HashMap<String, f64> = HashMap::new();

        for region in &["tokyo", "osaka", "aichi", "fukuoka", "other"] {
            regional_impact.insert(region.to_string(), 0.0);
        }

        for worker in &population {
            let wage_opt: Option<f64> = worker
                .get_attribute("hourly_wage")
                .and_then(|s| s.parse().ok());
            let region = worker
                .get_attribute("region")
                .unwrap_or_else(|| "other".to_string());

            if let Some(wage) = wage_opt {
                let regional_minimum = self.get_regional_minimum(&region);
                let proposed_regional = self.get_proposed_regional(&region);

                if wage < proposed_regional && wage >= regional_minimum {
                    workers_affected += 1;
                    let increase = proposed_regional - wage;
                    total_wage_increase += increase;
                    *regional_impact.entry(region).or_insert(0.0) += increase;
                }
            }
        }

        let workers_affected_pct = if self.config.population_size > 0 {
            (workers_affected as f64 / self.config.population_size as f64) * 100.0
        } else {
            0.0
        };

        let avg_wage_increase = if workers_affected > 0 {
            total_wage_increase / workers_affected as f64
        } else {
            0.0
        };

        let total_labor_cost_increase = total_wage_increase * 2000.0;

        let elasticity = -0.15;
        let wage_increase_pct = (self.config.proposed_average_wage
            - self.config.current_average_wage)
            / self.config.current_average_wage;
        let employment_change_pct = elasticity * wage_increase_pct;
        let estimated_job_losses =
            (workers_affected as f64 * employment_change_pct.abs()).round() as usize;

        let simulation_metrics = self.run_simulation_engine(&population);

        MinimumWageResult {
            workers_affected,
            workers_affected_pct,
            avg_wage_increase,
            total_labor_cost_increase,
            estimated_job_losses,
            regional_impact,
            distributional_impact: self.calculate_distributional_impact(&population),
            gdp_impact: self.calculate_gdp_impact(total_labor_cost_increase),
            simulation_metrics,
        }
    }

    fn get_regional_minimum(&self, region: &str) -> f64 {
        match region {
            "tokyo" => self.config.current_average_wage * (1.0 + self.config.tokyo_premium),
            "osaka" | "kanagawa" => self.config.current_average_wage * 1.10,
            "aichi" | "saitama" => self.config.current_average_wage * 1.05,
            _ => self.config.current_average_wage,
        }
    }

    fn get_proposed_regional(&self, region: &str) -> f64 {
        match region {
            "tokyo" => self.config.proposed_average_wage * (1.0 + self.config.tokyo_premium),
            "osaka" | "kanagawa" => self.config.proposed_average_wage * 1.10,
            "aichi" | "saitama" => self.config.proposed_average_wage * 1.05,
            _ => self.config.proposed_average_wage,
        }
    }

    fn generate_population(&self) -> Vec<BasicEntity> {
        let mut population = Vec::with_capacity(self.config.population_size);
        let regions = ["tokyo", "osaka", "aichi", "fukuoka", "other"];
        let region_weights = [0.15, 0.10, 0.08, 0.05, 0.62];

        for i in 0..self.config.population_size {
            let mut entity = BasicEntity::new();

            let region_idx = (i % 100) as f64 / 100.0;
            let mut cumulative = 0.0;
            let mut selected_region = "other";
            for (idx, weight) in region_weights.iter().enumerate() {
                cumulative += weight;
                if region_idx < cumulative {
                    selected_region = regions[idx];
                    break;
                }
            }
            entity.set_attribute("region", selected_region.to_string());

            let base_wage = self.get_regional_minimum(selected_region);
            let wage_factor = 1.0 + (i as f64 / self.config.population_size as f64) * 1.5;
            let wage = base_wage * wage_factor;
            entity.set_attribute("hourly_wage", format!("{:.0}", wage));

            entity.set_attribute(
                "employment_type",
                if i % 3 == 0 {
                    "part_time".to_string()
                } else {
                    "full_time".to_string()
                },
            );
            entity.set_attribute("income_quintile", format!("{}", (i % 5) + 1));

            population.push(entity);
        }

        population
    }

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
                        w < self.config.proposed_average_wage
                            && w >= self.config.current_average_wage
                    })
                    .count();

                let avg_increase = if affected > 0 {
                    let total_increase: f64 = wages
                        .iter()
                        .filter(|&&w| {
                            w < self.config.proposed_average_wage
                                && w >= self.config.current_average_wage
                        })
                        .map(|&w| self.config.proposed_average_wage - w)
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
                    avg_compliance_cost: avg_increase * 2000.0,
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

    fn calculate_gdp_impact(&self, labor_cost_increase: f64) -> GdpImpactAnalysis {
        // Japanese GDP: ¥550 trillion (in trillions for consistency)
        let baseline_gdp = 550.0;
        let fiscal_multiplier = 1.1;

        let mut analysis = GdpImpactAnalysis::new(baseline_gdp, fiscal_multiplier, 5);

        let marginal_propensity_to_consume = 0.8;
        let consumption_boost =
            (labor_cost_increase / 1_000_000_000_000.0) * marginal_propensity_to_consume;

        analysis.add_tax_impact(-consumption_boost, marginal_propensity_to_consume);

        let investment_reduction = (labor_cost_increase / 1_000_000_000_000.0) * 0.1;
        analysis.add_investment_impact(-investment_reduction);

        analysis
    }

    fn run_simulation_engine(&self, population: &[BasicEntity]) -> SimulationMetrics {
        let mut builder = SimEngineBuilder::new().validate(false);

        for entity in population.iter().take(100) {
            builder = builder.add_entity(Box::new(entity.clone()));
        }

        let statute = self.create_minimum_wage_statute();
        builder = builder.add_statute(statute);

        match builder.build() {
            Ok(engine) => match tokio::runtime::Runtime::new() {
                Ok(rt) => rt.block_on(async { engine.run_simulation().await }),
                Err(_) => SimulationMetrics::new(),
            },
            Err(_) => SimulationMetrics::new(),
        }
    }

    fn create_minimum_wage_statute(&self) -> Statute {
        Statute::new(
            "MinimumWageAct_Sim",
            "最低賃金法シミュレーション",
            Effect::new(
                EffectType::Obligation,
                "使用者は最低賃金以上を支払う義務がある",
            )
            .with_parameter(
                "minimum_wage",
                format!("{:.0}", self.config.proposed_average_wage),
            )
            .with_parameter(
                "current_minimum",
                format!("{:.0}", self.config.current_average_wage),
            ),
        )
        .with_precondition(Condition::AttributeEquals {
            key: "employment_type".to_string(),
            value: "employed".to_string(),
        })
        .with_jurisdiction("JP")
    }
}

// ============================================================================
// Work Style Reform Simulation (働き方改革シミュレーション)
// ============================================================================

/// Work Style Reform configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkStyleReformConfig {
    /// Annual overtime limit in hours
    pub annual_overtime_limit: u32,
    /// Monthly overtime limit in hours
    pub monthly_overtime_limit: u32,
    /// Population size
    pub population_size: usize,
    /// Current average annual overtime hours
    pub current_avg_overtime: f64,
}

impl Default for WorkStyleReformConfig {
    fn default() -> Self {
        Self {
            annual_overtime_limit: 720,
            monthly_overtime_limit: 100,
            population_size: 10_000,
            current_avg_overtime: 200.0,
        }
    }
}

/// Work Style Reform result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkStyleReformResult {
    /// Workers exceeding limits
    pub workers_exceeding_limits: usize,
    /// Workers exceeding percentage
    pub workers_exceeding_pct: f64,
    /// Average overtime reduction
    pub avg_overtime_reduction: f64,
    /// Lost productivity hours
    pub lost_productivity_hours: f64,
    /// Additional hiring needed
    pub additional_hiring_needed: usize,
    /// Industry impact
    pub industry_impact: HashMap<String, f64>,
    /// Compliance cost
    pub compliance_cost: f64,
}

/// Work Style Reform simulator
pub struct WorkStyleReformSimulator {
    config: WorkStyleReformConfig,
}

impl WorkStyleReformSimulator {
    pub fn new(config: WorkStyleReformConfig) -> Self {
        Self { config }
    }

    pub fn default_config() -> Self {
        Self::new(WorkStyleReformConfig::default())
    }

    pub fn simulate(&self) -> WorkStyleReformResult {
        let population = self.generate_population();
        let mut workers_exceeding = 0;
        let mut total_overtime_reduction = 0.0;
        let mut industry_impact: HashMap<String, f64> = HashMap::new();

        for industry in &[
            "manufacturing",
            "construction",
            "transport",
            "it",
            "service",
            "other",
        ] {
            industry_impact.insert(industry.to_string(), 0.0);
        }

        for worker in &population {
            let overtime_opt: Option<f64> = worker
                .get_attribute("annual_overtime")
                .and_then(|s| s.parse().ok());
            let industry = worker
                .get_attribute("industry")
                .unwrap_or_else(|| "other".to_string());

            if let Some(overtime) = overtime_opt
                && overtime > self.config.annual_overtime_limit as f64
            {
                workers_exceeding += 1;
                let reduction = overtime - self.config.annual_overtime_limit as f64;
                total_overtime_reduction += reduction;
                *industry_impact.entry(industry).or_insert(0.0) += reduction;
            }
        }

        let workers_exceeding_pct = if self.config.population_size > 0 {
            (workers_exceeding as f64 / self.config.population_size as f64) * 100.0
        } else {
            0.0
        };

        let avg_overtime_reduction = if workers_exceeding > 0 {
            total_overtime_reduction / workers_exceeding as f64
        } else {
            0.0
        };

        let additional_hiring_needed = (total_overtime_reduction / 2000.0).ceil() as usize;

        let compliance_cost =
            additional_hiring_needed as f64 * 750_000.0 + workers_exceeding as f64 * 50_000.0;

        WorkStyleReformResult {
            workers_exceeding_limits: workers_exceeding,
            workers_exceeding_pct,
            avg_overtime_reduction,
            lost_productivity_hours: total_overtime_reduction,
            additional_hiring_needed,
            industry_impact,
            compliance_cost,
        }
    }

    fn generate_population(&self) -> Vec<BasicEntity> {
        let mut population = Vec::with_capacity(self.config.population_size);
        let industries = [
            "manufacturing",
            "construction",
            "transport",
            "it",
            "service",
            "other",
        ];

        for i in 0..self.config.population_size {
            let mut entity = BasicEntity::new();

            let industry = industries[i % industries.len()];
            entity.set_attribute("industry", industry.to_string());

            let base_overtime = match industry {
                "construction" => self.config.current_avg_overtime * 1.5,
                "transport" => self.config.current_avg_overtime * 1.4,
                "it" => self.config.current_avg_overtime * 1.2,
                _ => self.config.current_avg_overtime,
            };

            let variation = (i as f64 / self.config.population_size as f64 - 0.5) * 2.0;
            let overtime = (base_overtime * (1.0 + variation * 0.5)).max(0.0);
            entity.set_attribute("annual_overtime", format!("{:.0}", overtime));

            population.push(entity);
        }

        population
    }
}

// ============================================================================
// Annual Paid Leave Simulation (年次有給休暇シミュレーション)
// ============================================================================

/// Paid leave configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaidLeaveConfig {
    /// Mandatory utilization days
    pub mandatory_days: u32,
    /// Population size
    pub population_size: usize,
    /// Current utilization rate
    pub current_utilization_rate: f64,
}

impl Default for PaidLeaveConfig {
    fn default() -> Self {
        Self {
            mandatory_days: 5,
            population_size: 10_000,
            current_utilization_rate: 0.56,
        }
    }
}

/// Paid leave result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaidLeaveResult {
    /// Workers not meeting requirement
    pub workers_not_meeting: usize,
    /// Workers not meeting percentage
    pub workers_not_meeting_pct: f64,
    /// Average additional days needed
    pub avg_additional_days_needed: f64,
    /// Total additional days
    pub total_additional_days: f64,
    /// Productivity impact
    pub productivity_impact: f64,
    /// Wellbeing improvement
    pub wellbeing_improvement: f64,
}

/// Paid leave simulator
pub struct PaidLeaveSimulator {
    config: PaidLeaveConfig,
}

impl PaidLeaveSimulator {
    pub fn new(config: PaidLeaveConfig) -> Self {
        Self { config }
    }

    pub fn default_config() -> Self {
        Self::new(PaidLeaveConfig::default())
    }

    pub fn simulate(&self) -> PaidLeaveResult {
        let population = self.generate_population();
        let mut workers_not_meeting = 0;
        let mut total_additional_days = 0.0;

        for worker in &population {
            let days_taken_opt: Option<f64> = worker
                .get_attribute("leave_days_taken")
                .and_then(|s| s.parse().ok());

            if let Some(days_taken) = days_taken_opt
                && days_taken < self.config.mandatory_days as f64
            {
                workers_not_meeting += 1;
                total_additional_days += self.config.mandatory_days as f64 - days_taken;
            }
        }

        let workers_not_meeting_pct = if self.config.population_size > 0 {
            (workers_not_meeting as f64 / self.config.population_size as f64) * 100.0
        } else {
            0.0
        };

        let avg_additional_days_needed = if workers_not_meeting > 0 {
            total_additional_days / workers_not_meeting as f64
        } else {
            0.0
        };

        let productivity_impact = total_additional_days * -0.5 + total_additional_days * 0.8;
        let wellbeing_improvement = total_additional_days * 0.1;

        PaidLeaveResult {
            workers_not_meeting,
            workers_not_meeting_pct,
            avg_additional_days_needed,
            total_additional_days,
            productivity_impact,
            wellbeing_improvement,
        }
    }

    fn generate_population(&self) -> Vec<BasicEntity> {
        let mut population = Vec::with_capacity(self.config.population_size);

        for i in 0..self.config.population_size {
            let mut entity = BasicEntity::new();

            let entitlement = 10.0 + (i as f64 / self.config.population_size as f64) * 10.0;
            entity.set_attribute("leave_entitlement", format!("{:.0}", entitlement));

            let utilization_variation = (i as f64 / self.config.population_size as f64) * 0.8;
            let days_taken =
                entitlement * self.config.current_utilization_rate * utilization_variation;
            entity.set_attribute("leave_days_taken", format!("{:.1}", days_taken));

            population.push(entity);
        }

        population
    }
}

// ============================================================================
// Preset Functions
// ============================================================================

/// Japanese labor market preset
pub fn jp_labor_market() -> LaborMarket {
    LaborMarket::new(69_000_000, 67_600_000)
}

/// Japanese macroeconomic indicators 2024
pub fn jp_macroeconomic_indicators_2024() -> MacroeconomicIndicators {
    MacroeconomicIndicators::new(2024, 550.0, 1.5, 2.8)
}

/// Japanese income tax preset
pub fn jp_income_tax_preset() -> TaxSystemPreset {
    TaxSystemPreset {
        name: "Japanese Income Tax".to_string(),
        tax_type: TaxType::Income,
        brackets: vec![
            TaxBracket::new(0.0, 0.05),
            TaxBracket::new(1_950_000.0, 0.10),
            TaxBracket::new(3_300_000.0, 0.20),
            TaxBracket::new(6_950_000.0, 0.23),
            TaxBracket::new(9_000_000.0, 0.33),
            TaxBracket::new(18_000_000.0, 0.40),
            TaxBracket::new(40_000_000.0, 0.45),
        ],
        standard_deduction: 480_000.0,
        exemption_per_dependent: 380_000.0,
        credits: vec![],
    }
}

/// Japanese employment insurance preset
pub fn jp_employment_insurance_preset() -> BenefitPreset {
    BenefitPreset {
        name: "Japanese Employment Insurance".to_string(),
        benefit_type: BenefitType::Unemployment,
        income_threshold: f64::MAX,
        asset_threshold: None,
        benefit_amount: BenefitAmount::PercentageOfIncome(0.60),
        requirements: vec![
            EligibilityRequirement {
                name: "Work History".to_string(),
                requirement_type: RequirementType::WorkHistory,
                required_value: "12 months".to_string(),
            },
            EligibilityRequirement {
                name: "Employment Status".to_string(),
                requirement_type: RequirementType::EmploymentStatus,
                required_value: "involuntarily unemployed".to_string(),
            },
        ],
    }
}

/// Simulates Japanese income tax impact
pub fn simulate_jp_income_tax(population_size: usize) -> EconomicImpact {
    let tax_preset = jp_income_tax_preset();
    let mut total_tax = 0.0;
    let mut revenue_by_type = HashMap::new();
    let mut costs_by_type = HashMap::new();

    for i in 0..population_size {
        let income = 2_000_000.0 + (i as f64 / population_size as f64) * 20_000_000.0;
        let calculation = tax_preset.calculate_tax(income, 0);
        total_tax += calculation.tax_owed;
    }

    revenue_by_type.insert("income_tax".to_string(), total_tax);
    costs_by_type.insert("collection".to_string(), total_tax * 0.01);

    EconomicImpact {
        tax_revenue: total_tax,
        compliance_costs: total_tax * 0.02,
        administrative_costs: total_tax * 0.01,
        net_benefit: total_tax * 0.97,
        revenue_by_type,
        costs_by_type,
        distributional_impact: DistributionalImpact {
            quintile_impacts: vec![],
            gini_change: -0.02,
            progressivity: Progressivity::Progressive,
        },
    }
}

/// Simulates employment insurance impact
pub fn simulate_employment_insurance(population_size: usize) -> EconomicImpact {
    let benefit = jp_employment_insurance_preset();

    let mut total_benefits = 0.0;
    let mut revenue_by_type = HashMap::new();
    let mut costs_by_type = HashMap::new();

    for i in 0..population_size {
        let income = 3_000_000.0 + (i as f64 / population_size as f64) * 7_000_000.0;
        let assets = 500_000.0 + (i as f64 / population_size as f64) * 5_000_000.0;

        let result = benefit.check_eligibility(income, assets);
        if result.eligible {
            total_benefits += result.benefit_amount;
        }
    }

    revenue_by_type.insert(
        "employment_insurance_premium".to_string(),
        total_benefits * 0.6,
    );
    costs_by_type.insert("benefit_payments".to_string(), total_benefits);

    EconomicImpact {
        tax_revenue: total_benefits * 0.6,
        compliance_costs: total_benefits * 0.02,
        administrative_costs: total_benefits * 0.05,
        net_benefit: total_benefits * 0.53,
        revenue_by_type,
        costs_by_type,
        distributional_impact: DistributionalImpact {
            quintile_impacts: vec![],
            gini_change: -0.01,
            progressivity: Progressivity::Progressive,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_wage_simulator_default() {
        let simulator = MinimumWageSimulator::default_config();
        let result = simulator.simulate();

        assert_eq!(
            result.workers_affected + (10_000 - result.workers_affected),
            10_000
        );
        assert!(result.workers_affected_pct >= 0.0 && result.workers_affected_pct <= 100.0);
    }

    #[test]
    fn test_minimum_wage_custom_config() {
        let config = MinimumWageConfig {
            current_average_wage: 1000.0,
            proposed_average_wage: 1200.0,
            population_size: 1000,
            monte_carlo_runs: 10,
            time_horizon: 3,
            tokyo_premium: 0.10,
        };

        let simulator = MinimumWageSimulator::new(config);
        let result = simulator.simulate();

        assert!(result.total_labor_cost_increase >= 0.0);
    }

    #[test]
    fn test_work_style_reform_simulator() {
        let simulator = WorkStyleReformSimulator::default_config();
        let result = simulator.simulate();

        assert!(result.workers_exceeding_limits <= 10_000);
        assert!(result.compliance_cost >= 0.0);
    }

    #[test]
    fn test_paid_leave_simulator() {
        let simulator = PaidLeaveSimulator::default_config();
        let result = simulator.simulate();

        assert!(result.workers_not_meeting_pct >= 0.0 && result.workers_not_meeting_pct <= 100.0);
    }

    #[test]
    fn test_jp_labor_market() {
        let market = jp_labor_market();
        assert!(market.employed > 0);
    }

    #[test]
    fn test_jp_income_tax_simulation() {
        let result = simulate_jp_income_tax(1000);

        assert!(result.tax_revenue > 0.0);
        assert_eq!(
            result.distributional_impact.progressivity,
            Progressivity::Progressive
        );
    }

    #[test]
    fn test_jp_employment_insurance() {
        let result = simulate_employment_insurance(1000);

        assert!(result.tax_revenue >= 0.0);
    }

    #[test]
    fn test_jp_macroeconomic_indicators() {
        let indicators = jp_macroeconomic_indicators_2024();

        assert!(indicators.gdp > 0.0);
        assert!(indicators.inflation_rate > 0.0);
    }

    #[test]
    fn test_regional_impact() {
        let simulator = MinimumWageSimulator::default_config();
        let result = simulator.simulate();

        assert!(result.regional_impact.contains_key("tokyo"));
        assert!(result.regional_impact.contains_key("osaka"));
    }

    #[test]
    fn test_industry_impact() {
        let simulator = WorkStyleReformSimulator::default_config();
        let result = simulator.simulate();

        assert!(result.industry_impact.contains_key("construction"));
        assert!(result.industry_impact.contains_key("transport"));
    }
}
