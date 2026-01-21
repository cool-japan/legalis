//! Scenario Simulation with Digital Twins
//!
//! This module provides scenario simulation capabilities using digital twins,
//! allowing "what-if" analysis of legal outcomes under different conditions.

use crate::digital_twin::{EntityState, LegalDigitalTwin};
use crate::{Effect, Statute};
use std::collections::HashMap;

/// Scenario for simulation
///
/// # Example
///
/// ```
/// use legalis_core::scenario_simulation::{Scenario, SimulationParameter};
/// use legalis_core::digital_twin::LegalDigitalTwin;
///
/// let mut scenario = Scenario::new("tax-increase");
/// scenario.add_parameter(SimulationParameter::Attribute {
///     key: "income".to_string(),
///     value: "100000".to_string(),
/// });
///
/// assert_eq!(scenario.parameter_count(), 1);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Scenario {
    /// Scenario ID
    pub id: String,
    /// Scenario description
    pub description: String,
    /// Parameters to modify
    parameters: Vec<SimulationParameter>,
    /// Statutes to apply
    statutes: Vec<Statute>,
    /// Metadata
    metadata: HashMap<String, String>,
}

impl Scenario {
    /// Create a new scenario
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            description: String::new(),
            parameters: Vec::new(),
            statutes: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Set description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Add a simulation parameter
    pub fn add_parameter(&mut self, param: SimulationParameter) {
        self.parameters.push(param);
    }

    /// Add a statute to apply
    pub fn add_statute(&mut self, statute: Statute) {
        self.statutes.push(statute);
    }

    /// Get parameters
    pub fn parameters(&self) -> &[SimulationParameter] {
        &self.parameters
    }

    /// Get statutes
    pub fn statutes(&self) -> &[Statute] {
        &self.statutes
    }

    /// Get parameter count
    pub fn parameter_count(&self) -> usize {
        self.parameters.len()
    }

    /// Set metadata
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Get metadata
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
}

/// Simulation parameter that modifies digital twin state
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SimulationParameter {
    /// Modify an attribute
    Attribute { key: String, value: String },
    /// Modify a relationship
    Relationship {
        relationship_type: String,
        target_id: String,
    },
    /// Modify timestamp
    Timestamp(u64),
}

/// Result of a scenario simulation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SimulationResult {
    /// Scenario ID
    pub scenario_id: String,
    /// Entity ID that was simulated
    pub entity_id: String,
    /// Applied effects
    pub effects: Vec<Effect>,
    /// Final state after simulation
    pub final_state: EntityState,
    /// Success flag
    pub success: bool,
    /// Error message if failed
    pub error: Option<String>,
    /// Timestamp of simulation
    pub timestamp: u64,
}

impl SimulationResult {
    /// Create a successful result
    pub fn success(
        scenario_id: String,
        entity_id: String,
        effects: Vec<Effect>,
        final_state: EntityState,
    ) -> Self {
        Self {
            scenario_id,
            entity_id,
            effects,
            final_state,
            success: true,
            error: None,
            timestamp: current_timestamp(),
        }
    }

    /// Create a failed result
    pub fn failure(scenario_id: String, entity_id: String, error: String) -> Self {
        Self {
            scenario_id,
            entity_id,
            effects: Vec::new(),
            final_state: EntityState::new(),
            success: false,
            error: Some(error),
            timestamp: current_timestamp(),
        }
    }

    /// Get effect count
    pub fn effect_count(&self) -> usize {
        self.effects.len()
    }
}

/// Scenario simulator for digital twins
///
/// # Example
///
/// ```
/// use legalis_core::scenario_simulation::{ScenarioSimulator, Scenario, SimulationParameter};
/// use legalis_core::digital_twin::LegalDigitalTwin;
///
/// let mut simulator = ScenarioSimulator::new();
///
/// let twin = LegalDigitalTwin::new("entity-001", "Individual");
/// simulator.add_twin(twin);
///
/// let mut scenario = Scenario::new("test-scenario");
/// scenario.add_parameter(SimulationParameter::Attribute {
///     key: "income".to_string(),
///     value: "50000".to_string(),
/// });
///
/// let result = simulator.simulate("entity-001", &scenario).unwrap();
/// assert!(result.success);
/// ```
pub struct ScenarioSimulator {
    twins: HashMap<String, LegalDigitalTwin>,
    results: Vec<SimulationResult>,
}

