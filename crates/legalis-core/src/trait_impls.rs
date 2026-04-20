//! # AnyJurisdiction - Trait Implementations
//!
//! This module contains trait implementations for `AnyJurisdiction`.
//!
//! ## Implemented Traits
//!
//! - `Jurisdiction`
//! - `EvaluationContext`
//! - `Default`
//! - `LegalEntity`
//! - `Jurisdiction`
//! - `Display`
//! - `Display`
//! - `Display`
//! - `Display`
//! - `Default`
//! - `Default`
//! - `Display`
//! - `Error`
//! - `Display`
//! - `Display`
//! - `Display`
//! - `Display`
//! - `Display`
//! - `Default`
//! - `EvaluationContext`
//! - `Default`
//! - `Display`
//! - `Display`
//! - `Display`
//! - `Error`
//! - `Display`
//! - `Jurisdiction`
//! - `Display`
//! - `Default`
//! - `Default`
//! - `Display`
//! - `Display`
//! - `Display`
//! - `Default`
//! - `Display`
//! - `Error`
//! - `Display`
//! - `Display`
//! - `EvaluationContext`
//! - `Default`
//! - `Display`
//! - `Display`
//! - `Jurisdiction`
//! - `Display`
//! - `Display`
//! - `Default`
//! - `Display`
//! - `Display`
//! - `Default`
//! - `Display`
//! - `Display`
//! - `Display`
//! - `Default`
//! - `IntoIterator`
//! - `FromIterator`
//! - `Display`
//! - `Display`
//! - `Display`
//! - `Default`
//! - `LegalEntity`
//! - `Default`
//! - `Jurisdiction`
//! - `Jurisdiction`
//! - `Display`
//! - `Error`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::builder_states::*;
pub use crate::typed_attributes::{AttributeError, AttributeValue, TypedAttributes};
use chrono::NaiveDate;
use std::fmt;
use uuid::Uuid;

use super::functions::{EvaluationContext, Jurisdiction, LegalEntity};
use super::types::{
    AnyJurisdiction, California, ConditionCache, ConditionalEffect, ConflictResolution,
    Contradiction, CrossJurisdictionAnalyzer, EU, Effect, EntailmentResult, ErrorSeverity,
    LegalExplanation, NewYork, SourceLocation, StatuteException, StatuteGraph, TypedEntity,
    TypedStatuteBuilder, UK, ValidationError,
};
use super::types_3::{
    AttributeBasedContext, BasicEntity, ConditionBuilder, ConditionError, ConflictReason,
    ContradictionType, DefaultValueContext, DiagnosticContext, DiagnosticReporter, DurationUnit,
    EffectBuilder, EffectType, EvaluationAuditTrail, EvaluationError, EvaluationExplanation,
    EvaluationRecord, JurisdictionStatuteRegistry, PartialBool, RegionType, StatuteRegistry,
    StepResult, TemporalEffect, US,
};
use super::types_4::{
    ComparisonOp, DiagnosticValidationError, EffectDependencyGraph, RecurrencePattern,
    RelationshipType, StatuteChange, TemporalValidity,
};
use super::types_5::{ComposedEffect, Condition, StatuteDiff};
use super::types_6::{CompositionStrategy, FallbackContext, LegalResult, Statute, StatuteBuilder};

impl Jurisdiction for AnyJurisdiction {
    fn code() -> &'static str {
        ""
    }
}

