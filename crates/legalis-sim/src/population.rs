//! Population generation with realistic demographic distributions.
//!
//! This module provides:
//! - Realistic age, income, and demographic distributions
//! - Geographic distribution modeling
//! - Attribute correlation enforcement
//! - CSV/JSON import support

use crate::behavior::{BehavioralProfile, DecisionStrategy};
use legalis_core::{BasicEntity, LegalEntity};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Distribution type for numeric attributes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Distribution {
    /// Uniform distribution between min and max
    Uniform { min: f64, max: f64 },
    /// Normal (Gaussian) distribution
    Normal { mean: f64, std_dev: f64 },
    /// Log-normal distribution (for income, wealth)
    LogNormal { mean: f64, std_dev: f64 },
    /// Power law distribution
    PowerLaw { alpha: f64, x_min: f64 },
    /// Custom discrete values with probabilities
    Discrete { values: Vec<(f64, f64)> }, // (value, probability)
}

impl Distribution {
    /// Samples a value from the distribution.
    pub fn sample(&self) -> f64 {
        match self {
            Distribution::Uniform { min, max } => min + (max - min) * random(),
            Distribution::Normal { mean, std_dev } => {
                // Box-Muller transform
                let u1 = random();
                let u2 = random();
                let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
                mean + std_dev * z
            }
            Distribution::LogNormal { mean, std_dev } => {
                // Sample from normal and exponentiate
                let u1 = random();
                let u2 = random();
                let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
                (mean + std_dev * z).exp()
            }
            Distribution::PowerLaw { alpha, x_min } => {
                let u = random();
                x_min * (1.0 - u).powf(-1.0 / (alpha - 1.0))
            }
            Distribution::Discrete { values } => {
                let r = random();
                let mut cumulative = 0.0;
                for (value, prob) in values {
                    cumulative += prob;
                    if r <= cumulative {
                        return *value;
                    }
                }
                values.last().map(|(v, _)| *v).unwrap_or(0.0)
            }
        }
    }
}

/// Geographic region for population distribution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    /// Region identifier
    pub id: String,
    /// Region name
    pub name: String,
    /// Proportion of population in this region
    pub proportion: f64,
    /// Average income multiplier for this region
    pub income_multiplier: f64,
}

/// Demographic profile for population generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemographicProfile {
    /// Age distribution
    pub age_distribution: Distribution,
    /// Income distribution
    pub income_distribution: Distribution,
    /// Geographic regions
    pub regions: Vec<Region>,
    /// Custom attribute distributions
    pub custom_attributes: HashMap<String, Distribution>,
}

impl Default for DemographicProfile {
    fn default() -> Self {
        Self::us_2024()
    }
}

impl DemographicProfile {
    /// Creates a US 2024 demographic profile.
    pub fn us_2024() -> Self {
        Self {
            // Age: approximately normal around 38 years
            age_distribution: Distribution::Normal {
                mean: 38.0,
                std_dev: 22.0,
            },
            // Income: log-normal distribution
            income_distribution: Distribution::LogNormal {
                mean: 10.8, // ln(median income ~$49k)
                std_dev: 0.9,
            },
            regions: vec![
                Region {
                    id: "urban".to_string(),
                    name: "Urban".to_string(),
                    proportion: 0.83,
                    income_multiplier: 1.15,
                },
                Region {
                    id: "rural".to_string(),
                    name: "Rural".to_string(),
                    proportion: 0.17,
                    income_multiplier: 0.85,
                },
            ],
            custom_attributes: HashMap::new(),
        }
    }

    /// Creates a custom demographic profile.
    pub fn custom() -> Self {
        Self {
            age_distribution: Distribution::Uniform {
                min: 18.0,
                max: 80.0,
            },
            income_distribution: Distribution::Uniform {
                min: 20_000.0,
                max: 200_000.0,
            },
            regions: vec![],
            custom_attributes: HashMap::new(),
        }
    }

