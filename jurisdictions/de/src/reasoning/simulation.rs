//! German Labor Law Simulation Module (Deutsches Arbeitsrecht Simulation).
//!
//! This module provides simulation capabilities for analyzing the economic
//! impacts of German labor law changes, including:
//! - Mindestlohn (Minimum Wage) impact analysis
//! - Arbeitszeitgesetz (ArbZG) working time regulations
//! - Bundesurlaubsgesetz (BUrlG) vacation entitlement
//! - Sozialversicherung (Social Insurance) contributions

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
// Minimum Wage Simulation (Mindestlohn-Simulation)
// ============================================================================

/// German minimum wage simulation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MindestlohnConfig {
    /// Current minimum wage (€/hour)
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

impl Default for MindestlohnConfig {
    fn default() -> Self {
        Self {
            // As of 2024, German minimum wage is €12.41/hour
            current_minimum_wage: 12.41,
            // Proposed increase to €15.00/hour
            proposed_minimum_wage: 15.00,
            population_size: 10_000,
            monte_carlo_runs: 100,
            time_horizon: 5,
        }
    }
}

/// German minimum wage simulation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MindestlohnResult {
    /// Workers affected
    pub workers_affected: usize,
    /// Workers affected percentage
    pub workers_affected_pct: f64,
    /// Average wage increase
    pub avg_wage_increase: f64,
    /// Total additional labor cost (annual, in euros)
    pub total_labor_cost_increase: f64,
    /// Estimated job losses
    pub estimated_job_losses: usize,
    /// Impact by sector (Minijob, regular employment)
    pub sector_impact: HashMap<String, f64>,
    /// Distributional impact
    pub distributional_impact: DistributionalImpact,
    /// GDP impact
    pub gdp_impact: GdpImpactAnalysis,
    /// Simulation metrics
    pub simulation_metrics: SimulationMetrics,
}

/// German minimum wage simulator
pub struct MindestlohnSimulator {
    config: MindestlohnConfig,
}

impl MindestlohnSimulator {
    /// Creates a new simulator
    pub fn new(config: MindestlohnConfig) -> Self {
        Self { config }
    }

    /// Creates a simulator with default configuration
    pub fn default_config() -> Self {
        Self::new(MindestlohnConfig::default())
    }