/// Implement EvaluationContext for AttributeBasedContext for compatibility.
impl EvaluationContext for AttributeBasedContext {
    fn get_attribute(&self, key: &str) -> Option<String> {
        self.attributes.get(key).cloned()
    }
    fn get_age(&self) -> Option<u32> {
        self.attributes.get("age").and_then(|v| v.parse().ok())
    }
    fn get_income(&self) -> Option<u64> {
        self.attributes.get("income").and_then(|v| v.parse().ok())
    }
    fn get_current_date(&self) -> Option<NaiveDate> {
        self.attributes
            .get("current_date")
            .and_then(|v| NaiveDate::parse_from_str(v, "%Y-%m-%d").ok())
    }
    fn check_geographic(&self, _region_type: RegionType, region_id: &str) -> bool {
        self.attributes
            .get("region")
            .is_some_and(|v| v == region_id)
    }
    fn check_relationship(
        &self,
        _relationship_type: RelationshipType,
        target_id: Option<&str>,
    ) -> bool {
        if let Some(target) = target_id {
            self.attributes
                .get("relationship")
                .is_some_and(|v| v == target)
        } else {
            self.attributes.contains_key("relationship")
        }
    }
    fn get_residency_months(&self) -> Option<u32> {
        self.attributes
            .get("residency_months")
            .and_then(|v| v.parse().ok())
    }
    fn get_duration(&self, unit: DurationUnit) -> Option<u32> {
        let key = format!("duration_{:?}", unit).to_lowercase();
        self.attributes.get(&key).and_then(|v| v.parse().ok())
    }
    fn get_percentage(&self, context: &str) -> Option<u32> {
        let key = format!("percentage_{}", context);
        self.attributes.get(&key).and_then(|v| v.parse().ok())
    }
    fn evaluate_formula(&self, _formula: &str) -> Option<f64> {
        None
    }
}

impl Default for BasicEntity {
    fn default() -> Self {
        Self::new()
    }
}

impl LegalEntity for BasicEntity {
    fn id(&self) -> Uuid {
        self.id
    }
    fn get_attribute(&self, key: &str) -> Option<String> {
        self.attributes.get(key).cloned()
    }
    fn set_attribute(&mut self, key: &str, value: String) {
        self.attributes.insert(key.to_string(), value);
    }
}

impl Jurisdiction for California {
    fn code() -> &'static str {
        "US-CA"
    }
}

impl fmt::Display for ComparisonOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Equal => write!(f, "=="),
            Self::NotEqual => write!(f, "!="),
            Self::GreaterThan => write!(f, ">"),
            Self::GreaterOrEqual => write!(f, ">="),
            Self::LessThan => write!(f, "<"),
            Self::LessOrEqual => write!(f, "<="),
        }
    }
}

impl fmt::Display for ComposedEffect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ComposedEffect[")?;
        for (i, effect) in self.effects.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", effect)?;
        }
        write!(f, "]")
    }
}

impl fmt::Display for CompositionStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FirstWins => write!(f, "FirstWins"),
            Self::LastWins => write!(f, "LastWins"),
            Self::MostSpecific => write!(f, "MostSpecific"),
            Self::AllApply => write!(f, "AllApply"),
        }
    }
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Age { operator, value } => write!(f, "age {} {}", operator, value),
            Self::Income { operator, value } => {
                write!(f, "income {} {}", operator, value)
            }
            Self::HasAttribute { key } => write!(f, "has_attribute({})", key),
            Self::AttributeEquals { key, value } => write!(f, "{} == \"{}\"", key, value),
            Self::DateRange { start, end } => match (start, end) {
                (Some(s), Some(e)) => write!(f, "date in [{}, {}]", s, e),
                (Some(s), None) => write!(f, "date >= {}", s),
                (None, Some(e)) => write!(f, "date <= {}", e),
                (None, None) => write!(f, "date (any)"),
            },
            Self::Geographic {
                region_type,
                region_id,
            } => {
                write!(f, "in {:?}({})", region_type, region_id)
            }
            Self::EntityRelationship {
                relationship_type,
                target_entity_id,
            } => match target_entity_id {
                Some(id) => write!(f, "{:?} with {}", relationship_type, id),
                None => write!(f, "has {:?}", relationship_type),
            },
            Self::ResidencyDuration { operator, months } => {
                write!(f, "residency {} {} months", operator, months)
            }
            Self::Duration {
                operator,
                value,
                unit,
            } => {
                write!(f, "duration {} {} {}", operator, value, unit)
            }
            Self::Percentage {
                operator,
                value,
                context,
            } => {
                write!(f, "{} {} {}%", context, operator, value)
            }
            Self::SetMembership {
                attribute,
                values,
                negated,
            } => {
                let op = if *negated { "NOT IN" } else { "IN" };
                write!(f, "{} {} {{{}}}", attribute, op, values.join(", "))
            }
            Self::Pattern {
                attribute,
                pattern,
                negated,
            } => {
                let op = if *negated { "!~" } else { "=~" };
                write!(f, "{} {} /{}/", attribute, op, pattern)
            }
            Self::Calculation {
                formula,
                operator,
                value,
            } => {
                write!(f, "({}) {} {}", formula, operator, value)
            }
            Self::Composite {
                conditions,
                threshold,
            } => {
                write!(f, "composite[")?;
                for (i, (weight, cond)) in conditions.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}*{}", weight, cond)?;
                }
                write!(f, "] >= {}", threshold)
            }
            Self::Threshold {
                attributes,
                operator,
                value,
            } => {
                write!(f, "sum[")?;
                for (i, (attr, mult)) in attributes.iter().enumerate() {
                    if i > 0 {
                        write!(f, " + ")?;
                    }
                    if (*mult - 1.0).abs() < f64::EPSILON {
                        write!(f, "{}", attr)?;
                    } else {
                        write!(f, "{}*{}", mult, attr)?;
                    }
                }
                write!(f, "] {} {}", operator, value)
            }
            Self::Fuzzy {
                attribute,
                membership_points,
                min_membership,
            } => {
                write!(
                    f,
                    "fuzzy({}, membership={:?}) >= {}",
                    attribute, membership_points, min_membership
                )
            }
            Self::Probabilistic {
                condition,
                probability,
                threshold,
            } => {
                write!(f, "prob({}, p={}) >= {}", condition, probability, threshold)
            }
            Self::Temporal {
                base_value,
                reference_time,
                rate,
                operator,
                target_value,
            } => {
                write!(
                    f,
                    "temporal(base={}, t0={}, rate={}) {} {}",
                    base_value, reference_time, rate, operator, target_value
                )
            }
            Self::And(left, right) => write!(f, "({} AND {})", left, right),
            Self::Or(left, right) => write!(f, "({} OR {})", left, right),
            Self::Not(inner) => write!(f, "NOT {}", inner),
            Self::Custom { description } => write!(f, "custom({})", description),
        }
    }
}

