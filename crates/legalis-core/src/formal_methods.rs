//! Formal methods integration for legal reasoning.
//!
//! This module provides exports to formal verification tools including:
//! - Coq proof assistant
//! - Lean 4 theorem prover
//! - TLA+ specification language
//! - Alloy modeling language
//! - SMT-LIB solver format
//!
//! ## Overview
//!
//! Formal methods allow rigorous verification of legal reasoning properties:
//! - **Consistency**: No contradictory conclusions
//! - **Completeness**: All cases are covered
//! - **Correctness**: Rules match intended semantics
//! - **Termination**: Evaluation always completes
//!
//! ## Example
//!
//! ```
//! use legalis_core::formal_methods::CoqExporter;
//! use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
//!
//! let statute = Statute::new("s1", "Adult Benefit", Effect::new(EffectType::Grant, "Benefit"))
//!     .with_precondition(Condition::Age { operator: ComparisonOp::GreaterOrEqual, value: 18 });
//!
//! let exporter = CoqExporter::new();
//! let coq_code = exporter.export_statute(&statute);
//! assert!(coq_code.contains("Definition"));
//! ```

use crate::{ComparisonOp, Condition, Effect, EffectType, Statute};

/// Coq proof assistant exporter.
///
/// Exports legal statutes and conditions as Coq definitions and theorems
/// for formal verification.
pub struct CoqExporter {
    /// Module name for exported Coq code
    module_name: String,
}

impl CoqExporter {
    /// Creates a new Coq exporter with default module name.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::formal_methods::CoqExporter;
    ///
    /// let exporter = CoqExporter::new();
    /// ```
    pub fn new() -> Self {
        Self {
            module_name: "LegalReasoning".to_string(),
        }
    }

    /// Creates a Coq exporter with custom module name.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::formal_methods::CoqExporter;
    ///
    /// let exporter = CoqExporter::with_module("TaxLaw");
    /// ```
    pub fn with_module(module_name: &str) -> Self {
        Self {
            module_name: module_name.to_string(),
        }
    }

    /// Exports a statute as Coq definition.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::formal_methods::CoqExporter;
    /// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
    ///
    /// let statute = Statute::new("adult_benefit", "Adult Benefit",
    ///     Effect::new(EffectType::Grant, "Benefit"))
    ///     .with_precondition(Condition::Age {
    ///         operator: ComparisonOp::GreaterOrEqual,
    ///         value: 18
    ///     });
    ///
    /// let exporter = CoqExporter::new();
    /// let coq = exporter.export_statute(&statute);
    /// assert!(coq.contains("Definition adult_benefit"));
    /// ```
    pub fn export_statute(&self, statute: &Statute) -> String {
        let mut output = String::new();

        // Generate a valid Coq identifier from statute ID
        let coq_id = self.sanitize_identifier(&statute.id);

        // Export statute definition
        output.push_str(&format!("(* Statute: {} *)\n", statute.title));
        output.push_str(&format!("Definition {}_preconditions : Prop :=\n", coq_id));

        if statute.preconditions.is_empty() {
            output.push_str("  True.\n");
        } else {
            for (i, cond) in statute.preconditions.iter().enumerate() {
                if i > 0 {
                    output.push_str("  /\\ ");
                } else {
                    output.push_str("  ");
                }
                output.push_str(&self.export_condition(cond));
                output.push('\n');
            }
            output.push_str(".\n");
        }

        output.push('\n');
        output.push_str(&format!("Definition {}_effect : Prop :=\n", coq_id));
        output.push_str(&format!("  {}.\n", self.export_effect(&statute.effect)));

        output.push('\n');
        output.push_str("(* Legal rule: if preconditions hold, then effect applies *)\n");
        output.push_str(&format!(
            "Axiom {}_rule : {}_preconditions -> {}_effect.\n",
            coq_id, coq_id, coq_id
        ));

        output
    }

