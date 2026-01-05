//! Healthcare simulation module for epidemiological modeling and capacity analysis.
//!
//! This module provides tools for:
//! - Epidemiological models (SIR, SEIR)
//! - Healthcare capacity simulation
//! - Vaccine distribution optimization
//! - Health policy impact analysis
//! - Social determinants of health modeling

use crate::error::{SimResult, SimulationError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Compartmental model for disease spread (SIR - Susceptible, Infected, Recovered)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIRModel {
    /// Susceptible population
    pub susceptible: f64,
    /// Infected population
    pub infected: f64,
    /// Recovered population
    pub recovered: f64,
    /// Transmission rate (beta)
    pub transmission_rate: f64,
    /// Recovery rate (gamma)
    pub recovery_rate: f64,
    /// Total population
    pub total_population: f64,
}

impl SIRModel {
    /// Create a new SIR model
    pub fn new(
        initial_susceptible: f64,
        initial_infected: f64,
        initial_recovered: f64,
        transmission_rate: f64,
        recovery_rate: f64,
    ) -> SimResult<Self> {
        if initial_susceptible < 0.0 || initial_infected < 0.0 || initial_recovered < 0.0 {
            return Err(SimulationError::InvalidParameter(
                "Population counts cannot be negative".to_string(),
            ));
        }

        let total = initial_susceptible + initial_infected + initial_recovered;
        if total <= 0.0 {
            return Err(SimulationError::InvalidParameter(
                "Total population must be positive".to_string(),
            ));
        }

        Ok(Self {
            susceptible: initial_susceptible,
            infected: initial_infected,
            recovered: initial_recovered,
            transmission_rate,
            recovery_rate,
            total_population: total,
        })
    }

    /// Step the model forward by one time unit using differential equations
    pub fn step(&mut self, dt: f64) {
        let s = self.susceptible;
        let i = self.infected;
        let n = self.total_population;

        // SIR differential equations:
        // dS/dt = -beta * S * I / N
        // dI/dt = beta * S * I / N - gamma * I
        // dR/dt = gamma * I

        let new_infections = self.transmission_rate * s * i / n * dt;
        let new_recoveries = self.recovery_rate * i * dt;

        self.susceptible = (s - new_infections).max(0.0);
        self.infected = (i + new_infections - new_recoveries).max(0.0);
        self.recovered = (self.recovered + new_recoveries).max(0.0);
    }

    /// Calculate the basic reproduction number (R0)
    pub fn basic_reproduction_number(&self) -> f64 {
        self.transmission_rate / self.recovery_rate
    }

    /// Check if the epidemic is ongoing
    pub fn is_active(&self) -> bool {
        self.infected > 0.1
    }

    /// Get the current state as a tuple (S, I, R)
    pub fn get_state(&self) -> (f64, f64, f64) {
        (self.susceptible, self.infected, self.recovered)
    }
}

/// Extended SEIR model (Susceptible, Exposed, Infected, Recovered)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEIRModel {
    /// Susceptible population
    pub susceptible: f64,
    /// Exposed (infected but not yet infectious) population
    pub exposed: f64,
    /// Infected (infectious) population
    pub infected: f64,
    /// Recovered population
    pub recovered: f64,
    /// Transmission rate (beta)
    pub transmission_rate: f64,
    /// Incubation rate (sigma) - rate at which exposed become infectious
    pub incubation_rate: f64,
    /// Recovery rate (gamma)
    pub recovery_rate: f64,
    /// Total population
    pub total_population: f64,
}

