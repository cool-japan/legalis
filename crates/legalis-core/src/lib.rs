//! Legalis-Core: Core types and traits for the Legalis-RS legal framework.
//!
//! This module defines the foundational types that represent legal concepts,
//! including the distinction between deterministic (computable) and
//! discretionary (requiring human judgment) legal outcomes.
//!
//! ## Design Philosophy
//!
//! ### "Not Everything Should Be Computable"
//!
//! Legalis-RS is built on the principle that while many legal determinations can be
//! automated, some require human judgment and interpretation. This is reflected in the
//! [`LegalResult`] type, which explicitly distinguishes between:
//!
//! - **Deterministic** outcomes: Mechanically derivable from rules (age >= 18, income < $50k)
//! - **Judicial Discretion**: Requires human interpretation (just cause, public welfare)
//! - **Void**: Logical contradictions in the law itself
//!
//! This design prevents "AI theocracy" by preserving human agency in legal interpretation
//! where it matters most.
//!
//! ### Type-Safe Legal Modeling
//!
//! The crate uses Rust's type system to enforce legal invariants at compile time:
//!
//! - Statutes must have IDs, titles, and effects
//! - Temporal validity is checked (expiry dates must follow effective dates)
//! - Conditions are strongly typed and composable
//! - Entity attributes can be type-safe via [`TypedEntity`]
//!
//! ### Builder Pattern for Clarity
//!
//! Complex legal structures use the builder pattern for readability:
//!
//! ```no_run
//! # use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp, TemporalValidity};
//! # use chrono::NaiveDate;
//! let statute = Statute::new("tax-law-2025", "Income Tax", Effect::new(EffectType::Grant, "Tax credit"))
//!     .with_precondition(Condition::Income { operator: ComparisonOp::LessThan, value: 50000 })
//!     .with_temporal_validity(TemporalValidity::new()
//!         .with_effective_date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()))
//!     .with_jurisdiction("US")
//!     .with_version(1);
//! ```
//!
//! ### Validation Over Panics
//!
//! The crate prefers validation methods that return errors over runtime panics:
//!
//! - [`Statute::validate()`] returns a list of validation errors
//! - [`Statute::validated()`] returns `Result<Statute, Vec<ValidationError>>`
//! - [`TypedEntity`] returns `Result` for type mismatches
//!
//! This allows calling code to decide how to handle invalid states.
//!
//! ### Temporal Awareness
//!
//! Legal rules change over time. The [`TemporalValidity`] type tracks:
//!
//! - Effective dates (when laws come into force)
//! - Expiry dates (sunset clauses)
//! - Enactment and amendment timestamps
//!
//! This enables historical legal queries and version management.
//!
//! ## Architecture Decisions
//!
//! ### Why ADTs for Conditions?
//!
//! The [`Condition`] enum uses algebraic data types (ADTs) with recursive composition
//! (AND/OR/NOT) rather than a trait-based visitor pattern. This provides:
//!
//! - Pattern matching exhaustiveness checking at compile time
//! - Easier serialization/deserialization
//! - Simpler mental model for legal rule composition
//! - Better performance (no dynamic dispatch)
//!
//! ### Why Trait Objects for Entities?
//!
//! [`LegalEntity`] is a trait rather than a concrete type because:
//!
//! - Different systems may have different entity storage needs
//! - Allows integration with existing database models
//! - Supports both simple ([`BasicEntity`]) and type-safe ([`TypedEntity`]) implementations
//! - Enables custom entity types for domain-specific needs
//!
//! ### Why HashMap for Effect Parameters?
//!
//! Effect parameters use `HashMap<String, String>` for flexibility:
//!
//! - Legal effects vary widely in their parameters
//! - Allows extension without breaking changes
//! - Simple serialization format
//! - Type safety can be added at higher layers if needed
//!
//! ## Type Relationships
//!
//! The following diagrams illustrate the core type relationships in legalis-core:
//!
//! ### Core Legal Types
//!
//! ```mermaid
//! classDiagram
//!     class Statute {
//!         +String id
//!         +String title
//!         +Effect effect
//!         +Option~Condition~ precondition
//!         +Option~TemporalValidity~ temporal
//!         +Vec~String~ tags
//!         +validate() Vec~ValidationError~
//!     }
//!
//!     class Effect {
//!         +EffectType effect_type
//!         +String description
//!         +HashMap parameters
//!     }
//!
//!     class EffectType {
//!         <<enumeration>>
//!         Grant
//!         Obligation
//!         Prohibition
//!         Conditional
//!         Delayed
//!         Compound
//!     }
//!
//!     class LegalResult~T~ {
//!         <<enumeration>>
//!         Deterministic(T)
//!         JudicialDiscretion
//!         Void
//!         +map(f) LegalResult~U~
//!         +and_then(f) LegalResult~U~
//!     }
//!
//!     class TemporalValidity {
//!         +Option~NaiveDate~ effective_date
//!         +Option~NaiveDate~ expiry_date
//!         +Option~DateTime~ enactment_date
//!         +is_valid_on(date) bool
//!     }
//!
//!     Statute --> Effect : contains
//!     Statute --> Condition : optional precondition
//!     Statute --> TemporalValidity : optional temporal
//!     Effect --> EffectType : has type
//!     Statute ..> LegalResult : validation returns
//! ```
//!
//! ### Condition Composition
//!
//! ```mermaid
//! classDiagram
//!     class Condition {
//!         <<enumeration>>
//!         Age
//!         Income
//!         Geographic
//!         DateRange
//!         EntityRelationship
//!         ResidencyDuration
//!         And(Vec~Condition~)
//!         Or(Vec~Condition~)
//!         Not(Box~Condition~)
//!         +evaluate(entity) LegalResult~bool~
//!         +normalize() Condition
//!     }
//!
//!     class ComparisonOp {
//!         <<enumeration>>
//!         Equal
//!         NotEqual
//!         LessThan
//!         LessThanOrEqual
//!         GreaterThan
//!         GreaterThanOrEqual
//!     }
//!
//!     Condition --> Condition : recursive composition
//!     Condition --> ComparisonOp : uses for comparisons
//! ```
//!
//! ### Entity Type Hierarchy
//!
//! ```mermaid
//! classDiagram
//!     class LegalEntity {
//!         <<trait>>
//!         +id() String
//!         +entity_type() String
//!         +get_attribute(key) Option~String~
//!         +set_attribute(key, value)
//!         +attributes() HashMap
//!     }
//!
//!     class BasicEntity {
//!         +String id
//!         +String entity_type
//!         +HashMap attributes
//!     }
//!
//!     class TypedEntity {
//!         +String id
//!         +String entity_type
//!         +TypedAttributes attributes
//!         +get_typed~T~(key) Result~T~
//!         +set_typed~T~(key, value)
//!     }
//!
//!     class TypedAttributes {
//!         +HashMap~String,AttributeValue~ data
//!         +get~T~(key) Result~T~
//!         +set~T~(key, value)
//!     }
//!
//!     class AttributeValue {
//!         <<enumeration>>
//!         String(String)
//!         U32(u32)
//!         Bool(bool)
//!         Date(NaiveDate)
//!     }
//!
//!     LegalEntity <|.. BasicEntity : implements
//!     LegalEntity <|.. TypedEntity : implements
//!     TypedEntity --> TypedAttributes : contains
//!     TypedAttributes --> AttributeValue : stores
//! ```
//!
//! ### Case Law Structure
//!
//! ```mermaid
//! classDiagram
//!     class Case {
//!         +String id
//!         +String title
//!         +Court court
//!         +NaiveDate decision_date
//!         +Vec~CaseRule~ rules
//!     }
//!
//!     class Court {
//!         +String name
//!         +String jurisdiction
//!         +u8 level
//!     }
//!
//!     class CaseRule {
//!         +String principle
//!         +String reasoning
//!         +Vec~String~ facts
//!     }
//!
//!     class Precedent {
//!         +String case_id
//!         +PrecedentWeight weight
//!     }
//!
//!     class PrecedentWeight {
//!         <<enumeration>>
//!         Binding
//!         Persuasive
//!         Distinguishable
//!     }
//!
//!     Case --> Court : decided by
//!     Case --> CaseRule : contains
//!     Precedent --> PrecedentWeight : has
//! ```
//!
//! ## Features
//!
//! - `serde` (default): Enable serialization/deserialization support for all types
//!
//! ## Performance Considerations
//!
//! - Condition evaluation is non-allocating where possible
//! - Entity attributes use copy-on-write semantics
//! - Statutes are cheaply cloneable (most fields are small or reference-counted)
//! - Property-based tests ensure reasonable performance across edge cases

pub mod case_law;
pub mod const_collections;
pub mod formats;
pub mod testing;
pub mod transactions;
pub mod typed_attributes;
pub mod typed_effects;
pub mod workflows;

// Performance & Memory (v0.1.9)
pub mod arena;
pub mod compact;
pub mod interning;
pub mod lazy;
pub mod parallel_eval;

// Distributed Legal Reasoning (v0.2.0)
pub mod distributed;

// Formal Methods Integration (v0.2.1)
pub mod formal_methods;

// Legal Knowledge Graphs (v0.2.2)
pub mod knowledge_graph;

// Advanced Temporal Logic (v0.2.3)
pub mod temporal;

// Legal Document Processing (v0.2.4)
pub mod document_processing;

// Probabilistic Legal Reasoning (v0.2.5)
pub mod probabilistic;

// Multi-Jurisdictional Support (v0.2.6)
pub mod multi_jurisdictional;

use chrono::{DateTime, Datelike, NaiveDate, Utc};
use std::collections::HashMap;
use std::fmt;
use uuid::Uuid;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "schema")]
use schemars::JsonSchema;

// Re-export Common Law types
pub use case_law::{
    Case, CaseDatabase, CaseRule, Court, DamageType, Precedent, PrecedentApplication,
    PrecedentWeight,
};

// Re-export Typed Attributes
pub use typed_attributes::{AttributeError, AttributeValue, TypedAttributes};

/// Legal judgment result as an Algebraic Data Type (ADT).
///
/// This type embodies the core philosophy of Legalis-RS:
/// "Not everything should be computable" - preserving human agency
/// in legal interpretation.
///
/// # Examples
///
/// ## Deterministic Result
///
/// ```
/// use legalis_core::LegalResult;
///
/// let age = 25;
/// let result: LegalResult<bool> = if age >= 18 {
///     LegalResult::Deterministic(true)
/// } else {
///     LegalResult::Deterministic(false)
/// };
///
/// assert!(result.is_deterministic());
/// ```
///
/// ## Judicial Discretion
///
/// ```
/// use legalis_core::LegalResult;
/// use uuid::Uuid;
///
/// let result: LegalResult<bool> = LegalResult::JudicialDiscretion {
///     issue: "Determine if there is just cause for termination".to_string(),
///     context_id: Uuid::new_v4(),
///     narrative_hint: Some("Consider employment history and circumstances".to_string()),
/// };
///
/// assert!(result.requires_discretion());
/// ```
///
/// ## Mapping Values
///
/// ```
/// use legalis_core::LegalResult;
///
/// let amount: LegalResult<u32> = LegalResult::Deterministic(100);
/// let doubled = amount.map(|x| x * 2);
///
/// assert_eq!(doubled, LegalResult::Deterministic(200));
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum LegalResult<T> {
    /// Deterministic domain: Results derived automatically through computation.
    /// Examples: age requirements, income limits, deadline calculations.
    Deterministic(T),

    /// Discretionary domain: Cannot be determined by logic alone,
    /// requires human "narrative" (interpretation).
    /// This is the safeguard against "AI theocracy".
    /// The system halts here and passes the ball to humans.
    JudicialDiscretion {
        /// The issue at hand (e.g., "existence of just cause", "violation of public welfare")
        issue: String,
        /// Reference to context data
        context_id: Uuid,
        /// Recommended judgment materials (generated by LLM, but does not decide)
        narrative_hint: Option<String>,
    },

    /// Logical contradiction: A bug in the law itself.
    Void { reason: String },
}

impl<T> LegalResult<T> {
    /// Returns true if this is a deterministic result.
    pub fn is_deterministic(&self) -> bool {
        matches!(self, Self::Deterministic(_))
    }

    /// Returns true if judicial discretion is required.
    pub fn requires_discretion(&self) -> bool {
        matches!(self, Self::JudicialDiscretion { .. })
    }

    /// Returns true if this represents a void/invalid state.
    pub fn is_void(&self) -> bool {
        matches!(self, Self::Void { .. })
    }

    /// Maps a deterministic value using the provided function.
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> LegalResult<U> {
        match self {
            Self::Deterministic(t) => LegalResult::Deterministic(f(t)),
            Self::JudicialDiscretion {
                issue,
                context_id,
                narrative_hint,
            } => LegalResult::JudicialDiscretion {
                issue,
                context_id,
                narrative_hint,
            },
            Self::Void { reason } => LegalResult::Void { reason },
        }
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

/// A simple implementation of LegalEntity for testing and basic use cases.
///
/// This struct provides a straightforward key-value string storage for entity attributes.
/// For type-safe attribute handling, consider using [`TypedEntity`] instead.
///
/// # Examples
///
/// ```
/// use legalis_core::{BasicEntity, LegalEntity};
///
/// let mut person = BasicEntity::new();
/// person.set_attribute("name", "Alice".to_string());
/// person.set_attribute("age", "30".to_string());
///
/// assert_eq!(person.get_attribute("name"), Some("Alice".to_string()));
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct BasicEntity {
    id: Uuid,
    attributes: std::collections::HashMap<String, String>,
}

impl BasicEntity {
    /// Creates a new BasicEntity with a random UUID.
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            attributes: std::collections::HashMap::new(),
        }
    }

    /// Creates a new BasicEntity with a specific UUID.
    pub fn with_id(id: Uuid) -> Self {
        Self {
            id,
            attributes: std::collections::HashMap::new(),
        }
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

/// A type-safe implementation of LegalEntity using strongly-typed attributes.
///
/// This provides compile-time type safety and runtime validation for entity attributes,
/// replacing error-prone string parsing with explicit type handling.
///
/// # Examples
///
/// ```
/// use legalis_core::TypedEntity;
/// use chrono::NaiveDate;
///
/// let mut person = TypedEntity::new();
/// person.set_string("name", "Bob");
/// person.set_u32("age", 25);
/// person.set_bool("is_citizen", true);
/// person.set_date("birth_date", NaiveDate::from_ymd_opt(1999, 1, 15).unwrap());
///
/// assert_eq!(person.get_string("name").unwrap(), "Bob");
/// assert_eq!(person.get_u32("age").unwrap(), 25);
/// assert!(person.get_bool("is_citizen").unwrap());
///
/// // Type safety: attempting to get a string as a number returns an error
/// assert!(person.get_u32("name").is_err());
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct TypedEntity {
    id: Uuid,
    attributes: TypedAttributes,
}

impl TypedEntity {
    /// Creates a new TypedEntity with a random UUID.
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            attributes: TypedAttributes::new(),
        }
    }

    /// Creates a new TypedEntity with a specific UUID.
    pub fn with_id(id: Uuid) -> Self {
        Self {
            id,
            attributes: TypedAttributes::new(),
        }
    }

    /// Gets the typed attributes storage.
    pub fn attributes(&self) -> &TypedAttributes {
        &self.attributes
    }

    /// Gets mutable access to the typed attributes storage.
    pub fn attributes_mut(&mut self) -> &mut TypedAttributes {
        &mut self.attributes
    }

    /// Sets a u32 attribute.
    pub fn set_u32(&mut self, key: impl Into<String>, value: u32) {
        self.attributes.set_u32(key, value);
    }

    /// Gets a u32 attribute.
    pub fn get_u32(&self, key: &str) -> Result<u32, AttributeError> {
        self.attributes.get_u32(key)
    }

    /// Sets a u64 attribute.
    pub fn set_u64(&mut self, key: impl Into<String>, value: u64) {
        self.attributes.set_u64(key, value);
    }

    /// Gets a u64 attribute.
    pub fn get_u64(&self, key: &str) -> Result<u64, AttributeError> {
        self.attributes.get_u64(key)
    }

    /// Sets a boolean attribute.
    pub fn set_bool(&mut self, key: impl Into<String>, value: bool) {
        self.attributes.set_bool(key, value);
    }

    /// Gets a boolean attribute.
    pub fn get_bool(&self, key: &str) -> Result<bool, AttributeError> {
        self.attributes.get_bool(key)
    }

    /// Sets a string attribute.
    pub fn set_string(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.attributes.set_string(key, value);
    }

    /// Gets a string attribute.
    pub fn get_string(&self, key: &str) -> Result<&str, AttributeError> {
        self.attributes.get_string(key)
    }

    /// Sets a date attribute.
    pub fn set_date(&mut self, key: impl Into<String>, value: NaiveDate) {
        self.attributes.set_date(key, value);
    }

    /// Gets a date attribute.
    pub fn get_date(&self, key: &str) -> Result<NaiveDate, AttributeError> {
        self.attributes.get_date(key)
    }

    /// Sets an f64 attribute.
    pub fn set_f64(&mut self, key: impl Into<String>, value: f64) {
        self.attributes.set_f64(key, value);
    }

    /// Gets an f64 attribute.
    pub fn get_f64(&self, key: &str) -> Result<f64, AttributeError> {
        self.attributes.get_f64(key)
    }

    /// Sets a typed attribute value.
    pub fn set_typed(&mut self, key: impl Into<String>, value: AttributeValue) {
        self.attributes.set(key, value);
    }

    /// Gets a typed attribute value.
    pub fn get_typed(&self, key: &str) -> Option<&AttributeValue> {
        self.attributes.get(key)
    }

    /// Checks if an attribute exists.
    pub fn has_attribute(&self, key: &str) -> bool {
        self.attributes.has(key)
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

/// Time unit for duration conditions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum DurationUnit {
    /// Days
    Days,
    /// Weeks
    Weeks,
    /// Months
    Months,
    /// Years
    Years,
}

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

/// Three-valued logic result with uncertainty propagation.
///
/// Represents the result of partial evaluation where some data may be unknown.
/// Each value includes a confidence score (0.0 to 1.0) representing certainty.
///
/// # Uncertainty Propagation
///
/// - **AND**: Confidence is minimum of operands; False propagates immediately
/// - **OR**: Confidence is minimum of operands; True propagates immediately
/// - **NOT**: Confidence is preserved; value is inverted
///
/// # Examples
///
/// ```
/// # use legalis_core::PartialBool;
/// let definite_true = PartialBool::true_with_confidence(1.0);
/// let uncertain = PartialBool::unknown(0.5, "missing data");
/// let definite_false = PartialBool::false_with_confidence(1.0);
///
/// assert!(matches!(definite_true, PartialBool::True { confidence, .. } if confidence == 1.0));
/// assert!(matches!(uncertain, PartialBool::Unknown { confidence, .. } if confidence == 0.5));
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum PartialBool {
    /// Definitely true with confidence score.
    True {
        /// Confidence in this result (0.0 to 1.0).
        confidence: f64,
        /// Optional reason or explanation.
        reason: String,
    },
    /// Definitely false with confidence score.
    False {
        /// Confidence in this result (0.0 to 1.0).
        confidence: f64,
        /// Optional reason or explanation.
        reason: String,
    },
    /// Unknown (insufficient data) with confidence score.
    Unknown {
        /// Confidence in knowing it's unknown (0.0 to 1.0).
        confidence: f64,
        /// Reason why value is unknown.
        reason: String,
    },
}

impl PartialBool {
    /// Creates a True value with the given confidence.
    #[must_use]
    pub fn true_with_confidence(confidence: f64) -> Self {
        Self::True {
            confidence,
            reason: String::new(),
        }
    }

    /// Creates a True value with confidence and reason.
    #[must_use]
    pub fn true_with_confidence_and_reason(confidence: f64, reason: &str) -> Self {
        Self::True {
            confidence,
            reason: reason.to_string(),
        }
    }

    /// Creates a False value with the given confidence.
    #[must_use]
    pub fn false_with_confidence(confidence: f64) -> Self {
        Self::False {
            confidence,
            reason: String::new(),
        }
    }

    /// Creates a False value with confidence and reason.
    #[must_use]
    pub fn false_with_confidence_and_reason(confidence: f64, reason: &str) -> Self {
        Self::False {
            confidence,
            reason: reason.to_string(),
        }
    }

    /// Creates an Unknown value with confidence and reason.
    #[must_use]
    pub fn unknown(confidence: f64, reason: &str) -> Self {
        Self::Unknown {
            confidence,
            reason: reason.to_string(),
        }
    }

    /// Returns the confidence score.
    #[must_use]
    pub fn confidence(&self) -> f64 {
        match self {
            Self::True { confidence, .. }
            | Self::False { confidence, .. }
            | Self::Unknown { confidence, .. } => *confidence,
        }
    }

    /// Returns the reason or explanation.
    #[must_use]
    pub fn reason(&self) -> &str {
        match self {
            Self::True { reason, .. }
            | Self::False { reason, .. }
            | Self::Unknown { reason, .. } => reason,
        }
    }

    /// Checks if the result is definitely true.
    #[must_use]
    pub fn is_true(&self) -> bool {
        matches!(self, Self::True { .. })
    }

    /// Checks if the result is definitely false.
    #[must_use]
    pub fn is_false(&self) -> bool {
        matches!(self, Self::False { .. })
    }

    /// Checks if the result is unknown.
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown { .. })
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

/// Detailed explanation of a condition evaluation.
///
/// Contains the evaluation result, the condition evaluated, and a trace
/// of all sub-evaluations that led to the final result.
///
/// # Examples
///
/// ```
/// # use legalis_core::{Condition, ComparisonOp, AttributeBasedContext, EvaluationExplanation};
/// # use std::collections::HashMap;
/// let mut attributes = HashMap::new();
/// attributes.insert("age".to_string(), "25".to_string());
/// attributes.insert("income".to_string(), "50000".to_string());
/// let ctx = AttributeBasedContext::new(attributes);
///
/// let age_check = Condition::age(ComparisonOp::GreaterOrEqual, 18);
/// let income_check = Condition::income(ComparisonOp::GreaterOrEqual, 30000);
/// let condition = age_check.and(income_check);
///
/// let (result, explanation) = condition.evaluate_with_explanation(&ctx).unwrap();
/// assert!(result);
/// println!("Explanation:\n{}", explanation);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EvaluationExplanation {
    /// The condition that was evaluated (formatted as string).
    pub condition: String,
    /// The final evaluation result.
    pub conclusion: bool,
    /// Step-by-step trace of the evaluation.
    pub steps: Vec<ExplanationStep>,
}

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

/// A single step in the evaluation trace.
///
/// Records one condition evaluation including timing and context.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExplanationStep {
    /// The condition being evaluated (formatted as string).
    pub condition: String,
    /// The result of this evaluation step.
    pub result: bool,
    /// Additional details about how the result was determined.
    pub details: String,
    /// Nesting depth (for compound conditions).
    pub depth: usize,
    /// Time taken for this evaluation step (in microseconds).
    pub duration_micros: u64,
}

/// Condition type for statute preconditions.
///
/// Conditions represent the requirements that must be met for a statute to apply.
/// They can be simple (age checks, attribute checks) or complex (combinations using AND/OR/NOT).
///
/// # Examples
///
/// ## Simple Condition
///
/// ```
/// use legalis_core::{Condition, ComparisonOp};
///
/// let age_check = Condition::Age {
///     operator: ComparisonOp::GreaterOrEqual,
///     value: 18,
/// };
///
/// assert_eq!(format!("{}", age_check), "age >= 18");
/// ```
///
/// ## Complex Condition
///
/// ```
/// use legalis_core::{Condition, ComparisonOp};
///
/// let age_check = Condition::Age {
///     operator: ComparisonOp::GreaterOrEqual,
///     value: 65,
/// };
/// let income_check = Condition::Income {
///     operator: ComparisonOp::LessThan,
///     value: 30000,
/// };
/// let eligibility = Condition::And(
///     Box::new(age_check),
///     Box::new(income_check),
/// );
///
/// assert!(format!("{}", eligibility).contains("AND"));
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum Condition {
    /// Age comparison (e.g., age >= 18)
    Age { operator: ComparisonOp, value: u32 },
    /// Income comparison
    Income { operator: ComparisonOp, value: u64 },
    /// Attribute existence check
    HasAttribute { key: String },
    /// Attribute value check
    AttributeEquals { key: String, value: String },
    /// Date range check (effective within date range)
    DateRange {
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
    },
    /// Geographic region check
    Geographic {
        region_type: RegionType,
        region_id: String,
    },
    /// Entity relationship check
    EntityRelationship {
        relationship_type: RelationshipType,
        target_entity_id: Option<String>,
    },
    /// Residency duration check
    ResidencyDuration { operator: ComparisonOp, months: u32 },
    /// Duration check (time periods, e.g., employment duration >= 5 years)
    Duration {
        operator: ComparisonOp,
        value: u32,
        unit: DurationUnit,
    },
    /// Percentage check (e.g., ownership >= 25%)
    Percentage {
        operator: ComparisonOp,
        value: u32,
        context: String,
    },
    /// Set membership check (e.g., status in {active, pending})
    SetMembership {
        attribute: String,
        values: Vec<String>,
        negated: bool,
    },
    /// Pattern matching check (regex for identifiers, codes, etc.)
    Pattern {
        attribute: String,
        pattern: String,
        negated: bool,
    },
    /// Calculation check (derived values, formulas)
    /// Example: `tax_owed = income * 0.2` where operator compares tax_owed
    Calculation {
        formula: String,
        operator: ComparisonOp,
        value: f64,
    },
    /// Composite condition - combines multiple conditions with weighted scoring
    /// Useful for complex eligibility where multiple factors contribute to a decision
    Composite {
        /// List of weighted conditions (weight, condition)
        /// Weights should be positive, typically 0.0-1.0 but not enforced
        conditions: Vec<(f64, Box<Condition>)>,
        /// Minimum total score required (sum of weights for satisfied conditions)
        threshold: f64,
    },
    /// Threshold condition - aggregate scoring across multiple numeric attributes
    /// Example: Combined income/asset test where total must exceed threshold
    Threshold {
        /// Attributes to sum (with optional multipliers)
        attributes: Vec<(String, f64)>,
        /// Comparison operator
        operator: ComparisonOp,
        /// Threshold value
        value: f64,
    },
    /// Fuzzy logic condition - membership in fuzzy set
    /// Supports gradual transitions between true/false
    Fuzzy {
        /// Attribute to evaluate
        attribute: String,
        /// Fuzzy set definition (value -> membership degree 0.0-1.0)
        /// For simplicity, uses linear interpolation between points
        membership_points: Vec<(f64, f64)>,
        /// Minimum membership degree required (0.0-1.0)
        min_membership: f64,
    },
    /// Probabilistic condition - probability-based evaluation
    /// Useful for modeling uncertain conditions or risk assessment
    Probabilistic {
        /// Base condition to evaluate
        condition: Box<Condition>,
        /// Probability that this condition is relevant (0.0-1.0)
        /// If p < 1.0, condition might be randomly evaluated as uncertain
        probability: f64,
        /// Minimum probability to consider condition satisfied
        threshold: f64,
    },
    /// Temporal condition - time-sensitive condition with decay/growth
    /// Value changes over time according to a decay or growth function
    Temporal {
        /// Base value at reference time
        base_value: f64,
        /// Reference timestamp (when base_value applies)
        reference_time: i64,
        /// Decay/growth rate per time unit (negative for decay, positive for growth)
        /// Applied as: value = base_value * (1 + rate)^time_elapsed
        rate: f64,
        /// Comparison operator
        operator: ComparisonOp,
        /// Target value to compare against
        target_value: f64,
    },
    /// Logical AND of conditions
    And(Box<Condition>, Box<Condition>),
    /// Logical OR of conditions
    Or(Box<Condition>, Box<Condition>),
    /// Logical NOT
    Not(Box<Condition>),
    /// Custom condition with description
    Custom { description: String },
}

impl Condition {
    /// Returns true if this is a compound condition (AND/OR/NOT).
    #[must_use]
    pub const fn is_compound(&self) -> bool {
        matches!(self, Self::And(..) | Self::Or(..) | Self::Not(..))
    }

    /// Returns true if this is a simple (non-compound) condition.
    #[must_use]
    pub const fn is_simple(&self) -> bool {
        !self.is_compound()
    }

    /// Returns true if this is a logical negation.
    #[must_use]
    pub const fn is_negation(&self) -> bool {
        matches!(self, Self::Not(..))
    }

    /// Counts the total number of conditions (including nested ones).
    #[must_use]
    pub fn count_conditions(&self) -> usize {
        match self {
            Self::And(left, right) | Self::Or(left, right) => {
                1 + left.count_conditions() + right.count_conditions()
            }
            Self::Not(inner) => 1 + inner.count_conditions(),
            Self::Composite { conditions, .. } => {
                1 + conditions
                    .iter()
                    .map(|(_, c)| c.count_conditions())
                    .sum::<usize>()
            }
            Self::Probabilistic { condition, .. } => 1 + condition.count_conditions(),
            _ => 1,
        }
    }

    /// Returns the depth of nested conditions.
    #[must_use]
    pub fn depth(&self) -> usize {
        match self {
            Self::And(left, right) | Self::Or(left, right) => 1 + left.depth().max(right.depth()),
            Self::Not(inner) => 1 + inner.depth(),
            Self::Composite { conditions, .. } => {
                1 + conditions.iter().map(|(_, c)| c.depth()).max().unwrap_or(0)
            }
            Self::Probabilistic { condition, .. } => 1 + condition.depth(),
            _ => 1,
        }
    }

    /// Creates a new Age condition.
    pub fn age(operator: ComparisonOp, value: u32) -> Self {
        Self::Age { operator, value }
    }

    /// Creates a new Income condition.
    pub fn income(operator: ComparisonOp, value: u64) -> Self {
        Self::Income { operator, value }
    }

    /// Creates a new HasAttribute condition.
    pub fn has_attribute(key: impl Into<String>) -> Self {
        Self::HasAttribute { key: key.into() }
    }

    /// Creates a new AttributeEquals condition.
    pub fn attribute_equals(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self::AttributeEquals {
            key: key.into(),
            value: value.into(),
        }
    }

    /// Creates a new Custom condition.
    pub fn custom(description: impl Into<String>) -> Self {
        Self::Custom {
            description: description.into(),
        }
    }

    /// Creates a new Duration condition.
    pub fn duration(operator: ComparisonOp, value: u32, unit: DurationUnit) -> Self {
        Self::Duration {
            operator,
            value,
            unit,
        }
    }

    /// Creates a new Percentage condition.
    pub fn percentage(operator: ComparisonOp, value: u32, context: impl Into<String>) -> Self {
        Self::Percentage {
            operator,
            value,
            context: context.into(),
        }
    }

    /// Creates a new SetMembership condition (attribute must be in set).
    pub fn in_set(attribute: impl Into<String>, values: Vec<String>) -> Self {
        Self::SetMembership {
            attribute: attribute.into(),
            values,
            negated: false,
        }
    }

    /// Creates a new SetMembership condition (attribute must NOT be in set).
    pub fn not_in_set(attribute: impl Into<String>, values: Vec<String>) -> Self {
        Self::SetMembership {
            attribute: attribute.into(),
            values,
            negated: true,
        }
    }

    /// Creates a new Pattern condition (attribute matches regex).
    pub fn matches_pattern(attribute: impl Into<String>, pattern: impl Into<String>) -> Self {
        Self::Pattern {
            attribute: attribute.into(),
            pattern: pattern.into(),
            negated: false,
        }
    }

    /// Creates a new Pattern condition (attribute does NOT match regex).
    pub fn not_matches_pattern(attribute: impl Into<String>, pattern: impl Into<String>) -> Self {
        Self::Pattern {
            attribute: attribute.into(),
            pattern: pattern.into(),
            negated: true,
        }
    }

    /// Creates a new Calculation condition (formula-based check).
    ///
    /// # Examples
    /// ```
    /// use legalis_core::{Condition, ComparisonOp};
    ///
    /// let tax_check = Condition::calculation("income * 0.2", ComparisonOp::GreaterThan, 5000.0);
    /// ```
    pub fn calculation(formula: impl Into<String>, operator: ComparisonOp, value: f64) -> Self {
        Self::Calculation {
            formula: formula.into(),
            operator,
            value,
        }
    }

    /// Creates a new Composite condition with weighted sub-conditions.
    ///
    /// # Arguments
    /// * `conditions` - Vector of (weight, condition) pairs
    /// * `threshold` - Minimum total score required
    ///
    /// # Example
    /// ```
    /// # use legalis_core::{Condition, ComparisonOp};
    /// let cond = Condition::composite(
    ///     vec![
    ///         (0.5, Box::new(Condition::age(ComparisonOp::GreaterOrEqual, 18))),
    ///         (0.3, Box::new(Condition::income(ComparisonOp::GreaterOrEqual, 30000))),
    ///     ],
    ///     0.6
    /// );
    /// ```
    pub fn composite(conditions: Vec<(f64, Box<Condition>)>, threshold: f64) -> Self {
        Self::Composite {
            conditions,
            threshold,
        }
    }

    /// Creates a new Threshold condition for aggregate scoring.
    ///
    /// # Arguments
    /// * `attributes` - Vector of (attribute_name, multiplier) pairs
    /// * `operator` - Comparison operator
    /// * `value` - Threshold value
    ///
    /// # Example
    /// ```
    /// # use legalis_core::{Condition, ComparisonOp};
    /// // Total assets (income + 10*savings) must be >= 50000
    /// let cond = Condition::threshold(
    ///     vec![("income".to_string(), 1.0), ("savings".to_string(), 10.0)],
    ///     ComparisonOp::GreaterOrEqual,
    ///     50000.0
    /// );
    /// ```
    pub fn threshold(attributes: Vec<(String, f64)>, operator: ComparisonOp, value: f64) -> Self {
        Self::Threshold {
            attributes,
            operator,
            value,
        }
    }

    /// Creates a new Fuzzy condition for gradual membership.
    ///
    /// # Arguments
    /// * `attribute` - Attribute to evaluate
    /// * `membership_points` - Vector of (value, membership_degree) pairs for linear interpolation
    /// * `min_membership` - Minimum membership degree required (0.0-1.0)
    ///
    /// # Example
    /// ```
    /// # use legalis_core::Condition;
    /// // Age is "young" with fuzzy membership
    /// let cond = Condition::fuzzy(
    ///     "age".to_string(),
    ///     vec![(0.0, 1.0), (25.0, 0.5), (50.0, 0.0)],
    ///     0.5
    /// );
    /// ```
    pub fn fuzzy(
        attribute: String,
        membership_points: Vec<(f64, f64)>,
        min_membership: f64,
    ) -> Self {
        Self::Fuzzy {
            attribute,
            membership_points,
            min_membership,
        }
    }

    /// Creates a new Probabilistic condition.
    ///
    /// # Arguments
    /// * `condition` - Base condition to evaluate
    /// * `probability` - Probability that this condition is relevant (0.0-1.0)
    /// * `threshold` - Minimum probability to consider satisfied
    ///
    /// # Example
    /// ```
    /// # use legalis_core::{Condition, ComparisonOp};
    /// // 80% chance that age >= 18 is relevant
    /// let cond = Condition::probabilistic(
    ///     Box::new(Condition::age(ComparisonOp::GreaterOrEqual, 18)),
    ///     0.8,
    ///     0.5
    /// );
    /// ```
    pub fn probabilistic(condition: Box<Condition>, probability: f64, threshold: f64) -> Self {
        Self::Probabilistic {
            condition,
            probability,
            threshold,
        }
    }