    /// Exports a condition as Coq proposition.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::formal_methods::CoqExporter;
    /// use legalis_core::{Condition, ComparisonOp};
    ///
    /// let condition = Condition::Age {
    ///     operator: ComparisonOp::GreaterOrEqual,
    ///     value: 18
    /// };
    ///
    /// let exporter = CoqExporter::new();
    /// let coq = exporter.export_condition(&condition);
    /// assert!(coq.contains("age"));
    /// ```
    pub fn export_condition(&self, condition: &Condition) -> String {
        match condition {
            Condition::Age { operator, value } => {
                format!("(age {} {})", self.export_comparison_op(operator), value)
            }
            Condition::Income { operator, value } => {
                format!("(income {} {})", self.export_comparison_op(operator), value)
            }
            Condition::HasAttribute { key } => {
                format!("(has_attribute \"{}\")", key)
            }
            Condition::AttributeEquals { key, value } => {
                format!("(attribute_equals \"{}\" \"{}\")", key, value)
            }
            Condition::And(left, right) => {
                format!(
                    "({} /\\ {})",
                    self.export_condition(left),
                    self.export_condition(right)
                )
            }
            Condition::Or(left, right) => {
                format!(
                    "({} \\/ {})",
                    self.export_condition(left),
                    self.export_condition(right)
                )
            }
            Condition::Not(condition) => {
                format!("(~ {})", self.export_condition(condition))
            }
            Condition::Geographic { region_id, .. } => {
                format!("(in_region \"{}\")", region_id)
            }
            Condition::DateRange { .. } => "(date_in_range)".to_string(),
            Condition::EntityRelationship { .. } => "(has_relationship)".to_string(),
            Condition::Custom { description } => {
                format!("(custom_condition \"{}\")", description)
            }
            Condition::ResidencyDuration { .. } => "(residency_duration_satisfied)".to_string(),
            Condition::Duration { .. } => "(duration_satisfied)".to_string(),
            Condition::Percentage {
                operator, value, ..
            } => {
                format!(
                    "(percentage {} {})",
                    self.export_comparison_op(operator),
                    value
                )
            }
            Condition::SetMembership {
                attribute, values, ..
            } => {
                format!("(set_membership \"{}\" {:?})", attribute, values)
            }
            Condition::Pattern {
                attribute, pattern, ..
            } => {
                format!("(matches_pattern \"{}\" \"{}\")", attribute, pattern)
            }
            Condition::Calculation { .. } => "(calculation_satisfied)".to_string(),
            Condition::Composite { .. } => "(composite_satisfied)".to_string(),
            Condition::Threshold { .. } => "(threshold_satisfied)".to_string(),
            Condition::Fuzzy { .. } => "(fuzzy_satisfied)".to_string(),
            Condition::Probabilistic { .. } => "(probabilistic_satisfied)".to_string(),
            Condition::Temporal { .. } => "(temporal_satisfied)".to_string(),
        }
    }

