//! Demographic Modeling Module
//!
//! This module provides advanced demographic modeling capabilities including:
//! - Census data integration and representation
//! - Mortality and fertility rate modeling
//! - Migration pattern simulation
//! - Household formation models
//! - Income mobility simulation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// ============================================================================
// Census Data Integration
// ============================================================================

/// Census data representation for a region
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CensusData {
    /// Region identifier
    pub region_id: String,
    /// Census year
    pub year: u32,
    /// Total population
    pub total_population: u64,
    /// Population by age group (age_group -> count)
    pub age_distribution: HashMap<AgeGroup, u64>,
    /// Population by gender (gender -> count)
    pub gender_distribution: HashMap<Gender, u64>,
    /// Population by education level
    pub education_distribution: HashMap<EducationLevel, u64>,
    /// Population by employment status
    pub employment_distribution: HashMap<EmploymentStatus, u64>,
    /// Median household income
    pub median_household_income: u64,
    /// Average household size
    pub average_household_size: f64,
}

impl CensusData {
    /// Create a new census data entry
    pub fn new(region_id: String, year: u32, total_population: u64) -> Self {
        Self {
            region_id,
            year,
            total_population,
            age_distribution: HashMap::new(),
            gender_distribution: HashMap::new(),
            education_distribution: HashMap::new(),
            employment_distribution: HashMap::new(),
            median_household_income: 0,
            average_household_size: 2.5,
        }
    }

    /// Get proportion of population in age group
    pub fn get_age_proportion(&self, age_group: &AgeGroup) -> f64 {
        if self.total_population == 0 {
            return 0.0;
        }
        let count = self.age_distribution.get(age_group).copied().unwrap_or(0);
        count as f64 / self.total_population as f64
    }

    /// Get gender ratio (males per female)
    pub fn get_gender_ratio(&self) -> f64 {
        let males = self.gender_distribution.get(&Gender::Male).copied().unwrap_or(0);
        let females = self.gender_distribution.get(&Gender::Female).copied().unwrap_or(0);
        if females == 0 {
            0.0
        } else {
            males as f64 / females as f64
        }
    }

    /// Get dependency ratio (young + elderly / working age)
    pub fn get_dependency_ratio(&self) -> f64 {
        let young = self.age_distribution.get(&AgeGroup::Children).copied().unwrap_or(0)
            + self.age_distribution.get(&AgeGroup::Youth).copied().unwrap_or(0);
        let elderly = self.age_distribution.get(&AgeGroup::Elderly).copied().unwrap_or(0);
        let working = self.age_distribution.get(&AgeGroup::WorkingAge).copied().unwrap_or(0);

        if working == 0 {
            0.0
        } else {
            (young + elderly) as f64 / working as f64
        }
    }
}

/// Age group categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AgeGroup {
    /// 0-14 years
    Children,
    /// 15-24 years
    Youth,
    /// 25-64 years
    WorkingAge,
    /// 65+ years
    Elderly,
}

/// Gender categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    Other,
}

/// Education level categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EducationLevel {
    LessThanHighSchool,
    HighSchool,
    SomeCollege,
    Bachelor,
    Graduate,
}

/// Employment status categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EmploymentStatus {
    Employed,
    Unemployed,
    NotInLaborForce,
}

// ============================================================================
// Mortality and Fertility Modeling
// ============================================================================

/// Mortality rate model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MortalityModel {
    /// Age-specific mortality rates (age -> rate per 1000)
    pub age_specific_rates: HashMap<u32, f64>,
    /// Baseline mortality rate
    pub baseline_rate: f64,
    /// Gender-specific adjustment factors
    pub gender_adjustments: HashMap<Gender, f64>,
}

impl MortalityModel {
    /// Create a new mortality model with baseline rate
    pub fn new(baseline_rate: f64) -> Self {
        Self {
            age_specific_rates: HashMap::new(),
            baseline_rate,
            gender_adjustments: HashMap::new(),
        }
    }

