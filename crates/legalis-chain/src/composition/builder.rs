//! Modular contract builder — compose a contract from reusable components.
//!
//! Part of the `composition` module. A [`ContractComponent`] is a self-contained,
//! reusable fragment (a mixin of imports, inherited bases, state, events,
//! modifiers and functions). [`ModularContractBuilder`] accumulates components and
//! assembles them into one coherent, deterministically-ordered Solidity contract,
//! deduplicating imports and bases and rejecting colliding members so the emitted
//! source compiles.
//!
//! The base contracts contributed by the components are linearized through the
//! [`super::inheritance::InheritanceHierarchy`] so the `is` clause is emitted in
//! the C3 order Solidity accepts.

use std::collections::BTreeSet;

use super::inheritance::{InheritanceHierarchy, InheritanceNode};
use crate::functions::ChainResult;
use crate::types_19::{ChainError, GeneratedContract, TargetPlatform};

/// Maximum number of components accepted in one build.
pub const MAX_COMPONENTS: usize = 256;

/// A named, reusable contract fragment.
///
/// Every field is optional except `name`; an empty list simply contributes
/// nothing of that kind. Members are emitted verbatim, so callers are responsible
/// for the inner Solidity being well-formed — the builder guarantees *structure*
/// (ordering, dedup, base linearization), not the semantics of each snippet.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ContractComponent {
    /// Unique component name (used for collision diagnostics).
    pub name: String,
    /// Import lines (e.g. `import "@openzeppelin/.../Ownable2Step.sol";`),
    /// deduplicated across components.
    pub imports: Vec<String>,
    /// Base contracts this component requires the assembled contract to inherit.
    pub bases: Vec<String>,
    /// State-variable declarations (one Solidity statement each).
    pub state_vars: Vec<String>,
    /// Event declarations.
    pub events: Vec<String>,
    /// Modifier definitions (full `modifier X(...) { ... }` blocks).
    pub modifiers: Vec<String>,
    /// Function definitions (full `function X(...) ... { ... }` blocks).
    pub functions: Vec<String>,
}

impl ContractComponent {
    /// Creates a named, empty component.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Self::default()
        }
    }

    /// Adds an import line (builder-style).
    #[must_use]
    pub fn with_import(mut self, import: impl Into<String>) -> Self {
        self.imports.push(import.into());
        self
    }

    /// Adds a required base contract (builder-style).
    #[must_use]
    pub fn with_base(mut self, base: impl Into<String>) -> Self {
        self.bases.push(base.into());
        self
    }

    /// Adds a state-variable declaration (builder-style).
    #[must_use]
    pub fn with_state_var(mut self, decl: impl Into<String>) -> Self {
        self.state_vars.push(decl.into());
        self
    }

    /// Adds an event declaration (builder-style).
    #[must_use]
    pub fn with_event(mut self, event: impl Into<String>) -> Self {
        self.events.push(event.into());
        self
    }

    /// Adds a modifier definition (builder-style).
    #[must_use]
    pub fn with_modifier(mut self, modifier: impl Into<String>) -> Self {
        self.modifiers.push(modifier.into());
        self
    }

    /// Adds a function definition (builder-style).
    #[must_use]
    pub fn with_function(mut self, function: impl Into<String>) -> Self {
        self.functions.push(function.into());
        self
    }
}

/// Assembles reusable [`ContractComponent`]s into a single Solidity contract.
#[derive(Debug, Clone)]
pub struct ModularContractBuilder {
    name: String,
    platform: TargetPlatform,
    pragma: String,
    constructor_body: Option<String>,
    components: Vec<ContractComponent>,
}