    /// Exports a comparison operator to Coq syntax.
    fn export_comparison_op(&self, op: &ComparisonOp) -> &'static str {
        match op {
            ComparisonOp::Equal => "=",
            ComparisonOp::NotEqual => "<>",
            ComparisonOp::LessThan => "<",
            ComparisonOp::LessOrEqual => "<=",
            ComparisonOp::GreaterThan => ">",
            ComparisonOp::GreaterOrEqual => ">=",
        }
    }

    /// Exports an effect as Coq proposition.
    fn export_effect(&self, effect: &Effect) -> String {
        let effect_str = match effect.effect_type {
            EffectType::Grant => "grants",
            EffectType::Revoke => "revokes",
            EffectType::Obligation => "obligates",
            EffectType::Prohibition => "prohibits",
            EffectType::MonetaryTransfer => "transfers",
            EffectType::StatusChange => "changes_status",
            EffectType::Custom => "custom_effect",
        };

        format!("({} \"{}\")", effect_str, effect.description)
    }

    /// Sanitizes a statute ID to be a valid Coq identifier.
    fn sanitize_identifier(&self, id: &str) -> String {
        id.chars()
            .map(|c| if c.is_alphanumeric() { c } else { '_' })
            .collect()
    }

    /// Exports multiple statutes with module header.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::formal_methods::CoqExporter;
    /// use legalis_core::{Statute, Effect, EffectType};
    ///
    /// let statutes = vec![
    ///     Statute::new("s1", "Statute 1", Effect::new(EffectType::Grant, "Benefit")),
    ///     Statute::new("s2", "Statute 2", Effect::new(EffectType::Revoke, "Penalty")),
    /// ];
    ///
    /// let exporter = CoqExporter::new();
    /// let coq_module = exporter.export_module(&statutes);
    /// assert!(coq_module.contains("Module LegalReasoning"));
    /// ```
    pub fn export_module(&self, statutes: &[Statute]) -> String {
        let mut output = String::new();

        // Module header
        output.push_str(&format!("Module {}.\n\n", self.module_name));

        // Type definitions
        output.push_str("(* Basic types for legal reasoning *)\n");
        output.push_str("Parameter age : nat -> Prop.\n");
        output.push_str("Parameter income : nat -> Prop.\n");
        output.push_str("Parameter has_attribute : string -> Prop.\n");
        output.push_str("Parameter attribute_equals : string -> string -> Prop.\n");
        output.push_str("Parameter in_region : string -> Prop.\n");
        output.push_str("Parameter grants : string -> Prop.\n");
        output.push_str("Parameter revokes : string -> Prop.\n");
        output.push_str("Parameter obligates : string -> Prop.\n");
        output.push_str("Parameter prohibits : string -> Prop.\n");
        output.push_str("Parameter transfers : string -> Prop.\n");
        output.push_str("Parameter changes_status : string -> Prop.\n");
        output.push_str("Parameter custom_effect : string -> Prop.\n");
        output.push('\n');

        // Export each statute
        for statute in statutes {
            output.push_str(&self.export_statute(statute));
            output.push('\n');
        }

        // Module footer
        output.push_str(&format!("End {}.\n", self.module_name));

        output
    }
}

impl Default for CoqExporter {
    fn default() -> Self {
        Self::new()
    }
}

/// Lean 4 theorem prover exporter.
///
/// Exports legal statutes and conditions as Lean 4 definitions and theorems
/// for formal verification and interactive theorem proving.
pub struct Lean4Exporter {
    /// Namespace for exported Lean code
    namespace: String,
}

impl Lean4Exporter {
    /// Creates a new Lean 4 exporter with default namespace.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::formal_methods::Lean4Exporter;
    ///
    /// let exporter = Lean4Exporter::new();
    /// ```
    pub fn new() -> Self {
        Self {
            namespace: "LegalReasoning".to_string(),
        }
    }

    /// Creates a Lean 4 exporter with custom namespace.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::formal_methods::Lean4Exporter;
    ///
    /// let exporter = Lean4Exporter::with_namespace("TaxLaw");
    /// ```
    pub fn with_namespace(namespace: &str) -> Self {
        Self {
            namespace: namespace.to_string(),
        }
    }

    /// Exports a statute as Lean 4 definition.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::formal_methods::Lean4Exporter;
    /// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
    ///
    /// let statute = Statute::new("adult_benefit", "Adult Benefit",
    ///     Effect::new(EffectType::Grant, "Benefit"))
    ///     .with_precondition(Condition::Age {
    ///         operator: ComparisonOp::GreaterOrEqual,
    ///         value: 18
    ///     });
    ///
    /// let exporter = Lean4Exporter::new();
    /// let lean = exporter.export_statute(&statute);
    /// assert!(lean.contains("def adult_benefit_preconditions"));
    /// ```
    pub fn export_statute(&self, statute: &Statute) -> String {
        let mut output = String::new();

        // Generate a valid Lean identifier from statute ID
        let lean_id = self.sanitize_identifier(&statute.id);

        // Export statute definition
        output.push_str(&format!("/-- Statute: {} -/\n", statute.title));
        output.push_str(&format!("def {}_preconditions : Prop :=\n", lean_id));

        if statute.preconditions.is_empty() {
            output.push_str("  True\n");
        } else {
            for (i, cond) in statute.preconditions.iter().enumerate() {
                if i > 0 {
                    output.push_str("  ∧ ");
                } else {
                    output.push_str("  ");
                }
                output.push_str(&self.export_condition(cond));
                output.push('\n');
            }
        }

        output.push('\n');
        output.push_str(&format!("def {}_effect : Prop :=\n", lean_id));
        output.push_str(&format!("  {}\n", self.export_effect(&statute.effect)));

        output.push('\n');
        output.push_str("/-- Legal rule: if preconditions hold, then effect applies -/\n");
        output.push_str(&format!(
            "axiom {}_rule : {}_preconditions → {}_effect\n",
            lean_id, lean_id, lean_id
        ));

        output
    }