    /// Set age-specific mortality rate
    pub fn set_age_rate(&mut self, age: u32, rate: f64) {
        self.age_specific_rates.insert(age, rate);
    }

    /// Get mortality rate for age and gender
    pub fn get_mortality_rate(&self, age: u32, gender: Gender) -> f64 {
        let base_rate = self.age_specific_rates.get(&age)
            .copied()
            .unwrap_or(self.baseline_rate);

        let gender_adjustment = self.gender_adjustments.get(&gender)
            .copied()
            .unwrap_or(1.0);

        base_rate * gender_adjustment
    }

    /// Calculate probability of death in one year
    pub fn death_probability(&self, age: u32, gender: Gender) -> f64 {
        let rate = self.get_mortality_rate(age, gender);
        // Convert rate per 1000 to probability
        rate / 1000.0
    }

    /// Create a Gompertz-Makeham mortality model
    /// Rate = A + B * exp(C * age)
    pub fn gompertz_makeham(a: f64, b: f64, c: f64) -> Self {
        let mut model = Self::new(a);
        for age in 0..120 {
            let rate = a + b * (c * age as f64).exp();
            model.set_age_rate(age, rate);
        }
        model
    }
}

/// Fertility rate model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FertilityModel {
    /// Age-specific fertility rates (age -> births per 1000 women)
    pub age_specific_rates: HashMap<u32, f64>,
    /// Total fertility rate (average children per woman)
    pub total_fertility_rate: f64,
}

impl FertilityModel {
    /// Create a new fertility model
    pub fn new(total_fertility_rate: f64) -> Self {
        Self {
            age_specific_rates: HashMap::new(),
            total_fertility_rate,
        }
    }

    /// Set age-specific fertility rate
    pub fn set_age_rate(&mut self, age: u32, rate: f64) {
        self.age_specific_rates.insert(age, rate);
    }

    /// Get fertility rate for age
    pub fn get_fertility_rate(&self, age: u32) -> f64 {
        self.age_specific_rates.get(&age).copied().unwrap_or(0.0)
    }

    /// Calculate probability of birth in one year
    pub fn birth_probability(&self, age: u32) -> f64 {
        let rate = self.get_fertility_rate(age);
        // Convert rate per 1000 to probability
        rate / 1000.0
    }

    /// Create a realistic fertility model (bell curve centered at age 28)
    pub fn realistic_model(total_fertility_rate: f64) -> Self {
        let mut model = Self::new(total_fertility_rate);

        // Peak fertility at age 28
        let peak_age = 28.0_f64;
        let std_dev = 6.0_f64;

        for age in 15..50 {
            // Normal distribution around peak age
            let x = age as f64;
            let rate = 100.0 * (-(x - peak_age).powi(2) / (2.0 * std_dev.powi(2))).exp();
            model.set_age_rate(age, rate);
        }

        model
    }
}

// ============================================================================
// Migration Modeling
// ============================================================================

/// Migration pattern model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationModel {
    /// In-migration rate (annual per 1000)
    pub in_migration_rate: f64,
    /// Out-migration rate (annual per 1000)
    pub out_migration_rate: f64,
    /// Age-specific migration propensities
    pub age_propensities: HashMap<u32, f64>,
    /// Economic migration multiplier (GDP growth correlation)
    pub economic_multiplier: f64,
    /// Source regions and their contribution weights
    pub source_regions: HashMap<String, f64>,
    /// Destination regions and their attraction weights
    pub destination_regions: HashMap<String, f64>,
}

impl MigrationModel {
    /// Create a new migration model
    pub fn new(in_migration_rate: f64, out_migration_rate: f64) -> Self {
        Self {
            in_migration_rate,
            out_migration_rate,
            age_propensities: HashMap::new(),
            economic_multiplier: 1.0,
            source_regions: HashMap::new(),
            destination_regions: HashMap::new(),
        }
    }