impl SEIRModel {
    /// Create a new SEIR model
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        initial_susceptible: f64,
        initial_exposed: f64,
        initial_infected: f64,
        initial_recovered: f64,
        transmission_rate: f64,
        incubation_rate: f64,
        recovery_rate: f64,
    ) -> SimResult<Self> {
        if initial_susceptible < 0.0
            || initial_exposed < 0.0
            || initial_infected < 0.0
            || initial_recovered < 0.0
        {
            return Err(SimulationError::InvalidParameter(
                "Population counts cannot be negative".to_string(),
            ));
        }

        let total = initial_susceptible + initial_exposed + initial_infected + initial_recovered;
        if total <= 0.0 {
            return Err(SimulationError::InvalidParameter(
                "Total population must be positive".to_string(),
            ));
        }

        Ok(Self {
            susceptible: initial_susceptible,
            exposed: initial_exposed,
            infected: initial_infected,
            recovered: initial_recovered,
            transmission_rate,
            incubation_rate,
            recovery_rate,
            total_population: total,
        })
    }

    /// Step the model forward by one time unit
    pub fn step(&mut self, dt: f64) {
        let s = self.susceptible;
        let e = self.exposed;
        let i = self.infected;
        let n = self.total_population;

        // SEIR differential equations:
        // dS/dt = -beta * S * I / N
        // dE/dt = beta * S * I / N - sigma * E
        // dI/dt = sigma * E - gamma * I
        // dR/dt = gamma * I

        let new_exposures = self.transmission_rate * s * i / n * dt;
        let new_infections = self.incubation_rate * e * dt;
        let new_recoveries = self.recovery_rate * i * dt;

        self.susceptible = (s - new_exposures).max(0.0);
        self.exposed = (e + new_exposures - new_infections).max(0.0);
        self.infected = (i + new_infections - new_recoveries).max(0.0);
        self.recovered = (self.recovered + new_recoveries).max(0.0);
    }

    /// Calculate the basic reproduction number (R0)
    pub fn basic_reproduction_number(&self) -> f64 {
        self.transmission_rate / self.recovery_rate
    }

    /// Check if the epidemic is ongoing
    pub fn is_active(&self) -> bool {
        self.infected > 0.1 || self.exposed > 0.1
    }

    /// Get the current state as a tuple (S, E, I, R)
    pub fn get_state(&self) -> (f64, f64, f64, f64) {
        (
            self.susceptible,
            self.exposed,
            self.infected,
            self.recovered,
        )
    }
}

/// Healthcare facility with capacity constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthcareFacility {
    /// Facility ID
    pub id: Uuid,
    /// Facility name
    pub name: String,
    /// Total number of beds
    pub total_beds: usize,
    /// Number of ICU beds
    pub icu_beds: usize,
    /// Number of ventilators
    pub ventilators: usize,
    /// Current occupied beds
    pub occupied_beds: usize,
    /// Current occupied ICU beds
    pub occupied_icu_beds: usize,
    /// Current occupied ventilators
    pub occupied_ventilators: usize,
    /// Staff count
    pub staff_count: usize,
    /// Geographic location (latitude, longitude)
    pub location: Option<(f64, f64)>,
}

