//! Urban Simulation Module
//!
//! This module provides comprehensive urban simulation capabilities including:
//! - Traffic and transportation modeling
//! - Housing market dynamics
//! - Urban sprawl simulation
//! - Infrastructure impact analysis
//! - Smart city policy testing

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// ============================================================================
// Traffic and Transportation Modeling
// ============================================================================

/// Mode of transportation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransportMode {
    /// Personal automobile
    Car,
    /// Public bus service
    Bus,
    /// Rail/subway system
    Rail,
    /// Bicycle
    Bicycle,
    /// Walking
    Walking,
    /// Ride-sharing service
    Rideshare,
}

/// Traffic network node (intersection, station, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficNode {
    /// Unique identifier
    pub id: Uuid,
    /// Node name
    pub name: String,
    /// Geographic coordinates (latitude, longitude)
    pub location: (f64, f64),
    /// Current traffic volume (vehicles per hour)
    pub traffic_volume: f64,
    /// Capacity (vehicles per hour)
    pub capacity: f64,
    /// Traffic signal cycle time (seconds)
    pub signal_cycle_time: Option<f64>,
}

impl TrafficNode {
    /// Create a new traffic node
    pub fn new(name: String, location: (f64, f64), capacity: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            location,
            traffic_volume: 0.0,
            capacity,
            signal_cycle_time: None,
        }
    }

    /// Calculate congestion level (0.0 = free flow, 1.0 = at capacity, >1.0 = over capacity)
    pub fn congestion_level(&self) -> f64 {
        self.traffic_volume / self.capacity
    }

    /// Check if the node is congested (>80% capacity)
    pub fn is_congested(&self) -> bool {
        self.congestion_level() > 0.8
    }

    /// Add traffic volume
    pub fn add_traffic(&mut self, volume: f64) {
        self.traffic_volume += volume;
    }
}

/// Traffic network edge (road segment, transit line)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficEdge {
    /// Source node ID
    pub from: Uuid,
    /// Destination node ID
    pub to: Uuid,
    /// Road segment length (kilometers)
    pub length: f64,
    /// Speed limit (km/h)
    pub speed_limit: f64,
    /// Current average speed (km/h)
    pub current_speed: f64,
    /// Number of lanes
    pub lanes: u32,
    /// Supported transport modes
    pub modes: Vec<TransportMode>,
}

impl TrafficEdge {
    /// Create a new traffic edge
    pub fn new(
        from: Uuid,
        to: Uuid,
        length: f64,
        speed_limit: f64,
        lanes: u32,
        modes: Vec<TransportMode>,
    ) -> Self {
        Self {
            from,
            to,
            length,
            speed_limit,
            current_speed: speed_limit,
            lanes,
            modes,
        }
    }

    /// Calculate travel time in minutes
    pub fn travel_time(&self) -> f64 {
        (self.length / self.current_speed) * 60.0
    }

    /// Update speed based on congestion
    pub fn update_speed(&mut self, congestion_factor: f64) {
        self.current_speed = self.speed_limit * (1.0 - congestion_factor.min(1.0) * 0.7);
    }
}

/// Transportation network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportationNetwork {
    /// Network nodes
    pub nodes: HashMap<Uuid, TrafficNode>,
    /// Network edges
    pub edges: Vec<TrafficEdge>,
    /// Mode share percentages (sum to 100)
    pub mode_share: HashMap<TransportMode, f64>,
}

impl TransportationNetwork {
    /// Create a new transportation network
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
            mode_share: HashMap::new(),
        }
    }

    /// Add a node to the network
    pub fn add_node(&mut self, node: TrafficNode) -> Uuid {
        let id = node.id;
        self.nodes.insert(id, node);
        id
    }

    /// Add an edge to the network
    pub fn add_edge(&mut self, edge: TrafficEdge) {
        self.edges.push(edge);
    }

    /// Calculate average congestion level across all nodes
    pub fn average_congestion(&self) -> f64 {
        if self.nodes.is_empty() {
            return 0.0;
        }
        let total: f64 = self.nodes.values().map(|n| n.congestion_level()).sum();
        total / self.nodes.len() as f64
    }

    /// Get number of congested nodes
    pub fn congested_node_count(&self) -> usize {
        self.nodes.values().filter(|n| n.is_congested()).count()
    }

    /// Calculate average travel time across all edges
    pub fn average_travel_time(&self) -> f64 {
        if self.edges.is_empty() {
            return 0.0;
        }
        let total: f64 = self.edges.iter().map(|e| e.travel_time()).sum();
        total / self.edges.len() as f64
    }

    /// Update mode share
    pub fn set_mode_share(&mut self, mode: TransportMode, share: f64) {
        self.mode_share.insert(mode, share);
    }

    /// Get mode share for a specific mode
    pub fn get_mode_share(&self, mode: TransportMode) -> f64 {
        *self.mode_share.get(&mode).unwrap_or(&0.0)
    }
}