impl Default for ConditionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ConditionCache {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ConditionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingAttribute { key } => write!(f, "Missing attribute: {}", key),
            Self::TypeMismatch { expected, actual } => {
                write!(f, "Type mismatch: expected {}, got {}", expected, actual)
            }
            Self::InvalidFormula { formula, error } => {
                write!(f, "Invalid formula '{}': {}", formula, error)
            }
            Self::PatternError { pattern, error } => {
                write!(f, "Pattern error '{}': {}", pattern, error)
            }
            Self::MaxDepthExceeded { max_depth } => {
                write!(f, "Maximum evaluation depth ({}) exceeded", max_depth)
            }
            Self::Custom { message } => write!(f, "{}", message),
        }
    }
}

impl std::error::Error for ConditionError {}

impl fmt::Display for ConditionalEffect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} WHEN {}", self.effect, self.condition)
    }
}

impl fmt::Display for ConflictReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TemporalPrecedence => write!(f, "lex posterior (later law prevails)"),
            Self::Specificity => write!(f, "lex specialis (more specific law prevails)"),
            Self::Hierarchy => write!(f, "lex superior (higher authority prevails)"),
            Self::ExplicitAmendment => write!(f, "explicit amendment/repeal"),
        }
    }
}

impl fmt::Display for ConflictResolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FirstPrevails(reason) => {
                write!(f, "First statute prevails: {}", reason)
            }
            Self::SecondPrevails(reason) => {
                write!(f, "Second statute prevails: {}", reason)
            }
            Self::NoConflict => write!(f, "No conflict - statutes are compatible"),
            Self::Unresolvable(msg) => write!(f, "Unresolvable conflict: {}", msg),
        }
    }
}

impl fmt::Display for Contradiction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} <-> {}: {}",
            self.severity, self.statute_a_id, self.statute_b_id, self.description
        )
    }
}

impl fmt::Display for ContradictionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConflictingEffects => write!(f, "Conflicting Effects"),
            Self::IdenticalConditionsConflictingEffects => {
                write!(f, "Identical Conditions, Conflicting Effects")
            }
            Self::CircularDependency => write!(f, "Circular Dependency"),
            Self::LogicalInconsistency => write!(f, "Logical Inconsistency"),
        }
    }
}