impl HealthcareFacility {
    /// Create a new healthcare facility
    pub fn new(name: String, total_beds: usize, icu_beds: usize, ventilators: usize) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            total_beds,
            icu_beds,
            ventilators,
            occupied_beds: 0,
            occupied_icu_beds: 0,
            occupied_ventilators: 0,
            staff_count: 0,
            location: None,
        }
    }

    /// Set the facility location
    pub fn with_location(mut self, latitude: f64, longitude: f64) -> Self {
        self.location = Some((latitude, longitude));
        self
    }

    /// Set the staff count
    pub fn with_staff(mut self, staff_count: usize) -> Self {
        self.staff_count = staff_count;
        self
    }

    /// Calculate available beds
    pub fn available_beds(&self) -> usize {
        self.total_beds.saturating_sub(self.occupied_beds)
    }

    /// Calculate available ICU beds
    pub fn available_icu_beds(&self) -> usize {
        self.icu_beds.saturating_sub(self.occupied_icu_beds)
    }

    /// Calculate available ventilators
    pub fn available_ventilators(&self) -> usize {
        self.ventilators.saturating_sub(self.occupied_ventilators)
    }

    /// Calculate bed utilization rate (0.0 to 1.0)
    pub fn bed_utilization(&self) -> f64 {
        if self.total_beds == 0 {
            0.0
        } else {
            self.occupied_beds as f64 / self.total_beds as f64
        }
    }

    /// Calculate ICU utilization rate (0.0 to 1.0)
    pub fn icu_utilization(&self) -> f64 {
        if self.icu_beds == 0 {
            0.0
        } else {
            self.occupied_icu_beds as f64 / self.icu_beds as f64
        }
    }

    /// Check if the facility is at capacity
    pub fn is_at_capacity(&self) -> bool {
        self.available_beds() == 0
    }

    /// Check if ICU is at capacity
    pub fn is_icu_at_capacity(&self) -> bool {
        self.available_icu_beds() == 0
    }

    /// Admit a patient (returns true if successful)
    pub fn admit_patient(&mut self, requires_icu: bool, requires_ventilator: bool) -> bool {
        if requires_icu {
            if self.available_icu_beds() == 0 {
                return false;
            }
            self.occupied_icu_beds += 1;
            if requires_ventilator {
                if self.available_ventilators() == 0 {
                    self.occupied_icu_beds -= 1;
                    return false;
                }
                self.occupied_ventilators += 1;
            }
        } else {
            if self.available_beds() == 0 {
                return false;
            }
            self.occupied_beds += 1;
        }
        true
    }

    /// Discharge a patient
    pub fn discharge_patient(&mut self, was_icu: bool, had_ventilator: bool) {
        if was_icu {
            self.occupied_icu_beds = self.occupied_icu_beds.saturating_sub(1);
            if had_ventilator {
                self.occupied_ventilators = self.occupied_ventilators.saturating_sub(1);
            }
        } else {
            self.occupied_beds = self.occupied_beds.saturating_sub(1);
        }
    }
}

/// Healthcare system with multiple facilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthcareSystem {
    /// All facilities in the system
    pub facilities: Vec<HealthcareFacility>,
}

impl HealthcareSystem {
    /// Create a new healthcare system
    pub fn new() -> Self {
        Self {
            facilities: Vec::new(),
        }
    }

    /// Add a facility to the system
    pub fn add_facility(&mut self, facility: HealthcareFacility) {
        self.facilities.push(facility);
    }

    /// Get total capacity across all facilities
    pub fn total_capacity(&self) -> usize {
        self.facilities.iter().map(|f| f.total_beds).sum()
    }

    /// Get total ICU capacity
    pub fn total_icu_capacity(&self) -> usize {
        self.facilities.iter().map(|f| f.icu_beds).sum()
    }

    /// Get total available beds
    pub fn total_available_beds(&self) -> usize {
        self.facilities.iter().map(|f| f.available_beds()).sum()
    }

    /// Get total available ICU beds
    pub fn total_available_icu_beds(&self) -> usize {
        self.facilities.iter().map(|f| f.available_icu_beds()).sum()
    }

    /// Calculate system-wide bed utilization
    pub fn system_bed_utilization(&self) -> f64 {
        let total_capacity = self.total_capacity();
        if total_capacity == 0 {
            0.0
        } else {
            let total_occupied: usize = self.facilities.iter().map(|f| f.occupied_beds).sum();
            total_occupied as f64 / total_capacity as f64
        }
    }

    /// Calculate system-wide ICU utilization
    pub fn system_icu_utilization(&self) -> f64 {
        let total_icu_capacity = self.total_icu_capacity();
        if total_icu_capacity == 0 {
            0.0
        } else {
            let total_occupied: usize = self.facilities.iter().map(|f| f.occupied_icu_beds).sum();
            total_occupied as f64 / total_icu_capacity as f64
        }
    }

