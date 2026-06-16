//! Contract inheritance resolution and optimization (C3 linearization).
//!
//! Part of the `composition` module. Solidity resolves multiple inheritance with
//! C3 linearization — the same Method Resolution Order (MRO) algorithm Python
//! uses — and *requires* that the bases be listed "from most base-like to most
//! derived". Getting this order wrong is a frequent source of subtle override
//! bugs and outright compile errors (`Linearization of inheritance graph
//! impossible`).
//!
//! [`InheritanceHierarchy`] models a set of contracts and their declared parents
//! and produces:
//!
//! * the **C3 linearization** of any contract (its full MRO), and
//! * an **optimized base list** — the direct parents reordered into the sequence
//!   Solidity will accept, with redundant (transitively-implied) bases removed.
//!
//! The linearization is computed exactly per the C3 merge rules, so an
//! inconsistent hierarchy is reported as an error rather than silently producing
//! a wrong order.

use std::collections::BTreeMap;

use crate::functions::ChainResult;
use crate::types_19::ChainError;

/// Maximum number of contracts accepted in one hierarchy.
pub const MAX_INHERITANCE_NODES: usize = 256;

/// A single contract declaration in an inheritance hierarchy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InheritanceNode {
    /// Contract name.
    pub name: String,
    /// Direct base contracts, in the order the author declared them.
    pub parents: Vec<String>,
}

/// A set of contracts and their declared parents, resolvable via C3 linearization.
#[derive(Debug, Clone, Default)]
pub struct InheritanceHierarchy {
    /// Declared parents keyed by contract name (declaration order preserved).
    nodes: BTreeMap<String, Vec<String>>,
    /// Insertion order of contract names, for stable iteration.
    order: Vec<String>,
}

impl InheritanceHierarchy {
    /// Creates an empty hierarchy.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of declared contracts.
    pub fn len(&self) -> usize {
        self.order.len()
    }

    /// Returns whether the hierarchy declares no contracts.
    pub fn is_empty(&self) -> bool {
        self.order.is_empty()
    }

    /// Declares a contract with its direct parents (most-base-first or any order;
    /// the linearizer fixes the ordering).
    ///
    /// # Errors
    ///
    /// Returns [`ChainError::GenerationError`] if the name is empty, if a parent
    /// name is empty, if the contract lists itself as a parent, if a parent is
    /// duplicated, or if declaring it would exceed [`MAX_INHERITANCE_NODES`].
    pub fn declare(&mut self, node: InheritanceNode) -> ChainResult<()> {
        if node.name.trim().is_empty() {
            return Err(ChainError::GenerationError(
                "inheritance node name must not be empty".to_string(),
            ));
        }
        if !self.nodes.contains_key(&node.name) && self.order.len() >= MAX_INHERITANCE_NODES {
            return Err(ChainError::GenerationError(format!(
                "inheritance hierarchy exceeds the {MAX_INHERITANCE_NODES}-node limit"
            )));
        }
        for (index, parent) in node.parents.iter().enumerate() {
            if parent.trim().is_empty() {
                return Err(ChainError::GenerationError(format!(
                    "contract '{}' has an empty parent name",
                    node.name
                )));
            }
            if parent == &node.name {
                return Err(ChainError::GenerationError(format!(
                    "contract '{}' cannot inherit from itself",
                    node.name
                )));
            }
            for other in node.parents.iter().skip(index + 1) {
                if parent == other {
                    return Err(ChainError::GenerationError(format!(
                        "contract '{}' lists duplicate parent '{}'",
                        node.name, parent
                    )));
                }
            }
        }
        if !self.nodes.contains_key(&node.name) {
            self.order.push(node.name.clone());
        }
        self.nodes.insert(node.name, node.parents);
        Ok(())
    }

    /// Returns the declared direct parents of `name`. An unknown name (e.g. an
    /// external base like `Ownable`) is treated as a leaf with no parents.
    fn parents_of(&self, name: &str) -> Vec<String> {
        self.nodes.get(name).cloned().unwrap_or_default()
    }

    /// Computes the C3 linearization (MRO) of `name`, most-derived first.
    ///
    /// The result begins with `name` itself and lists every ancestor exactly once
    /// in the order method-resolution would consult them. This is the canonical
    /// algorithm used by Solidity and Python.
    ///
    /// # Errors
    ///
    /// Returns [`ChainError::GenerationError`] if the hierarchy is inconsistent
    /// (no valid linearization exists — the same condition Solidity reports as
    /// "Linearization of inheritance graph impossible") or if a parent cycle is
    /// detected.
    pub fn linearize(&self, name: &str) -> ChainResult<Vec<String>> {
        let mut stack: Vec<String> = Vec::new();
        self.linearize_inner(name, &mut stack)
    }

