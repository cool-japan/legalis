//! Environmental simulation module for climate, disasters, and resource modeling.
//!
//! This module provides tools for:
//! - Climate impact on populations
//! - Natural disaster simulations
//! - Resource scarcity modeling
//! - Environmental policy simulation
//! - Carbon footprint tracking

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Climate scenario based on IPCC projections
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClimateScenario {
    /// Business as usual - high emissions (RCP 8.5)
    HighEmissions,
    /// Moderate mitigation (RCP 4.5)
    ModerateMitigation,
    /// Strong mitigation (RCP 2.6)
    StrongMitigation,
    /// Net-zero by 2050
    NetZero,
}

/// Climate impact model for population effects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateImpact {
    /// Climate scenario
    pub scenario: ClimateScenario,
    /// Current year
    pub current_year: u32,
    /// Baseline year for comparison (typically 2020)
    pub baseline_year: u32,
    /// Temperature increase in Celsius above baseline
    pub temperature_increase: f64,
    /// Sea level rise in meters
    pub sea_level_rise: f64,
    /// Extreme weather event frequency multiplier (1.0 = baseline)
    pub extreme_weather_multiplier: f64,
    /// Agricultural productivity impact (-1.0 to 1.0, negative is decrease)
    pub agricultural_impact: f64,
    /// Water availability impact (-1.0 to 1.0, negative is decrease)
    pub water_availability: f64,
}

impl ClimateImpact {
    /// Create a new climate impact model
    pub fn new(scenario: ClimateScenario, current_year: u32, baseline_year: u32) -> Self {
        Self {
            scenario,
            current_year,
            baseline_year,
            temperature_increase: 0.0,
            sea_level_rise: 0.0,
            extreme_weather_multiplier: 1.0,
            agricultural_impact: 0.0,
            water_availability: 0.0,
        }
    }

    /// Project climate impacts based on scenario and year
    pub fn project(&mut self) {
        let years_elapsed = (self.current_year.saturating_sub(self.baseline_year)) as f64;

        // Project temperature increase based on scenario
        self.temperature_increase = match self.scenario {
            ClimateScenario::HighEmissions => years_elapsed * 0.05, // ~5°C by 2100
            ClimateScenario::ModerateMitigation => years_elapsed * 0.03, // ~3°C by 2100
            ClimateScenario::StrongMitigation => years_elapsed * 0.015, // ~1.5°C by 2100
            ClimateScenario::NetZero => years_elapsed * 0.01,       // ~1°C by 2100
        };

        // Project sea level rise (roughly 15cm per degree of warming)
        self.sea_level_rise = self.temperature_increase * 0.15;

        // Extreme weather increases exponentially with temperature
        self.extreme_weather_multiplier = 1.0 + (self.temperature_increase * 0.2).exp() - 1.0;

        // Agricultural impact (negative = worse)
        self.agricultural_impact = -self.temperature_increase * 0.1;

        // Water availability decreases with warming
        self.water_availability = -self.temperature_increase * 0.08;
    }

    /// Calculate mortality impact rate (deaths per 100k population per year)
    pub fn mortality_impact(&self) -> f64 {
        // Heat-related deaths + disaster deaths
        let heat_deaths = self.temperature_increase * 5.0;
        let disaster_deaths = (self.extreme_weather_multiplier - 1.0) * 10.0;
        heat_deaths + disaster_deaths
    }

    /// Calculate migration pressure (0.0 to 1.0, higher = more migration)
    pub fn migration_pressure(&self) -> f64 {
        let temp_pressure = self.temperature_increase * 0.1;
        let sea_level_pressure = self.sea_level_rise * 2.0;
        let resource_pressure = -self.water_availability * 0.5;

        (temp_pressure + sea_level_pressure + resource_pressure).clamp(0.0, 1.0)
    }

    /// Calculate economic impact (percentage GDP loss per year)
    pub fn economic_impact(&self) -> f64 {
        let ag_loss = -self.agricultural_impact * 0.05;
        let disaster_loss = (self.extreme_weather_multiplier - 1.0) * 0.02;
        let adaptation_cost = self.temperature_increase * 0.01;

        ag_loss + disaster_loss + adaptation_cost
    }
}