    /// Creates a new Temporal condition with decay/growth over time.
    ///
    /// # Arguments
    /// * `base_value` - Value at reference time
    /// * `reference_time` - Reference timestamp
    /// * `rate` - Decay/growth rate (negative for decay, positive for growth)
    /// * `operator` - Comparison operator
    /// * `target_value` - Target value to compare against
    ///
    /// # Example
    /// ```
    /// # use legalis_core::{Condition, ComparisonOp};
    /// // Asset value decays 5% per year, must stay above 10000
    /// let cond = Condition::temporal(
    ///     100000.0,
    ///     1609459200, // Jan 1, 2021
    ///     -0.05,
    ///     ComparisonOp::GreaterOrEqual,
    ///     10000.0
    /// );
    /// ```
    pub fn temporal(
        base_value: f64,
        reference_time: i64,
        rate: f64,
        operator: ComparisonOp,
        target_value: f64,
    ) -> Self {
        Self::Temporal {
            base_value,
            reference_time,
            rate,
            operator,
            target_value,
        }
    }

    /// Combines this condition with another using AND.
    pub fn and(self, other: Condition) -> Self {
        Self::And(Box::new(self), Box::new(other))
    }

    /// Combines this condition with another using OR.
    pub fn or(self, other: Condition) -> Self {
        Self::Or(Box::new(self), Box::new(other))
    }

    /// Negates this condition.
    #[allow(clippy::should_implement_trait)]
    pub fn not(self) -> Self {
        Self::Not(Box::new(self))
    }

    /// Normalizes this condition by applying logical simplifications.
    ///
    /// This method optimizes conditions by:
    /// - Removing double negations: `NOT (NOT A)` → `A`
    /// - Applying De Morgan's laws: `NOT (A AND B)` → `(NOT A) OR (NOT B)`
    /// - Recursively normalizing sub-conditions
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Condition, ComparisonOp};
    ///
    /// // Double negation elimination
    /// let condition = Condition::age(ComparisonOp::GreaterOrEqual, 18).not().not();
    /// let normalized = condition.normalize();
    /// // normalized is equivalent to: age >= 18
    /// ```
    #[must_use]
    pub fn normalize(self) -> Self {
        match self {
            // Double negation elimination: NOT (NOT A) → A
            Self::Not(inner) => match *inner {
                Self::Not(double_inner) => double_inner.normalize(),
                // De Morgan's laws
                Self::And(left, right) => {
                    // NOT (A AND B) → (NOT A) OR (NOT B)
                    Self::Or(
                        Box::new(Self::Not(left).normalize()),
                        Box::new(Self::Not(right).normalize()),
                    )
                }
                Self::Or(left, right) => {
                    // NOT (A OR B) → (NOT A) AND (NOT B)
                    Self::And(
                        Box::new(Self::Not(left).normalize()),
                        Box::new(Self::Not(right).normalize()),
                    )
                }
                other => Self::Not(Box::new(other.normalize())),
            },
            // Recursively normalize compound conditions
            Self::And(left, right) => {
                Self::And(Box::new(left.normalize()), Box::new(right.normalize()))
            }
            Self::Or(left, right) => {
                Self::Or(Box::new(left.normalize()), Box::new(right.normalize()))
            }
            // Simple conditions are already normalized
            other => other,
        }
    }

    /// Checks if this condition is in normalized form.
    #[must_use]
    pub fn is_normalized(&self) -> bool {
        match self {
            // Check for double negation
            Self::Not(inner) => !matches!(**inner, Self::Not(_)) && inner.is_normalized(),
            Self::And(left, right) | Self::Or(left, right) => {
                left.is_normalized() && right.is_normalized()
            }
            _ => true,
        }
    }

    /// Evaluates this condition with lazy evaluation and short-circuit logic.
    ///
    /// This method implements:
    /// - **Short-circuit AND**: Returns false as soon as any condition is false
    /// - **Short-circuit OR**: Returns true as soon as any condition is true
    /// - **Maximum depth protection**: Prevents stack overflow from deeply nested conditions
    ///
    /// # Arguments
    ///
    /// * `ctx` - Evaluation context containing entity data and settings
    ///
    /// # Errors
    ///
    /// Returns [`ConditionError`] if:
    /// - Required attributes are missing
    /// - Type mismatches occur
    /// - Formula evaluation fails
    /// - Maximum evaluation depth is exceeded
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Condition, ComparisonOp, AttributeBasedContext};
    /// use std::collections::HashMap;
    ///
    /// let mut attrs = HashMap::new();
    /// attrs.insert("age".to_string(), "25".to_string());
    /// attrs.insert("income".to_string(), "45000".to_string());
    ///
    /// let ctx = AttributeBasedContext::new(attrs);
    /// let condition = Condition::age(ComparisonOp::GreaterOrEqual, 18)
    ///     .and(Condition::income(ComparisonOp::LessThan, 50000));
    ///
    /// assert_eq!(condition.evaluate_simple(&ctx).unwrap(), true);
    /// ```
    pub fn evaluate_simple(&self, ctx: &AttributeBasedContext) -> Result<bool, ConditionError> {
        self.evaluate_simple_with_depth(ctx, 0)
    }

    /// Internal evaluation with depth tracking.
    fn evaluate_simple_with_depth(
        &self,
        ctx: &AttributeBasedContext,
        depth: usize,
    ) -> Result<bool, ConditionError> {
        // Protect against stack overflow from deeply nested conditions
        if depth > ctx.max_depth {
            return Err(ConditionError::MaxDepthExceeded {
                max_depth: ctx.max_depth,
            });
        }

        match self {
            // Lazy evaluation with short-circuit for AND
            Self::And(left, right) => {
                let left_result = left.evaluate_simple_with_depth(ctx, depth + 1)?;
                if !left_result {
                    // Short-circuit: if left is false, return false immediately
                    return Ok(false);
                }
                // Only evaluate right if left is true
                right.evaluate_simple_with_depth(ctx, depth + 1)
            }
            // Lazy evaluation with short-circuit for OR
            Self::Or(left, right) => {
                let left_result = left.evaluate_simple_with_depth(ctx, depth + 1)?;
                if left_result {
                    // Short-circuit: if left is true, return true immediately
                    return Ok(true);
                }
                // Only evaluate right if left is false
                right.evaluate_simple_with_depth(ctx, depth + 1)
            }
            Self::Not(inner) => {
                let result = inner.evaluate_simple_with_depth(ctx, depth + 1)?;
                Ok(!result)
            }
            Self::Age { operator, value } => {
                let age_str =
                    ctx.attributes
                        .get("age")
                        .ok_or_else(|| ConditionError::MissingAttribute {
                            key: "age".to_string(),
                        })?;
                let age: u32 = age_str.parse().map_err(|_| ConditionError::TypeMismatch {
                    expected: "u32".to_string(),
                    actual: age_str.clone(),
                })?;
                Ok(operator.compare_u32(age, *value))
            }
            Self::Income { operator, value } => {
                let income_str = ctx.attributes.get("income").ok_or_else(|| {
                    ConditionError::MissingAttribute {
                        key: "income".to_string(),
                    }
                })?;
                let income: u64 = income_str
                    .parse()
                    .map_err(|_| ConditionError::TypeMismatch {
                        expected: "u64".to_string(),
                        actual: income_str.clone(),
                    })?;
                Ok(operator.compare_u64(income, *value))
            }
            Self::HasAttribute { key } => Ok(ctx.attributes.contains_key(key)),
            Self::AttributeEquals { key, value } => Ok(ctx.attributes.get(key) == Some(value)),
            Self::Calculation {
                formula,
                operator,
                value,
            } => {
                // Simple formula evaluation (can be extended with a proper expression parser)
                let result = Self::evaluate_formula(formula, ctx)?;
                Ok(operator.compare_f64(result, *value))
            }
            Self::Pattern {
                attribute,
                pattern,
                negated,
            } => {
                let attr_value = ctx.attributes.get(attribute).ok_or_else(|| {
                    ConditionError::MissingAttribute {
                        key: attribute.clone(),
                    }
                })?;
                // Simple substring matching (can be extended with regex crate if needed)
                let matches = attr_value.contains(pattern);
                Ok(if *negated { !matches } else { matches })
            }
            // For other conditions, return Ok(true) as a placeholder
            // (implementation depends on specific evaluation logic)
            _ => Ok(true),
        }
    }

    /// Evaluates a simple formula.
    /// This is a basic implementation - can be extended with a proper expression parser.
    #[allow(dead_code)]
    fn evaluate_formula(
        formula: &str,
        _ctx: &AttributeBasedContext,
    ) -> Result<f64, ConditionError> {
        // Simple implementation: support basic arithmetic with attributes
        // For production use, consider using a proper expression parser like `meval` or `evalexpr`

        // For now, just return an error indicating formula evaluation needs implementation
        Err(ConditionError::InvalidFormula {
            formula: formula.to_string(),
            error: "Formula evaluation not yet implemented - consider using 'meval' or 'evalexpr' crate".to_string(),
        })
    }

    /// Evaluates this condition using the `EvaluationContext` trait.
    ///
    /// This is the trait-based evaluation method that allows custom context implementations.
    /// For a simpler attribute-based approach, see [`evaluate_simple`](Self::evaluate_simple).
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Condition, ComparisonOp, EvaluationContext, RegionType, RelationshipType, DurationUnit};
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
    ///
    /// let ctx = MyContext { age: 25, income: 45000 };
    /// let condition = Condition::age(ComparisonOp::GreaterOrEqual, 18)
    ///     .and(Condition::income(ComparisonOp::LessThan, 50000));
    ///
    /// assert_eq!(condition.evaluate(&ctx).unwrap(), true);
    /// ```
    pub fn evaluate<C: EvaluationContext>(&self, context: &C) -> Result<bool, EvaluationError> {
        self.evaluate_with_depth(context, 0)
    }

    /// Internal evaluation with depth tracking using the `EvaluationContext` trait.
    fn evaluate_with_depth<C: EvaluationContext>(
        &self,
        context: &C,
        depth: usize,
    ) -> Result<bool, EvaluationError> {
        const MAX_DEPTH: usize = 100;

        // Protect against stack overflow from deeply nested conditions
        if depth > MAX_DEPTH {
            return Err(EvaluationError::MaxDepthExceeded {
                max_depth: MAX_DEPTH,
            });
        }

        match self {
            // Lazy evaluation with short-circuit for AND
            Self::And(left, right) => {
                let left_result = left.evaluate_with_depth(context, depth + 1)?;
                if !left_result {
                    return Ok(false);
                }
                right.evaluate_with_depth(context, depth + 1)
            }
            // Lazy evaluation with short-circuit for OR
            Self::Or(left, right) => {
                let left_result = left.evaluate_with_depth(context, depth + 1)?;
                if left_result {
                    return Ok(true);
                }
                right.evaluate_with_depth(context, depth + 1)
            }
            Self::Not(inner) => {
                let result = inner.evaluate_with_depth(context, depth + 1)?;
                Ok(!result)
            }
            Self::Age { operator, value } => {
                let age = context
                    .get_age()
                    .ok_or_else(|| EvaluationError::MissingAttribute {
                        key: "age".to_string(),
                    })?;
                Ok(operator.compare_u32(age, *value))
            }
            Self::Income { operator, value } => {
                let income =
                    context
                        .get_income()
                        .ok_or_else(|| EvaluationError::MissingAttribute {
                            key: "income".to_string(),
                        })?;
                Ok(operator.compare_u64(income, *value))
            }
            Self::HasAttribute { key } => Ok(context.get_attribute(key).is_some()),
            Self::AttributeEquals { key, value } => {
                Ok(context.get_attribute(key).as_ref() == Some(value))
            }
            Self::Geographic {
                region_type,
                region_id,
            } => Ok(context.check_geographic(*region_type, region_id)),
            Self::EntityRelationship {
                relationship_type,
                target_entity_id,
            } => Ok(context.check_relationship(*relationship_type, target_entity_id.as_deref())),
            Self::ResidencyDuration { operator, months } => {
                let residency = context.get_residency_months().ok_or_else(|| {
                    EvaluationError::MissingContext {
                        description: "residency months".to_string(),
                    }
                })?;
                Ok(operator.compare_u32(residency, *months))
            }
            Self::Duration {
                operator,
                value,
                unit,
            } => {
                let duration =
                    context
                        .get_duration(*unit)
                        .ok_or_else(|| EvaluationError::MissingContext {
                            description: format!("duration for unit {:?}", unit),
                        })?;
                Ok(operator.compare_u32(duration, *value))
            }
            Self::Percentage {
                operator,
                value,
                context: pct_context,
            } => {
                let percentage = context.get_percentage(pct_context).ok_or_else(|| {
                    EvaluationError::MissingContext {
                        description: format!("percentage for context '{}'", pct_context),
                    }
                })?;
                Ok(operator.compare_u32(percentage, *value))
            }
            Self::Calculation {
                formula,
                operator,
                value,
            } => {
                let result = context.evaluate_formula(formula).ok_or_else(|| {
                    EvaluationError::InvalidFormula {
                        formula: formula.clone(),
                        reason: "Formula evaluation not supported".to_string(),
                    }
                })?;
                Ok(operator.compare_f64(result, *value))
            }
            Self::Pattern {
                attribute,
                pattern,
                negated,
            } => {
                let attr_value = context.get_attribute(attribute).ok_or_else(|| {
                    EvaluationError::MissingAttribute {
                        key: attribute.clone(),
                    }
                })?;
                let matches = attr_value.contains(pattern);
                Ok(if *negated { !matches } else { matches })
            }
            Self::SetMembership {
                attribute,
                values,
                negated,
            } => {
                let attr_value = context.get_attribute(attribute).ok_or_else(|| {
                    EvaluationError::MissingAttribute {
                        key: attribute.clone(),
                    }
                })?;
                let is_member = values.contains(&attr_value);
                Ok(if *negated { !is_member } else { is_member })
            }
            Self::DateRange { start, end } => {
                let current_date =
                    context
                        .get_current_date()
                        .ok_or_else(|| EvaluationError::MissingContext {
                            description: "current date".to_string(),
                        })?;
                let after_start = start.is_none_or(|s| current_date >= s);
                let before_end = end.is_none_or(|e| current_date <= e);
                Ok(after_start && before_end)
            }
            Self::Composite {
                conditions,
                threshold,
            } => {
                let mut total_score = 0.0;
                for (weight, condition) in conditions {
                    let satisfied = condition.evaluate_with_depth(context, depth + 1)?;
                    if satisfied {
                        total_score += weight;
                    }
                }
                Ok(total_score >= *threshold)
            }
            Self::Threshold {
                attributes,
                operator,
                value,
            } => {
                let mut total = 0.0;
                for (attr_name, multiplier) in attributes {
                    // Try to get attribute as f64
                    let attr_value = context
                        .get_attribute(attr_name)
                        .and_then(|s| s.parse::<f64>().ok())
                        .ok_or_else(|| EvaluationError::MissingAttribute {
                            key: attr_name.clone(),
                        })?;
                    total += attr_value * multiplier;
                }
                Ok(operator.compare_f64(total, *value))
            }
            Self::Fuzzy {
                attribute,
                membership_points,
                min_membership,
            } => {
                // Get attribute value
                let attr_value = context
                    .get_attribute(attribute)
                    .and_then(|s| s.parse::<f64>().ok())
                    .ok_or_else(|| EvaluationError::MissingAttribute {
                        key: attribute.clone(),
                    })?;

                // Linear interpolation of membership degree
                let membership = if membership_points.is_empty() {
                    0.0
                } else if membership_points.len() == 1 {
                    membership_points[0].1
                } else {
                    // Sort points by value
                    let mut sorted = membership_points.clone();
                    sorted
                        .sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

                    // Find interpolation range
                    if attr_value <= sorted[0].0 {
                        sorted[0].1
                    } else if attr_value >= sorted[sorted.len() - 1].0 {
                        sorted[sorted.len() - 1].1
                    } else {
                        // Linear interpolation
                        let mut result = 0.0;
                        for i in 0..sorted.len() - 1 {
                            if attr_value >= sorted[i].0 && attr_value <= sorted[i + 1].0 {
                                let (x0, y0) = sorted[i];
                                let (x1, y1) = sorted[i + 1];
                                let t = (attr_value - x0) / (x1 - x0);
                                result = y0 + t * (y1 - y0);
                                break;
                            }
                        }
                        result
                    }
                };

                Ok(membership >= *min_membership)
            }
            Self::Probabilistic {
                condition,
                probability,
                threshold,
            } => {
                // Evaluate the base condition
                let satisfied = condition.evaluate_with_depth(context, depth + 1)?;

                // If condition is satisfied, check if probability meets threshold
                // If not satisfied, probability is 0
                let effective_probability = if satisfied { *probability } else { 0.0 };
                Ok(effective_probability >= *threshold)
            }
            Self::Temporal {
                base_value,
                reference_time,
                rate,
                operator,
                target_value,
            } => {
                // Get current timestamp from context
                let current_time = context.get_current_timestamp().ok_or_else(|| {
                    EvaluationError::MissingContext {
                        description: "current timestamp".to_string(),
                    }
                })?;

                // Calculate time elapsed (in some unit, e.g., years)
                // Assuming timestamps are Unix timestamps (seconds)
                let time_elapsed =
                    (current_time - reference_time) as f64 / (365.25 * 24.0 * 3600.0); // Convert to years

                // Apply decay/growth: value = base_value * (1 + rate)^time_elapsed
                let current_value = base_value * (1.0 + rate).powf(time_elapsed);

                Ok(operator.compare_f64(current_value, *target_value))
            }
            // For Custom conditions, we can't evaluate without more context
            Self::Custom { description } => Err(EvaluationError::Custom {
                message: format!("Cannot evaluate custom condition: {}", description),
            }),
        }
    }

    /// Evaluates the condition with detailed step-by-step explanation.
    ///
    /// This method provides a full trace of the evaluation process, useful for:
    /// - Debugging complex conditions
    /// - Explaining legal decisions to users
    /// - Auditing and compliance
    /// - Educational purposes
    ///
    /// # Example
    /// ```
    /// # use legalis_core::{Condition, ComparisonOp, AttributeBasedContext};
    /// # use std::collections::HashMap;
    /// let mut attributes = HashMap::new();
    /// attributes.insert("age".to_string(), "25".to_string());
    /// attributes.insert("income".to_string(), "50000".to_string());
    /// let ctx = AttributeBasedContext::new(attributes);
    ///
    /// let age_check = Condition::age(ComparisonOp::GreaterOrEqual, 18);
    /// let income_check = Condition::income(ComparisonOp::GreaterOrEqual, 30000);
    /// let condition = age_check.and(income_check);
    ///
    /// let (result, explanation) = condition.evaluate_with_explanation(&ctx).unwrap();
    /// assert!(result);
    /// assert!(explanation.steps.len() >= 3); // AND, Age, Income checks
    /// assert!(explanation.conclusion);
    /// ```
    pub fn evaluate_with_explanation<C: EvaluationContext>(
        &self,
        context: &C,
    ) -> Result<(bool, EvaluationExplanation), EvaluationError> {
        let mut steps = Vec::new();
        let result = self.evaluate_with_explanation_recursive(context, 0, &mut steps)?;

        let explanation = EvaluationExplanation {
            condition: format!("{}", self),
            conclusion: result,
            steps,
        };

        Ok((result, explanation))
    }

    /// Internal recursive helper for evaluation with explanation.
    fn evaluate_with_explanation_recursive<C: EvaluationContext>(
        &self,
        context: &C,
        depth: usize,
        steps: &mut Vec<ExplanationStep>,
    ) -> Result<bool, EvaluationError> {
        const MAX_DEPTH: usize = 100;

        if depth > MAX_DEPTH {
            return Err(EvaluationError::MaxDepthExceeded {
                max_depth: MAX_DEPTH,
            });
        }

        let start_time = std::time::Instant::now();

        let (result, details) = match self {
            Self::And(left, right) => {
                let left_result =
                    left.evaluate_with_explanation_recursive(context, depth + 1, steps)?;
                if !left_result {
                    (
                        false,
                        "AND operation short-circuited (left operand is false)".to_string(),
                    )
                } else {
                    let right_result =
                        right.evaluate_with_explanation_recursive(context, depth + 1, steps)?;
                    (
                        right_result,
                        format!("AND: left={}, right={}", left_result, right_result),
                    )
                }
            }
            Self::Or(left, right) => {
                let left_result =
                    left.evaluate_with_explanation_recursive(context, depth + 1, steps)?;
                if left_result {
                    (
                        true,
                        "OR operation short-circuited (left operand is true)".to_string(),
                    )
                } else {
                    let right_result =
                        right.evaluate_with_explanation_recursive(context, depth + 1, steps)?;
                    (
                        right_result,
                        format!("OR: left={}, right={}", left_result, right_result),
                    )
                }
            }
            Self::Not(inner) => {
                let inner_result =
                    inner.evaluate_with_explanation_recursive(context, depth + 1, steps)?;
                (
                    !inner_result,
                    format!("NOT: inner={} -> {}", inner_result, !inner_result),
                )
            }
            Self::Age { operator, value } => {
                let age = context
                    .get_age()
                    .ok_or_else(|| EvaluationError::MissingAttribute {
                        key: "age".to_string(),
                    })?;
                let result = operator.compare_u32(age, *value);
                (
                    result,
                    format!("Age check: {} {} {} = {}", age, operator, value, result),
                )
            }
            Self::Income { operator, value } => {
                let income =
                    context
                        .get_income()
                        .ok_or_else(|| EvaluationError::MissingAttribute {
                            key: "income".to_string(),
                        })?;
                let result = operator.compare_u64(income, *value);
                (
                    result,
                    format!(
                        "Income check: {} {} {} = {}",
                        income, operator, value, result
                    ),
                )
            }
            Self::HasAttribute { key } => {
                let has_it = context.get_attribute(key).is_some();
                (has_it, format!("HasAttribute '{}': {}", key, has_it))
            }
            Self::AttributeEquals { key, value } => {
                let actual = context.get_attribute(key);
                let equals = actual.as_ref() == Some(value);
                (
                    equals,
                    format!(
                        "AttributeEquals '{}': expected='{}', actual={:?}, result={}",
                        key, value, actual, equals
                    ),
                )
            }
            _ => {
                // For other condition types, use regular evaluation
                let result = self.evaluate_with_depth(context, depth)?;
                (
                    result,
                    format!("Condition '{}' evaluated to {}", self, result),
                )
            }
        };

        let elapsed = start_time.elapsed().as_micros() as u64;

        steps.push(ExplanationStep {
            condition: format!("{}", self),
            result,
            details,
            depth,
            duration_micros: elapsed,
        });

        Ok(result)
    }

    /// Performs partial evaluation, allowing unknown values.
    ///
    /// Unlike `evaluate()`, this method can handle cases where some attributes
    /// or context values are unknown. It returns a three-valued logic result:
    /// - `PartialBool::True` - Definitely true
    /// - `PartialBool::False` - Definitely false
    /// - `PartialBool::Unknown` - Cannot determine (missing data)
    ///
    /// This is useful for:
    /// - Pre-checking eligibility with incomplete data
    /// - Planning data collection (what's missing?)
    /// - Optimistic evaluation strategies
    ///
    /// # Example
    /// ```
    /// # use legalis_core::{Condition, ComparisonOp, PartialBool, AttributeBasedContext};
    /// # use std::collections::HashMap;
    /// let mut attributes = HashMap::new();
    /// attributes.insert("age".to_string(), "25".to_string());
    /// // income is missing
    /// let ctx = AttributeBasedContext::new(attributes);
    ///
    /// let age_check = Condition::age(ComparisonOp::GreaterOrEqual, 18);
    /// let income_check = Condition::income(ComparisonOp::GreaterOrEqual, 30000);
    ///
    /// // Age check has data -> True
    /// assert!(matches!(age_check.partial_evaluate(&ctx), PartialBool::True { .. }));
    ///
    /// // Income check is missing data -> Unknown
    /// assert!(matches!(income_check.partial_evaluate(&ctx), PartialBool::Unknown { .. }));
    ///
    /// // AND with unknown propagates uncertainty
    /// let condition = age_check.and(income_check);
    /// assert!(matches!(condition.partial_evaluate(&ctx), PartialBool::Unknown { .. }));
    /// ```
    pub fn partial_evaluate<C: EvaluationContext>(&self, context: &C) -> PartialBool {
        self.partial_evaluate_with_depth(context, 0)
    }

    /// Internal recursive helper for partial evaluation.
    fn partial_evaluate_with_depth<C: EvaluationContext>(
        &self,
        context: &C,
        depth: usize,
    ) -> PartialBool {
        const MAX_DEPTH: usize = 100;

        if depth > MAX_DEPTH {
            return PartialBool::unknown(0.0, "Maximum depth exceeded");
        }

        match self {
            // Three-valued logic for AND with uncertainty propagation
            Self::And(left, right) => {
                let left_result = left.partial_evaluate_with_depth(context, depth + 1);
                let right_result = right.partial_evaluate_with_depth(context, depth + 1);

                match (&left_result, &right_result) {
                    (PartialBool::False { .. }, _) => left_result, // False AND anything = False
                    (_, PartialBool::False { .. }) => right_result, // anything AND False = False
                    (
                        PartialBool::True { confidence: c1, .. },
                        PartialBool::True { confidence: c2, .. },
                    ) => {
                        // Both true: confidence is minimum of both
                        PartialBool::true_with_confidence((*c1).min(*c2))
                    }
                    (
                        PartialBool::Unknown {
                            confidence: c1,
                            reason: r1,
                        },
                        PartialBool::Unknown {
                            confidence: c2,
                            reason: r2,
                        },
                    ) => {
                        // Both unknown: combine uncertainties
                        let combined_confidence = (*c1).min(*c2);
                        PartialBool::unknown(combined_confidence, &format!("{} AND {}", r1, r2))
                    }
                    _ => {
                        // One true, one unknown: result is unknown
                        PartialBool::unknown(0.5, "AND with unknown operand")
                    }
                }
            }
            // Three-valued logic for OR with uncertainty propagation
            Self::Or(left, right) => {
                let left_result = left.partial_evaluate_with_depth(context, depth + 1);
                let right_result = right.partial_evaluate_with_depth(context, depth + 1);

                match (&left_result, &right_result) {
                    (PartialBool::True { .. }, _) => left_result, // True OR anything = True
                    (_, PartialBool::True { .. }) => right_result, // anything OR True = True
                    (
                        PartialBool::False { confidence: c1, .. },
                        PartialBool::False { confidence: c2, .. },
                    ) => {
                        // Both false: confidence is minimum of both
                        PartialBool::false_with_confidence((*c1).min(*c2))
                    }
                    (
                        PartialBool::Unknown {
                            confidence: c1,
                            reason: r1,
                        },
                        PartialBool::Unknown {
                            confidence: c2,
                            reason: r2,
                        },
                    ) => {
                        // Both unknown: combine uncertainties
                        let combined_confidence = (*c1).min(*c2);
                        PartialBool::unknown(combined_confidence, &format!("{} OR {}", r1, r2))
                    }
                    _ => {
                        // One false, one unknown: result is unknown
                        PartialBool::unknown(0.5, "OR with unknown operand")
                    }
                }
            }
            // NOT inverts the partial value
            Self::Not(inner) => {
                let inner_result = inner.partial_evaluate_with_depth(context, depth + 1);
                match inner_result {
                    PartialBool::True { confidence, reason } => {
                        PartialBool::false_with_confidence_and_reason(
                            confidence,
                            &format!("NOT ({})", reason),
                        )
                    }
                    PartialBool::False { confidence, reason } => {
                        PartialBool::true_with_confidence_and_reason(
                            confidence,
                            &format!("NOT ({})", reason),
                        )
                    }
                    PartialBool::Unknown { confidence, reason } => {
                        PartialBool::unknown(confidence, &format!("NOT ({})", reason))
                    }
                }
            }
            // Simple conditions
            Self::Age { operator, value } => match context.get_age() {
                Some(age) => {
                    let result = operator.compare_u32(age, *value);
                    if result {
                        PartialBool::true_with_confidence(1.0)
                    } else {
                        PartialBool::false_with_confidence(1.0)
                    }
                }
                None => PartialBool::unknown(0.0, "age attribute missing"),
            },
            Self::Income { operator, value } => match context.get_income() {
                Some(income) => {
                    let result = operator.compare_u64(income, *value);
                    if result {
                        PartialBool::true_with_confidence(1.0)
                    } else {
                        PartialBool::false_with_confidence(1.0)
                    }
                }
                None => PartialBool::unknown(0.0, "income attribute missing"),
            },
            Self::HasAttribute { key } => match context.get_attribute(key) {
                Some(_) => PartialBool::true_with_confidence(1.0),
                None => PartialBool::false_with_confidence(1.0),
            },
            Self::AttributeEquals { key, value } => match context.get_attribute(key) {
                Some(actual) => {
                    if &actual == value {
                        PartialBool::true_with_confidence(1.0)
                    } else {
                        PartialBool::false_with_confidence(1.0)
                    }
                }
                None => PartialBool::unknown(0.0, &format!("attribute '{}' missing", key)),
            },
            Self::DateRange { start, end } => match context.get_current_date() {
                Some(current_date) => {
                    let after_start = start.is_none_or(|s| current_date >= s);
                    let before_end = end.is_none_or(|e| current_date <= e);
                    let result = after_start && before_end;
                    if result {
                        PartialBool::true_with_confidence(1.0)
                    } else {
                        PartialBool::false_with_confidence(1.0)
                    }
                }
                None => PartialBool::unknown(0.0, "current date missing"),
            },
            // For other complex conditions, try to evaluate or return unknown
            _ => match self.evaluate(context) {
                Ok(result) => {
                    if result {
                        PartialBool::true_with_confidence(1.0)
                    } else {
                        PartialBool::false_with_confidence(1.0)
                    }
                }
                Err(_) => PartialBool::unknown(0.0, "evaluation failed or data missing"),
            },
        }
    }
}

/// Context for evaluating conditions (simple attribute-based implementation).
///
/// Contains entity attributes and evaluation settings.
/// For a more flexible trait-based approach, see the `EvaluationContext` trait.
#[derive(Debug, Clone)]
pub struct AttributeBasedContext {
    /// Entity attributes as key-value pairs.
    pub attributes: HashMap<String, String>,
    /// Maximum evaluation depth to prevent stack overflow.
    pub max_depth: usize,
    /// Optional cache for memoizing condition evaluation results.
    pub cache: Option<ConditionCache>,
    /// Optional audit trail for tracking evaluation history.
    pub audit_trail: Option<EvaluationAuditTrail>,
}

impl AttributeBasedContext {
    /// Creates a new evaluation context with default max depth (100).
    #[must_use]
    pub fn new(attributes: HashMap<String, String>) -> Self {
        Self {
            attributes,
            max_depth: 100,
            cache: None,
            audit_trail: None,
        }
    }

    /// Creates a new evaluation context with custom max depth.
    #[must_use]
    pub fn with_max_depth(attributes: HashMap<String, String>, max_depth: usize) -> Self {
        Self {
            attributes,
            max_depth,
            cache: None,
            audit_trail: None,
        }
    }

    /// Creates a new evaluation context with caching enabled.
    #[must_use]
    pub fn with_cache(attributes: HashMap<String, String>) -> Self {
        Self {
            attributes,
            max_depth: 100,
            cache: Some(ConditionCache::new()),
            audit_trail: None,
        }
    }

    /// Creates a new evaluation context with custom max depth and cache capacity.
    #[must_use]
    pub fn with_cache_capacity(
        attributes: HashMap<String, String>,
        max_depth: usize,
        cache_capacity: usize,
    ) -> Self {
        Self {
            attributes,
            max_depth,
            cache: Some(ConditionCache::with_capacity(cache_capacity)),
            audit_trail: None,
        }
    }

    /// Creates a new evaluation context with audit trail enabled.
    #[must_use]
    pub fn with_audit_trail(attributes: HashMap<String, String>) -> Self {
        Self {
            attributes,
            max_depth: 100,
            cache: None,
            audit_trail: Some(EvaluationAuditTrail::new()),
        }
    }

    /// Records an evaluation in the audit trail if enabled.
    pub fn record_evaluation(&mut self, condition: &str, result: bool, duration_micros: u64) {
        if let Some(trail) = &mut self.audit_trail {
            trail.record(condition.to_string(), result, duration_micros);
        }
    }
}

/// Cache for memoizing condition evaluation results.
///
/// This cache improves performance when the same conditions are evaluated repeatedly
/// with the same entity attributes.
#[derive(Debug, Clone)]
pub struct ConditionCache {
    /// Cache storage mapping condition strings to evaluation results.
    cache: HashMap<String, bool>,
    /// Maximum number of entries to store (LRU eviction).
    max_capacity: usize,
    /// Access order for LRU eviction.
    access_order: Vec<String>,
}

impl ConditionCache {
    /// Creates a new cache with default capacity (1000).
    #[must_use]
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            max_capacity: 1000,
            access_order: Vec::new(),
        }
    }

    /// Creates a new cache with custom capacity.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            cache: HashMap::with_capacity(capacity),
            max_capacity: capacity,
            access_order: Vec::with_capacity(capacity),
        }
    }

    /// Gets a cached evaluation result if available.
    pub fn get(&mut self, condition_key: &str) -> Option<bool> {
        if let Some(&result) = self.cache.get(condition_key) {
            // Update access order (move to end for LRU)
            if let Some(pos) = self.access_order.iter().position(|k| k == condition_key) {
                self.access_order.remove(pos);
            }
            self.access_order.push(condition_key.to_string());
            Some(result)
        } else {
            None
        }
    }

    /// Stores an evaluation result in the cache.
    pub fn insert(&mut self, condition_key: String, result: bool) {
        // Evict oldest entry if at capacity
        if self.cache.len() >= self.max_capacity && !self.cache.contains_key(&condition_key) {
            if let Some(oldest_key) = self.access_order.first().cloned() {
                self.cache.remove(&oldest_key);
                self.access_order.remove(0);
            }
        }

        self.cache.insert(condition_key.clone(), result);
        self.access_order.push(condition_key);
    }

    /// Clears all cached entries.
    pub fn clear(&mut self) {
        self.cache.clear();
        self.access_order.clear();
    }

    /// Returns the number of cached entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Returns true if the cache is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    /// Returns cache hit rate (for performance monitoring).
    #[must_use]
    pub fn hit_rate(&self) -> f64 {
        // Note: This is a simplified implementation
        // For production, track hits/misses separately
        0.0
    }
}

impl Default for ConditionCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Audit trail for tracking condition evaluations.
///
/// Records each evaluation with timestamp, condition, result, and duration.
/// Useful for debugging, compliance, and performance analysis.
#[derive(Debug, Clone)]
pub struct EvaluationAuditTrail {
    /// List of evaluation records
    records: Vec<EvaluationRecord>,
    /// Maximum number of records to keep
    max_records: usize,
}

