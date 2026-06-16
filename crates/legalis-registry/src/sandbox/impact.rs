//! Impact prediction sandbox.
//!
//! Given a candidate statute and a sample of (synthetic or supplied) entities,
//! the [`ImpactPredictionSandbox`] evaluates the statute's preconditions
//! against each entity using the `legalis-core` condition engine and aggregates
//! the predicted legal effects into an [`ImpactReport`]. The candidate statute
//! is normally staged inside a [`SandboxEnvironment`] first, so prediction
//! never touches the production registry.

use std::collections::{BTreeMap, HashMap};

use chrono::{DateTime, Utc};
use legalis_core::{AttributeBasedContext, Effect, EffectType, Statute};
use serde::{Deserialize, Serialize};

use super::environment::SandboxEnvironment;
use crate::{RegistryError, RegistryResult};

/// Returns a stable label for an effect type, used for serializable breakdowns.
fn effect_type_label(effect_type: &EffectType) -> &'static str {
    match effect_type {
        EffectType::Grant => "Grant",
        EffectType::Revoke => "Revoke",
        EffectType::Obligation => "Obligation",
        EffectType::Prohibition => "Prohibition",
        EffectType::MonetaryTransfer => "MonetaryTransfer",
        EffectType::StatusChange => "StatusChange",
        EffectType::Custom => "Custom",
    }
}

/// A synthetic or supplied entity used to probe a statute's impact.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SyntheticEntity {
    /// Unique identifier for the entity.
    pub id: String,
    /// Attribute key/value pairs (e.g. `age`, `income`, `region`).
    pub attributes: HashMap<String, String>,
}

impl SyntheticEntity {
    /// Creates a new entity with no attributes.
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            attributes: HashMap::new(),
        }
    }

    /// Adds an attribute and returns the updated entity (builder style).
    #[must_use]
    pub fn with_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }

    /// Returns an attribute value by key.
    #[must_use]
    pub fn attribute(&self, key: &str) -> Option<&String> {
        self.attributes.get(key)
    }

    /// Parses an attribute value as a floating-point number.
    #[must_use]
    pub fn numeric_attribute(&self, key: &str) -> Option<f64> {
        self.attributes.get(key).and_then(|v| v.parse::<f64>().ok())
    }

    /// Builds an evaluation context for `legalis-core` condition evaluation.
    fn context(&self) -> AttributeBasedContext {
        AttributeBasedContext::new(self.attributes.clone())
    }
}

/// Configurable model translating a legal effect into numeric impact magnitudes.
///
/// The model produces two quantities per affected entity:
/// - a *welfare delta*: a signed score derived from the effect type weight
///   (e.g. a `Grant` is positive while a `Prohibition` is negative);
/// - a *monetary delta*: a currency magnitude parsed from an effect parameter,
///   optionally scaled by an entity attribute (for income-proportional effects).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactModel {
    /// Effect parameter key holding a monetary amount.
    pub amount_parameter: String,
    /// Optional entity attribute used to scale the monetary amount.
    pub scaling_attribute: Option<String>,
    /// Per-effect-type welfare weights, keyed by effect label.
    pub effect_weights: BTreeMap<String, f64>,
}

impl ImpactModel {
    /// Creates a model with conventional default welfare weights.
    #[must_use]
    pub fn new() -> Self {
        Self {
            amount_parameter: "amount".to_string(),
            scaling_attribute: None,
            effect_weights: Self::default_weights(),
        }
    }

    /// Default welfare weights for each effect type.
    fn default_weights() -> BTreeMap<String, f64> {
        let mut weights = BTreeMap::new();
        weights.insert("Grant".to_string(), 1.0);
        weights.insert("MonetaryTransfer".to_string(), 1.0);
        weights.insert("StatusChange".to_string(), 0.5);
        weights.insert("Custom".to_string(), 0.0);
        weights.insert("Obligation".to_string(), -0.5);
        weights.insert("Prohibition".to_string(), -1.0);
        weights.insert("Revoke".to_string(), -1.0);
        weights
    }

    /// Sets the effect parameter key holding the monetary amount.
    #[must_use]
    pub fn with_amount_parameter(mut self, key: impl Into<String>) -> Self {
        self.amount_parameter = key.into();
        self
    }

