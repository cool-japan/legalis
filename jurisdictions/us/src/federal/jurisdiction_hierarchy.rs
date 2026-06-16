//! Hierarchical Federal / State Jurisdiction and Conflict Resolution
//!
//! This module wires the generic [`JurisdictionConflictResolver`] from
//! `legalis-core` into the United States' two-tier federalism so that callers can
//! register concrete federal and state laws and have conflicts resolved by the
//! Supremacy Clause (U.S. Const. art. VI, cl. 2).
//!
//! ## Why a wrapper?
//!
//! `legalis_core::multi_jurisdictional` is jurisdiction-agnostic: it models an
//! abstract [`JurisdictionLevel`] ladder (International → Federal → State →
//! Local) and resolves conflicts by *lex superior* (higher authority prevails),
//! then *lex posterior* (later version prevails). For the US we want to:
//!
//! 1. speak in terms of [`FederalismLevel`] (Federal vs State vs Local) and
//!    state codes (e.g., `US-CA`);
//! 2. classify *why* federal law displaces state law using the three preemption
//!    doctrines (express / field / conflict) developed under the Supremacy
//!    Clause; and
//! 3. respect the presumption against preemption in fields of traditional state
//!    police power.
//!
//! This wrapper adapts US statutes into the core [`Statute`] type, delegates the
//! precedence math to the core resolver, and overlays the US-specific preemption
//! vocabulary.

use legalis_core::multi_jurisdictional::{
    ConflictResolution, JurisdictionConflictResolver, JurisdictionLevel,
};
use legalis_core::{Effect, Statute};
use serde::{Deserialize, Serialize};

/// The US federalism tier of a law.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FederalismLevel {
    /// Federal (national) law — enacted by Congress or a federal agency.
    Federal,
    /// State law — enacted by a state legislature or agency.
    State,
    /// Local (municipal / county) law — ordinances and regulations.
    Local,
}

impl FederalismLevel {
    /// Map to the generic core jurisdiction level.
    #[must_use]
    pub fn to_core(self) -> JurisdictionLevel {
        match self {
            Self::Federal => JurisdictionLevel::Federal,
            Self::State => JurisdictionLevel::State,
            Self::Local => JurisdictionLevel::Local,
        }
    }

    /// Human-readable label.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::Federal => "Federal",
            Self::State => "State",
            Self::Local => "Local",
        }
    }
}

/// The doctrinal basis on which federal law displaces state law under the
/// Supremacy Clause.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PreemptionKind {
    /// Express preemption — Congress stated, in statutory text, that federal law
    /// preempts state law.
    Express,
    /// Implied field preemption — the federal scheme is so pervasive, or the
    /// field so dominantly federal, that no room is left for state regulation.
    Field,
    /// Implied conflict preemption — compliance with both is impossible, or state
    /// law stands as an obstacle to the federal objective.
    Conflict,
    /// No preemption — federal and state law coexist.
    None,
}

impl PreemptionKind {
    /// Human-readable description with the controlling doctrine.
    #[must_use]
    pub fn description(&self) -> &'static str {
        match self {
            Self::Express => "Express preemption (explicit statutory preemption clause)",
            Self::Field => {
                "Implied field preemption (comprehensive federal scheme occupies the field; \
                 Rice v. Santa Fe Elevator Corp.)"
            }
            Self::Conflict => {
                "Implied conflict preemption (impossibility or obstacle; Hines v. Davidowitz)"
            }
            Self::None => "No preemption (federal and state law coexist)",
        }
    }
}

/// The result of analyzing a federal–state conflict.
#[derive(Debug, Clone)]
pub struct JurisdictionConflict {
    /// Identifier of the law that prevails.
    pub winner: String,
    /// Whether the federal law preempts the state law, and on what basis.
    pub preemption: PreemptionKind,
    /// Whether the presumption against preemption (traditional state police
    /// power) was applied.
    pub presumption_against_preemption: bool,
    /// Explanation drawn from the core resolver plus US overlay.
    pub explanation: String,
}

/// A US-flavored hierarchical jurisdiction store backed by the core resolver.
#[derive(Debug, Clone, Default)]
pub struct JurisdictionHierarchy {
    resolver: JurisdictionConflictResolver,
}