    /// Get net migration rate
    pub fn net_migration_rate(&self) -> f64 {
        self.in_migration_rate - self.out_migration_rate
    }

    /// Get migration probability for age
    pub fn migration_probability(&self, age: u32, is_outward: bool) -> f64 {
        let base_rate = if is_outward {
            self.out_migration_rate
        } else {
            self.in_migration_rate
        };

        let age_propensity = self.age_propensities.get(&age).copied().unwrap_or(1.0);

        // Young adults (20-35) have higher migration propensity
        let age_factor = if (20..=35).contains(&age) {
            1.5
        } else if age > 65 {
            0.5
        } else {
            1.0
        };

        (base_rate / 1000.0) * age_propensity * age_factor * self.economic_multiplier
    }

    /// Set economic conditions (positive = growth, negative = recession)
    pub fn set_economic_conditions(&mut self, gdp_growth_rate: f64) {
        // Higher GDP growth attracts more migrants, reduces emigration
        self.economic_multiplier = 1.0 + gdp_growth_rate;
    }
}

// ============================================================================
// Household Formation
// ============================================================================

/// Household type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HouseholdType {
    Single,
    Couple,
    Family,
    MultiGenerational,
    NonFamily,
}

/// Household structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Household {
    /// Household identifier
    pub id: Uuid,
    /// Household type
    pub household_type: HouseholdType,
    /// Member IDs
    pub members: Vec<Uuid>,
    /// Household income
    pub income: u64,
    /// Head of household age
    pub head_age: u32,
    /// Number of children
    pub num_children: u32,
}

impl Household {
    /// Create a new household
    pub fn new(household_type: HouseholdType, head_age: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            household_type,
            members: Vec::new(),
            income: 0,
            head_age,
            num_children: 0,
        }
    }

    /// Add member to household
    pub fn add_member(&mut self, member_id: Uuid) {
        self.members.push(member_id);
    }

    /// Get household size
    pub fn size(&self) -> usize {
        self.members.len()
    }
}

/// Household formation model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HouseholdFormationModel {
    /// Age-specific marriage rates
    pub marriage_rates: HashMap<u32, f64>,
    /// Age-specific cohabitation rates
    pub cohabitation_rates: HashMap<u32, f64>,
    /// Average age at first marriage
    pub avg_marriage_age: f64,
    /// Average household size by type
    pub avg_sizes: HashMap<HouseholdType, f64>,
    /// Probability of household type formation by age
    pub formation_probabilities: HashMap<u32, HashMap<HouseholdType, f64>>,
}

impl HouseholdFormationModel {
    /// Create a new household formation model
    pub fn new() -> Self {
        Self {
            marriage_rates: HashMap::new(),
            cohabitation_rates: HashMap::new(),
            avg_marriage_age: 28.0,
            avg_sizes: HashMap::new(),
            formation_probabilities: HashMap::new(),
        }
    }

    /// Get probability of forming household type at given age
    pub fn formation_probability(&self, age: u32, household_type: &HouseholdType) -> f64 {
        self.formation_probabilities
            .get(&age)
            .and_then(|probs| probs.get(household_type))
            .copied()
            .unwrap_or(0.0)
    }

    /// Set formation probability
    pub fn set_formation_probability(&mut self, age: u32, household_type: HouseholdType, probability: f64) {
        self.formation_probabilities
            .entry(age)
            .or_default()
            .insert(household_type, probability);
    }