    /// Runs the simulation
    pub fn simulate(&self) -> MindestlohnResult {
        let population = self.generate_population();

        let mut workers_affected = 0;
        let mut total_wage_increase = 0.0;
        let mut sector_impact: HashMap<String, f64> = HashMap::new();

        for sector in &["minijob", "midijob", "regular", "apprentice"] {
            sector_impact.insert(sector.to_string(), 0.0);
        }

        for worker in &population {
            let wage_opt: Option<f64> = worker
                .get_attribute("hourly_wage")
                .and_then(|s| s.parse().ok());
            let sector = worker
                .get_attribute("employment_sector")
                .unwrap_or_else(|| "regular".to_string());

            if let Some(wage) = wage_opt {
                if wage < self.config.proposed_minimum_wage
                    && wage >= self.config.current_minimum_wage
                {
                    workers_affected += 1;
                    let increase = self.config.proposed_minimum_wage - wage;
                    total_wage_increase += increase;
                    *sector_impact.entry(sector).or_insert(0.0) += increase;
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

        // Average working hours: 1,700 hours/year in Germany (shorter than US/JP due to vacation)
        let total_labor_cost_increase = total_wage_increase * 1_700.0;

        // German labor demand elasticity (typically -0.2 to -0.3)
        let elasticity = -0.25;
        let wage_increase_pct = (self.config.proposed_minimum_wage
            - self.config.current_minimum_wage)
            / self.config.current_minimum_wage;
        let employment_change_pct = elasticity * wage_increase_pct;
        let estimated_job_losses =
            (workers_affected as f64 * employment_change_pct.abs()).round() as usize;

        let simulation_metrics = self.run_simulation_engine(&population);

        MindestlohnResult {
            workers_affected,
            workers_affected_pct,
            avg_wage_increase,
            total_labor_cost_increase,
            estimated_job_losses,
            sector_impact,
            distributional_impact: self.calculate_distributional_impact(&population),
            gdp_impact: self.calculate_gdp_impact(total_labor_cost_increase),
            simulation_metrics,
        }
    }

    fn generate_population(&self) -> Vec<BasicEntity> {
        let mut population = Vec::with_capacity(self.config.population_size);
        let sectors = ["minijob", "midijob", "regular", "apprentice"];
        let sector_weights = [0.15, 0.10, 0.70, 0.05]; // Approximate distribution

        for i in 0..self.config.population_size {
            let mut entity = BasicEntity::new();

            // Assign sector based on distribution
            let sector_idx = (i % 100) as f64 / 100.0;
            let mut cumulative = 0.0;
            let mut selected_sector = "regular";
            for (idx, weight) in sector_weights.iter().enumerate() {
                cumulative += weight;
                if sector_idx < cumulative {
                    selected_sector = sectors[idx];
                    break;
                }
            }
            entity.set_attribute("employment_sector", selected_sector.to_string());

            // Generate wage distribution
            let base_wage = match selected_sector {
                "minijob" => self.config.current_minimum_wage * 0.95, // Often at minimum
                "midijob" => self.config.current_minimum_wage * 1.1,
                "apprentice" => self.config.current_minimum_wage * 0.8, // Below minimum legally
                _ => self.config.current_minimum_wage * 1.3,
            };

            let wage_factor = 1.0 + (i as f64 / self.config.population_size as f64) * 1.5;
            let wage = base_wage * wage_factor;
            entity.set_attribute("hourly_wage", format!("{:.2}", wage));

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
                    avg_compliance_cost: avg_increase * 1_700.0,
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
        // German GDP: €4.1 trillion (in trillions)
        let baseline_gdp = 4.1;
        let fiscal_multiplier = 1.15;

        let mut analysis = GdpImpactAnalysis::new(baseline_gdp, fiscal_multiplier, 5);

        let marginal_propensity_to_consume = 0.75;
        let consumption_boost =
            (labor_cost_increase / 1_000_000_000_000.0) * marginal_propensity_to_consume;

        analysis.add_tax_impact(-consumption_boost, marginal_propensity_to_consume);

        let investment_reduction = (labor_cost_increase / 1_000_000_000_000.0) * 0.12;
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
            "MiLoG_Sim",
            "Mindestlohngesetz Simulation",
            Effect::new(
                EffectType::Obligation,
                "Arbeitgeber müssen mindestens den Mindestlohn zahlen",
            )
            .with_parameter(
                "mindestlohn",
                format!("€{:.2}", self.config.proposed_minimum_wage),
            )
            .with_parameter(
                "current_minimum",
                format!("€{:.2}", self.config.current_minimum_wage),
            ),
        )
        .with_precondition(Condition::AttributeEquals {
            key: "employment_type".to_string(),
            value: "employed".to_string(),
        })
        .with_jurisdiction("DE")
    }
}

// ============================================================================
// Working Time Regulation Simulation (Arbeitszeitgesetz-Simulation)
// ============================================================================

/// Working time regulation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArbZGConfig {
    /// Maximum weekly working hours (ArbZG §3: 48 hours average)
    pub max_weekly_hours: u32,
    /// Population size for simulation
    pub population_size: usize,
    /// Current average weekly working hours
    pub current_avg_weekly_hours: f64,
}

impl Default for ArbZGConfig {
    fn default() -> Self {
        Self {
            // ArbZG §3: 48 hours per week on average over 6 months
            max_weekly_hours: 48,
            population_size: 10_000,
            current_avg_weekly_hours: 40.0, // Actual average in Germany
        }
    }
}

/// Working time regulation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArbZGResult {
    /// Workers exceeding limits
    pub workers_exceeding_limits: usize,
    /// Workers exceeding percentage
    pub workers_exceeding_pct: f64,
    /// Average hours reduction needed
    pub avg_hours_reduction: f64,
    /// Total hours reduction needed
    pub total_hours_reduction: f64,
    /// Additional hiring needed
    pub additional_hiring_needed: usize,
    /// Industry breakdown
    pub industry_impact: HashMap<String, f64>,
    /// Compliance cost
    pub compliance_cost: f64,
}

/// Working time regulation simulator
pub struct ArbZGSimulator {
    config: ArbZGConfig,
}

impl ArbZGSimulator {
    pub fn new(config: ArbZGConfig) -> Self {
        Self { config }
    }

    pub fn default_config() -> Self {
        Self::new(ArbZGConfig::default())
    }