    /// Sets an entity attribute used to scale the monetary amount.
    #[must_use]
    pub fn with_scaling_attribute(mut self, attribute: impl Into<String>) -> Self {
        self.scaling_attribute = Some(attribute.into());
        self
    }

    /// Overrides the welfare weight for a specific effect type.
    #[must_use]
    pub fn with_effect_weight(mut self, effect_type: EffectType, weight: f64) -> Self {
        self.effect_weights
            .insert(effect_type_label(&effect_type).to_string(), weight);
        self
    }

    /// Returns the welfare weight for an effect type (0.0 if unspecified).
    #[must_use]
    pub fn welfare_weight(&self, effect_type: &EffectType) -> f64 {
        self.effect_weights
            .get(effect_type_label(effect_type))
            .copied()
            .unwrap_or(0.0)
    }

    /// Estimates the monetary delta of an effect for a given entity.
    #[must_use]
    pub fn monetary_effect(&self, effect: &Effect, entity: &SyntheticEntity) -> f64 {
        let raw = effect
            .get_parameter(&self.amount_parameter)
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(0.0);
        match &self.scaling_attribute {
            Some(attribute) => match entity.numeric_attribute(attribute) {
                Some(factor) => raw * factor,
                None => raw,
            },
            None => raw,
        }
    }
}

impl Default for ImpactModel {
    fn default() -> Self {
        Self::new()
    }
}

/// The predicted effect of a candidate statute on a single entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityImpact {
    /// Entity identifier.
    pub entity_id: String,
    /// Whether all preconditions were satisfied and the statute applies.
    pub affected: bool,
    /// Whether at least one precondition could not be evaluated (missing data).
    pub indeterminate: bool,
    /// The effect type that would apply (only set when affected), as a label.
    pub effect_type: Option<String>,
    /// Estimated monetary delta (only non-zero when affected).
    pub monetary_delta: f64,
    /// Estimated welfare delta (only non-zero when affected).
    pub welfare_delta: f64,
    /// Number of preconditions satisfied.
    pub satisfied_conditions: usize,
    /// Total number of preconditions evaluated.
    pub total_conditions: usize,
}

/// Aggregated impact for a cohort grouped by an attribute value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortImpact {
    /// The grouping attribute value (or `(unset)` for entities missing it).
    pub cohort: String,
    /// Number of entities in the cohort.
    pub size: usize,
    /// Number of affected entities in the cohort.
    pub affected: usize,
    /// Fraction of the cohort affected (`affected / size`).
    pub coverage: f64,
    /// Total monetary delta across the cohort.
    pub total_monetary_delta: f64,
}

/// Aggregated prediction of a candidate statute's impact across an entity sample.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactReport {
    /// Identifier of the statute that was evaluated.
    pub statute_id: String,
    /// Number of entities in the sample.
    pub sample_size: usize,
    /// Number of entities the statute applies to.
    pub affected_count: usize,
    /// Number of entities the statute definitively does not apply to.
    pub unaffected_count: usize,
    /// Number of entities whose status could not be determined.
    pub indeterminate_count: usize,
    /// Fraction of the sample affected (`affected_count / sample_size`).
    pub coverage: f64,
    /// Affected-entity counts per effect type label.
    pub effect_breakdown: BTreeMap<String, usize>,
    /// Total monetary delta across affected entities.
    pub total_monetary_delta: f64,
    /// Mean monetary delta per affected entity.
    pub mean_monetary_delta: f64,
    /// Total welfare delta across affected entities.
    pub total_welfare_delta: f64,
    /// Mean welfare delta per affected entity.
    pub mean_welfare_delta: f64,
    /// Per-cohort impact breakdown (empty when no cohort attribute was given).
    pub cohort_breakdown: BTreeMap<String, CohortImpact>,
    /// Per-entity prediction detail.
    pub per_entity: Vec<EntityImpact>,
    /// When the report was generated.
    pub generated_at: DateTime<Utc>,
}

impl ImpactReport {
    /// Returns the fraction of the sample affected.
    #[must_use]
    pub fn affected_fraction(&self) -> f64 {
        self.coverage
    }

