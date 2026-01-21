//! EU GDPR Simulation Module.
//!
//! This module provides simulation capabilities for analyzing GDPR compliance
//! impacts across the European Union:
//! - GDPR fine impact analysis (Article 83)
//! - Compliance cost simulation
//! - Data protection impact assessment simulation
//! - Risk analysis for data controllers and processors

use legalis_core::{BasicEntity, Effect, EffectType, LegalEntity, Statute};
use legalis_sim::{
    CompliancePreset, DistributionalImpact, EconomicImpact, Progressivity, QuintileImpact,
    SimEngineBuilder, SimulationMetrics,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// GDPR Fine Simulation (Article 83)
// ============================================================================

/// GDPR fine simulation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GdprFineConfig {
    /// Number of companies to simulate
    pub company_count: usize,
    /// Number of Monte Carlo runs
    pub monte_carlo_runs: usize,
    /// Distribution of company sizes (percentage as SME)
    pub sme_percentage: f64,
    /// Average violation rate
    pub violation_rate: f64,
}

impl Default for GdprFineConfig {
    fn default() -> Self {
        Self {
            company_count: 10_000,
            monte_carlo_runs: 100,
            sme_percentage: 0.85,
            violation_rate: 0.05,
        }
    }
}

/// GDPR fine tier under Article 83
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GdprFineTier {
    /// Lower tier: up to 10M EUR or 2% of global annual turnover
    LowerTier,
    /// Higher tier: up to 20M EUR or 4% of global annual turnover
    HigherTier,
}

/// GDPR violation type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GdprViolation {
    /// Violation ID
    pub id: String,
    /// Article violated
    pub article: String,
    /// Description
    pub description: String,
    /// Fine tier
    pub tier: GdprFineTier,
    /// Base fine amount (EUR)
    pub base_fine: f64,
}

impl GdprViolation {
    /// Creates a violation for Article 5 (Processing Principles)
    pub fn article_5_principles() -> Self {
        Self {
            id: "GDPR_Art5".to_string(),
            article: "Article 5".to_string(),
            description: "Violation of data processing principles".to_string(),
            tier: GdprFineTier::HigherTier,
            base_fine: 10_000_000.0,
        }
    }

    /// Creates a violation for Article 6 (Lawfulness of Processing)
    pub fn article_6_lawfulness() -> Self {
        Self {
            id: "GDPR_Art6".to_string(),
            article: "Article 6".to_string(),
            description: "Processing without lawful basis".to_string(),
            tier: GdprFineTier::HigherTier,
            base_fine: 10_000_000.0,
        }
    }

    /// Creates a violation for Article 7 (Consent)
    pub fn article_7_consent() -> Self {
        Self {
            id: "GDPR_Art7".to_string(),
            article: "Article 7".to_string(),
            description: "Invalid consent conditions".to_string(),
            tier: GdprFineTier::HigherTier,
            base_fine: 8_000_000.0,
        }
    }

    /// Creates a violation for Article 17 (Right to Erasure)
    pub fn article_17_erasure() -> Self {
        Self {
            id: "GDPR_Art17".to_string(),
            article: "Article 17".to_string(),
            description: "Failure to comply with erasure request".to_string(),
            tier: GdprFineTier::HigherTier,
            base_fine: 5_000_000.0,
        }
    }

    /// Creates a violation for Article 32 (Security of Processing)
    pub fn article_32_security() -> Self {
        Self {
            id: "GDPR_Art32".to_string(),
            article: "Article 32".to_string(),
            description: "Inadequate security measures".to_string(),
            tier: GdprFineTier::LowerTier,
            base_fine: 3_000_000.0,
        }
    }

    /// Creates a violation for Article 33 (Data Breach Notification)
    pub fn article_33_breach_notification() -> Self {
        Self {
            id: "GDPR_Art33".to_string(),
            article: "Article 33".to_string(),
            description: "Failure to notify data breach within 72 hours".to_string(),
            tier: GdprFineTier::LowerTier,
            base_fine: 2_000_000.0,
        }
    }

    /// Creates a violation for Article 44 (International Transfers)
    pub fn article_44_transfers() -> Self {
        Self {
            id: "GDPR_Art44".to_string(),
            article: "Article 44".to_string(),
            description: "Unlawful international data transfer".to_string(),
            tier: GdprFineTier::HigherTier,
            base_fine: 15_000_000.0,
        }
    }
}