impl JurisdictionHierarchy {
    /// Create an empty hierarchy.
    #[must_use]
    pub fn new() -> Self {
        Self {
            resolver: JurisdictionConflictResolver::new(),
        }
    }

    /// Register a law at a given federalism level.
    ///
    /// `id` is the statute identifier used to refer to the law later (e.g.,
    /// `"fda-labeling"`); `jurisdiction` is the US jurisdiction string (e.g.,
    /// `"US"` for federal, `"US-CA"` for California). `version` is used by the
    /// core resolver for *lex posterior* tie-breaking among same-level laws.
    pub fn register(
        &mut self,
        id: impl Into<String>,
        title: impl Into<String>,
        level: FederalismLevel,
        jurisdiction: impl Into<String>,
        version: u32,
    ) {
        let id = id.into();
        let statute = Statute::new(id, title, Effect::grant("regulatory-effect"))
            .with_jurisdiction(jurisdiction)
            .with_version(version);
        self.resolver
            .add_statute_with_level(statute, level.to_core());
    }

    /// Register a federal law (jurisdiction `"US"`, version 1).
    pub fn register_federal(&mut self, id: impl Into<String>, title: impl Into<String>) {
        self.register(id, title, FederalismLevel::Federal, "US", 1);
    }

    /// Register a state law for the given two-letter state code (jurisdiction
    /// `"US-{code}"`, version 1).
    pub fn register_state(
        &mut self,
        id: impl Into<String>,
        title: impl Into<String>,
        state_code: &str,
    ) {
        let jurisdiction = format!("US-{}", state_code.to_uppercase());
        self.register(id, title, FederalismLevel::State, jurisdiction, 1);
    }

    /// Resolve which of the listed laws controls, using the core resolver's
    /// precedence rules (federal > state > local; then later version).
    #[must_use]
    pub fn resolve(&self, ids: &[&str]) -> ConflictResolution {
        self.resolver.resolve_conflict(ids)
    }

    /// Whether a registered federal law is present that would supersede a
    /// state law in a conflict.
    #[must_use]
    pub fn has_federal_law(&self) -> bool {
        self.resolver.count_by_level(JurisdictionLevel::Federal) > 0
    }