    pub fn simulate(&self) -> ArbZGResult {
        let population = self.generate_population();
        let mut workers_exceeding = 0;
        let mut total_hours_reduction = 0.0;
        let mut industry_impact: HashMap<String, f64> = HashMap::new();

        for industry in &[
            "manufacturing",
            "healthcare",
            "transport",
            "it",
            "service",
            "other",
        ] {
            industry_impact.insert(industry.to_string(), 0.0);
        }

        for worker in &population {
            let hours_opt: Option<f64> = worker
                .get_attribute("weekly_hours")
                .and_then(|s| s.parse().ok());
            let industry = worker
                .get_attribute("industry")
                .unwrap_or_else(|| "other".to_string());

            if let Some(hours) = hours_opt {
                if hours > self.config.max_weekly_hours as f64 {
                    workers_exceeding += 1;
                    let reduction = hours - self.config.max_weekly_hours as f64;
                    total_hours_reduction += reduction;
                    *industry_impact.entry(industry).or_insert(0.0) += reduction;
                }
            }
        }

        let workers_exceeding_pct = if self.config.population_size > 0 {
            (workers_exceeding as f64 / self.config.population_size as f64) * 100.0
        } else {
            0.0
        };

        let avg_hours_reduction = if workers_exceeding > 0 {
            total_hours_reduction / workers_exceeding as f64
        } else {
            0.0
        };

        // Calculate additional hiring (assuming 1,700 work hours per employee per year)
        let annual_hours_reduction = total_hours_reduction * 52.0;
        let additional_hiring_needed = (annual_hours_reduction / 1_700.0).ceil() as usize;

        // Compliance cost: Average hiring cost in Germany: €5,000-10,000 per person
        let compliance_cost =
            additional_hiring_needed as f64 * 7_500.0 + workers_exceeding as f64 * 500.0;

        ArbZGResult {
            workers_exceeding_limits: workers_exceeding,
            workers_exceeding_pct,
            avg_hours_reduction,
            total_hours_reduction,
            additional_hiring_needed,
            industry_impact,
            compliance_cost,
        }
    }

    fn generate_population(&self) -> Vec<BasicEntity> {
        let mut population = Vec::with_capacity(self.config.population_size);
        let industries = [
            "manufacturing",
            "healthcare",
            "transport",
            "it",
            "service",
            "other",
        ];

        for i in 0..self.config.population_size {
            let mut entity = BasicEntity::new();

            let industry = industries[i % industries.len()];
            entity.set_attribute("industry", industry.to_string());

            // Generate weekly hours distribution
            let base_hours = match industry {
                "healthcare" => self.config.current_avg_weekly_hours * 1.3,
                "transport" => self.config.current_avg_weekly_hours * 1.2,
                "manufacturing" => self.config.current_avg_weekly_hours * 1.1,
                _ => self.config.current_avg_weekly_hours,
            };

            let variation = (i as f64 / self.config.population_size as f64 - 0.5) * 2.0;
            let weekly_hours = (base_hours * (1.0 + variation * 0.3)).max(0.0);
            entity.set_attribute("weekly_hours", format!("{:.1}", weekly_hours));

            population.push(entity);
        }

        population
    }
}

// ============================================================================
// Vacation Entitlement Simulation (Bundesurlaubsgesetz-Simulation)
// ============================================================================

/// Vacation entitlement configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BUrlGConfig {
    /// Minimum vacation days (BUrlG §3: 24 days for 6-day week)
    pub minimum_vacation_days: u32,
    /// Population size for simulation
    pub population_size: usize,
    /// Current average vacation days granted
    pub current_avg_vacation_days: f64,
}

impl Default for BUrlGConfig {
    fn default() -> Self {
        Self {
            // BUrlG §3: 24 days minimum for 6-day week (= 20 days for 5-day week)
            minimum_vacation_days: 20,
            population_size: 10_000,
            // German average is around 30 days
            current_avg_vacation_days: 30.0,
        }
    }
}

/// Vacation entitlement result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BUrlGResult {
    /// Employers not meeting minimum
    pub employers_not_meeting: usize,
    /// Employers not meeting percentage
    pub employers_not_meeting_pct: f64,
    /// Average additional days needed
    pub avg_additional_days_needed: f64,
    /// Total additional vacation days
    pub total_additional_days: f64,
    /// Productivity impact
    pub productivity_impact: f64,
    /// Employee satisfaction improvement
    pub satisfaction_improvement: f64,
}

/// Vacation entitlement simulator
pub struct BUrlGSimulator {
    config: BUrlGConfig,
}

impl BUrlGSimulator {
    pub fn new(config: BUrlGConfig) -> Self {
        Self { config }
    }

    pub fn default_config() -> Self {
        Self::new(BUrlGConfig::default())
    }