    /// Check if the system is overwhelmed (>90% utilization)
    pub fn is_overwhelmed(&self) -> bool {
        self.system_bed_utilization() > 0.9 || self.system_icu_utilization() > 0.9
    }

    /// Find facility with most available beds
    pub fn find_best_facility(&self) -> Option<&HealthcareFacility> {
        self.facilities.iter().max_by_key(|f| f.available_beds())
    }
}

impl Default for HealthcareSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Vaccine distribution strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VaccinationStrategy {
    /// Prioritize elderly (65+)
    PrioritizeElderly,
    /// Prioritize healthcare workers
    PrioritizeHealthcareWorkers,
    /// Prioritize high-risk individuals
    PrioritizeHighRisk,
    /// Uniform distribution
    Uniform,
    /// Prioritize based on transmission hotspots
    PrioritizeHotspots,
}

/// Vaccine distribution plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaccineDistribution {
    /// Total available doses
    pub total_doses: usize,
    /// Doses administered
    pub doses_administered: usize,
    /// Distribution strategy
    pub strategy: VaccinationStrategy,
    /// Vaccine efficacy (0.0 to 1.0)
    pub efficacy: f64,
    /// Doses per person required
    pub doses_per_person: usize,
    /// Coverage by age group (age_group -> vaccinated_count)
    pub coverage_by_age: HashMap<String, usize>,
}

impl VaccineDistribution {
    /// Create a new vaccine distribution plan
    pub fn new(total_doses: usize, efficacy: f64, strategy: VaccinationStrategy) -> Self {
        Self {
            total_doses,
            doses_administered: 0,
            strategy,
            efficacy,
            doses_per_person: 2,
            coverage_by_age: HashMap::new(),
        }
    }

    /// Set doses per person
    pub fn with_doses_per_person(mut self, doses: usize) -> Self {
        self.doses_per_person = doses;
        self
    }

    /// Calculate remaining doses
    pub fn remaining_doses(&self) -> usize {
        self.total_doses.saturating_sub(self.doses_administered)
    }

    /// Calculate vaccination coverage rate
    pub fn coverage_rate(&self, total_population: usize) -> f64 {
        if total_population == 0 {
            0.0
        } else {
            let fully_vaccinated = self.doses_administered / self.doses_per_person;
            fully_vaccinated as f64 / total_population as f64
        }
    }

    /// Administer vaccines to a group
    pub fn administer(&mut self, count: usize, age_group: Option<String>) -> usize {
        let available = self.remaining_doses();
        let to_administer = count.min(available);

        self.doses_administered += to_administer;

        if let Some(group) = age_group {
            *self.coverage_by_age.entry(group).or_insert(0) += to_administer;
        }

        to_administer
    }

    /// Check if distribution is complete
    pub fn is_complete(&self) -> bool {
        self.remaining_doses() == 0
    }
}

/// Health policy intervention
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthPolicyIntervention {
    /// Policy name
    pub name: String,
    /// Reduction in transmission rate (0.0 to 1.0)
    pub transmission_reduction: f64,
    /// Economic cost per day
    pub daily_cost: f64,
    /// Compliance rate (0.0 to 1.0)
    pub compliance_rate: f64,
    /// Whether the policy is currently active
    pub is_active: bool,
}

impl HealthPolicyIntervention {
    /// Create a new health policy intervention
    pub fn new(name: String, transmission_reduction: f64, daily_cost: f64) -> Self {
        Self {
            name,
            transmission_reduction,
            daily_cost,
            compliance_rate: 1.0,
            is_active: false,
        }
    }

    /// Create a lockdown policy
    pub fn lockdown() -> Self {
        Self::new("Lockdown".to_string(), 0.7, 1_000_000.0).with_compliance(0.85)
    }

    /// Create a mask mandate policy
    pub fn mask_mandate() -> Self {
        Self::new("Mask Mandate".to_string(), 0.3, 50_000.0).with_compliance(0.75)
    }