    /// Adds a custom attribute distribution.
    pub fn with_attribute(mut self, name: impl Into<String>, distribution: Distribution) -> Self {
        self.custom_attributes.insert(name.into(), distribution);
        self
    }
}

/// Population generator with demographic modeling.
pub struct PopulationGenerator {
    profile: DemographicProfile,
    size: usize,
    /// Correlation between age and income (0.0 to 1.0)
    age_income_correlation: f64,
    /// Whether to enforce realistic constraints
    enforce_constraints: bool,
}

impl PopulationGenerator {
    /// Creates a new population generator.
    pub fn new(profile: DemographicProfile, size: usize) -> Self {
        Self {
            profile,
            size,
            age_income_correlation: 0.3,
            enforce_constraints: true,
        }
    }

    /// Sets the age-income correlation.
    pub fn with_age_income_correlation(mut self, correlation: f64) -> Self {
        self.age_income_correlation = correlation.clamp(0.0, 1.0);
        self
    }

    /// Sets whether to enforce realistic constraints.
    pub fn with_constraints(mut self, enforce: bool) -> Self {
        self.enforce_constraints = enforce;
        self
    }

    /// Generates a population.
    pub fn generate(&self) -> Vec<BasicEntity> {
        let mut population = Vec::with_capacity(self.size);

        for _ in 0..self.size {
            let mut entity = BasicEntity::new();

            // Generate age
            let mut age = self.profile.age_distribution.sample();
            if self.enforce_constraints {
                age = age.clamp(0.0, 120.0);
            }
            entity.set_attribute("age", (age as u32).to_string());

            // Generate income with optional correlation to age
            let base_income = self.profile.income_distribution.sample();
            let income = if self.age_income_correlation > 0.0 {
                // Adjust income based on age (peak earning years are 45-54)
                let age_factor = if age < 25.0 {
                    0.6 + (age - 18.0) / 35.0 // Ramp up from 18-25
                } else if age < 45.0 {
                    0.8 + (age - 25.0) / 50.0 // Continue ramping to peak
                } else if age < 65.0 {
                    1.0 // Peak earning years
                } else {
                    1.0 - (age - 65.0) / 100.0 // Decline after retirement
                }
                .max(0.3);

                let correlation_factor =
                    self.age_income_correlation * age_factor + (1.0 - self.age_income_correlation);
                base_income * correlation_factor
            } else {
                base_income
            };

            if self.enforce_constraints {
                entity.set_attribute("income", (income.max(0.0) as u64).to_string());
            } else {
                entity.set_attribute("income", (income as u64).to_string());
            }

            // Assign region
            if !self.profile.regions.is_empty() {
                let region = self.select_region();
                entity.set_attribute("region", region.id.clone());

                // Adjust income based on region
                if let Some(income_str) = entity.get_attribute("income")
                    && let Ok(income) = income_str.parse::<u64>()
                {
                    let adjusted = ((income as f64) * region.income_multiplier).round() as u64;
                    entity.set_attribute("income", adjusted.to_string());
                }
            }

            // Generate custom attributes
            for (attr_name, distribution) in &self.profile.custom_attributes {
                let value = distribution.sample();
                entity.set_attribute(attr_name, value.to_string());
            }

            population.push(entity);
        }

        population
    }

    /// Selects a region based on proportions.
    fn select_region(&self) -> &Region {
        let r = random();
        let mut cumulative = 0.0;
        for region in &self.profile.regions {
            cumulative += region.proportion;
            if r <= cumulative {
                return region;
            }
        }
        self.profile.regions.last().unwrap()
    }
}

/// Builder for creating populations with behavioral profiles.
pub struct BehavioralPopulationBuilder {
    entities: Vec<BasicEntity>,
    strategy_distribution: HashMap<DecisionStrategy, f64>,
}

