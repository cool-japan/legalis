//! `legalis-dsl` integration for EU law.
//!
//! Renders the statutes modelled in [`crate::reasoning::statute_adapter`] as
//! `legalis-dsl` source text, so the GDPR / competition / consumer-rights rules
//! can be inspected, formatted, diffed, and consumed by the DSL tooling (LSP,
//! documentation generation, structural diffing).

use super::statute_adapter::all_eu_statutes;

/// Renders every modelled EU statute as `legalis-dsl` source text.
///
/// Each statute is emitted as a `STATUTE … { WHEN … THEN … }` block by
/// [`legalis_dsl::format_statutes`].
#[must_use]
pub fn statutes_as_dsl() -> String {
    legalis_dsl::format_statutes(&all_eu_statutes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn statutes_render_as_valid_dsl() {
        let statutes = all_eu_statutes();
        assert!(!statutes.is_empty(), "EU must model at least one statute");

        let dsl = statutes_as_dsl();
        assert!(!dsl.is_empty(), "DSL export must not be empty");

        for statute in &statutes {
            assert!(
                dsl.contains(statute.id.as_str()),
                "statute {} missing from DSL export",
                statute.id
            );
        }
    }
}
