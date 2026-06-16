//! Error type for the Hindley–Milner inference engine.

use crate::DslError;
use std::fmt;

/// An error raised while inferring or checking types in the typed IR.
///
/// The variants carry pre-rendered strings (rather than borrowed type values)
/// so the error stays `Clone` + `PartialEq` and free of inference-engine
/// lifetimes; this also keeps every variant comfortably small.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeInferError {
    /// The occurs-check failed: binding `var` to `ty` would build an infinite
    /// type (e.g. `a ~ List a`).
    OccursCheck {
        /// Rendered name of the offending unification variable.
        var: String,
        /// Rendered type that contains the variable.
        ty: String,
    },
    /// Two monomorphic types could not be unified.
    Mismatch {
        /// Type that was expected (left-hand side of unification).
        expected: String,
        /// Type that was found (right-hand side of unification).
        found: String,
    },
    /// Two record rows could not be unified.
    RowMismatch {
        /// Human readable description of the row conflict.
        message: String,
    },
    /// A closed record row is missing a label demanded by the other row.
    MissingLabel {
        /// The label that could not be found.
        label: String,
        /// Rendering of the closed row that lacked it.
        row: String,
    },
    /// A free term variable was referenced without a binding in the environment.
    UnboundVariable(String),
    /// A data constructor was referenced that is not declared.
    UnknownConstructor(String),
    /// A data type was referenced that is not declared.
    UnknownDataType(String),
    /// A type class was referenced that is not declared.
    UnknownClass(String),
    /// A constructor was applied to the wrong number of arguments.
    ConstructorArity {
        /// Constructor name.
        ctor: String,
        /// Declared arity.
        expected: usize,
        /// Supplied argument count.
        found: usize,
    },
    /// A `match` did not cover all constructors of its scrutinee type.
    NonExhaustiveMatch {
        /// Constructor names (or `_`) that remain uncovered.
        missing: Vec<String>,
    },
    /// A class constraint had no matching instance and could not be discharged.
    NoInstance {
        /// The unsatisfiable predicate, rendered as `Class Type`.
        predicate: String,
    },
    /// A retained constraint mentions a type variable that does not appear in
    /// the inferred type, so it can never be resolved.
    AmbiguousConstraint {
        /// The ambiguous predicate, rendered as `Class Type`.
        predicate: String,
    },
    /// A record literal or extension declared the same label twice.
    DuplicateLabel(String),
}

impl fmt::Display for TypeInferError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OccursCheck { var, ty } => {
                write!(f, "occurs check failed: {var} occurs in {ty}")
            }
            Self::Mismatch { expected, found } => {
                write!(f, "type mismatch: expected {expected}, found {found}")
            }
            Self::RowMismatch { message } => write!(f, "record row mismatch: {message}"),
            Self::MissingLabel { label, row } => {
                write!(f, "record {row} has no label `{label}`")
            }
            Self::UnboundVariable(name) => write!(f, "unbound variable `{name}`"),
            Self::UnknownConstructor(name) => write!(f, "unknown constructor `{name}`"),
            Self::UnknownDataType(name) => write!(f, "unknown data type `{name}`"),
            Self::UnknownClass(name) => write!(f, "unknown type class `{name}`"),
            Self::ConstructorArity {
                ctor,
                expected,
                found,
            } => write!(
                f,
                "constructor `{ctor}` expects {expected} argument(s) but got {found}"
            ),
            Self::NonExhaustiveMatch { missing } => {
                write!(f, "non-exhaustive match; missing: {}", missing.join(", "))
            }
            Self::NoInstance { predicate } => {
                write!(f, "no instance for constraint `{predicate}`")
            }
            Self::AmbiguousConstraint { predicate } => {
                write!(f, "ambiguous constraint `{predicate}`")
            }
            Self::DuplicateLabel(label) => write!(f, "duplicate record label `{label}`"),
        }
    }
}

impl std::error::Error for TypeInferError {}

/// Lets inference errors flow into the crate-wide [`DslError`] so callers that
/// already work with `DslResult` can use `?` against the inference entry points.
impl From<TypeInferError> for DslError {
    fn from(err: TypeInferError) -> Self {
        DslError::InvalidCondition(format!("type inference: {err}"))
    }
}

/// Convenience result alias for inference operations.
pub type InferResult<T> = Result<T, TypeInferError>;