    /// Exports a condition as Lean 4 proposition.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::formal_methods::Lean4Exporter;
    /// use legalis_core::{Condition, ComparisonOp};
    ///
    /// let condition = Condition::Age {
    ///     operator: ComparisonOp::GreaterOrEqual,
    ///     value: 18
    /// };
    ///
    /// let exporter = Lean4Exporter::new();
    /// let lean = exporter.export_condition(&condition);
    /// assert!(lean.contains("age"));
    /// ```
    pub fn export_condition(&self, condition: &Condition) -> String {
        match condition {
            Condition::Age { operator, value } => {
                format!("(age {} {})", self.export_comparison_op(operator), value)
            }
            Condition::Income { operator, value } => {
                format!("(income {} {})", self.export_comparison_op(operator), value)
            }
            Condition::HasAttribute { key } => {
                format!("(hasAttribute \"{}\")", key)
            }
            Condition::AttributeEquals { key, value } => {
                format!("(attributeEquals \"{}\" \"{}\")", key, value)
            }
            Condition::And(left, right) => {
                format!(
                    "({} ∧ {})",
                    self.export_condition(left),
                    self.export_condition(right)
                )
            }
            Condition::Or(left, right) => {
                format!(
                    "({} ∨ {})",
                    self.export_condition(left),
                    self.export_condition(right)
                )
            }
            Condition::Not(condition) => {
                format!("(¬ {})", self.export_condition(condition))
            }
            Condition::Geographic { region_id, .. } => {
                format!("(inRegion \"{}\")", region_id)
            }
            Condition::DateRange { .. } => "(dateInRange)".to_string(),
            Condition::EntityRelationship { .. } => "(hasRelationship)".to_string(),
            Condition::Custom { description } => {
                format!("(customCondition \"{}\")", description)
            }
            Condition::ResidencyDuration { .. } => "(residencyDurationSatisfied)".to_string(),
            Condition::Duration { .. } => "(durationSatisfied)".to_string(),
            Condition::Percentage {
                operator, value, ..
            } => {
                format!(
                    "(percentage {} {})",
                    self.export_comparison_op(operator),
                    value
                )
            }
            Condition::SetMembership {
                attribute, values, ..
            } => {
                format!("(setMembership \"{}\" {:?})", attribute, values)
            }
            Condition::Pattern {
                attribute, pattern, ..
            } => {
                format!("(matchesPattern \"{}\" \"{}\")", attribute, pattern)
            }
            Condition::Calculation { .. } => "(calculationSatisfied)".to_string(),
            Condition::Composite { .. } => "(compositeSatisfied)".to_string(),
            Condition::Threshold { .. } => "(thresholdSatisfied)".to_string(),
            Condition::Fuzzy { .. } => "(fuzzySatisfied)".to_string(),
            Condition::Probabilistic { .. } => "(probabilisticSatisfied)".to_string(),
            Condition::Temporal { .. } => "(temporalSatisfied)".to_string(),
        }
    }