    /// Analyze a federal–state conflict, layering the US preemption doctrine on
    /// top of the core resolver's lex-superior determination.
    ///
    /// * `federal_id` / `state_id` are previously [`register`](Self::register)ed
    ///   law identifiers.
    /// * `kind` is the preemption doctrine asserted.
    /// * `traditional_state_police_power` triggers the presumption against
    ///   preemption, which (for field/conflict, the *implied* doctrines) defeats
    ///   preemption absent a clear and manifest purpose of Congress.
    #[must_use]
    pub fn analyze_conflict(
        &self,
        federal_id: &str,
        state_id: &str,
        kind: PreemptionKind,
        traditional_state_police_power: bool,
    ) -> JurisdictionConflict {
        let core = self.resolver.resolve_conflict(&[federal_id, state_id]);

        // Express preemption is not defeated by the presumption (Congress has
        // spoken). Implied preemption (field/conflict) yields to the presumption
        // in fields of traditional state police power.
        let presumption_applies = traditional_state_police_power
            && matches!(kind, PreemptionKind::Field | PreemptionKind::Conflict);

        let (winner, preemption, explanation) = match kind {
            PreemptionKind::None => (
                state_id.to_string(),
                PreemptionKind::None,
                format!(
                    "No preemption: federal law '{federal_id}' and state law '{state_id}' coexist; \
                     both may be enforced."
                ),
            ),
            _ if presumption_applies => (
                state_id.to_string(),
                PreemptionKind::None,
                format!(
                    "Presumption against preemption applies (traditional state police power): \
                     {} requires a clear and manifest purpose of Congress, which is not shown; \
                     state law '{state_id}' survives.",
                    kind.description()
                ),
            ),
            _ => (
                core.winner.clone(),
                kind,
                format!(
                    "{}: under the Supremacy Clause, federal law '{federal_id}' preempts state law \
                     '{state_id}'. (Core resolution: {} via {}.)",
                    kind.description(),
                    core.winner,
                    core.rule
                ),
            ),
        };

        JurisdictionConflict {
            winner,
            preemption,
            presumption_against_preemption: presumption_applies,
            explanation,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fed_vs_state() -> JurisdictionHierarchy {
        let mut h = JurisdictionHierarchy::new();
        h.register_federal("fda-labeling", "FDA drug labeling requirements");
        h.register_state("ca-warning", "California failure-to-warn tort duty", "CA");
        h
    }

    #[test]
    fn test_federalism_level_mapping() {
        assert_eq!(
            FederalismLevel::Federal.to_core(),
            JurisdictionLevel::Federal
        );
        assert_eq!(FederalismLevel::State.to_core(), JurisdictionLevel::State);
        assert_eq!(FederalismLevel::Local.to_core(), JurisdictionLevel::Local);
        assert_eq!(FederalismLevel::Federal.label(), "Federal");
    }

    #[test]
    fn test_preemption_kind_descriptions() {
        assert!(PreemptionKind::Express.description().contains("Express"));
        assert!(PreemptionKind::Field.description().contains("field"));
        assert!(PreemptionKind::Conflict.description().contains("obstacle"));
        assert!(PreemptionKind::None.description().contains("coexist"));
    }

    #[test]
    fn test_federal_beats_state_by_lex_superior() {
        let h = fed_vs_state();
        assert!(h.has_federal_law());
        let resolution = h.resolve(&["fda-labeling", "ca-warning"]);
        // Federal law (precedence 30) supersedes state law (precedence 20).
        assert_eq!(resolution.winner, "fda-labeling");
    }

    #[test]
    fn test_express_preemption_displaces_state() {
        let h = fed_vs_state();
        let conflict =
            h.analyze_conflict("fda-labeling", "ca-warning", PreemptionKind::Express, false);
        assert_eq!(conflict.preemption, PreemptionKind::Express);
        assert_eq!(conflict.winner, "fda-labeling");
        assert!(!conflict.presumption_against_preemption);
        assert!(conflict.explanation.contains("preempts"));
    }

    #[test]
    fn test_express_preemption_survives_presumption() {
        // Express preemption is NOT defeated by the presumption against preemption.
        let h = fed_vs_state();
        let conflict =
            h.analyze_conflict("fda-labeling", "ca-warning", PreemptionKind::Express, true);
        assert_eq!(conflict.winner, "fda-labeling");
        assert_eq!(conflict.preemption, PreemptionKind::Express);
        assert!(!conflict.presumption_against_preemption);
    }

    #[test]
    fn test_field_preemption_yields_to_state_police_power() {
        // Implied field preemption yields to the presumption in a field of
        // traditional state police power (e.g., health and safety).
        let h = fed_vs_state();
        let conflict =
            h.analyze_conflict("fda-labeling", "ca-warning", PreemptionKind::Field, true);
        assert_eq!(conflict.winner, "ca-warning");
        assert_eq!(conflict.preemption, PreemptionKind::None);
        assert!(conflict.presumption_against_preemption);
        assert!(
            conflict
                .explanation
                .contains("Presumption against preemption")
        );
    }

    #[test]
    fn test_conflict_preemption_without_presumption_displaces_state() {
        let h = fed_vs_state();
        let conflict = h.analyze_conflict(
            "fda-labeling",
            "ca-warning",
            PreemptionKind::Conflict,
            false,
        );
        assert_eq!(conflict.winner, "fda-labeling");
        assert_eq!(conflict.preemption, PreemptionKind::Conflict);
    }

    #[test]
    fn test_no_preemption_coexistence() {
        let h = fed_vs_state();
        let conflict =
            h.analyze_conflict("fda-labeling", "ca-warning", PreemptionKind::None, false);
        assert_eq!(conflict.winner, "ca-warning");
        assert_eq!(conflict.preemption, PreemptionKind::None);
        assert!(conflict.explanation.contains("coexist"));
    }

    #[test]
    fn test_empty_hierarchy_has_no_federal_law() {
        let h = JurisdictionHierarchy::new();
        assert!(!h.has_federal_law());
    }
}