impl EvaluationAuditTrail {
    /// Creates a new audit trail with default capacity (1000 records).
    #[must_use]
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            max_records: 1000,
        }
    }

    /// Creates a new audit trail with custom capacity.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            records: Vec::with_capacity(capacity),
            max_records: capacity,
        }
    }

    /// Records an evaluation.
    pub fn record(&mut self, condition: String, result: bool, duration_micros: u64) {
        // Evict oldest record if at capacity
        if self.records.len() >= self.max_records {
            self.records.remove(0);
        }

        self.records.push(EvaluationRecord {
            timestamp: Utc::now(),
            condition,
            result,
            duration_micros,
        });
    }

    /// Returns all evaluation records.
    #[must_use]
    pub fn records(&self) -> &[EvaluationRecord] {
        &self.records
    }

    /// Returns the number of records.
    #[must_use]
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Returns true if there are no records.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Clears all records.
    pub fn clear(&mut self) {
        self.records.clear();
    }

    /// Returns average evaluation duration in microseconds.
    #[must_use]
    pub fn average_duration(&self) -> f64 {
        if self.records.is_empty() {
            return 0.0;
        }
        let total: u64 = self.records.iter().map(|r| r.duration_micros).sum();
        total as f64 / self.records.len() as f64
    }

    /// Returns the slowest evaluation record.
    #[must_use]
    pub fn slowest_evaluation(&self) -> Option<&EvaluationRecord> {
        self.records.iter().max_by_key(|r| r.duration_micros)
    }

    /// Returns records where evaluation took longer than threshold (microseconds).
    #[must_use]
    pub fn slow_evaluations(&self, threshold_micros: u64) -> Vec<&EvaluationRecord> {
        self.records
            .iter()
            .filter(|r| r.duration_micros > threshold_micros)
            .collect()
    }
}

impl Default for EvaluationAuditTrail {
    fn default() -> Self {
        Self::new()
    }
}

/// A single evaluation record in the audit trail.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct EvaluationRecord {
    /// When the evaluation occurred
    pub timestamp: DateTime<Utc>,
    /// The condition that was evaluated (as string)
    pub condition: String,
    /// The result of the evaluation
    pub result: bool,
    /// How long the evaluation took (microseconds)
    pub duration_micros: u64,
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

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Age { operator, value } => write!(f, "age {} {}", operator, value),
            Self::Income { operator, value } => write!(f, "income {} {}", operator, value),
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

/// Geographic region types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum RegionType {
    /// Country level
    Country,
    /// State/Province level
    State,
    /// City/Municipality level
    City,
    /// District/Ward level
    District,
    /// Postal/ZIP code area
    PostalCode,
    /// Custom region
    Custom,
}

/// Entity relationship types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum RelationshipType {
    /// Parent-child relationship
    ParentChild,
    /// Spousal relationship
    Spouse,
    /// Employment relationship
    Employment,
    /// Guardianship
    Guardian,
    /// Business ownership
    BusinessOwner,
    /// Contractual relationship
    Contractual,
}

/// Comparison operators for conditions.
///
/// Used in conditions to compare numeric values (age, income, duration, etc.).
///
/// # Examples
///
/// ```
/// use legalis_core::ComparisonOp;
///
/// let op = ComparisonOp::GreaterOrEqual;
/// assert_eq!(format!("{}", op), ">=");
///
/// let eq = ComparisonOp::Equal;
/// assert_eq!(format!("{}", eq), "==");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum ComparisonOp {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterOrEqual,
    LessThan,
    LessOrEqual,
}

impl ComparisonOp {
    /// Returns the inverse of this comparison operator.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::ComparisonOp;
    ///
    /// assert_eq!(ComparisonOp::GreaterThan.inverse(), ComparisonOp::LessOrEqual);
    /// assert_eq!(ComparisonOp::Equal.inverse(), ComparisonOp::NotEqual);
    /// ```
    #[must_use]
    pub const fn inverse(&self) -> Self {
        match self {
            Self::Equal => Self::NotEqual,
            Self::NotEqual => Self::Equal,
            Self::GreaterThan => Self::LessOrEqual,
            Self::GreaterOrEqual => Self::LessThan,
            Self::LessThan => Self::GreaterOrEqual,
            Self::LessOrEqual => Self::GreaterThan,
        }
    }

    /// Returns true if this is an equality check (Equal or NotEqual).
    #[must_use]
    pub const fn is_equality(&self) -> bool {
        matches!(self, Self::Equal | Self::NotEqual)
    }

    /// Returns true if this is an ordering comparison.
    #[must_use]
    pub const fn is_ordering(&self) -> bool {
        !self.is_equality()
    }

    /// Compares two u32 values using this operator.
    #[must_use]
    pub const fn compare_u32(&self, left: u32, right: u32) -> bool {
        match self {
            Self::Equal => left == right,
            Self::NotEqual => left != right,
            Self::GreaterThan => left > right,
            Self::GreaterOrEqual => left >= right,
            Self::LessThan => left < right,
            Self::LessOrEqual => left <= right,
        }
    }

    /// Compares two u64 values using this operator.
    #[must_use]
    pub const fn compare_u64(&self, left: u64, right: u64) -> bool {
        match self {
            Self::Equal => left == right,
            Self::NotEqual => left != right,
            Self::GreaterThan => left > right,
            Self::GreaterOrEqual => left >= right,
            Self::LessThan => left < right,
            Self::LessOrEqual => left <= right,
        }
    }

    /// Compares two i64 values using this operator.
    #[must_use]
    pub const fn compare_i64(&self, left: i64, right: i64) -> bool {
        match self {
            Self::Equal => left == right,
            Self::NotEqual => left != right,
            Self::GreaterThan => left > right,
            Self::GreaterOrEqual => left >= right,
            Self::LessThan => left < right,
            Self::LessOrEqual => left <= right,
        }
    }

    /// Compares two f64 values using this operator.
    #[must_use]
    pub fn compare_f64(&self, left: f64, right: f64) -> bool {
        match self {
            Self::Equal => (left - right).abs() < f64::EPSILON,
            Self::NotEqual => (left - right).abs() >= f64::EPSILON,
            Self::GreaterThan => left > right,
            Self::GreaterOrEqual => left >= right,
            Self::LessThan => left < right,
            Self::LessOrEqual => left <= right,
        }
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

// ==================================================
// Evaluation Context Trait for Flexible Condition Evaluation
// ==================================================

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

/// Errors that can occur during condition evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum EvaluationError {
    /// Missing required attribute for evaluation
    MissingAttribute { key: String },
    /// Missing required context data
    MissingContext { description: String },
    /// Invalid formula or calculation
    InvalidFormula { formula: String, reason: String },
    /// Pattern matching error
    PatternError { pattern: String, reason: String },
    /// Maximum evaluation depth exceeded (prevents infinite recursion)
    MaxDepthExceeded { max_depth: usize },
    /// Custom error
    Custom { message: String },
}

impl fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingAttribute { key } => write!(f, "Missing attribute: {}", key),
            Self::MissingContext { description } => write!(f, "Missing context: {}", description),
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

/// Context wrapper that provides default values for missing attributes.
///
/// This is useful for handling optional attributes with sensible defaults.
///
/// # Example
/// ```
/// # use legalis_core::{Condition, ComparisonOp, AttributeBasedContext, DefaultValueContext};
/// # use std::collections::HashMap;
/// let mut attributes = HashMap::new();
/// attributes.insert("name".to_string(), "Alice".to_string());
/// // age is missing
/// let entity = AttributeBasedContext::new(attributes);
///
/// let mut defaults = HashMap::new();
/// defaults.insert("age".to_string(), "18".to_string());
///
/// let ctx_with_defaults = DefaultValueContext::new(&entity, defaults);
///
/// // Will use default age of 18
/// let condition = Condition::age(ComparisonOp::GreaterOrEqual, 18);
/// assert!(condition.evaluate(&ctx_with_defaults).unwrap());
/// ```
#[derive(Debug)]
pub struct DefaultValueContext<'a, C: EvaluationContext> {
    inner: &'a C,
    defaults: HashMap<String, String>,
}

impl<'a, C: EvaluationContext> DefaultValueContext<'a, C> {
    /// Creates a new context with default values.
    pub fn new(inner: &'a C, defaults: HashMap<String, String>) -> Self {
        Self { inner, defaults }
    }

    /// Adds a default value for an attribute.
    pub fn with_default(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.defaults.insert(key.into(), value.into());
        self
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

/// Context wrapper that provides fallback evaluation strategies.
///
/// When the primary context cannot provide a value, it falls back to a secondary context.
///
/// # Example
/// ```
/// # use legalis_core::{Condition, ComparisonOp, AttributeBasedContext, FallbackContext, EvaluationContext};
/// # use std::collections::HashMap;
/// let mut primary_attrs = HashMap::new();
/// primary_attrs.insert("name".to_string(), "Alice".to_string());
/// let primary = AttributeBasedContext::new(primary_attrs);
///
/// let mut fallback_attrs = HashMap::new();
/// fallback_attrs.insert("age".to_string(), "25".to_string());
/// fallback_attrs.insert("name".to_string(), "Bob".to_string()); // Will not be used
/// let fallback = AttributeBasedContext::new(fallback_attrs);
///
/// let ctx = FallbackContext::new(&primary, &fallback);
///
/// // name comes from primary
/// assert_eq!(ctx.get_attribute("name"), Some("Alice".to_string()));
/// // age comes from fallback
/// assert_eq!(ctx.get_attribute("age"), Some("25".to_string()));
/// ```
#[derive(Debug)]
pub struct FallbackContext<'a, C1: EvaluationContext, C2: EvaluationContext> {
    primary: &'a C1,
    fallback: &'a C2,
}

impl<'a, C1: EvaluationContext, C2: EvaluationContext> FallbackContext<'a, C1, C2> {
    /// Creates a new context with fallback.
    pub fn new(primary: &'a C1, fallback: &'a C2) -> Self {
        Self { primary, fallback }
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
        // Basic formula evaluation - can be extended with a proper parser
        None
    }
}

/// Memoization cache for condition evaluation results.
///
/// Caches evaluation results to avoid re-evaluating the same conditions.
///
/// # Examples
///
/// ```
/// use legalis_core::{Condition, ComparisonOp, ConditionEvaluator};
/// use legalis_core::{EvaluationContext, RegionType, RelationshipType, DurationUnit};
/// use chrono::NaiveDate;
///
/// struct MyContext { age: u32 }
///
/// impl EvaluationContext for MyContext {
///     fn get_attribute(&self, _key: &str) -> Option<String> { None }
///     fn get_age(&self) -> Option<u32> { Some(self.age) }
///     fn get_income(&self) -> Option<u64> { None }
///     fn get_current_date(&self) -> Option<NaiveDate> { None }
///     fn check_geographic(&self, _region_type: RegionType, _region_id: &str) -> bool { false }
///     fn check_relationship(&self, _relationship_type: RelationshipType, _target_id: Option<&str>) -> bool { false }
///     fn get_residency_months(&self) -> Option<u32> { None }
///     fn get_duration(&self, _unit: DurationUnit) -> Option<u32> { None }
///     fn get_percentage(&self, _context: &str) -> Option<u32> { None }
///     fn evaluate_formula(&self, _formula: &str) -> Option<f64> { None }
/// }
///
/// let mut evaluator = ConditionEvaluator::new();
/// let ctx = MyContext { age: 25 };
/// let condition = Condition::age(ComparisonOp::GreaterOrEqual, 18);
///
/// // First evaluation - not cached
/// assert_eq!(evaluator.evaluate(&condition, &ctx).ok(), Some(true));
///
/// // Second evaluation - retrieved from cache
/// assert_eq!(evaluator.evaluate(&condition, &ctx).ok(), Some(true));
/// assert_eq!(evaluator.cache_hits(), 1);
/// ```
#[derive(Debug, Default)]
pub struct ConditionEvaluator {
    cache: std::collections::HashMap<String, bool>,
    cache_hits: usize,
    cache_misses: usize,
}

impl ConditionEvaluator {
    /// Creates a new condition evaluator with an empty cache.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Evaluates a condition with memoization.
    ///
    /// Results are cached based on the condition's string representation.
    pub fn evaluate<C: EvaluationContext>(
        &mut self,
        condition: &Condition,
        context: &C,
    ) -> Result<bool, EvaluationError> {
        let cache_key = format!("{}", condition);

        if let Some(&result) = self.cache.get(&cache_key) {
            self.cache_hits += 1;
            return Ok(result);
        }

        self.cache_misses += 1;
        let result = condition.evaluate(context)?;
        self.cache.insert(cache_key, result);
        Ok(result)
    }

    /// Clears the evaluation cache.
    pub fn clear_cache(&mut self) {
        self.cache.clear();
        self.cache_hits = 0;
        self.cache_misses = 0;
    }

    /// Returns the number of cache hits.
    #[must_use]
    pub const fn cache_hits(&self) -> usize {
        self.cache_hits
    }

    /// Returns the number of cache misses.
    #[must_use]
    pub const fn cache_misses(&self) -> usize {
        self.cache_misses
    }

    /// Returns the cache hit ratio (0.0 to 1.0).
    #[must_use]
    pub fn hit_ratio(&self) -> f64 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            0.0
        } else {
            self.cache_hits as f64 / total as f64
        }
    }
}

// ==================================================
// Parallel Condition Evaluation (requires "parallel" feature)
// ==================================================

#[cfg(feature = "parallel")]
use rayon::prelude::*;

impl Condition {
    /// Evaluates this condition with parallel processing for independent conditions.
    ///
    /// When the `parallel` feature is enabled, this method will evaluate independent
    /// And/Or branches in parallel for better performance on multi-core systems.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use legalis_core::{Condition, ComparisonOp, EvaluationContext, RegionType, RelationshipType, DurationUnit};
    /// use chrono::NaiveDate;
    ///
    /// struct MyContext { age: u32, income: u64 }
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
    ///
    /// let ctx = MyContext { age: 25, income: 45000 };
    ///
    /// // Complex condition with multiple independent checks
    /// let condition = Condition::age(ComparisonOp::GreaterOrEqual, 18)
    ///     .and(Condition::income(ComparisonOp::LessThan, 50000));
    ///
    /// // Evaluates branches in parallel when possible
    /// let result = condition.evaluate_parallel(&ctx);
    /// ```
    #[cfg(feature = "parallel")]
    pub fn evaluate_parallel<C: EvaluationContext + Sync>(
        &self,
        context: &C,
    ) -> Result<bool, EvaluationError> {
        self.evaluate_parallel_with_depth(context, 0, 100)
    }

    #[cfg(feature = "parallel")]
    #[allow(clippy::too_many_lines)]
    fn evaluate_parallel_with_depth<C: EvaluationContext + Sync>(
        &self,
        context: &C,
        depth: usize,
        max_depth: usize,
    ) -> Result<bool, EvaluationError> {
        if depth > max_depth {
            return Err(EvaluationError::MaxDepthExceeded { max_depth });
        }

        match self {
            // For simple conditions, use sequential evaluation
            Self::Age { .. }
            | Self::Income { .. }
            | Self::HasAttribute { .. }
            | Self::AttributeEquals { .. }
            | Self::DateRange { .. }
            | Self::Geographic { .. }
            | Self::EntityRelationship { .. }
            | Self::ResidencyDuration { .. }
            | Self::Duration { .. }
            | Self::Percentage { .. }
            | Self::SetMembership { .. }
            | Self::Pattern { .. }
            | Self::Calculation { .. }
            | Self::Threshold { .. }
            | Self::Fuzzy { .. }
            | Self::Temporal { .. }
            | Self::Custom { .. } => {
                // Delegate to sequential evaluation
                self.evaluate(context)
            }

            // Composite and Probabilistic have nested conditions, evaluate them recursively
            Self::Composite {
                conditions,
                threshold,
            } => {
                // Evaluate all conditions in parallel
                let results: Vec<_> = conditions
                    .par_iter()
                    .map(|(weight, cond)| {
                        cond.evaluate_parallel_with_depth(context, depth + 1, max_depth)
                            .map(|satisfied| if satisfied { *weight } else { 0.0 })
                    })
                    .collect();

                // Check for errors
                for result in &results {
                    if let Err(e) = result {
                        return Err(e.clone());
                    }
                }

                // Sum up the scores
                let total_score: f64 = results.iter().filter_map(|r| r.as_ref().ok()).sum();
                Ok(total_score >= *threshold)
            }

            Self::Probabilistic {
                condition,
                probability,
                threshold,
            } => {
                let satisfied =
                    condition.evaluate_parallel_with_depth(context, depth + 1, max_depth)?;
                let effective_probability = if satisfied { *probability } else { 0.0 };
                Ok(effective_probability >= *threshold)
            }

            // Parallel evaluation for compound conditions
            Self::And(left, right) => {
                // Evaluate both sides in parallel
                let (left_result, right_result) = rayon::join(
                    || left.evaluate_parallel_with_depth(context, depth + 1, max_depth),
                    || right.evaluate_parallel_with_depth(context, depth + 1, max_depth),
                );

                // Both must succeed and be true
                match (left_result, right_result) {
                    (Ok(true), Ok(true)) => Ok(true),
                    (Ok(false), _) | (_, Ok(false)) => Ok(false),
                    (Err(e), _) | (_, Err(e)) => Err(e),
                }
            }

            Self::Or(left, right) => {
                // Evaluate both sides in parallel
                let (left_result, right_result) = rayon::join(
                    || left.evaluate_parallel_with_depth(context, depth + 1, max_depth),
                    || right.evaluate_parallel_with_depth(context, depth + 1, max_depth),
                );

                // Either can be true
                match (left_result, right_result) {
                    (Ok(true), _) | (_, Ok(true)) => Ok(true),
                    (Ok(false), Ok(false)) => Ok(false),
                    (Err(e), _) | (_, Err(e)) => Err(e),
                }
            }

            Self::Not(inner) => {
                let result = inner.evaluate_parallel_with_depth(context, depth + 1, max_depth)?;
                Ok(!result)
            }
        }
    }

    /// Evaluates a collection of conditions in parallel.
    ///
    /// This is useful when you have multiple independent conditions to evaluate
    /// and want to leverage parallel processing.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use legalis_core::{Condition, ComparisonOp, EvaluationContext, RegionType, RelationshipType, DurationUnit};
    /// use chrono::NaiveDate;
    ///
    /// struct MyContext { age: u32, income: u64 }
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
    ///
    /// let ctx = MyContext { age: 25, income: 45000 };
    /// let conditions = vec![
    ///     Condition::age(ComparisonOp::GreaterOrEqual, 18),
    ///     Condition::income(ComparisonOp::LessThan, 50000),
    /// ];
    ///
    /// let results = Condition::evaluate_all_parallel(&conditions, &ctx);
    /// assert_eq!(results.len(), 2);
    /// ```
    #[cfg(feature = "parallel")]
    pub fn evaluate_all_parallel<C: EvaluationContext + Sync>(
        conditions: &[Condition],
        context: &C,
    ) -> Vec<Result<bool, EvaluationError>> {
        conditions
            .par_iter()
            .map(|cond| cond.evaluate_parallel(context))
            .collect()
    }
}

/// Parallel evaluation support for ConditionEvaluator.
#[cfg(feature = "parallel")]
impl ConditionEvaluator {
    /// Evaluates a condition with memoization and parallel processing.
    ///
    /// Note: The cache is not thread-safe, so this method requires mutable access.
    /// For truly concurrent evaluation, use separate evaluators per thread.
    pub fn evaluate_parallel<C: EvaluationContext + Sync>(
        &mut self,
        condition: &Condition,
        context: &C,
    ) -> Result<bool, EvaluationError> {
        let cache_key = format!("{}", condition);

        if let Some(&result) = self.cache.get(&cache_key) {
            self.cache_hits += 1;
            return Ok(result);
        }

        self.cache_misses += 1;
        let result = condition.evaluate_parallel(context)?;
        self.cache.insert(cache_key, result);
        Ok(result)
    }
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

        // Evaluate all preconditions in parallel
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

// ==================================================
// Statute Subsumption Checking
// ==================================================

/// Subsumption analyzer for determining if one statute subsumes another.
///
/// In legal reasoning, statute A subsumes statute B if:
/// - A and B have the same legal effect
/// - B's conditions are more specific than (or equal to) A's conditions
/// - Whenever B applies, A also applies (but not necessarily vice versa)
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, Condition, ComparisonOp, SubsumptionAnalyzer};
///
/// // General statute: anyone over 18 can vote
/// let general = Statute::new("vote-general", "Voting Rights", Effect::grant("vote"))
///     .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 18));
///
/// // Specific statute: citizens over 18 can vote (more specific)
/// let specific = Statute::new("vote-citizen", "Citizen Voting", Effect::grant("vote"))
///     .with_precondition(
///         Condition::age(ComparisonOp::GreaterOrEqual, 18)
///             .and(Condition::has_attribute("citizenship"))
///     );
///
/// // General subsumes specific
/// assert!(SubsumptionAnalyzer::subsumes(&general, &specific));
/// assert!(!SubsumptionAnalyzer::subsumes(&specific, &general));
/// ```
pub struct SubsumptionAnalyzer;

impl SubsumptionAnalyzer {
    /// Checks if statute A subsumes statute B.
    ///
    /// Returns `true` if B is more specific than A (A subsumes B).
    #[must_use]
    pub fn subsumes(a: &Statute, b: &Statute) -> bool {
        // Must have compatible effects
        if !Self::effects_compatible(&a.effect, &b.effect) {
            return false;
        }

        // If A has no preconditions, it subsumes everything with the same effect
        if a.preconditions.is_empty() {
            return true;
        }

        // If A has preconditions but B doesn't, A doesn't subsume B
        if b.preconditions.is_empty() {
            return false;
        }

        // Check if B's preconditions are more specific than A's
        Self::conditions_subsume(&a.preconditions, &b.preconditions)
    }

    /// Checks if effects are compatible for subsumption.
    fn effects_compatible(a: &Effect, b: &Effect) -> bool {
        // For basic subsumption, effects must be identical
        a.effect_type == b.effect_type && a.description == b.description
    }

    /// Checks if condition set A subsumes condition set B.
    ///
    /// Returns `true` if B is more specific (adds more constraints).
    fn conditions_subsume(a_conds: &[Condition], b_conds: &[Condition]) -> bool {
        // If B has more conditions than A, it might be more specific
        // We need to check if all of A's conditions are present in B

        for a_cond in a_conds {
            if !Self::condition_present_in(a_cond, b_conds) {
                return false;
            }
        }

        true
    }

    /// Checks if a single condition from A is present (or implied) in B's conditions.
    fn condition_present_in(a_cond: &Condition, b_conds: &[Condition]) -> bool {
        // Direct match
        if b_conds
            .iter()
            .any(|b_cond| Self::conditions_equivalent(a_cond, b_cond))
        {
            return true;
        }

        // Check if any of B's conditions are stricter versions of A's condition
        if b_conds
            .iter()
            .any(|b_cond| Self::condition_subsumes_condition(a_cond, b_cond))
        {
            return true;
        }

        // Check if the condition is present in a compound condition
        for b_cond in b_conds {
            if Self::condition_in_compound(a_cond, b_cond) {
                return true;
            }
        }

        false
    }

    /// Checks if condition A subsumes condition B (B is stricter than A).
    fn condition_subsumes_condition(a: &Condition, b: &Condition) -> bool {
        match (a, b) {
            // Age subsumption: age >= 18 subsumes age >= 21
            (
                Condition::Age {
                    operator: op_a,
                    value: val_a,
                },
                Condition::Age {
                    operator: op_b,
                    value: val_b,
                },
            ) => {
                match (op_a, op_b) {
                    // >= subsumes >= if B's value is higher
                    (ComparisonOp::GreaterOrEqual, ComparisonOp::GreaterOrEqual) => val_b >= val_a,
                    // <= subsumes <= if B's value is lower
                    (ComparisonOp::LessOrEqual, ComparisonOp::LessOrEqual) => val_b <= val_a,
                    _ => false,
                }
            }

            // Income subsumption
            (
                Condition::Income {
                    operator: op_a,
                    value: val_a,
                },
                Condition::Income {
                    operator: op_b,
                    value: val_b,
                },
            ) => match (op_a, op_b) {
                (ComparisonOp::LessThan, ComparisonOp::LessThan) => val_b <= val_a,
                (ComparisonOp::GreaterThan, ComparisonOp::GreaterThan) => val_b >= val_a,
                _ => false,
            },

            // Percentage subsumption
            (
                Condition::Percentage {
                    operator: op_a,
                    value: val_a,
                    context: ctx_a,
                },
                Condition::Percentage {
                    operator: op_b,
                    value: val_b,
                    context: ctx_b,
                },
            ) => {
                if ctx_a != ctx_b {
                    return false;
                }
                match (op_a, op_b) {
                    (ComparisonOp::GreaterOrEqual, ComparisonOp::GreaterOrEqual) => val_b >= val_a,
                    (ComparisonOp::LessOrEqual, ComparisonOp::LessOrEqual) => val_b <= val_a,
                    _ => false,
                }
            }

            // Compound conditions
            (Condition::And(a_left, a_right), Condition::And(b_left, b_right)) => {
                Self::condition_subsumes_condition(a_left, b_left)
                    && Self::condition_subsumes_condition(a_right, b_right)
            }

            _ => false,
        }
    }

    /// Checks if two conditions are logically equivalent.
    fn conditions_equivalent(a: &Condition, b: &Condition) -> bool {
        match (a, b) {
            (
                Condition::Age {
                    operator: op_a,
                    value: val_a,
                },
                Condition::Age {
                    operator: op_b,
                    value: val_b,
                },
            ) => op_a == op_b && val_a == val_b,
            (
                Condition::Income {
                    operator: op_a,
                    value: val_a,
                },
                Condition::Income {
                    operator: op_b,
                    value: val_b,
                },
            ) => op_a == op_b && val_a == val_b,
            (Condition::HasAttribute { key: key_a }, Condition::HasAttribute { key: key_b }) => {
                key_a == key_b
            }
            (
                Condition::AttributeEquals {
                    key: key_a,
                    value: val_a,
                },
                Condition::AttributeEquals {
                    key: key_b,
                    value: val_b,
                },
            ) => key_a == key_b && val_a == val_b,
            (
                Condition::Geographic {
                    region_type: rt_a,
                    region_id: rid_a,
                },
                Condition::Geographic {
                    region_type: rt_b,
                    region_id: rid_b,
                },
            ) => rt_a == rt_b && rid_a == rid_b,
            _ => false,
        }
    }

    /// Checks if a condition appears within a compound condition.
    fn condition_in_compound(target: &Condition, compound: &Condition) -> bool {
        match compound {
            Condition::And(left, right) | Condition::Or(left, right) => {
                Self::conditions_equivalent(target, left)
                    || Self::conditions_equivalent(target, right)
                    || Self::condition_subsumes_condition(target, left)
                    || Self::condition_subsumes_condition(target, right)
                    || Self::condition_in_compound(target, left)
                    || Self::condition_in_compound(target, right)
            }
            Condition::Not(inner) => {
                Self::conditions_equivalent(target, inner)
                    || Self::condition_in_compound(target, inner)
            }
            _ => false,
        }
    }

    /// Finds all statutes that are subsumed by the given statute.
    ///
    /// Returns statutes that are more specific than the given statute.
    #[must_use]
    pub fn find_subsumed<'a>(statute: &Statute, candidates: &'a [Statute]) -> Vec<&'a Statute> {
        candidates
            .iter()
            .filter(|candidate| candidate.id != statute.id && Self::subsumes(statute, candidate))
            .collect()
    }

    /// Finds all statutes that subsume the given statute.
    ///
    /// Returns statutes that are more general than the given statute.
    #[must_use]
    pub fn find_subsuming<'a>(statute: &Statute, candidates: &'a [Statute]) -> Vec<&'a Statute> {
        candidates
            .iter()
            .filter(|candidate| candidate.id != statute.id && Self::subsumes(candidate, statute))
            .collect()
    }
}

// ==================================================
// Legal Entailment Engine
// ==================================================

/// Result of applying a statute in the entailment process.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct EntailmentResult {
    /// The statute that was applied
    pub statute_id: String,
    /// The effect that was produced
    pub effect: Effect,
    /// Whether all preconditions were satisfied
    pub conditions_satisfied: bool,
    /// Evaluation errors if any
    pub errors: Vec<String>,
}

/// Legal entailment engine that determines what conclusions follow from statutes and facts.
///
/// Given a set of statutes and an evaluation context, the entailment engine:
/// 1. Evaluates each statute's preconditions
/// 2. Applies statutes whose conditions are met
/// 3. Returns the resulting legal effects
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, Condition, ComparisonOp};
/// use legalis_core::{EntailmentEngine, AttributeBasedContext};
/// use std::collections::HashMap;
///
/// let voting_statute = Statute::new("vote", "Voting Rights", Effect::grant("vote"))
///     .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 18));
///
/// let tax_credit = Statute::new("tax", "Tax Credit", Effect::grant("tax_credit"))
///     .with_precondition(Condition::income(ComparisonOp::LessThan, 50000));
///
/// let mut attributes = HashMap::new();
/// attributes.insert("age".to_string(), "25".to_string());
/// attributes.insert("income".to_string(), "45000".to_string());
/// let context = AttributeBasedContext::new(attributes);
///
/// let statutes = vec![voting_statute, tax_credit];
/// let engine = EntailmentEngine::new(statutes);
/// let results = engine.entail(&context);
///
/// // Both statutes apply
/// assert_eq!(results.len(), 2);
/// assert!(results.iter().all(|r| r.conditions_satisfied));
/// ```
#[derive(Debug, Clone)]
pub struct EntailmentEngine {
    statutes: Vec<Statute>,
}

impl EntailmentEngine {
    /// Creates a new entailment engine with the given statutes.
    #[must_use]
    pub fn new(statutes: Vec<Statute>) -> Self {
        Self { statutes }
    }

    /// Determines what legal effects follow from the statutes given the context.
    ///
    /// Returns all applicable effects where preconditions are satisfied.
    pub fn entail(&self, context: &AttributeBasedContext) -> Vec<EntailmentResult> {
        self.statutes
            .iter()
            .map(|statute| self.apply_statute(statute, context))
            .collect()
    }

    /// Determines what legal effects follow, filtering to only satisfied statutes.
    ///
    /// Returns only the effects where all preconditions are met.
    pub fn entail_satisfied(&self, context: &AttributeBasedContext) -> Vec<EntailmentResult> {
        self.entail(context)
            .into_iter()
            .filter(|result| result.conditions_satisfied)
            .collect()
    }

    /// Applies a single statute and returns the result.
    fn apply_statute(
        &self,
        statute: &Statute,
        context: &AttributeBasedContext,
    ) -> EntailmentResult {
        let mut errors = Vec::new();
        let mut all_satisfied = true;

        // If no preconditions, statute always applies
        if statute.preconditions.is_empty() {
            return EntailmentResult {
                statute_id: statute.id.clone(),
                effect: statute.effect.clone(),
                conditions_satisfied: true,
                errors: Vec::new(),
            };
        }

        // Evaluate all preconditions
        for condition in &statute.preconditions {
            match condition.evaluate_simple(context) {
                Ok(true) => {
                    // Condition satisfied
                }
                Ok(false) => {
                    all_satisfied = false;
                }
                Err(e) => {
                    all_satisfied = false;
                    errors.push(format!("{}", e));
                }
            }
        }

        EntailmentResult {
            statute_id: statute.id.clone(),
            effect: statute.effect.clone(),
            conditions_satisfied: all_satisfied,
            errors,
        }
    }

    /// Adds a statute to the engine.
    pub fn add_statute(&mut self, statute: Statute) {
        self.statutes.push(statute);
    }

    /// Removes a statute by ID.
    pub fn remove_statute(&mut self, statute_id: &str) -> bool {
        let original_len = self.statutes.len();
        self.statutes.retain(|s| s.id != statute_id);
        self.statutes.len() < original_len
    }

    /// Returns a reference to all statutes in the engine.
    #[must_use]
    pub fn statutes(&self) -> &[Statute] {
        &self.statutes
    }

    /// Returns the number of statutes in the engine.
    #[must_use]
    pub fn statute_count(&self) -> usize {
        self.statutes.len()
    }

    /// Checks if a statute exists in the engine by ID.
    #[must_use]
    pub fn has_statute(&self, statute_id: &str) -> bool {
        self.statutes.iter().any(|s| s.id == statute_id)
    }
}

/// Inference step in legal reasoning chains.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct InferenceStep {
    /// The statute applied in this step
    pub statute_id: String,
    /// The effect produced
    pub effect: Effect,
    /// Previous steps this inference depends on
    pub depends_on: Vec<usize>,
}

/// Forward chaining entailment with multi-step inference.
///
/// This engine can perform multi-step legal reasoning, where the effects
/// of one statute can enable the conditions of another statute.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, Condition, ComparisonOp};
/// use legalis_core::{ForwardChainingEngine, AttributeBasedContext};
/// use std::collections::HashMap;
///
/// // Step 1: Being 18+ grants eligibility
/// let eligibility = Statute::new("eligibility", "Eligibility", Effect::grant("eligible"))
///     .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 18));
///
/// // Step 2: Having eligibility grants voting rights
/// let voting = Statute::new("voting", "Voting", Effect::grant("vote"))
///     .with_precondition(Condition::has_attribute("eligible"));
///
/// let mut attributes = HashMap::new();
/// attributes.insert("age".to_string(), "25".to_string());
/// let context = AttributeBasedContext::new(attributes);
///
/// let statutes = vec![eligibility, voting];
/// let engine = ForwardChainingEngine::new(statutes);
/// let chain = engine.infer(&context, 5);
///
/// // Should derive both eligibility and voting rights
/// assert!(!chain.is_empty());
/// ```
#[derive(Debug, Clone)]
pub struct ForwardChainingEngine {
    statutes: Vec<Statute>,
}

impl ForwardChainingEngine {
    /// Creates a new forward chaining engine.
    #[must_use]
    pub fn new(statutes: Vec<Statute>) -> Self {
        Self { statutes }
    }

    /// Performs forward chaining inference up to max_steps.
    ///
    /// Returns the chain of inferences that can be derived.
    pub fn infer(&self, context: &AttributeBasedContext, max_steps: usize) -> Vec<InferenceStep> {
        let mut inferences = Vec::new();
        let mut changed = true;
        let mut steps = 0;

        while changed && steps < max_steps {
            changed = false;
            steps += 1;

            for statute in &self.statutes {
                // Skip if already inferred
                if inferences
                    .iter()
                    .any(|inf: &InferenceStep| inf.statute_id == statute.id)
                {
                    continue;
                }

                // Check if conditions are met
                if self.can_apply_statute(statute, context) {
                    let depends_on = self.find_dependencies(&inferences, statute);

                    inferences.push(InferenceStep {
                        statute_id: statute.id.clone(),
                        effect: statute.effect.clone(),
                        depends_on,
                    });

                    changed = true;
                }
            }
        }

        inferences
    }

    /// Checks if a statute's conditions can be applied given the current context.
    fn can_apply_statute(&self, statute: &Statute, context: &AttributeBasedContext) -> bool {
        if statute.preconditions.is_empty() {
            return true;
        }

        statute
            .preconditions
            .iter()
            .all(|cond| cond.evaluate_simple(context).unwrap_or(false))
    }

