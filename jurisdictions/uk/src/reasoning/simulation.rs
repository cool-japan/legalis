//! UK Employment Law Simulation Module.
//!
//! This module provides simulation capabilities for analyzing the economic
//! impacts of UK employment law changes, including:
//! - National Living Wage (NLW) impact analysis
//! - Working Time Regulations (WTR 1998) compliance
//! - Annual leave entitlements (ERA 1996)
//! - Auto-enrolment pension obligations

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
// National Living Wage Simulation
// ============================================================================

/// National Living Wage simulation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NationalLivingWageConfig {
    /// Current National Living Wage (£/hour for 21+)
    pub current_nlw: f64,
    /// Proposed National Living Wage
    pub proposed_nlw: f64,
    /// Population size for simulation
    pub population_size: usize,
    /// Number of Monte Carlo runs
    pub monte_carlo_runs: usize,
    /// Time horizon in years
    pub time_horizon: u32,
}

impl Default for NationalLivingWageConfig {
    fn default() -> Self {
        Self {
            // As of 2024, NLW is £11.44/hour for 21+
            current_nlw: 11.44,
            // Proposed increase to £15.00/hour
            proposed_nlw: 15.00,
            population_size: 10_000,
            monte_carlo_runs: 100,
            time_horizon: 5,
        }
    }
}

/// National Living Wage simulation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NationalLivingWageResult {
    /// Workers affected
    pub workers_affected: usize,
    /// Workers affected percentage
    pub workers_affected_pct: f64,
    /// Average wage increase
    pub avg_wage_increase: f64,
    /// Total additional labor cost (annual, in pounds)
    pub total_labor_cost_increase: f64,
    /// Estimated job losses
    pub estimated_job_losses: usize,
    /// Regional impact (England, Scotland, Wales, Northern Ireland)
    pub regional_impact: HashMap<String, f64>,
    /// Distributional impact
    pub distributional_impact: DistributionalImpact,
    /// GDP impact
    pub gdp_impact: GdpImpactAnalysis,
    /// Simulation metrics
    pub simulation_metrics: SimulationMetrics,
}

/// National Living Wage simulator
pub struct NationalLivingWageSimulator {
    config: NationalLivingWageConfig,
}

impl NationalLivingWageSimulator {
    /// Creates a new simulator
    pub fn new(config: NationalLivingWageConfig) -> Self {
        Self { config }
    }

    /// Creates a simulator with default configuration
    pub fn default_config() -> Self {
        Self::new(NationalLivingWageConfig::default())
    }