/// Type of natural disaster
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisasterType {
    /// Hurricane/typhoon/cyclone
    Hurricane,
    /// Earthquake
    Earthquake,
    /// Flood
    Flood,
    /// Wildfire
    Wildfire,
    /// Drought
    Drought,
    /// Tornado
    Tornado,
    /// Tsunami
    Tsunami,
    /// Volcanic eruption
    Volcanic,
}

/// Severity level of a disaster
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DisasterSeverity {
    /// Minor impact
    Minor,
    /// Moderate impact
    Moderate,
    /// Major impact
    Major,
    /// Catastrophic impact
    Catastrophic,
}

/// Natural disaster event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NaturalDisaster {
    /// Disaster ID
    pub id: Uuid,
    /// Type of disaster
    pub disaster_type: DisasterType,
    /// Severity level
    pub severity: DisasterSeverity,
    /// Geographic location (latitude, longitude)
    pub location: (f64, f64),
    /// Radius of impact in kilometers
    pub impact_radius: f64,
    /// Duration in days
    pub duration_days: u32,
    /// Estimated casualties
    pub casualties: usize,
    /// Estimated displaced persons
    pub displaced: usize,
    /// Economic damage in USD
    pub economic_damage: f64,
    /// Infrastructure damage (0.0 to 1.0)
    pub infrastructure_damage: f64,
}

impl NaturalDisaster {
    /// Create a new natural disaster
    pub fn new(
        disaster_type: DisasterType,
        severity: DisasterSeverity,
        location: (f64, f64),
    ) -> Self {
        let (
            impact_radius,
            duration_days,
            casualties,
            displaced,
            economic_damage,
            infrastructure_damage,
        ) = Self::calculate_impacts(disaster_type, severity);

        Self {
            id: Uuid::new_v4(),
            disaster_type,
            severity,
            location,
            impact_radius,
            duration_days,
            casualties,
            displaced,
            economic_damage,
            infrastructure_damage,
        }
    }

    /// Calculate typical impacts based on disaster type and severity
    #[allow(clippy::type_complexity)]
    fn calculate_impacts(
        disaster_type: DisasterType,
        severity: DisasterSeverity,
    ) -> (f64, u32, usize, usize, f64, f64) {
        let severity_multiplier: f64 = match severity {
            DisasterSeverity::Minor => 1.0,
            DisasterSeverity::Moderate => 3.0,
            DisasterSeverity::Major => 10.0,
            DisasterSeverity::Catastrophic => 30.0,
        };

        let (base_radius, base_duration, base_casualties, base_displaced, base_damage, base_infra) =
            match disaster_type {
                DisasterType::Hurricane => (50.0, 3, 100, 10000, 1e9, 0.3),
                DisasterType::Earthquake => (30.0, 1, 500, 50000, 5e9, 0.5),
                DisasterType::Flood => (20.0, 7, 50, 5000, 5e8, 0.2),
                DisasterType::Wildfire => (10.0, 14, 10, 1000, 1e8, 0.15),
                DisasterType::Drought => (100.0, 180, 1000, 100000, 1e10, 0.1),
                DisasterType::Tornado => (5.0, 1, 50, 500, 1e8, 0.25),
                DisasterType::Tsunami => (40.0, 1, 1000, 20000, 1e10, 0.6),
                DisasterType::Volcanic => (15.0, 30, 200, 10000, 2e9, 0.4),
            };

        (
            base_radius * severity_multiplier.sqrt(),
            base_duration,
            (base_casualties as f64 * severity_multiplier) as usize,
            (base_displaced as f64 * severity_multiplier) as usize,
            base_damage * severity_multiplier,
            (base_infra * severity_multiplier).clamp(0.0, 1.0),
        )
    }

    /// Check if a location is within the disaster's impact zone
    pub fn affects_location(&self, lat: f64, lon: f64) -> bool {
        let distance = Self::haversine_distance(self.location.0, self.location.1, lat, lon);
        distance <= self.impact_radius
    }