    /// Exports a comparison operator to Lean 4 syntax.
    fn export_comparison_op(&self, op: &ComparisonOp) -> &'static str {
        match op {
            ComparisonOp::Equal => "=",
            ComparisonOp::NotEqual => "≠",
            ComparisonOp::LessThan => "<",
            ComparisonOp::LessOrEqual => "≤",
            ComparisonOp::GreaterThan => ">",
            ComparisonOp::GreaterOrEqual => "≥",
        }
    }

    /// Exports an effect as Lean 4 proposition.
    fn export_effect(&self, effect: &Effect) -> String {
        let effect_str = match effect.effect_type {
            EffectType::Grant => "grants",
            EffectType::Revoke => "revokes",
            EffectType::Obligation => "obligates",
            EffectType::Prohibition => "prohibits",
            EffectType::MonetaryTransfer => "transfers",
            EffectType::StatusChange => "changesStatus",
            EffectType::Custom => "customEffect",
        };

        format!("({} \"{}\")", effect_str, effect.description)
    }

    /// Sanitizes a statute ID to be a valid Lean identifier.
    fn sanitize_identifier(&self, id: &str) -> String {
        id.chars()
            .map(|c| if c.is_alphanumeric() { c } else { '_' })
            .collect()
    }

    /// Exports multiple statutes with namespace header.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::formal_methods::Lean4Exporter;
    /// use legalis_core::{Statute, Effect, EffectType};
    ///
    /// let statutes = vec![
    ///     Statute::new("s1", "Statute 1", Effect::new(EffectType::Grant, "Benefit")),
    ///     Statute::new("s2", "Statute 2", Effect::new(EffectType::Revoke, "Penalty")),
    /// ];
    ///
    /// let exporter = Lean4Exporter::new();
    /// let lean_module = exporter.export_module(&statutes);
    /// assert!(lean_module.contains("namespace LegalReasoning"));
    /// ```
    pub fn export_module(&self, statutes: &[Statute]) -> String {
        let mut output = String::new();

        // Namespace header
        output.push_str(&format!("namespace {}\n\n", self.namespace));

        // Type definitions
        output.push_str("/-- Basic types for legal reasoning -/\n");
        output.push_str("axiom age : Nat → Prop\n");
        output.push_str("axiom income : Nat → Prop\n");
        output.push_str("axiom hasAttribute : String → Prop\n");
        output.push_str("axiom attributeEquals : String → String → Prop\n");
        output.push_str("axiom inRegion : String → Prop\n");
        output.push_str("axiom grants : String → Prop\n");
        output.push_str("axiom revokes : String → Prop\n");
        output.push_str("axiom obligates : String → Prop\n");
        output.push_str("axiom prohibits : String → Prop\n");
        output.push_str("axiom transfers : String → Prop\n");
        output.push_str("axiom changesStatus : String → Prop\n");
        output.push_str("axiom customEffect : String → Prop\n");
        output.push('\n');

        // Export each statute
        for statute in statutes {
            output.push_str(&self.export_statute(statute));
            output.push('\n');
        }

        // Namespace footer
        output.push_str(&format!("end {}\n", self.namespace));

        output
    }
}

impl Default for Lean4Exporter {
    fn default() -> Self {
        Self::new()
    }
}

/// TLA+ specification exporter for temporal properties.
///
/// Exports legal statutes with temporal validity as TLA+ specifications.
pub struct TLAPlusExporter {
    module_name: String,
}

impl TLAPlusExporter {
    /// Creates a new TLA+ exporter.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::formal_methods::TLAPlusExporter;
    ///
    /// let exporter = TLAPlusExporter::new();
    /// ```
    pub fn new() -> Self {
        Self {
            module_name: "LegalReasoning".to_string(),
        }
    }

    /// Exports statutes as TLA+ specification.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::formal_methods::TLAPlusExporter;
    /// use legalis_core::{Statute, Effect, EffectType};
    ///
    /// let statutes = vec![
    ///     Statute::new("s1", "Statute 1", Effect::new(EffectType::Grant, "Benefit")),
    /// ];
    ///
    /// let exporter = TLAPlusExporter::new();
    /// let spec = exporter.export_module(&statutes);
    /// assert!(spec.contains("MODULE LegalReasoning"));
    /// ```
    pub fn export_module(&self, statutes: &[Statute]) -> String {
        let mut output = String::new();

        output.push_str(&format!("---- MODULE {} ----\n", self.module_name));
        output.push_str("EXTENDS Naturals, Sequences\n\n");

        output.push_str("VARIABLES\n");
        output.push_str("  entity_age,\n");
        output.push_str("  entity_income,\n");
        output.push_str("  entity_attributes,\n");
        output.push_str("  active_effects\n\n");

        for statute in statutes {
            let id = statute.id.replace('-', "_");
            output.push_str(&format!("\\* Statute: {}\n", statute.title));
            output.push_str(&format!("{}Applies ==\n", id));
            output.push_str(&format!(
                "  /\\ active_effects' = active_effects \\cup {{\"{}\"}} \n\n",
                statute.effect.description
            ));
        }

        output.push_str("====\n");
        output
    }
}