impl Default for CrossJurisdictionAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, C: EvaluationContext> EvaluationContext for DefaultValueContext<'a, C> {
    fn get_attribute(&self, key: &str) -> Option<String> {
        self.inner
            .get_attribute(key)
            .or_else(|| self.defaults.get(key).cloned())
    }
    fn get_age(&self) -> Option<u32> {
        self.inner
            .get_age()
            .or_else(|| self.defaults.get("age").and_then(|s| s.parse::<u32>().ok()))
    }
    fn get_income(&self) -> Option<u64> {
        self.inner.get_income().or_else(|| {
            self.defaults
                .get("income")
                .and_then(|s| s.parse::<u64>().ok())
        })
    }
    fn get_current_date(&self) -> Option<NaiveDate> {
        self.inner.get_current_date()
    }
    fn get_current_timestamp(&self) -> Option<i64> {
        self.inner.get_current_timestamp()
    }
    fn check_geographic(&self, region_type: RegionType, region_id: &str) -> bool {
        self.inner.check_geographic(region_type, region_id)
    }
    fn check_relationship(
        &self,
        relationship_type: RelationshipType,
        target_id: Option<&str>,
    ) -> bool {
        self.inner.check_relationship(relationship_type, target_id)
    }
    fn get_residency_months(&self) -> Option<u32> {
        self.inner.get_residency_months()
    }
    fn get_duration(&self, unit: DurationUnit) -> Option<u32> {
        self.inner.get_duration(unit)
    }
    fn get_percentage(&self, context: &str) -> Option<u32> {
        self.inner.get_percentage(context)
    }
    fn evaluate_formula(&self, formula: &str) -> Option<f64> {
        self.inner.evaluate_formula(formula)
    }
}

impl Default for DiagnosticContext {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DiagnosticContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(location) = &self.location {
            writeln!(f, "at {}", location)?;
            if let Some(snippet) = &location.snippet {
                writeln!(f, "  {}", snippet)?;
            }
        }
        if let Some(statute_id) = &self.statute_id {
            writeln!(f, "in statute: {}", statute_id)?;
        }
        if let Some(condition) = &self.condition {
            writeln!(f, "condition: {}", condition)?;
        }
        if !self.stack.is_empty() {
            writeln!(f, "\nStack trace:")?;
            for (i, frame) in self.stack.iter().enumerate() {
                writeln!(f, "  {}: {}", i, frame)?;
            }
        }
        if !self.notes.is_empty() {
            writeln!(f, "\nNotes:")?;
            for note in &self.notes {
                writeln!(f, "  - {}", note)?;
            }
        }
        if !self.suggestions.is_empty() {
            writeln!(f, "\nSuggestions:")?;
            for suggestion in &self.suggestions {
                writeln!(f, "  - {}", suggestion)?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for DiagnosticReporter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.report())
    }
}

impl fmt::Display for DiagnosticValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Error [{}]: {}", self.error_code(), self.error)?;
        write!(f, "{}", self.context)?;
        Ok(())
    }
}

impl std::error::Error for DiagnosticValidationError {}

impl fmt::Display for DurationUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Days => write!(f, "days"),
            Self::Weeks => write!(f, "weeks"),
            Self::Months => write!(f, "months"),
            Self::Years => write!(f, "years"),
        }
    }
}

impl Jurisdiction for EU {
    fn code() -> &'static str {
        "EU"
    }
}

impl fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.effect_type, self.description)
    }
}

impl Default for EffectBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for EffectDependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for EffectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Grant => write!(f, "GRANT"),
            Self::Revoke => write!(f, "REVOKE"),
            Self::Obligation => write!(f, "OBLIGATION"),
            Self::Prohibition => write!(f, "PROHIBITION"),
            Self::MonetaryTransfer => write!(f, "MONETARY_TRANSFER"),
            Self::StatusChange => write!(f, "STATUS_CHANGE"),
            Self::Custom => write!(f, "CUSTOM"),
        }
    }
}

impl fmt::Display for EntailmentResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} (satisfied: {})",
            self.statute_id, self.effect, self.conditions_satisfied
        )
    }
}

impl fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Warning => write!(f, "WARNING"),
            Self::Error => write!(f, "ERROR"),
            Self::Critical => write!(f, "CRITICAL"),
        }
    }
}

