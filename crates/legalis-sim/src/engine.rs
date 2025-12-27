//! Simulation engine implementation.

use crate::metrics::SimulationMetrics;
use legalis_core::{
    BasicEntity, ComparisonOp, Condition, Effect, LegalEntity, LegalResult, Statute,
};
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
    pub fn new(statutes: Vec<Statute>, population: Vec<Box<dyn LegalEntity>>) -> Self {
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
                match agent
                    .get_attribute("age")
                    .and_then(|v| v.parse::<u32>().ok())
                {
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
                    None => ConditionResult::Indeterminate(
                        "Age attribute not found or invalid".to_string(),
                    ),
                }
            }
            Condition::Income { operator, value } => {
                match agent
                    .get_attribute("income")
                    .and_then(|v| v.parse::<u64>().ok())
                {
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
                    None => ConditionResult::Indeterminate(
                        "Income attribute not found or invalid".to_string(),
                    ),
                }
            }
            Condition::HasAttribute { key } => {
                if agent.get_attribute(key).is_some() {
                    ConditionResult::True
                } else {
                    ConditionResult::False
                }
            }
            Condition::AttributeEquals { key, value } => match agent.get_attribute(key) {
                Some(attr_value) if attr_value == *value => ConditionResult::True,
                Some(_) => ConditionResult::False,
                None => ConditionResult::Indeterminate(format!("Attribute '{key}' not found")),
            },
            Condition::And(left, right) => {
                match (
                    Self::evaluate_condition(agent, left),
                    Self::evaluate_condition(agent, right),
                ) {
                    (ConditionResult::True, ConditionResult::True) => ConditionResult::True,
                    (ConditionResult::False, _) | (_, ConditionResult::False) => {
                        ConditionResult::False
                    }
                    (ConditionResult::Indeterminate(r), _)
                    | (_, ConditionResult::Indeterminate(r)) => ConditionResult::Indeterminate(r),
                }
            }
            Condition::Or(left, right) => {
                match (
                    Self::evaluate_condition(agent, left),
                    Self::evaluate_condition(agent, right),
                ) {
                    (ConditionResult::True, _) | (_, ConditionResult::True) => {
                        ConditionResult::True
                    }
                    (ConditionResult::False, ConditionResult::False) => ConditionResult::False,
                    (ConditionResult::Indeterminate(r), _)
                    | (_, ConditionResult::Indeterminate(r)) => ConditionResult::Indeterminate(r),
                }
            }
            Condition::Not(inner) => match Self::evaluate_condition(agent, inner) {
                ConditionResult::True => ConditionResult::False,
                ConditionResult::False => ConditionResult::True,
                ConditionResult::Indeterminate(r) => ConditionResult::Indeterminate(r),
            },
            Condition::Custom { description } => ConditionResult::Indeterminate(format!(
                "Custom condition requires evaluation: {description}"
            )),
            Condition::DateRange { start, end } => {
                match agent
                    .get_attribute("current_date")
                    .and_then(|v| chrono::NaiveDate::parse_from_str(&v, "%Y-%m-%d").ok())
                {
                    Some(date) => {
                        let after_start = start.is_none_or(|s| date >= s);
                        let before_end = end.is_none_or(|e| date <= e);
                        if after_start && before_end {
                            ConditionResult::True
                        } else {
                            ConditionResult::False
                        }
                    }
                    None => ConditionResult::Indeterminate(
                        "Date attribute not found or invalid".to_string(),
                    ),
                }
            }
            Condition::Geographic { region_id, .. } => match agent.get_attribute("region") {
                Some(agent_region) if agent_region == *region_id => ConditionResult::True,
                Some(_) => ConditionResult::False,
                None => ConditionResult::Indeterminate("Region attribute not found".to_string()),
            },
            Condition::EntityRelationship {
                relationship_type,
                target_entity_id,
            } => {
                let rel_key = format!("relationship_{:?}", relationship_type).to_lowercase();
                match agent.get_attribute(&rel_key) {
                    Some(rel_value) => match target_entity_id {
                        Some(target) if rel_value == *target => ConditionResult::True,
                        Some(_) => ConditionResult::False,
                        None => ConditionResult::True,
                    },
                    None => ConditionResult::Indeterminate(format!(
                        "Relationship {:?} not found",
                        relationship_type
                    )),
                }
            }
            Condition::ResidencyDuration { operator, months } => {
                match agent
                    .get_attribute("residency_months")
                    .and_then(|v| v.parse::<u32>().ok())
                {
                    Some(residency) => {
                        let result = match operator {
                            ComparisonOp::Equal => residency == *months,
                            ComparisonOp::NotEqual => residency != *months,
                            ComparisonOp::GreaterThan => residency > *months,
                            ComparisonOp::GreaterOrEqual => residency >= *months,
                            ComparisonOp::LessThan => residency < *months,
                            ComparisonOp::LessOrEqual => residency <= *months,
                        };
                        if result {
                            ConditionResult::True
                        } else {
                            ConditionResult::False
                        }
                    }
                    None => ConditionResult::Indeterminate(
                        "Residency duration not found or invalid".to_string(),
                    ),
                }
            }
            Condition::Duration {
                operator,
                value,
                unit,
            } => {
                // Convert unit to attribute key (e.g., "duration_days", "duration_months")
                let attr_key = match unit {
                    legalis_core::DurationUnit::Days => "duration_days",
                    legalis_core::DurationUnit::Weeks => "duration_weeks",
                    legalis_core::DurationUnit::Months => "duration_months",
                    legalis_core::DurationUnit::Years => "duration_years",
                };
                match agent
                    .get_attribute(attr_key)
                    .and_then(|v| v.parse::<u32>().ok())
                {
                    Some(duration) => {
                        let result = match operator {
                            ComparisonOp::Equal => duration == *value,
                            ComparisonOp::NotEqual => duration != *value,
                            ComparisonOp::GreaterThan => duration > *value,
                            ComparisonOp::GreaterOrEqual => duration >= *value,
                            ComparisonOp::LessThan => duration < *value,
                            ComparisonOp::LessOrEqual => duration <= *value,
                        };
                        if result {
                            ConditionResult::True
                        } else {
                            ConditionResult::False
                        }
                    }
                    None => ConditionResult::Indeterminate(format!(
                        "Duration attribute '{}' not found or invalid",
                        attr_key
                    )),
                }
            }
            Condition::Percentage {
                operator,
                value,
                context,
            } => {
                // Use context as attribute key (e.g., "ownership_percentage")
                let attr_key = format!("{}_percentage", context);
                match agent
                    .get_attribute(&attr_key)
                    .and_then(|v| v.parse::<u32>().ok())
                {
                    Some(percentage) => {
                        let result = match operator {
                            ComparisonOp::Equal => percentage == *value,
                            ComparisonOp::NotEqual => percentage != *value,
                            ComparisonOp::GreaterThan => percentage > *value,
                            ComparisonOp::GreaterOrEqual => percentage >= *value,
                            ComparisonOp::LessThan => percentage < *value,
                            ComparisonOp::LessOrEqual => percentage <= *value,
                        };
                        if result {
                            ConditionResult::True
                        } else {
                            ConditionResult::False
                        }
                    }
                    None => ConditionResult::Indeterminate(format!(
                        "Percentage attribute '{}' not found or invalid",
                        attr_key
                    )),
                }
            }
            Condition::SetMembership {
                attribute,
                values,
                negated,
            } => match agent.get_attribute(attribute) {
                Some(attr_value) => {
                    let is_member = values.iter().any(|v| v == &attr_value);
                    let result = if *negated { !is_member } else { is_member };
                    if result {
                        ConditionResult::True
                    } else {
                        ConditionResult::False
                    }
                }
                None => ConditionResult::Indeterminate(format!(
                    "Attribute '{}' not found for set membership check",
                    attribute
                )),
            },
            Condition::Pattern {
                attribute,
                pattern,
                negated,
            } => match agent.get_attribute(attribute) {
                Some(attr_value) => {
                    // Simple pattern matching - check if pattern is contained in value
                    // For full regex support, would need regex crate
                    let matches = attr_value.contains(pattern);
                    let result = if *negated { !matches } else { matches };
                    if result {
                        ConditionResult::True
                    } else {
                        ConditionResult::False
                    }
                }
                None => ConditionResult::Indeterminate(format!(
                    "Attribute '{}' not found for pattern check",
                    attribute
                )),
            },
            Condition::Calculation { formula, .. } => {
                // Calculation conditions require external evaluation
                ConditionResult::Indeterminate(format!(
                    "Calculation condition requires evaluation: {formula}"
                ))
            }
            Condition::Composite {
                conditions,
                threshold,
            } => {
                // Evaluate all sub-conditions and sum weights of satisfied ones
                let mut score = 0.0;
                for (weight, condition) in conditions {
                    match Self::evaluate_condition(agent, condition) {
                        ConditionResult::True => score += weight,
                        ConditionResult::False => {}
                        ConditionResult::Indeterminate(_) => {
                            // For composite, treat indeterminate as false
                        }
                    }
                }
                if score >= *threshold {
                    ConditionResult::True
                } else {
                    ConditionResult::False
                }
            }
            Condition::Threshold {
                attributes,
                operator,
                value,
            } => {
                // Sum all attribute values (with multipliers)
                let mut total = 0.0;
                let mut has_error = false;
                for (attr_name, multiplier) in attributes {
                    match agent
                        .get_attribute(attr_name)
                        .and_then(|v| v.parse::<f64>().ok())
                    {
                        Some(attr_value) => total += attr_value * multiplier,
                        None => {
                            has_error = true;
                            break;
                        }
                    }
                }
                if has_error {
                    ConditionResult::Indeterminate("One or more attributes not found".to_string())
                } else {
                    let result = match operator {
                        ComparisonOp::Equal => (total - value).abs() < 1e-6,
                        ComparisonOp::NotEqual => (total - value).abs() >= 1e-6,
                        ComparisonOp::GreaterThan => total > *value,
                        ComparisonOp::GreaterOrEqual => total >= *value,
                        ComparisonOp::LessThan => total < *value,
                        ComparisonOp::LessOrEqual => total <= *value,
                    };
                    if result {
                        ConditionResult::True
                    } else {
                        ConditionResult::False
                    }
                }
            }
            Condition::Fuzzy {
                attribute,
                membership_points,
                min_membership,
            } => {
                match agent
                    .get_attribute(attribute)
                    .and_then(|v| v.parse::<f64>().ok())
                {
                    Some(value) => {
                        // Linear interpolation between membership points
                        let membership = if membership_points.is_empty() {
                            0.0
                        } else if membership_points.len() == 1 {
                            if (value - membership_points[0].0).abs() < 1e-6 {
                                membership_points[0].1
                            } else {
                                0.0
                            }
                        } else {
                            // Find surrounding points
                            let mut lower = None;
                            let mut upper = None;
                            for point in membership_points {
                                if point.0 <= value {
                                    lower = Some(point);
                                }
                                if point.0 >= value && upper.is_none() {
                                    upper = Some(point);
                                }
                            }
                            match (lower, upper) {
                                (Some(l), Some(u)) if (l.0 - u.0).abs() < 1e-6 => l.1,
                                (Some(l), Some(u)) => {
                                    // Linear interpolation
                                    let t = (value - l.0) / (u.0 - l.0);
                                    l.1 + t * (u.1 - l.1)
                                }
                                (Some(l), None) => l.1,
                                (None, Some(u)) => u.1,
                                (None, None) => 0.0,
                            }
                        };
                        if membership >= *min_membership {
                            ConditionResult::True
                        } else {
                            ConditionResult::False
                        }
                    }
                    None => ConditionResult::Indeterminate(format!(
                        "Attribute '{}' not found for fuzzy evaluation",
                        attribute
                    )),
                }
            }
            Condition::Probabilistic {
                condition,
                probability: _,
                threshold: _,
            } => {
                // For simulation, evaluate the base condition deterministically
                // A full implementation would use probability and threshold for uncertain evaluation
                Self::evaluate_condition(agent, condition)
            }
            Condition::Temporal {
                base_value,
                reference_time,
                rate,
                operator,
                target_value,
            } => {
                // Get current time from agent attributes
                match agent
                    .get_attribute("current_timestamp")
                    .and_then(|v| v.parse::<i64>().ok())
                {
                    Some(current_time) => {
                        let time_elapsed = (current_time - reference_time) as f64;
                        let current_value = base_value * (1.0 + rate).powf(time_elapsed);
                        let result = match operator {
                            ComparisonOp::Equal => (current_value - target_value).abs() < 1e-6,
                            ComparisonOp::NotEqual => (current_value - target_value).abs() >= 1e-6,
                            ComparisonOp::GreaterThan => current_value > *target_value,
                            ComparisonOp::GreaterOrEqual => current_value >= *target_value,
                            ComparisonOp::LessThan => current_value < *target_value,
                            ComparisonOp::LessOrEqual => current_value <= *target_value,
                        };
                        if result {
                            ConditionResult::True
                        } else {
                            ConditionResult::False
                        }
                    }
                    None => ConditionResult::Indeterminate(
                        "Current timestamp not found for temporal evaluation".to_string(),
                    ),
                }
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

/// Relationship-based condition evaluator.
pub struct RelationshipConditions {
    graph: crate::relationships::RelationshipGraph,
}

impl RelationshipConditions {
    /// Creates a new relationship condition evaluator.
    pub fn new(graph: crate::relationships::RelationshipGraph) -> Self {
        Self { graph }
    }

    /// Checks if an entity has a specific relationship type.
    pub fn has_relationship(
        &self,
        entity_id: uuid::Uuid,
        rel_type: crate::relationships::RelationshipType,
    ) -> bool {
        !self.graph.get_relationships(entity_id, rel_type).is_empty()
    }

    /// Checks if an entity has at least N relationships of a given type.
    pub fn has_min_relationships(
        &self,
        entity_id: uuid::Uuid,
        rel_type: crate::relationships::RelationshipType,
        min_count: usize,
    ) -> bool {
        self.graph.get_relationships(entity_id, rel_type).len() >= min_count
    }

    /// Checks if an entity has at most N relationships of a given type.
    pub fn has_max_relationships(
        &self,
        entity_id: uuid::Uuid,
        rel_type: crate::relationships::RelationshipType,
        max_count: usize,
    ) -> bool {
        self.graph.get_relationships(entity_id, rel_type).len() <= max_count
    }

    /// Checks if an entity is related to a specific target entity.
    pub fn is_related_to(
        &self,
        entity_id: uuid::Uuid,
        target_id: uuid::Uuid,
        rel_type: crate::relationships::RelationshipType,
    ) -> bool {
        self.graph.has_relationship(entity_id, target_id, rel_type)
    }

    /// Checks if an entity is connected to another entity through any path (within max_depth hops).
    pub fn is_connected_to(
        &self,
        entity_id: uuid::Uuid,
        target_id: uuid::Uuid,
        max_depth: usize,
    ) -> bool {
        self.graph
            .find_connected(entity_id, max_depth)
            .contains(&target_id)
    }

    /// Gets relationship count for a specific type.
    pub fn count_relationships(
        &self,
        entity_id: uuid::Uuid,
        rel_type: crate::relationships::RelationshipType,
    ) -> usize {
        self.graph.get_relationships(entity_id, rel_type).len()
    }
}

/// Contract-based condition evaluator.
pub struct ContractConditions {
    registry: crate::relationships::ContractRegistry,
}

impl ContractConditions {
    /// Creates a new contract condition evaluator.
    pub fn new(registry: crate::relationships::ContractRegistry) -> Self {
        Self { registry }
    }

    /// Checks if an entity has active contracts.
    pub fn has_active_contracts(&self, entity_id: uuid::Uuid) -> bool {
        !self.registry.get_active_contracts(entity_id).is_empty()
    }

    /// Checks if an entity has at least N active contracts.
    pub fn has_min_active_contracts(&self, entity_id: uuid::Uuid, min_count: usize) -> bool {
        self.registry.get_active_contracts(entity_id).len() >= min_count
    }

    /// Checks if an entity has a contract of a specific type.
    pub fn has_contract_type(
        &self,
        entity_id: uuid::Uuid,
        contract_type: crate::relationships::ContractType,
    ) -> bool {
        self.registry.get_by_party(entity_id).iter().any(|c| {
            c.contract_type == contract_type
                && c.status == crate::relationships::ContractStatus::Active
        })
    }

    /// Checks if total contract value exceeds a threshold.
    pub fn contract_value_exceeds(&self, entity_id: uuid::Uuid, threshold: f64) -> bool {
        self.registry.total_value(entity_id) > threshold
    }

    /// Gets count of active contracts for an entity.
    pub fn count_active_contracts(&self, entity_id: uuid::Uuid) -> usize {
        self.registry.get_active_contracts(entity_id).len()
    }
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

        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"))
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            });

        let result = SimEngine::apply_law(&entity, &statute);
        assert!(result.is_deterministic());
    }

    #[test]
    fn test_population_builder() {
        let population = PopulationBuilder::new().generate_random(100).build();

        assert_eq!(population.len(), 100);
    }
}