impl Default for TLAPlusExporter {
    fn default() -> Self {
        Self::new()
    }
}

/// Alloy model exporter for constraint analysis.
///
/// Exports legal statutes as Alloy models for constraint solving.
pub struct AlloyExporter {
    module_name: String,
}

impl AlloyExporter {
    /// Creates a new Alloy exporter.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::formal_methods::AlloyExporter;
    ///
    /// let exporter = AlloyExporter::new();
    /// ```
    pub fn new() -> Self {
        Self {
            module_name: "LegalReasoning".to_string(),
        }
    }

    /// Exports statutes as Alloy model.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::formal_methods::AlloyExporter;
    /// use legalis_core::{Statute, Effect, EffectType};
    ///
    /// let statutes = vec![
    ///     Statute::new("s1", "Statute 1", Effect::new(EffectType::Grant, "Benefit")),
    /// ];
    ///
    /// let exporter = AlloyExporter::new();
    /// let spec = exporter.export_module(&statutes);
    /// assert!(spec.contains("module LegalReasoning"));
    /// ```
    pub fn export_module(&self, statutes: &[Statute]) -> String {
        let mut output = String::new();

        output.push_str(&format!("module {}\n\n", self.module_name));

        output.push_str("sig Entity {\n");
        output.push_str("  age: Int,\n");
        output.push_str("  income: Int,\n");
        output.push_str("  attributes: set Attribute\n");
        output.push_str("}\n\n");

        output.push_str("sig Attribute {}\n\n");
        output.push_str("sig Effect {}\n\n");

        for statute in statutes {
            let id = statute.id.replace('-', "_");
            output.push_str(&format!("// Statute: {}\n", statute.title));
            output.push_str(&format!("pred {}[e: Entity] {{\n", id));
            output.push_str("  // preconditions go here\n");
            output.push_str("}\n\n");
        }

        output
    }
}

impl Default for AlloyExporter {
    fn default() -> Self {
        Self::new()
    }
}

/// SMT-LIB exporter for solver interoperability.
///
/// Exports legal conditions as SMT-LIB format for use with SMT solvers.
pub struct SMTLIBExporter;

impl SMTLIBExporter {
    /// Creates a new SMT-LIB exporter.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::formal_methods::SMTLIBExporter;
    ///
    /// let exporter = SMTLIBExporter::new();
    /// ```
    pub fn new() -> Self {
        Self
    }

    /// Exports a condition as SMT-LIB formula.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::formal_methods::SMTLIBExporter;
    /// use legalis_core::{Condition, ComparisonOp};
    ///
    /// let condition = Condition::Age {
    ///     operator: ComparisonOp::GreaterOrEqual,
    ///     value: 18
    /// };
    ///
    /// let exporter = SMTLIBExporter::new();
    /// let smt = exporter.export_condition(&condition);
    /// assert!(smt.contains(">="));
    /// ```
    pub fn export_condition(&self, condition: &Condition) -> String {
        match condition {
            Condition::Age { operator, value } => {
                format!("({} age {})", self.export_op(operator), value)
            }
            Condition::Income { operator, value } => {
                format!("({} income {})", self.export_op(operator), value)
            }
            Condition::And(left, right) => {
                format!(
                    "(and {} {})",
                    self.export_condition(left),
                    self.export_condition(right)
                )
            }
            Condition::Or(left, right) => {
                format!(
                    "(or {} {})",
                    self.export_condition(left),
                    self.export_condition(right)
                )
            }
            Condition::Not(cond) => {
                format!("(not {})", self.export_condition(cond))
            }
            _ => "true".to_string(),
        }
    }

