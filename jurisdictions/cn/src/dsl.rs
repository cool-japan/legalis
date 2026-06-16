//! `legalis-dsl` integration for Chinese law (中国法).
//!
//! Renders the statutes modelled at the crate root (PIPL, Cybersecurity Law,
//! Data Security Law, Civil Code, Company Law, Labor Contract Law, Foreign
//! Investment Law, Anti-Monopoly Law) as `legalis-dsl` source text, so the
//! Chinese statute set can be inspected, formatted, diffed, and consumed by the
//! DSL tooling (LSP, documentation generation, structural diffing).
//!
//! legalis-dsl 连携。crate 根の Statute を DSL ソースとして出力する。

use crate::{
    create_anti_monopoly_statute, create_civil_code_statute, create_company_law_statute,
    create_cybersecurity_statute, create_data_security_statute, create_foreign_investment_statute,
    create_labor_contract_statute, create_pipl_statute,
};
use legalis_core::Statute;

/// Collects every modelled Chinese statute into a single [`Vec`].
///
/// Aggregates all `create_*_statute` builders defined at the crate root, in the
/// order they appear in the legal-system overview (data protection, civil law,
/// corporate law, labour law, foreign investment, antitrust).
#[must_use]
pub fn all_statutes() -> Vec<Statute> {
    vec![
        create_pipl_statute(),
        create_cybersecurity_statute(),
        create_data_security_statute(),
        create_civil_code_statute(),
        create_company_law_statute(),
        create_labor_contract_statute(),
        create_foreign_investment_statute(),
        create_anti_monopoly_statute(),
    ]
}

/// Renders every modelled Chinese statute as `legalis-dsl` source text.
///
/// Each statute is emitted as a `STATUTE … { … }` block by
/// [`legalis_dsl::format_statutes`].
#[must_use]
pub fn statutes_as_dsl() -> String {
    legalis_dsl::format_statutes(&all_statutes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn statutes_render_as_valid_dsl() {
        let statutes = all_statutes();
        assert!(!statutes.is_empty(), "CN must model at least one statute");

        let dsl = statutes_as_dsl();
        assert!(!dsl.is_empty(), "DSL export must not be empty");

        // Every modelled statute's id must appear in the rendered DSL, proving
        // the printer handled each one.
        for statute in &statutes {
            assert!(
                dsl.contains(statute.id.as_str()),
                "statute {} missing from DSL export",
                statute.id
            );
        }
    }
}