impl Default for TransportationNetwork {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Housing Market Dynamics
// ============================================================================

/// Type of housing unit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HousingType {
    /// Single-family detached home
    SingleFamily,
    /// Townhouse or row house
    Townhouse,
    /// Condominium
    Condo,
    /// Apartment
    Apartment,
    /// Mixed-use development
    MixedUse,
}

/// Housing unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HousingUnit {
    /// Unique identifier
    pub id: Uuid,
    /// Housing type
    pub housing_type: HousingType,
    /// Location (latitude, longitude)
    pub location: (f64, f64),
    /// Size in square meters
    pub size: f64,
    /// Number of bedrooms
    pub bedrooms: u32,
    /// Current price (sale price or rental price per month)
    pub price: f64,
    /// Is this a rental unit?
    pub is_rental: bool,
    /// Is currently occupied?
    pub occupied: bool,
    /// Year built
    pub year_built: u32,
}

impl HousingUnit {
    /// Create a new housing unit
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        housing_type: HousingType,
        location: (f64, f64),
        size: f64,
        bedrooms: u32,
        price: f64,
        is_rental: bool,
        year_built: u32,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            housing_type,
            location,
            size,
            bedrooms,
            price,
            is_rental,
            occupied: false,
            year_built,
        }
    }

    /// Calculate price per square meter
    pub fn price_per_sqm(&self) -> f64 {
        if self.size > 0.0 {
            self.price / self.size
        } else {
            0.0
        }
    }

    /// Calculate age in years
    pub fn age(&self, current_year: u32) -> u32 {
        current_year.saturating_sub(self.year_built)
    }
}

/// Urban housing market with detailed unit tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrbanHousingMarket {
    /// All housing units
    pub units: HashMap<Uuid, HousingUnit>,
    /// Vacancy rate by housing type
    pub vacancy_rates: HashMap<HousingType, f64>,
    /// Average price by housing type
    pub avg_prices: HashMap<HousingType, f64>,
    /// Year-over-year price growth rate
    pub price_growth_rate: f64,
    /// Current year for age calculations
    pub current_year: u32,
}

impl UrbanHousingMarket {
    /// Create a new urban housing market
    pub fn new(current_year: u32) -> Self {
        Self {
            units: HashMap::new(),
            vacancy_rates: HashMap::new(),
            avg_prices: HashMap::new(),
            price_growth_rate: 0.0,
            current_year,
        }
    }

    /// Add a housing unit
    pub fn add_unit(&mut self, unit: HousingUnit) -> Uuid {
        let id = unit.id;
        self.units.insert(id, unit);
        id
    }

    /// Calculate overall vacancy rate
    pub fn overall_vacancy_rate(&self) -> f64 {
        if self.units.is_empty() {
            return 0.0;
        }
        let vacant = self.units.values().filter(|u| !u.occupied).count();
        vacant as f64 / self.units.len() as f64
    }

    /// Calculate vacancy rate for a specific housing type
    pub fn vacancy_rate_by_type(&self, housing_type: HousingType) -> f64 {
        let type_units: Vec<_> = self
            .units
            .values()
            .filter(|u| u.housing_type == housing_type)
            .collect();
        if type_units.is_empty() {
            return 0.0;
        }
        let vacant = type_units.iter().filter(|u| !u.occupied).count();
        vacant as f64 / type_units.len() as f64
    }

    /// Calculate average price for a specific housing type
    pub fn average_price(&self, housing_type: HousingType) -> f64 {
        let prices: Vec<_> = self
            .units
            .values()
            .filter(|u| u.housing_type == housing_type)
            .map(|u| u.price)
            .collect();
        if prices.is_empty() {
            return 0.0;
        }
        prices.iter().sum::<f64>() / prices.len() as f64
    }

    /// Update all prices based on growth rate
    pub fn apply_price_growth(&mut self) {
        for unit in self.units.values_mut() {
            unit.price *= 1.0 + self.price_growth_rate;
        }
    }

    /// Calculate affordability index (median price / median income)
    pub fn affordability_index(&self, median_income: f64) -> f64 {
        let prices: Vec<_> = self.units.values().map(|u| u.price).collect();
        if prices.is_empty() {
            return 0.0;
        }
        let mut sorted_prices = prices;
        sorted_prices.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median_price = sorted_prices[sorted_prices.len() / 2];
        median_price / median_income
    }
}

// ============================================================================
// Urban Sprawl Simulation
// ============================================================================

/// Land use type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LandUse {
    /// Residential area
    Residential,
    /// Commercial area
    Commercial,
    /// Industrial area
    Industrial,
    /// Parks and recreation
    Park,
    /// Agricultural land
    Agricultural,
    /// Undeveloped land
    Undeveloped,
    /// Mixed-use area
    MixedUse,
}

/// Land parcel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandParcel {
    /// Unique identifier
    pub id: Uuid,
    /// Location (latitude, longitude)
    pub location: (f64, f64),
    /// Area in hectares
    pub area: f64,
    /// Current land use
    pub land_use: LandUse,
    /// Zoning designation
    pub zoning: LandUse,
    /// Distance to city center (km)
    pub distance_to_center: f64,
    /// Population density (people per hectare)
    pub population_density: f64,
}