/// GDPR fine simulation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GdprFineResult {
    /// Total companies simulated
    pub companies_simulated: usize,
    /// Companies with violations
    pub companies_with_violations: usize,
    /// Total fines assessed (EUR)
    pub total_fines: f64,
    /// Average fine per violation
    pub average_fine: f64,
    /// Maximum fine assessed
    pub max_fine: f64,
    /// Fine distribution by tier
    pub fines_by_tier: HashMap<String, f64>,
    /// Fine distribution by violation type
    pub fines_by_violation: HashMap<String, f64>,
    /// Companies affected by company size
    pub affected_by_size: HashMap<String, usize>,
    /// Economic distributional impact
    pub distributional_impact: DistributionalImpact,
    /// Simulation metrics
    pub simulation_metrics: SimulationMetrics,
}

/// GDPR fine simulator
pub struct GdprFineSimulator {
    config: GdprFineConfig,
    violations: Vec<GdprViolation>,
}

impl GdprFineSimulator {
    /// Creates a new simulator with the given configuration
    pub fn new(config: GdprFineConfig) -> Self {
        Self {
            config,
            violations: vec![
                GdprViolation::article_5_principles(),
                GdprViolation::article_6_lawfulness(),
                GdprViolation::article_7_consent(),
                GdprViolation::article_17_erasure(),
                GdprViolation::article_32_security(),
                GdprViolation::article_33_breach_notification(),
                GdprViolation::article_44_transfers(),
            ],
        }
    }

    /// Creates a simulator with default configuration
    pub fn default_config() -> Self {
        Self::new(GdprFineConfig::default())
    }

    /// Runs the GDPR fine impact simulation
    pub fn simulate(&self) -> GdprFineResult {
        let companies = self.generate_company_population();

        let mut companies_with_violations = 0;
        let mut total_fines = 0.0;
        let mut max_fine = 0.0;
        let mut fines_by_tier: HashMap<String, f64> = HashMap::new();
        let mut fines_by_violation: HashMap<String, f64> = HashMap::new();
        let mut affected_by_size: HashMap<String, usize> = HashMap::new();

        fines_by_tier.insert("LowerTier".to_string(), 0.0);
        fines_by_tier.insert("HigherTier".to_string(), 0.0);

        for company in &companies {
            if let Some(has_violation) = company.get_attribute("has_violation")
                && has_violation == "true"
            {
                companies_with_violations += 1;

                let fine = self.calculate_fine(company);
                total_fines += fine;
                if fine > max_fine {
                    max_fine = fine;
                }

                // Track by violation type
                if let Some(violation_type) = company.get_attribute("violation_type") {
                    *fines_by_violation
                        .entry(violation_type.clone())
                        .or_insert(0.0) += fine;

                    // Determine tier
                    let violation = self
                        .violations
                        .iter()
                        .find(|v| v.id == violation_type)
                        .cloned();
                    if let Some(v) = violation {
                        let tier_key = match v.tier {
                            GdprFineTier::LowerTier => "LowerTier",
                            GdprFineTier::HigherTier => "HigherTier",
                        };
                        *fines_by_tier.entry(tier_key.to_string()).or_insert(0.0) += fine;
                    }
                }

                // Track by company size
                if let Some(size) = company.get_attribute("company_size") {
                    *affected_by_size.entry(size.clone()).or_insert(0) += 1;
                }
            }
        }

        let average_fine = if companies_with_violations > 0 {
            total_fines / companies_with_violations as f64
        } else {
            0.0
        };

        // Run engine simulation
        let metrics = self.run_engine_simulation(&companies);

        // Calculate distributional impact
        let distributional_impact = self.calculate_distributional_impact(&companies);

        GdprFineResult {
            companies_simulated: self.config.company_count,
            companies_with_violations,
            total_fines,
            average_fine,
            max_fine,
            fines_by_tier,
            fines_by_violation,
            affected_by_size,
            distributional_impact,
            simulation_metrics: metrics,
        }
    }