    /// Finds which previous inferences this statute depends on.
    fn find_dependencies(&self, _inferences: &[InferenceStep], _statute: &Statute) -> Vec<usize> {
        // For now, return empty dependencies
        // TODO: Implement dependency tracking based on which effects enable conditions
        Vec::new()
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

// ==================================================
// Abductive Reasoning Engine for Legal Outcome Explanation
// ==================================================

/// Explanation for why a legal outcome occurred.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct LegalExplanation {
    /// The observed outcome being explained
    pub outcome: Effect,
    /// Statutes that contributed to this outcome
    pub applicable_statutes: Vec<String>,
    /// Conditions that were satisfied
    pub satisfied_conditions: Vec<String>,
    /// Conditions that were not satisfied
    pub unsatisfied_conditions: Vec<String>,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Step-by-step reasoning chain
    pub reasoning_chain: Vec<ReasoningStep>,
}

/// A single step in the reasoning chain.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ReasoningStep {
    /// Step number
    pub step: usize,
    /// Description of this reasoning step
    pub description: String,
    /// Statute ID involved in this step
    pub statute_id: Option<String>,
    /// Condition evaluated in this step
    pub condition: Option<String>,
    /// Result of this step
    pub result: StepResult,
}

/// Result of a reasoning step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum StepResult {
    /// Condition was satisfied
    Satisfied,
    /// Condition was not satisfied
    NotSatisfied,
    /// Statute was applied
    Applied,
    /// Statute was not applicable
    NotApplicable,
    /// Uncertain result
    Uncertain,
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

/// Abductive reasoning engine for explaining legal outcomes.
///
/// This engine works backwards from an observed outcome to determine which
/// statutes and conditions led to that outcome.
///
/// # Examples
///
/// ```
/// use legalis_core::{AbductiveReasoner, Statute, Effect, EffectType, Condition, ComparisonOp};
/// use legalis_core::{EvaluationContext, RegionType, RelationshipType, DurationUnit};
/// use chrono::NaiveDate;
///
/// struct Person { age: u32, income: u64 }
///
/// impl EvaluationContext for Person {
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
///
/// let voting_law = Statute::new("vote", "Voting Rights", Effect::grant("vote"))
///     .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 18));
///
/// let statutes = vec![voting_law];
/// let person = Person { age: 25, income: 50000 };
///
/// let reasoner = AbductiveReasoner::new(statutes);
/// let explanations = reasoner.explain_effect(Effect::grant("vote"), &person);
///
/// assert!(!explanations.is_empty());
/// println!("{}", explanations[0]);
/// ```
#[derive(Debug, Clone)]
pub struct AbductiveReasoner {
    statutes: Vec<Statute>,
}

impl AbductiveReasoner {
    /// Creates a new abductive reasoner with the given statutes.
    #[must_use]
    pub fn new(statutes: Vec<Statute>) -> Self {
        Self { statutes }
    }

    /// Explains why a specific effect occurred.
    ///
    /// Returns all possible explanations ranked by confidence.
    pub fn explain_effect<C: EvaluationContext>(
        &self,
        target_effect: Effect,
        context: &C,
    ) -> Vec<LegalExplanation> {
        let mut explanations = Vec::new();

        // Find all statutes that produce this effect
        for statute in &self.statutes {
            if self.effects_match(&statute.effect, &target_effect) {
                if let Some(explanation) = self.explain_statute(statute, context) {
                    explanations.push(explanation);
                }
            }
        }

        // Sort by confidence (highest first)
        explanations.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());

        explanations
    }

    /// Explains why a specific statute was or was not applied.
    pub fn explain_statute<C: EvaluationContext>(
        &self,
        statute: &Statute,
        context: &C,
    ) -> Option<LegalExplanation> {
        let mut reasoning_chain = Vec::new();
        let mut satisfied_conditions = Vec::new();
        let mut unsatisfied_conditions = Vec::new();
        let mut step_num = 1;

        // If no preconditions, statute always applies
        if statute.preconditions.is_empty() {
            reasoning_chain.push(ReasoningStep {
                step: step_num,
                description: format!("Statute '{}' has no preconditions", statute.id),
                statute_id: Some(statute.id.clone()),
                condition: None,
                result: StepResult::Applied,
            });

            return Some(LegalExplanation {
                outcome: statute.effect.clone(),
                applicable_statutes: vec![statute.id.clone()],
                satisfied_conditions: vec!["No preconditions".to_string()],
                unsatisfied_conditions: Vec::new(),
                confidence: 1.0,
                reasoning_chain,
            });
        }

        // Evaluate each precondition
        for condition in &statute.preconditions {
            let condition_str = format!("{}", condition);

            match condition.evaluate(context) {
                Ok(true) => {
                    satisfied_conditions.push(condition_str.clone());
                    reasoning_chain.push(ReasoningStep {
                        step: step_num,
                        description: format!("Condition satisfied: {}", condition_str),
                        statute_id: Some(statute.id.clone()),
                        condition: Some(condition_str),
                        result: StepResult::Satisfied,
                    });
                }
                Ok(false) => {
                    unsatisfied_conditions.push(condition_str.clone());
                    reasoning_chain.push(ReasoningStep {
                        step: step_num,
                        description: format!("Condition not satisfied: {}", condition_str),
                        statute_id: Some(statute.id.clone()),
                        condition: Some(condition_str),
                        result: StepResult::NotSatisfied,
                    });
                }
                Err(_) => {
                    unsatisfied_conditions.push(condition_str.clone());
                    reasoning_chain.push(ReasoningStep {
                        step: step_num,
                        description: format!("Condition evaluation failed: {}", condition_str),
                        statute_id: Some(statute.id.clone()),
                        condition: Some(condition_str),
                        result: StepResult::Uncertain,
                    });
                }
            }

            step_num += 1;
        }

        // Calculate confidence based on satisfied conditions
        let total_conditions = statute.preconditions.len();
        let satisfied_count = satisfied_conditions.len();
        let confidence = if total_conditions > 0 {
            satisfied_count as f64 / total_conditions as f64
        } else {
            1.0
        };

        // Determine if statute was applied
        let all_satisfied = unsatisfied_conditions.is_empty();
        let applicable_statutes = if all_satisfied {
            vec![statute.id.clone()]
        } else {
            Vec::new()
        };

        reasoning_chain.push(ReasoningStep {
            step: step_num,
            description: if all_satisfied {
                format!("Statute '{}' applies", statute.id)
            } else {
                format!("Statute '{}' does not apply", statute.id)
            },
            statute_id: Some(statute.id.clone()),
            condition: None,
            result: if all_satisfied {
                StepResult::Applied
            } else {
                StepResult::NotApplicable
            },
        });

        Some(LegalExplanation {
            outcome: statute.effect.clone(),
            applicable_statutes,
            satisfied_conditions,
            unsatisfied_conditions,
            confidence,
            reasoning_chain,
        })
    }

    /// Explains why a specific outcome did NOT occur.
    ///
    /// This is useful for understanding what conditions would need to be satisfied
    /// for a desired outcome.
    pub fn explain_why_not<C: EvaluationContext>(
        &self,
        target_effect: Effect,
        context: &C,
    ) -> Vec<LegalExplanation> {
        let mut explanations = Vec::new();

        // Find statutes that could produce this effect but didn't
        for statute in &self.statutes {
            if self.effects_match(&statute.effect, &target_effect) {
                if let Some(explanation) = self.explain_statute(statute, context) {
                    // Only include if statute didn't apply (confidence < 1.0)
                    if explanation.confidence < 1.0 {
                        explanations.push(explanation);
                    }
                }
            }
        }

        // Sort by how close they came (highest confidence first)
        explanations.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());

        explanations
    }

    /// Checks if two effects match for explanation purposes.
    fn effects_match(&self, effect1: &Effect, effect2: &Effect) -> bool {
        effect1.effect_type == effect2.effect_type
            && effect1.description.contains(&effect2.description)
    }

    /// Finds alternative paths to achieve an outcome.
    ///
    /// Returns explanations for all statutes that could produce the target effect,
    /// showing which conditions need to be satisfied for each path.
    pub fn find_alternatives<C: EvaluationContext>(
        &self,
        target_effect: Effect,
        context: &C,
    ) -> Vec<LegalExplanation> {
        let mut alternatives = Vec::new();

        for statute in &self.statutes {
            if self.effects_match(&statute.effect, &target_effect) {
                if let Some(explanation) = self.explain_statute(statute, context) {
                    alternatives.push(explanation);
                }
            }
        }

        alternatives
    }
}

// ==================================================
// Statute Registry Query DSL
// ==================================================

/// Fluent query builder for searching and filtering statutes.
///
/// Provides a chainable API for constructing complex queries over statute collections.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, StatuteQuery, Condition, ComparisonOp};
/// use chrono::NaiveDate;
///
/// let statutes = vec![
///     Statute::new("law1", "Voting Rights", Effect::grant("vote"))
///         .with_jurisdiction("US")
///         .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 18)),
///     Statute::new("law2", "Tax Credit", Effect::grant("credit"))
///         .with_jurisdiction("US-CA")
///         .with_precondition(Condition::income(ComparisonOp::LessThan, 50000)),
/// ];
///
/// // Find all US statutes with preconditions
/// let results = StatuteQuery::new(&statutes)
///     .jurisdiction("US")
///     .with_preconditions()
///     .execute();
///
/// assert_eq!(results.len(), 1);
/// assert_eq!(results[0].id, "law1");
/// ```
pub struct StatuteQuery<'a> {
    statutes: &'a [Statute],
    #[allow(clippy::type_complexity)]
    filters: Vec<Box<dyn Fn(&Statute) -> bool + 'a>>,
}

impl<'a> StatuteQuery<'a> {
    /// Creates a new query over the given statute collection.
    #[must_use]
    pub fn new(statutes: &'a [Statute]) -> Self {
        Self {
            statutes,
            filters: Vec::new(),
        }
    }

    /// Filters statutes by jurisdiction.
    #[must_use]
    pub fn jurisdiction(mut self, jurisdiction: &'a str) -> Self {
        self.filters.push(Box::new(move |s| {
            s.jurisdiction.as_ref().is_some_and(|j| j == jurisdiction)
        }));
        self
    }

    /// Filters statutes by jurisdiction prefix (e.g., "US" matches "US", "US-CA", "US-NY").
    #[must_use]
    pub fn jurisdiction_prefix(mut self, prefix: &'a str) -> Self {
        self.filters.push(Box::new(move |s| {
            s.jurisdiction
                .as_ref()
                .is_some_and(|j| j.starts_with(prefix))
        }));
        self
    }

    /// Filters statutes by effect type.
    #[must_use]
    pub fn effect_type(mut self, effect_type: EffectType) -> Self {
        self.filters
            .push(Box::new(move |s| s.effect.effect_type == effect_type));
        self
    }

    /// Filters statutes that grant a specific right or privilege.
    #[must_use]
    pub fn grants(mut self, description: &'a str) -> Self {
        self.filters.push(Box::new(move |s| {
            s.effect.effect_type == EffectType::Grant && s.effect.description.contains(description)
        }));
        self
    }

    /// Filters statutes that revoke a specific right or privilege.
    #[must_use]
    pub fn revokes(mut self, description: &'a str) -> Self {
        self.filters.push(Box::new(move |s| {
            s.effect.effect_type == EffectType::Revoke && s.effect.description.contains(description)
        }));
        self
    }

    /// Filters statutes that have preconditions.
    #[must_use]
    pub fn with_preconditions(mut self) -> Self {
        self.filters.push(Box::new(|s| !s.preconditions.is_empty()));
        self
    }

    /// Filters statutes that have no preconditions (unconditional).
    #[must_use]
    pub fn unconditional(mut self) -> Self {
        self.filters.push(Box::new(|s| s.preconditions.is_empty()));
        self
    }

    /// Filters statutes by minimum number of preconditions.
    #[must_use]
    pub fn min_preconditions(mut self, min: usize) -> Self {
        self.filters
            .push(Box::new(move |s| s.preconditions.len() >= min));
        self
    }

    /// Filters statutes effective at a given date.
    #[must_use]
    pub fn effective_at(mut self, date: NaiveDate) -> Self {
        self.filters
            .push(Box::new(move |s| s.temporal_validity.is_active(date)));
        self
    }

    /// Filters statutes that are currently effective.
    #[must_use]
    pub fn currently_effective(mut self) -> Self {
        let today = chrono::Utc::now().date_naive();
        self.filters
            .push(Box::new(move |s| s.temporal_validity.is_active(today)));
        self
    }

    /// Filters statutes with a specific version.
    #[must_use]
    pub fn version(mut self, version: u32) -> Self {
        self.filters.push(Box::new(move |s| s.version == version));
        self
    }

    /// Filters statutes by ID prefix.
    #[must_use]
    pub fn id_prefix(mut self, prefix: &'a str) -> Self {
        self.filters
            .push(Box::new(move |s| s.id.starts_with(prefix)));
        self
    }

    /// Filters statutes by ID suffix.
    #[must_use]
    pub fn id_suffix(mut self, suffix: &'a str) -> Self {
        self.filters.push(Box::new(move |s| s.id.ends_with(suffix)));
        self
    }

    /// Filters statutes containing a keyword in title or ID.
    #[must_use]
    pub fn keyword(mut self, keyword: &'a str) -> Self {
        self.filters.push(Box::new(move |s| {
            s.id.contains(keyword) || s.title.contains(keyword)
        }));
        self
    }

    /// Filters statutes with a custom predicate.
    #[must_use]
    pub fn filter<F>(mut self, predicate: F) -> Self
    where
        F: Fn(&Statute) -> bool + 'a,
    {
        self.filters.push(Box::new(predicate));
        self
    }

    /// Executes the query and returns matching statutes.
    #[must_use]
    pub fn execute(self) -> Vec<&'a Statute> {
        self.statutes
            .iter()
            .filter(|statute| self.filters.iter().all(|f| f(statute)))
            .collect()
    }

    /// Executes the query and returns the first matching statute.
    #[must_use]
    pub fn first(self) -> Option<&'a Statute> {
        self.statutes
            .iter()
            .find(|statute| self.filters.iter().all(|f| f(statute)))
    }

    /// Executes the query and returns the count of matching statutes.
    #[must_use]
    pub fn count(self) -> usize {
        self.statutes
            .iter()
            .filter(|statute| self.filters.iter().all(|f| f(statute)))
            .count()
    }

    /// Executes the query and checks if any statutes match.
    #[must_use]
    pub fn exists(self) -> bool {
        self.statutes
            .iter()
            .any(|statute| self.filters.iter().all(|f| f(statute)))
    }
}

/// Statute registry for managing collections of statutes with query capabilities.
///
/// # Examples
///
/// ```
/// use legalis_core::{StatuteRegistry, Statute, Effect};
///
/// let mut registry = StatuteRegistry::new();
/// registry.add(Statute::new("law1", "Example Law", Effect::grant("right")));
/// registry.add(Statute::new("law2", "Another Law", Effect::revoke("privilege")));
///
/// assert_eq!(registry.len(), 2);
///
/// // Query the registry
/// let grants = registry.query().effect_type(legalis_core::EffectType::Grant).execute();
/// assert_eq!(grants.len(), 1);
/// ```
#[derive(Debug, Clone, Default)]
pub struct StatuteRegistry {
    statutes: Vec<Statute>,
}

impl StatuteRegistry {
    /// Creates a new empty statute registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a statute registry from a vector of statutes.
    #[must_use]
    pub fn from_statutes(statutes: Vec<Statute>) -> Self {
        Self { statutes }
    }

    /// Adds a statute to the registry.
    pub fn add(&mut self, statute: Statute) {
        self.statutes.push(statute);
    }

    /// Removes a statute by ID.
    ///
    /// Returns `true` if a statute was removed.
    pub fn remove(&mut self, id: &str) -> bool {
        let original_len = self.statutes.len();
        self.statutes.retain(|s| s.id != id);
        self.statutes.len() < original_len
    }

    /// Gets a statute by ID.
    #[must_use]
    pub fn get(&self, id: &str) -> Option<&Statute> {
        self.statutes.iter().find(|s| s.id == id)
    }

    /// Gets a mutable reference to a statute by ID.
    pub fn get_mut(&mut self, id: &str) -> Option<&mut Statute> {
        self.statutes.iter_mut().find(|s| s.id == id)
    }

    /// Returns the number of statutes in the registry.
    #[must_use]
    pub fn len(&self) -> usize {
        self.statutes.len()
    }

    /// Returns `true` if the registry is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.statutes.is_empty()
    }

    /// Returns an iterator over all statutes.
    pub fn iter(&self) -> impl Iterator<Item = &Statute> {
        self.statutes.iter()
    }

    /// Returns a mutable iterator over all statutes.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Statute> {
        self.statutes.iter_mut()
    }

    /// Creates a new query over the statutes in this registry.
    #[must_use]
    pub fn query(&self) -> StatuteQuery<'_> {
        StatuteQuery::new(&self.statutes)
    }

    /// Clears all statutes from the registry.
    pub fn clear(&mut self) {
        self.statutes.clear();
    }

    /// Returns all statutes as a vector.
    #[must_use]
    pub fn all(&self) -> &[Statute] {
        &self.statutes
    }

    /// Finds statutes that conflict with each other at a given date.
    #[must_use]
    pub fn find_conflicts(&self, date: NaiveDate) -> Vec<(&Statute, &Statute)> {
        let mut conflicts = Vec::new();
        let effective: Vec<_> = self
            .statutes
            .iter()
            .filter(|s| s.temporal_validity.is_active(date))
            .collect();

        for i in 0..effective.len() {
            for j in (i + 1)..effective.len() {
                let a = effective[i];
                let b = effective[j];

                // Simple conflict check: same effect type but different descriptions
                if a.effect.effect_type == b.effect.effect_type
                    && a.effect.description != b.effect.description
                    && !a.preconditions.is_empty()
                    && !b.preconditions.is_empty()
                {
                    conflicts.push((a, b));
                }
            }
        }

        conflicts
    }

    /// Merges another registry into this one.
    pub fn merge(&mut self, other: StatuteRegistry) {
        self.statutes.extend(other.statutes);
    }
}

impl IntoIterator for StatuteRegistry {
    type Item = Statute;
    type IntoIter = std::vec::IntoIter<Statute>;

    fn into_iter(self) -> Self::IntoIter {
        self.statutes.into_iter()
    }
}

impl<'a> IntoIterator for &'a StatuteRegistry {
    type Item = &'a Statute;
    type IntoIter = std::slice::Iter<'a, Statute>;

    fn into_iter(self) -> Self::IntoIter {
        self.statutes.iter()
    }
}

impl FromIterator<Statute> for StatuteRegistry {
    fn from_iter<T: IntoIterator<Item = Statute>>(iter: T) -> Self {
        Self {
            statutes: iter.into_iter().collect(),
        }
    }
}

/// Statute dependency graph for tracking relationships between statutes.
///
/// The `StatuteGraph` maintains a directed graph where nodes are statutes
/// and edges represent various relationships (derivation, amendments, cross-references).
///
/// # Examples
///
/// ```
/// use legalis_core::{StatuteGraph, Statute, Effect};
///
/// let mut graph = StatuteGraph::new();
///
/// let federal_law = Statute::new("federal-1", "Federal Law", Effect::grant("Benefit"));
/// let state_law = Statute::new("state-1", "State Law", Effect::grant("Benefit"))
///     .with_derives_from("federal-1");
///
/// graph.add_statute(federal_law);
/// graph.add_statute(state_law);
///
/// let derived = graph.find_derived_from("federal-1");
/// assert_eq!(derived.len(), 1);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct StatuteGraph {
    /// All statutes in the graph
    statutes: std::collections::HashMap<String, Statute>,
    /// Adjacency list: statute_id -> list of related statute IDs
    derivation_edges: std::collections::HashMap<String, Vec<String>>,
}

impl StatuteGraph {
    /// Creates a new empty statute graph.
    #[must_use]
    pub fn new() -> Self {
        Self {
            statutes: std::collections::HashMap::new(),
            derivation_edges: std::collections::HashMap::new(),
        }
    }

    /// Adds a statute to the graph.
    ///
    /// This automatically builds derivation edges based on the statute's `derives_from` field.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{StatuteGraph, Statute, Effect};
    ///
    /// let mut graph = StatuteGraph::new();
    /// let statute = Statute::new("law-1", "Law", Effect::grant("Benefit"));
    /// graph.add_statute(statute);
    ///
    /// assert_eq!(graph.len(), 1);
    /// ```
    pub fn add_statute(&mut self, statute: Statute) {
        let id = statute.id.clone();

        // Build derivation edges
        for source_id in &statute.derives_from {
            self.derivation_edges
                .entry(source_id.clone())
                .or_default()
                .push(id.clone());
        }

        self.statutes.insert(id, statute);
    }

    /// Returns the number of statutes in the graph.
    #[must_use]
    pub fn len(&self) -> usize {
        self.statutes.len()
    }

    /// Returns whether the graph is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.statutes.is_empty()
    }

    /// Gets a statute by ID.
    #[must_use]
    pub fn get(&self, id: &str) -> Option<&Statute> {
        self.statutes.get(id)
    }

    /// Finds all statutes derived from a given statute.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{StatuteGraph, Statute, Effect};
    ///
    /// let mut graph = StatuteGraph::new();
    ///
    /// graph.add_statute(Statute::new("parent", "Parent", Effect::grant("Benefit")));
    /// graph.add_statute(Statute::new("child", "Child", Effect::grant("Benefit"))
    ///     .with_derives_from("parent"));
    ///
    /// let derived = graph.find_derived_from("parent");
    /// assert_eq!(derived.len(), 1);
    /// assert_eq!(derived[0].id, "child");
    /// ```
    #[must_use]
    pub fn find_derived_from(&self, source_id: &str) -> Vec<&Statute> {
        self.derivation_edges
            .get(source_id)
            .map(|ids| ids.iter().filter_map(|id| self.statutes.get(id)).collect())
            .unwrap_or_default()
    }

    /// Finds all statutes that a given statute is derived from (its sources).
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{StatuteGraph, Statute, Effect};
    ///
    /// let mut graph = StatuteGraph::new();
    ///
    /// graph.add_statute(Statute::new("source-1", "Source 1", Effect::grant("B1")));
    /// graph.add_statute(Statute::new("source-2", "Source 2", Effect::grant("B2")));
    /// graph.add_statute(Statute::new("derived", "Derived", Effect::grant("B"))
    ///     .with_derives_from("source-1")
    ///     .with_derives_from("source-2"));
    ///
    /// let sources = graph.find_sources("derived");
    /// assert_eq!(sources.len(), 2);
    /// ```
    #[must_use]
    pub fn find_sources(&self, statute_id: &str) -> Vec<&Statute> {
        self.statutes
            .get(statute_id)
            .map(|statute| {
                statute
                    .derives_from
                    .iter()
                    .filter_map(|id| self.statutes.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Finds the transitive closure of all statutes derived from a given statute.
    ///
    /// This includes direct derivatives and all their derivatives recursively.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{StatuteGraph, Statute, Effect};
    ///
    /// let mut graph = StatuteGraph::new();
    ///
    /// graph.add_statute(Statute::new("root", "Root", Effect::grant("B")));
    /// graph.add_statute(Statute::new("child", "Child", Effect::grant("B"))
    ///     .with_derives_from("root"));
    /// graph.add_statute(Statute::new("grandchild", "Grandchild", Effect::grant("B"))
    ///     .with_derives_from("child"));
    ///
    /// let all_derived = graph.find_all_derived_from("root");
    /// assert_eq!(all_derived.len(), 2); // child and grandchild
    /// ```
    #[must_use]
    pub fn find_all_derived_from(&self, source_id: &str) -> Vec<&Statute> {
        let mut result = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut queue = vec![source_id];

        while let Some(current_id) = queue.pop() {
            if !visited.insert(current_id) {
                continue; // Already visited
            }

            if let Some(derived_ids) = self.derivation_edges.get(current_id) {
                for derived_id in derived_ids {
                    if let Some(statute) = self.statutes.get(derived_id) {
                        result.push(statute);
                        queue.push(derived_id);
                    }
                }
            }
        }

        result
    }

    /// Detects cycles in the derivation graph.
    ///
    /// Returns statute IDs that form derivation cycles (circular dependencies).
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{StatuteGraph, Statute, Effect};
    ///
    /// let mut graph = StatuteGraph::new();
    ///
    /// // Normal case: no cycles
    /// graph.add_statute(Statute::new("a", "A", Effect::grant("B")));
    /// graph.add_statute(Statute::new("b", "B", Effect::grant("B"))
    ///     .with_derives_from("a"));
    ///
    /// assert!(graph.detect_cycles().is_empty());
    /// ```
    #[must_use]
    pub fn detect_cycles(&self) -> Vec<Vec<String>> {
        let mut cycles = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut rec_stack = std::collections::HashSet::new();
        let mut path = Vec::new();

        for id in self.statutes.keys() {
            if !visited.contains(id.as_str()) {
                self.detect_cycles_dfs(id, &mut visited, &mut rec_stack, &mut path, &mut cycles);
            }
        }

        cycles
    }

    #[allow(clippy::too_many_arguments)]
    fn detect_cycles_dfs(
        &self,
        node: &str,
        visited: &mut std::collections::HashSet<String>,
        rec_stack: &mut std::collections::HashSet<String>,
        path: &mut Vec<String>,
        cycles: &mut Vec<Vec<String>>,
    ) {
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());
        path.push(node.to_string());

        if let Some(neighbors) = self.derivation_edges.get(node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    self.detect_cycles_dfs(neighbor, visited, rec_stack, path, cycles);
                } else if rec_stack.contains(neighbor) {
                    // Found a cycle
                    if let Some(pos) = path.iter().position(|x| x == neighbor) {
                        cycles.push(path[pos..].to_vec());
                    }
                }
            }
        }

        path.pop();
        rec_stack.remove(node);
    }

    /// Returns an iterator over all statutes in the graph.
    pub fn iter(&self) -> impl Iterator<Item = &Statute> {
        self.statutes.values()
    }
}

impl Default for StatuteGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Cross-jurisdiction statute equivalence detector.
///
/// This analyzer identifies statutes from different jurisdictions that serve
/// equivalent legal purposes, even if their exact wording differs.
///
/// # Examples
///
/// ```
/// use legalis_core::{CrossJurisdictionAnalyzer, Statute, Effect, Condition, ComparisonOp};
///
/// let us_law = Statute::new("us-voting", "Voting Rights", Effect::grant("Vote"))
///     .with_jurisdiction("US")
///     .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 18));
///
/// let uk_law = Statute::new("uk-voting", "Electoral Rights", Effect::grant("Vote"))
///     .with_jurisdiction("UK")
///     .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 18));
///
/// let analyzer = CrossJurisdictionAnalyzer::new();
/// let candidates = vec![uk_law.clone()];
/// let equiv = analyzer.find_equivalents(&us_law, &candidates);
///
/// assert_eq!(equiv.len(), 1);
/// ```
pub struct CrossJurisdictionAnalyzer {
    /// Similarity threshold (0.0 to 1.0)
    similarity_threshold: f64,
}

impl CrossJurisdictionAnalyzer {
    /// Creates a new cross-jurisdiction analyzer with default threshold (0.7).
    #[must_use]
    pub fn new() -> Self {
        Self {
            similarity_threshold: 0.7,
        }
    }

    /// Creates a new analyzer with a custom similarity threshold.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::CrossJurisdictionAnalyzer;
    ///
    /// let analyzer = CrossJurisdictionAnalyzer::with_threshold(0.8);
    /// ```
    #[must_use]
    pub fn with_threshold(threshold: f64) -> Self {
        Self {
            similarity_threshold: threshold.clamp(0.0, 1.0),
        }
    }

    /// Finds statutes from different jurisdictions that are equivalent to the given statute.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{CrossJurisdictionAnalyzer, Statute, Effect, Condition, ComparisonOp};
    ///
    /// let reference = Statute::new("ref", "Age Requirement", Effect::grant("Benefit"))
    ///     .with_jurisdiction("US")
    ///     .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 21));
    ///
    /// let candidate = Statute::new("can", "Age Eligibility", Effect::grant("Benefit"))
    ///     .with_jurisdiction("CA")
    ///     .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 21));
    ///
    /// let analyzer = CrossJurisdictionAnalyzer::new();
    /// let candidates = vec![candidate];
    /// let equivalents = analyzer.find_equivalents(&reference, &candidates);
    ///
    /// assert_eq!(equivalents.len(), 1);
    /// ```
    #[must_use]
    pub fn find_equivalents<'a>(
        &self,
        reference: &Statute,
        candidates: &'a [Statute],
    ) -> Vec<&'a Statute> {
        candidates
            .iter()
            .filter(|candidate| {
                // Don't compare statutes from the same jurisdiction
                if reference.jurisdiction == candidate.jurisdiction {
                    return false;
                }

                let similarity = self.calculate_similarity(reference, candidate);
                similarity >= self.similarity_threshold
            })
            .collect()
    }

    /// Calculates similarity score between two statutes (0.0 to 1.0).
    ///
    /// Higher scores indicate greater equivalence.
    #[must_use]
    pub fn calculate_similarity(&self, s1: &Statute, s2: &Statute) -> f64 {
        let mut score = 0.0;
        let mut weight_sum = 0.0;

        // Effect similarity (weight: 0.4)
        let effect_weight = 0.4;
        if s1.effect.effect_type == s2.effect.effect_type {
            score += effect_weight;
        }
        weight_sum += effect_weight;

        // Precondition count similarity (weight: 0.3)
        let precond_weight = 0.3;
        let precond_similarity = if s1.preconditions.is_empty() && s2.preconditions.is_empty() {
            1.0
        } else {
            let min_count = s1.preconditions.len().min(s2.preconditions.len()) as f64;
            let max_count = s1.preconditions.len().max(s2.preconditions.len()) as f64;
            if max_count == 0.0 {
                0.0
            } else {
                min_count / max_count
            }
        };
        score += precond_weight * precond_similarity;
        weight_sum += precond_weight;

        // Entity type similarity (weight: 0.3)
        let entity_weight = 0.3;
        let entity_similarity = if s1.applies_to.is_empty() && s2.applies_to.is_empty() {
            1.0 // Both apply to all entities
        } else {
            let common = s1
                .applies_to
                .iter()
                .filter(|t| s2.applies_to.contains(t))
                .count() as f64;
            let total = (s1.applies_to.len() + s2.applies_to.len()) as f64;
            if total == 0.0 {
                0.0
            } else {
                2.0 * common / total // Jaccard similarity
            }
        };
        score += entity_weight * entity_similarity;
        weight_sum += entity_weight;

        score / weight_sum
    }
}

impl Default for CrossJurisdictionAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Legal effect produced when statute conditions are met.
///
/// Effects represent the legal consequences that occur when a statute's conditions are satisfied.
/// They can include granting rights, imposing obligations, or changing legal status.
///
/// # Examples
///
/// ```
/// use legalis_core::{Effect, EffectType};
///
/// let grant = Effect::new(EffectType::Grant, "Right to vote")
///     .with_parameter("scope", "federal")
///     .with_parameter("duration", "permanent");
///
/// assert_eq!(grant.effect_type, EffectType::Grant);
/// assert_eq!(grant.parameters.get("scope"), Some(&"federal".to_string()));
/// ```
///
/// ```
/// use legalis_core::{Effect, EffectType};
///
/// let tax = Effect::new(EffectType::MonetaryTransfer, "Income tax")
///     .with_parameter("rate", "0.22")
///     .with_parameter("bracket", "middle");
///
/// assert!(format!("{}", tax).contains("Income tax"));
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Effect {
    /// Type of effect
    pub effect_type: EffectType,
    /// Description of the effect
    pub description: String,
    /// Parameters for the effect
    pub parameters: std::collections::HashMap<String, String>,
}

impl Effect {
    /// Creates a new Effect.
    pub fn new(effect_type: EffectType, description: impl Into<String>) -> Self {
        Self {
            effect_type,
            description: description.into(),
            parameters: std::collections::HashMap::new(),
        }
    }

    /// Adds a parameter to the effect.
    pub fn with_parameter(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.parameters.insert(key.into(), value.into());
        self
    }

    /// Gets a parameter value by key.
    #[must_use]
    pub fn get_parameter(&self, key: &str) -> Option<&String> {
        self.parameters.get(key)
    }

    /// Checks if a parameter exists.
    #[must_use]
    pub fn has_parameter(&self, key: &str) -> bool {
        self.parameters.contains_key(key)
    }

    /// Returns the number of parameters.
    #[must_use]
    pub fn parameter_count(&self) -> usize {
        self.parameters.len()
    }

    /// Removes a parameter by key.
    pub fn remove_parameter(&mut self, key: &str) -> Option<String> {
        self.parameters.remove(key)
    }

    /// Creates a Grant effect.
    pub fn grant(description: impl Into<String>) -> Self {
        Self::new(EffectType::Grant, description)
    }

    /// Creates a Revoke effect.
    pub fn revoke(description: impl Into<String>) -> Self {
        Self::new(EffectType::Revoke, description)
    }

    /// Creates an Obligation effect.
    pub fn obligation(description: impl Into<String>) -> Self {
        Self::new(EffectType::Obligation, description)
    }

    /// Creates a Prohibition effect.
    pub fn prohibition(description: impl Into<String>) -> Self {
        Self::new(EffectType::Prohibition, description)
    }

    /// Composes multiple effects with priority ordering.
    ///
    /// Creates a `ComposedEffect` that represents the combination of multiple effects.
    /// Effects are applied in the order specified, with earlier effects having higher priority
    /// for conflict resolution.
    ///
    /// # Example
    /// ```
    /// # use legalis_core::{Effect, EffectType, CompositionStrategy};
    /// let grant = Effect::grant("access to resource");
    /// let obligation = Effect::obligation("must report annually");
    /// let revoke = Effect::revoke("temporary access");
    ///
    /// let composed = Effect::compose(vec![grant, obligation, revoke]);
    /// assert_eq!(composed.effects.len(), 3);
    /// assert_eq!(composed.resolution_strategy, CompositionStrategy::FirstWins);
    /// ```
    pub fn compose(effects: Vec<Effect>) -> ComposedEffect {
        ComposedEffect::new(effects)
    }

    /// Computes the inverse effect for rollback operations.
    ///
    /// Returns the effect that would reverse this effect's action.
    /// For example, Grant ↔ Revoke, Obligation ↔ lifting of obligation.
    ///
    /// # Returns
    /// - `Some(Effect)` if an inverse exists
    /// - `None` if the effect cannot be inverted (e.g., Custom effects)
    ///
    /// # Example
    /// ```
    /// # use legalis_core::{Effect, EffectType};
    /// let grant = Effect::grant("access to resource");
    /// let inverse = grant.inverse().unwrap();
    /// assert_eq!(inverse.effect_type, EffectType::Revoke);
    /// assert_eq!(inverse.description, "access to resource");
    ///
    /// let obligation = Effect::obligation("must file taxes");
    /// let inverse_obligation = obligation.inverse().unwrap();
    /// assert_eq!(inverse_obligation.effect_type, EffectType::Grant);
    /// assert!(inverse_obligation.description.contains("relief from"));
    /// ```
    #[must_use]
    pub fn inverse(&self) -> Option<Effect> {
        let (inverse_type, inverse_description) = match self.effect_type {
            EffectType::Grant => (EffectType::Revoke, self.description.clone()),
            EffectType::Revoke => (EffectType::Grant, self.description.clone()),
            EffectType::Obligation => (
                EffectType::Grant,
                format!("relief from {}", self.description),
            ),
            EffectType::Prohibition => (
                EffectType::Grant,
                format!("permission for {}", self.description),
            ),
            EffectType::MonetaryTransfer => {
                // Reverse the monetary transfer (e.g., tax → refund)
                let desc = if self.description.contains("tax") {
                    self.description.replace("tax", "refund")
                } else if self.description.contains("fine") {
                    self.description.replace("fine", "reimbursement")
                } else {
                    format!("reverse {}", self.description)
                };
                (EffectType::MonetaryTransfer, desc)
            }
            EffectType::StatusChange => {
                // For status changes, we'd need to know the old status
                // Mark it as a reverse status change
                (
                    EffectType::StatusChange,
                    format!("reverse {}", self.description),
                )
            }
            EffectType::Custom => return None, // Cannot invert custom effects generically
        };

        let mut inverse = Effect::new(inverse_type, inverse_description);
        // Copy parameters but mark as inverse
        inverse.parameters = self.parameters.clone();
        inverse
            .parameters
            .insert("_is_inverse".to_string(), "true".to_string());
        inverse.parameters.insert(
            "_original_type".to_string(),
            format!("{:?}", self.effect_type),
        );

        Some(inverse)
    }

