//! Test modules for legalis-viz
//!
//! Re-exports from the parent crate so test submodules can use `use super::*;`.

#[allow(unused_imports)]
pub use super::*;
#[allow(unused_imports)]
pub use legalis_core::{
    ComparisonOp, Condition, DurationUnit, Effect, EffectType, Statute, StatuteChange,
};

mod advanced;
mod basic;
mod complete;
mod extended;