    /// Generates a population of companies for simulation
    fn generate_company_population(&self) -> Vec<BasicEntity> {
        let mut companies = Vec::with_capacity(self.config.company_count);

        for i in 0..self.config.company_count {
            let mut entity = BasicEntity::new();

            // Company size (SME vs Large)
            let is_sme = (i as f64 / self.config.company_count as f64) < self.config.sme_percentage;
            entity.set_attribute(
                "company_size",
                if is_sme { "SME" } else { "Large" }.to_string(),
            );

            // Annual turnover (EUR)
            let turnover = if is_sme {
                1_000_000.0 + (i as f64 / self.config.company_count as f64) * 49_000_000.0
            } else {
                50_000_000.0 + (i as f64 / self.config.company_count as f64) * 9_950_000_000.0
            };
            entity.set_attribute("annual_turnover", format!("{:.0}", turnover));

            // Violation status
            let has_violation =
                (i as f64 / self.config.company_count as f64) < self.config.violation_rate;
            entity.set_attribute("has_violation", has_violation.to_string());

            if has_violation {
                // Assign random violation type
                let violation_index = i % self.violations.len();
                entity.set_attribute(
                    "violation_type",
                    self.violations[violation_index].id.clone(),
                );
            }

            // Data processing activities
            let processing_scale = if is_sme { "small" } else { "large" };
            entity.set_attribute("processing_scale", processing_scale.to_string());

            // Country (distributed across EU)
            let country = match i % 10 {
                0 => "DE",
                1 => "FR",
                2 => "IT",
                3 => "ES",
                4 => "PL",
                5 => "NL",
                6 => "BE",
                7 => "SE",
                8 => "AT",
                _ => "Other",
            };
            entity.set_attribute("country", country.to_string());

            // Revenue quintile (1-5)
            let quintile = if is_sme { 1 + (i % 3) } else { 4 + (i % 2) };
            entity.set_attribute("revenue_quintile", (quintile as u8).to_string());

            companies.push(entity);
        }

        companies
    }

    /// Calculates the fine for a company based on GDPR rules
    fn calculate_fine(&self, company: &BasicEntity) -> f64 {
        let violation_type = company
            .get_attribute("violation_type")
            .unwrap_or_else(|| "GDPR_Art5".to_string());
        let turnover: f64 = company
            .get_attribute("annual_turnover")
            .and_then(|t| t.parse().ok())
            .unwrap_or(1_000_000.0);

        let violation = self
            .violations
            .iter()
            .find(|v| v.id == violation_type)
            .cloned()
            .unwrap_or_else(GdprViolation::article_5_principles);

        // GDPR Article 83 calculation
        let (max_fixed, turnover_pct): (f64, f64) = match violation.tier {
            GdprFineTier::LowerTier => (10_000_000.0, 0.02),
            GdprFineTier::HigherTier => (20_000_000.0, 0.04),
        };

        let turnover_based: f64 = turnover * turnover_pct;
        let max_possible: f64 = max_fixed.max(turnover_based);

        // Apply proportionality (typically 10-30% of maximum)
        let proportionality_factor: f64 = 0.15;
        (max_possible * proportionality_factor).min(max_possible)
    }

    /// Runs the simulation engine
    fn run_engine_simulation(&self, companies: &[BasicEntity]) -> SimulationMetrics {
        let mut builder = SimEngineBuilder::new().validate(false);

        for company in companies {
            builder = builder.add_entity(Box::new(company.clone()));
        }

        // Add GDPR compliance statute
        let gdpr_statute = self.create_gdpr_statute();
        builder = builder.add_statute(gdpr_statute);

        match builder.build() {
            Ok(engine) => match tokio::runtime::Runtime::new() {
                Ok(rt) => rt.block_on(async { engine.run_simulation().await }),
                Err(_) => SimulationMetrics::new(),
            },
            Err(_) => SimulationMetrics::new(),
        }
    }

    /// Creates GDPR compliance statute for simulation
    fn create_gdpr_statute(&self) -> Statute {
        Statute::new(
            "GDPR_Art83_sim",
            "GDPR Administrative Fines (Simulation)",
            Effect::new(
                EffectType::MonetaryTransfer,
                "Administrative fine for GDPR violation",
            )
            .with_parameter("lower_tier_max", "10000000")
            .with_parameter("higher_tier_max", "20000000")
            .with_parameter("lower_tier_turnover_pct", "0.02")
            .with_parameter("higher_tier_turnover_pct", "0.04"),
        )
        .with_jurisdiction("EU")
    }