    fn export_op(&self, op: &ComparisonOp) -> &'static str {
        match op {
            ComparisonOp::Equal => "=",
            ComparisonOp::NotEqual => "distinct",
            ComparisonOp::LessThan => "<",
            ComparisonOp::LessOrEqual => "<=",
            ComparisonOp::GreaterThan => ">",
            ComparisonOp::GreaterOrEqual => ">=",
        }
    }

    /// Exports a statute as SMT-LIB script.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::formal_methods::SMTLIBExporter;
    /// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
    ///
    /// let statute = Statute::new("s1", "Adult Benefit",
    ///     Effect::new(EffectType::Grant, "Benefit"))
    ///     .with_precondition(Condition::Age {
    ///         operator: ComparisonOp::GreaterOrEqual,
    ///         value: 18
    ///     });
    ///
    /// let exporter = SMTLIBExporter::new();
    /// let smt = exporter.export_statute(&statute);
    /// assert!(smt.contains("declare-const"));
    /// ```
    pub fn export_statute(&self, statute: &Statute) -> String {
        let mut output = String::new();

        output.push_str("; Statute: ");
        output.push_str(&statute.title);
        output.push('\n');
        output.push_str("(declare-const age Int)\n");
        output.push_str("(declare-const income Int)\n");

        if !statute.preconditions.is_empty() {
            output.push_str("(assert ");
            if statute.preconditions.len() == 1 {
                output.push_str(&self.export_condition(&statute.preconditions[0]));
            } else {
                output.push_str("(and");
                for cond in &statute.preconditions {
                    output.push(' ');
                    output.push_str(&self.export_condition(cond));
                }
                output.push(')');
            }
            output.push_str(")\n");
        }

        output.push_str("(check-sat)\n");
        output
    }
}