impl ScenarioSimulator {
    /// Create a new scenario simulator
    pub fn new() -> Self {
        Self {
            twins: HashMap::new(),
            results: Vec::new(),
        }
    }

    /// Add a digital twin
    pub fn add_twin(&mut self, twin: LegalDigitalTwin) {
        self.twins.insert(twin.id.clone(), twin);
    }

    /// Simulate a scenario for an entity
    pub fn simulate(
        &mut self,
        entity_id: &str,
        scenario: &Scenario,
    ) -> Result<SimulationResult, SimulationError> {
        // Get the twin
        let twin = self
            .twins
            .get(entity_id)
            .ok_or_else(|| SimulationError::TwinNotFound(entity_id.to_string()))?;

        // Clone the current state
        let mut simulated_state = twin.get_current_state().clone();

        // Apply parameters
        for param in scenario.parameters() {
            match param {
                SimulationParameter::Attribute { key, value } => {
                    simulated_state.set_attribute(key.clone(), value.clone());
                }
                SimulationParameter::Relationship {
                    relationship_type,
                    target_id,
                } => {
                    simulated_state.add_relationship(relationship_type.clone(), target_id.clone());
                }
                SimulationParameter::Timestamp(ts) => {
                    // Timestamp modification would affect time-based conditions
                    simulated_state.timestamp = *ts;
                }
            }
        }

        // Apply statutes and collect effects
        let effects: Vec<Effect> = scenario
            .statutes()
            .iter()
            .map(|statute| statute.effect.clone())
            .collect();

        let result = SimulationResult::success(
            scenario.id.clone(),
            entity_id.to_string(),
            effects,
            simulated_state,
        );

        self.results.push(result.clone());
        Ok(result)
    }

    /// Run multiple scenarios in batch
    pub fn batch_simulate(
        &mut self,
        entity_id: &str,
        scenarios: &[Scenario],
    ) -> Vec<Result<SimulationResult, SimulationError>> {
        scenarios
            .iter()
            .map(|scenario| self.simulate(entity_id, scenario))
            .collect()
    }

    /// Get all simulation results
    pub fn get_results(&self) -> &[SimulationResult] {
        &self.results
    }

    /// Get results for a specific entity
    pub fn get_results_for_entity(&self, entity_id: &str) -> Vec<&SimulationResult> {
        self.results
            .iter()
            .filter(|r| r.entity_id == entity_id)
            .collect()
    }

    /// Clear all results
    pub fn clear_results(&mut self) {
        self.results.clear();
    }

    /// Get twin count
    pub fn twin_count(&self) -> usize {
        self.twins.len()
    }

    /// Get result count
    pub fn result_count(&self) -> usize {
        self.results.len()
    }
}

impl Default for ScenarioSimulator {
    fn default() -> Self {
        Self::new()
    }
}

/// Comparison of multiple simulation results
#[derive(Debug)]
pub struct SimulationComparison {
    /// Results being compared
    pub results: Vec<SimulationResult>,
    /// Differences found
    pub differences: Vec<StateDifference>,
}

impl SimulationComparison {
    /// Compare multiple simulation results
    pub fn compare(results: Vec<SimulationResult>) -> Self {
        let mut differences = Vec::new();

        if results.len() < 2 {
            return Self {
                results,
                differences,
            };
        }

        // Compare each pair of consecutive results
        for i in 0..results.len() - 1 {
            let state1 = &results[i].final_state;
            let state2 = &results[i + 1].final_state;

            // Compare attributes
            for (key, value1) in state1.all_attributes() {
                if let Some(value2) = state2.get_attribute(key)
                    && value1 != value2
                {
                    differences.push(StateDifference::AttributeChanged {
                        key: key.clone(),
                        old_value: value1.clone(),
                        new_value: value2.clone(),
                    });
                }
            }
        }

        Self {
            results,
            differences,
        }
    }

    /// Get difference count
    pub fn difference_count(&self) -> usize {
        self.differences.len()
    }
}

/// Difference between two states
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StateDifference {
    /// Attribute value changed
    AttributeChanged {
        key: String,
        old_value: String,
        new_value: String,
    },
    /// Relationship added
    RelationshipAdded {
        relationship_type: String,
        target_id: String,
    },
    /// Relationship removed
    RelationshipRemoved {
        relationship_type: String,
        target_id: String,
    },
}