    /// Create realistic household formation model
    pub fn realistic_model() -> Self {
        let mut model = Self::new();
        model.avg_marriage_age = 28.0;

        // Set average sizes
        model.avg_sizes.insert(HouseholdType::Single, 1.0);
        model.avg_sizes.insert(HouseholdType::Couple, 2.0);
        model.avg_sizes.insert(HouseholdType::Family, 3.5);
        model.avg_sizes.insert(HouseholdType::MultiGenerational, 5.0);
        model.avg_sizes.insert(HouseholdType::NonFamily, 2.5);

        // Young adults tend to be single
        for age in 18..25 {
            model.set_formation_probability(age, HouseholdType::Single, 0.7);
            model.set_formation_probability(age, HouseholdType::Couple, 0.2);
            model.set_formation_probability(age, HouseholdType::Family, 0.1);
        }

        // Middle age tends to be family
        for age in 25..45 {
            model.set_formation_probability(age, HouseholdType::Single, 0.2);
            model.set_formation_probability(age, HouseholdType::Couple, 0.3);
            model.set_formation_probability(age, HouseholdType::Family, 0.5);
        }

        // Older age tends to be couple or single
        for age in 45..65 {
            model.set_formation_probability(age, HouseholdType::Single, 0.3);
            model.set_formation_probability(age, HouseholdType::Couple, 0.5);
            model.set_formation_probability(age, HouseholdType::Family, 0.2);
        }

        model
    }
}

impl Default for HouseholdFormationModel {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Income Mobility
// ============================================================================

/// Income quintile
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IncomeQuintile {
    Bottom,
    Second,
    Middle,
    Fourth,
    Top,
}

/// Income mobility model (transition matrix between quintiles)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomeMobilityModel {
    /// Transition probabilities: from quintile -> to quintile -> probability
    pub transition_matrix: HashMap<IncomeQuintile, HashMap<IncomeQuintile, f64>>,
    /// Intergenerational mobility (parent quintile -> child quintile -> probability)
    pub intergenerational_matrix: HashMap<IncomeQuintile, HashMap<IncomeQuintile, f64>>,
    /// Income growth rates by quintile
    pub growth_rates: HashMap<IncomeQuintile, f64>,
}

impl IncomeMobilityModel {
    /// Create a new income mobility model
    pub fn new() -> Self {
        Self {
            transition_matrix: HashMap::new(),
            intergenerational_matrix: HashMap::new(),
            growth_rates: HashMap::new(),
        }
    }

    /// Set transition probability
    pub fn set_transition(&mut self, from: IncomeQuintile, to: IncomeQuintile, probability: f64) {
        self.transition_matrix
            .entry(from)
            .or_default()
            .insert(to, probability);
    }

    /// Get transition probability
    pub fn get_transition(&self, from: &IncomeQuintile, to: &IncomeQuintile) -> f64 {
        self.transition_matrix
            .get(from)
            .and_then(|transitions| transitions.get(to))
            .copied()
            .unwrap_or(0.0)
    }

    /// Simulate income transition
    pub fn simulate_transition(&self, current_quintile: &IncomeQuintile) -> IncomeQuintile {
        let quintiles = [
            IncomeQuintile::Bottom,
            IncomeQuintile::Second,
            IncomeQuintile::Middle,
            IncomeQuintile::Fourth,
            IncomeQuintile::Top,
        ];

        let rand_val = rand::random::<f64>();
        let mut cumulative = 0.0;

        for quintile in &quintiles {
            cumulative += self.get_transition(current_quintile, quintile);
            if rand_val < cumulative {
                return *quintile;
            }
        }

        *current_quintile
    }

    /// Create a model with high mobility
    pub fn high_mobility_model() -> Self {
        let mut model = Self::new();

        // Bottom quintile has 40% chance to move up
        model.set_transition(IncomeQuintile::Bottom, IncomeQuintile::Bottom, 0.60);
        model.set_transition(IncomeQuintile::Bottom, IncomeQuintile::Second, 0.30);
        model.set_transition(IncomeQuintile::Bottom, IncomeQuintile::Middle, 0.10);

        // Middle quintile has equal chances to move
        model.set_transition(IncomeQuintile::Middle, IncomeQuintile::Second, 0.20);
        model.set_transition(IncomeQuintile::Middle, IncomeQuintile::Middle, 0.60);
        model.set_transition(IncomeQuintile::Middle, IncomeQuintile::Fourth, 0.20);

        // Top quintile has 80% chance to stay
        model.set_transition(IncomeQuintile::Top, IncomeQuintile::Fourth, 0.15);
        model.set_transition(IncomeQuintile::Top, IncomeQuintile::Top, 0.85);

        model
    }