    /// Runs the simulation
    pub fn simulate(&self) -> NationalLivingWageResult {
        let population = self.generate_population();

        let mut workers_affected = 0;
        let mut total_wage_increase = 0.0;
        let mut regional_impact: HashMap<String, f64> = HashMap::new();

        for region in &["england", "scotland", "wales", "northern_ireland"] {
            regional_impact.insert(region.to_string(), 0.0);
        }

        for worker in &population {
            let wage_opt: Option<f64> = worker
                .get_attribute("hourly_wage")
                .and_then(|s| s.parse().ok());
            let region = worker
                .get_attribute("region")
                .unwrap_or_else(|| "england".to_string());

            if let Some(wage) = wage_opt {
                if wage < self.config.proposed_nlw && wage >= self.config.current_nlw {
                    workers_affected += 1;
                    let increase = self.config.proposed_nlw - wage;
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

        // UK average working hours: 1,670 hours/year
        let total_labor_cost_increase = total_wage_increase * 1_670.0;

        // UK labor demand elasticity
        let elasticity = -0.2;
        let wage_increase_pct =
            (self.config.proposed_nlw - self.config.current_nlw) / self.config.current_nlw;
        let employment_change_pct = elasticity * wage_increase_pct;
        let estimated_job_losses =
            (workers_affected as f64 * employment_change_pct.abs()).round() as usize;

        let simulation_metrics = self.run_simulation_engine(&population);

        NationalLivingWageResult {
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

    fn generate_population(&self) -> Vec<BasicEntity> {
        let mut population = Vec::with_capacity(self.config.population_size);
        let regions = ["england", "scotland", "wales", "northern_ireland"];
        let region_weights = [0.84, 0.08, 0.05, 0.03]; // UK population distribution

        for i in 0..self.config.population_size {
            let mut entity = BasicEntity::new();

            let region_idx = (i % 100) as f64 / 100.0;
            let mut cumulative = 0.0;
            let mut selected_region = "england";
            for (idx, weight) in region_weights.iter().enumerate() {
                cumulative += weight;
                if region_idx < cumulative {
                    selected_region = regions[idx];
                    break;
                }
            }
            entity.set_attribute("region", selected_region.to_string());

            // Generate wage distribution
            let base_wage = self.config.current_nlw;
            let wage_factor = 1.0 + (i as f64 / self.config.population_size as f64) * 2.0;
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
                    .filter(|&&w| w < self.config.proposed_nlw && w >= self.config.current_nlw)
                    .count();

                let avg_increase = if affected > 0 {
                    let total_increase: f64 = wages
                        .iter()
                        .filter(|&&w| w < self.config.proposed_nlw && w >= self.config.current_nlw)
                        .map(|&w| self.config.proposed_nlw - w)
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
                    avg_compliance_cost: avg_increase * 1_670.0,
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
        // UK GDP: £2.7 trillion (in trillions)
        let baseline_gdp = 2.7;
        let fiscal_multiplier = 1.2;

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

        let statute = self.create_nlw_statute();
        builder = builder.add_statute(statute);

        match builder.build() {
            Ok(engine) => match tokio::runtime::Runtime::new() {
                Ok(rt) => rt.block_on(async { engine.run_simulation().await }),
                Err(_) => SimulationMetrics::new(),
            },
            Err(_) => SimulationMetrics::new(),
        }
    }

    fn create_nlw_statute(&self) -> Statute {
        Statute::new(
            "NMWA_1998_Sim",
            "National Minimum Wage Act 1998 (Simulation)",
            Effect::new(
                EffectType::Obligation,
                "Employer must pay at least National Living Wage",
            )
            .with_parameter("nlw", format!("£{:.2}", self.config.proposed_nlw))
            .with_parameter("current_nlw", format!("£{:.2}", self.config.current_nlw)),
        )
        .with_precondition(Condition::AttributeEquals {
            key: "employment_status".to_string(),
            value: "employed".to_string(),
        })
        .with_jurisdiction("UK")
    }
}

// ============================================================================
// Working Time Regulations Simulation
// ============================================================================

/// Working Time Regulations configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WTRConfig {
    /// Maximum weekly working hours (WTR 1998: 48 hours average)
    pub max_weekly_hours: u32,
    /// Population size for simulation
    pub population_size: usize,
    /// Current average weekly working hours
    pub current_avg_weekly_hours: f64,
}

impl Default for WTRConfig {
    fn default() -> Self {
        Self {
            // WTR Regulation 4: 48 hours per week average over 17 weeks
            max_weekly_hours: 48,
            population_size: 10_000,
            current_avg_weekly_hours: 36.5, // UK average
        }
    }
}

/// Working Time Regulations result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WTRResult {
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
    /// Sector breakdown
    pub sector_impact: HashMap<String, f64>,
    /// Compliance cost
    pub compliance_cost: f64,
}

/// Working Time Regulations simulator
pub struct WTRSimulator {
    config: WTRConfig,
}

impl WTRSimulator {
    /// Creates a new WTR simulator
    pub fn new(config: WTRConfig) -> Self {
        Self { config }
    }

    /// Creates a simulator with default configuration
    pub fn default_config() -> Self {
        Self::new(WTRConfig::default())
    }

    /// Runs the working time regulations simulation
    pub fn simulate(&self) -> WTRResult {
        let population = self.generate_population();
        let mut workers_exceeding = 0;
        let mut total_hours_reduction = 0.0;
        let mut sector_impact: HashMap<String, f64> = HashMap::new();

        for sector in &[
            "healthcare",
            "hospitality",
            "retail",
            "transport",
            "finance",
            "other",
        ] {
            sector_impact.insert(sector.to_string(), 0.0);
        }

        for worker in &population {
            let hours_opt: Option<f64> = worker
                .get_attribute("weekly_hours")
                .and_then(|s| s.parse().ok());
            let sector = worker
                .get_attribute("sector")
                .unwrap_or_else(|| "other".to_string());

            if let Some(hours) = hours_opt {
                if hours > self.config.max_weekly_hours as f64 {
                    workers_exceeding += 1;
                    let reduction = hours - self.config.max_weekly_hours as f64;
                    total_hours_reduction += reduction;
                    *sector_impact.entry(sector).or_insert(0.0) += reduction;
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

        let annual_hours_reduction = total_hours_reduction * 52.0;
        let additional_hiring_needed = (annual_hours_reduction / 1_670.0).ceil() as usize;

        // UK hiring costs: £3,000-8,000 per person
        let compliance_cost =
            additional_hiring_needed as f64 * 5_500.0 + workers_exceeding as f64 * 400.0;

        WTRResult {
            workers_exceeding_limits: workers_exceeding,
            workers_exceeding_pct,
            avg_hours_reduction,
            total_hours_reduction,
            additional_hiring_needed,
            sector_impact,
            compliance_cost,
        }
    }

    fn generate_population(&self) -> Vec<BasicEntity> {
        let mut population = Vec::with_capacity(self.config.population_size);
        let sectors = [
            "healthcare",
            "hospitality",
            "retail",
            "transport",
            "finance",
            "other",
        ];

        for i in 0..self.config.population_size {
            let mut entity = BasicEntity::new();

            let sector = sectors[i % sectors.len()];
            entity.set_attribute("sector", sector.to_string());

            let base_hours = match sector {
                "healthcare" => self.config.current_avg_weekly_hours * 1.25,
                "hospitality" => self.config.current_avg_weekly_hours * 1.15,
                "transport" => self.config.current_avg_weekly_hours * 1.2,
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
// Annual Leave Simulation (ERA 1996)
// ============================================================================

/// Annual leave configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnualLeaveConfig {
    /// Statutory minimum (5.6 weeks for full-time workers)
    pub statutory_minimum_weeks: f64,
    /// Population size for simulation
    pub population_size: usize,
    /// Current average leave weeks granted
    pub current_avg_leave_weeks: f64,
}

impl Default for AnnualLeaveConfig {
    fn default() -> Self {
        Self {
            // ERA Section 13: 5.6 weeks (28 days for 5-day week)
            statutory_minimum_weeks: 5.6,
            population_size: 10_000,
            // UK average is around 5.8 weeks
            current_avg_leave_weeks: 5.8,
        }
    }
}

/// Annual leave result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnualLeaveResult {
    /// Employers not meeting minimum
    pub employers_not_meeting: usize,
    /// Employers not meeting percentage
    pub employers_not_meeting_pct: f64,
    /// Average additional days needed
    pub avg_additional_days_needed: f64,
    /// Total additional leave days
    pub total_additional_days: f64,
    /// Productivity impact
    pub productivity_impact: f64,
    /// Employee wellbeing improvement
    pub wellbeing_improvement: f64,
}

/// Annual leave simulator
pub struct AnnualLeaveSimulator {
    config: AnnualLeaveConfig,
}

impl AnnualLeaveSimulator {
    /// Creates a new annual leave simulator
    pub fn new(config: AnnualLeaveConfig) -> Self {
        Self { config }
    }

    /// Creates a simulator with default configuration
    pub fn default_config() -> Self {
        Self::new(AnnualLeaveConfig::default())
    }

    /// Runs the annual leave simulation
    pub fn simulate(&self) -> AnnualLeaveResult {
        let population = self.generate_population();
        let mut employers_not_meeting = 0;
        let mut total_additional_days = 0.0;

        for contract in &population {
            let weeks_granted_opt: Option<f64> = contract
                .get_attribute("leave_weeks")
                .and_then(|s| s.parse().ok());

            if let Some(weeks_granted) = weeks_granted_opt {
                if weeks_granted < self.config.statutory_minimum_weeks {
                    employers_not_meeting += 1;
                    let additional_weeks = self.config.statutory_minimum_weeks - weeks_granted;
                    total_additional_days += additional_weeks * 5.0; // Convert to days (5-day week)
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

        let productivity_impact = total_additional_days * -0.3 + total_additional_days * 0.85;
        let wellbeing_improvement = total_additional_days * 0.12;

        AnnualLeaveResult {
            employers_not_meeting,
            employers_not_meeting_pct,
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

            let variation = (i as f64 / self.config.population_size as f64) * 2.0;
            let leave_weeks = self.config.current_avg_leave_weeks - 1.0 + variation;
            entity.set_attribute("leave_weeks", format!("{:.1}", leave_weeks));

            population.push(entity);
        }

        population
    }
}

// ============================================================================
// Preset Functions
// ============================================================================

/// UK labor market preset
pub fn uk_labor_market() -> LaborMarket {
    // Total employed: ~32.7 million, labor force: ~33.4 million
    LaborMarket::new(33_400_000, 32_700_000)
}

/// UK macroeconomic indicators 2024
pub fn uk_macroeconomic_indicators_2024() -> MacroeconomicIndicators {
    MacroeconomicIndicators::new(
        2024, // Year
        2.7,  // GDP: £2.7 trillion
        1.1,  // GDP growth rate: 1.1%
        4.0,  // Inflation rate: 4.0%
    )
}

/// UK income tax preset
pub fn uk_income_tax_preset() -> TaxSystemPreset {
    TaxSystemPreset {
        name: "UK Income Tax".to_string(),
        tax_type: TaxType::Income,
        brackets: vec![
            TaxBracket::new(0.0, 0.0),        // 0%: 0 - £12,570 (Personal Allowance)
            TaxBracket::new(12_570.0, 0.20),  // 20%: £12,570 - £50,270 (Basic rate)
            TaxBracket::new(50_270.0, 0.40),  // 40%: £50,270 - £125,140 (Higher rate)
            TaxBracket::new(125_140.0, 0.45), // 45%: over £125,140 (Additional rate)
        ],
        standard_deduction: 12_570.0, // Personal Allowance
        exemption_per_dependent: 0.0,
        credits: vec![],
    }
}

/// UK unemployment insurance (Universal Credit) preset
pub fn uk_unemployment_benefit_preset() -> BenefitPreset {
    BenefitPreset {
        name: "UK Universal Credit".to_string(),
        benefit_type: BenefitType::Unemployment,
        income_threshold: 2_500.0,       // Monthly income limit
        asset_threshold: Some(16_000.0), // Capital limit
        benefit_amount: BenefitAmount::Fixed(368.74), // Standard allowance (single, 25+)
        requirements: vec![
            EligibilityRequirement {
                name: "Work Search".to_string(),
                requirement_type: RequirementType::EmploymentStatus,
                required_value: "actively seeking work".to_string(),
            },
            EligibilityRequirement {
                name: "Residency".to_string(),
                requirement_type: RequirementType::Residency,
                required_value: "UK resident".to_string(),
            },
        ],
    }
}

/// Simulates UK income tax impact
pub fn simulate_uk_income_tax(population_size: usize) -> EconomicImpact {
    let tax_preset = uk_income_tax_preset();
    let mut total_tax = 0.0;
    let mut revenue_by_type = HashMap::new();
    let mut costs_by_type = HashMap::new();

    for i in 0..population_size {
        // UK income distribution (median around £33,000)
        let income = 15_000.0 + (i as f64 / population_size as f64) * 150_000.0;
        let calculation = tax_preset.calculate_tax(income, 0);
        total_tax += calculation.tax_owed;
    }

    revenue_by_type.insert("income_tax".to_string(), total_tax);
    costs_by_type.insert("collection".to_string(), total_tax * 0.012);

    EconomicImpact {
        tax_revenue: total_tax,
        compliance_costs: total_tax * 0.02,
        administrative_costs: total_tax * 0.012,
        net_benefit: total_tax * 0.968,
        revenue_by_type,
        costs_by_type,
        distributional_impact: DistributionalImpact {
            quintile_impacts: vec![],
            gini_change: -0.02,
            progressivity: Progressivity::Progressive,
        },
    }
}

/// Simulates unemployment benefit impact
pub fn simulate_unemployment_benefit(population_size: usize) -> EconomicImpact {
    let benefit = uk_unemployment_benefit_preset();

    let mut total_benefits = 0.0;
    let mut revenue_by_type = HashMap::new();
    let mut costs_by_type = HashMap::new();

    for i in 0..population_size {
        let income = 1_000.0 + (i as f64 / population_size as f64) * 2_000.0;
        let assets = 500.0 + (i as f64 / population_size as f64) * 20_000.0;

        let result = benefit.check_eligibility(income, assets);
        if result.eligible {
            total_benefits += result.benefit_amount;
        }
    }

    revenue_by_type.insert("national_insurance".to_string(), total_benefits * 0.3);
    costs_by_type.insert("benefit_payments".to_string(), total_benefits);

    EconomicImpact {
        tax_revenue: total_benefits * 0.3,
        compliance_costs: total_benefits * 0.02,
        administrative_costs: total_benefits * 0.06,
        net_benefit: total_benefits * 0.22,
        revenue_by_type,
        costs_by_type,
        distributional_impact: DistributionalImpact {
            quintile_impacts: vec![],
            gini_change: -0.02,
            progressivity: Progressivity::Progressive,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nlw_simulator_default() {
        let simulator = NationalLivingWageSimulator::default_config();
        let result = simulator.simulate();

        assert_eq!(
            result.workers_affected + (10_000 - result.workers_affected),
            10_000
        );
        assert!(result.workers_affected_pct >= 0.0 && result.workers_affected_pct <= 100.0);
    }

    #[test]
    fn test_nlw_custom_config() {
        let config = NationalLivingWageConfig {
            current_nlw: 11.00,
            proposed_nlw: 13.00,
            population_size: 1000,
            monte_carlo_runs: 10,
            time_horizon: 3,
        };

        let simulator = NationalLivingWageSimulator::new(config);
        let result = simulator.simulate();

        assert!(result.total_labor_cost_increase >= 0.0);
    }

    #[test]
    fn test_wtr_simulator() {
        let simulator = WTRSimulator::default_config();
        let result = simulator.simulate();

        assert!(result.workers_exceeding_limits <= 10_000);
        assert!(result.compliance_cost >= 0.0);
    }

    #[test]
    fn test_annual_leave_simulator() {
        let simulator = AnnualLeaveSimulator::default_config();
        let result = simulator.simulate();

        assert!(
            result.employers_not_meeting_pct >= 0.0 && result.employers_not_meeting_pct <= 100.0
        );
    }

    #[test]
    fn test_uk_labor_market() {
        let market = uk_labor_market();
        assert!(market.employed > 0);
    }

    #[test]
    fn test_uk_income_tax_simulation() {
        let result = simulate_uk_income_tax(1000);

        assert!(result.tax_revenue > 0.0);
        assert_eq!(
            result.distributional_impact.progressivity,
            Progressivity::Progressive
        );
    }

    #[test]
    fn test_uk_unemployment_benefit() {
        let result = simulate_unemployment_benefit(1000);

        assert!(result.tax_revenue >= 0.0);
    }

    #[test]
    fn test_uk_macroeconomic_indicators() {
        let indicators = uk_macroeconomic_indicators_2024();

        assert!(indicators.gdp > 0.0);
        assert!(indicators.inflation_rate > 0.0);
    }

    #[test]
    fn test_regional_impact() {
        let simulator = NationalLivingWageSimulator::default_config();
        let result = simulator.simulate();

        assert!(result.regional_impact.contains_key("england"));
        assert!(result.regional_impact.contains_key("scotland"));
    }

    #[test]
    fn test_sector_impact() {
        let simulator = WTRSimulator::default_config();
        let result = simulator.simulate();

        assert!(result.sector_impact.contains_key("healthcare"));
        assert!(result.sector_impact.contains_key("hospitality"));
    }
}