    /// Returns `true` when coverage meets or exceeds the given threshold.
    #[must_use]
    pub fn is_high_impact(&self, threshold: f64) -> bool {
        self.coverage >= threshold
    }

    /// Returns the effect type label affecting the most entities, if any.
    #[must_use]
    pub fn dominant_effect(&self) -> Option<&str> {
        self.effect_breakdown
            .iter()
            .max_by(|a, b| a.1.cmp(b.1).then_with(|| b.0.cmp(a.0)))
            .map(|(label, _)| label.as_str())
    }
}

/// Predicts the impact of candidate statutes against entity samples.
#[derive(Debug, Clone)]
pub struct ImpactPredictionSandbox {
    model: ImpactModel,
}

impl ImpactPredictionSandbox {
    /// Creates a prediction sandbox with the given impact model.
    #[must_use]
    pub fn new(model: ImpactModel) -> Self {
        Self { model }
    }

    /// Creates a prediction sandbox with the default impact model.
    #[must_use]
    pub fn with_default_model() -> Self {
        Self {
            model: ImpactModel::new(),
        }
    }

    /// Returns the impact model in use.
    #[must_use]
    pub fn model(&self) -> &ImpactModel {
        &self.model
    }

    /// Predicts the impact of a statute that has been staged in a sandbox.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError::StatuteNotFound`] when the statute is not
    /// visible in the sandbox's effective view.
    pub fn predict(
        &self,
        env: &SandboxEnvironment,
        statute_id: &str,
        entities: &[SyntheticEntity],
        cohort_attribute: Option<&str>,
    ) -> RegistryResult<ImpactReport> {
        let entry = env
            .effective(statute_id)
            .ok_or_else(|| RegistryError::StatuteNotFound(statute_id.to_string()))?;
        Ok(self.predict_statute(&entry.statute, entities, cohort_attribute))
    }

    /// Predicts the impact of a statute directly against an entity sample.
    #[must_use]
    pub fn predict_statute(
        &self,
        statute: &Statute,
        entities: &[SyntheticEntity],
        cohort_attribute: Option<&str>,
    ) -> ImpactReport {
        let per_entity: Vec<EntityImpact> = entities
            .iter()
            .map(|entity| self.evaluate_entity(statute, entity))
            .collect();

        let mut affected_count = 0;
        let mut indeterminate_count = 0;
        let mut total_monetary = 0.0;
        let mut total_welfare = 0.0;
        let mut effect_breakdown: BTreeMap<String, usize> = BTreeMap::new();

        for impact in &per_entity {
            if impact.affected {
                affected_count += 1;
                total_monetary += impact.monetary_delta;
                total_welfare += impact.welfare_delta;
                if let Some(label) = &impact.effect_type {
                    *effect_breakdown.entry(label.clone()).or_insert(0) += 1;
                }
            } else if impact.indeterminate {
                indeterminate_count += 1;
            }
        }

        let sample_size = entities.len();
        let unaffected_count = sample_size - affected_count - indeterminate_count;
        let coverage = if sample_size == 0 {
            0.0
        } else {
            affected_count as f64 / sample_size as f64
        };
        let mean_monetary = if affected_count == 0 {
            0.0
        } else {
            total_monetary / affected_count as f64
        };
        let mean_welfare = if affected_count == 0 {
            0.0
        } else {
            total_welfare / affected_count as f64
        };

        let cohort_breakdown =
            Self::build_cohort_breakdown(entities, &per_entity, cohort_attribute);

        ImpactReport {
            statute_id: statute.id.clone(),
            sample_size,
            affected_count,
            unaffected_count,
            indeterminate_count,
            coverage,
            effect_breakdown,
            total_monetary_delta: total_monetary,
            mean_monetary_delta: mean_monetary,
            total_welfare_delta: total_welfare,
            mean_welfare_delta: mean_welfare,
            cohort_breakdown,
            per_entity,
            generated_at: Utc::now(),
        }
    }