    /// Create a social distancing policy
    pub fn social_distancing() -> Self {
        Self::new("Social Distancing".to_string(), 0.4, 100_000.0).with_compliance(0.7)
    }

    /// Set compliance rate
    pub fn with_compliance(mut self, rate: f64) -> Self {
        self.compliance_rate = rate.clamp(0.0, 1.0);
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

    /// Calculate effective transmission reduction
    pub fn effective_reduction(&self) -> f64 {
        if self.is_active {
            self.transmission_reduction * self.compliance_rate
        } else {
            0.0
        }
    }

    /// Calculate cost if active
    pub fn current_cost(&self) -> f64 {
        if self.is_active { self.daily_cost } else { 0.0 }
    }
}

/// Social determinants of health
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialDeterminants {
    /// Income level (0.0 to 1.0, where 1.0 is high income)
    pub income_level: f64,
    /// Education level (0.0 to 1.0, where 1.0 is high education)
    pub education_level: f64,
    /// Access to healthcare (0.0 to 1.0)
    pub healthcare_access: f64,
    /// Housing quality (0.0 to 1.0)
    pub housing_quality: f64,
    /// Food security (0.0 to 1.0)
    pub food_security: f64,
    /// Social support (0.0 to 1.0)
    pub social_support: f64,
}

impl SocialDeterminants {
    /// Create new social determinants
    pub fn new(income_level: f64, education_level: f64, healthcare_access: f64) -> Self {
        Self {
            income_level: income_level.clamp(0.0, 1.0),
            education_level: education_level.clamp(0.0, 1.0),
            healthcare_access: healthcare_access.clamp(0.0, 1.0),
            housing_quality: 0.5,
            food_security: 0.5,
            social_support: 0.5,
        }
    }

    /// Set housing quality
    pub fn with_housing_quality(mut self, quality: f64) -> Self {
        self.housing_quality = quality.clamp(0.0, 1.0);
        self
    }

    /// Set food security
    pub fn with_food_security(mut self, security: f64) -> Self {
        self.food_security = security.clamp(0.0, 1.0);
        self
    }

    /// Set social support
    pub fn with_social_support(mut self, support: f64) -> Self {
        self.social_support = support.clamp(0.0, 1.0);
        self
    }

    /// Calculate overall health risk (0.0 = low risk, 1.0 = high risk)
    pub fn health_risk_score(&self) -> f64 {
        // Higher values in determinants = lower risk
        // So we invert them to get risk score
        let risk = 1.0
            - (self.income_level * 0.2
                + self.education_level * 0.15
                + self.healthcare_access * 0.25
                + self.housing_quality * 0.15
                + self.food_security * 0.15
                + self.social_support * 0.1);
        risk.clamp(0.0, 1.0)
    }

    /// Calculate disease susceptibility modifier (1.0 = baseline, >1.0 = more susceptible)
    pub fn susceptibility_modifier(&self) -> f64 {
        // Base modifier is 1.0, increases with risk
        1.0 + self.health_risk_score() * 0.5
    }