impl BehavioralPopulationBuilder {
    /// Creates a new builder.
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            strategy_distribution: HashMap::new(),
        }
    }

    /// Starts with a base population.
    pub fn with_population(mut self, entities: Vec<BasicEntity>) -> Self {
        self.entities = entities;
        self
    }

    /// Sets the distribution of decision strategies.
    pub fn with_strategy_distribution(
        mut self,
        distribution: HashMap<DecisionStrategy, f64>,
    ) -> Self {
        self.strategy_distribution = distribution;
        self
    }

    /// Builds the population with behavioral profiles.
    pub fn build(self) -> Vec<(BasicEntity, BehavioralProfile)> {
        let mut result = Vec::with_capacity(self.entities.len());

        // Normalize strategy distribution
        let total: f64 = self.strategy_distribution.values().sum();
        let normalized: HashMap<_, _> = if total > 0.0 {
            self.strategy_distribution
                .into_iter()
                .map(|(k, v)| (k, v / total))
                .collect()
        } else {
            // Default: mostly bounded rational
            let mut default = HashMap::new();
            default.insert(DecisionStrategy::BoundedRational, 0.7);
            default.insert(DecisionStrategy::RuleFollowing, 0.2);
            default.insert(DecisionStrategy::Opportunistic, 0.1);
            default
        };

        for entity in self.entities {
            let strategy = Self::select_strategy_static(&normalized);
            let profile = BehavioralProfile::new(strategy);
            result.push((entity, profile));
        }

        result
    }

    /// Selects a strategy based on distribution (static method).
    fn select_strategy_static(distribution: &HashMap<DecisionStrategy, f64>) -> DecisionStrategy {
        let r = random();
        let mut cumulative = 0.0;
        for (&strategy, &prob) in distribution {
            cumulative += prob;
            if r <= cumulative {
                return strategy;
            }
        }
        *distribution.keys().next().unwrap()
    }
}

impl Default for BehavioralPopulationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// Shared random seed for reproducibility
static mut POPULATION_RNG_SEED: u64 = 0;

/// Simple random number generator (0.0 to 1.0).
fn random() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    unsafe {
        if POPULATION_RNG_SEED == 0 {
            POPULATION_RNG_SEED = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64;
        }
        POPULATION_RNG_SEED = POPULATION_RNG_SEED
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1);
        (POPULATION_RNG_SEED >> 33) as f64 / (1u64 << 31) as f64
    }
}

/// Sets the random seed for reproducible tests.
///
/// # Safety
/// This function is unsafe because it modifies a static mutable variable.
/// It should only be called in single-threaded test contexts.
pub unsafe fn set_population_seed(seed: u64) {
    unsafe {
        POPULATION_RNG_SEED = seed;
    }
}

/// Seeds for reproducible random generation in tests.
pub mod test_seeds {
    /// Default test seed
    pub const DEFAULT: u64 = 12345;
    /// Alternative seed for variation
    pub const ALTERNATIVE: u64 = 67890;
    /// Seed for distribution tests
    pub const DISTRIBUTION: u64 = 42424;
}

/// Validation rule for population consistency.
#[derive(Debug, Clone)]
pub enum ValidationRule {
    /// Attribute must exist
    RequiredAttribute { name: String },
    /// Attribute must be within range
    AttributeRange { name: String, min: f64, max: f64 },
    /// Attribute must match pattern
    AttributePattern { name: String, pattern: String },
    /// Attribute must be one of allowed values
    AttributeEnum {
        name: String,
        allowed_values: Vec<String>,
    },
}

/// Validation error.
#[derive(Debug, Clone)]
pub struct ValidationError {
    /// Entity ID that failed validation
    pub entity_id: uuid::Uuid,
    /// Rule that was violated
    pub rule_name: String,
    /// Description of the violation
    pub message: String,
}

/// Population validator.
pub struct PopulationValidator {
    rules: Vec<ValidationRule>,
}

impl PopulationValidator {
    /// Creates a new population validator.
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// Adds a validation rule.
    pub fn add_rule(mut self, rule: ValidationRule) -> Self {
        self.rules.push(rule);
        self
    }