    /// Evaluates a single entity against the statute's preconditions and effect.
    fn evaluate_entity(&self, statute: &Statute, entity: &SyntheticEntity) -> EntityImpact {
        let context = entity.context();
        let total_conditions = statute.preconditions.len();
        let mut satisfied = 0;
        let mut indeterminate = false;

        for condition in &statute.preconditions {
            match condition.evaluate_simple(&context) {
                Ok(true) => satisfied += 1,
                Ok(false) => {}
                Err(_) => indeterminate = true,
            }
        }

        // The statute applies when every precondition was satisfied and no
        // precondition was left indeterminate. An empty precondition set means
        // the statute applies universally.
        let affected = !indeterminate && satisfied == total_conditions;

        let (effect_type, monetary_delta, welfare_delta) = if affected {
            (
                Some(effect_type_label(&statute.effect.effect_type).to_string()),
                self.model.monetary_effect(&statute.effect, entity),
                self.model.welfare_weight(&statute.effect.effect_type),
            )
        } else {
            (None, 0.0, 0.0)
        };

        EntityImpact {
            entity_id: entity.id.clone(),
            affected,
            indeterminate: indeterminate && !affected,
            effect_type,
            monetary_delta,
            welfare_delta,
            satisfied_conditions: satisfied,
            total_conditions,
        }
    }

    /// Groups per-entity impacts into cohorts by an attribute value.
    fn build_cohort_breakdown(
        entities: &[SyntheticEntity],
        per_entity: &[EntityImpact],
        cohort_attribute: Option<&str>,
    ) -> BTreeMap<String, CohortImpact> {
        let mut breakdown = BTreeMap::new();
        let Some(attribute) = cohort_attribute else {
            return breakdown;
        };
        // Accumulator: (size, affected, monetary).
        let mut groups: BTreeMap<String, (usize, usize, f64)> = BTreeMap::new();
        for (entity, impact) in entities.iter().zip(per_entity.iter()) {
            let key = entity
                .attribute(attribute)
                .cloned()
                .unwrap_or_else(|| "(unset)".to_string());
            let slot = groups.entry(key).or_insert((0, 0, 0.0));
            slot.0 += 1;
            if impact.affected {
                slot.1 += 1;
                slot.2 += impact.monetary_delta;
            }
        }
        for (key, (size, affected, monetary)) in groups {
            let coverage = if size == 0 {
                0.0
            } else {
                affected as f64 / size as f64
            };
            breakdown.insert(
                key.clone(),
                CohortImpact {
                    cohort: key,
                    size,
                    affected,
                    coverage,
                    total_monetary_delta: monetary,
                },
            );
        }
        breakdown
    }
}