impl LandParcel {
    /// Create a new land parcel
    pub fn new(
        location: (f64, f64),
        area: f64,
        land_use: LandUse,
        zoning: LandUse,
        distance_to_center: f64,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            location,
            area,
            land_use,
            zoning,
            distance_to_center,
            population_density: 0.0,
        }
    }

    /// Check if parcel conforms to zoning
    pub fn conforms_to_zoning(&self) -> bool {
        self.land_use == self.zoning || matches!(self.zoning, LandUse::MixedUse)
    }

    /// Calculate development potential score (0.0 to 1.0)
    pub fn development_potential(&self) -> f64 {
        let mut score: f64 = 0.0;

        // Undeveloped land has higher potential
        if matches!(self.land_use, LandUse::Undeveloped | LandUse::Agricultural) {
            score += 0.4;
        }

        // Closer to city center has higher potential
        if self.distance_to_center < 10.0 {
            score += 0.3;
        } else if self.distance_to_center < 20.0 {
            score += 0.15;
        }

        // Conforms to zoning
        if self.conforms_to_zoning() {
            score += 0.3;
        }

        score.min(1.0)
    }
}

/// Urban sprawl model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrbanSprawlModel {
    /// All land parcels
    pub parcels: HashMap<Uuid, LandParcel>,
    /// Urban boundary (radius from center in km)
    pub urban_boundary: f64,
    /// Sprawl rate (hectares per year)
    pub sprawl_rate: f64,
    /// City center location
    pub city_center: (f64, f64),
}

impl UrbanSprawlModel {
    /// Create a new urban sprawl model
    pub fn new(city_center: (f64, f64), urban_boundary: f64, sprawl_rate: f64) -> Self {
        Self {
            parcels: HashMap::new(),
            urban_boundary,
            sprawl_rate,
            city_center,
        }
    }

    /// Add a land parcel
    pub fn add_parcel(&mut self, parcel: LandParcel) -> Uuid {
        let id = parcel.id;
        self.parcels.insert(id, parcel);
        id
    }

    /// Calculate total developed area
    pub fn total_developed_area(&self) -> f64 {
        self.parcels
            .values()
            .filter(|p| !matches!(p.land_use, LandUse::Undeveloped | LandUse::Agricultural))
            .map(|p| p.area)
            .sum()
    }

    /// Calculate urban area (within urban boundary)
    pub fn urban_area(&self) -> f64 {
        self.parcels
            .values()
            .filter(|p| p.distance_to_center <= self.urban_boundary)
            .map(|p| p.area)
            .sum()
    }

    /// Calculate sprawl index (developed area beyond boundary / total developed area)
    pub fn sprawl_index(&self) -> f64 {
        let total_developed = self.total_developed_area();
        if total_developed == 0.0 {
            return 0.0;
        }

        let beyond_boundary: f64 = self
            .parcels
            .values()
            .filter(|p| {
                p.distance_to_center > self.urban_boundary
                    && !matches!(p.land_use, LandUse::Undeveloped | LandUse::Agricultural)
            })
            .map(|p| p.area)
            .sum();

        beyond_boundary / total_developed
    }

    /// Get parcels with high development potential
    pub fn high_potential_parcels(&self) -> Vec<&LandParcel> {
        let mut parcels: Vec<_> = self
            .parcels
            .values()
            .filter(|p| p.development_potential() > 0.6)
            .collect();
        parcels.sort_by(|a, b| {
            b.development_potential()
                .partial_cmp(&a.development_potential())
                .unwrap()
        });
        parcels
    }

    /// Calculate land use distribution
    pub fn land_use_distribution(&self) -> HashMap<LandUse, f64> {
        let mut distribution = HashMap::new();
        for parcel in self.parcels.values() {
            *distribution.entry(parcel.land_use).or_insert(0.0) += parcel.area;
        }
        distribution
    }
}

// ============================================================================
// Infrastructure Impact Analysis
// ============================================================================

/// Type of infrastructure
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InfrastructureType {
    /// Roads and highways
    Roads,
    /// Water supply system
    Water,
    /// Sewer system
    Sewer,
    /// Electrical grid
    Electricity,
    /// Public transit
    Transit,
    /// Parks and recreation
    Parks,
    /// Schools
    Schools,
    /// Healthcare facilities
    Healthcare,
}

/// Infrastructure project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrastructureProject {
    /// Unique identifier
    pub id: Uuid,
    /// Project name
    pub name: String,
    /// Infrastructure type
    pub infrastructure_type: InfrastructureType,
    /// Capital cost (millions)
    pub capital_cost: f64,
    /// Annual operating cost (millions)
    pub operating_cost: f64,
    /// Expected lifespan (years)
    pub lifespan: u32,
    /// Service capacity (units depend on type)
    pub capacity: f64,
    /// Current utilization rate (0.0 to 1.0)
    pub utilization: f64,
    /// Location (latitude, longitude)
    pub location: (f64, f64),
}

