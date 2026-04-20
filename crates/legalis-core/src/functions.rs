//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

pub use crate::typed_attributes::{AttributeError, AttributeValue, TypedAttributes};
use chrono::NaiveDate;
#[cfg(feature = "parallel")]
use rayon::prelude::*;
use uuid::Uuid;

#[cfg(feature = "parallel")]
use super::types::EntailmentResult;
use super::types_3::{DurationUnit, RegionType, StatuteRegistry};
#[cfg(feature = "parallel")]
use super::types_4::EntailmentEngine;
use super::types_4::RelationshipType;
use super::types_6::Statute;

/// Legal entity (natural person, legal person, or AI agent).
///
/// This trait represents any entity that can participate in legal relationships
/// and be subject to legal rules. Implementors can store and retrieve attributes
/// that are used in legal condition evaluation.
///
/// # Examples
///
/// ```
/// use legalis_core::{BasicEntity, LegalEntity};
///
/// let mut entity = BasicEntity::new();
/// entity.set_attribute("age", "25".to_string());
/// entity.set_attribute("citizenship", "US".to_string());
///
/// assert_eq!(entity.get_attribute("age"), Some("25".to_string()));
/// assert_eq!(entity.get_attribute("citizenship"), Some("US".to_string()));
/// assert_eq!(entity.get_attribute("nonexistent"), None);
/// ```
pub trait LegalEntity: Send + Sync {
    /// Returns the unique identifier of this entity.
    fn id(&self) -> Uuid;
    /// Gets an attribute value by key.
    fn get_attribute(&self, key: &str) -> Option<String>;
    /// Sets an attribute value.
    fn set_attribute(&mut self, key: &str, value: String);
}
/// Context trait for evaluating conditions against legal entities.
///
/// Implement this trait to provide custom evaluation logic for your domain.
/// This trait-based approach allows integration with any entity storage system.
///
/// # Examples
///
/// ```
/// use legalis_core::{EvaluationContext, RegionType, RelationshipType, DurationUnit};
/// use chrono::NaiveDate;
///
/// struct MyContext {
///     age: u32,
///     income: u64,
/// }
///
/// impl EvaluationContext for MyContext {
///     fn get_attribute(&self, _key: &str) -> Option<String> { None }
///     fn get_age(&self) -> Option<u32> { Some(self.age) }
///     fn get_income(&self) -> Option<u64> { Some(self.income) }
///     fn get_current_date(&self) -> Option<NaiveDate> { None }
///     fn check_geographic(&self, _region_type: RegionType, _region_id: &str) -> bool { false }
///     fn check_relationship(&self, _relationship_type: RelationshipType, _target_id: Option<&str>) -> bool { false }
///     fn get_residency_months(&self) -> Option<u32> { None }
///     fn get_duration(&self, _unit: DurationUnit) -> Option<u32> { None }
///     fn get_percentage(&self, _context: &str) -> Option<u32> { None }
///     fn evaluate_formula(&self, _formula: &str) -> Option<f64> { None }
/// }
/// ```
pub trait EvaluationContext {
    /// Get an attribute value from the entity.
    fn get_attribute(&self, key: &str) -> Option<String>;
    /// Get entity's age.
    fn get_age(&self) -> Option<u32>;
    /// Get entity's income.
    fn get_income(&self) -> Option<u64>;
    /// Get current date for date range checks.
    fn get_current_date(&self) -> Option<NaiveDate>;
    /// Get current timestamp (Unix timestamp in seconds) for temporal conditions.
    fn get_current_timestamp(&self) -> Option<i64> {
        None
    }
    /// Check geographic location.
    fn check_geographic(&self, region_type: RegionType, region_id: &str) -> bool;
    /// Check entity relationship.
    fn check_relationship(
        &self,
        relationship_type: RelationshipType,
        target_id: Option<&str>,
    ) -> bool;
    /// Get residency duration in months.
    fn get_residency_months(&self) -> Option<u32>;
    /// Get duration value for a given unit.
    fn get_duration(&self, unit: DurationUnit) -> Option<u32>;
    /// Get percentage value for a given context.
    fn get_percentage(&self, context: &str) -> Option<u32>;
    /// Evaluate a custom formula and return the result.
    fn evaluate_formula(&self, formula: &str) -> Option<f64>;
}
/// Parallel evaluation support for EntailmentEngine.
#[cfg(feature = "parallel")]
impl EntailmentEngine {
    /// Determines what legal effects follow using parallel evaluation.
    ///
    /// This method evaluates all statutes in parallel for improved performance
    /// on multi-core systems.
    pub fn entail_parallel<C: EvaluationContext + Sync>(
        &self,
        context: &C,
    ) -> Vec<EntailmentResult> {
        self.statutes
            .par_iter()
            .map(|statute| self.apply_statute_parallel(statute, context))
            .collect()
    }
    /// Determines what legal effects follow (parallel), filtering to only satisfied statutes.
    pub fn entail_satisfied_parallel<C: EvaluationContext + Sync>(
        &self,
        context: &C,
    ) -> Vec<EntailmentResult> {
        self.entail_parallel(context)
            .into_par_iter()
            .filter(|result| result.conditions_satisfied)
            .collect()
    }
    fn apply_statute_parallel<C: EvaluationContext + Sync>(
        &self,
        statute: &Statute,
        context: &C,
    ) -> EntailmentResult {
        let mut errors = Vec::new();
        if statute.preconditions.is_empty() {
            return EntailmentResult {
                statute_id: statute.id.clone(),
                effect: statute.effect.clone(),
                conditions_satisfied: true,
                errors: Vec::new(),
            };
        }
        let results: Vec<_> = statute
            .preconditions
            .par_iter()
            .map(|condition| condition.evaluate_parallel(context))
            .collect();
        let all_satisfied = results.iter().all(|r| matches!(r, Ok(true)));
        for result in results {
            if let Err(e) = result {
                errors.push(format!("{}", e));
            }
        }
        EntailmentResult {
            statute_id: statute.id.clone(),
            effect: statute.effect.clone(),
            conditions_satisfied: all_satisfied,
            errors,
        }
    }
}
impl<'a> IntoIterator for &'a StatuteRegistry {
    type Item = &'a Statute;
    type IntoIter = std::slice::Iter<'a, Statute>;
    fn into_iter(self) -> Self::IntoIter {
        self.statutes.iter()
    }
}
/// Marker types for typestate builder pattern.
///
/// These types are used to track the builder state at compile time,
/// ensuring that required fields are set before building a `Statute`.
pub mod builder_states {
    /// Marker indicating ID is not set.
    #[derive(Debug, Clone, Copy)]
    pub struct NoId;
    /// Marker indicating ID is set.
    #[derive(Debug, Clone, Copy)]
    pub struct HasId;
    /// Marker indicating title is not set.
    #[derive(Debug, Clone, Copy)]
    pub struct NoTitle;
    /// Marker indicating title is set.
    #[derive(Debug, Clone, Copy)]
    pub struct HasTitle;
    /// Marker indicating effect is not set.
    #[derive(Debug, Clone, Copy)]
    pub struct NoEffect;
    /// Marker indicating effect is set.
    #[derive(Debug, Clone, Copy)]
    pub struct HasEffect;
}
/// Marker trait for jurisdictions.
///
/// This trait enables compile-time verification that statutes are used
/// in the correct jurisdiction context.
pub trait Jurisdiction: std::fmt::Debug + Clone {
    /// Returns the jurisdiction code (e.g., "US", "UK", "US-CA").
    fn code() -> &'static str;
}
