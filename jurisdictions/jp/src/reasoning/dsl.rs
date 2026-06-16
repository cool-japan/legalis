//! `legalis-dsl` integration for Japanese law (日本法).
//!
//! Renders the statutes modelled in [`crate::reasoning::statute_adapter`] as
//! `legalis-dsl` source text, so the Labour Standards Act / Labour Contract Act
//! rules can be inspected, formatted, diffed, and consumed by the DSL tooling
//! (LSP, documentation generation, structural diffing).
//!
//! legalis-dsl 連携。statute_adapter の Statute を DSL ソースとして出力する。

use super::statute_adapter::all_labor_statutes;

/// Renders every modelled Japanese labour statute as `legalis-dsl` source text.
///
/// Each statute is emitted as a `STATUTE … { WHEN … THEN … }` block by
/// [`legalis_dsl::format_statutes`].
#[must_use]
pub fn statutes_as_dsl() -> String {
    legalis_dsl::format_statutes(&all_labor_statutes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn statutes_render_as_valid_dsl() {
        let statutes = all_labor_statutes();
        assert!(!statutes.is_empty(), "JP must model at least one statute");

        let dsl = statutes_as_dsl();
        assert!(!dsl.is_empty(), "DSL export must not be empty");

        // Every modelled statute's id must appear in the rendered DSL, proving
        // the printer handled each one (covers the full range of condition
        // kinds the JP adapters use: Duration, AttributeEquals, ...).
        for statute in &statutes {
            assert!(
                dsl.contains(statute.id.as_str()),
                "statute {} missing from DSL export",
                statute.id
            );
        }
    }
}