    /// Calculates distributional impact across company sizes
    fn calculate_distributional_impact(&self, companies: &[BasicEntity]) -> DistributionalImpact {
        let mut quintile_data: HashMap<u8, Vec<(f64, bool)>> = HashMap::new();

        for company in companies {
            let quintile_opt: Option<u8> = company
                .get_attribute("revenue_quintile")
                .and_then(|s| s.parse().ok());
            let turnover_opt: Option<f64> = company
                .get_attribute("annual_turnover")
                .and_then(|s| s.parse().ok());
            let has_violation = company
                .get_attribute("has_violation")
                .map(|s| s == "true")
                .unwrap_or(false);

            if let (Some(quintile), Some(turnover)) = (quintile_opt, turnover_opt) {
                quintile_data
                    .entry(quintile)
                    .or_default()
                    .push((turnover, has_violation));
            }
        }

        let mut quintile_impacts = Vec::new();
        let mut first_quintile_impact = 0.0;
        let mut fifth_quintile_impact = 0.0;

        for quintile in 1..=5 {
            if let Some(data) = quintile_data.get(&quintile) {
                let avg_turnover: f64 =
                    data.iter().map(|(t, _)| t).sum::<f64>() / data.len() as f64;
                let violations: usize = data.iter().filter(|(_, v)| *v).count();
                let violation_rate = violations as f64 / data.len() as f64;

                // Calculate average fine impact as percentage of turnover
                let avg_fine = avg_turnover * 0.02 * 0.15; // 2% * proportionality
                let impact_pct = if avg_turnover > 0.0 {
                    avg_fine / avg_turnover * 100.0
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
                    avg_tax_burden: avg_fine * violation_rate,
                    avg_compliance_cost: avg_turnover * 0.01, // 1% compliance cost
                    entity_count: data.len(),
                    impact_pct_of_income: impact_pct * violation_rate,
                });
            }
        }

        // GDPR fines are proportional (same percentage regardless of size)
        let progressivity = if (first_quintile_impact - fifth_quintile_impact).abs() < 0.5 {
            Progressivity::Proportional
        } else if first_quintile_impact > fifth_quintile_impact {
            Progressivity::Regressive
        } else {
            Progressivity::Progressive
        };

        DistributionalImpact {
            quintile_impacts,
            gini_change: 0.0, // Proportional fines don't change inequality much
            progressivity,
        }
    }
}

// ============================================================================
// GDPR Compliance Cost Simulation
// ============================================================================

/// GDPR compliance cost configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GdprComplianceConfig {
    /// Number of companies to simulate
    pub company_count: usize,
    /// Percentage that are SMEs
    pub sme_percentage: f64,
    /// Time period for cost calculation (days)
    pub time_period_days: usize,
}

impl Default for GdprComplianceConfig {
    fn default() -> Self {
        Self {
            company_count: 1000,
            sme_percentage: 0.85,
            time_period_days: 365,
        }
    }
}

/// GDPR compliance cost result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GdprComplianceCostResult {
    /// Total one-time costs across all companies
    pub total_one_time_costs: f64,
    /// Total recurring annual costs
    pub total_recurring_costs: f64,
    /// Average cost per company
    pub avg_cost_per_company: f64,
    /// Cost by company size
    pub cost_by_size: HashMap<String, f64>,
    /// Cost by activity type
    pub cost_by_activity: HashMap<String, f64>,
}

/// Creates GDPR compliance preset for simulation
pub fn gdpr_compliance_preset() -> CompliancePreset {
    CompliancePreset::gdpr_compliance()
}

/// Simulates GDPR compliance costs
pub fn simulate_gdpr_compliance_costs(config: &GdprComplianceConfig) -> GdprComplianceCostResult {
    let compliance = gdpr_compliance_preset();
    let base_cost = compliance.total_cost(config.time_period_days);

    let mut total_one_time = 0.0;
    let mut total_recurring = 0.0;
    let mut cost_by_size: HashMap<String, f64> = HashMap::new();
    let mut cost_by_activity: HashMap<String, f64> = HashMap::new();

    for i in 0..config.company_count {
        let is_sme = (i as f64 / config.company_count as f64) < config.sme_percentage;
        let size_key = if is_sme { "SME" } else { "Large" };

        // Scale costs based on company size
        let size_factor = if is_sme { 0.3 } else { 1.0 };

        let company_one_time = base_cost.one_time_costs * size_factor;
        let company_recurring = base_cost.recurring_costs * size_factor;

        total_one_time += company_one_time;
        total_recurring += company_recurring;

        *cost_by_size.entry(size_key.to_string()).or_insert(0.0) +=
            company_one_time + company_recurring;
    }

    // Cost breakdown by activity
    cost_by_activity.insert("privacy_policy".to_string(), total_one_time * 0.1);
    cost_by_activity.insert("dpo_appointment".to_string(), total_recurring * 0.7);
    cost_by_activity.insert("annual_audit".to_string(), total_recurring * 0.2);
    cost_by_activity.insert("training".to_string(), total_recurring * 0.1);

    let avg_cost = (total_one_time + total_recurring) / config.company_count as f64;

    GdprComplianceCostResult {
        total_one_time_costs: total_one_time,
        total_recurring_costs: total_recurring,
        avg_cost_per_company: avg_cost,
        cost_by_size,
        cost_by_activity,
    }
}