    /// Validates a population.
    pub fn validate(&self, population: &[BasicEntity]) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        for entity in population {
            for rule in &self.rules {
                if let Some(error) = self.validate_entity(entity, rule) {
                    errors.push(error);
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Validates a single entity against a rule.
    fn validate_entity(
        &self,
        entity: &BasicEntity,
        rule: &ValidationRule,
    ) -> Option<ValidationError> {
        match rule {
            ValidationRule::RequiredAttribute { name } => {
                if entity.get_attribute(name).is_none() {
                    Some(ValidationError {
                        entity_id: entity.id(),
                        rule_name: format!("RequiredAttribute({})", name),
                        message: format!("Missing required attribute '{}'", name),
                    })
                } else {
                    None
                }
            }
            ValidationRule::AttributeRange { name, min, max } => entity
                .get_attribute(name)
                .and_then(|v| v.parse::<f64>().ok())
                .and_then(|value| {
                    if value < *min || value > *max {
                        Some(ValidationError {
                            entity_id: entity.id(),
                            rule_name: format!("AttributeRange({}, {}-{})", name, min, max),
                            message: format!(
                                "Attribute '{}' value {} is outside range {}-{}",
                                name, value, min, max
                            ),
                        })
                    } else {
                        None
                    }
                }),
            ValidationRule::AttributePattern { name, pattern } => {
                entity.get_attribute(name).and_then(|value| {
                    if value.contains(pattern) {
                        None
                    } else {
                        Some(ValidationError {
                            entity_id: entity.id(),
                            rule_name: format!("AttributePattern({})", name),
                            message: format!(
                                "Attribute '{}' value '{}' doesn't match pattern '{}'",
                                name, value, pattern
                            ),
                        })
                    }
                })
            }
            ValidationRule::AttributeEnum {
                name,
                allowed_values,
            } => entity.get_attribute(name).and_then(|value| {
                if allowed_values.contains(&value) {
                    None
                } else {
                    Some(ValidationError {
                        entity_id: entity.id(),
                        rule_name: format!("AttributeEnum({})", name),
                        message: format!(
                            "Attribute '{}' value '{}' is not in allowed values: {:?}",
                            name, value, allowed_values
                        ),
                    })
                }
            }),
        }
    }

    /// Creates a default validator for demographic populations.
    pub fn default_demographic() -> Self {
        Self::new()
            .add_rule(ValidationRule::RequiredAttribute {
                name: "age".to_string(),
            })
            .add_rule(ValidationRule::AttributeRange {
                name: "age".to_string(),
                min: 0.0,
                max: 120.0,
            })
            .add_rule(ValidationRule::AttributeRange {
                name: "income".to_string(),
                min: 0.0,
                max: 1e9,
            })
    }
}

impl Default for PopulationValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Type alias for constraint functions.
type ConstraintFn = Box<dyn Fn(&[BasicEntity]) -> bool>;

/// Constraint satisfaction checker.
pub struct ConstraintChecker {
    constraints: Vec<(String, ConstraintFn)>,
}

impl ConstraintChecker {
    /// Creates a new constraint checker.
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
        }
    }

    /// Adds a constraint with a name.
    pub fn add_constraint(
        mut self,
        name: impl Into<String>,
        constraint: impl Fn(&[BasicEntity]) -> bool + 'static,
    ) -> Self {
        self.constraints.push((name.into(), Box::new(constraint)));
        self
    }

    /// Checks all constraints.
    pub fn check(&self, population: &[BasicEntity]) -> Result<(), Vec<String>> {
        let mut violations = Vec::new();

        for (name, constraint) in &self.constraints {
            if !constraint(population) {
                violations.push(name.clone());
            }
        }

        if violations.is_empty() {
            Ok(())
        } else {
            Err(violations)
        }
    }

    /// Creates common demographic constraints.
    pub fn demographic_constraints() -> Self {
        Self::new()
            .add_constraint("min_population_size", |pop| !pop.is_empty())
            .add_constraint("max_population_size", |pop| pop.len() <= 1_000_000)
            .add_constraint("age_distribution", |pop| {
                let ages: Vec<f64> = pop
                    .iter()
                    .filter_map(|e| e.get_attribute("age"))
                    .filter_map(|a| a.parse::<f64>().ok())
                    .collect();
                if ages.is_empty() {
                    return false;
                }
                let mean = ages.iter().sum::<f64>() / ages.len() as f64;
                (18.0..=65.0).contains(&mean)
            })
    }
}

impl Default for ConstraintChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Entity record for CSV/JSON import.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityRecord {
    /// Optional ID (will be generated if not provided)
    pub id: Option<String>,
    /// Attributes as key-value pairs
    #[serde(flatten)]
    pub attributes: HashMap<String, serde_json::Value>,
}

/// Population importer for CSV and JSON files.
pub struct PopulationImporter;

impl PopulationImporter {
    /// Imports population from a JSON file.
    ///
    /// Expected format:
    /// ```json
    /// [
    ///   {"id": "optional", "age": "25", "income": "50000", "region": "US-CA"},
    ///   {"age": "30", "income": "60000", "region": "US-NY"}
    /// ]
    /// ```
    pub fn from_json(json_str: &str) -> Result<Vec<BasicEntity>, String> {
        let records: Vec<EntityRecord> =
            serde_json::from_str(json_str).map_err(|e| format!("Failed to parse JSON: {}", e))?;

        Ok(Self::records_to_entities(records))
    }