impl Default for EvaluationAuditTrail {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingAttribute { key } => write!(f, "Missing attribute: {}", key),
            Self::MissingContext { description } => {
                write!(f, "Missing context: {}", description)
            }
            Self::InvalidFormula { formula, reason } => {
                write!(f, "Invalid formula '{}': {}", formula, reason)
            }
            Self::PatternError { pattern, reason } => {
                write!(f, "Pattern error '{}': {}", pattern, reason)
            }
            Self::MaxDepthExceeded { max_depth } => {
                write!(f, "Maximum evaluation depth {} exceeded", max_depth)
            }
            Self::Custom { message } => write!(f, "{}", message),
        }
    }
}

impl std::error::Error for EvaluationError {}

impl fmt::Display for EvaluationExplanation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Evaluation of: {}", self.condition)?;
        writeln!(f, "Result: {}", self.conclusion)?;
        writeln!(f, "\nEvaluation trace:")?;
        for (i, step) in self.steps.iter().enumerate() {
            let indent = "  ".repeat(step.depth);
            writeln!(
                f,
                "{}{}. {} -> {} ({}μs)",
                indent,
                i + 1,
                step.condition,
                step.result,
                step.duration_micros
            )?;
            if !step.details.is_empty() {
                writeln!(f, "{}   Details: {}", indent, step.details)?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for EvaluationRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} = {} ({} μs)",
            self.timestamp.format("%Y-%m-%d %H:%M:%S"),
            self.condition,
            self.result,
            self.duration_micros
        )
    }
}

impl<'a, C1: EvaluationContext, C2: EvaluationContext> EvaluationContext
    for FallbackContext<'a, C1, C2>
{
    fn get_attribute(&self, key: &str) -> Option<String> {
        self.primary
            .get_attribute(key)
            .or_else(|| self.fallback.get_attribute(key))
    }
    fn get_age(&self) -> Option<u32> {
        self.primary.get_age().or_else(|| self.fallback.get_age())
    }
    fn get_income(&self) -> Option<u64> {
        self.primary
            .get_income()
            .or_else(|| self.fallback.get_income())
    }
    fn get_current_date(&self) -> Option<NaiveDate> {
        self.primary
            .get_current_date()
            .or_else(|| self.fallback.get_current_date())
    }
    fn get_current_timestamp(&self) -> Option<i64> {
        self.primary
            .get_current_timestamp()
            .or_else(|| self.fallback.get_current_timestamp())
    }
    fn check_geographic(&self, region_type: RegionType, region_id: &str) -> bool {
        self.primary.check_geographic(region_type, region_id)
            || self.fallback.check_geographic(region_type, region_id)
    }
    fn check_relationship(
        &self,
        relationship_type: RelationshipType,
        target_id: Option<&str>,
    ) -> bool {
        self.primary
            .check_relationship(relationship_type, target_id)
            || self
                .fallback
                .check_relationship(relationship_type, target_id)
    }
    fn get_residency_months(&self) -> Option<u32> {
        self.primary
            .get_residency_months()
            .or_else(|| self.fallback.get_residency_months())
    }
    fn get_duration(&self, unit: DurationUnit) -> Option<u32> {
        self.primary
            .get_duration(unit)
            .or_else(|| self.fallback.get_duration(unit))
    }
    fn get_percentage(&self, context: &str) -> Option<u32> {
        self.primary
            .get_percentage(context)
            .or_else(|| self.fallback.get_percentage(context))
    }
    fn evaluate_formula(&self, formula: &str) -> Option<f64> {
        self.primary
            .evaluate_formula(formula)
            .or_else(|| self.fallback.evaluate_formula(formula))
    }
}