// ============================================================================
// Economic Impact Simulation
// ============================================================================

/// Simulates overall GDPR economic impact for the EU
pub fn simulate_gdpr_economic_impact(company_count: usize) -> EconomicImpact {
    let fine_config = GdprFineConfig {
        company_count,
        ..Default::default()
    };
    let fine_sim = GdprFineSimulator::new(fine_config);
    let fine_result = fine_sim.simulate();

    let compliance_config = GdprComplianceConfig {
        company_count,
        ..Default::default()
    };
    let compliance_result = simulate_gdpr_compliance_costs(&compliance_config);

    let mut revenue_by_type = HashMap::new();
    revenue_by_type.insert("gdpr_fines".to_string(), fine_result.total_fines);

    let mut costs_by_type = HashMap::new();
    costs_by_type.insert(
        "compliance_one_time".to_string(),
        compliance_result.total_one_time_costs,
    );
    costs_by_type.insert(
        "compliance_recurring".to_string(),
        compliance_result.total_recurring_costs,
    );

    let total_compliance =
        compliance_result.total_one_time_costs + compliance_result.total_recurring_costs;

    EconomicImpact {
        tax_revenue: fine_result.total_fines,
        compliance_costs: total_compliance,
        administrative_costs: fine_result.total_fines * 0.05,
        net_benefit: fine_result.total_fines - total_compliance * 0.1,
        revenue_by_type,
        costs_by_type,
        distributional_impact: fine_result.distributional_impact,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_sim::ComplianceType;

    #[test]
    fn test_gdpr_fine_simulator_default() {
        let simulator = GdprFineSimulator::default_config();
        let result = simulator.simulate();

        assert_eq!(result.companies_simulated, 10_000);
        assert!(result.companies_with_violations > 0);
        assert!(result.total_fines >= 0.0);
    }

    #[test]
    fn test_gdpr_fine_custom_config() {
        let config = GdprFineConfig {
            company_count: 1000,
            monte_carlo_runs: 10,
            sme_percentage: 0.90,
            violation_rate: 0.10,
        };

        let simulator = GdprFineSimulator::new(config);
        let result = simulator.simulate();

        assert_eq!(result.companies_simulated, 1000);
        assert!(result.companies_with_violations > 0);
    }

    #[test]
    fn test_gdpr_violation_types() {
        let v1 = GdprViolation::article_5_principles();
        assert_eq!(v1.tier, GdprFineTier::HigherTier);

        let v2 = GdprViolation::article_32_security();
        assert_eq!(v2.tier, GdprFineTier::LowerTier);
    }

    #[test]
    fn test_gdpr_compliance_cost_simulation() {
        let config = GdprComplianceConfig::default();
        let result = simulate_gdpr_compliance_costs(&config);

        assert!(result.total_one_time_costs > 0.0);
        assert!(result.total_recurring_costs > 0.0);
        assert!(result.avg_cost_per_company > 0.0);
    }

    #[test]
    fn test_gdpr_economic_impact() {
        let result = simulate_gdpr_economic_impact(500);

        assert!(result.tax_revenue >= 0.0);
        assert!(result.compliance_costs > 0.0);
    }

    #[test]
    fn test_fine_calculation_sme() {
        let config = GdprFineConfig {
            company_count: 100,
            sme_percentage: 1.0,
            ..Default::default()
        };
        let simulator = GdprFineSimulator::new(config);
        let result = simulator.simulate();

        // SME fines should be lower than large company fines
        assert!(result.average_fine < 20_000_000.0);
    }

    #[test]
    fn test_fine_tiers_distribution() {
        let simulator = GdprFineSimulator::default_config();
        let result = simulator.simulate();

        assert!(result.fines_by_tier.contains_key("LowerTier"));
        assert!(result.fines_by_tier.contains_key("HigherTier"));
    }

    #[test]
    fn test_distributional_impact() {
        let config = GdprFineConfig {
            company_count: 500,
            ..Default::default()
        };
        let simulator = GdprFineSimulator::new(config);
        let result = simulator.simulate();

        // GDPR fines should be roughly proportional
        assert!(!result.distributional_impact.quintile_impacts.is_empty());
    }

    #[test]
    fn test_gdpr_compliance_preset() {
        let preset = gdpr_compliance_preset();

        assert_eq!(preset.compliance_type, ComplianceType::DataPrivacy);
        assert!(!preset.required_actions.is_empty());
    }
}