    /// Calculate Haversine distance in kilometers between two points
    fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
        let r = 6371.0; // Earth radius in km
        let dlat = (lat2 - lat1).to_radians();
        let dlon = (lon2 - lon1).to_radians();

        let a = (dlat / 2.0).sin().powi(2)
            + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        r * c
    }

    /// Calculate recovery time in days based on severity
    pub fn recovery_time_days(&self) -> u32 {
        match self.severity {
            DisasterSeverity::Minor => 30,
            DisasterSeverity::Moderate => 180,
            DisasterSeverity::Major => 365,
            DisasterSeverity::Catastrophic => 1825, // 5 years
        }
    }
}

/// Resource type for scarcity modeling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceType {
    /// Freshwater
    Water,
    /// Food/agricultural products
    Food,
    /// Energy (electricity, fuel)
    Energy,
    /// Critical minerals and materials
    Minerals,
    /// Arable land
    Land,
}

/// Resource availability and scarcity model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceScarcity {
    /// Resource type
    pub resource_type: ResourceType,
    /// Total available supply (in appropriate units)
    pub total_supply: f64,
    /// Current demand
    pub current_demand: f64,
    /// Supply growth rate (per year)
    pub supply_growth_rate: f64,
    /// Demand growth rate (per year)
    pub demand_growth_rate: f64,
    /// Scarcity threshold (demand/supply ratio for crisis)
    pub scarcity_threshold: f64,
}

impl ResourceScarcity {
    /// Create a new resource scarcity model
    pub fn new(resource_type: ResourceType, total_supply: f64, current_demand: f64) -> Self {
        Self {
            resource_type,
            total_supply,
            current_demand,
            supply_growth_rate: 0.02, // 2% default
            demand_growth_rate: 0.03, // 3% default
            scarcity_threshold: 0.9,  // Crisis at 90% utilization
        }
    }

    /// Set supply growth rate
    pub fn with_supply_growth(mut self, rate: f64) -> Self {
        self.supply_growth_rate = rate;
        self
    }

    /// Set demand growth rate
    pub fn with_demand_growth(mut self, rate: f64) -> Self {
        self.demand_growth_rate = rate;
        self
    }

    /// Calculate supply/demand ratio (< 1.0 means shortage)
    pub fn supply_demand_ratio(&self) -> f64 {
        if self.current_demand <= 0.0 {
            f64::INFINITY
        } else {
            self.total_supply / self.current_demand
        }
    }

    /// Check if resource is in scarcity
    pub fn is_scarce(&self) -> bool {
        self.supply_demand_ratio() < 1.0 / self.scarcity_threshold
    }

    /// Calculate scarcity index (0.0 = abundant, 1.0 = severe scarcity)
    pub fn scarcity_index(&self) -> f64 {
        let ratio = self.supply_demand_ratio();
        if ratio >= 1.0 {
            0.0
        } else {
            (1.0 - ratio).clamp(0.0, 1.0)
        }
    }

    /// Project supply and demand forward by a number of years
    pub fn project(&mut self, years: f64) {
        self.total_supply *= (1.0 + self.supply_growth_rate).powf(years);
        self.current_demand *= (1.0 + self.demand_growth_rate).powf(years);
    }

    /// Calculate price multiplier due to scarcity (1.0 = normal price)
    pub fn price_multiplier(&self) -> f64 {
        let scarcity = self.scarcity_index();
        1.0 + scarcity * 3.0 // Prices can go up to 4x in severe scarcity
    }

    /// Calculate years until scarcity threshold is reached
    pub fn years_to_scarcity(&self) -> Option<f64> {
        if self.supply_growth_rate >= self.demand_growth_rate {
            return None; // Never reaches scarcity
        }

        let ratio = self.supply_demand_ratio();
        if ratio < 1.0 / self.scarcity_threshold {
            return Some(0.0); // Already scarce
        }

        let net_growth = self.demand_growth_rate - self.supply_growth_rate;
        let target_ratio = 1.0 / self.scarcity_threshold;

        Some((ratio / target_ratio).ln() / net_growth)
    }
}