    /// Checks if this effect is an inverse of another effect.
    #[must_use]
    pub fn is_inverse_of(&self, other: &Effect) -> bool {
        if let Some(inv) = other.inverse() {
            self.effect_type == inv.effect_type
                && (self.description == inv.description
                    || self.description.contains(&other.description))
        } else {
            false
        }
    }

    /// Creates a temporal effect with start/end times and recurrence.
    ///
    /// Wraps this effect in a `TemporalEffect` that controls when the effect is active.
    ///
    /// # Example
    /// ```
    /// # use legalis_core::{Effect, RecurrencePattern};
    /// # use chrono::{NaiveDate, Utc};
    /// let grant = Effect::grant("seasonal parking permit");
    /// let start = NaiveDate::from_ymd_opt(2025, 6, 1).unwrap();
    /// let end = NaiveDate::from_ymd_opt(2025, 9, 1).unwrap();
    ///
    /// let temporal = grant.with_temporal_validity(start, Some(end), None);
    /// assert!(temporal.is_active_on(NaiveDate::from_ymd_opt(2025, 7, 15).unwrap()));
    /// assert!(!temporal.is_active_on(NaiveDate::from_ymd_opt(2025, 10, 1).unwrap()));
    /// ```
    #[must_use]
    pub fn with_temporal_validity(
        self,
        start: NaiveDate,
        end: Option<NaiveDate>,
        recurrence: Option<RecurrencePattern>,
    ) -> TemporalEffect {
        TemporalEffect::new(self, start, end, recurrence)
    }

    /// Creates a conditional effect that depends on runtime conditions.
    ///
    /// The effect will only be applied if the condition evaluates to true
    /// at the time of application.
    ///
    /// # Example
    /// ```
    /// # use legalis_core::{Effect, Condition, ComparisonOp};
    /// let grant = Effect::grant("bonus payment");
    /// let condition = Condition::income(ComparisonOp::GreaterOrEqual, 50000);
    ///
    /// let conditional = grant.when(condition);
    /// assert_eq!(conditional.effect.description, "bonus payment");
    /// ```
    #[must_use]
    pub fn when(self, condition: Condition) -> ConditionalEffect {
        ConditionalEffect::new(self, condition)
    }
}

impl fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.effect_type, self.description)
    }
}

/// Composed effect combining multiple effects with priority ordering.
///
/// When multiple effects need to be applied together, a ComposedEffect
/// provides conflict resolution strategies and ordering guarantees.
///
/// # Example
/// ```
/// # use legalis_core::{Effect, ComposedEffect, CompositionStrategy};
/// let effects = vec![
///     Effect::grant("resource access"),
///     Effect::obligation("annual reporting"),
/// ];
/// let composed = ComposedEffect::new(effects)
///     .with_resolution_strategy(CompositionStrategy::MostSpecific);
///
/// assert_eq!(composed.effects.len(), 2);
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ComposedEffect {
    /// The effects to be composed (applied in order).
    pub effects: Vec<Effect>,
    /// Strategy for resolving conflicts between effects.
    pub resolution_strategy: CompositionStrategy,
}

impl ComposedEffect {
    /// Creates a new composed effect with default conflict resolution (FirstWins).
    #[must_use]
    pub fn new(effects: Vec<Effect>) -> Self {
        Self {
            effects,
            resolution_strategy: CompositionStrategy::FirstWins,
        }
    }

    /// Sets the conflict resolution strategy.
    #[must_use]
    pub fn with_resolution_strategy(mut self, strategy: CompositionStrategy) -> Self {
        self.resolution_strategy = strategy;
        self
    }

    /// Adds an effect to the composition.
    pub fn add_effect(&mut self, effect: Effect) {
        self.effects.push(effect);
    }

    /// Returns the number of effects.
    #[must_use]
    pub fn len(&self) -> usize {
        self.effects.len()
    }

    /// Checks if there are no effects.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.effects.is_empty()
    }

    /// Resolves the composition to a single effective result.
    ///
    /// Applies the conflict resolution strategy to determine which effects take precedence.
    #[must_use]
    pub fn resolve(&self) -> Vec<&Effect> {
        match self.resolution_strategy {
            CompositionStrategy::FirstWins => {
                // First effect of each type wins
                let mut seen_types = std::collections::HashSet::new();
                self.effects
                    .iter()
                    .filter(|e| seen_types.insert(e.effect_type.clone()))
                    .collect()
            }
            CompositionStrategy::LastWins => {
                // Last effect of each type wins
                let mut result = std::collections::HashMap::new();
                for effect in &self.effects {
                    result.insert(effect.effect_type.clone(), effect);
                }
                result.values().copied().collect()
            }
            CompositionStrategy::MostSpecific => {
                // Prefer effects with more parameters (more specific)
                let mut result = std::collections::HashMap::new();
                for effect in &self.effects {
                    result
                        .entry(effect.effect_type.clone())
                        .and_modify(|e: &mut &Effect| {
                            if effect.parameter_count() > e.parameter_count() {
                                *e = effect;
                            }
                        })
                        .or_insert(effect);
                }
                result.values().copied().collect()
            }
            CompositionStrategy::AllApply => {
                // All effects apply (no conflict resolution)
                self.effects.iter().collect()
            }
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

/// Strategies for resolving conflicts between composed effects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum CompositionStrategy {
    /// First effect of each type wins (default).
    FirstWins,
    /// Last effect of each type wins (later overrides earlier).
    LastWins,
    /// Most specific effect wins (most parameters).
    MostSpecific,
    /// All effects apply (no deduplication).
    AllApply,
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

/// Effect with temporal validity constraints.
///
/// Wraps an effect with start/end dates and optional recurrence pattern.
/// The effect is only active during specified time periods.
///
/// # Example
/// ```
/// # use legalis_core::{Effect, TemporalEffect, RecurrencePattern};
/// # use chrono::NaiveDate;
/// let effect = Effect::grant("summer internship");
/// let start = NaiveDate::from_ymd_opt(2025, 6, 1).unwrap();
/// let end = NaiveDate::from_ymd_opt(2025, 8, 31).unwrap();
///
/// let temporal = TemporalEffect::new(effect, start, Some(end), None);
/// assert!(temporal.is_active_on(NaiveDate::from_ymd_opt(2025, 7, 15).unwrap()));
/// assert!(!temporal.is_active_on(NaiveDate::from_ymd_opt(2025, 9, 1).unwrap()));
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct TemporalEffect {
    /// The underlying effect.
    pub effect: Effect,
    /// Start date (inclusive).
    pub start_date: NaiveDate,
    /// End date (inclusive), if any.
    pub end_date: Option<NaiveDate>,
    /// Recurrence pattern, if any.
    pub recurrence: Option<RecurrencePattern>,
}

impl TemporalEffect {
    /// Creates a new temporal effect.
    #[must_use]
    pub fn new(
        effect: Effect,
        start_date: NaiveDate,
        end_date: Option<NaiveDate>,
        recurrence: Option<RecurrencePattern>,
    ) -> Self {
        Self {
            effect,
            start_date,
            end_date,
            recurrence,
        }
    }

    /// Checks if the effect is active on a given date.
    #[must_use]
    pub fn is_active_on(&self, date: NaiveDate) -> bool {
        // Check basic date range
        if date < self.start_date {
            return false;
        }
        if let Some(end) = self.end_date {
            if date > end {
                return false;
            }
        }

        // Check recurrence pattern if present
        if let Some(ref pattern) = self.recurrence {
            pattern.matches(date, self.start_date)
        } else {
            true
        }
    }

    /// Returns the next activation date after the given date.
    #[must_use]
    pub fn next_activation(&self, after: NaiveDate) -> Option<NaiveDate> {
        if let Some(ref pattern) = self.recurrence {
            pattern.next_occurrence(after, self.start_date, self.end_date)
        } else if after < self.start_date {
            Some(self.start_date)
        } else {
            None
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

/// Recurrence patterns for temporal effects.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum RecurrencePattern {
    /// Recurs daily.
    Daily,
    /// Recurs weekly (every N weeks).
    Weekly { interval: u32 },
    /// Recurs monthly (every N months, on same day).
    Monthly { interval: u32 },
    /// Recurs yearly (every N years, on same date).
    Yearly { interval: u32 },
    /// Recurs on specific days of week (0=Sunday, 6=Saturday).
    DaysOfWeek { days: Vec<u32> },
    /// Custom cron-like pattern (simplified).
    Custom { description: String },
}

impl RecurrencePattern {
    /// Checks if the pattern matches a given date.
    #[must_use]
    pub fn matches(&self, date: NaiveDate, start: NaiveDate) -> bool {
        match self {
            Self::Daily => true,
            Self::Weekly { interval } => {
                let days_diff = (date - start).num_days();
                days_diff >= 0 && days_diff % ((*interval as i64) * 7) == 0
            }
            Self::Monthly { interval } => {
                let months_diff = (date.year() - start.year()) * 12
                    + (date.month() as i32 - start.month() as i32);
                months_diff >= 0
                    && months_diff % (*interval as i32) == 0
                    && date.day() == start.day()
            }
            Self::Yearly { interval } => {
                let years_diff = date.year() - start.year();
                years_diff >= 0
                    && years_diff % (*interval as i32) == 0
                    && date.month() == start.month()
                    && date.day() == start.day()
            }
            Self::DaysOfWeek { days } => {
                let weekday = date.weekday().num_days_from_sunday();
                days.contains(&weekday)
            }
            Self::Custom { .. } => true, // Custom patterns need external evaluation
        }
    }

    /// Finds the next occurrence after a given date.
    #[must_use]
    pub fn next_occurrence(
        &self,
        after: NaiveDate,
        start: NaiveDate,
        end: Option<NaiveDate>,
    ) -> Option<NaiveDate> {
        let mut candidate = after.succ_opt()?;

        // Search up to 1 year ahead
        for _ in 0..365 {
            if let Some(end_date) = end {
                if candidate > end_date {
                    return None;
                }
            }
            if candidate >= start && self.matches(candidate, start) {
                return Some(candidate);
            }
            candidate = candidate.succ_opt()?;
        }
        None
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

/// Effect that depends on runtime conditions.
///
/// The effect is only applied if the condition evaluates to true.
/// This allows for dynamic, context-dependent effects.
///
/// # Example
/// ```
/// # use legalis_core::{Effect, ConditionalEffect, Condition, ComparisonOp, AttributeBasedContext};
/// # use std::collections::HashMap;
/// let effect = Effect::grant("senior discount");
/// let condition = Condition::age(ComparisonOp::GreaterOrEqual, 65);
/// let conditional = ConditionalEffect::new(effect, condition);
///
/// let mut attributes = HashMap::new();
/// attributes.insert("age".to_string(), "70".to_string());
/// let ctx = AttributeBasedContext::new(attributes);
///
/// assert!(conditional.should_apply(&ctx).unwrap());
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConditionalEffect {
    /// The effect to apply conditionally.
    pub effect: Effect,
    /// The condition that must be satisfied.
    pub condition: Condition,
}

impl ConditionalEffect {
    /// Creates a new conditional effect.
    #[must_use]
    pub fn new(effect: Effect, condition: Condition) -> Self {
        Self { effect, condition }
    }

    /// Checks if the effect should be applied given an evaluation context.
    pub fn should_apply<C: EvaluationContext>(&self, context: &C) -> Result<bool, EvaluationError> {
        self.condition.evaluate(context)
    }

    /// Applies the effect if the condition is met, returns the effect or None.
    pub fn apply_if<C: EvaluationContext>(
        &self,
        context: &C,
    ) -> Result<Option<&Effect>, EvaluationError> {
        if self.should_apply(context)? {
            Ok(Some(&self.effect))
        } else {
            Ok(None)
        }
    }
}

impl fmt::Display for ConditionalEffect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} WHEN {}", self.effect, self.condition)
    }
}

/// Effect dependency graph for tracking and detecting cycles.
///
/// Tracks dependencies between effects to ensure proper ordering
/// and detect circular dependencies.
///
/// # Example
/// ```
/// # use legalis_core::{Effect, EffectDependencyGraph};
/// let mut graph = EffectDependencyGraph::new();
/// let e1 = Effect::grant("base access");
/// let e2 = Effect::grant("extended access");
/// let e3 = Effect::obligation("reporting");
///
/// graph.add_effect("e1".to_string(), e1);
/// graph.add_effect("e2".to_string(), e2);
/// graph.add_effect("e3".to_string(), e3);
///
/// graph.add_dependency("e2", "e1"); // e2 depends on e1
/// graph.add_dependency("e3", "e2"); // e3 depends on e2
///
/// assert!(!graph.has_cycle());
/// assert_eq!(graph.topological_sort().unwrap(), vec!["e1", "e2", "e3"]);
/// ```
#[derive(Debug, Clone)]
pub struct EffectDependencyGraph {
    /// Effects indexed by ID.
    effects: std::collections::HashMap<String, Effect>,
    /// Dependencies: effect_id -> list of effect_ids it depends on.
    dependencies: std::collections::HashMap<String, Vec<String>>,
}

impl EffectDependencyGraph {
    /// Creates a new empty dependency graph.
    #[must_use]
    pub fn new() -> Self {
        Self {
            effects: std::collections::HashMap::new(),
            dependencies: std::collections::HashMap::new(),
        }
    }

    /// Adds an effect to the graph.
    pub fn add_effect(&mut self, id: String, effect: Effect) {
        self.effects.insert(id.clone(), effect);
        self.dependencies.entry(id).or_default();
    }

    /// Adds a dependency: `from` depends on `to`.
    ///
    /// Returns an error if it would create a cycle.
    pub fn add_dependency(&mut self, from: &str, to: &str) -> Result<(), String> {
        if !self.effects.contains_key(from) {
            return Err(format!("Effect '{}' not found", from));
        }
        if !self.effects.contains_key(to) {
            return Err(format!("Effect '{}' not found", to));
        }

        // Add the dependency
        self.dependencies
            .entry(from.to_string())
            .or_default()
            .push(to.to_string());

        // Check for cycles
        if self.has_cycle() {
            // Remove the dependency we just added
            if let Some(deps) = self.dependencies.get_mut(from) {
                deps.retain(|d| d != to);
            }
            return Err(format!(
                "Adding dependency {} -> {} would create a cycle",
                from, to
            ));
        }

        Ok(())
    }

    /// Checks if the graph contains a cycle.
    #[must_use]
    pub fn has_cycle(&self) -> bool {
        let mut visited = std::collections::HashSet::new();
        let mut rec_stack = std::collections::HashSet::new();

        for node in self.effects.keys() {
            if self.has_cycle_util(node, &mut visited, &mut rec_stack) {
                return true;
            }
        }
        false
    }

    /// Helper function for cycle detection (DFS).
    fn has_cycle_util(
        &self,
        node: &str,
        visited: &mut std::collections::HashSet<String>,
        rec_stack: &mut std::collections::HashSet<String>,
    ) -> bool {
        if rec_stack.contains(node) {
            return true;
        }
        if visited.contains(node) {
            return false;
        }

        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());

        if let Some(deps) = self.dependencies.get(node) {
            for dep in deps {
                if self.has_cycle_util(dep, visited, rec_stack) {
                    return true;
                }
            }
        }

        rec_stack.remove(node);
        false
    }

    /// Returns a topological sort of the effects (dependency order).
    ///
    /// Returns None if there's a cycle.
    #[must_use]
    pub fn topological_sort(&self) -> Option<Vec<String>> {
        if self.has_cycle() {
            return None;
        }

        let mut visited = std::collections::HashSet::new();
        let mut stack = Vec::new();

        for node in self.effects.keys() {
            if !visited.contains(node) {
                self.topological_sort_util(node, &mut visited, &mut stack);
            }
        }

        // Stack is already in correct dependency order (dependencies first)
        Some(stack)
    }

    /// Helper for topological sort (DFS).
    fn topological_sort_util(
        &self,
        node: &str,
        visited: &mut std::collections::HashSet<String>,
        stack: &mut Vec<String>,
    ) {
        visited.insert(node.to_string());

        if let Some(deps) = self.dependencies.get(node) {
            for dep in deps {
                if !visited.contains(dep) {
                    self.topological_sort_util(dep, visited, stack);
                }
            }
        }

        stack.push(node.to_string());
    }

    /// Gets an effect by ID.
    #[must_use]
    pub fn get_effect(&self, id: &str) -> Option<&Effect> {
        self.effects.get(id)
    }

    /// Returns the number of effects in the graph.
    #[must_use]
    pub fn len(&self) -> usize {
        self.effects.len()
    }

    /// Checks if the graph is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.effects.is_empty()
    }
}

impl Default for EffectDependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Types of legal effects.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum EffectType {
    /// Grant a right or permission
    Grant,
    /// Revoke a right or permission
    Revoke,
    /// Impose an obligation
    Obligation,
    /// Impose a prohibition
    Prohibition,
    /// Monetary transfer (subsidy, tax, fine, etc.)
    MonetaryTransfer,
    /// Status change
    StatusChange,
    /// Custom effect
    Custom,
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

/// Temporal validity for statutes.
///
/// Defines when a statute is in force, including effective dates, expiry dates (sunset clauses),
/// and amendment history.
///
/// # Examples
///
/// ```
/// use legalis_core::TemporalValidity;
/// use chrono::NaiveDate;
///
/// let validity = TemporalValidity::new()
///     .with_effective_date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
///     .with_expiry_date(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap());
///
/// let today = NaiveDate::from_ymd_opt(2025, 6, 15).unwrap();
/// assert!(validity.is_active(today));
///
/// let before = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
/// assert!(!validity.is_active(before));
///
/// let after = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
/// assert!(!validity.is_active(after));
/// ```
#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct TemporalValidity {
    /// Effective date (when the statute comes into force)
    pub effective_date: Option<NaiveDate>,
    /// Expiry date (sunset clause)
    pub expiry_date: Option<NaiveDate>,
    /// Enactment timestamp
    pub enacted_at: Option<DateTime<Utc>>,
    /// Last amended timestamp
    pub amended_at: Option<DateTime<Utc>>,
}

impl TemporalValidity {
    /// Creates a new TemporalValidity with no dates set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the effective date.
    pub fn with_effective_date(mut self, date: NaiveDate) -> Self {
        self.effective_date = Some(date);
        self
    }

    /// Sets the expiry date.
    pub fn with_expiry_date(mut self, date: NaiveDate) -> Self {
        self.expiry_date = Some(date);
        self
    }

    /// Sets the enacted timestamp.
    pub fn with_enacted_at(mut self, timestamp: DateTime<Utc>) -> Self {
        self.enacted_at = Some(timestamp);
        self
    }

    /// Sets the amended timestamp.
    pub fn with_amended_at(mut self, timestamp: DateTime<Utc>) -> Self {
        self.amended_at = Some(timestamp);
        self
    }

    /// Checks if the statute is currently active.
    pub fn is_active(&self, as_of: NaiveDate) -> bool {
        let after_effective = self.effective_date.is_none_or(|d| as_of >= d);
        let before_expiry = self.expiry_date.is_none_or(|d| as_of <= d);
        after_effective && before_expiry
    }

    /// Returns whether this has an effective date set.
    #[must_use]
    pub fn has_effective_date(&self) -> bool {
        self.effective_date.is_some()
    }

    /// Returns whether this has an expiry date set.
    #[must_use]
    pub fn has_expiry_date(&self) -> bool {
        self.expiry_date.is_some()
    }

    /// Returns whether this has been enacted (has an enacted_at timestamp).
    #[must_use]
    pub fn is_enacted(&self) -> bool {
        self.enacted_at.is_some()
    }

    /// Returns whether this has been amended.
    #[must_use]
    pub fn is_amended(&self) -> bool {
        self.amended_at.is_some()
    }

    /// Returns whether the statute has expired as of the given date.
    #[must_use]
    pub fn has_expired(&self, as_of: NaiveDate) -> bool {
        self.expiry_date.is_some_and(|exp| as_of > exp)
    }

    /// Returns whether the statute is not yet effective as of the given date.
    #[must_use]
    pub fn is_pending(&self, as_of: NaiveDate) -> bool {
        self.effective_date.is_some_and(|eff| as_of < eff)
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

/// Statute (legal article) definition.
///
/// A statute represents a legal rule with preconditions, effects, and optional discretionary logic.
/// Statutes follow an "If-Then-Else If Maybe" pattern:
/// - **If**: Preconditions must be met
/// - **Then**: Legal effect occurs
/// - **Else If Maybe**: Discretionary logic for edge cases
///
/// # Examples
///
/// ## Simple Statute
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
///
/// let voting_rights = Statute::new(
///     "voting-rights-act",
///     "Right to Vote",
///     Effect::new(EffectType::Grant, "Right to participate in elections"),
/// )
/// .with_precondition(Condition::Age {
///     operator: ComparisonOp::GreaterOrEqual,
///     value: 18,
/// })
/// .with_jurisdiction("US");
///
/// assert_eq!(voting_rights.id, "voting-rights-act");
/// assert!(voting_rights.is_valid());
/// ```
///
/// ## Statute with Temporal Validity
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType, TemporalValidity};
/// use chrono::NaiveDate;
///
/// let temporary_law = Statute::new(
///     "covid-relief-2025",
///     "COVID-19 Relief Act",
///     Effect::new(EffectType::Grant, "Emergency assistance"),
/// )
/// .with_temporal_validity(
///     TemporalValidity::new()
///         .with_effective_date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
///         .with_expiry_date(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap())
/// );
///
/// assert!(temporary_law.is_active(NaiveDate::from_ymd_opt(2025, 6, 1).unwrap()));
/// assert!(!temporary_law.is_active(NaiveDate::from_ymd_opt(2026, 1, 1).unwrap()));
/// ```
///
/// ## Statute with Discretion
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
///
/// let employment_termination = Statute::new(
///     "just-cause-termination",
///     "Employment Termination for Just Cause",
///     Effect::new(EffectType::Grant, "Right to terminate employment"),
/// )
/// .with_discretion("Determine if just cause exists based on circumstances");
///
/// assert!(employment_termination.discretion_logic.is_some());
/// ```
/// A structured exception to a statute's application.
///
/// Exceptions represent specific circumstances where a statute does not apply,
/// even when its preconditions would otherwise be satisfied.
///
/// # Examples
///
/// ```
/// use legalis_core::{StatuteException, Condition, ComparisonOp};
///
/// let exception = StatuteException::new(
///     "minor-exception",
///     "Exception for minors",
///     Condition::age(ComparisonOp::LessThan, 18)
/// );
///
/// assert_eq!(exception.id, "minor-exception");
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct StatuteException {
    /// Unique identifier for this exception
    pub id: String,
    /// Description of the exception
    pub description: String,
    /// Condition under which the exception applies
    pub condition: Condition,
}

impl StatuteException {
    /// Creates a new statute exception.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{StatuteException, Condition, ComparisonOp};
    ///
    /// let exception = StatuteException::new(
    ///     "medical-exception",
    ///     "Exception for medical emergencies",
    ///     Condition::has_attribute("medical_emergency")
    /// );
    /// ```
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        description: impl Into<String>,
        condition: Condition,
    ) -> Self {
        Self {
            id: id.into(),
            description: description.into(),
            condition,
        }
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

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Statute {
    /// Unique identifier (e.g., "civil-code-article-1")
    pub id: String,
    /// Title of the statute
    pub title: String,
    /// Preconditions (If)
    pub preconditions: Vec<Condition>,
    /// Legal effect (Then)
    pub effect: Effect,
    /// Discretion logic description (Else If Maybe)
    pub discretion_logic: Option<String>,
    /// Temporal validity (effective dates, sunset clauses)
    pub temporal_validity: TemporalValidity,
    /// Version number
    pub version: u32,
    /// Jurisdiction identifier
    pub jurisdiction: Option<String>,
    /// Derivation source - the statute(s) this one is derived from
    pub derives_from: Vec<String>,
    /// Applicable entity types - what types of entities this statute applies to
    pub applies_to: Vec<String>,
    /// Structured exceptions - conditions under which this statute does not apply
    pub exceptions: Vec<StatuteException>,
}

impl Statute {
    /// Creates a new Statute.
    pub fn new(id: impl Into<String>, title: impl Into<String>, effect: Effect) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            preconditions: Vec::new(),
            effect,
            discretion_logic: None,
            temporal_validity: TemporalValidity::default(),
            version: 1,
            jurisdiction: None,
            derives_from: Vec::new(),
            applies_to: Vec::new(),
            exceptions: Vec::new(),
        }
    }

    /// Adds a precondition.
    pub fn with_precondition(mut self, condition: Condition) -> Self {
        self.preconditions.push(condition);
        self
    }

    /// Sets the discretion logic.
    pub fn with_discretion(mut self, logic: impl Into<String>) -> Self {
        self.discretion_logic = Some(logic.into());
        self
    }

    /// Sets temporal validity.
    pub fn with_temporal_validity(mut self, validity: TemporalValidity) -> Self {
        self.temporal_validity = validity;
        self
    }

    /// Sets the version.
    pub fn with_version(mut self, version: u32) -> Self {
        self.version = version;
        self
    }

    /// Sets the jurisdiction.
    pub fn with_jurisdiction(mut self, jurisdiction: impl Into<String>) -> Self {
        self.jurisdiction = Some(jurisdiction.into());
        self
    }

    /// Adds a statute ID that this statute is derived from.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    ///
    /// let derived = Statute::new("state-law-1", "State Law", Effect::grant("Benefit"))
    ///     .with_derives_from("federal-law-1");
    ///
    /// assert_eq!(derived.derives_from, vec!["federal-law-1"]);
    /// ```
    pub fn with_derives_from(mut self, source_id: impl Into<String>) -> Self {
        self.derives_from.push(source_id.into());
        self
    }

    /// Adds an entity type that this statute applies to.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    ///
    /// let law = Statute::new("business-law-1", "Business Regulation", Effect::grant("License"))
    ///     .with_applies_to("Corporation")
    ///     .with_applies_to("LLC");
    ///
    /// assert!(law.applies_to.contains(&"Corporation".to_string()));
    /// ```
    pub fn with_applies_to(mut self, entity_type: impl Into<String>) -> Self {
        self.applies_to.push(entity_type.into());
        self
    }

    /// Adds an exception to this statute.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType, StatuteException, Condition, ComparisonOp};
    ///
    /// let law = Statute::new("tax-law-1", "Income Tax", Effect::grant("Tax liability"))
    ///     .with_exception(StatuteException::new(
    ///         "minor-exception",
    ///         "Minors are exempt",
    ///         Condition::age(ComparisonOp::LessThan, 18)
    ///     ));
    ///
    /// assert_eq!(law.exceptions.len(), 1);
    /// ```
    pub fn with_exception(mut self, exception: StatuteException) -> Self {
        self.exceptions.push(exception);
        self
    }

    /// Checks if the statute is currently active.
    pub fn is_active(&self, as_of: NaiveDate) -> bool {
        self.temporal_validity.is_active(as_of)
    }

    /// Returns the number of preconditions.
    #[must_use]
    pub fn precondition_count(&self) -> usize {
        self.preconditions.len()
    }

    /// Returns whether this statute has any preconditions.
    #[must_use]
    pub fn has_preconditions(&self) -> bool {
        !self.preconditions.is_empty()
    }

    /// Returns whether this statute has discretion logic.
    #[must_use]
    pub fn has_discretion(&self) -> bool {
        self.discretion_logic.is_some()
    }

    /// Returns whether this statute has a jurisdiction set.
    #[must_use]
    pub fn has_jurisdiction(&self) -> bool {
        self.jurisdiction.is_some()
    }

    /// Returns a reference to the preconditions.
    pub fn preconditions(&self) -> &[Condition] {
        &self.preconditions
    }

    /// Returns whether this statute is derived from other statutes.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect};
    ///
    /// let derived = Statute::new("derived", "Derived Law", Effect::grant("Benefit"))
    ///     .with_derives_from("source-law");
    ///
    /// assert!(derived.is_derived());
    /// ```
    #[must_use]
    pub fn is_derived(&self) -> bool {
        !self.derives_from.is_empty()
    }

    /// Returns the IDs of statutes this statute is derived from.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect};
    ///
    /// let derived = Statute::new("derived", "Derived Law", Effect::grant("Benefit"))
    ///     .with_derives_from("source-1")
    ///     .with_derives_from("source-2");
    ///
    /// assert_eq!(derived.derivation_sources(), &["source-1", "source-2"]);
    /// ```
    pub fn derivation_sources(&self) -> &[String] {
        &self.derives_from
    }

    /// Returns whether this statute applies to a specific entity type.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect};
    ///
    /// let law = Statute::new("law-1", "Law", Effect::grant("License"))
    ///     .with_applies_to("Corporation");
    ///
    /// assert!(law.applies_to_entity_type("Corporation"));
    /// assert!(!law.applies_to_entity_type("Individual"));
    /// ```
    #[must_use]
    pub fn applies_to_entity_type(&self, entity_type: &str) -> bool {
        self.applies_to.iter().any(|t| t == entity_type)
    }

    /// Returns whether this statute has any entity type restrictions.
    ///
    /// If this returns `false`, the statute applies to all entity types.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect};
    ///
    /// let general_law = Statute::new("law-1", "General Law", Effect::grant("Benefit"));
    /// assert!(!general_law.has_entity_restrictions());
    ///
    /// let specific_law = Statute::new("law-2", "Specific Law", Effect::grant("Benefit"))
    ///     .with_applies_to("Corporation");
    /// assert!(specific_law.has_entity_restrictions());
    /// ```
    #[must_use]
    pub fn has_entity_restrictions(&self) -> bool {
        !self.applies_to.is_empty()
    }

    /// Returns the entity types this statute applies to.
    pub fn applicable_entity_types(&self) -> &[String] {
        &self.applies_to
    }

    /// Returns whether this statute has exceptions.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, StatuteException, Condition, ComparisonOp};
    ///
    /// let law = Statute::new("law-1", "Law", Effect::grant("Benefit"))
    ///     .with_exception(StatuteException::new(
    ///         "exc-1",
    ///         "Exception",
    ///         Condition::age(ComparisonOp::LessThan, 18)
    ///     ));
    ///
    /// assert!(law.has_exceptions());
    /// ```
    #[must_use]
    pub fn has_exceptions(&self) -> bool {
        !self.exceptions.is_empty()
    }

    /// Returns a reference to the exceptions.
    pub fn exception_list(&self) -> &[StatuteException] {
        &self.exceptions
    }

    /// Returns the number of exceptions.
    #[must_use]
    pub fn exception_count(&self) -> usize {
        self.exceptions.len()
    }

    /// Validates the statute and returns a list of validation errors.
    pub fn validate(&self) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        // Validate ID
        if self.id.is_empty() {
            errors.push(ValidationError::EmptyId);
        } else if !self.is_valid_id(&self.id) {
            errors.push(ValidationError::InvalidId(self.id.clone()));
        }

        // Validate title
        if self.title.is_empty() {
            errors.push(ValidationError::EmptyTitle);
        }

        // Validate temporal consistency
        if let (Some(eff), Some(exp)) = (
            self.temporal_validity.effective_date,
            self.temporal_validity.expiry_date,
        ) {
            if exp < eff {
                errors.push(ValidationError::ExpiryBeforeEffective {
                    effective: eff,
                    expiry: exp,
                });
            }
        }

        // Validate preconditions
        for (i, cond) in self.preconditions.iter().enumerate() {
            if let Some(err) = Self::validate_condition(cond) {
                errors.push(ValidationError::InvalidCondition {
                    index: i,
                    message: err,
                });
            }
        }

        // Validate effect
        if self.effect.description.is_empty() {
            errors.push(ValidationError::EmptyEffectDescription);
        }

        // Validate version
        if self.version == 0 {
            errors.push(ValidationError::InvalidVersion);
        }

        errors
    }

    /// Returns true if the statute is valid (has no validation errors).
    pub fn is_valid(&self) -> bool {
        self.validate().is_empty()
    }

    /// Validates the statute and returns an error if invalid.
    pub fn validated(self) -> Result<Self, Vec<ValidationError>> {
        let errors = self.validate();
        if errors.is_empty() {
            Ok(self)
        } else {
            Err(errors)
        }
    }

    /// Checks if an ID is valid (alphanumeric with dashes/underscores).
    fn is_valid_id(&self, id: &str) -> bool {
        !id.is_empty()
            && id
                .chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
            && id.chars().next().is_some_and(|c| c.is_alphabetic())
    }

    /// Validates a condition recursively.
    fn validate_condition(condition: &Condition) -> Option<String> {
        match condition {
            Condition::Age { value, .. } => {
                if *value > 150 {
                    Some(format!("Unrealistic age value: {}", value))
                } else {
                    None
                }
            }
            Condition::And(left, right) | Condition::Or(left, right) => {
                Self::validate_condition(left).or_else(|| Self::validate_condition(right))
            }
            Condition::Not(inner) => Self::validate_condition(inner),
            Condition::ResidencyDuration { months, .. } => {
                if *months > 1200 {
                    Some(format!("Unrealistic residency duration: {} months", months))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Checks if this statute subsumes another statute.
    ///
    /// Statute A subsumes statute B if:
    /// - A's preconditions are more general than (or equal to) B's preconditions
    /// - A's effect is the same or broader than B's effect
    /// - Whenever B applies, A also applies
    ///
    /// This is useful for detecting redundancy and logical relationships between statutes.
    ///
    /// **Note**: This is a simplified heuristic-based implementation.
    /// Full subsumption checking would require logical analysis of condition relationships
    /// (e.g., recognizing that age >= 18 subsumes age >= 21).
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
    ///
    /// // Statute with no preconditions subsumes one with preconditions (same effect)
    /// let general = Statute::new("general", "Voting Rights", Effect::grant("Vote"));
    ///
    /// let specific = Statute::new("specific", "Voting Rights (21+)", Effect::grant("Vote"))
    ///     .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 21));
    ///
    /// // General (no conditions) subsumes specific (has conditions)
    /// assert_eq!(general.subsumes(&specific), true);
    /// assert_eq!(specific.subsumes(&general), false);
    /// ```
    #[must_use]
    pub fn subsumes(&self, other: &Self) -> bool {
        // For a basic implementation, check if:
        // 1. Effects are compatible (same type and description)
        // 2. This statute's preconditions are more general

        // Check effect compatibility
        if self.effect.effect_type != other.effect.effect_type
            || self.effect.description != other.effect.description
        {
            return false;
        }

        // Check jurisdiction compatibility
        if self.jurisdiction != other.jurisdiction && self.jurisdiction.is_some() {
            return false;
        }

        // For preconditions, we do a simplified check:
        // If this statute has fewer or equally general preconditions, it subsumes the other
        // This is a simplified implementation - full subsumption would require
        // logical analysis of condition relationships

        // If this has no preconditions, it subsumes any statute with the same effect
        if self.preconditions.is_empty() {
            return true;
        }

        // If other has no preconditions but this does, this doesn't subsume
        if other.preconditions.is_empty() {
            return false;
        }

        // Simplified heuristic: if precondition count is less, more general
        // For a proper implementation, use logical subsumption checking
        self.preconditions.len() <= other.preconditions.len()
    }

    /// Checks if this statute is subsumed by another statute.
    ///
    /// This is the inverse of [`Self::subsumes`].
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
    ///
    /// let general = Statute::new("general", "General Rule", Effect::grant("Benefit"))
    ///     .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 18));
    ///
    /// let specific = Statute::new("specific", "Specific Rule", Effect::grant("Benefit"))
    ///     .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 21));
    ///
    /// assert_eq!(specific.is_subsumed_by(&general), true);
    /// ```
    #[must_use]
    pub fn is_subsumed_by(&self, other: &Self) -> bool {
        other.subsumes(self)
    }

    /// Computes the differences between this statute and another version.
    ///
    /// This is useful for tracking amendments, understanding changes over time,
    /// and generating change logs for legal documents.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
    ///
    /// let version1 = Statute::new("tax-1", "Tax Law", Effect::grant("Tax Credit"))
    ///     .with_precondition(Condition::income(ComparisonOp::LessThan, 50000))
    ///     .with_version(1);
    ///
    /// let version2 = Statute::new("tax-1", "Tax Law (Amended)", Effect::grant("Tax Credit"))
    ///     .with_precondition(Condition::income(ComparisonOp::LessThan, 60000))
    ///     .with_version(2);
    ///
    /// let diff = version1.diff(&version2);
    /// assert!(!diff.changes.is_empty());
    /// ```
    #[must_use]
    pub fn diff(&self, other: &Self) -> StatuteDiff {
        let mut changes = Vec::new();

        // Check ID change
        if self.id != other.id {
            changes.push(StatuteChange::IdChanged {
                old: self.id.clone(),
                new: other.id.clone(),
            });
        }

        // Check title change
        if self.title != other.title {
            changes.push(StatuteChange::TitleChanged {
                old: self.title.clone(),
                new: other.title.clone(),
            });
        }

        // Check effect change
        if self.effect != other.effect {
            changes.push(StatuteChange::EffectChanged {
                old: format!("{}", self.effect),
                new: format!("{}", other.effect),
            });
        }

        // Check preconditions change
        if self.preconditions != other.preconditions {
            changes.push(StatuteChange::PreconditionsChanged {
                added: other
                    .preconditions
                    .len()
                    .saturating_sub(self.preconditions.len()),
                removed: self
                    .preconditions
                    .len()
                    .saturating_sub(other.preconditions.len()),
            });
        }

        // Check temporal validity change
        if self.temporal_validity != other.temporal_validity {
            changes.push(StatuteChange::TemporalValidityChanged);
        }

        // Check version change
        if self.version != other.version {
            changes.push(StatuteChange::VersionChanged {
                old: self.version,
                new: other.version,
            });
        }

        // Check jurisdiction change
        if self.jurisdiction != other.jurisdiction {
            changes.push(StatuteChange::JurisdictionChanged {
                old: self.jurisdiction.clone(),
                new: other.jurisdiction.clone(),
            });
        }

        StatuteDiff {
            statute_id: self.id.clone(),
            changes,
        }
    }
}

/// Represents differences between two versions of a statute.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct StatuteDiff {
    /// ID of the statute being compared
    pub statute_id: String,
    /// List of changes detected
    pub changes: Vec<StatuteChange>,
}

impl StatuteDiff {
    /// Returns true if there are no changes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }

    /// Returns the number of changes.
    #[must_use]
    pub fn change_count(&self) -> usize {
        self.changes.len()
    }
}

/// Types of changes that can occur in a statute.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum StatuteChange {
    /// The statute ID was changed
    IdChanged { old: String, new: String },
    /// The statute title was changed
    TitleChanged { old: String, new: String },
    /// The effect was changed
    EffectChanged { old: String, new: String },
    /// Preconditions were modified
    PreconditionsChanged { added: usize, removed: usize },
    /// Temporal validity was changed
    TemporalValidityChanged,
    /// Version number was changed
    VersionChanged { old: u32, new: u32 },
    /// Jurisdiction was changed
    JurisdictionChanged {
        old: Option<String>,
        new: Option<String>,
    },
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

impl fmt::Display for StatuteChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IdChanged { old, new } => write!(f, "ID: '{}' → '{}'", old, new),
            Self::TitleChanged { old, new } => write!(f, "Title: '{}' → '{}'", old, new),
            Self::EffectChanged { old, new } => write!(f, "Effect: {} → {}", old, new),
            Self::PreconditionsChanged { added, removed } => {
                write!(f, "Preconditions: +{} -{}", added, removed)
            }
            Self::TemporalValidityChanged => write!(f, "Temporal validity changed"),
            Self::VersionChanged { old, new } => write!(f, "Version: {} → {}", old, new),
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

/// Error severity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum ErrorSeverity {
    /// Warning - potential issue but not critical.
    Warning,
    /// Error - significant problem that should be addressed.
    Error,
    /// Critical - fundamental issue that prevents operation.
    Critical,
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

/// Validation errors for statutes with error codes and severity.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum ValidationError {
    /// Statute ID is empty.
    EmptyId,
    /// Statute ID contains invalid characters.
    InvalidId(String),
    /// Statute title is empty.
    EmptyTitle,
    /// Expiry date is before effective date.
    ExpiryBeforeEffective {
        effective: NaiveDate,
        expiry: NaiveDate,
    },
    /// A precondition is invalid.
    InvalidCondition { index: usize, message: String },
    /// Effect description is empty.
    EmptyEffectDescription,
    /// Version must be > 0.
    InvalidVersion,
}

impl ValidationError {
    /// Returns the error code for this validation error.
    #[must_use]
    pub const fn error_code(&self) -> &'static str {
        match self {
            Self::EmptyId => "E001",
            Self::InvalidId(_) => "E002",
            Self::EmptyTitle => "E003",
            Self::ExpiryBeforeEffective { .. } => "E004",
            Self::InvalidCondition { .. } => "E005",
            Self::EmptyEffectDescription => "E006",
            Self::InvalidVersion => "E007",
        }
    }

    /// Returns the severity level of this error.
    #[must_use]
    pub const fn severity(&self) -> ErrorSeverity {
        match self {
            Self::EmptyId | Self::EmptyTitle | Self::EmptyEffectDescription => {
                ErrorSeverity::Critical
            }
            Self::InvalidId(_) | Self::InvalidVersion => ErrorSeverity::Error,
            Self::ExpiryBeforeEffective { .. } | Self::InvalidCondition { .. } => {
                ErrorSeverity::Warning
            }
        }
    }

    /// Returns a suggestion for how to fix this error.
    #[must_use]
    pub fn suggestion(&self) -> Option<&str> {
        match self {
            Self::EmptyId => Some("Provide a non-empty ID for the statute"),
            Self::InvalidId(_) => {
                Some("Use only alphanumeric characters, hyphens, and underscores in IDs")
            }
            Self::EmptyTitle => Some("Provide a descriptive title for the statute"),
            Self::ExpiryBeforeEffective { .. } => {
                Some("Ensure the expiry date is after the effective date")
            }
            Self::InvalidCondition { .. } => {
                Some("Review and fix the condition, or remove it if not needed")
            }
            Self::EmptyEffectDescription => Some("Provide a description for the effect"),
            Self::InvalidVersion => Some("Version must be greater than 0"),
        }
    }

    /// Returns multiple recovery options for this error.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::ValidationError;
    ///
    /// let err = ValidationError::EmptyId;
    /// let options = err.recovery_options();
    /// assert!(!options.is_empty());
    /// ```
    #[must_use]
    pub fn recovery_options(&self) -> Vec<String> {
        match self {
            Self::EmptyId => vec![
                "Generate a unique ID based on title".to_string(),
                "Use a UUID as the ID".to_string(),
                "Derive ID from jurisdiction and statute number".to_string(),
            ],
            Self::InvalidId(id) => vec![
                format!("Remove invalid characters from '{}'", id),
                "Replace spaces with hyphens or underscores".to_string(),
                "Start ID with a letter if it begins with a number".to_string(),
            ],
            Self::EmptyTitle => vec![
                "Add a descriptive title summarizing the statute".to_string(),
                "Use the statute ID as a temporary title".to_string(),
            ],
            Self::ExpiryBeforeEffective { effective, expiry } => vec![
                format!("Change expiry date to be after {}", effective),
                format!("Change effective date to be before {}", expiry),
                "Remove the expiry date if statute doesn't expire".to_string(),
            ],
            Self::InvalidCondition { index, message } => vec![
                format!("Fix condition at index {}: {}", index, message),
                format!("Remove condition at index {}", index),
                "Simplify the condition to avoid validation issues".to_string(),
            ],
            Self::EmptyEffectDescription => vec![
                "Add a description explaining what the effect does".to_string(),
                "Use the effect type as a default description".to_string(),
            ],
            Self::InvalidVersion => vec![
                "Set version to 1 for new statutes".to_string(),
                "Increment version number from previous version".to_string(),
            ],
        }
    }

    /// Attempts to automatically fix this error if possible.
    ///
    /// Returns a description of the fix applied, or None if auto-fix is not available.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::ValidationError;
    ///
    /// let err = ValidationError::InvalidId("my statute!".to_string());
    /// let fixed = err.try_auto_fix();
    /// assert!(fixed.is_some());
    /// ```
    #[must_use]
    pub fn try_auto_fix(&self) -> Option<(String, String)> {
        match self {
            Self::InvalidId(id) => {
                let fixed = id
                    .chars()
                    .map(|c| {
                        if c.is_alphanumeric() || c == '-' || c == '_' {
                            c
                        } else if c.is_whitespace() {
                            '-'
                        } else {
                            '_'
                        }
                    })
                    .collect::<String>();
                Some((
                    fixed,
                    "Replaced invalid characters with hyphens/underscores".to_string(),
                ))
            }
            Self::InvalidVersion => Some((
                "1".to_string(),
                "Set version to 1 (default for new statutes)".to_string(),
            )),
            _ => None,
        }
    }
}

/// Condition evaluation errors.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum ConditionError {
    /// Missing attribute in entity.
    MissingAttribute { key: String },
    /// Type mismatch when evaluating condition.
    TypeMismatch { expected: String, actual: String },
    /// Invalid calculation formula.
    InvalidFormula { formula: String, error: String },
    /// Pattern matching error.
    PatternError { pattern: String, error: String },
    /// Evaluation exceeded maximum depth (possible infinite recursion).
    MaxDepthExceeded { max_depth: usize },
    /// Custom evaluation error.
    Custom { message: String },
}