    /// Create a model with low mobility (sticky ends)
    pub fn low_mobility_model() -> Self {
        let mut model = Self::new();

        // Bottom quintile has 80% chance to stay
        model.set_transition(IncomeQuintile::Bottom, IncomeQuintile::Bottom, 0.80);
        model.set_transition(IncomeQuintile::Bottom, IncomeQuintile::Second, 0.15);
        model.set_transition(IncomeQuintile::Bottom, IncomeQuintile::Middle, 0.05);

        // Middle quintile is more stable
        model.set_transition(IncomeQuintile::Middle, IncomeQuintile::Second, 0.10);
        model.set_transition(IncomeQuintile::Middle, IncomeQuintile::Middle, 0.80);
        model.set_transition(IncomeQuintile::Middle, IncomeQuintile::Fourth, 0.10);

        // Top quintile has 90% chance to stay
        model.set_transition(IncomeQuintile::Top, IncomeQuintile::Fourth, 0.10);
        model.set_transition(IncomeQuintile::Top, IncomeQuintile::Top, 0.90);

        model
    }
}

impl Default for IncomeMobilityModel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_census_data_creation() {
        let census = CensusData::new("US-CA".to_string(), 2020, 39_500_000);
        assert_eq!(census.region_id, "US-CA");
        assert_eq!(census.year, 2020);
        assert_eq!(census.total_population, 39_500_000);
    }

    #[test]
    fn test_census_age_proportion() {
        let mut census = CensusData::new("US-CA".to_string(), 2020, 100);
        census.age_distribution.insert(AgeGroup::WorkingAge, 60);

        let proportion = census.get_age_proportion(&AgeGroup::WorkingAge);
        assert!((proportion - 0.6).abs() < 1e-6);
    }

    #[test]
    fn test_census_gender_ratio() {
        let mut census = CensusData::new("US-CA".to_string(), 2020, 100);
        census.gender_distribution.insert(Gender::Male, 48);
        census.gender_distribution.insert(Gender::Female, 52);

        let ratio = census.get_gender_ratio();
        assert!((ratio - 0.923).abs() < 0.01);
    }

    #[test]
    fn test_census_dependency_ratio() {
        let mut census = CensusData::new("US-CA".to_string(), 2020, 100);
        census.age_distribution.insert(AgeGroup::Children, 20);
        census.age_distribution.insert(AgeGroup::WorkingAge, 60);
        census.age_distribution.insert(AgeGroup::Elderly, 15);

        let ratio = census.get_dependency_ratio();
        assert!((ratio - 0.583).abs() < 0.01);
    }

    #[test]
    fn test_mortality_model_basic() {
        let mut model = MortalityModel::new(5.0);
        model.set_age_rate(65, 20.0);

        let rate = model.get_mortality_rate(65, Gender::Male);
        assert_eq!(rate, 20.0);
    }

    #[test]
    fn test_mortality_death_probability() {
        let mut model = MortalityModel::new(5.0);
        model.set_age_rate(80, 100.0);

        let prob = model.death_probability(80, Gender::Female);
        assert_eq!(prob, 0.1);
    }

    #[test]
    fn test_mortality_gompertz_makeham() {
        let model = MortalityModel::gompertz_makeham(0.001, 0.0001, 0.08);

        // Mortality should increase with age
        let rate_20 = model.get_mortality_rate(20, Gender::Male);
        let rate_80 = model.get_mortality_rate(80, Gender::Male);
        assert!(rate_80 > rate_20);
    }

    #[test]
    fn test_fertility_model_basic() {
        let mut model = FertilityModel::new(2.1);
        model.set_age_rate(28, 120.0);

        let rate = model.get_fertility_rate(28);
        assert_eq!(rate, 120.0);
    }

    #[test]
    fn test_fertility_realistic_model() {
        let model = FertilityModel::realistic_model(2.1);

        // Peak fertility should be around age 28
        let rate_28 = model.get_fertility_rate(28);
        let rate_20 = model.get_fertility_rate(20);
        let rate_40 = model.get_fertility_rate(40);

        assert!(rate_28 > rate_20);
        assert!(rate_28 > rate_40);
    }

    #[test]
    fn test_migration_model_basic() {
        let model = MigrationModel::new(10.0, 5.0);
        assert_eq!(model.net_migration_rate(), 5.0);
    }

    #[test]
    fn test_migration_economic_conditions() {
        let mut model = MigrationModel::new(10.0, 5.0);
        model.set_economic_conditions(0.05); // 5% GDP growth

        assert!(model.economic_multiplier > 1.0);
    }

    #[test]
    fn test_migration_age_probability() {
        let model = MigrationModel::new(10.0, 5.0);

        // Young adults should have higher migration probability
        let prob_25 = model.migration_probability(25, false);
        let prob_70 = model.migration_probability(70, false);

        assert!(prob_25 > prob_70);
    }

    #[test]
    fn test_household_creation() {
        let household = Household::new(HouseholdType::Family, 35);
        assert_eq!(household.household_type, HouseholdType::Family);
        assert_eq!(household.head_age, 35);
    }

    #[test]
    fn test_household_members() {
        let mut household = Household::new(HouseholdType::Family, 35);
        household.add_member(Uuid::new_v4());
        household.add_member(Uuid::new_v4());

        assert_eq!(household.size(), 2);
    }

    #[test]
    fn test_household_formation_model() {
        let mut model = HouseholdFormationModel::new();
        model.set_formation_probability(25, HouseholdType::Single, 0.6);

        let prob = model.formation_probability(25, &HouseholdType::Single);
        assert_eq!(prob, 0.6);
    }

    #[test]
    fn test_household_formation_realistic() {
        let model = HouseholdFormationModel::realistic_model();

        // Young adults should have higher single probability
        let single_prob_20 = model.formation_probability(20, &HouseholdType::Single);
        let family_prob_20 = model.formation_probability(20, &HouseholdType::Family);

        assert!(single_prob_20 > family_prob_20);
    }

    #[test]
    fn test_income_mobility_model() {
        let mut model = IncomeMobilityModel::new();
        model.set_transition(IncomeQuintile::Bottom, IncomeQuintile::Second, 0.3);

        let prob = model.get_transition(&IncomeQuintile::Bottom, &IncomeQuintile::Second);
        assert_eq!(prob, 0.3);
    }

    #[test]
    fn test_income_mobility_high_mobility() {
        let model = IncomeMobilityModel::high_mobility_model();

        // Bottom quintile should have reasonable chance to move up
        let stay_bottom = model.get_transition(&IncomeQuintile::Bottom, &IncomeQuintile::Bottom);
        assert!(stay_bottom < 0.7);
    }

    #[test]
    fn test_income_mobility_low_mobility() {
        let model = IncomeMobilityModel::low_mobility_model();

        // Bottom quintile should have high chance to stay
        let stay_bottom = model.get_transition(&IncomeQuintile::Bottom, &IncomeQuintile::Bottom);
        assert!(stay_bottom >= 0.8);
    }

    #[test]
    fn test_income_mobility_simulation() {
        let model = IncomeMobilityModel::high_mobility_model();

        // Run simulation multiple times (just verify no panic)
        for _ in 0..10 {
            let _next = model.simulate_transition(&IncomeQuintile::Middle);
        }
    }
}