    /// Calculate recovery rate modifier (1.0 = baseline, >1.0 = faster recovery)
    pub fn recovery_modifier(&self) -> f64 {
        // Better determinants = faster recovery
        let determinants_score = self.income_level * 0.2
            + self.education_level * 0.15
            + self.healthcare_access * 0.35
            + self.housing_quality * 0.1
            + self.food_security * 0.1
            + self.social_support * 0.1;
        0.5 + determinants_score
    }
}

impl Default for SocialDeterminants {
    fn default() -> Self {
        Self::new(0.5, 0.5, 0.5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sir_model_creation() {
        let model = SIRModel::new(990.0, 10.0, 0.0, 0.3, 0.1).unwrap();
        assert_eq!(model.susceptible, 990.0);
        assert_eq!(model.infected, 10.0);
        assert_eq!(model.recovered, 0.0);
        assert_eq!(model.total_population, 1000.0);
    }

    #[test]
    fn test_sir_model_step() {
        let mut model = SIRModel::new(990.0, 10.0, 0.0, 0.3, 0.1).unwrap();
        model.step(1.0);

        // After one step, some susceptible become infected, some infected recover
        assert!(model.susceptible < 990.0);
        assert!(model.infected > 0.0);
        assert!(model.recovered > 0.0);
    }

    #[test]
    fn test_sir_basic_reproduction_number() {
        let model = SIRModel::new(990.0, 10.0, 0.0, 0.3, 0.1).unwrap();
        let r0 = model.basic_reproduction_number();
        assert!((r0 - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_seir_model_creation() {
        let model = SEIRModel::new(990.0, 5.0, 5.0, 0.0, 0.3, 0.2, 0.1).unwrap();
        assert_eq!(model.susceptible, 990.0);
        assert_eq!(model.exposed, 5.0);
        assert_eq!(model.infected, 5.0);
        assert_eq!(model.recovered, 0.0);
    }

    #[test]
    fn test_seir_model_step() {
        let mut model = SEIRModel::new(990.0, 5.0, 5.0, 0.0, 0.3, 0.2, 0.1).unwrap();
        model.step(1.0);

        // Model should evolve
        assert!(model.susceptible < 990.0 || model.exposed != 5.0 || model.infected != 5.0);
    }

    #[test]
    fn test_healthcare_facility_creation() {
        let facility = HealthcareFacility::new("City Hospital".to_string(), 100, 20, 10);
        assert_eq!(facility.name, "City Hospital");
        assert_eq!(facility.total_beds, 100);
        assert_eq!(facility.icu_beds, 20);
        assert_eq!(facility.ventilators, 10);
        assert_eq!(facility.available_beds(), 100);
    }

    #[test]
    fn test_healthcare_facility_admission() {
        let mut facility = HealthcareFacility::new("City Hospital".to_string(), 100, 20, 10);

        // Admit regular patient
        assert!(facility.admit_patient(false, false));
        assert_eq!(facility.occupied_beds, 1);
        assert_eq!(facility.available_beds(), 99);

        // Admit ICU patient with ventilator
        assert!(facility.admit_patient(true, true));
        assert_eq!(facility.occupied_icu_beds, 1);
        assert_eq!(facility.occupied_ventilators, 1);
    }

    #[test]
    fn test_healthcare_facility_at_capacity() {
        let mut facility = HealthcareFacility::new("Small Clinic".to_string(), 2, 1, 1);

        assert!(facility.admit_patient(false, false));
        assert!(facility.admit_patient(false, false));
        assert!(!facility.admit_patient(false, false)); // Should fail
        assert!(facility.is_at_capacity());
    }

    #[test]
    fn test_healthcare_facility_discharge() {
        let mut facility = HealthcareFacility::new("City Hospital".to_string(), 100, 20, 10);

        facility.admit_patient(true, true);
        facility.discharge_patient(true, true);

        assert_eq!(facility.occupied_icu_beds, 0);
        assert_eq!(facility.occupied_ventilators, 0);
    }

    #[test]
    fn test_healthcare_system() {
        let mut system = HealthcareSystem::new();

        system.add_facility(HealthcareFacility::new(
            "Hospital A".to_string(),
            100,
            20,
            10,
        ));
        system.add_facility(HealthcareFacility::new(
            "Hospital B".to_string(),
            150,
            30,
            15,
        ));

        assert_eq!(system.total_capacity(), 250);
        assert_eq!(system.total_icu_capacity(), 50);
    }

    #[test]
    fn test_vaccine_distribution() {
        let mut distribution =
            VaccineDistribution::new(10000, 0.95, VaccinationStrategy::PrioritizeElderly);

        assert_eq!(distribution.remaining_doses(), 10000);

        distribution.administer(200, Some("65+".to_string()));
        assert_eq!(distribution.doses_administered, 200);
        assert_eq!(distribution.remaining_doses(), 9800);
    }

    #[test]
    fn test_vaccine_coverage_rate() {
        let mut distribution = VaccineDistribution::new(1000, 0.95, VaccinationStrategy::Uniform);

        distribution.administer(200, None);

        // 200 doses / 2 doses per person = 100 fully vaccinated
        // 100 / 1000 population = 0.1 coverage
        assert_eq!(distribution.coverage_rate(1000), 0.1);
    }

    #[test]
    fn test_health_policy_lockdown() {
        let mut policy = HealthPolicyIntervention::lockdown();
        assert_eq!(policy.name, "Lockdown");
        assert!(!policy.is_active);

        policy.activate();
        assert!(policy.is_active);
        assert!(policy.effective_reduction() > 0.0);
        assert!(policy.current_cost() > 0.0);
    }

    #[test]
    fn test_health_policy_effective_reduction() {
        let mut policy = HealthPolicyIntervention::new("Test Policy".to_string(), 0.5, 1000.0)
            .with_compliance(0.8);

        policy.activate();
        assert_eq!(policy.effective_reduction(), 0.4); // 0.5 * 0.8
    }

    #[test]
    fn test_social_determinants_creation() {
        let determinants = SocialDeterminants::new(0.8, 0.7, 0.9);
        assert_eq!(determinants.income_level, 0.8);
        assert_eq!(determinants.education_level, 0.7);
        assert_eq!(determinants.healthcare_access, 0.9);
    }

    #[test]
    fn test_social_determinants_health_risk() {
        let high_risk = SocialDeterminants::new(0.2, 0.2, 0.2)
            .with_housing_quality(0.2)
            .with_food_security(0.2)
            .with_social_support(0.2);

        let low_risk = SocialDeterminants::new(0.9, 0.9, 0.9)
            .with_housing_quality(0.9)
            .with_food_security(0.9)
            .with_social_support(0.9);

        assert!(high_risk.health_risk_score() > low_risk.health_risk_score());
    }

    #[test]
    fn test_social_determinants_susceptibility() {
        let determinants = SocialDeterminants::new(0.5, 0.5, 0.5);
        let modifier = determinants.susceptibility_modifier();

        // Should be > 1.0 (more susceptible than baseline)
        assert!(modifier >= 1.0);
        assert!(modifier <= 1.5);
    }

    #[test]
    fn test_social_determinants_recovery() {
        let good_determinants = SocialDeterminants::new(0.9, 0.9, 0.9);
        let poor_determinants = SocialDeterminants::new(0.1, 0.1, 0.1);

        // Better determinants = faster recovery (higher modifier)
        assert!(good_determinants.recovery_modifier() > poor_determinants.recovery_modifier());
    }

    #[test]
    fn test_sir_model_invalid_population() {
        let result = SIRModel::new(-100.0, 10.0, 0.0, 0.3, 0.1);
        assert!(result.is_err());
    }

    #[test]
    fn test_healthcare_utilization() {
        let mut facility = HealthcareFacility::new("Test".to_string(), 100, 20, 10);
        facility.admit_patient(false, false);
        facility.admit_patient(false, false);

        assert_eq!(facility.bed_utilization(), 0.02);
    }

    #[test]
    fn test_healthcare_system_overwhelmed() {
        let mut system = HealthcareSystem::new();
        let mut facility = HealthcareFacility::new("Hospital".to_string(), 10, 2, 1);

        // Fill 9 of 10 beds
        for _ in 0..9 {
            facility.admit_patient(false, false);
        }

        system.add_facility(facility);
        assert!(!system.is_overwhelmed()); // 90% exactly, not > 90%

        // Add one more to push over 90%
        if let Some(fac) = system.facilities.get_mut(0) {
            fac.admit_patient(false, false);
        }
        assert!(system.is_overwhelmed());
    }
}