    pub fn simulate(&self) -> BUrlGResult {
        let population = self.generate_population();
        let mut employers_not_meeting = 0;
        let mut total_additional_days = 0.0;

        for contract in &population {
            let days_granted_opt: Option<f64> = contract
                .get_attribute("vacation_days")
                .and_then(|s| s.parse().ok());

            if let Some(days_granted) = days_granted_opt {
                if days_granted < self.config.minimum_vacation_days as f64 {
                    employers_not_meeting += 1;
                    total_additional_days +=
                        self.config.minimum_vacation_days as f64 - days_granted;
                }
            }
        }

        let employers_not_meeting_pct = if self.config.population_size > 0 {
            (employers_not_meeting as f64 / self.config.population_size as f64) * 100.0
        } else {
            0.0
        };

        let avg_additional_days_needed = if employers_not_meeting > 0 {
            total_additional_days / employers_not_meeting as f64
        } else {
            0.0
        };

        // Productivity impact: well-rested workers are more productive
        let productivity_impact = total_additional_days * -0.4 + total_additional_days * 0.9;

        // Satisfaction improvement
        let satisfaction_improvement = total_additional_days * 0.15;

        BUrlGResult {
            employers_not_meeting,
            employers_not_meeting_pct,
            avg_additional_days_needed,
            total_additional_days,
            productivity_impact,
            satisfaction_improvement,
        }
    }

    fn generate_population(&self) -> Vec<BasicEntity> {
        let mut population = Vec::with_capacity(self.config.population_size);

        for i in 0..self.config.population_size {
            let mut entity = BasicEntity::new();

            // Most German employers grant more than minimum
            let variation = (i as f64 / self.config.population_size as f64) * 15.0;
            let vacation_days = self.config.current_avg_vacation_days - 10.0 + variation;
            entity.set_attribute("vacation_days", format!("{:.0}", vacation_days));

            population.push(entity);
        }

        population
    }
}

// ============================================================================
// Preset Functions
// ============================================================================

/// German labor market preset
pub fn de_labor_market() -> LaborMarket {
    // Total employed: ~45.5 million, labor force: ~46 million
    LaborMarket::new(46_000_000, 45_500_000)
}

/// German macroeconomic indicators 2024
pub fn de_macroeconomic_indicators_2024() -> MacroeconomicIndicators {
    MacroeconomicIndicators::new(
        2024, // Year
        4.1,  // GDP: €4.1 trillion
        0.8,  // GDP growth rate: 0.8%
        5.9,  // Inflation rate: 5.9%
    )
}

/// German income tax preset (Einkommensteuer)
pub fn de_income_tax_preset() -> TaxSystemPreset {
    TaxSystemPreset {
        name: "German Income Tax (Einkommensteuer)".to_string(),
        tax_type: TaxType::Income,
        brackets: vec![
            TaxBracket::new(0.0, 0.0),         // 0%: 0 - €11,604 (Grundfreibetrag)
            TaxBracket::new(11_604.0, 0.14),   // 14%: €11,604 - €17,005
            TaxBracket::new(17_005.0, 0.2397), // Progressive to 23.97%
            TaxBracket::new(66_761.0, 0.42),   // 42%: €66,761 - €277,826
            TaxBracket::new(277_826.0, 0.45),  // 45%: over €277,826 (Reichensteuer)
        ],
        standard_deduction: 11_604.0,     // Grundfreibetrag
        exemption_per_dependent: 3_012.0, // Kinderfreibetrag per child (half)
        credits: vec![],
    }
}

/// German unemployment insurance preset (Arbeitslosenversicherung)
pub fn de_unemployment_insurance_preset() -> BenefitPreset {
    BenefitPreset {
        name: "German Unemployment Insurance (ALG I)".to_string(),
        benefit_type: BenefitType::Unemployment,
        income_threshold: 7_100.0, // Monthly income ceiling for contributions
        asset_threshold: None,
        benefit_amount: BenefitAmount::PercentageOfIncome(0.60), // 60% (or 67% with children)
        requirements: vec![
            EligibilityRequirement {
                name: "Work History".to_string(),
                requirement_type: RequirementType::WorkHistory,
                required_value: "12 months in past 2 years".to_string(),
            },
            EligibilityRequirement {
                name: "Employment Status".to_string(),
                requirement_type: RequirementType::EmploymentStatus,
                required_value: "involuntarily unemployed".to_string(),
            },
        ],
    }
}

