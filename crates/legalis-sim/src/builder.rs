//! Builder patterns for simulation components.

use crate::{SimEngine, SimResult, SimulationError};
use legalis_core::{LegalEntity, Statute};

/// Builder for configuring and creating a simulation engine.
#[derive(Default)]
pub struct SimEngineBuilder {
    statutes: Vec<Statute>,
    population: Vec<Box<dyn LegalEntity>>,
    validate: bool,
}

impl SimEngineBuilder {
    /// Creates a new simulation engine builder.
    pub fn new() -> Self {
        Self {
            statutes: Vec::new(),
            population: Vec::new(),
            validate: true,
        }
    }

    /// Adds a statute to the simulation.
    pub fn add_statute(mut self, statute: Statute) -> Self {
        self.statutes.push(statute);
        self
    }

    /// Adds multiple statutes to the simulation.
    pub fn add_statutes(mut self, statutes: Vec<Statute>) -> Self {
        self.statutes.extend(statutes);
        self
    }

    /// Sets the statutes for the simulation, replacing any previously set.
    pub fn with_statutes(mut self, statutes: Vec<Statute>) -> Self {
        self.statutes = statutes;
        self
    }

    /// Adds an entity to the population.
    pub fn add_entity(mut self, entity: Box<dyn LegalEntity>) -> Self {
        self.population.push(entity);
        self
    }

    /// Adds multiple entities to the population.
    pub fn add_entities(mut self, entities: Vec<Box<dyn LegalEntity>>) -> Self {
        self.population.extend(entities);
        self
    }

    /// Sets the population for the simulation, replacing any previously set.
    pub fn with_population(mut self, population: Vec<Box<dyn LegalEntity>>) -> Self {
        self.population = population;
        self
    }

    /// Sets whether to validate the configuration before building.
    pub fn validate(mut self, validate: bool) -> Self {
        self.validate = validate;
        self
    }

    /// Returns the current number of statutes.
    pub fn statute_count(&self) -> usize {
        self.statutes.len()
    }

    /// Returns the current population size.
    pub fn population_size(&self) -> usize {
        self.population.len()
    }

    /// Validates the builder configuration.
    fn validate_config(&self) -> SimResult<()> {
        if self.statutes.is_empty() {
            return Err(SimulationError::NoStatutes);
        }

        if self.population.is_empty() {
            return Err(SimulationError::EmptyPopulation);
        }

        Ok(())
    }

    /// Builds the simulation engine.
    ///
    /// # Errors
    ///
    /// Returns an error if validation is enabled and the configuration is invalid.
    pub fn build(self) -> SimResult<SimEngine> {
        if self.validate {
            self.validate_config()?;
        }

        Ok(SimEngine::new(self.statutes, self.population))
    }

    /// Builds the simulation engine without validation.
    ///
    /// # Panics
    ///
    /// May panic if the configuration is invalid.
    #[allow(dead_code)]
    pub fn build_unchecked(self) -> SimEngine {
        SimEngine::new(self.statutes, self.population)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{BasicEntity, ComparisonOp, Condition, Effect, EffectType};

    #[test]
    fn test_builder_empty_statutes() {
        let result = SimEngineBuilder::new()
            .add_entity(Box::new(BasicEntity::new()))
            .build();

        assert!(matches!(result, Err(SimulationError::NoStatutes)));
    }

    #[test]
    fn test_builder_empty_population() {
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));

        let result = SimEngineBuilder::new().add_statute(statute).build();

        assert!(matches!(result, Err(SimulationError::EmptyPopulation)));
    }

    #[test]
    fn test_builder_valid() {
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"))
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            });

        let mut entity = BasicEntity::new();
        entity.set_attribute("age", "25".to_string());

        let result = SimEngineBuilder::new()
            .add_statute(statute)
            .add_entity(Box::new(entity))
            .build();

        assert!(result.is_ok());
        let engine = result.unwrap();
        assert_eq!(engine.statute_count(), 1);
        assert_eq!(engine.population_size(), 1);
    }

    #[test]
    fn test_builder_multiple_statutes() {
        let statute1 = Statute::new("test1", "Test 1", Effect::new(EffectType::Grant, "Test 1"));
        let statute2 = Statute::new("test2", "Test 2", Effect::new(EffectType::Grant, "Test 2"));

        let engine = SimEngineBuilder::new()
            .add_statute(statute1)
            .add_statute(statute2)
            .add_entity(Box::new(BasicEntity::new()))
            .build()
            .unwrap();

        assert_eq!(engine.statute_count(), 2);
    }

    #[test]
    fn test_builder_no_validation() {
        // Should succeed even with no statutes when validation is disabled
        let engine = SimEngineBuilder::new()
            .validate(false)
            .add_entity(Box::new(BasicEntity::new()))
            .build();

        // Should succeed because validation is disabled
        assert!(engine.is_ok());
    }
}