impl InfrastructureProject {
    /// Create a new infrastructure project
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        infrastructure_type: InfrastructureType,
        capital_cost: f64,
        operating_cost: f64,
        lifespan: u32,
        capacity: f64,
        location: (f64, f64),
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            infrastructure_type,
            capital_cost,
            operating_cost,
            lifespan,
            capacity,
            utilization: 0.0,
            location,
        }
    }

    /// Calculate total lifecycle cost
    pub fn lifecycle_cost(&self) -> f64 {
        self.capital_cost + (self.operating_cost * self.lifespan as f64)
    }

    /// Calculate cost per unit of capacity
    pub fn cost_per_capacity_unit(&self) -> f64 {
        if self.capacity > 0.0 {
            self.lifecycle_cost() / self.capacity
        } else {
            0.0
        }
    }

    /// Check if infrastructure is over-utilized (>90%)
    pub fn is_over_utilized(&self) -> bool {
        self.utilization > 0.9
    }

    /// Check if infrastructure is under-utilized (<50%)
    pub fn is_under_utilized(&self) -> bool {
        self.utilization < 0.5
    }

    /// Calculate annual cost per capita (given population served)
    pub fn annual_cost_per_capita(&self, population: f64) -> f64 {
        if population > 0.0 {
            (self.capital_cost / self.lifespan as f64 + self.operating_cost) / population
        } else {
            0.0
        }
    }
}

/// Infrastructure impact analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrastructureImpact {
    /// All infrastructure projects
    pub projects: HashMap<Uuid, InfrastructureProject>,
    /// Total population served
    pub population: f64,
    /// Infrastructure spending as % of budget
    pub budget_allocation: f64,
}

impl InfrastructureImpact {
    /// Create a new infrastructure impact analysis
    pub fn new(population: f64, budget_allocation: f64) -> Self {
        Self {
            projects: HashMap::new(),
            population,
            budget_allocation,
        }
    }

    /// Add an infrastructure project
    pub fn add_project(&mut self, project: InfrastructureProject) -> Uuid {
        let id = project.id;
        self.projects.insert(id, project);
        id
    }

    /// Calculate total capital investment
    pub fn total_capital_cost(&self) -> f64 {
        self.projects.values().map(|p| p.capital_cost).sum()
    }

    /// Calculate total annual operating cost
    pub fn total_operating_cost(&self) -> f64 {
        self.projects.values().map(|p| p.operating_cost).sum()
    }

    /// Calculate infrastructure per capita cost
    pub fn per_capita_cost(&self) -> f64 {
        if self.population > 0.0 {
            (self.total_capital_cost() + self.total_operating_cost()) / self.population
        } else {
            0.0
        }
    }

    /// Get over-utilized infrastructure projects
    pub fn over_utilized_projects(&self) -> Vec<&InfrastructureProject> {
        self.projects
            .values()
            .filter(|p| p.is_over_utilized())
            .collect()
    }

    /// Get under-utilized infrastructure projects
    pub fn under_utilized_projects(&self) -> Vec<&InfrastructureProject> {
        self.projects
            .values()
            .filter(|p| p.is_under_utilized())
            .collect()
    }

    /// Calculate infrastructure type distribution (by capital cost)
    pub fn type_distribution(&self) -> HashMap<InfrastructureType, f64> {
        let mut distribution = HashMap::new();
        for project in self.projects.values() {
            *distribution
                .entry(project.infrastructure_type)
                .or_insert(0.0) += project.capital_cost;
        }
        distribution
    }

    /// Calculate average utilization rate
    pub fn average_utilization(&self) -> f64 {
        if self.projects.is_empty() {
            return 0.0;
        }
        let total: f64 = self.projects.values().map(|p| p.utilization).sum();
        total / self.projects.len() as f64
    }
}

// ============================================================================
// Smart City Policy Testing
// ============================================================================

/// Smart city policy type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SmartCityPolicyType {
    /// Traffic optimization and intelligent transportation
    SmartTraffic,
    /// Energy efficiency and smart grid
    SmartEnergy,
    /// Waste management optimization
    SmartWaste,
    /// Water management and conservation
    SmartWater,
    /// Public safety and surveillance
    PublicSafety,
    /// Digital governance and citizen services
    DigitalGovernance,
    /// Environmental monitoring
    EnvironmentalMonitoring,
}

/// Smart city policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartCityPolicy {
    /// Unique identifier
    pub id: Uuid,
    /// Policy name
    pub name: String,
    /// Policy type
    pub policy_type: SmartCityPolicyType,
    /// Implementation cost (millions)
    pub implementation_cost: f64,
    /// Expected annual savings (millions)
    pub annual_savings: f64,
    /// Expected efficiency improvement (0.0 to 1.0)
    pub efficiency_gain: f64,
    /// Expected quality of life improvement (0.0 to 1.0)
    pub quality_of_life_gain: f64,
    /// Implementation time (years)
    pub implementation_time: f64,
    /// Technology readiness level (1-9, where 9 is fully deployed)
    pub technology_readiness: u32,
}