impl Default for ImpactPredictionSandbox {
    fn default() -> Self {
        Self::with_default_model()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sandbox::environment::{IsolationLevel, SandboxEnvironment};
    use crate::{StatuteEntry, StatuteRegistry};
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

    fn eligibility_statute() -> Statute {
        Statute::new(
            "benefit-1",
            "Senior Benefit",
            Effect::new(EffectType::MonetaryTransfer, "monthly subsidy")
                .with_parameter("amount", "200"),
        )
        .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 65))
        .with_precondition(Condition::income(ComparisonOp::LessThan, 30000))
    }

    fn entity(id: &str, age: u32, income: u64, region: &str) -> SyntheticEntity {
        SyntheticEntity::new(id)
            .with_attribute("age", age.to_string())
            .with_attribute("income", income.to_string())
            .with_attribute("region", region)
    }

    #[test]
    fn test_entity_affected_when_conditions_met() {
        let predictor = ImpactPredictionSandbox::with_default_model();
        let statute = eligibility_statute();
        let entities = vec![entity("e1", 70, 20000, "north")];
        let report = predictor.predict_statute(&statute, &entities, None);
        assert_eq!(report.affected_count, 1);
        assert_eq!(report.coverage, 1.0);
        assert_eq!(
            report.per_entity[0].effect_type.as_deref(),
            Some("MonetaryTransfer")
        );
    }

    #[test]
    fn test_entity_unaffected_when_condition_fails() {
        let predictor = ImpactPredictionSandbox::with_default_model();
        let statute = eligibility_statute();
        let entities = vec![entity("e1", 40, 20000, "north")];
        let report = predictor.predict_statute(&statute, &entities, None);
        assert_eq!(report.affected_count, 0);
        assert_eq!(report.unaffected_count, 1);
        assert_eq!(report.indeterminate_count, 0);
    }

    #[test]
    fn test_indeterminate_when_attribute_missing() {
        let predictor = ImpactPredictionSandbox::with_default_model();
        let statute = eligibility_statute();
        // Missing the `income` attribute makes income condition indeterminate.
        let entities = vec![SyntheticEntity::new("e1").with_attribute("age", "70")];
        let report = predictor.predict_statute(&statute, &entities, None);
        assert_eq!(report.affected_count, 0);
        assert_eq!(report.indeterminate_count, 1);
        assert!(report.per_entity[0].indeterminate);
    }

    #[test]
    fn test_empty_preconditions_apply_universally() {
        let predictor = ImpactPredictionSandbox::with_default_model();
        let statute = Statute::new("universal", "Applies to all", Effect::grant("right"));
        let entities = vec![entity("e1", 10, 5, "x"), entity("e2", 99, 1_000_000, "y")];
        let report = predictor.predict_statute(&statute, &entities, None);
        assert_eq!(report.affected_count, 2);
        assert_eq!(report.coverage, 1.0);
    }

    #[test]
    fn test_monetary_aggregation_and_scaling() {
        // Scale the subsidy by the entity's `household` size.
        let model = ImpactModel::new().with_scaling_attribute("household");
        let predictor = ImpactPredictionSandbox::new(model);
        let statute = Statute::new(
            "subsidy",
            "Household subsidy",
            Effect::new(EffectType::MonetaryTransfer, "subsidy").with_parameter("amount", "100"),
        );
        let entities = vec![
            SyntheticEntity::new("e1").with_attribute("household", "3"),
            SyntheticEntity::new("e2").with_attribute("household", "1"),
        ];
        let report = predictor.predict_statute(&statute, &entities, None);
        assert_eq!(report.affected_count, 2);
        // 100*3 + 100*1 = 400.
        assert!((report.total_monetary_delta - 400.0).abs() < 1e-9);
        assert!((report.mean_monetary_delta - 200.0).abs() < 1e-9);
    }

    #[test]
    fn test_cohort_breakdown_by_region() {
        let predictor = ImpactPredictionSandbox::with_default_model();
        let statute = eligibility_statute();
        let entities = vec![
            entity("e1", 70, 10000, "north"),
            entity("e2", 80, 5000, "north"),
            entity("e3", 30, 10000, "south"),
        ];
        let report = predictor.predict_statute(&statute, &entities, Some("region"));
        let north = report.cohort_breakdown.get("north").expect("north cohort");
        assert_eq!(north.size, 2);
        assert_eq!(north.affected, 2);
        assert!((north.coverage - 1.0).abs() < 1e-9);
        let south = report.cohort_breakdown.get("south").expect("south cohort");
        assert_eq!(south.affected, 0);
    }

    #[test]
    fn test_dominant_effect() {
        let predictor = ImpactPredictionSandbox::with_default_model();
        let statute = Statute::new("ob", "Obligation", Effect::obligation("file report"));
        let entities = vec![entity("e1", 1, 1, "x"), entity("e2", 2, 2, "y")];
        let report = predictor.predict_statute(&statute, &entities, None);
        assert_eq!(report.dominant_effect(), Some("Obligation"));
    }

    #[test]
    fn test_predict_via_sandbox_environment() {
        let registry = StatuteRegistry::new();
        let mut env =
            SandboxEnvironment::from_registry("exp", &registry, IsolationLevel::CopyOnWrite);
        let statute = eligibility_statute();
        env.stage(StatuteEntry::new(statute, "US"))
            .expect("stage candidate");
        let predictor = ImpactPredictionSandbox::with_default_model();
        let entities = vec![entity("e1", 70, 10000, "north")];
        let report = predictor
            .predict(&env, "benefit-1", &entities, None)
            .expect("prediction");
        assert_eq!(report.affected_count, 1);
        // Missing candidate yields an error.
        assert!(predictor.predict(&env, "missing", &entities, None).is_err());
    }
}