impl ModularContractBuilder {
    /// Starts a new builder for a contract named `name`.
    ///
    /// Defaults to the Solidity target and `pragma solidity ^0.8.20;`.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            platform: TargetPlatform::Solidity,
            pragma: "^0.8.20".to_string(),
            constructor_body: None,
            components: Vec::new(),
        }
    }

    /// Overrides the Solidity pragma version range (builder-style).
    #[must_use]
    pub fn with_pragma(mut self, pragma: impl Into<String>) -> Self {
        self.pragma = pragma.into();
        self
    }

    /// Sets an explicit constructor body (builder-style).
    ///
    /// When omitted, no constructor is emitted (or an empty one if any base
    /// requires construction — that is the author's responsibility via a component
    /// function).
    #[must_use]
    pub fn with_constructor_body(mut self, body: impl Into<String>) -> Self {
        self.constructor_body = Some(body.into());
        self
    }

    /// Appends a component (builder-style).
    #[must_use]
    pub fn with_component(mut self, component: ContractComponent) -> Self {
        self.components.push(component);
        self
    }

    /// Appends a component, validating the running count.
    ///
    /// # Errors
    ///
    /// Returns [`ChainError::GenerationError`] if the component name is empty or
    /// if adding it would exceed [`MAX_COMPONENTS`].
    pub fn add_component(&mut self, component: ContractComponent) -> ChainResult<()> {
        if component.name.trim().is_empty() {
            return Err(ChainError::GenerationError(
                "contract component name must not be empty".to_string(),
            ));
        }
        if self.components.len() >= MAX_COMPONENTS {
            return Err(ChainError::GenerationError(format!(
                "modular builder exceeds the {MAX_COMPONENTS}-component limit"
            )));
        }
        self.components.push(component);
        Ok(())
    }

    /// Returns the de-duplicated, insertion-ordered list of base contracts
    /// contributed by all components, linearized into Solidity's required order.
    ///
    /// # Errors
    ///
    /// Returns [`ChainError::GenerationError`] if the combined bases cannot be
    /// linearized.
    pub fn resolved_bases(&self) -> ChainResult<Vec<String>> {
        let mut seen: BTreeSet<String> = BTreeSet::new();
        let mut bases: Vec<String> = Vec::new();
        for component in &self.components {
            for base in &component.bases {
                if seen.insert(base.clone()) {
                    bases.push(base.clone());
                }
            }
        }
        if bases.is_empty() {
            return Ok(bases);
        }

        // Linearize so a base implied by another base is ordered correctly. The
        // assembled contract declares all collected bases as its direct parents;
        // unknown (external) bases are treated as leaves.
        let mut hierarchy = InheritanceHierarchy::new();
        hierarchy.declare(InheritanceNode {
            name: self.name.clone(),
            parents: bases.clone(),
        })?;
        hierarchy.optimized_bases(&self.name)
    }

    /// Assembles the components into a finished [`GeneratedContract`].
    ///
    /// The emitted source orders sections deterministically — imports, contract
    /// declaration with linearized bases, state, events, modifiers, optional
    /// constructor, then functions — with each member group separated by a comment
    /// banner naming its source component for traceability.
    ///
    /// # Errors
    ///
    /// Returns [`ChainError::GenerationError`] if the contract name is empty, if no
    /// components were added, if two components declare a colliding member, or if
    /// the base set cannot be linearized.
    pub fn build(&self) -> ChainResult<GeneratedContract> {
        if self.name.trim().is_empty() {
            return Err(ChainError::GenerationError(
                "assembled contract name must not be empty".to_string(),
            ));
        }
        if self.components.is_empty() {
            return Err(ChainError::GenerationError(
                "modular builder requires at least one component".to_string(),
            ));
        }
        self.check_member_collisions()?;

        let bases = self.resolved_bases()?;
        let mut source = String::from("// SPDX-License-Identifier: MIT\n");
        source.push_str(&format!("pragma solidity {};\n\n", self.pragma));

        // -- imports (deduplicated, insertion order) ---------------------------
        let mut seen_import: BTreeSet<String> = BTreeSet::new();
        for component in &self.components {
            for import in &component.imports {
                if seen_import.insert(import.clone()) {
                    source.push_str(import);
                    source.push('\n');
                }
            }
        }
        if !seen_import.is_empty() {
            source.push('\n');
        }

        // -- NatSpec + declaration ---------------------------------------------
        source.push_str(&format!("/// @title {}\n", self.name));
        source.push_str(&format!(
            "/// @notice Composed from {} reusable component(s) by Legalis-Chain.\n",
            self.components.len()
        ));
        source.push_str("/// @dev Components: ");
        let names: Vec<&str> = self
            .components
            .iter()
            .map(|component| component.name.as_str())
            .collect();
        source.push_str(&names.join(", "));
        source.push_str(".\n");

        if bases.is_empty() {
            source.push_str(&format!("contract {} {{\n", self.name));
        } else {
            source.push_str(&format!(
                "contract {} is {} {{\n",
                self.name,
                bases.join(", ")
            ));
        }

        self.push_section(
            &mut source,
            "State",
            |component| &component.state_vars,
            false,
        );
        self.push_section(&mut source, "Events", |component| &component.events, false);
        self.push_section(
            &mut source,
            "Modifiers",
            |component| &component.modifiers,
            true,
        );

        if let Some(body) = &self.constructor_body {
            source.push_str("    constructor() {\n");
            for line in body.lines() {
                source.push_str("        ");
                source.push_str(line);
                source.push('\n');
            }
            source.push_str("    }\n\n");
        }

        self.push_section(
            &mut source,
            "Functions",
            |component| &component.functions,
            true,
        );

        source.push_str("}\n");

        Ok(GeneratedContract {
            name: self.name.clone(),
            source,
            platform: self.platform,
            abi: None,
            deployment_script: None,
        })
    }

    /// Appends one member section, banner-commenting each contributing component.
    ///
    /// `block_member` controls spacing: function/modifier blocks get a blank line
    /// between them; single-statement members (state vars, events) are listed
    /// densely.
    fn push_section<F>(&self, source: &mut String, title: &str, select: F, block_member: bool)
    where
        F: Fn(&ContractComponent) -> &Vec<String>,
    {
        let has_any = self
            .components
            .iter()
            .any(|component| !select(component).is_empty());
        if !has_any {
            return;
        }
        source.push_str(&format!("    // ==== {title} ====\n"));
        for component in &self.components {
            let members = select(component);
            if members.is_empty() {
                continue;
            }
            source.push_str(&format!(
                "    // -- from component: {} --\n",
                component.name
            ));
            for member in members {
                for line in member.lines() {
                    source.push_str("    ");
                    source.push_str(line);
                    source.push('\n');
                }
                if block_member {
                    source.push('\n');
                }
            }
        }
        if !block_member {
            source.push('\n');
        }
    }

    /// Rejects exact-duplicate member declarations across components.
    ///
    /// State variables, events, modifiers and functions are compared after
    /// whitespace normalization so that two components contributing the *same*
    /// declaration (a genuine conflict, since Solidity forbids redeclaration) are
    /// caught before emission.
    fn check_member_collisions(&self) -> ChainResult<()> {
        let mut seen: BTreeSet<String> = BTreeSet::new();
        for component in &self.components {
            let groups = [
                ("state variable", &component.state_vars),
                ("event", &component.events),
                ("modifier", &component.modifiers),
                ("function", &component.functions),
            ];
            for (kind, members) in groups {
                for member in members {
                    let key = format!("{kind}:{}", normalize_member(member));
                    if !seen.insert(key) {
                        return Err(ChainError::GenerationError(format!(
                            "duplicate {kind} contributed by component '{}': {}",
                            component.name,
                            member.trim()
                        )));
                    }
                }
            }
        }
        Ok(())
    }
}

/// Collapses all ASCII whitespace runs in `member` to single spaces and trims, so
/// cosmetic formatting differences do not hide genuine duplicate declarations.
fn normalize_member(member: &str) -> String {
    member.split_whitespace().collect::<Vec<_>>().join(" ")
}