impl<J: Jurisdiction> Default for JurisdictionStatuteRegistry<J> {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for LegalExplanation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Explanation for outcome: {}", self.outcome)?;
        writeln!(f, "Confidence: {:.0}%", self.confidence * 100.0)?;
        writeln!(f)?;
        if !self.applicable_statutes.is_empty() {
            writeln!(f, "Applicable statutes:")?;
            for statute_id in &self.applicable_statutes {
                writeln!(f, "  - {}", statute_id)?;
            }
            writeln!(f)?;
        }
        if !self.satisfied_conditions.is_empty() {
            writeln!(f, "Satisfied conditions:")?;
            for condition in &self.satisfied_conditions {
                writeln!(f, "  ✓ {}", condition)?;
            }
            writeln!(f)?;
        }
        if !self.unsatisfied_conditions.is_empty() {
            writeln!(f, "Unsatisfied conditions:")?;
            for condition in &self.unsatisfied_conditions {
                writeln!(f, "  ✗ {}", condition)?;
            }
            writeln!(f)?;
        }
        if !self.reasoning_chain.is_empty() {
            writeln!(f, "Reasoning chain:")?;
            for step in &self.reasoning_chain {
                write!(f, "  {}. {} ", step.step, step.description)?;
                writeln!(f, "[{}]", step.result)?;
            }
        }
        Ok(())
    }
}

impl<T: fmt::Display> fmt::Display for LegalResult<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Deterministic(value) => write!(f, "Deterministic({})", value),
            Self::JudicialDiscretion {
                issue,
                narrative_hint,
                ..
            } => {
                write!(f, "JudicialDiscretion: {}", issue)?;
                if let Some(hint) = narrative_hint {
                    write!(f, " [hint: {}]", hint)?;
                }
                Ok(())
            }
            Self::Void { reason } => write!(f, "Void: {}", reason),
        }
    }
}

impl Jurisdiction for NewYork {
    fn code() -> &'static str {
        "US-NY"
    }
}

impl fmt::Display for PartialBool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::True { confidence, reason } => {
                if reason.is_empty() {
                    write!(f, "True (confidence: {:.2})", confidence)
                } else {
                    write!(
                        f,
                        "True (confidence: {:.2}, reason: {})",
                        confidence, reason
                    )
                }
            }
            Self::False { confidence, reason } => {
                if reason.is_empty() {
                    write!(f, "False (confidence: {:.2})", confidence)
                } else {
                    write!(
                        f,
                        "False (confidence: {:.2}, reason: {})",
                        confidence, reason
                    )
                }
            }
            Self::Unknown { confidence, reason } => {
                write!(
                    f,
                    "Unknown (confidence: {:.2}, reason: {})",
                    confidence, reason
                )
            }
        }
    }
}

impl fmt::Display for RecurrencePattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Daily => write!(f, "daily"),
            Self::Weekly { interval } => write!(f, "every {} week(s)", interval),
            Self::Monthly { interval } => write!(f, "every {} month(s)", interval),
            Self::Yearly { interval } => write!(f, "every {} year(s)", interval),
            Self::DaysOfWeek { days } => {
                write!(f, "on days: ")?;
                for (i, day) in days.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", day)?;
                }
                Ok(())
            }
            Self::Custom { description } => write!(f, "custom: {}", description),
        }
    }
}

impl Default for SourceLocation {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(file) = &self.file {
            write!(f, "{}", file)?;
            if let Some(line) = self.line {
                write!(f, ":{}", line)?;
                if let Some(column) = self.column {
                    write!(f, ":{}", column)?;
                }
            }
        } else if let Some(line) = self.line {
            write!(f, "line {}", line)?;
            if let Some(column) = self.column {
                write!(f, ":{}", column)?;
            }
        } else {
            write!(f, "unknown location")?;
        }
        Ok(())
    }
}

impl fmt::Display for Statute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "STATUTE {}: \"{}\"", self.id, self.title)?;
        if let Some(ref jur) = self.jurisdiction {
            writeln!(f, "  JURISDICTION: {}", jur)?;
        }
        writeln!(f, "  VERSION: {}", self.version)?;
        writeln!(f, "  {}", self.temporal_validity)?;
        if !self.preconditions.is_empty() {
            writeln!(f, "  WHEN:")?;
            for cond in &self.preconditions {
                writeln!(f, "    {}", cond)?;
            }
        }
        writeln!(f, "  THEN: {}", self.effect)?;
        if let Some(ref disc) = self.discretion_logic {
            writeln!(f, "  DISCRETION: {}", disc)?;
        }
        Ok(())
    }
}