    fn linearize_inner(&self, name: &str, stack: &mut Vec<String>) -> ChainResult<Vec<String>> {
        if stack.iter().any(|entry| entry == name) {
            return Err(ChainError::GenerationError(format!(
                "inheritance cycle detected at '{name}'"
            )));
        }
        stack.push(name.to_string());

        let parents = self.parents_of(name);
        // C3: L[C] = C + merge(L[P1], L[P2], ..., [P1, P2, ...]).
        // Solidity lists bases most-base-first, so the C3 inputs are reversed to
        // give the most-derived-first MRO that override resolution expects.
        let mut sequences: Vec<Vec<String>> = Vec::with_capacity(parents.len() + 1);
        for parent in parents.iter().rev() {
            sequences.push(self.linearize_inner(parent, stack)?);
        }
        let reversed_parents: Vec<String> = parents.iter().rev().cloned().collect();
        if !reversed_parents.is_empty() {
            sequences.push(reversed_parents);
        }

        let merged = merge_linearizations(name, sequences)?;
        let mut result = Vec::with_capacity(merged.len() + 1);
        result.push(name.to_string());
        result.extend(merged);

        stack.pop();
        Ok(result)
    }

    /// Produces an optimized direct-base list for `name` suitable for emission in
    /// a Solidity `contract X is <bases>` clause.
    ///
    /// The declared parents are:
    /// 1. de-duplicated against each other's transitive ancestors (a base already
    ///    implied by another base is redundant and dropped), and
    /// 2. ordered most-base-first as Solidity requires, consistent with the C3
    ///    linearization so the compiler will accept them.
    ///
    /// # Errors
    ///
    /// Returns [`ChainError::GenerationError`] if the hierarchy cannot be
    /// linearized.
    pub fn optimized_bases(&self, name: &str) -> ChainResult<Vec<String>> {
        let parents = self.parents_of(name);
        if parents.is_empty() {
            return Ok(Vec::new());
        }

        // A direct parent is redundant if it appears in the linearization of
        // another direct parent (i.e. it is transitively inherited already).
        let mut redundant: Vec<String> = Vec::new();
        for parent in &parents {
            for other in &parents {
                if parent == other {
                    continue;
                }
                let other_mro = self.linearize(other)?;
                if other_mro.iter().any(|entry| entry == parent) {
                    redundant.push(parent.clone());
                    break;
                }
            }
        }

        // The MRO lists ancestors most-derived-first; reversing yields the
        // most-base-first order Solidity wants. Keep only the (non-redundant)
        // direct parents, preserving that global ordering.
        let mro = self.linearize(name)?;
        let mut ordered: Vec<String> = Vec::with_capacity(parents.len());
        for ancestor in mro.iter().rev() {
            if ancestor == name {
                continue;
            }
            if parents.iter().any(|parent| parent == ancestor)
                && !redundant.iter().any(|entry| entry == ancestor)
            {
                ordered.push(ancestor.clone());
            }
        }
        Ok(ordered)
    }
}

/// Performs the C3 `merge` over a list of linearization sequences.
///
/// Repeatedly selects a valid "head" — a candidate that appears at the front of
/// some sequence and in the *tail* of none — appends it, and removes it from
/// every sequence. If no valid head exists while sequences remain, the hierarchy
/// is inconsistent.
fn merge_linearizations(target: &str, mut sequences: Vec<Vec<String>>) -> ChainResult<Vec<String>> {
    let mut result: Vec<String> = Vec::new();
    loop {
        sequences.retain(|sequence| !sequence.is_empty());
        if sequences.is_empty() {
            return Ok(result);
        }

        let mut chosen: Option<String> = None;
        for sequence in &sequences {
            let Some(head) = sequence.first() else {
                continue;
            };
            let in_some_tail = sequences
                .iter()
                .any(|other| other.iter().skip(1).any(|candidate| candidate == head));
            if !in_some_tail {
                chosen = Some(head.clone());
                break;
            }
        }

        let Some(head) = chosen else {
            return Err(ChainError::GenerationError(format!(
                "linearization of inheritance graph for '{target}' is impossible \
                 (inconsistent base ordering)"
            )));
        };

        result.push(head.clone());
        for sequence in &mut sequences {
            sequence.retain(|entry| entry != &head);
        }
    }
}