impl ConditionError {
    /// Returns the error code for this condition error.
    #[must_use]
    pub const fn error_code(&self) -> &'static str {
        match self {
            Self::MissingAttribute { .. } => "C001",
            Self::TypeMismatch { .. } => "C002",
            Self::InvalidFormula { .. } => "C003",
            Self::PatternError { .. } => "C004",
            Self::MaxDepthExceeded { .. } => "C005",
            Self::Custom { .. } => "C999",
        }
    }

    /// Returns the severity level of this error.
    #[must_use]
    pub const fn severity(&self) -> ErrorSeverity {
        match self {
            Self::MissingAttribute { .. } | Self::TypeMismatch { .. } => ErrorSeverity::Error,
            Self::InvalidFormula { .. } | Self::PatternError { .. } => ErrorSeverity::Critical,
            Self::MaxDepthExceeded { .. } => ErrorSeverity::Critical,
            Self::Custom { .. } => ErrorSeverity::Error,
        }
    }

    /// Returns a suggestion for how to fix this error.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::ConditionError;
    ///
    /// let err = ConditionError::MissingAttribute {
    ///     key: "age".to_string(),
    /// };
    /// assert!(err.suggestion().is_some());
    /// ```
    #[must_use]
    pub fn suggestion(&self) -> Option<String> {
        match self {
            Self::MissingAttribute { key } => Some(format!(
                "Add the '{}' attribute to the entity before evaluation",
                key
            )),
            Self::TypeMismatch { expected, actual } => Some(format!(
                "Convert the value from {} to {} or adjust the condition type",
                actual, expected
            )),
            Self::InvalidFormula { formula, error } => Some(format!(
                "Fix the formula '{}': {}. Check syntax and ensure all variables are defined.",
                formula, error
            )),
            Self::PatternError { pattern, error } => Some(format!(
                "Fix the regex pattern '{}': {}. Ensure the pattern is valid regex syntax.",
                pattern, error
            )),
            Self::MaxDepthExceeded { max_depth } => Some(format!(
                "Simplify the condition structure to reduce nesting below {} levels, or check for circular references",
                max_depth
            )),
            Self::Custom { .. } => None,
        }
    }

    /// Returns multiple recovery options for this error.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::ConditionError;
    ///
    /// let err = ConditionError::TypeMismatch {
    ///     expected: "u32".to_string(),
    ///     actual: "String".to_string(),
    /// };
    /// let options = err.recovery_options();
    /// assert!(!options.is_empty());
    /// ```
    #[must_use]
    pub fn recovery_options(&self) -> Vec<String> {
        match self {
            Self::MissingAttribute { key } => vec![
                format!("Add '{}' to entity attributes", key),
                "Use default value for missing attribute".to_string(),
                "Make this condition optional".to_string(),
            ],
            Self::TypeMismatch { expected, actual } => vec![
                format!("Convert {} to {}", actual, expected),
                "Change condition to accept current type".to_string(),
                "Add type conversion in evaluation context".to_string(),
            ],
            Self::InvalidFormula { .. } => vec![
                "Fix formula syntax".to_string(),
                "Use simpler condition type instead of calculation".to_string(),
                "Define missing variables in context".to_string(),
            ],
            Self::PatternError { .. } => vec![
                "Fix regex syntax".to_string(),
                "Escape special regex characters".to_string(),
                "Use simpler string comparison instead".to_string(),
            ],
            Self::MaxDepthExceeded { .. } => vec![
                "Flatten nested conditions using normalization".to_string(),
                "Break complex condition into multiple simpler ones".to_string(),
                "Check for and remove circular condition references".to_string(),
            ],
            Self::Custom { .. } => vec![],
        }
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

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyId => write!(f, "Statute ID cannot be empty"),
            Self::InvalidId(id) => write!(
                f,
                "Invalid statute ID: '{}' (must start with letter, contain only alphanumeric/dash/underscore)",
                id
            ),
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
            Self::EmptyEffectDescription => write!(f, "Effect description cannot be empty"),
            Self::InvalidVersion => write!(f, "Version must be greater than 0"),
        }
    }
}

impl std::error::Error for ValidationError {}

// ==================================================
// Enhanced Diagnostic Context for Validation Errors
// ==================================================

/// Source location information for error diagnostics.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct SourceLocation {
    /// File path or source identifier
    pub file: Option<String>,
    /// Line number (1-indexed)
    pub line: Option<usize>,
    /// Column number (1-indexed)
    pub column: Option<usize>,
    /// Source snippet for context
    pub snippet: Option<String>,
}

impl SourceLocation {
    /// Creates a new source location.
    #[must_use]
    pub fn new() -> Self {
        Self {
            file: None,
            line: None,
            column: None,
            snippet: None,
        }
    }

    /// Sets the file path.
    #[must_use]
    pub fn with_file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }

    /// Sets the line number.
    #[must_use]
    pub const fn with_line(mut self, line: usize) -> Self {
        self.line = Some(line);
        self
    }

    /// Sets the column number.
    #[must_use]
    pub const fn with_column(mut self, column: usize) -> Self {
        self.column = Some(column);
        self
    }

    /// Sets the source snippet.
    #[must_use]
    pub fn with_snippet(mut self, snippet: impl Into<String>) -> Self {
        self.snippet = Some(snippet.into());
        self
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

/// Diagnostic context for detailed error reporting.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct DiagnosticContext {
    /// Source location where the error occurred
    pub location: Option<SourceLocation>,
    /// Related statute ID
    pub statute_id: Option<String>,
    /// Related condition description
    pub condition: Option<String>,
    /// Stack trace or call chain
    pub stack: Vec<String>,
    /// Additional contextual notes
    pub notes: Vec<String>,
    /// Suggested fixes
    pub suggestions: Vec<String>,
}

impl DiagnosticContext {
    /// Creates a new empty diagnostic context.
    #[must_use]
    pub fn new() -> Self {
        Self {
            location: None,
            statute_id: None,
            condition: None,
            stack: Vec::new(),
            notes: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    /// Sets the source location.
    #[must_use]
    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }

    /// Sets the statute ID.
    #[must_use]
    pub fn with_statute_id(mut self, id: impl Into<String>) -> Self {
        self.statute_id = Some(id.into());
        self
    }

    /// Sets the condition description.
    #[must_use]
    pub fn with_condition(mut self, condition: impl Into<String>) -> Self {
        self.condition = Some(condition.into());
        self
    }

    /// Adds a stack frame.
    pub fn add_stack_frame(&mut self, frame: impl Into<String>) {
        self.stack.push(frame.into());
    }

    /// Adds a note.
    pub fn add_note(&mut self, note: impl Into<String>) {
        self.notes.push(note.into());
    }

    /// Adds a suggestion.
    pub fn add_suggestion(&mut self, suggestion: impl Into<String>) {
        self.suggestions.push(suggestion.into());
    }

    /// Builder method to add a stack frame.
    #[must_use]
    pub fn with_stack_frame(mut self, frame: impl Into<String>) -> Self {
        self.stack.push(frame.into());
        self
    }

    /// Builder method to add a note.
    #[must_use]
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    /// Builder method to add a suggestion.
    #[must_use]
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestions.push(suggestion.into());
        self
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

/// Enhanced validation error with diagnostic context.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DiagnosticValidationError {
    /// The base validation error
    pub error: ValidationError,
    /// Diagnostic context
    pub context: DiagnosticContext,
}

impl DiagnosticValidationError {
    /// Creates a new diagnostic validation error.
    #[must_use]
    pub fn new(error: ValidationError) -> Self {
        Self {
            error,
            context: DiagnosticContext::new(),
        }
    }

    /// Adds diagnostic context.
    #[must_use]
    pub fn with_context(mut self, context: DiagnosticContext) -> Self {
        self.context = context;
        self
    }

    /// Gets the error code.
    #[must_use]
    pub fn error_code(&self) -> &str {
        self.error.error_code()
    }

    /// Gets the error severity.
    #[must_use]
    pub fn severity(&self) -> ErrorSeverity {
        self.error.severity()
    }

    /// Gets a suggestion for fixing the error.
    #[must_use]
    pub fn suggestion(&self) -> Option<&str> {
        self.error.suggestion()
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

/// Diagnostic error reporter for collecting and formatting errors.
///
/// # Examples
///
/// ```
/// use legalis_core::{DiagnosticReporter, ValidationError, SourceLocation, DiagnosticContext};
///
/// let mut reporter = DiagnosticReporter::new();
///
/// reporter.add_error(
///     ValidationError::EmptyTitle,
///     DiagnosticContext::new()
///         .with_statute_id("law-123")
///         .with_location(SourceLocation::new().with_file("statutes.json").with_line(45))
///         .with_suggestion("Add a 'title' field to the statute definition")
/// );
///
/// reporter.add_error(
///     ValidationError::InvalidVersion,
///     DiagnosticContext::new()
///         .with_statute_id("law-456")
///         .with_note("Version must be greater than 0")
/// );
///
/// // Print all errors with diagnostic context
/// println!("{}", reporter.report());
/// assert_eq!(reporter.error_count(), 2);
/// ```
#[derive(Debug, Default)]
pub struct DiagnosticReporter {
    errors: Vec<DiagnosticValidationError>,
}

impl DiagnosticReporter {
    /// Creates a new diagnostic reporter.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an error with diagnostic context.
    pub fn add_error(&mut self, error: ValidationError, context: DiagnosticContext) {
        self.errors
            .push(DiagnosticValidationError { error, context });
    }

    /// Adds an error without context.
    pub fn add_simple_error(&mut self, error: ValidationError) {
        self.errors.push(DiagnosticValidationError::new(error));
    }

    /// Returns the number of errors.
    #[must_use]
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    /// Returns `true` if there are no errors.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Returns `true` if there are errors.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Gets all errors.
    #[must_use]
    pub fn errors(&self) -> &[DiagnosticValidationError] {
        &self.errors
    }

    /// Filters errors by severity.
    #[must_use]
    pub fn errors_with_severity(&self, severity: ErrorSeverity) -> Vec<&DiagnosticValidationError> {
        self.errors
            .iter()
            .filter(|e| e.severity() == severity)
            .collect()
    }

    /// Returns only critical errors.
    #[must_use]
    pub fn critical_errors(&self) -> Vec<&DiagnosticValidationError> {
        self.errors_with_severity(ErrorSeverity::Critical)
    }

    /// Clears all errors.
    pub fn clear(&mut self) {
        self.errors.clear();
    }

    /// Generates a formatted error report.
    #[must_use]
    pub fn report(&self) -> String {
        if self.errors.is_empty() {
            return "No errors".to_string();
        }

        let mut output = String::new();
        output.push_str(&format!("\n{} error(s) found:\n\n", self.errors.len()));

        for (i, error) in self.errors.iter().enumerate() {
            output.push_str(&format!("{}. {}\n", i + 1, error));
        }

        output
    }

    /// Generates a summary of errors by type.
    #[must_use]
    pub fn summary(&self) -> String {
        let critical = self.critical_errors().len();
        let errors = self.errors_with_severity(ErrorSeverity::Error).len();
        let warnings = self.errors_with_severity(ErrorSeverity::Warning).len();

        format!(
            "{} total ({} critical, {} errors, {} warnings)",
            self.error_count(),
            critical,
            errors,
            warnings
        )
    }
}

impl fmt::Display for DiagnosticReporter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.report())
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

/// Represents the outcome of a conflict resolution between two statutes.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum ConflictResolution {
    /// First statute prevails
    FirstPrevails(ConflictReason),
    /// Second statute prevails
    SecondPrevails(ConflictReason),
    /// No conflict - statutes are compatible
    NoConflict,
    /// Statutes conflict but cannot be automatically resolved
    Unresolvable(String),
}

/// Reason why one statute prevails over another.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum ConflictReason {
    /// Later law prevails (lex posterior derogat legi priori)
    TemporalPrecedence,
    /// More specific law prevails (lex specialis derogat legi generali)
    Specificity,
    /// Higher authority prevails (lex superior derogat legi inferiori)
    Hierarchy,
    /// Explicit amendment/repeal relationship
    ExplicitAmendment,
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
            Self::FirstPrevails(reason) => write!(f, "First statute prevails: {}", reason),
            Self::SecondPrevails(reason) => write!(f, "Second statute prevails: {}", reason),
            Self::NoConflict => write!(f, "No conflict - statutes are compatible"),
            Self::Unresolvable(msg) => write!(f, "Unresolvable conflict: {}", msg),
        }
    }
}

/// Statute conflict analyzer.
///
/// Provides methods to detect and resolve conflicts between statutes
/// using established legal principles.
pub struct StatuteConflictAnalyzer;

impl StatuteConflictAnalyzer {
    /// Analyzes two statutes for conflicts and determines which should prevail.
    ///
    /// Uses the following hierarchy of resolution principles:
    /// 1. Explicit amendment relationships
    /// 2. Temporal precedence (newer laws)
    /// 3. Specificity (more specific laws)
    /// 4. Hierarchy (jurisdictional authority)
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType, StatuteConflictAnalyzer, TemporalValidity};
    /// use chrono::NaiveDate;
    ///
    /// let old_law = Statute::new("old-1", "Old Law", Effect::new(EffectType::Grant, "Old grant"))
    ///     .with_temporal_validity(
    ///         TemporalValidity::new()
    ///             .with_effective_date(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
    ///     )
    ///     .with_version(1);
    ///
    /// let new_law = Statute::new("new-1", "New Law", Effect::new(EffectType::Grant, "New grant"))
    ///     .with_temporal_validity(
    ///         TemporalValidity::new()
    ///             .with_effective_date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
    ///     )
    ///     .with_version(1);
    ///
    /// let resolution = StatuteConflictAnalyzer::resolve(&old_law, &new_law);
    /// // New law prevails by temporal precedence
    /// ```
    pub fn resolve(first: &Statute, second: &Statute) -> ConflictResolution {
        // Check if statutes actually conflict
        if !Self::has_conflict(first, second) {
            return ConflictResolution::NoConflict;
        }

        // 1. Check for explicit amendments (would require hierarchy module integration)
        // For now, we'll implement temporal and specificity checks

        // 2. Check temporal precedence
        if let Some(resolution) = Self::check_temporal_precedence(first, second) {
            return resolution;
        }

        // 3. Check specificity
        if let Some(resolution) = Self::check_specificity(first, second) {
            return resolution;
        }

        // 4. Check hierarchy (jurisdiction-based)
        if let Some(resolution) = Self::check_hierarchy(first, second) {
            return resolution;
        }

        // Cannot automatically resolve
        ConflictResolution::Unresolvable(
            "Statutes conflict but resolution requires human judgment".to_string(),
        )
    }

    /// Checks if two statutes have conflicting effects.
    fn has_conflict(first: &Statute, second: &Statute) -> bool {
        // Statutes conflict if they have incompatible effects
        // For simplicity, we consider Grant vs Prohibition/Revoke as conflicts
        use EffectType::*;

        matches!(
            (&first.effect.effect_type, &second.effect.effect_type),
            (Grant, Prohibition)
                | (Grant, Revoke)
                | (Prohibition, Grant)
                | (Revoke, Grant)
                | (Obligation, Prohibition)
                | (Prohibition, Obligation)
        )
    }

    /// Applies lex posterior (later law prevails).
    fn check_temporal_precedence(first: &Statute, second: &Statute) -> Option<ConflictResolution> {
        let first_date = first.temporal_validity.effective_date?;
        let second_date = second.temporal_validity.effective_date?;

        if first_date > second_date {
            Some(ConflictResolution::FirstPrevails(
                ConflictReason::TemporalPrecedence,
            ))
        } else if second_date > first_date {
            Some(ConflictResolution::SecondPrevails(
                ConflictReason::TemporalPrecedence,
            ))
        } else {
            None // Same date, cannot resolve by temporal precedence
        }
    }

    /// Applies lex specialis (more specific law prevails).
    ///
    /// A statute is considered more specific if it has more preconditions.
    fn check_specificity(first: &Statute, second: &Statute) -> Option<ConflictResolution> {
        let first_specificity = Self::calculate_specificity(first);
        let second_specificity = Self::calculate_specificity(second);

        if first_specificity > second_specificity {
            Some(ConflictResolution::FirstPrevails(
                ConflictReason::Specificity,
            ))
        } else if second_specificity > first_specificity {
            Some(ConflictResolution::SecondPrevails(
                ConflictReason::Specificity,
            ))
        } else {
            None // Same specificity
        }
    }

    /// Calculates specificity score based on number and complexity of conditions.
    fn calculate_specificity(statute: &Statute) -> usize {
        statute
            .preconditions
            .iter()
            .map(|c| c.count_conditions())
            .sum()
    }

    /// Applies lex superior (higher authority prevails).
    ///
    /// Uses jurisdiction hierarchy: federal > state > local
    fn check_hierarchy(first: &Statute, second: &Statute) -> Option<ConflictResolution> {
        let first_level = Self::jurisdiction_level(&first.jurisdiction);
        let second_level = Self::jurisdiction_level(&second.jurisdiction);

        if first_level > second_level {
            Some(ConflictResolution::FirstPrevails(ConflictReason::Hierarchy))
        } else if second_level > first_level {
            Some(ConflictResolution::SecondPrevails(
                ConflictReason::Hierarchy,
            ))
        } else {
            None // Same level
        }
    }

    /// Determines jurisdiction hierarchy level.
    ///
    /// Higher number = higher authority
    fn jurisdiction_level(jurisdiction: &Option<String>) -> u32 {
        jurisdiction.as_ref().map_or(0, |j| {
            if j.to_lowercase().contains("federal") || j.to_lowercase().contains("national") {
                3
            } else if j.to_lowercase().contains("state") || j.to_lowercase().contains("provincial")
            {
                2
            } else if j.to_lowercase().contains("local") || j.to_lowercase().contains("municipal") {
                1
            } else {
                // Try to infer from format (e.g., "US" = federal, "US-NY" = state)
                if j.len() <= 3 && j.chars().all(|c| c.is_ascii_uppercase()) {
                    3 // Likely national/federal
                } else if j.contains('-') {
                    2 // Likely state/provincial
                } else {
                    0 // Unknown
                }
            }
        })
    }

    /// Checks if a statute is still in effect on a given date.
    pub fn is_in_effect(statute: &Statute, date: NaiveDate) -> bool {
        statute.temporal_validity.is_active(date)
    }

    /// Finds which statutes from a set apply to a given date and resolves conflicts.
    ///
    /// Returns statutes in order of precedence (highest priority first).
    pub fn resolve_conflicts_at_date(statutes: &[Statute], date: NaiveDate) -> Vec<&Statute> {
        // Filter to only in-effect statutes
        let mut active: Vec<&Statute> = statutes
            .iter()
            .filter(|s| Self::is_in_effect(s, date))
            .collect();

        // Sort by precedence (most specific and recent first)
        active.sort_by(|a, b| {
            // First by effective date (newer first)
            let date_cmp = b
                .temporal_validity
                .effective_date
                .cmp(&a.temporal_validity.effective_date);
            if date_cmp != std::cmp::Ordering::Equal {
                return date_cmp;
            }

            // Then by specificity (more specific first)
            let spec_cmp = Self::calculate_specificity(b).cmp(&Self::calculate_specificity(a));
            if spec_cmp != std::cmp::Ordering::Equal {
                return spec_cmp;
            }

            // Then by hierarchy
            Self::jurisdiction_level(&b.jurisdiction)
                .cmp(&Self::jurisdiction_level(&a.jurisdiction))
        });

        active
    }

    /// Detects contradictions across a set of statutes.
    ///
    /// A contradiction occurs when:
    /// - Two statutes have conflicting effects (Grant vs Revoke) for the same thing
    /// - Two statutes have mutually exclusive preconditions but same effects
    /// - Statutes create logical inconsistencies in the legal system
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType, StatuteConflictAnalyzer, Condition, ComparisonOp};
    ///
    /// let grant = Statute::new("grant-1", "Grant Right", Effect::new(EffectType::Grant, "Voting"))
    ///     .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 18));
    ///
    /// let revoke = Statute::new("revoke-1", "Revoke Right", Effect::new(EffectType::Revoke, "Voting"))
    ///     .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 18));
    ///
    /// let statutes = vec![grant, revoke];
    /// let contradictions = StatuteConflictAnalyzer::detect_contradictions(&statutes);
    ///
    /// assert!(!contradictions.is_empty());
    /// ```
    #[must_use]
    pub fn detect_contradictions(statutes: &[Statute]) -> Vec<Contradiction> {
        let mut contradictions = Vec::new();

        // Check all pairs of statutes
        for (i, statute_a) in statutes.iter().enumerate() {
            for statute_b in statutes.iter().skip(i + 1) {
                // Check for effect contradictions
                if Self::effects_contradict(&statute_a.effect, &statute_b.effect) {
                    // Check if conditions overlap (could both apply)
                    if Self::conditions_may_overlap(
                        &statute_a.preconditions,
                        &statute_b.preconditions,
                    ) {
                        contradictions.push(Contradiction {
                            statute_a_id: statute_a.id.clone(),
                            statute_b_id: statute_b.id.clone(),
                            contradiction_type: ContradictionType::ConflictingEffects,
                            description: format!(
                                "Statute '{}' grants while '{}' revokes the same right",
                                statute_a.id, statute_b.id
                            ),
                            severity: ErrorSeverity::Critical,
                        });
                    }
                }

                // Check for identical preconditions with conflicting effects
                if statute_a.preconditions == statute_b.preconditions
                    && statute_a.effect.effect_type != statute_b.effect.effect_type
                {
                    contradictions.push(Contradiction {
                        statute_a_id: statute_a.id.clone(),
                        statute_b_id: statute_b.id.clone(),
                        contradiction_type: ContradictionType::IdenticalConditionsConflictingEffects,
                        description: format!(
                            "Statutes '{}' and '{}' have identical conditions but conflicting effects",
                            statute_a.id, statute_b.id
                        ),
                        severity: ErrorSeverity::Critical,
                    });
                }
            }
        }

        contradictions
    }

    /// Checks if two effects contradict each other.
    fn effects_contradict(effect_a: &Effect, effect_b: &Effect) -> bool {
        // Grant vs Revoke is a contradiction
        matches!(
            (&effect_a.effect_type, &effect_b.effect_type),
            (EffectType::Grant, EffectType::Revoke) | (EffectType::Revoke, EffectType::Grant)
        ) && effect_a.description == effect_b.description
    }

    /// Checks if two sets of conditions may overlap (both could be true).
    /// This is a simplified heuristic - full overlap detection requires SAT solving.
    #[allow(dead_code)]
    fn conditions_may_overlap(conds_a: &[Condition], conds_b: &[Condition]) -> bool {
        // Simplified: if either is empty or they're identical, they overlap
        conds_a.is_empty() || conds_b.is_empty() || conds_a == conds_b
    }
}

/// Represents a logical contradiction between statutes.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Contradiction {
    /// ID of the first statute involved
    pub statute_a_id: String,
    /// ID of the second statute involved
    pub statute_b_id: String,
    /// Type of contradiction
    pub contradiction_type: ContradictionType,
    /// Human-readable description
    pub description: String,
    /// Severity of the contradiction
    pub severity: ErrorSeverity,
}

/// Types of contradictions that can occur between statutes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum ContradictionType {
    /// Statutes have conflicting effects (grant vs revoke)
    ConflictingEffects,
    /// Identical conditions but conflicting effects
    IdenticalConditionsConflictingEffects,
    /// Circular dependency between statutes
    CircularDependency,
    /// Logical inconsistency in rule set
    LogicalInconsistency,
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

// ============================================================================
// Fluent Builders for Enhanced Developer Experience
// ============================================================================

/// Builder for constructing `Condition` objects with a fluent API.
///
/// Provides a convenient way to construct complex conditions with chaining.
///
/// # Examples
///
/// ```
/// use legalis_core::{ConditionBuilder, ComparisonOp};
///
/// let condition = ConditionBuilder::new()
///     .age(ComparisonOp::GreaterOrEqual, 18)
///     .and()
///     .income(ComparisonOp::LessThan, 50000)
///     .build();
///
/// assert!(!condition.to_string().is_empty());
/// ```
#[derive(Debug, Clone)]
pub struct ConditionBuilder {
    conditions: Vec<Condition>,
    operation: ConditionOperation,
}

#[derive(Debug, Clone)]
enum ConditionOperation {
    None,
    And,
    Or,
}

impl ConditionBuilder {
    /// Creates a new condition builder.
    #[must_use]
    pub fn new() -> Self {
        Self {
            conditions: Vec::new(),
            operation: ConditionOperation::None,
        }
    }