/// Helper function to get current timestamp
fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Simulation errors
#[derive(Debug, Clone, thiserror::Error)]
pub enum SimulationError {
    #[error("Twin not found: {0}")]
    TwinNotFound(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Simulation failed: {0}")]
    SimulationFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EffectType;

    #[test]
    fn test_scenario_creation() {
        let scenario = Scenario::new("test-scenario").with_description("A test scenario");

        assert_eq!(scenario.id, "test-scenario");
        assert_eq!(scenario.description, "A test scenario");
    }

    #[test]
    fn test_add_parameters() {
        let mut scenario = Scenario::new("test");

        scenario.add_parameter(SimulationParameter::Attribute {
            key: "age".to_string(),
            value: "25".to_string(),
        });

        assert_eq!(scenario.parameter_count(), 1);
    }

    #[test]
    fn test_simulator() {
        let mut simulator = ScenarioSimulator::new();

        let twin = LegalDigitalTwin::new("entity-001", "Individual");
        simulator.add_twin(twin);

        assert_eq!(simulator.twin_count(), 1);
    }

    #[test]
    fn test_simulation() {
        let mut simulator = ScenarioSimulator::new();

        let twin = LegalDigitalTwin::new("entity-001", "Individual");
        simulator.add_twin(twin);

        let mut scenario = Scenario::new("income-test");
        scenario.add_parameter(SimulationParameter::Attribute {
            key: "income".to_string(),
            value: "50000".to_string(),
        });

        let result = simulator.simulate("entity-001", &scenario).unwrap();

        assert!(result.success);
        assert_eq!(result.entity_id, "entity-001");
        assert_eq!(
            result.final_state.get_attribute("income"),
            Some(&"50000".to_string())
        );
    }

    #[test]
    fn test_batch_simulation() {
        let mut simulator = ScenarioSimulator::new();

        let twin = LegalDigitalTwin::new("entity-001", "Individual");
        simulator.add_twin(twin);

        let scenario1 = Scenario::new("scenario-1");
        let scenario2 = Scenario::new("scenario-2");

        let results = simulator.batch_simulate("entity-001", &[scenario1, scenario2]);

        assert_eq!(results.len(), 2);
        assert!(results[0].is_ok());
        assert!(results[1].is_ok());
    }

    #[test]
    fn test_simulation_with_statute() {
        let mut simulator = ScenarioSimulator::new();

        let twin = LegalDigitalTwin::new("entity-001", "Individual");
        simulator.add_twin(twin);

        let mut scenario = Scenario::new("tax-scenario");
        let statute = Statute::new(
            "tax-001",
            "Tax Statute",
            Effect::new(EffectType::Grant, "Tax benefit"),
        );
        scenario.add_statute(statute);

        let result = simulator.simulate("entity-001", &scenario).unwrap();

        assert!(result.success);
        assert_eq!(result.effect_count(), 1);
    }

    #[test]
    fn test_simulation_result_filtering() {
        let mut simulator = ScenarioSimulator::new();

        let twin1 = LegalDigitalTwin::new("entity-001", "Individual");
        let twin2 = LegalDigitalTwin::new("entity-002", "Individual");

        simulator.add_twin(twin1);
        simulator.add_twin(twin2);

        let scenario = Scenario::new("test");

        simulator.simulate("entity-001", &scenario).unwrap();
        simulator.simulate("entity-002", &scenario).unwrap();

        let results = simulator.get_results_for_entity("entity-001");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_simulation_comparison() {
        let result1 = SimulationResult::success(
            "scenario-1".to_string(),
            "entity-001".to_string(),
            vec![],
            EntityState::new(),
        );

        let result2 = SimulationResult::success(
            "scenario-2".to_string(),
            "entity-001".to_string(),
            vec![],
            EntityState::new(),
        );

        let comparison = SimulationComparison::compare(vec![result1, result2]);

        assert_eq!(comparison.results.len(), 2);
    }

    #[test]
    fn test_twin_not_found() {
        let mut simulator = ScenarioSimulator::new();
        let scenario = Scenario::new("test");

        let result = simulator.simulate("nonexistent", &scenario);

        assert!(result.is_err());
    }
}