/// Environmental policy intervention
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalPolicy {
    /// Policy name
    pub name: String,
    /// Carbon emission reduction target (0.0 to 1.0)
    pub emission_reduction_target: f64,
    /// Annual implementation cost (USD)
    pub annual_cost: f64,
    /// Renewable energy target (0.0 to 1.0)
    pub renewable_energy_target: f64,
    /// Years to full implementation
    pub implementation_years: u32,
    /// Current progress (0.0 to 1.0)
    pub current_progress: f64,
    /// Whether policy is active
    pub is_active: bool,
}

impl EnvironmentalPolicy {
    /// Create a new environmental policy
    pub fn new(name: String, emission_reduction_target: f64, annual_cost: f64) -> Self {
        Self {
            name,
            emission_reduction_target: emission_reduction_target.clamp(0.0, 1.0),
            annual_cost,
            renewable_energy_target: 0.5,
            implementation_years: 10,
            current_progress: 0.0,
            is_active: false,
        }
    }

    /// Paris Agreement-style policy (reduce emissions 50% by 2030)
    pub fn paris_agreement() -> Self {
        Self::new("Paris Agreement Compliance".to_string(), 0.5, 5e11)
            .with_renewable_target(0.7)
            .with_implementation_years(10)
    }

    /// Aggressive net-zero policy
    pub fn net_zero_2050() -> Self {
        Self::new("Net Zero by 2050".to_string(), 1.0, 1e12)
            .with_renewable_target(0.95)
            .with_implementation_years(30)
    }

    /// Green New Deal-style policy
    pub fn green_new_deal() -> Self {
        Self::new("Green New Deal".to_string(), 0.8, 1.5e12)
            .with_renewable_target(1.0)
            .with_implementation_years(15)
    }

    /// Set renewable energy target
    pub fn with_renewable_target(mut self, target: f64) -> Self {
        self.renewable_energy_target = target.clamp(0.0, 1.0);
        self
    }

    /// Set implementation timeframe
    pub fn with_implementation_years(mut self, years: u32) -> Self {
        self.implementation_years = years;
        self
    }

    /// Activate the policy
    pub fn activate(&mut self) {
        self.is_active = true;
    }

    /// Deactivate the policy
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    /// Update progress (call once per year)
    pub fn update_progress(&mut self) {
        if self.is_active && self.current_progress < 1.0 {
            let annual_progress = 1.0 / self.implementation_years as f64;
            self.current_progress = (self.current_progress + annual_progress).min(1.0);
        }
    }

    /// Calculate current emission reduction achieved
    pub fn current_emission_reduction(&self) -> f64 {
        if self.is_active {
            self.emission_reduction_target * self.current_progress
        } else {
            0.0
        }
    }

    /// Calculate cost to date
    pub fn total_cost(&self, years_active: u32) -> f64 {
        self.annual_cost * years_active as f64
    }
}

/// Carbon footprint tracker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarbonFootprint {
    /// Entity ID being tracked
    pub entity_id: Uuid,
    /// Annual emissions in tonnes CO2 equivalent
    pub annual_emissions_tonnes: f64,
    /// Emissions by category
    pub emissions_by_category: HashMap<String, f64>,
    /// Carbon offsets in tonnes CO2
    pub carbon_offsets: f64,
}

impl CarbonFootprint {
    /// Create a new carbon footprint tracker
    pub fn new(entity_id: Uuid, annual_emissions_tonnes: f64) -> Self {
        Self {
            entity_id,
            annual_emissions_tonnes,
            emissions_by_category: HashMap::new(),
            carbon_offsets: 0.0,
        }
    }

    /// Add emissions from a specific category
    pub fn add_category(&mut self, category: String, emissions: f64) {
        *self.emissions_by_category.entry(category).or_insert(0.0) += emissions;
        self.annual_emissions_tonnes += emissions;
    }

    /// Add carbon offsets
    pub fn add_offsets(&mut self, offsets: f64) {
        self.carbon_offsets += offsets;
    }

