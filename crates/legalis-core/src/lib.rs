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

// -------------------------------------------------------------------------
// Sub-crate modules (pre-existing)
// -------------------------------------------------------------------------
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

// Legal Explanation Generation (v0.2.7)
pub mod explanation;

// Rule Learning & Discovery (v0.2.8)
pub mod rule_learning;

// Performance Optimization (v0.2.9)
pub mod adaptive_cache;
pub mod gpu_offload;
pub mod jit_framework;
pub mod memory_pool;
pub mod simd_eval;

// AI-Native Legal Reasoning (v0.3.0)
pub mod explainable_ai;
pub mod finetuned_llm;
pub mod hybrid_reasoning;
pub mod llm_interpretation;
pub mod neural_entailment;

// Blockchain & Smart Contract Bridge (v0.3.1)
pub mod blockchain_verification;
pub mod decentralized_registry;
pub mod oracle;
pub mod smart_contract;
pub mod zkp;

// Legal Digital Twins (v0.3.2)
pub mod digital_twin;
pub mod event_sourcing;
pub mod realtime_sync;
pub mod scenario_simulation;
pub mod time_travel;

// Quantum-Ready Legal Logic (v0.3.3)
pub mod quantum;

// Autonomous Legal Agents (v0.3.4)
pub mod autonomous_agents;

// -------------------------------------------------------------------------
// Split modules — contain types previously inlined in lib.rs
// -------------------------------------------------------------------------
pub mod functions;
pub mod trait_impls;
pub mod types;
pub mod types_3;
pub mod types_4;
pub mod types_5;
pub mod types_6;

// Re-export builder_states at crate root so sibling modules can find it
// via `use builder_states::*`
pub use functions::builder_states;

// -------------------------------------------------------------------------
// Re-exports — preserve the original public API
// -------------------------------------------------------------------------

// From case_law
pub use case_law::{
    Case, CaseDatabase, CaseRule, Court, DamageType, Precedent, PrecedentApplication,
    PrecedentWeight,
};

// From typed_attributes
pub use typed_attributes::{AttributeError, AttributeValue, TypedAttributes};

// From functions (traits + core context items)
pub use functions::{EvaluationContext, Jurisdiction, LegalEntity};

// From types (Effect, TypedEntity, entailment, jurisdictions, builders, etc.)
pub use types::{
    AbductiveReasoner, AnyJurisdiction, California, ConditionCache, ConditionalEffect,
    ConflictResolution, Contradiction, CrossJurisdictionAnalyzer, EU, Effect, EntailmentResult,
    ErrorSeverity, ExplanationStep, InferenceStep, LegalExplanation, NewYork, SourceLocation,
    StatuteException, StatuteGraph, TypedEntity, TypedStatuteBuilder, UK, ValidationError,
};

// From types_3 (evaluators, builders, diagnostics, more)
pub use types_3::{
    AttributeBasedContext, BasicEntity, ConditionBuilder, ConditionError, ConflictReason,
    ContradictionType, DefaultValueContext, DiagnosticContext, DiagnosticReporter, DurationUnit,
    EffectBuilder, EffectType, EvaluationAuditTrail, EvaluationError, EvaluationExplanation,
    EvaluationRecord, JurisdictionStatute, JurisdictionStatuteRegistry, PartialBool, ReasoningStep,
    RegionType, StatuteRegistry, StepResult, TemporalEffect, US,
};

// From types_4 (conditions support, conflict analysis, entailment, temporal)
pub use types_4::{
    ComparisonOp, DiagnosticValidationError, EffectDependencyGraph, EntailmentEngine,
    RecurrencePattern, RelationshipType, StatuteChange, StatuteConflictAnalyzer,
    SubsumptionAnalyzer, TemporalValidity,
};

// From types_5 (Condition, ComposedEffect, ForwardChaining, StatuteDiff)
pub use types_5::{ComposedEffect, Condition, ForwardChainingEngine, StatuteDiff};

// From types_6 (LegalResult, Statute, StatuteBuilder, FallbackContext, CompositionStrategy)
pub use types_6::{
    CompositionStrategy, FallbackContext, LegalResult, Statute, StatuteBuilder, StatuteQuery,
};

// Re-export chrono types needed by downstream users and doctests
pub use chrono::{DateTime, NaiveDate};
pub use uuid::Uuid;

// -------------------------------------------------------------------------
// Macros (must remain in lib.rs because they reference $crate::)
// -------------------------------------------------------------------------

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

// -------------------------------------------------------------------------
// Tests
// -------------------------------------------------------------------------
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
