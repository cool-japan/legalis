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
pub mod typed_attributes;

use chrono::{DateTime, NaiveDate, Utc};
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
    /// Logical AND of conditions
    And(Box<Condition>, Box<Condition>),
    /// Logical OR of conditions
    Or(Box<Condition>, Box<Condition>),
    /// Logical NOT
    Not(Box<Condition>),
    /// Custom condition with description
    Custom { description: String },
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
            Self::And(left, right) => write!(f, "({} AND {})", left, right),
            Self::Or(left, right) => write!(f, "({} OR {})", left, right),
            Self::Not(inner) => write!(f, "NOT {}", inner),
            Self::Custom { description } => write!(f, "custom({})", description),
        }
    }
}

/// Geographic region types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
}

impl fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.effect_type, self.description)
    }
}

/// Types of legal effects.
#[derive(Debug, Clone, PartialEq, Eq)]
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

    /// Checks if the statute is currently active.
    pub fn is_active(&self, as_of: NaiveDate) -> bool {
        let after_effective = self.effective_date.is_none_or(|d| as_of >= d);
        let before_expiry = self.expiry_date.is_none_or(|d| as_of <= d);
        after_effective && before_expiry
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

    /// Checks if the statute is currently active.
    pub fn is_active(&self, as_of: NaiveDate) -> bool {
        self.temporal_validity.is_active(as_of)
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
}

/// Validation errors for statutes.
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
}