    /// Calculate net emissions (after offsets)
    pub fn net_emissions(&self) -> f64 {
        (self.annual_emissions_tonnes - self.carbon_offsets).max(0.0)
    }

    /// Check if carbon neutral
    pub fn is_carbon_neutral(&self) -> bool {
        self.net_emissions() < 0.1
    }

    /// Calculate emissions per capita (given population)
    pub fn per_capita_emissions(&self, population: usize) -> f64 {
        if population == 0 {
            0.0
        } else {
            self.annual_emissions_tonnes / population as f64
        }
    }

    /// Estimate carbon tax liability (USD)
    pub fn carbon_tax_liability(&self, tax_per_tonne: f64) -> f64 {
        self.net_emissions() * tax_per_tonne
    }

    /// Create a footprint for a typical US individual
    pub fn typical_us_individual() -> Self {
        let mut footprint = Self::new(Uuid::new_v4(), 0.0);
        footprint.add_category("Transportation".to_string(), 5.0);
        footprint.add_category("Home Energy".to_string(), 6.0);
        footprint.add_category("Food".to_string(), 2.5);
        footprint.add_category("Goods & Services".to_string(), 3.0);
        footprint
    }

    /// Create a footprint for a typical EU individual
    pub fn typical_eu_individual() -> Self {
        let mut footprint = Self::new(Uuid::new_v4(), 0.0);
        footprint.add_category("Transportation".to_string(), 3.0);
        footprint.add_category("Home Energy".to_string(), 4.0);
        footprint.add_category("Food".to_string(), 2.0);
        footprint.add_category("Goods & Services".to_string(), 2.0);
        footprint
    }
}

/// Aggregate carbon tracking for populations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationCarbonTracker {
    /// Total population size
    pub population_size: usize,
    /// Total emissions in tonnes CO2e
    pub total_emissions: f64,
    /// Total offsets
    pub total_offsets: f64,
    /// Emissions by sector
    pub sector_emissions: HashMap<String, f64>,
}

impl PopulationCarbonTracker {
    /// Create a new population carbon tracker
    pub fn new(population_size: usize) -> Self {
        Self {
            population_size,
            total_emissions: 0.0,
            total_offsets: 0.0,
            sector_emissions: HashMap::new(),
        }
    }

    /// Add individual footprint to population total
    pub fn add_footprint(&mut self, footprint: &CarbonFootprint) {
        self.total_emissions += footprint.annual_emissions_tonnes;
        self.total_offsets += footprint.carbon_offsets;

        for (category, emissions) in &footprint.emissions_by_category {
            *self.sector_emissions.entry(category.clone()).or_insert(0.0) += emissions;
        }
    }

    /// Calculate per capita emissions
    pub fn per_capita_emissions(&self) -> f64 {
        if self.population_size == 0 {
            0.0
        } else {
            self.total_emissions / self.population_size as f64
        }
    }

    /// Calculate net emissions
    pub fn net_emissions(&self) -> f64 {
        (self.total_emissions - self.total_offsets).max(0.0)
    }

