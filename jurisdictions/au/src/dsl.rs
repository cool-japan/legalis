//! `legalis-dsl` integration for Australian law.
//!
//! Renders the major Commonwealth statutes modelled by
//! [`crate::create_major_statutes`] as `legalis-dsl` source text, so the
//! Australian Consumer Law / Fair Work Act / Corporations Act rules can be
//! inspected, formatted, diffed, and consumed by the DSL tooling (LSP,
//! documentation generation, structural diffing).

use crate::create_major_statutes;

/// Renders every modelled major Australian statute as `legalis-dsl` source text.
///
/// Each statute is emitted as a `STATUTE … { WHEN … THEN … }` block by
/// [`legalis_dsl::format_statutes`].
#[must_use]
pub fn statutes_as_dsl() -> String {
    legalis_dsl::format_statutes(&create_major_statutes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn statutes_render_as_valid_dsl() {
        let statutes = create_major_statutes();
        assert!(!statutes.is_empty(), "AU must model at least one statute");

        let dsl = statutes_as_dsl();
        assert!(!dsl.is_empty(), "DSL export must not be empty");

        // Every modelled statute's id must appear in the rendered DSL, proving
        // the printer handled each one across the full range of condition kinds
        // the AU statutes use.
        for statute in &statutes {
            assert!(
                dsl.contains(statute.id.as_str()),
                "statute {} missing from DSL export",
                statute.id
            );
        }
    }
}
