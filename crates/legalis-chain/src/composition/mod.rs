//! Contract-composition toolkit — build large multi-contract systems from
//! reusable parts.
//!
//! This module provides the "Contract Composition" capabilities used to assemble,
//! parameterize and order multi-contract deployments. Unlike the codegen modules
//! ([`crate::tokenization`], [`crate::evolution`], [`crate::autonomous`]) which
//! each emit one specialised contract, this module works at the *structural*
//! level and is platform-agnostic where possible:
//!
//! * **Modular contract builder** — [`ModularContractBuilder`] composes a single
//!   Solidity contract from reusable [`ContractComponent`] mixins, deduplicating
//!   imports/bases and rejecting colliding members.
//! * **Template library** — [`TemplateLibrary`] holds parameterized
//!   [`ContractTemplate`]s with typed, validated parameters and ships a curated
//!   set of production EVM building blocks via [`TemplateLibrary::with_builtins`].
//! * **Inheritance optimizer** — [`InheritanceHierarchy`] resolves multiple
//!   inheritance via exact C3 linearization (the same MRO algorithm Solidity
//!   uses) and flattens declared parents into the order the compiler accepts,
//!   dropping transitively-redundant bases.
//! * **Dependency management** — [`DependencyGraph`] tracks inter-contract
//!   "depends-on" edges and produces a deterministic topological deployment
//!   order, detecting cycles.
//!
//! All logic here is pure Rust with no external dependencies beyond the standard
//! library and the crate's own error type, so every transformation is validated
//! and independently unit-testable.

mod builder;
mod dependencies;
mod inheritance;
mod templates;

#[cfg(test)]
mod tests;

pub use builder::{ContractComponent, MAX_COMPONENTS, ModularContractBuilder};
pub use dependencies::{DependencyGraph, MAX_DEPENDENCY_NODES};
pub use inheritance::{InheritanceHierarchy, InheritanceNode, MAX_INHERITANCE_NODES};
pub use templates::{ContractTemplate, ParamKind, TemplateLibrary, TemplateParam};