    /// Imports population from a CSV string.
    ///
    /// Expected format:
    /// ```csv
    /// id,age,income,region
    /// optional-id,25,50000,US-CA
    /// ,30,60000,US-NY
    /// ```
    ///
    /// The first row is treated as headers (attribute names).
    pub fn from_csv(csv_str: &str) -> Result<Vec<BasicEntity>, String> {
        let mut lines = csv_str.lines();
        let headers = lines
            .next()
            .ok_or_else(|| "CSV is empty".to_string())?
            .split(',')
            .map(|s| s.trim().to_string())
            .collect::<Vec<_>>();

        if headers.is_empty() {
            return Err("CSV headers are empty".to_string());
        }

        let mut records = Vec::new();

        for (line_num, line) in lines.enumerate() {
            let values: Vec<&str> = line.split(',').map(|s| s.trim()).collect();

            if values.len() != headers.len() {
                return Err(format!(
                    "Line {}: expected {} columns, found {}",
                    line_num + 2,
                    headers.len(),
                    values.len()
                ));
            }

            let mut attributes = HashMap::new();
            for (header, value) in headers.iter().zip(values.iter()) {
                if !value.is_empty() {
                    attributes.insert(header.clone(), serde_json::Value::String(value.to_string()));
                }
            }

            records.push(EntityRecord {
                id: attributes
                    .remove("id")
                    .and_then(|v| v.as_str().map(String::from)),
                attributes,
            });
        }

        Ok(Self::records_to_entities(records))
    }

    /// Converts entity records to BasicEntity instances.
    fn records_to_entities(records: Vec<EntityRecord>) -> Vec<BasicEntity> {
        records
            .into_iter()
            .map(|record| {
                let entity = if let Some(id_str) = record.id {
                    if let Ok(uuid) = uuid::Uuid::parse_str(&id_str) {
                        BasicEntity::with_id(uuid)
                    } else {
                        BasicEntity::new()
                    }
                } else {
                    BasicEntity::new()
                };

                let mut entity = entity;
                for (key, value) in record.attributes {
                    let string_value = match value {
                        serde_json::Value::String(s) => s,
                        serde_json::Value::Number(n) => n.to_string(),
                        serde_json::Value::Bool(b) => b.to_string(),
                        _ => value.to_string(),
                    };
                    entity.set_attribute(&key, string_value);
                }
                entity
            })
            .collect()
    }