    /// Check if population is carbon neutral
    pub fn is_carbon_neutral(&self) -> bool {
        self.net_emissions() < (self.population_size as f64 * 0.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_climate_impact_creation() {
        let climate = ClimateImpact::new(ClimateScenario::ModerateMitigation, 2050, 2020);
        assert_eq!(climate.scenario, ClimateScenario::ModerateMitigation);
        assert_eq!(climate.current_year, 2050);
        assert_eq!(climate.baseline_year, 2020);
    }

    #[test]
    fn test_climate_impact_projection() {
        let mut climate = ClimateImpact::new(ClimateScenario::HighEmissions, 2050, 2020);
        climate.project();

        // After 30 years at 0.05°C/year, should be around 1.5°C
        assert!(climate.temperature_increase > 1.0);
        assert!(climate.sea_level_rise > 0.0);
        assert!(climate.extreme_weather_multiplier > 1.0);
    }

    #[test]
    fn test_climate_mortality_impact() {
        let mut climate = ClimateImpact::new(ClimateScenario::HighEmissions, 2050, 2020);
        climate.project();

        let mortality = climate.mortality_impact();
        assert!(mortality > 0.0);
    }

    #[test]
    fn test_climate_scenarios() {
        let mut high = ClimateImpact::new(ClimateScenario::HighEmissions, 2050, 2020);
        let mut low = ClimateImpact::new(ClimateScenario::NetZero, 2050, 2020);

        high.project();
        low.project();

        assert!(high.temperature_increase > low.temperature_increase);
        assert!(high.migration_pressure() > low.migration_pressure());
    }

    #[test]
    fn test_natural_disaster_creation() {
        let disaster = NaturalDisaster::new(
            DisasterType::Hurricane,
            DisasterSeverity::Major,
            (25.0, -80.0),
        );

        assert_eq!(disaster.disaster_type, DisasterType::Hurricane);
        assert_eq!(disaster.severity, DisasterSeverity::Major);
        assert!(disaster.casualties > 0);
        assert!(disaster.economic_damage > 0.0);
    }

    #[test]
    fn test_disaster_severity_scaling() {
        let minor = NaturalDisaster::new(DisasterType::Flood, DisasterSeverity::Minor, (0.0, 0.0));
        let catastrophic = NaturalDisaster::new(
            DisasterType::Flood,
            DisasterSeverity::Catastrophic,
            (0.0, 0.0),
        );

        assert!(catastrophic.casualties > minor.casualties);
        assert!(catastrophic.economic_damage > minor.economic_damage);
    }

    #[test]
    fn test_disaster_affects_location() {
        let disaster = NaturalDisaster::new(
            DisasterType::Earthquake,
            DisasterSeverity::Major,
            (35.0, 139.0), // Tokyo area
        );

        // Within impact radius
        assert!(disaster.affects_location(35.1, 139.1));

        // Far away (New York)
        assert!(!disaster.affects_location(40.7, -74.0));
    }

    #[test]
    fn test_disaster_recovery_time() {
        let minor =
            NaturalDisaster::new(DisasterType::Tornado, DisasterSeverity::Minor, (0.0, 0.0));
        let catastrophic = NaturalDisaster::new(
            DisasterType::Earthquake,
            DisasterSeverity::Catastrophic,
            (0.0, 0.0),
        );

        assert!(catastrophic.recovery_time_days() > minor.recovery_time_days());
    }

    #[test]
    fn test_resource_scarcity_creation() {
        let water = ResourceScarcity::new(ResourceType::Water, 1000.0, 800.0);
        assert_eq!(water.resource_type, ResourceType::Water);
        assert_eq!(water.total_supply, 1000.0);
        assert_eq!(water.current_demand, 800.0);
    }

    #[test]
    fn test_resource_supply_demand_ratio() {
        let resource = ResourceScarcity::new(ResourceType::Food, 1000.0, 800.0);
        assert_eq!(resource.supply_demand_ratio(), 1.25);
    }

    #[test]
    fn test_resource_scarcity_detection() {
        let abundant = ResourceScarcity::new(ResourceType::Water, 1000.0, 500.0);
        let scarce = ResourceScarcity::new(ResourceType::Water, 1000.0, 1100.0);

        assert!(!abundant.is_scarce());
        assert!(scarce.is_scarce());
    }

    #[test]
    fn test_resource_projection() {
        let mut resource = ResourceScarcity::new(ResourceType::Energy, 1000.0, 800.0)
            .with_supply_growth(0.01)
            .with_demand_growth(0.05);

        resource.project(10.0);

        // Demand growing faster than supply
        assert!(resource.supply_demand_ratio() < 1.25);
    }

    #[test]
    fn test_resource_price_multiplier() {
        let abundant = ResourceScarcity::new(ResourceType::Food, 1000.0, 500.0);
        let scarce = ResourceScarcity::new(ResourceType::Food, 500.0, 1000.0);

        assert_eq!(abundant.price_multiplier(), 1.0);
        assert!(scarce.price_multiplier() > 1.0);
    }

    #[test]
    fn test_environmental_policy_creation() {
        let policy = EnvironmentalPolicy::new("Test Policy".to_string(), 0.5, 1e10);
        assert_eq!(policy.name, "Test Policy");
        assert_eq!(policy.emission_reduction_target, 0.5);
        assert!(!policy.is_active);
    }

    #[test]
    fn test_environmental_policy_presets() {
        let paris = EnvironmentalPolicy::paris_agreement();
        let net_zero = EnvironmentalPolicy::net_zero_2050();

        assert!(net_zero.emission_reduction_target > paris.emission_reduction_target);
    }

    #[test]
    fn test_environmental_policy_progress() {
        let mut policy =
            EnvironmentalPolicy::new("Test".to_string(), 0.5, 1e10).with_implementation_years(10);

        policy.activate();
        assert_eq!(policy.current_progress, 0.0);

        policy.update_progress();
        assert!(policy.current_progress > 0.0);
        assert!(policy.current_progress <= 0.1);
    }

    #[test]
    fn test_environmental_policy_emission_reduction() {
        let mut policy =
            EnvironmentalPolicy::new("Test".to_string(), 0.5, 1e10).with_implementation_years(5);

        policy.activate();
        policy.update_progress();

        let reduction = policy.current_emission_reduction();
        assert!(reduction > 0.0);
        assert!(reduction <= 0.5);
    }

    #[test]
    fn test_carbon_footprint_creation() {
        let footprint = CarbonFootprint::new(Uuid::new_v4(), 10.0);
        assert_eq!(footprint.annual_emissions_tonnes, 10.0);
        assert_eq!(footprint.carbon_offsets, 0.0);
    }

    #[test]
    fn test_carbon_footprint_categories() {
        let mut footprint = CarbonFootprint::new(Uuid::new_v4(), 0.0);
        footprint.add_category("Transportation".to_string(), 5.0);
        footprint.add_category("Home Energy".to_string(), 3.0);

        assert_eq!(footprint.annual_emissions_tonnes, 8.0);
        assert_eq!(footprint.emissions_by_category.len(), 2);
    }

    #[test]
    fn test_carbon_footprint_offsets() {
        let mut footprint = CarbonFootprint::new(Uuid::new_v4(), 10.0);
        footprint.add_offsets(3.0);

        assert_eq!(footprint.net_emissions(), 7.0);
    }

    #[test]
    fn test_carbon_neutral() {
        let mut neutral = CarbonFootprint::new(Uuid::new_v4(), 10.0);
        neutral.add_offsets(10.0);

        let not_neutral = CarbonFootprint::new(Uuid::new_v4(), 10.0);

        assert!(neutral.is_carbon_neutral());
        assert!(!not_neutral.is_carbon_neutral());
    }

    #[test]
    fn test_carbon_tax_liability() {
        let footprint = CarbonFootprint::new(Uuid::new_v4(), 10.0);
        let tax = footprint.carbon_tax_liability(50.0); // $50/tonne

        assert_eq!(tax, 500.0);
    }

    #[test]
    fn test_typical_footprints() {
        let us = CarbonFootprint::typical_us_individual();
        let eu = CarbonFootprint::typical_eu_individual();

        assert!(us.annual_emissions_tonnes > eu.annual_emissions_tonnes);
    }

    #[test]
    fn test_population_carbon_tracker() {
        let mut tracker = PopulationCarbonTracker::new(1000);
        let footprint = CarbonFootprint::new(Uuid::new_v4(), 10.0);

        tracker.add_footprint(&footprint);

        assert_eq!(tracker.total_emissions, 10.0);
        assert_eq!(tracker.per_capita_emissions(), 0.01);
    }

    #[test]
    fn test_population_carbon_neutral() {
        let mut tracker = PopulationCarbonTracker::new(1000);

        // Add emissions
        for _ in 0..1000 {
            let mut footprint = CarbonFootprint::new(Uuid::new_v4(), 10.0);
            footprint.add_offsets(10.0);
            tracker.add_footprint(&footprint);
        }

        assert!(tracker.is_carbon_neutral());
    }
}
