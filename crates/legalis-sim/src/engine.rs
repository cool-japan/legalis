//! Simulation engine implementation.

use crate::metrics::SimulationMetrics;
use legalis_core::{BasicEntity, Condition, ComparisonOp, Effect, LegalEntity, LegalResult, Statute};
use std::sync::Arc;
use tokio::sync::mpsc;
use uuid::Uuid;

/// Result of applying a law to an entity.
#[derive(Debug, Clone)]
pub struct LawApplicationResult {
    pub agent_id: Uuid,
    pub statute_id: String,
    pub result: LegalResult<Effect>,
}

/// Simulation engine for running legal simulations.
pub struct SimEngine {
    statutes: Vec<Statute>,
    population: Vec<Arc<dyn LegalEntity>>,
}

impl SimEngine {
    /// Creates a new simulation engine.
    pub fn new(
        statutes: Vec<Statute>,
        population: Vec<Box<dyn LegalEntity>>,
    ) -> Self {
        Self {
            statutes,
            population: population.into_iter().map(Arc::from).collect(),
        }
    }

    /// Returns the number of agents in the simulation.
    pub fn population_size(&self) -> usize {
        self.population.len()
    }

    /// Returns the number of statutes being simulated.
    pub fn statute_count(&self) -> usize {
        self.statutes.len()
    }

    /// Runs the simulation and returns metrics.
    pub async fn run_simulation(&self) -> SimulationMetrics {
        let (tx, mut rx) = mpsc::channel::<LawApplicationResult>(1000);

        let mut metrics = SimulationMetrics::new();

        for agent in &self.population {
            let agent_ref = agent.clone();
            let statutes_ref = self.statutes.clone();
            let tx_clone = tx.clone();

            tokio::spawn(async move {
                for statute in statutes_ref {
                    let result = Self::apply_law(agent_ref.as_ref(), &statute);
                    let _ = tx_clone
                        .send(LawApplicationResult {
                            agent_id: agent_ref.id(),
                            statute_id: statute.id.clone(),
                            result,
                        })
                        .await;
                }
            });
        }

        // Drop sender to allow receiver to complete
        drop(tx);

        // Collect results
        while let Some(result) = rx.recv().await {
            metrics.record_result(&result);
        }

        metrics
    }

    /// Applies a single law to an entity and returns the result.
    pub fn apply_law(agent: &dyn LegalEntity, law: &Statute) -> LegalResult<Effect> {
        // Check all preconditions
        for condition in &law.preconditions {
            match Self::evaluate_condition(agent, condition) {
                ConditionResult::True => continue,
                ConditionResult::False => {
                    return LegalResult::Void {
                        reason: format!("Precondition not met: {condition:?}"),
                    };
                }
                ConditionResult::Indeterminate(reason) => {
                    return LegalResult::JudicialDiscretion {
                        issue: reason,
                        context_id: agent.id(),
                        narrative_hint: law.discretion_logic.clone(),
                    };
                }
            }
        }

        // If discretion logic is specified, flag for human review
        if law.discretion_logic.is_some() {
            return LegalResult::JudicialDiscretion {
                issue: "Discretionary review required".to_string(),
                context_id: agent.id(),
                narrative_hint: law.discretion_logic.clone(),
            };
        }

        // All conditions met, apply effect
        LegalResult::Deterministic(law.effect.clone())
    }