    /// Adds an age condition.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{ConditionBuilder, ComparisonOp};
    ///
    /// let cond = ConditionBuilder::new()
    ///     .age(ComparisonOp::GreaterOrEqual, 21)
    ///     .build();
    /// ```
    #[must_use]
    pub fn age(mut self, operator: ComparisonOp, value: u32) -> Self {
        self.conditions.push(Condition::Age { operator, value });
        self
    }

    /// Adds an income condition.
    #[must_use]
    pub fn income(mut self, operator: ComparisonOp, value: u64) -> Self {
        self.conditions.push(Condition::Income { operator, value });
        self
    }

    /// Adds a has-attribute condition.
    #[must_use]
    pub fn has_attribute(mut self, attr: impl Into<String>) -> Self {
        self.conditions
            .push(Condition::HasAttribute { key: attr.into() });
        self
    }

    /// Adds an attribute-equals condition.
    #[must_use]
    pub fn attribute_equals(mut self, attr: impl Into<String>, value: impl Into<String>) -> Self {
        self.conditions.push(Condition::AttributeEquals {
            key: attr.into(),
            value: value.into(),
        });
        self
    }

    /// Adds a custom condition.
    #[must_use]
    pub fn custom(mut self, description: impl Into<String>) -> Self {
        self.conditions.push(Condition::Custom {
            description: description.into(),
        });
        self
    }

    /// Combines the next condition with AND logic.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{ConditionBuilder, ComparisonOp};
    ///
    /// let cond = ConditionBuilder::new()
    ///     .age(ComparisonOp::GreaterOrEqual, 18)
    ///     .and()
    ///     .income(ComparisonOp::LessThan, 50000)
    ///     .build();
    /// ```
    #[must_use]
    pub fn and(mut self) -> Self {
        self.operation = ConditionOperation::And;
        self
    }

    /// Combines the next condition with OR logic.
    #[must_use]
    pub fn or(mut self) -> Self {
        self.operation = ConditionOperation::Or;
        self
    }

    /// Builds the final condition.
    ///
    /// If multiple conditions were added, they are combined according to the
    /// specified operations (AND/OR).
    #[must_use]
    pub fn build(self) -> Condition {
        if self.conditions.is_empty() {
            Condition::Custom {
                description: "true".to_string(),
            }
        } else if self.conditions.len() == 1 {
            self.conditions.into_iter().next().unwrap()
        } else {
            // Combine all conditions with the operation
            let mut result = self.conditions[0].clone();
            for cond in self.conditions.into_iter().skip(1) {
                result = match self.operation {
                    ConditionOperation::And => Condition::And(Box::new(result), Box::new(cond)),
                    ConditionOperation::Or => Condition::Or(Box::new(result), Box::new(cond)),
                    ConditionOperation::None => Condition::And(Box::new(result), Box::new(cond)),
                };
            }
            result
        }
    }
}

impl Default for ConditionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for constructing `Effect` objects with a fluent API.
///
/// Provides a convenient way to construct effects with parameters.
///
/// # Examples
///
/// ```
/// use legalis_core::{EffectBuilder, EffectType};
///
/// let effect = EffectBuilder::new()
///     .effect_type(EffectType::Grant)
///     .description("Tax credit")
///     .parameter("amount", "1000")
///     .parameter("currency", "USD")
///     .build();
///
/// assert_eq!(effect.effect_type, EffectType::Grant);
/// assert_eq!(effect.parameters.get("amount"), Some(&"1000".to_string()));
/// ```
#[derive(Debug, Clone)]
pub struct EffectBuilder {
    effect_type: Option<EffectType>,
    description: Option<String>,
    parameters: std::collections::HashMap<String, String>,
}

impl EffectBuilder {
    /// Creates a new effect builder.
    #[must_use]
    pub fn new() -> Self {
        Self {
            effect_type: None,
            description: None,
            parameters: std::collections::HashMap::new(),
        }
    }

    /// Creates a builder initialized with an effect type and description.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{EffectBuilder, EffectType};
    ///
    /// let effect = EffectBuilder::grant("Tax credit")
    ///     .parameter("amount", "1000")
    ///     .build();
    ///
    /// assert_eq!(effect.effect_type, EffectType::Grant);
    /// ```
    #[must_use]
    pub fn grant(description: impl Into<String>) -> Self {
        Self {
            effect_type: Some(EffectType::Grant),
            description: Some(description.into()),
            parameters: std::collections::HashMap::new(),
        }
    }

    /// Creates a builder for a revoke effect.
    #[must_use]
    pub fn revoke(description: impl Into<String>) -> Self {
        Self {
            effect_type: Some(EffectType::Revoke),
            description: Some(description.into()),
            parameters: std::collections::HashMap::new(),
        }
    }

    /// Creates a builder for an obligation effect.
    #[must_use]
    pub fn obligation(description: impl Into<String>) -> Self {
        Self {
            effect_type: Some(EffectType::Obligation),
            description: Some(description.into()),
            parameters: std::collections::HashMap::new(),
        }
    }

    /// Sets the effect type.
    #[must_use]
    pub fn effect_type(mut self, effect_type: EffectType) -> Self {
        self.effect_type = Some(effect_type);
        self
    }

    /// Sets the description.
    #[must_use]
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Adds a parameter to the effect.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{EffectBuilder, EffectType};
    ///
    /// let effect = EffectBuilder::new()
    ///     .effect_type(EffectType::MonetaryTransfer)
    ///     .description("Tax payment")
    ///     .parameter("amount", "5000")
    ///     .parameter("currency", "USD")
    ///     .build();
    ///
    /// assert_eq!(effect.parameters.len(), 2);
    /// ```
    #[must_use]
    pub fn parameter(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.parameters.insert(key.into(), value.into());
        self
    }

    /// Builds the final effect.
    ///
    /// # Panics
    ///
    /// Panics if effect_type or description is not set.
    #[must_use]
    pub fn build(self) -> Effect {
        Effect {
            effect_type: self.effect_type.expect("Effect type must be set"),
            description: self.description.expect("Description must be set"),
            parameters: self.parameters,
        }
    }

    /// Builds the effect, returning an error if required fields are missing.
    pub fn try_build(self) -> Result<Effect, String> {
        let effect_type = self.effect_type.ok_or("Effect type not set")?;
        let description = self.description.ok_or("Description not set")?;
        Ok(Effect {
            effect_type,
            description,
            parameters: self.parameters,
        })
    }
}

impl Default for EffectBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple builder for constructing `Statute` objects with template support
/// and progressive validation.
///
/// Unlike `TypedStatuteBuilder`, this builder is runtime-validated and provides
/// convenience methods like `from_template()` and progressive validation.
///
/// # Examples
///
/// ```
/// use legalis_core::{StatuteBuilder, Effect, EffectType, Condition, ComparisonOp};
///
/// let statute = StatuteBuilder::new()
///     .id("tax-law-1")
///     .title("Tax Credit Law")
///     .effect(Effect::new(EffectType::Grant, "Tax credit"))
///     .precondition(Condition::age(ComparisonOp::GreaterOrEqual, 18))
///     .validate_progressive(true)
///     .build()
///     .expect("Failed to build statute");
///
/// assert_eq!(statute.id, "tax-law-1");
/// ```
#[derive(Debug, Clone)]
pub struct StatuteBuilder {
    id: Option<String>,
    title: Option<String>,
    effect: Option<Effect>,
    preconditions: Vec<Condition>,
    discretion_logic: Option<String>,
    temporal_validity: TemporalValidity,
    version: u32,
    jurisdiction: Option<String>,
    derives_from: Vec<String>,
    applies_to: Vec<String>,
    exceptions: Vec<StatuteException>,
    progressive_validation: bool,
    validation_errors: Vec<ValidationError>,
}

impl StatuteBuilder {
    /// Creates a new statute builder.
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: None,
            title: None,
            effect: None,
            preconditions: Vec::new(),
            discretion_logic: None,
            temporal_validity: TemporalValidity::default(),
            version: 1,
            jurisdiction: None,
            derives_from: Vec::new(),
            applies_to: Vec::new(),
            exceptions: Vec::new(),
            progressive_validation: false,
            validation_errors: Vec::new(),
        }
    }

    /// Creates a builder from an existing statute template.
    ///
    /// This copies all fields from the template statute, allowing you to modify
    /// specific fields while keeping others the same.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{StatuteBuilder, Statute, Effect, EffectType};
    ///
    /// let template = Statute::new("template-1", "Template Law", Effect::grant("Benefit"))
    ///     .with_version(1)
    ///     .with_jurisdiction("US");
    ///
    /// let derived = StatuteBuilder::from_template(&template)
    ///     .id("derived-1")
    ///     .title("Derived Law")
    ///     .build()
    ///     .expect("Failed to build");
    ///
    /// assert_eq!(derived.jurisdiction, Some("US".to_string()));
    /// assert_eq!(derived.version, 1);
    /// ```
    #[must_use]
    pub fn from_template(template: &Statute) -> Self {
        Self {
            id: Some(template.id.clone()),
            title: Some(template.title.clone()),
            effect: Some(template.effect.clone()),
            preconditions: template.preconditions.clone(),
            discretion_logic: template.discretion_logic.clone(),
            temporal_validity: template.temporal_validity.clone(),
            version: template.version,
            jurisdiction: template.jurisdiction.clone(),
            derives_from: template.derives_from.clone(),
            applies_to: template.applies_to.clone(),
            exceptions: template.exceptions.clone(),
            progressive_validation: false,
            validation_errors: Vec::new(),
        }
    }

    /// Enables or disables progressive validation.
    ///
    /// When enabled, the builder validates each field as it's set and accumulates
    /// validation errors.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{StatuteBuilder, Effect, EffectType};
    ///
    /// let result = StatuteBuilder::new()
    ///     .validate_progressive(true)
    ///     .id("") // Invalid ID - empty
    ///     .title("Test")
    ///     .effect(Effect::grant("Benefit"))
    ///     .build();
    ///
    /// assert!(result.is_err());
    /// ```
    #[must_use]
    pub fn validate_progressive(mut self, enabled: bool) -> Self {
        self.progressive_validation = enabled;
        self
    }

    /// Sets the statute ID.
    #[must_use]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        let id = id.into();
        if self.progressive_validation {
            if id.is_empty() {
                self.validation_errors.push(ValidationError::EmptyId);
            } else if !self.is_valid_id(&id) {
                self.validation_errors
                    .push(ValidationError::InvalidId(id.clone()));
            }
        }
        self.id = Some(id);
        self
    }

    /// Sets the statute title.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        let title = title.into();
        if self.progressive_validation && title.is_empty() {
            self.validation_errors.push(ValidationError::EmptyTitle);
        }
        self.title = Some(title);
        self
    }

    /// Sets the effect.
    #[must_use]
    pub fn effect(mut self, effect: Effect) -> Self {
        if self.progressive_validation && effect.description.is_empty() {
            self.validation_errors
                .push(ValidationError::EmptyEffectDescription);
        }
        self.effect = Some(effect);
        self
    }

    /// Adds a precondition.
    #[must_use]
    pub fn precondition(mut self, condition: Condition) -> Self {
        self.preconditions.push(condition);
        self
    }

    /// Sets the discretion logic.
    #[must_use]
    pub fn discretion(mut self, logic: impl Into<String>) -> Self {
        self.discretion_logic = Some(logic.into());
        self
    }

    /// Sets temporal validity.
    #[must_use]
    pub fn temporal_validity(mut self, validity: TemporalValidity) -> Self {
        self.temporal_validity = validity;
        self
    }

    /// Sets the version.
    #[must_use]
    pub fn version(mut self, version: u32) -> Self {
        if self.progressive_validation && version == 0 {
            self.validation_errors.push(ValidationError::InvalidVersion);
        }
        self.version = version;
        self
    }

    /// Sets the jurisdiction.
    #[must_use]
    pub fn jurisdiction(mut self, jurisdiction: impl Into<String>) -> Self {
        self.jurisdiction = Some(jurisdiction.into());
        self
    }

    /// Adds a derivation source.
    #[must_use]
    pub fn derives_from(mut self, source: impl Into<String>) -> Self {
        self.derives_from.push(source.into());
        self
    }

    /// Adds an applicable entity type.
    #[must_use]
    pub fn applies_to(mut self, entity_type: impl Into<String>) -> Self {
        self.applies_to.push(entity_type.into());
        self
    }

    /// Adds an exception.
    #[must_use]
    pub fn exception(mut self, exception: StatuteException) -> Self {
        self.exceptions.push(exception);
        self
    }

    /// Returns accumulated validation errors (when progressive validation is enabled).
    #[must_use]
    pub fn validation_errors(&self) -> &[ValidationError] {
        &self.validation_errors
    }

    /// Checks if an ID is valid.
    fn is_valid_id(&self, id: &str) -> bool {
        !id.is_empty()
            && id
                .chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
            && id.chars().next().is_some_and(|c| c.is_alphabetic())
    }

    /// Builds the statute, returning an error if required fields are missing or validation fails.
    pub fn build(self) -> Result<Statute, Vec<ValidationError>> {
        let mut errors = self.validation_errors;

        // Check required fields
        if self.id.is_none() {
            errors.push(ValidationError::EmptyId);
        }
        if self.title.is_none() {
            errors.push(ValidationError::EmptyTitle);
        }
        if self.effect.is_none() {
            errors.push(ValidationError::EmptyEffectDescription);
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        let statute = Statute {
            id: self.id.unwrap(),
            title: self.title.unwrap(),
            effect: self.effect.unwrap(),
            preconditions: self.preconditions,
            discretion_logic: self.discretion_logic,
            temporal_validity: self.temporal_validity,
            version: self.version,
            jurisdiction: self.jurisdiction,
            derives_from: self.derives_from,
            applies_to: self.applies_to,
            exceptions: self.exceptions,
        };

        // Final validation
        let validation_errors = statute.validate();
        if !validation_errors.is_empty() {
            Err(validation_errors)
        } else {
            Ok(statute)
        }
    }
}

impl Default for StatuteBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Typestate Builder Pattern for Compile-Time Verification
// ============================================================================

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

use builder_states::*;

/// Type-safe builder for `Statute` using the typestate pattern.
///
/// This builder ensures at compile time that all required fields (id, title, effect)
/// are set before building a statute. The type parameters track which fields have been set.
///
/// # Type Parameters
///
/// - `I`: ID state (`NoId` or `HasId`)
/// - `T`: Title state (`NoTitle` or `HasTitle`)
/// - `E`: Effect state (`NoEffect` or `HasEffect`)
///
/// # Examples
///
/// ```
/// use legalis_core::{TypedStatuteBuilder, Effect, EffectType, Condition, ComparisonOp};
///
/// // This compiles - all required fields are set
/// let statute = TypedStatuteBuilder::new()
///     .id("tax-law-2025")
///     .title("Income Tax Credit")
///     .effect(Effect::new(EffectType::Grant, "Tax credit of $1000"))
///     .with_precondition(Condition::Income {
///         operator: ComparisonOp::LessThan,
///         value: 50000,
///     })
///     .build();
///
/// assert_eq!(statute.id, "tax-law-2025");
/// ```
///
/// ```compile_fail
/// use legalis_core::TypedStatuteBuilder;
///
/// // This won't compile - missing title and effect
/// let statute = TypedStatuteBuilder::new()
///     .id("tax-law-2025")
///     .build(); // ERROR: build() not available
/// ```
#[derive(Debug, Clone)]
pub struct TypedStatuteBuilder<I, T, E> {
    id: Option<String>,
    title: Option<String>,
    effect: Option<Effect>,
    preconditions: Vec<Condition>,
    discretion_logic: Option<String>,
    temporal_validity: TemporalValidity,
    version: u32,
    jurisdiction: Option<String>,
    _phantom: std::marker::PhantomData<(I, T, E)>,
}