impl Default for StatuteBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for StatuteChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IdChanged { old, new } => write!(f, "ID: '{}' → '{}'", old, new),
            Self::TitleChanged { old, new } => {
                write!(f, "Title: '{}' → '{}'", old, new)
            }
            Self::EffectChanged { old, new } => write!(f, "Effect: {} → {}", old, new),
            Self::PreconditionsChanged { added, removed } => {
                write!(f, "Preconditions: +{} -{}", added, removed)
            }
            Self::TemporalValidityChanged => write!(f, "Temporal validity changed"),
            Self::VersionChanged { old, new } => {
                write!(f, "Version: {} → {}", old, new)
            }
            Self::JurisdictionChanged { old, new } => {
                write!(
                    f,
                    "Jurisdiction: {} → {}",
                    old.as_deref().unwrap_or("None"),
                    new.as_deref().unwrap_or("None")
                )
            }
        }
    }
}

impl fmt::Display for StatuteDiff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "No changes for statute '{}'", self.statute_id);
        }
        writeln!(f, "Changes for statute '{}':", self.statute_id)?;
        for (i, change) in self.changes.iter().enumerate() {
            writeln!(f, "  {}. {}", i + 1, change)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for StatuteException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Exception '{}': {} when {}",
            self.id, self.description, self.condition
        )
    }
}

impl Default for StatuteGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for StatuteRegistry {
    type Item = Statute;
    type IntoIter = std::vec::IntoIter<Statute>;
    fn into_iter(self) -> Self::IntoIter {
        self.statutes.into_iter()
    }
}

impl FromIterator<Statute> for StatuteRegistry {
    fn from_iter<T: IntoIterator<Item = Statute>>(iter: T) -> Self {
        Self {
            statutes: iter.into_iter().collect(),
        }
    }
}

impl fmt::Display for StepResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Satisfied => write!(f, "✓ satisfied"),
            Self::NotSatisfied => write!(f, "✗ not satisfied"),
            Self::Applied => write!(f, "→ applied"),
            Self::NotApplicable => write!(f, "- not applicable"),
            Self::Uncertain => write!(f, "? uncertain"),
        }
    }
}

impl fmt::Display for TemporalEffect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (active from {}", self.effect, self.start_date)?;
        if let Some(end) = self.end_date {
            write!(f, " to {}", end)?;
        }
        if let Some(ref rec) = self.recurrence {
            write!(f, ", {}", rec)?;
        }
        write!(f, ")")
    }
}

impl fmt::Display for TemporalValidity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (&self.effective_date, &self.expiry_date) {
            (Some(eff), Some(exp)) => write!(f, "valid {} to {}", eff, exp),
            (Some(eff), None) => write!(f, "effective from {}", eff),
            (None, Some(exp)) => write!(f, "expires {}", exp),
            (None, None) => write!(f, "no temporal constraints"),
        }
    }
}

impl Default for TypedEntity {
    fn default() -> Self {
        Self::new()
    }
}

impl LegalEntity for TypedEntity {
    fn id(&self) -> Uuid {
        self.id
    }
    fn get_attribute(&self, key: &str) -> Option<String> {
        self.attributes.get(key).map(|v| v.to_string_value())
    }
    fn set_attribute(&mut self, key: &str, value: String) {
        self.attributes
            .set(key, AttributeValue::parse_from_string(&value));
    }
}

impl Default for TypedStatuteBuilder<NoId, NoTitle, NoEffect> {
    fn default() -> Self {
        Self::new()
    }
}

impl Jurisdiction for UK {
    fn code() -> &'static str {
        "UK"
    }
}

impl Jurisdiction for US {
    fn code() -> &'static str {
        "US"
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyId => write!(f, "Statute ID cannot be empty"),
            Self::InvalidId(id) => {
                write!(
                    f,
                    "Invalid statute ID: '{}' (must start with letter, contain only alphanumeric/dash/underscore)",
                    id
                )
            }
            Self::EmptyTitle => write!(f, "Statute title cannot be empty"),
            Self::ExpiryBeforeEffective { effective, expiry } => {
                write!(
                    f,
                    "Expiry date ({}) cannot be before effective date ({})",
                    expiry, effective
                )
            }
            Self::InvalidCondition { index, message } => {
                write!(f, "Invalid condition at index {}: {}", index, message)
            }
            Self::EmptyEffectDescription => {
                write!(f, "Effect description cannot be empty")
            }
            Self::InvalidVersion => write!(f, "Version must be greater than 0"),
        }
    }
}

impl std::error::Error for ValidationError {}