impl SmartCityPolicy {
    /// Create a new smart city policy
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        policy_type: SmartCityPolicyType,
        implementation_cost: f64,
        annual_savings: f64,
        efficiency_gain: f64,
        quality_of_life_gain: f64,
        implementation_time: f64,
        technology_readiness: u32,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            policy_type,
            implementation_cost,
            annual_savings,
            efficiency_gain,
            quality_of_life_gain,
            implementation_time,
            technology_readiness,
        }
    }

    /// Calculate return on investment (ROI) over a period
    pub fn roi(&self, years: f64) -> f64 {
        let total_savings = self.annual_savings * years;
        if self.implementation_cost > 0.0 {
            (total_savings - self.implementation_cost) / self.implementation_cost
        } else {
            0.0
        }
    }

    /// Calculate payback period (years)
    pub fn payback_period(&self) -> f64 {
        if self.annual_savings > 0.0 {
            self.implementation_cost / self.annual_savings
        } else {
            f64::INFINITY
        }
    }

    /// Calculate benefit-cost ratio
    pub fn benefit_cost_ratio(&self, years: f64) -> f64 {
        let total_savings = self.annual_savings * years;
        if self.implementation_cost > 0.0 {
            total_savings / self.implementation_cost
        } else {
            0.0
        }
    }

    /// Calculate overall impact score (weighted combination of metrics)
    pub fn impact_score(&self) -> f64 {
        // Weight: 40% efficiency, 30% quality of life, 30% financial
        let financial_score = if self.annual_savings > 0.0 {
            (self.annual_savings / self.implementation_cost).min(1.0)
        } else {
            0.0
        };

        0.4 * self.efficiency_gain + 0.3 * self.quality_of_life_gain + 0.3 * financial_score
    }

    /// Check if policy is mature enough for deployment (TRL >= 7)
    pub fn is_deployment_ready(&self) -> bool {
        self.technology_readiness >= 7
    }
}

/// Smart city policy portfolio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartCityPortfolio {
    /// All policies
    pub policies: HashMap<Uuid, SmartCityPolicy>,
    /// Total budget available (millions)
    pub total_budget: f64,
    /// Implementation timeline (years)
    pub timeline: f64,
}

impl SmartCityPortfolio {
    /// Create a new smart city policy portfolio
    pub fn new(total_budget: f64, timeline: f64) -> Self {
        Self {
            policies: HashMap::new(),
            total_budget,
            timeline,
        }
    }

    /// Add a policy to the portfolio
    pub fn add_policy(&mut self, policy: SmartCityPolicy) -> Uuid {
        let id = policy.id;
        self.policies.insert(id, policy);
        id
    }

    /// Calculate total implementation cost
    pub fn total_cost(&self) -> f64 {
        self.policies.values().map(|p| p.implementation_cost).sum()
    }

    /// Calculate total expected annual savings
    pub fn total_annual_savings(&self) -> f64 {
        self.policies.values().map(|p| p.annual_savings).sum()
    }

    /// Check if portfolio is within budget
    pub fn is_within_budget(&self) -> bool {
        self.total_cost() <= self.total_budget
    }

    /// Calculate portfolio ROI
    pub fn portfolio_roi(&self) -> f64 {
        let total_savings = self.total_annual_savings() * self.timeline;
        let total_cost = self.total_cost();
        if total_cost > 0.0 {
            (total_savings - total_cost) / total_cost
        } else {
            0.0
        }
    }

    /// Get policies ranked by impact score
    pub fn ranked_by_impact(&self) -> Vec<&SmartCityPolicy> {
        let mut policies: Vec<_> = self.policies.values().collect();
        policies.sort_by(|a, b| b.impact_score().partial_cmp(&a.impact_score()).unwrap());
        policies
    }

    /// Get deployment-ready policies
    pub fn deployment_ready_policies(&self) -> Vec<&SmartCityPolicy> {
        self.policies
            .values()
            .filter(|p| p.is_deployment_ready())
            .collect()
    }

    /// Calculate average efficiency gain
    pub fn average_efficiency_gain(&self) -> f64 {
        if self.policies.is_empty() {
            return 0.0;
        }
        let total: f64 = self.policies.values().map(|p| p.efficiency_gain).sum();
        total / self.policies.len() as f64
    }

    /// Calculate average quality of life gain
    pub fn average_quality_of_life_gain(&self) -> f64 {
        if self.policies.is_empty() {
            return 0.0;
        }
        let total: f64 = self.policies.values().map(|p| p.quality_of_life_gain).sum();
        total / self.policies.len() as f64
    }