impl TypedStatuteBuilder<NoId, NoTitle, NoEffect> {
    /// Creates a new builder with no fields set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: None,
            title: None,
            effect: None,
            preconditions: Vec::new(),
            discretion_logic: None,
            temporal_validity: TemporalValidity::default(),
            version: 1,
            jurisdiction: None,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T, E> TypedStatuteBuilder<NoId, T, E> {
    /// Sets the statute ID (required field).
    ///
    /// Transitions from `NoId` to `HasId` state.
    #[must_use]
    pub fn id(self, id: impl Into<String>) -> TypedStatuteBuilder<HasId, T, E> {
        TypedStatuteBuilder {
            id: Some(id.into()),
            title: self.title,
            effect: self.effect,
            preconditions: self.preconditions,
            discretion_logic: self.discretion_logic,
            temporal_validity: self.temporal_validity,
            version: self.version,
            jurisdiction: self.jurisdiction,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<I, E> TypedStatuteBuilder<I, NoTitle, E> {
    /// Sets the statute title (required field).
    ///
    /// Transitions from `NoTitle` to `HasTitle` state.
    #[must_use]
    pub fn title(self, title: impl Into<String>) -> TypedStatuteBuilder<I, HasTitle, E> {
        TypedStatuteBuilder {
            id: self.id,
            title: Some(title.into()),
            effect: self.effect,
            preconditions: self.preconditions,
            discretion_logic: self.discretion_logic,
            temporal_validity: self.temporal_validity,
            version: self.version,
            jurisdiction: self.jurisdiction,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<I, T> TypedStatuteBuilder<I, T, NoEffect> {
    /// Sets the statute effect (required field).
    ///
    /// Transitions from `NoEffect` to `HasEffect` state.
    #[must_use]
    pub fn effect(self, effect: Effect) -> TypedStatuteBuilder<I, T, HasEffect> {
        TypedStatuteBuilder {
            id: self.id,
            title: self.title,
            effect: Some(effect),
            preconditions: self.preconditions,
            discretion_logic: self.discretion_logic,
            temporal_validity: self.temporal_validity,
            version: self.version,
            jurisdiction: self.jurisdiction,
            _phantom: std::marker::PhantomData,
        }
    }
}

// Methods available in all states
impl<I, T, E> TypedStatuteBuilder<I, T, E> {
    /// Adds a precondition (optional field).
    #[must_use]
    pub fn with_precondition(mut self, condition: Condition) -> Self {
        self.preconditions.push(condition);
        self
    }

    /// Sets the discretion logic (optional field).
    #[must_use]
    pub fn with_discretion(mut self, logic: impl Into<String>) -> Self {
        self.discretion_logic = Some(logic.into());
        self
    }

    /// Sets temporal validity (optional field).
    #[must_use]
    pub fn with_temporal_validity(mut self, validity: TemporalValidity) -> Self {
        self.temporal_validity = validity;
        self
    }

    /// Sets the version (optional field, defaults to 1).
    #[must_use]
    pub fn with_version(mut self, version: u32) -> Self {
        self.version = version;
        self
    }

    /// Sets the jurisdiction (optional field).
    #[must_use]
    pub fn with_jurisdiction(mut self, jurisdiction: impl Into<String>) -> Self {
        self.jurisdiction = Some(jurisdiction.into());
        self
    }
}

// build() only available when all required fields are set
impl TypedStatuteBuilder<HasId, HasTitle, HasEffect> {
    /// Builds the `Statute` (only available when all required fields are set).
    ///
    /// This method is only callable when the builder has transitioned through
    /// all required states (HasId, HasTitle, HasEffect).
    #[must_use]
    pub fn build(self) -> Statute {
        Statute {
            id: self.id.expect("ID must be set"),
            title: self.title.expect("Title must be set"),
            effect: self.effect.expect("Effect must be set"),
            preconditions: self.preconditions,
            discretion_logic: self.discretion_logic,
            temporal_validity: self.temporal_validity,
            version: self.version,
            jurisdiction: self.jurisdiction,
            derives_from: Vec::new(),
            applies_to: Vec::new(),
            exceptions: Vec::new(),
        }
    }
}

impl Default for TypedStatuteBuilder<NoId, NoTitle, NoEffect> {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Phantom Types for Jurisdiction-Specific Statutes
// ============================================================================

/// Marker trait for jurisdictions.
///
/// This trait enables compile-time verification that statutes are used
/// in the correct jurisdiction context.
pub trait Jurisdiction: std::fmt::Debug + Clone {
    /// Returns the jurisdiction code (e.g., "US", "UK", "US-CA").
    fn code() -> &'static str;
}

/// United States jurisdiction marker.
#[derive(Debug, Clone, Copy)]
pub struct US;

impl Jurisdiction for US {
    fn code() -> &'static str {
        "US"
    }
}

/// United Kingdom jurisdiction marker.
#[derive(Debug, Clone, Copy)]
pub struct UK;

impl Jurisdiction for UK {
    fn code() -> &'static str {
        "UK"
    }
}

/// European Union jurisdiction marker.
#[derive(Debug, Clone, Copy)]
pub struct EU;

impl Jurisdiction for EU {
    fn code() -> &'static str {
        "EU"
    }
}

/// California (US-CA) jurisdiction marker.
#[derive(Debug, Clone, Copy)]
pub struct California;

impl Jurisdiction for California {
    fn code() -> &'static str {
        "US-CA"
    }
}

/// New York (US-NY) jurisdiction marker.
#[derive(Debug, Clone, Copy)]
pub struct NewYork;

impl Jurisdiction for NewYork {
    fn code() -> &'static str {
        "US-NY"
    }
}

/// Generic marker for any jurisdiction.
#[derive(Debug, Clone, Copy)]
pub struct AnyJurisdiction;

impl Jurisdiction for AnyJurisdiction {
    fn code() -> &'static str {
        ""
    }
}

/// Jurisdiction-specific statute wrapper using phantom types.
///
/// This type enforces at compile time that statutes are used in the correct
/// jurisdiction context. The type parameter `J` ensures that you can't mix
/// statutes from different jurisdictions without explicit conversion.
///
/// # Type Parameters
///
/// - `J`: Jurisdiction marker type implementing the `Jurisdiction` trait
///
/// # Examples
///
/// ```
/// use legalis_core::{JurisdictionStatute, US, UK, Statute, Effect, EffectType};
///
/// // Create a US-specific statute
/// let us_statute = Statute::new("tax-law", "Tax Law", Effect::new(EffectType::Grant, "Tax credit"));
/// let us_law = JurisdictionStatute::<US>::new(us_statute);
///
/// // Create a UK-specific statute
/// let uk_statute = Statute::new("uk-law", "UK Law", Effect::new(EffectType::Grant, "Benefit"));
/// let uk_law = JurisdictionStatute::<UK>::new(uk_statute);
///
/// // These types are different and can't be mixed
/// assert_eq!(us_law.jurisdiction_code(), "US");
/// assert_eq!(uk_law.jurisdiction_code(), "UK");
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct JurisdictionStatute<J: Jurisdiction> {
    statute: Statute,
    _phantom: std::marker::PhantomData<J>,
}

impl<J: Jurisdiction> JurisdictionStatute<J> {
    /// Creates a new jurisdiction-specific statute.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{JurisdictionStatute, US, Statute, Effect, EffectType};
    ///
    /// let statute = Statute::new("law-1", "Law", Effect::new(EffectType::Grant, "Benefit"));
    /// let us_law = JurisdictionStatute::<US>::new(statute);
    /// ```
    #[must_use]
    pub fn new(mut statute: Statute) -> Self {
        // Automatically set the jurisdiction if not already set
        if statute.jurisdiction.is_none() {
            statute.jurisdiction = Some(J::code().to_string());
        }
        Self {
            statute,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Returns the jurisdiction code for this statute.
    #[must_use]
    pub fn jurisdiction_code(&self) -> &'static str {
        J::code()
    }

    /// Returns a reference to the underlying statute.
    #[must_use]
    pub fn statute(&self) -> &Statute {
        &self.statute
    }

    /// Consumes self and returns the underlying statute.
    #[must_use]
    pub fn into_statute(self) -> Statute {
        self.statute
    }

    /// Converts this statute to a different jurisdiction.
    ///
    /// This is an explicit operation that requires the caller to acknowledge
    /// they are changing jurisdictions.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{JurisdictionStatute, US, UK, Statute, Effect, EffectType};
    ///
    /// let statute = Statute::new("law", "Law", Effect::new(EffectType::Grant, "Benefit"));
    /// let us_law = JurisdictionStatute::<US>::new(statute);
    /// let uk_law = us_law.convert_to::<UK>();
    /// assert_eq!(uk_law.jurisdiction_code(), "UK");
    /// ```
    #[must_use]
    pub fn convert_to<K: Jurisdiction>(mut self) -> JurisdictionStatute<K> {
        self.statute.jurisdiction = Some(K::code().to_string());
        JurisdictionStatute {
            statute: self.statute,
            _phantom: std::marker::PhantomData,
        }
    }
}

/// Collection of jurisdiction-specific statutes.
///
/// This type ensures all statutes in the collection belong to the same jurisdiction.
///
/// # Examples
///
/// ```
/// use legalis_core::{JurisdictionStatuteRegistry, US, JurisdictionStatute, Statute, Effect, EffectType};
///
/// let mut registry = JurisdictionStatuteRegistry::<US>::new();
///
/// let statute1 = Statute::new("law-1", "Law 1", Effect::new(EffectType::Grant, "Benefit 1"));
/// let statute2 = Statute::new("law-2", "Law 2", Effect::new(EffectType::Grant, "Benefit 2"));
///
/// registry.add(JurisdictionStatute::new(statute1));
/// registry.add(JurisdictionStatute::new(statute2));
///
/// assert_eq!(registry.len(), 2);
/// assert_eq!(registry.jurisdiction_code(), "US");
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct JurisdictionStatuteRegistry<J: Jurisdiction> {
    statutes: Vec<JurisdictionStatute<J>>,
    _phantom: std::marker::PhantomData<J>,
}

impl<J: Jurisdiction> JurisdictionStatuteRegistry<J> {
    /// Creates a new empty registry for a specific jurisdiction.
    #[must_use]
    pub fn new() -> Self {
        Self {
            statutes: Vec::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Adds a statute to the registry.
    pub fn add(&mut self, statute: JurisdictionStatute<J>) {
        self.statutes.push(statute);
    }

    /// Returns the number of statutes in the registry.
    #[must_use]
    pub fn len(&self) -> usize {
        self.statutes.len()
    }

    /// Returns true if the registry is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.statutes.is_empty()
    }

    /// Returns the jurisdiction code for this registry.
    #[must_use]
    pub fn jurisdiction_code(&self) -> &'static str {
        J::code()
    }

    /// Returns an iterator over the statutes.
    pub fn iter(&self) -> impl Iterator<Item = &JurisdictionStatute<J>> {
        self.statutes.iter()
    }

    /// Finds a statute by ID.
    #[must_use]
    pub fn find(&self, id: &str) -> Option<&JurisdictionStatute<J>> {
        self.statutes.iter().find(|s| s.statute().id == id)
    }
}

impl<J: Jurisdiction> Default for JurisdictionStatuteRegistry<J> {
    fn default() -> Self {
        Self::new()
    }
}

/// Macro for defining custom jurisdiction types with automatic boilerplate.
///
/// # Examples
///
/// ```
/// use legalis_core::{define_jurisdiction, Jurisdiction};
///
/// define_jurisdiction! {
///     /// Texas jurisdiction
///     Texas => "US-TX"
/// }
///
/// // Now Texas can be used as a jurisdiction marker
/// let code = Texas::code();
/// assert_eq!(code, "US-TX");
/// ```
#[macro_export]
macro_rules! define_jurisdiction {
    (
        $(#[$meta:meta])*
        $name:ident => $code:expr
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy)]
        pub struct $name;

        impl $crate::Jurisdiction for $name {
            fn code() -> &'static str {
                $code
            }
        }
    };
}

/// Macro for defining custom condition types with automatic boilerplate.
///
/// This macro generates a custom condition wrapper type with:
/// - A struct to hold the condition data
/// - Constructor methods
/// - Display trait implementation
/// - Conversion to `Condition::Custom` variant
/// - Evaluation helper method
///
/// # Examples
///
/// ```
/// use legalis_core::{define_custom_condition, Condition, EvaluationContext, EvaluationError};
///
/// // Define a custom "Employment Status" condition
/// define_custom_condition! {
///     /// Checks if entity has specific employment status
///     EmploymentStatus {
///         status: String,
///         requires_full_time: bool,
///     }
/// }
///
/// // Use the custom condition
/// let cond = EmploymentStatus::new("engineer".to_string(), true);
/// let custom_cond: Condition = cond.into();
/// assert!(matches!(custom_cond, Condition::Custom { .. }));
/// ```
///
/// The macro generates:
///
/// ```text
/// pub struct EmploymentStatus {
///     pub status: String,
///     pub requires_full_time: bool,
/// }
///
/// impl EmploymentStatus {
///     pub fn new(status: String, requires_full_time: bool) -> Self { ... }
///     pub fn to_condition(&self) -> Condition { ... }
/// }
///
/// impl From<EmploymentStatus> for Condition { ... }
/// impl std::fmt::Display for EmploymentStatus { ... }
/// ```
#[macro_export]
macro_rules! define_custom_condition {
    (
        $(#[$meta:meta])*
        $name:ident {
            $($field:ident: $field_type:ty),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq)]
        pub struct $name {
            $(pub $field: $field_type,)*
        }

        impl $name {
            /// Creates a new instance of this custom condition.
            #[must_use]
            pub fn new($($field: $field_type),*) -> Self {
                Self {
                    $($field,)*
                }
            }

            /// Converts this custom condition to a `Condition::Custom`.
            #[must_use]
            pub fn to_condition(&self) -> $crate::Condition {
                $crate::Condition::Custom {
                    description: self.to_string(),
                }
            }

            /// Evaluates this condition against a context.
            ///
            /// Override this method in your implementation to provide custom evaluation logic.
            #[allow(dead_code)]
            pub fn evaluate<C: $crate::EvaluationContext>(
                &self,
                _context: &C,
            ) -> Result<bool, $crate::EvaluationError> {
                // Default implementation always returns an error
                // Users should implement their own evaluation logic
                Err($crate::EvaluationError::Custom {
                    message: format!(
                        "Evaluation not implemented for custom condition type '{}'",
                        stringify!($name)
                    ),
                })
            }
        }

        impl From<$name> for $crate::Condition {
            fn from(custom: $name) -> Self {
                custom.to_condition()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}(", stringify!($name))?;
                let mut first = true;
                $(
                    if !first {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {:?}", stringify!($field), self.$field)?;
                    first = false;
                )*
                write!(f, ")")
            }
        }
    };
}

// Helper macro for adding conditions
#[doc(hidden)]
#[macro_export]
macro_rules! statute_impl_add_conditions {
    ($statute:ident, age >= $age:expr, $($rest:tt)*) => {
        $statute = $statute.with_precondition(
            $crate::Condition::age($crate::ComparisonOp::GreaterOrEqual, $age)
        );
        statute_impl_add_conditions!($statute, $($rest)*);
    };
    ($statute:ident, age > $age:expr, $($rest:tt)*) => {
        $statute = $statute.with_precondition(
            $crate::Condition::age($crate::ComparisonOp::GreaterThan, $age)
        );
        statute_impl_add_conditions!($statute, $($rest)*);
    };
    ($statute:ident, age < $age:expr, $($rest:tt)*) => {
        $statute = $statute.with_precondition(
            $crate::Condition::age($crate::ComparisonOp::LessThan, $age)
        );
        statute_impl_add_conditions!($statute, $($rest)*);
    };
    ($statute:ident, age <= $age:expr, $($rest:tt)*) => {
        $statute = $statute.with_precondition(
            $crate::Condition::age($crate::ComparisonOp::LessOrEqual, $age)
        );
        statute_impl_add_conditions!($statute, $($rest)*);
    };
    ($statute:ident, income >= $income:expr, $($rest:tt)*) => {
        $statute = $statute.with_precondition(
            $crate::Condition::income($crate::ComparisonOp::GreaterOrEqual, $income)
        );
        statute_impl_add_conditions!($statute, $($rest)*);
    };
    ($statute:ident, income < $income:expr, $($rest:tt)*) => {
        $statute = $statute.with_precondition(
            $crate::Condition::income($crate::ComparisonOp::LessThan, $income)
        );
        statute_impl_add_conditions!($statute, $($rest)*);
    };
    ($statute:ident, has_attribute $attr:expr, $($rest:tt)*) => {
        $statute = $statute.with_precondition(
            $crate::Condition::has_attribute($attr)
        );
        statute_impl_add_conditions!($statute, $($rest)*);
    };
    ($statute:ident,) => {};
    ($statute:ident) => {};
}

// Helper macro for adding exceptions
#[doc(hidden)]
#[macro_export]
macro_rules! statute_impl_add_exceptions {
    // We'll just skip exceptions for now - users can add them manually
    // This is a simplified version for the macro
    ($statute:ident, $($rest:tt)*) => {};
}

/// Declarative macro for defining statutes with a clean, readable syntax.
///
/// This macro provides a convenient way to define statutes using a declarative syntax
/// that resembles natural language and legal documents.
///
/// # Examples
///
/// ```
/// use legalis_core::{statute, EffectType};
///
/// let voting_law = statute! {
///     id: "voting-rights-2025",
///     title: "Voting Rights Act",
///     effect: Grant("Right to vote in federal elections"),
///     jurisdiction: "US",
///     version: 1,
/// };
///
/// assert_eq!(voting_law.id, "voting-rights-2025");
/// assert_eq!(voting_law.jurisdiction, Some("US".to_string()));
/// ```
///
/// Simple syntax:
///
/// ```
/// use legalis_core::{statute, EffectType};
///
/// let tax_law = statute! {
///     id: "income-tax-2025",
///     title: "Income Tax Law",
///     effect: Obligation("Pay income tax"),
///     jurisdiction: "US",
/// };
///
/// assert_eq!(tax_law.id, "income-tax-2025");
/// ```
#[macro_export]
macro_rules! statute {
    // Syntax with all optional fields
    (
        id: $id:expr,
        title: $title:expr,
        effect: $effect_type:ident($effect_desc:expr)
        $(, jurisdiction: $jurisdiction:expr)?
        $(, version: $version:expr)?
        $(, discretion: $discretion:expr)?
        $(,)?
    ) => {{
        let mut statute = $crate::Statute::new(
            $id,
            $title,
            $crate::Effect::new(
                $crate::EffectType::$effect_type,
                $effect_desc
            )
        );

        $(
            statute = statute.with_jurisdiction($jurisdiction);
        )?
        $(
            statute = statute.with_version($version);
        )?
        $(
            statute = statute.with_discretion($discretion);
        )?

        statute
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    // Property-based testing with proptest
    #[cfg(test)]
    mod proptests {
        use super::*;
        use proptest::prelude::*;

        // Strategy for generating valid statute IDs
        fn statute_id_strategy() -> impl Strategy<Value = String> {
            "[a-z][a-z0-9_-]{0,30}".prop_map(|s| s.to_string())
        }

        // Strategy for generating comparison operators
        fn comparison_op_strategy() -> impl Strategy<Value = ComparisonOp> {
            prop_oneof![
                Just(ComparisonOp::Equal),
                Just(ComparisonOp::NotEqual),
                Just(ComparisonOp::GreaterThan),
                Just(ComparisonOp::GreaterOrEqual),
                Just(ComparisonOp::LessThan),
                Just(ComparisonOp::LessOrEqual),
            ]
        }

        // Strategy for generating reasonable ages
        fn age_strategy() -> impl Strategy<Value = u32> {
            0u32..150u32
        }

        // Strategy for generating conditions
        fn condition_strategy() -> impl Strategy<Value = Condition> {
            let leaf = prop_oneof![
                (comparison_op_strategy(), age_strategy()).prop_map(|(op, age)| Condition::Age {
                    operator: op,
                    value: age
                }),
                (comparison_op_strategy(), any::<u64>()).prop_map(|(op, income)| {
                    Condition::Income {
                        operator: op,
                        value: income,
                    }
                }),
                any::<String>().prop_map(|key| Condition::HasAttribute { key }),
                (any::<String>(), any::<String>())
                    .prop_map(|(key, value)| Condition::AttributeEquals { key, value }),
                any::<String>().prop_map(|desc| Condition::Custom { description: desc }),
            ];
            leaf.prop_recursive(
                3,  // max depth
                16, // max nodes
                5,  // items per collection
                |inner| {
                    prop_oneof![
                        (inner.clone(), inner.clone())
                            .prop_map(|(a, b)| Condition::And(Box::new(a), Box::new(b))),
                        (inner.clone(), inner.clone())
                            .prop_map(|(a, b)| Condition::Or(Box::new(a), Box::new(b))),
                        inner.clone().prop_map(|c| Condition::Not(Box::new(c))),
                    ]
                },
            )
        }

        proptest! {
            #[test]
            fn test_legal_result_map_preserves_deterministic(value in any::<i32>()) {
                let result = LegalResult::Deterministic(value);
                let mapped = result.map(|x| x + 1);
                prop_assert!(mapped.is_deterministic());
                if let LegalResult::Deterministic(v) = mapped {
                    prop_assert_eq!(v, value + 1);
                }
            }

            #[test]
            fn test_legal_result_discretion_stays_discretion(issue in "\\PC+", hint in proptest::option::of("\\PC+")) {
                let result: LegalResult<i32> = LegalResult::JudicialDiscretion {
                    issue: issue.clone(),
                    context_id: Uuid::new_v4(),
                    narrative_hint: hint.clone(),
                };
                let mapped = result.map(|x: i32| x + 1);
                prop_assert!(mapped.requires_discretion());
            }

            #[test]
            fn test_comparison_op_display_roundtrip(op in comparison_op_strategy()) {
                let display = format!("{}", op);
                prop_assert!(!display.is_empty());
                prop_assert!(display.len() <= 2);
            }

            #[test]
            fn test_condition_display_not_empty(cond in condition_strategy()) {
                let display = format!("{}", cond);
                prop_assert!(!display.is_empty());
            }

            #[test]
            fn test_statute_id_validation(id in statute_id_strategy()) {
                let statute = Statute::new(
                    id.clone(),
                    "Test Statute",
                    Effect::new(EffectType::Grant, "Test effect"),
                );
                // Valid IDs should not produce InvalidId error
                let errors = statute.validate();
                prop_assert!(!errors.iter().any(|e| matches!(e, ValidationError::InvalidId(_))));
            }

            #[test]
            fn test_temporal_validity_consistency(
                eff_days in 0i64..1000i64,
                exp_days in 0i64..1000i64
            ) {
                let base_date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
                let effective = base_date + chrono::Duration::days(eff_days);
                let expiry = base_date + chrono::Duration::days(exp_days);

                let validity = TemporalValidity::new()
                    .with_effective_date(effective)
                    .with_expiry_date(expiry);

                // The statute should be active between effective and expiry dates
                if effective <= expiry {
                    prop_assert!(validity.is_active(effective));
                    prop_assert!(validity.is_active(expiry));
                    if eff_days < exp_days {
                        let mid_date = base_date + chrono::Duration::days((eff_days + exp_days) / 2);
                        prop_assert!(validity.is_active(mid_date));
                    }
                }
            }

            #[test]
            fn test_typed_entity_u32_roundtrip(key in "[a-z_]{1,20}", value in any::<u32>()) {
                let mut entity = TypedEntity::new();
                entity.set_u32(key.clone(), value);
                let retrieved = entity.get_u32(&key);
                prop_assert_eq!(retrieved, Ok(value));
            }

            #[test]
            fn test_typed_entity_bool_roundtrip(key in "[a-z_]{1,20}", value in any::<bool>()) {
                let mut entity = TypedEntity::new();
                entity.set_bool(key.clone(), value);
                let retrieved = entity.get_bool(&key);
                prop_assert_eq!(retrieved, Ok(value));
            }

            #[test]
            fn test_typed_entity_string_roundtrip(
                key in "[a-z_]{1,20}",
                value in "\\PC*"
            ) {
                let mut entity = TypedEntity::new();
                entity.set_string(key.clone(), value.clone());
                let retrieved = entity.get_string(&key);
                prop_assert_eq!(retrieved, Ok(value.as_str()));
            }

            #[test]
            fn test_effect_parameters_preservation(
                key in "[a-z_]{1,10}",
                value in "\\PC{0,20}"
            ) {
                let effect = Effect::new(EffectType::Grant, "Test effect")
                    .with_parameter(key.clone(), value.clone());

                prop_assert_eq!(effect.parameters.get(&key).map(String::as_str), Some(value.as_str()));
                prop_assert_eq!(effect.parameters.len(), 1);
            }

            #[test]
            fn test_statute_version_validation(version in 1u32..1000u32) {
                let statute = Statute::new(
                    "test-statute",
                    "Test Statute",
                    Effect::new(EffectType::Grant, "Test"),
                ).with_version(version);

                let errors = statute.validate();
                // Versions >= 1 should not produce InvalidVersion error
                prop_assert!(!errors.iter().any(|e| matches!(e, ValidationError::InvalidVersion)));
            }
        }

        // Additional property tests for edge cases
        proptest! {
            #[test]
            fn test_basic_entity_attribute_storage(
                key in "[a-z_]{1,20}",
                value in "\\PC{0,50}"
            ) {
                let mut entity = BasicEntity::new();
                entity.set_attribute(&key, value.clone());
                let retrieved = entity.get_attribute(&key);
                prop_assert_eq!(retrieved, Some(value));
            }

            #[test]
            fn test_statute_builder_preserves_properties(
                id in statute_id_strategy(),
                title in "\\PC{1,100}",
                jurisdiction in proptest::option::of("[A-Z]{2,3}")
            ) {
                let statute = Statute::new(
                    id.clone(),
                    title.clone(),
                    Effect::new(EffectType::Grant, "Test"),
                ).with_jurisdiction(jurisdiction.clone().unwrap_or_default());

                prop_assert_eq!(statute.id, id);
                prop_assert_eq!(statute.title, title);
            }

            #[test]
            fn test_legal_result_void_stays_void(reason in "\\PC+") {
                let result: LegalResult<i32> = LegalResult::Void { reason: reason.clone() };
                let mapped = result.map(|x| x * 2);
                prop_assert!(mapped.is_void());
            }
        }
    }

    #[test]
    fn test_legal_result_deterministic() {
        let result: LegalResult<i32> = LegalResult::Deterministic(42);
        assert!(result.is_deterministic());
        assert!(!result.requires_discretion());
        assert!(!result.is_void());
    }

    #[test]
    fn test_legal_result_discretion() {
        let result: LegalResult<i32> = LegalResult::JudicialDiscretion {
            issue: "test issue".to_string(),
            context_id: Uuid::new_v4(),
            narrative_hint: None,
        };
        assert!(!result.is_deterministic());
        assert!(result.requires_discretion());
    }

    #[test]
    fn test_legal_result_map() {
        let result: LegalResult<i32> = LegalResult::Deterministic(21);
        let mapped = result.map(|x| x * 2);
        assert_eq!(mapped, LegalResult::Deterministic(42));
    }

    #[test]
    fn test_legal_result_display() {
        let det: LegalResult<i32> = LegalResult::Deterministic(42);
        assert_eq!(format!("{}", det), "Deterministic(42)");

        let disc: LegalResult<i32> = LegalResult::JudicialDiscretion {
            issue: "test issue".to_string(),
            context_id: Uuid::new_v4(),
            narrative_hint: Some("consider facts".to_string()),
        };
        assert!(format!("{}", disc).contains("test issue"));
        assert!(format!("{}", disc).contains("consider facts"));
    }

    #[test]
    fn test_basic_entity() {
        let mut entity = BasicEntity::new();
        entity.set_attribute("age", "25".to_string());
        assert_eq!(entity.get_attribute("age"), Some("25".to_string()));
        assert_eq!(entity.get_attribute("nonexistent"), None);
    }

    #[test]
    fn test_statute_builder() {
        let statute = Statute::new(
            "test-statute-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "Grant test permission"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
        .with_discretion("Consider special circumstances");

        assert_eq!(statute.id, "test-statute-1");
        assert_eq!(statute.preconditions.len(), 1);
        assert!(statute.discretion_logic.is_some());
    }

    #[test]
    fn test_temporal_validity() {
        let today = NaiveDate::from_ymd_opt(2025, 6, 15).unwrap();
        let past = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let future = NaiveDate::from_ymd_opt(2026, 12, 31).unwrap();

        let validity = TemporalValidity::new()
            .with_effective_date(past)
            .with_expiry_date(future);

        assert!(validity.is_active(today));
        assert!(!validity.is_active(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()));
        assert!(!validity.is_active(NaiveDate::from_ymd_opt(2027, 1, 1).unwrap()));
    }

    #[test]
    fn test_statute_with_temporal_validity() {
        let statute = Statute::new(
            "sunset-test",
            "Sunset Test Act",
            Effect::new(EffectType::Grant, "Temporary grant"),
        )
        .with_temporal_validity(
            TemporalValidity::new()
                .with_effective_date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
                .with_expiry_date(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()),
        )
        .with_jurisdiction("US-CA");

        assert!(statute.is_active(NaiveDate::from_ymd_opt(2025, 6, 1).unwrap()));
        assert!(!statute.is_active(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()));
        assert_eq!(statute.jurisdiction, Some("US-CA".to_string()));
    }

    #[test]
    fn test_condition_display() {
        let age_cond = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };
        assert_eq!(format!("{}", age_cond), "age >= 18");

        let and_cond = Condition::And(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            }),
            Box::new(Condition::Income {
                operator: ComparisonOp::LessThan,
                value: 50000,
            }),
        );
        assert!(format!("{}", and_cond).contains("AND"));
    }

    #[test]
    fn test_geographic_condition() {
        let cond = Condition::Geographic {
            region_type: RegionType::State,
            region_id: "CA".to_string(),
        };
        assert!(format!("{}", cond).contains("State"));
        assert!(format!("{}", cond).contains("CA"));
    }

    #[test]
    fn test_entity_relationship_condition() {
        let cond = Condition::EntityRelationship {
            relationship_type: RelationshipType::Employment,
            target_entity_id: Some("employer-123".to_string()),
        };
        assert!(format!("{}", cond).contains("Employment"));
    }

    #[test]
    fn test_statute_display() {
        let statute = Statute::new(
            "display-test",
            "Display Test Act",
            Effect::new(EffectType::Grant, "Test grant"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 21,
        })
        .with_version(2)
        .with_jurisdiction("JP");

        let display = format!("{}", statute);
        assert!(display.contains("display-test"));
        assert!(display.contains("Display Test Act"));
        assert!(display.contains("VERSION: 2"));
        assert!(display.contains("JP"));
    }

    #[test]
    fn test_statute_validation_valid() {
        let statute = Statute::new(
            "valid-statute",
            "Valid Statute",
            Effect::new(EffectType::Grant, "Grant something"),
        );

        assert!(statute.is_valid());
        assert!(statute.validate().is_empty());
    }

    #[test]
    fn test_statute_validation_empty_id() {
        let mut statute = Statute::new("temp", "Test", Effect::new(EffectType::Grant, "Grant"));
        statute.id = String::new();

        let errors = statute.validate();
        assert!(errors.iter().any(|e| matches!(e, ValidationError::EmptyId)));
    }

    #[test]
    fn test_statute_validation_invalid_id() {
        let mut statute = Statute::new("temp", "Test", Effect::new(EffectType::Grant, "Grant"));
        statute.id = "123-invalid".to_string(); // Starts with number

        let errors = statute.validate();
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, ValidationError::InvalidId(_)))
        );
    }

    #[test]
    fn test_statute_validation_empty_title() {
        let mut statute = Statute::new("test-id", "temp", Effect::new(EffectType::Grant, "Grant"));
        statute.title = String::new();

        let errors = statute.validate();
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, ValidationError::EmptyTitle))
        );
    }

    #[test]
    fn test_statute_validation_expiry_before_effective() {
        let statute = Statute::new(
            "temporal-error",
            "Temporal Error Statute",
            Effect::new(EffectType::Grant, "Grant"),
        )
        .with_temporal_validity(TemporalValidity {
            effective_date: Some(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()),
            expiry_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()), // Before effective!
            enacted_at: None,
            amended_at: None,
        });

        let errors = statute.validate();
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, ValidationError::ExpiryBeforeEffective { .. }))
        );
    }

    #[test]
    fn test_statute_validation_invalid_condition() {
        let statute = Statute::new(
            "age-error",
            "Age Error Statute",
            Effect::new(EffectType::Grant, "Grant"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 200, // Unrealistic age
        });

        let errors = statute.validate();
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, ValidationError::InvalidCondition { .. }))
        );
    }

    #[test]
    fn test_statute_validation_zero_version() {
        let mut statute = Statute::new(
            "zero-version",
            "Zero Version Statute",
            Effect::new(EffectType::Grant, "Grant"),
        );
        statute.version = 0;

        let errors = statute.validate();
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, ValidationError::InvalidVersion))
        );
    }

    #[test]
    fn test_statute_validated_method() {
        let valid_statute = Statute::new(
            "valid",
            "Valid Statute",
            Effect::new(EffectType::Grant, "Grant"),
        );
        assert!(valid_statute.validated().is_ok());

        let mut invalid_statute = Statute::new(
            "invalid",
            "Invalid",
            Effect::new(EffectType::Grant, "Grant"),
        );
        invalid_statute.id = String::new();
        assert!(invalid_statute.validated().is_err());
    }

    #[test]
    fn test_validation_error_display() {
        assert!(ValidationError::EmptyId.to_string().contains("empty"));
        assert!(ValidationError::EmptyTitle.to_string().contains("title"));
        assert!(
            ValidationError::InvalidVersion
                .to_string()
                .contains("Version")
        );
    }

    #[test]
    fn test_typed_entity_basic_operations() {
        let mut entity = TypedEntity::new();

        // Set various typed attributes
        entity.set_u32("age", 25);
        entity.set_u64("income", 50000);
        entity.set_bool("is_citizen", true);
        entity.set_string("name", "Alice");
        entity.set_date("birth_date", NaiveDate::from_ymd_opt(1999, 1, 15).unwrap());
        entity.set_f64("tax_rate", 0.15);

        // Get typed attributes
        assert_eq!(entity.get_u32("age").unwrap(), 25);
        assert_eq!(entity.get_u64("income").unwrap(), 50000);
        assert!(entity.get_bool("is_citizen").unwrap());
        assert_eq!(entity.get_string("name").unwrap(), "Alice");
        assert_eq!(
            entity.get_date("birth_date").unwrap(),
            NaiveDate::from_ymd_opt(1999, 1, 15).unwrap()
        );
        assert_eq!(entity.get_f64("tax_rate").unwrap(), 0.15);

        // Test attribute existence
        assert!(entity.has_attribute("age"));
        assert!(!entity.has_attribute("nonexistent"));
    }

    #[test]
    fn test_typed_entity_type_safety() {
        let mut entity = TypedEntity::new();
        entity.set_string("name", "Bob");

        // Attempting to get string as u32 should fail
        assert!(entity.get_u32("name").is_err());

        // Not found error
        assert!(entity.get_u32("missing").is_err());
    }

    #[test]
    fn test_typed_entity_legal_entity_trait() {
        let mut entity = TypedEntity::new();

        // Test LegalEntity trait implementation
        let _id = entity.id();
        assert!(entity.get_attribute("age").is_none());

        // Set via LegalEntity trait (uses string parsing)
        entity.set_attribute("age", "30".to_string());
        assert_eq!(entity.get_attribute("age").unwrap(), "30");

        // Verify it was parsed as u32
        assert_eq!(entity.get_u32("age").unwrap(), 30);

        // Set boolean via trait
        entity.set_attribute("active", "true".to_string());
        assert!(entity.get_bool("active").unwrap());
    }

    #[test]
    fn test_typed_entity_backward_compatibility() {
        let mut entity = TypedEntity::new();

        // Simulate old string-based code
        entity.set_attribute("age", "25".to_string());
        entity.set_attribute("income", "50000".to_string());
        entity.set_attribute("is_citizen", "true".to_string());

        // New typed code can read these
        assert_eq!(entity.get_u32("age").unwrap(), 25);
        assert_eq!(entity.get_u64("income").unwrap(), 50000);
        assert!(entity.get_bool("is_citizen").unwrap());

        // Verify string retrieval still works
        assert_eq!(entity.get_attribute("age").unwrap(), "25");
        assert_eq!(entity.get_attribute("income").unwrap(), "50000");
        assert_eq!(entity.get_attribute("is_citizen").unwrap(), "true");
    }

    #[test]
    fn test_typed_entity_attribute_value_conversions() {
        let mut entity = TypedEntity::new();

        // Test AttributeValue directly
        entity.set_typed("count", AttributeValue::U32(42));
        let val = entity.get_typed("count").unwrap();
        assert_eq!(val.as_u32().unwrap(), 42);
        assert_eq!(val.as_u64().unwrap(), 42); // Upcasting works

        // Test date parsing
        entity.set_attribute("registration_date", "2024-01-15".to_string());
        assert_eq!(
            entity.get_date("registration_date").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()
        );
    }

    #[test]
    fn test_typed_entity_integration_with_condition() {
        let mut entity = TypedEntity::new();
        entity.set_u32("age", 25);

        // TypedEntity implements LegalEntity, so it works with existing condition checking
        assert_eq!(entity.get_attribute("age").unwrap(), "25");

        // This demonstrates backward compatibility with existing SimEngine code
        let age_from_trait = entity
            .get_attribute("age")
            .and_then(|v| v.parse::<u32>().ok());
        assert_eq!(age_from_trait, Some(25));
    }

    #[test]
    fn test_condition_helpers() {
        let simple = Condition::age(ComparisonOp::GreaterOrEqual, 18);
        assert!(simple.is_simple());
        assert!(!simple.is_compound());
        assert_eq!(simple.count_conditions(), 1);
        assert_eq!(simple.depth(), 1);

        let compound = simple
            .clone()
            .and(Condition::income(ComparisonOp::LessThan, 50000));
        assert!(compound.is_compound());
        assert!(!compound.is_simple());
        assert_eq!(compound.count_conditions(), 3); // AND + 2 leaves
        assert_eq!(compound.depth(), 2);

        let negated = simple.clone().not();
        assert!(negated.is_negation());
        assert!(negated.is_compound());
        assert_eq!(negated.count_conditions(), 2); // NOT + 1 leaf
    }

    #[test]
    fn test_condition_constructors() {
        let age_cond = Condition::age(ComparisonOp::GreaterOrEqual, 21);
        assert!(matches!(age_cond, Condition::Age { value: 21, .. }));

        let income_cond = Condition::income(ComparisonOp::LessThan, 100000);
        assert!(matches!(
            income_cond,
            Condition::Income { value: 100000, .. }
        ));

        let attr_cond = Condition::has_attribute("license");
        assert!(matches!(attr_cond, Condition::HasAttribute { .. }));

        let eq_cond = Condition::attribute_equals("status", "active");
        assert!(matches!(eq_cond, Condition::AttributeEquals { .. }));

        let custom = Condition::custom("Complex eligibility check");
        assert!(matches!(custom, Condition::Custom { .. }));
    }

    #[test]
    fn test_condition_combinators() {
        let c1 = Condition::age(ComparisonOp::GreaterOrEqual, 18);
        let c2 = Condition::income(ComparisonOp::LessThan, 50000);
        let c3 = Condition::has_attribute("citizenship");

        let combined = c1.and(c2).or(c3);
        assert_eq!(combined.count_conditions(), 5); // OR + (AND + 2 leaves) + 1 leaf
        assert_eq!(combined.depth(), 3);
    }

    #[test]
    fn test_comparison_op_inverse() {
        assert_eq!(ComparisonOp::Equal.inverse(), ComparisonOp::NotEqual);
        assert_eq!(ComparisonOp::NotEqual.inverse(), ComparisonOp::Equal);
        assert_eq!(
            ComparisonOp::GreaterThan.inverse(),
            ComparisonOp::LessOrEqual
        );
        assert_eq!(
            ComparisonOp::GreaterOrEqual.inverse(),
            ComparisonOp::LessThan
        );
        assert_eq!(
            ComparisonOp::LessThan.inverse(),
            ComparisonOp::GreaterOrEqual
        );
        assert_eq!(
            ComparisonOp::LessOrEqual.inverse(),
            ComparisonOp::GreaterThan
        );
    }

    #[test]
    fn test_comparison_op_classification() {
        assert!(ComparisonOp::Equal.is_equality());
        assert!(ComparisonOp::NotEqual.is_equality());
        assert!(!ComparisonOp::GreaterThan.is_equality());

        assert!(ComparisonOp::GreaterThan.is_ordering());
        assert!(ComparisonOp::LessThan.is_ordering());
        assert!(!ComparisonOp::Equal.is_ordering());
    }

    #[test]
    fn test_effect_helpers() {
        let mut effect = Effect::new(EffectType::Grant, "Test effect")
            .with_parameter("key1", "value1")
            .with_parameter("key2", "value2");

        assert_eq!(effect.parameter_count(), 2);
        assert!(effect.has_parameter("key1"));
        assert!(!effect.has_parameter("key3"));
        assert_eq!(effect.get_parameter("key1"), Some(&"value1".to_string()));
        assert_eq!(effect.get_parameter("key3"), None);

        let removed = effect.remove_parameter("key1");
        assert_eq!(removed, Some("value1".to_string()));
        assert_eq!(effect.parameter_count(), 1);
    }

    #[test]
    fn test_effect_constructors() {
        let grant = Effect::grant("Right to vote");
        assert_eq!(grant.effect_type, EffectType::Grant);

        let revoke = Effect::revoke("Driving privileges");
        assert_eq!(revoke.effect_type, EffectType::Revoke);

        let obligation = Effect::obligation("Pay taxes");
        assert_eq!(obligation.effect_type, EffectType::Obligation);

        let prohibition = Effect::prohibition("Smoking in public");
        assert_eq!(prohibition.effect_type, EffectType::Prohibition);
    }

    #[test]
    fn test_statute_helper_methods() {
        let statute = Statute::new("test-id", "Test Statute", Effect::grant("Test permission"))
            .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 18))
            .with_precondition(Condition::has_attribute("citizenship"))
            .with_discretion("Consider special circumstances");

        assert_eq!(statute.precondition_count(), 2);
        assert!(statute.has_preconditions());
        assert!(statute.has_discretion());
        assert!(!statute.has_jurisdiction());

        let with_jurisdiction = statute.clone().with_jurisdiction("US");
        assert!(with_jurisdiction.has_jurisdiction());

        let conditions = statute.preconditions();
        assert_eq!(conditions.len(), 2);
    }

    #[test]
    fn test_temporal_validity_helpers() {
        use chrono::Utc;

        let validity = TemporalValidity::new()
            .with_effective_date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
            .with_expiry_date(NaiveDate::from_ymd_opt(2030, 12, 31).unwrap())
            .with_enacted_at(Utc::now());

        assert!(validity.has_effective_date());
        assert!(validity.has_expiry_date());
        assert!(validity.is_enacted());
        assert!(!validity.is_amended());

        let test_date_active = NaiveDate::from_ymd_opt(2026, 6, 15).unwrap();
        let test_date_expired = NaiveDate::from_ymd_opt(2031, 1, 1).unwrap();
        let test_date_pending = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();

        assert!(validity.is_active(test_date_active));
        assert!(validity.has_expired(test_date_expired));
        assert!(validity.is_pending(test_date_pending));
    }

    #[test]
    fn test_duration_condition() {
        let employment = Condition::duration(ComparisonOp::GreaterOrEqual, 5, DurationUnit::Years);
        assert!(matches!(employment, Condition::Duration { .. }));
        assert_eq!(format!("{}", employment), "duration >= 5 years");

        let probation = Condition::duration(ComparisonOp::LessThan, 90, DurationUnit::Days);
        assert_eq!(format!("{}", probation), "duration < 90 days");
    }

    #[test]
    fn test_percentage_condition() {
        let ownership = Condition::percentage(ComparisonOp::GreaterOrEqual, 25, "ownership");
        assert!(matches!(ownership, Condition::Percentage { .. }));
        assert_eq!(format!("{}", ownership), "ownership >= 25%");

        let threshold = Condition::percentage(ComparisonOp::LessThan, 50, "voting_power");
        assert_eq!(format!("{}", threshold), "voting_power < 50%");
    }

    #[test]
    fn test_set_membership_condition() {
        let status_in = Condition::in_set(
            "status",
            vec![
                "active".to_string(),
                "pending".to_string(),
                "approved".to_string(),
            ],
        );
        assert!(matches!(status_in, Condition::SetMembership { .. }));
        assert_eq!(
            format!("{}", status_in),
            "status IN {active, pending, approved}"
        );

        let status_not_in = Condition::not_in_set(
            "status",
            vec!["rejected".to_string(), "canceled".to_string()],
        );
        assert_eq!(
            format!("{}", status_not_in),
            "status NOT IN {rejected, canceled}"
        );
    }

    #[test]
    fn test_pattern_condition() {
        let matches = Condition::matches_pattern("id", "^[A-Z]{2}[0-9]{6}$");
        assert!(matches!(matches, Condition::Pattern { .. }));
        assert_eq!(format!("{}", matches), "id =~ /^[A-Z]{2}[0-9]{6}$/");

        let not_matches = Condition::not_matches_pattern("email", ".*@spam\\.com$");
        assert_eq!(format!("{}", not_matches), "email !~ /.*@spam\\.com$/");
    }

    #[test]
    fn test_duration_unit_display() {
        assert_eq!(format!("{}", DurationUnit::Days), "days");
        assert_eq!(format!("{}", DurationUnit::Weeks), "weeks");
        assert_eq!(format!("{}", DurationUnit::Months), "months");
        assert_eq!(format!("{}", DurationUnit::Years), "years");
    }

    #[test]
    fn test_duration_unit_ordering() {
        assert!(DurationUnit::Days < DurationUnit::Weeks);
        assert!(DurationUnit::Weeks < DurationUnit::Months);
        assert!(DurationUnit::Months < DurationUnit::Years);
    }

    #[test]
    fn test_new_conditions_with_combinators() {
        let employment_eligible =
            Condition::duration(ComparisonOp::GreaterOrEqual, 1, DurationUnit::Years).and(
                Condition::percentage(ComparisonOp::GreaterOrEqual, 80, "attendance"),
            );

        assert!(format!("{}", employment_eligible).contains("AND"));
        assert!(format!("{}", employment_eligible).contains("duration"));
        assert!(format!("{}", employment_eligible).contains("attendance"));

        let status_check =
            Condition::in_set("status", vec!["active".to_string(), "verified".to_string()])
                .or(Condition::matches_pattern("id", "^VIP-"));

        assert!(format!("{}", status_check).contains("OR"));
        assert!(format!("{}", status_check).contains("IN"));
        assert!(format!("{}", status_check).contains("=~"));
    }

    #[test]
    fn test_new_conditions_count_and_depth() {
        let simple = Condition::duration(ComparisonOp::GreaterOrEqual, 5, DurationUnit::Years);
        assert_eq!(simple.count_conditions(), 1);
        assert_eq!(simple.depth(), 1);
        assert!(simple.is_simple());
        assert!(!simple.is_compound());

        let compound = Condition::percentage(ComparisonOp::GreaterOrEqual, 25, "ownership").and(
            Condition::in_set("status", vec!["active".to_string(), "verified".to_string()]),
        );
        assert_eq!(compound.count_conditions(), 3);
        assert_eq!(compound.depth(), 2);
        assert!(!compound.is_simple());
        assert!(compound.is_compound());
    }

    #[test]
    fn test_conflict_resolution_temporal_precedence() {
        let old_law = Statute::new(
            "old-1",
            "Old Law",
            Effect::new(EffectType::Grant, "Old grant"),
        )
        .with_temporal_validity(
            TemporalValidity::new()
                .with_effective_date(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()),
        );

        let new_law = Statute::new(
            "new-1",
            "New Law",
            Effect::new(EffectType::Prohibition, "New prohibition"),
        )
        .with_temporal_validity(
            TemporalValidity::new()
                .with_effective_date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
        );

        let resolution = StatuteConflictAnalyzer::resolve(&old_law, &new_law);
        assert_eq!(
            resolution,
            ConflictResolution::SecondPrevails(ConflictReason::TemporalPrecedence)
        );
    }

    #[test]
    fn test_conflict_resolution_specificity() {
        let general = Statute::new(
            "general",
            "General Law",
            Effect::new(EffectType::Grant, "General grant"),
        )
        .with_temporal_validity(
            TemporalValidity::new()
                .with_effective_date(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()),
        );

        let specific = Statute::new(
            "specific",
            "Specific Law",
            Effect::new(EffectType::Prohibition, "Specific prohibition"),
        )
        .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 18))
        .with_precondition(Condition::income(ComparisonOp::LessThan, 50000))
        .with_temporal_validity(
            TemporalValidity::new()
                .with_effective_date(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()),
        );

        let resolution = StatuteConflictAnalyzer::resolve(&general, &specific);
        assert_eq!(
            resolution,
            ConflictResolution::SecondPrevails(ConflictReason::Specificity)
        );
    }

    #[test]
    fn test_conflict_resolution_hierarchy() {
        let state_law = Statute::new(
            "state-1",
            "State Law",
            Effect::new(EffectType::Grant, "State grant"),
        )
        .with_jurisdiction("US-NY")
        .with_temporal_validity(
            TemporalValidity::new()
                .with_effective_date(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()),
        );

        let federal_law = Statute::new(
            "fed-1",
            "Federal Law",
            Effect::new(EffectType::Prohibition, "Federal prohibition"),
        )
        .with_jurisdiction("US")
        .with_temporal_validity(
            TemporalValidity::new()
                .with_effective_date(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()),
        );

        let resolution = StatuteConflictAnalyzer::resolve(&state_law, &federal_law);
        assert_eq!(
            resolution,
            ConflictResolution::SecondPrevails(ConflictReason::Hierarchy)
        );
    }

    #[test]
    fn test_conflict_resolution_no_conflict() {
        let law1 = Statute::new("law-1", "Law 1", Effect::new(EffectType::Grant, "Grant A"));

        let law2 = Statute::new("law-2", "Law 2", Effect::new(EffectType::Grant, "Grant B"));

        let resolution = StatuteConflictAnalyzer::resolve(&law1, &law2);
        assert_eq!(resolution, ConflictResolution::NoConflict);
    }

    #[test]
    fn test_conflict_resolution_unresolvable() {
        // Two conflicting statutes with same date, specificity, and jurisdiction
        let law1 = Statute::new("law-1", "Law 1", Effect::new(EffectType::Grant, "Grant"))
            .with_temporal_validity(
                TemporalValidity::new()
                    .with_effective_date(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()),
            )
            .with_jurisdiction("US");

        let law2 = Statute::new(
            "law-2",
            "Law 2",
            Effect::new(EffectType::Prohibition, "Prohibition"),
        )
        .with_temporal_validity(
            TemporalValidity::new()
                .with_effective_date(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()),
        )
        .with_jurisdiction("US");

        let resolution = StatuteConflictAnalyzer::resolve(&law1, &law2);
        assert!(matches!(resolution, ConflictResolution::Unresolvable(_)));
    }

    #[test]
    fn test_conflict_resolution_is_in_effect() {
        let statute = Statute::new("test", "Test", Effect::grant("Test")).with_temporal_validity(
            TemporalValidity::new()
                .with_effective_date(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
                .with_expiry_date(NaiveDate::from_ymd_opt(2030, 12, 31).unwrap()),
        );

        assert!(StatuteConflictAnalyzer::is_in_effect(
            &statute,
            NaiveDate::from_ymd_opt(2025, 6, 15).unwrap()
        ));
        assert!(!StatuteConflictAnalyzer::is_in_effect(
            &statute,
            NaiveDate::from_ymd_opt(2019, 1, 1).unwrap()
        ));
        assert!(!StatuteConflictAnalyzer::is_in_effect(
            &statute,
            NaiveDate::from_ymd_opt(2031, 1, 1).unwrap()
        ));
    }

    #[test]
    fn test_conflict_resolution_resolve_conflicts_at_date() {
        let old_general = Statute::new("old-gen", "Old General", Effect::grant("Grant"))
            .with_temporal_validity(
                TemporalValidity::new()
                    .with_effective_date(NaiveDate::from_ymd_opt(2015, 1, 1).unwrap()),
            );

        let new_specific = Statute::new("new-spec", "New Specific", Effect::grant("Grant"))
            .with_temporal_validity(
                TemporalValidity::new()
                    .with_effective_date(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()),
            )
            .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 18));

        let federal = Statute::new("federal", "Federal", Effect::grant("Grant"))
            .with_temporal_validity(
                TemporalValidity::new()
                    .with_effective_date(NaiveDate::from_ymd_opt(2018, 1, 1).unwrap()),
            )
            .with_jurisdiction("US");

        let expired = Statute::new("expired", "Expired", Effect::grant("Grant"))
            .with_temporal_validity(
                TemporalValidity::new()
                    .with_effective_date(NaiveDate::from_ymd_opt(2010, 1, 1).unwrap())
                    .with_expiry_date(NaiveDate::from_ymd_opt(2015, 12, 31).unwrap()),
            );

        let statutes = vec![old_general, new_specific, federal, expired];
        let active = StatuteConflictAnalyzer::resolve_conflicts_at_date(
            &statutes,
            NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
        );

        // Should have 3 active (expired should be filtered out)
        assert_eq!(active.len(), 3);

        // Most recent and specific should be first
        assert_eq!(active[0].id, "new-spec");
    }

    #[test]
    fn test_conflict_reason_display() {
        assert_eq!(
            format!("{}", ConflictReason::TemporalPrecedence),
            "lex posterior (later law prevails)"
        );
        assert_eq!(
            format!("{}", ConflictReason::Specificity),
            "lex specialis (more specific law prevails)"
        );
        assert_eq!(
            format!("{}", ConflictReason::Hierarchy),
            "lex superior (higher authority prevails)"
        );
    }

    #[test]
    fn test_jurisdiction_level_detection() {
        // Federal/National
        assert_eq!(
            StatuteConflictAnalyzer::jurisdiction_level(&Some("US".to_string())),
            3
        );
        assert_eq!(
            StatuteConflictAnalyzer::jurisdiction_level(&Some("Federal".to_string())),
            3
        );

        // State/Provincial
        assert_eq!(
            StatuteConflictAnalyzer::jurisdiction_level(&Some("US-NY".to_string())),
            2
        );
        assert_eq!(
            StatuteConflictAnalyzer::jurisdiction_level(&Some("State-CA".to_string())),
            2
        );

        // Local/Municipal
        assert_eq!(
            StatuteConflictAnalyzer::jurisdiction_level(&Some("Local-NYC".to_string())),
            1
        );

        // Unknown/None
        assert_eq!(StatuteConflictAnalyzer::jurisdiction_level(&None), 0);
    }
}