    fn evaluate_condition(agent: &dyn LegalEntity, condition: &Condition) -> ConditionResult {
        match condition {
            Condition::Age { operator, value } => {
                match agent.get_attribute("age").and_then(|v| v.parse::<u32>().ok()) {
                    Some(age) => {
                        let result = match operator {
                            ComparisonOp::Equal => age == *value,
                            ComparisonOp::NotEqual => age != *value,
                            ComparisonOp::GreaterThan => age > *value,
                            ComparisonOp::GreaterOrEqual => age >= *value,
                            ComparisonOp::LessThan => age < *value,
                            ComparisonOp::LessOrEqual => age <= *value,
                        };
                        if result {
                            ConditionResult::True
                        } else {
                            ConditionResult::False
                        }
                    }
                    None => ConditionResult::Indeterminate("Age attribute not found or invalid".to_string()),
                }
            }
            Condition::Income { operator, value } => {
                match agent.get_attribute("income").and_then(|v| v.parse::<u64>().ok()) {
                    Some(income) => {
                        let result = match operator {
                            ComparisonOp::Equal => income == *value,
                            ComparisonOp::NotEqual => income != *value,
                            ComparisonOp::GreaterThan => income > *value,
                            ComparisonOp::GreaterOrEqual => income >= *value,
                            ComparisonOp::LessThan => income < *value,
                            ComparisonOp::LessOrEqual => income <= *value,
                        };
                        if result {
                            ConditionResult::True
                        } else {
                            ConditionResult::False
                        }
                    }
                    None => ConditionResult::Indeterminate("Income attribute not found or invalid".to_string()),
                }
            }
            Condition::HasAttribute { key } => {
                if agent.get_attribute(key).is_some() {
                    ConditionResult::True
                } else {
                    ConditionResult::False
                }
            }
            Condition::AttributeEquals { key, value } => {
                match agent.get_attribute(key) {
                    Some(attr_value) if attr_value == *value => ConditionResult::True,
                    Some(_) => ConditionResult::False,
                    None => ConditionResult::Indeterminate(format!("Attribute '{key}' not found")),
                }
            }
            Condition::And(left, right) => {
                match (
                    Self::evaluate_condition(agent, left),
                    Self::evaluate_condition(agent, right),
                ) {
                    (ConditionResult::True, ConditionResult::True) => ConditionResult::True,
                    (ConditionResult::False, _) | (_, ConditionResult::False) => ConditionResult::False,
                    (ConditionResult::Indeterminate(r), _) | (_, ConditionResult::Indeterminate(r)) => {
                        ConditionResult::Indeterminate(r)
                    }
                }
            }
            Condition::Or(left, right) => {
                match (
                    Self::evaluate_condition(agent, left),
                    Self::evaluate_condition(agent, right),
                ) {
                    (ConditionResult::True, _) | (_, ConditionResult::True) => ConditionResult::True,
                    (ConditionResult::False, ConditionResult::False) => ConditionResult::False,
                    (ConditionResult::Indeterminate(r), _) | (_, ConditionResult::Indeterminate(r)) => {
                        ConditionResult::Indeterminate(r)
                    }
                }
            }
            Condition::Not(inner) => match Self::evaluate_condition(agent, inner) {
                ConditionResult::True => ConditionResult::False,
                ConditionResult::False => ConditionResult::True,
                ConditionResult::Indeterminate(r) => ConditionResult::Indeterminate(r),
            },
            Condition::Custom { description } => {
                ConditionResult::Indeterminate(format!("Custom condition requires evaluation: {description}"))
            }
        }
    }
}

/// Result of evaluating a condition.
#[derive(Debug, Clone)]
enum ConditionResult {
    True,
    False,
    Indeterminate(String),
}

/// Builder for creating test populations.
pub struct PopulationBuilder {
    entities: Vec<Box<dyn LegalEntity>>,
}

impl PopulationBuilder {
    /// Creates a new population builder.
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    /// Adds an entity to the population.
    pub fn add_entity(mut self, entity: impl LegalEntity + 'static) -> Self {
        self.entities.push(Box::new(entity));
        self
    }

    /// Generates a random population with the given size.
    pub fn generate_random(mut self, count: usize) -> Self {
        for _ in 0..count {
            let mut entity = BasicEntity::new();
            let age = (rand_simple() * 80.0) as u32 + 1;
            let income = (rand_simple() * 10_000_000.0) as u64;

            entity.set_attribute("age", age.to_string());
            entity.set_attribute("income", income.to_string());

            self.entities.push(Box::new(entity));
        }
        self
    }

    /// Builds the population.
    pub fn build(self) -> Vec<Box<dyn LegalEntity>> {
        self.entities
    }
}

impl Default for PopulationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple random number generator (0.0 to 1.0).
fn rand_simple() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    static mut SEED: u64 = 0;
    unsafe {
        if SEED == 0 {
            SEED = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64;
        }
        SEED = SEED.wrapping_mul(6364136223846793005).wrapping_add(1);
        (SEED >> 33) as f64 / (1u64 << 31) as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType};

    #[tokio::test]
    async fn test_sim_engine_basic() {
        let statute = Statute::new(
            "adult-rights",
            "Adult Rights",
            Effect::new(EffectType::Grant, "Full legal capacity"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let mut adult = BasicEntity::new();
        adult.set_attribute("age", "25".to_string());

        let mut minor = BasicEntity::new();
        minor.set_attribute("age", "15".to_string());

        let population: Vec<Box<dyn LegalEntity>> = vec![Box::new(adult), Box::new(minor)];

        let engine = SimEngine::new(vec![statute], population);
        let metrics = engine.run_simulation().await;

        assert_eq!(metrics.total_applications, 2);
        assert_eq!(metrics.deterministic_count, 1);
    }

    #[test]
    fn test_condition_evaluation() {
        let mut entity = BasicEntity::new();
        entity.set_attribute("age", "25".to_string());
        entity.set_attribute("income", "5000000".to_string());

        let statute = Statute::new(
            "test",
            "Test",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let result = SimEngine::apply_law(&entity, &statute);
        assert!(result.is_deterministic());
    }

    #[test]
    fn test_population_builder() {
        let population = PopulationBuilder::new()
            .generate_random(100)
            .build();

        assert_eq!(population.len(), 100);
    }
}