impl Default for SMTLIBExporter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coq_exporter_new() {
        let exporter = CoqExporter::new();
        assert_eq!(exporter.module_name, "LegalReasoning");
    }

    #[test]
    fn test_coq_exporter_with_module() {
        let exporter = CoqExporter::with_module("TaxLaw");
        assert_eq!(exporter.module_name, "TaxLaw");
    }

    #[test]
    fn test_export_simple_statute() {
        let statute = Statute::new(
            "s1",
            "Adult Benefit",
            Effect::new(EffectType::Grant, "Benefit"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let exporter = CoqExporter::new();
        let coq = exporter.export_statute(&statute);

        assert!(coq.contains("Definition s1_preconditions"));
        assert!(coq.contains("age >= 18"));
        assert!(coq.contains("grants \"Benefit\""));
        assert!(coq.contains("Axiom s1_rule"));
    }

    #[test]
    fn test_export_compound_conditions() {
        let condition = Condition::And(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            }),
            Box::new(Condition::Income {
                operator: ComparisonOp::LessThan,
                value: 50000,
            }),
        );

        let exporter = CoqExporter::new();
        let coq = exporter.export_condition(&condition);

        assert!(coq.contains("/\\"));
        assert!(coq.contains("age >= 18"));
        assert!(coq.contains("income < 50000"));
    }

    #[test]
    fn test_export_module() {
        let statutes = vec![
            Statute::new("s1", "Statute 1", Effect::new(EffectType::Grant, "Benefit")),
            Statute::new(
                "s2",
                "Statute 2",
                Effect::new(EffectType::Revoke, "Penalty"),
            ),
        ];

        let exporter = CoqExporter::new();
        let module = exporter.export_module(&statutes);

        assert!(module.contains("Module LegalReasoning"));
        assert!(module.contains("End LegalReasoning"));
        assert!(module.contains("Parameter age"));
        assert!(module.contains("s1_preconditions"));
        assert!(module.contains("s2_preconditions"));
    }

    #[test]
    fn test_sanitize_identifier() {
        let exporter = CoqExporter::new();
        assert_eq!(
            exporter.sanitize_identifier("tax-credit-2024"),
            "tax_credit_2024"
        );
        assert_eq!(exporter.sanitize_identifier("section.42.a"), "section_42_a");
    }

    // Lean 4 exporter tests
    #[test]
    fn test_lean4_exporter_new() {
        let exporter = Lean4Exporter::new();
        assert_eq!(exporter.namespace, "LegalReasoning");
    }

    #[test]
    fn test_lean4_exporter_with_namespace() {
        let exporter = Lean4Exporter::with_namespace("TaxLaw");
        assert_eq!(exporter.namespace, "TaxLaw");
    }

    #[test]
    fn test_lean4_export_simple_statute() {
        let statute = Statute::new(
            "s1",
            "Adult Benefit",
            Effect::new(EffectType::Grant, "Benefit"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let exporter = Lean4Exporter::new();
        let lean = exporter.export_statute(&statute);

        assert!(lean.contains("def s1_preconditions"));
        assert!(lean.contains("age ≥ 18"));
        assert!(lean.contains("grants \"Benefit\""));
        assert!(lean.contains("axiom s1_rule"));
    }

    #[test]
    fn test_lean4_export_compound_conditions() {
        let condition = Condition::And(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            }),
            Box::new(Condition::Income {
                operator: ComparisonOp::LessThan,
                value: 50000,
            }),
        );

        let exporter = Lean4Exporter::new();
        let lean = exporter.export_condition(&condition);

        assert!(lean.contains("∧"));
        assert!(lean.contains("age ≥ 18"));
        assert!(lean.contains("income < 50000"));
    }

    #[test]
    fn test_lean4_export_module() {
        let statutes = vec![
            Statute::new("s1", "Statute 1", Effect::new(EffectType::Grant, "Benefit")),
            Statute::new(
                "s2",
                "Statute 2",
                Effect::new(EffectType::Revoke, "Penalty"),
            ),
        ];

        let exporter = Lean4Exporter::new();
        let module = exporter.export_module(&statutes);

        assert!(module.contains("namespace LegalReasoning"));
        assert!(module.contains("end LegalReasoning"));
        assert!(module.contains("axiom age"));
        assert!(module.contains("s1_preconditions"));
        assert!(module.contains("s2_preconditions"));
    }

    // TLA+ exporter tests
    #[test]
    fn test_tlaplus_exporter() {
        let statutes = vec![Statute::new(
            "s1",
            "Statute 1",
            Effect::new(EffectType::Grant, "Benefit"),
        )];

        let exporter = TLAPlusExporter::new();
        let spec = exporter.export_module(&statutes);

        assert!(spec.contains("MODULE LegalReasoning"));
        assert!(spec.contains("VARIABLES"));
        assert!(spec.contains("entity_age"));
    }

    // Alloy exporter tests
    #[test]
    fn test_alloy_exporter() {
        let statutes = vec![Statute::new(
            "s1",
            "Statute 1",
            Effect::new(EffectType::Grant, "Benefit"),
        )];

        let exporter = AlloyExporter::new();
        let spec = exporter.export_module(&statutes);

        assert!(spec.contains("module LegalReasoning"));
        assert!(spec.contains("sig Entity"));
        assert!(spec.contains("age: Int"));
    }

    // SMT-LIB exporter tests
    #[test]
    fn test_smtlib_exporter_condition() {
        let condition = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };

        let exporter = SMTLIBExporter::new();
        let smt = exporter.export_condition(&condition);

        assert!(smt.contains(">="));
        assert!(smt.contains("age"));
        assert!(smt.contains("18"));
    }

    #[test]
    fn test_smtlib_exporter_statute() {
        let statute = Statute::new(
            "s1",
            "Adult Benefit",
            Effect::new(EffectType::Grant, "Benefit"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let exporter = SMTLIBExporter::new();
        let smt = exporter.export_statute(&statute);

        assert!(smt.contains("declare-const"));
        assert!(smt.contains("assert"));
        assert!(smt.contains("check-sat"));
    }
}
