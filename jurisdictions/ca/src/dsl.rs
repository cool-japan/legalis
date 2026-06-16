//! `legalis-dsl` integration for Canadian law.
//!
//! Renders the federal statutes modelled in this crate as `legalis-dsl` source
//! text, so the Canada Labour Code / Canadian Human Rights Act and the Criminal
//! Code / CDSA / YCJA / Charter criminal rights can be inspected, formatted,
//! diffed, and consumed by the DSL tooling (LSP, documentation generation,
//! structural diffing).
//!
//! The export combines both modelled federal aggregates
//! ([`crate::create_federal_employment_statutes`] and
//! [`crate::create_criminal_statutes`]) into a single DSL document.

use legalis_core::Statute;

use crate::{create_criminal_statutes, create_federal_employment_statutes};

/// Renders every modelled Canadian federal statute as `legalis-dsl` source text.
///
/// Combines [`crate::create_federal_employment_statutes`] and
/// [`crate::create_criminal_statutes`] into a single statute set, then emits
/// each one as a `STATUTE … { WHEN … THEN … }` block via
/// [`legalis_dsl::format_statutes`].
#[must_use]
pub fn statutes_as_dsl() -> String {
    let combined: Vec<Statute> = create_federal_employment_statutes()
        .into_iter()
        .chain(create_criminal_statutes())
        .collect();
    legalis_dsl::format_statutes(&combined)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn statutes_render_as_valid_dsl() {
        let statutes: Vec<Statute> = create_federal_employment_statutes()
            .into_iter()
            .chain(create_criminal_statutes())
            .collect();
        assert!(!statutes.is_empty(), "CA must model at least one statute");

        let dsl = statutes_as_dsl();
        assert!(!dsl.is_empty(), "DSL export must not be empty");

        // Every modelled statute's id must appear in the rendered DSL, proving
        // the printer handled each one across both federal aggregates
        // (employment + criminal).
        for statute in &statutes {
            assert!(
                dsl.contains(statute.id.as_str()),
                "statute {} missing from DSL export",
                statute.id
            );
        }
    }
}