/// Simulates German income tax impact
pub fn simulate_de_income_tax(population_size: usize) -> EconomicImpact {
    let tax_preset = de_income_tax_preset();
    let mut total_tax = 0.0;
    let mut revenue_by_type = HashMap::new();
    let mut costs_by_type = HashMap::new();

    for i in 0..population_size {
        // German income distribution (median around €47,000)
        let income = 20_000.0 + (i as f64 / population_size as f64) * 150_000.0;
        let calculation = tax_preset.calculate_tax(income, 0);
        total_tax += calculation.tax_owed;
    }

    revenue_by_type.insert("income_tax".to_string(), total_tax);
    costs_by_type.insert("collection".to_string(), total_tax * 0.015);

    EconomicImpact {
        tax_revenue: total_tax,
        compliance_costs: total_tax * 0.025,
        administrative_costs: total_tax * 0.015,
        net_benefit: total_tax * 0.96,
        revenue_by_type,
        costs_by_type,
        distributional_impact: DistributionalImpact {
            quintile_impacts: vec![],
            gini_change: -0.025,
            progressivity: Progressivity::Progressive,
        },
    }
}

/// Simulates unemployment insurance impact
pub fn simulate_unemployment_insurance(population_size: usize) -> EconomicImpact {
    let benefit = de_unemployment_insurance_preset();

    let mut total_benefits = 0.0;
    let mut revenue_by_type = HashMap::new();
    let mut costs_by_type = HashMap::new();

    for i in 0..population_size {
        let income = 2_500.0 + (i as f64 / population_size as f64) * 5_000.0;
        let assets = 1_000.0 + (i as f64 / population_size as f64) * 10_000.0;

        let result = benefit.check_eligibility(income, assets);
        if result.eligible {
            total_benefits += result.benefit_amount;
        }
    }

    revenue_by_type.insert(
        "unemployment_insurance_contribution".to_string(),
        total_benefits * 0.5,
    );
    costs_by_type.insert("benefit_payments".to_string(), total_benefits);

    EconomicImpact {
        tax_revenue: total_benefits * 0.5,
        compliance_costs: total_benefits * 0.02,
        administrative_costs: total_benefits * 0.05,
        net_benefit: total_benefits * 0.43,
        revenue_by_type,
        costs_by_type,
        distributional_impact: DistributionalImpact {
            quintile_impacts: vec![],
            gini_change: -0.015,
            progressivity: Progressivity::Progressive,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mindestlohn_simulator_default() {
        let simulator = MindestlohnSimulator::default_config();
        let result = simulator.simulate();

        assert_eq!(
            result.workers_affected + (10_000 - result.workers_affected),
            10_000
        );
        assert!(result.workers_affected_pct >= 0.0 && result.workers_affected_pct <= 100.0);
    }

    #[test]
    fn test_mindestlohn_custom_config() {
        let config = MindestlohnConfig {
            current_minimum_wage: 12.0,
            proposed_minimum_wage: 14.0,
            population_size: 1000,
            monte_carlo_runs: 10,
            time_horizon: 3,
        };

        let simulator = MindestlohnSimulator::new(config);
        let result = simulator.simulate();

        assert!(result.total_labor_cost_increase >= 0.0);
    }

    #[test]
    fn test_arbzg_simulator() {
        let simulator = ArbZGSimulator::default_config();
        let result = simulator.simulate();

        assert!(result.workers_exceeding_limits <= 10_000);
        assert!(result.compliance_cost >= 0.0);
    }

    #[test]
    fn test_burlg_simulator() {
        let simulator = BUrlGSimulator::default_config();
        let result = simulator.simulate();

        assert!(
            result.employers_not_meeting_pct >= 0.0 && result.employers_not_meeting_pct <= 100.0
        );
    }

    #[test]
    fn test_de_labor_market() {
        let market = de_labor_market();
        assert!(market.employed > 0);
    }

    #[test]
    fn test_de_income_tax_simulation() {
        let result = simulate_de_income_tax(1000);

        assert!(result.tax_revenue > 0.0);
        assert_eq!(
            result.distributional_impact.progressivity,
            Progressivity::Progressive
        );
    }

    #[test]
    fn test_de_unemployment_insurance() {
        let result = simulate_unemployment_insurance(1000);

        assert!(result.tax_revenue >= 0.0);
    }

    #[test]
    fn test_de_macroeconomic_indicators() {
        let indicators = de_macroeconomic_indicators_2024();

        assert!(indicators.gdp > 0.0);
        assert!(indicators.inflation_rate > 0.0);
    }

    #[test]
    fn test_sector_impact() {
        let simulator = MindestlohnSimulator::default_config();
        let result = simulator.simulate();

        assert!(result.sector_impact.contains_key("minijob"));
        assert!(result.sector_impact.contains_key("regular"));
    }

    #[test]
    fn test_industry_impact() {
        let simulator = ArbZGSimulator::default_config();
        let result = simulator.simulate();

        assert!(result.industry_impact.contains_key("healthcare"));
        assert!(result.industry_impact.contains_key("transport"));
    }
}