    /// Get policies by type
    pub fn policies_by_type(&self, policy_type: SmartCityPolicyType) -> Vec<&SmartCityPolicy> {
        self.policies
            .values()
            .filter(|p| p.policy_type == policy_type)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Traffic and Transportation Tests
    #[test]
    fn test_traffic_node_creation() {
        let node = TrafficNode::new("Main St & 1st Ave".to_string(), (40.7, -74.0), 1000.0);
        assert_eq!(node.name, "Main St & 1st Ave");
        assert_eq!(node.capacity, 1000.0);
        assert_eq!(node.traffic_volume, 0.0);
    }

    #[test]
    fn test_traffic_congestion() {
        let mut node = TrafficNode::new("Intersection".to_string(), (40.7, -74.0), 1000.0);
        node.add_traffic(850.0);
        assert_eq!(node.congestion_level(), 0.85);
        assert!(node.is_congested());
    }

    #[test]
    fn test_traffic_edge_travel_time() {
        let from = Uuid::new_v4();
        let to = Uuid::new_v4();
        let edge = TrafficEdge::new(from, to, 10.0, 50.0, 2, vec![TransportMode::Car]);
        assert_eq!(edge.travel_time(), 12.0); // 10 km at 50 km/h = 12 minutes
    }

    #[test]
    fn test_traffic_speed_update() {
        let from = Uuid::new_v4();
        let to = Uuid::new_v4();
        let mut edge = TrafficEdge::new(from, to, 10.0, 50.0, 2, vec![TransportMode::Car]);
        edge.update_speed(0.5); // 50% congestion
        assert_eq!(edge.current_speed, 32.5); // 50 * (1 - 0.5 * 0.7)
    }

    #[test]
    fn test_transportation_network() {
        let mut network = TransportationNetwork::new();
        let node1 = TrafficNode::new("Node1".to_string(), (40.7, -74.0), 1000.0);
        let node2 = TrafficNode::new("Node2".to_string(), (40.8, -74.1), 1200.0);
        let id1 = network.add_node(node1);
        let id2 = network.add_node(node2);

        let edge = TrafficEdge::new(id1, id2, 5.0, 60.0, 2, vec![TransportMode::Car]);
        network.add_edge(edge);

        assert_eq!(network.nodes.len(), 2);
        assert_eq!(network.edges.len(), 1);
    }

    #[test]
    fn test_mode_share() {
        let mut network = TransportationNetwork::new();
        network.set_mode_share(TransportMode::Car, 60.0);
        network.set_mode_share(TransportMode::Bus, 20.0);
        network.set_mode_share(TransportMode::Bicycle, 20.0);

        assert_eq!(network.get_mode_share(TransportMode::Car), 60.0);
        assert_eq!(network.get_mode_share(TransportMode::Bus), 20.0);
    }

    // Housing Market Tests
    #[test]
    fn test_housing_unit_creation() {
        let unit = HousingUnit::new(
            HousingType::SingleFamily,
            (40.7, -74.0),
            150.0,
            3,
            500000.0,
            false,
            2010,
        );
        assert_eq!(unit.bedrooms, 3);
        assert_eq!(unit.price, 500000.0);
        assert!(!unit.is_rental);
    }

    #[test]
    fn test_price_per_sqm() {
        let unit = HousingUnit::new(
            HousingType::Condo,
            (40.7, -74.0),
            100.0,
            2,
            300000.0,
            false,
            2015,
        );
        assert_eq!(unit.price_per_sqm(), 3000.0);
    }

    #[test]
    fn test_housing_age() {
        let unit = HousingUnit::new(
            HousingType::Apartment,
            (40.7, -74.0),
            80.0,
            2,
            200000.0,
            true,
            2000,
        );
        assert_eq!(unit.age(2024), 24);
    }

    #[test]
    fn test_housing_market_vacancy() {
        let mut market = UrbanHousingMarket::new(2024);
        let mut unit1 = HousingUnit::new(
            HousingType::Apartment,
            (40.7, -74.0),
            80.0,
            2,
            200000.0,
            true,
            2010,
        );
        unit1.occupied = true;

        let unit2 = HousingUnit::new(
            HousingType::Apartment,
            (40.8, -74.1),
            75.0,
            1,
            180000.0,
            true,
            2012,
        );

        market.add_unit(unit1);
        market.add_unit(unit2);

        assert_eq!(market.overall_vacancy_rate(), 0.5);
    }

    #[test]
    fn test_housing_price_growth() {
        let mut market = UrbanHousingMarket::new(2024);
        let unit = HousingUnit::new(
            HousingType::SingleFamily,
            (40.7, -74.0),
            150.0,
            3,
            500000.0,
            false,
            2010,
        );
        market.add_unit(unit);
        market.price_growth_rate = 0.05; // 5% growth

        market.apply_price_growth();

        let updated_unit = market.units.values().next().unwrap();
        assert_eq!(updated_unit.price, 525000.0);
    }

    // Urban Sprawl Tests
    #[test]
    fn test_land_parcel_creation() {
        let parcel = LandParcel::new(
            (40.7, -74.0),
            10.0,
            LandUse::Residential,
            LandUse::Residential,
            5.0,
        );
        assert_eq!(parcel.area, 10.0);
        assert_eq!(parcel.land_use, LandUse::Residential);
        assert!(parcel.conforms_to_zoning());
    }

    #[test]
    fn test_development_potential() {
        let parcel = LandParcel::new(
            (40.7, -74.0),
            20.0,
            LandUse::Undeveloped,
            LandUse::Residential,
            8.0,
        );
        let potential = parcel.development_potential();
        assert!(potential > 0.6); // Should be high for undeveloped land near center
    }

    #[test]
    fn test_urban_sprawl_model() {
        let mut model = UrbanSprawlModel::new((40.7, -74.0), 15.0, 100.0);
        let parcel1 = LandParcel::new(
            (40.7, -74.0),
            10.0,
            LandUse::Residential,
            LandUse::Residential,
            5.0,
        );
        let parcel2 = LandParcel::new(
            (40.8, -74.1),
            15.0,
            LandUse::Commercial,
            LandUse::Commercial,
            20.0,
        );

        model.add_parcel(parcel1);
        model.add_parcel(parcel2);

        assert_eq!(model.total_developed_area(), 25.0);
    }

    #[test]
    fn test_sprawl_index() {
        let mut model = UrbanSprawlModel::new((40.7, -74.0), 10.0, 50.0);
        let parcel1 = LandParcel::new(
            (40.7, -74.0),
            10.0,
            LandUse::Residential,
            LandUse::Residential,
            5.0,
        );
        let parcel2 = LandParcel::new(
            (40.8, -74.1),
            10.0,
            LandUse::Residential,
            LandUse::Residential,
            15.0,
        );

        model.add_parcel(parcel1);
        model.add_parcel(parcel2);

        assert_eq!(model.sprawl_index(), 0.5); // 50% beyond boundary
    }

    #[test]
    fn test_land_use_distribution() {
        let mut model = UrbanSprawlModel::new((40.7, -74.0), 15.0, 100.0);
        let parcel1 = LandParcel::new(
            (40.7, -74.0),
            10.0,
            LandUse::Residential,
            LandUse::Residential,
            5.0,
        );
        let parcel2 = LandParcel::new(
            (40.8, -74.1),
            20.0,
            LandUse::Commercial,
            LandUse::Commercial,
            8.0,
        );

        model.add_parcel(parcel1);
        model.add_parcel(parcel2);

        let distribution = model.land_use_distribution();
        assert_eq!(*distribution.get(&LandUse::Residential).unwrap(), 10.0);
        assert_eq!(*distribution.get(&LandUse::Commercial).unwrap(), 20.0);
    }

    // Infrastructure Tests
    #[test]
    fn test_infrastructure_project_creation() {
        let project = InfrastructureProject::new(
            "Water Treatment Plant".to_string(),
            InfrastructureType::Water,
            50.0,
            5.0,
            30,
            100000.0,
            (40.7, -74.0),
        );
        assert_eq!(project.capital_cost, 50.0);
        assert_eq!(project.operating_cost, 5.0);
        assert_eq!(project.lifespan, 30);
    }

    #[test]
    fn test_lifecycle_cost() {
        let project = InfrastructureProject::new(
            "Bridge".to_string(),
            InfrastructureType::Roads,
            100.0,
            2.0,
            50,
            50000.0,
            (40.7, -74.0),
        );
        assert_eq!(project.lifecycle_cost(), 200.0); // 100 + (2 * 50)
    }

    #[test]
    fn test_infrastructure_utilization() {
        let mut project = InfrastructureProject::new(
            "School".to_string(),
            InfrastructureType::Schools,
            20.0,
            3.0,
            40,
            1000.0,
            (40.7, -74.0),
        );
        project.utilization = 0.95;
        assert!(project.is_over_utilized());
        assert!(!project.is_under_utilized());
    }

    #[test]
    fn test_infrastructure_impact() {
        let mut impact = InfrastructureImpact::new(100000.0, 0.15);
        let project1 = InfrastructureProject::new(
            "Water Plant".to_string(),
            InfrastructureType::Water,
            50.0,
            5.0,
            30,
            100000.0,
            (40.7, -74.0),
        );
        let project2 = InfrastructureProject::new(
            "Transit Line".to_string(),
            InfrastructureType::Transit,
            200.0,
            15.0,
            40,
            50000.0,
            (40.8, -74.1),
        );

        impact.add_project(project1);
        impact.add_project(project2);

        assert_eq!(impact.total_capital_cost(), 250.0);
        assert_eq!(impact.total_operating_cost(), 20.0);
    }

    #[test]
    fn test_infrastructure_type_distribution() {
        let mut impact = InfrastructureImpact::new(100000.0, 0.15);
        let project1 = InfrastructureProject::new(
            "Water".to_string(),
            InfrastructureType::Water,
            50.0,
            5.0,
            30,
            100000.0,
            (40.7, -74.0),
        );
        let project2 = InfrastructureProject::new(
            "Transit".to_string(),
            InfrastructureType::Transit,
            100.0,
            10.0,
            40,
            50000.0,
            (40.8, -74.1),
        );

        impact.add_project(project1);
        impact.add_project(project2);

        let distribution = impact.type_distribution();
        assert_eq!(*distribution.get(&InfrastructureType::Water).unwrap(), 50.0);
        assert_eq!(
            *distribution.get(&InfrastructureType::Transit).unwrap(),
            100.0
        );
    }

    // Smart City Policy Tests
    #[test]
    fn test_smart_city_policy_creation() {
        let policy = SmartCityPolicy::new(
            "Intelligent Traffic System".to_string(),
            SmartCityPolicyType::SmartTraffic,
            10.0,
            2.0,
            0.3,
            0.25,
            2.0,
            8,
        );
        assert_eq!(policy.implementation_cost, 10.0);
        assert_eq!(policy.annual_savings, 2.0);
        assert!(policy.is_deployment_ready());
    }

    #[test]
    fn test_policy_roi() {
        let policy = SmartCityPolicy::new(
            "Smart Grid".to_string(),
            SmartCityPolicyType::SmartEnergy,
            20.0,
            5.0,
            0.4,
            0.3,
            3.0,
            7,
        );
        let roi = policy.roi(10.0);
        assert_eq!(roi, 1.5); // (5*10 - 20) / 20 = 1.5
    }

    #[test]
    fn test_payback_period() {
        let policy = SmartCityPolicy::new(
            "Smart Waste".to_string(),
            SmartCityPolicyType::SmartWaste,
            15.0,
            3.0,
            0.35,
            0.2,
            2.5,
            8,
        );
        assert_eq!(policy.payback_period(), 5.0); // 15 / 3
    }

    #[test]
    fn test_impact_score() {
        let policy = SmartCityPolicy::new(
            "Environmental Monitor".to_string(),
            SmartCityPolicyType::EnvironmentalMonitoring,
            5.0,
            1.0,
            0.5,
            0.6,
            1.0,
            9,
        );
        let score = policy.impact_score();
        assert!(score > 0.4 && score <= 1.0);
    }

    #[test]
    fn test_smart_city_portfolio() {
        let mut portfolio = SmartCityPortfolio::new(100.0, 10.0);
        let policy1 = SmartCityPolicy::new(
            "Smart Traffic".to_string(),
            SmartCityPolicyType::SmartTraffic,
            30.0,
            5.0,
            0.4,
            0.35,
            2.0,
            8,
        );
        let policy2 = SmartCityPolicy::new(
            "Smart Energy".to_string(),
            SmartCityPolicyType::SmartEnergy,
            40.0,
            8.0,
            0.5,
            0.4,
            3.0,
            7,
        );

        portfolio.add_policy(policy1);
        portfolio.add_policy(policy2);

        assert_eq!(portfolio.total_cost(), 70.0);
        assert!(portfolio.is_within_budget());
        assert_eq!(portfolio.total_annual_savings(), 13.0);
    }

    #[test]
    fn test_portfolio_ranking() {
        let mut portfolio = SmartCityPortfolio::new(100.0, 10.0);
        let policy1 = SmartCityPolicy::new(
            "Policy1".to_string(),
            SmartCityPolicyType::SmartTraffic,
            20.0,
            3.0,
            0.3,
            0.2,
            2.0,
            8,
        );
        let policy2 = SmartCityPolicy::new(
            "Policy2".to_string(),
            SmartCityPolicyType::SmartEnergy,
            30.0,
            6.0,
            0.6,
            0.5,
            3.0,
            9,
        );

        portfolio.add_policy(policy1);
        portfolio.add_policy(policy2);

        let ranked = portfolio.ranked_by_impact();
        assert_eq!(ranked[0].name, "Policy2"); // Higher impact score
    }

    #[test]
    fn test_deployment_ready_filter() {
        let mut portfolio = SmartCityPortfolio::new(100.0, 10.0);
        let policy1 = SmartCityPolicy::new(
            "Ready".to_string(),
            SmartCityPolicyType::SmartTraffic,
            20.0,
            3.0,
            0.3,
            0.2,
            2.0,
            8,
        );
        let policy2 = SmartCityPolicy::new(
            "Not Ready".to_string(),
            SmartCityPolicyType::SmartEnergy,
            30.0,
            6.0,
            0.6,
            0.5,
            3.0,
            5,
        );

        portfolio.add_policy(policy1);
        portfolio.add_policy(policy2);

        let ready = portfolio.deployment_ready_policies();
        assert_eq!(ready.len(), 1);
        assert_eq!(ready[0].name, "Ready");
    }

    #[test]
    fn test_portfolio_gains() {
        let mut portfolio = SmartCityPortfolio::new(100.0, 10.0);
        let policy1 = SmartCityPolicy::new(
            "P1".to_string(),
            SmartCityPolicyType::SmartTraffic,
            20.0,
            3.0,
            0.4,
            0.3,
            2.0,
            8,
        );
        let policy2 = SmartCityPolicy::new(
            "P2".to_string(),
            SmartCityPolicyType::SmartEnergy,
            30.0,
            6.0,
            0.6,
            0.5,
            3.0,
            9,
        );

        portfolio.add_policy(policy1);
        portfolio.add_policy(policy2);

        assert_eq!(portfolio.average_efficiency_gain(), 0.5);
        assert_eq!(portfolio.average_quality_of_life_gain(), 0.4);
    }
}