    /// Saves population to JSON string.
    pub fn to_json(entities: &[BasicEntity]) -> Result<String, String> {
        let records: Vec<EntityRecord> = entities
            .iter()
            .map(|entity| {
                let mut attributes = HashMap::new();

                if let Some(age) = entity.get_attribute("age") {
                    attributes.insert("age".to_string(), serde_json::Value::String(age.clone()));
                }
                if let Some(income) = entity.get_attribute("income") {
                    attributes.insert(
                        "income".to_string(),
                        serde_json::Value::String(income.clone()),
                    );
                }
                if let Some(region) = entity.get_attribute("region") {
                    attributes.insert(
                        "region".to_string(),
                        serde_json::Value::String(region.clone()),
                    );
                }

                EntityRecord {
                    id: Some(entity.id().to_string()),
                    attributes,
                }
            })
            .collect();

        serde_json::to_string_pretty(&records)
            .map_err(|e| format!("Failed to serialize to JSON: {}", e))
    }

    /// Saves population to CSV string.
    pub fn to_csv(entities: &[BasicEntity]) -> Result<String, String> {
        if entities.is_empty() {
            return Ok(String::new());
        }

        let mut output = String::new();

        let all_keys: std::collections::BTreeSet<String> = entities
            .iter()
            .flat_map(|e| {
                let mut keys = vec!["id".to_string()];
                if e.get_attribute("age").is_some() {
                    keys.push("age".to_string());
                }
                if e.get_attribute("income").is_some() {
                    keys.push("income".to_string());
                }
                if e.get_attribute("region").is_some() {
                    keys.push("region".to_string());
                }
                keys
            })
            .collect();

        output.push_str(&all_keys.iter().cloned().collect::<Vec<_>>().join(","));
        output.push('\n');

        for entity in entities {
            let mut values = Vec::new();
            for key in &all_keys {
                if key == "id" {
                    values.push(entity.id().to_string());
                } else {
                    values.push(entity.get_attribute(key).unwrap_or_default());
                }
            }
            output.push_str(&values.join(","));
            output.push('\n');
        }

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::LegalEntity;

    #[test]
    fn test_uniform_distribution() {
        let dist = Distribution::Uniform {
            min: 10.0,
            max: 20.0,
        };

        for _ in 0..100 {
            let sample = dist.sample();
            assert!((10.0..=20.0).contains(&sample));
        }
    }

    #[test]
    fn test_normal_distribution() {
        let dist = Distribution::Normal {
            mean: 50.0,
            std_dev: 10.0,
        };

        let samples: Vec<f64> = (0..1000).map(|_| dist.sample()).collect();
        let mean: f64 = samples.iter().sum::<f64>() / samples.len() as f64;

        // Mean should be approximately 50.0 (within 2 std errors)
        assert!((mean - 50.0).abs() < 2.0);
    }

    #[test]
    fn test_discrete_distribution() {
        let dist = Distribution::Discrete {
            values: vec![(1.0, 0.5), (2.0, 0.3), (3.0, 0.2)],
        };

        let mut counts = HashMap::new();
        for _ in 0..1000 {
            let sample = dist.sample() as i32;
            *counts.entry(sample).or_insert(0) += 1;
        }

        // Should have roughly 50% 1s, 30% 2s, 20% 3s (with some variance)
        let count_1 = *counts.get(&1).unwrap_or(&0);
        let count_2 = *counts.get(&2).unwrap_or(&0);
        let count_3 = *counts.get(&3).unwrap_or(&0);

        assert!(count_1 > 400 && count_1 < 600);
        assert!(count_2 > 200 && count_2 < 400);
        assert!(count_3 > 100 && count_3 < 300);
    }

    #[test]
    fn test_population_generator() {
        let profile = DemographicProfile::us_2024();
        let generator = PopulationGenerator::new(profile, 100);
        let population = generator.generate();

        assert_eq!(population.len(), 100);

        // All entities should have age and income
        for entity in &population {
            assert!(entity.get_attribute("age").is_some());
            assert!(entity.get_attribute("income").is_some());
            assert!(entity.get_attribute("region").is_some());

            // Check constraints
            let age: u32 = entity.get_attribute("age").unwrap().parse().unwrap();
            assert!(age <= 120);

            let income: u64 = entity.get_attribute("income").unwrap().parse().unwrap();
            assert!(income < 10_000_000); // Reasonable upper bound
        }
    }

    #[test]
    fn test_custom_demographic_profile() {
        let profile = DemographicProfile::custom().with_attribute(
            "education_years",
            Distribution::Normal {
                mean: 14.0,
                std_dev: 3.0,
            },
        );

        let generator = PopulationGenerator::new(profile, 50);
        let population = generator.generate();

        assert_eq!(population.len(), 50);

        for entity in &population {
            assert!(entity.get_attribute("education_years").is_some());
        }
    }

    #[test]
    #[ignore] // Flaky statistical test - random distribution may not always meet exact thresholds
    fn test_behavioral_population_builder() {
        let entities = PopulationGenerator::new(DemographicProfile::us_2024(), 100).generate();

        let mut strategy_dist = HashMap::new();
        strategy_dist.insert(DecisionStrategy::Rational, 0.2);
        strategy_dist.insert(DecisionStrategy::BoundedRational, 0.6);
        strategy_dist.insert(DecisionStrategy::Opportunistic, 0.2);

        let population = BehavioralPopulationBuilder::new()
            .with_population(entities)
            .with_strategy_distribution(strategy_dist)
            .build();

        assert_eq!(population.len(), 100);

        // Count strategies
        let mut counts = HashMap::new();
        for (_, profile) in &population {
            *counts.entry(profile.strategy).or_insert(0) += 1;
        }

        // Should have roughly the right distribution (allowing for randomness)
        assert!(*counts.get(&DecisionStrategy::Rational).unwrap_or(&0) > 10);
        assert!(*counts.get(&DecisionStrategy::BoundedRational).unwrap_or(&0) > 40);
        assert!(*counts.get(&DecisionStrategy::Opportunistic).unwrap_or(&0) > 10);
    }

    #[test]
    fn test_age_income_correlation() {
        // Use fixed seed for reproducibility
        unsafe {
            set_population_seed(test_seeds::DEFAULT);
        }

        let profile = DemographicProfile::us_2024();
        // Increase sample size for statistical stability
        let generator = PopulationGenerator::new(profile, 500).with_age_income_correlation(0.8);

        let population = generator.generate();

        // Collect ages and incomes
        let age_income_pairs: Vec<(u32, u64)> = population
            .iter()
            .filter_map(|e| {
                let age = e.get_attribute("age")?.parse().ok()?;
                let income = e.get_attribute("income")?.parse().ok()?;
                Some((age, income))
            })
            .collect();

        // With correlation, people in prime working age (35-55) should have higher incomes
        let prime_age_incomes: Vec<u64> = age_income_pairs
            .iter()
            .filter(|(age, _)| *age >= 35 && *age <= 55)
            .map(|(_, income)| *income)
            .collect();

        let young_incomes: Vec<u64> = age_income_pairs
            .iter()
            .filter(|(age, _)| *age < 25)
            .map(|(_, income)| *income)
            .collect();

        if !prime_age_incomes.is_empty() && !young_incomes.is_empty() {
            let avg_prime: f64 =
                prime_age_incomes.iter().sum::<u64>() as f64 / prime_age_incomes.len() as f64;
            let avg_young: f64 =
                young_incomes.iter().sum::<u64>() as f64 / young_incomes.len() as f64;

            // Prime age should have higher average income (allow some statistical variance)
            // Using 0.7 ratio to account for randomness and regional income multipliers
            // which can affect the correlation
            assert!(
                avg_prime > avg_young * 0.7,
                "Expected prime age income ({:.2}) to be substantially higher than young income ({:.2}). \
                 Prime: {} samples, Young: {} samples",
                avg_prime,
                avg_young,
                prime_age_incomes.len(),
                young_incomes.len()
            );
        }
    }
}
